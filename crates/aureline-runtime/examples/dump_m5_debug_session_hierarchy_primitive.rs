//! Conformance dump for the M5 debug-session-hierarchy primitive packet.
//!
//! Prints the canonical support export (default), the CSV projection (`csv`
//! argument), or the Markdown report (`summary` argument) so the checked-in
//! artifacts stay byte-aligned with the in-crate seeded builder.
//!
//! Regenerate the checked artifacts with:
//!
//! ```sh
//! cargo run -p aureline-runtime --example dump_m5_debug_session_hierarchy_primitive -- support \
//!   > artifacts/release/m5-debug-session-hierarchy-primitive-proof/support_export.json
//! cargo run -p aureline-runtime --example dump_m5_debug_session_hierarchy_primitive -- csv \
//!   > artifacts/release/m5-debug-session-hierarchy-primitive-proof/matrix.csv
//! cargo run -p aureline-runtime --example dump_m5_debug_session_hierarchy_primitive -- summary \
//!   > artifacts/release/m5-debug-session-hierarchy-primitive-proof/report.md
//! ```

use aureline_runtime::implement_the_m5_debug_session_header_thread_process_tree_and_dump_crash_artifact_card_primitive::seeded_m5_debug_hierarchy_packet;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = seeded_m5_debug_hierarchy_packet();

    let violations = packet.validate();
    assert!(violations.is_empty(), "packet must validate: {violations:?}");

    match which.as_str() {
        "support" => print!("{}", packet.export_safe_json()),
        "csv" => print!("{}", packet.render_matrix_csv()),
        "summary" => print!("{}", packet.render_markdown_summary()),
        other => {
            eprintln!("unknown mode: {other} (expected support|csv|summary)");
            std::process::exit(2);
        }
    }
}
