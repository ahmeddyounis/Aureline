//! Headless emitter for the M5 compatibility-state badge primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-compatibility-state-badge-proof/`, its matrix CSV, the Markdown
//! report `artifacts/components/m5-compatibility-state-badges.md`, and the narrowed
//! fixtures under `fixtures/ui/m5-compatibility-state-badges/`. Every claimed M5 badge
//! consumer (the workspace-reopen card, the toolchain-install row, the extension-import
//! row, the workflow-bundle-apply card, the compare/review panel, and the support-export
//! row) reads this primitive so compatibility-state truth stays one distinct, composable
//! cue, and so a Limited or Mismatch badge always preserves its reconciliation, repair, and
//! compare detail before an install / import / apply / reopen flow proceeds.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_ship_compatibility_state_badges_mismatch -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_ship_compatibility_state_badges_mismatch -- report
//! cargo run -q -p aureline-release --bin aureline_release_ship_compatibility_state_badges_mismatch -- csv
//! cargo run -q -p aureline-release --bin aureline_release_ship_compatibility_state_badges_mismatch -- fixture-compare-review-panel-beta-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_ship_compatibility_state_badges_mismatch -- fixture-support-export-row-preview-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_ship_compatibility_state_badges_mismatch -- validate
//! ```

use aureline_release::ship_compatibility_state_badges_and_mismatch_review_affordances_across_claimed_m5_workspace_toolchain_extension_bundle_and_artifact_flows::{
    seeded_m5_compatibility_state_badge_primitive_compare_review_panel_beta_narrowed,
    seeded_m5_compatibility_state_badge_primitive_packet,
    seeded_m5_compatibility_state_badge_primitive_support_export_row_preview_narrowed,
    M5CompatibilityStateBadgePrimitivePacket,
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
            let packet = seeded_m5_compatibility_state_badge_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_compatibility_state_badge_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_compatibility_state_badge_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-compare-review-panel-beta-narrowed") => {
            let packet =
                seeded_m5_compatibility_state_badge_primitive_compare_review_panel_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-support-export-row-preview-narrowed") => {
            let packet =
                seeded_m5_compatibility_state_badge_primitive_support_export_row_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_compatibility_state_badge_primitive_packet(),
                seeded_m5_compatibility_state_badge_primitive_compare_review_panel_beta_narrowed(),
                seeded_m5_compatibility_state_badge_primitive_support_export_row_preview_narrowed(),
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

fn assert_valid(
    packet: &M5CompatibilityStateBadgePrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
