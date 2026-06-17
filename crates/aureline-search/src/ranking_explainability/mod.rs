//! User-visible ranking explainers, withheld/policy-hidden/partial-index cues,
//! and query-debug truth across the M5 search surfaces.
//!
//! Where [`crate::result_truth_packet`] owns the *structured* result identity,
//! ranking-reason, and action-binding contract, and
//! [`crate::query_session_first_consumers`] *materializes* one durable session
//! per surface, this module promotes the ranking explanation from debug-only
//! metadata into a first-class, **user-visible explain sheet** and makes
//! withheld or policy-hidden candidates inspectable instead of silently
//! dropping them.
//!
//! Two row shapes carry the truth:
//!
//! - [`RankingExplainSheet`] is the `Why this result?` sheet for one *visible*
//!   row. It reuses the canonical [`RankingReason`] verbatim (so the UI, support
//!   export, and replay/debug all read one explanation object) and projects a
//!   user-facing headline, prose reason lines, and a partiality caveat. Its
//!   [`ExplainStateClass`] names whether the row was promoted, suppressed, tied,
//!   or answered off a partial index.
//! - [`OmittedCandidateRow`] makes a *non-surfaced* candidate visible: a
//!   candidate withheld for a latency budget, hidden by trust/policy posture, or
//!   not yet indexed. It carries the omission reason, a count, and the answering
//!   source stratum — never literal query text — so policy-hidden and
//!   withheld candidates stop being silent omissions.
//!
//! [`SearchExplainSurfaceRow`] binds the visible sheets and omitted rows to one
//! of the five claimed M5 surfaces ([`ExplainSurfaceClass`]: palette, sidebars,
//! docs results, graph-backed results, and saved-query reopen), and the
//! [`RankingExplainabilityPacket`] proves the same explanation object is reused
//! by the product UI, support export, and replay/debug consumers
//! ([`ExplainabilityConsumerClass`]).
//!
//! The packet is metadata-only by construction. Sessions are referenced
//! hash-only, omitted rows exclude literal query text, and the export posture
//! ([`ExportConsentClass`]) keeps hashes, counts, reason summaries, and omission
//! reasons exportable while gating literal query text behind elevated consent.
//! The guardrail is truthful reasons, not raw model weights: every sheet asserts
//! that raw numeric score weights are excluded.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::query_session::stable_query_hash;
use crate::result_id::{build_canonical_result_id, StableResultKind};
use crate::result_truth_packet::{
    FactLabelClass, RankingReason, RankingSignalClass, ScopeCounters, SourceStratumClass,
    TieBreakClass, SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF,
};

/// Stable record-kind tag for [`RankingExplainabilityPacket`].
pub const RANKING_EXPLAINABILITY_PACKET_RECORD_KIND: &str = "search_ranking_explainability_packet";

/// Stable record-kind tag for [`RankingExplainabilitySupportExport`].
pub const RANKING_EXPLAINABILITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "search_ranking_explainability_support_export";

/// Integer schema version for the ranking-explainability packet.
pub const RANKING_EXPLAINABILITY_SCHEMA_VERSION: u32 = 1;

/// Stable packet identifier reused by every consumer projection.
pub const RANKING_EXPLAINABILITY_PACKET_ID: &str = "search.m5.ranking_explainability.v1";

/// Repository-relative path of the boundary schema.
pub const RANKING_EXPLAINABILITY_SCHEMA_REF: &str =
    "schemas/search/ranking-explainability.schema.json";

/// Repository-relative path of the reviewer-facing contract document.
pub const RANKING_EXPLAINABILITY_DOC_REF: &str = "docs/search/ranking-explainability.md";

/// Repository-relative path of the checked review artifact.
pub const RANKING_EXPLAINABILITY_ARTIFACT_REF: &str =
    "artifacts/search/m5/ranking-explainability.md";

/// Repository-relative path of the protected fixture directory.
pub const RANKING_EXPLAINABILITY_FIXTURE_DIR: &str = "fixtures/search/m5/ranking-partiality";

/// Workspace id used by the seeded corpus.
const SEEDED_WORKSPACE_ID: &str = "ws-aureline";

/// Fixed generation timestamp for the seeded corpus.
const SEEDED_GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// One claimed M5 search surface that keeps inspectable ranking explainers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainSurfaceClass {
    /// Quick-open / command-palette result list.
    Palette,
    /// Sidebar search, outline, and reference panes.
    Sidebar,
    /// Documentation and help result list.
    DocsResults,
    /// Graph-backed result list (references, callers, neighbors).
    GraphBackedResults,
    /// Reopening a saved query and replaying its results.
    SavedQueryReopen,
}

impl ExplainSurfaceClass {
    /// Every claimed surface, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Palette,
        Self::Sidebar,
        Self::DocsResults,
        Self::GraphBackedResults,
        Self::SavedQueryReopen,
    ];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Palette => "palette",
            Self::Sidebar => "sidebar",
            Self::DocsResults => "docs_results",
            Self::GraphBackedResults => "graph_backed_results",
            Self::SavedQueryReopen => "saved_query_reopen",
        }
    }

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Palette => "Palette",
            Self::Sidebar => "Sidebar",
            Self::DocsResults => "Docs results",
            Self::GraphBackedResults => "Graph-backed results",
            Self::SavedQueryReopen => "Saved-query reopen",
        }
    }
}

/// Closed explainer-state vocabulary surfaced to the user.
///
/// This is the presentation axis of the explanation: it names what the explain
/// sheet emphasizes for a visible row, or why a candidate was omitted. It is
/// distinct from (but kept consistent with) the [`FactLabelClass`] match kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainStateClass {
    /// Row was promoted by a positive ranking signal.
    Promoted,
    /// Row carries a suppressed signal (e.g., a generated-artifact sibling was deprioritized).
    Suppressed,
    /// Row's order was settled by a tie-break.
    Tied,
    /// Row or candidate was answered off a partial, still-warming index.
    PartialIndex,
    /// Candidate was withheld because its lane exceeded a latency budget.
    WithheldLatency,
    /// Candidate was hidden or narrowed by trust/policy posture.
    PolicyHidden,
}

