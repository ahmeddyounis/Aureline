//! Headless emitter for the M5 orr-review and rehearsal-drill registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-orr-review-and-rehearsal-drill-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-orr-review-and-rehearsal-drill-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- report
//! cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- orr-review-table
//! cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- fixture-orr-review-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- fixture-rehearsal-drill-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- validate
//! ```

use aureline_ui::m5_orr_review_and_rehearsal_drill_registries::{
    seeded_m5_orr_review_and_rehearsal_drill_registries,
    seeded_m5_orr_review_and_rehearsal_drill_registries_orr_review_beta_narrowed,
    seeded_m5_orr_review_and_rehearsal_drill_registries_rehearsal_drill_preview_narrowed,
    M5OrrReviewRehearsalDrillRegistriesPacket,
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
            let packet = seeded_m5_orr_review_and_rehearsal_drill_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_orr_review_and_rehearsal_drill_registries().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_orr_review_and_rehearsal_drill_registries().render_matrix_csv()
            );
        }
        Some("orr-review-table") => {
            print!(
                "{}",
                seeded_m5_orr_review_and_rehearsal_drill_registries().render_orr_review_table()
            );
        }
        Some("fixture-orr-review-beta-narrowed") => {
            let packet =
                seeded_m5_orr_review_and_rehearsal_drill_registries_orr_review_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-rehearsal-drill-preview-narrowed") => {
            let packet =
                seeded_m5_orr_review_and_rehearsal_drill_registries_rehearsal_drill_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_orr_review_and_rehearsal_drill_registries(),
                seeded_m5_orr_review_and_rehearsal_drill_registries_orr_review_beta_narrowed(),
                seeded_m5_orr_review_and_rehearsal_drill_registries_rehearsal_drill_preview_narrowed(),
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
    packet: &M5OrrReviewRehearsalDrillRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
