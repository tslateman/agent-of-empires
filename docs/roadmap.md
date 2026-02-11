# Roadmap

Planned features and improvements, drawn from [open issues](https://github.com/njbrake/agent-of-empires/issues) and community requests.

## In Progress

- **Non-root container support** ([#205](https://github.com/njbrake/agent-of-empires/issues/205)): Run sandbox containers as a standard user instead of root, fixing file ownership issues in mounted volumes.

## Next Up

- **Apple container runtime** ([#156](https://github.com/njbrake/agent-of-empires/issues/156)): Add Apple's lightweight container tool as an alternative to Docker. No daemon needed, uses Virtualization.framework. Unblocks running multiple containers simultaneously without named volume conflicts.
- **Env var passthrough for non-sandbox mode** ([#208](https://github.com/njbrake/agent-of-empires/issues/208)): Pass host environment variables (like `ANTHROPIC_API_KEY`) through to tmux sessions when not using Docker sandboxing.
- **Optional named auth volumes** ([#229](https://github.com/njbrake/agent-of-empires/issues/229)): Make auth volume mounting configurable (named volume vs bind mount) to support running multiple containers simultaneously.
- **Custom instructions on sandbox launch** ([#167](https://github.com/njbrake/agent-of-empires/issues/167)): Pass CLAUDE.md or custom instructions into sandbox containers so agents get project context on launch.

## Planned

- **Cursor CLI support** ([#7](https://github.com/njbrake/agent-of-empires/issues/7)): Add Cursor as a supported coding agent.
- **Version in help dialog** ([#134](https://github.com/njbrake/agent-of-empires/issues/134)): Display the AoE version when pressing `?` to open help.
- **Integration/e2e testing** ([#118](https://github.com/njbrake/agent-of-empires/issues/118)): Build out a proper integration test suite, possibly using tmux-based or Docker-based test environments.
- **Group rename**: Allow renaming a group by pressing `r` when a group is selected.
- **Session discovery**: Scan for existing tmux sessions running Claude and offer to import/adopt them into aoe.

## Ideas

- **OpenTelemetry for state monitoring** ([#49](https://github.com/njbrake/agent-of-empires/issues/49)): Replace polling-based state detection with OTEL instrumentation.
- **Automated docs updating** ([#202](https://github.com/njbrake/agent-of-empires/issues/202)): Bot or CI workflow to keep documentation in sync with code changes.
- **Tutorial videos and guides** ([#81](https://github.com/njbrake/agent-of-empires/issues/81)): Video walkthroughs and blog posts showing common workflows.

## Recently Completed

- Shared context system for agent collaboration
- Git worktree automation with bare repo support
- Docker sandboxing with persistent auth volumes
- Diff view with inline editing
- Per-repo config and hooks with trust system
- Multi-agent support (Claude Code, OpenCode, Mistral Vibe, Codex CLI, Gemini CLI)
- Profiles for separate workspaces
- tmux status bar integration
- Sound effects
- XDG Base Directory support on Linux ([#92](https://github.com/njbrake/agent-of-empires/issues/92))
