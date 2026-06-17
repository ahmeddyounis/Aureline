//! Durable query-session and result-identity substrate with its first consumers.
//!
//! Where [`crate::m5_search_navigation_qualification`] *freezes* the search
//! contract into a qualification matrix, this module *materializes* it: it mints
//! one durable [`SearchQuerySession`] per search surface, binds materialized
//! [`SearchResultRef`] rows — each carrying full source-stratum dedupe lineage,
//! a structured [`RankingReason`], and an explicit [`SearchActionBinding`] — to
//! those sessions, and proves the same session and result identities are reused
//! by the first real consumers (desktop, CLI/headless inspect, AI context
//! assembly, and support export) instead of each surface reconstructing private
//! candidate state from rendered row text.
//!
//! The substrate covers the six M5 search/navigation surfaces named by the
//! depth lane — quick open, symbol search, full-text search, references, docs
//! search, and recent navigation ([`ConsumerSurfaceKind`]) — so docs recall,
//! symbol lookup, and references stop inventing throwaway candidate lists that
//! neither the user nor support can inspect.
//!
//! The packet is metadata-only by construction: sessions are minted hash-only so
//! no raw query text crosses the boundary, and every row asserts that raw query
//! text, source bodies, provider payloads, and secrets are excluded.
//!
//! Validation is fail-closed and enforces the lane guardrails directly:
//!
//! - result identity may never collapse into a display label or a transient list
//!   index, and canonical target refs, anchors, snapshots, and freshness survive
//!   presentation churn ([`PresentationChurnEvent`]);
//! - deduplicated rows still expose every contributing source stratum;
//! - the same result identity is reused across surfaces ([`CrossSurfaceReuseRow`]);
//!   and
//! - every first consumer reuses the materialized session and result ids, never
//!   reconstructs state from UI text, and never mints a private candidate list.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::lexical::ScopeClass;
use crate::planner::SEARCH_PLANNER_ALPHA_VERSION;
use crate::query_session::{
    SearchQuerySession, SearchSurface, SEARCH_QUERY_SESSION_SCHEMA_VERSION,
};
use crate::result_id::{build_canonical_result_id, StableResultKind};
use crate::result_truth_packet::{
    ActionFallbackModeClass, ConfidenceClass, DedupeContributor, FactLabelClass, FreshnessClass,
    HistoryPolicyClass, RankingReason, RankingSignalClass, ResultKindClass, SearchActionBinding,
    SearchResultRef, SourceStratumClass, TieBreakClass, SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF,
};

/// Stable record-kind tag for [`QuerySessionFirstConsumersPacket`].
pub const QUERY_SESSION_FIRST_CONSUMERS_PACKET_RECORD_KIND: &str =
    "search_query_session_first_consumers_packet";

/// Integer schema version for the first-consumers packet.
pub const QUERY_SESSION_FIRST_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet identifier reused by every consumer binding.
pub const QUERY_SESSION_FIRST_CONSUMERS_PACKET_ID: &str =
    "search.m5.query_session_first_consumers.v1";

/// Repository-relative path of the boundary schema.
pub const QUERY_SESSION_FIRST_CONSUMERS_SCHEMA_REF: &str =
    "schemas/search/query-session-first-consumers.schema.json";

/// Repository-relative path of the reviewer-facing contract document.
pub const QUERY_SESSION_FIRST_CONSUMERS_DOC_REF: &str =
    "docs/search/query-session-first-consumers.md";

/// Repository-relative path of the checked review artifact.
pub const QUERY_SESSION_FIRST_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/search/m5/query-session-first-consumers.md";

/// Repository-relative path of the protected fixture directory.
pub const QUERY_SESSION_FIRST_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/search/m5/query-session-first-consumers";

/// Workspace id used by the seeded corpus.
const SEEDED_WORKSPACE_ID: &str = "ws-aureline";

/// Fixed generation timestamp for the seeded corpus.
const SEEDED_GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// One M5 search/navigation surface backed by the durable substrate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceKind {
    /// Fast file, recent-place, and symbol jump surface.
    QuickOpen,
    /// Symbol and structural-navigation search surface.
    SymbolSearch,
    /// Full workspace text-search surface.
    FullTextSearch,
    /// Reference / usage-site lookup surface.
    References,
    /// Documentation and help search surface.
    DocsSearch,
    /// Recent-navigation and history-recall surface.
    RecentNavigation,
}

impl ConsumerSurfaceKind {
    /// All covered surfaces in canonical order.
    pub const ALL: [Self; 6] = [
        Self::QuickOpen,
        Self::SymbolSearch,
        Self::FullTextSearch,
        Self::References,
        Self::DocsSearch,
        Self::RecentNavigation,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuickOpen => "quick_open",
            Self::SymbolSearch => "symbol_search",
            Self::FullTextSearch => "full_text_search",
            Self::References => "references",
            Self::DocsSearch => "docs_search",
            Self::RecentNavigation => "recent_navigation",
        }
    }

    /// Returns a review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::QuickOpen => "Quick open",
            Self::SymbolSearch => "Symbol search",
            Self::FullTextSearch => "Full-text search",
            Self::References => "References",
            Self::DocsSearch => "Docs search",
            Self::RecentNavigation => "Recent navigation",
        }
    }

    /// The planner-facing query-session surface family this surface maps onto.
    ///
    /// The embedded [`SearchQuerySession`] uses the four-way planner surface
    /// family; references map onto symbol search and recent navigation maps onto
    /// quick open, while this richer six-way taxonomy stays on the durable row.
    pub const fn matching_query_session_surface(self) -> SearchSurface {
        match self {
            Self::QuickOpen | Self::RecentNavigation => SearchSurface::QuickOpen,
            Self::SymbolSearch | Self::References => SearchSurface::SymbolSearch,
            Self::FullTextSearch => SearchSurface::FileSearch,
            Self::DocsSearch => SearchSurface::DocsSearch,
        }
    }

    /// True when the surface answers off the live index and so narrows to a
    /// partial-index claim while the index is warming; recent navigation reads
    /// local history and stays live under a warming index.
    pub const fn depends_on_live_index(self) -> bool {
        !matches!(self, Self::RecentNavigation)
    }
}

/// One first consumer that reuses the durable substrate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionConsumerClass {
    /// The desktop product search surfaces and result panes.
    Desktop,
    /// The CLI / headless inspect emitter.
    CliHeadlessInspect,
    /// AI context assembly / context-picker.
    AiContextAssembly,
    /// Support-export and handoff bundles.
    SupportExport,
}

impl SessionConsumerClass {
    /// All first consumers in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Desktop,
        Self::CliHeadlessInspect,
        Self::AiContextAssembly,
        Self::SupportExport,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::CliHeadlessInspect => "cli_headless_inspect",
            Self::AiContextAssembly => "ai_context_assembly",
            Self::SupportExport => "support_export",
        }
    }

    /// Returns a review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Desktop => "Desktop",
            Self::CliHeadlessInspect => "CLI / headless inspect",
            Self::AiContextAssembly => "AI context assembly",
            Self::SupportExport => "Support export",
        }
    }
}

