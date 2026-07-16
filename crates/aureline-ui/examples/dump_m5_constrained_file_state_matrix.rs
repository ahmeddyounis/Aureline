//! Headless emitter for the frozen M5 constrained-file-state matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/support/m5-constrained-object-state/`, its matrix CSV, the Markdown design report at
//! `artifacts/program/m5-constrained-file-state-matrix.md`, the constrained-object-health dashboard at
//! `dashboards/m5-constrained-object-health.json`, and the narrowed fixtures under
//! `fixtures/editor/m5-constrained-object-states/`. The shell, editor, review, AI / automation, help / docs,
//! and support / export surfaces read this matrix so a constrained object never looks directly writable by
//! omission, no generated / managed / projection / archived object silently falls back to a lossy direct
//! write, no AI / automation / import / repair flow bypasses the constrained-state rules, and the canonical
//! source, exact write target, preserved-versus-lost sync, and recovery / regenerate path stay explicit.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- dashboard
//! cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- fixture-managed-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- fixture-projection-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- validate
//! ```

use aureline_ui::m5_constrained_file_state_matrix::{
    seeded_m5_constrained_file_state_matrix,
    seeded_m5_constrained_file_state_matrix_managed_beta_narrowed,
    seeded_m5_constrained_file_state_matrix_projection_preview_narrowed,
    M5ConstrainedFileStateMatrixPacket,
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
            let packet = seeded_m5_constrained_file_state_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_constrained_file_state_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_constrained_file_state_matrix().render_matrix_csv()
            );
        }
        Some("dashboard") => {
            println!(
                "{}",
                seeded_m5_constrained_file_state_matrix().render_dashboard_json()
            );
        }
        Some("fixture-managed-beta-narrowed") => {
            let packet = seeded_m5_constrained_file_state_matrix_managed_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-projection-preview-narrowed") => {
            let packet = seeded_m5_constrained_file_state_matrix_projection_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_constrained_file_state_matrix(),
                seeded_m5_constrained_file_state_matrix_managed_beta_narrowed(),
                seeded_m5_constrained_file_state_matrix_projection_preview_narrowed(),
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
    packet: &M5ConstrainedFileStateMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
