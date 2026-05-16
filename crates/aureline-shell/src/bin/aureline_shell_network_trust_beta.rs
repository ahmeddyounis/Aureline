//! Headless inspector for the beta network-trust page.
//!
//! The bin emits the same audited records consumed by the admin/settings
//! center, support-export wrapper, shell summary, diagnostics views, and
//! fixture replay.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_network_trust_beta -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_network_trust_beta -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_network_trust_beta -- support-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_network_trust_beta -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_network_trust_beta -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_network_trust_beta -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_network_trust_beta -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_network_trust_beta -- drill-unsigned-managed
//! cargo run -q -p aureline-shell --bin aureline_shell_network_trust_beta -- drill-public-fallback
//! cargo run -q -p aureline-shell --bin aureline_shell_network_trust_beta -- drill-lock-source-mismatch
//! ```

use aureline_shell::network_trust_beta::{
    audit_network_trust_beta_rows, seeded_network_trust_beta_page,
    validate_network_trust_beta_page, NetworkSettingLockClass, NetworkTrustBetaPage,
    NetworkTrustBetaProfileClass, NetworkTrustBetaRenderSummary, NetworkTrustBetaSummary,
    NetworkTrustBetaSupportExport, NetworkTrustBetaSupportRow,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_network_trust_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("support-rows") => print_json(&page.support_rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = NetworkTrustBetaSupportExport::from_page(
                "support-export:network-trust-beta:001",
                "2026-05-16T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("summary") => {
            let summary = NetworkTrustBetaRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("drill-unsigned-managed") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|row| row.facet_token == "trust_store")
                .ok_or("seeded page must include the trust_store row")?;
            let binding = row
                .profile_bindings
                .iter_mut()
                .find(|b| b.profile_class == NetworkTrustBetaProfileClass::EnterpriseManaged)
                .ok_or("trust_store row must include enterprise_managed binding")?;
            binding.managed_attribution_ref.clear();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-public-fallback") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|row| row.facet_token == "proxy")
                .ok_or("seeded page must include the proxy row")?;
            row.no_public_endpoint_fallback = false;
            let binding = row
                .profile_bindings
                .iter_mut()
                .find(|b| b.profile_class == NetworkTrustBetaProfileClass::MirrorOnly)
                .ok_or("proxy row must include mirror_only binding")?;
            binding.no_public_endpoint_fallback = false;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-lock-source-mismatch") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|row| row.facet_token == "ssh_host_proof")
                .ok_or("seeded page must include the ssh_host_proof row")?;
            let binding = row
                .profile_bindings
                .iter_mut()
                .find(|b| b.profile_class == NetworkTrustBetaProfileClass::Connected)
                .ok_or("ssh_host_proof row must include connected binding")?;
            binding.lock_class = NetworkSettingLockClass::SignedManagedPolicyLocked;
            binding.lock_token = NetworkSettingLockClass::SignedManagedPolicyLocked
                .as_str()
                .to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("validate") => match validate_network_trust_beta_page(&page) {
            Ok(()) => println!("ok"),
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} row_id={} facet={} profile={} field={} note={}",
                        defect.defect_kind_token,
                        defect.row_id,
                        defect.facet_token,
                        defect.profile_token,
                        defect.field,
                        defect.note,
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

fn rebuild_with_defects(mut page: NetworkTrustBetaPage) -> NetworkTrustBetaPage {
    page.support_rows = page
        .rows
        .iter()
        .map(NetworkTrustBetaSupportRow::from_row)
        .collect();
    page.defects = audit_network_trust_beta_rows(&page.rows, &page.support_rows);
    page.summary = NetworkTrustBetaSummary::from_rows(&page.rows, &page.support_rows, &page.defects);
    page
}
