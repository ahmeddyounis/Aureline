//! Breadcrumb, outline, bookmark, recent-location, back/forward, and peek
//! continuity bound to canonical anchors across the M5 editor, diff, notebook,
//! docs, search, and graph surfaces.
//!
//! Where [`aureline_navigation`] owns the canonical drift vocabulary
//! ([`NavigationDriftState`]), the continuity surfaces
//! ([`NavigationContinuitySurface`]), and the artifact kinds
//! ([`NavigationContinuityArtifactKind`]), this module binds that vocabulary to
//! the M5 search-session and result-identity world so a bookmark, breadcrumb,
//! outline node, history entry, or peek return context resolves through the same
//! durable [`build_canonical_result_id`] identity the search lane already
//! materializes — and degrades *visibly* when the anchor drifts, the workset
//! narrows, freshness lapses, or the target disappears.
//!
//! The contract is deliberately honesty-first:
//!
//! - [`ContinuityArtifactBinding`] is one continuity artifact bound to one
//!   canonical anchor. It keeps the canonical pre-remap target ref separate from
//!   the resolved target ref, names the drift state, and — when the drift state
//!   requires review — carries a user-visible reason and recovery choices instead
//!   of silently relocating to a nearby symbol, line, or document. A remap is
//!   only allowed with stable evidence and never via a nearest-target fallback
//!   (`used_nearby_fallback` must be `false`).
//! - [`SurfaceContinuityBinding`] groups artifacts under one of the six surfaces
//!   so editor, diff, notebook, docs, search, and graph share *one* continuity
//!   vocabulary.
//! - [`NavigationContinuityRestoreProjection`] proves restore reopens continuity
//!   artifacts with their drift/missing-target reasons preserved instead of
//!   dropping them.
//! - [`ContinuityConsumerProjection`] proves the same continuity objects are
//!   reused by the product UI, back/forward history, session restore, and
//!   support replay consumers — never re-minted from rendered chrome text.
//!
//! The packet is metadata-only by construction: it carries no raw query text,
//! source bodies, provider payloads, secrets, or private rank weights; query
//! sessions are referenced hash-only and convenience routing never widens
//! authority.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_navigation::{
    NavigationContinuityArtifactKind, NavigationContinuitySurface, NavigationDriftState,
    REQUIRED_CONTINUITY_SURFACES, REQUIRED_DRIFT_STATES,
};

use crate::query_session::stable_query_hash;
use crate::result_id::{build_canonical_result_id, StableResultKind};
use crate::result_truth_packet::SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF;

/// Stable record-kind tag for [`NavigationContinuityBindingPacket`].
pub const NAVIGATION_CONTINUITY_BINDING_PACKET_RECORD_KIND: &str =
    "navigation_continuity_binding_packet";

/// Stable record-kind tag for [`NavigationContinuityBindingSupportExport`].
pub const NAVIGATION_CONTINUITY_BINDING_SUPPORT_EXPORT_RECORD_KIND: &str =
    "navigation_continuity_binding_support_export";

/// Integer schema version for the navigation-continuity binding packet.
pub const NAVIGATION_CONTINUITY_BINDING_SCHEMA_VERSION: u32 = 1;

/// Stable packet identifier reused by every consumer projection.
pub const NAVIGATION_CONTINUITY_BINDING_PACKET_ID: &str = "search.m5.navigation_continuity.v1";

/// Repository-relative path of the boundary schema.
pub const NAVIGATION_CONTINUITY_BINDING_SCHEMA_REF: &str =
    "schemas/search/navigation-continuity.schema.json";

/// Repository-relative path of the reviewer-facing contract document.
pub const NAVIGATION_CONTINUITY_BINDING_DOC_REF: &str = "docs/search/navigation-continuity.md";

/// Repository-relative path of the checked review artifact.
pub const NAVIGATION_CONTINUITY_BINDING_ARTIFACT_REF: &str =
    "artifacts/search/m5/navigation-continuity.md";

/// Repository-relative path of the protected fixture directory.
pub const NAVIGATION_CONTINUITY_BINDING_FIXTURE_DIR: &str =
    "fixtures/search/m5/navigation-continuity";

/// Artifact kinds the continuity matrix realizes across its surfaces.
pub const COVERED_ARTIFACT_KINDS: [NavigationContinuityArtifactKind; 5] = [
    NavigationContinuityArtifactKind::BreadcrumbTrail,
    NavigationContinuityArtifactKind::OutlineSnapshot,
    NavigationContinuityArtifactKind::NavigationMark,
    NavigationContinuityArtifactKind::NavigationHistoryEntry,
    NavigationContinuityArtifactKind::PeekContext,
];

/// Workspace id used by the seeded corpus.
const SEEDED_WORKSPACE_ID: &str = "ws-aureline";

/// Fixed generation timestamp for the seeded corpus.
const SEEDED_GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// Role a navigation-history artifact plays in the recent-navigation model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryRole {
    /// Entry reachable from the back stack.
    Back,
    /// Entry reachable from the forward stack.
    Forward,
    /// Entry surfaced in the recent-locations list.
    Recent,
}

impl HistoryRole {
    /// Every history role, in canonical order.
    pub const ALL: [Self; 3] = [Self::Back, Self::Forward, Self::Recent];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Back => "back",
            Self::Forward => "forward",
            Self::Recent => "recent",
        }
    }
}

/// One first consumer that reuses the same continuity objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityConsumerClass {
    /// The live shell chrome (breadcrumb bar, outline tree, bookmark gutter,
    /// back/forward, recent-locations, peek overlay).
    ProductUi,
    /// Back/forward and recent-navigation replay.
    HistoryBackForward,
    /// Session restore reopening continuity artifacts after a restart.
    SessionRestore,
    /// Support/export replay and inspection tooling.
    SupportReplay,
}

impl ContinuityConsumerClass {
    /// Every required first consumer, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::ProductUi,
        Self::HistoryBackForward,
        Self::SessionRestore,
        Self::SupportReplay,
    ];

    /// Stable snake-case token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductUi => "product_ui",
            Self::HistoryBackForward => "history_back_forward",
            Self::SessionRestore => "session_restore",
            Self::SupportReplay => "support_replay",
        }
    }

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProductUi => "Product UI",
            Self::HistoryBackForward => "History / back-forward",
            Self::SessionRestore => "Session restore",
            Self::SupportReplay => "Support replay",
        }
    }
}