/// A presentation-churn event that result identity must survive unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationChurnEvent {
    /// The result list virtualized rows in and out of the viewport.
    RowVirtualization,
    /// The preview pane was opened or closed.
    PreviewPaneToggle,
    /// A ranking reason chip was expanded or collapsed.
    ReasonChipToggle,
    /// A pane or window was restored from a saved layout.
    PaneRestore,
}

impl PresentationChurnEvent {
    /// All churn events in canonical order.
    pub const ALL: [Self; 4] = [
        Self::RowVirtualization,
        Self::PreviewPaneToggle,
        Self::ReasonChipToggle,
        Self::PaneRestore,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RowVirtualization => "row_virtualization",
            Self::PreviewPaneToggle => "preview_pane_toggle",
            Self::ReasonChipToggle => "reason_chip_toggle",
            Self::PaneRestore => "pane_restore",
        }
    }
}

/// One materialized, durable result row bound to a surface session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableResultRow {
    /// Stable result identity with source-stratum dedupe lineage.
    pub result_ref: SearchResultRef,
    /// Structured ranking explanation for the row.
    pub ranking_reason: RankingReason,
    /// Explicit action binding pinned to the row.
    pub action_binding: SearchActionBinding,
    /// Display title preserved verbatim in product copy.
    pub display_title: String,
    /// Presentation-churn events the result identity survives unchanged.
    pub stable_across_churn: Vec<PresentationChurnEvent>,
    /// True when raw query text, source bodies, provider payloads, and secrets are excluded.
    pub raw_boundary_material_excluded: bool,
}

/// One durable surface session and its materialized result rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableSurfaceSession {
    /// Surface backed by the session.
    pub surface: ConsumerSurfaceKind,
    /// Human-readable surface label.
    pub surface_label: String,
    /// The durable query session minted before rerank.
    pub query_session: SearchQuerySession,
    /// Materialized result rows bound to the session.
    pub result_rows: Vec<DurableResultRow>,
    /// Review-safe summary for downstream consumers.
    pub summary: String,
}

impl DurableSurfaceSession {
    /// Returns the result ids carried by this surface session.
    pub fn result_ids(&self) -> Vec<&str> {
        self.result_rows
            .iter()
            .map(|row| row.result_ref.result_id.as_str())
            .collect()
    }
}

/// One proof that a single result identity is reused across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceReuseRow {
    /// Canonical target ref the surfaces share.
    pub canonical_target_ref: String,
    /// The single durable result id every listed surface reuses.
    pub shared_result_id: String,
    /// Surfaces that reuse the same identity for the target.
    pub surfaces: Vec<ConsumerSurfaceKind>,
    /// Review-safe summary.
    pub summary: String,
}

/// One first-consumer binding proving the substrate is reused, not rebuilt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerReuseBinding {
    /// Consumer that reuses the substrate.
    pub consumer: SessionConsumerClass,
    /// Checked consumer or contract ref.
    pub consumer_ref: String,
    /// Packet identifier the consumer ingests verbatim.
    pub ingested_packet_id: String,
    /// Durable session ids the consumer reuses (subset of the packet's sessions).
    pub reused_query_session_ids: Vec<String>,
    /// Durable result ids the consumer reuses (subset of the packet's rows).
    pub reused_result_ids: Vec<String>,
    /// True when the consumer preserves the full source-stratum dedupe lineage.
    pub preserves_source_stratum_lineage: bool,
    /// True when the consumer reconstructs state from rendered UI text (must be false).
    pub reconstructs_from_ui_text: bool,
    /// True when the consumer mints a private candidate list (must be false).
    pub invents_private_candidate_list: bool,
    /// True when the consumer can reopen a result from the packet.
    pub supports_reopen: bool,
    /// True when the consumer can replay the captured result set.
    pub supports_replay: bool,
    /// True when the consumer can export the result set.
    pub supports_export: bool,
    /// True when the consumer can explain a result's ranking from the packet.
    pub supports_explain: bool,
    /// True when raw private material is excluded from the consumer projection.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority / credentials are excluded.
    pub ambient_authority_excluded: bool,
    /// Review-safe summary of the binding contract.
    pub summary: String,
}

/// One validation finding emitted by
/// [`QuerySessionFirstConsumersPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstConsumerValidationFinding {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Durable query-session and result-identity substrate packet with first consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuerySessionFirstConsumersPacket {
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
    /// Existing lane schemas the substrate composes.
    pub supporting_contract_refs: Vec<String>,
    /// Surfaces covered by the substrate.
    pub covered_surfaces: Vec<ConsumerSurfaceKind>,
    /// Durable surface sessions and their materialized result rows.
    pub durable_sessions: Vec<DurableSurfaceSession>,
    /// Cross-surface result-identity reuse proofs.
    pub cross_surface_reuse: Vec<CrossSurfaceReuseRow>,
    /// First-consumer bindings that prove the substrate is reused, not rebuilt.
    pub consumer_bindings: Vec<ConsumerReuseBinding>,
    /// Metadata-safe summary safe for support and release surfaces.
    pub export_safe_summary: String,
}

impl QuerySessionFirstConsumersPacket {
    /// Returns every durable session id carried by the packet.
    pub fn session_ids(&self) -> BTreeSet<&str> {
        self.durable_sessions
            .iter()
            .map(|session| session.query_session.query_session_id.as_str())
            .collect()
    }

    /// Returns every durable result id carried by the packet.
    pub fn result_ids(&self) -> BTreeSet<&str> {
        self.durable_sessions
            .iter()
            .flat_map(|session| session.result_rows.iter())
            .map(|row| row.result_ref.result_id.as_str())
            .collect()
    }

