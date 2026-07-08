//! Headless emitter for the M5 deployment-scope badge primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-deployment-scope-badge-proof/`, its matrix CSV, the Markdown
//! report `artifacts/components/m5-deployment-scope-badges.md`, and the narrowed fixtures
//! under `fixtures/ui/m5-deployment-scope-badges/`. Every claimed M5 badge consumer (the
//! runtime capability row, the install/deployment card, the Help/About panel, the
//! diagnostics report, the support-export row, and the companion-mode card) reads this
//! primitive so deployment-scope truth stays one distinct, composable cue, and so a
//! local/offline/self-host/mirror/companion badge always discloses its residual
//! dependency and local-safe continuity while preserving the scope it was running in.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_implement_deployment_scope_badges_with_local_only_managed_self_hosted_mirrored_offline_capable_and_browser_companion_truth_across_claimed_m5_runtime_install_help_and_export_surfaces -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_implement_deployment_scope_badges_with_local_only_managed_self_hosted_mirrored_offline_capable_and_browser_companion_truth_across_claimed_m5_runtime_install_help_and_export_surfaces -- report
//! cargo run -q -p aureline-release --bin aureline_release_implement_deployment_scope_badges_with_local_only_managed_self_hosted_mirrored_offline_capable_and_browser_companion_truth_across_claimed_m5_runtime_install_help_and_export_surfaces -- csv
//! cargo run -q -p aureline-release --bin aureline_release_implement_deployment_scope_badges_with_local_only_managed_self_hosted_mirrored_offline_capable_and_browser_companion_truth_across_claimed_m5_runtime_install_help_and_export_surfaces -- fixture-companion-mode-card-beta-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_implement_deployment_scope_badges_with_local_only_managed_self_hosted_mirrored_offline_capable_and_browser_companion_truth_across_claimed_m5_runtime_install_help_and_export_surfaces -- fixture-diagnostics-report-preview-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_implement_deployment_scope_badges_with_local_only_managed_self_hosted_mirrored_offline_capable_and_browser_companion_truth_across_claimed_m5_runtime_install_help_and_export_surfaces -- validate
//! ```

use aureline_release::implement_deployment_scope_badges_with_local_only_managed_self_hosted_mirrored_offline_capable_and_browser_companion_truth_across_claimed_m5_runtime_install_help_and_export_surfaces::{
    seeded_m5_deployment_scope_badge_primitive_companion_mode_card_beta_narrowed,
    seeded_m5_deployment_scope_badge_primitive_diagnostics_report_preview_narrowed,
    seeded_m5_deployment_scope_badge_primitive_packet, M5DeploymentScopeBadgePrimitivePacket,
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
            let packet = seeded_m5_deployment_scope_badge_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_deployment_scope_badge_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_deployment_scope_badge_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-companion-mode-card-beta-narrowed") => {
            let packet =
                seeded_m5_deployment_scope_badge_primitive_companion_mode_card_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-diagnostics-report-preview-narrowed") => {
            let packet =
                seeded_m5_deployment_scope_badge_primitive_diagnostics_report_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_deployment_scope_badge_primitive_packet(),
                seeded_m5_deployment_scope_badge_primitive_companion_mode_card_beta_narrowed(),
                seeded_m5_deployment_scope_badge_primitive_diagnostics_report_preview_narrowed(),
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
    packet: &M5DeploymentScopeBadgePrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
