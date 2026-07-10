//! Conformance dump for the M5 scaffold-template-card / starter-parameter-row controls.
//!
//! Prints the canonical support export, the machine-readable matrix CSV, the Markdown design
//! report, or one of the two checked-in scenario fixtures, so the checked artifacts and fixtures
//! can be regenerated deterministically from the canonical seed builders.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_scaffold_entry_controls -- support-export
//! cargo run -p aureline-templates --example dump_scaffold_entry_controls -- csv
//! cargo run -p aureline-templates --example dump_scaffold_entry_controls -- report
//! cargo run -p aureline-templates --example dump_scaffold_entry_controls -- fixture-template-card-community
//! cargo run -p aureline-templates --example dump_scaffold_entry_controls -- fixture-parameter-row-secret-reference
//! cargo run -p aureline-templates --example dump_scaffold_entry_controls -- validate
//! ```

use aureline_templates::implement_scaffold_template_cards_and_starter_parameter_rows_with_source_support_host_boundary_and_portability_truth_across_claimed_m5_project_entry_surfaces::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support-export".to_owned());
    match which.as_str() {
        "support-export" => {
            let packet = seeded_scaffold_entry_controls();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "csv" => {
            let packet = seeded_scaffold_entry_controls();
            assert_valid(&packet);
            print!("{}", packet.render_matrix_csv());
        }
        "report" => {
            let packet = seeded_scaffold_entry_controls();
            assert_valid(&packet);
            print!("{}", packet.render_markdown_summary());
        }
        "fixture-template-card-community" => {
            let packet = seeded_scaffold_entry_controls_template_card_community();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "fixture-parameter-row-secret-reference" => {
            let packet = seeded_scaffold_entry_controls_parameter_row_secret_reference();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "validate" => {
            let packet = current_scaffold_entry_controls_export()
                .expect("checked scaffold-entry controls export validates");
            println!(
                "checked scaffold-entry controls export valid: {} template cards, {} parameter rows",
                packet.template_cards.len(),
                packet.parameter_rows.len()
            );
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
}

fn assert_valid(packet: &ScaffoldTemplateCardStarterParameterRowControlsPacket) {
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
}
