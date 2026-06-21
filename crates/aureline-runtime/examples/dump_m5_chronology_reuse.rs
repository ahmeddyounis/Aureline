//! Conformance dump for the M5 task/problem/output chronology-reuse set packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifacts stay byte-aligned with the
//! in-crate builder.

use aureline_runtime::m5_task_problem_output_chronology_reuse::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = seeded_chronology_reuse_set();

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