/// One continuity artifact bound to one canonical anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityArtifactBinding {
    /// Stable artifact id (distinct from the result identity).
    pub artifact_id: String,
    /// Artifact kind realized by this binding.
    pub artifact_kind: NavigationContinuityArtifactKind,
    /// Surface that owns the artifact.
    pub surface: NavigationContinuitySurface,
    /// Export-safe display label, when the artifact carries one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Canonical anchor identity resolved *before* any remap rules run.
    pub canonical_target_ref: String,
    /// Currently resolved target ref; present iff the artifact is bound or
    /// resolved through stable remap evidence, absent when drift requires review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_target_ref: Option<String>,
    /// Durable, surface-independent search/docs/graph result URN this artifact
    /// binds to, when it originates from a result-bearing surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_id_ref: Option<String>,
    /// Hash-only durable query-session ref this artifact came from, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_session_id_ref: Option<String>,
    /// Origin anchor for history entries (where navigation came from) and the
    /// return anchor for peek contexts; absent for other artifact kinds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_target_ref: Option<String>,
    /// Recent-navigation role; present iff the artifact is a history entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_role: Option<HistoryRole>,
    /// Current drift state.
    pub drift_state: NavigationDriftState,
    /// User-visible drift reason; present iff the drift state requires review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift_reason: Option<String>,
    /// Stable remap evidence refs; non-empty iff the artifact was remapped.
    pub remap_evidence_refs: Vec<String>,
    /// True when a nearest-target fallback was used (must always be false).
    pub used_nearby_fallback: bool,
    /// User-visible recovery choices; non-empty iff the drift requires review.
    pub recovery_choices: Vec<String>,
    /// True when surfacing the artifact did not widen authority.
    pub authority_not_widened: bool,
    /// True when raw query text, source bodies, secrets, and weights are excluded.
    pub raw_boundary_material_excluded: bool,
    /// Review-safe summary for downstream consumers.
    pub summary: String,
}

impl ContinuityArtifactBinding {
    /// True when the artifact's drift state still needs visible review.
    pub fn requires_visible_reason(&self) -> bool {
        self.drift_state.requires_visible_reason()
    }

    /// True when the artifact is a back/forward/recent history entry.
    pub fn is_history_entry(&self) -> bool {
        self.artifact_kind == NavigationContinuityArtifactKind::NavigationHistoryEntry
    }

    /// True when the artifact is a peek return context.
    pub fn is_peek_context(&self) -> bool {
        self.artifact_kind == NavigationContinuityArtifactKind::PeekContext
    }
}

/// One surface and its continuity artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceContinuityBinding {
    /// Surface this row covers.
    pub surface: NavigationContinuitySurface,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Continuity artifacts owned by this surface.
    pub artifacts: Vec<ContinuityArtifactBinding>,
    /// Review-safe summary for downstream consumers.
    pub summary: String,
}

impl SurfaceContinuityBinding {
    /// Number of artifacts on this surface whose drift requires review.
    pub fn unresolved_artifact_count(&self) -> usize {
        self.artifacts
            .iter()
            .filter(|artifact| artifact.requires_visible_reason())
            .count()
    }
}

/// One restored continuity artifact preserved across a restart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoredContinuityArtifact {
    /// Artifact ref being restored.
    pub artifact_id_ref: String,
    /// Drift state evaluated after restore validation.
    pub drift_state: NavigationDriftState,
    /// True when the target still resolves under workspace/trust/scope rules.
    pub target_resolves_under_current_scope: bool,
    /// True when the artifact remains visible even if exact restore failed.
    pub artifact_preserved: bool,
    /// User-visible restore reason; present iff exact resolution failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_reason: Option<String>,
    /// User-visible recovery choices for the restored artifact.
    pub recovery_choices: Vec<String>,
}

/// Restore projection that preserves continuity artifacts with explicit reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationContinuityRestoreProjection {
    /// Stable restore projection id.
    pub restore_packet_id: String,
    /// Restore source snapshot ref.
    pub restore_source_ref: String,
    /// Restored artifacts.
    pub artifacts: Vec<RestoredContinuityArtifact>,
}

impl NavigationContinuityRestoreProjection {
    /// Number of restored artifacts whose target no longer resolves.
    pub fn preserved_unresolved_count(&self) -> usize {
        self.artifacts
            .iter()
            .filter(|artifact| !artifact.target_resolves_under_current_scope)
            .count()
    }
}

/// One consumer projection proving the continuity objects are reused, not rebuilt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityConsumerProjection {
    /// Consumer that reuses the continuity objects.
    pub consumer: ContinuityConsumerClass,
    /// Checked consumer or contract ref.
    pub consumer_ref: String,
    /// Packet identifier the consumer ingests verbatim.
    pub ingested_packet_id: String,
    /// True when the consumer preserves the export-safe artifact ids.
    pub preserves_export_safe_ids: bool,
    /// True when the consumer preserves the full drift vocabulary.
    pub preserves_drift_vocabulary: bool,
    /// True when the consumer preserves drift/missing-target reasons.
    pub preserves_drift_reasons: bool,
    /// True when the consumer preserves origin/destination/return refs.
    pub preserves_origin_destination: bool,
    /// True when the consumer reuses the same continuity objects, not a copy.
    pub reuses_same_continuity_objects: bool,
    /// True when the consumer widened authority (must be false).
    pub widens_authority: bool,
    /// True when ambient authority / credentials are excluded.
    pub ambient_authority_excluded: bool,
    /// Review-safe summary of the continuity contract.
    pub summary: String,
}

impl ContinuityConsumerProjection {
    fn reuses_continuity(&self) -> bool {
        self.preserves_export_safe_ids
            && self.preserves_drift_vocabulary
            && self.preserves_drift_reasons
            && self.preserves_origin_destination
            && self.reuses_same_continuity_objects
            && !self.widens_authority
            && self.ambient_authority_excluded
            && !self.consumer_ref.trim().is_empty()
    }
}

/// One validation finding emitted by [`NavigationContinuityBindingPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationContinuityBindingValidationFinding {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Navigation-continuity binding truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationContinuityBindingPacket {
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
    /// Existing lane contracts the continuity matrix composes.
    pub supporting_contract_refs: Vec<String>,
    /// Surfaces covered by the matrix.
    pub covered_surfaces: Vec<NavigationContinuitySurface>,
    /// Artifact kinds covered across the matrix.
    pub covered_artifact_kinds: Vec<NavigationContinuityArtifactKind>,
    /// Drift states covered across the matrix.
    pub covered_drift_states: Vec<NavigationDriftState>,
    /// Per-surface continuity bindings.
    pub surfaces: Vec<SurfaceContinuityBinding>,
    /// Restore projection preserving continuity artifacts across a restart.
    pub restore: NavigationContinuityRestoreProjection,
    /// Consumer projections that reuse the continuity objects.
    pub consumer_projections: Vec<ContinuityConsumerProjection>,
    /// Metadata-safe summary safe for support and release surfaces.
    pub export_safe_summary: String,
}

impl NavigationContinuityBindingPacket {
    /// Returns the surface binding for one surface, if present.
    pub fn surface_for(
        &self,
        surface: NavigationContinuitySurface,
    ) -> Option<&SurfaceContinuityBinding> {
        self.surfaces.iter().find(|row| row.surface == surface)
    }

    /// Iterates every artifact across every surface.
    pub fn all_artifacts(&self) -> impl Iterator<Item = &ContinuityArtifactBinding> {
        self.surfaces.iter().flat_map(|surface| &surface.artifacts)
    }

    /// Returns the artifact with the given id, if present.
    pub fn artifact_for(&self, artifact_id: &str) -> Option<&ContinuityArtifactBinding> {
        self.all_artifacts()
            .find(|artifact| artifact.artifact_id == artifact_id)
    }

