//! Template content for shared context files.

/// Template for HANDOFF.md - session handoff notes between agents.
pub const HANDOFF_TEMPLATE: &str = r#"# Session Handoff

## Current State
<!-- What is the current state of the work? -->

## Recently Completed
<!-- What was just finished? -->

## Next Steps
<!-- What should the next agent work on? -->

## Blockers
<!-- Any issues blocking progress? -->
"#;

/// Template for TASKS.md - shared task tracking between agents.
pub const TASKS_TEMPLATE: &str = r#"# Shared Tasks

## In Progress
- [ ]

## Up Next
- [ ]

## Completed
- [x]

## Backlog
- [ ]
"#;
