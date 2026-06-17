//! Search action bindings, preview/open/split/peek parity, and no-wrong-target
//! fallbacks across the M5 search, docs, graph, history, and support flows.
//!
//! Where [`crate::result_truth_packet`] owns the *row-level*
//! [`SearchActionBinding`] (open target, alternate behaviors, required surface
//! capabilities, fallback mode, history policy) and
//! [`aureline_navigation::target_model`] owns the semantic [`RelationKind`], this
//! module binds the two together into one inspectable, exportable contract: for
//! each result row it pins *which action* (preview, open-in-place, split, peek,
//! external handoff) launches *which relation kind* against *which canonical
//! target*, with *which return anchor*, and — when the original target no longer
//! resolves under the current scope, trust, or freshness posture — *which
//! wrong-target-safe fallback* was taken and *why*.
//!
//! Two row shapes carry the truth:
//!
//! - [`ResolvedActionBinding`] is one action bound to one result row. It embeds
//!   the canonical [`SearchActionBinding`] verbatim (so the open target,
//!   alternate behaviors, required capability, fallback mode, and history policy
//!   are reused, not reminted), names the requested and resolved
//!   [`RelationKind`], keeps the return anchor for split/peek/open-in-place, and
//!   carries an optional [`WrongTargetFallback`].
//! - [`WrongTargetFallback`] makes a degraded resolution explicit instead of
//!   silent: when the live target drifted out of scope, trust, or freshness, it
//!   records the original and fallback target refs, whether the relation kind
//!   changed (definition → declaration), whether the action crossed to an
//!   external handoff (local docs → browser), a user-visible reason, and a
//!   recovery action. A fallback may never widen authority.
//!
//! [`ActionFlowRow`] groups bindings under one of the five claimed flows
//! ([`ActionFlowClass`]: search results, docs results, graph-backed results,
//! history/back-forward replay, and support handoff replay), and the
//! [`SearchActionBindingPacket`] proves the same binding objects are reused by
//! the product UI, history/back-forward, and support replay consumers
//! ([`ActionConsumerClass`]).
//!
//! The packet is metadata-only by construction: it carries no raw query text,
//! source bodies, provider payloads, secrets, or private rank weights, and every
//! binding asserts that ambient authority is excluded and that convenience
//! routing never widened authority.
//!
//! [`SearchActionBinding`]: crate::result_truth_packet::SearchActionBinding
//! [`RelationKind`]: aureline_navigation::target_model::RelationKind

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_navigation::target_model::RelationKind;

use crate::query_session::stable_query_hash;
use crate::result_id::{build_canonical_result_id, StableResultKind};
use crate::result_truth_packet::{
    ActionFallbackModeClass, HistoryPolicyClass, SearchActionBinding,
    SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF,
};

/// Stable record-kind tag for [`SearchActionBindingPacket`].
pub const SEARCH_ACTION_BINDING_PACKET_RECORD_KIND: &str = "search_action_binding_packet";

/// Stable record-kind tag for [`SearchActionBindingSupportExport`].
pub const SEARCH_ACTION_BINDING_SUPPORT_EXPORT_RECORD_KIND: &str =
    "search_action_binding_support_export";

/// Integer schema version for the action-binding packet.
pub const SEARCH_ACTION_BINDING_SCHEMA_VERSION: u32 = 1;

/// Stable packet identifier reused by every consumer projection.
pub const SEARCH_ACTION_BINDING_PACKET_ID: &str = "search.m5.action_bindings.v1";

/// Repository-relative path of the boundary schema.
pub const SEARCH_ACTION_BINDING_SCHEMA_REF: &str = "schemas/search/action-binding.schema.json";

/// Repository-relative path of the reviewer-facing contract document.
pub const SEARCH_ACTION_BINDING_DOC_REF: &str = "docs/search/action-bindings.md";

/// Repository-relative path of the checked review artifact.
pub const SEARCH_ACTION_BINDING_ARTIFACT_REF: &str = "artifacts/search/m5/action-bindings.md";

/// Repository-relative path of the protected fixture directory.
pub const SEARCH_ACTION_BINDING_FIXTURE_DIR: &str = "fixtures/search/m5/navigation-targets";

/// Workspace id used by the seeded corpus.
const SEEDED_WORKSPACE_ID: &str = "ws-aureline";

/// Fixed generation timestamp for the seeded corpus.
const SEEDED_GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// Relation kinds the action-binding matrix realizes across its flows.
///
/// The matrix deliberately keeps both [`RelationKind::Definition`] and
/// [`RelationKind::Declaration`] distinguishable so a fallback can never silently
/// collapse a definition jump into a declaration jump.
pub const COVERED_RELATION_KINDS: [RelationKind; 7] = [
    RelationKind::Definition,
    RelationKind::Declaration,
    RelationKind::Implementation,
    RelationKind::Reference,
    RelationKind::Type,
    RelationKind::Call,
    RelationKind::DocLink,
];

/// Closed action-kind vocabulary bound to every result row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchActionKind {
    /// Transient preview that does not commit navigation.
    Preview,
    /// Open the target in the active pane.
    OpenInPlace,
    /// Open the target in a split pane beside the current one.
    Split,
    /// Inline peek that keeps the caller's context.
    Peek,
    /// Hand the target off to an external surface (e.g., the system browser).
    ExternalHandoff,
}

impl SearchActionKind {
    /// Every action kind, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Preview,
        Self::OpenInPlace,
        Self::Split,
        Self::Peek,
        Self::ExternalHandoff,
    ];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::OpenInPlace => "open_in_place",
            Self::Split => "split",
            Self::Peek => "peek",
            Self::ExternalHandoff => "external_handoff",
        }
    }

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Preview => "Preview",
            Self::OpenInPlace => "Open in place",
            Self::Split => "Split",
            Self::Peek => "Peek",
            Self::ExternalHandoff => "External handoff",
        }
    }

    /// Surface capability a row must hold to launch this action.
    pub const fn required_capability(self) -> &'static str {
        match self {
            Self::Preview => "preview_pane",
            Self::OpenInPlace => "open_in_place",
            Self::Split => "split_pane",
            Self::Peek => "peek_overlay",
            Self::ExternalHandoff => "external_browser",
        }
    }
}

/// One claimed M5 action flow that keeps attributable action bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionFlowClass {
    /// Quick-open / palette / full-text search result actions.
    SearchResults,
    /// Documentation and help result actions, including local-vs-browser handoff.
    DocsResults,
    /// Graph-backed result actions (references, callers, neighbors).
    GraphResults,
    /// Back/forward and recent-navigation replay actions.
    HistoryReplay,
    /// Support/export replay and inspection of the same binding objects.
    SupportHandoff,
}

