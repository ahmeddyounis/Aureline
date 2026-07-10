//! Conformance dump for the M5 generator-preview-sheet / run-config-scaffold-card controls.
//!
//! Prints the canonical support export, the machine-readable matrix CSV, the Markdown design report,
//! or one of the two checked-in scenario fixtures, so the checked artifacts and fixtures can be
//! regenerated deterministically from the canonical seed builders.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_generator_preview_run_config_controls -- support-export
//! cargo run -p aureline-templates --example dump_generator_preview_run_config_controls -- csv
//! cargo run -p aureline-templates --example dump_generator_preview_run_config_controls -- report
//! cargo run -p aureline-templates --example dump_generator_preview_run_config_controls -- fixture-writing-generator
//! cargo run -p aureline-templates --example dump_generator_preview_run_config_controls -- fixture-remote-run-config
//! cargo run -p aureline-templates --example dump_generator_preview_run_config_controls -- validate
//! ```

use aureline_templates::implement_generator_preview_sheets_and_run_config_scaffold_cards_with_generator_version_file_effect_classes_dependency_config_impact_rollback_or_regenerate_posture_required_toolchains_and_local_container_ssh_managed_target_truth::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support-export".to_owned());
    match which.as_str() {
        "support-export" => {
            let packet = seeded_generator_run_config_controls();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "csv" => {
            let packet = seeded_generator_run_config_controls();
            assert_valid(&packet);
            print!("{}", packet.render_matrix_csv());
        }
        "report" => {
            let packet = seeded_generator_run_config_controls();
            assert_valid(&packet);
            print!("{}", packet.render_markdown_summary());
        }
        "fixture-writing-generator" => {
            let packet = seeded_generator_run_config_controls_writing_generator();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "fixture-remote-run-config" => {
            let packet = seeded_generator_run_config_controls_remote_run_config();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "validate" => {
            let packet = current_generator_run_config_controls_export()
                .expect("checked generator run config controls export validates");
            println!(
                "checked generator run config controls export valid: {} generator sheets, {} run-config cards",
                packet.generator_sheets.len(),
                packet.run_config_cards.len()
            );
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
}

fn assert_valid(packet: &GeneratorPreviewRunConfigControlsPacket) {
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
}
