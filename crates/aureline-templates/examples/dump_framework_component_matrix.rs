//! Conformance dump for the frozen M5 framework-component matrix.
//!
//! Prints the canonical support export, the machine-readable matrix CSV, the Markdown design
//! report, or one of the two checked-in narrowed fixtures, so the checked artifacts and
//! fixtures can be regenerated deterministically from the canonical seed builders.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_framework_component_matrix -- support-export
//! cargo run -p aureline-templates --example dump_framework_component_matrix -- csv
//! cargo run -p aureline-templates --example dump_framework_component_matrix -- report
//! cargo run -p aureline-templates --example dump_framework_component_matrix -- fixture-route-endpoint-row-beta-narrowed
//! cargo run -p aureline-templates --example dump_framework_component_matrix -- fixture-generator-preview-sheet-preview-narrowed
//! cargo run -p aureline-templates --example dump_framework_component_matrix -- validate
//! ```

use aureline_templates::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support-export".to_owned());
    match which.as_str() {
        "support-export" => {
            let packet = seeded_m5_framework_component_matrix();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "csv" => {
            let packet = seeded_m5_framework_component_matrix();
            assert_valid(&packet);
            print!("{}", packet.render_matrix_csv());
        }
        "report" => {
            let packet = seeded_m5_framework_component_matrix();
            assert_valid(&packet);
            print!("{}", packet.render_markdown_summary());
        }
        "fixture-route-endpoint-row-beta-narrowed" => {
            let packet = seeded_m5_framework_component_matrix_route_endpoint_row_beta_narrowed();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "fixture-generator-preview-sheet-preview-narrowed" => {
            let packet =
                seeded_m5_framework_component_matrix_generator_preview_sheet_preview_narrowed();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "validate" => {
            let packet = current_stable_m5_framework_component_matrix_export()
                .expect("checked framework-component matrix export validates");
            println!(
                "checked framework-component matrix export valid: {} rows",
                packet.component_rows.len()
            );
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
}

fn assert_valid(packet: &M5FrameworkComponentMatrixPacket) {
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
}
