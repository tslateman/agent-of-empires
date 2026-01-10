#!/bin/bash
# Capture terminal fixtures for status detection golden tests
#
# Usage:
#   ./scripts/capture-fixtures.sh <tool> <state> <tmux_session>
#
# Examples:
#   ./scripts/capture-fixtures.sh claude running aoe_myproject_abc12345
#   ./scripts/capture-fixtures.sh opencode waiting_question aoe_task_def67890
#
# States: running, waiting_question, waiting_permission, idle
#
# This script captures the current terminal content from a tmux session
# and saves it as a fixture file for golden testing.

set -e

capitalize() {
    echo "$1" | awk '{print toupper(substr($0,1,1)) tolower(substr($0,2))}'
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
FIXTURES_DIR="$PROJECT_ROOT/tests/fixtures"

usage() {
    echo "Usage: $0 <tool> <state> <tmux_session>"
    echo ""
    echo "Arguments:"
    echo "  tool          Tool name: 'claude' or 'opencode'"
    echo "  state         State to capture: 'running', 'waiting_question', 'waiting_permission', 'idle'"
    echo "  tmux_session  Name of the tmux session to capture from"
    echo ""
    echo "Examples:"
    echo "  $0 claude running aoe_myproject_abc12345"
    echo "  $0 opencode waiting_question aoe_task_def67890"
    echo ""
    echo "Steps to capture a fixture:"
    echo "  1. Start the tool in a tmux session managed by aoe"
    echo "  2. Get the tool into the desired state (running, waiting, etc.)"
    echo "  3. Run this script with the appropriate arguments"
    echo "  4. Verify the captured output looks correct"
    echo "  5. Run 'cargo test --test status_detection' to verify detection works"
    exit 1
}

# Validate arguments
if [ $# -ne 3 ]; then
    usage
fi

TOOL="$1"
STATE="$2"
SESSION="$3"

# Validate tool
case "$TOOL" in
    claude|claude_code)
        TOOL_DIR="claude_code"
        TOOL_DISPLAY="Claude Code"
        ;;
    opencode)
        TOOL_DIR="opencode"
        TOOL_DISPLAY="OpenCode"
        ;;
    *)
        echo "Error: Invalid tool '$TOOL'. Must be 'claude' or 'opencode'."
        exit 1
        ;;
esac

# Validate state
case "$STATE" in
    running|waiting_question|waiting_permission|idle)
        ;;
    *)
        echo "Error: Invalid state '$STATE'."
        echo "Valid states: running, waiting_question, waiting_permission, idle"
        exit 1
        ;;
esac

# Check if tmux session exists
if ! tmux has-session -t "$SESSION" 2>/dev/null; then
    echo "Error: tmux session '$SESSION' does not exist."
    echo ""
    echo "Available sessions:"
    tmux list-sessions 2>/dev/null || echo "  (no sessions)"
    exit 1
fi

# Create output directory if needed
OUTPUT_DIR="$FIXTURES_DIR/$TOOL_DIR"
mkdir -p "$OUTPUT_DIR"

OUTPUT_FILE="$OUTPUT_DIR/$STATE.txt"

# Get tool version if possible
get_version() {
    case "$TOOL_DIR" in
        claude_code)
            claude --version 2>/dev/null | head -1 || echo "unknown"
            ;;
        opencode)
            opencode --version 2>/dev/null | head -1 || echo "unknown"
            ;;
    esac
}

VERSION=$(get_version)
DATE=$(date +%Y-%m-%d)

# Capture pane content (last 50 lines to match detection logic)
CONTENT=$(tmux capture-pane -t "$SESSION" -p -S -50)

# Capitalize state for display
STATE_DISPLAY=$(capitalize "$STATE")

# Write fixture file with header
cat > "$OUTPUT_FILE" << EOF
# FIXTURE: $TOOL_DISPLAY - $STATE_DISPLAY State
# Captured from: $VERSION
# Capture date: $DATE
# To update: scripts/capture-fixtures.sh $TOOL $STATE <tmux_session>
#
# Expected status: $(echo "$STATE" | sed 's/waiting.*/Waiting/; s/running/Running/; s/idle/Idle/')
# Key indicators: (update this after reviewing the capture)

$CONTENT
EOF

echo "Fixture captured successfully!"
echo ""
echo "Output: $OUTPUT_FILE"
echo ""
echo "Next steps:"
echo "  1. Review the captured content: cat $OUTPUT_FILE"
echo "  2. Update the 'Key indicators' comment if needed"
echo "  3. Run tests: cargo test --test status_detection"
echo ""
echo "If tests fail, you may need to update the detection logic in:"
echo "  src/tmux/session.rs (detect_claude_status or detect_opencode_status)"
