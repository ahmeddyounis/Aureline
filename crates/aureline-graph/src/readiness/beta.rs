//! Semantic graph readiness and exact-vs-imported fact label beta projection.
//!
//! This module is the canonical loader, validator, and reporter for
//! the beta graph-readiness contract. It binds each graph consumer
//! surface (`navigation`, `ai_context`, `review`, `support_export`)
//! to one claimed fact lane drawn from the closed beta vocabulary
//! (`exact_local_graph_fact`, `imported_graph_fact`,
//! `inferred_graph_fact`, `partial_graph_fact`, `stale_graph_fact`,
//! `waiting_on_graph_provider`, `out_of_scope_graph_fact`,
//! `fallback_search_fact`), the lane the underlying alpha graph fact
//! cue packet actually observed, the alpha readiness state, the
//! derived `claim_alignment_state` (`aligned`,
//! `weaker_claim_accepted`, `overclaim_blocked`), a metadata-safe
//! `evidence_export` projection, and a closed `downgrade_label`.
//!
//! Bound to the boundary schema at
//! [`/schemas/search/graph_readiness_beta.schema.json`](../../../../../schemas/search/graph_readiness_beta.schema.json),
//! the reviewer doc at
//! [`/docs/search/m3/graph_readiness_beta.md`](../../../../../docs/search/m3/graph_readiness_beta.md),
//! and the baseline report at
//! [`/artifacts/support/m3/graph_readiness_beta_report.md`](../../../../../artifacts/support/m3/graph_readiness_beta_report.md).

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for a graph-readiness beta case record.
pub const GRAPH_READINESS_BETA_CASE_RECORD_KIND: &str = "graph_readiness_beta_case_record";

/// Stable record-kind tag for the graph-readiness beta report record.
pub const GRAPH_READINESS_BETA_REPORT_RECORD_KIND: &str = "graph_readiness_beta_report_record";

/// Frozen schema version for the graph-readiness beta records.
pub const GRAPH_READINESS_BETA_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const GRAPH_READINESS_BETA_SCHEMA_REF: &str =
    "schemas/search/graph_readiness_beta.schema.json";

/// Repo-relative path of the reviewer doc.
pub const GRAPH_READINESS_BETA_DOC_REF: &str = "docs/search/m3/graph_readiness_beta.md";

/// Repo-relative path of the baseline report.
pub const GRAPH_READINESS_BETA_REPORT_REF: &str =
    "artifacts/support/m3/graph_readiness_beta_report.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const GRAPH_READINESS_BETA_CORPUS_DIR: &str = "fixtures/graph/m3/readiness_truth";

/// Repo-relative path of the protected corpus manifest.
pub const GRAPH_READINESS_BETA_CORPUS_MANIFEST_REF: &str =
    "fixtures/graph/m3/readiness_truth/manifest.yaml";

/// Closed graph-consumer-surface vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaConsumerSurface {
    Navigation,
    AiContext,
    Review,
    SupportExport,
}

impl BetaConsumerSurface {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Navigation => "navigation",
            Self::AiContext => "ai_context",
            Self::Review => "review",
            Self::SupportExport => "support_export",
        }
    }

    /// Returns true when this surface can project the given lane as an
    /// `aligned` claim. Other lanes degrade through
    /// `weaker_claim_accepted` or `overclaim_blocked`.
    pub fn accepts_as_aligned(self, lane: FactLane) -> bool {
        match self {
            Self::Navigation => matches!(lane, FactLane::ExactLocalGraphFact),
            Self::AiContext => matches!(
                lane,
                FactLane::ExactLocalGraphFact | FactLane::ImportedGraphFact
            ),
            Self::Review => matches!(
                lane,
                FactLane::ExactLocalGraphFact
                    | FactLane::ImportedGraphFact
                    | FactLane::InferredGraphFact
            ),
            // Support export preserves whatever the envelope carried;
            // alignment here means the label travels faithfully.
            Self::SupportExport => true,
        }
    }
}

/// Closed list of consumer surfaces the corpus must cover.
pub const REQUIRED_CONSUMER_SURFACES: [BetaConsumerSurface; 4] = [
    BetaConsumerSurface::Navigation,
    BetaConsumerSurface::AiContext,
    BetaConsumerSurface::Review,
    BetaConsumerSurface::SupportExport,
];

/// Closed fact-lane vocabulary shared by graph consumer surfaces and
/// the support export pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactLane {
    ExactLocalGraphFact,
    ImportedGraphFact,
    InferredGraphFact,
    PartialGraphFact,
    StaleGraphFact,
    WaitingOnGraphProvider,
    OutOfScopeGraphFact,
    FallbackSearchFact,
}