    /// Returns the surface tokens realized across the matrix.
    pub fn realized_surface_tokens(&self) -> Vec<&'static str> {
        self.surfaces
            .iter()
            .map(|surface| surface.surface.as_str())
            .collect()
    }

    /// Returns the artifact-kind tokens realized across the matrix.
    pub fn realized_artifact_kind_tokens(&self) -> Vec<&'static str> {
        self.present_artifact_kinds()
            .into_iter()
            .map(NavigationContinuityArtifactKind::as_str)
            .collect()
    }

    /// Returns the drift-state tokens realized across the matrix.
    pub fn realized_drift_state_tokens(&self) -> Vec<&'static str> {
        self.present_drift_states()
            .into_iter()
            .map(NavigationDriftState::as_str)
            .collect()
    }

    /// Returns the history-role tokens realized across the matrix.
    pub fn realized_history_role_tokens(&self) -> Vec<&'static str> {
        self.present_history_roles()
            .into_iter()
            .map(HistoryRole::as_str)
            .collect()
    }

    /// Number of artifacts whose drift requires visible review.
    pub fn unresolved_artifact_count(&self) -> usize {
        self.all_artifacts()
            .filter(|artifact| artifact.requires_visible_reason())
            .count()
    }

    fn present_artifact_kinds(&self) -> BTreeSet<NavigationContinuityArtifactKind> {
        self.all_artifacts()
            .map(|artifact| artifact.artifact_kind)
            .collect()
    }

    fn present_drift_states(&self) -> BTreeSet<NavigationDriftState> {
        self.all_artifacts()
            .map(|artifact| artifact.drift_state)
            .collect()
    }

    fn present_history_roles(&self) -> BTreeSet<HistoryRole> {
        self.all_artifacts()
            .filter_map(|artifact| artifact.history_role)
            .collect()
    }

    /// True when the packet remains metadata-safe by construction.
    pub fn is_export_safe(&self) -> bool {
        self.export_safe_summary.contains("metadata-safe")
            && self.all_artifacts().all(|artifact| {
                artifact.raw_boundary_material_excluded && artifact.authority_not_widened
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
    ) -> NavigationContinuityBindingSupportExport {
        NavigationContinuityBindingSupportExport {
            record_kind: NAVIGATION_CONTINUITY_BINDING_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: NAVIGATION_CONTINUITY_BINDING_SCHEMA_VERSION,
            export_id: export_id.into(),
            navigation_continuity_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            navigation_continuity_packet: self.clone(),
        }
    }

    /// Validates the matrix against the lane guardrails. An empty result means
    /// the packet is fully covered, attributable, and metadata-safe.
    pub fn validate(&self) -> Vec<NavigationContinuityBindingValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != NAVIGATION_CONTINUITY_BINDING_PACKET_RECORD_KIND {
            push(&mut findings, "record_kind", "unexpected record_kind");
        }
        if self.schema_version != NAVIGATION_CONTINUITY_BINDING_SCHEMA_VERSION {
            push(&mut findings, "schema_version", "unexpected schema_version");
        }
        if self.packet_id != NAVIGATION_CONTINUITY_BINDING_PACKET_ID {
            push(&mut findings, "packet_id", "unexpected packet_id");
        }
        if self.doc_ref != NAVIGATION_CONTINUITY_BINDING_DOC_REF {
            push(
                &mut findings,
                "doc_ref",
                "packet must quote the reviewer doc",
            );
        }
        if self.schema_ref != NAVIGATION_CONTINUITY_BINDING_SCHEMA_REF {
            push(
                &mut findings,
                "schema_ref",
                "packet must quote the schema ref",
            );
        }
        if self.artifact_ref != NAVIGATION_CONTINUITY_BINDING_ARTIFACT_REF {
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
        self.validate_surfaces(&mut findings);
        self.validate_restore(&mut findings);
        self.validate_consumers(&mut findings);

        findings
    }

    fn validate_coverage(&self, findings: &mut Vec<NavigationContinuityBindingValidationFinding>) {
        for required in REQUIRED_CONTINUITY_SURFACES {
            if !self.covered_surfaces.contains(&required) {
                push(
                    findings,
                    "covered_surfaces",
                    &format!("missing covered surface {}", required.as_str()),
                );
            }
        }
        let present_kinds = self.present_artifact_kinds();
        for required in COVERED_ARTIFACT_KINDS {
            if !self.covered_artifact_kinds.contains(&required) {
                push(
                    findings,
                    "covered_artifact_kinds",
                    &format!("missing covered artifact kind {}", required.as_str()),
                );
            }
            if !present_kinds.contains(&required) {
                push(
                    findings,
                    "covered_artifact_kinds",
                    &format!("no artifact realizes the kind {}", required.as_str()),
                );
            }
        }
        let present_drift = self.present_drift_states();
        for required in REQUIRED_DRIFT_STATES {
            if !self.covered_drift_states.contains(&required) {
                push(
                    findings,
                    "covered_drift_states",
                    &format!("missing covered drift state {}", required.as_str()),
                );
            }
            if !present_drift.contains(&required) {
                push(
                    findings,
                    "covered_drift_states",
                    &format!("no artifact realizes the drift state {}", required.as_str()),
                );
            }
        }
        // Acceptance: back, forward, and recent navigation are all real,
        // attributable states, never collapsed into a single "recent" bucket.
        let present_roles = self.present_history_roles();
        for required in HistoryRole::ALL {
            if !present_roles.contains(&required) {
                push(
                    findings,
                    "surfaces",
                    &format!("no history entry realizes the {} role", required.as_str()),
                );
            }
        }
    }

    fn validate_surfaces(&self, findings: &mut Vec<NavigationContinuityBindingValidationFinding>) {
        for required in REQUIRED_CONTINUITY_SURFACES {
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

        let mut seen_artifact_ids = BTreeSet::new();
        for surface in &self.surfaces {
            let base = format!("surfaces.{}", surface.surface.as_str());
            if surface.artifacts.is_empty() {
                push(
                    findings,
                    &format!("{base}.artifacts"),
                    "surface must carry at least one continuity artifact",
                );
            }
            for artifact in &surface.artifacts {
                self.validate_artifact(
                    findings,
                    &base,
                    surface.surface,
                    artifact,
                    &mut seen_artifact_ids,
                );
            }
        }
    }

    fn validate_artifact(
        &self,
        findings: &mut Vec<NavigationContinuityBindingValidationFinding>,
        base: &str,
        surface: NavigationContinuitySurface,
        artifact: &ContinuityArtifactBinding,
        seen_artifact_ids: &mut BTreeSet<String>,
    ) {
        let id = artifact.artifact_id.trim();
        if id.is_empty() {
            push(
                findings,
                &format!("{base}.artifacts"),
                "continuity artifact is missing a stable id",
            );
            return;
        }
        let artifact_base = format!("{base}.artifacts.{id}");
        if !seen_artifact_ids.insert(id.to_owned()) {
            push(
                findings,
                &format!("{artifact_base}.artifact_id"),
                "artifact id must be unique across the packet",
            );
        }
        if artifact.surface != surface {
            push(
                findings,
                &format!("{artifact_base}.surface"),
                "artifact surface must match the enclosing surface binding",
            );
        }
        if artifact.canonical_target_ref.trim().is_empty() {
            push(
                findings,
                &format!("{artifact_base}.canonical_target_ref"),
                "artifact must keep a non-empty canonical target ref",
            );
        }

        // Result identity, when present, is a durable URN, never a display label
        // or a transient list index. Result-bearing surfaces must carry one.
        let result_bearing = matches!(
            surface,
            NavigationContinuitySurface::Search
                | NavigationContinuitySurface::Docs
                | NavigationContinuitySurface::Topology
        );
        match &artifact.result_id_ref {
            Some(result_id) => {
                let value = result_id.trim();
                if value.is_empty() || value.parse::<u64>().is_ok() || !value.contains(':') {
                    push(
                        findings,
                        &format!("{artifact_base}.result_id_ref"),
                        "result identity must be a durable URN, not a label or list index",
                    );
                }
                if let Some(label) = &artifact.label {
                    if value.eq_ignore_ascii_case(label.trim()) {
                        push(
                            findings,
                            &format!("{artifact_base}.result_id_ref"),
                            "result identity must not collapse into the display label",
                        );
                    }
                }
            }
            None if result_bearing => {
                push(
                    findings,
                    &format!("{artifact_base}.result_id_ref"),
                    "search, docs, and graph artifacts must bind to a durable result identity",
                );
            }
            None => {}
        }

        // Drift honesty: a bound or remapped anchor resolves; an anchor whose
        // drift requires review must not carry a resolved target and must keep a
        // visible reason and recovery choices.
        let resolved_present = artifact
            .resolved_target_ref
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty());
        match artifact.drift_state {
            NavigationDriftState::Bound | NavigationDriftState::Remapped => {
                if !resolved_present {
                    push(
                        findings,
                        &format!("{artifact_base}.resolved_target_ref"),
                        "bound and remapped artifacts must carry a resolved target ref",
                    );
                }
            }
            state if state.requires_visible_reason() => {
                if resolved_present {
                    push(
                        findings,
                        &format!("{artifact_base}.resolved_target_ref"),
                        "an unresolved or archived artifact must not carry a resolved target ref",
                    );
                }
                if artifact.drift_reason.as_deref().map_or(true, str::is_empty) {
                    push(
                        findings,
                        &format!("{artifact_base}.drift_reason"),
                        "drifted, missing, unavailable, or archived artifacts must keep a visible reason",
                    );
                }
                if artifact.recovery_choices.is_empty() {
                    push(
                        findings,
                        &format!("{artifact_base}.recovery_choices"),
                        "drifted, missing, unavailable, or archived artifacts must keep recovery choices",
                    );
                }
            }
            _ => {}
        }

        // Bind first, remap second: only a remapped anchor carries remap
        // evidence, and a remap may never use a nearest-target fallback.
        if artifact.used_nearby_fallback {
            push(
                findings,
                &format!("{artifact_base}.used_nearby_fallback"),
                "continuity must never relocate to a nearby target via a fallback",
            );
        }
        if artifact.drift_state == NavigationDriftState::Remapped {
            if artifact.remap_evidence_refs.is_empty() {
                push(
                    findings,
                    &format!("{artifact_base}.remap_evidence_refs"),
                    "remapped artifacts must cite stable remap evidence",
                );
            }
            if artifact
                .resolved_target_ref
                .as_deref()
                .is_some_and(|resolved| resolved == artifact.canonical_target_ref)
            {
                push(
                    findings,
                    &format!("{artifact_base}.resolved_target_ref"),
                    "a remapped artifact must resolve to a target different from the canonical ref",
                );
            }
        } else if !artifact.remap_evidence_refs.is_empty() {
            push(
                findings,
                &format!("{artifact_base}.remap_evidence_refs"),
                "only remapped artifacts may carry remap evidence",
            );
        }

        // History and peek keep an attributable origin/return anchor distinct
        // from the destination; other kinds carry none.
        let needs_origin = artifact.is_history_entry() || artifact.is_peek_context();
        match &artifact.origin_target_ref {
            Some(origin) if needs_origin => {
                if origin.trim().is_empty() {
                    push(
                        findings,
                        &format!("{artifact_base}.origin_target_ref"),
                        "history and peek artifacts must keep a non-empty origin/return anchor",
                    );
                } else if *origin == artifact.canonical_target_ref {
                    push(
                        findings,
                        &format!("{artifact_base}.origin_target_ref"),
                        "the origin/return anchor must differ from the destination anchor",
                    );
                }
            }
            Some(_) => {
                push(
                    findings,
                    &format!("{artifact_base}.origin_target_ref"),
                    "only history and peek artifacts may carry an origin/return anchor",
                );
            }
            None if needs_origin => {
                push(
                    findings,
                    &format!("{artifact_base}.origin_target_ref"),
                    "history and peek artifacts must keep an origin/return anchor",
                );
            }
            None => {}
        }

        // A history entry plays exactly one back/forward/recent role; other
        // kinds carry none.
        match (artifact.is_history_entry(), artifact.history_role) {
            (true, None) => push(
                findings,
                &format!("{artifact_base}.history_role"),
                "history entries must declare a back, forward, or recent role",
            ),
            (false, Some(_)) => push(
                findings,
                &format!("{artifact_base}.history_role"),
                "only history entries may declare a history role",
            ),
            _ => {}
        }

        if !artifact.authority_not_widened {
            push(
                findings,
                &format!("{artifact_base}.authority_not_widened"),
                "surfacing continuity must not widen authority",
            );
        }
        if !artifact.raw_boundary_material_excluded {
            push(
                findings,
                &format!("{artifact_base}.raw_boundary_material_excluded"),
                "artifact must exclude raw query text, bodies, secrets, and weights",
            );
        }
        if artifact.summary.trim().is_empty() {
            push(
                findings,
                &format!("{artifact_base}.summary"),
                "artifact must keep a review-safe summary",
            );
        }
    }

    fn validate_restore(&self, findings: &mut Vec<NavigationContinuityBindingValidationFinding>) {
        if self.restore.restore_packet_id.trim().is_empty() {
            push(
                findings,
                "restore.restore_packet_id",
                "restore projection must keep a stable id",
            );
        }
        if self.restore.restore_source_ref.trim().is_empty() {
            push(
                findings,
                "restore.restore_source_ref",
                "restore projection must cite a source snapshot ref",
            );
        }
        let artifact_ids: BTreeSet<&str> = self
            .all_artifacts()
            .map(|artifact| artifact.artifact_id.as_str())
            .collect();
        for restored in &self.restore.artifacts {
            let base = format!("restore.artifacts.{}", restored.artifact_id_ref);
            if !artifact_ids.contains(restored.artifact_id_ref.as_str()) {
                push(
                    findings,
                    &base,
                    "restore must point at a continuity artifact in the packet",
                );
            }
            if !restored.target_resolves_under_current_scope {
                // Guardrail: restore preserves drifted artifacts with visible
                // reasons instead of dropping them.
                if !restored.artifact_preserved {
                    push(
                        findings,
                        &format!("{base}.artifact_preserved"),
                        "restore must preserve a non-resolving artifact instead of dropping it",
                    );
                }
                if restored
                    .restore_reason
                    .as_deref()
                    .map_or(true, str::is_empty)
                {
                    push(
                        findings,
                        &format!("{base}.restore_reason"),
                        "restore must surface a reason when exact resolution fails",
                    );
                }
                if restored.recovery_choices.is_empty() {
                    push(
                        findings,
                        &format!("{base}.recovery_choices"),
                        "restore must surface recovery choices when exact resolution fails",
                    );
                }
            }
        }
        // Acceptance: restore reopening a drifted/missing artifact with a visible
        // reason is a real, exercised state.
        if self.restore.preserved_unresolved_count() == 0 {
            push(
                findings,
                "restore.artifacts",
                "restore must preserve at least one non-resolving artifact with a visible reason",
            );
        }
    }

    fn validate_consumers(&self, findings: &mut Vec<NavigationContinuityBindingValidationFinding>) {
        for required in ContinuityConsumerClass::ALL {
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
            // Acceptance: every consumer reuses the same continuity objects,
            // preserves the drift vocabulary and reasons, and keeps origin and
            // destination refs attributable without widening authority.
            if !projection.reuses_continuity() {
                push(
                    findings,
                    &base,
                    "consumer must reuse the continuity objects, drift vocabulary, reasons, and origin/destination refs without widening authority",
                );
            }
        }
    }
}

/// Support-export wrapper that preserves the product continuity packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationContinuityBindingSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Continuity packet id preserved by the export.
    pub navigation_continuity_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials / authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub navigation_continuity_packet: NavigationContinuityBindingPacket,
}

