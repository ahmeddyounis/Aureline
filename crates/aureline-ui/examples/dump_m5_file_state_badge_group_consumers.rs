//! Headless emitter for the M5 file-state badge-group consumer packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/support/m5-file-state-badge-group-consumers/`, its matrix CSV, the Markdown summary, and the
//! narrowed fixtures under `fixtures/editor/m5-file-state-badge-group-consumers/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_file_state_badge_group_consumers -- support-export
//! cargo run -p aureline-ui --example dump_m5_file_state_badge_group_consumers -- report
//! cargo run -p aureline-ui --example dump_m5_file_state_badge_group_consumers -- csv
//! cargo run -p aureline-ui --example dump_m5_file_state_badge_group_consumers -- fixture-compact-status-narrowed
//! cargo run -p aureline-ui --example dump_m5_file_state_badge_group_consumers -- fixture-palette-gated-narrowed
//! cargo run -p aureline-ui --example dump_m5_file_state_badge_group_consumers -- validate
//! ```

use aureline_ui::m5_file_state_badge_group_and_reason_strip_consumers::{
    seeded_m5_file_state_badge_group_consumers,
    seeded_m5_file_state_badge_group_consumers_compact_status_narrowed,
    seeded_m5_file_state_badge_group_consumers_palette_gated_narrowed,
    M5FileStateBadgeGroupConsumersPacket,
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
            let packet = seeded_m5_file_state_badge_group_consumers();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_file_state_badge_group_consumers().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_file_state_badge_group_consumers().render_matrix_csv()
            );
        }
        Some("fixture-compact-status-narrowed") => {
            let packet = seeded_m5_file_state_badge_group_consumers_compact_status_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-palette-gated-narrowed") => {
            let packet = seeded_m5_file_state_badge_group_consumers_palette_gated_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_file_state_badge_group_consumers(),
                seeded_m5_file_state_badge_group_consumers_compact_status_narrowed(),
                seeded_m5_file_state_badge_group_consumers_palette_gated_narrowed(),
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
    packet: &M5FileStateBadgeGroupConsumersPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "file-state badge-group consumer packet failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
