//! Headless emitter for the frozen M5 change-orchestration matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-change-orchestration-proof/`, its matrix CSV, the Markdown design report at
//! `artifacts/design/m5-change-orchestration-component-matrix.md`, the change-orchestration-health dashboard at
//! `dashboards/m5-change-orchestration-health.json`, and the narrowed fixtures under
//! `fixtures/git/m5-change-orchestration/`. The Git, review, AI, provider, help / docs, and support / export
//! surfaces read this matrix so stack membership is never inferred from branch names alone, no command / AI tool /
//! refactor / formatter / provider action mutates files in another worktree without a selected change object and
//! worktree binding, stack members are never silently reordered, nothing lands from ambient branch state, and no
//! orphaned worktree or stale stack member is deleted without previewing running work and export-safe evidence.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_change_orchestration_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_change_orchestration_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_change_orchestration_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_change_orchestration_matrix -- dashboard
//! cargo run -p aureline-ui --example dump_m5_change_orchestration_matrix -- fixture-patch-stack-queue-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_change_orchestration_matrix -- fixture-worktree-cleanup-preview-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_change_orchestration_matrix -- validate
//! ```

use aureline_ui::m5_change_object_patch_stack_and_landing_matrix::{
    seeded_m5_change_orchestration_matrix,
    seeded_m5_change_orchestration_matrix_patch_stack_queue_beta_narrowed,
    seeded_m5_change_orchestration_matrix_worktree_cleanup_preview_preview_narrowed,
    M5ChangeOrchestrationMatrixPacket,
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
            let packet = seeded_m5_change_orchestration_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_change_orchestration_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_change_orchestration_matrix().render_matrix_csv()
            );
        }
        Some("dashboard") => {
            println!(
                "{}",
                seeded_m5_change_orchestration_matrix().render_dashboard_json()
            );
        }
        Some("fixture-patch-stack-queue-beta-narrowed") => {
            let packet = seeded_m5_change_orchestration_matrix_patch_stack_queue_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-worktree-cleanup-preview-preview-narrowed") => {
            let packet =
                seeded_m5_change_orchestration_matrix_worktree_cleanup_preview_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_change_orchestration_matrix(),
                seeded_m5_change_orchestration_matrix_patch_stack_queue_beta_narrowed(),
                seeded_m5_change_orchestration_matrix_worktree_cleanup_preview_preview_narrowed(),
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
    packet: &M5ChangeOrchestrationMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
