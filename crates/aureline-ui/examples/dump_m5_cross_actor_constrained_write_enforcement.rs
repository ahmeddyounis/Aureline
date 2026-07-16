//! Headless emitter for the M5 cross-actor constrained-write enforcement packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/support/m5-cross-actor-constrained-write-enforcement/`, its matrix CSV, the Markdown summary, and
//! the narrowed fixtures under `fixtures/editor/m5-cross-actor-constrained-write-enforcement/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_cross_actor_constrained_write_enforcement -- support-export
//! cargo run -p aureline-ui --example dump_m5_cross_actor_constrained_write_enforcement -- report
//! cargo run -p aureline-ui --example dump_m5_cross_actor_constrained_write_enforcement -- csv
//! cargo run -p aureline-ui --example dump_m5_cross_actor_constrained_write_enforcement -- fixture-fail-closed-narrowed
//! cargo run -p aureline-ui --example dump_m5_cross_actor_constrained_write_enforcement -- fixture-export-redacted-narrowed
//! cargo run -p aureline-ui --example dump_m5_cross_actor_constrained_write_enforcement -- validate
//! ```

use aureline_ui::m5_cross_actor_constrained_write_enforcement::{
    seeded_m5_cross_actor_constrained_write_enforcement,
    seeded_m5_cross_actor_constrained_write_enforcement_export_redacted_narrowed,
    seeded_m5_cross_actor_constrained_write_enforcement_fail_closed_narrowed,
    M5CrossActorConstrainedWriteEnforcementPacket,
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
            let packet = seeded_m5_cross_actor_constrained_write_enforcement();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_cross_actor_constrained_write_enforcement().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_cross_actor_constrained_write_enforcement().render_matrix_csv()
            );
        }
        Some("fixture-fail-closed-narrowed") => {
            let packet = seeded_m5_cross_actor_constrained_write_enforcement_fail_closed_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-export-redacted-narrowed") => {
            let packet =
                seeded_m5_cross_actor_constrained_write_enforcement_export_redacted_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_cross_actor_constrained_write_enforcement(),
                seeded_m5_cross_actor_constrained_write_enforcement_fail_closed_narrowed(),
                seeded_m5_cross_actor_constrained_write_enforcement_export_redacted_narrowed(),
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
    packet: &M5CrossActorConstrainedWriteEnforcementPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "cross-actor constrained-write enforcement packet failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
