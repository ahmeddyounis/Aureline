//! Headless inspector for the store-lock / external-root recovery report.
//!
//! The bin emits the same recovery records consumed by the live recovery
//! affordances, the Help/About and docs rails, and the support inspector; the
//! markdown artifact under
//! `artifacts/platform/m5-store-lock-and-external-root-recovery.md`; the
//! support-export wrapper; the four per-incident case exports; and the CI gate
//! `tools/ci/m5/store_lock_and_external_root_check.py`. It is the only
//! mint-from-truth path for the JSON fixtures checked in under
//! `fixtures/platform/m5-store-lock-and-missing-root/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- report
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- support-export
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- cases
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- case missing_root
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- report-md
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- compact
//! cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- validate
//! ```

use aureline_auth::m5_store_lock_and_external_root_recovery::{
    seeded_store_lock_recovery_case_exports, seeded_store_lock_recovery_report,
    validate_store_lock_recovery_report, StoreLockRecoverySupportExport,
    STORE_LOCK_RECOVERY_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_store_lock_recovery_report();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            let export = StoreLockRecoverySupportExport::from_report(
                STORE_LOCK_RECOVERY_SUPPORT_EXPORT_ID,
                report,
            );
            print_json(&export)?;
        }
        Some("cases") => {
            print_json(&seeded_store_lock_recovery_case_exports())?;
        }
        Some("case") => {
            let label = args.get(1).map(String::as_str).ok_or(
                "usage: aureline_auth_m5_store_lock_and_external_root_recovery case <credential_store_locked|trust_store_drift|missing_root|root_returned>",
            )?;
            let exports = seeded_store_lock_recovery_case_exports();
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
        Some("validate") => match validate_store_lock_recovery_report(&report) {
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