impl ActionFlowClass {
    /// Every claimed flow, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::SearchResults,
        Self::DocsResults,
        Self::GraphResults,
        Self::HistoryReplay,
        Self::SupportHandoff,
    ];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchResults => "search_results",
            Self::DocsResults => "docs_results",
            Self::GraphResults => "graph_results",
            Self::HistoryReplay => "history_replay",
            Self::SupportHandoff => "support_handoff",
        }
    }

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SearchResults => "Search results",
            Self::DocsResults => "Docs results",
            Self::GraphResults => "Graph-backed results",
            Self::HistoryReplay => "History / back-forward replay",
            Self::SupportHandoff => "Support handoff replay",
        }
    }
}

/// Closed trigger vocabulary explaining why a wrong-target-safe fallback fired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackTriggerClass {
    /// The original target resolved exactly; no fallback was taken.
    None,
    /// The original target is outside the active workset / slice / scope.
    ScopeNarrowed,
    /// Trust or policy posture hid or narrowed the original target.
    TrustPolicy,
    /// The original target's snapshot or index is stale.
    FreshnessStale,
    /// The original target no longer resolves at all and was remapped.
    TargetMissing,
}

impl FallbackTriggerClass {
    /// Every trigger, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::None,
        Self::ScopeNarrowed,
        Self::TrustPolicy,
        Self::FreshnessStale,
        Self::TargetMissing,
    ];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ScopeNarrowed => "scope_narrowed",
            Self::TrustPolicy => "trust_policy",
            Self::FreshnessStale => "freshness_stale",
            Self::TargetMissing => "target_missing",
        }
    }

    /// True when this trigger requires an explicit, recoverable fallback record.
    pub const fn requires_visible_fallback(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// One first consumer that reuses the same action-binding objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionConsumerClass {
    /// The live action launchers (keyboard, mouse, AI, automation).
    ProductUi,
    /// Back/forward and recent-navigation history replay.
    HistoryBackForward,
    /// Support/export replay and inspection tooling.
    SupportReplay,
}

impl ActionConsumerClass {
    /// Every required first consumer, in canonical order.
    pub const ALL: [Self; 3] = [
        Self::ProductUi,
        Self::HistoryBackForward,
        Self::SupportReplay,
    ];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductUi => "product_ui",
            Self::HistoryBackForward => "history_back_forward",
            Self::SupportReplay => "support_replay",
        }
    }

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProductUi => "Product UI",
            Self::HistoryBackForward => "History / back-forward",
            Self::SupportReplay => "Support replay",
        }
    }
}

/// An explicit, recoverable fallback taken when the original target no longer
/// resolves under the current scope, trust, or freshness posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrongTargetFallback {
    /// Trigger that forced the fallback.
    pub trigger: FallbackTriggerClass,
    /// Fallback mode reused from the canonical action-binding vocabulary.
    pub fallback_mode: ActionFallbackModeClass,
    /// Original target ref that no longer resolves directly.
    pub original_target_ref: String,
    /// Fallback target ref the action actually lands on.
    pub fallback_target_ref: String,
    /// True when the resolved relation kind differs from the requested one.
    pub relation_kind_changed: bool,
    /// True when the action crossed from a local target to an external handoff.
    pub crosses_to_external_handoff: bool,
    /// User-visible reason the fallback was taken (never silent).
    pub visible_reason: String,
    /// User-visible action that recovers the original target.
    pub recovery_action: String,
    /// True when the fallback can be recovered to the original target.
    pub recoverable: bool,
}

/// One action bound to one result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedActionBinding {
    /// Stable binding id (distinct from the result identity).
    pub binding_id: String,
    /// Action launched by this binding.
    pub action_kind: SearchActionKind,
    /// Durable, surface-independent result identity this action targets.
    pub result_id: String,
    /// Display title preserved verbatim in product copy.
    pub display_title: String,
    /// Relation kind the user requested (e.g., go-to-definition).
    pub requested_relation_kind: RelationKind,
    /// Relation kind the action actually resolved to (may differ on fallback).
    pub resolved_relation_kind: RelationKind,
    /// Return anchor focus comes back to after the action.
    pub return_anchor_ref: String,
    /// Canonical row-level action binding, reused verbatim from the result-truth
    /// contract.
    pub action_binding: SearchActionBinding,
    /// Trigger for any wrong-target-safe fallback (`none` for a direct action).
    pub fallback_trigger: FallbackTriggerClass,
    /// Explicit fallback record, present iff `fallback_trigger` is not `none`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback: Option<WrongTargetFallback>,
    /// True when convenience routing did not widen authority or capability.
    pub authority_not_widened: bool,
    /// True when raw query text, source bodies, secrets, and weights are excluded.
    pub raw_boundary_material_excluded: bool,
    /// Review-safe summary for downstream consumers.
    pub summary: String,
}

impl ResolvedActionBinding {
    /// True when the resolved relation kind differs from the requested one.
    pub fn relation_kind_degraded(&self) -> bool {
        self.requested_relation_kind != self.resolved_relation_kind
    }
}

/// One flow and its resolved action bindings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionFlowRow {
    /// Flow this row covers.
    pub flow: ActionFlowClass,
    /// Human-readable flow label.
    pub flow_label: String,
    /// Durable query-session id this flow answered from (hash-only session).
    pub query_session_id_ref: String,
    /// Deterministic query hash; never the raw query text.
    pub query_hash: String,
    /// Resolved action bindings, in action order.
    pub bindings: Vec<ResolvedActionBinding>,
    /// Review-safe summary for downstream consumers.
    pub summary: String,
}

impl ActionFlowRow {
    /// Number of bindings in this flow that took a wrong-target-safe fallback.
    pub fn fallback_binding_count(&self) -> usize {
        self.bindings
            .iter()
            .filter(|binding| binding.fallback.is_some())
            .count()
    }
}

/// One consumer projection proving the binding objects are reused, not rebuilt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionBindingConsumerProjection {
    /// Consumer that reuses the binding objects.
    pub consumer: ActionConsumerClass,
    /// Checked consumer or contract ref.
    pub consumer_ref: String,
    /// Packet identifier the consumer ingests verbatim.
    pub ingested_packet_id: String,
    /// True when the consumer preserves the action bindings verbatim.
    pub preserves_action_bindings: bool,
    /// True when the consumer preserves the requested/resolved relation kinds.
    pub preserves_relation_kinds: bool,
    /// True when the consumer preserves the return anchors.
    pub preserves_return_anchors: bool,
    /// True when the consumer preserves the wrong-target fallback reasons.
    pub preserves_fallback_reasons: bool,
    /// True when the consumer reuses the same binding objects, not a private copy.
    pub reuses_same_binding_objects: bool,
    /// True when the consumer widened authority (must be false).
    pub widens_authority: bool,
    /// True when ambient authority / credentials are excluded.
    pub ambient_authority_excluded: bool,
    /// Review-safe summary of the binding contract.
    pub summary: String,
}

