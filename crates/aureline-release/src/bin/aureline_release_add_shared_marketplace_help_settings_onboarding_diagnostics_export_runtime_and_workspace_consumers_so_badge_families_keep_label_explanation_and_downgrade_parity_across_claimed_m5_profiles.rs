//! Headless emitter for the M5 badge-family-consumer parity lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-badge-family-consumer-proof/`, its matrix CSV, the
//! Markdown report `artifacts/components/m5-badge-family-consumer.md`, and the
//! narrowed fixtures under `fixtures/ui/m5-badge-family-consumers/`. Every claimed
//! M5 badge consumer (the marketplace/install surface, Help/About,
//! settings/policy, onboarding/start-center, diagnostics, the support export,
//! runtime/deployment cards, and workspace/archetype qualification) adopts the
//! same canonical badge families so the label, explanation, and downgrade reason
//! stay aligned, and so a narrowed badge is understood from a self-contained narrow
//! banner rather than a generic note.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_add_shared_marketplace_help_settings -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_add_shared_marketplace_help_settings -- report
//! cargo run -q -p aureline-release --bin aureline_release_add_shared_marketplace_help_settings -- csv
//! cargo run -q -p aureline-release --bin aureline_release_add_shared_marketplace_help_settings -- fixture-diagnostics-freshness-beta-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_add_shared_marketplace_help_settings -- fixture-support-export-scope-preview-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_add_shared_marketplace_help_settings -- validate
//! ```

use aureline_release::add_shared_marketplace_help_settings_onboarding_diagnostics_export_runtime_and_workspace_consumers_so_badge_families_keep_label_explanation_and_downgrade_parity_across_claimed_m5_profiles::{
    seeded_m5_badge_family_consumer_diagnostics_freshness_beta_narrowed,
    seeded_m5_badge_family_consumer_packet,
    seeded_m5_badge_family_consumer_support_export_scope_preview_narrowed,
    M5BadgeFamilyConsumerPacket,
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
            let packet = seeded_m5_badge_family_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_badge_family_consumer_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_badge_family_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-diagnostics-freshness-beta-narrowed") => {
            let packet = seeded_m5_badge_family_consumer_diagnostics_freshness_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-support-export-scope-preview-narrowed") => {
            let packet = seeded_m5_badge_family_consumer_support_export_scope_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_badge_family_consumer_packet(),
                seeded_m5_badge_family_consumer_diagnostics_freshness_beta_narrowed(),
                seeded_m5_badge_family_consumer_support_export_scope_preview_narrowed(),
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

fn assert_valid(packet: &M5BadgeFamilyConsumerPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
