//! CLI/headless explain for the continuity certification report.
//!
//! Reads a [`ContinuityCertificationReport`] from stdin or a file, re-audits it
//! (required-dimension coverage, evidence-ref coherence, the shared-reference
//! drill guardrail, the local-core guardrail, and surface reuse), and emits a
//! redaction-safe support-export projection as JSON. The CLI renders the exact
//! same certification-verdict, narrow-reason, and dimension vocabulary as the
//! release-center, docs/public-truth, Help/About, service-health, and
//! support-center surfaces, and carries no raw drill logs or backup bytes.

use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

use aureline_continuity::{
    audit_continuity_certification_report, ContinuityCertificationReport,
    ContinuityCertificationSupportExport,
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

    let report: ContinuityCertificationReport = serde_json::from_str(&input).unwrap_or_else(|e| {
        eprintln!("failed to parse continuity certification report: {}", e);
        process::exit(1);
    });

    let defects = audit_continuity_certification_report(&report);
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

    let export = ContinuityCertificationSupportExport::from_report(
        format!("{}:cli-inspect", report.report_id),
        report.generated_at.clone(),
        report,
    );
    match serde_json::to_string_pretty(&export) {
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("failed to serialize support export: {}", e);
            process::exit(1);
        }
    }
}
