//! Conformance dump for the M5 convention-diagnostic-row / derived-relationship-banner controls.
//!
//! Prints the canonical support export, the machine-readable matrix CSV, the Markdown design
//! report, or one of the two checked-in scenario fixtures, so the checked artifacts and fixtures
//! can be regenerated deterministically from the canonical seed builders.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_convention_diagnostic_relationship_controls -- support-export
//! cargo run -p aureline-templates --example dump_convention_diagnostic_relationship_controls -- csv
//! cargo run -p aureline-templates --example dump_convention_diagnostic_relationship_controls -- report
//! cargo run -p aureline-templates --example dump_convention_diagnostic_relationship_controls -- fixture-heuristic-diagnostic
//! cargo run -p aureline-templates --example dump_convention_diagnostic_relationship_controls -- fixture-inferred-relationship
//! cargo run -p aureline-templates --example dump_convention_diagnostic_relationship_controls -- validate
//! ```

use aureline_templates::implement_convention_diagnostic_rows_and_derived_relationship_banners_with_diagnostic_class_affected_entity_or_file_certainty_detected_source_suggested_fix_or_open_docs_actions_support_class_caveats_and_open_raw_source_or_wider_graph_continuity::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support-export".to_owned());
    match which.as_str() {
        "support-export" => {
            let packet = seeded_convention_relationship_controls();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "csv" => {
            let packet = seeded_convention_relationship_controls();
            assert_valid(&packet);
            print!("{}", packet.render_matrix_csv());
        }
        "report" => {
            let packet = seeded_convention_relationship_controls();
            assert_valid(&packet);
            print!("{}", packet.render_markdown_summary());
        }
        "fixture-heuristic-diagnostic" => {
            let packet = seeded_convention_relationship_controls_heuristic_diagnostic();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "fixture-inferred-relationship" => {
            let packet = seeded_convention_relationship_controls_inferred_relationship();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "validate" => {
            let packet = current_convention_relationship_controls_export()
                .expect("checked convention relationship controls export validates");
            println!(
                "checked convention relationship controls export valid: {} diagnostic rows, {} relationship banners",
                packet.diagnostic_rows.len(),
                packet.relationship_banners.len()
            );
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
}

fn assert_valid(packet: &ConventionDiagnosticDerivedRelationshipControlsPacket) {
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
}