impl ExplainStateClass {
    /// Every explainer state, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Promoted,
        Self::Suppressed,
        Self::Tied,
        Self::PartialIndex,
        Self::WithheldLatency,
        Self::PolicyHidden,
    ];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Promoted => "promoted",
            Self::Suppressed => "suppressed",
            Self::Tied => "tied",
            Self::PartialIndex => "partial_index",
            Self::WithheldLatency => "withheld_latency",
            Self::PolicyHidden => "policy_hidden",
        }
    }

    /// True when this state names a candidate that was not surfaced.
    pub const fn is_omission(self) -> bool {
        matches!(
            self,
            Self::WithheldLatency | Self::PolicyHidden | Self::PartialIndex
        )
    }

    /// True when this state can headline a *visible* row's explain sheet.
    pub const fn valid_for_visible_sheet(self) -> bool {
        matches!(
            self,
            Self::Promoted | Self::Suppressed | Self::Tied | Self::PartialIndex
        )
    }

    /// True when this state can label an *omitted* candidate row.
    pub const fn valid_for_omission(self) -> bool {
        matches!(
            self,
            Self::WithheldLatency | Self::PolicyHidden | Self::PartialIndex
        )
    }
}

/// One first consumer that reuses the same explanation object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainabilityConsumerClass {
    /// The product search UI (palette, sidebars, docs, graph, saved-query reopen).
    ProductUi,
    /// Support-export / handoff bundles.
    SupportExport,
    /// Replay / debug tooling (CLI/headless inspect, query-debug sheets).
    ReplayDebug,
}

impl ExplainabilityConsumerClass {
    /// Every required first consumer, in canonical order.
    pub const ALL: [Self; 3] = [Self::ProductUi, Self::SupportExport, Self::ReplayDebug];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductUi => "product_ui",
            Self::SupportExport => "support_export",
            Self::ReplayDebug => "replay_debug",
        }
    }

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProductUi => "Product UI",
            Self::SupportExport => "Support export",
            Self::ReplayDebug => "Replay / debug",
        }
    }
}

/// Export consent posture for the query material an explainer references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportConsentClass {
    /// Default: hashes, counts, reason summaries, and omission reasons only.
    MetadataOnly,
    /// Elevated: the user opted in to include literal query text in the export.
    QueryTextElevated,
}

impl ExportConsentClass {
    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::QueryTextElevated => "query_text_elevated",
        }
    }

    /// True when literal query text may be included under this posture.
    pub const fn allows_literal_query_text(self) -> bool {
        matches!(self, Self::QueryTextElevated)
    }
}

/// The user-visible `Why this result?` sheet for one *visible* row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankingExplainSheet {
    /// Stable explain-sheet id (distinct from the result identity).
    pub explain_sheet_id: String,
    /// Durable, surface-independent result identity this sheet explains.
    pub result_id: String,
    /// Display title preserved verbatim in product copy.
    pub display_title: String,
    /// Explainer state headlined for the row.
    pub explain_state: ExplainStateClass,
    /// Canonical structured ranking explanation, reused verbatim by every consumer.
    pub ranking_reason: RankingReason,
    /// User-visible headline (one line, no raw weights).
    pub headline: String,
    /// Prose reason lines derived from the ranking signals — truthful, not weights.
    pub reason_lines: Vec<String>,
    /// Optional partiality / freshness caveat shown with the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caveat_note: Option<String>,
    /// True when raw numeric score weights are excluded from the sheet.
    pub raw_score_weights_excluded: bool,
}

impl RankingExplainSheet {
    /// True when the headlined state is consistent with the embedded reason.
    fn state_is_consistent(&self) -> bool {
        match self.explain_state {
            ExplainStateClass::Promoted => !self.ranking_reason.promoted_signals.is_empty(),
            ExplainStateClass::Suppressed => !self.ranking_reason.suppressed_signals.is_empty(),
            ExplainStateClass::Tied => self.ranking_reason.tie_break_class != TieBreakClass::None,
            ExplainStateClass::PartialIndex => {
                self.ranking_reason.fact_label == FactLabelClass::PartialIndex
            }
            // Withheld/policy-hidden states never headline a visible sheet.
            ExplainStateClass::WithheldLatency | ExplainStateClass::PolicyHidden => false,
        }
    }
}

/// One candidate that was *not* surfaced, made inspectable instead of silent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmittedCandidateRow {
    /// Stable omission id.
    pub omission_id: String,
    /// Explainer state for the omission (withheld-latency, policy-hidden, partial-index).
    pub explain_state: ExplainStateClass,
    /// Fact label for the omission.
    pub fact_label: FactLabelClass,
    /// Number of candidates this row accounts for.
    pub omitted_count: u64,
    /// User-visible omission reason summary (no literal query text).
    pub omission_reason: String,
    /// Source stratum the omitted candidates came from.
    pub source_stratum: SourceStratumClass,
    /// Optional recovery hint (re-run, warm the index, elevate trust).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_hint: Option<String>,
    /// True when literal query text is excluded from the omission row.
    pub literal_query_text_excluded: bool,
}

/// One surface and its visible explain sheets plus omitted candidate rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchExplainSurfaceRow {
    /// Surface this row covers.
    pub surface: ExplainSurfaceClass,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Durable query-session id this surface answered from (hash-only session).
    pub query_session_id_ref: String,
    /// Deterministic query hash; never the raw query text.
    pub query_hash: String,
    /// Visible `Why this result?` sheets, in result order.
    pub explain_sheets: Vec<RankingExplainSheet>,
    /// Omitted / withheld / policy-hidden candidate rows for the surface.
    #[serde(default)]
    pub omitted_candidates: Vec<OmittedCandidateRow>,
    /// Scope counters (visible, hidden, policy-hidden, latency-omitted).
    pub scope_counters: ScopeCounters,
    /// Review-safe summary for downstream consumers.
    pub summary: String,
}

impl SearchExplainSurfaceRow {
    /// Returns the omitted rows that match an explainer state.
    fn omissions_with(&self, state: ExplainStateClass) -> usize {
        self.omitted_candidates
            .iter()
            .filter(|row| row.explain_state == state)
            .count()
    }
}

