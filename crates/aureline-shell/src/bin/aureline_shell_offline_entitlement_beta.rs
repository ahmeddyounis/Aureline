//! Headless inspector for the offline policy-bundle and entitlement
//! verifier beta page.
//!
//! The bin emits the same audited records consumed by the admin/settings
//! center, support-export wrapper, shell summary, diagnostics views, and
//! fixture replay.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_offline_entitlement_beta -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_offline_entitlement_beta -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_offline_entitlement_beta -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_offline_entitlement_beta -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_offline_entitlement_beta -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_offline_entitlement_beta -- validate
//! ```

use aureline_shell::offline_entitlement_beta::{
    audit_offline_entitlement_verifier_beta_rows, seeded_offline_entitlement_verifier_beta_page,
    validate_offline_entitlement_verifier_beta_page, LocalEditingPreservationClass,
    ManagedCapabilityImpactClass, OfflineEntitlementVerifierBetaPage,
    OfflineEntitlementVerifierBetaRenderSummary, OfflineEntitlementVerifierBetaSummary,
    OfflineEntitlementVerifierBetaSupportExport, VerifierOutcomeClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_offline_entitlement_verifier_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = OfflineEntitlementVerifierBetaSupportExport::from_page(
                "offline-entitlement-verifier:support-export:001",
                "2026-05-16T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("summary") => {
            let summary = OfflineEntitlementVerifierBetaRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("drill-expired-bundle-accepted") => {
            let mut page = page;
            let expired = page
                .rows
                .iter_mut()
                .find(|row| row.outcome_class == VerifierOutcomeClass::Expired)
                .ok_or("seeded page must include an expired row")?;
            expired.managed_capability_impact = ManagedCapabilityImpactClass::FullAuthorityActive;
            expired.managed_capability_impact_token =
                ManagedCapabilityImpactClass::FullAuthorityActive
                    .as_str()
                    .to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-untrusted-signer-accepted") => {
            let mut page = page;
            let untrusted = page
                .rows
                .iter_mut()
                .find(|row| row.outcome_class == VerifierOutcomeClass::UntrustedSigner)
                .ok_or("seeded page must include an untrusted-signer row")?;
            untrusted.managed_capability_impact = ManagedCapabilityImpactClass::FullAuthorityActive;
            untrusted.managed_capability_impact_token =
                ManagedCapabilityImpactClass::FullAuthorityActive
                    .as_str()
                    .to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-local-editing-blocked") => {
            let mut page = page;
            let missing = page
                .rows
                .iter_mut()
                .find(|row| row.outcome_class == VerifierOutcomeClass::SignatureMissing)
                .ok_or("seeded page must include a signature-missing row")?;
            missing.local_editing_preservation =
                LocalEditingPreservationClass::NotApplicableLocalOnly;
            missing.local_editing_preservation_token =
                LocalEditingPreservationClass::NotApplicableLocalOnly
                    .as_str()
                    .to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("validate") => match validate_offline_entitlement_verifier_beta_page(&page) {
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

fn rebuild_with_defects(
    mut page: OfflineEntitlementVerifierBetaPage,
) -> OfflineEntitlementVerifierBetaPage {
    page.defects = audit_offline_entitlement_verifier_beta_rows(&page.rows);
    page.summary = OfflineEntitlementVerifierBetaSummary::from_records(&page.rows, &page.defects);
    page
}