    /// Returns the unique source-stratum tokens contributing across every row.
    pub fn contributing_stratum_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for session in &self.durable_sessions {
            for row in &session.result_rows {
                for contributor in &row.result_ref.dedupe_lineage {
                    set.insert(contributor.source_stratum);
                }
            }
        }
        set.into_iter().map(SourceStratumClass::as_str).collect()
    }

    /// Returns true when the packet remains metadata-safe by construction.
    pub fn is_export_safe(&self) -> bool {
        self.export_safe_summary.contains("metadata-safe")
            && self.durable_sessions.iter().all(|session| {
                session
                    .result_rows
                    .iter()
                    .all(|row| row.raw_boundary_material_excluded)
            })
            && self
                .consumer_bindings
                .iter()
                .all(|binding| binding.raw_private_material_excluded)
    }

    /// Validates the substrate against the lane guardrails. An empty result means
    /// the substrate is fully materialized, reused, and metadata-safe.
    pub fn validate(&self) -> Vec<FirstConsumerValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != QUERY_SESSION_FIRST_CONSUMERS_PACKET_RECORD_KIND {
            push(&mut findings, "record_kind", "unexpected record_kind");
        }
        if self.schema_version != QUERY_SESSION_FIRST_CONSUMERS_SCHEMA_VERSION {
            push(&mut findings, "schema_version", "unexpected schema_version");
        }
        if self.packet_id != QUERY_SESSION_FIRST_CONSUMERS_PACKET_ID {
            push(&mut findings, "packet_id", "unexpected packet_id");
        }
        if self.doc_ref != QUERY_SESSION_FIRST_CONSUMERS_DOC_REF {
            push(
                &mut findings,
                "doc_ref",
                "packet must quote the reviewer doc",
            );
        }
        if self.schema_ref != QUERY_SESSION_FIRST_CONSUMERS_SCHEMA_REF {
            push(
                &mut findings,
                "schema_ref",
                "packet must quote the schema ref",
            );
        }
        if self.artifact_ref != QUERY_SESSION_FIRST_CONSUMERS_ARTIFACT_REF {
            push(
                &mut findings,
                "artifact_ref",
                "packet must quote the review artifact ref",
            );
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

        for required in ConsumerSurfaceKind::ALL {
            if !self.covered_surfaces.contains(&required) {
                push(
                    &mut findings,
                    "covered_surfaces",
                    &format!("missing covered surface {}", required.as_str()),
                );
            }
        }

        self.validate_sessions(&mut findings);
        self.validate_cross_surface_reuse(&mut findings);
        self.validate_consumer_bindings(&mut findings);

        findings
    }

    fn validate_sessions(&self, findings: &mut Vec<FirstConsumerValidationFinding>) {
        for required in ConsumerSurfaceKind::ALL {
            let matches: Vec<&DurableSurfaceSession> = self
                .durable_sessions
                .iter()
                .filter(|session| session.surface == required)
                .collect();
            if matches.is_empty() {
                push(
                    findings,
                    "durable_sessions",
                    &format!("missing durable session for surface {}", required.as_str()),
                );
            } else if matches.len() > 1 {
                push(
                    findings,
                    "durable_sessions",
                    &format!(
                        "surface {} must have exactly one durable session",
                        required.as_str()
                    ),
                );
            }
        }

        for session in &self.durable_sessions {
            let base = format!("durable_sessions.{}", session.surface.as_str());
            if session.surface_label != session.surface.label() {
                push(
                    findings,
                    &base,
                    "surface_label must match the canonical surface label",
                );
            }
            let query_session = &session.query_session;
            if query_session.record_kind != SearchQuerySession::RECORD_KIND {
                push(
                    findings,
                    &format!("{base}.query_session.record_kind"),
                    "embedded query session has the wrong record kind",
                );
            }
            if query_session.query_session_id.trim().is_empty() {
                push(
                    findings,
                    &format!("{base}.query_session.query_session_id"),
                    "durable query session must carry a stable id",
                );
            }
            if query_session.surface != session.surface.matching_query_session_surface() {
                push(
                    findings,
                    &format!("{base}.query_session.surface"),
                    "embedded query session surface must match the surface mapping",
                );
            }
            // The packet is metadata-safe: sessions must not embed raw query text.
            if query_session.query_text.is_some() {
                push(
                    findings,
                    &format!("{base}.query_session.query_text"),
                    "durable session must stay hash-only; raw query text may not cross the boundary",
                );
            }
            if query_session.query_hash.is_none() {
                push(
                    findings,
                    &format!("{base}.query_session.query_hash"),
                    "durable session must retain a deterministic query hash",
                );
            }
            if session.result_rows.is_empty() {
                push(
                    findings,
                    &format!("{base}.result_rows"),
                    "durable session must materialize at least one result row",
                );
            }
            for row in &session.result_rows {
                self.validate_row(findings, &base, row);
            }
        }
    }

    fn validate_row(
        &self,
        findings: &mut Vec<FirstConsumerValidationFinding>,
        base: &str,
        row: &DurableResultRow,
    ) {
        let result_id = row.result_ref.result_id.trim();
        let row_base = format!("{base}.rows.{result_id}");
        if result_id.is_empty() {
            push(
                findings,
                &format!("{base}.rows"),
                "result row is missing a stable result id",
            );
            return;
        }
        // Guardrail: identity must not collapse into a display label.
        if result_id.eq_ignore_ascii_case(row.display_title.trim()) {
            push(
                findings,
                &format!("{row_base}.result_id"),
                "result identity must not collapse into the display label",
            );
        }
        // Guardrail: identity must not be a transient list index.
        if result_id.parse::<u64>().is_ok() || !result_id.contains(':') {
            push(
                findings,
                &format!("{row_base}.result_id"),
                "result identity must be a durable URN, not a transient list index",
            );
        }
        if row.result_ref.canonical_object_refs.is_empty() {
            push(
                findings,
                &format!("{row_base}.canonical_object_refs"),
                "result row must keep its canonical object refs",
            );
        }
        if row.result_ref.anchor_or_span_ref.trim().is_empty()
            || row.result_ref.snapshot_or_commit_ref.trim().is_empty()
        {
            push(
                findings,
                &format!("{row_base}.anchor_or_snapshot"),
                "anchor/span and snapshot refs must survive presentation churn",
            );
        }
        // Guardrail: deduplicated rows still expose every contributing stratum.
        if !row.result_ref.dedupe_lineage_is_complete() {
            push(
                findings,
                &format!("{row_base}.dedupe_lineage"),
                "deduped row must expose every contributing source stratum with a canonical anchor",
            );
        }
        if row.ranking_reason.promoted_signals.is_empty()
            && row.ranking_reason.suppressed_signals.is_empty()
        {
            push(
                findings,
                &format!("{row_base}.ranking_reason"),
                "ranking reason must carry at least one promoted or suppressed signal",
            );
        }
        if row.ranking_reason.fact_label.requires_row_caveat()
            && row.ranking_reason.partiality_note.is_none()
        {
            push(
                findings,
                &format!("{row_base}.ranking_reason.partiality_note"),
                "a caveated fact label must keep a partiality note",
            );
        }
        if matches!(
            row.ranking_reason.fact_label,
            FactLabelClass::WithheldLatency | FactLabelClass::PolicyHidden
        ) && row.ranking_reason.withheld_candidate_note.is_none()
        {
            push(
                findings,
                &format!("{row_base}.ranking_reason.withheld_candidate_note"),
                "a withheld fact label must keep a withheld-candidate note",
            );
        }
        if row.action_binding.open_target_ref.trim().is_empty() {
            push(
                findings,
                &format!("{row_base}.action_binding"),
                "action binding must keep an open-target ref",
            );
        }
        // Acceptance: identity stable across every presentation-churn event.
        for event in PresentationChurnEvent::ALL {
            if !row.stable_across_churn.contains(&event) {
                push(
                    findings,
                    &format!("{row_base}.stable_across_churn"),
                    &format!("result identity must survive {}", event.as_str()),
                );
            }
        }
        if !row.raw_boundary_material_excluded {
            push(
                findings,
                &format!("{row_base}.raw_boundary_material_excluded"),
                "row must exclude raw query text, source bodies, and secrets",
            );
        }
    }

    fn validate_cross_surface_reuse(&self, findings: &mut Vec<FirstConsumerValidationFinding>) {
        if self.cross_surface_reuse.is_empty() {
            push(
                findings,
                "cross_surface_reuse",
                "packet must prove at least one cross-surface result-identity reuse",
            );
        }
        for reuse in &self.cross_surface_reuse {
            let base = format!("cross_surface_reuse.{}", reuse.shared_result_id);
            if reuse.canonical_target_ref.trim().is_empty() {
                push(
                    findings,
                    &format!("{base}.canonical_target_ref"),
                    "cross-surface reuse must cite a canonical target ref",
                );
            }
            if reuse.surfaces.len() < 2 {
                push(
                    findings,
                    &base,
                    "cross-surface reuse must list at least two surfaces",
                );
            }
            for surface in &reuse.surfaces {
                let Some(session) = self
                    .durable_sessions
                    .iter()
                    .find(|session| session.surface == *surface)
                else {
                    push(
                        findings,
                        &base,
                        &format!("reused surface {} has no session", surface.as_str()),
                    );
                    continue;
                };
                let Some(row) = session
                    .result_rows
                    .iter()
                    .find(|row| row.result_ref.result_id == reuse.shared_result_id)
                else {
                    push(
                        findings,
                        &base,
                        &format!(
                            "surface {} does not reuse shared result id {}",
                            surface.as_str(),
                            reuse.shared_result_id
                        ),
                    );
                    continue;
                };
                if !row
                    .result_ref
                    .canonical_object_refs
                    .iter()
                    .any(|reference| reference == &reuse.canonical_target_ref)
                {
                    push(
                        findings,
                        &base,
                        &format!(
                            "surface {} reuse row drops the canonical target ref",
                            surface.as_str()
                        ),
                    );
                }
            }
        }
    }

    fn validate_consumer_bindings(&self, findings: &mut Vec<FirstConsumerValidationFinding>) {
        let session_ids = self.session_ids();
        let result_ids = self.result_ids();
        for required in SessionConsumerClass::ALL {
            if !self
                .consumer_bindings
                .iter()
                .any(|binding| binding.consumer == required)
            {
                push(
                    findings,
                    "consumer_bindings",
                    &format!("missing first consumer {}", required.as_str()),
                );
            }
        }
        for binding in &self.consumer_bindings {
            let base = format!("consumer_bindings.{}", binding.consumer.as_str());
            if binding.consumer_ref.trim().is_empty() {
                push(findings, &base, "consumer binding must cite a consumer ref");
            }
            if binding.ingested_packet_id != self.packet_id {
                push(findings, &base, "consumer must ingest the same packet id");
            }
            if binding.reused_query_session_ids.is_empty() {
                push(
                    findings,
                    &base,
                    "consumer must reuse at least one durable query session",
                );
            }
            for id in &binding.reused_query_session_ids {
                if !session_ids.contains(id.as_str()) {
                    push(
                        findings,
                        &base,
                        &format!("consumer reuses unknown query session id {id}"),
                    );
                }
            }
            if binding.reused_result_ids.is_empty() {
                push(
                    findings,
                    &base,
                    "consumer must reuse at least one materialized result id",
                );
            }
            for id in &binding.reused_result_ids {
                if !result_ids.contains(id.as_str()) {
                    push(
                        findings,
                        &base,
                        &format!("consumer reuses unknown result id {id}"),
                    );
                }
            }
            if !binding.preserves_source_stratum_lineage {
                push(
                    findings,
                    &base,
                    "consumer must preserve the source-stratum lineage",
                );
            }
            if binding.reconstructs_from_ui_text {
                push(
                    findings,
                    &base,
                    "consumer must not reconstruct state from rendered UI text",
                );
            }
            if binding.invents_private_candidate_list {
                push(
                    findings,
                    &base,
                    "consumer must not invent a private candidate list",
                );
            }
            if !(binding.supports_reopen
                && binding.supports_replay
                && binding.supports_export
                && binding.supports_explain)
            {
                push(
                    findings,
                    &base,
                    "consumer must support reopen, replay, export, and explain",
                );
            }
            if !(binding.raw_private_material_excluded && binding.ambient_authority_excluded) {
                push(
                    findings,
                    &base,
                    "consumer projection must exclude raw private material and ambient authority",
                );
            }
        }
    }
}

