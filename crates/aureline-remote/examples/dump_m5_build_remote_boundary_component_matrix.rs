//! Headless emitter for the frozen M5 build/remote-boundary component matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-build-remote-boundary-proof/`, its matrix CSV, the Markdown design
//! report, and the narrowed fixtures under `fixtures/ui/m5-build-remote-boundary-components/`.
//! Shell, run/test/debug, notebook, preview, AI, companion, incident, and support/export surfaces
//! read this matrix so one adapter-confidence chip names the build/runtime adapter confidence, one
//! discovery-diff card names heuristic-vs-resolved drift, one host-boundary strip names the host
//! kind, one execution-origin receipt row names the origin locus, one managed-workspace lifecycle
//! card names the lifecycle state, one suspend/resume/rebuild review sheet names continuity and
//! changed persistence, one workspace-expiry banner names the expiry timing, and one local-safe
//! continuation card names the local-safe continuation.
//!
//! ```text
//! cargo run -p aureline-remote --example dump_m5_build_remote_boundary_component_matrix -- support-export
//! cargo run -p aureline-remote --example dump_m5_build_remote_boundary_component_matrix -- report
//! cargo run -p aureline-remote --example dump_m5_build_remote_boundary_component_matrix -- csv
//! cargo run -p aureline-remote --example dump_m5_build_remote_boundary_component_matrix -- fixture-adapter-confidence-chip-beta-narrowed
//! cargo run -p aureline-remote --example dump_m5_build_remote_boundary_component_matrix -- fixture-suspend-resume-rebuild-review-sheet-preview-narrowed
//! cargo run -p aureline-remote --example dump_m5_build_remote_boundary_component_matrix -- validate
//! ```

use aureline_remote::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix::{
    seeded_m5_build_remote_boundary_component_matrix,
    seeded_m5_build_remote_boundary_component_matrix_adapter_confidence_chip_beta_narrowed,
    seeded_m5_build_remote_boundary_component_matrix_suspend_resume_rebuild_review_sheet_preview_narrowed,
    M5BuildRemoteBoundaryComponentMatrixPacket,
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
            let packet = seeded_m5_build_remote_boundary_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_build_remote_boundary_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_build_remote_boundary_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-adapter-confidence-chip-beta-narrowed") => {
            let packet =
                seeded_m5_build_remote_boundary_component_matrix_adapter_confidence_chip_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-suspend-resume-rebuild-review-sheet-preview-narrowed") => {
            let packet =
                seeded_m5_build_remote_boundary_component_matrix_suspend_resume_rebuild_review_sheet_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_build_remote_boundary_component_matrix(),
                seeded_m5_build_remote_boundary_component_matrix_adapter_confidence_chip_beta_narrowed(),
                seeded_m5_build_remote_boundary_component_matrix_suspend_resume_rebuild_review_sheet_preview_narrowed(),
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
    packet: &M5BuildRemoteBoundaryComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
