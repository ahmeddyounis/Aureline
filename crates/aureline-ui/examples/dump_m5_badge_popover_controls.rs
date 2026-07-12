//! Headless emitter for the M5 badge / popover controls packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-badge-chip-pill-and-popover-controls-proof/`, its matrix CSV, the Markdown
//! summary, and the narrowed fixtures under `fixtures/ui/m5-badge-chip-pill-and-popover-controls/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_badge_popover_controls -- support-export
//! cargo run -p aureline-ui --example dump_m5_badge_popover_controls -- report
//! cargo run -p aureline-ui --example dump_m5_badge_popover_controls -- csv
//! cargo run -p aureline-ui --example dump_m5_badge_popover_controls -- fixture-help-ui-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_badge_popover_controls -- fixture-review-ui-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_badge_popover_controls -- validate
//! ```

use aureline_ui::m5_badge_chip_pill_and_popover_expansion_and_anchored_focus_return::{
    seeded_m5_badge_popover_controls, seeded_m5_badge_popover_controls_help_ui_beta_narrowed,
    seeded_m5_badge_popover_controls_review_ui_preview_narrowed, M5BadgePopoverControlsPacket,
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
            let packet = seeded_m5_badge_popover_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_badge_popover_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_badge_popover_controls().render_matrix_csv());
        }
        Some("fixture-help-ui-beta-narrowed") => {
            let packet = seeded_m5_badge_popover_controls_help_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-review-ui-preview-narrowed") => {
            let packet = seeded_m5_badge_popover_controls_review_ui_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_badge_popover_controls(),
                seeded_m5_badge_popover_controls_help_ui_beta_narrowed(),
                seeded_m5_badge_popover_controls_review_ui_preview_narrowed(),
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

fn assert_valid(packet: &M5BadgePopoverControlsPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
