//! Headless emitter for the M5 AI high-friction-approval-sheet /
//! tool-call-timeline-row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes/`,
//! its matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes/`.
//! Read-only tool invocations, mutating tool runs, test-generation validations,
//! branch-agent checkpoints, and the CLI / support export all read this primitive so one
//! approval sheet names its requested action, scope, side effect, boundary, rollback /
//! checkpoint, effective gate, and explicit approve-once/deny/open-plan controls, and one
//! tool-call timeline row names its time, tool, side-effect class, boundary, outcome, and
//! governed open-output / remove-from-context / view-provenance follow-up actions.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_high_friction_approval_sheet_tool_call_timeline_row_primitive -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_high_friction_approval_sheet_tool_call_timeline_row_primitive -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_high_friction_approval_sheet_tool_call_timeline_row_primitive -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_high_friction_approval_sheet_tool_call_timeline_row_primitive -- fixture-mutating-tool-run-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_high_friction_approval_sheet_tool_call_timeline_row_primitive -- fixture-branch-agent-checkpoint-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_high_friction_approval_sheet_tool_call_timeline_row_primitive -- validate
//! ```

use aureline_ai::implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes::{
    seeded_m5_ai_approval_tool_call_primitive_branch_agent_checkpoint_beta_narrowed,
    seeded_m5_ai_approval_tool_call_primitive_mutating_tool_run_preview_narrowed,
    seeded_m5_ai_approval_tool_call_primitive_packet, M5AiApprovalToolCallPrimitivePacket,
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
            let packet = seeded_m5_ai_approval_tool_call_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_approval_tool_call_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_approval_tool_call_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-mutating-tool-run-preview-narrowed") => {
            let packet = seeded_m5_ai_approval_tool_call_primitive_mutating_tool_run_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-branch-agent-checkpoint-beta-narrowed") => {
            let packet =
                seeded_m5_ai_approval_tool_call_primitive_branch_agent_checkpoint_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_approval_tool_call_primitive_packet(),
                seeded_m5_ai_approval_tool_call_primitive_mutating_tool_run_preview_narrowed(),
                seeded_m5_ai_approval_tool_call_primitive_branch_agent_checkpoint_beta_narrowed(),
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
    packet: &M5AiApprovalToolCallPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
