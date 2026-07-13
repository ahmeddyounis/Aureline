//! Headless emitter for the M5 banner / empty-state controls packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-banner-inline-notice-and-empty-state-controls-proof/`, its matrix CSV, the
//! Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-banner-inline-notice-and-empty-state-controls/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_banner_empty_state_controls -- support-export
//! cargo run -p aureline-ui --example dump_m5_banner_empty_state_controls -- report
//! cargo run -p aureline-ui --example dump_m5_banner_empty_state_controls -- csv
//! cargo run -p aureline-ui --example dump_m5_banner_empty_state_controls -- fixture-review-ui-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_banner_empty_state_controls -- fixture-updates-ui-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_banner_empty_state_controls -- validate
//! ```

use aureline_ui::m5_banner_inline_notice_and_empty_state_scoped_cause_and_next_action::{
    seeded_m5_banner_empty_state_controls,
    seeded_m5_banner_empty_state_controls_review_ui_beta_narrowed,
    seeded_m5_banner_empty_state_controls_updates_ui_preview_narrowed,
    M5BannerEmptyStateControlsPacket,
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
            let packet = seeded_m5_banner_empty_state_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_banner_empty_state_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_banner_empty_state_controls().render_matrix_csv()
            );
        }
        Some("fixture-review-ui-beta-narrowed") => {
            let packet = seeded_m5_banner_empty_state_controls_review_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-updates-ui-preview-narrowed") => {
            let packet = seeded_m5_banner_empty_state_controls_updates_ui_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_banner_empty_state_controls(),
                seeded_m5_banner_empty_state_controls_review_ui_beta_narrowed(),
                seeded_m5_banner_empty_state_controls_updates_ui_preview_narrowed(),
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
    packet: &M5BannerEmptyStateControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
