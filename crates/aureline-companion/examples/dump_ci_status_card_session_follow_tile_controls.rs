//! Headless emitter for the M5 CI-status-card / session-follow-tile controls.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-ci-status-card-session-follow-tile-proof/`, its matrix CSV, the
//! Markdown summary, and the scenario fixtures under
//! `fixtures/ui/m5-ci-status-card-session-follow-tile-controls/`. The CI-status and
//! session-follow UIs read this packet so the first glance at a pipeline result or a followed
//! session names the object, the scope, the freshness, the provider/source, and the
//! companion-versus-desktop capability boundary before a tap.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_ci_status_card_session_follow_tile_controls -- support-export
//! cargo run -p aureline-companion --example dump_ci_status_card_session_follow_tile_controls -- report
//! cargo run -p aureline-companion --example dump_ci_status_card_session_follow_tile_controls -- csv
//! cargo run -p aureline-companion --example dump_ci_status_card_session_follow_tile_controls -- fixture-ci-status-card-stale
//! cargo run -p aureline-companion --example dump_ci_status_card_session_follow_tile_controls -- fixture-session-follow-tile-not-joinable
//! cargo run -p aureline-companion --example dump_ci_status_card_session_follow_tile_controls -- validate
//! ```

use aureline_companion::implement_ci_status_cards_and_session_follow_tiles_with_provider_source_run_or_session_identity_stale_state_labeling_and_follow_or_handoff_continuity::{
    seeded_ci_status_card_session_follow_tile_controls,
    seeded_ci_status_card_session_follow_tile_controls_ci_status_card_stale,
    seeded_ci_status_card_session_follow_tile_controls_session_follow_tile_not_joinable,
    CiStatusCardSessionFollowTileControlsPacket,
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
            let packet = seeded_ci_status_card_session_follow_tile_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_ci_status_card_session_follow_tile_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_ci_status_card_session_follow_tile_controls().render_matrix_csv()
            );
        }
        Some("fixture-ci-status-card-stale") => {
            let packet = seeded_ci_status_card_session_follow_tile_controls_ci_status_card_stale();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-session-follow-tile-not-joinable") => {
            let packet =
                seeded_ci_status_card_session_follow_tile_controls_session_follow_tile_not_joinable(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_ci_status_card_session_follow_tile_controls(),
                seeded_ci_status_card_session_follow_tile_controls_ci_status_card_stale(),
                seeded_ci_status_card_session_follow_tile_controls_session_follow_tile_not_joinable(
                ),
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
    packet: &CiStatusCardSessionFollowTileControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls failed validation: {}", tokens.join(",")).into())
    }
}
