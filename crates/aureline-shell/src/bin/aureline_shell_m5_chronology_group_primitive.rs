//! Headless emitter for the M5 chronology-group primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-chronology-groups-proof/`, its matrix CSV, the
//! Markdown report `artifacts/components/m5-chronology-groups-primitive.md`, and the
//! narrowed fixtures under `fixtures/ui/m5-chronology-groups-primitive/`. Every M5
//! history lane that leaves the live surface (AI, policy, task, remote, update, and
//! support) reads this primitive so grouped phases, one-sentence narrative cards,
//! timezone-safe export previews, and no-lost-causality ordering stay consistent,
//! and so the support export reconstructs the grouped chronology from one shared
//! model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_chronology_group_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_chronology_group_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_chronology_group_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_chronology_group_primitive -- fixture-update-history-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_chronology_group_primitive -- fixture-support-exports-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_chronology_group_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_chronology_group_narrative_card_and_export_preview_primitive::{
    seeded_m5_chronology_group_primitive_packet,
    seeded_m5_chronology_group_primitive_support_exports_preview_narrowed,
    seeded_m5_chronology_group_primitive_update_history_beta_narrowed,
    M5ChronologyGroupPrimitivePacket,
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
            let packet = seeded_m5_chronology_group_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_chronology_group_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_chronology_group_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-update-history-beta-narrowed") => {
            let packet = seeded_m5_chronology_group_primitive_update_history_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-support-exports-preview-narrowed") => {
            let packet = seeded_m5_chronology_group_primitive_support_exports_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_chronology_group_primitive_packet(),
                seeded_m5_chronology_group_primitive_update_history_beta_narrowed(),
                seeded_m5_chronology_group_primitive_support_exports_preview_narrowed(),
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
    packet: &M5ChronologyGroupPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
