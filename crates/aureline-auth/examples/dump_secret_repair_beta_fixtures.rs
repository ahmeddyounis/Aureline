//! Emits the seeded secret-repair beta fixtures.
//!
//! The example is a one-shot helper that prints either the seeded page, one
//! of the per-record arrays, the support-export wrapper, or a named drill so
//! reviewer-facing fixtures can be regenerated:
//!
//! ```sh
//! cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- page
//! cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- lock-state-rows
//! cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- denied-projection-rows
//! cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- repair-events
//! cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- defects
//! cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- support-export
//! cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- drill-plaintext-fallback-attempted
//! cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- drill-repair-action-missing
//! cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- drill-store-lock-denial-unlinked
//! cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- drill-terminal-outcome-missing-resolved-at
//! ```

use aureline_auth::{
    audit_secret_repair_beta_page, seeded_secret_repair_beta_page, DenialReasonClass,
    RepairActionClass, RepairOutcomeClass, SecretRepairBetaPage, SecretRepairBetaSummary,
    SecretRepairBetaSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_secret_repair_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("lock-state-rows") => print_json(&page.lock_state_rows)?,
        Some("denied-projection-rows") => print_json(&page.denied_projection_rows)?,
        Some("repair-events") => print_json(&page.repair_events)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = SecretRepairBetaSupportExport::from_page(
                "secret-repair-beta:support-export:001",
                "2026-05-16T05:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-plaintext-fallback-attempted") => {
            let mut page = page;
            page.lock_state_rows[0].plaintext_fallback_attempted = true;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-repair-action-missing") => {
            let mut page = page;
            page.lock_state_rows[0].repair_action = RepairActionClass::NoneRequired;
            page.lock_state_rows[0].repair_action_token =
                RepairActionClass::NoneRequired.as_str().to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-store-lock-denial-unlinked") => {
            let mut page = page;
            if let Some(row) = page
                .denied_projection_rows
                .iter_mut()
                .find(|row| row.denial_reason == DenialReasonClass::BackingStoreLocked)
            {
                row.linked_lock_state_row_ref = None;
            }
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-terminal-outcome-missing-resolved-at") => {
            let mut page = page;
            if let Some(event) = page
                .repair_events
                .iter_mut()
                .find(|event| event.outcome == RepairOutcomeClass::Resolved)
            {
                event.resolved_at = None;
            }
            print_json(&rebuild_with_defects(page))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}

fn rebuild_with_defects(mut page: SecretRepairBetaPage) -> SecretRepairBetaPage {
    page.defects = audit_secret_repair_beta_page(
        &page.lock_state_rows,
        &page.denied_projection_rows,
        &page.repair_events,
    );
    page.summary = SecretRepairBetaSummary::from_records(
        &page.lock_state_rows,
        &page.denied_projection_rows,
        &page.repair_events,
        &page.defects,
    );
    page
}
