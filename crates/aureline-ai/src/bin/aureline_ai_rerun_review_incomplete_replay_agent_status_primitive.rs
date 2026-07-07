//! Headless emitter for the M5 AI rerun-review-sheet / incomplete-replay-banner /
//! agent-status-card primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows/`,
//! its matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows/`.
//! The rerun-review, branch-agent console, run-history, support, and CLI surfaces all read
//! this primitive so one rerun-review sheet names its drift and approval-reuse verdict, one
//! incomplete-replay banner names its retained versus missing segments, and one agent-status
//! card names its lifecycle, checkpoint, blast radius, and safe continue-manually / restart /
//! takeover options.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_rerun_review_incomplete_replay_agent_status_primitive -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_rerun_review_incomplete_replay_agent_status_primitive -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_rerun_review_incomplete_replay_agent_status_primitive -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_rerun_review_incomplete_replay_agent_status_primitive -- fixture-rerun-blocked-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_rerun_review_incomplete_replay_agent_status_primitive -- fixture-support-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_rerun_review_incomplete_replay_agent_status_primitive -- validate
//! ```

use aureline_ai::implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows::{
    seeded_m5_ai_background_agent_replay_primitive_packet,
    seeded_m5_ai_background_agent_replay_primitive_rerun_blocked_preview_narrowed,
    seeded_m5_ai_background_agent_replay_primitive_support_beta_narrowed,
    M5AiBackgroundAgentReplayPrimitivePacket,
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
            let packet = seeded_m5_ai_background_agent_replay_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_background_agent_replay_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_background_agent_replay_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-rerun-blocked-preview-narrowed") => {
            let packet =
                seeded_m5_ai_background_agent_replay_primitive_rerun_blocked_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-support-beta-narrowed") => {
            let packet = seeded_m5_ai_background_agent_replay_primitive_support_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_background_agent_replay_primitive_packet(),
                seeded_m5_ai_background_agent_replay_primitive_rerun_blocked_preview_narrowed(),
                seeded_m5_ai_background_agent_replay_primitive_support_beta_narrowed(),
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
    packet: &M5AiBackgroundAgentReplayPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
