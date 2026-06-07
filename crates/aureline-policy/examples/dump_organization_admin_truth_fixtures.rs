//! Emits the seeded organization-admin truth fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- providers
//! cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- seats
//! cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- impacts
//! cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- rollout-rings
//! cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- defects
//! cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- summary
//! cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- support-export
//! cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- drill-local-safety-withdrawn
//! cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- drill-missing-provisioning-preview
//! cargo run -q -p aureline-policy --example dump_organization_admin_truth_fixtures -- drill-generic-failure-beta
//! ```

use aureline_policy::{
    seeded_organization_admin_truth_page, AdminFailureKind, LifecycleFlowClass,
    OrganizationAdminTruthPage, OrganizationAdminTruthSupportExport, OrganizationProvisioningClass,
    ProviderStateClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_organization_admin_truth_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("overview") => print_json(&page.overview)?,
        Some("providers") => print_json(&page.provider_cards)?,
        Some("seats") => print_json(&page.seat_lifecycle_rows)?,
        Some("impacts") => print_json(&page.impact_previews)?,
        Some("rollout-rings") => print_json(&page.rollout_rings)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = OrganizationAdminTruthSupportExport::from_page(
                "policy:organization-admin-truth:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-local-safety-withdrawn") => {
            let mut page = page;
            page.seat_lifecycle_rows[0].export_offboarding_available = false;
            let drill = rebuild_page(
                "policy:organization-admin-truth:drill:local-safety",
                "Drill - local safety guarantee missing (withdrawn)",
                page,
            );
            print_json(&drill)?;
        }
        Some("drill-missing-provisioning-preview") => {
            let mut page = page;
            page.provider_cards.retain(|provider| {
                provider.provisioning_class != OrganizationProvisioningClass::Manual
            });
            let drill = rebuild_page(
                "policy:organization-admin-truth:drill:missing-provisioning",
                "Drill - missing provisioning class coverage (preview)",
                page,
            );
            print_json(&drill)?;
        }
        Some("drill-generic-failure-beta") => {
            let mut page = page;
            if let Some(provider) = page
                .provider_cards
                .iter_mut()
                .find(|provider| provider.provider_state == ProviderStateClass::Degraded)
            {
                provider.failure_kind = None;
                provider.failure_kind_token = "generic_admin_error".to_owned();
            }
            if let Some(preview) = page
                .impact_previews
                .iter_mut()
                .find(|preview| preview.flow_class == LifecycleFlowClass::OrgSwitch)
            {
                preview.failure_kind = Some(AdminFailureKind::ScopeMismatch);
            }
            let drill = rebuild_page(
                "policy:organization-admin-truth:drill:generic-failure",
                "Drill - generic provisioning failure (beta)",
                page,
            );
            print_json(&drill)?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn rebuild_page(
    page_id: &str,
    page_label: &str,
    page: OrganizationAdminTruthPage,
) -> OrganizationAdminTruthPage {
    OrganizationAdminTruthPage::new(
        page_id,
        page_label,
        "2026-06-01T00:00:00Z",
        page.overview,
        page.provider_cards,
        page.seat_lifecycle_rows,
        page.impact_previews,
        page.rollout_rings,
    )
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
