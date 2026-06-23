//! Headless emitter for the rename-preview corpus.
//!
//! Prints the canonical corpus that freezes how Aureline turns a broad rename into a
//! governed preview: candidates split into the editable set and the held set (blocked,
//! conflict, generated, read-only, partial-scope), change-versus-held and
//! current-versus-captured counts, omission reasons and labels that keep every held
//! candidate visible, an evidence class naming whether the set is semantic,
//! framework-derived, runtime-observed, imported, or a lexical fallback, an
//! inspect-before-mutate apply gate that always blocks a blind apply and binds an undo
//! checkpoint, and consumer projections that never flatten the rename into one apply
//! action. With `--lines`, prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-navigation --example dump_rename_preview            # JSON
//! cargo run -p aureline-navigation --example dump_rename_preview -- --lines
//! ```

use aureline_navigation::rename_preview::{rename_preview_lines, rename_preview_set};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = rename_preview_set();
    set.validate()
        .expect("canonical rename-preview corpus validates");

    if want_lines {
        for line in rename_preview_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize rename-preview corpus")
        );
    }
}
