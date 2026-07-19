//! Conformance dump for the M5 execution-lifecycle accessibility fallback packet.
//!
//! Prints the canonical support export (default), the CSV projection (`csv`
//! argument), or the Markdown report (`summary` argument) so the checked-in
//! artifacts stay byte-aligned with the in-crate seeded builder.
//!
//! Regenerate the checked artifacts with:
//!
//! ```sh
//! cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_accessibility_fallback -- support \
//!   > artifacts/release/m5-execution-lifecycle-accessibility-fallback-proof/support_export.json
//! cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_accessibility_fallback -- csv \
//!   > artifacts/release/m5-execution-lifecycle-accessibility-fallback-proof/matrix.csv
//! cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_accessibility_fallback -- summary \
//!   > artifacts/release/m5-execution-lifecycle-accessibility-fallback-proof/report.md
//! ```

use aureline_runtime::implement_keyboard_screen_reader_cli_export_parity_and_execution_lifecycle_auto_narrowing::seeded_m5_execution_a11y_fallback_packet;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = seeded_m5_execution_a11y_fallback_packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

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