impl NavigationContinuityBindingSupportExport {
    /// True when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == NAVIGATION_CONTINUITY_BINDING_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == NAVIGATION_CONTINUITY_BINDING_SCHEMA_VERSION
            && self.navigation_continuity_packet_id_ref
                == self.navigation_continuity_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.navigation_continuity_packet.validate().is_empty()
            && self.navigation_continuity_packet.is_export_safe()
    }
}

/// Errors returned when reading a checked-in continuity packet.
#[derive(Debug)]
pub enum NavigationContinuityBindingArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<NavigationContinuityBindingValidationFinding>),
}

impl fmt::Display for NavigationContinuityBindingArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    formatter,
                    "navigation continuity packet parse failed: {error}"
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
                    "navigation continuity packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for NavigationContinuityBindingArtifactError {}

/// Returns the checked-in canonical navigation-continuity packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_navigation_continuity_packet(
) -> Result<NavigationContinuityBindingPacket, NavigationContinuityBindingArtifactError> {
    parse_checked_in(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/search/m5/navigation-continuity/packet.json"
    )))
}

/// Returns the checked-in workset-drift navigation-continuity packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_workset_drift_navigation_continuity_packet(
) -> Result<NavigationContinuityBindingPacket, NavigationContinuityBindingArtifactError> {
    parse_checked_in(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/search/m5/navigation-continuity/workset_drift.json"
    )))
}

