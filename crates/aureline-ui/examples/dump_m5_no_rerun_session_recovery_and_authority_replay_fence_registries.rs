//! Headless emitter for the M5 no-rerun session-recovery and authority-replay-fence registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-no-rerun-session-recovery-and-authority-replay-fence-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-no-rerun-session-recovery-and-authority-replay-fence-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- report
//! cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- recovery-posture-table
//! cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- fixture-reconnect-posture-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- fixture-context-only-continuity-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- validate
//! ```

use aureline_ui::m5_no_rerun_session_recovery_and_authority_replay_fence_registries::{
    seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries,
    seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_context_only_continuity_preview_narrowed,
    seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_reconnect_posture_beta_narrowed,
    M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket,
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
            let packet =
                seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries()
                    .render_matrix_csv()
            );
        }
        Some("recovery-posture-table") => {
            print!(
                "{}",
                seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries()
                    .render_recovery_posture_table()
            );
        }
        Some("fixture-reconnect-posture-beta-narrowed") => {
            let packet =
                seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_reconnect_posture_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-context-only-continuity-preview-narrowed") => {
            let packet =
                seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_context_only_continuity_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries(),
                seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_reconnect_posture_beta_narrowed(),
                seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_context_only_continuity_preview_narrowed(),
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
    packet: &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