impl FactLane {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactLocalGraphFact => "exact_local_graph_fact",
            Self::ImportedGraphFact => "imported_graph_fact",
            Self::InferredGraphFact => "inferred_graph_fact",
            Self::PartialGraphFact => "partial_graph_fact",
            Self::StaleGraphFact => "stale_graph_fact",
            Self::WaitingOnGraphProvider => "waiting_on_graph_provider",
            Self::OutOfScopeGraphFact => "out_of_scope_graph_fact",
            Self::FallbackSearchFact => "fallback_search_fact",
        }
    }

    /// Lane strength index. Lower indices are stronger claims; a claim
    /// stronger than the observed envelope lane is an overclaim.
    pub const fn strength_index(self) -> u8 {
        match self {
            Self::ExactLocalGraphFact => 0,
            Self::ImportedGraphFact => 1,
            Self::InferredGraphFact => 2,
            Self::PartialGraphFact => 3,
            Self::StaleGraphFact => 4,
            Self::WaitingOnGraphProvider => 5,
            Self::OutOfScopeGraphFact => 6,
            Self::FallbackSearchFact => 7,
        }
    }
}

/// Closed list of fact lanes the corpus must cover as
/// `observed_envelope_lane`.
pub const REQUIRED_FACT_LANES: [FactLane; 8] = [
    FactLane::ExactLocalGraphFact,
    FactLane::ImportedGraphFact,
    FactLane::InferredGraphFact,
    FactLane::PartialGraphFact,
    FactLane::StaleGraphFact,
    FactLane::WaitingOnGraphProvider,
    FactLane::OutOfScopeGraphFact,
    FactLane::FallbackSearchFact,
];

/// Closed alpha-readiness vocabulary reused by the beta cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessClaim {
    Ready,
    HotSetReady,
    Partial,
    Warming,
    Stale,
    Unavailable,
    OutOfScope,
}

impl ReadinessClaim {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::HotSetReady => "hot_set_ready",
            Self::Partial => "partial",
            Self::Warming => "warming",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
            Self::OutOfScope => "out_of_scope",
        }
    }
}

/// Closed claim-alignment-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimAlignmentState {
    Aligned,
    WeakerClaimAccepted,
    OverclaimBlocked,
}

impl ClaimAlignmentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Aligned => "aligned",
            Self::WeakerClaimAccepted => "weaker_claim_accepted",
            Self::OverclaimBlocked => "overclaim_blocked",
        }
    }
}

/// Closed downgrade-label vocabulary; a failing row downgrades using
/// one of these labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeLabel {
    None,
    RedBlocksBetaRow,
    YellowFactLanePartial,
    YellowEvidenceExportSkew,
    DegradedToFallbackSearchOnly,
    StaleCorpusBlocksReleaseCandidate,
}

impl DowngradeLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RedBlocksBetaRow => "red_blocks_beta_row",
            Self::YellowFactLanePartial => "yellow_fact_lane_partial",
            Self::YellowEvidenceExportSkew => "yellow_evidence_export_skew",
            Self::DegradedToFallbackSearchOnly => "degraded_to_fallback_search_only",
            Self::StaleCorpusBlocksReleaseCandidate => "stale_corpus_blocks_release_candidate",
        }
    }

    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::None)
    }
}

/// Closed open-gap class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenGapClass {
    None,
    ConsumerSurfacePending,
    FactLanePending,
    EvidenceExportPending,
    FallbackTruthOnly,
    OverclaimBlocked,
}

impl OpenGapClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ConsumerSurfacePending => "consumer_surface_pending",
            Self::FactLanePending => "fact_lane_pending",
            Self::EvidenceExportPending => "evidence_export_pending",
            Self::FallbackTruthOnly => "fallback_truth_only",
            Self::OverclaimBlocked => "overclaim_blocked",
        }
    }
}

/// One open-gap row attached to a graph-readiness beta case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenGapEntry {
    pub gap_class: OpenGapClass,
    pub summary: String,
}

/// Metadata-safe evidence-export projection pinned on each case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceExportProjection {
    pub preserves_fact_lane_label: bool,
    pub preserves_readiness_token: bool,
    pub preserves_consumer_surface_label: bool,
    pub preserves_envelope_packet_ref: bool,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub preserves_user_authored_files: bool,
}

impl EvidenceExportProjection {
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            preserves_fact_lane_label: true,
            preserves_readiness_token: true,
            preserves_consumer_surface_label: true,
            preserves_envelope_packet_ref: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            preserves_user_authored_files: true,
        }
    }
}

/// Safety baseline pinned on every case and on the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseSafety {
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub destructive_resets_present: bool,
    pub preserves_user_authored_files: bool,
}

impl CaseSafety {
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
            preserves_user_authored_files: true,
        }
    }
}

/// Companion refs quoted on each case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseReferences {
    pub doc_ref: String,
    pub schema_ref: String,
    pub report_ref: String,
}

impl CaseReferences {
    pub fn pinned() -> Self {
        Self {
            doc_ref: GRAPH_READINESS_BETA_DOC_REF.to_owned(),
            schema_ref: GRAPH_READINESS_BETA_SCHEMA_REF.to_owned(),
            report_ref: GRAPH_READINESS_BETA_REPORT_REF.to_owned(),
        }
    }
}

