//! Headless emitter for the M5 lifecycle / channel badge primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-lifecycle-and-channel-badge-proof/`, its matrix CSV, the
//! Markdown report `artifacts/components/m5-lifecycle-and-channel-badges.md`, and the
//! narrowed fixtures under `fixtures/ui/m5-lifecycle-and-channel-badges/`. Every claimed
//! M5 badge consumer (the command row, the feature surface, the workflow bundle, the
//! extension/install row, the release/install surface, and the ecosystem lifecycle
//! review lane) reads this primitive so lifecycle and channel truth stay two distinct,
//! composable cues, and so a deprecated or removal-scheduled badge always points to a
//! replacement/migration path while preserving the channel it was running on.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_implement_lifecycle_channel_badges_claimed -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_implement_lifecycle_channel_badges_claimed -- report
//! cargo run -q -p aureline-release --bin aureline_release_implement_lifecycle_channel_badges_claimed -- csv
//! cargo run -q -p aureline-release --bin aureline_release_implement_lifecycle_channel_badges_claimed -- fixture-extension-install-row-beta-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_implement_lifecycle_channel_badges_claimed -- fixture-ecosystem-review-preview-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_implement_lifecycle_channel_badges_claimed -- validate
//! ```

use aureline_release::implement_lifecycle_and_channel_badges_across_claimed_m5_command_feature_bundle_extension_and_install_surfaces::{
    seeded_m5_maturity_badge_primitive_ecosystem_review_preview_narrowed,
    seeded_m5_maturity_badge_primitive_extension_install_row_beta_narrowed,
    seeded_m5_maturity_badge_primitive_packet, M5MaturityBadgePrimitivePacket,
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
            let packet = seeded_m5_maturity_badge_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_maturity_badge_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_maturity_badge_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-extension-install-row-beta-narrowed") => {
            let packet = seeded_m5_maturity_badge_primitive_extension_install_row_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-ecosystem-review-preview-narrowed") => {
            let packet = seeded_m5_maturity_badge_primitive_ecosystem_review_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_maturity_badge_primitive_packet(),
                seeded_m5_maturity_badge_primitive_extension_install_row_beta_narrowed(),
                seeded_m5_maturity_badge_primitive_ecosystem_review_preview_narrowed(),
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

fn assert_valid(packet: &M5MaturityBadgePrimitivePacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
