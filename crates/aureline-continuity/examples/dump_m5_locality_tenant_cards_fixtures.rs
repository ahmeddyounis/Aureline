//! Emits the canonical locality-descriptor and tenant-boundary-card fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-continuity --example dump_m5_locality_tenant_cards_fixtures -- page
//! cargo run -q -p aureline-continuity --example dump_m5_locality_tenant_cards_fixtures -- summary
//! cargo run -q -p aureline-continuity --example dump_m5_locality_tenant_cards_fixtures -- support-export
//! cargo run -q -p aureline-continuity --example dump_m5_locality_tenant_cards_fixtures -- case-region-pin-unhonored-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_locality_tenant_cards_fixtures -- case-region-pin-undeclared-preview
//! cargo run -q -p aureline-continuity --example dump_m5_locality_tenant_cards_fixtures -- case-self-hosted-locality-overclaimed-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_locality_tenant_cards_fixtures -- case-retention-undisclosed-beta
//! cargo run -q -p aureline-continuity --example dump_m5_locality_tenant_cards_fixtures -- case-tenant-boundary-unverified-preview
//! cargo run -q -p aureline-continuity --example dump_m5_locality_tenant_cards_fixtures -- case-surface-projection-incomplete-beta
//! ```

use aureline_continuity::{
    seeded_locality_tenant_card_page, seeded_locality_tenant_input, LocalityClass,
    LocalitySurfaceClass, LocalityTenantCardPage, LocalityTenantEntry, LocalityTenantInput,
    LocalityTenantSupportExport, RegionPinClass, RegionPinHonorState, RetentionClass,
    TenantIsolationClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_locality_tenant_card_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = LocalityTenantSupportExport::from_page(
                "continuity:locality-tenant:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("case-region-pin-unhonored-withdrawn") => {
            let mut input = seeded_locality_tenant_input();
            with_entry(&mut input, "continuity-row:managed-cloud-sync", |entry| {
                entry.region_pin_honor = RegionPinHonorState::CannotHonor;
                entry.region_pin_honor_token = RegionPinHonorState::CannotHonor.as_str().to_owned();
            });
            print_json(&case_page(
                "continuity:locality-tenant:case:region-pin-unhonored",
                "Case - managed region pin cannot be honored, fails closed (withdrawn)",
                input,
            ))?;
        }
        Some("case-region-pin-undeclared-preview") => {
            let mut input = seeded_locality_tenant_input();
            with_entry(&mut input, "continuity-row:managed-cloud-sync", |entry| {
                entry.region_pin = RegionPinClass::Unpinned;
                entry.region_pin_token = RegionPinClass::Unpinned.as_str().to_owned();
                entry.region_pin_label = String::new();
            });
            print_json(&case_page(
                "continuity:locality-tenant:case:region-pin-undeclared",
                "Case - managed row does not declare a region pin (preview)",
                input,
            ))?;
        }
        Some("case-self-hosted-locality-overclaimed-withdrawn") => {
            let mut input = seeded_locality_tenant_input();
            with_entry(&mut input, "continuity-row:self-hosted-restore", |entry| {
                entry.storage_location = LocalityClass::MultiRegion;
                entry.storage_location_token = LocalityClass::MultiRegion.as_str().to_owned();
            });
            print_json(&case_page(
                "continuity:locality-tenant:case:self-hosted-locality-overclaimed",
                "Case - self-hosted row claims a broad vendor region (withdrawn)",
                input,
            ))?;
        }
        Some("case-retention-undisclosed-beta") => {
            let mut input = seeded_locality_tenant_input();
            with_entry(
                &mut input,
                "continuity-row:managed-relay-failover",
                |entry| {
                    entry.retention_class = RetentionClass::RetentionUndisclosed;
                    entry.retention_class_token =
                        RetentionClass::RetentionUndisclosed.as_str().to_owned();
                },
            );
            print_json(&case_page(
                "continuity:locality-tenant:case:retention-undisclosed",
                "Case - managed relay row hides its retention class (beta)",
                input,
            ))?;
        }
        Some("case-tenant-boundary-unverified-preview") => {
            let mut input = seeded_locality_tenant_input();
            with_entry(
                &mut input,
                "continuity-row:managed-relay-failover",
                |entry| {
                    entry.tenant_isolation = TenantIsolationClass::IsolationUnverified;
                    entry.tenant_isolation_token = TenantIsolationClass::IsolationUnverified
                        .as_str()
                        .to_owned();
                },
            );
            print_json(&case_page(
                "continuity:locality-tenant:case:tenant-boundary-unverified",
                "Case - managed relay row cannot verify its tenant boundary (preview)",
                input,
            ))?;
        }
        Some("case-surface-projection-incomplete-beta") => {
            let mut input = seeded_locality_tenant_input();
            with_entry(&mut input, "continuity-row:managed-cloud-sync", |entry| {
                entry
                    .projected_surfaces
                    .retain(|surface| *surface != LocalitySurfaceClass::SupportExport);
            });
            print_json(&case_page(
                "continuity:locality-tenant:case:surface-projection-incomplete",
                "Case - managed row is not projected onto the support-export surface (beta)",
                input,
            ))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn with_entry(
    input: &mut LocalityTenantInput,
    row_id: &str,
    mutate: impl FnOnce(&mut LocalityTenantEntry),
) {
    let entry = input
        .entries
        .iter_mut()
        .find(|entry| entry.row_id == row_id)
        .unwrap_or_else(|| panic!("missing seeded entry: {row_id}"));
    mutate(entry);
}

fn case_page(
    page_id: &str,
    page_label: &str,
    input: LocalityTenantInput,
) -> LocalityTenantCardPage {
    LocalityTenantCardPage::new(page_id, page_label, "2026-06-01T00:00:00Z", input)
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
