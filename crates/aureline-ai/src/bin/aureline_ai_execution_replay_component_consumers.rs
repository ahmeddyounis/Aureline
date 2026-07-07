//! Headless emitter for the M5 AI execution/replay-component-consumer adoption lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/ai/m5/m5-ai-execution-replay-component-consumer-proof/`, its matrix CSV,
//! the Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/m5-ai-execution-replay-component-consumers/`. Patch review, the
//! evidence inspector, the branch/worktree agent queue, the support export, and the
//! docs/help surface all read this adoption matrix so the reusable AI action-state,
//! connector/local-model, approval, tool-call, run-history, replay, and agent-status
//! components keep route, approval, checkpoint-lineage, and replay-completeness truth
//! aligned, and auto-narrow their claim language whenever route/provider/model drift,
//! a missing connector output, a redaction fence, or a stale approval weakens
//! replayability.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_consumers -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_consumers -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_consumers -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_consumers -- fixture-branch-queue-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_consumers -- fixture-docs-help-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_consumers -- validate
//! ```

use aureline_ai::add_shared_patch_review_evidence_inspector_branch_worktree_queue_support_export_and_docs_help_ai_execution_replay_component_consumers::{
    seeded_m5_ai_execution_replay_consumer_branch_queue_beta_narrowed,
    seeded_m5_ai_execution_replay_consumer_docs_help_preview_narrowed,
    seeded_m5_ai_execution_replay_consumer_packet, M5AiExecutionReplayConsumerPacket,
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
            let packet = seeded_m5_ai_execution_replay_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_execution_replay_consumer_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_execution_replay_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-branch-queue-beta-narrowed") => {
            let packet = seeded_m5_ai_execution_replay_consumer_branch_queue_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-docs-help-preview-narrowed") => {
            let packet = seeded_m5_ai_execution_replay_consumer_docs_help_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_execution_replay_consumer_packet(),
                seeded_m5_ai_execution_replay_consumer_branch_queue_beta_narrowed(),
                seeded_m5_ai_execution_replay_consumer_docs_help_preview_narrowed(),
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
    packet: &M5AiExecutionReplayConsumerPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("consumer matrix failed validation: {}", tokens.join(",")).into())
    }
}
