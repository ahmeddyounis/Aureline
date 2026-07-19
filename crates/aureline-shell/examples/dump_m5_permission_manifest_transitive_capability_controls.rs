//! Headless emitter for the M5 permission-manifest-summary / transitive-capability-drawer controls
//! packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-permission-manifest-summary-transitive-capability-drawer-controls-proof/`,
//! its matrix CSV, the Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-permission-manifest-summary-transitive-capability-drawer-controls/`.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_permission_manifest_transitive_capability_controls -- support-export
//! cargo run -p aureline-shell --example dump_m5_permission_manifest_transitive_capability_controls -- report
//! cargo run -p aureline-shell --example dump_m5_permission_manifest_transitive_capability_controls -- csv
//! cargo run -p aureline-shell --example dump_m5_permission_manifest_transitive_capability_controls -- fixture-marketplace-ui-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_permission_manifest_transitive_capability_controls -- fixture-install-review-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_permission_manifest_transitive_capability_controls -- validate
//! ```

use aureline_shell::implement_the_m5_permission_manifest_summary_and_transitive_capability_drawer_required_optional_inherited_capability_classes_runtime_host_model_data_network_boundary_and_no_vague_full_access_primitive::{
    seeded_m5_permission_manifest_controls,
    seeded_m5_permission_manifest_controls_install_review_ui_preview_narrowed,
    seeded_m5_permission_manifest_controls_marketplace_ui_beta_narrowed,
    M5PermissionManifestControlsPacket,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("support-export") | None => {
            let packet = seeded_m5_permission_manifest_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_permission_manifest_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_permission_manifest_controls().render_matrix_csv()
            );
        }
        Some("fixture-marketplace-ui-beta-narrowed") => {
            let packet = seeded_m5_permission_manifest_controls_marketplace_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-install-review-preview-narrowed") => {
            let packet =
                seeded_m5_permission_manifest_controls_install_review_ui_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_permission_manifest_controls(),
                seeded_m5_permission_manifest_controls_marketplace_ui_beta_narrowed(),
                seeded_m5_permission_manifest_controls_install_review_ui_preview_narrowed(),
            ] {
                assert_valid(&packet)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(
    packet: &M5PermissionManifestControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
