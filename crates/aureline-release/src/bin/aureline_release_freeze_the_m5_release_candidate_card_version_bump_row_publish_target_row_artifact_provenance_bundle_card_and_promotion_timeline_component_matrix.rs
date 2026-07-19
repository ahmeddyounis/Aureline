//! Headless emitter for the frozen M5 release-candidate-card, version-bump-row,
//! publish-target-row, artifact-provenance-bundle-card, and promotion-timeline
//! component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-release-center-component-proof/`, its matrix CSV,
//! the Markdown report `artifacts/components/m5-release-center-components.md`, and
//! the narrowed fixtures under `fixtures/ui/m5-release-center-components/`.
//! Release-center, update-center, registry, mirror, enterprise-evaluation,
//! support, docs, and admin surfaces read this matrix so one candidate card
//! carries scope and blocker freshness, one version-bump row states its impact,
//! one publish-target row names auth source and mutability, one provenance card
//! shows signature/attestation/SBOM truth over an immutable digest lineage, one
//! promotion timeline step names its rollout ring and stage, and one
//! rollback/revocation row states its blast radius before any promotion.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_freeze_m5_release_candidate_card -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_freeze_m5_release_candidate_card -- report
//! cargo run -q -p aureline-release --bin aureline_release_freeze_m5_release_candidate_card -- csv
//! cargo run -q -p aureline-release --bin aureline_release_freeze_m5_release_candidate_card -- fixture-promotion-timeline-step-beta-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_freeze_m5_release_candidate_card -- fixture-rollback-revocation-row-preview-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_freeze_m5_release_candidate_card -- validate
//! ```

use aureline_release::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix::{
    seeded_m5_release_center_component_matrix,
    seeded_m5_release_center_component_matrix_promotion_timeline_step_beta_narrowed,
    seeded_m5_release_center_component_matrix_rollback_revocation_row_preview_narrowed,
    M5ReleaseCenterMatrixPacket,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("support-export") | None => {
            let packet = seeded_m5_release_center_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_release_center_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_release_center_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-promotion-timeline-step-beta-narrowed") => {
            let packet =
                seeded_m5_release_center_component_matrix_promotion_timeline_step_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-rollback-revocation-row-preview-narrowed") => {
            let packet =
                seeded_m5_release_center_component_matrix_rollback_revocation_row_preview_narrowed(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_release_center_component_matrix(),
                seeded_m5_release_center_component_matrix_promotion_timeline_step_beta_narrowed(),
                seeded_m5_release_center_component_matrix_rollback_revocation_row_preview_narrowed(
                ),
            ] {
                assert_valid(&packet)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &M5ReleaseCenterMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
