//! Headless emitter for the frozen M5 lifecycle-state and journey-checkpoint
//! matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-lifecycle-proof/`, the matrix CSV
//! `artifacts/release/m5-lifecycle-proof/matrix.csv`, the Markdown lifecycle
//! report `artifacts/lifecycle/m5-lifecycle-matrix.md`, and the narrowed
//! fixtures under `fixtures/state/m5-lifecycle-scenarios/`. Product UI, CLI,
//! docs/help, diagnostics, support export, telemetry, and claim tooling read
//! this matrix so a claimed M5 row cannot invent a private state vocabulary or
//! an anonymous checkpoint.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_matrix -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_matrix -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_matrix -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_matrix -- fixture-remote-session-degraded-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_matrix -- fixture-notebook-runtime-retest-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_matrix -- validate
//! ```

use aureline_shell::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix::{
    seeded_m5_lifecycle_matrix, seeded_m5_lifecycle_matrix_notebook_runtime_retest_narrowed,
    seeded_m5_lifecycle_matrix_remote_session_degraded_narrowed, M5LifecycleMatrixPacket,
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
            let packet = seeded_m5_lifecycle_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!("{}", seeded_m5_lifecycle_matrix().render_markdown_summary());
        }
        Some("csv") => {
            print!("{}", seeded_m5_lifecycle_matrix().render_matrix_csv());
        }
        Some("fixture-remote-session-degraded-narrowed") => {
            let packet = seeded_m5_lifecycle_matrix_remote_session_degraded_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-notebook-runtime-retest-narrowed") => {
            let packet = seeded_m5_lifecycle_matrix_notebook_runtime_retest_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_lifecycle_matrix(),
                seeded_m5_lifecycle_matrix_remote_session_degraded_narrowed(),
                seeded_m5_lifecycle_matrix_notebook_runtime_retest_narrowed(),
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

fn assert_valid(packet: &M5LifecycleMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
