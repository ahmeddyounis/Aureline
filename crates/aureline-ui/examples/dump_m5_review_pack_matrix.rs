//! Headless emitter for the frozen M5 review-pack matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/review/m5-review-pack-results/`, its matrix CSV, the Markdown design report at
//! `artifacts/review/m5-review-pack-evaluator-matrix.md`, the review-pack-health dashboard at
//! `dashboards/m5-review-pack-health.json`, and the narrowed fixtures under
//! `fixtures/review/m5-review-pack-parity/`. The review, merge-readiness, AI review, provider handoff, help /
//! docs, and support / export surfaces read this matrix so a local parity estimate never masquerades as
//! provider-authoritative mergeability, no ci-only / not-evaluated-here / provider-unavailable state is hidden
//! behind a green summary, advisory-owner and enforced-owner are never flattened, AI review never runs under a
//! different pack version without disclosure, and no review-pack version / digest or template attribution is
//! lost on export, publish, or reopen.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- dashboard
//! cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- fixture-local-ci-parity-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- fixture-ai-policy-hook-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- validate
//! ```

use aureline_ui::m5_review_pack_evaluator_matrix::{
    seeded_m5_review_pack_matrix, seeded_m5_review_pack_matrix_ai_policy_hook_preview_narrowed,
    seeded_m5_review_pack_matrix_local_ci_parity_beta_narrowed, M5ReviewPackMatrixPacket,
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
            let packet = seeded_m5_review_pack_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_review_pack_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_review_pack_matrix().render_matrix_csv());
        }
        Some("dashboard") => {
            println!("{}", seeded_m5_review_pack_matrix().render_dashboard_json());
        }
        Some("fixture-local-ci-parity-beta-narrowed") => {
            let packet = seeded_m5_review_pack_matrix_local_ci_parity_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-ai-policy-hook-preview-narrowed") => {
            let packet = seeded_m5_review_pack_matrix_ai_policy_hook_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_review_pack_matrix(),
                seeded_m5_review_pack_matrix_local_ci_parity_beta_narrowed(),
                seeded_m5_review_pack_matrix_ai_policy_hook_preview_narrowed(),
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

fn assert_valid(packet: &M5ReviewPackMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
