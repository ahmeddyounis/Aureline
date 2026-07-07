//! Headless emitter for the M5 release-candidate-card / promotion-blocked-banner
//! primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-release-candidate-card-proof/`, its matrix CSV, the
//! Markdown report `artifacts/components/m5-release-candidate-card-primitive.md`,
//! and the narrowed fixtures under
//! `fixtures/ui/m5-release-candidate-card-primitive/`. Every claimed M5
//! release-candidate consumer (the release-center card, the update-center card,
//! the CLI release inspect, the admin release report, and the support / evaluation
//! export) reads this primitive so candidate identity, channel family, scoped
//! artifact set, blocker summary, evidence freshness, known issues, and
//! rollback-path truth stay consistent, and so a blocked promotion state is
//! understood from a self-contained banner rather than a secondary pipeline page.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_implement_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_implement_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces -- report
//! cargo run -q -p aureline-release --bin aureline_release_implement_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces -- csv
//! cargo run -q -p aureline-release --bin aureline_release_implement_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces -- fixture-update-center-card-beta-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_implement_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces -- fixture-cli-release-inspect-preview-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_implement_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces -- validate
//! ```

use aureline_release::implement_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces::{
    seeded_m5_release_candidate_primitive_cli_release_inspect_preview_narrowed,
    seeded_m5_release_candidate_primitive_packet,
    seeded_m5_release_candidate_primitive_update_center_card_beta_narrowed,
    M5ReleaseCandidatePrimitivePacket,
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
            let packet = seeded_m5_release_candidate_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_release_candidate_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_release_candidate_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-update-center-card-beta-narrowed") => {
            let packet = seeded_m5_release_candidate_primitive_update_center_card_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-cli-release-inspect-preview-narrowed") => {
            let packet =
                seeded_m5_release_candidate_primitive_cli_release_inspect_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_release_candidate_primitive_packet(),
                seeded_m5_release_candidate_primitive_update_center_card_beta_narrowed(),
                seeded_m5_release_candidate_primitive_cli_release_inspect_preview_narrowed(),
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
    packet: &M5ReleaseCandidatePrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
