//! Stable bookmark, history, breadcrumb, outline, and peek continuity contracts.
//!
//! This module owns the export-safe packet that surfaces use when navigation
//! targets survive edits, branch changes, workset narrowing, restore, or index
//! degradation. It keeps durable anchors and drift reasons explicit so editor,
//! diff, notebook, docs, search, and topology consumers never silently retarget
//! a bookmark, history entry, breadcrumb, outline node, or peek return context.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`NavigationContinuityPacket`].
pub const BOOKMARK_HISTORY_CONTINUITY_PACKET_RECORD_KIND: &str =
    "bookmark_history_and_drift_continuity_packet";

/// Integer schema version for bookmark/history continuity packets.
pub const BOOKMARK_HISTORY_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const BOOKMARK_HISTORY_CONTINUITY_SCHEMA_REF: &str =
    "schemas/search/bookmark-history-and-drift-continuity.schema.json";

/// Repo-relative path of the reviewer document.
pub const BOOKMARK_HISTORY_CONTINUITY_DOC_REF: &str =
    "docs/m4/bookmark-history-and-drift-continuity.md";

/// Repo-relative path of the human-readable release artifact.
pub const BOOKMARK_HISTORY_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/search/m4/bookmark-history-and-drift-continuity.md";

/// Repo-relative directory of protected continuity fixtures.
pub const BOOKMARK_HISTORY_CONTINUITY_FIXTURE_DIR: &str =
    "fixtures/search/m4/bookmark-history-and-drift-continuity";

/// Closed drift vocabulary used by all stable navigation continuity artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationDriftState {
    /// Target still resolves exactly for the active workspace and scope.
    Bound,
    /// Target resolved through stable remap evidence.
    Remapped,
    /// Target changed but cannot be opened without review.
    Drifted,
    /// Target no longer resolves.
    MissingTarget,
    /// Target may exist but is outside the active workspace, trust, or scope contract.
    ScopeUnavailable,
    /// Artifact is intentionally retained as archive or tombstone metadata.
    Archived,
}

impl NavigationDriftState {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bound => "bound",
            Self::Remapped => "remapped",
            Self::Drifted => "drifted",
            Self::MissingTarget => "missing_target",
            Self::ScopeUnavailable => "scope_unavailable",
            Self::Archived => "archived",
        }
    }

    /// Returns true when an artifact must not open without visible review.
    pub const fn requires_visible_reason(self) -> bool {
        !matches!(self, Self::Bound | Self::Remapped)
    }
}

/// Drift states that every stable packet must cover.
pub const REQUIRED_DRIFT_STATES: [NavigationDriftState; 6] = [
    NavigationDriftState::Bound,
    NavigationDriftState::Remapped,
    NavigationDriftState::Drifted,
    NavigationDriftState::MissingTarget,
    NavigationDriftState::ScopeUnavailable,
    NavigationDriftState::Archived,
];

/// Surface that consumes durable navigation continuity truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationContinuitySurface {
    /// Normal editor tabs and editor groups.
    Editor,
    /// Diff editor, review, or comparison surface.
    Diff,
    /// Notebook cell, output, or notebook outline surface.
    Notebook,
    /// Docs/help card or docs browser surface.
    Docs,
    /// Search result, quick-open, or search history surface.
    Search,
    /// Graph or topology explorer surface.
    Topology,
}

impl NavigationContinuitySurface {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Diff => "diff",
            Self::Notebook => "notebook",
            Self::Docs => "docs",
            Self::Search => "search",
            Self::Topology => "topology",
        }
    }
}

/// Surfaces that must preserve the packet vocabulary.
pub const REQUIRED_CONTINUITY_SURFACES: [NavigationContinuitySurface; 6] = [
    NavigationContinuitySurface::Editor,
    NavigationContinuitySurface::Diff,
    NavigationContinuitySurface::Notebook,
    NavigationContinuitySurface::Docs,
    NavigationContinuitySurface::Search,
    NavigationContinuitySurface::Topology,
];

/// Kind of durable navigation artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationContinuityArtifactKind {
    /// Breadcrumb trail for the active target.
    BreadcrumbTrail,
    /// Outline snapshot for a file, document, notebook, or topology node.
    OutlineSnapshot,
    /// Bookmark or named navigation mark.
    NavigationMark,
    /// Back/forward or recent-location entry.
    NavigationHistoryEntry,
    /// Peek return context.
    PeekContext,
}

impl NavigationContinuityArtifactKind {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BreadcrumbTrail => "breadcrumb_trail",
            Self::OutlineSnapshot => "outline_snapshot",
            Self::NavigationMark => "navigation_mark",
            Self::NavigationHistoryEntry => "navigation_history_entry",
            Self::PeekContext => "peek_context",
        }
    }
}

