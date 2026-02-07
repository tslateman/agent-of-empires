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
