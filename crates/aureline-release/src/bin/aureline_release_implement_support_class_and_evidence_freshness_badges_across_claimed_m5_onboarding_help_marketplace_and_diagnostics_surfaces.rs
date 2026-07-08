//! Headless emitter for the M5 support-class / evidence-freshness badge primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-support-class-and-evidence-freshness-badge-proof/`, its matrix
//! CSV, the Markdown report
//! `artifacts/components/m5-support-class-and-evidence-freshness-badges.md`, and the
//! narrowed fixtures under
//! `fixtures/ui/m5-support-class-and-evidence-freshness-badges/`. Every claimed M5
//! badge consumer (the onboarding checklist, the Help capability card, the marketplace
//! listing, the diagnostics report, the certification record, and the evaluation pack)
//! reads this primitive so support-class and evidence-freshness truth stay two distinct,
//! composable cues, and so imported or stale evidence auto-narrows a claim while
//! preserving its underlying support-class context.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_implement_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_implement_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces -- report
//! cargo run -q -p aureline-release --bin aureline_release_implement_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces -- csv
//! cargo run -q -p aureline-release --bin aureline_release_implement_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces -- fixture-marketplace-listing-beta-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_implement_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces -- fixture-certification-record-preview-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_implement_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces -- validate
//! ```

use aureline_release::implement_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces::{
    seeded_m5_badge_claim_primitive_certification_record_preview_narrowed,
    seeded_m5_badge_claim_primitive_marketplace_listing_beta_narrowed,
    seeded_m5_badge_claim_primitive_packet, M5BadgeClaimPrimitivePacket,
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
            let packet = seeded_m5_badge_claim_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_badge_claim_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_badge_claim_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-marketplace-listing-beta-narrowed") => {
            let packet = seeded_m5_badge_claim_primitive_marketplace_listing_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-certification-record-preview-narrowed") => {
            let packet = seeded_m5_badge_claim_primitive_certification_record_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_badge_claim_primitive_packet(),
                seeded_m5_badge_claim_primitive_marketplace_listing_beta_narrowed(),
                seeded_m5_badge_claim_primitive_certification_record_preview_narrowed(),
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

fn assert_valid(packet: &M5BadgeClaimPrimitivePacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
