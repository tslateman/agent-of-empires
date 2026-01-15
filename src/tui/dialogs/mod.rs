//! TUI dialog components

mod confirm;
mod delete_options;
mod new_session;
mod rename;

pub use confirm::ConfirmDialog;
pub use delete_options::{DeleteOptions, DeleteOptionsDialog};
pub use new_session::{NewSessionData, NewSessionDialog};
pub use rename::RenameDialog;

pub enum DialogResult<T> {
    Continue,
    Cancel,
    Submit(T),
}
