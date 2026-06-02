//! Emits the seeded hardened OS keychain and trust-store integration fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_harden_os_keychain_trust_store_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_harden_os_keychain_trust_store_fixtures -- rows
//! cargo run -q -p aureline-policy --example dump_harden_os_keychain_trust_store_fixtures -- change-events
//! cargo run -q -p aureline-policy --example dump_harden_os_keychain_trust_store_fixtures -- defects
//! cargo run -q -p aureline-policy --example dump_harden_os_keychain_trust_store_fixtures -- summary
//! cargo run -q -p aureline-policy --example dump_harden_os_keychain_trust_store_fixtures -- support-export
//! cargo run -q -p aureline-policy --example dump_harden_os_keychain_trust_store_fixtures -- drill-raw-material-withdrawn
//! cargo run -q -p aureline-policy --example dump_harden_os_keychain_trust_store_fixtures -- drill-missing-layer-preview
//! cargo run -q -p aureline-policy --example dump_harden_os_keychain_trust_store_fixtures -- drill-missing-local-continuity-beta
//! ```

use aureline_policy::{
    seeded_harden_os_keychain_trust_store_page, HardenOsKeychainTrustStorePage,
    HardenOsKeychainTrustStoreSupportExport, SessionImpactClass, TrustStoreLayerClass,
    TrustStoreLayerRow, TrustStoreRepairActionClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_harden_os_keychain_trust_store_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("change-events") => print_json(&page.change_events)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = HardenOsKeychainTrustStoreSupportExport::from_page(
                "policy:harden-os-keychain-trust-store:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-raw-material-withdrawn") => {
            // Sets raw_trust_material_excluded = false on the os_roots row to trigger withdrawal.
            let mut rows: Vec<TrustStoreLayerRow> = page.rows;
            if let Some(row) = rows
                .iter_mut()
                .find(|r| r.layer == TrustStoreLayerClass::OsRoots)
            {
                row.raw_trust_material_excluded = false;
            }
            let drill = HardenOsKeychainTrustStorePage::new(
                "policy:harden-os-keychain-trust-store:drill:raw-material",
                "Drill — raw trust material in row (withdrawn)",
                "2026-06-01T00:00:00Z",
                rows,
                page.change_events,
            );
            print_json(&drill)?;
        }
        Some("drill-missing-layer-preview") => {
            // Removes all but os_roots and custom_ca_bundle rows to trigger preview.
            let rows: Vec<TrustStoreLayerRow> = page
                .rows
                .into_iter()
                .filter(|r| {
                    r.layer == TrustStoreLayerClass::OsRoots
                        || r.layer == TrustStoreLayerClass::CustomCaBundle
                })
                .collect();
            let drill = HardenOsKeychainTrustStorePage::new(
                "policy:harden-os-keychain-trust-store:drill:missing-layer",
                "Drill — required trust-store layer missing (preview)",
                "2026-06-01T00:00:00Z",
                rows,
                vec![],
            );
            print_json(&drill)?;
        }
        Some("drill-missing-local-continuity-beta") => {
            // Clears local_continuity_explicit on custom_ca_bundle to trigger beta narrowing.
            let mut rows: Vec<TrustStoreLayerRow> = page.rows;
            if let Some(row) = rows
                .iter_mut()
                .find(|r| r.layer == TrustStoreLayerClass::CustomCaBundle)
            {
                row.local_continuity_explicit = false;
            }
            let drill = HardenOsKeychainTrustStorePage::new(
                "policy:harden-os-keychain-trust-store:drill:missing-local-continuity",
                "Drill — missing local continuity declaration (beta)",
                "2026-06-01T00:00:00Z",
                rows,
                page.change_events,
            );
            print_json(&drill)?;
        }
        Some("drill-missing-repair-action-beta") => {
            // Forces a blocking session impact with NoneRequired repair action.
            let mut events = page.change_events;
            if let Some(event) = events.first_mut() {
                event.session_impact = SessionImpactClass::RouteBlockedLocalContinuity;
                event.session_impact_token = SessionImpactClass::RouteBlockedLocalContinuity
                    .as_str()
                    .to_owned();
                event.repair_action = TrustStoreRepairActionClass::NoneRequired;
                event.repair_action_token = TrustStoreRepairActionClass::NoneRequired
                    .as_str()
                    .to_owned();
                event.affected_route_refs = vec!["tls_enterprise".to_owned()];
            }
            let drill = HardenOsKeychainTrustStorePage::new(
                "policy:harden-os-keychain-trust-store:drill:missing-repair-action",
                "Drill — blocking event missing repair action (beta)",
                "2026-06-01T00:00:00Z",
                page.rows,
                events,
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
