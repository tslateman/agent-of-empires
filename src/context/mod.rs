//! Shared context system for AoE agents.
//!
//! Provides file-based shared context so agents working on the same project can
//! share handoff notes and tasks. Context is stored in `.aoe/context/` in the
//! main repo (accessible from all worktrees).

pub mod templates;

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::git::GitWorktree;

/// Default path for context directory relative to project root.
pub const DEFAULT_CONTEXT_PATH: &str = ".aoe/context";

/// Name of the symlink created in worktrees pointing to the context directory.
pub const WORKTREE_SYMLINK_NAME: &str = ".aoe-context";

/// Environment variable name for the context directory path.
pub const CONTEXT_DIR_ENV_VAR: &str = "AOE_CONTEXT_DIR";

/// Find the context directory for a given project path.
///
/// For worktrees, this resolves to the main repo's context directory.
/// Returns the path to the context directory if it exists, None otherwise.
pub fn find_context_dir(project_path: &Path, context_path: &str) -> Result<Option<PathBuf>> {
    let main_repo = resolve_main_repo(project_path)?;
    let context_dir = main_repo.join(context_path);

    if context_dir.exists() && context_dir.is_dir() {
        Ok(Some(context_dir))
    } else {
        Ok(None)
    }
}

/// Resolve the main repository path from any project path (including worktrees).
fn resolve_main_repo(project_path: &Path) -> Result<PathBuf> {
    if GitWorktree::is_git_repo(project_path) {
        GitWorktree::find_main_repo(project_path)
            .map_err(|e| anyhow::anyhow!("Failed to find main repo: {}", e))
    } else {
        Ok(project_path.to_path_buf())
    }
}

/// Initialize the shared context directory with template files.
///
/// Creates the context directory (default: `.aoe/context/`) in the main repo
/// and populates it with HANDOFF.md and TASKS.md if they don't exist.
///
/// Returns the path to the created context directory.
pub fn init_context(project_path: &Path, context_path: &str) -> Result<PathBuf> {
    let main_repo = resolve_main_repo(project_path)?;
    let context_dir = main_repo.join(context_path);

    // Create context directory if it doesn't exist
    if !context_dir.exists() {
        fs::create_dir_all(&context_dir).with_context(|| {
            format!(
                "Failed to create context directory: {}",
                context_dir.display()
            )
        })?;
        tracing::info!("Created context directory at {}", context_dir.display());
    }

    // Create HANDOFF.md if it doesn't exist
    let handoff_path = context_dir.join("HANDOFF.md");
    if !handoff_path.exists() {
        fs::write(&handoff_path, templates::HANDOFF_TEMPLATE)
            .with_context(|| format!("Failed to create HANDOFF.md: {}", handoff_path.display()))?;
        tracing::info!("Created HANDOFF.md");
    }

    // Create TASKS.md if it doesn't exist
    let tasks_path = context_dir.join("TASKS.md");
    if !tasks_path.exists() {
        fs::write(&tasks_path, templates::TASKS_TEMPLATE)
            .with_context(|| format!("Failed to create TASKS.md: {}", tasks_path.display()))?;
        tracing::info!("Created TASKS.md");
    }

    // Ensure .aoe/context/ is in .gitignore
    ensure_gitignored(&main_repo, context_path)?;

    Ok(context_dir)
}

/// Ensure the context path is in the project's .gitignore.
fn ensure_gitignored(main_repo: &Path, context_path: &str) -> Result<()> {
    let gitignore_path = main_repo.join(".gitignore");

    // Normalize the context path for gitignore (ensure leading slash for root-relative)
    let gitignore_entry = if context_path.starts_with('/') {
        context_path.to_string()
    } else {
        format!("/{}", context_path)
    };

    if gitignore_path.exists() {
        let content = fs::read_to_string(&gitignore_path)?;

        // Check if already gitignored (exact match or with trailing slash)
        let patterns_to_check = [
            gitignore_entry.clone(),
            format!("{}/", gitignore_entry),
            context_path.to_string(),
            format!("{}/", context_path),
        ];

        let already_ignored = content
            .lines()
            .any(|line| patterns_to_check.iter().any(|p| line.trim() == p));

        if !already_ignored {
            // Append to .gitignore
            let mut new_content = content;
            if !new_content.ends_with('\n') && !new_content.is_empty() {
                new_content.push('\n');
            }
            new_content.push_str(&format!("{}/\n", gitignore_entry));
            fs::write(&gitignore_path, new_content)?;
            tracing::info!("Added {} to .gitignore", gitignore_entry);
        }
    } else {
        // Create .gitignore with the context entry
        fs::write(&gitignore_path, format!("{}/\n", gitignore_entry))?;
        tracing::info!("Created .gitignore with {}", gitignore_entry);
    }

    Ok(())
}

