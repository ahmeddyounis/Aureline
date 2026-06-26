//! Headless emitter for the M5 event-class coverage catalog.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/a11y/m5-event-coverage-proof/` and the narrowed fixtures under
//! `fixtures/a11y/m5-event-coverage/`. Editor, terminal, debug, review,
//! collaboration, AI, and notebook surfaces route their dynamic events through this
//! coverage catalog so each high-churn workflow narrates its meaning-changing state
//! transitions — diagnostics, completion/snippet sessions, run/debug/test state,
//! terminal command boundaries, collaboration changes, AI/review milestones, and
//! stale/degraded truth — with a concise identity plus blocked/degraded reason and a
//! reopenable durable fallback, rather than per-surface improvised prose.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_event_coverage -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_event_coverage -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_event_coverage -- fixture-proof-stale-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_event_coverage -- fixture-bridge-unavailable-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_event_coverage -- validate
//! ```

use aureline_shell::accessibility::events::{
    seeded_m5_event_coverage_catalog, seeded_m5_event_coverage_catalog_bridge_unavailable_narrowed,
    seeded_m5_event_coverage_catalog_proof_stale_narrowed, M5EventCoverageCatalogPacket,
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
            let packet = seeded_m5_event_coverage_catalog();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_event_coverage_catalog().render_markdown_summary()
            );
        }
        Some("fixture-proof-stale-narrowed") => {
            let packet = seeded_m5_event_coverage_catalog_proof_stale_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-bridge-unavailable-narrowed") => {
            let packet = seeded_m5_event_coverage_catalog_bridge_unavailable_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_event_coverage_catalog(),
                seeded_m5_event_coverage_catalog_proof_stale_narrowed(),
                seeded_m5_event_coverage_catalog_bridge_unavailable_narrowed(),
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

fn assert_valid(packet: &M5EventCoverageCatalogPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "event coverage catalog failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
