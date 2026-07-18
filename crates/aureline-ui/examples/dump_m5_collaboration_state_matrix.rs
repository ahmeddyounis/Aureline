//! Headless emitter for the frozen M5 collaboration-state matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-collaboration-convergence-proof/`, its matrix CSV, the Markdown design report at
//! `artifacts/design/m5-collaboration-state-authority-matrix.md`, the collaboration-convergence-health dashboard
//! at `dashboards/m5-collaboration-convergence-health.json`, and the narrowed fixtures under
//! `fixtures/collaboration/m5-convergence/`. The shared editor, shared terminal / debug, review / comment,
//! companion follow, search / AI, and support / export surfaces read this matrix so a collaboration replica
//! never overwrites the canonical local buffer, VFS, or Git truth, every shared object declares its authority
//! model, a permission or relay downgrade preserves local unsent work first, anchor drift stays append-only and
//! reviewable, convergence-degraded and awareness-degraded states are never collapsed into a generic stale badge,
//! and op-logs, snapshots, and archives export only with policy-labeled redaction and actor lineage.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_matrix -- dashboard
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_matrix -- fixture-higher-risk-control-plane-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_matrix -- fixture-sealed-session-archive-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_matrix -- validate
//! ```

use aureline_ui::m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix::{
    seeded_m5_collaboration_state_matrix,
    seeded_m5_collaboration_state_matrix_higher_risk_control_plane_beta_narrowed,
    seeded_m5_collaboration_state_matrix_sealed_session_archive_preview_narrowed,
    M5CollaborationStateMatrixPacket,
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
            let packet = seeded_m5_collaboration_state_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_collaboration_state_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_collaboration_state_matrix().render_matrix_csv()
            );
        }
        Some("dashboard") => {
            println!(
                "{}",
                seeded_m5_collaboration_state_matrix().render_dashboard_json()
            );
        }
        Some("fixture-higher-risk-control-plane-beta-narrowed") => {
            let packet =
                seeded_m5_collaboration_state_matrix_higher_risk_control_plane_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-sealed-session-archive-preview-narrowed") => {
            let packet =
                seeded_m5_collaboration_state_matrix_sealed_session_archive_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_collaboration_state_matrix(),
                seeded_m5_collaboration_state_matrix_higher_risk_control_plane_beta_narrowed(),
                seeded_m5_collaboration_state_matrix_sealed_session_archive_preview_narrowed(),
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
    packet: &M5CollaborationStateMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