fn parse_checked_in(
    payload: &str,
) -> Result<NavigationContinuityBindingPacket, NavigationContinuityBindingArtifactError> {
    let packet: NavigationContinuityBindingPacket =
        serde_json::from_str(payload).map_err(NavigationContinuityBindingArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(NavigationContinuityBindingArtifactError::Validation(
            findings,
        ))
    }
}

/// Variant of the seeded continuity corpus.
#[derive(Debug, Clone, Copy)]
enum ContinuityVariant {
    Canonical,
    WorksetDrift,
}

/// Returns the canonical seeded navigation-continuity packet.
pub fn seeded_navigation_continuity_packet() -> NavigationContinuityBindingPacket {
    build_packet(ContinuityVariant::Canonical)
}

/// Returns a seeded packet where the active workset narrows further, so a search
/// bookmark that was previously bound now drifts visibly (strictly more
/// unresolved artifacts) while the surface, artifact-kind, and drift-state
/// vocabulary, the artifact identities, and the reused continuity objects are
/// preserved unchanged. The drifted bookmark survives restore with a visible
/// reason instead of being dropped or relocated.
pub fn seeded_workset_drift_navigation_continuity_packet() -> NavigationContinuityBindingPacket {
    build_packet(ContinuityVariant::WorksetDrift)
}

fn build_packet(variant: ContinuityVariant) -> NavigationContinuityBindingPacket {
    let surfaces = vec![
        editor_surface(),
        diff_surface(),
        notebook_surface(),
        docs_surface(),
        search_surface(variant),
        topology_surface(),
    ];

    NavigationContinuityBindingPacket {
        record_kind: NAVIGATION_CONTINUITY_BINDING_PACKET_RECORD_KIND.to_owned(),
        schema_version: NAVIGATION_CONTINUITY_BINDING_SCHEMA_VERSION,
        packet_id: NAVIGATION_CONTINUITY_BINDING_PACKET_ID.to_owned(),
        generated_at: SEEDED_GENERATED_AT.to_owned(),
        doc_ref: NAVIGATION_CONTINUITY_BINDING_DOC_REF.to_owned(),
        schema_ref: NAVIGATION_CONTINUITY_BINDING_SCHEMA_REF.to_owned(),
        artifact_ref: NAVIGATION_CONTINUITY_BINDING_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".t2/docs/Aureline_Milestones_Document.md".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md".to_owned(),
            ".t2/docs/Aureline_UX_Design_System_Style_Guide.md".to_owned(),
            ".t2/docs/Aureline_PRD.md".to_owned(),
        ],
        supporting_contract_refs: vec![
            "schemas/search/bookmark-history-and-drift-continuity.schema.json".to_owned(),
            "schemas/navigation/navigation_target.schema.json".to_owned(),
            "schemas/search/query-session-first-consumers.schema.json".to_owned(),
            SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF.to_owned(),
            NAVIGATION_CONTINUITY_BINDING_SCHEMA_REF.to_owned(),
        ],
        covered_surfaces: REQUIRED_CONTINUITY_SURFACES.to_vec(),
        covered_artifact_kinds: COVERED_ARTIFACT_KINDS.to_vec(),
        covered_drift_states: REQUIRED_DRIFT_STATES.to_vec(),
        surfaces,
        restore: restore_projection(variant),
        consumer_projections: seeded_consumer_projections(),
        export_safe_summary:
            "This metadata-safe navigation-continuity matrix binds breadcrumb, outline, bookmark, recent-location, back/forward, and peek artifacts to canonical anchors across the editor, diff, notebook, docs, search, and graph surfaces. Bookmarks bind to canonical anchors first and remap only with stable evidence; drift, missing-target, scope-unavailable, and archived states stay visible with reasons and recovery choices instead of relocating to a nearby symbol, line, or document. Restore reopens continuity artifacts with their drift reasons preserved, and the same continuity objects are reused by the product UI, back/forward history, session restore, and support replay consumers. Only refs, drift states, reasons, and origin/destination anchors leave the boundary; no raw query text or bodies are admitted, sessions are referenced hash-only, and convenience routing never widens authority."
                .to_owned(),
    }
}

