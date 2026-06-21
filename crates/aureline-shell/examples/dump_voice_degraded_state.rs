//! Conformance dump for the voice degraded-state packet.
//!
//! Prints the canonical packet JSON (default), the Markdown matrix (`summary`),
//! or the compact lines (`compact`), so the published doc and matrix stay
//! byte-aligned with the in-crate builder. With `write [dir]` it re-mints the
//! checked-in fixtures under `fixtures/voice/fallback-and-noisy-env/`.

use std::path::PathBuf;

use aureline_shell::voice_degraded_state::{
    seeded_voice_degraded_state_packet, write_fixtures, VOICE_DEGRADED_STATE_FIXTURES_DIR_REF,
};

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "packet".to_owned());
    let packet = seeded_voice_degraded_state_packet();

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
                .unwrap_or_else(|| PathBuf::from(VOICE_DEGRADED_STATE_FIXTURES_DIR_REF));
            write_fixtures(&dir, &packet).expect("write fixtures");
            eprintln!("wrote voice degraded-state fixtures to {}", dir.display());
        }
        _ => println!("{}", packet.export_safe_json()),
    }
}
