//! Headless inspector for the auth-callback and deep-link review report.
//!
//! The bin emits the same review records consumed by the live shell entry
//! interstitials and the auth-recovery surface, the Help/About and docs rails,
//! and the support inspector; the markdown artifact under
//! `artifacts/platform/m5-auth-callback-and-deep-link.md`; the support-export
//! wrapper; the four per-incident case exports; and the CI gate
//! `tools/ci/m5/callback_and_deep_link_check.py`. It is the only mint-from-truth
//! path for the JSON fixtures checked in under
//! `fixtures/platform/m5-callback-and-deep-link/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- report
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- support-export
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- cases
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- case wrong_origin
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- report-md
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- compact
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- validate
//! ```

use aureline_auth::m5_callback_and_deep_link_review::{
    seeded_callback_review_case_exports, seeded_callback_review_report,
    validate_callback_review_report, CallbackReviewSupportExport,
    CALLBACK_REVIEW_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_callback_review_report();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            let export =
                CallbackReviewSupportExport::from_report(CALLBACK_REVIEW_SUPPORT_EXPORT_ID, report);
            print_json(&export)?;
        }
        Some("cases") => {
            print_json(&seeded_callback_review_case_exports())?;
        }
        Some("case") => {
            let label = args.get(1).map(String::as_str).ok_or(
                "usage: aureline_auth_m5_callback_and_deep_link_review case <wrong_origin|expired|stale|denied>",
            )?;
            let exports = seeded_callback_review_case_exports();
            let found = exports
                .into_iter()
                .find(|export| export.case_label == label)
                .ok_or_else(|| format!("unknown case label: {label}"))?;
            print_json(&found)?;
        }
        Some("report-md") => {
            print!("{}", report.render_markdown());
        }
        Some("compact") => {
            for line in report.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => match validate_callback_review_report(&report) {
            Ok(()) => {
                println!("ok");
            }
            Err(errors) => {
                for err in &errors {
                    eprintln!(
                        "error: {}",
                        serde_json::to_string(err).unwrap_or_else(|_| format!("{err:?}"))
                    );
                }
                std::process::exit(3);
            }
        },
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