/// One graph-readiness beta case record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphReadinessBetaCase {
    pub schema_version: u32,
    pub record_kind: String,
    pub case_id: String,
    pub title: String,
    pub consumer_surface: BetaConsumerSurface,
    pub subject_ref: String,
    pub envelope_packet_ref: String,
    pub claimed_fact_lane: FactLane,
    pub observed_envelope_lane: FactLane,
    pub observed_readiness: ReadinessClaim,
    pub claim_alignment_state: ClaimAlignmentState,
    pub evidence_export: EvidenceExportProjection,
    pub downgrade_label: DowngradeLabel,
    #[serde(default)]
    pub open_gaps: Vec<OpenGapEntry>,
    pub safety: CaseSafety,
    pub references: CaseReferences,
    pub captured_at: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reviewer_summary: Option<String>,
}

/// One fixture-bound entry in the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphReadinessBetaCorpusEntry {
    pub fixture_ref: String,
    pub case: GraphReadinessBetaCase,
}

/// Graph-readiness beta corpus loaded from checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphReadinessBetaCorpus {
    pub entries: Vec<GraphReadinessBetaCorpusEntry>,
}

/// One row in the report matrix projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportMatrixRow {
    pub case_id: String,
    pub consumer_surface: BetaConsumerSurface,
    pub subject_ref: String,
    pub claimed_fact_lane: FactLane,
    pub observed_envelope_lane: FactLane,
    pub observed_readiness: ReadinessClaim,
    pub claim_alignment_state: ClaimAlignmentState,
    pub downgrade_label: DowngradeLabel,
    pub open_gap_classes: Vec<OpenGapClass>,
}

impl ReportMatrixRow {
    fn from_case(case: &GraphReadinessBetaCase) -> Self {
        let mut open_gap_classes: Vec<OpenGapClass> =
            case.open_gaps.iter().map(|gap| gap.gap_class).collect();
        if open_gap_classes.is_empty() {
            open_gap_classes.push(OpenGapClass::None);
        }
        Self {
            case_id: case.case_id.clone(),
            consumer_surface: case.consumer_surface,
            subject_ref: case.subject_ref.clone(),
            claimed_fact_lane: case.claimed_fact_lane,
            observed_envelope_lane: case.observed_envelope_lane,
            observed_readiness: case.observed_readiness,
            claim_alignment_state: case.claim_alignment_state,
            downgrade_label: case.downgrade_label,
            open_gap_classes,
        }
    }
}

/// Per-fact-lane summary row of the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FactLaneSummaryRow {
    pub fact_lane: FactLane,
    pub case_count: u32,
    pub aligned_count: u32,
    pub weaker_claim_count: u32,
    pub overclaim_blocked_count: u32,
    pub downgrade_required_count: u32,
}

/// Per-consumer-surface summary row of the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerSurfaceSummaryRow {
    pub consumer_surface: BetaConsumerSurface,
    pub case_count: u32,
    pub aligned_count: u32,
    pub weaker_claim_count: u32,
    pub overclaim_blocked_count: u32,
    pub downgrade_required_count: u32,
}

/// Metadata-safe graph-readiness beta report record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphReadinessBetaReport {
    pub schema_version: u32,
    pub record_kind: String,
    pub report_id: String,
    pub captured_at: String,
    pub doc_ref: String,
    pub schema_ref: String,
    pub corpus_manifest_ref: String,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub required_consumer_surfaces: Vec<BetaConsumerSurface>,
    pub required_fact_lanes: Vec<FactLane>,
    pub matrix_rows: Vec<ReportMatrixRow>,
    pub fact_lane_summaries: Vec<FactLaneSummaryRow>,
    pub consumer_surface_summaries: Vec<ConsumerSurfaceSummaryRow>,
}

impl GraphReadinessBetaReport {
    pub fn is_export_safe(&self) -> bool {
        if !self.raw_private_material_excluded || !self.ambient_authority_excluded {
            return false;
        }
        if self.matrix_rows.is_empty() {
            return false;
        }
        if self.fact_lane_summaries.is_empty() || self.consumer_surface_summaries.is_empty() {
            return false;
        }
        true
    }
}

/// One validation violation emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphReadinessBetaViolation {
    pub check_id: String,
    pub subject_ref: String,
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphReadinessBetaValidationReport {
    pub violations: Vec<GraphReadinessBetaViolation>,
}

impl fmt::Display for GraphReadinessBetaValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} graph-readiness beta violation(s)",
            self.violations.len()
        )
    }
}

impl Error for GraphReadinessBetaValidationReport {}

/// Graph-readiness beta evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct GraphReadinessBetaEvaluator;

impl GraphReadinessBetaEvaluator {
    pub const fn new() -> Self {
        Self
    }

