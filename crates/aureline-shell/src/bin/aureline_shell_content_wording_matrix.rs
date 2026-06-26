//! Headless emitter for the frozen M5 content-design, controlled-vocabulary,
//! content-ops metadata, and commercial-boundary wording matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under
//! `artifacts/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/`
//! and the narrowed fixtures under
//! `fixtures/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/`.
//! Release, help, docs, and support automation read this matrix so claimed M5
//! rows cannot harden wording with prose that lacks a governed source.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_content_wording_matrix -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_content_wording_matrix -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_content_wording_matrix -- fixture-commercial-boundary-held
//! cargo run -q -p aureline-shell --bin aureline_shell_content_wording_matrix -- fixture-ai-guardrail-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_content_wording_matrix -- validate
//! ```

use aureline_shell::freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix::{
    seeded_m5_content_wording_matrix, seeded_m5_content_wording_matrix_ai_guardrail_narrowed,
    seeded_m5_content_wording_matrix_commercial_boundary_held,
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
            let packet = seeded_m5_content_wording_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_content_wording_matrix().render_markdown_summary()
            );
        }
        Some("fixture-commercial-boundary-held") => {
            let packet = seeded_m5_content_wording_matrix_commercial_boundary_held();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-ai-guardrail-narrowed") => {
            let packet = seeded_m5_content_wording_matrix_ai_guardrail_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_content_wording_matrix(),
                seeded_m5_content_wording_matrix_commercial_boundary_held(),
                seeded_m5_content_wording_matrix_ai_guardrail_narrowed(),
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
    packet: &aureline_shell::freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix::M5ContentWordingMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
