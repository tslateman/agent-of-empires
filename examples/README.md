# Examples

Sample configurations for common project setups. Copy the relevant `.aoe/config.toml` into your project and adjust as needed.

| Example                                 | Description                                              |
| --------------------------------------- | -------------------------------------------------------- |
| [node-project](node-project/)           | Node.js/TypeScript with npm, sandbox, and volume ignores |
| [python-project](python-project/)       | Python with uv, virtual environments                     |
| [rust-project](rust-project/)           | Rust with cargo, target directory ignored                |
| [monorepo](monorepo/)                   | Multi-package monorepo with worktrees                    |
| [custom-dockerfile](custom-dockerfile/) | Extending the sandbox image with project-specific tools  |

## Usage

```bash
# Copy an example config into your project
cp -r examples/node-project/.aoe /path/to/your/project/

# Edit to match your needs
$EDITOR /path/to/your/project/.aoe/config.toml

# Initialize (or just use the copied config)
cd /path/to/your/project
aoe
```
