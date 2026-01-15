//! Delete options dialog

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::*;

use super::DialogResult;
use crate::tui::styles::Theme;

/// Options for what to clean up when deleting a session
#[derive(Clone, Debug)]
pub struct DeleteOptions {
    pub delete_worktree: bool,
    pub delete_container: bool,
}

impl Default for DeleteOptions {
    fn default() -> Self {
        Self {
            delete_worktree: false, // Keep worktree by default
            delete_container: true, // Delete container by default
        }
    }
}

/// Dialog for configuring delete options
pub struct DeleteOptionsDialog {
    session_title: String,
    options: DeleteOptions,
    has_worktree: bool,
    has_container: bool,
    worktree_branch: Option<String>,
    container_name: Option<String>,
    focused_field: usize,
}

impl DeleteOptionsDialog {
    pub fn new(
        session_title: String,
        worktree_info: Option<(String, String)>, // (branch, path)
        container_name: Option<String>,
    ) -> Self {
        let has_worktree = worktree_info.is_some();
        let has_container = container_name.is_some();
        let worktree_branch = worktree_info.map(|(b, _)| b);

        Self {
            session_title,
            options: DeleteOptions::default(),
            has_worktree,
            has_container,
            worktree_branch,
            container_name,
            focused_field: 0,
        }
    }

    fn num_fields(&self) -> usize {
        let mut count = 0;
        if self.has_worktree {
            count += 1;
        }
        if self.has_container {
            count += 1;
        }
        count.max(1) // At least 1 to avoid modulo by zero
    }

    fn worktree_field(&self) -> Option<usize> {
        if self.has_worktree {
            Some(0)
        } else {
            None
        }
    }

