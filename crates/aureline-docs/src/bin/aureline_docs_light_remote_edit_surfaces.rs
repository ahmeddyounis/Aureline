//! Headless emitter for the light-remote-edit-surfaces packet and its fixture corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_light_remote_edit_surfaces -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_light_remote_edit_surfaces -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_light_remote_edit_surfaces -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_light_remote_edit_surfaces -- fixture authority_narrowed_rerun_narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_light_remote_edit_surfaces -- validate
//! ```

use aureline_docs::{
    seeded_stable_light_remote_edit_input, AuthorityScope, BaseStateKind,
    LightRemoteEditDegradation, LightRemoteEditDegradationClass, LightRemoteEditFindingSeverity,
    LightRemoteEditScope, LightRemoteEditSurfacesPacket, LightRemoteEditSurfacesPacketInput,
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
    let packet =
        LightRemoteEditSurfacesPacket::materialize(seeded_stable_light_remote_edit_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        LightRemoteEditSurfacesPacket::materialize(seeded_stable_light_remote_edit_input());
    let export = packet.support_export(
        "support-export:light_remote_edit:001",
        "2026-06-10T00:00:10Z",
    );
    print_json(&export)
}

fn emit_summary() {
    let packet =
        LightRemoteEditSurfacesPacket::materialize(seeded_stable_light_remote_edit_input());
    print!("{}", packet.render_markdown_summary());
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "authority_narrowed_rerun_narrowed" => authority_narrowed_fixture(),
        "stale_base_undisclosed_blocks_stable" => stale_base_undisclosed_fixture(),
        "authority_expansion_blocks_stable" => authority_expansion_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet =
        LightRemoteEditSurfacesPacket::materialize(seeded_stable_light_remote_edit_input());
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
struct LightRemoteEditFixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: &'static str,
    scenario: &'static str,
    input: LightRemoteEditSurfacesPacketInput,
    expect: ExpectedFixture,
}

#[derive(Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

fn fixture_input(packet_id: &str) -> LightRemoteEditSurfacesPacketInput {
    let mut input = seeded_stable_light_remote_edit_input();
    input.packet_id = packet_id.to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.to_owned();
    }
    input
}

fn authority_narrowed_fixture() -> LightRemoteEditFixture {
    let mut input = fixture_input("packet:m5:light_remote_edit:authority_narrowed_rerun");
    input.edit_degradations.push(LightRemoteEditDegradation {
        degradation_class: LightRemoteEditDegradationClass::AuthorityNarrowed,
        severity: LightRemoteEditFindingSeverity::Narrowing,
        summary: "the remote single-file edit's authority was narrowed to read-only after a policy change; the set narrows below stable".to_owned(),
        surface_id_ref: Some("surface:single_file_text_edit:retry_log_message".to_owned()),
        evidence_ref: Some("evidence:light-remote-edit:authority-narrow-state".to_owned()),
    });
    LightRemoteEditFixture {
        record_kind: "light_remote_edit_surfaces_case",
        schema_version: 1,
        case_name: "authority_narrowed_rerun_narrowed",
        scenario: "The remote single-file edit's authority was narrowed to read-only after a policy change, so the set records a narrowing degradation. The surfaces stay valid and attributable but narrow below Stable instead of hiding them.",
        input,
        expect: ExpectedFixture {
            promotion_state: "narrowed_below_stable",
            expected_finding_kinds: vec![],
        },
    }
}

fn stale_base_undisclosed_fixture() -> LightRemoteEditFixture {
    let mut input = fixture_input("packet:m5:light_remote_edit:stale_base_undisclosed");
    // Flip the single-file edit's base to a known-stale snapshot but hide it.
    let mut surface_id = String::new();
    for surface in input.surfaces.iter_mut() {
        if surface.scope == LightRemoteEditScope::SingleFileTextEdit {
            surface.stale_state.base_state_kind = BaseStateKind::StaleSnapshot;
            surface.stale_state.disclosed = false;
            surface_id = surface.surface_id.clone();
        }
    }
    for row in input.export.rows.iter_mut() {
        if row.surface_id_ref == surface_id {
            row.base_state_kind = BaseStateKind::StaleSnapshot;
            row.stale_disclosed = false;
        }
    }
    LightRemoteEditFixture {
        record_kind: "light_remote_edit_surfaces_case",
        schema_version: 1,
        case_name: "stale_base_undisclosed_blocks_stable",
        scenario: "A single-file remote edit is prepared against a known-stale snapshot but does not disclose it. Stale-state honesty is mandatory, so the validator blocks promotion with stale_state_not_disclosed.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["stale_state_not_disclosed"],
        },
    }
}

fn authority_expansion_fixture() -> LightRemoteEditFixture {
    let mut input = fixture_input("packet:m5:light_remote_edit:authority_expansion");
    // Lower the single-file edit's grant below its (unchanged) effective
    // authority so the effective authority becomes a hidden expansion that
    // still stays within the scope ceiling.
    let mut surface_id = String::new();
    for surface in input.surfaces.iter_mut() {
        if surface.scope == LightRemoteEditScope::SingleFileTextEdit {
            surface.authority.granted = AuthorityScope::SingleFieldWrite;
            surface_id = surface.surface_id.clone();
        }
    }
    for row in input.export.rows.iter_mut() {
        if row.surface_id_ref == surface_id {
            row.granted_authority = AuthorityScope::SingleFieldWrite;
        }
    }
    LightRemoteEditFixture {
        record_kind: "light_remote_edit_surfaces_case",
        schema_version: 1,
        case_name: "authority_expansion_blocks_stable",
        scenario: "A single-file remote edit exercises single-file-write authority while only single-field-write was granted. A light remote edit may never expand its authority beyond the grant, so the validator blocks promotion with authority_expansion_detected.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["authority_expansion_detected"],
        },
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