    pub fn validate_case(
        &self,
        case: &GraphReadinessBetaCase,
    ) -> Result<(), GraphReadinessBetaValidationReport> {
        let violations = validate_case(case);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(GraphReadinessBetaValidationReport { violations })
        }
    }

    pub fn validate_corpus(
        &self,
        corpus: &GraphReadinessBetaCorpus,
    ) -> Result<(), GraphReadinessBetaValidationReport> {
        let violations = validate_corpus(corpus);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(GraphReadinessBetaValidationReport { violations })
        }
    }

    pub fn report(
        &self,
        report_id: impl Into<String>,
        captured_at: impl Into<String>,
        corpus: &GraphReadinessBetaCorpus,
    ) -> Result<GraphReadinessBetaReport, GraphReadinessBetaValidationReport> {
        self.validate_corpus(corpus)?;
        let mut matrix_rows: Vec<ReportMatrixRow> = corpus
            .entries
            .iter()
            .map(|entry| ReportMatrixRow::from_case(&entry.case))
            .collect();
        matrix_rows.sort_by(|a, b| a.case_id.cmp(&b.case_id));

        let fact_lane_summaries = REQUIRED_FACT_LANES
            .iter()
            .map(|lane| summarize_fact_lane(corpus, *lane))
            .collect();
        let consumer_surface_summaries = REQUIRED_CONSUMER_SURFACES
            .iter()
            .map(|surface| summarize_consumer_surface(corpus, *surface))
            .collect();

        Ok(GraphReadinessBetaReport {
            schema_version: GRAPH_READINESS_BETA_SCHEMA_VERSION,
            record_kind: GRAPH_READINESS_BETA_REPORT_RECORD_KIND.to_owned(),
            report_id: report_id.into(),
            captured_at: captured_at.into(),
            doc_ref: GRAPH_READINESS_BETA_DOC_REF.to_owned(),
            schema_ref: GRAPH_READINESS_BETA_SCHEMA_REF.to_owned(),
            corpus_manifest_ref: GRAPH_READINESS_BETA_CORPUS_MANIFEST_REF.to_owned(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            required_consumer_surfaces: REQUIRED_CONSUMER_SURFACES.to_vec(),
            required_fact_lanes: REQUIRED_FACT_LANES.to_vec(),
            matrix_rows,
            fact_lane_summaries,
            consumer_surface_summaries,
        })
    }
}

fn summarize_fact_lane(corpus: &GraphReadinessBetaCorpus, lane: FactLane) -> FactLaneSummaryRow {
    let mut row = FactLaneSummaryRow {
        fact_lane: lane,
        case_count: 0,
        aligned_count: 0,
        weaker_claim_count: 0,
        overclaim_blocked_count: 0,
        downgrade_required_count: 0,
    };
    for entry in &corpus.entries {
        if entry.case.observed_envelope_lane != lane {
            continue;
        }
        row.case_count += 1;
        match entry.case.claim_alignment_state {
            ClaimAlignmentState::Aligned => row.aligned_count += 1,
            ClaimAlignmentState::WeakerClaimAccepted => row.weaker_claim_count += 1,
            ClaimAlignmentState::OverclaimBlocked => row.overclaim_blocked_count += 1,
        }
        if !entry.case.downgrade_label.is_healthy() {
            row.downgrade_required_count += 1;
        }
    }
    row
}

fn summarize_consumer_surface(
    corpus: &GraphReadinessBetaCorpus,
    surface: BetaConsumerSurface,
) -> ConsumerSurfaceSummaryRow {
    let mut row = ConsumerSurfaceSummaryRow {
        consumer_surface: surface,
        case_count: 0,
        aligned_count: 0,
        weaker_claim_count: 0,
        overclaim_blocked_count: 0,
        downgrade_required_count: 0,
    };
    for entry in &corpus.entries {
        if entry.case.consumer_surface != surface {
            continue;
        }
        row.case_count += 1;
        match entry.case.claim_alignment_state {
            ClaimAlignmentState::Aligned => row.aligned_count += 1,
            ClaimAlignmentState::WeakerClaimAccepted => row.weaker_claim_count += 1,
            ClaimAlignmentState::OverclaimBlocked => row.overclaim_blocked_count += 1,
        }
        if !entry.case.downgrade_label.is_healthy() {
            row.downgrade_required_count += 1;
        }
    }
    row
}

fn validate_corpus(corpus: &GraphReadinessBetaCorpus) -> Vec<GraphReadinessBetaViolation> {
    let mut violations = Vec::new();

    if corpus.entries.is_empty() {
        push_violation(
            &mut violations,
            "corpus.empty",
            GRAPH_READINESS_BETA_CORPUS_DIR,
            "corpus must contain at least one graph-readiness beta case",
        );
        return violations;
    }

    let mut case_ids: BTreeSet<String> = BTreeSet::new();
    let mut fixture_refs: BTreeSet<String> = BTreeSet::new();
    let mut seen_surfaces: BTreeSet<BetaConsumerSurface> = BTreeSet::new();
    let mut seen_envelope_lanes: BTreeSet<FactLane> = BTreeSet::new();
    let mut seen_overclaim_case = false;

    for entry in &corpus.entries {
        if !fixture_refs.insert(entry.fixture_ref.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_fixture_ref",
                &entry.fixture_ref,
                "fixture_ref must be unique within the corpus",
            );
        }
        let case = &entry.case;
        if !case_ids.insert(case.case_id.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_case_id",
                &case.case_id,
                "case_id must be unique within the corpus",
            );
        }
        seen_surfaces.insert(case.consumer_surface);
        seen_envelope_lanes.insert(case.observed_envelope_lane);
        if case.claim_alignment_state == ClaimAlignmentState::OverclaimBlocked {
            seen_overclaim_case = true;
        }
        violations.extend(validate_case(case));
    }

    for surface in REQUIRED_CONSUMER_SURFACES {
        if !seen_surfaces.contains(&surface) {
            push_violation(
                &mut violations,
                "corpus.required_consumer_surface_missing",
                surface.as_str(),
                format!(
                    "corpus must seed at least one case for consumer_surface = {}",
                    surface.as_str()
                ),
            );
        }
    }

    for lane in REQUIRED_FACT_LANES {
        if !seen_envelope_lanes.contains(&lane) {
            push_violation(
                &mut violations,
                "corpus.required_fact_lane_missing",
                lane.as_str(),
                format!(
                    "corpus must seed at least one case with observed_envelope_lane = {}",
                    lane.as_str()
                ),
            );
        }
    }

    if !seen_overclaim_case {
        push_violation(
            &mut violations,
            "corpus.overclaim_case_missing",
            GRAPH_READINESS_BETA_CORPUS_DIR,
            "corpus must seed at least one case with claim_alignment_state = overclaim_blocked so the overclaim-guard contract is exercised by a fixture",
        );
    }

    violations
}