/// Provider or source reference attached to a durable anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationSourceRef {
    /// Provider family or source namespace.
    pub provider_class: String,
    /// Opaque provider/source id.
    pub provider_ref: String,
    /// Freshness class visible to consumers.
    pub freshness_class: String,
    /// True when the provider admits only partial knowledge for this scope.
    pub partial: bool,
}

/// Workspace, trust, and scope identity captured with an artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationScopeRef {
    /// Workspace authority that owns the continuity decision.
    pub workspace_ref: String,
    /// Stable scope/workset/slice ref.
    pub scope_ref: String,
    /// Trust or policy contract active when the packet was emitted.
    pub trust_contract_ref: String,
    /// Human-safe scope class.
    pub scope_class: String,
}

/// Export-safe durable anchor used by all continuity objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableNavigationAnchor {
    /// Stable anchor id.
    pub anchor_id: String,
    /// Canonical object ref resolved before remap rules run.
    pub canonical_target_ref: String,
    /// Current resolved target ref, when exact or stable remap resolution exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_target_ref: Option<String>,
    /// Source refs that can revalidate the anchor.
    pub source_refs: Vec<NavigationSourceRef>,
    /// Scope identity attached to the anchor.
    pub scope: NavigationScopeRef,
    /// Current drift state.
    pub drift_state: NavigationDriftState,
    /// Explicit drift or restore reason visible to users and support exports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift_reason: Option<String>,
    /// Recovery choices the surface may offer.
    #[serde(default)]
    pub recovery_choices: Vec<String>,
}

impl DurableNavigationAnchor {
    fn validate(&self, artifact_ref: &str, findings: &mut Vec<NavigationContinuityFinding>) {
        validate_nonempty(
            findings,
            NavigationContinuityFindingKind::MissingIdentity,
            artifact_ref,
            &self.anchor_id,
            "anchor_id must be non-empty",
        );
        validate_nonempty(
            findings,
            NavigationContinuityFindingKind::MissingIdentity,
            artifact_ref,
            &self.canonical_target_ref,
            "canonical_target_ref must be non-empty",
        );
        validate_nonempty(
            findings,
            NavigationContinuityFindingKind::MissingIdentity,
            artifact_ref,
            &self.scope.workspace_ref,
            "scope.workspace_ref must be non-empty",
        );
        validate_nonempty(
            findings,
            NavigationContinuityFindingKind::MissingIdentity,
            artifact_ref,
            &self.scope.scope_ref,
            "scope.scope_ref must be non-empty",
        );
        validate_nonempty(
            findings,
            NavigationContinuityFindingKind::MissingIdentity,
            artifact_ref,
            &self.scope.trust_contract_ref,
            "scope.trust_contract_ref must be non-empty",
        );
        if self.source_refs.is_empty() {
            push_finding(
                findings,
                NavigationContinuityFindingKind::MissingProviderSource,
                artifact_ref,
                "durable anchor must carry at least one provider/source ref",
            );
        }
        for source in &self.source_refs {
            validate_nonempty(
                findings,
                NavigationContinuityFindingKind::MissingProviderSource,
                artifact_ref,
                &source.provider_class,
                "source_refs[].provider_class must be non-empty",
            );
            validate_nonempty(
                findings,
                NavigationContinuityFindingKind::MissingProviderSource,
                artifact_ref,
                &source.provider_ref,
                "source_refs[].provider_ref must be non-empty",
            );
        }
        match self.drift_state {
            NavigationDriftState::Bound => {
                if self
                    .resolved_target_ref
                    .as_deref()
                    .map_or(true, str::is_empty)
                {
                    push_finding(
                        findings,
                        NavigationContinuityFindingKind::MissingResolvedTarget,
                        artifact_ref,
                        "bound anchors must carry resolved_target_ref",
                    );
                }
            }
            NavigationDriftState::Remapped => {
                if self
                    .resolved_target_ref
                    .as_deref()
                    .map_or(true, str::is_empty)
                {
                    push_finding(
                        findings,
                        NavigationContinuityFindingKind::MissingResolvedTarget,
                        artifact_ref,
                        "remapped anchors must carry resolved_target_ref",
                    );
                }
            }
            state if state.requires_visible_reason() => {
                if self.drift_reason.as_deref().map_or(true, str::is_empty) {
                    push_finding(
                        findings,
                        NavigationContinuityFindingKind::MissingDriftReason,
                        artifact_ref,
                        "drifted, missing, unavailable, or archived anchors must carry drift_reason",
                    );
                }
                if self.recovery_choices.is_empty() {
                    push_finding(
                        findings,
                        NavigationContinuityFindingKind::MissingRecoveryChoice,
                        artifact_ref,
                        "drifted, missing, unavailable, or archived anchors must carry recovery choices",
                    );
                }
            }
            _ => {}
        }
    }
}

