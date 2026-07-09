//! Headless emitter for the M5 companion component-consumer adoption lane.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-companion-component-consumer-proof/`, its matrix CSV, the Markdown
//! report, and the narrowed fixtures under `fixtures/ui/m5-companion-component-consumers/`. The
//! companion consumer surfaces read this packet so object identity, workspace/repo client scope,
//! freshness, the companion-versus-desktop capability boundary, severity, and the exact
//! desktop-handoff target stay one truth across every claimed M5 companion surface.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_companion_component_consumers -- support-export
//! cargo run -p aureline-companion --example dump_companion_component_consumers -- report
//! cargo run -p aureline-companion --example dump_companion_component_consumers -- csv
//! cargo run -p aureline-companion --example dump_companion_component_consumers -- fixture-advisory-beta-narrowed
//! cargo run -p aureline-companion --example dump_companion_component_consumers -- fixture-handoff-preview-narrowed
//! cargo run -p aureline-companion --example dump_companion_component_consumers -- validate
//! ```

use aureline_companion::add_shared_inbox_review_ci_session_follow_incident_advisory_and_browser_or_desktop_handoff_consumers_so_companion_components_keep_scope_freshness_and_desktop_required_language_aligned_across_claimed_m5_profiles::{
    seeded_m5_companion_component_consumer_advisory_beta_narrowed,
    seeded_m5_companion_component_consumer_handoff_preview_narrowed,
    seeded_m5_companion_component_consumer_packet, M5CompanionComponentConsumerPacket,
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
            let packet = seeded_m5_companion_component_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_companion_component_consumer_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_companion_component_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-advisory-beta-narrowed") => {
            let packet = seeded_m5_companion_component_consumer_advisory_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-handoff-preview-narrowed") => {
            let packet = seeded_m5_companion_component_consumer_handoff_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_companion_component_consumer_packet(),
                seeded_m5_companion_component_consumer_advisory_beta_narrowed(),
                seeded_m5_companion_component_consumer_handoff_preview_narrowed(),
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
    packet: &M5CompanionComponentConsumerPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("consumers failed validation: {}", tokens.join(",")).into())
    }
}
