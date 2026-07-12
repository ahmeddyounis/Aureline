//! Headless emitter for the M5 editor-tab / gutter controls packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-editor-tab-gutter-controls-proof/`, its matrix CSV, the Markdown summary,
//! and the narrowed fixtures under `fixtures/ui/m5-editor-tab-gutter-controls/`.
//!
//! ```text
//! cargo run -p aureline-editor --example dump_m5_editor_tab_gutter_controls -- support-export
//! cargo run -p aureline-editor --example dump_m5_editor_tab_gutter_controls -- report
//! cargo run -p aureline-editor --example dump_m5_editor_tab_gutter_controls -- csv
//! cargo run -p aureline-editor --example dump_m5_editor_tab_gutter_controls -- fixture-editor-ui-beta-narrowed
//! cargo run -p aureline-editor --example dump_m5_editor_tab_gutter_controls -- fixture-diagnostics-ui-preview-narrowed
//! cargo run -p aureline-editor --example dump_m5_editor_tab_gutter_controls -- validate
//! ```

use aureline_editor::m5_editor_tab_and_gutter_state_and_marker_layering::{
    seeded_m5_editor_tab_gutter_controls,
    seeded_m5_editor_tab_gutter_controls_diagnostics_ui_preview_narrowed,
    seeded_m5_editor_tab_gutter_controls_editor_ui_beta_narrowed, M5EditorTabGutterControlsPacket,
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
            let packet = seeded_m5_editor_tab_gutter_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_editor_tab_gutter_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_editor_tab_gutter_controls().render_matrix_csv()
            );
        }
        Some("fixture-editor-ui-beta-narrowed") => {
            let packet = seeded_m5_editor_tab_gutter_controls_editor_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-diagnostics-ui-preview-narrowed") => {
            let packet = seeded_m5_editor_tab_gutter_controls_diagnostics_ui_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_editor_tab_gutter_controls(),
                seeded_m5_editor_tab_gutter_controls_editor_ui_beta_narrowed(),
                seeded_m5_editor_tab_gutter_controls_diagnostics_ui_preview_narrowed(),
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
    packet: &M5EditorTabGutterControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
