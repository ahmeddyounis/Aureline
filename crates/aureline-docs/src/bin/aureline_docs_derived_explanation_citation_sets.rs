//! Headless emitter for the derived-explanation citation-sets packet.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_derived_explanation_citation_sets -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_derived_explanation_citation_sets -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_derived_explanation_citation_sets -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_derived_explanation_citation_sets -- fixture baseline_stable
//! cargo run -q -p aureline-docs --bin aureline_docs_derived_explanation_citation_sets -- validate
//! ```

use aureline_docs::{
    seeded_stable_derived_explanation_citation_input, CitationRedactionState,
    DerivedExplanationCitationPacket, DerivedExplanationCitationPacketInput,
    DerivedExplanationCitationPromotionState, DerivedExplanationCitationValidationKind,
    DerivedExplanationSurface, DocsContractFreshnessState, DocsContractTrustClass,
    InferenceConfidence, DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
};
use serde::Serialize;

/// Stable export id pinned for the checked-in support export.
const SUPPORT_EXPORT_ID: &str = "support-export:derived_explanation_citation_sets:001";
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
        Some("summary") => emit_summary()?,
        Some("fixture") => emit_fixture(args.get(1).map(String::as_str))?,
        Some("validate") => validate_packet()?,
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn emit_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DerivedExplanationCitationPacket::materialize(
        seeded_stable_derived_explanation_citation_input(),
    );
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DerivedExplanationCitationPacket::materialize(
        seeded_stable_derived_explanation_citation_input(),
    );
    let export = packet.support_export(SUPPORT_EXPORT_ID, SUPPORT_EXPORTED_AT);
    print_json(&export)
}

fn emit_summary() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DerivedExplanationCitationPacket::materialize(
        seeded_stable_derived_explanation_citation_input(),
    );
    print!("{}", packet.render_markdown_summary());
    Ok(())
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "baseline_stable" => baseline_fixture(),
        "direct_citation_without_evidence_blocks_stable" => {
            direct_citation_without_evidence_fixture()
        }
        "inference_without_label_blocks_stable" => inference_without_label_fixture(),
        "inference_claims_authority_blocks_stable" => inference_claims_authority_fixture(),
        "redaction_drops_basis_blocks_stable" => redaction_drops_basis_fixture(),
        "surface_coverage_missing_blocks_stable" => surface_coverage_missing_fixture(),
        "support_export_drops_basis_blocks_stable" => support_export_drops_basis_fixture(),
        "projection_drops_reuse_blocks_stable" => projection_drops_reuse_fixture(),
        "stale_citation_narrows_below_stable" => stale_citation_narrows_fixture(),
        "speculative_inference_narrows_below_stable" => speculative_inference_narrows_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DerivedExplanationCitationPacket::materialize(
        seeded_stable_derived_explanation_citation_input(),
    );
    if packet.is_clean_stable() {
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
    input: DerivedExplanationCitationPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

const FIXTURE_RECORD_KIND: &str = "derived_explanation_citation_sets_case";

fn baseline_fixture() -> Fixture {
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
        case_name: "baseline_stable".to_owned(),
        scenario: "Baseline stable packet binds one citation set to every claimed derived-explanation surface (docs browser, AI answer, glossary card, guided tour, architecture explainer, support export). Direct citations name cited files, symbols, and docs nodes; the architecture explainer is an explicitly labeled inference; and the support export reuses every citation set so an export never drops a derived explanation's evidence basis.".to_owned(),
        input: seeded_stable_derived_explanation_citation_input(),
        expect: ExpectedFixture {
            promotion_state: DerivedExplanationCitationPromotionState::Stable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn direct_citation_without_evidence_fixture() -> Fixture {
    let mut input = relabel("direct_citation_without_evidence");
    if let Some(set) = input
        .citation_sets
        .iter_mut()
        .find(|set| set.explanation_surface == DerivedExplanationSurface::AiAnswer)
    {
        set.cited_files.clear();
        set.cited_symbols.clear();
        set.cited_docs.clear();
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
        case_name: "direct_citation_without_evidence_blocks_stable".to_owned(),
        scenario: "A direct-citation AI answer drops every cited file, symbol, and docs node. The validator blocks promotion because prose claiming a direct citation must name the evidence it actually depended on.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DerivedExplanationCitationPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DerivedExplanationCitationValidationKind::CitationBasisMissing.as_str(),
            ],
        },
    }
}

fn inference_without_label_fixture() -> Fixture {
    let mut input = relabel("inference_without_label");
    if let Some(set) = input
        .citation_sets
        .iter_mut()
        .find(|set| set.explanation_surface == DerivedExplanationSurface::ArchitectureExplainer)
    {
        set.inference_label = None;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
        case_name: "inference_without_label_blocks_stable".to_owned(),
        scenario: "A labeled-inference architecture explainer drops its inference label. The validator blocks promotion because an explanation with no direct citation must explicitly label itself an inference and name why no direct citation exists.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DerivedExplanationCitationPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DerivedExplanationCitationValidationKind::InferenceLabelMissing.as_str(),
            ],
        },
    }
}

fn inference_claims_authority_fixture() -> Fixture {
    let mut input = relabel("inference_claims_authority");
    if let Some(set) = input
        .citation_sets
        .iter_mut()
        .find(|set| set.explanation_surface == DerivedExplanationSurface::ArchitectureExplainer)
    {
        set.trust_class = DocsContractTrustClass::FirstPartyAuthoritative;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
        case_name: "inference_claims_authority_blocks_stable".to_owned(),
        scenario: "A labeled inference records first-party authoritative trust. The validator blocks promotion because a derived inference never claims primary authority; its trust class must stay derived-inference-only.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DerivedExplanationCitationPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DerivedExplanationCitationValidationKind::BasisTrustInconsistent.as_str(),
            ],
        },
    }
}

