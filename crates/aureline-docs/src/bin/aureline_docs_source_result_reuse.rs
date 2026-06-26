//! Headless emitter for the stable docs-source/result object reuse packet.

use aureline_docs::{
    seeded_stable_docs_source_result_reuse_input, CitationSourceClass, DocsFreshnessClass,
    DocsMirrorOfflinePosture, DocsObjectFindingKind, DocsObjectPromotionState,
    DocsObjectReusePacket, DocsObjectReusePacketInput, DocsObjectTrustClass, SourcePrecedenceClass,
    DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION,
};
use serde::Serialize;

/// Stable export id pinned for the checked-in support export.
const SUPPORT_EXPORT_ID: &str = "support-export:stable_docs_source_and_result_object_reuse:001";
/// Stable export timestamp pinned for the checked-in support export.
const SUPPORT_EXPORTED_AT: &str = "2026-06-26T00:00:00Z";

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
    let packet = DocsObjectReusePacket::materialize(seeded_stable_docs_source_result_reuse_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsObjectReusePacket::materialize(seeded_stable_docs_source_result_reuse_input());
    let export = packet.support_export(SUPPORT_EXPORT_ID, SUPPORT_EXPORTED_AT);
    print_json(&export)
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "baseline_stable" => baseline_fixture(),
        "project_docs_relabeled_as_vendor_blocks_stable" => project_docs_relabeled_fixture(),
        "derived_explanation_claims_precedence_blocks_stable" => derived_precedence_fixture(),
        "live_external_inlined_without_handoff_blocks_stable" => live_external_inlined_fixture(),
        "result_freshness_drift_blocks_stable" => result_drift_fixture(),
        "consumer_projection_drops_truth_blocks_stable" => projection_drift_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsObjectReusePacket::materialize(seeded_stable_docs_source_result_reuse_input());
    if packet.promotion_state == DocsObjectPromotionState::Stable
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
    input: DocsObjectReusePacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

const FIXTURE_RECORD_KIND: &str = "docs_source_result_reuse_case";

fn baseline_fixture() -> Fixture {
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION,
        case_name: "baseline_stable".to_owned(),
        scenario: "Baseline stable packet proves one canonical docs-source descriptor and docs-result object per source class, reused across docs search, symbol-linked reference cards, hover/peek docs, AI citations, glossary cards, and support exports without re-deriving source/version/freshness truth.".to_owned(),
        input: seeded_stable_docs_source_result_reuse_input(),
        expect: ExpectedFixture {
            promotion_state: DocsObjectPromotionState::Stable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn project_docs_relabeled_fixture() -> Fixture {
    let mut input = seeded_stable_docs_source_result_reuse_input();
    input.packet_id =
        "packet:stable_docs_source_and_result_object_reuse:project_relabel".to_owned();
    if let Some(source) = input
        .sources
        .iter_mut()
        .find(|source| source.source_class == CitationSourceClass::ProjectDocs)
    {
        source.trust_class = DocsObjectTrustClass::LiveProviderHandoff;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION,
        case_name: "project_docs_relabeled_as_vendor_blocks_stable".to_owned(),
        scenario: "Project documentation is relabeled with a live-provider trust class. The validator blocks promotion because project docs must never masquerade as vendor docs across any consuming surface.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsObjectPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![DocsObjectFindingKind::SourceTrustClassMismatch.as_str()],
        },
    }
}

fn derived_precedence_fixture() -> Fixture {
    let mut input = seeded_stable_docs_source_result_reuse_input();
    input.packet_id =
        "packet:stable_docs_source_and_result_object_reuse:derived_precedence".to_owned();
    if let Some(source) = input
        .sources
        .iter_mut()
        .find(|source| source.source_class == CitationSourceClass::DerivedExplanation)
    {
        source.precedence_class = SourcePrecedenceClass::ProjectOutranksVendorDefault;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION,
        case_name: "derived_explanation_claims_precedence_blocks_stable".to_owned(),
        scenario: "A derived explanation claims source precedence over primary docs. The validator blocks promotion because derived explanations must never masquerade as primary documentation authority.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsObjectPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsObjectFindingKind::DerivedExplanationMasqueradesAsPrimary.as_str(),
            ],
        },
    }
}

fn live_external_inlined_fixture() -> Fixture {
    let mut input = seeded_stable_docs_source_result_reuse_input();
    input.packet_id = "packet:stable_docs_source_and_result_object_reuse:live_inlined".to_owned();
    if let Some(source) = input
        .sources
        .iter_mut()
        .find(|source| source.source_class == CitationSourceClass::VendorProviderDocs)
    {
        source.mirror_offline_posture = DocsMirrorOfflinePosture::CachedLocal;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION,
        case_name: "live_external_inlined_without_handoff_blocks_stable".to_owned(),
        scenario: "Live external docs are treated as a local cache instead of requiring an explicit browser handoff. The validator blocks promotion because live external docs must always resolve through an explicit, isolated handoff.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsObjectPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsObjectFindingKind::LiveExternalDocsHandoffMissing.as_str(),
            ],
        },
    }
}

fn result_drift_fixture() -> Fixture {
    let mut input = seeded_stable_docs_source_result_reuse_input();
    input.packet_id = "packet:stable_docs_source_and_result_object_reuse:result_drift".to_owned();
    if let Some(result) = input.results.first_mut() {
        result.freshness_state = DocsFreshnessClass::Unverified;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION,
        case_name: "result_freshness_drift_blocks_stable".to_owned(),
        scenario: "A result silently changes freshness from its source descriptor. The validator blocks promotion because every surface must read one source/version/freshness truth for the same object.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsObjectPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![DocsObjectFindingKind::SourceResultTruthMismatch.as_str()],
        },
    }
}

fn projection_drift_fixture() -> Fixture {
    let mut input = seeded_stable_docs_source_result_reuse_input();
    input.packet_id =
        "packet:stable_docs_source_and_result_object_reuse:projection_drift".to_owned();
    if let Some(projection) = input.surface_projections.first_mut() {
        projection.shows_source_class = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION,
        case_name: "consumer_projection_drops_truth_blocks_stable".to_owned(),
        scenario: "A consumer-surface projection stops showing the source class. The validator blocks promotion because every surface must keep source class, version match, freshness, trust class, and symbol/citation linkage visible.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsObjectPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsObjectFindingKind::ConsumerSurfaceProjectionDrift.as_str(),
            ],
        },
    }
}
