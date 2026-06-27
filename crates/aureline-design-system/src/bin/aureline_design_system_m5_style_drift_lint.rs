//! Headless emitter for the M5 design-system style-drift-lint lane.
//!
//! The bin is the only mint-from-truth path for the checked-in lint-report fixtures under
//! `fixtures/ui/m5-style-drift-lint/` (the conformant report plus the drift, waived, and
//! expired-waiver drills), the lint-outcome proof at
//! `artifacts/release/m5-design-system-proof/style-drift-lint-outcome.json`, and the release packet
//! at `artifacts/release/m5-design-system-proof/style-drift-lint-release.json`. Shell code,
//! docs/help, QA, and the release center consume the lint result this bin mints, so the gate
//! decision reads from one governed source.
//!
//! The `lint` and `lint-drift` subcommands print the outcome and exit non-zero when the gate blocks,
//! so a protected-surface drift fails CI directly.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_style_drift_lint -- report
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_style_drift_lint -- report-drift
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_style_drift_lint -- report-waived
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_style_drift_lint -- report-expired-waiver
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_style_drift_lint -- outcome
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_style_drift_lint -- release-packet
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_style_drift_lint -- lint
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_style_drift_lint -- validate
//! ```

use aureline_design_system::m5_style_drift_lint::{
    seeded_m5_style_drift_lint_report, seeded_m5_style_drift_lint_report_drift,
    seeded_m5_style_drift_lint_report_expired_waiver, seeded_m5_style_drift_lint_report_waived,
    M5StyleDriftLintReport,
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
        Some("report") | None => {
            let report = seeded_m5_style_drift_lint_report();
            assert_valid(&report)?;
            println!("{}", report.export_safe_json());
        }
        Some("report-drift") => {
            let report = seeded_m5_style_drift_lint_report_drift();
            assert_valid(&report)?;
            println!("{}", report.export_safe_json());
        }
        Some("report-waived") => {
            let report = seeded_m5_style_drift_lint_report_waived();
            assert_valid(&report)?;
            println!("{}", report.export_safe_json());
        }
        Some("report-expired-waiver") => {
            let report = seeded_m5_style_drift_lint_report_expired_waiver();
            assert_valid(&report)?;
            println!("{}", report.export_safe_json());
        }
        Some("outcome") => {
            let report = seeded_m5_style_drift_lint_report();
            assert_valid(&report)?;
            println!("{}", report.lint().export_safe_json());
        }
        Some("release-packet") => {
            let report = seeded_m5_style_drift_lint_report();
            assert_valid(&report)?;
            println!("{}", report.release_packet().export_safe_json());
        }
        Some("lint") => {
            // Lint the conformant report and exit non-zero if the gate blocks.
            let report = seeded_m5_style_drift_lint_report();
            assert_valid(&report)?;
            let outcome = report.lint();
            println!("{}", outcome.export_safe_json());
            if outcome.blocks_stable_promotion() {
                return Err(format!(
                    "style drift gate blocks stable promotion for surfaces: {}",
                    outcome.blocked_surface_ids().join(",")
                )
                .into());
            }
        }
        Some("lint-drift") => {
            // Lint the drift drill; this is expected to block and exit non-zero.
            let report = seeded_m5_style_drift_lint_report_drift();
            assert_valid(&report)?;
            let outcome = report.lint();
            println!("{}", outcome.export_safe_json());
            if outcome.blocks_stable_promotion() {
                return Err(format!(
                    "style drift gate blocks stable promotion for surfaces: {}",
                    outcome.blocked_surface_ids().join(",")
                )
                .into());
            }
        }
        Some("validate") => {
            for report in [
                seeded_m5_style_drift_lint_report(),
                seeded_m5_style_drift_lint_report_drift(),
                seeded_m5_style_drift_lint_report_waived(),
                seeded_m5_style_drift_lint_report_expired_waiver(),
            ] {
                assert_valid(&report)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(report: &M5StyleDriftLintReport) -> Result<(), Box<dyn std::error::Error>> {
    let violations = report.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "style drift lint report failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
