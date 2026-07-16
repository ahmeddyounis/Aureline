//! Headless emitter for the M5 supported-line ORR-history-event and follow-up-closure registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-supported-line-orr-history-and-follow-up-closure-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-supported-line-orr-history-and-follow-up-closure-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- report
//! cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- orr-history-event-table
//! cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- fixture-orr-history-event-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- fixture-follow-up-closure-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- validate
//! ```

use aureline_ui::m5_supported_line_orr_history_and_follow_up_closure_registries::{
    seeded_m5_supported_line_orr_history_and_follow_up_closure_registries,
    seeded_m5_supported_line_orr_history_and_follow_up_closure_registries_follow_up_closure_preview_narrowed,
    seeded_m5_supported_line_orr_history_and_follow_up_closure_registries_orr_history_event_beta_narrowed,
    M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket,
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
            let packet = seeded_m5_supported_line_orr_history_and_follow_up_closure_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_supported_line_orr_history_and_follow_up_closure_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_supported_line_orr_history_and_follow_up_closure_registries()
                    .render_matrix_csv()
            );
        }
        Some("orr-history-event-table") => {
            print!(
                "{}",
                seeded_m5_supported_line_orr_history_and_follow_up_closure_registries()
                    .render_orr_history_event_table()
            );
        }
        Some("fixture-orr-history-event-beta-narrowed") => {
            let packet =
                seeded_m5_supported_line_orr_history_and_follow_up_closure_registries_orr_history_event_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-follow-up-closure-preview-narrowed") => {
            let packet =
                seeded_m5_supported_line_orr_history_and_follow_up_closure_registries_follow_up_closure_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_supported_line_orr_history_and_follow_up_closure_registries(),
                seeded_m5_supported_line_orr_history_and_follow_up_closure_registries_orr_history_event_beta_narrowed(),
                seeded_m5_supported_line_orr_history_and_follow_up_closure_registries_follow_up_closure_preview_narrowed(),
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
    packet: &M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
