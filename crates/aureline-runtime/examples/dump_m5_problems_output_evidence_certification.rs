//! Conformance dump for the M5 Problems / output / execution-evidence qualification
//! capstone packet.
//!
//! Prints the canonical support export (default), the Markdown qualification report
//! (`report` argument), or the waiver-and-downgrade log (`waiver` argument) so the
//! checked-in artifacts stay byte-aligned with the in-crate builder. The `corpus`
//! argument prints the perturbation corpus index plus every case as one JSON object
//! so the checked-in fixtures stay byte-aligned with the in-crate builder.

use aureline_runtime::certify_m5_problems_output_and_execution_evidence_truth::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = seeded_m5_problems_output_evidence_certification_packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    match which.as_str() {
        "report" => print!("{}", packet.render_markdown_report()),
        "waiver" => print!("{}", packet.render_waiver_and_downgrade_log()),
        "corpus" => {
            let cases = seeded_m5_problems_output_evidence_certification_corpus();
            for case in &cases {
                case.check()
                    .expect("corpus case re-derives to its expected outcome");
            }
            let bundle = serde_json::json!({
                "index": seeded_m5_problems_output_evidence_certification_corpus_index(),
                "cases": cases,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&bundle).expect("corpus serializes")
            );
        }
        _ => println!("{}", packet.export_safe_json()),
    }
}
