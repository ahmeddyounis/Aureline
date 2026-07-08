//! Headless emitter for the frozen M5 support-class, evidence-freshness,
//! lifecycle, channel, deployment-scope, compatibility-state, and
//! explanation-drawer badge matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-badge-family-proof/`, its matrix CSV, the Markdown
//! report `artifacts/components/m5-badge-family-components.md`, and the narrowed
//! fixtures under `fixtures/ui/m5-badge-family-consumers/`. Marketplace,
//! Help/Docs, Settings, onboarding, diagnostics, runtime, and exported-evidence
//! surfaces read this matrix so one support-class badge never implies freshness,
//! one deployment-scope badge never implies a lifecycle stage, and no badge
//! collapses two axes into a single overloaded pill.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix -- report
//! cargo run -q -p aureline-release --bin aureline_release_freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix -- csv
//! cargo run -q -p aureline-release --bin aureline_release_freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix -- fixture-channel-badge-beta-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix -- fixture-compatibility-state-badge-preview-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix -- validate
//! ```

use aureline_release::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix::{
    seeded_m5_badge_family_matrix, seeded_m5_badge_family_matrix_channel_badge_beta_narrowed,
    seeded_m5_badge_family_matrix_compatibility_state_badge_preview_narrowed,
    M5BadgeFamilyMatrixPacket,
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
            let packet = seeded_m5_badge_family_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_badge_family_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_badge_family_matrix().render_matrix_csv());
        }
        Some("fixture-channel-badge-beta-narrowed") => {
            let packet = seeded_m5_badge_family_matrix_channel_badge_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-compatibility-state-badge-preview-narrowed") => {
            let packet = seeded_m5_badge_family_matrix_compatibility_state_badge_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_badge_family_matrix(),
                seeded_m5_badge_family_matrix_channel_badge_beta_narrowed(),
                seeded_m5_badge_family_matrix_compatibility_state_badge_preview_narrowed(),
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

fn assert_valid(packet: &M5BadgeFamilyMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
