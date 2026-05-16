//! Headless inspector for the secret-broker beta page.
//!
//! The bin emits the same audited records consumed by the admin/settings
//! center, support-export wrapper, shell summary, diagnostics views, and
//! fixture replay.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- handle-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- consumer-audit
//! cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- drill-raw-secret-material
//! cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- drill-managed-authority-missing-signature
//! cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- drill-consumer-audit-missing
//! cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- drill-denied-audit-missing-reason
//! ```

use aureline_shell::secret_broker_beta::{
    audit_secret_broker_beta_page, seeded_secret_broker_beta_page,
    validate_secret_broker_beta_page, SecretBrokerBetaPage, SecretBrokerBetaRenderSummary,
    SecretBrokerBetaSummary, SecretBrokerBetaSupportExport, VaultSignatureStateClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_secret_broker_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("handle-rows") => print_json(&page.handle_rows)?,
        Some("consumer-audit") => print_json(&page.consumer_audit)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = SecretBrokerBetaSupportExport::from_page(
                "secret-broker-beta:support-export:001",
                "2026-05-16T05:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("summary") => {
            let summary = SecretBrokerBetaRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("drill-raw-secret-material") => {
            let mut page = page;
            page.handle_rows[0].raw_secret_material_present = true;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-managed-authority-missing-signature") => {
            let mut page = page;
            if let Some(row) = page
                .handle_rows
                .iter_mut()
                .find(|row| row.vault_binding.vault_adapter.is_managed_authority())
            {
                row.vault_binding.signature_blob_ref.clear();
                row.vault_binding.signature_state =
                    VaultSignatureStateClass::NotRequiredLocalOrigin;
                row.vault_binding.signature_state_token =
                    row.vault_binding.signature_state.as_str().to_owned();
            }
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-consumer-audit-missing") => {
            let mut page = page;
            let removed_row_id = page.handle_rows[0].secret_broker_row_id.clone();
            page.consumer_audit
                .retain(|event| event.secret_broker_row_ref != removed_row_id);
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-denied-audit-missing-reason") => {
            let mut page = page;
            if let Some(event) = page
                .consumer_audit
                .iter_mut()
                .find(|event| event.outcome.is_denial())
            {
                event.denial_note = None;
            }
            print_json(&rebuild_with_defects(page))?;
        }
        Some("validate") => match validate_secret_broker_beta_page(&page) {
            Ok(()) => println!("ok"),
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} subject_id={} field={} note={}",
                        defect.defect_kind_token, defect.subject_id, defect.field, defect.note,
                    );
                }
                std::process::exit(3);
            }
        },
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}

fn rebuild_with_defects(mut page: SecretBrokerBetaPage) -> SecretBrokerBetaPage {
    page.defects = audit_secret_broker_beta_page(&page.handle_rows, &page.consumer_audit);
    page.summary = SecretBrokerBetaSummary::from_records(
        &page.handle_rows,
        &page.consumer_audit,
        &page.defects,
    );
    page
}
