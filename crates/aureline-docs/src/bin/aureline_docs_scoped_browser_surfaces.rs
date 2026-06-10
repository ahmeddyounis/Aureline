//! Headless emitter for the scoped-browser-surfaces packet and its fixture corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_scoped_browser_surfaces -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_scoped_browser_surfaces -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_scoped_browser_surfaces -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_scoped_browser_surfaces -- fixture scope_narrowed_rerun_narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_scoped_browser_surfaces -- validate
//! ```

use aureline_docs::{
    seeded_stable_scoped_browser_input, ScopedBrowserDegradation, ScopedBrowserDegradationClass,
    ScopedBrowserFindingSeverity, ScopedBrowserScope, ScopedBrowserSurfacesPacket,
    ScopedBrowserSurfacesPacketInput, ScopedBrowserTrustClass,
};
use serde::Serialize;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        Some("packet") | None => emit_packet()?,
        Some("support-export") => emit_support_export()?,
        Some("summary") => emit_summary(),
        Some("fixture") => emit_fixture(args.get(1).map(String::as_str))?,
        Some("validate") => validate_packet(),
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn emit_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet = ScopedBrowserSurfacesPacket::materialize(seeded_stable_scoped_browser_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = ScopedBrowserSurfacesPacket::materialize(seeded_stable_scoped_browser_input());
    let export = packet.support_export("support-export:scoped_browser:001", "2026-06-09T00:00:10Z");
    print_json(&export)
}

fn emit_summary() {
    let packet = ScopedBrowserSurfacesPacket::materialize(seeded_stable_scoped_browser_input());
    print!("{}", packet.render_markdown_summary());
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "scope_narrowed_rerun_narrowed" => scope_narrowed_fixture(),
        "out_of_bounds_scope_blocks_stable" => out_of_bounds_scope_fixture(),
        "missing_return_path_blocks_stable" => missing_return_path_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet = ScopedBrowserSurfacesPacket::materialize(seeded_stable_scoped_browser_input());
    if packet.is_clean_stable() {
        println!("ok");
    } else {
        for finding in &packet.validation_findings {
            eprintln!("{}: {}", finding.finding_kind.as_str(), finding.summary);
        }
        std::process::exit(3);
    }
}

#[derive(Serialize)]
struct ScopedBrowserFixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: &'static str,
    scenario: &'static str,
    input: ScopedBrowserSurfacesPacketInput,
    expect: ExpectedFixture,
}

#[derive(Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

fn fixture_input(packet_id: &str) -> ScopedBrowserSurfacesPacketInput {
    let mut input = seeded_stable_scoped_browser_input();
    input.packet_id = packet_id.to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.to_owned();
    }
    input
}

fn scope_narrowed_fixture() -> ScopedBrowserFixture {
    let mut input = fixture_input("packet:m5:scoped_browser:scope_narrowed_rerun");
    input.browser_degradations.push(ScopedBrowserDegradation {
        degradation_class: ScopedBrowserDegradationClass::ScopeNarrowedRerun,
        severity: ScopedBrowserFindingSeverity::Narrowing,
        summary: "the review surface was rerun at a narrowed scope after a policy change; the set narrows below stable".to_owned(),
        surface_id_ref: Some("surface:review:retry_backoff_thread".to_owned()),
        evidence_ref: Some("evidence:scoped-browser:scope-narrow-state".to_owned()),
    });
    ScopedBrowserFixture {
        record_kind: "scoped_browser_surfaces_for_docs_and_review_case",
        schema_version: 1,
        case_name: "scope_narrowed_rerun_narrowed",
        scenario: "The review surface was rerun at a narrowed scope, so the set records a narrowing degradation. The surfaces stay valid and attributable but narrow below Stable instead of hiding them.",
        input,
        expect: ExpectedFixture {
            promotion_state: "narrowed_below_stable",
            expected_finding_kinds: vec![],
        },
    }
}

fn out_of_bounds_scope_fixture() -> ScopedBrowserFixture {
    let mut input = fixture_input("packet:m5:scoped_browser:out_of_bounds_scope");
    // Promote the docs-reading surface to a general-web scope on both the
    // surface and its export row.
    let mut surface_id = String::new();
    for surface in input.surfaces.iter_mut() {
        if surface.scope == ScopedBrowserScope::DocsReading {
            surface.scope = ScopedBrowserScope::GeneralWeb;
            surface_id = surface.surface_id.clone();
        }
    }
    for row in input.export.rows.iter_mut() {
        if row.surface_id_ref == surface_id {
            row.scope = ScopedBrowserScope::GeneralWeb;
        }
    }
    ScopedBrowserFixture {
        record_kind: "scoped_browser_surfaces_for_docs_and_review_case",
        schema_version: 1,
        case_name: "out_of_bounds_scope_blocks_stable",
        scenario: "A surface declares a general-web scope outside the qualified docs/review/light-edit scope. The validator blocks promotion with surface_scope_out_of_bounds and required_scope_missing because the docs scope is no longer covered.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["surface_scope_out_of_bounds"],
        },
    }
}

fn missing_return_path_fixture() -> ScopedBrowserFixture {
    let mut input = fixture_input("packet:m5:scoped_browser:missing_return_path");
    // Strip the trust class to a live-provider handoff while removing the
    // return path so the return-path safety check blocks promotion.
    for surface in input.surfaces.iter_mut() {
        if surface.trust_class == ScopedBrowserTrustClass::LiveProviderHandoff {
            surface.return_path.return_ref = " ".to_owned();
            surface.return_path.label = " ".to_owned();
        }
    }
    ScopedBrowserFixture {
        record_kind: "scoped_browser_surfaces_for_docs_and_review_case",
        schema_version: 1,
        case_name: "missing_return_path_blocks_stable",
        scenario: "A live-provider review handoff drops its return path. Every scoped browser surface must stay return-path safe, so the validator blocks promotion with return_path_missing.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["return_path_missing"],
        },
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