/// One consumer projection proving the explanation object is reused, not rebuilt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplainabilityConsumerProjection {
    /// Consumer that reuses the explanation object.
    pub consumer: ExplainabilityConsumerClass,
    /// Checked consumer or contract ref.
    pub consumer_ref: String,
    /// Packet identifier the consumer ingests verbatim.
    pub ingested_packet_id: String,
    /// True when the consumer preserves the visible explain sheets verbatim.
    pub preserves_explain_sheets: bool,
    /// True when the consumer preserves the omitted / withheld candidate rows.
    pub preserves_omitted_candidates: bool,
    /// True when the consumer preserves the hashes and scope counts.
    pub preserves_counts_and_hashes: bool,
    /// True when the consumer includes literal query text (gated by consent).
    pub includes_literal_query_text: bool,
    /// True when raw numeric score weights are excluded from the projection.
    pub raw_score_weights_excluded: bool,
    /// True when ambient authority / credentials are excluded.
    pub ambient_authority_excluded: bool,
    /// Review-safe summary of the binding contract.
    pub summary: String,
}

impl ExplainabilityConsumerProjection {
    fn reuses_explanation(&self) -> bool {
        self.preserves_explain_sheets
            && self.preserves_omitted_candidates
            && self.preserves_counts_and_hashes
            && self.raw_score_weights_excluded
            && self.ambient_authority_excluded
            && !self.consumer_ref.trim().is_empty()
    }
}

/// One validation finding emitted by [`RankingExplainabilityPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankingExplainabilityValidationFinding {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// User-visible ranking-explainability and withheld-candidate truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankingExplainabilityPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Reviewer-facing contract document ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Checked review artifact ref.
    pub artifact_ref: String,
    /// Authoritative spec sections quoted by the packet.
    pub source_spec_refs: Vec<String>,
    /// Existing lane schemas the explainer composes.
    pub supporting_contract_refs: Vec<String>,
    /// Export consent posture for the referenced query material.
    pub export_consent: ExportConsentClass,
    /// True when literal query text is included anywhere in the packet.
    pub literal_query_text_included: bool,
    /// Surfaces covered by the explainer.
    pub covered_surfaces: Vec<ExplainSurfaceClass>,
    /// Explainer states covered across visible sheets and omitted rows.
    pub covered_states: Vec<ExplainStateClass>,
    /// Fact labels covered across visible sheets and omitted rows.
    pub covered_fact_labels: Vec<FactLabelClass>,
    /// Per-surface explain sheets and omitted candidate rows.
    pub surfaces: Vec<SearchExplainSurfaceRow>,
    /// Consumer projections that reuse the explanation object.
    pub consumer_projections: Vec<ExplainabilityConsumerProjection>,
    /// Metadata-safe summary safe for support and release surfaces.
    pub export_safe_summary: String,
}

impl RankingExplainabilityPacket {
    /// Returns the surface row for one surface, if present.
    pub fn surface_for(&self, surface: ExplainSurfaceClass) -> Option<&SearchExplainSurfaceRow> {
        self.surfaces.iter().find(|row| row.surface == surface)
    }

