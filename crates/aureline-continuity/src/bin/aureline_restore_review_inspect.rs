//! CLI/headless explain for restore-from-backup reviews.
//!
//! Reads a [`RestoreReviewPage`] from stdin or a file, re-audits it (including its
//! stored surface projections), and emits a redaction-safe support-export
//! projection as JSON. The CLI renders the exact same restore-identity,
//! replay-fence, and compare/export vocabulary as the service-health,
//! support-center, managed-action-sheet, release-center, and public claim-manifest
//! surfaces, and carries no raw provider payloads.

use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

use aureline_continuity::{
    audit_restore_review_page, RestoreReviewPage, RestoreReviewSupportExport,
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

    let page: RestoreReviewPage = serde_json::from_str(&input).unwrap_or_else(|e| {
        eprintln!("failed to parse restore-review page: {}", e);
        process::exit(1);
    });

    let defects = audit_restore_review_page(&page);
    if !defects.is_empty() {
        eprintln!("validation failed:");
        for defect in &defects {
            eprintln!(
                "  [{}] {}: {}",
                defect.defect_id, defect.narrow_reason_token, defect.note
            );
        }
        process::exit(1);
    }

    let export = RestoreReviewSupportExport::from_page(
        format!("{}:cli-inspect", page.page_id),
        page.generated_at.clone(),
        page,
    );
    match serde_json::to_string_pretty(&export) {
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("failed to serialize support export: {}", e);
            process::exit(1);
        }
    }
}