// ----- seeded surface corpus ------------------------------------------------

fn result_id(kind: StableResultKind, canonical_ref: &str) -> String {
    build_canonical_result_id(SEEDED_WORKSPACE_ID, kind, canonical_ref)
}

fn session_ref(tag: &str) -> String {
    format!(
        "{NAVIGATION_CONTINUITY_BINDING_PACKET_ID}:session:{}",
        stable_query_hash(tag)
    )
}

/// Builds a continuity artifact with safe defaults; callers tweak the fields
/// that distinguish each kind and drift state.
fn base_artifact(
    surface: NavigationContinuitySurface,
    tag: &str,
    artifact_kind: NavigationContinuityArtifactKind,
    drift_state: NavigationDriftState,
    canonical_target_ref: impl Into<String>,
    summary: &str,
) -> ContinuityArtifactBinding {
    ContinuityArtifactBinding {
        artifact_id: format!(
            "{NAVIGATION_CONTINUITY_BINDING_PACKET_ID}:{}:{tag}",
            surface.as_str()
        ),
        artifact_kind,
        surface,
        label: None,
        canonical_target_ref: canonical_target_ref.into(),
        resolved_target_ref: None,
        result_id_ref: None,
        query_session_id_ref: None,
        origin_target_ref: None,
        history_role: None,
        drift_state,
        drift_reason: None,
        remap_evidence_refs: Vec::new(),
        used_nearby_fallback: false,
        recovery_choices: Vec::new(),
        authority_not_widened: true,
        raw_boundary_material_excluded: true,
        summary: summary.to_owned(),
    }
}

fn editor_surface() -> SurfaceContinuityBinding {
    let mut breadcrumb = base_artifact(
        NavigationContinuitySurface::Editor,
        "breadcrumb",
        NavigationContinuityArtifactKind::BreadcrumbTrail,
        NavigationDriftState::Bound,
        "symbol:aureline_search::query_session::SearchQuerySession::parse",
        "Editor breadcrumb resolves the active symbol path exactly and stays bound to the canonical symbol anchor.",
    );
    breadcrumb.label = Some("SearchQuerySession › parse".to_owned());
    breadcrumb.resolved_target_ref =
        Some("symbol:aureline_search::query_session::SearchQuerySession::parse".to_owned());

    let mut bookmark = base_artifact(
        NavigationContinuitySurface::Editor,
        "bookmark",
        NavigationContinuityArtifactKind::NavigationMark,
        NavigationDriftState::Remapped,
        "symbol:aureline_search::query_session::parse_query",
        "Editor bookmark followed the renamed symbol through its stable symbol id, not a nearby line.",
    );
    bookmark.label = Some("Bookmark: parse_query".to_owned());
    bookmark.resolved_target_ref =
        Some("symbol:aureline_search::query_session::SearchQuerySession::parse".to_owned());
    bookmark.remap_evidence_refs =
        vec!["evidence:symbol-stable-id:aureline_search::query_session::sym_0042".to_owned()];

    SurfaceContinuityBinding {
        surface: NavigationContinuitySurface::Editor,
        surface_label: "Editor".to_owned(),
        artifacts: vec![breadcrumb, bookmark],
        summary: "Editor breadcrumb stays bound and the bookmark remaps to the renamed symbol through stable evidence.".to_owned(),
    }
}

fn diff_surface() -> SurfaceContinuityBinding {
    let mut outline = base_artifact(
        NavigationContinuitySurface::Diff,
        "outline",
        NavigationContinuityArtifactKind::OutlineSnapshot,
        NavigationDriftState::Bound,
        "diff:hunk:crates/aureline-search/src/query_session.rs@rev-9f2a",
        "Diff outline snapshot lists the changed hunks for the compared revision and stays bound.",
    );
    outline.resolved_target_ref =
        Some("diff:hunk:crates/aureline-search/src/query_session.rs@rev-9f2a".to_owned());

    let mut history_back = base_artifact(
        NavigationContinuitySurface::Diff,
        "history_back",
        NavigationContinuityArtifactKind::NavigationHistoryEntry,
        NavigationDriftState::Drifted,
        "diff:line:crates/aureline-search/src/query_session.rs:120@rev-9f2a",
        "Back-stack diff entry drifted after a rebase; it stays visible as drifted rather than jumping to a nearby line.",
    );
    history_back.history_role = Some(HistoryRole::Back);
    history_back.origin_target_ref =
        Some("diff:line:crates/aureline-search/src/lib.rs:44@rev-9f2a".to_owned());
    history_back.drift_reason = Some(
        "The compared revision was rebased; the recorded diff line no longer maps to a current hunk."
            .to_owned(),
    );
    history_back.recovery_choices = vec![
        "reopen_diff_against_current_revision".to_owned(),
        "inspect_drift".to_owned(),
    ];

    SurfaceContinuityBinding {
        surface: NavigationContinuitySurface::Diff,
        surface_label: "Diff".to_owned(),
        artifacts: vec![outline, history_back],
        summary: "Diff outline stays bound while a rebased back-stack entry stays visible as drifted with a recovery path.".to_owned(),
    }
}

fn notebook_surface() -> SurfaceContinuityBinding {
    let mut outline = base_artifact(
        NavigationContinuitySurface::Notebook,
        "outline",
        NavigationContinuityArtifactKind::OutlineSnapshot,
        NavigationDriftState::ScopeUnavailable,
        "notebook:outline:nb-analysis-01",
        "Notebook outline is outside the active trust scope and is disclosed as scope-unavailable, not hidden.",
    );
    outline.drift_reason = Some(
        "The notebook is outside the active trust scope; its outline is unavailable without elevating scope."
            .to_owned(),
    );
    outline.recovery_choices = vec![
        "elevate_trust_scope".to_owned(),
        "open_notebook_in_full_scope".to_owned(),
    ];

    let mut peek = base_artifact(
        NavigationContinuitySurface::Notebook,
        "peek",
        NavigationContinuityArtifactKind::PeekContext,
        NavigationDriftState::Bound,
        "notebook:cell:nb-analysis-01:cell-3:output",
        "Notebook peek shows a cell output inline and keeps the return anchor to the invoking cell.",
    );
    peek.resolved_target_ref = Some("notebook:cell:nb-analysis-01:cell-3:output".to_owned());
    peek.origin_target_ref = Some("notebook:cell:nb-analysis-01:cell-1".to_owned());

    SurfaceContinuityBinding {
        surface: NavigationContinuitySurface::Notebook,
        surface_label: "Notebook".to_owned(),
        artifacts: vec![outline, peek],
        summary: "Notebook outline is disclosed as scope-unavailable while a peek keeps an attributable return anchor.".to_owned(),
    }
}

