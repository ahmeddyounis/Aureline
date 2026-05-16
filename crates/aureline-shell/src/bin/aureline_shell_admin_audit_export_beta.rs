//! Headless inspector for the admin-audit export beta page.
//!
//! The bin emits the same audited records consumed by the admin/settings
//! center, support-export wrapper, shell summary, diagnostics views, and
//! fixture replay.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- provisioning-events
//! cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- policy-bundle-history
//! cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- entitlement-changes
//! cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- validate
//! ```

use aureline_shell::admin_audit_export_beta::{
    audit_admin_audit_export_beta_page, seeded_admin_audit_export_beta_page,
    validate_admin_audit_export_beta_page, AdminAuditExportBetaPage,
    AdminAuditExportBetaRenderSummary, AdminAuditExportBetaSummary,
    AdminAuditExportBetaSupportExport, PolicyBundleTransitionClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_admin_audit_export_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("provisioning-events") => print_json(&page.provisioning_events)?,
        Some("policy-bundle-history") => print_json(&page.policy_bundle_history)?,
        Some("entitlement-changes") => print_json(&page.entitlement_changes)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = AdminAuditExportBetaSupportExport::from_page(
                "support-export:admin-audit:001",
                "2026-05-16T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("summary") => {
            let summary = AdminAuditExportBetaRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("drill-managed-source-missing-signature") => {
            let mut page = page;
            page.provisioning_events[0]
                .provenance
                .signature_blob_ref
                .clear();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-history-missing-predecessor") => {
            let mut page = page;
            if let Some(event) = page
                .policy_bundle_history
                .iter_mut()
                .find(|event| event.transition == PolicyBundleTransitionClass::ReplacedBySuccessor)
            {
                event.replaces_pack_id.clear();
            }
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-public-fallback") => {
            let mut page = page;
            page.provisioning_events[0].no_public_endpoint_fallback = false;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("validate") => match validate_admin_audit_export_beta_page(&page) {
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

fn rebuild_with_defects(mut page: AdminAuditExportBetaPage) -> AdminAuditExportBetaPage {
    page.defects = audit_admin_audit_export_beta_page(
        &page.provisioning_events,
        &page.policy_bundle_history,
        &page.entitlement_changes,
    );
    page.summary = AdminAuditExportBetaSummary::from_records(
        &page.provisioning_events,
        &page.policy_bundle_history,
        &page.entitlement_changes,
        &page.defects,
    );
    page
}