fn validate_case(case: &GraphReadinessBetaCase) -> Vec<GraphReadinessBetaViolation> {
    let mut violations = Vec::new();
    let target = case.case_id.as_str();

    if case.schema_version != GRAPH_READINESS_BETA_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "case.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if case.record_kind != GRAPH_READINESS_BETA_CASE_RECORD_KIND {
        push_violation(
            &mut violations,
            "case.record_kind",
            target,
            format!("record_kind must be {GRAPH_READINESS_BETA_CASE_RECORD_KIND}"),
        );
    }
    for (field, value) in [
        ("case_id", case.case_id.as_str()),
        ("title", case.title.as_str()),
        ("subject_ref", case.subject_ref.as_str()),
        ("envelope_packet_ref", case.envelope_packet_ref.as_str()),
        ("captured_at", case.captured_at.as_str()),
    ] {
        if value.trim().is_empty() {
            push_violation(
                &mut violations,
                format!("case.{field}"),
                target,
                format!("{field} must be non-empty"),
            );
        }
    }

    validate_alignment(&mut violations, target, case);
    validate_outcome_and_downgrade(&mut violations, target, case);
    validate_evidence_export(&mut violations, target, &case.evidence_export);
    validate_open_gaps(&mut violations, target, &case.open_gaps);
    validate_safety(&mut violations, target, &case.safety);
    validate_references(&mut violations, target, &case.references);

    violations
}

fn derive_alignment_state(
    consumer_surface: BetaConsumerSurface,
    claimed_fact_lane: FactLane,
    observed_envelope_lane: FactLane,
) -> ClaimAlignmentState {
    if claimed_fact_lane.strength_index() < observed_envelope_lane.strength_index() {
        return ClaimAlignmentState::OverclaimBlocked;
    }
    if claimed_fact_lane == observed_envelope_lane
        && consumer_surface.accepts_as_aligned(claimed_fact_lane)
    {
        return ClaimAlignmentState::Aligned;
    }
    ClaimAlignmentState::WeakerClaimAccepted
}

fn validate_alignment(
    violations: &mut Vec<GraphReadinessBetaViolation>,
    target: &str,
    case: &GraphReadinessBetaCase,
) {
    let derived = derive_alignment_state(
        case.consumer_surface,
        case.claimed_fact_lane,
        case.observed_envelope_lane,
    );
    if derived != case.claim_alignment_state {
        push_violation(
            violations,
            "case.alignment.derived_state_mismatch",
            target,
            format!(
                "claim_alignment_state must be {} for consumer_surface = {}, claimed_fact_lane = {}, observed_envelope_lane = {}; got {}",
                derived.as_str(),
                case.consumer_surface.as_str(),
                case.claimed_fact_lane.as_str(),
                case.observed_envelope_lane.as_str(),
                case.claim_alignment_state.as_str()
            ),
        );
    }
}

