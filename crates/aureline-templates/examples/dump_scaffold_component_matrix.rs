//! Conformance dump for the frozen M5 scaffold-component matrix.
//!
//! Prints the canonical support export, the machine-readable matrix CSV, the Markdown design
//! report, or one of the two checked-in narrowed fixtures, so the checked artifacts and
//! fixtures can be regenerated deterministically from the canonical seed builders.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_scaffold_component_matrix -- support-export
//! cargo run -p aureline-templates --example dump_scaffold_component_matrix -- csv
//! cargo run -p aureline-templates --example dump_scaffold_component_matrix -- report
//! cargo run -p aureline-templates --example dump_scaffold_component_matrix -- fixture-scaffold-preflight-card-beta-narrowed
//! cargo run -p aureline-templates --example dump_scaffold_component_matrix -- fixture-scaffold-handoff-banner-preview-narrowed
//! cargo run -p aureline-templates --example dump_scaffold_component_matrix -- validate
//! ```

use aureline_templates::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support-export".to_owned());
    match which.as_str() {
        "support-export" => {
            let packet = seeded_m5_scaffold_component_matrix();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "csv" => {
            let packet = seeded_m5_scaffold_component_matrix();
            assert_valid(&packet);
            print!("{}", packet.render_matrix_csv());
        }
        "report" => {
            let packet = seeded_m5_scaffold_component_matrix();
            assert_valid(&packet);
            print!("{}", packet.render_markdown_summary());
        }
        "fixture-scaffold-preflight-card-beta-narrowed" => {
            let packet =
                seeded_m5_scaffold_component_matrix_scaffold_preflight_card_beta_narrowed();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "fixture-scaffold-handoff-banner-preview-narrowed" => {
            let packet =
                seeded_m5_scaffold_component_matrix_scaffold_handoff_banner_preview_narrowed();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "validate" => {
            let packet = current_stable_m5_scaffold_component_matrix_export()
                .expect("checked scaffold-component matrix export validates");
            println!(
                "checked scaffold-component matrix export valid: {} rows",
                packet.component_rows.len()
            );
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
}

fn assert_valid(packet: &M5ScaffoldComponentMatrixPacket) {
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
}