    /// Returns the explainer-state tokens present across sheets and omissions.
    pub fn covered_state_tokens(&self) -> Vec<&'static str> {
        self.present_states()
            .into_iter()
            .map(ExplainStateClass::as_str)
            .collect()
    }

    /// Returns the fact-label tokens present across sheets and omissions.
    pub fn covered_fact_label_tokens(&self) -> Vec<&'static str> {
        self.present_fact_labels()
            .into_iter()
            .map(FactLabelClass::as_str)
            .collect()
    }

    fn present_states(&self) -> BTreeSet<ExplainStateClass> {
        let mut set = BTreeSet::new();
        for surface in &self.surfaces {
            for sheet in &surface.explain_sheets {
                set.insert(sheet.explain_state);
            }
            for omitted in &surface.omitted_candidates {
                set.insert(omitted.explain_state);
            }
        }
        set
    }

    fn present_fact_labels(&self) -> BTreeSet<FactLabelClass> {
        let mut set = BTreeSet::new();
        for surface in &self.surfaces {
            for sheet in &surface.explain_sheets {
                set.insert(sheet.ranking_reason.fact_label);
            }
            for omitted in &surface.omitted_candidates {
                set.insert(omitted.fact_label);
            }
        }
        set
    }

    /// True when the packet remains metadata-safe by construction.
    pub fn is_export_safe(&self) -> bool {
        self.export_safe_summary.contains("metadata-safe")
            && !self.literal_query_text_included
            && self.export_consent == ExportConsentClass::MetadataOnly
            && self.surfaces.iter().all(|surface| {
                surface
                    .omitted_candidates
                    .iter()
                    .all(|row| row.literal_query_text_excluded)
                    && surface
                        .explain_sheets
                        .iter()
                        .all(|sheet| sheet.raw_score_weights_excluded)
            })
    }

    /// Builds a support export that wraps the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> RankingExplainabilitySupportExport {
        RankingExplainabilitySupportExport {
            record_kind: RANKING_EXPLAINABILITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RANKING_EXPLAINABILITY_SCHEMA_VERSION,
            export_id: export_id.into(),
            explainability_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            explainability_packet: self.clone(),
        }
    }

    /// Validates the explainer against the lane guardrails. An empty result
    /// means the packet is fully covered, reused, and metadata-safe.
    pub fn validate(&self) -> Vec<RankingExplainabilityValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != RANKING_EXPLAINABILITY_PACKET_RECORD_KIND {
            push(&mut findings, "record_kind", "unexpected record_kind");
        }
        if self.schema_version != RANKING_EXPLAINABILITY_SCHEMA_VERSION {
            push(&mut findings, "schema_version", "unexpected schema_version");
        }
        if self.packet_id != RANKING_EXPLAINABILITY_PACKET_ID {
            push(&mut findings, "packet_id", "unexpected packet_id");
        }
        if self.doc_ref != RANKING_EXPLAINABILITY_DOC_REF {
            push(
                &mut findings,
                "doc_ref",
                "packet must quote the reviewer doc",
            );
        }
        if self.schema_ref != RANKING_EXPLAINABILITY_SCHEMA_REF {
            push(
                &mut findings,
                "schema_ref",
                "packet must quote the schema ref",
            );
        }
        if self.artifact_ref != RANKING_EXPLAINABILITY_ARTIFACT_REF {
            push(
                &mut findings,
                "artifact_ref",
                "packet must quote the review artifact ref",
            );
        }
        if self.generated_at.trim().is_empty() {
            push(&mut findings, "generated_at", "generated_at is required");
        }
        if self.source_spec_refs.is_empty() {
            push(
                &mut findings,
                "source_spec_refs",
                "packet must quote at least one authoritative spec ref",
            );
        }
        if self.supporting_contract_refs.is_empty() {
            push(
                &mut findings,
                "supporting_contract_refs",
                "packet must cite the composed lane contracts",
            );
        }
        if !self.export_safe_summary.contains("metadata-safe") {
            push(
                &mut findings,
                "export_safe_summary",
                "summary must assert the packet is metadata-safe",
            );
        }

        self.validate_coverage(&mut findings);
        self.validate_consent(&mut findings);
        self.validate_surfaces(&mut findings);
        self.validate_consumers(&mut findings);

        findings
    }

    fn validate_coverage(&self, findings: &mut Vec<RankingExplainabilityValidationFinding>) {
        for required in ExplainSurfaceClass::ALL {
            if !self.covered_surfaces.contains(&required) {
                push(
                    findings,
                    "covered_surfaces",
                    &format!("missing covered surface {}", required.as_str()),
                );
            }
        }
        // Acceptance: the six match/withheld states stay distinguishable.
        let present_states = self.present_states();
        for required in ExplainStateClass::ALL {
            if !self.covered_states.contains(&required) {
                push(
                    findings,
                    "covered_states",
                    &format!("missing covered state {}", required.as_str()),
                );
            }
            if !present_states.contains(&required) {
                push(
                    findings,
                    "covered_states",
                    &format!("no row realizes the covered state {}", required.as_str()),
                );
            }
        }
        let present_labels = self.present_fact_labels();
        for required in FactLabelClass::ALL {
            if !self.covered_fact_labels.contains(&required) {
                push(
                    findings,
                    "covered_fact_labels",
                    &format!("missing covered fact label {}", required.as_str()),
                );
            }
            if !present_labels.contains(&required) {
                push(
                    findings,
                    "covered_fact_labels",
                    &format!(
                        "no row realizes the covered fact label {}",
                        required.as_str()
                    ),
                );
            }
        }
    }

    fn validate_consent(&self, findings: &mut Vec<RankingExplainabilityValidationFinding>) {
        // Out of scope: never widen retention to capture raw query text by
        // default. Literal query text only travels under elevated consent.
        if !self.export_consent.allows_literal_query_text() && self.literal_query_text_included {
            push(
                findings,
                "literal_query_text_included",
                "literal query text may not be included without elevated consent",
            );
        }
        if !self.export_consent.allows_literal_query_text()
            && self
                .consumer_projections
                .iter()
                .any(|projection| projection.includes_literal_query_text)
        {
            push(
                findings,
                "consumer_projections.includes_literal_query_text",
                "no consumer may include literal query text without elevated consent",
            );
        }
    }

    fn validate_surfaces(&self, findings: &mut Vec<RankingExplainabilityValidationFinding>) {
        for required in ExplainSurfaceClass::ALL {
            let count = self
                .surfaces
                .iter()
                .filter(|row| row.surface == required)
                .count();
            if count == 0 {
                push(
                    findings,
                    "surfaces",
                    &format!("missing surface row for {}", required.as_str()),
                );
            } else if count > 1 {
                push(
                    findings,
                    "surfaces",
                    &format!("surface {} must appear exactly once", required.as_str()),
                );
            }
        }

        for surface in &self.surfaces {
            let base = format!("surfaces.{}", surface.surface.as_str());
            if surface.surface_label != surface.surface.label() {
                push(
                    findings,
                    &base,
                    "surface_label must match the canonical surface label",
                );
            }
            if surface.query_session_id_ref.trim().is_empty() {
                push(
                    findings,
                    &format!("{base}.query_session_id_ref"),
                    "surface must reference a durable query session",
                );
            }
            // Sessions are referenced hash-only; the surface keeps the hash.
            if surface.query_hash.trim().is_empty() {
                push(
                    findings,
                    &format!("{base}.query_hash"),
                    "surface must keep a deterministic query hash",
                );
            }
            if surface.explain_sheets.is_empty() {
                push(
                    findings,
                    &format!("{base}.explain_sheets"),
                    "surface must materialize at least one visible explain sheet",
                );
            }
            // Visible-row count must agree with the scope counter so counts are
            // never silent.
            if surface.scope_counters.visible_rows != surface.explain_sheets.len() as u64 {
                push(
                    findings,
                    &format!("{base}.scope_counters.visible_rows"),
                    "visible_rows must equal the number of visible explain sheets",
                );
            }

            for sheet in &surface.explain_sheets {
                self.validate_sheet(findings, &base, sheet);
            }
            for omitted in &surface.omitted_candidates {
                self.validate_omission(findings, &base, omitted);
            }
            self.validate_omission_coverage(findings, &base, surface);
        }
    }

    fn validate_sheet(
        &self,
        findings: &mut Vec<RankingExplainabilityValidationFinding>,
        base: &str,
        sheet: &RankingExplainSheet,
    ) {
        let id = sheet.explain_sheet_id.trim();
        let sheet_base = format!("{base}.sheets.{id}");
        if id.is_empty() {
            push(
                findings,
                &format!("{base}.sheets"),
                "explain sheet is missing a stable id",
            );
            return;
        }
        let result_id = sheet.result_id.trim();
        // Identity must be a durable URN, never a display label or a list index.
        if result_id.is_empty() || result_id.parse::<u64>().is_ok() || !result_id.contains(':') {
            push(
                findings,
                &format!("{sheet_base}.result_id"),
                "explain sheet must point at a durable result URN, not a label or list index",
            );
        }
        if result_id.eq_ignore_ascii_case(sheet.display_title.trim()) {
            push(
                findings,
                &format!("{sheet_base}.result_id"),
                "result identity must not collapse into the display label",
            );
        }
        if !sheet.explain_state.valid_for_visible_sheet() {
            push(
                findings,
                &format!("{sheet_base}.explain_state"),
                "a visible explain sheet may not headline a withheld/policy-hidden state",
            );
        } else if !sheet.state_is_consistent() {
            push(
                findings,
                &format!("{sheet_base}.explain_state"),
                "headlined explainer state is not consistent with the ranking reason",
            );
        }
        if sheet.ranking_reason.promoted_signals.is_empty()
            && sheet.ranking_reason.suppressed_signals.is_empty()
        {
            push(
                findings,
                &format!("{sheet_base}.ranking_reason"),
                "ranking reason must carry at least one promoted or suppressed signal",
            );
        }
        // A caveated fact label must keep a partiality caveat the user can read.
        if sheet.ranking_reason.fact_label.requires_row_caveat()
            && sheet.ranking_reason.partiality_note.is_none()
            && sheet.caveat_note.is_none()
        {
            push(
                findings,
                &format!("{sheet_base}.caveat_note"),
                "a caveated fact label must keep a visible partiality caveat",
            );
        }
        if sheet.headline.trim().is_empty() {
            push(
                findings,
                &format!("{sheet_base}.headline"),
                "explain sheet must keep a user-visible headline",
            );
        }
        if sheet.reason_lines.is_empty()
            || sheet.reason_lines.iter().any(|line| line.trim().is_empty())
        {
            push(
                findings,
                &format!("{sheet_base}.reason_lines"),
                "explain sheet must carry non-empty prose reason lines",
            );
        }
        // Guardrail: truthful reasons, not raw model weights.
        if !sheet.raw_score_weights_excluded {
            push(
                findings,
                &format!("{sheet_base}.raw_score_weights_excluded"),
                "explain sheet must exclude raw numeric score weights",
            );
        }
    }

    fn validate_omission(
        &self,
        findings: &mut Vec<RankingExplainabilityValidationFinding>,
        base: &str,
        omitted: &OmittedCandidateRow,
    ) {
        let id = omitted.omission_id.trim();
        let omission_base = format!("{base}.omissions.{id}");
        if id.is_empty() {
            push(
                findings,
                &format!("{base}.omissions"),
                "omitted candidate row is missing a stable id",
            );
            return;
        }
        if !omitted.explain_state.valid_for_omission() {
            push(
                findings,
                &format!("{omission_base}.explain_state"),
                "an omitted candidate must carry a withheld/policy-hidden/partial-index state",
            );
        }
        // Fact label must agree with the omission state vocabulary.
        let label_ok = match omitted.explain_state {
            ExplainStateClass::WithheldLatency => {
                omitted.fact_label == FactLabelClass::WithheldLatency
            }
            ExplainStateClass::PolicyHidden => omitted.fact_label == FactLabelClass::PolicyHidden,
            ExplainStateClass::PartialIndex => omitted.fact_label == FactLabelClass::PartialIndex,
            _ => false,
        };
        if !label_ok {
            push(
                findings,
                &format!("{omission_base}.fact_label"),
                "omitted candidate fact label must match its omission state",
            );
        }
        if omitted.omitted_count == 0 {
            push(
                findings,
                &format!("{omission_base}.omitted_count"),
                "omitted candidate row must account for at least one candidate",
            );
        }
        if omitted.omission_reason.trim().is_empty() {
            push(
                findings,
                &format!("{omission_base}.omission_reason"),
                "omitted candidate row must keep a user-visible omission reason",
            );
        }
        if !omitted.literal_query_text_excluded {
            push(
                findings,
                &format!("{omission_base}.literal_query_text_excluded"),
                "omitted candidate row must exclude literal query text",
            );
        }
    }

    fn validate_omission_coverage(
        &self,
        findings: &mut Vec<RankingExplainabilityValidationFinding>,
        base: &str,
        surface: &SearchExplainSurfaceRow,
    ) {
        // Acceptance: policy-hidden and withheld candidates are never silent. A
        // non-zero policy/latency count must be explained by an omission row.
        if surface.scope_counters.hidden_by_policy_rows > 0
            && surface.omissions_with(ExplainStateClass::PolicyHidden) == 0
        {
            push(
                findings,
                &format!("{base}.omitted_candidates"),
                "policy-hidden rows are counted but no policy-hidden omission row explains them",
            );
        }
        if surface.scope_counters.omitted_by_latency_budget_rows > 0
            && surface.omissions_with(ExplainStateClass::WithheldLatency) == 0
        {
            push(
                findings,
                &format!("{base}.omitted_candidates"),
                "latency-omitted rows are counted but no withheld-latency omission row explains them",
            );
        }
    }

    fn validate_consumers(&self, findings: &mut Vec<RankingExplainabilityValidationFinding>) {
        for required in ExplainabilityConsumerClass::ALL {
            if !self
                .consumer_projections
                .iter()
                .any(|projection| projection.consumer == required)
            {
                push(
                    findings,
                    "consumer_projections",
                    &format!("missing first consumer {}", required.as_str()),
                );
            }
        }
        for projection in &self.consumer_projections {
            let base = format!("consumer_projections.{}", projection.consumer.as_str());
            if projection.ingested_packet_id != self.packet_id {
                push(findings, &base, "consumer must ingest the same packet id");
            }
            // Acceptance: the same explanation object is reused, not rebuilt.
            if !projection.reuses_explanation() {
                push(
                    findings,
                    &base,
                    "consumer must reuse the explain sheets, omitted rows, counts, and hashes",
                );
            }
        }
    }
}

