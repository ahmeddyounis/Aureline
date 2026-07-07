//! Headless emitter for the frozen M5 AI-execution/replay component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix/`,
//! its matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix/`.
//! Assistant, patch-review, branch-agent-console, connector-admin, model-manager,
//! run-history, replay-review, and support surfaces read this matrix so one
//! action-state banner names its mode and state, one connector row names its
//! capability and auth posture, one local-model card names its pack state and
//! provenance, one approval sheet names its gate and friction, one tool-call row
//! names its boundary and side effect, one run-history row names its outcome and
//! route, one replay-review sheet names its completeness and rerun reason, and one
//! agent-status card names its lifecycle and manual-takeover path.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_matrix -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_matrix -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_matrix -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_matrix -- fixture-replay-review-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_matrix -- fixture-agent-status-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_matrix -- validate
//! ```

use aureline_ai::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix::{
    seeded_m5_ai_execution_replay_component_matrix,
    seeded_m5_ai_execution_replay_component_matrix_agent_status_preview_narrowed,
    seeded_m5_ai_execution_replay_component_matrix_replay_review_beta_narrowed,
    M5AiExecutionComponentMatrixPacket,
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
            let packet = seeded_m5_ai_execution_replay_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_execution_replay_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_execution_replay_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-replay-review-beta-narrowed") => {
            let packet =
                seeded_m5_ai_execution_replay_component_matrix_replay_review_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-agent-status-preview-narrowed") => {
            let packet =
                seeded_m5_ai_execution_replay_component_matrix_agent_status_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_execution_replay_component_matrix(),
                seeded_m5_ai_execution_replay_component_matrix_replay_review_beta_narrowed(),
                seeded_m5_ai_execution_replay_component_matrix_agent_status_preview_narrowed(),
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
    packet: &M5AiExecutionComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