/// Errors returned when reading the checked-in first-consumers packet.
#[derive(Debug)]
pub enum QuerySessionFirstConsumersArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<FirstConsumerValidationFinding>),
}

impl fmt::Display for QuerySessionFirstConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    formatter,
                    "query-session first-consumers packet parse failed: {error}"
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
                    "query-session first-consumers packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for QuerySessionFirstConsumersArtifactError {}

/// Returns the checked-in canonical first-consumers packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_query_session_first_consumers_packet(
) -> Result<QuerySessionFirstConsumersPacket, QuerySessionFirstConsumersArtifactError> {
    let packet: QuerySessionFirstConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/search/m5/query-session-first-consumers/packet.json"
    )))
    .map_err(QuerySessionFirstConsumersArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(QuerySessionFirstConsumersArtifactError::Validation(
            findings,
        ))
    }
}

/// Variant of the seeded substrate.
#[derive(Debug, Clone, Copy)]
enum SubstrateVariant {
    Canonical,
    PartialIndexStale,
}

/// Returns the canonical seeded first-consumers packet.
pub fn seeded_query_session_first_consumers_packet() -> QuerySessionFirstConsumersPacket {
    build_packet(SubstrateVariant::Canonical)
}

/// Returns a seeded packet where the live index is partial/stale, so every
/// live-retrieval surface narrows to a partial-index claim while result identity
/// and source-stratum lineage are preserved unchanged and recent navigation
/// (local history) stays live.
pub fn seeded_partial_index_stale_query_session_first_consumers_packet(
) -> QuerySessionFirstConsumersPacket {
    build_packet(SubstrateVariant::PartialIndexStale)
}