fn validate_outcome_and_downgrade(
    violations: &mut Vec<GraphReadinessBetaViolation>,
    target: &str,
    case: &GraphReadinessBetaCase,
) {
    let healthy = case.downgrade_label.is_healthy();
    match (case.claim_alignment_state, healthy) {
        (ClaimAlignmentState::Aligned, false) => {
            push_violation(
                violations,
                "case.outcome.aligned_must_not_carry_downgrade",
                target,
                "claim_alignment_state = aligned must declare downgrade_label = none",
            );
        }
        (
            ClaimAlignmentState::WeakerClaimAccepted | ClaimAlignmentState::OverclaimBlocked,
            true,
        ) => {
            push_violation(
                violations,
                "case.outcome.non_aligned_must_declare_downgrade",
                target,
                "non-aligned claim_alignment_state must declare a non-none downgrade_label",
            );
        }
        _ => {}
    }
    if !healthy {
        let has_open_gap = case
            .open_gaps
            .iter()
            .any(|gap| gap.gap_class != OpenGapClass::None);
        if !has_open_gap {
            push_violation(
                violations,
                "case.outcome.non_aligned_must_record_open_gap",
                target,
                "downgraded rows must record at least one open_gap with a non-none gap_class",
            );
        }
    } else if case
        .open_gaps
        .iter()
        .any(|gap| gap.gap_class != OpenGapClass::None)
    {
        push_violation(
            violations,
            "case.outcome.aligned_must_not_record_open_gap",
            target,
            "aligned rows must not declare any open_gap with a non-none gap_class",
        );
    }
    if case.claim_alignment_state == ClaimAlignmentState::OverclaimBlocked {
        if case.downgrade_label != DowngradeLabel::RedBlocksBetaRow {
            push_violation(
                violations,
                "case.outcome.overclaim_must_be_red",
                target,
                "overclaim_blocked must downgrade with red_blocks_beta_row",
            );
        }
        let has_overclaim_gap = case
            .open_gaps
            .iter()
            .any(|gap| gap.gap_class == OpenGapClass::OverclaimBlocked);
        if !has_overclaim_gap {
            push_violation(
                violations,
                "case.outcome.overclaim_must_record_overclaim_gap",
                target,
                "overclaim_blocked must record an open_gap with gap_class = overclaim_blocked",
            );
        }
    }
}

fn validate_evidence_export(
    violations: &mut Vec<GraphReadinessBetaViolation>,
    target: &str,
    export: &EvidenceExportProjection,
) {
    if !export.preserves_fact_lane_label {
        push_violation(
            violations,
            "case.evidence_export.preserves_fact_lane_label",
            target,
            "evidence_export.preserves_fact_lane_label must be true",
        );
    }
    if !export.preserves_readiness_token {
        push_violation(
            violations,
            "case.evidence_export.preserves_readiness_token",
            target,
            "evidence_export.preserves_readiness_token must be true",
        );
    }
    if !export.preserves_consumer_surface_label {
        push_violation(
            violations,
            "case.evidence_export.preserves_consumer_surface_label",
            target,
            "evidence_export.preserves_consumer_surface_label must be true",
        );
    }
    if !export.preserves_envelope_packet_ref {
        push_violation(
            violations,
            "case.evidence_export.preserves_envelope_packet_ref",
            target,
            "evidence_export.preserves_envelope_packet_ref must be true",
        );
    }
    if !export.raw_private_material_excluded {
        push_violation(
            violations,
            "case.evidence_export.raw_private_material_excluded",
            target,
            "evidence_export.raw_private_material_excluded must be true",
        );
    }
    if !export.ambient_authority_excluded {
        push_violation(
            violations,
            "case.evidence_export.ambient_authority_excluded",
            target,
            "evidence_export.ambient_authority_excluded must be true",
        );
    }
    if !export.preserves_user_authored_files {
        push_violation(
            violations,
            "case.evidence_export.preserves_user_authored_files",
            target,
            "evidence_export.preserves_user_authored_files must be true",
        );
    }
}

fn validate_open_gaps(
    violations: &mut Vec<GraphReadinessBetaViolation>,
    target: &str,
    gaps: &[OpenGapEntry],
) {
    let mut seen: BTreeSet<OpenGapClass> = BTreeSet::new();
    for gap in gaps {
        if gap.summary.trim().is_empty() {
            push_violation(
                violations,
                "case.open_gaps.summary",
                target,
                "open_gaps.summary must be non-empty",
            );
        }
        if !seen.insert(gap.gap_class) {
            push_violation(
                violations,
                "case.open_gaps.duplicate_gap_class",
                target,
                format!("duplicate open_gap_class {}", gap.gap_class.as_str()),
            );
        }
    }
}

fn validate_safety(
    violations: &mut Vec<GraphReadinessBetaViolation>,
    target: &str,
    safety: &CaseSafety,
) {
    if !safety.raw_private_material_excluded {
        push_violation(
            violations,
            "case.safety.raw_private_material_excluded",
            target,
            "raw_private_material_excluded must be true",
        );
    }
    if !safety.ambient_authority_excluded {
        push_violation(
            violations,
            "case.safety.ambient_authority_excluded",
            target,
            "ambient_authority_excluded must be true",
        );
    }
    if safety.destructive_resets_present {
        push_violation(
            violations,
            "case.safety.destructive_resets_present",
            target,
            "destructive_resets_present must be false",
        );
    }
    if !safety.preserves_user_authored_files {
        push_violation(
            violations,
            "case.safety.preserves_user_authored_files",
            target,
            "preserves_user_authored_files must be true",
        );
    }
}

fn validate_references(
    violations: &mut Vec<GraphReadinessBetaViolation>,
    target: &str,
    refs: &CaseReferences,
) {
    if refs.doc_ref != GRAPH_READINESS_BETA_DOC_REF {
        push_violation(
            violations,
            "case.references.doc_ref",
            target,
            format!("references.doc_ref must pin {GRAPH_READINESS_BETA_DOC_REF}"),
        );
    }
    if refs.schema_ref != GRAPH_READINESS_BETA_SCHEMA_REF {
        push_violation(
            violations,
            "case.references.schema_ref",
            target,
            format!("references.schema_ref must pin {GRAPH_READINESS_BETA_SCHEMA_REF}"),
        );
    }
    if refs.report_ref != GRAPH_READINESS_BETA_REPORT_REF {
        push_violation(
            violations,
            "case.references.report_ref",
            target,
            format!("references.report_ref must pin {GRAPH_READINESS_BETA_REPORT_REF}"),
        );
    }
}

