//! CLI/headless inspect for service-health continuity pages.
//!
//! Reads a [`ServiceHealthContinuityPage`] from stdin or a file, validates it,
//! and emits a redaction-safe support export projection as JSON.

use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

use aureline_service_health::ServiceHealthContinuityPage;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        fs::read_to_string(&args[1]).unwrap_or_else(|e| {
            eprintln!("failed to read {}: {}", args[1], e);
            process::exit(1);
        })
    } else {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .unwrap_or_else(|e| {
                eprintln!("failed to read stdin: {}", e);
                process::exit(1);
            });
        buf
    };

    let page: ServiceHealthContinuityPage = serde_json::from_str(&input).unwrap_or_else(|e| {
        eprintln!("failed to parse service-health continuity page: {}", e);
        process::exit(1);
    });

    let report = page.validate();
    if !report.passed {
        eprintln!("validation failed:");
        for finding in &report.findings {
            eprintln!("  [{}] {:?}: {}", finding.check_id, finding.severity, finding.message);
        }
        process::exit(1);
    }

    let export = page.support_export_projection();
    match serde_json::to_string_pretty(&export) {
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("failed to serialize support export: {}", e);
            process::exit(1);
        }
    }
}
