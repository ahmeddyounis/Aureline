//! Conformance dump for the scaffold-planner packet.
//!
//! Prints the canonical export, the two checked-in narrowed fixtures, or the
//! Markdown summary, so the artifact and fixtures can be regenerated
//! deterministically from the canonical builder.
//!
//! ```text
//! cargo run -p aureline-scaffold --example dump_scaffold_planner -- canonical
//! cargo run -p aureline-scaffold --example dump_scaffold_planner -- parameter_unresolved
//! cargo run -p aureline-scaffold --example dump_scaffold_planner -- create_empty_parity_broken
//! cargo run -p aureline-scaffold --example dump_scaffold_planner -- markdown
//! ```

use aureline_scaffold::ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity::*;

const PACKET_ID: &str = "scaffold-planner:stable:0001";
const PLANNER_LABEL: &str =
    "Scaffold Planner, Parameter Review, Environment Preflights, and Create-Empty Parity";
const MINTED_AT: &str = "2026-06-07T00:00:00Z";
const READY_ROW: &str = "scaffold-plan:rust.cli.ready:2026.04";
const CREATE_EMPTY_ROW: &str = "scaffold-plan:create_empty.workspace:2026.05";

fn canonical() -> ScaffoldPlannerPacket {
    canonical_scaffold_planner(
        PACKET_ID.to_owned(),
        PLANNER_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        ScaffoldPlannerProofFreshness {
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
        "parameter_unresolved" => {
            // A ready templated plan whose required parameters became unresolved
            // is blocked and labeled, not hidden.
            packet.apply_downgrade_automation(&[ScaffoldPlanRowObservation {
                plan_id: READY_ROW.to_owned(),
                parameters_resolved: false,
                environment_ready: true,
                write_impact_preview_available: true,
                rollback_boundary_available: true,
                create_empty_parity_intact: true,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        "create_empty_parity_broken" => {
            // A create-empty plan whose parity with the templated flow broke is
            // blocked rather than silently downgraded to an unreviewed shortcut.
            packet.apply_downgrade_automation(&[ScaffoldPlanRowObservation {
                plan_id: CREATE_EMPTY_ROW.to_owned(),
                parameters_resolved: true,
                environment_ready: true,
                write_impact_preview_available: true,
                rollback_boundary_available: true,
                create_empty_parity_intact: false,
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
