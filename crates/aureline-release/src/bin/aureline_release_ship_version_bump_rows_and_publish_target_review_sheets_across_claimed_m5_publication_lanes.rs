//! Headless emitter for the M5 version-bump-row / publish-target-review-sheet
//! primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-publish-target-review-sheet-proof/`, its matrix CSV, the
//! Markdown report `artifacts/components/m5-version-bump-and-publish-target-primitive.md`,
//! and the narrowed fixtures under
//! `fixtures/ui/m5-publish-target-review-sheet-primitive/`. Every claimed M5
//! publication consumer (the release-center publish sheet, the update-center publish
//! row, the CLI publish inspect, the admin publish report, and the support /
//! evaluation export) reads this primitive so prior/next version, delta kind,
//! public-surface impact, publish-target class, visibility, mutability, auth source,
//! dry-run availability, and rollout-ring truth stay consistent, and so a blocked
//! publication state is understood from a self-contained banner rather than a
//! secondary pipeline page.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_ship_version_bump_rows_publish -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_ship_version_bump_rows_publish -- report
//! cargo run -q -p aureline-release --bin aureline_release_ship_version_bump_rows_publish -- csv
//! cargo run -q -p aureline-release --bin aureline_release_ship_version_bump_rows_publish -- fixture-update-center-publish-row-beta-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_ship_version_bump_rows_publish -- fixture-cli-publish-inspect-preview-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_ship_version_bump_rows_publish -- validate
//! ```

use aureline_release::ship_version_bump_rows_and_publish_target_review_sheets_across_claimed_m5_publication_lanes::{
    seeded_m5_publication_review_primitive_cli_publish_inspect_preview_narrowed,
    seeded_m5_publication_review_primitive_packet,
    seeded_m5_publication_review_primitive_update_center_publish_row_beta_narrowed,
    M5PublicationReviewPrimitivePacket,
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
            let packet = seeded_m5_publication_review_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_publication_review_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_publication_review_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-update-center-publish-row-beta-narrowed") => {
            let packet =
                seeded_m5_publication_review_primitive_update_center_publish_row_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-cli-publish-inspect-preview-narrowed") => {
            let packet =
                seeded_m5_publication_review_primitive_cli_publish_inspect_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_publication_review_primitive_packet(),
                seeded_m5_publication_review_primitive_update_center_publish_row_beta_narrowed(),
                seeded_m5_publication_review_primitive_cli_publish_inspect_preview_narrowed(),
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
    packet: &M5PublicationReviewPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
