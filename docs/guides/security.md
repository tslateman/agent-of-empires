# Security Best Practices

AoE runs AI coding agents that can read and modify your codebase. This guide covers how to manage credentials, understand the sandbox security model, and reduce risk.

## API Key Management

AI agents need API keys (e.g., `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`). Handle them carefully:

**Do**: Store keys in your shell profile (`~/.bashrc`, `~/.zshrc`) or a secrets manager. Pass them to AoE via environment variables.

**Don't**: Put keys in `.aoe/config.toml` if you commit that file to version control. Use `environment` (pass-through) instead of `environment_values` (inline) for secrets:

```toml
# Good: reads from host environment at runtime
[sandbox]
environment = ["ANTHROPIC_API_KEY"]

# Careful: value stored in config file
[sandbox.environment_values]
GH_TOKEN = "$AOE_GH_TOKEN"    # OK: references host env var
API_KEY = "sk-literal-value"   # Risky if config is committed
```

For repo-level config (`.aoe/config.toml`), use the `$VAR` syntax so actual secrets stay in your shell environment, not in version control.

## Docker Sandbox Security Model

Sandboxing isolates agents in Docker containers. This provides meaningful protection but is not a hard security boundary.

### What sandboxing prevents

- Agents modifying files outside your project directory
- Agents installing packages or tools on your host system
- Agents accessing other projects or home directory files
- Cross-session interference (each session gets its own container)

### What sandboxing does not prevent

- Agents reading your project source code (the project is mounted read-write at `/workspace`)
- Network access from within the container (agents can make API calls, clone repos, etc.)
- Access to credentials you explicitly pass via `environment` or `environment_values`
- Access to mounted volumes (`extra_volumes`, auth volumes)

### Container runs as root by default

The default sandbox images run as root inside the container. This is a convenience trade-off: AI agents often need to install packages. The container's root user has no special privileges on the host. Support for non-root containers is tracked in [#205](https://github.com/njbrake/agent-of-empires/issues/205).

### Auth volumes are shared

Persistent Docker volumes (e.g., `aoe-claude-auth`) store agent credentials and are shared across all sandboxed sessions. Any container can read another session's auth tokens. If you need session-level credential isolation, use separate auth volumes per profile.

## Hook Trust System

Repo config files (`.aoe/config.toml`) can define hooks that run shell commands:

```toml
[hooks]
on_create = ["npm install"]
on_launch = ["npm run dev"]
```

When AoE encounters hooks in a repo for the first time, it prompts you to review and approve them. This prevents cloned repos from silently running arbitrary commands.

- Trust decisions are stored in `~/.agent-of-empires/trusted_repos.toml`
- If hook commands change (someone updates `.aoe/config.toml`), AoE re-prompts
- Use `--trust-hooks` only for repos you control: `aoe add --trust-hooks .`

**Review hooks before approving them.** A malicious repo could define hooks that exfiltrate data or install backdoors.

## File Permissions

AoE stores configuration and session data in your home directory:

```
~/.agent-of-empires/
  config.toml           # May contain env var references
  trusted_repos.toml    # Hook trust decisions
  profiles/*/           # Session data
```

These files are created with your user's default permissions. If your config references sensitive values, ensure the directory isn't world-readable:

```bash
chmod 700 ~/.agent-of-empires
```

## Recommendations

1. **Use sandboxing** for untrusted codebases or when running agents on code you didn't write
2. **Pass secrets via environment**, not inline values in committed config files
3. **Review hooks** before approving them in new repositories
4. **Keep AoE updated** to get security fixes: `aoe update` or `brew upgrade aoe`
5. **Use per-repo config** to scope sandbox settings (custom images, volume mounts, env vars) to each project rather than relying on global defaults
6. **Minimize volume mounts**: only add `extra_volumes` that the agent actually needs

## See Also

- [Docker Sandbox](sandbox.md) -- container configuration and volume mounts
- [Repo Config & Hooks](repo-config.md) -- hook system and trust model
- [Configuration Reference](configuration.md) -- all config options
