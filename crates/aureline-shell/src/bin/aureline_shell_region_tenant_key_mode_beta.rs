//! Headless inspector for the region / tenant / key-mode beta page.
//!
//! The bin emits the same audited records consumed by the admin/settings
//! center, support-export wrapper, shell summary, diagnostics views, and
//! fixture replay.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- region-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- tenant-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- key-mode-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- drill-packets
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- drill-region-pinning-failure
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- drill-region-failover
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- drill-tenant-failover
//! cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- drill-key-mode-failover
//! ```

use aureline_shell::region_tenant_key_mode_beta::{
    audit_region_tenant_key_mode_beta_page, seeded_region_tenant_key_mode_beta_page,
    validate_region_tenant_key_mode_beta_page, RegionTenantDrillKindClass,
    RegionTenantKeyModeBetaPage, RegionTenantKeyModeBetaRenderSummary,
    RegionTenantKeyModeBetaSummary, RegionTenantKeyModeBetaSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_region_tenant_key_mode_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("region-rows") => print_json(&page.region_rows)?,
        Some("tenant-rows") => print_json(&page.tenant_rows)?,
        Some("key-mode-rows") => print_json(&page.key_mode_rows)?,
        Some("drill-packets") => print_json(&page.drill_packets)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = RegionTenantKeyModeBetaSupportExport::from_page(
                "support-export:region-tenant-key-mode:001",
                "2026-05-16T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("summary") => {
            let summary = RegionTenantKeyModeBetaRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("drill-region-pinning-failure") => print_drill_by_kind(
            page,
            RegionTenantDrillKindClass::RegionPinningFailure,
        )?,
        Some("drill-region-failover") => {
            print_drill_by_kind(page, RegionTenantDrillKindClass::RegionFailover)?
        }
        Some("drill-tenant-boundary-drift") => print_drill_by_kind(
            page,
            RegionTenantDrillKindClass::TenantBoundaryDrift,
        )?,
        Some("drill-tenant-failover") => {
            print_drill_by_kind(page, RegionTenantDrillKindClass::TenantFailover)?
        }
        Some("drill-key-mode-drift") => {
            print_drill_by_kind(page, RegionTenantDrillKindClass::KeyModeDrift)?
        }
        Some("drill-key-mode-failover") => {
            print_drill_by_kind(page, RegionTenantDrillKindClass::KeyModeFailover)?
        }
        Some("validate") => match validate_region_tenant_key_mode_beta_page(&page) {
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

fn print_drill_by_kind(
    page: RegionTenantKeyModeBetaPage,
    kind: RegionTenantDrillKindClass,
) -> Result<(), Box<dyn std::error::Error>> {
    let packet = page
        .drill_packets
        .iter()
        .find(|packet| packet.drill_kind == kind)
        .ok_or_else(|| format!("no seeded drill packet of kind {}", kind.as_str()))?;
    print_json(packet)?;
    Ok(())
}

#[allow(dead_code)]
fn rebuild_with_defects(mut page: RegionTenantKeyModeBetaPage) -> RegionTenantKeyModeBetaPage {
    page.defects = audit_region_tenant_key_mode_beta_page(
        &page.region_rows,
        &page.tenant_rows,
        &page.key_mode_rows,
        &page.drill_packets,
    );
    page.summary = RegionTenantKeyModeBetaSummary::from_records(
        &page.region_rows,
        &page.tenant_rows,
        &page.key_mode_rows,
        &page.drill_packets,
        &page.defects,
    );
    page
}