impl ActionBindingConsumerProjection {
    fn reuses_bindings(&self) -> bool {
        self.preserves_action_bindings
            && self.preserves_relation_kinds
            && self.preserves_return_anchors
            && self.preserves_fallback_reasons
            && self.reuses_same_binding_objects
            && !self.widens_authority
            && self.ambient_authority_excluded
            && !self.consumer_ref.trim().is_empty()
    }
}

/// One validation finding emitted by [`SearchActionBindingPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchActionBindingValidationFinding {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Search action-binding, parity, and no-wrong-target fallback truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchActionBindingPacket {
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
    /// Existing lane schemas the binding matrix composes.
    pub supporting_contract_refs: Vec<String>,
    /// Flows covered by the matrix.
    pub covered_flows: Vec<ActionFlowClass>,
    /// Action kinds covered across the matrix.
    pub covered_action_kinds: Vec<SearchActionKind>,
    /// Relation kinds covered across the matrix.
    pub covered_relation_kinds: Vec<RelationKind>,
    /// Fallback triggers covered across the matrix.
    pub covered_fallback_triggers: Vec<FallbackTriggerClass>,
    /// Per-flow resolved action bindings.
    pub flows: Vec<ActionFlowRow>,
    /// Consumer projections that reuse the binding objects.
    pub consumer_projections: Vec<ActionBindingConsumerProjection>,
    /// Metadata-safe summary safe for support and release surfaces.
    pub export_safe_summary: String,
}

impl SearchActionBindingPacket {
    /// Returns the flow row for one flow, if present.
    pub fn flow_for(&self, flow: ActionFlowClass) -> Option<&ActionFlowRow> {
        self.flows.iter().find(|row| row.flow == flow)
    }

