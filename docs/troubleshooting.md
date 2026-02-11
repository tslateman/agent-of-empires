# Troubleshooting

## tmux

### "tmux not found"

AoE requires tmux. Install it:

```bash
# macOS
brew install tmux

# Debian/Ubuntu
sudo apt install tmux

# Fedora
sudo dnf install tmux

# Arch
sudo pacman -S tmux
```

### Can't return to the TUI after attaching

Press `Ctrl+b d` to detach from the tmux session. This is the standard tmux detach shortcut and returns you to the AoE dashboard.

### Sessions disappear after reboot

tmux sessions don't survive reboots. AoE tracks session metadata in `sessions.json`, but the underlying tmux sessions are gone. Delete stale sessions from the TUI and recreate them.

### Status shows "Unknown" for all sessions

AoE detects agent status by reading tmux pane content. If detection fails:

1. Ensure the agent is actually running (attach with `Enter` to check)
2. Try restarting the session
3. Enable debug logging: `AGENT_OF_EMPIRES_DEBUG=1 aoe`

### Using AoE over SSH

Run `aoe` inside a tmux session to prevent disconnects from killing your dashboard:

```bash
tmux new-session -s main
aoe
```

Use `Ctrl+b L` to toggle back to `aoe` after attaching to an agent session.

### Using AoE with mobile SSH clients (Termius, Blink)

Same approach as SSH above. The outer tmux session protects against flaky mobile connections.

## Docker Sandboxing

### "Docker not found" or sandbox checkbox missing

Install [Docker Desktop](https://www.docker.com/products/docker-desktop/) (macOS/Windows) or [Docker Engine](https://docs.docker.com/engine/install/) (Linux). The sandbox option only appears when AoE detects Docker on your system.

### First sandbox session is slow to start

The first sandboxed session pulls the Docker image, which can take several minutes depending on your connection. Subsequent sessions reuse the cached image.

Pre-pull to avoid the wait:

```bash
docker build -t ghcr.io/tslateman/aoe-sandbox:lite -f docker/Dockerfile docker/
```

### Container killed (OOM)

Symptoms: session exits unexpectedly, container disappears, or "Killed" in output.

**macOS/Windows**: Docker runs inside a VM with limited memory. Increase it in Docker Desktop under **Settings > Resources > Advanced** (8 GB+ recommended).

**All platforms**: Set a per-container limit in `~/.agent-of-empires/config.toml`:

```toml
[sandbox]
memory_limit = "8g"
```

See the [Docker Sandbox guide](guides/sandbox.md#troubleshooting) for details.

### Authentication fails inside sandbox

Agent credentials are stored in persistent Docker volumes (e.g., `aoe-claude-auth`). If auth fails:

1. The first session requires interactive authentication. Attach and complete the auth flow.
2. Subsequent sessions reuse the stored credentials.
3. If credentials expire, delete the auth volume and re-authenticate:

```bash
docker volume rm aoe-claude-auth
```

### GitHub CLI (`gh`) not working in sandbox

The base sandbox image doesn't include `gh`. Use the dev sandbox image:

```toml
[sandbox]
default_image = "ghcr.io/tslateman/aoe-sandbox:full"
```

Or pass a GitHub token:

```toml
[sandbox.environment_values]
GH_TOKEN = "$GITHUB_TOKEN"
```

### Volume mount errors on Ubuntu/Linux

Docker on Linux may have permission issues with volume mounts due to UID/GID mapping. Possible fixes:

1. Run Docker in rootless mode
2. Ensure your user is in the `docker` group: `sudo usermod -aG docker $USER`
3. Check directory permissions on the project folder

### Env vars not reaching the container

Only explicitly listed variables are forwarded. Add them to your config:

```toml
[sandbox]
environment = ["ANTHROPIC_API_KEY", "OPENAI_API_KEY"]
```

Terminal variables (`TERM`, `COLORTERM`, `FORCE_COLOR`, `NO_COLOR`) are always passed through automatically.

## Git Worktrees

### "Not in a git repository"

Run `aoe` from inside a git repository, or provide the full path:

```bash
aoe add /path/to/your/repo -w feat/branch -b
```

### Worktree creation fails with "branch already exists"

The branch exists in the repo. Either:

- Use the existing branch (omit `-b`): `aoe add . -w feat/existing-branch`
- Choose a different branch name

### Git operations fail inside sandboxed worktrees

Worktrees reference the parent repo's `.git` directory. If that directory is outside the Docker mount, git operations break.

**Fix**: Use the bare repo pattern so all paths stay within the project root:

```bash
cd my-project
mv .git .bare
echo "gitdir: ./.bare" > .git
```

See [Worktrees and Sandboxing](guides/sandbox.md#worktrees-and-sandboxing) for the full setup.

### Deleting worktrees fails with untracked changes

Git won't remove a worktree with uncommitted or untracked changes. Either:

1. Commit or stash the changes first
2. Force-remove: `git worktree remove --force <path>`

### Orphaned worktrees after deleting sessions

If sessions were removed without cleaning up worktrees:

```bash
aoe worktree cleanup    # Find orphaned worktrees
git worktree prune      # Clean up stale worktree references
```

## Agent Detection

### Wrong agent launches

AoE auto-detects installed agents. To force a specific agent:

```bash
aoe add -c claude .     # Force Claude Code
aoe add -c opencode .   # Force OpenCode
```

Or set a default in your config:

```toml
[session]
default_tool = "claude"
```

Per-repo defaults go in `.aoe/config.toml`. See [Repo Config](guides/repo-config.md).

## Configuration

### Changes to config.toml have no effect

1. Verify you're editing the right file:
   - **macOS**: `~/.agent-of-empires/config.toml`
   - **Linux**: `~/.config/agent-of-empires/config.toml`
2. Config changes apply to new sessions. Existing sessions keep their original settings.
3. Check [config precedence](guides/configuration.md): repo config overrides profile config, which overrides global config.

### Where is my data stored?

```
~/.agent-of-empires/          # macOS
~/.config/agent-of-empires/   # Linux (XDG)
  config.toml                 # Global settings
  trusted_repos.toml          # Hook trust decisions
  profiles/
    default/
      sessions.json           # Session metadata
      groups.json             # Group hierarchy
```

## TUI

### Small terminal causes rendering issues

AoE needs a minimum terminal size to render properly. Resize your terminal or use a larger font size. Known issue: [#84](https://github.com/njbrake/agent-of-empires/issues/84).

### Claude Code flickering

This is a known Claude Code issue, not an AoE problem. See: https://github.com/anthropics/claude-code/issues/1913

## Still stuck?

- Enable debug logging: `AGENT_OF_EMPIRES_DEBUG=1 aoe`
- Check [open issues](https://github.com/njbrake/agent-of-empires/issues)
- File a new issue with debug logs and your platform info
