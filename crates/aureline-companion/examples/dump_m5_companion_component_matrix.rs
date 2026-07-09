//! Headless emitter for the frozen M5 companion component matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-companion-component-proof/`, its matrix CSV, the Markdown design
//! report, and the narrowed fixtures under `fixtures/ui/m5-companion-components/`. Browser,
//! mobile, desktop-panel, diagnostics, support, and Help/About surfaces read this matrix so
//! one notification row names which object a tap opens and its severity, one mobile review
//! card names whether it is review-only or comment-capable, one CI-status card names its
//! status and freshness, one session-follow tile names its scope and whether it is live or
//! stale, one incident-snapshot card names its severity and freshness, and one desktop-handoff
//! sheet names the exact target it will open on desktop.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_m5_companion_component_matrix -- support-export
//! cargo run -p aureline-companion --example dump_m5_companion_component_matrix -- report
//! cargo run -p aureline-companion --example dump_m5_companion_component_matrix -- csv
//! cargo run -p aureline-companion --example dump_m5_companion_component_matrix -- fixture-session-follow-tile-beta-narrowed
//! cargo run -p aureline-companion --example dump_m5_companion_component_matrix -- fixture-desktop-handoff-sheet-preview-narrowed
//! cargo run -p aureline-companion --example dump_m5_companion_component_matrix -- validate
//! ```

use aureline_companion::freeze_the_m5_companion_component_matrix::{
    seeded_m5_companion_component_matrix,
    seeded_m5_companion_component_matrix_desktop_handoff_sheet_preview_narrowed,
    seeded_m5_companion_component_matrix_session_follow_tile_beta_narrowed,
    M5CompanionComponentMatrixPacket,
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
            let packet = seeded_m5_companion_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_companion_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_companion_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-session-follow-tile-beta-narrowed") => {
            let packet = seeded_m5_companion_component_matrix_session_follow_tile_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-desktop-handoff-sheet-preview-narrowed") => {
            let packet =
                seeded_m5_companion_component_matrix_desktop_handoff_sheet_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_companion_component_matrix(),
                seeded_m5_companion_component_matrix_session_follow_tile_beta_narrowed(),
                seeded_m5_companion_component_matrix_desktop_handoff_sheet_preview_narrowed(),
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
    packet: &M5CompanionComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