    /// Returns the action-kind tokens realized across all bindings.
    pub fn realized_action_kind_tokens(&self) -> Vec<&'static str> {
        self.present_action_kinds()
            .into_iter()
            .map(SearchActionKind::as_str)
            .collect()
    }

    /// Returns the relation-kind tokens realized across all bindings.
    pub fn realized_relation_kind_tokens(&self) -> Vec<&'static str> {
        self.present_relation_kinds()
            .into_iter()
            .map(RelationKind::as_str)
            .collect()
    }

    /// Returns the fallback-trigger tokens realized across all bindings.
    pub fn realized_fallback_trigger_tokens(&self) -> Vec<&'static str> {
        self.present_fallback_triggers()
            .into_iter()
            .map(FallbackTriggerClass::as_str)
            .collect()
    }

    fn present_action_kinds(&self) -> BTreeSet<SearchActionKind> {
        let mut set = BTreeSet::new();
        for flow in &self.flows {
            for binding in &flow.bindings {
                set.insert(binding.action_kind);
            }
        }
        set
    }

    fn present_relation_kinds(&self) -> BTreeSet<RelationKind> {
        let mut set = BTreeSet::new();
        for flow in &self.flows {
            for binding in &flow.bindings {
                set.insert(binding.requested_relation_kind);
                set.insert(binding.resolved_relation_kind);
            }
        }
        set
    }

    fn present_fallback_triggers(&self) -> BTreeSet<FallbackTriggerClass> {
        let mut set = BTreeSet::new();
        for flow in &self.flows {
            for binding in &flow.bindings {
                set.insert(binding.fallback_trigger);
            }
        }
        set
    }

    /// True when the packet remains metadata-safe by construction.
    pub fn is_export_safe(&self) -> bool {
        self.export_safe_summary.contains("metadata-safe")
            && self.flows.iter().all(|flow| {
                flow.bindings.iter().all(|binding| {
                    binding.raw_boundary_material_excluded && binding.authority_not_widened
                })
            })
            && self.consumer_projections.iter().all(|projection| {
                !projection.widens_authority && projection.ambient_authority_excluded
            })
    }

    /// Builds a support export that wraps the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> SearchActionBindingSupportExport {
        SearchActionBindingSupportExport {
            record_kind: SEARCH_ACTION_BINDING_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SEARCH_ACTION_BINDING_SCHEMA_VERSION,
            export_id: export_id.into(),
            action_binding_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            action_binding_packet: self.clone(),
        }
    }

    /// Validates the matrix against the lane guardrails. An empty result means
    /// the packet is fully covered, reused, and metadata-safe.
    pub fn validate(&self) -> Vec<SearchActionBindingValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != SEARCH_ACTION_BINDING_PACKET_RECORD_KIND {
            push(&mut findings, "record_kind", "unexpected record_kind");
        }
        if self.schema_version != SEARCH_ACTION_BINDING_SCHEMA_VERSION {
            push(&mut findings, "schema_version", "unexpected schema_version");
        }
        if self.packet_id != SEARCH_ACTION_BINDING_PACKET_ID {
            push(&mut findings, "packet_id", "unexpected packet_id");
        }
        if self.doc_ref != SEARCH_ACTION_BINDING_DOC_REF {
            push(
                &mut findings,
                "doc_ref",
                "packet must quote the reviewer doc",
            );
        }
        if self.schema_ref != SEARCH_ACTION_BINDING_SCHEMA_REF {
            push(
                &mut findings,
                "schema_ref",
                "packet must quote the schema ref",
            );
        }
        if self.artifact_ref != SEARCH_ACTION_BINDING_ARTIFACT_REF {
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
        self.validate_flows(&mut findings);
        self.validate_consumers(&mut findings);

        findings
    }

    fn validate_coverage(&self, findings: &mut Vec<SearchActionBindingValidationFinding>) {
        for required in ActionFlowClass::ALL {
            if !self.covered_flows.contains(&required) {
                push(
                    findings,
                    "covered_flows",
                    &format!("missing covered flow {}", required.as_str()),
                );
            }
        }
        let present_kinds = self.present_action_kinds();
        for required in SearchActionKind::ALL {
            if !self.covered_action_kinds.contains(&required) {
                push(
                    findings,
                    "covered_action_kinds",
                    &format!("missing covered action kind {}", required.as_str()),
                );
            }
            if !present_kinds.contains(&required) {
                push(
                    findings,
                    "covered_action_kinds",
                    &format!("no binding realizes the action kind {}", required.as_str()),
                );
            }
        }
        let present_relations = self.present_relation_kinds();
        for required in COVERED_RELATION_KINDS {
            if !self.covered_relation_kinds.contains(&required) {
                push(
                    findings,
                    "covered_relation_kinds",
                    &format!("missing covered relation kind {}", required.as_str()),
                );
            }
        }
        for declared in &self.covered_relation_kinds {
            if !present_relations.contains(declared) {
                push(
                    findings,
                    "covered_relation_kinds",
                    &format!(
                        "no binding realizes the relation kind {}",
                        declared.as_str()
                    ),
                );
            }
        }
        // Definition and declaration must both be realized so a fallback can
        // never silently collapse one into the other.
        if !present_relations.contains(&RelationKind::Definition)
            || !present_relations.contains(&RelationKind::Declaration)
        {
            push(
                findings,
                "covered_relation_kinds",
                "definition and declaration must both be realized to keep degrades visible",
            );
        }
        let present_triggers = self.present_fallback_triggers();
        for required in FallbackTriggerClass::ALL {
            if !self.covered_fallback_triggers.contains(&required) {
                push(
                    findings,
                    "covered_fallback_triggers",
                    &format!("missing covered fallback trigger {}", required.as_str()),
                );
            }
            if !present_triggers.contains(&required) {
                push(
                    findings,
                    "covered_fallback_triggers",
                    &format!(
                        "no binding realizes the fallback trigger {}",
                        required.as_str()
                    ),
                );
            }
        }
    }

    fn validate_flows(&self, findings: &mut Vec<SearchActionBindingValidationFinding>) {
        for required in ActionFlowClass::ALL {
            let count = self.flows.iter().filter(|row| row.flow == required).count();
            if count == 0 {
                push(
                    findings,
                    "flows",
                    &format!("missing flow row for {}", required.as_str()),
                );
            } else if count > 1 {
                push(
                    findings,
                    "flows",
                    &format!("flow {} must appear exactly once", required.as_str()),
                );
            }
        }

        let mut saw_relation_degrade = false;
        let mut saw_external_handoff_fallback = false;

        for flow in &self.flows {
            let base = format!("flows.{}", flow.flow.as_str());
            if flow.flow_label != flow.flow.label() {
                push(
                    findings,
                    &base,
                    "flow_label must match the canonical flow label",
                );
            }
            if flow.query_session_id_ref.trim().is_empty() {
                push(
                    findings,
                    &format!("{base}.query_session_id_ref"),
                    "flow must reference a durable query session",
                );
            }
            if flow.query_hash.trim().is_empty() {
                push(
                    findings,
                    &format!("{base}.query_hash"),
                    "flow must keep a deterministic query hash",
                );
            }
            if flow.bindings.is_empty() {
                push(
                    findings,
                    &format!("{base}.bindings"),
                    "flow must carry at least one resolved action binding",
                );
            }
            for binding in &flow.bindings {
                self.validate_binding(
                    findings,
                    &base,
                    binding,
                    &mut saw_relation_degrade,
                    &mut saw_external_handoff_fallback,
                );
            }
        }

        // Acceptance: a definition-to-declaration degrade must be a real,
        // visible state somewhere in the matrix, never an unrepresented edge.
        if !saw_relation_degrade {
            push(
                findings,
                "flows",
                "matrix must realize at least one relation-kind degrade with a visible fallback",
            );
        }
        // Acceptance: local docs to browser handoff must be a real, visible state.
        if !saw_external_handoff_fallback {
            push(
                findings,
                "flows",
                "matrix must realize at least one local-to-external handoff with a visible reason",
            );
        }
    }

    fn validate_binding(
        &self,
        findings: &mut Vec<SearchActionBindingValidationFinding>,
        base: &str,
        binding: &ResolvedActionBinding,
        saw_relation_degrade: &mut bool,
        saw_external_handoff_fallback: &mut bool,
    ) {
        let id = binding.binding_id.trim();
        let binding_base = format!("{base}.bindings.{id}");
        if id.is_empty() {
            push(
                findings,
                &format!("{base}.bindings"),
                "action binding is missing a stable id",
            );
            return;
        }
        let result_id = binding.result_id.trim();
        // Identity must be a durable URN, never a display label or a list index.
        if result_id.is_empty() || result_id.parse::<u64>().is_ok() || !result_id.contains(':') {
            push(
                findings,
                &format!("{binding_base}.result_id"),
                "action binding must point at a durable result URN, not a label or list index",
            );
        }
        if result_id.eq_ignore_ascii_case(binding.display_title.trim()) {
            push(
                findings,
                &format!("{binding_base}.result_id"),
                "result identity must not collapse into the display label",
            );
        }
        // Action binding objects are reused, not reminted: the canonical open
        // target and required capability must be present and consistent.
        if binding.action_binding.open_target_ref.trim().is_empty() {
            push(
                findings,
                &format!("{binding_base}.action_binding.open_target_ref"),
                "action binding must keep a non-empty open target ref",
            );
        }
        if !binding
            .action_binding
            .required_surface_capabilities
            .iter()
            .any(|cap| cap == binding.action_kind.required_capability())
        {
            push(
                findings,
                &format!("{binding_base}.action_binding.required_surface_capabilities"),
                "action binding must require the capability implied by its action kind",
            );
        }
        // Acceptance: split, peek, and open-in-place preserve attributable return
        // anchors; every action keeps one, and it is a genuine return point.
        if binding.return_anchor_ref.trim().is_empty() {
            push(
                findings,
                &format!("{binding_base}.return_anchor_ref"),
                "action binding must keep a return anchor",
            );
        } else if binding.return_anchor_ref == binding.action_binding.open_target_ref {
            push(
                findings,
                &format!("{binding_base}.return_anchor_ref"),
                "return anchor must differ from the open target so focus can return",
            );
        }
        if !binding.authority_not_widened {
            push(
                findings,
                &format!("{binding_base}.authority_not_widened"),
                "convenience routing must not widen authority",
            );
        }
        if !binding.raw_boundary_material_excluded {
            push(
                findings,
                &format!("{binding_base}.raw_boundary_material_excluded"),
                "action binding must exclude raw query text, bodies, secrets, and weights",
            );
        }

        let direct = binding.fallback_trigger == FallbackTriggerClass::None;
        match (&binding.fallback, direct) {
            (None, true) => {
                // Direct action: the canonical fallback mode must stay direct and
                // the relation kind must not have changed.
                if binding.action_binding.fallback_mode != ActionFallbackModeClass::Direct {
                    push(
                        findings,
                        &format!("{binding_base}.action_binding.fallback_mode"),
                        "a direct action binding must keep a direct fallback mode",
                    );
                }
                if binding.relation_kind_degraded() {
                    push(
                        findings,
                        &format!("{binding_base}.resolved_relation_kind"),
                        "a relation-kind degrade must carry an explicit wrong-target fallback",
                    );
                    *saw_relation_degrade = true;
                }
            }
            (Some(fallback), false) => {
                self.validate_fallback(findings, &binding_base, binding, fallback);
                if binding.relation_kind_degraded() {
                    *saw_relation_degrade = true;
                }
                if fallback.crosses_to_external_handoff {
                    *saw_external_handoff_fallback = true;
                }
            }
            (Some(_), true) => {
                push(
                    findings,
                    &format!("{binding_base}.fallback"),
                    "a direct action binding (trigger none) must not carry a fallback",
                );
            }
            (None, false) => {
                push(
                    findings,
                    &format!("{binding_base}.fallback"),
                    "a non-direct trigger must carry an explicit, recoverable fallback",
                );
            }
        }
    }

    fn validate_fallback(
        &self,
        findings: &mut Vec<SearchActionBindingValidationFinding>,
        binding_base: &str,
        binding: &ResolvedActionBinding,
        fallback: &WrongTargetFallback,
    ) {
        let base = format!("{binding_base}.fallback");
        if fallback.trigger != binding.fallback_trigger {
            push(
                findings,
                &format!("{base}.trigger"),
                "fallback trigger must match the binding trigger",
            );
        }
        // The fallback mode is reused from the canonical action binding so the
        // two views never disagree.
        if fallback.fallback_mode != binding.action_binding.fallback_mode {
            push(
                findings,
                &format!("{base}.fallback_mode"),
                "fallback mode must match the canonical action-binding fallback mode",
            );
        }
        if fallback.fallback_mode == ActionFallbackModeClass::Direct {
            push(
                findings,
                &format!("{base}.fallback_mode"),
                "a wrong-target fallback may not use the direct mode",
            );
        }
        if fallback.original_target_ref.trim().is_empty()
            || fallback.fallback_target_ref.trim().is_empty()
        {
            push(
                findings,
                &format!("{base}.original_target_ref"),
                "fallback must keep both the original and fallback target refs",
            );
        }
        // No silent degrade: relation_kind_changed must agree with the relation
        // kinds, and a changed relation must be visible.
        if fallback.relation_kind_changed != binding.relation_kind_degraded() {
            push(
                findings,
                &format!("{base}.relation_kind_changed"),
                "relation_kind_changed must agree with the requested vs resolved relation kinds",
            );
        }
        if fallback.crosses_to_external_handoff
            && binding.action_kind != SearchActionKind::ExternalHandoff
        {
            push(
                findings,
                &format!("{base}.crosses_to_external_handoff"),
                "a fallback that crosses to an external handoff must use the external handoff action",
            );
        }
        // Guardrail: wrong-target fallbacks must be explicit and recoverable.
        if fallback.visible_reason.trim().is_empty() {
            push(
                findings,
                &format!("{base}.visible_reason"),
                "fallback must keep a user-visible reason",
            );
        }
        if fallback.recovery_action.trim().is_empty() {
            push(
                findings,
                &format!("{base}.recovery_action"),
                "fallback must keep a recovery action",
            );
        }
        if !fallback.recoverable {
            push(
                findings,
                &format!("{base}.recoverable"),
                "wrong-target fallbacks must be recoverable",
            );
        }
    }

    fn validate_consumers(&self, findings: &mut Vec<SearchActionBindingValidationFinding>) {
        for required in ActionConsumerClass::ALL {
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
            // Acceptance: history/back-forward and support replay reuse the same
            // binding objects, not a private reconstruction.
            if !projection.reuses_bindings() {
                push(
                    findings,
                    &base,
                    "consumer must reuse the bindings, relation kinds, return anchors, and fallback reasons without widening authority",
                );
            }
        }
    }
}

