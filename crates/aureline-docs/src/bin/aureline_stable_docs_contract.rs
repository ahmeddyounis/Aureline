//! Headless emitter for the stable docs-source/result/pack/citation packet.

use aureline_docs::{
    seeded_stable_docs_source_result_pack_and_citation_input, StableDocsFindingKind,
    StableDocsPromotionState, StableDocsSourceResultPackCitationInput,
    StableDocsSourceResultPackCitationPacket, STABLE_DOCS_CONTRACT_SCHEMA_VERSION,
};
use serde::Serialize;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        Some("packet") | None => emit_packet()?,
        Some("support-export") => emit_support_export()?,
        Some("fixture") => emit_fixture(args.get(1).map(String::as_str))?,
        Some("validate") => validate_packet()?,
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn emit_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet = StableDocsSourceResultPackCitationPacket::materialize(
        seeded_stable_docs_source_result_pack_and_citation_input(),
    );
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = StableDocsSourceResultPackCitationPacket::materialize(
        seeded_stable_docs_source_result_pack_and_citation_input(),
    );
    let export = packet.support_export(
        "support-export:stable_docs_contract:001",
        "2026-06-04T16:00:10Z",
    );
    print_json(&export)
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "baseline_stable" => baseline_fixture(),
        "source_result_freshness_drift_blocks_stable" => source_result_drift_fixture(),
        "citation_set_bundles_raw_pack_blocks_stable" => raw_citation_fixture(),
        "pack_detail_sheet_hides_actions_blocks_stable" => detail_sheet_actions_fixture(),
        "citation_drawer_drops_inference_marker_blocks_stable" => drawer_drift_fixture(),
        "consumer_projection_drops_precedence_blocks_stable" => projection_drift_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet = StableDocsSourceResultPackCitationPacket::materialize(
        seeded_stable_docs_source_result_pack_and_citation_input(),
    );
    if packet.promotion_state == StableDocsPromotionState::Stable
        && packet.validation_findings.is_empty()
    {
        println!("ok");
        Ok(())
    } else {
        for finding in &packet.validation_findings {
            eprintln!("{}: {}", finding.finding_kind.as_str(), finding.summary);
        }
        std::process::exit(3);
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

#[derive(Debug, Serialize)]
struct Fixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: StableDocsSourceResultPackCitationInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

const FIXTURE_RECORD_KIND: &str = "stable_docs_contract_case";

fn baseline_fixture() -> Fixture {
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: STABLE_DOCS_CONTRACT_SCHEMA_VERSION,
        case_name: "baseline_stable".to_owned(),
        scenario: "Baseline stable packet proves shared source/result objects, pack manifests, detail sheets, reference-only citation export, source precedence, handoff state, and citation-drawer parity across docs browser, Help/About, onboarding, AI explainers, extension/help APIs, and support export.".to_owned(),
        input: seeded_stable_docs_source_result_pack_and_citation_input(),
        expect: ExpectedFixture {
            promotion_state: StableDocsPromotionState::Stable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn source_result_drift_fixture() -> Fixture {
    let mut input = seeded_stable_docs_source_result_pack_and_citation_input();
    input.packet_id = "packet:stable_docs_contract:source_result_drift".to_owned();
    update_projection_packet_refs(&mut input);
    if let Some(result) = input.results.first_mut() {
        result.freshness_state = aureline_docs::DocsFreshnessClass::Unverified;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: STABLE_DOCS_CONTRACT_SCHEMA_VERSION,
        case_name: "source_result_freshness_drift_blocks_stable".to_owned(),
        scenario: "A result object silently changes freshness from the source descriptor. The validator blocks promotion because docs browser, help, onboarding, AI, and support must agree on source/version/freshness truth for the same object.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: StableDocsPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![StableDocsFindingKind::SourceResultTruthMismatch.as_str()],
        },
    }
}

fn raw_citation_fixture() -> Fixture {
    let mut input = seeded_stable_docs_source_result_pack_and_citation_input();
    input.packet_id = "packet:stable_docs_contract:raw_citation".to_owned();
    update_projection_packet_refs(&mut input);
    if let Some(set) = input.derived_citation_sets.first_mut() {
        set.raw_pack_bodies_excluded = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: STABLE_DOCS_CONTRACT_SCHEMA_VERSION,
        case_name: "citation_set_bundles_raw_pack_blocks_stable".to_owned(),
        scenario: "A derived citation export includes raw pack bodies by default. The validator blocks promotion because explanations and tours must export enough identity to reopen sources without bundling entire docs packs.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: StableDocsPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![StableDocsFindingKind::CitationSetBundlesRawPack.as_str()],
        },
    }
}

fn detail_sheet_actions_fixture() -> Fixture {
    let mut input = seeded_stable_docs_source_result_pack_and_citation_input();
    input.packet_id = "packet:stable_docs_contract:detail_sheet_actions".to_owned();
    update_projection_packet_refs(&mut input);
    if let Some(sheet) = input.pack_detail_sheets.first_mut() {
        sheet.actions.remove_action_ref = None;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: STABLE_DOCS_CONTRACT_SCHEMA_VERSION,
        case_name: "pack_detail_sheet_hides_actions_blocks_stable".to_owned(),
        scenario: "A docs-pack detail sheet hides required remove/update/offline/citation controls. The validator blocks promotion because owner, version, locale coverage, trust/support class, pin/offline state, and actions must be visible across pack detail sheets.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: StableDocsPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                StableDocsFindingKind::PackDetailSheetIncomplete.as_str(),
                StableDocsFindingKind::PackOwnerVersionActionVisibilityMissing.as_str(),
            ],
        },
    }
}

fn drawer_drift_fixture() -> Fixture {
    let mut input = seeded_stable_docs_source_result_pack_and_citation_input();
    input.packet_id = "packet:stable_docs_contract:drawer_drift".to_owned();
    update_projection_packet_refs(&mut input);
    if let Some(drawer) = input.citation_drawers.first_mut() {
        drawer.inference_markers.clear();
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: STABLE_DOCS_CONTRACT_SCHEMA_VERSION,
        case_name: "citation_drawer_drops_inference_marker_blocks_stable".to_owned(),
        scenario: "A citation drawer keeps source links but drops inference markers. The validator blocks promotion because drawer parity must preserve supporting file/symbol/docs anchors plus omitted-source and inference markers.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: StableDocsPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![StableDocsFindingKind::CitationDrawerParityDropped.as_str()],
        },
    }
}

fn projection_drift_fixture() -> Fixture {
    let mut input = seeded_stable_docs_source_result_pack_and_citation_input();
    input.packet_id = "packet:stable_docs_contract:projection_drift".to_owned();
    update_projection_packet_refs(&mut input);
    if let Some(projection) = input.consumer_projections.first_mut() {
        projection.preserves_source_precedence = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: STABLE_DOCS_CONTRACT_SCHEMA_VERSION,
        case_name: "consumer_projection_drops_precedence_blocks_stable".to_owned(),
        scenario: "A consumer projection preserves citations but drops source precedence. The validator blocks promotion because repo-local project docs must continue to outrank vendor docs for repo-specific explanations.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: StableDocsPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![StableDocsFindingKind::ConsumerProjectionDrift.as_str()],
        },
    }
}

fn update_projection_packet_refs(input: &mut StableDocsSourceResultPackCitationInput) {
    for projection in &mut input.consumer_projections {
        projection.packet_id_ref = input.packet_id.clone();
    }
}
