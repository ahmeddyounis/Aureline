//! Conformance dump for the M5 scaffold-preflight-card / template-health-row controls.
//!
//! Prints the canonical support export, the machine-readable matrix CSV, the Markdown design
//! report, or one of the two checked-in scenario fixtures, so the checked artifacts and fixtures
//! can be regenerated deterministically from the canonical seed builders.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_scaffold_readiness_controls -- support-export
//! cargo run -p aureline-templates --example dump_scaffold_readiness_controls -- csv
//! cargo run -p aureline-templates --example dump_scaffold_readiness_controls -- report
//! cargo run -p aureline-templates --example dump_scaffold_readiness_controls -- fixture-preflight-card-blocked
//! cargo run -p aureline-templates --example dump_scaffold_readiness_controls -- fixture-health-row-stale
//! cargo run -p aureline-templates --example dump_scaffold_readiness_controls -- validate
//! ```

use aureline_templates::ship_scaffold_preflight_cards_and_template_health_rows_with_generated_file_counts_immediate_versus_deferred_actions_blocked_warning_optional_checks_and_create_empty_parity_across_claimed_m5_bootstrap_lanes::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support-export".to_owned());
    match which.as_str() {
        "support-export" => {
            let packet = seeded_scaffold_readiness_controls();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "csv" => {
            let packet = seeded_scaffold_readiness_controls();
            assert_valid(&packet);
            print!("{}", packet.render_matrix_csv());
        }
        "report" => {
            let packet = seeded_scaffold_readiness_controls();
            assert_valid(&packet);
            print!("{}", packet.render_markdown_summary());
        }
        "fixture-preflight-card-blocked" => {
            let packet = seeded_scaffold_readiness_controls_preflight_card_blocked();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "fixture-health-row-stale" => {
            let packet = seeded_scaffold_readiness_controls_health_row_stale();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "validate" => {
            let packet = current_scaffold_readiness_controls_export()
                .expect("checked scaffold-readiness controls export validates");
            println!(
                "checked scaffold-readiness controls export valid: {} preflight cards, {} health rows",
                packet.preflight_cards.len(),
                packet.health_rows.len()
            );
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
}

fn assert_valid(packet: &ScaffoldPreflightCardTemplateHealthRowControlsPacket) {
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
}