/// Support-export wrapper that preserves the product action-binding packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchActionBindingSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Action-binding packet id preserved by the export.
    pub action_binding_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials / authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub action_binding_packet: SearchActionBindingPacket,
}

impl SearchActionBindingSupportExport {
    /// True when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SEARCH_ACTION_BINDING_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SEARCH_ACTION_BINDING_SCHEMA_VERSION
            && self.action_binding_packet_id_ref == self.action_binding_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.action_binding_packet.validate().is_empty()
            && self.action_binding_packet.is_export_safe()
    }
}

/// Errors returned when reading the checked-in action-binding packet.
#[derive(Debug)]
pub enum SearchActionBindingArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<SearchActionBindingValidationFinding>),
}

impl fmt::Display for SearchActionBindingArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    formatter,
                    "search action-binding packet parse failed: {error}"
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
                    "search action-binding packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SearchActionBindingArtifactError {}

/// Returns the checked-in canonical action-binding packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_search_action_binding_packet(
) -> Result<SearchActionBindingPacket, SearchActionBindingArtifactError> {
    let packet: SearchActionBindingPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/search/m5/navigation-targets/packet.json"
    )))
    .map_err(SearchActionBindingArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(SearchActionBindingArtifactError::Validation(findings))
    }
}

/// Variant of the seeded action-binding corpus.
#[derive(Debug, Clone, Copy)]
enum ActionBindingVariant {
    Canonical,
    ScopeTrustNarrowed,
}

/// Returns the canonical seeded action-binding packet.
pub fn seeded_search_action_binding_packet() -> SearchActionBindingPacket {
    build_packet(ActionBindingVariant::Canonical)
}

/// Returns a seeded packet where scope, trust, and freshness narrow further, so
/// a search-results action that was previously direct now takes an explicit,
/// recoverable wrong-target fallback — while the action-kind, relation-kind, and
/// fallback-trigger vocabulary, the result identity, and the reused binding
/// objects are preserved unchanged. History/back-forward replay reads local
/// material and is unchanged.
pub fn seeded_scope_trust_narrowed_search_action_binding_packet() -> SearchActionBindingPacket {
    build_packet(ActionBindingVariant::ScopeTrustNarrowed)
}