/// Stable remap evidence for one anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableAnchorRemap {
    /// Stable remap id.
    pub remap_id: String,
    /// Anchor being remapped.
    pub anchor_id_ref: String,
    /// Original canonical target ref.
    pub from_target_ref: String,
    /// Current target ref.
    pub to_target_ref: String,
    /// Stable evidence classes such as filesystem identity, symbol id, or docs-pack entry id.
    pub stable_evidence_refs: Vec<String>,
    /// True when the remap was produced by nearest-line, nearest-symbol, or path similarity fallback.
    pub used_nearby_fallback: bool,
}

/// Breadcrumb trail object with stable segment anchors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreadcrumbTrail {
    /// Stable breadcrumb trail id.
    pub trail_id: String,
    /// Anchor refs for each trail segment.
    pub segment_anchor_refs: Vec<String>,
    /// Trail-level drift state.
    pub drift_state: NavigationDriftState,
}

/// Outline snapshot object with stable node anchors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutlineSnapshot {
    /// Stable outline snapshot id.
    pub outline_id: String,
    /// Provider or parser snapshot ref.
    pub snapshot_ref: String,
    /// Anchor refs for outline nodes.
    pub node_anchor_refs: Vec<String>,
    /// Snapshot freshness class.
    pub freshness_class: String,
    /// True when the outline is partial for the declared scope.
    pub partial: bool,
    /// Snapshot-level drift state.
    pub drift_state: NavigationDriftState,
}

/// Bookmark or named navigation mark.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationMark {
    /// Stable mark id.
    pub mark_id: String,
    /// Anchor ref owned by this mark.
    pub anchor_ref: String,
    /// Optional export-safe label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Mark-level drift state.
    pub drift_state: NavigationDriftState,
}

/// Back/forward or recent-location history entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationHistoryEntry {
    /// Stable history entry id.
    pub history_entry_id: String,
    /// Surface where the entry originated.
    pub origin_surface: NavigationContinuitySurface,
    /// Origin anchor ref.
    pub origin_anchor_ref: String,
    /// Destination anchor ref.
    pub destination_anchor_ref: String,
    /// True when this entry participates in the back stack.
    pub back_stack: bool,
    /// True when this entry participates in the forward stack.
    pub forward_stack: bool,
    /// True when this entry participates in recent locations.
    pub recent_location: bool,
    /// Entry-level drift state.
    pub drift_state: NavigationDriftState,
}

/// Peek return context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeekContext {
    /// Stable peek context id.
    pub peek_context_id: String,
    /// Anchor that opened the peek.
    pub invocation_anchor_ref: String,
    /// Anchor used when returning from peek.
    pub return_anchor_ref: String,
    /// Surface where the peek was opened.
    pub surface: NavigationContinuitySurface,
    /// Peek-level drift state.
    pub drift_state: NavigationDriftState,
}

/// One artifact row in a continuity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationContinuityArtifact {
    /// Stable artifact id.
    pub artifact_id: String,
    /// Artifact kind.
    pub artifact_kind: NavigationContinuityArtifactKind,
    /// Anchor refs owned or used by this artifact.
    pub anchor_refs: Vec<String>,
    /// Artifact-level drift state.
    pub drift_state: NavigationDriftState,
    /// Breadcrumb trail payload when `artifact_kind` is breadcrumb.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breadcrumb_trail: Option<BreadcrumbTrail>,
    /// Outline payload when `artifact_kind` is outline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outline_snapshot: Option<OutlineSnapshot>,
    /// Bookmark/mark payload when `artifact_kind` is navigation mark.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub navigation_mark: Option<NavigationMark>,
    /// History payload when `artifact_kind` is history.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_entry: Option<NavigationHistoryEntry>,
    /// Peek payload when `artifact_kind` is peek.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peek_context: Option<PeekContext>,
}

/// Session-restore result for one preserved artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreNavigationArtifact {
    /// Artifact ref being restored.
    pub artifact_id_ref: String,
    /// Drift state after restore validation.
    pub drift_state: NavigationDriftState,
    /// True when the target still resolves under workspace/trust/scope rules.
    pub target_resolves_under_current_scope: bool,
    /// True when the artifact remains visible even if exact restore failed.
    pub artifact_preserved: bool,
    /// Visible restore reason shown in-product and export.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_reason: Option<String>,
    /// Recovery choices shown for the artifact.
    pub recovery_choices: Vec<String>,
}

