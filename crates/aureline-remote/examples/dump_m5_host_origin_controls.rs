//! Headless emitter for the M5 host-boundary-strip / execution-origin-receipt-row controls packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-host-boundary-strip-execution-origin-receipt-row-controls-proof/`, its
//! matrix CSV, the Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-host-boundary-strip-execution-origin-receipt-row-controls/`.
//!
//! ```text
//! cargo run -p aureline-remote --example dump_m5_host_origin_controls -- support-export
//! cargo run -p aureline-remote --example dump_m5_host_origin_controls -- report
//! cargo run -p aureline-remote --example dump_m5_host_origin_controls -- csv
//! cargo run -p aureline-remote --example dump_m5_host_origin_controls -- fixture-host-boundary-strip-beta-narrowed
//! cargo run -p aureline-remote --example dump_m5_host_origin_controls -- fixture-execution-origin-receipt-row-preview-narrowed
//! cargo run -p aureline-remote --example dump_m5_host_origin_controls -- validate
//! ```

use aureline_remote::implement_the_m5_host_boundary_strip_and_execution_origin_receipt_row_locality_class_target_label_owning_runtime_service_lane_reconnect_degraded_state_action_class_resolved_target_identity_provenance_and_export_safe_lineage_primitive::{
    seeded_m5_host_origin_controls,
    seeded_m5_host_origin_controls_execution_origin_receipt_row_preview_narrowed,
    seeded_m5_host_origin_controls_host_boundary_strip_beta_narrowed,
    M5HostOriginControlsPacket,
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
            let packet = seeded_m5_host_origin_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_host_origin_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_host_origin_controls().render_matrix_csv());
        }
        Some("fixture-host-boundary-strip-beta-narrowed") => {
            let packet = seeded_m5_host_origin_controls_host_boundary_strip_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-execution-origin-receipt-row-preview-narrowed") => {
            let packet =
                seeded_m5_host_origin_controls_execution_origin_receipt_row_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_host_origin_controls(),
                seeded_m5_host_origin_controls_host_boundary_strip_beta_narrowed(),
                seeded_m5_host_origin_controls_execution_origin_receipt_row_preview_narrowed(),
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

fn assert_valid(packet: &M5HostOriginControlsPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