fn push_violation(
    violations: &mut Vec<GraphReadinessBetaViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(GraphReadinessBetaViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

/// Loads a YAML-encoded [`GraphReadinessBetaCase`].
pub fn load_graph_readiness_beta_case(
    yaml: &str,
) -> Result<GraphReadinessBetaCase, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Returns the checked-in graph-readiness beta corpus.
pub fn current_graph_readiness_beta_corpus(
) -> Result<GraphReadinessBetaCorpus, serde_yaml::Error> {
    let entries = CASE_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<GraphReadinessBetaCase>(yaml).map(|case| {
                GraphReadinessBetaCorpusEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    case,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(GraphReadinessBetaCorpus { entries })
}

/// Returns the set of fixture refs the corpus loads, in declaration
/// order.
pub fn current_graph_readiness_beta_fixture_refs() -> impl Iterator<Item = &'static str> {
    CASE_FIXTURES.iter().map(|(fixture_ref, _)| *fixture_ref)
}

const CASE_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/graph/m3/readiness_truth/navigation_exact_local_aligned_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/readiness_truth/navigation_exact_local_aligned_case.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/readiness_truth/review_imported_aligned_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/readiness_truth/review_imported_aligned_case.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/readiness_truth/ai_context_inferred_weaker_claim_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/readiness_truth/ai_context_inferred_weaker_claim_case.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/readiness_truth/support_export_stale_aligned_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/readiness_truth/support_export_stale_aligned_case.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/readiness_truth/ai_context_partial_weaker_claim_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/readiness_truth/ai_context_partial_weaker_claim_case.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/readiness_truth/navigation_waiting_weaker_claim_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/readiness_truth/navigation_waiting_weaker_claim_case.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/readiness_truth/navigation_out_of_scope_blocked_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/readiness_truth/navigation_out_of_scope_blocked_case.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/readiness_truth/ai_context_fallback_only_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/readiness_truth/ai_context_fallback_only_case.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/readiness_truth/navigation_exact_overclaim_blocked_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/readiness_truth/navigation_exact_overclaim_blocked_case.yaml"
        )),
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn aligned_navigation_case() -> GraphReadinessBetaCase {
        GraphReadinessBetaCase {
            schema_version: GRAPH_READINESS_BETA_SCHEMA_VERSION,
            record_kind: GRAPH_READINESS_BETA_CASE_RECORD_KIND.to_owned(),
            case_id: "case:test:aligned".to_owned(),
            title: "Aligned navigation test case".to_owned(),
            consumer_surface: BetaConsumerSurface::Navigation,
            subject_ref: "graph:symbol:test".to_owned(),
            envelope_packet_ref: "packet:test".to_owned(),
            claimed_fact_lane: FactLane::ExactLocalGraphFact,
            observed_envelope_lane: FactLane::ExactLocalGraphFact,
            observed_readiness: ReadinessClaim::Ready,
            claim_alignment_state: ClaimAlignmentState::Aligned,
            evidence_export: EvidenceExportProjection::metadata_safe_baseline(),
            downgrade_label: DowngradeLabel::None,
            open_gaps: vec![],
            safety: CaseSafety::metadata_safe_baseline(),
            references: CaseReferences::pinned(),
            captured_at: "2026-05-16T00:00:00Z".to_owned(),
            reviewer_summary: None,
        }
    }

    #[test]
    fn aligned_case_validates() {
        GraphReadinessBetaEvaluator::new()
            .validate_case(&aligned_navigation_case())
            .expect("aligned test case must validate");
    }

    #[test]
    fn refuses_overclaim_when_observed_envelope_is_weaker() {
        let mut case = aligned_navigation_case();
        case.observed_envelope_lane = FactLane::ImportedGraphFact;
        case.observed_readiness = ReadinessClaim::Stale;
        // Keep alignment claim wrong so that mismatch is caught first.
        let err = GraphReadinessBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("overclaim must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.alignment.derived_state_mismatch"));
    }

    #[test]
    fn refuses_aligned_with_downgrade_label() {
        let mut case = aligned_navigation_case();
        case.downgrade_label = DowngradeLabel::YellowFactLanePartial;
        let err = GraphReadinessBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("aligned with downgrade must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.outcome.aligned_must_not_carry_downgrade"));
    }

    #[test]
    fn refuses_aligned_with_open_gap() {
        let mut case = aligned_navigation_case();
        case.open_gaps.push(OpenGapEntry {
            gap_class: OpenGapClass::ConsumerSurfacePending,
            summary: "stray gap".into(),
        });
        let err = GraphReadinessBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("aligned with open gap must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.outcome.aligned_must_not_record_open_gap"));
    }

    #[test]
    fn refuses_overclaim_without_red_label() {
        let mut case = aligned_navigation_case();
        case.observed_envelope_lane = FactLane::ImportedGraphFact;
        case.observed_readiness = ReadinessClaim::Stale;
        case.claim_alignment_state = ClaimAlignmentState::OverclaimBlocked;
        case.downgrade_label = DowngradeLabel::YellowFactLanePartial;
        case.open_gaps.push(OpenGapEntry {
            gap_class: OpenGapClass::OverclaimBlocked,
            summary: "overclaim".into(),
        });
        let err = GraphReadinessBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("overclaim without red must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.outcome.overclaim_must_be_red"));
    }

    #[test]
    fn refuses_overclaim_without_overclaim_gap() {
        let mut case = aligned_navigation_case();
        case.observed_envelope_lane = FactLane::ImportedGraphFact;
        case.observed_readiness = ReadinessClaim::Stale;
        case.claim_alignment_state = ClaimAlignmentState::OverclaimBlocked;
        case.downgrade_label = DowngradeLabel::RedBlocksBetaRow;
        case.open_gaps.push(OpenGapEntry {
            gap_class: OpenGapClass::ConsumerSurfacePending,
            summary: "not overclaim".into(),
        });
        let err = GraphReadinessBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("overclaim without overclaim gap must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.outcome.overclaim_must_record_overclaim_gap"));
    }

    #[test]
    fn refuses_evidence_export_dropping_fact_lane_label() {
        let mut case = aligned_navigation_case();
        case.evidence_export.preserves_fact_lane_label = false;
        let err = GraphReadinessBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("dropping fact lane label must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.evidence_export.preserves_fact_lane_label"));
    }

    #[test]
    fn refuses_destructive_reset() {
        let mut case = aligned_navigation_case();
        case.safety.destructive_resets_present = true;
        let err = GraphReadinessBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("destructive reset must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.safety.destructive_resets_present"));
    }

    #[test]
    fn refuses_corpus_missing_required_consumer_surface() {
        let corpus = GraphReadinessBetaCorpus {
            entries: vec![GraphReadinessBetaCorpusEntry {
                fixture_ref: "fixtures/test/only_nav.yaml".to_owned(),
                case: aligned_navigation_case(),
            }],
        };
        let err = GraphReadinessBetaEvaluator::new()
            .validate_corpus(&corpus)
            .expect_err("missing required surfaces must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "corpus.required_consumer_surface_missing"));
    }

    #[test]
    fn refuses_corpus_missing_overclaim_case() {
        let corpus = GraphReadinessBetaCorpus {
            entries: vec![GraphReadinessBetaCorpusEntry {
                fixture_ref: "fixtures/test/only_nav.yaml".to_owned(),
                case: aligned_navigation_case(),
            }],
        };
        let err = GraphReadinessBetaEvaluator::new()
            .validate_corpus(&corpus)
            .expect_err("missing overclaim case must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "corpus.overclaim_case_missing"));
    }

    #[test]
    fn checked_in_corpus_loads_and_validates() {
        let corpus =
            current_graph_readiness_beta_corpus().expect("checked-in corpus must parse");
        GraphReadinessBetaEvaluator::new()
            .validate_corpus(&corpus)
            .expect("checked-in corpus must validate");
        for surface in REQUIRED_CONSUMER_SURFACES {
            assert!(
                corpus
                    .entries
                    .iter()
                    .any(|entry| entry.case.consumer_surface == surface),
                "checked-in corpus must seed a case for consumer_surface = {}",
                surface.as_str()
            );
        }
        for lane in REQUIRED_FACT_LANES {
            assert!(
                corpus
                    .entries
                    .iter()
                    .any(|entry| entry.case.observed_envelope_lane == lane),
                "checked-in corpus must seed a case with observed_envelope_lane = {}",
                lane.as_str()
            );
        }
        assert!(
            corpus
                .entries
                .iter()
                .any(|entry| entry.case.claim_alignment_state
                    == ClaimAlignmentState::OverclaimBlocked),
            "checked-in corpus must seed at least one overclaim_blocked case"
        );
    }

    #[test]
    fn checked_in_report_is_export_safe() {
        let corpus = current_graph_readiness_beta_corpus().unwrap();
        let report = GraphReadinessBetaEvaluator::new()
            .report("report:test", "2026-05-16T00:00:00Z", &corpus)
            .expect("report must build");
        assert!(report.is_export_safe());
        assert_eq!(report.matrix_rows.len(), corpus.entries.len());
        assert_eq!(
            report.fact_lane_summaries.len(),
            REQUIRED_FACT_LANES.len()
        );
        assert_eq!(
            report.consumer_surface_summaries.len(),
            REQUIRED_CONSUMER_SURFACES.len()
        );
        let total_lane: u32 = report
            .fact_lane_summaries
            .iter()
            .map(|s| s.case_count)
            .sum();
        assert_eq!(total_lane as usize, corpus.entries.len());
        let total_surface: u32 = report
            .consumer_surface_summaries
            .iter()
            .map(|s| s.case_count)
            .sum();
        assert_eq!(total_surface as usize, corpus.entries.len());
    }
}
