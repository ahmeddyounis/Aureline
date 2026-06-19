//! Emits the canonical control-plane-versus-data-plane outage fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-continuity --example dump_m5_control_plane_vs_data_plane_outage_fixtures -- page
//! cargo run -q -p aureline-continuity --example dump_m5_control_plane_vs_data_plane_outage_fixtures -- summary
//! cargo run -q -p aureline-continuity --example dump_m5_control_plane_vs_data_plane_outage_fixtures -- support-export
//! cargo run -q -p aureline-continuity --example dump_m5_control_plane_vs_data_plane_outage_fixtures -- case-ide-down-conflation-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_control_plane_vs_data_plane_outage_fixtures -- case-local-editing-conflated-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_control_plane_vs_data_plane_outage_fixtures -- case-fallback-undeclared-beta
//! cargo run -q -p aureline-continuity --example dump_m5_control_plane_vs_data_plane_outage_fixtures -- case-operational-inconsistent-preview
//! cargo run -q -p aureline-continuity --example dump_m5_control_plane_vs_data_plane_outage_fixtures -- case-outage-evidence-stale-preview
//! cargo run -q -p aureline-continuity --example dump_m5_control_plane_vs_data_plane_outage_fixtures -- case-family-coverage-incomplete-beta
//! ```

use aureline_continuity::{
    seeded_service_outage_taxonomy_input, seeded_service_outage_taxonomy_page,
    DegradedFallbackClass, ImpairmentSeverityClass, OptionalServiceFamily,
    OutageEvidenceStateClass, ServiceOutageEntry, ServiceOutageTaxonomyInput,
    ServiceOutageTaxonomyPage, ServiceOutageTaxonomySupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_service_outage_taxonomy_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = ServiceOutageTaxonomySupportExport::from_page(
                "continuity:outage-taxonomy:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("case-ide-down-conflation-withdrawn") => {
            let mut input = seeded_service_outage_taxonomy_input();
            with_entry(&mut input, "continuity-outage:collaboration", |entry| {
                entry.sets_global_ide_down = true;
            });
            print_json(&case_page(
                "continuity:outage-taxonomy:case:ide-down-conflation",
                "Case - collaboration outage flips a global IDE-down state (withdrawn)",
                input,
            ))?;
        }
        Some("case-local-editing-conflated-withdrawn") => {
            let mut input = seeded_service_outage_taxonomy_input();
            with_entry(
                &mut input,
                "continuity-outage:remote-control-plane",
                |entry| {
                    entry.local_core.editing_available = false;
                    entry.local_core.save_available = false;
                },
            );
            print_json(&case_page(
                "continuity:outage-taxonomy:case:local-editing-conflated",
                "Case - remote control-plane outage marks local editing/save down (withdrawn)",
                input,
            ))?;
        }
        Some("case-fallback-undeclared-beta") => {
            let mut input = seeded_service_outage_taxonomy_input();
            with_entry(&mut input, "continuity-outage:ai-gateway", |entry| {
                entry.fallback = DegradedFallbackClass::NotDeclared;
                entry.fallback_token = DegradedFallbackClass::NotDeclared.as_str().to_owned();
            });
            print_json(&case_page(
                "continuity:outage-taxonomy:case:fallback-undeclared",
                "Case - impaired AI gateway names no narrower fallback (beta)",
                input,
            ))?;
        }
        Some("case-operational-inconsistent-preview") => {
            let mut input = seeded_service_outage_taxonomy_input();
            with_entry(&mut input, "continuity-outage:identity-policy", |entry| {
                entry.severity = ImpairmentSeverityClass::Operational;
                entry.severity_token = ImpairmentSeverityClass::Operational.as_str().to_owned();
            });
            print_json(&case_page(
                "continuity:outage-taxonomy:case:operational-inconsistent",
                "Case - operational lane still claims an active fallback (preview)",
                input,
            ))?;
        }
        Some("case-outage-evidence-stale-preview") => {
            let mut input = seeded_service_outage_taxonomy_input();
            with_entry(
                &mut input,
                "continuity-outage:registry-updates-docs",
                |entry| {
                    entry.evidence_state = OutageEvidenceStateClass::StaleNeedsRefresh;
                    entry.evidence_state_token = OutageEvidenceStateClass::StaleNeedsRefresh
                        .as_str()
                        .to_owned();
                },
            );
            print_json(&case_page(
                "continuity:outage-taxonomy:case:outage-evidence-stale",
                "Case - registry/updates/docs outage evidence is stale (preview)",
                input,
            ))?;
        }
        Some("case-family-coverage-incomplete-beta") => {
            let mut input = seeded_service_outage_taxonomy_input();
            input
                .entries
                .retain(|entry| entry.family != OptionalServiceFamily::TelemetrySupport);
            print_json(&case_page(
                "continuity:outage-taxonomy:case:family-coverage-incomplete",
                "Case - telemetry/support family is missing from the taxonomy (beta)",
                input,
            ))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn with_entry(
    input: &mut ServiceOutageTaxonomyInput,
    packet_id: &str,
    mutate: impl FnOnce(&mut ServiceOutageEntry),
) {
    let entry = input
        .entries
        .iter_mut()
        .find(|entry| entry.packet_id == packet_id)
        .unwrap_or_else(|| panic!("missing seeded packet: {packet_id}"));
    mutate(entry);
}

fn case_page(
    page_id: &str,
    page_label: &str,
    input: ServiceOutageTaxonomyInput,
) -> ServiceOutageTaxonomyPage {
    ServiceOutageTaxonomyPage::new(page_id, page_label, "2026-06-01T00:00:00Z", input)
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
