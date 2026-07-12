//! Headless emitter for the frozen M5 core-action-input component matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-core-action-input-proof/`, its matrix CSV, the Markdown design report, and the
//! narrowed fixtures under `fixtures/ui/m5-core-action-input-components/`. The forms, settings, search,
//! entry, review, and repair surfaces read this matrix so one button names a permanent label and stable
//! emphasis, one icon button never leaves a destructive action unlabeled, one split button keeps its
//! default action safe, one text field keeps a permanent label and legible validation, one search field
//! preserves clear/submit/privacy truth, one combobox names its value source, one toggle keeps
//! checkbox/radio/switch semantics distinct, and one segmented control stays a mode toggle.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_core_action_input_component_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_core_action_input_component_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_core_action_input_component_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_core_action_input_component_matrix -- fixture-combobox-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_core_action_input_component_matrix -- fixture-segmented-control-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_core_action_input_component_matrix -- validate
//! ```

use aureline_ui::m5_core_action_input_component_matrix::{
    seeded_m5_core_action_input_component_matrix,
    seeded_m5_core_action_input_component_matrix_combobox_beta_narrowed,
    seeded_m5_core_action_input_component_matrix_segmented_control_preview_narrowed,
    M5CoreControlComponentMatrixPacket,
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
            let packet = seeded_m5_core_action_input_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_core_action_input_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_core_action_input_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-combobox-beta-narrowed") => {
            let packet = seeded_m5_core_action_input_component_matrix_combobox_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-segmented-control-preview-narrowed") => {
            let packet =
                seeded_m5_core_action_input_component_matrix_segmented_control_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_core_action_input_component_matrix(),
                seeded_m5_core_action_input_component_matrix_combobox_beta_narrowed(),
                seeded_m5_core_action_input_component_matrix_segmented_control_preview_narrowed(),
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
    packet: &M5CoreControlComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
