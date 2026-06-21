//! Conformance dump for the voice support-export packet.
//!
//! Prints the canonical packet JSON (default), the Markdown summary (`summary`),
//! or the compact lines (`compact`), so the published doc and the checked-in
//! artifacts stay byte-aligned with the in-crate builder. With `write [dir]` it
//! re-mints the checked-in fixtures under `fixtures/voice/redaction-and-export/`,
//! `support [path]` re-mints the support-export JSON artifact, and
//! `report [path]` re-mints the rendered Markdown report.

use std::path::PathBuf;

use aureline_support::voice_redaction::seed::{
    FIXTURES_DIR_REF, SUPPORT_EXPORT_REF, SUPPORT_REPORT_REF,
};
use aureline_support::voice_redaction::{
    seeded_voice_support_export_packet, write_fixtures, write_report, write_support_export,
};

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "export".to_owned());
    let packet = seeded_voice_support_export_packet();

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
                .unwrap_or_else(|| PathBuf::from(FIXTURES_DIR_REF));
            write_fixtures(&dir, &packet).expect("write fixtures");
            eprintln!("wrote voice support-export fixtures to {}", dir.display());
        }
        "support" => {
            let path = std::env::args()
                .nth(2)
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(SUPPORT_EXPORT_REF));
            write_support_export(&path, &packet).expect("write support export");
            eprintln!("wrote voice support export to {}", path.display());
        }
        "report" => {
            let path = std::env::args()
                .nth(2)
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(SUPPORT_REPORT_REF));
            write_report(&path, &packet).expect("write report");
            eprintln!("wrote voice support report to {}", path.display());
        }
        _ => println!("{}", packet.export_safe_json()),
    }
}
