//! Golden tests for status detection
//!
//! These tests verify that status detection works correctly against real
//! terminal captures from Claude Code and OpenCode. When either tool updates
//! their TUI, these tests will fail if the detection logic no longer works.
//!
//! To update fixtures after a tool update:
//! 1. Run: scripts/capture-fixtures.sh <tool> <state>
//! 2. Verify the new captures look correct
//! 3. Update detection logic if needed
//! 4. Re-run tests

use agent_of_empires::session::Status;
use agent_of_empires::tmux::{detect_claude_status, detect_opencode_status};
use std::fs;
use std::path::Path;

fn load_fixture(tool: &str, state: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(tool)
        .join(format!("{}.txt", state));

    fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "Failed to load fixture {}/{}.txt: {}\nPath: {:?}",
            tool, state, e, path
        )
    })
}

fn strip_fixture_header(content: &str) -> String {
    content
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
}

mod claude_code {
    use super::*;

    #[test]
    fn test_running_state() {
        let fixture = load_fixture("claude_code", "running");
        let content = strip_fixture_header(&fixture);
        let status = detect_claude_status(&content);

        assert_eq!(
            status,
            Status::Running,
            "Claude Code running fixture should detect as Running.\n\
             Fixture content:\n{}\n\n\
             If Claude Code changed their TUI, update the fixture and/or detection logic.",
            content
        );
    }

    #[test]
    fn test_waiting_question_state() {
        let fixture = load_fixture("claude_code", "waiting_question");
        let content = strip_fixture_header(&fixture);
        let status = detect_claude_status(&content);

        assert_eq!(
            status,
            Status::Waiting,
            "Claude Code waiting_question fixture should detect as Waiting.\n\
             Fixture content:\n{}\n\n\
             If Claude Code changed their TUI, update the fixture and/or detection logic.",
            content
        );
    }

    #[test]
    fn test_waiting_permission_state() {
        let fixture = load_fixture("claude_code", "waiting_permission");
        let content = strip_fixture_header(&fixture);
        let status = detect_claude_status(&content);

        assert_eq!(
            status,
            Status::Waiting,
            "Claude Code waiting_permission fixture should detect as Waiting.\n\
             Fixture content:\n{}\n\n\
             If Claude Code changed their TUI, update the fixture and/or detection logic.",
            content
        );
    }

    #[test]
    fn test_idle_state() {
        let fixture = load_fixture("claude_code", "idle");
        let content = strip_fixture_header(&fixture);
        let status = detect_claude_status(&content);

        assert_eq!(
            status,
            Status::Idle,
            "Claude Code idle fixture should detect as Idle.\n\
             Fixture content:\n{}\n\n\
             If Claude Code changed their TUI, update the fixture and/or detection logic.",
            content
        );
    }
}

mod opencode {
    use super::*;

    #[test]
    fn test_running_state() {
        let fixture = load_fixture("opencode", "running");
        let content = strip_fixture_header(&fixture).to_lowercase();
        let status = detect_opencode_status(&content);

        assert_eq!(
            status,
            Status::Running,
            "OpenCode running fixture should detect as Running.\n\
             Fixture content:\n{}\n\n\
             If OpenCode changed their TUI, update the fixture and/or detection logic.",
            content
        );
    }

    #[test]
    fn test_waiting_permission_state() {
        let fixture = load_fixture("opencode", "waiting_permission");
        let content = strip_fixture_header(&fixture).to_lowercase();
        let status = detect_opencode_status(&content);

        assert_eq!(
            status,
            Status::Waiting,
            "OpenCode waiting_permission fixture should detect as Waiting.\n\
             Fixture content:\n{}\n\n\
             If OpenCode changed their TUI, update the fixture and/or detection logic.",
            content
        );
    }

    #[test]
    fn test_idle_state() {
        let fixture = load_fixture("opencode", "idle");
        let content = strip_fixture_header(&fixture).to_lowercase();
        let status = detect_opencode_status(&content);

        assert_eq!(
            status,
            Status::Idle,
            "OpenCode idle fixture should detect as Idle.\n\
             Fixture content:\n{}\n\n\
             If OpenCode changed their TUI, update the fixture and/or detection logic.",
            content
        );
    }
}
