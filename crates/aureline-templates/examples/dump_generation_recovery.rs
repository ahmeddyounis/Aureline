//! Conformance dump for the generation diff-review and recovery packet.
//!
//! Prints one of the canonical export, the two checked-in narrowed fixtures, or
//! the Markdown summary, so the artifact and fixtures can be regenerated
//! deterministically from the canonical builder.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_generation_recovery -- canonical
//! cargo run -p aureline-templates --example dump_generation_recovery -- lineage_unknown
//! cargo run -p aureline-templates --example dump_generation_recovery -- authored_protection
//! cargo run -p aureline-templates --example dump_generation_recovery -- markdown
//! ```

use aureline_templates::add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty::*;

const PACKET_ID: &str = "generation-recovery:stable:0001";
const PACKET_LABEL: &str =
    "Generation Diff Review, Rollback/Delete-Generated Recovery, and Managed-Zone Honesty";
const MINTED_AT: &str = "2026-06-07T00:00:00Z";
const CLEAN_ROW: &str = "generation-recovery-row:rust_cli.clean_generate:2026.05";
const MIXED_ROW: &str = "generation-recovery-row:node_service.mixed_zone:2026.05";

fn canonical() -> GenerationRecoveryPacket {
    canonical_generation_recovery(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        GenerationRecoveryProofFreshness {
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
        "lineage_unknown" => {
            // A clean generated row whose lineage can no longer be resolved is
            // blocked and labeled lineage-unknown rather than overwritten.
            packet.apply_downgrade_automation(&[GenerationRecoveryRowObservation {
                row_id: CLEAN_ROW.to_owned(),
                diff_preview_available: true,
                checkpoint_available: true,
                lineage_known: false,
                authored_protection_verified: true,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        "authored_protection" => {
            // A destructive recovery whose authored-content protection cannot be
            // verified is downgraded to a quarantine and withdrawn from generation.
            packet.apply_downgrade_automation(&[GenerationRecoveryRowObservation {
                row_id: MIXED_ROW.to_owned(),
                diff_preview_available: true,
                checkpoint_available: true,
                lineage_known: true,
                authored_protection_verified: false,
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