fn build_packet(variant: ActionBindingVariant) -> SearchActionBindingPacket {
    let flows: Vec<ActionFlowRow> = ActionFlowClass::ALL
        .into_iter()
        .map(|flow| seed_flow(flow, variant))
        .collect();

    SearchActionBindingPacket {
        record_kind: SEARCH_ACTION_BINDING_PACKET_RECORD_KIND.to_owned(),
        schema_version: SEARCH_ACTION_BINDING_SCHEMA_VERSION,
        packet_id: SEARCH_ACTION_BINDING_PACKET_ID.to_owned(),
        generated_at: SEEDED_GENERATED_AT.to_owned(),
        doc_ref: SEARCH_ACTION_BINDING_DOC_REF.to_owned(),
        schema_ref: SEARCH_ACTION_BINDING_SCHEMA_REF.to_owned(),
        artifact_ref: SEARCH_ACTION_BINDING_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".t2/docs/Aureline_Milestones_Document.md".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md".to_owned(),
            ".t2/docs/Aureline_UX_Design_System_Style_Guide.md".to_owned(),
            ".t2/docs/Aureline_PRD.md".to_owned(),
        ],
        supporting_contract_refs: vec![
            SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF.to_owned(),
            "schemas/navigation/navigation_target.schema.json".to_owned(),
            "schemas/search/query_session.schema.json".to_owned(),
            "schemas/search/ranking-explainability.schema.json".to_owned(),
            SEARCH_ACTION_BINDING_SCHEMA_REF.to_owned(),
        ],
        covered_flows: ActionFlowClass::ALL.to_vec(),
        covered_action_kinds: SearchActionKind::ALL.to_vec(),
        covered_relation_kinds: COVERED_RELATION_KINDS.to_vec(),
        covered_fallback_triggers: FallbackTriggerClass::ALL.to_vec(),
        flows,
        consumer_projections: seeded_consumer_projections(),
        export_safe_summary:
            "This metadata-safe action-binding matrix pins preview, open-in-place, split, peek, and external-handoff actions to canonical result refs and relation kinds across search results, docs results, graph-backed results, history/back-forward replay, and support handoff replay. Definition and declaration stay distinguishable, wrong-target fallbacks under narrowed scope, trust, or freshness are explicit and recoverable, and split/peek/open-in-place preserve attributable return anchors. The same binding objects are reused by the product UI, history/back-forward, and support replay consumers; only refs, relation kinds, return anchors, and fallback reasons leave the boundary, no raw query text or bodies are admitted, and convenience routing never widens authority."
                .to_owned(),
    }
}

// ----- seeded flow corpus ---------------------------------------------------

/// Canonical file definition targeted by the search-results flow.
const TARGET_FILE: &str = "crates/aureline-search/src/query_session.rs";
/// Canonical symbol targeted by the search-results split.
const TARGET_SYMBOL: &str = "aureline_search::result_truth_packet::SearchResultRef";
/// Canonical local docs anchor peeked by the docs flow.
const TARGET_DOCS_LOCAL: &str = "docs/search/action-bindings.md#action-bindings";
/// Canonical docs anchor handed off to the browser by the docs flow.
const TARGET_DOCS_EXTERNAL: &str = "docs/search/action-bindings.md#fallbacks";
/// Canonical implementation targeted by the graph flow.
const TARGET_GRAPH_IMPL: &str = "aureline_search::planner::SearchPlannerAlpha";
/// Canonical reference set peeked by the graph flow.
const TARGET_GRAPH_REF: &str = "aureline_search::query_session::SearchQuerySession";
/// Canonical type opened by the history flow.
const TARGET_HISTORY_TYPE: &str = "aureline_navigation::target_model::NavigationTarget";
/// Canonical call site previewed by the history flow.
const TARGET_HISTORY_CALL: &str = "aureline_search::result_id::build_canonical_result_id";

fn result_id(kind: StableResultKind, canonical_ref: &str) -> String {
    build_canonical_result_id(SEEDED_WORKSPACE_ID, kind, canonical_ref)
}

fn flow_session(flow: ActionFlowClass) -> (String, String) {
    let session_id = format!("{SEARCH_ACTION_BINDING_PACKET_ID}:{}", flow.as_str());
    let hash = stable_query_hash(flow.as_str());
    (session_id, hash)
}

fn open_target_ref(result_id: &str, relation_kind: RelationKind) -> String {
    format!("open:{}:{}", relation_kind.as_str(), result_id)
}

fn return_anchor(flow: ActionFlowClass, tag: &str) -> String {
    format!("return:{}:{tag}", flow.as_str())
}

/// Builds a canonical row-level action binding for the action kind and modes.
fn action_binding(
    action_kind: SearchActionKind,
    open_target_ref: String,
    fallback_mode: ActionFallbackModeClass,
    history_policy: HistoryPolicyClass,
    alternate_behaviors: &[SearchActionKind],
) -> SearchActionBinding {
    SearchActionBinding {
        open_target_ref,
        alternate_behaviors: alternate_behaviors
            .iter()
            .map(|kind| kind.as_str().to_owned())
            .collect(),
        required_surface_capabilities: vec![action_kind.required_capability().to_owned()],
        fallback_mode,
        history_policy,
    }
}

