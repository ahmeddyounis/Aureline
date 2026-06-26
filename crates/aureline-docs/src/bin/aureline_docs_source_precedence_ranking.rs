//! Headless emitter for the docs-source precedence/ranking parity packet.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_source_precedence_ranking -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_source_precedence_ranking -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_source_precedence_ranking -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_source_precedence_ranking -- fixture baseline_stable
//! cargo run -q -p aureline-docs --bin aureline_docs_source_precedence_ranking -- validate
//! ```

use aureline_docs::{
    seeded_stable_docs_precedence_ranking_input, DocsPrecedenceRankingFindingKind,
    DocsPrecedenceRankingPacket, DocsPrecedenceRankingPacketInput,
    DocsPrecedenceRankingPromotionState, DocsSourceLane, PrecedenceReason, RankExplanationSurface,
    SourcePrecedenceClass, DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
};
use serde::Serialize;

/// Stable export id pinned for the checked-in support export.
const SUPPORT_EXPORT_ID: &str = "support-export:docs_source_precedence_and_ranking_parity:001";
/// Stable export timestamp pinned for the checked-in support export.
const SUPPORT_EXPORTED_AT: &str = "2026-06-26T00:00:00Z";

/// Candidate id of the project-docs candidate in the repo-specific ranking set.
const PROJECT_CANDIDATE_ID: &str = "candidate:repo:project-runbook";
/// Candidate id of the mirrored-official-docs candidate in the repo-specific set.
const MIRROR_CANDIDATE_ID: &str = "candidate:repo:mirror-std";
/// Candidate id of the derived-explanation candidate in the repo-specific set.
const DERIVED_CANDIDATE_ID: &str = "candidate:repo:derived-summary";

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
    let packet =
        DocsPrecedenceRankingPacket::materialize(seeded_stable_docs_precedence_ranking_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsPrecedenceRankingPacket::materialize(seeded_stable_docs_precedence_ranking_input());
    let export = packet.support_export(SUPPORT_EXPORT_ID, SUPPORT_EXPORTED_AT);
    print_json(&export)
}

fn emit_summary() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsPrecedenceRankingPacket::materialize(seeded_stable_docs_precedence_ranking_input());
    print!("{}", packet.render_markdown_summary());
    Ok(())
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "baseline_stable" => baseline_fixture(),
        "source_lane_flattened_blocks_stable" => source_lane_flattened_fixture(),
        "project_masquerades_as_vendor_blocks_stable" => project_masquerades_fixture(),
        "unexplained_rank_inversion_blocks_stable" => unexplained_rank_inversion_fixture(),
        "outrank_without_visible_alternative_blocks_stable" => {
            outrank_without_visible_alternative_fixture()
        }
        "derived_ranked_as_primary_blocks_stable" => derived_ranked_as_primary_fixture(),
        "reason_class_mismatch_blocks_stable" => reason_class_mismatch_fixture(),
        "hidden_ranking_model_blocks_stable" => hidden_ranking_model_fixture(),
        "offline_unavailable_reason_missing_blocks_stable" => {
            offline_unavailable_reason_missing_fixture()
        }
        "missing_rank_explanation_surface_blocks_stable" => {
            missing_rank_explanation_surface_fixture()
        }
        "air_gapped_candidate_narrows_below_stable" => air_gapped_candidate_narrows_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsPrecedenceRankingPacket::materialize(seeded_stable_docs_precedence_ranking_input());
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
    input: DocsPrecedenceRankingPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

const FIXTURE_RECORD_KIND: &str = "docs_source_precedence_and_ranking_parity_case";

fn baseline_fixture() -> Fixture {
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
        case_name: "baseline_stable".to_owned(),
        scenario: "Baseline stable packet ranks two subjects across seven distinguishable source lanes. For the repo-specific question project docs outrank the mirrored and live-external alternatives, but both stay visible and referenced; every candidate carries a precedence reason and note; the derived explanation never ranks primary; and docs search, hover/peek, onboarding, AI context, and support export project the same ranking explanation.".to_owned(),
        input: seeded_stable_docs_precedence_ranking_input(),
        expect: ExpectedFixture {
            promotion_state: DocsPrecedenceRankingPromotionState::Stable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn source_lane_flattened_fixture() -> Fixture {
    let mut input = relabel("source_lane_flattened");
    // Drop every derived-explanation candidate so the seventh lane disappears.
    for set in input.ranking_sets.iter_mut() {
        set.candidates
            .retain(|candidate| candidate.lane != DocsSourceLane::DerivedExplanation);
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
        case_name: "source_lane_flattened_blocks_stable".to_owned(),
        scenario: "The derived-explanation lane is dropped, flattening the source set. The validator blocks promotion because the seven source lanes must each stay distinguishable.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPrecedenceRankingPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPrecedenceRankingFindingKind::SourceClassDistinguishabilityMissing.as_str(),
            ],
        },
    }
}

