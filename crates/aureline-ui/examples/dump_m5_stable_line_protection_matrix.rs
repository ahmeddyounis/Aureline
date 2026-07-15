//! Headless emitter for the frozen M5 stable-line-protection matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-stable-line-correction-reports/`, its matrix CSV, the Markdown design report at
//! `artifacts/program/m5-stable-line-protection-matrix.md`, the stable-line-protection dashboard at
//! `dashboards/m5-stable-line-health.json`, and the narrowed fixtures under
//! `fixtures/release/m5-stable-line-protection/`. The release, help, support, public-proof, shiproom, and
//! program-governance surfaces read this matrix so no shipping line drifts on stale evidence or frozen launch
//! bundles, supported-line defects stay owned and resolved within SLA, backport decisions stay documented
//! rather than tribal memory, LTS remains a checked-in decision packet backed by current rollback and support
//! evidence, and support language never outruns current refresh and correction proof.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_matrix -- dashboard
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_matrix -- fixture-bundle-currentness-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_matrix -- fixture-lts-candidate-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_matrix -- validate
//! ```

use aureline_ui::m5_stable_line_protection_matrix::{
    seeded_m5_stable_line_protection_matrix,
    seeded_m5_stable_line_protection_matrix_bundle_currentness_beta_narrowed,
    seeded_m5_stable_line_protection_matrix_lts_candidate_preview_narrowed,
    M5StableLineProtectionMatrixPacket,
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
            let packet = seeded_m5_stable_line_protection_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_stable_line_protection_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_stable_line_protection_matrix().render_matrix_csv()
            );
        }
        Some("dashboard") => {
            println!(
                "{}",
                seeded_m5_stable_line_protection_matrix().render_dashboard_json()
            );
        }
        Some("fixture-bundle-currentness-beta-narrowed") => {
            let packet = seeded_m5_stable_line_protection_matrix_bundle_currentness_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-lts-candidate-preview-narrowed") => {
            let packet = seeded_m5_stable_line_protection_matrix_lts_candidate_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_stable_line_protection_matrix(),
                seeded_m5_stable_line_protection_matrix_bundle_currentness_beta_narrowed(),
                seeded_m5_stable_line_protection_matrix_lts_candidate_preview_narrowed(),
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
    packet: &M5StableLineProtectionMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
