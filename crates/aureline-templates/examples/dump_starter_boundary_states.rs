//! Conformance dump for the M5 starter-boundary-state controls.
//!
//! Prints the canonical support export, the machine-readable matrix CSV, the Markdown design
//! report, or one of the two checked-in scenario fixtures, so the checked artifacts and fixtures
//! can be regenerated deterministically from the canonical seed builders.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_starter_boundary_states -- support-export
//! cargo run -p aureline-templates --example dump_starter_boundary_states -- csv
//! cargo run -p aureline-templates --example dump_starter_boundary_states -- report
//! cargo run -p aureline-templates --example dump_starter_boundary_states -- fixture-mirror-only-offline
//! cargo run -p aureline-templates --example dump_starter_boundary_states -- fixture-sign-in-required
//! cargo run -p aureline-templates --example dump_starter_boundary_states -- validate
//! ```

use aureline_templates::ship_mirror_offline_auth_boundary_and_managed_zone_starter_states_with_no_silent_trust_no_silent_install_and_non_durable_temp_staging_honesty_across_claimed_m5_scaffold_surfaces::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support-export".to_owned());
    match which.as_str() {
        "support-export" => {
            let packet = seeded_starter_boundary_states();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "csv" => {
            let packet = seeded_starter_boundary_states();
            assert_valid(&packet);
            print!("{}", packet.render_matrix_csv());
        }
        "report" => {
            let packet = seeded_starter_boundary_states();
            assert_valid(&packet);
            print!("{}", packet.render_markdown_summary());
        }
        "fixture-mirror-only-offline" => {
            let packet = seeded_starter_boundary_states_mirror_only_offline();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "fixture-sign-in-required" => {
            let packet = seeded_starter_boundary_states_sign_in_required();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "validate" => {
            let packet = current_starter_boundary_state_export()
                .expect("checked starter boundary state export validates");
            println!(
                "checked starter boundary state export valid: {} boundary states",
                packet.boundary_states.len()
            );
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
}

fn assert_valid(packet: &StarterBoundaryStateControlsPacket) {
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
}