/// Support-export wrapper that preserves the product explainability packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankingExplainabilitySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Explainability packet id preserved by the export.
    pub explainability_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials / authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub explainability_packet: RankingExplainabilityPacket,
}

impl RankingExplainabilitySupportExport {
    /// True when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == RANKING_EXPLAINABILITY_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == RANKING_EXPLAINABILITY_SCHEMA_VERSION
            && self.explainability_packet_id_ref == self.explainability_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.explainability_packet.validate().is_empty()
            && self.explainability_packet.is_export_safe()
    }
}

/// Errors returned when reading the checked-in explainability packet.
#[derive(Debug)]
pub enum RankingExplainabilityArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<RankingExplainabilityValidationFinding>),
}

impl fmt::Display for RankingExplainabilityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    formatter,
                    "ranking-explainability packet parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.path.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "ranking-explainability packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RankingExplainabilityArtifactError {}

/// Returns the checked-in canonical explainability packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_ranking_explainability_packet(
) -> Result<RankingExplainabilityPacket, RankingExplainabilityArtifactError> {
    let packet: RankingExplainabilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/search/m5/ranking-partiality/packet.json"
    )))
    .map_err(RankingExplainabilityArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(RankingExplainabilityArtifactError::Validation(findings))
    }
}

/// Variant of the seeded explainer corpus.
#[derive(Debug, Clone, Copy)]
enum ExplainabilityVariant {
    Canonical,
    PartialIndexStale,
}

