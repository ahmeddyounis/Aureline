//! Conformance dump for the voice provider routing packet.
//!
//! Prints the canonical support export (default), the Markdown summary
//! (`summary`), or the compact lines (`compact`), so the published doc and the
//! checked-in artifact stay byte-aligned with the in-crate builder. With
//! `write [dir]` it re-mints the checked-in fixtures under
//! `fixtures/voice/provider-locality-and-policy/`, and `support [path]` re-mints
//! the support-export artifact.

use std::path::PathBuf;

use aureline_shell::voice_provider_routing::{
    seeded_voice_provider_routing_packet, write_fixtures, write_support_export,
    VOICE_PROVIDER_ROUTING_ARTIFACT_REF, VOICE_PROVIDER_ROUTING_FIXTURES_DIR_REF,
};

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "export".to_owned());
    let packet = seeded_voice_provider_routing_packet();

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
                .unwrap_or_else(|| PathBuf::from(VOICE_PROVIDER_ROUTING_FIXTURES_DIR_REF));
            write_fixtures(&dir, &packet).expect("write fixtures");
            eprintln!("wrote voice routing fixtures to {}", dir.display());
        }
        "support" => {
            let path = std::env::args()
                .nth(2)
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(VOICE_PROVIDER_ROUTING_ARTIFACT_REF));
            write_support_export(&path, &packet).expect("write support export");
            eprintln!("wrote voice routing support export to {}", path.display());
        }
        _ => println!("{}", packet.export_safe_json()),
    }
}