fn project_masquerades_fixture() -> Fixture {
    let mut input = relabel("project_masquerades_as_vendor");
    if let Some(candidate) = find_candidate(&mut input, PROJECT_CANDIDATE_ID) {
        // Project docs labelled with a live-provider trust class: no real lane.
        candidate.trust_class = aureline_docs::DocsObjectTrustClass::LiveProviderHandoff;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
        case_name: "project_masquerades_as_vendor_blocks_stable".to_owned(),
        scenario: "A project-docs candidate is labelled with a live-provider trust class so it would masquerade as vendor docs. The validator blocks promotion because that source/trust pair resolves to no distinguishable lane.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPrecedenceRankingPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPrecedenceRankingFindingKind::CandidateLaneUnresolved.as_str(),
            ],
        },
    }
}

fn unexplained_rank_inversion_fixture() -> Fixture {
    let mut input = relabel("unexplained_rank_inversion");
    if let Some(candidate) = find_candidate(&mut input, PROJECT_CANDIDATE_ID) {
        // Project still ranks above the more-authoritative mirror, but now claims
        // a reason that does not justify the inversion.
        candidate.precedence_reason = PrecedenceReason::OfficialUpstreamAuthority;
        candidate.precedence_class = SourcePrecedenceClass::NotApplicable;
        candidate.outranks_refs.clear();
        candidate.disclosure_note = Some("Project answer, no override claimed.".to_owned());
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
        case_name: "unexplained_rank_inversion_blocks_stable".to_owned(),
        scenario: "Project docs keep the top rank above the more-authoritative mirrored docs but no longer carry a reason that justifies outranking them. The validator blocks promotion because a less-authoritative source may outrank a more-authoritative one only with an explicit justifying reason.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPrecedenceRankingPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPrecedenceRankingFindingKind::UnexplainedRankInversion.as_str(),
            ],
        },
    }
}

fn outrank_without_visible_alternative_fixture() -> Fixture {
    let mut input = relabel("outrank_without_visible_alternative");
    if let Some(set) = input
        .ranking_sets
        .iter_mut()
        .find(|set| set.subject_id == aureline_docs_repo_subject_id())
    {
        set.candidates
            .retain(|candidate| !candidate.lane.is_vendor_alternative());
    }
    if let Some(candidate) = find_candidate(&mut input, PROJECT_CANDIDATE_ID) {
        candidate.outranks_refs.clear();
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
        case_name: "outrank_without_visible_alternative_blocks_stable".to_owned(),
        scenario: "Project docs claim to outrank vendor docs but the mirrored and live-external alternatives are removed from the set. The validator blocks promotion because project docs may outrank vendor docs only while keeping the vendor difference visible.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPrecedenceRankingPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPrecedenceRankingFindingKind::OutrankWithoutVisibleAlternative.as_str(),
            ],
        },
    }
}

fn derived_ranked_as_primary_fixture() -> Fixture {
    let mut input = relabel("derived_ranked_as_primary");
    if let Some(candidate) = find_candidate(&mut input, DERIVED_CANDIDATE_ID) {
        candidate.rank_position = 1;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
        case_name: "derived_ranked_as_primary_blocks_stable".to_owned(),
        scenario: "A derived explanation is promoted to rank 1. The validator blocks promotion because a derived explanation never claims primary authority.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPrecedenceRankingPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPrecedenceRankingFindingKind::DerivedExplanationRankedAsPrimary.as_str(),
            ],
        },
    }
}

fn reason_class_mismatch_fixture() -> Fixture {
    let mut input = relabel("reason_class_mismatch");
    if let Some(candidate) = find_candidate(&mut input, MIRROR_CANDIDATE_ID) {
        // Vendor-override reason while still declaring a non-override class.
        candidate.precedence_reason = PrecedenceReason::VendorOverridePolicy;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
        case_name: "reason_class_mismatch_blocks_stable".to_owned(),
        scenario: "A candidate claims a vendor-override reason while declaring a non-override precedence class. The validator blocks promotion because the precedence reason must stay consistent with the precedence class.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPrecedenceRankingPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPrecedenceRankingFindingKind::PrecedenceReasonClassMismatch.as_str(),
            ],
        },
    }
}

