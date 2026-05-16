//! Headless inspector for the beta network-badge page.
//!
//! The bin emits the same audited records consumed by the admin/settings
//! center, support-export wrapper, shell network strip, diagnostics views, and
//! fixture replay.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_network_badges -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_network_badges -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_network_badges -- support-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_network_badges -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_network_badges -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_network_badges -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_network_badges -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_network_badges -- drill-locality-egress-mismatch
//! cargo run -q -p aureline-shell --bin aureline_shell_network_badges -- drill-hidden-public-cloud
//! cargo run -q -p aureline-shell --bin aureline_shell_network_badges -- drill-missing-surface
//! ```

use aureline_shell::network_badges::{
    audit_network_badge_beta_rows, seeded_network_badge_beta_page,
    validate_network_badge_beta_page, LocalityClass, NetworkBadgeBetaPage,
    NetworkBadgeBetaProfileClass, NetworkBadgeBetaRenderSummary, NetworkBadgeBetaSummary,
    NetworkBadgeBetaSupportExport, NetworkBadgeBetaSupportRow, NetworkBadgeSurfaceClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_network_badge_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("support-rows") => print_json(&page.support_rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = NetworkBadgeBetaSupportExport::from_page(
                "support-export:network-badges-beta:001",
                "2026-05-16T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("summary") => {
            let summary = NetworkBadgeBetaRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("drill-locality-egress-mismatch") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|row| row.surface == NetworkBadgeSurfaceClass::Update)
                .ok_or("seeded page must include the update row")?;
            let binding = row
                .profile_bindings
                .iter_mut()
                .find(|b| b.profile_class == NetworkBadgeBetaProfileClass::Connected)
                .ok_or("update row must include connected binding")?;
            binding.locality = LocalityClass::Mirrored;
            binding.locality_token = LocalityClass::Mirrored.as_str().to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-hidden-public-cloud") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|row| row.surface == NetworkBadgeSurfaceClass::Provider)
                .ok_or("seeded page must include the provider row")?;
            let binding = row
                .profile_bindings
                .iter_mut()
                .find(|b| b.profile_class == NetworkBadgeBetaProfileClass::Connected)
                .ok_or("provider row must include connected binding")?;
            binding.route_label.clear();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-missing-surface") => {
            let mut page = page;
            page.rows
                .retain(|row| row.surface != NetworkBadgeSurfaceClass::DocsHelp);
            page.support_rows
                .retain(|row| row.surface_token != "docs_help");
            print_json(&rebuild_with_defects(page))?;
        }
        Some("validate") => match validate_network_badge_beta_page(&page) {
            Ok(()) => println!("ok"),
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} row_id={} surface={} profile={} field={} note={}",
                        defect.defect_kind_token,
                        defect.row_id,
                        defect.surface_token,
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

fn rebuild_with_defects(mut page: NetworkBadgeBetaPage) -> NetworkBadgeBetaPage {
    page.support_rows = page
        .rows
        .iter()
        .map(NetworkBadgeBetaSupportRow::from_row)
        .collect();
    page.defects = audit_network_badge_beta_rows(&page.rows, &page.support_rows);
    page.summary =
        NetworkBadgeBetaSummary::from_rows(&page.rows, &page.support_rows, &page.defects);
    page
}
