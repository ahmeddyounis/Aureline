//! Conformance dump for the M5 generated-project-diff-card / scaffold-handoff-banner controls.
//!
//! Prints the canonical support export, the machine-readable matrix CSV, the Markdown design
//! report, or one of the two checked-in scenario fixtures, so the checked artifacts and fixtures
//! can be regenerated deterministically from the canonical seed builders.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_scaffold_generation_controls -- support-export
//! cargo run -p aureline-templates --example dump_scaffold_generation_controls -- csv
//! cargo run -p aureline-templates --example dump_scaffold_generation_controls -- report
//! cargo run -p aureline-templates --example dump_scaffold_generation_controls -- fixture-diff-card-conflict
//! cargo run -p aureline-templates --example dump_scaffold_generation_controls -- fixture-handoff-banner-partial
//! cargo run -p aureline-templates --example dump_scaffold_generation_controls -- validate
//! ```

use aureline_templates::implement_generated_project_diff_cards_and_scaffold_handoff_banners_with_create_modify_rename_delete_counts_dependency_task_extension_impact_trust_state_and_run_now_later_review_recovery_truth_across_claimed_m5_generation_flows::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support-export".to_owned());
    match which.as_str() {
        "support-export" => {
            let packet = seeded_scaffold_generation_controls();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "csv" => {
            let packet = seeded_scaffold_generation_controls();
            assert_valid(&packet);
            print!("{}", packet.render_matrix_csv());
        }
        "report" => {
            let packet = seeded_scaffold_generation_controls();
            assert_valid(&packet);
            print!("{}", packet.render_markdown_summary());
        }
        "fixture-diff-card-conflict" => {
            let packet = seeded_scaffold_generation_controls_diff_card_conflict();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "fixture-handoff-banner-partial" => {
            let packet = seeded_scaffold_generation_controls_handoff_banner_partial();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "validate" => {
            let packet = current_scaffold_generation_controls_export()
                .expect("checked scaffold-generation controls export validates");
            println!(
                "checked scaffold-generation controls export valid: {} diff cards, {} handoff banners",
                packet.diff_cards.len(),
                packet.handoff_banners.len()
            );
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
}

fn assert_valid(packet: &GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket) {
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
}
