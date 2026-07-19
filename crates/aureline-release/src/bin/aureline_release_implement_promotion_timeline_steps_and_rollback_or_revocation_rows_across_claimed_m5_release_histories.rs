//! Headless emitter for the M5 promotion-timeline-step / rollback-or-revocation-row
//! primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-promotion-timeline-and-rollback-revocation-proof/`, its matrix
//! CSV, the Markdown report
//! `artifacts/components/m5-promotion-timeline-step-and-rollback-revocation-row-primitive.md`,
//! and the narrowed fixtures under
//! `fixtures/ui/m5-promotion-timeline-and-rollback-revocation-primitive/`. Every claimed
//! M5 release-history consumer (the release-center timeline, the update-center history,
//! the CLI history inspect, the admin history report, and the support history export)
//! reads this primitive so promotion steps and rollback/revocation rows stay
//! reconstructable, bounded, and attributable, and so a blocked event is understood from
//! a self-contained banner rather than CI-only metadata.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_implement_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_implement_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories -- report
//! cargo run -q -p aureline-release --bin aureline_release_implement_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories -- csv
//! cargo run -q -p aureline-release --bin aureline_release_implement_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories -- fixture-update-center-history-beta-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_implement_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories -- fixture-cli-history-inspect-preview-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_implement_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories -- validate
//! ```

use aureline_release::implement_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories::{
    seeded_m5_release_history_primitive_cli_history_inspect_preview_narrowed,
    seeded_m5_release_history_primitive_packet,
    seeded_m5_release_history_primitive_update_center_history_beta_narrowed,
    M5ReleaseHistoryPrimitivePacket,
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
            let packet = seeded_m5_release_history_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_release_history_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_release_history_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-update-center-history-beta-narrowed") => {
            let packet = seeded_m5_release_history_primitive_update_center_history_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-cli-history-inspect-preview-narrowed") => {
            let packet = seeded_m5_release_history_primitive_cli_history_inspect_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_release_history_primitive_packet(),
                seeded_m5_release_history_primitive_update_center_history_beta_narrowed(),
                seeded_m5_release_history_primitive_cli_history_inspect_preview_narrowed(),
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
    packet: &M5ReleaseHistoryPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
