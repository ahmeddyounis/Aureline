//! Conformance dump for the M5 structured-versus-heuristic fallback drill set packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifacts stay byte-aligned with the
//! in-crate builder.

use aureline_runtime::m5_structured_versus_heuristic_fallback_drills::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = seeded_fallback_evidence_drill_set();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