fn build_packet(variant: SubstrateVariant) -> QuerySessionFirstConsumersPacket {
    let durable_sessions: Vec<DurableSurfaceSession> = ConsumerSurfaceKind::ALL
        .into_iter()
        .map(|surface| seed_session(surface, variant))
        .collect();
    let cross_surface_reuse = seeded_cross_surface_reuse();
    let consumer_bindings = seeded_consumer_bindings(&durable_sessions);

    QuerySessionFirstConsumersPacket {
        record_kind: QUERY_SESSION_FIRST_CONSUMERS_PACKET_RECORD_KIND.to_owned(),
        schema_version: QUERY_SESSION_FIRST_CONSUMERS_SCHEMA_VERSION,
        packet_id: QUERY_SESSION_FIRST_CONSUMERS_PACKET_ID.to_owned(),
        generated_at: SEEDED_GENERATED_AT.to_owned(),
        doc_ref: QUERY_SESSION_FIRST_CONSUMERS_DOC_REF.to_owned(),
        schema_ref: QUERY_SESSION_FIRST_CONSUMERS_SCHEMA_REF.to_owned(),
        artifact_ref: QUERY_SESSION_FIRST_CONSUMERS_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".t2/docs/Aureline_Milestones_Document.md".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md".to_owned(),
            ".t2/docs/Aureline_UX_Design_System_Style_Guide.md".to_owned(),
            ".t2/docs/Aureline_PRD.md".to_owned(),
        ],
        supporting_contract_refs: vec![
            "schemas/search/query_session.schema.json".to_owned(),
            SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF.to_owned(),
            "schemas/search/search_export_snapshot.schema.json".to_owned(),
            QUERY_SESSION_FIRST_CONSUMERS_SCHEMA_REF.to_owned(),
        ],
        covered_surfaces: ConsumerSurfaceKind::ALL.to_vec(),
        durable_sessions,
        cross_surface_reuse,
        consumer_bindings,
        export_safe_summary:
            "This metadata-safe substrate materializes one durable, hash-only query session per search surface (quick open, symbol search, full-text search, references, docs search, recent navigation) and binds materialized result identities with full source-stratum dedupe lineage, structured ranking reasons, and explicit action bindings; the same session and result ids are reused across surfaces and by the first consumers (desktop, CLI/headless inspect, AI context assembly, support export) so results reopen, replay, export, and explain without reconstructing state from UI text, and no raw query text, source bodies, provider payloads, or secrets cross the boundary."
                .to_owned(),
    }
}

/// Canonical file target reused by quick open and full-text search.
const TARGET_FILE: &str = "crates/aureline-search/src/query_session.rs";
/// Canonical symbol target reused by quick open and symbol search.
const TARGET_SYMBOL: &str = "aureline_search::query_session::SearchQuerySession";
/// Second canonical file target.
const TARGET_FILE_2: &str = "crates/aureline-search/src/result_truth_packet/mod.rs";
/// Second canonical symbol target.
const TARGET_SYMBOL_2: &str = "aureline_search::result_truth_packet::SearchResultRef";

fn shared_file_result_id() -> String {
    build_canonical_result_id(
        SEEDED_WORKSPACE_ID,
        StableResultKind::WorkspaceFile,
        TARGET_FILE,
    )
}

fn shared_symbol_result_id() -> String {
    build_canonical_result_id(SEEDED_WORKSPACE_ID, StableResultKind::Symbol, TARGET_SYMBOL)
}

fn seed_session(surface: ConsumerSurfaceKind, variant: SubstrateVariant) -> DurableSurfaceSession {
    let readiness = match variant {
        SubstrateVariant::PartialIndexStale if surface.depends_on_live_index() => "partial_index",
        _ => "ready",
    };
    let query_session = SearchQuerySession::for_hash_only(
        format!(
            "{}:{}",
            QUERY_SESSION_FIRST_CONSUMERS_PACKET_ID,
            surface.as_str()
        ),
        surface.matching_query_session_surface(),
        crate::query_session::stable_query_hash(surface.as_str()),
        ScopeClass::CurrentRepo,
        "Current repo",
        SEARCH_PLANNER_ALPHA_VERSION,
        readiness,
        SEEDED_GENERATED_AT,
    )
    .with_parsed_query(
        "search-query-parser-v1",
        format!("surface({})", surface.as_str()),
        Vec::new(),
    );

    let result_rows = seed_rows(surface, variant);

    DurableSurfaceSession {
        surface,
        surface_label: surface.label().to_owned(),
        query_session,
        result_rows,
        summary: format!(
            "{} mints one durable hash-only query session before rerank and binds materialized result identities with source-stratum lineage that the desktop, CLI/headless inspect, AI context, and support-export consumers reuse verbatim.",
            surface.label()
        ),
    }
}

fn seed_rows(surface: ConsumerSurfaceKind, variant: SubstrateVariant) -> Vec<DurableResultRow> {
    let degrade =
        matches!(variant, SubstrateVariant::PartialIndexStale) && surface.depends_on_live_index();
    match surface {
        ConsumerSurfaceKind::QuickOpen => vec![file_row(degrade), symbol_row(degrade)],
        ConsumerSurfaceKind::SymbolSearch => {
            vec![symbol_row(degrade), symbol_row_secondary(degrade)]
        }
        ConsumerSurfaceKind::FullTextSearch => {
            vec![file_row(degrade), file_row_secondary(degrade)]
        }
        ConsumerSurfaceKind::References => vec![reference_row(degrade)],
        ConsumerSurfaceKind::DocsSearch => vec![docs_row(degrade), docs_row_secondary(degrade)],
        ConsumerSurfaceKind::RecentNavigation => vec![recent_file_row(), recent_symbol_row()],
    }
}

fn churn_cover() -> Vec<PresentationChurnEvent> {
    PresentationChurnEvent::ALL.to_vec()
}

fn partial_note() -> Option<String> {
    Some(
        "Answered off a partial, still-warming index; result identity and source-stratum lineage are preserved while coverage stays scope-limited until the index warms."
            .to_owned(),
    )
}

fn file_row(degrade: bool) -> DurableResultRow {
    let (freshness, fact_label, partiality_note, confidence) = if degrade {
        (
            FreshnessClass::PartialIndex,
            FactLabelClass::PartialIndex,
            partial_note(),
            ConfidenceClass::Medium,
        )
    } else {
        (
            FreshnessClass::Live,
            FactLabelClass::ContextPromoted,
            None,
            ConfidenceClass::High,
        )
    };
    DurableResultRow {
        result_ref: SearchResultRef {
            result_id: shared_file_result_id(),
            result_kind: ResultKindClass::WorkspaceFile,
            canonical_object_refs: vec![TARGET_FILE.to_owned()],
            anchor_or_span_ref: format!("file:{TARGET_FILE}#L1"),
            snapshot_or_commit_ref: "worktree:HEAD".to_owned(),
            freshness,
            confidence,
            dedupe_lineage: vec![
                contributor(SourceStratumClass::LexicalFilename, TARGET_FILE),
                contributor(SourceStratumClass::RecentTargets, TARGET_FILE),
            ],
        },
        ranking_reason: RankingReason {
            fact_label,
            promoted_signals: vec![
                RankingSignalClass::ExactNameMatch,
                RankingSignalClass::RecencyOrHotSet,
            ],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::Recency,
            withheld_candidate_note: None,
            partiality_note,
        },
        action_binding: SearchActionBinding {
            open_target_ref: format!("file:{TARGET_FILE}"),
            alternate_behaviors: vec!["peek".to_owned(), "split".to_owned()],
            required_surface_capabilities: vec!["open_text_document".to_owned()],
            fallback_mode: if degrade {
                ActionFallbackModeClass::RerunLiveQuery
            } else {
                ActionFallbackModeClass::Direct
            },
            history_policy: HistoryPolicyClass::RecordHistoryEntry,
        },
        display_title: "query_session.rs".to_owned(),
        stable_across_churn: churn_cover(),
        raw_boundary_material_excluded: true,
    }
}