#[allow(clippy::too_many_arguments)]
fn direct_binding(
    flow: ActionFlowClass,
    tag: &str,
    action_kind: SearchActionKind,
    kind: StableResultKind,
    canonical_ref: &str,
    relation_kind: RelationKind,
    display_title: &str,
    history_policy: HistoryPolicyClass,
    alternate_behaviors: &[SearchActionKind],
    summary: &str,
) -> ResolvedActionBinding {
    let id = result_id(kind, canonical_ref);
    let target = open_target_ref(&id, relation_kind);
    ResolvedActionBinding {
        binding_id: format!(
            "{SEARCH_ACTION_BINDING_PACKET_ID}:{}:binding:{tag}",
            flow.as_str()
        ),
        action_kind,
        result_id: id,
        display_title: display_title.to_owned(),
        requested_relation_kind: relation_kind,
        resolved_relation_kind: relation_kind,
        return_anchor_ref: return_anchor(flow, tag),
        action_binding: action_binding(
            action_kind,
            target,
            ActionFallbackModeClass::Direct,
            history_policy,
            alternate_behaviors,
        ),
        fallback_trigger: FallbackTriggerClass::None,
        fallback: None,
        authority_not_widened: true,
        raw_boundary_material_excluded: true,
        summary: summary.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn fallback_binding(
    flow: ActionFlowClass,
    tag: &str,
    action_kind: SearchActionKind,
    kind: StableResultKind,
    canonical_ref: &str,
    requested_relation_kind: RelationKind,
    resolved_relation_kind: RelationKind,
    display_title: &str,
    trigger: FallbackTriggerClass,
    fallback_mode: ActionFallbackModeClass,
    history_policy: HistoryPolicyClass,
    alternate_behaviors: &[SearchActionKind],
    crosses_to_external_handoff: bool,
    visible_reason: &str,
    recovery_action: &str,
    summary: &str,
) -> ResolvedActionBinding {
    let id = result_id(kind, canonical_ref);
    let target = open_target_ref(&id, resolved_relation_kind);
    let original = open_target_ref(&id, requested_relation_kind);
    let relation_kind_changed = requested_relation_kind != resolved_relation_kind;
    ResolvedActionBinding {
        binding_id: format!(
            "{SEARCH_ACTION_BINDING_PACKET_ID}:{}:binding:{tag}",
            flow.as_str()
        ),
        action_kind,
        result_id: id,
        display_title: display_title.to_owned(),
        requested_relation_kind,
        resolved_relation_kind,
        return_anchor_ref: return_anchor(flow, tag),
        action_binding: action_binding(
            action_kind,
            target.clone(),
            fallback_mode,
            history_policy,
            alternate_behaviors,
        ),
        fallback_trigger: trigger,
        fallback: Some(WrongTargetFallback {
            trigger,
            fallback_mode,
            original_target_ref: original,
            fallback_target_ref: target,
            relation_kind_changed,
            crosses_to_external_handoff,
            visible_reason: visible_reason.to_owned(),
            recovery_action: recovery_action.to_owned(),
            recoverable: true,
        }),
        authority_not_widened: true,
        raw_boundary_material_excluded: true,
        summary: summary.to_owned(),
    }
}

fn seed_flow(flow: ActionFlowClass, variant: ActionBindingVariant) -> ActionFlowRow {
    let degrade = matches!(variant, ActionBindingVariant::ScopeTrustNarrowed);
    let bindings = match flow {
        ActionFlowClass::SearchResults => search_results_bindings(degrade),
        ActionFlowClass::DocsResults => docs_results_bindings(),
        ActionFlowClass::GraphResults => graph_results_bindings(),
        ActionFlowClass::HistoryReplay => history_replay_bindings(),
        ActionFlowClass::SupportHandoff => support_handoff_bindings(),
    };
    let (query_session_id_ref, query_hash) = flow_session(flow);
    ActionFlowRow {
        flow,
        flow_label: flow.label().to_owned(),
        query_session_id_ref,
        query_hash,
        bindings,
        summary: flow_summary(flow).to_owned(),
    }
}

fn flow_summary(flow: ActionFlowClass) -> &'static str {
    match flow {
        ActionFlowClass::SearchResults => {
            "Search-result actions open definitions in place and split a declaration snapshot when the live definition is stale, keeping the degrade visible."
        }
        ActionFlowClass::DocsResults => {
            "Docs actions peek a local docs anchor and explicitly route to the browser when the page is not in the offline pack, never a silent handoff."
        }
        ActionFlowClass::GraphResults => {
            "Graph-backed actions split an implementation and peek a policy-admitted reference set, surfacing the trust narrowing as a recoverable fallback."
        }
        ActionFlowClass::HistoryReplay => {
            "History replay opens a type in place and previews a drifted call site after an authoritative remap, reusing the same binding objects."
        }
        ActionFlowClass::SupportHandoff => {
            "Support handoff replays the search and docs fallback bindings verbatim so a reported navigation can be inspected without guessing from UI text."
        }
    }
}

fn search_results_bindings(degrade: bool) -> Vec<ResolvedActionBinding> {
    let open = if degrade {
        // Under a narrowed workset the definition is out of scope; route to the
        // canonical definition in the full workspace index instead of guessing.
        fallback_binding(
            ActionFlowClass::SearchResults,
            "open_definition",
            SearchActionKind::OpenInPlace,
            StableResultKind::WorkspaceFile,
            TARGET_FILE,
            RelationKind::Definition,
            RelationKind::Definition,
            "query_session.rs",
            FallbackTriggerClass::ScopeNarrowed,
            ActionFallbackModeClass::RouteToCanonicalSource,
            HistoryPolicyClass::RecordHistoryEntry,
            &[SearchActionKind::Peek, SearchActionKind::Split],
            false,
            "The definition is outside the active workset scope; routed to the canonical definition in the full workspace index.",
            "Widen the workset scope to open the in-scope definition directly.",
            "Open-in-place routed to the canonical definition because the active workset hid the in-scope target.",
        )
    } else {
        direct_binding(
            ActionFlowClass::SearchResults,
            "open_definition",
            SearchActionKind::OpenInPlace,
            StableResultKind::WorkspaceFile,
            TARGET_FILE,
            RelationKind::Definition,
            "query_session.rs",
            HistoryPolicyClass::RecordHistoryEntry,
            &[SearchActionKind::Peek, SearchActionKind::Split],
            "Open-in-place lands on the exact definition with a return anchor to the result list.",
        )
    };

    // The live definition body is stale; the split opens the indexed
    // declaration/signature snapshot instead and says so — a definition jump
    // never silently becomes a declaration jump.
    let split = fallback_binding(
        ActionFlowClass::SearchResults,
        "split_declaration",
        SearchActionKind::Split,
        StableResultKind::Symbol,
        TARGET_SYMBOL,
        RelationKind::Definition,
        RelationKind::Declaration,
        "SearchResultRef",
        FallbackTriggerClass::FreshnessStale,
        ActionFallbackModeClass::OpenCapturedSnapshot,
        HistoryPolicyClass::RecordHistoryEntry,
        &[SearchActionKind::Peek],
        false,
        "The live definition body is stale; split opened the indexed declaration/signature snapshot instead of the definition.",
        "Re-run the search to revalidate the definition against the live index.",
        "Split fell back from definition to declaration because the live definition body was stale.",
    );

    vec![open, split]
}

fn docs_results_bindings() -> Vec<ResolvedActionBinding> {
    let peek = direct_binding(
        ActionFlowClass::DocsResults,
        "peek_local_docs",
        SearchActionKind::Peek,
        StableResultKind::DocsAnchor,
        TARGET_DOCS_LOCAL,
        RelationKind::DocLink,
        "Action bindings",
        HistoryPolicyClass::ReuseExistingEntry,
        &[SearchActionKind::OpenInPlace],
        "Peek shows the local docs anchor inline and returns to the docs result list.",
    );

    let handoff = fallback_binding(
        ActionFlowClass::DocsResults,
        "external_handoff",
        SearchActionKind::ExternalHandoff,
        StableResultKind::DocsAnchor,
        TARGET_DOCS_EXTERNAL,
        RelationKind::DocLink,
        RelationKind::DocLink,
        "Fallbacks",
        FallbackTriggerClass::ScopeNarrowed,
        ActionFallbackModeClass::RouteToCanonicalSource,
        HistoryPolicyClass::ReuseExistingEntry,
        &[SearchActionKind::Peek],
        true,
        "This docs page is not in the offline docs pack for the active scope; opened the canonical page in the browser.",
        "Add the docs pack to the workspace to keep this handoff local.",
        "External handoff to the browser is explicit because the local docs pack did not cover the page.",
    );

    vec![peek, handoff]
}

fn graph_results_bindings() -> Vec<ResolvedActionBinding> {
    let split = direct_binding(
        ActionFlowClass::GraphResults,
        "split_implementation",
        SearchActionKind::Split,
        StableResultKind::Symbol,
        TARGET_GRAPH_IMPL,
        RelationKind::Implementation,
        "SearchPlannerAlpha",
        HistoryPolicyClass::RecordHistoryEntry,
        &[SearchActionKind::Peek],
        "Split opens the implementation beside the graph view with a return anchor to the graph node.",
    );

    let peek = fallback_binding(
        ActionFlowClass::GraphResults,
        "peek_references",
        SearchActionKind::Peek,
        StableResultKind::Symbol,
        TARGET_GRAPH_REF,
        RelationKind::Reference,
        RelationKind::Reference,
        "SearchQuerySession references",
        FallbackTriggerClass::TrustPolicy,
        ActionFallbackModeClass::PolicyNarrowed,
        HistoryPolicyClass::ReuseExistingEntry,
        &[SearchActionKind::Split],
        false,
        "Some references are in a restricted scope hidden by the active trust policy; peeked the policy-admitted reference set.",
        "Elevate trust for this workspace to peek the full reference set.",
        "Peek narrowed the reference set under the trust policy and disclosed the hidden references instead of dropping them.",
    );

    vec![split, peek]
}

fn history_replay_bindings() -> Vec<ResolvedActionBinding> {
    let open = direct_binding(
        ActionFlowClass::HistoryReplay,
        "open_type",
        SearchActionKind::OpenInPlace,
        StableResultKind::Symbol,
        TARGET_HISTORY_TYPE,
        RelationKind::Type,
        "NavigationTarget",
        HistoryPolicyClass::ReuseExistingEntry,
        &[SearchActionKind::Peek],
        "Back/forward replay re-opens the type in place using the recorded binding, not reconstructed UI text.",
    );

    let preview = fallback_binding(
        ActionFlowClass::HistoryReplay,
        "preview_call",
        SearchActionKind::Preview,
        StableResultKind::Symbol,
        TARGET_HISTORY_CALL,
        RelationKind::Call,
        RelationKind::Call,
        "build_canonical_result_id call",
        FallbackTriggerClass::TargetMissing,
        ActionFallbackModeClass::RouteToCanonicalSource,
        HistoryPolicyClass::ReuseExistingEntry,
        &[SearchActionKind::Peek],
        false,
        "The recorded call-site anchor drifted; previewed the canonical call target after an authoritative remap.",
        "Open the call hierarchy to re-pick the exact call site.",
        "Preview routed a drifted history call site to the canonical target through a recoverable remap.",
    );

    vec![open, preview]
}

fn support_handoff_bindings() -> Vec<ResolvedActionBinding> {
    // Support handoff replays the search-results stale-definition split and the
    // docs external handoff so a reported navigation can be inspected off the
    // bundle using the exact same binding objects.
    let replay_split = fallback_binding(
        ActionFlowClass::SupportHandoff,
        "replay_split_declaration",
        SearchActionKind::Split,
        StableResultKind::Symbol,
        TARGET_SYMBOL,
        RelationKind::Definition,
        RelationKind::Declaration,
        "SearchResultRef",
        FallbackTriggerClass::FreshnessStale,
        ActionFallbackModeClass::OpenCapturedSnapshot,
        HistoryPolicyClass::SuppressForCapturedReplay,
        &[SearchActionKind::Peek],
        false,
        "The live definition body is stale; split opened the indexed declaration/signature snapshot instead of the definition.",
        "Re-run the search to revalidate the definition against the live index.",
        "Support replay inspects the same stale definition-to-declaration split binding from the bundle.",
    );

    let replay_handoff = fallback_binding(
        ActionFlowClass::SupportHandoff,
        "replay_external_handoff",
        SearchActionKind::ExternalHandoff,
        StableResultKind::DocsAnchor,
        TARGET_DOCS_EXTERNAL,
        RelationKind::DocLink,
        RelationKind::DocLink,
        "Fallbacks",
        FallbackTriggerClass::ScopeNarrowed,
        ActionFallbackModeClass::RouteToCanonicalSource,
        HistoryPolicyClass::SuppressForCapturedReplay,
        &[SearchActionKind::Peek],
        true,
        "This docs page is not in the offline docs pack for the active scope; opened the canonical page in the browser.",
        "Add the docs pack to the workspace to keep this handoff local.",
        "Support replay inspects the same local-to-browser docs handoff binding from the bundle.",
    );

    vec![replay_split, replay_handoff]
}

fn seeded_consumer_projections() -> Vec<ActionBindingConsumerProjection> {
    let make = |consumer: ActionConsumerClass, consumer_ref: &str, summary: &str| {
        ActionBindingConsumerProjection {
            consumer,
            consumer_ref: consumer_ref.to_owned(),
            ingested_packet_id: SEARCH_ACTION_BINDING_PACKET_ID.to_owned(),
            preserves_action_bindings: true,
            preserves_relation_kinds: true,
            preserves_return_anchors: true,
            preserves_fallback_reasons: true,
            reuses_same_binding_objects: true,
            widens_authority: false,
            ambient_authority_excluded: true,
            summary: summary.to_owned(),
        }
    };

    vec![
        make(
            ActionConsumerClass::ProductUi,
            "crates/aureline-shell/src/navigation_targets/mod.rs",
            "The product UI launches preview/open/split/peek/external-handoff from these bindings, so keyboard, mouse, AI, and automation land on the same target semantics and render wrong-target fallbacks as visible, recoverable cues.",
        ),
        make(
            ActionConsumerClass::HistoryBackForward,
            "crates/aureline-navigation/src/target_model/mod.rs",
            "Back/forward and recent-navigation replay re-open targets from the same binding objects and return anchors, so history never re-mints a near-miss target from rendered row text.",
        ),
        make(
            ActionConsumerClass::SupportReplay,
            SEARCH_ACTION_BINDING_ARTIFACT_REF,
            "Support replay wraps the same metadata-only bindings, relation kinds, return anchors, and fallback reasons so a reported navigation can be replayed or inspected off the bundle without guessing from UI text.",
        ),
    ]
}

fn push(findings: &mut Vec<SearchActionBindingValidationFinding>, path: &str, message: &str) {
    findings.push(SearchActionBindingValidationFinding {
        path: path.to_owned(),
        message: message.to_owned(),
    });
}

#[cfg(test)]
mod tests;
