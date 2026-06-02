//! Emits the seeded hardened enterprise network proxy fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_harden_enterprise_network_proxy_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_harden_enterprise_network_proxy_fixtures -- rows
//! cargo run -q -p aureline-policy --example dump_harden_enterprise_network_proxy_fixtures -- defects
//! cargo run -q -p aureline-policy --example dump_harden_enterprise_network_proxy_fixtures -- summary
//! cargo run -q -p aureline-policy --example dump_harden_enterprise_network_proxy_fixtures -- support-export
//! cargo run -q -p aureline-policy --example dump_harden_enterprise_network_proxy_fixtures -- drill-raw-secret-withdrawn
//! cargo run -q -p aureline-policy --example dump_harden_enterprise_network_proxy_fixtures -- drill-missing-route-preview
//! cargo run -q -p aureline-policy --example dump_harden_enterprise_network_proxy_fixtures -- drill-empty-selector-reason-beta
//! ```

use aureline_auth::network_trust::seeded_network_trust_beta_page;
use aureline_policy::{
    seeded_harden_enterprise_network_proxy_page, HardenEnterpriseNetworkProxyPage,
    HardenEnterpriseNetworkProxyRow, HardenEnterpriseNetworkProxySupportExport, ProxyRouteClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_harden_enterprise_network_proxy_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = HardenEnterpriseNetworkProxySupportExport::from_page(
                "policy:harden-enterprise-network-proxy:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-raw-secret-withdrawn") => {
            // Sets raw_secret_or_private_material_excluded = false on the system row
            // to trigger the raw_secret_or_private_material_exposed withdrawal guardrail.
            let network_trust_page = seeded_network_trust_beta_page();
            let mut rows: Vec<HardenEnterpriseNetworkProxyRow> = page.rows;
            if let Some(row) = rows
                .iter_mut()
                .find(|r| r.proxy_route == ProxyRouteClass::System)
            {
                row.raw_secret_or_private_material_excluded = false;
            }
            let drill = HardenEnterpriseNetworkProxyPage::new(
                "policy:harden-enterprise-network-proxy:drill:raw-secret",
                "Drill — raw secret material in row (withdrawn)",
                "2026-06-01T00:00:00Z",
                rows,
                network_trust_page,
            );
            print_json(&drill)?;
        }
        Some("drill-missing-route-preview") => {
            // Removes all but system and pac rows to trigger missing_route_coverage preview.
            let network_trust_page = seeded_network_trust_beta_page();
            let rows: Vec<HardenEnterpriseNetworkProxyRow> = page
                .rows
                .into_iter()
                .filter(|r| {
                    r.proxy_route == ProxyRouteClass::System
                        || r.proxy_route == ProxyRouteClass::Pac
                })
                .collect();
            let drill = HardenEnterpriseNetworkProxyPage::new(
                "policy:harden-enterprise-network-proxy:drill:missing-route",
                "Drill — required proxy route missing (preview)",
                "2026-06-01T00:00:00Z",
                rows,
                network_trust_page,
            );
            print_json(&drill)?;
        }
        Some("drill-empty-selector-reason-beta") => {
            // Clears selector_reason_token on the first row to trigger empty_selector_reason.
            let network_trust_page = seeded_network_trust_beta_page();
            let mut rows: Vec<HardenEnterpriseNetworkProxyRow> = page.rows;
            if let Some(row) = rows.first_mut() {
                row.selector_reason_token.clear();
            }
            let drill = HardenEnterpriseNetworkProxyPage::new(
                "policy:harden-enterprise-network-proxy:drill:empty-selector-reason",
                "Drill — empty selector reason (beta)",
                "2026-06-01T00:00:00Z",
                rows,
                network_trust_page,
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
