//! Conformance dump for the M5 execution-lifecycle surface certification packet.
//!
//! Prints the canonical support export (default), the CSV projection (`csv`
//! argument), or the Markdown report (`summary` argument) so the checked-in
//! artifacts stay byte-aligned with the in-crate seeded builder.
//!
//! Regenerate the checked artifacts with:
//!
//! ```sh
//! cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_surface_certification -- support \
//!   > artifacts/release/m5-execution-lifecycle-surface-certification-proof/support_export.json
//! cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_surface_certification -- csv \
//!   > artifacts/release/m5-execution-lifecycle-surface-certification-proof/matrix.csv
//! cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_surface_certification -- summary \
//!   > artifacts/release/m5-execution-lifecycle-surface-certification-proof/report.md
//! ```

use aureline_runtime::certify_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_truth_across_claimed_execution_surfaces::seeded_m5_execution_surface_cert_packet;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = seeded_m5_execution_surface_cert_packet();

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