/// Returns the canonical seeded explainability packet.
pub fn seeded_ranking_explainability_packet() -> RankingExplainabilityPacket {
    build_packet(ExplainabilityVariant::Canonical)
}

/// Returns a seeded packet where the live index is partial/stale, so semantic
/// coverage narrows and more candidates are withheld for latency or are not yet
/// indexed — while result identity, the explainer-state vocabulary, and the
/// reused explanation object are preserved unchanged.
pub fn seeded_partial_index_stale_ranking_explainability_packet() -> RankingExplainabilityPacket {
    build_packet(ExplainabilityVariant::PartialIndexStale)
}

fn build_packet(variant: ExplainabilityVariant) -> RankingExplainabilityPacket {
    let surfaces: Vec<SearchExplainSurfaceRow> = ExplainSurfaceClass::ALL
        .into_iter()
        .map(|surface| seed_surface(surface, variant))
        .collect();
    let covered_states = ExplainStateClass::ALL.to_vec();
    let covered_fact_labels = FactLabelClass::ALL.to_vec();
    let consumer_projections = seeded_consumer_projections();

    RankingExplainabilityPacket {
        record_kind: RANKING_EXPLAINABILITY_PACKET_RECORD_KIND.to_owned(),
        schema_version: RANKING_EXPLAINABILITY_SCHEMA_VERSION,
        packet_id: RANKING_EXPLAINABILITY_PACKET_ID.to_owned(),
        generated_at: SEEDED_GENERATED_AT.to_owned(),
        doc_ref: RANKING_EXPLAINABILITY_DOC_REF.to_owned(),
        schema_ref: RANKING_EXPLAINABILITY_SCHEMA_REF.to_owned(),
        artifact_ref: RANKING_EXPLAINABILITY_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".t2/docs/Aureline_Milestones_Document.md".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md".to_owned(),
            ".t2/docs/Aureline_UX_Design_System_Style_Guide.md".to_owned(),
            ".t2/docs/Aureline_PRD.md".to_owned(),
        ],
        supporting_contract_refs: vec![
            SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF.to_owned(),
            "schemas/search/query_session.schema.json".to_owned(),
            "schemas/search/query-session-first-consumers.schema.json".to_owned(),
            RANKING_EXPLAINABILITY_SCHEMA_REF.to_owned(),
        ],
        export_consent: ExportConsentClass::MetadataOnly,
        literal_query_text_included: false,
        covered_surfaces: ExplainSurfaceClass::ALL.to_vec(),
        covered_states,
        covered_fact_labels,
        surfaces,
        consumer_projections,
        export_safe_summary:
            "This metadata-safe explainer promotes ranking reasons into user-visible Why this result? sheets across palette, sidebars, docs results, graph-backed results, and saved-query reopen, and makes withheld-latency, policy-hidden, and partial-index candidates inspectable instead of silent. The same RankingReason object is reused by the product UI, support export, and replay/debug consumers; only hashes, counts, reason summaries, and omission reasons leave the boundary, raw numeric score weights are excluded, and literal query text travels only under elevated consent."
                .to_owned(),
    }
}

// ----- seeded surface corpus ------------------------------------------------

/// Canonical file target explained by the palette surface.
const TARGET_FILE: &str = "crates/aureline-search/src/query_session.rs";
/// Canonical symbol target explained by the sidebar surface.
const TARGET_SYMBOL: &str = "aureline_search::result_truth_packet::SearchResultRef";
/// Canonical symbol target explained by the graph-backed surface.
const TARGET_GRAPH_SYMBOL: &str = "aureline_search::query_session::SearchQuerySession";
/// Canonical docs anchors explained by the docs surface.
const TARGET_DOCS_SEMANTIC: &str = "docs/search/ranking-explainability.md#ranking-explainers";
const TARGET_DOCS_PARTIAL: &str = "docs/search/ranking-explainability.md#partiality";

fn result_id(kind: StableResultKind, canonical_ref: &str) -> String {
    build_canonical_result_id(SEEDED_WORKSPACE_ID, kind, canonical_ref)
}

fn seed_surface(
    surface: ExplainSurfaceClass,
    variant: ExplainabilityVariant,
) -> SearchExplainSurfaceRow {
    let degrade = matches!(variant, ExplainabilityVariant::PartialIndexStale);
    match surface {
        ExplainSurfaceClass::Palette => palette_surface(degrade),
        ExplainSurfaceClass::Sidebar => sidebar_surface(),
        ExplainSurfaceClass::DocsResults => docs_surface(degrade),
        ExplainSurfaceClass::GraphBackedResults => graph_surface(degrade),
        ExplainSurfaceClass::SavedQueryReopen => saved_query_surface(),
    }
}

fn surface_session(surface: ExplainSurfaceClass) -> (String, String) {
    let session_id = format!("{RANKING_EXPLAINABILITY_PACKET_ID}:{}", surface.as_str());
    let hash = stable_query_hash(surface.as_str());
    (session_id, hash)
}

fn sheet(
    surface: ExplainSurfaceClass,
    result_id: String,
    display_title: &str,
    explain_state: ExplainStateClass,
    ranking_reason: RankingReason,
    headline: &str,
    reason_lines: &[&str],
) -> RankingExplainSheet {
    // The user-visible caveat mirrors the embedded ranking reason's partiality
    // note, so the sheet never contradicts the explanation object it reuses.
    let caveat_note = ranking_reason.partiality_note.clone();
    RankingExplainSheet {
        explain_sheet_id: format!(
            "{}:{}:sheet:{display_title}",
            RANKING_EXPLAINABILITY_PACKET_ID,
            surface.as_str()
        ),
        result_id,
        display_title: display_title.to_owned(),
        explain_state,
        ranking_reason,
        headline: headline.to_owned(),
        reason_lines: reason_lines.iter().map(|line| (*line).to_owned()).collect(),
        caveat_note,
        raw_score_weights_excluded: true,
    }
}

/// The omission fact label is fully determined by the omission state.
fn omission_fact_label(state: ExplainStateClass) -> FactLabelClass {
    match state {
        ExplainStateClass::WithheldLatency => FactLabelClass::WithheldLatency,
        ExplainStateClass::PolicyHidden => FactLabelClass::PolicyHidden,
        _ => FactLabelClass::PartialIndex,
    }
}

