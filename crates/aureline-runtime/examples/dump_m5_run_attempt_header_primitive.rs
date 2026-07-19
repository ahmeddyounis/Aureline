//! Conformance dump for the M5 run/attempt-header primitive packet.
//!
//! Prints the canonical support export (default), the CSV projection (`csv`
//! argument), or the Markdown report (`summary` argument) so the checked-in
//! artifacts stay byte-aligned with the in-crate seeded builder.
//!
//! Regenerate the checked artifacts with:
//!
//! ```sh
//! cargo run -p aureline-runtime --example dump_m5_run_attempt_header_primitive -- support \
//!   > artifacts/release/m5-run-attempt-header-primitive-proof/support_export.json
//! cargo run -p aureline-runtime --example dump_m5_run_attempt_header_primitive -- csv \
//!   > artifacts/release/m5-run-attempt-header-primitive-proof/matrix.csv
//! cargo run -p aureline-runtime --example dump_m5_run_attempt_header_primitive -- summary \
//!   > artifacts/release/m5-run-attempt-header-primitive-proof/report.md
//! ```

use aureline_runtime::implement_the_m5_run_attempt_header_and_attempt_selector_primitive::seeded_m5_run_attempt_header_packet;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = seeded_m5_run_attempt_header_packet();

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
