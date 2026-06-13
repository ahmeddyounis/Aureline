//! Emits the seeded networked-surface proxy-resolution fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- page
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- rows
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- defects
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- summary
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- support-export
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- cli-view
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- drill-missing-surface-preview
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- drill-raw-material-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- drill-private-stack-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- drill-ca-override-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- drill-silent-fallback-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- drill-denied-no-reason-beta
//! cargo run -q -p aureline-remote --example dump_networked_surface_proxy_resolution_fixtures -- drill-precedence-not-respected-beta
//! ```

use aureline_remote::{
    seeded_proxy_resolution_page, seeded_proxy_resolution_snapshot, NetworkedSurfaceClass,
    ProxyCandidate, ProxyResolutionGovernancePage, ProxyResolutionOutcomeClass,
    ProxyResolutionSupportExport, ProxyResolutionTierClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_proxy_resolution_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("cli-view") => print!("{}", page.render_cli_view()),
        Some("support-export") => {
            let export = ProxyResolutionSupportExport::from_page(
                "remote:networked_surface_proxy_resolution:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-missing-surface-preview") => {
            let mut snapshot = seeded_proxy_resolution_snapshot();
            snapshot
                .records
                .retain(|r| r.surface != NetworkedSurfaceClass::AiGateway);
            let drill = ProxyResolutionGovernancePage::new(
                "remote:networked_surface_proxy_resolution:drill:missing-surface",
                "Drill — required surface absent (preview)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-raw-material-withdrawn") => {
            let mut snapshot = seeded_proxy_resolution_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == NetworkedSurfaceClass::RequestApiClient {
                    r.raw_private_material_excluded = false;
                }
            }
            let drill = ProxyResolutionGovernancePage::new(
                "remote:networked_surface_proxy_resolution:drill:raw-material",
                "Drill — raw private material exposed (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-private-stack-withdrawn") => {
            let mut snapshot = seeded_proxy_resolution_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == NetworkedSurfaceClass::DocsBrowserFetcher {
                    r.candidates.push(ProxyCandidate::private_stack(
                        ProxyResolutionTierClass::ManualPinned,
                        "proxy_source:docs_browser_fetcher:private:0001",
                        "A bundled private proxy stack attempted to intercept the route.",
                    ));
                    r.no_private_proxy_stack = !r.candidates.iter().any(|c| c.is_private_stack);
                }
            }
            let drill = ProxyResolutionGovernancePage::new(
                "remote:networked_surface_proxy_resolution:drill:private-stack",
                "Drill — private proxy stack shipped (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-ca-override-withdrawn") => {
            let mut snapshot = seeded_proxy_resolution_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == NetworkedSurfaceClass::DatabaseCloudConnector {
                    r.no_direct_ca_override = false;
                }
            }
            let drill = ProxyResolutionGovernancePage::new(
                "remote:networked_surface_proxy_resolution:drill:ca-override",
                "Drill — direct CA override shipped (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-silent-fallback-withdrawn") => {
            let mut snapshot = seeded_proxy_resolution_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == NetworkedSurfaceClass::RegistryRead {
                    r.no_silent_direct_fallback = false;
                }
            }
            let drill = ProxyResolutionGovernancePage::new(
                "remote:networked_surface_proxy_resolution:drill:silent-fallback",
                "Drill — silent direct-to-public fallback (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-denied-no-reason-beta") => {
            let mut snapshot = seeded_proxy_resolution_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == NetworkedSurfaceClass::SyncOffboarding {
                    r.outcome = ProxyResolutionOutcomeClass::DeniedProxyResolution;
                    r.outcome_token = ProxyResolutionOutcomeClass::DeniedProxyResolution
                        .as_str()
                        .to_owned();
                    r.denial_reason = None;
                    r.denial_reason_token = String::new();
                    for c in r.candidates.iter_mut() {
                        c.selected = false;
                    }
                    r.selected_tier = None;
                    r.selected_tier_token = String::new();
                }
            }
            let drill = ProxyResolutionGovernancePage::new(
                "remote:networked_surface_proxy_resolution:drill:denied-no-reason",
                "Drill — denied without reason (beta)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-precedence-not-respected-beta") => {
            let mut snapshot = seeded_proxy_resolution_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == NetworkedSurfaceClass::RequestApiClient {
                    for c in r.candidates.iter_mut() {
                        c.selected = c.tier == ProxyResolutionTierClass::SystemProxy;
                    }
                    r.selected_tier = Some(ProxyResolutionTierClass::SystemProxy);
                    r.selected_tier_token =
                        ProxyResolutionTierClass::SystemProxy.as_str().to_owned();
                }
            }
            let drill = ProxyResolutionGovernancePage::new(
                "remote:networked_surface_proxy_resolution:drill:precedence",
                "Drill — precedence not respected (beta)",
                "2026-06-01T00:00:00Z",
                snapshot,
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