/// Restore packet preserving navigation artifacts with explicit drift reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreNavigationPacket {
    /// Stable restore packet id.
    pub restore_packet_id: String,
    /// Restore source snapshot ref.
    pub restore_source_ref: String,
    /// Restored artifacts.
    pub artifacts: Vec<RestoreNavigationArtifact>,
}

/// Support/export projection for one consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationConsumerProjection {
    /// Consumer surface.
    pub surface: NavigationContinuitySurface,
    /// Packet id visible to this consumer.
    pub packet_id_ref: String,
    /// True when IDs remain unchanged.
    pub preserves_export_safe_ids: bool,
    /// True when the full six-state drift vocabulary remains visible.
    pub preserves_full_drift_vocabulary: bool,
    /// True when provider/source refs remain visible.
    pub preserves_provider_source_refs: bool,
    /// True when restore reasons remain visible.
    pub preserves_restore_reasons: bool,
    /// True when the consumer keeps origin, destination, and scope refs attributable.
    pub preserves_origin_destination_scope: bool,
}

/// Stable packet shared by navigation surfaces and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationContinuityPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Source schema ref.
    pub schema_ref: String,
    /// Source docs ref.
    pub doc_ref: String,
    /// Durable anchors in the packet.
    pub anchors: Vec<DurableNavigationAnchor>,
    /// Stable remap rows used by remapped anchors.
    pub remaps: Vec<StableAnchorRemap>,
    /// Continuity artifacts.
    pub artifacts: Vec<NavigationContinuityArtifact>,
    /// Restore packet.
    pub restore_packet: RestoreNavigationPacket,
    /// Consumer projections.
    pub consumer_projections: Vec<NavigationConsumerProjection>,
    /// Covered drift vocabulary declared by the packet.
    pub covered_drift_states: Vec<NavigationDriftState>,
    /// Validation findings stored with the packet.
    #[serde(default)]
    pub validation_findings: Vec<NavigationContinuityFinding>,
}

impl NavigationContinuityPacket {
    /// Validates packet invariants and returns all findings.
    pub fn validate(&self) -> Vec<NavigationContinuityFinding> {
        let mut findings = Vec::new();
        self.validate_identity(&mut findings);
        self.validate_coverage(&mut findings);
        self.validate_anchors(&mut findings);
        self.validate_remaps(&mut findings);
        self.validate_artifacts(&mut findings);
        self.validate_restore(&mut findings);
        self.validate_projections(&mut findings);
        findings
    }

    /// Returns true when validation emits no blocker finding.
    pub fn is_stable(&self) -> bool {
        self.validate()
            .iter()
            .all(|finding| finding.severity != NavigationContinuityFindingSeverity::Blocker)
    }

    fn validate_identity(&self, findings: &mut Vec<NavigationContinuityFinding>) {
        if self.record_kind != BOOKMARK_HISTORY_CONTINUITY_PACKET_RECORD_KIND {
            push_finding(
                findings,
                NavigationContinuityFindingKind::WrongRecordKind,
                &self.packet_id,
                "record_kind does not match bookmark/history continuity packet",
            );
        }
        if self.schema_version != BOOKMARK_HISTORY_CONTINUITY_SCHEMA_VERSION {
            push_finding(
                findings,
                NavigationContinuityFindingKind::WrongSchemaVersion,
                &self.packet_id,
                "schema_version does not match bookmark/history continuity schema",
            );
        }
        validate_nonempty(
            findings,
            NavigationContinuityFindingKind::MissingIdentity,
            &self.packet_id,
            &self.packet_id,
            "packet_id must be non-empty",
        );
        if self.schema_ref != BOOKMARK_HISTORY_CONTINUITY_SCHEMA_REF {
            push_finding(
                findings,
                NavigationContinuityFindingKind::SchemaDocRefMismatch,
                &self.packet_id,
                "schema_ref must point at the checked-in continuity schema",
            );
        }
        if self.doc_ref != BOOKMARK_HISTORY_CONTINUITY_DOC_REF {
            push_finding(
                findings,
                NavigationContinuityFindingKind::SchemaDocRefMismatch,
                &self.packet_id,
                "doc_ref must point at the checked-in continuity document",
            );
        }
    }

