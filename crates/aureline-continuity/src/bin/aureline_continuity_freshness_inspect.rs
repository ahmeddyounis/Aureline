//! CLI/headless explain for the continuity-proof freshness SLO dashboard.
//!
//! Reads a [`ContinuityFreshnessSloDashboard`] from stdin or a file, re-audits it
//! (its freshness windows, rerun-path declarations, stop-rule wiring, and the
//! promotion verdict), and emits a redaction-safe support-export projection as
//! JSON. The CLI renders the exact same freshness-state, stop-reason, and
//! promotion vocabulary as the shiproom, release-center, docs/public-truth, and
//! support-center surfaces, and carries no raw drill logs or backup bytes.

use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

use aureline_continuity::{
    audit_continuity_freshness_slo_dashboard, ContinuityFreshnessSloDashboard,
    ContinuityFreshnessSloSupportExport,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        fs::read_to_string(&args[1]).unwrap_or_else(|e| {
            eprintln!("failed to read {}: {}", args[1], e);
            process::exit(1);
        })
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap_or_else(|e| {
            eprintln!("failed to read stdin: {}", e);
            process::exit(1);
        });
        buf
    };

    let dashboard: ContinuityFreshnessSloDashboard =
        serde_json::from_str(&input).unwrap_or_else(|e| {
            eprintln!("failed to parse continuity freshness-SLO dashboard: {}", e);
            process::exit(1);
        });

    let defects = audit_continuity_freshness_slo_dashboard(&dashboard);
    if !defects.is_empty() {
        eprintln!("validation failed:");
        for defect in &defects {
            eprintln!(
                "  [{}] {}: {}",
                defect.defect_id, defect.defect_kind_token, defect.note
            );
        }
        process::exit(1);
    }

    let export = ContinuityFreshnessSloSupportExport::from_dashboard(
        format!("{}:cli-inspect", dashboard.dashboard_id),
        dashboard.generated_at.clone(),
        dashboard,
    );
    match serde_json::to_string_pretty(&export) {
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("failed to serialize support export: {}", e);
            process::exit(1);
        }
    }
}
