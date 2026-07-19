//! Headless emitter for the M5 draft-state-row / stale-banner / send-review-control primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces/`,
//! its matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces/`.
//! The inline composer, the side panel, the patch draft, the CLI / headless surface, and the
//! support export all read this primitive so one draft-state row names the draft locality and
//! retention posture, one attachment-stale banner preserves the draft and offers a refresh or a
//! local-safe alternative, and one send-review control splits a high-authority send into
//! explain-only / review / mutating paths instead of one unqualified send.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_draft_state_row_attachment_stale -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_draft_state_row_attachment_stale -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_draft_state_row_attachment_stale -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_draft_state_row_attachment_stale -- fixture-patch-draft-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_draft_state_row_attachment_stale -- fixture-cli-headless-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_draft_state_row_attachment_stale -- validate
//! ```

use aureline_ai::ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces::{
    seeded_m5_draft_send_cli_headless_beta_narrowed, seeded_m5_draft_send_packet,
    seeded_m5_draft_send_patch_draft_preview_narrowed, M5DraftSendPacket,
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
            let packet = seeded_m5_draft_send_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_draft_send_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_draft_send_packet().render_matrix_csv());
        }
        Some("fixture-patch-draft-preview-narrowed") => {
            let packet = seeded_m5_draft_send_patch_draft_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-cli-headless-beta-narrowed") => {
            let packet = seeded_m5_draft_send_cli_headless_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_draft_send_packet(),
                seeded_m5_draft_send_patch_draft_preview_narrowed(),
                seeded_m5_draft_send_cli_headless_beta_narrowed(),
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

fn assert_valid(packet: &M5DraftSendPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
