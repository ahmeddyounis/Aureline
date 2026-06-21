//! Conformance dump for the dictation-edit-parity packet.
//!
//! Prints the canonical packet JSON (default), the Markdown summary (`summary`),
//! or the compact lines (`compact`), so the published doc stays byte-aligned
//! with the in-crate builder. With `write [dir]` it re-mints the checked-in
//! fixtures under `fixtures/voice/dictation-edit-parity/`; with `doc [path]` it
//! re-writes the published companion doc.

use std::path::PathBuf;

use aureline_editor::seeded_dictation_edit_parity_packet;
use aureline_editor::voice_input::{
    write_fixtures, DICTATION_EDIT_PARITY_DOC_REF, DICTATION_EDIT_PARITY_FIXTURES_DIR_REF,
};

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "packet".to_owned());
    let packet = seeded_dictation_edit_parity_packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    match which.as_str() {
        "summary" => print!("{}", packet.render_markdown()),
        "compact" => println!("{}", packet.compact_lines().join("\n")),
        "write" => {
            let dir = std::env::args()
                .nth(2)
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(DICTATION_EDIT_PARITY_FIXTURES_DIR_REF));
            write_fixtures(&dir, &packet).expect("write fixtures");
            eprintln!("wrote dictation edit-parity fixtures to {}", dir.display());
        }
        "doc" => {
            let path = std::env::args()
                .nth(2)
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(DICTATION_EDIT_PARITY_DOC_REF));
            std::fs::write(&path, packet.render_markdown()).expect("write doc");
            eprintln!("wrote dictation edit contract to {}", path.display());
        }
        _ => println!("{}", packet.export_safe_json()),
    }
}
