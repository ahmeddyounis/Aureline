//! Conformance dump for the framework generator-run packet.
//!
//! Prints one of the canonical export, the two checked-in narrowed fixtures, or
//! the Markdown summary, so the artifact and fixtures can be regenerated
//! deterministically from the canonical builder.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_framework_generators -- canonical
//! cargo run -p aureline-templates --example dump_framework_generators -- rollback_unavailable
//! cargo run -p aureline-templates --example dump_framework_generators -- context_reuse_unavailable
//! cargo run -p aureline-templates --example dump_framework_generators -- markdown
//! ```

use aureline_templates::implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse::*;

const PACKET_ID: &str = "generator-run:stable:0001";
const PACKET_LABEL: &str =
    "Framework Generators and Codemods with Preview, Diff, Rollback, and Execution-Context Reuse";
const MINTED_AT: &str = "2026-06-08T00:00:00Z";
const SCAFFOLD_EXACT: &str = "generator-run-row:scaffold.resource.exact.reused:2026.06";

fn canonical() -> GeneratorRunPacket {
    canonical_generator_runs(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        GeneratorRunProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: MINTED_AT.to_owned(),
            auto_narrow_on_stale: true,
        },
    )
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "canonical".to_owned());
    let mut packet = canonical();
    match which.as_str() {
        "canonical" => {}
        "markdown" => {
            print!("{}", packet.render_markdown_summary());
            return;
        }
        "rollback_unavailable" => {
            // A scaffold run whose rollback handle can no longer be captured is
            // blocked rather than applied without a way back.
            packet.apply_downgrade_automation(&[GeneratorRunRowObservation {
                row_id: SCAFFOLD_EXACT.to_owned(),
                preview_available: true,
                diff_available: true,
                rollback_available: false,
                context_reused: true,
                run_fresh: true,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        "context_reuse_unavailable" => {
            // A scaffold run whose warm execution context could not be reused falls
            // back to a fresh context, labeled rather than hidden, and stays offered.
            packet.apply_downgrade_automation(&[GeneratorRunRowObservation {
                row_id: SCAFFOLD_EXACT.to_owned(),
                preview_available: true,
                diff_available: true,
                rollback_available: true,
                context_reused: false,
                run_fresh: true,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
    println!("{}", packet.export_safe_json());
}
