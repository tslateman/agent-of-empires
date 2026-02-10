//! Template content for shared context files.

/// Template for HANDOFF.md - session handoff notes between agents.
///
/// Structured with agent-parseable sections: Recent Activity as a timestamped
/// log, Active Decisions for in-flight choices, and Next Steps as prioritized
/// actions.
pub const HANDOFF_TEMPLATE: &str = r#"# Session Handoff

## Recent Activity
<!-- Log format: YYYY-MM-DD HH:MM - [agent/session] - what happened -->
<!-- Most recent first -->

## Active Decisions
<!-- Decisions in flight that the next agent should know about -->
<!-- Format: **Decision**: context and current thinking -->

## Next Steps
<!-- Prioritized list of what to work on next -->
<!-- Format: 1. [priority] description -->

## Blockers
<!-- Any issues blocking progress -->
"#;

/// Template for TASKS.md - shared task tracking between agents.
///
/// Organized into Active (in-progress), Completed (done this cycle), and
/// Deferred (parked with reason) sections.
pub const TASKS_TEMPLATE: &str = r#"# Shared Tasks

## Active
<!-- Tasks currently in progress -->
<!-- Format: - [ ] description (owner, started YYYY-MM-DD) -->

## Completed
<!-- Tasks finished this cycle -->
<!-- Format: - [x] description (completed YYYY-MM-DD) -->

## Deferred
<!-- Tasks parked with reason -->
<!-- Format: - [ ] description -- reason for deferral -->
"#;

/// Template for .claude/CLAUDE.md -- instructions for Claude Code agents
/// on using the shared context protocol.
pub const TEAM_INSTRUCTIONS_TEMPLATE: &str = r#"# Team Context Protocol

This project uses AoE shared context for cross-session coordination.

## On Session Start

1. Read `.aoe/context/HANDOFF.md` for recent activity and active decisions
2. Read `.aoe/context/TASKS.md` for current task state
3. Understand what other agents have done before starting new work

## Before Going Idle or Finishing

1. Update `.aoe/context/HANDOFF.md`:
   - Add a timestamped entry to **Recent Activity** (most recent first)
   - Update **Active Decisions** if you made or changed any decisions
   - Update **Next Steps** with what should happen next
2. Update `.aoe/context/TASKS.md`:
   - Move completed tasks to **Completed** with date
   - Add new tasks discovered during work to **Active**
   - Move blocked tasks to **Deferred** with reason

## Rules

- Always read context files before starting work
- Always update context files before finishing
- Use the log format in HANDOFF.md: `YYYY-MM-DD HH:MM - [agent] - description`
- Keep entries concise -- a few sentences, not paragraphs
- Do not delete other agents' entries; append yours
"#;

/// Bash script for Claude Code SessionStart hook.
/// Reads HANDOFF.md and TASKS.md and prints their content for context injection.
pub const SESSION_START_HOOK: &str = r#"#!/usr/bin/env bash
# AoE SessionStart hook: inject shared context into Claude Code sessions
set -euo pipefail

CONTEXT_DIR="${AOE_CONTEXT_DIR:-.aoe/context}"

if [ -d "$CONTEXT_DIR" ]; then
    echo "=== AoE Shared Context ==="
    echo ""
    if [ -f "$CONTEXT_DIR/HANDOFF.md" ]; then
        echo "--- HANDOFF.md ---"
        cat "$CONTEXT_DIR/HANDOFF.md"
        echo ""
    fi
    if [ -f "$CONTEXT_DIR/TASKS.md" ]; then
        echo "--- TASKS.md ---"
        cat "$CONTEXT_DIR/TASKS.md"
        echo ""
    fi
    echo "=== End Shared Context ==="
fi
"#;

/// Bash script for Claude Code TaskCompleted hook.
/// Validates that HANDOFF.md was updated recently (within 5 minutes).
/// Exits with code 2 if stale, signaling Claude Code to block completion.
pub const TASK_COMPLETED_HOOK: &str = r#"#!/usr/bin/env bash
# AoE TaskCompleted hook: ensure HANDOFF.md was updated before finishing
set -euo pipefail

CONTEXT_DIR="${AOE_CONTEXT_DIR:-.aoe/context}"
HANDOFF="$CONTEXT_DIR/HANDOFF.md"

if [ ! -f "$HANDOFF" ]; then
    exit 0
fi

# Check if HANDOFF.md was modified in the last 5 minutes
if [ "$(uname)" = "Darwin" ]; then
    MTIME=$(stat -f %m "$HANDOFF")
else
    MTIME=$(stat -c %Y "$HANDOFF")
fi
NOW=$(date +%s)
AGE=$(( NOW - MTIME ))

if [ "$AGE" -gt 300 ]; then
    echo "WARNING: HANDOFF.md has not been updated in this session."
    echo "Please update .aoe/context/HANDOFF.md with:"
    echo "  - Recent Activity: what you did"
    echo "  - Next Steps: what should happen next"
    echo "  - Active Decisions: any decisions made"
    exit 2
fi
"#;

/// JSON template for .claude/settings.local.json.
/// Wires SessionStart and TaskCompleted hooks to the AoE hook scripts.
pub const CLAUDE_SETTINGS_TEMPLATE: &str = r#"{
  "hooks": {
    "SessionStart": [
      {
        "type": "command",
        "command": ".aoe/hooks/session-start.sh"
      }
    ],
    "TaskCompleted": [
      {
        "type": "command",
        "command": ".aoe/hooks/task-completed.sh"
      }
    ]
  }
}
"#;
