//! Headless emitter for the M5 surface-qualification packet.
//!
//! The bin is the only mint-from-truth path for the qualification support export and Markdown proof
//! checked in under `artifacts/release/m5-design-system-proof/`, the published dashboard at
//! `artifacts/design-system/m5-surface-qualification-dashboard.json`, and the
//! stale / token-drift / missing-manifest / waiver drill fixtures under
//! `fixtures/ui/m5-surface-qualification/`. Help/About, the release center, shiproom, support
//! exports, and the stable-claim matrix consume this packet so each claimed M5 surface either binds
//! current foundation, component, layout, and evidence contracts or is auto-narrowed / blocked
//! before Stable promotion, with the gap named rather than left invisible.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_surface_qualification -- support-export
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_surface_qualification -- dashboard
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_surface_qualification -- markdown
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_surface_qualification -- fixture-stale-narrowed
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_surface_qualification -- fixture-token-drift-narrowed
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_surface_qualification -- fixture-missing-manifest-blocked
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_surface_qualification -- fixture-waived-narrowed
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_surface_qualification -- validate
//! ```

use aureline_design_system::m5_surface_qualification::{
    seeded_m5_surface_qualification_packet,
    seeded_m5_surface_qualification_packet_missing_manifest_blocked,
    seeded_m5_surface_qualification_packet_stale_narrowed,
    seeded_m5_surface_qualification_packet_token_drift_narrowed,
    seeded_m5_surface_qualification_packet_waived_narrowed, M5SurfaceQualificationPacket,
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
            let packet = seeded_m5_surface_qualification_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("dashboard") => {
            let packet = seeded_m5_surface_qualification_packet();
            assert_valid(&packet)?;
            println!("{}", packet.dashboard_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_surface_qualification_packet().render_markdown_summary()
            );
        }
        Some("fixture-stale-narrowed") => {
            let packet = seeded_m5_surface_qualification_packet_stale_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-token-drift-narrowed") => {
            let packet = seeded_m5_surface_qualification_packet_token_drift_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-missing-manifest-blocked") => {
            let packet = seeded_m5_surface_qualification_packet_missing_manifest_blocked();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-waived-narrowed") => {
            let packet = seeded_m5_surface_qualification_packet_waived_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_surface_qualification_packet(),
                seeded_m5_surface_qualification_packet_stale_narrowed(),
                seeded_m5_surface_qualification_packet_token_drift_narrowed(),
                seeded_m5_surface_qualification_packet_missing_manifest_blocked(),
                seeded_m5_surface_qualification_packet_waived_narrowed(),
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

fn assert_valid(packet: &M5SurfaceQualificationPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "qualification packet failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