    fn validate_coverage(&self, findings: &mut Vec<NavigationContinuityFinding>) {
        let covered: BTreeSet<_> = self.covered_drift_states.iter().copied().collect();
        for drift_state in REQUIRED_DRIFT_STATES {
            if !covered.contains(&drift_state) {
                push_finding(
                    findings,
                    NavigationContinuityFindingKind::DriftVocabularyDropped,
                    drift_state.as_str(),
                    "packet must cover every stable drift state",
                );
            }
        }
    }

    fn validate_anchors(&self, findings: &mut Vec<NavigationContinuityFinding>) {
        if self.anchors.is_empty() {
            push_finding(
                findings,
                NavigationContinuityFindingKind::MissingAnchor,
                &self.packet_id,
                "packet must include durable anchors",
            );
        }
        for anchor in &self.anchors {
            anchor.validate(&anchor.anchor_id, findings);
        }
    }

    fn validate_remaps(&self, findings: &mut Vec<NavigationContinuityFinding>) {
        let anchor_ids: BTreeSet<_> = self
            .anchors
            .iter()
            .map(|anchor| anchor.anchor_id.as_str())
            .collect();
        for anchor in self
            .anchors
            .iter()
            .filter(|anchor| anchor.drift_state == NavigationDriftState::Remapped)
        {
            let Some(remap) = self
                .remaps
                .iter()
                .find(|remap| remap.anchor_id_ref == anchor.anchor_id)
            else {
                push_finding(
                    findings,
                    NavigationContinuityFindingKind::MissingStableRemapEvidence,
                    &anchor.anchor_id,
                    "remapped anchors must cite a stable remap row",
                );
                continue;
            };
            if remap.used_nearby_fallback {
                push_finding(
                    findings,
                    NavigationContinuityFindingKind::SilentRetargetForbidden,
                    &remap.remap_id,
                    "stable remap must not use nearest target fallback",
                );
            }
            if remap.stable_evidence_refs.is_empty() {
                push_finding(
                    findings,
                    NavigationContinuityFindingKind::MissingStableRemapEvidence,
                    &remap.remap_id,
                    "stable remap must carry evidence refs",
                );
            }
        }
        for remap in &self.remaps {
            if !anchor_ids.contains(remap.anchor_id_ref.as_str()) {
                push_finding(
                    findings,
                    NavigationContinuityFindingKind::MissingAnchor,
                    &remap.remap_id,
                    "remap anchor_id_ref must point at a packet anchor",
                );
            }
        }
    }

    fn validate_artifacts(&self, findings: &mut Vec<NavigationContinuityFinding>) {
        let anchor_ids: BTreeSet<_> = self
            .anchors
            .iter()
            .map(|anchor| anchor.anchor_id.as_str())
            .collect();
        for artifact in &self.artifacts {
            validate_nonempty(
                findings,
                NavigationContinuityFindingKind::MissingIdentity,
                &artifact.artifact_id,
                &artifact.artifact_id,
                "artifact_id must be non-empty",
            );
            if artifact.anchor_refs.is_empty() {
                push_finding(
                    findings,
                    NavigationContinuityFindingKind::MissingAnchor,
                    &artifact.artifact_id,
                    "artifact must cite at least one anchor",
                );
            }
            for anchor_ref in &artifact.anchor_refs {
                if !anchor_ids.contains(anchor_ref.as_str()) {
                    push_finding(
                        findings,
                        NavigationContinuityFindingKind::MissingAnchor,
                        &artifact.artifact_id,
                        "artifact anchor_refs must point at packet anchors",
                    );
                }
            }
            let payload_count = [
                artifact.breadcrumb_trail.is_some(),
                artifact.outline_snapshot.is_some(),
                artifact.navigation_mark.is_some(),
                artifact.history_entry.is_some(),
                artifact.peek_context.is_some(),
            ]
            .into_iter()
            .filter(|present| *present)
            .count();
            if payload_count != 1 {
                push_finding(
                    findings,
                    NavigationContinuityFindingKind::ArtifactPayloadMismatch,
                    &artifact.artifact_id,
                    "artifact must carry exactly one payload matching artifact_kind",
                );
            }
            let kind_matches = match artifact.artifact_kind {
                NavigationContinuityArtifactKind::BreadcrumbTrail => {
                    artifact.breadcrumb_trail.is_some()
                }
                NavigationContinuityArtifactKind::OutlineSnapshot => {
                    artifact.outline_snapshot.is_some()
                }
                NavigationContinuityArtifactKind::NavigationMark => {
                    artifact.navigation_mark.is_some()
                }
                NavigationContinuityArtifactKind::NavigationHistoryEntry => {
                    artifact.history_entry.is_some()
                }
                NavigationContinuityArtifactKind::PeekContext => artifact.peek_context.is_some(),
            };
            if !kind_matches {
                push_finding(
                    findings,
                    NavigationContinuityFindingKind::ArtifactPayloadMismatch,
                    &artifact.artifact_id,
                    "artifact payload must match artifact_kind",
                );
            }
        }
    }

