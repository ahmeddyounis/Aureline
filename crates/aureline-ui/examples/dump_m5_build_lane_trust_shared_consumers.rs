//! Headless emitter for the M5 build-lane-trust shared-consumer parity packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-build-lane-trust-shared-consumers-proof/`, its matrix CSV, the Markdown summary, and
//! the narrowed fixtures under `fixtures/release/m5-build-lane-trust-shared-consumers/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_build_lane_trust_shared_consumers -- support-export
//! cargo run -p aureline-ui --example dump_m5_build_lane_trust_shared_consumers -- report
//! cargo run -p aureline-ui --example dump_m5_build_lane_trust_shared_consumers -- csv
//! cargo run -p aureline-ui --example dump_m5_build_lane_trust_shared_consumers -- fixture-compact-remote-narrowed
//! cargo run -p aureline-ui --example dump_m5_build_lane_trust_shared_consumers -- fixture-exported-redaction-narrowed
//! cargo run -p aureline-ui --example dump_m5_build_lane_trust_shared_consumers -- validate
//! ```

use aureline_ui::m5_build_lane_trust_shared_consumers_one_registry_across_surfaces::{
    seeded_m5_build_lane_trust_shared_consumers,
    seeded_m5_build_lane_trust_shared_consumers_compact_remote_narrowed,
    seeded_m5_build_lane_trust_shared_consumers_exported_redaction_narrowed,
    M5BuildLaneTrustSharedConsumersPacket,
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
            let packet = seeded_m5_build_lane_trust_shared_consumers();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_build_lane_trust_shared_consumers().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_build_lane_trust_shared_consumers().render_matrix_csv()
            );
        }
        Some("fixture-compact-remote-narrowed") => {
            let packet = seeded_m5_build_lane_trust_shared_consumers_compact_remote_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-exported-redaction-narrowed") => {
            let packet = seeded_m5_build_lane_trust_shared_consumers_exported_redaction_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_build_lane_trust_shared_consumers(),
                seeded_m5_build_lane_trust_shared_consumers_compact_remote_narrowed(),
                seeded_m5_build_lane_trust_shared_consumers_exported_redaction_narrowed(),
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
    packet: &M5BuildLaneTrustSharedConsumersPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "build-lane-trust shared-consumer packet failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
