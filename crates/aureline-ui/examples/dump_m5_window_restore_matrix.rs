//! Headless emitter for the frozen M5 window-restore matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-window-restore-proof/`, its matrix CSV, the Markdown design report, and the
//! narrowed fixtures under `fixtures/ui/m5-window-restore/`. The shell, recovery, diagnostics, admin, docs,
//! and support surfaces read this matrix so workspace authority and window topology stay separately
//! inspectable, session-scoped tools never silently rerun or reattach, shared authority never clobbers a
//! window-local selection, restore rebuilds the layout skeleton before hydrating heavy dependencies, and
//! display-topology changes keep every window and dialog reachable.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_window_restore_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_window_restore_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_window_restore_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_window_restore_matrix -- fixture-no-rerun-session-hydration-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_window_restore_matrix -- fixture-display-topology-recovery-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_window_restore_matrix -- validate
//! ```

use aureline_ui::m5_window_restore_matrix::{
    seeded_m5_window_restore_matrix,
    seeded_m5_window_restore_matrix_display_topology_recovery_preview_narrowed,
    seeded_m5_window_restore_matrix_no_rerun_session_hydration_beta_narrowed,
    M5WindowRestoreMatrixPacket,
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
            let packet = seeded_m5_window_restore_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_window_restore_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_window_restore_matrix().render_matrix_csv());
        }
        Some("fixture-no-rerun-session-hydration-beta-narrowed") => {
            let packet = seeded_m5_window_restore_matrix_no_rerun_session_hydration_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-display-topology-recovery-preview-narrowed") => {
            let packet =
                seeded_m5_window_restore_matrix_display_topology_recovery_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_window_restore_matrix(),
                seeded_m5_window_restore_matrix_no_rerun_session_hydration_beta_narrowed(),
                seeded_m5_window_restore_matrix_display_topology_recovery_preview_narrowed(),
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

fn assert_valid(packet: &M5WindowRestoreMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