    fn validate_restore(&self, findings: &mut Vec<NavigationContinuityFinding>) {
        let artifact_ids: BTreeSet<_> = self
            .artifacts
            .iter()
            .map(|artifact| artifact.artifact_id.as_str())
            .collect();
        for restored in &self.restore_packet.artifacts {
            if !artifact_ids.contains(restored.artifact_id_ref.as_str()) {
                push_finding(
                    findings,
                    NavigationContinuityFindingKind::RestoreArtifactMissing,
                    &restored.artifact_id_ref,
                    "restore artifact must point at a packet artifact",
                );
            }
            if !restored.target_resolves_under_current_scope {
                if !restored.artifact_preserved {
                    push_finding(
                        findings,
                        NavigationContinuityFindingKind::RestoreDroppedDriftedArtifact,
                        &restored.artifact_id_ref,
                        "restore must preserve drifted artifacts with visible reasons",
                    );
                }
                if restored
                    .restore_reason
                    .as_deref()
                    .map_or(true, str::is_empty)
                {
                    push_finding(
                        findings,
                        NavigationContinuityFindingKind::RestoreReasonMissing,
                        &restored.artifact_id_ref,
                        "restore must surface a reason when exact target resolution fails",
                    );
                }
                if restored.recovery_choices.is_empty() {
                    push_finding(
                        findings,
                        NavigationContinuityFindingKind::MissingRecoveryChoice,
                        &restored.artifact_id_ref,
                        "restore must surface recovery choices when exact target resolution fails",
                    );
                }
            }
        }
    }

    fn validate_projections(&self, findings: &mut Vec<NavigationContinuityFinding>) {
        let covered: BTreeSet<_> = self
            .consumer_projections
            .iter()
            .map(|projection| projection.surface)
            .collect();
        for surface in REQUIRED_CONTINUITY_SURFACES {
            if !covered.contains(&surface) {
                push_finding(
                    findings,
                    NavigationContinuityFindingKind::ConsumerProjectionMissing,
                    surface.as_str(),
                    "packet must include every required consumer surface",
                );
            }
        }
        for projection in &self.consumer_projections {
            if projection.packet_id_ref != self.packet_id {
                push_finding(
                    findings,
                    NavigationContinuityFindingKind::ConsumerProjectionDrift,
                    projection.surface.as_str(),
                    "consumer projection must cite the same packet id",
                );
            }
            if !projection.preserves_export_safe_ids
                || !projection.preserves_full_drift_vocabulary
                || !projection.preserves_provider_source_refs
                || !projection.preserves_restore_reasons
                || !projection.preserves_origin_destination_scope
            {
                push_finding(
                    findings,
                    NavigationContinuityFindingKind::ConsumerProjectionDrift,
                    projection.surface.as_str(),
                    "consumer projection dropped continuity truth",
                );
            }
        }
    }
}

/// Validation finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationContinuityFindingSeverity {
    /// Informational finding.
    Info,
    /// Finding that needs review but does not block the packet by itself.
    Warning,
    /// Finding that blocks stable publication.
    Blocker,
}

/// Closed validation finding vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationContinuityFindingKind {
    /// Record kind does not match.
    WrongRecordKind,
    /// Schema version does not match.
    WrongSchemaVersion,
    /// Schema or docs refs drifted.
    SchemaDocRefMismatch,
    /// Required identity field is empty.
    MissingIdentity,
    /// Anchor row is missing.
    MissingAnchor,
    /// Provider/source refs are missing or incomplete.
    MissingProviderSource,
    /// Bound or remapped anchor lacks a resolved target.
    MissingResolvedTarget,
    /// Drifted artifact lacks a visible drift reason.
    MissingDriftReason,
    /// Drifted artifact lacks recovery choices.
    MissingRecoveryChoice,
    /// Remapped anchor lacks stable remap evidence.
    MissingStableRemapEvidence,
    /// Remap attempted a nearby fallback.
    SilentRetargetForbidden,
    /// Packet dropped one of the required drift states.
    DriftVocabularyDropped,
    /// Artifact payload is absent or mismatched.
    ArtifactPayloadMismatch,
    /// Restore row points at an unknown artifact.
    RestoreArtifactMissing,
    /// Restore dropped an unresolved artifact.
    RestoreDroppedDriftedArtifact,
    /// Restore lacks a visible reason.
    RestoreReasonMissing,
    /// Consumer projection is missing.
    ConsumerProjectionMissing,
    /// Consumer projection drifted from packet truth.
    ConsumerProjectionDrift,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationContinuityFinding {
    /// Closed finding kind.
    pub finding_kind: NavigationContinuityFindingKind,
    /// Finding severity.
    pub severity: NavigationContinuityFindingSeverity,
    /// Subject ref.
    pub subject_ref: String,
    /// Support-safe summary.
    pub summary: String,
}