/// Create a symlink in the worktree pointing to the main repo's context directory.
///
/// This allows agents in worktrees to find the context directory easily via
/// a consistent `.aoe-context` symlink in their working directory.
pub fn setup_worktree_symlink(worktree_path: &Path, context_dir: &Path) -> Result<()> {
    let symlink_path = worktree_path.join(WORKTREE_SYMLINK_NAME);

    // Skip if symlink already exists and points to the right place
    if symlink_path.exists() || symlink_path.is_symlink() {
        if let Ok(target) = fs::read_link(&symlink_path) {
            // Resolve both paths for comparison
            let target_canonical = worktree_path.join(&target).canonicalize().ok();
            let context_canonical = context_dir.canonicalize().ok();

            if target_canonical == context_canonical {
                return Ok(());
            }
        }
        // Remove stale symlink
        if symlink_path.is_symlink() || symlink_path.exists() {
            fs::remove_file(&symlink_path).with_context(|| {
                format!("Failed to remove stale symlink: {}", symlink_path.display())
            })?;
        }
    }

    // Calculate relative path from worktree to context dir
    let relative_path = compute_relative_path(worktree_path, context_dir)?;

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&relative_path, &symlink_path)
            .with_context(|| format!("Failed to create symlink: {}", symlink_path.display()))?;
    }

    #[cfg(windows)]
    {
        // On Windows, use directory junction for better compatibility
        std::os::windows::fs::symlink_dir(&relative_path, &symlink_path)
            .with_context(|| format!("Failed to create symlink: {}", symlink_path.display()))?;
    }

    tracing::info!(
        "Created context symlink: {} -> {}",
        symlink_path.display(),
        relative_path.display()
    );

    Ok(())
}

