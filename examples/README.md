# Examples

## Quick Start

```bash
cd /path/to/your/project
aoe import examples/mongols/manifest.toml --launch
```

This creates a group of named sessions, all pointed at your project.

## By Stack

Repo configs (`.aoe/config.toml`) that set defaults for new sessions in a project.

| Example                                 | Description                                              |
| --------------------------------------- | -------------------------------------------------------- |
| [node-project](node-project/)           | Node.js/TypeScript with npm, sandbox, and volume ignores |
| [python-project](python-project/)       | Python with uv, virtual environments                     |
| [rust-project](rust-project/)           | Rust with cargo, target directory ignored                |
| [monorepo](monorepo/)                   | Multi-package monorepo with worktrees                    |
| [custom-dockerfile](custom-dockerfile/) | Extending the sandbox image with project-specific tools  |

```bash
cp -r examples/node-project/.aoe /path/to/your/project/
```

## By Philosophy

Import manifests that create a themed group of agents. Each is functionally different.

| Theme                     | Crew                               | What Makes It Different                                |
| ------------------------- | ---------------------------------- | ------------------------------------------------------ |
| [Mongols](mongols/)       | khan, rider, archer                | No sandbox, no worktrees -- raw speed                  |
| [Spartans](spartans/)     | master-chief, cortana, noble-six   | No sandbox, no worktrees, yolo -- feet first into hell |
| [Zerg](zerg/)             | overlord, zergling, hydralisk      | Sandboxed, yolo mode -- the swarm asks no permission   |
| [Fellowship](fellowship/) | frodo, aragorn, gandalf            | No sandbox, each on its own worktree branch            |
| [Romans](romans/)         | centurion, architect, scout        | Sandboxed, each on its own worktree branch             |
| [Matrix](matrix/)         | neo, trinity, morpheus, oracle     | Sandboxed, worktrees, yolo -- the full stack           |
| [Starfleet](starfleet/)   | picard, riker, data, worf, laforge | Mixed -- Picard on the bridge, away team sandboxed     |

### Usage

```bash
# Preview what would be created
aoe import examples/starfleet/manifest.toml --dry-run

# Import and launch
aoe import examples/starfleet/manifest.toml --launch

# Import into an existing setup (skip duplicates)
aoe import examples/romans/manifest.toml --skip-existing
```