fn file_row_secondary(degrade: bool) -> DurableResultRow {
    DurableResultRow {
        result_ref: SearchResultRef {
            result_id: build_canonical_result_id(
                SEEDED_WORKSPACE_ID,
                StableResultKind::WorkspaceFile,
                TARGET_FILE_2,
            ),
            result_kind: ResultKindClass::WorkspaceFile,
            canonical_object_refs: vec![TARGET_FILE_2.to_owned()],
            anchor_or_span_ref: format!("file:{TARGET_FILE_2}#L620"),
            snapshot_or_commit_ref: "worktree:HEAD".to_owned(),
            freshness: if degrade {
                FreshnessClass::PartialIndex
            } else {
                FreshnessClass::Live
            },
            confidence: ConfidenceClass::Medium,
            dedupe_lineage: vec![
                contributor(SourceStratumClass::LexicalContent, TARGET_FILE_2),
                contributor(SourceStratumClass::SemanticVector, TARGET_FILE_2),
            ],
        },
        ranking_reason: RankingReason {
            fact_label: if degrade {
                FactLabelClass::PartialIndex
            } else {
                FactLabelClass::Semantic
            },
            promoted_signals: vec![
                RankingSignalClass::LexicalSubstring,
                RankingSignalClass::SemanticVectorSimilarity,
            ],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::CanonicalSource,
            withheld_candidate_note: None,
            partiality_note: if degrade {
                partial_note()
            } else {
                Some(
                    "Semantic match: ranked by embedding similarity, not an exact name match."
                        .to_owned(),
                )
            },
        },
        action_binding: SearchActionBinding {
            open_target_ref: format!("file:{TARGET_FILE_2}"),
            alternate_behaviors: vec!["peek".to_owned()],
            required_surface_capabilities: vec!["open_text_document".to_owned()],
            fallback_mode: ActionFallbackModeClass::Direct,
            history_policy: HistoryPolicyClass::RecordHistoryEntry,
        },
        display_title: "result_truth_packet/mod.rs".to_owned(),
        stable_across_churn: churn_cover(),
        raw_boundary_material_excluded: true,
    }
}

fn symbol_row(degrade: bool) -> DurableResultRow {
    DurableResultRow {
        result_ref: SearchResultRef {
            result_id: shared_symbol_result_id(),
            result_kind: ResultKindClass::Symbol,
            canonical_object_refs: vec![TARGET_SYMBOL.to_owned()],
            anchor_or_span_ref: format!("symbol:{TARGET_SYMBOL}"),
            snapshot_or_commit_ref: "graph:epoch-1".to_owned(),
            freshness: if degrade {
                FreshnessClass::PartialIndex
            } else {
                FreshnessClass::Live
            },
            confidence: ConfidenceClass::High,
            dedupe_lineage: vec![
                contributor(SourceStratumClass::StructuralSymbol, TARGET_SYMBOL),
                contributor(SourceStratumClass::LexicalContent, TARGET_SYMBOL),
            ],
        },
        ranking_reason: RankingReason {
            fact_label: if degrade {
                FactLabelClass::PartialIndex
            } else {
                FactLabelClass::Exact
            },
            promoted_signals: vec![RankingSignalClass::ExactNameMatch],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::None,
            withheld_candidate_note: None,
            partiality_note: if degrade { partial_note() } else { None },
        },
        action_binding: SearchActionBinding {
            open_target_ref: format!("symbol:{TARGET_SYMBOL}"),
            alternate_behaviors: vec!["peek".to_owned(), "reveal".to_owned()],
            required_surface_capabilities: vec!["open_symbol".to_owned()],
            fallback_mode: ActionFallbackModeClass::Direct,
            history_policy: HistoryPolicyClass::RecordHistoryEntry,
        },
        display_title: "SearchQuerySession".to_owned(),
        stable_across_churn: churn_cover(),
        raw_boundary_material_excluded: true,
    }
}

fn symbol_row_secondary(degrade: bool) -> DurableResultRow {
    DurableResultRow {
        result_ref: SearchResultRef {
            result_id: build_canonical_result_id(
                SEEDED_WORKSPACE_ID,
                StableResultKind::Symbol,
                TARGET_SYMBOL_2,
            ),
            result_kind: ResultKindClass::Symbol,
            canonical_object_refs: vec![TARGET_SYMBOL_2.to_owned()],
            anchor_or_span_ref: format!("symbol:{TARGET_SYMBOL_2}"),
            snapshot_or_commit_ref: "graph:epoch-1".to_owned(),
            freshness: if degrade {
                FreshnessClass::PartialIndex
            } else {
                FreshnessClass::Live
            },
            confidence: ConfidenceClass::High,
            dedupe_lineage: vec![contributor(
                SourceStratumClass::StructuralSymbol,
                TARGET_SYMBOL_2,
            )],
        },
        ranking_reason: RankingReason {
            fact_label: if degrade {
                FactLabelClass::PartialIndex
            } else {
                FactLabelClass::Exact
            },
            promoted_signals: vec![RankingSignalClass::ExactNameMatch],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::None,
            withheld_candidate_note: None,
            partiality_note: if degrade { partial_note() } else { None },
        },
        action_binding: SearchActionBinding {
            open_target_ref: format!("symbol:{TARGET_SYMBOL_2}"),
            alternate_behaviors: vec!["peek".to_owned()],
            required_surface_capabilities: vec!["open_symbol".to_owned()],
            fallback_mode: ActionFallbackModeClass::Direct,
            history_policy: HistoryPolicyClass::RecordHistoryEntry,
        },
        display_title: "SearchResultRef".to_owned(),
        stable_across_churn: churn_cover(),
        raw_boundary_material_excluded: true,
    }
}

