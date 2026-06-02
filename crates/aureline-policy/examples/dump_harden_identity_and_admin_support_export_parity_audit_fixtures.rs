//! Emits the seeded hardened identity-admin fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_harden_identity_and_admin_support_export_parity_audit_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_harden_identity_and_admin_support_export_parity_audit_fixtures -- rows
//! cargo run -q -p aureline-policy --example dump_harden_identity_and_admin_support_export_parity_audit_fixtures -- defects
//! cargo run -q -p aureline-policy --example dump_harden_identity_and_admin_support_export_parity_audit_fixtures -- summary
//! cargo run -q -p aureline-policy --example dump_harden_identity_and_admin_support_export_parity_audit_fixtures -- support-export
//! cargo run -q -p aureline-policy --example dump_harden_identity_and_admin_support_export_parity_audit_fixtures -- drill-raw-secret-withdrawn
//! cargo run -q -p aureline-policy --example dump_harden_identity_and_admin_support_export_parity_audit_fixtures -- drill-missing-row-class-preview
//! cargo run -q -p aureline-policy --example dump_harden_identity_and_admin_support_export_parity_audit_fixtures -- drill-empty-provisioning-class-beta
//! cargo run -q -p aureline-policy --example dump_harden_identity_and_admin_support_export_parity_audit_fixtures -- drill-generic-failure-kind-beta
//! ```

use aureline_policy::{
    seeded_harden_identity_admin_page, HardenIdentityAdminPage, IdentityAdminRow,
    HardenIdentityAdminSupportExport, IdentityAdminRowClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_harden_identity_admin_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = HardenIdentityAdminSupportExport::from_page(
                "policy:harden-identity-admin:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-raw-secret-withdrawn") => {
            let mut rows: Vec<IdentityAdminRow> = page.rows;
            if let Some(row) = rows.first_mut() {
                row.raw_secret_or_private_material_excluded = false;
            }
            let drill = HardenIdentityAdminPage::new(
                "policy:harden-identity-admin:drill:raw-secret",
                "Drill — raw secret material in row (withdrawn)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&drill)?;
        }
        Some("drill-missing-row-class-preview") => {
            let rows: Vec<IdentityAdminRow> = page
                .rows
                .into_iter()
                .filter(|r| {
                    r.row_class == IdentityAdminRowClass::DirectoryProviderCard
                        || r.row_class == IdentityAdminRowClass::UserSeatLifecycle
                })
                .collect();
            let drill = HardenIdentityAdminPage::new(
                "policy:harden-identity-admin:drill:missing-row-class",
                "Drill — required row class missing (preview)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&drill)?;
        }
        Some("drill-empty-provisioning-class-beta") => {
            let mut rows: Vec<IdentityAdminRow> = page.rows;
            if let Some(row) = rows.iter_mut().find(|r| {
                r.row_class == IdentityAdminRowClass::DirectoryProviderCard
            }) {
                row.provisioning_class_token.clear();
            }
            let drill = HardenIdentityAdminPage::new(
                "policy:harden-identity-admin:drill:empty-provisioning-class",
                "Drill — empty provisioning class (beta)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&drill)?;
        }
        Some("drill-generic-failure-kind-beta") => {
            let mut rows: Vec<IdentityAdminRow> = page.rows;
            if let Some(row) = rows.iter_mut().find(|r| {
                r.row_class == IdentityAdminRowClass::ProvisioningFailureLog
            }) {
                row.failure_kind_token = "generic_error".to_owned();
                row.failure_kind = None;
            }
            let drill = HardenIdentityAdminPage::new(
                "policy:harden-identity-admin:drill:generic-failure-kind",
                "Drill — generic failure kind (beta)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&drill)?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
