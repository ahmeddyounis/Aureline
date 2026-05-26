//! Headless emitter for the stable docs-browser truth packet and its
//! fixture corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_truth_packet -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_truth_packet -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_truth_packet -- fixture baseline_stable
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_truth_packet -- validate
//! ```

use aureline_docs::{
    seeded_stable_docs_browser_truth_packet_input, DocsBrowserConsumerSurface,
    DocsBrowserFindingKind, DocsBrowserPromotionState, DocsBrowserSourceClass,
    DocsBrowserSymbolFlowStep, DocsBrowserTruthPacket, DocsBrowserTruthPacketInput,
    DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION,
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
        Some("fixture") => emit_fixture(args.get(1).map(String::as_str))?,
        Some("validate") => validate_packet()?,
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn emit_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsBrowserTruthPacket::materialize(seeded_stable_docs_browser_truth_packet_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsBrowserTruthPacket::materialize(seeded_stable_docs_browser_truth_packet_input());
    let export = packet.support_export(
        "support-export:docs_browser_truth:001",
        "2026-05-26T12:00:10Z",
    );
    print_json(&export)
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "baseline_stable" => baseline_stable_fixture(),
        "missing_required_source_class_blocks_stable" => missing_source_class_fixture(),
        "symbol_flow_drops_split_step_blocks_stable" => symbol_flow_split_dropped_fixture(),
        "result_source_ref_unpinned_blocks_stable" => unpinned_source_ref_fixture(),
        "consumer_projection_drops_source_class_blocks_stable" => {
            projection_drops_source_class_fixture()
        }
        "live_external_handoff_missing_packet_blocks_stable" => {
            live_handoff_missing_packet_fixture()
        }
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsBrowserTruthPacket::materialize(seeded_stable_docs_browser_truth_packet_input());
    if packet.promotion_state == DocsBrowserPromotionState::Stable
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
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}

#[derive(Debug, Serialize)]
struct Fixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DocsBrowserTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

const FIXTURE_RECORD_KIND: &str = "docs_browser_truth_stable_case";

fn baseline_stable_fixture() -> Fixture {
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "baseline_stable".to_owned(),
        scenario:
            "Baseline stable posture: the packet binds every required source class \
             (project_docs, mirrored_official_docs, extension_docs_pack, \
             live_external_docs, derived_explanation), preserves docs-result \
             identity through every required symbol-flow step (peek, split, \
             browser_handoff, support_export, ai_handoff), and the nine required \
             consumer projections preserve the packet verbatim.".to_owned(),
        input: seeded_stable_docs_browser_truth_packet_input(),
        expect: ExpectedFixture {
            promotion_state: DocsBrowserPromotionState::Stable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn missing_source_class_fixture() -> Fixture {
    let mut input = seeded_stable_docs_browser_truth_packet_input();
    input.packet_id = "packet:m4:docs_browser_truth:missing_extension_pack".to_owned();
    input
        .sources
        .retain(|source| source.source_class != DocsBrowserSourceClass::ExtensionDocsPack);
    input
        .results
        .retain(|result| result.docs_source_ref != "src:extension_pack:python-stdlib");
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "missing_required_source_class_blocks_stable".to_owned(),
        scenario:
            "Required source-class coverage missing: the packet drops the \
             extension_docs_pack source descriptor and the validator blocks \
             promotion with required_source_class_coverage_missing."
                .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsBrowserPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsBrowserFindingKind::RequiredSourceClassCoverageMissing.as_str(),
            ],
        },
    }
}

fn symbol_flow_split_dropped_fixture() -> Fixture {
    let mut input = seeded_stable_docs_browser_truth_packet_input();
    input.packet_id = "packet:m4:docs_browser_truth:symbol_flow_drops_split".to_owned();
    if let Some(flow) = input.symbol_flows.first_mut() {
        flow.steps_preserving_identity
            .retain(|step| *step != DocsBrowserSymbolFlowStep::Split);
    }
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "symbol_flow_drops_split_step_blocks_stable".to_owned(),
        scenario:
            "Symbol flow identity lost: a symbol-linked docs flow drops the \
             split step from its preserved-identity list and the validator \
             blocks promotion with symbol_flow_identity_lost."
                .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsBrowserPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsBrowserFindingKind::SymbolFlowIdentityLost.as_str(),
            ],
        },
    }
}

fn unpinned_source_ref_fixture() -> Fixture {
    let mut input = seeded_stable_docs_browser_truth_packet_input();
    input.packet_id = "packet:m4:docs_browser_truth:result_source_ref_unpinned".to_owned();
    if let Some(result) = input.results.first_mut() {
        result.docs_source_ref = "src:ghost".to_owned();
    }
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "result_source_ref_unpinned_blocks_stable".to_owned(),
        scenario:
            "Result references an unpinned docs-source ref: a docs-result \
             object points at a source id that no descriptor declared and the \
             validator blocks promotion with result_source_ref_unpinned."
                .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsBrowserPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsBrowserFindingKind::ResultSourceRefUnpinned.as_str(),
            ],
        },
    }
}

fn projection_drops_source_class_fixture() -> Fixture {
    let mut input = seeded_stable_docs_browser_truth_packet_input();
    input.packet_id =
        "packet:m4:docs_browser_truth:projection_drops_source_class".to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    if let Some(projection) = input
        .consumer_projections
        .iter_mut()
        .find(|projection| projection.consumer_surface == DocsBrowserConsumerSurface::SupportExport)
    {
        projection.preserves_source_class = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "consumer_projection_drops_source_class_blocks_stable".to_owned(),
        scenario:
            "Consumer projection drops the source-class taxonomy: the support \
             export projection sets preserves_source_class = false and the \
             validator blocks promotion with source_class_taxonomy_dropped and \
             consumer_projection_drift."
                .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsBrowserPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsBrowserFindingKind::SourceClassTaxonomyDropped.as_str(),
                DocsBrowserFindingKind::ConsumerProjectionDrift.as_str(),
            ],
        },
    }
}

fn live_handoff_missing_packet_fixture() -> Fixture {
    let mut input = seeded_stable_docs_browser_truth_packet_input();
    input.packet_id =
        "packet:m4:docs_browser_truth:live_handoff_missing_packet".to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    if let Some(source) = input
        .sources
        .iter_mut()
        .find(|source| source.source_class == DocsBrowserSourceClass::LiveExternalDocs)
    {
        source.browser_handoff_packet_ref = None;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "live_external_handoff_missing_packet_blocks_stable".to_owned(),
        scenario:
            "Live external docs declare available browser handoff but the \
             descriptor drops its handoff packet ref. The validator blocks \
             promotion with browser_handoff_packet_missing."
                .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsBrowserPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsBrowserFindingKind::BrowserHandoffPacketMissing.as_str(),
            ],
        },
    }
}