    fn container_field(&self) -> Option<usize> {
        if self.has_container {
            if self.has_worktree {
                Some(1)
            } else {
                Some(0)
            }
        } else {
            None
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> DialogResult<DeleteOptions> {
        match key.code {
            KeyCode::Esc => DialogResult::Cancel,
            KeyCode::Enter => {
                // Enter always confirms
                DialogResult::Submit(self.options.clone())
            }
            KeyCode::Tab => {
                if self.num_fields() > 1 {
                    self.focused_field = (self.focused_field + 1) % self.num_fields();
                }
                DialogResult::Continue
            }
            KeyCode::BackTab => {
                if self.num_fields() > 1 {
                    self.focused_field = if self.focused_field == 0 {
                        self.num_fields() - 1
                    } else {
                        self.focused_field - 1
                    };
                }
                DialogResult::Continue
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.focused_field > 0 {
                    self.focused_field -= 1;
                }
                DialogResult::Continue
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.focused_field < self.num_fields() - 1 {
                    self.focused_field += 1;
                }
                DialogResult::Continue
            }
            KeyCode::Char(' ') => {
                self.toggle_current_checkbox();
                DialogResult::Continue
            }
            _ => DialogResult::Continue,
        }
    }

    fn toggle_current_checkbox(&mut self) {
        if Some(self.focused_field) == self.worktree_field() {
            self.options.delete_worktree = !self.options.delete_worktree;
        } else if Some(self.focused_field) == self.container_field() {
            self.options.delete_container = !self.options.delete_container;
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let num_options = self.num_fields();
        let dialog_width = 55;
        let dialog_height = 7 + num_options as u16 * 2;

        let x = area.x + (area.width.saturating_sub(dialog_width)) / 2;
        let y = area.y + (area.height.saturating_sub(dialog_height)) / 2;

        let dialog_area = Rect {
            x,
            y,
            width: dialog_width.min(area.width),
            height: dialog_height.min(area.height),
        };

        frame.render_widget(Clear, dialog_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.error))
            .title(" Delete Session ")
            .title_style(Style::default().fg(theme.error).bold());

        let inner = block.inner(dialog_area);
        frame.render_widget(block, dialog_area);

        let mut constraints = vec![
            Constraint::Length(2), // Session title
            Constraint::Length(1), // "Cleanup options:" label
        ];
        for _ in 0..num_options {
            constraints.push(Constraint::Length(2)); // Checkbox
        }
        constraints.push(Constraint::Min(1)); // Hints

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints)
            .split(inner);

        // Session title
        let title_line = Line::from(vec![
            Span::styled("Session: ", Style::default().fg(theme.text)),
            Span::styled(
                format!("\"{}\"", self.session_title),
                Style::default().fg(theme.accent).bold(),
            ),
        ]);
        frame.render_widget(Paragraph::new(title_line), chunks[0]);

        // "Cleanup options:" label
        let label = Paragraph::new(Line::from(Span::styled(
            "Cleanup options:",
            Style::default().fg(theme.text),
        )));
        frame.render_widget(label, chunks[1]);

        let mut chunk_idx = 2;

        // Worktree checkbox
        if self.has_worktree {
            let is_focused = Some(self.focused_field) == self.worktree_field();
            let checkbox = if self.options.delete_worktree {
                "[x]"
            } else {
                "[ ]"
            };

            let label_style = if is_focused {
                Style::default().fg(theme.accent).underlined()
            } else {
                Style::default().fg(theme.text)
            };
            let checkbox_style = if self.options.delete_worktree {
                Style::default().fg(theme.error).bold()
            } else {
                Style::default().fg(theme.dimmed)
            };

            let branch = self.worktree_branch.as_deref().unwrap_or("unknown");
            let wt_line = Line::from(vec![
                Span::styled(checkbox, checkbox_style),
                Span::raw(" "),
                Span::styled("Delete worktree", label_style),
                Span::raw(" "),
                Span::styled(format!("({})", branch), Style::default().fg(theme.dimmed)),
            ]);
            frame.render_widget(Paragraph::new(wt_line), chunks[chunk_idx]);
            chunk_idx += 1;
        }

        // Container checkbox
        if self.has_container {
            let is_focused = Some(self.focused_field) == self.container_field();
            let checkbox = if self.options.delete_container {
                "[x]"
            } else {
                "[ ]"
            };

            let label_style = if is_focused {
                Style::default().fg(theme.accent).underlined()
            } else {
                Style::default().fg(theme.text)
            };
            let checkbox_style = if self.options.delete_container {
                Style::default().fg(theme.error).bold()
            } else {
                Style::default().fg(theme.dimmed)
            };

            let container = self.container_name.as_deref().unwrap_or("unknown");
            let container_line = Line::from(vec![
                Span::styled(checkbox, checkbox_style),
                Span::raw(" "),
                Span::styled("Delete container", label_style),
                Span::raw(" "),
                Span::styled(
                    format!("({})", container),
                    Style::default().fg(theme.dimmed),
                ),
            ]);
            frame.render_widget(Paragraph::new(container_line), chunks[chunk_idx]);
            chunk_idx += 1;
        }

        // Hints
        let hints = Line::from(vec![
            Span::styled("Tab", Style::default().fg(theme.hint)),
            Span::raw(" next  "),
            Span::styled("Space", Style::default().fg(theme.hint)),
            Span::raw(" toggle  "),
            Span::styled("Enter", Style::default().fg(theme.hint)),
            Span::raw(" delete  "),
            Span::styled("Esc", Style::default().fg(theme.hint)),
            Span::raw(" cancel"),
        ]);
        frame.render_widget(Paragraph::new(hints), chunks[chunk_idx]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn shift_key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::SHIFT)
    }

    fn dialog_with_both() -> DeleteOptionsDialog {
        DeleteOptionsDialog::new(
            "Test Session".to_string(),
            Some((
                "feature-branch".to_string(),
                "/path/to/worktree".to_string(),
            )),
            Some("aoe-abc123".to_string()),
        )
    }

    fn dialog_with_worktree_only() -> DeleteOptionsDialog {
        DeleteOptionsDialog::new(
            "Test Session".to_string(),
            Some((
                "feature-branch".to_string(),
                "/path/to/worktree".to_string(),
            )),
            None,
        )
    }

    fn dialog_with_container_only() -> DeleteOptionsDialog {
        DeleteOptionsDialog::new(
            "Test Session".to_string(),
            None,
            Some("aoe-abc123".to_string()),
        )
    }

    #[test]
    fn test_default_options() {
        let options = DeleteOptions::default();
        assert!(!options.delete_worktree); // Keep worktree by default
        assert!(options.delete_container); // Delete container by default
    }

    #[test]
    fn test_initial_state_with_both() {
        let dialog = dialog_with_both();
        assert!(dialog.has_worktree);
        assert!(dialog.has_container);
        assert_eq!(dialog.num_fields(), 2);
        assert_eq!(dialog.worktree_field(), Some(0));
        assert_eq!(dialog.container_field(), Some(1));
    }

    #[test]
    fn test_initial_state_worktree_only() {
        let dialog = dialog_with_worktree_only();
        assert!(dialog.has_worktree);
        assert!(!dialog.has_container);
        assert_eq!(dialog.num_fields(), 1);
        assert_eq!(dialog.worktree_field(), Some(0));
        assert_eq!(dialog.container_field(), None);
    }

    #[test]
    fn test_initial_state_container_only() {
        let dialog = dialog_with_container_only();
        assert!(!dialog.has_worktree);
        assert!(dialog.has_container);
        assert_eq!(dialog.num_fields(), 1);
        assert_eq!(dialog.worktree_field(), None);
        assert_eq!(dialog.container_field(), Some(0));
    }

    #[test]
    fn test_esc_cancels() {
        let mut dialog = dialog_with_both();
        let result = dialog.handle_key(key(KeyCode::Esc));
        assert!(matches!(result, DialogResult::Cancel));
    }

    #[test]
    fn test_enter_confirms() {
        let mut dialog = dialog_with_both();
        let result = dialog.handle_key(key(KeyCode::Enter));
        assert!(matches!(result, DialogResult::Submit(_)));
    }

    #[test]
    fn test_tab_cycles_fields() {
        let mut dialog = dialog_with_both();
        assert_eq!(dialog.focused_field, 0);

        dialog.handle_key(key(KeyCode::Tab));
        assert_eq!(dialog.focused_field, 1);

        dialog.handle_key(key(KeyCode::Tab));
        assert_eq!(dialog.focused_field, 0); // Wrap around
    }

    #[test]
    fn test_tab_single_field_stays() {
        let mut dialog = dialog_with_worktree_only();
        assert_eq!(dialog.focused_field, 0);

        dialog.handle_key(key(KeyCode::Tab));
        assert_eq!(dialog.focused_field, 0); // No change with single field
    }

    #[test]
    fn test_backtab_cycles_reverse() {
        let mut dialog = dialog_with_both();
        assert_eq!(dialog.focused_field, 0);

        dialog.handle_key(shift_key(KeyCode::BackTab));
        assert_eq!(dialog.focused_field, 1); // Wrap to last

        dialog.handle_key(shift_key(KeyCode::BackTab));
        assert_eq!(dialog.focused_field, 0);
    }

    #[test]
    fn test_space_toggles_worktree() {
        let mut dialog = dialog_with_both();
        dialog.focused_field = 0; // Worktree field
        assert!(!dialog.options.delete_worktree);

        dialog.handle_key(key(KeyCode::Char(' ')));
        assert!(dialog.options.delete_worktree);

        dialog.handle_key(key(KeyCode::Char(' ')));
        assert!(!dialog.options.delete_worktree);
    }

    #[test]
    fn test_space_toggles_container() {
        let mut dialog = dialog_with_both();
        dialog.focused_field = 1; // Container field
        assert!(dialog.options.delete_container);

        dialog.handle_key(key(KeyCode::Char(' ')));
        assert!(!dialog.options.delete_container);

        dialog.handle_key(key(KeyCode::Char(' ')));
        assert!(dialog.options.delete_container);
    }

    #[test]
    fn test_submit_returns_options() {
        let mut dialog = dialog_with_both();
        dialog.options.delete_worktree = true;
        dialog.options.delete_container = false;

        let result = dialog.handle_key(key(KeyCode::Enter));
        match result {
            DialogResult::Submit(opts) => {
                assert!(opts.delete_worktree);
                assert!(!opts.delete_container);
            }
            _ => panic!("Expected Submit"),
        }
    }

    #[test]
    fn test_up_down_navigation() {
        let mut dialog = dialog_with_both();
        assert_eq!(dialog.focused_field, 0);

        dialog.handle_key(key(KeyCode::Down));
        assert_eq!(dialog.focused_field, 1);

        dialog.handle_key(key(KeyCode::Down));
        assert_eq!(dialog.focused_field, 1); // Can't go past last

        dialog.handle_key(key(KeyCode::Up));
        assert_eq!(dialog.focused_field, 0);

        dialog.handle_key(key(KeyCode::Up));
        assert_eq!(dialog.focused_field, 0); // Can't go before first
    }
}