fn hidden_ranking_model_fixture() -> Fixture {
    let mut input = relabel("hidden_ranking_model");
    if let Some(projection) = input
        .surface_projections
        .iter_mut()
        .find(|projection| projection.surface == RankExplanationSurface::AiContext)
    {
        projection.uses_shared_ranking_vocabulary = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
        case_name: "hidden_ranking_model_blocks_stable".to_owned(),
        scenario: "The AI-context surface stops reusing the shared ranking vocabulary and mints a hidden ranking model. The validator blocks promotion because no surface may run a second ranking model that ignores source-class, version-match, or freshness truth.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPrecedenceRankingPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPrecedenceRankingFindingKind::HiddenRankingModel.as_str(),
            ],
        },
    }
}

fn offline_unavailable_reason_missing_fixture() -> Fixture {
    let mut input = relabel("offline_unavailable_reason_missing");
    if let Some(candidate) = find_candidate(&mut input, MIRROR_CANDIDATE_ID) {
        candidate.available_in_offline_profile = false;
        candidate.unavailable_reason = None;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
        case_name: "offline_unavailable_reason_missing_blocks_stable".to_owned(),
        scenario: "A candidate is unavailable in an offline profile but gives no reason. The validator blocks promotion because an offline / air-gapped profile must keep candidates inspectable with an explicit unavailable reason rather than silently dropping them.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPrecedenceRankingPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPrecedenceRankingFindingKind::OfflineUnavailableReasonMissing.as_str(),
            ],
        },
    }
}

fn missing_rank_explanation_surface_fixture() -> Fixture {
    let mut input = relabel("missing_rank_explanation_surface");
    input
        .surface_projections
        .retain(|projection| projection.surface != RankExplanationSurface::Onboarding);
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
        case_name: "missing_rank_explanation_surface_blocks_stable".to_owned(),
        scenario: "The onboarding surface loses its ranking-explanation projection. The validator blocks promotion because the ranking explanation must stay inspectable across docs search, hover/peek, onboarding, AI context, and support export.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPrecedenceRankingPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPrecedenceRankingFindingKind::MissingRankExplanationSurface.as_str(),
            ],
        },
    }
}

fn air_gapped_candidate_narrows_fixture() -> Fixture {
    let mut input = relabel("air_gapped_candidate_narrows");
    if let Some(candidate) = find_candidate(&mut input, MIRROR_CANDIDATE_ID) {
        candidate.available_in_offline_profile = false;
        candidate.unavailable_reason =
            Some("Mirror pack is not installed in this air-gapped profile.".to_owned());
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
        case_name: "air_gapped_candidate_narrows_below_stable".to_owned(),
        scenario: "A candidate is unavailable in an air-gapped profile but honestly discloses why. The packet narrows below stable rather than blocking, because offline inspectability with an explicit unavailable reason is degraded but honest.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPrecedenceRankingPromotionState::NarrowedBelowStable.as_str(),
            expected_finding_kinds: vec![
                DocsPrecedenceRankingFindingKind::AirGappedCandidateNarrowed.as_str(),
            ],
        },
    }
}

/// Subject id of the repo-specific ranking set in the seed.
fn aureline_docs_repo_subject_id() -> String {
    seeded_stable_docs_precedence_ranking_input().ranking_sets[0]
        .subject_id
        .clone()
}

/// Finds a candidate across every ranking set by id.
fn find_candidate<'a>(
    input: &'a mut DocsPrecedenceRankingPacketInput,
    candidate_id: &str,
) -> Option<&'a mut aureline_docs::RankedDocsCandidate> {
    input
        .ranking_sets
        .iter_mut()
        .flat_map(|set| set.candidates.iter_mut())
        .find(|candidate| candidate.candidate_id == candidate_id)
}

/// Reseeds the packet id with a per-fixture suffix and realigns each projection's
/// `ranking_set_ref` is untouched (it references subject ids, not the packet id),
/// so a failing fixture isolates the single invariant it exercises.
fn relabel(suffix: &str) -> DocsPrecedenceRankingPacketInput {
    let mut input = seeded_stable_docs_precedence_ranking_input();
    input.packet_id = format!("packet:docs_source_precedence_and_ranking_parity:{suffix}");
    input
}