fn reference_row(degrade: bool) -> DurableResultRow {
    let usage_site = "crates/aureline-shell/src/palette/query_session.rs#L18";
    DurableResultRow {
        result_ref: SearchResultRef {
            result_id: build_canonical_result_id(
                SEEDED_WORKSPACE_ID,
                StableResultKind::Symbol,
                "reference:palette_query_session_uses_SearchQuerySession",
            ),
            result_kind: ResultKindClass::Symbol,
            // A reference row carries both its own usage site and the canonical
            // symbol it refers to, so references stop inventing private candidate
            // lists that cannot be inspected.
            canonical_object_refs: vec![
                TARGET_SYMBOL.to_owned(),
                "crates/aureline-shell/src/palette/query_session.rs".to_owned(),
            ],
            anchor_or_span_ref: format!("file:{usage_site}"),
            snapshot_or_commit_ref: "graph:epoch-1".to_owned(),
            freshness: if degrade {
                FreshnessClass::PartialIndex
            } else {
                FreshnessClass::Live
            },
            confidence: ConfidenceClass::High,
            dedupe_lineage: vec![
                contributor(SourceStratumClass::StructuralSymbol, TARGET_SYMBOL),
                contributor(SourceStratumClass::GraphEntity, usage_site),
            ],
        },
        ranking_reason: RankingReason {
            fact_label: if degrade {
                FactLabelClass::PartialIndex
            } else {
                FactLabelClass::Exact
            },
            promoted_signals: vec![RankingSignalClass::GraphExpansion],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::StableIdOrder,
            withheld_candidate_note: None,
            partiality_note: if degrade { partial_note() } else { None },
        },
        action_binding: SearchActionBinding {
            open_target_ref: format!("file:{usage_site}"),
            alternate_behaviors: vec!["peek".to_owned()],
            required_surface_capabilities: vec!["open_text_document".to_owned()],
            fallback_mode: ActionFallbackModeClass::Direct,
            history_policy: HistoryPolicyClass::RecordHistoryEntry,
        },
        display_title: "palette/query_session.rs:18".to_owned(),
        stable_across_churn: churn_cover(),
        raw_boundary_material_excluded: true,
    }
}

fn docs_row(degrade: bool) -> DurableResultRow {
    let anchor = "docs/search/query-session-first-consumers.md#durable-sessions";
    DurableResultRow {
        result_ref: SearchResultRef {
            result_id: build_canonical_result_id(
                SEEDED_WORKSPACE_ID,
                StableResultKind::DocsAnchor,
                anchor,
            ),
            result_kind: ResultKindClass::DocsAnchor,
            canonical_object_refs: vec![anchor.to_owned()],
            anchor_or_span_ref: format!("docs:{anchor}"),
            snapshot_or_commit_ref: "docs:index-epoch-1".to_owned(),
            freshness: if degrade {
                FreshnessClass::PartialIndex
            } else {
                FreshnessClass::Live
            },
            confidence: ConfidenceClass::Medium,
            dedupe_lineage: vec![
                contributor(SourceStratumClass::DocsIndex, anchor),
                contributor(SourceStratumClass::SemanticVector, anchor),
            ],
        },
        ranking_reason: RankingReason {
            fact_label: if degrade {
                FactLabelClass::PartialIndex
            } else {
                FactLabelClass::Semantic
            },
            promoted_signals: vec![RankingSignalClass::SemanticVectorSimilarity],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::CanonicalSource,
            withheld_candidate_note: None,
            partiality_note: if degrade {
                partial_note()
            } else {
                Some(
                    "Semantic docs match: ranked by embedding similarity over the docs index."
                        .to_owned(),
                )
            },
        },
        action_binding: SearchActionBinding {
            open_target_ref: format!("docs:{anchor}"),
            alternate_behaviors: vec!["peek".to_owned()],
            required_surface_capabilities: vec!["open_docs_anchor".to_owned()],
            fallback_mode: ActionFallbackModeClass::Direct,
            history_policy: HistoryPolicyClass::RecordHistoryEntry,
        },
        display_title: "Durable sessions".to_owned(),
        stable_across_churn: churn_cover(),
        raw_boundary_material_excluded: true,
    }
}

fn docs_row_secondary(degrade: bool) -> DurableResultRow {
    let anchor = "docs/search/result_identity_and_ranking.md#ranking";
    DurableResultRow {
        result_ref: SearchResultRef {
            result_id: build_canonical_result_id(
                SEEDED_WORKSPACE_ID,
                StableResultKind::DocsAnchor,
                anchor,
            ),
            result_kind: ResultKindClass::DocsAnchor,
            canonical_object_refs: vec![anchor.to_owned()],
            anchor_or_span_ref: format!("docs:{anchor}"),
            snapshot_or_commit_ref: "docs:index-epoch-1".to_owned(),
            freshness: if degrade {
                FreshnessClass::PartialIndex
            } else {
                FreshnessClass::Live
            },
            confidence: ConfidenceClass::High,
            dedupe_lineage: vec![contributor(SourceStratumClass::DocsIndex, anchor)],
        },
        ranking_reason: RankingReason {
            fact_label: if degrade {
                FactLabelClass::PartialIndex
            } else {
                FactLabelClass::Exact
            },
            promoted_signals: vec![RankingSignalClass::ExactNameMatch],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::None,
            withheld_candidate_note: None,
            partiality_note: if degrade { partial_note() } else { None },
        },
        action_binding: SearchActionBinding {
            open_target_ref: format!("docs:{anchor}"),
            alternate_behaviors: vec!["peek".to_owned()],
            required_surface_capabilities: vec!["open_docs_anchor".to_owned()],
            fallback_mode: ActionFallbackModeClass::Direct,
            history_policy: HistoryPolicyClass::RecordHistoryEntry,
        },
        display_title: "Ranking".to_owned(),
        stable_across_churn: churn_cover(),
        raw_boundary_material_excluded: true,
    }
}

fn recent_file_row() -> DurableResultRow {
    DurableResultRow {
        result_ref: SearchResultRef {
            result_id: build_canonical_result_id(
                SEEDED_WORKSPACE_ID,
                StableResultKind::RecentTarget,
                TARGET_FILE,
            ),
            result_kind: ResultKindClass::RecentTarget,
            canonical_object_refs: vec![TARGET_FILE.to_owned()],
            anchor_or_span_ref: format!("file:{TARGET_FILE}#L1"),
            snapshot_or_commit_ref: "recents:local".to_owned(),
            freshness: FreshnessClass::Live,
            confidence: ConfidenceClass::High,
            dedupe_lineage: vec![contributor(SourceStratumClass::RecentTargets, TARGET_FILE)],
        },
        ranking_reason: RankingReason {
            fact_label: FactLabelClass::ContextPromoted,
            promoted_signals: vec![RankingSignalClass::RecencyOrHotSet],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::Recency,
            withheld_candidate_note: None,
            partiality_note: None,
        },
        action_binding: SearchActionBinding {
            open_target_ref: format!("file:{TARGET_FILE}"),
            alternate_behaviors: vec!["peek".to_owned()],
            required_surface_capabilities: vec!["open_text_document".to_owned()],
            fallback_mode: ActionFallbackModeClass::Direct,
            history_policy: HistoryPolicyClass::ReuseExistingEntry,
        },
        display_title: "query_session.rs (recent)".to_owned(),
        stable_across_churn: churn_cover(),
        raw_boundary_material_excluded: true,
    }
}

