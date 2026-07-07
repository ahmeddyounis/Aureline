//! Headless emitter for the M5 budget-strip / tainted-context-warning primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes/`,
//! its matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes/`.
//! The inline composer, the side panel, the patch draft, the CLI / headless surface, and the
//! support export all read this primitive so one budget strip names the included versus
//! omitted context classes, the pressure band, the truncation reason, and the route-switch
//! consequence, and one tainted-context warning names the taint source, severity,
//! data-treatment, and the review path before a side-effecting route runs.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_budget_size_strip_tainted_context_warning_primitive -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_budget_size_strip_tainted_context_warning_primitive -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_budget_size_strip_tainted_context_warning_primitive -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_budget_size_strip_tainted_context_warning_primitive -- fixture-patch-draft-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_budget_size_strip_tainted_context_warning_primitive -- fixture-cli-headless-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_budget_size_strip_tainted_context_warning_primitive -- validate
//! ```

use aureline_ai::implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes::{
    seeded_m5_budget_taint_cli_headless_beta_narrowed, seeded_m5_budget_taint_packet,
    seeded_m5_budget_taint_patch_draft_preview_narrowed, M5BudgetTaintPacket,
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
            let packet = seeded_m5_budget_taint_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_budget_taint_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_budget_taint_packet().render_matrix_csv());
        }
        Some("fixture-patch-draft-preview-narrowed") => {
            let packet = seeded_m5_budget_taint_patch_draft_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-cli-headless-beta-narrowed") => {
            let packet = seeded_m5_budget_taint_cli_headless_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_budget_taint_packet(),
                seeded_m5_budget_taint_patch_draft_preview_narrowed(),
                seeded_m5_budget_taint_cli_headless_beta_narrowed(),
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

fn assert_valid(packet: &M5BudgetTaintPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
