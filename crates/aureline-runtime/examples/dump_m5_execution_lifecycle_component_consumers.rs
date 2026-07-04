//! Conformance dump for the M5 execution-lifecycle component consumer packet.
//!
//! Prints the canonical support export (default), the CSV projection (`csv`
//! argument), or the Markdown report (`summary` argument) so the checked-in
//! artifacts stay byte-aligned with the in-crate seeded builder.
//!
//! Regenerate the checked artifacts with:
//!
//! ```sh
//! cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_component_consumers -- support \
//!   > artifacts/release/m5-execution-lifecycle-component-consumer-proof/support_export.json
//! cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_component_consumers -- csv \
//!   > artifacts/release/m5-execution-lifecycle-component-consumer-proof/matrix.csv
//! cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_component_consumers -- summary \
//!   > artifacts/release/m5-execution-lifecycle-component-consumer-proof/report.md
//! ```

use aureline_runtime::add_shared_task_test_request_database_notebook_preview_ai_publish_and_support_execution_lifecycle_component_consumers::seeded_m5_execution_lifecycle_component_consumers_packet;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = seeded_m5_execution_lifecycle_component_consumers_packet();

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
