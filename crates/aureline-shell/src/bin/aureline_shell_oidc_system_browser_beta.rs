//! Headless inspector for the beta OIDC system-browser sign-in, recovery, and
//! session-continuity projection.
//!
//! The bin emits the same audited records consumed by the live shell, by the
//! support-export wrapper, and by the integration test that replays the
//! checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_oidc_system_browser_beta -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_oidc_system_browser_beta -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_oidc_system_browser_beta -- support-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_oidc_system_browser_beta -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_oidc_system_browser_beta -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_oidc_system_browser_beta -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_oidc_system_browser_beta -- validate
//! ```

use aureline_shell::oidc_system_browser_beta::{
    audit_oidc_system_browser_beta_rows, seeded_oidc_system_browser_beta_page,
    validate_oidc_system_browser_beta_page, OidcAuthorityScopeClass, OidcIdentityOutageClass,
    OidcSystemBrowserBetaPage, OidcSystemBrowserBetaRenderSummary,
    OidcSystemBrowserBetaSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_oidc_system_browser_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("support-rows") => print_json(&page.support_rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = OidcSystemBrowserBetaSupportExport::from_page(
                "support-export:oidc-system-browser-beta:001",
                "2026-05-16T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("summary") => {
            let summary = OidcSystemBrowserBetaRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("drill-public-issuer-fallback") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|r| {
                    r.session_continuity.session_state_token == "identity_outage_managed_blocked"
                })
                .ok_or("seeded page must include an issuer-outage row")?;
            row.issuer.public_issuer_fallback_used = true;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-sign-out-loses-local-editing") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|r| r.session_continuity.session_state_token == "signed_out_local_intact")
                .ok_or("seeded page must include a signed-out row")?;
            row.session_continuity.local_editing_preserved = false;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-outage-widens-authority") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|r| {
                    r.session_continuity.session_state_token == "identity_outage_managed_blocked"
                })
                .ok_or("seeded page must include an issuer-outage row")?;
            row.granted_authority_scope_token = OidcAuthorityScopeClass::TenantAdminScope
                .as_str()
                .to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-outage-missing-class") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|r| {
                    r.session_continuity.session_state_token == "identity_outage_managed_blocked"
                })
                .ok_or("seeded page must include an issuer-outage row")?;
            row.identity_outage.outage_token =
                OidcIdentityOutageClass::NoOutage.as_str().to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("validate") => match validate_oidc_system_browser_beta_page(&page) {
            Ok(()) => println!("ok"),
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} row_id={} field={} note={}",
                        defect.defect_kind_token, defect.row_id, defect.field, defect.note,
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

fn rebuild_with_defects(mut page: OidcSystemBrowserBetaPage) -> OidcSystemBrowserBetaPage {
    page.defects = audit_oidc_system_browser_beta_rows(&page.rows, &page.support_rows);
    page.summary.defect_count = page.defects.len();
    page
}