fn docs_surface() -> SurfaceContinuityBinding {
    let mut breadcrumb = base_artifact(
        NavigationContinuitySurface::Docs,
        "breadcrumb",
        NavigationContinuityArtifactKind::BreadcrumbTrail,
        NavigationDriftState::Bound,
        "docs:anchor:docs/search/navigation-continuity.md#overview",
        "Docs card breadcrumb stays bound to the canonical docs anchor and durable result identity.",
    );
    breadcrumb.label = Some("Navigation continuity › Overview".to_owned());
    breadcrumb.resolved_target_ref =
        Some("docs:anchor:docs/search/navigation-continuity.md#overview".to_owned());
    breadcrumb.result_id_ref = Some(result_id(
        StableResultKind::DocsAnchor,
        "docs/search/navigation-continuity.md#overview",
    ));
    breadcrumb.query_session_id_ref = Some(session_ref("docs_breadcrumb"));

    let mut bookmark = base_artifact(
        NavigationContinuitySurface::Docs,
        "bookmark",
        NavigationContinuityArtifactKind::NavigationMark,
        NavigationDriftState::MissingTarget,
        "docs:anchor:docs/legacy/removed-page.md#section",
        "A bookmarked docs anchor is missing from the offline pack and stays visible as missing-target, not silently relinked.",
    );
    bookmark.label = Some("Bookmark: Removed docs page".to_owned());
    bookmark.result_id_ref = Some(result_id(
        StableResultKind::DocsAnchor,
        "docs/legacy/removed-page.md#section",
    ));
    bookmark.query_session_id_ref = Some(session_ref("docs_bookmark"));
    bookmark.drift_reason = Some(
        "The bookmarked docs anchor was removed from the offline docs pack for this scope."
            .to_owned(),
    );
    bookmark.recovery_choices = vec![
        "search_docs_for_replacement".to_owned(),
        "open_canonical_docs_in_browser".to_owned(),
    ];

    SurfaceContinuityBinding {
        surface: NavigationContinuitySurface::Docs,
        surface_label: "Docs".to_owned(),
        artifacts: vec![breadcrumb, bookmark],
        summary: "Docs breadcrumb stays bound while a removed-page bookmark stays visible as missing-target with recovery.".to_owned(),
    }
}

fn search_surface(variant: ContinuityVariant) -> SurfaceContinuityBinding {
    let recent_rid = result_id(
        StableResultKind::WorkspaceFile,
        "crates/aureline-search/src/lib.rs",
    );
    let mut recent = base_artifact(
        NavigationContinuitySurface::Search,
        "recent",
        NavigationContinuityArtifactKind::NavigationHistoryEntry,
        NavigationDriftState::Bound,
        format!("nav:dest:{recent_rid}"),
        "A recent-location entry from a search result stays bound to its durable result identity and origin.",
    );
    recent.history_role = Some(HistoryRole::Recent);
    recent.resolved_target_ref = Some(format!("nav:dest:{recent_rid}"));
    recent.result_id_ref = Some(recent_rid);
    recent.query_session_id_ref = Some(session_ref("search_recent"));
    recent.origin_target_ref = Some("nav:origin:search-results-list".to_owned());

    let forward_rid = result_id(
        StableResultKind::Symbol,
        "aureline_search::planner::SearchPlannerAlpha",
    );
    let mut forward = base_artifact(
        NavigationContinuitySurface::Search,
        "forward",
        NavigationContinuityArtifactKind::NavigationHistoryEntry,
        NavigationDriftState::Bound,
        format!("nav:dest:{forward_rid}"),
        "A forward-stack entry from a symbol-search result stays bound with its origin preserved.",
    );
    forward.history_role = Some(HistoryRole::Forward);
    forward.resolved_target_ref = Some(format!("nav:dest:{forward_rid}"));
    forward.result_id_ref = Some(forward_rid);
    forward.query_session_id_ref = Some(session_ref("search_forward"));
    forward.origin_target_ref = Some("nav:origin:symbol-search-results".to_owned());

    let bookmark_rid = result_id(
        StableResultKind::Symbol,
        "aureline_search::result_id::build_canonical_result_id",
    );
    let mut bookmark = base_artifact(
        NavigationContinuitySurface::Search,
        "bookmark",
        NavigationContinuityArtifactKind::NavigationMark,
        NavigationDriftState::Bound,
        format!("nav:mark:{bookmark_rid}"),
        "A bookmarked search result stays bound to its durable, surface-independent result identity.",
    );
    bookmark.label = Some("Bookmark: build_canonical_result_id".to_owned());
    bookmark.resolved_target_ref = Some(format!("nav:mark:{bookmark_rid}"));
    bookmark.result_id_ref = Some(bookmark_rid);
    bookmark.query_session_id_ref = Some(session_ref("search_bookmark"));

    // Under a narrowed workset the bookmarked result leaves the active slice; it
    // stays visible as drifted with a recovery path instead of relocating to a
    // nearby in-scope result.
    if matches!(variant, ContinuityVariant::WorksetDrift) {
        bookmark.drift_state = NavigationDriftState::Drifted;
        bookmark.resolved_target_ref = None;
        bookmark.drift_reason = Some(
            "The bookmarked result is outside the narrowed active workset; it stays visible as drifted rather than relocating to a nearby in-scope result."
                .to_owned(),
        );
        bookmark.recovery_choices = vec![
            "widen_workset_scope".to_owned(),
            "open_canonical_result_in_full_index".to_owned(),
        ];
        bookmark.summary =
            "Under a narrowed workset the bookmarked result drifts out of scope and stays visible as drifted with a recovery path."
                .to_owned();
    }

    SurfaceContinuityBinding {
        surface: NavigationContinuitySurface::Search,
        surface_label: "Search".to_owned(),
        artifacts: vec![recent, forward, bookmark],
        summary: "Recent, forward, and bookmark search artifacts bind to durable result identities and preserve origin anchors.".to_owned(),
    }
}

fn topology_surface() -> SurfaceContinuityBinding {
    let peek_rid = result_id(
        StableResultKind::Symbol,
        "aureline_graph::topology::GraphNode",
    );
    let mut peek = base_artifact(
        NavigationContinuitySurface::Topology,
        "peek",
        NavigationContinuityArtifactKind::PeekContext,
        NavigationDriftState::Remapped,
        "graph:node:aureline_graph::topology::graph-node-old",
        "Graph peek followed the moved node through its stable node id and keeps the return anchor.",
    );
    peek.resolved_target_ref =
        Some("graph:node:aureline_graph::topology::graph-node-new".to_owned());
    peek.result_id_ref = Some(peek_rid);
    peek.query_session_id_ref = Some(session_ref("graph_peek"));
    peek.origin_target_ref = Some("graph:node:aureline_graph::topology::caller-node".to_owned());
    peek.remap_evidence_refs =
        vec!["evidence:graph-node-stable-id:aureline_graph::node_0117".to_owned()];

    let mark_rid = result_id(
        StableResultKind::Symbol,
        "aureline_legacy::removed_subsystem",
    );
    let mut mark = base_artifact(
        NavigationContinuitySurface::Topology,
        "mark",
        NavigationContinuityArtifactKind::NavigationMark,
        NavigationDriftState::Archived,
        "graph:node:aureline_legacy::removed_subsystem",
        "An archived graph bookmark is retained as a tombstone instead of relinking to a nearby node.",
    );
    mark.label = Some("Bookmark: archived subsystem".to_owned());
    mark.result_id_ref = Some(mark_rid);
    mark.query_session_id_ref = Some(session_ref("graph_mark"));
    mark.drift_reason = Some(
        "The graph node was archived when the subsystem was removed; the bookmark is retained as a tombstone."
            .to_owned(),
    );
    mark.recovery_choices = vec![
        "inspect_archived_node_metadata".to_owned(),
        "remove_archived_bookmark".to_owned(),
    ];

    SurfaceContinuityBinding {
        surface: NavigationContinuitySurface::Topology,
        surface_label: "Graph".to_owned(),
        artifacts: vec![peek, mark],
        summary: "Graph peek remaps through stable evidence while an archived bookmark is kept as a visible tombstone.".to_owned(),
    }
}