fn recent_symbol_row() -> DurableResultRow {
    DurableResultRow {
        result_ref: SearchResultRef {
            result_id: build_canonical_result_id(
                SEEDED_WORKSPACE_ID,
                StableResultKind::RecentTarget,
                TARGET_SYMBOL,
            ),
            result_kind: ResultKindClass::RecentTarget,
            canonical_object_refs: vec![TARGET_SYMBOL.to_owned()],
            anchor_or_span_ref: format!("symbol:{TARGET_SYMBOL}"),
            snapshot_or_commit_ref: "recents:local".to_owned(),
            freshness: FreshnessClass::Live,
            confidence: ConfidenceClass::High,
            dedupe_lineage: vec![contributor(
                SourceStratumClass::RecentTargets,
                TARGET_SYMBOL,
            )],
        },
        ranking_reason: RankingReason {
            fact_label: FactLabelClass::ContextPromoted,
            promoted_signals: vec![RankingSignalClass::RecencyOrHotSet],
            suppressed_signals: Vec::new(),
            tie_break_class: TieBreakClass::Recency,
            withheld_candidate_note: None,
            partiality_note: None,
        },
        action_binding: SearchActionBinding {
            open_target_ref: format!("symbol:{TARGET_SYMBOL}"),
            alternate_behaviors: vec!["peek".to_owned()],
            required_surface_capabilities: vec!["open_symbol".to_owned()],
            fallback_mode: ActionFallbackModeClass::Direct,
            history_policy: HistoryPolicyClass::ReuseExistingEntry,
        },
        display_title: "SearchQuerySession (recent)".to_owned(),
        stable_across_churn: churn_cover(),
        raw_boundary_material_excluded: true,
    }
}

fn contributor(source_stratum: SourceStratumClass, canonical_ref: &str) -> DedupeContributor {
    DedupeContributor {
        source_stratum,
        canonical_anchor_ref: canonical_ref.to_owned(),
        contributor_result_id: None,
    }
}

fn seeded_cross_surface_reuse() -> Vec<CrossSurfaceReuseRow> {
    vec![
        CrossSurfaceReuseRow {
            canonical_target_ref: TARGET_FILE.to_owned(),
            shared_result_id: shared_file_result_id(),
            surfaces: vec![
                ConsumerSurfaceKind::QuickOpen,
                ConsumerSurfaceKind::FullTextSearch,
            ],
            summary:
                "Quick open and full-text search resolve the same file target to one durable result id; opening it from either surface reopens the identical, exportable identity."
                    .to_owned(),
        },
        CrossSurfaceReuseRow {
            canonical_target_ref: TARGET_SYMBOL.to_owned(),
            shared_result_id: shared_symbol_result_id(),
            surfaces: vec![
                ConsumerSurfaceKind::QuickOpen,
                ConsumerSurfaceKind::SymbolSearch,
            ],
            summary:
                "Quick open's symbol jump and symbol search resolve the same symbol target to one durable result id, so references and AI context cite a single canonical identity."
                    .to_owned(),
        },
    ]
}

fn seeded_consumer_bindings(
    durable_sessions: &[DurableSurfaceSession],
) -> Vec<ConsumerReuseBinding> {
    let all_session_ids: Vec<String> = durable_sessions
        .iter()
        .map(|session| session.query_session.query_session_id.clone())
        .collect();
    let all_result_ids: Vec<String> = durable_sessions
        .iter()
        .flat_map(|session| session.result_rows.iter())
        .map(|row| row.result_ref.result_id.clone())
        .collect();
    // AI context assembly selects a meaningful subset (the symbol and primary
    // file identities) rather than every row.
    let ai_session_ids: Vec<String> = durable_sessions
        .iter()
        .filter(|session| {
            matches!(
                session.surface,
                ConsumerSurfaceKind::SymbolSearch
                    | ConsumerSurfaceKind::QuickOpen
                    | ConsumerSurfaceKind::References
            )
        })
        .map(|session| session.query_session.query_session_id.clone())
        .collect();
    let ai_result_ids = vec![shared_symbol_result_id(), shared_file_result_id()];

    let make = |consumer: SessionConsumerClass,
                consumer_ref: &str,
                session_ids: Vec<String>,
                result_ids: Vec<String>,
                summary: &str| {
        ConsumerReuseBinding {
            consumer,
            consumer_ref: consumer_ref.to_owned(),
            ingested_packet_id: QUERY_SESSION_FIRST_CONSUMERS_PACKET_ID.to_owned(),
            reused_query_session_ids: session_ids,
            reused_result_ids: result_ids,
            preserves_source_stratum_lineage: true,
            reconstructs_from_ui_text: false,
            invents_private_candidate_list: false,
            supports_reopen: true,
            supports_replay: true,
            supports_export: true,
            supports_explain: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            summary: summary.to_owned(),
        }
    };

    vec![
        make(
            SessionConsumerClass::Desktop,
            "crates/aureline-shell/src/search/search_surface_bindings.rs",
            all_session_ids.clone(),
            all_result_ids.clone(),
            "The desktop search surfaces and result panes bind directly to the durable session and result ids, so row virtualization, preview toggles, reason-chip toggles, and pane restore never re-mint identity.",
        ),
        make(
            SessionConsumerClass::CliHeadlessInspect,
            QUERY_SESSION_FIRST_CONSUMERS_ARTIFACT_REF,
            all_session_ids.clone(),
            all_result_ids.clone(),
            "CLI/headless inspect emits the same session and result ids with their ranking reasons and source-stratum lineage, so an inspect dump matches the desktop result identity exactly.",
        ),
        make(
            SessionConsumerClass::AiContextAssembly,
            "crates/aureline-shell/src/ai_context_inspector/mod.rs",
            ai_session_ids,
            ai_result_ids,
            "AI context assembly cites the canonical result ids it selected instead of inventing a private candidate list, so the context picker is inspectable and the chosen targets are attributable.",
        ),
        make(
            SessionConsumerClass::SupportExport,
            "schemas/search/search_export_snapshot.schema.json",
            all_session_ids,
            all_result_ids,
            "Support export wraps the same metadata-only session and result ids so a reported result can be replayed and explained off the bundle without reconstructing state from UI text.",
        ),
    ]
}

fn push(findings: &mut Vec<FirstConsumerValidationFinding>, path: &str, message: &str) {
    findings.push(FirstConsumerValidationFinding {
        path: path.to_owned(),
        message: message.to_owned(),
    });
}

// The packet schema version tracks the upstream query-session schema it
// materializes; a bump there should be a deliberate, reviewed change here too.
const _: () =
    assert!(QUERY_SESSION_FIRST_CONSUMERS_SCHEMA_VERSION == SEARCH_QUERY_SESSION_SCHEMA_VERSION);

#[cfg(test)]
mod tests;
