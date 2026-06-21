//! Conformance dump for the M5 structured-input / staged-review set packet.
//!
//! Prints the canonical export-safe support export (default) or the Markdown
//! report (`report` argument) so the checked-in artifacts stay byte-aligned with
//! the in-crate builder.

use aureline_ui::m5_structured_input_and_staged_review::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = seeded_m5_structured_input_set();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    match which.as_str() {
        "report" => print!("{}", packet.render_markdown_report()),
        _ => println!("{}", packet.export_safe_json()),
    }
}
