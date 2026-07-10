//! Conformance dump for the M5 scaffold-component consumer-adoption lane.
//!
//! Prints the canonical support export, the machine-readable matrix CSV, the Markdown report, or one
//! of the two checked-in narrowed fixtures, so the checked artifacts and fixtures can be regenerated
//! deterministically from the canonical seed builders.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_scaffold_component_consumers -- support-export
//! cargo run -p aureline-templates --example dump_scaffold_component_consumers -- csv
//! cargo run -p aureline-templates --example dump_scaffold_component_consumers -- report
//! cargo run -p aureline-templates --example dump_scaffold_component_consumers -- fixture-framework-pack-beta-narrowed
//! cargo run -p aureline-templates --example dump_scaffold_component_consumers -- fixture-workspace-admission-preview-narrowed
//! cargo run -p aureline-templates --example dump_scaffold_component_consumers -- validate
//! ```

use aureline_templates::add_shared_start_center_workspace_admission_template_registry_framework_pack_workflow_bundle_and_support_consumers_so_scaffold_components_keep_source_side_effect_and_health_language_aligned_across_claimed_m5_profiles::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support-export".to_owned());
    match which.as_str() {
        "support-export" => {
            let packet = seeded_m5_scaffold_component_consumer_packet();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "csv" => {
            let packet = seeded_m5_scaffold_component_consumer_packet();
            assert_valid(&packet);
            print!("{}", packet.render_matrix_csv());
        }
        "report" => {
            let packet = seeded_m5_scaffold_component_consumer_packet();
            assert_valid(&packet);
            print!("{}", packet.render_markdown_summary());
        }
        "fixture-framework-pack-beta-narrowed" => {
            let packet = seeded_m5_scaffold_component_consumer_framework_pack_beta_narrowed();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "fixture-workspace-admission-preview-narrowed" => {
            let packet =
                seeded_m5_scaffold_component_consumer_workspace_admission_preview_narrowed();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "validate" => {
            let packet = current_stable_m5_scaffold_component_consumer_export()
                .expect("checked scaffold-component consumer export validates");
            println!(
                "checked scaffold-component consumer export valid: {} rows",
                packet.consumer_rows.len()
            );
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
}

fn assert_valid(packet: &M5ScaffoldComponentConsumerPacket) {
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
}
