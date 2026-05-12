//! Native folder picker adapter for workspace entry.

use std::path::PathBuf;

pub(crate) fn pick_folder() -> Option<PathBuf> {
    let dialog = rfd::FileDialog::new().set_title("Open folder");
    let dialog = match std::env::current_dir() {
        Ok(cwd) => dialog.set_directory(cwd),
        Err(_) => dialog,
    };
    dialog.pick_folder()
}