impl NavigationContinuityFinding {
    fn blocker(
        finding_kind: NavigationContinuityFindingKind,
        subject_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity: NavigationContinuityFindingSeverity::Blocker,
            subject_ref: subject_ref.into(),
            summary: summary.into(),
        }
    }
}

/// Validation error wrapper for callers that need a `Result`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationContinuityError {
    findings: Vec<NavigationContinuityFinding>,
}

impl NavigationContinuityError {
    /// Returns validation findings that caused the error.
    pub fn findings(&self) -> &[NavigationContinuityFinding] {
        &self.findings
    }
}

impl fmt::Display for NavigationContinuityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "navigation continuity packet has {} validation finding(s)",
            self.findings.len()
        )
    }
}

impl Error for NavigationContinuityError {}

/// Validates a packet and returns an error when any blocker is present.
pub fn validate_navigation_continuity_packet(
    packet: &NavigationContinuityPacket,
) -> Result<(), NavigationContinuityError> {
    let findings = packet.validate();
    if findings
        .iter()
        .any(|finding| finding.severity == NavigationContinuityFindingSeverity::Blocker)
    {
        Err(NavigationContinuityError { findings })
    } else {
        Ok(())
    }
}

fn validate_nonempty(
    findings: &mut Vec<NavigationContinuityFinding>,
    finding_kind: NavigationContinuityFindingKind,
    subject_ref: &str,
    value: &str,
    summary: &str,
) {
    if value.trim().is_empty() {
        push_finding(findings, finding_kind, subject_ref, summary);
    }
}

