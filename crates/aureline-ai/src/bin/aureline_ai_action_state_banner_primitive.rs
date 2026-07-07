//! Headless emitter for the M5 AI action-state-banner / boundary-blocked-banner
//! primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces/`,
//! its matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces/`.
//! The inline explain/fix overlay, the assistant panel, the patch-review lane, the
//! background branch/worktree agent, and the CLI / support export all read this
//! primitive so one banner names its execution mode, action state, scope reach,
//! placement, approval posture, and operator controls, and a boundary-blocked request
//! names the exact blocked boundary and the next safe alternative.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_action_state_banner_primitive -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_action_state_banner_primitive -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_action_state_banner_primitive -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_action_state_banner_primitive -- fixture-patch-review-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_action_state_banner_primitive -- fixture-branch-worktree-agent-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_action_state_banner_primitive -- validate
//! ```

use aureline_ai::implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces::{
    seeded_m5_ai_action_state_banner_primitive_branch_worktree_agent_preview_narrowed,
    seeded_m5_ai_action_state_banner_primitive_packet,
    seeded_m5_ai_action_state_banner_primitive_patch_review_beta_narrowed,
    M5AiActionStateBannerPrimitivePacket,
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
            let packet = seeded_m5_ai_action_state_banner_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_action_state_banner_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_action_state_banner_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-patch-review-beta-narrowed") => {
            let packet = seeded_m5_ai_action_state_banner_primitive_patch_review_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-branch-worktree-agent-preview-narrowed") => {
            let packet =
                seeded_m5_ai_action_state_banner_primitive_branch_worktree_agent_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_action_state_banner_primitive_packet(),
                seeded_m5_ai_action_state_banner_primitive_patch_review_beta_narrowed(),
                seeded_m5_ai_action_state_banner_primitive_branch_worktree_agent_preview_narrowed(),
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
    packet: &M5AiActionStateBannerPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
