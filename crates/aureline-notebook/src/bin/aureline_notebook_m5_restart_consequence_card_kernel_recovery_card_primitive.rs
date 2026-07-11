//! Headless emitter for the M5 restart-consequence-card / kernel-recovery-card controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-restart-consequence-card-kernel-recovery-card-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-restart-consequence-card-kernel-recovery-card-controls/`. The notebook,
//! debug-bridge, review, support, and companion-handoff surfaces read these components so one
//! restart consequence card names a restart / interrupt / shutdown action, what state it preserves
//! (notebook source, prior outputs) and loses (live variables, debugger frames, session), and its
//! rerun requirement before restart — and one kernel recovery card names where a kernel's recovery
//! stands and offers reconnect / restart-clean / choose-another-kernel / open-inspect-only /
//! export-evidence recovery — so a restart that loses live state never reads as one that preserved
//! it and no recovery ever implies a hidden rerun.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_restart_consequence_card_kernel_recovery_card_primitive -- support-export
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_restart_consequence_card_kernel_recovery_card_primitive -- report
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_restart_consequence_card_kernel_recovery_card_primitive -- csv
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_restart_consequence_card_kernel_recovery_card_primitive -- fixture-restart-consequence-card-lost-state
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_restart_consequence_card_kernel_recovery_card_primitive -- fixture-kernel-recovery-card-clean-session
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_restart_consequence_card_kernel_recovery_card_primitive -- validate
//! ```

use aureline_notebook::implement_restart_consequence_cards_and_kernel_recovery_cards_with_preserved_state_lost_state_reconnect_restart_clean_choose_another_kernel_actions_and_no_hidden_rerun_truth_across_claimed_m5_notebook_restore_and_failure_flows::{
    seeded_restart_consequence_card_kernel_recovery_card_controls,
    seeded_restart_consequence_card_kernel_recovery_card_controls_kernel_recovery_card_clean_session,
    seeded_restart_consequence_card_kernel_recovery_card_controls_restart_consequence_card_lost_state,
    RestartConsequenceCardKernelRecoveryCardControlsPacket,
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
            let packet = seeded_restart_consequence_card_kernel_recovery_card_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_restart_consequence_card_kernel_recovery_card_controls()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_restart_consequence_card_kernel_recovery_card_controls().render_matrix_csv()
            );
        }
        Some("fixture-restart-consequence-card-lost-state") => {
            let packet =
                seeded_restart_consequence_card_kernel_recovery_card_controls_restart_consequence_card_lost_state();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-kernel-recovery-card-clean-session") => {
            let packet =
                seeded_restart_consequence_card_kernel_recovery_card_controls_kernel_recovery_card_clean_session();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_restart_consequence_card_kernel_recovery_card_controls(),
                seeded_restart_consequence_card_kernel_recovery_card_controls_restart_consequence_card_lost_state(),
                seeded_restart_consequence_card_kernel_recovery_card_controls_kernel_recovery_card_clean_session(),
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
    packet: &RestartConsequenceCardKernelRecoveryCardControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "restart consequence card kernel recovery card primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
