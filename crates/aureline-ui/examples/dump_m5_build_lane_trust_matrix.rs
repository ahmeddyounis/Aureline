//! Headless emitter for the frozen M5 build-lane-trust matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-exact-build-supportability-proof/`, its matrix CSV, the Markdown design report at
//! `artifacts/release/m5-build-lane-trust-matrix.md`, and the narrowed fixtures under
//! `fixtures/release/m5-clean-room-rebuild/`. The release-center, shiproom, diagnostics, admin, docs, and
//! support surfaces read this matrix so contributor lanes never publish release artifacts, remote-cache hits
//! are never treated as reproducibility proof, sidecars never drift from the binary build identity, clean-room
//! parity is never overclaimed, and non-hermetic inputs never hide behind green publication rows.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_build_lane_trust_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_build_lane_trust_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_build_lane_trust_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_build_lane_trust_matrix -- fixture-release-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_build_lane_trust_matrix -- fixture-emergency-hotfix-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_build_lane_trust_matrix -- validate
//! ```

use aureline_ui::m5_build_lane_trust_matrix::{
    seeded_m5_build_lane_trust_matrix,
    seeded_m5_build_lane_trust_matrix_emergency_hotfix_preview_narrowed,
    seeded_m5_build_lane_trust_matrix_release_beta_narrowed, M5BuildLaneTrustMatrixPacket,
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
            let packet = seeded_m5_build_lane_trust_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_build_lane_trust_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_build_lane_trust_matrix().render_matrix_csv()
            );
        }
        Some("fixture-release-beta-narrowed") => {
            let packet = seeded_m5_build_lane_trust_matrix_release_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-emergency-hotfix-preview-narrowed") => {
            let packet = seeded_m5_build_lane_trust_matrix_emergency_hotfix_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_build_lane_trust_matrix(),
                seeded_m5_build_lane_trust_matrix_release_beta_narrowed(),
                seeded_m5_build_lane_trust_matrix_emergency_hotfix_preview_narrowed(),
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

fn assert_valid(packet: &M5BuildLaneTrustMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