fn omission(
    surface: ExplainSurfaceClass,
    tag: &str,
    explain_state: ExplainStateClass,
    omitted_count: u64,
    omission_reason: &str,
    source_stratum: SourceStratumClass,
    recovery_hint: &str,
) -> OmittedCandidateRow {
    OmittedCandidateRow {
        omission_id: format!(
            "{}:{}:omission:{tag}",
            RANKING_EXPLAINABILITY_PACKET_ID,
            surface.as_str()
        ),
        explain_state,
        fact_label: omission_fact_label(explain_state),
        omitted_count,
        omission_reason: omission_reason.to_owned(),
        source_stratum,
        recovery_hint: Some(recovery_hint.to_owned()),
        literal_query_text_excluded: true,
    }
}

fn counters(
    visible: u64,
    all_matching: u64,
    hidden_by_policy: u64,
    omitted_by_latency: u64,
) -> ScopeCounters {
    ScopeCounters {
        visible_rows: visible,
        loaded_rows: visible,
        all_matching_rows: Some(all_matching),
        hidden_by_current_scope_rows: 0,
        hidden_by_policy_rows: hidden_by_policy,
        hidden_by_remote_cache_rows: 0,
        omitted_by_latency_budget_rows: omitted_by_latency,
    }
}

fn palette_surface(degrade: bool) -> SearchExplainSurfaceRow {
    let surface = ExplainSurfaceClass::Palette;
    let (query_session_id_ref, query_hash) = surface_session(surface);
    let exact = sheet(
        surface,
        result_id(StableResultKind::WorkspaceFile, TARGET_FILE),
        "query_session.rs",
        ExplainStateClass::Promoted,
        RankingReason {
            fact_label: FactLabelClass::Exact,
            promoted_signals: vec![
                RankingSignalClass::ExactNameMatch,
                RankingSignalClass::RecencyOrHotSet,
            ],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::Recency,
            withheld_candidate_note: None,
            partiality_note: None,
        },
        "Exact filename match, opened recently",
        &[
            "The file basename matches your query exactly.",
            "Promoted because you opened this file recently.",
        ],
    );

    let mut omitted_candidates = vec![omission(
        surface,
        "latency",
        ExplainStateClass::WithheldLatency,
        if degrade { 4 } else { 2 },
        if degrade {
            "The semantic lane exceeded its latency budget while the index warms; 4 candidates were withheld to keep the palette responsive."
        } else {
            "The semantic lane exceeded its latency budget; 2 candidates were withheld to keep the palette responsive."
        },
        SourceStratumClass::SemanticVector,
        "Re-run the search to include the slower semantic lane.",
    )];

    if degrade {
        omitted_candidates.push(omission(
            surface,
            "warming",
            ExplainStateClass::PartialIndex,
            5,
            "5 files are still being indexed and are not yet searchable.",
            SourceStratumClass::LexicalContent,
            "Wait for indexing to finish to see the full result set.",
        ));
    }

    SearchExplainSurfaceRow {
        surface,
        surface_label: surface.label().to_owned(),
        query_session_id_ref,
        query_hash,
        explain_sheets: vec![exact],
        scope_counters: counters(1, if degrade { 9 } else { 3 }, 0, if degrade { 4 } else { 2 }),
        omitted_candidates,
        summary:
            "Palette rows headline an exact-match explainer and account for candidates withheld for latency or still indexing."
                .to_owned(),
    }
}

fn sidebar_surface() -> SearchExplainSurfaceRow {
    let surface = ExplainSurfaceClass::Sidebar;
    let (query_session_id_ref, query_hash) = surface_session(surface);
    let suppressed = sheet(
        surface,
        result_id(StableResultKind::Symbol, TARGET_SYMBOL),
        "SearchResultRef",
        ExplainStateClass::Suppressed,
        RankingReason {
            fact_label: FactLabelClass::ContextPromoted,
            promoted_signals: vec![RankingSignalClass::RecencyOrHotSet],
            suppressed_signals: vec![RankingSignalClass::GeneratedArtifactDeprioritization],
            tie_break_class: TieBreakClass::None,
            withheld_candidate_note: None,
            partiality_note: None,
        },
        "Promoted by recents; a generated sibling was deprioritized",
        &[
            "Promoted because this symbol is in your recent working set.",
            "A generated-artifact sibling with the same name was pushed down.",
        ],
    );

    SearchExplainSurfaceRow {
        surface,
        surface_label: surface.label().to_owned(),
        query_session_id_ref,
        query_hash,
        explain_sheets: vec![suppressed],
        omitted_candidates: Vec::new(),
        scope_counters: counters(1, 1, 0, 0),
        summary:
            "Sidebar rows expose suppressed signals so a deprioritized generated artifact is explained, not silently dropped."
                .to_owned(),
    }
}

fn docs_surface(degrade: bool) -> SearchExplainSurfaceRow {
    let surface = ExplainSurfaceClass::DocsResults;
    let (query_session_id_ref, query_hash) = surface_session(surface);

    let semantic_note = if degrade {
        "Semantic docs match off a partial docs index: ranked by embedding similarity, and coverage is shallower until indexing finishes."
    } else {
        "Semantic docs match: ranked by embedding similarity over the docs index, not an exact title match."
    };
    let semantic = sheet(
        surface,
        result_id(StableResultKind::DocsAnchor, TARGET_DOCS_SEMANTIC),
        "Ranking explainers",
        ExplainStateClass::Promoted,
        RankingReason {
            fact_label: FactLabelClass::Semantic,
            promoted_signals: vec![RankingSignalClass::SemanticVectorSimilarity],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::CanonicalSource,
            withheld_candidate_note: None,
            partiality_note: Some(semantic_note.to_owned()),
        },
        "Semantic match in the documentation",
        &[
            "Ranked by embedding similarity to your query.",
            "Promoted over near-duplicates by canonical-source preference.",
        ],
    );

    let partial_note =
        "Answered off a partial docs index; coverage is incomplete until indexing finishes.";
    let partial = sheet(
        surface,
        result_id(StableResultKind::DocsAnchor, TARGET_DOCS_PARTIAL),
        "Partiality",
        ExplainStateClass::PartialIndex,
        RankingReason {
            fact_label: FactLabelClass::PartialIndex,
            promoted_signals: vec![RankingSignalClass::LexicalSubstring],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::None,
            withheld_candidate_note: None,
            partiality_note: Some(partial_note.to_owned()),
        },
        "Match from a partial docs index",
        &[
            "Matched a substring in a section heading.",
            "Shown from a partial index; more matches may appear once indexing finishes.",
        ],
    );

    let mut omitted_candidates = Vec::new();
    if degrade {
        omitted_candidates.push(omission(
            surface,
            "warming",
            ExplainStateClass::PartialIndex,
            3,
            "3 documentation pages are not yet indexed and may also match.",
            SourceStratumClass::DocsIndex,
            "Wait for the docs index to finish warming.",
        ));
        omitted_candidates.push(omission(
            surface,
            "latency",
            ExplainStateClass::WithheldLatency,
            2,
            "The semantic docs lane exceeded its latency budget; 2 candidates were withheld.",
            SourceStratumClass::SemanticVector,
            "Re-run the docs search to include slower semantic matches.",
        ));
    }

    SearchExplainSurfaceRow {
        surface,
        surface_label: surface.label().to_owned(),
        query_session_id_ref,
        query_hash,
        explain_sheets: vec![semantic, partial],
        scope_counters: counters(2, if degrade { 7 } else { 2 }, 0, if degrade { 2 } else { 0 }),
        omitted_candidates,
        summary:
            "Docs rows distinguish a semantic match from a partial-index match and, under a warming index, account for not-yet-indexed and latency-withheld candidates."
                .to_owned(),
    }
}

