# FAQ

## Why not just use iTerm2?

iTerm2 is a great terminal emulator with solid tmux integration, but it solves a different problem than AoE.

### What iTerm2 provides

**tmux Integration:** Running `tmux -CC` lets iTerm2 act as a native tmux client with native tabs/windows instead of the text-based UI. Sessions persist when iTerm2 quits or SSH disconnects.

**AI Features:** Via an optional AI Plugin, iTerm2 offers an AI Chat sidebar that can suggest and execute terminal commands. It exposes terminal context (directory, shell, history) to the AI.

### What AoE adds

| Feature                                     | AoE | iTerm2 |
| ------------------------------------------- | --- | ------ |
| Multi-agent dashboard with status detection | Yes | No     |
| Git worktree automation                     | Yes | No     |
| Docker sandboxing                           | Yes | No     |
| Per-repo config (`.aoe/config.toml`)        | Yes | No     |
| Agent/terminal view toggle                  | Yes | No     |
| Diff view with editing                      | Yes | No     |
| Session groups/profiles                     | Yes | No     |
| Agent status (running/waiting/idle/error)   | Yes | No     |

### The difference

iTerm2's tmux integration provides session persistence and native window management. AoE is purpose-built for running **multiple AI coding agents in parallel** across different branches, with automation for git worktrees, Docker isolation, and status monitoring.

If you're running one agent at a time, iTerm2 works fine. If you're running several agents on different tasks or branches simultaneously, AoE gives you a dashboard and workflow automation that iTerm2 doesn't provide.

## What about ai-terminal-agent?

[ai-terminal-agent](https://github.com/wpoPR/ai-terminal-agent) is another project that adds multi-AI workspace management to iTerm2. It orchestrates Claude, Gemini, and Codex with shared context.

The difference: ai-terminal-agent focuses on **orchestrating different AIs for different roles** (Gemini for analysis, Claude for implementation). AoE focuses on running **parallel instances of the same agent on different branches** of your codebase.

## What happens when I close AoE?

Nothing. Sessions are tmux sessions running in the background. Open and close `aoe` as often as you like. Sessions only get removed when you explicitly delete them.

## Which AI tools are supported?

Claude Code, OpenCode, Mistral Vibe, Codex CLI, and Gemini CLI. AoE auto-detects which are installed on your system.

## Does AoE work on Linux?

Yes. AoE runs on Linux and macOS. The only requirement is tmux (and Docker if you want sandboxing).

## Can I use AoE over SSH?

Yes. Run `aoe` inside a tmux session when connecting remotely:

```bash
tmux new-session -s main
aoe
```

Use `Ctrl+b L` to toggle back to `aoe` after attaching to an agent session.

## How do I get back to the TUI after attaching to a session?

Press `Ctrl+b d` to detach from the tmux session and return to the TUI. This is the standard tmux detach shortcut.

## What are profiles for?

Profiles let you maintain separate workspaces for different projects or clients. Each profile has its own set of sessions, so you can switch contexts without cluttering a single view.

## Do I need Docker?

No. Docker is optional and only required for sandboxed sessions. Without Docker, AoE runs agents directly on your system.

## What are git worktrees and why use them?

Git worktrees let you have multiple working directories from the same repo, each on a different branch. AoE automates this: create a session with `-w feat/my-feature -b` and AoE creates the branch, worktree, and session together. Delete the session and AoE offers to clean up the worktree.

This lets you run parallel agents on different features without them stepping on each other's files.

## How do I switch between the agent and a regular terminal?

Press `t` to toggle between Agent View and Terminal View. Each session has a paired shell where you can run builds, tests, or git commands without interrupting the agent.

## Can I use the CLI instead of the TUI?

Yes. Everything in the TUI is available via CLI:

```bash
aoe add /path/to/project    # Create a session
aoe list                    # List sessions
aoe attach <session>        # Attach to a session
aoe delete <session>        # Delete a session
```

See `aoe --help` or the [CLI Reference](cli/reference.md) for all commands.

## Claude Code is flickering

This is a known Claude Code issue, not an AoE problem. See: https://github.com/anthropics/claude-code/issues/1913

## Where does AoE store its data?

- **Linux:** `$XDG_CONFIG_HOME/agent-of-empires/` (defaults to `~/.config/agent-of-empires/`)
- **macOS/Windows:** `~/.agent-of-empires/`

## How do I enable debug logging?

```bash
AGENT_OF_EMPIRES_DEBUG=1 aoe
```

Or with the Rust log level:

```bash
RUST_LOG=agent_of_empires=debug aoe
```