fn redaction_drops_basis_fixture() -> Fixture {
    let mut input = relabel("redaction_drops_basis");
    if let Some(set) = input
        .citation_sets
        .iter_mut()
        .find(|set| set.explanation_surface == DerivedExplanationSurface::SupportExportNote)
    {
        set.redaction = CitationRedactionState::ContentOmittedBasisPreserved;
        set.cited_files.clear();
        set.cited_symbols.clear();
        set.cited_docs.clear();
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
        case_name: "redaction_drops_basis_blocks_stable".to_owned(),
        scenario: "A support-export note omits its cited content and also drops every citation ref. The validator blocks promotion because redaction may withhold content but must always preserve the citation basis (refs, graph epoch, derivation).".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DerivedExplanationCitationPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DerivedExplanationCitationValidationKind::RedactionDropsCitationBasis.as_str(),
            ],
        },
    }
}

fn surface_coverage_missing_fixture() -> Fixture {
    let mut input = relabel("surface_coverage_missing");
    input
        .citation_sets
        .retain(|set| set.explanation_surface != DerivedExplanationSurface::GlossaryCard);
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
        case_name: "surface_coverage_missing_blocks_stable".to_owned(),
        scenario: "The glossary card surface loses its citation set. The validator blocks promotion because every claimed derived-explanation surface must attach one citation set.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DerivedExplanationCitationPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DerivedExplanationCitationValidationKind::SurfaceCoverageMissing.as_str(),
            ],
        },
    }
}

fn support_export_drops_basis_fixture() -> Fixture {
    let mut input = relabel("support_export_drops_basis");
    if let Some(projection) = input
        .consumer_projections
        .iter_mut()
        .find(|projection| projection.surface == DerivedExplanationSurface::SupportExportNote)
    {
        // Drop the docs-browser citation set from the export basis.
        projection
            .citation_set_id_refs
            .retain(|set_ref| set_ref != "citation-set:docs_browser:tokio-spawn");
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
        case_name: "support_export_drops_basis_blocks_stable".to_owned(),
        scenario: "The support-export projection stops referencing one citation set. The validator blocks promotion because a support export must preserve the citation basis of every derived explanation, not a subset.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DerivedExplanationCitationPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DerivedExplanationCitationValidationKind::SupportExportDropsCitationBasis.as_str(),
            ],
        },
    }
}

fn projection_drops_reuse_fixture() -> Fixture {
    let mut input = relabel("projection_drops_reuse");
    if let Some(projection) = input
        .consumer_projections
        .iter_mut()
        .find(|projection| projection.surface == DerivedExplanationSurface::AiAnswer)
    {
        projection.reuses_shared_citation_object = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
        case_name: "projection_drops_reuse_blocks_stable".to_owned(),
        scenario: "The AI surface stops reusing the shared citation object. The validator blocks promotion because surfaces must reuse the same citation object instead of inventing prose-only private explanation state.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DerivedExplanationCitationPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DerivedExplanationCitationValidationKind::ConsumerProjectionDropsReuse.as_str(),
            ],
        },
    }
}

fn stale_citation_narrows_fixture() -> Fixture {
    let mut input = relabel("stale_citation_narrows");
    if let Some(set) = input
        .citation_sets
        .iter_mut()
        .find(|set| set.explanation_surface == DerivedExplanationSurface::DocsBrowserExplanation)
    {
        set.freshness = DocsContractFreshnessState::Stale;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
        case_name: "stale_citation_narrows_below_stable".to_owned(),
        scenario: "A direct citation rests on stale freshness. The packet narrows below stable rather than blocking, because the citation basis still exists but the explanation must not claim current authority.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DerivedExplanationCitationPromotionState::NarrowedBelowStable.as_str(),
            expected_finding_kinds: vec![
                DerivedExplanationCitationValidationKind::CitationFreshnessNarrowed.as_str(),
            ],
        },
    }
}

fn speculative_inference_narrows_fixture() -> Fixture {
    let mut input = relabel("speculative_inference_narrows");
    if let Some(set) = input
        .citation_sets
        .iter_mut()
        .find(|set| set.explanation_surface == DerivedExplanationSurface::ArchitectureExplainer)
    {
        if let Some(label) = set.inference_label.as_mut() {
            label.confidence = InferenceConfidence::Speculative;
        }
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
        case_name: "speculative_inference_narrows_below_stable".to_owned(),
        scenario: "The architecture explainer marks its inference speculative. The packet narrows below stable rather than blocking, because the inference is still explicitly labeled but reads as low confidence.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DerivedExplanationCitationPromotionState::NarrowedBelowStable.as_str(),
            expected_finding_kinds: vec![
                DerivedExplanationCitationValidationKind::SpeculativeInferenceNarrowed.as_str(),
            ],
        },
    }
}

/// Reseeds the packet id with a per-fixture suffix and realigns each projection's
/// `packet_id_ref` so a failing fixture isolates the single invariant it
/// exercises.
fn relabel(suffix: &str) -> DerivedExplanationCitationPacketInput {
    let mut input = seeded_stable_derived_explanation_citation_input();
    let packet_id = format!("packet:derived_explanation_citation_sets:{suffix}");
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.clone();
    }
    input.packet_id = packet_id;
    input
}