fn graph_surface(degrade: bool) -> SearchExplainSurfaceRow {
    let surface = ExplainSurfaceClass::GraphBackedResults;
    let (query_session_id_ref, query_hash) = surface_session(surface);
    let partiality_note = if degrade {
        Some("Graph epoch is still warming; neighbor expansion may be incomplete.".to_owned())
    } else {
        None
    };
    let tied = sheet(
        surface,
        result_id(StableResultKind::Symbol, TARGET_GRAPH_SYMBOL),
        "SearchQuerySession",
        ExplainStateClass::Tied,
        RankingReason {
            fact_label: FactLabelClass::Exact,
            promoted_signals: vec![
                RankingSignalClass::ExactNameMatch,
                RankingSignalClass::GraphExpansion,
            ],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::CanonicalSource,
            withheld_candidate_note: None,
            partiality_note,
        },
        "Exact graph match, tie broken by canonical source",
        &[
            "Exact entity match in the code graph.",
            "Tied with a neighbor; ordered by canonical-source preference.",
        ],
    );

    let policy_hidden = omission(
        surface,
        "policy",
        ExplainStateClass::PolicyHidden,
        1,
        "1 graph result is in a restricted scope and was hidden by the active trust policy.",
        SourceStratumClass::GraphEntity,
        "Elevate trust for this workspace to view policy-hidden results.",
    );

    SearchExplainSurfaceRow {
        surface,
        surface_label: surface.label().to_owned(),
        query_session_id_ref,
        query_hash,
        explain_sheets: vec![tied],
        omitted_candidates: vec![policy_hidden],
        scope_counters: counters(1, 2, 1, 0),
        summary:
            "Graph-backed rows expose tie-breaks and surface a policy-hidden candidate as a controlled state instead of an opaque omission."
                .to_owned(),
    }
}

fn saved_query_surface() -> SearchExplainSurfaceRow {
    let surface = ExplainSurfaceClass::SavedQueryReopen;
    let (query_session_id_ref, query_hash) = surface_session(surface);
    // Reopening a saved query reuses the same durable file identity as the
    // palette surface, proving the explanation survives reopen.
    let reopened = sheet(
        surface,
        result_id(StableResultKind::WorkspaceFile, TARGET_FILE),
        "query_session.rs (saved)",
        ExplainStateClass::Promoted,
        RankingReason {
            fact_label: FactLabelClass::ContextPromoted,
            promoted_signals: vec![RankingSignalClass::RecencyOrHotSet],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::Recency,
            withheld_candidate_note: None,
            partiality_note: None,
        },
        "Reopened from a saved query",
        &[
            "Reopened from a saved query with its ranking explanation intact.",
            "Still promoted because the target remains in your recent working set.",
        ],
    );

    SearchExplainSurfaceRow {
        surface,
        surface_label: surface.label().to_owned(),
        query_session_id_ref,
        query_hash,
        explain_sheets: vec![reopened],
        omitted_candidates: Vec::new(),
        scope_counters: counters(1, 1, 0, 0),
        summary:
            "Reopening a saved query replays the same durable result identity and ranking explanation instead of reconstructing them from rendered text."
                .to_owned(),
    }
}

fn seeded_consumer_projections() -> Vec<ExplainabilityConsumerProjection> {
    let make = |consumer: ExplainabilityConsumerClass, consumer_ref: &str, summary: &str| {
        ExplainabilityConsumerProjection {
            consumer,
            consumer_ref: consumer_ref.to_owned(),
            ingested_packet_id: RANKING_EXPLAINABILITY_PACKET_ID.to_owned(),
            preserves_explain_sheets: true,
            preserves_omitted_candidates: true,
            preserves_counts_and_hashes: true,
            includes_literal_query_text: false,
            raw_score_weights_excluded: true,
            ambient_authority_excluded: true,
            summary: summary.to_owned(),
        }
    };

    vec![
        make(
            ExplainabilityConsumerClass::ProductUi,
            "crates/aureline-shell/src/search/search_debug_sheet.rs",
            "The product search UI renders the explain sheets and omitted-candidate cues directly from the packet, so the Why this result? and Why withheld? chrome reads one explanation object.",
        ),
        make(
            ExplainabilityConsumerClass::SupportExport,
            "schemas/search/search_export_snapshot.schema.json",
            "Support export wraps the same metadata-only explain sheets, omission reasons, hashes, and counts so a reported ranking can be explained off the bundle without literal query text.",
        ),
        make(
            ExplainabilityConsumerClass::ReplayDebug,
            RANKING_EXPLAINABILITY_ARTIFACT_REF,
            "Replay/debug tooling (CLI/headless inspect and the query-debug sheet) reuses the same explain sheets and omitted rows, so an inspect dump matches the product explanation exactly.",
        ),
    ]
}

fn push(findings: &mut Vec<RankingExplainabilityValidationFinding>, path: &str, message: &str) {
    findings.push(RankingExplainabilityValidationFinding {
        path: path.to_owned(),
        message: message.to_owned(),
    });
}

#[cfg(test)]
mod tests;
