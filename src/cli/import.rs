//! `agent-of-empires import` command implementation

use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Args;
use serde::Deserialize;

use crate::session::builder::{self, InstanceParams};
use crate::session::{GroupTree, Storage};

use super::add::{detect_tool, is_duplicate_session};

#[derive(Args)]
pub struct ImportArgs {
    /// Path to TOML manifest file
    file: PathBuf,

    /// Validate and print what would be created without touching storage
    #[arg(long)]
    dry_run: bool,

    /// Skip sessions that already exist (match by title+path)
    #[arg(long)]
    skip_existing: bool,

    /// Launch sessions after importing
    #[arg(long)]
    launch: bool,
}

#[derive(Deserialize)]
struct Manifest {
    sessions: Vec<SessionEntry>,
}

fn default_tool() -> String {
    "claude".to_string()
}

#[derive(Deserialize)]
struct SessionEntry {
    title: String,
    path: String,
    #[serde(default)]
    group: Option<String>,
    #[serde(default = "default_tool")]
    tool: String,
    #[serde(default)]
    command: Option<String>,
    #[serde(default)]
    worktree: Option<String>,
    #[serde(default)]
    create_branch: bool,
    #[serde(default)]
    sandbox: bool,
    #[serde(default)]
    sandbox_image: Option<String>,
    #[serde(default)]
    yolo: bool,
}

/// Expand leading `~` to the user's home directory.
fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }
    PathBuf::from(path)
}

pub async fn run(profile: &str, args: ImportArgs) -> Result<()> {
    let content = std::fs::read_to_string(&args.file)
        .with_context(|| format!("Failed to read manifest: {}", args.file.display()))?;

    let manifest: Manifest =
        toml::from_str(&content).with_context(|| "Failed to parse manifest TOML")?;

    if manifest.sessions.is_empty() {
        println!("Manifest contains no sessions.");
        return Ok(());
    }

    let storage = Storage::new(profile)?;
    let (mut instances, groups) = storage.load_with_groups()?;
    let mut group_tree = GroupTree::new_with_groups(&instances, &groups);

    let mut created: Vec<String> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    let mut errors: Vec<(String, String)> = Vec::new();

    for entry in &manifest.sessions {
        let expanded_path = expand_tilde(&entry.path);

        // Validate path exists
        if !expanded_path.is_dir() {
            errors.push((
                entry.title.clone(),
                format!("Path is not a directory: {}", expanded_path.display()),
            ));
            continue;
        }

        let path_str = expanded_path.to_string_lossy().to_string();

        // Check for duplicates
        if is_duplicate_session(&instances, &entry.title, &path_str) {
            if args.skip_existing {
                skipped.push(entry.title.clone());
                continue;
            }
            errors.push((
                entry.title.clone(),
                "Session already exists with same title and path".to_string(),
            ));
            continue;
        }

        // Determine tool and command
        let (tool, command) = if let Some(ref cmd) = entry.command {
            match detect_tool(cmd) {
                Ok(detected) => (detected, Some(cmd.clone())),
                Err(_) => (entry.tool.clone(), Some(cmd.clone())),
            }
        } else {
            (entry.tool.clone(), None)
        };

        let group = entry.group.clone().unwrap_or_default();

        if args.dry_run {
            println!("  {} -> {}", entry.title, expanded_path.display());
            if !group.is_empty() {
                println!("    group: {}", group);
            }
            if tool != "claude" {
                println!("    tool: {}", tool);
            }
            if let Some(ref cmd) = command {
                println!("    command: {}", cmd);
            }
            if let Some(ref branch) = entry.worktree {
                println!(
                    "    worktree: {}{}",
                    branch,
                    if entry.create_branch { " (new)" } else { "" }
                );
            }
            if entry.sandbox {
                println!(
                    "    sandbox: {}",
                    entry.sandbox_image.as_deref().unwrap_or("default image")
                );
                if entry.yolo {
                    println!("    yolo: true");
                }
            }
            created.push(entry.title.clone());
            continue;
        }

        let sandbox_image = entry
            .sandbox_image
            .clone()
            .unwrap_or_else(crate::docker::effective_default_image);

        let params = InstanceParams {
            title: entry.title.clone(),
            path: path_str.clone(),
            group: group.clone(),
            tool: tool.clone(),
            worktree_branch: entry.worktree.clone(),
            create_new_branch: entry.create_branch,
            sandbox: entry.sandbox,
            sandbox_image,
            yolo_mode: entry.yolo,
            extra_env_keys: Vec::new(),
            extra_env_values: Vec::new(),
        };

        let existing_titles: Vec<&str> = instances.iter().map(|i| i.title.as_str()).collect();

        match builder::build_instance(params, &existing_titles) {
            Ok(result) => {
                let mut instance = result.instance;

                // Apply custom command override if specified
                if let Some(ref cmd) = command {
                    instance.command = cmd.clone();
                }

                // Ensure group exists
                if !instance.group_path.is_empty() {
                    group_tree.create_group(&instance.group_path);
                }

                created.push(instance.title.clone());
                instances.push(instance);
            }
            Err(e) => {
                errors.push((entry.title.clone(), format!("{}", e)));
            }
        }
    }

    if !args.dry_run && !created.is_empty() {
        storage.save_with_groups(&instances, &group_tree)?;
    }

    // Launch if requested
    if args.launch && !args.dry_run && !created.is_empty() {
        let term_size = crate::terminal::get_size();
        for title in &created {
            if let Some(idx) = instances.iter().position(|i| i.title == *title) {
                if let Err(e) = instances[idx].start_with_size(term_size) {
                    eprintln!("Failed to launch {}: {}", title, e);
                }
            }
        }
        storage.save_with_groups(&instances, &group_tree)?;
    }

    // Print summary
    if args.dry_run {
        println!("\nDry run: {} session(s) would be created", created.len());
    } else if !created.is_empty() {
        println!(
            "Imported {} session(s): {}",
            created.len(),
            created.join(", ")
        );
    }

    if !skipped.is_empty() {
        println!("Skipped {} existing: {}", skipped.len(), skipped.join(", "));
    }

    if !errors.is_empty() {
        eprintln!("\n{} error(s):", errors.len());
        for (title, err) in &errors {
            eprintln!("  {}: {}", title, err);
        }
        if created.is_empty() {
            bail!("Import failed: no sessions created");
        }
    }

    Ok(())
}
