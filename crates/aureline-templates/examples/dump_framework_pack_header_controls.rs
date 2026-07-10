//! Conformance dump for the M5 framework-pack-header / status-strip controls.
//!
//! Prints the canonical support export, the machine-readable matrix CSV, the Markdown design
//! report, or one of the two checked-in scenario fixtures, so the checked artifacts and fixtures
//! can be regenerated deterministically from the canonical seed builders.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_framework_pack_header_controls -- support-export
//! cargo run -p aureline-templates --example dump_framework_pack_header_controls -- csv
//! cargo run -p aureline-templates --example dump_framework_pack_header_controls -- report
//! cargo run -p aureline-templates --example dump_framework_pack_header_controls -- fixture-pack-header-bridged-remote
//! cargo run -p aureline-templates --example dump_framework_pack_header_controls -- fixture-status-strip-drifted
//! cargo run -p aureline-templates --example dump_framework_pack_header_controls -- validate
//! ```

use aureline_templates::implement_framework_pack_headers_and_framework_status_strips_with_pack_identity_version_support_range_provider_source_freshness_compatibility_and_local_versus_remote_scope_truth::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support-export".to_owned());
    match which.as_str() {
        "support-export" => {
            let packet = seeded_framework_pack_header_controls();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "csv" => {
            let packet = seeded_framework_pack_header_controls();
            assert_valid(&packet);
            print!("{}", packet.render_matrix_csv());
        }
        "report" => {
            let packet = seeded_framework_pack_header_controls();
            assert_valid(&packet);
            print!("{}", packet.render_markdown_summary());
        }
        "fixture-pack-header-bridged-remote" => {
            let packet = seeded_framework_pack_header_controls_bridged_remote();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "fixture-status-strip-drifted" => {
            let packet = seeded_framework_pack_header_controls_status_strip_drifted();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "validate" => {
            let packet = current_framework_pack_header_controls_export()
                .expect("checked framework pack header controls export validates");
            println!(
                "checked framework pack header controls export valid: {} pack headers, {} status strips",
                packet.pack_headers.len(),
                packet.status_strips.len()
            );
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
}

fn assert_valid(packet: &FrameworkPackHeaderStatusStripControlsPacket) {
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
}