fn restore_projection(variant: ContinuityVariant) -> NavigationContinuityRestoreProjection {
    let editor_bookmark = format!("{NAVIGATION_CONTINUITY_BINDING_PACKET_ID}:editor:bookmark");
    let diff_history = format!("{NAVIGATION_CONTINUITY_BINDING_PACKET_ID}:diff:history_back");
    let notebook_outline = format!("{NAVIGATION_CONTINUITY_BINDING_PACKET_ID}:notebook:outline");
    let docs_bookmark = format!("{NAVIGATION_CONTINUITY_BINDING_PACKET_ID}:docs:bookmark");
    let search_recent = format!("{NAVIGATION_CONTINUITY_BINDING_PACKET_ID}:search:recent");
    let search_bookmark = format!("{NAVIGATION_CONTINUITY_BINDING_PACKET_ID}:search:bookmark");

    let mut artifacts = vec![
        RestoredContinuityArtifact {
            artifact_id_ref: editor_bookmark,
            drift_state: NavigationDriftState::Remapped,
            target_resolves_under_current_scope: true,
            artifact_preserved: true,
            restore_reason: None,
            recovery_choices: Vec::new(),
        },
        RestoredContinuityArtifact {
            artifact_id_ref: search_recent,
            drift_state: NavigationDriftState::Bound,
            target_resolves_under_current_scope: true,
            artifact_preserved: true,
            restore_reason: None,
            recovery_choices: Vec::new(),
        },
        RestoredContinuityArtifact {
            artifact_id_ref: diff_history,
            drift_state: NavigationDriftState::Drifted,
            target_resolves_under_current_scope: false,
            artifact_preserved: true,
            restore_reason: Some(
                "The diff revision is still rebased after restart; the entry is preserved as drifted."
                    .to_owned(),
            ),
            recovery_choices: vec!["reopen_diff_against_current_revision".to_owned()],
        },
        RestoredContinuityArtifact {
            artifact_id_ref: notebook_outline,
            drift_state: NavigationDriftState::ScopeUnavailable,
            target_resolves_under_current_scope: false,
            artifact_preserved: true,
            restore_reason: Some(
                "Trust scope still excludes the notebook after restart; the outline is preserved as scope-unavailable."
                    .to_owned(),
            ),
            recovery_choices: vec!["elevate_trust_scope".to_owned()],
        },
        RestoredContinuityArtifact {
            artifact_id_ref: docs_bookmark,
            drift_state: NavigationDriftState::MissingTarget,
            target_resolves_under_current_scope: false,
            artifact_preserved: true,
            restore_reason: Some(
                "The bookmarked docs anchor is still missing after restart; it is preserved as a missing-target bookmark."
                    .to_owned(),
            ),
            recovery_choices: vec!["search_docs_for_replacement".to_owned()],
        },
    ];

    if matches!(variant, ContinuityVariant::WorksetDrift) {
        artifacts.push(RestoredContinuityArtifact {
            artifact_id_ref: search_bookmark,
            drift_state: NavigationDriftState::Drifted,
            target_resolves_under_current_scope: false,
            artifact_preserved: true,
            restore_reason: Some(
                "The narrowed workset still excludes the bookmarked result after restart; it is preserved as drifted, not relocated."
                    .to_owned(),
            ),
            recovery_choices: vec!["widen_workset_scope".to_owned()],
        });
    }

    NavigationContinuityRestoreProjection {
        restore_packet_id: format!("{NAVIGATION_CONTINUITY_BINDING_PACKET_ID}:restore"),
        restore_source_ref: "restore:snapshot:session-resume".to_owned(),
        artifacts,
    }
}

fn seeded_consumer_projections() -> Vec<ContinuityConsumerProjection> {
    let make = |consumer: ContinuityConsumerClass, consumer_ref: &str, summary: &str| {
        ContinuityConsumerProjection {
            consumer,
            consumer_ref: consumer_ref.to_owned(),
            ingested_packet_id: NAVIGATION_CONTINUITY_BINDING_PACKET_ID.to_owned(),
            preserves_export_safe_ids: true,
            preserves_drift_vocabulary: true,
            preserves_drift_reasons: true,
            preserves_origin_destination: true,
            reuses_same_continuity_objects: true,
            widens_authority: false,
            ambient_authority_excluded: true,
            summary: summary.to_owned(),
        }
    };

    vec![
        make(
            ContinuityConsumerClass::ProductUi,
            "crates/aureline-shell/src/navigation_continuity/mod.rs",
            "The shell renders breadcrumbs, outlines, bookmarks, history, and peek from these artifacts, so a drifted or missing anchor shows a visible, recoverable cue instead of silently jumping to a nearby target.",
        ),
        make(
            ContinuityConsumerClass::HistoryBackForward,
            "crates/aureline-navigation/src/bookmark_history_and_drift_continuity/mod.rs",
            "Back/forward and recent-navigation replay re-open targets from the same continuity anchors and origin/destination refs, so history never re-mints a near-miss target from rendered chrome text.",
        ),
        make(
            ContinuityConsumerClass::SessionRestore,
            "crates/aureline-shell/src/navigation_continuity/mod.rs",
            "Session restore reopens the same continuity artifacts with their drift and missing-target reasons preserved, so a restart never drops a bookmark or history entry or relocates it silently.",
        ),
        make(
            ContinuityConsumerClass::SupportReplay,
            NAVIGATION_CONTINUITY_BINDING_ARTIFACT_REF,
            "Support replay wraps the same metadata-only continuity artifacts, drift states, reasons, and origin/destination anchors so a reported navigation can be inspected off the bundle without guessing from UI text.",
        ),
    ]
}

fn push(
    findings: &mut Vec<NavigationContinuityBindingValidationFinding>,
    path: &str,
    message: &str,
) {
    findings.push(NavigationContinuityBindingValidationFinding {
        path: path.to_owned(),
        message: message.to_owned(),
    });
}

#[cfg(test)]
mod tests;
