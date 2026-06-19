//! CLI/headless explain for backup/restore/failover continuity packets.
//!
//! Reads a [`BackupRestoreFailoverPage`] from stdin or a file, re-audits it
//! (including its stored surface projections), and emits a redaction-safe
//! support-export projection as JSON. The CLI renders the exact same
//! restore-identity and partial-loss vocabulary as the release-center, shiproom,
//! support-center, partner-qualification, and public claim-manifest surfaces,
//! and carries no raw provider payloads.

use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

use aureline_continuity::{
    audit_backup_restore_failover_page, BackupRestoreFailoverPage,
    BackupRestoreFailoverSupportExport,
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

    let page: BackupRestoreFailoverPage = serde_json::from_str(&input).unwrap_or_else(|e| {
        eprintln!("failed to parse backup/restore/failover page: {}", e);
        process::exit(1);
    });

    let defects = audit_backup_restore_failover_page(&page);
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

    let export = BackupRestoreFailoverSupportExport::from_page(
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