fn push_finding(
    findings: &mut Vec<NavigationContinuityFinding>,
    finding_kind: NavigationContinuityFindingKind,
    subject_ref: &str,
    summary: &str,
) {
    findings.push(NavigationContinuityFinding::blocker(
        finding_kind,
        subject_ref,
        summary,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source_ref() -> NavigationSourceRef {
        NavigationSourceRef {
            provider_class: "semantic_graph".to_owned(),
            provider_ref: "provider:graph:current".to_owned(),
            freshness_class: "authoritative_live".to_owned(),
            partial: false,
        }
    }

    fn scope_ref() -> NavigationScopeRef {
        NavigationScopeRef {
            workspace_ref: "workspace:current".to_owned(),
            scope_ref: "scope:full".to_owned(),
            trust_contract_ref: "trust:local:source".to_owned(),
            scope_class: "full_workspace".to_owned(),
        }
    }

    fn anchor(id: &str, drift_state: NavigationDriftState) -> DurableNavigationAnchor {
        DurableNavigationAnchor {
            anchor_id: id.to_owned(),
            canonical_target_ref: format!("target:{id}:old"),
            resolved_target_ref: matches!(
                drift_state,
                NavigationDriftState::Bound | NavigationDriftState::Remapped
            )
            .then(|| format!("target:{id}:current")),
            source_refs: vec![source_ref()],
            scope: scope_ref(),
            drift_state,
            drift_reason: drift_state
                .requires_visible_reason()
                .then(|| format!("{} requires review", drift_state.as_str())),
            recovery_choices: drift_state
                .requires_visible_reason()
                .then(|| vec!["inspect_drift".to_owned()])
                .unwrap_or_default(),
        }
    }

    fn stable_packet() -> NavigationContinuityPacket {
        let anchors = REQUIRED_DRIFT_STATES
            .into_iter()
            .enumerate()
            .map(|(index, state)| anchor(&format!("anchor:{index}"), state))
            .collect::<Vec<_>>();
        NavigationContinuityPacket {
            record_kind: BOOKMARK_HISTORY_CONTINUITY_PACKET_RECORD_KIND.to_owned(),
            schema_version: BOOKMARK_HISTORY_CONTINUITY_SCHEMA_VERSION,
            packet_id: "packet:navigation-continuity:stable".to_owned(),
            schema_ref: BOOKMARK_HISTORY_CONTINUITY_SCHEMA_REF.to_owned(),
            doc_ref: BOOKMARK_HISTORY_CONTINUITY_DOC_REF.to_owned(),
            remaps: vec![StableAnchorRemap {
                remap_id: "remap:anchor:1".to_owned(),
                anchor_id_ref: "anchor:1".to_owned(),
                from_target_ref: "target:anchor:1:old".to_owned(),
                to_target_ref: "target:anchor:1:current".to_owned(),
                stable_evidence_refs: vec!["evidence:symbol-stable-id".to_owned()],
                used_nearby_fallback: false,
            }],
            artifacts: vec![
                NavigationContinuityArtifact {
                    artifact_id: "artifact:breadcrumb".to_owned(),
                    artifact_kind: NavigationContinuityArtifactKind::BreadcrumbTrail,
                    anchor_refs: vec!["anchor:0".to_owned()],
                    drift_state: NavigationDriftState::Bound,
                    breadcrumb_trail: Some(BreadcrumbTrail {
                        trail_id: "trail:editor".to_owned(),
                        segment_anchor_refs: vec!["anchor:0".to_owned()],
                        drift_state: NavigationDriftState::Bound,
                    }),
                    outline_snapshot: None,
                    navigation_mark: None,
                    history_entry: None,
                    peek_context: None,
                },
                NavigationContinuityArtifact {
                    artifact_id: "artifact:history".to_owned(),
                    artifact_kind: NavigationContinuityArtifactKind::NavigationHistoryEntry,
                    anchor_refs: vec!["anchor:1".to_owned(), "anchor:2".to_owned()],
                    drift_state: NavigationDriftState::Remapped,
                    breadcrumb_trail: None,
                    outline_snapshot: None,
                    navigation_mark: None,
                    history_entry: Some(NavigationHistoryEntry {
                        history_entry_id: "history:recent:1".to_owned(),
                        origin_surface: NavigationContinuitySurface::Search,
                        origin_anchor_ref: "anchor:1".to_owned(),
                        destination_anchor_ref: "anchor:2".to_owned(),
                        back_stack: true,
                        forward_stack: false,
                        recent_location: true,
                        drift_state: NavigationDriftState::Remapped,
                    }),
                    peek_context: None,
                },
            ],
            restore_packet: RestoreNavigationPacket {
                restore_packet_id: "restore:navigation:1".to_owned(),
                restore_source_ref: "restore:snapshot:1".to_owned(),
                artifacts: vec![RestoreNavigationArtifact {
                    artifact_id_ref: "artifact:history".to_owned(),
                    drift_state: NavigationDriftState::Drifted,
                    target_resolves_under_current_scope: false,
                    artifact_preserved: true,
                    restore_reason: Some("branch changed before restore".to_owned()),
                    recovery_choices: vec!["inspect_drift".to_owned()],
                }],
            },
            consumer_projections: REQUIRED_CONTINUITY_SURFACES
                .into_iter()
                .map(|surface| NavigationConsumerProjection {
                    surface,
                    packet_id_ref: "packet:navigation-continuity:stable".to_owned(),
                    preserves_export_safe_ids: true,
                    preserves_full_drift_vocabulary: true,
                    preserves_provider_source_refs: true,
                    preserves_restore_reasons: true,
                    preserves_origin_destination_scope: true,
                })
                .collect(),
            anchors,
            covered_drift_states: REQUIRED_DRIFT_STATES.to_vec(),
            validation_findings: Vec::new(),
        }
    }

    #[test]
    fn stable_packet_validates() {
        let packet = stable_packet();
        assert!(packet.validate().is_empty());
        assert!(validate_navigation_continuity_packet(&packet).is_ok());
    }

    #[test]
    fn nearby_fallback_blocks_stable_remap() {
        let mut packet = stable_packet();
        packet.remaps[0].used_nearby_fallback = true;

        let findings = packet.validate();

        assert!(findings.iter().any(|finding| {
            finding.finding_kind == NavigationContinuityFindingKind::SilentRetargetForbidden
        }));
    }

    #[test]
    fn restore_must_preserve_drifted_artifact() {
        let mut packet = stable_packet();
        packet.restore_packet.artifacts[0].artifact_preserved = false;

        let findings = packet.validate();

        assert!(findings.iter().any(|finding| {
            finding.finding_kind == NavigationContinuityFindingKind::RestoreDroppedDriftedArtifact
        }));
    }

    #[test]
    fn baseline_fixture_validates() {
        let fixture = include_str!(
            "../../../../fixtures/search/m4/bookmark-history-and-drift-continuity/baseline_stable.json"
        );
        let packet: NavigationContinuityPacket =
            serde_json::from_str(fixture).expect("baseline fixture parses");

        assert!(packet.validate().is_empty());
    }
}