/// Compute a relative path from base to target.
fn compute_relative_path(base: &Path, target: &Path) -> Result<PathBuf> {
    let base_canonical = base
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize base path: {}", base.display()))?;
    let target_canonical = target
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize target path: {}", target.display()))?;

    // Find common ancestor and compute relative path
    let mut base_components = base_canonical.components().peekable();
    let mut target_components = target_canonical.components().peekable();

    // Skip common prefix
    while let (Some(b), Some(t)) = (base_components.peek(), target_components.peek()) {
        if b != t {
            break;
        }
        base_components.next();
        target_components.next();
    }

    // Build relative path: ".." for each remaining base component, then target components
    let mut relative = PathBuf::new();
    for _ in base_components {
        relative.push("..");
    }
    for component in target_components {
        relative.push(component);
    }

    Ok(relative)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_test_repo() -> (TempDir, PathBuf) {
        let dir = TempDir::new().unwrap();
        let repo_path = dir.path().to_path_buf();

        // Initialize as git repo
        git2::Repository::init(&repo_path).unwrap();

        (dir, repo_path)
    }

    #[test]
    fn test_find_context_dir_not_exists() {
        let (dir, repo_path) = setup_test_repo();
        let result = find_context_dir(&repo_path, DEFAULT_CONTEXT_PATH).unwrap();
        assert!(result.is_none());
        drop(dir);
    }

    #[test]
    fn test_find_context_dir_exists() {
        let (dir, repo_path) = setup_test_repo();
        let context_dir = repo_path.join(DEFAULT_CONTEXT_PATH);
        fs::create_dir_all(&context_dir).unwrap();

        let result = find_context_dir(&repo_path, DEFAULT_CONTEXT_PATH).unwrap();
        assert!(result.is_some());
        // Canonicalize both paths to handle /var -> /private/var symlinks on macOS
        let result_canonical = result.unwrap().canonicalize().unwrap();
        let expected_canonical = context_dir.canonicalize().unwrap();
        assert_eq!(result_canonical, expected_canonical);
        drop(dir);
    }

    #[test]
    fn test_init_context_creates_directory() {
        let (dir, repo_path) = setup_test_repo();

        let result = init_context(&repo_path, DEFAULT_CONTEXT_PATH).unwrap();

        assert!(result.exists());
        assert!(result.join("HANDOFF.md").exists());
        assert!(result.join("TASKS.md").exists());
        drop(dir);
    }

    #[test]
    fn test_init_context_preserves_existing_files() {
        let (dir, repo_path) = setup_test_repo();
        let context_dir = repo_path.join(DEFAULT_CONTEXT_PATH);
        fs::create_dir_all(&context_dir).unwrap();

        // Create existing HANDOFF.md with custom content
        let custom_content = "# Custom Handoff\nMy notes here";
        fs::write(context_dir.join("HANDOFF.md"), custom_content).unwrap();

        init_context(&repo_path, DEFAULT_CONTEXT_PATH).unwrap();

        // Verify custom content was preserved
        let content = fs::read_to_string(context_dir.join("HANDOFF.md")).unwrap();
        assert_eq!(content, custom_content);

        // Verify TASKS.md was created
        assert!(context_dir.join("TASKS.md").exists());
        drop(dir);
    }

    #[test]
    fn test_init_context_adds_to_gitignore() {
        let (dir, repo_path) = setup_test_repo();

        init_context(&repo_path, DEFAULT_CONTEXT_PATH).unwrap();

        let gitignore = fs::read_to_string(repo_path.join(".gitignore")).unwrap();
        assert!(gitignore.contains("/.aoe/context/"));
        drop(dir);
    }

    #[test]
    fn test_init_context_preserves_existing_gitignore() {
        let (dir, repo_path) = setup_test_repo();

        // Create existing .gitignore
        fs::write(repo_path.join(".gitignore"), "node_modules/\n.env\n").unwrap();

        init_context(&repo_path, DEFAULT_CONTEXT_PATH).unwrap();

        let gitignore = fs::read_to_string(repo_path.join(".gitignore")).unwrap();
        assert!(gitignore.contains("node_modules/"));
        assert!(gitignore.contains(".env"));
        assert!(gitignore.contains("/.aoe/context/"));
        drop(dir);
    }

    #[test]
    fn test_init_context_skips_duplicate_gitignore_entry() {
        let (dir, repo_path) = setup_test_repo();

        // Create existing .gitignore with context already ignored
        fs::write(repo_path.join(".gitignore"), "/.aoe/context/\n").unwrap();

        init_context(&repo_path, DEFAULT_CONTEXT_PATH).unwrap();

        let gitignore = fs::read_to_string(repo_path.join(".gitignore")).unwrap();
        // Should not have duplicate entries
        assert_eq!(gitignore.matches("/.aoe/context/").count(), 1);
        drop(dir);
    }

    #[test]
    fn test_compute_relative_path() {
        let dir = TempDir::new().unwrap();
        let base = dir.path().join("worktree");
        let target = dir.path().join(".aoe/context");

        fs::create_dir_all(&base).unwrap();
        fs::create_dir_all(&target).unwrap();

        let relative = compute_relative_path(&base, &target).unwrap();

        // Should go up one level then into .aoe/context
        assert!(relative.to_string_lossy().contains(".."));
        assert!(relative.to_string_lossy().contains(".aoe"));
    }

    #[test]
    fn test_setup_worktree_symlink() {
        let dir = TempDir::new().unwrap();
        let worktree_path = dir.path().join("worktree");
        let context_dir = dir.path().join(".aoe/context");

        fs::create_dir_all(&worktree_path).unwrap();
        fs::create_dir_all(&context_dir).unwrap();

        setup_worktree_symlink(&worktree_path, &context_dir).unwrap();

        let symlink_path = worktree_path.join(WORKTREE_SYMLINK_NAME);
        assert!(symlink_path.is_symlink());

        // Verify symlink resolves to context dir
        let resolved = symlink_path.canonicalize().unwrap();
        assert_eq!(resolved, context_dir.canonicalize().unwrap());
    }

    #[test]
    fn test_setup_worktree_symlink_idempotent() {
        let dir = TempDir::new().unwrap();
        let worktree_path = dir.path().join("worktree");
        let context_dir = dir.path().join(".aoe/context");

        fs::create_dir_all(&worktree_path).unwrap();
        fs::create_dir_all(&context_dir).unwrap();

        // Call twice - should not error
        setup_worktree_symlink(&worktree_path, &context_dir).unwrap();
        setup_worktree_symlink(&worktree_path, &context_dir).unwrap();

        let symlink_path = worktree_path.join(WORKTREE_SYMLINK_NAME);
        assert!(symlink_path.is_symlink());
    }

    #[test]
    fn test_custom_context_path() {
        let (dir, repo_path) = setup_test_repo();

        let custom_path = ".custom/shared-context";
        let result = init_context(&repo_path, custom_path).unwrap();

        assert!(result.exists());
        // Canonicalize both paths to handle /var -> /private/var symlinks on macOS
        let result_canonical = result.canonicalize().unwrap();
        let expected_canonical = repo_path.join(custom_path).canonicalize().unwrap();
        assert_eq!(result_canonical, expected_canonical);
        assert!(result.join("HANDOFF.md").exists());
        assert!(result.join("TASKS.md").exists());

        let gitignore = fs::read_to_string(repo_path.join(".gitignore")).unwrap();
        assert!(gitignore.contains("/.custom/shared-context/"));
        drop(dir);
    }
}
