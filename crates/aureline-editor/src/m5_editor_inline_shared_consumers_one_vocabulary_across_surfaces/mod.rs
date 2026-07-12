//! Shared editor / notebook / review / AI / support / export consumers that keep the
//! B133 inline components — editor tabs, gutter markers, diagnostic decorations,
//! code-action chips, diff views, review threads, AI message cards, and evidence
//! timelines — at **one vocabulary** across every claimed M5 code surface.
//!
//! This module is the closing consumer-adoption lane for the eight reusable inline
//! components frozen in [`crate::m5_editor_inline_component_matrix`] and implemented by
//! the editor-tab / gutter lane
//! ([`crate::m5_editor_tab_and_gutter_state_and_marker_layering`]), the
//! diagnostic-decoration / code-action-chip lane
//! ([`crate::m5_diagnostic_decoration_and_code_action_chip_state_and_fix_posture`]),
//! the diff-view / review-thread lane
//! ([`crate::m5_diff_view_and_review_thread_anchor_durability_and_review_state`]), and
//! the AI-message-card / evidence-timeline lane
//! ([`crate::m5_ai_message_card_and_evidence_timeline_source_confidence_and_evidence_lineage`]).
//!
//! It binds each shared component to the concrete editor, diff, review, notebook, AI,
//! diagnostics, CLI/export, support, and product consumers that render it, and proves
//! — by fixtures, not screenshots — that the same inline object presents the same
//! state, anchor/freshness, confidence/source, approval, and evidence-lineage
//! vocabulary wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the eight shared components must be adopted by at least two
//!    distinct consumers, so a component is proven to be shared product infrastructure
//!    rather than a one-surface renderer detail.
//! 2. **One vocabulary / no drift.** For a given inline object every consumer surface
//!    must present identical [`EditorInlineStateFacetValues`] — the same state word,
//!    the same severity/confidence word, the same anchor/freshness word, the same
//!    approval word, and the same evidence-lineage word. The state word must be a token
//!    from the frozen [`M5EditorInlineDisposition`] vocabulary, so no feature rewrites
//!    "modified", "outdated", "resolved", or "inferred_fix" in its own words. A surface
//!    may narrow *how much* it shows across desktop, compact, remote, and exported
//!    representations, but it may never reword the underlying vocabulary per surface.
//! 3. **Map back to one family.** Support, CLI/export consumers must point at the
//!    canonical per-component schema and the frozen matrix by id, so an exported packet
//!    can always map editor / review / AI inline state back to one shared contract
//!    family.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation
//! carries an explicit [`EditorInlineNarrowNote`] naming the reason, the preserved
//! vocabulary, and the next action, and an exported representation additionally names
//! its export-safe evidence boundary rather than collapsing the object out of view.
//!
//! The packet references upstream component contracts by id rather than embedding their
//! content. Raw provider payloads, credentials, and un-redacted evidence stay outside
//! the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-editor-inline-shared-consumers.schema.json`](../../../../schemas/ui/m5-editor-inline-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/editor/m5_editor_inline_shared_consumers_one_vocabulary.md`](../../../../docs/editor/m5_editor_inline_shared_consumers_one_vocabulary.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-editor-inline-shared-consumers/`](../../../../fixtures/ui/m5-editor-inline-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_editor_inline_shared_consumers,
    seeded_m5_editor_inline_shared_consumers_compact_remote_narrowed,
    seeded_m5_editor_inline_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_editor_inline_component_matrix::{
    M5EditorInlineComponentFamily, M5EditorInlineConsumerSurface, M5EditorInlineDisposition,
    M5_EDITOR_INLINE_COMPONENT_DOC_REF, M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5EditorInlineSharedConsumersPacket`].
pub const M5_EDITOR_INLINE_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_editor_inline_shared_consumer_vocabulary_parity";

/// Schema version for editor-inline shared-consumer parity records.
pub const M5_EDITOR_INLINE_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_EDITOR_INLINE_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-editor-inline-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_EDITOR_INLINE_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/ui/m5-editor-inline-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EDITOR_INLINE_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/editor/m5_editor_inline_shared_consumers_one_vocabulary.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EDITOR_INLINE_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-editor-inline-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_EDITOR_INLINE_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-editor-inline-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_EDITOR_INLINE_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-editor-inline-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_EDITOR_INLINE_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-editor-inline-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_EDITOR_INLINE_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Whether a consumer surface is an export / support path that must map an inline
/// component back to its canonical contract family by id.
pub const fn consumer_must_reference_canonical(consumer: M5EditorInlineConsumerSurface) -> bool {
    matches!(
        consumer,
        M5EditorInlineConsumerSurface::SupportExport | M5EditorInlineConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5EditorInlineDisposition`] vocabulary.
///
/// This is the "one vocabulary" gate: an inline object's state word must be a controlled
/// disposition token rather than a per-surface synonym.
pub fn is_known_disposition_token(token: &str) -> bool {
    M5EditorInlineDisposition::ALL
        .iter()
        .any(|disposition| disposition.as_str() == token)
}

/// How much of a shared component a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying vocabulary: a narrowed
/// representation still carries the same state, anchor, confidence, approval, and
/// evidence-lineage words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorInlineRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl EditorInlineRepresentation {
    /// Every representation, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DesktopFull,
        Self::CompactNarrowed,
        Self::RemoteProjected,
        Self::ExportedRedacted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompactNarrowed => "compact_narrowed",
            Self::RemoteProjected => "remote_projected",
            Self::ExportedRedacted => "exported_redacted",
        }
    }

    /// Whether this representation narrows below full desktop disclosure.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }
}

/// A vocabulary axis whose word must stay identical across surfaces for one object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorInlineParityFacet {
    /// The inline state / disposition word (a frozen disposition token).
    StateWord,
    /// The marker severity or AI confidence word.
    SeverityOrConfidenceWord,
    /// The anchor durability / freshness word.
    AnchorFreshnessWord,
    /// The approval / review-required word.
    ApprovalStateWord,
    /// The evidence-lineage word.
    EvidenceLineageWord,
}

impl EditorInlineParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::StateWord,
        Self::SeverityOrConfidenceWord,
        Self::AnchorFreshnessWord,
        Self::ApprovalStateWord,
        Self::EvidenceLineageWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateWord => "state_word",
            Self::SeverityOrConfidenceWord => "severity_or_confidence_word",
            Self::AnchorFreshnessWord => "anchor_freshness_word",
            Self::ApprovalStateWord => "approval_state_word",
            Self::EvidenceLineageWord => "evidence_lineage_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorInlineNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl EditorInlineNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactionNarrowed => "compaction_narrowed",
            Self::RemoteProjectionNarrowed => "remote_projection_narrowed",
            Self::ExportRedactionNarrowed => "export_redaction_narrowed",
        }
    }
}

/// The next action a narrow note offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorInlineNarrowNextAction {
    /// Expand the component in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full evidence behind the redacted export.
    OpenFullEvidence,
}

impl EditorInlineNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandInDesktop => "expand_in_desktop",
            Self::OpenRemoteSource => "open_remote_source",
            Self::OpenFullEvidence => "open_full_evidence",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorInlineParityState {
    /// All vocabulary is preserved and shown in full.
    FacetsPreserved,
    /// All vocabulary is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl EditorInlineParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorInlineSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Inline vocabulary drifted between surfaces for the same object.
    VocabularyDriftDetected,
    /// A comment anchor or evidence pointer drifted silently.
    AnchorOrEvidenceDrift,
    /// Outdated and resolved review state were blurred.
    OutdatedResolvedBlurred,
    /// An inferred fix was shown as exact.
    InferredFixShownAsExact,
    /// An evidence timeline was hidden in an opaque log.
    EvidenceHiddenInOpaqueLog,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalReferenceMissing,
    /// An upstream shared component narrowed.
    UpstreamComponentNarrowed,
}

impl EditorInlineSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::VocabularyDriftDetected,
        Self::AnchorOrEvidenceDrift,
        Self::OutdatedResolvedBlurred,
        Self::InferredFixShownAsExact,
        Self::EvidenceHiddenInOpaqueLog,
        Self::CanonicalReferenceMissing,
        Self::UpstreamComponentNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::VocabularyDriftDetected => "vocabulary_drift_detected",
            Self::AnchorOrEvidenceDrift => "anchor_or_evidence_drift",
            Self::OutdatedResolvedBlurred => "outdated_resolved_blurred",
            Self::InferredFixShownAsExact => "inferred_fix_shown_as_exact",
            Self::EvidenceHiddenInOpaqueLog => "evidence_hidden_in_opaque_log",
            Self::CanonicalReferenceMissing => "canonical_reference_missing",
            Self::UpstreamComponentNarrowed => "upstream_component_narrowed",
        }
    }
}

/// The controlled vocabulary an inline object presents.
///
/// These five words must be identical across every consumer surface that shows the
/// same inline object. The state word must be a frozen disposition token; the rest are
/// controlled words the object's family carries. A surface may narrow how much it
/// renders, but it may never reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineStateFacetValues {
    /// Inline state / disposition word (must be a frozen disposition token).
    pub state_word: String,
    /// Marker severity or AI confidence word.
    pub severity_or_confidence_word: String,
    /// Anchor durability / freshness word.
    pub anchor_freshness_word: String,
    /// Approval / review-required word.
    pub approval_state_word: String,
    /// Evidence-lineage word.
    pub evidence_lineage_word: String,
}

impl EditorInlineStateFacetValues {
    /// Whether every vocabulary word is present.
    pub fn all_present(&self) -> bool {
        !self.state_word.trim().is_empty()
            && !self.severity_or_confidence_word.trim().is_empty()
            && !self.anchor_freshness_word.trim().is_empty()
            && !self.approval_state_word.trim().is_empty()
            && !self.evidence_lineage_word.trim().is_empty()
    }

    /// Whether the state word is a member of the frozen disposition vocabulary.
    pub fn state_word_in_vocabulary(&self) -> bool {
        is_known_disposition_token(self.state_word.trim())
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineNarrowNote {
    /// Why the representation narrowed.
    pub reason: EditorInlineNarrowReason,
    /// Note naming the preserved vocabulary (never omitted).
    pub preserved_vocabulary_note: String,
    /// The next action offered.
    pub next_action: EditorInlineNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditorInlineRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: EditorInlineParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<EditorInlineNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<EditorInlineNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit remote-source note.
    pub needs_remote_source_note: bool,
    /// Whether the binding must carry an explicit export-safe-evidence note.
    pub needs_export_evidence_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its representation.
///
/// The full desktop representation renders at full parity. A compact representation
/// narrows disclosure depth, a remote-projected representation names its remote source,
/// and an exported representation names its export-safe-evidence boundary — but all three
/// keep every vocabulary word and disclose the narrowing through an explicit note.
pub const fn resolve_editor_inline_render_disclosure(
    representation: EditorInlineRepresentation,
) -> EditorInlineRenderDisclosure {
    match representation {
        EditorInlineRepresentation::DesktopFull => EditorInlineRenderDisclosure {
            parity_state: EditorInlineParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_evidence_note: false,
        },
        EditorInlineRepresentation::CompactNarrowed => EditorInlineRenderDisclosure {
            parity_state: EditorInlineParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(EditorInlineNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(EditorInlineNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_evidence_note: false,
        },
        EditorInlineRepresentation::RemoteProjected => EditorInlineRenderDisclosure {
            parity_state: EditorInlineParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(EditorInlineNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(EditorInlineNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_evidence_note: false,
        },
        EditorInlineRepresentation::ExportedRedacted => EditorInlineRenderDisclosure {
            parity_state: EditorInlineParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(EditorInlineNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(EditorInlineNarrowNextAction::OpenFullEvidence),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_evidence_note: true,
        },
    }
}

/// One consumer binding: a shared component rendered on one consumer surface in one
/// representation for one inline object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable inline-object id (shared across surfaces that show the same object).
    pub inline_object_id: String,
    /// Human-readable inline-object identity.
    pub inline_object_label: String,
    /// Which shared component this binding renders.
    pub component: M5EditorInlineComponentFamily,
    /// Which consumer surface renders it.
    pub consumer: M5EditorInlineConsumerSurface,
    /// Which representation this surface renders.
    pub representation: EditorInlineRepresentation,
    /// The controlled vocabulary presented (identical across surfaces for one object).
    pub state_facets: EditorInlineStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: EditorInlineParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<EditorInlineNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-evidence note; required and non-empty when the disclosure demands it.
    pub export_evidence_note: String,
    /// Guardrail: this surface encodes inline state by color alone. MUST be `false`.
    pub encodes_state_by_color_alone: bool,
    /// Guardrail: this surface lets a comment anchor or evidence pointer silently drift.
    /// MUST be `false`.
    pub lets_anchor_or_evidence_pointer_silently_drift: bool,
    /// Guardrail: this surface blurs outdated and resolved review state. MUST be `false`.
    pub blurs_outdated_and_resolved_review_state: bool,
    /// Guardrail: this surface presents an inferred fix as exact. MUST be `false`.
    pub presents_inferred_fix_as_exact: bool,
    /// Guardrail: this surface hides an evidence timeline in an opaque log. MUST be `false`.
    pub hides_evidence_in_opaque_log: bool,
    /// Guardrail: this surface rewords the inline vocabulary per surface. MUST be `false`.
    pub rewords_inline_vocabulary_per_surface: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl EditorInlineConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> EditorInlineRenderDisclosure {
        resolve_editor_inline_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.encodes_state_by_color_alone
            && !self.lets_anchor_or_evidence_pointer_silently_drift
            && !self.blurs_outdated_and_resolved_review_state
            && !self.presents_inferred_fix_as_exact
            && !self.hides_evidence_in_opaque_log
            && !self.rewords_inline_vocabulary_per_surface
    }

    /// Whether this binding points at the canonical component schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = self.component.canonical_component_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineSharedConsumersTrustReview {
    /// Component reuse is proven by fixtures rather than inferred from screenshots.
    pub component_reuse_proven_by_fixtures: bool,
    /// The same inline object presents the same vocabulary across surfaces.
    pub same_object_same_vocabulary_across_surfaces: bool,
    /// Every state word is a frozen disposition token.
    pub state_words_stay_in_frozen_vocabulary: bool,
    /// Inline state is never encoded by color alone.
    pub state_never_encoded_by_color_alone: bool,
    /// Comment anchors and evidence pointers never silently drift.
    pub anchors_and_evidence_never_silently_drift: bool,
    /// Outdated and resolved review state stay distinct.
    pub outdated_and_resolved_stay_distinct: bool,
    /// Inferred fixes are never presented as exact.
    pub inferred_fix_never_shown_as_exact: bool,
    /// Evidence timelines keep inspectable structure, never an opaque log.
    pub evidence_keeps_inspectable_structure: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl EditorInlineSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.component_reuse_proven_by_fixtures
            && self.same_object_same_vocabulary_across_surfaces
            && self.state_words_stay_in_frozen_vocabulary
            && self.state_never_encoded_by_color_alone
            && self.anchors_and_evidence_never_silently_drift
            && self.outdated_and_resolved_stay_distinct
            && self.inferred_fix_never_shown_as_exact
            && self.evidence_keeps_inspectable_structure
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineSharedConsumersProjection {
    /// The editor UI reuses the shared components.
    pub editor_ui_reuses_shared_components: bool,
    /// The diff UI reuses the shared components.
    pub diff_ui_reuses_shared_components: bool,
    /// The review UI reuses the shared components.
    pub review_ui_reuses_shared_components: bool,
    /// The notebook UI reuses the shared components.
    pub notebook_ui_reuses_shared_components: bool,
    /// The AI UI reuses the shared components.
    pub ai_ui_reuses_shared_components: bool,
    /// The diagnostics UI reuses the shared components.
    pub diagnostics_ui_reuses_shared_components: bool,
    /// The support / export path reuses the shared components.
    pub support_export_reuses_shared_components: bool,
    /// Every component is adopted by two or more consumers.
    pub every_component_adopted_by_two_or_more_consumers: bool,
    /// Vocabulary is identical for the same inline object.
    pub vocabulary_identical_for_same_object: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps inline state back to one shared contract family.
    pub export_maps_back_to_one_contract_family: bool,
}

impl EditorInlineSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.editor_ui_reuses_shared_components
            && self.diff_ui_reuses_shared_components
            && self.review_ui_reuses_shared_components
            && self.notebook_ui_reuses_shared_components
            && self.ai_ui_reuses_shared_components
            && self.diagnostics_ui_reuses_shared_components
            && self.support_export_reuses_shared_components
            && self.every_component_adopted_by_two_or_more_consumers
            && self.vocabulary_identical_for_same_object
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_contract_family
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5EditorInlineSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EditorInlineSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<EditorInlineConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<EditorInlineSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5EditorInlineConsumerSurface>,
    /// Trust review block.
    pub trust_review: EditorInlineSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: EditorInlineSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: EditorInlineSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe editor-inline shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorInlineSharedConsumersPacket {
    /// Record kind; must equal [`M5_EDITOR_INLINE_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EDITOR_INLINE_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<EditorInlineConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<EditorInlineSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5EditorInlineConsumerSurface>,
    /// Trust review block.
    pub trust_review: EditorInlineSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: EditorInlineSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: EditorInlineSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5EditorInlineSharedConsumersPacket {
    /// Builds an editor-inline shared-consumer packet from stable-lane input.
    pub fn new(input: M5EditorInlineSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_EDITOR_INLINE_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_EDITOR_INLINE_SHARED_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            consumer_bindings: input.consumer_bindings,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the editor-inline shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5EditorInlineSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EDITOR_INLINE_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5EditorInlineSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EDITOR_INLINE_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5EditorInlineSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5EditorInlineSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5EditorInlineSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5EditorInlineSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5EditorInlineSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5EditorInlineSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5EditorInlineSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("editor-inline shared-consumer packet serializes"),
        ) {
            violations.push(M5EditorInlineSharedConsumersViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("editor-inline shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from("component,consumer,representation,state_word,parity_state\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.component.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.state_word,
                binding.parity_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Shared Editor-Inline Component Consumers: One Vocabulary Across Surfaces\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed)\n",
            self.consumer_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: component `{}` on `{}`, representation `{}`, state `{}`\n",
                binding.inline_object_label,
                binding.binding_id,
                binding.component.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.state_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in editor-inline shared-consumer export.
#[derive(Debug)]
pub enum M5EditorInlineSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5EditorInlineSharedConsumersViolation>),
}

impl fmt::Display for M5EditorInlineSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "editor-inline shared-consumer export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "editor-inline shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5EditorInlineSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5EditorInlineSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5EditorInlineSharedConsumersViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No consumer bindings are present.
    ConsumerBindingsMissing,
    /// A consumer binding is incomplete.
    BindingIncomplete,
    /// A binding's vocabulary values are incomplete.
    VocabularyFacetIncomplete,
    /// A binding's state word is not a frozen disposition token.
    StateWordOutsideVocabulary,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same inline object with different vocabulary.
    VocabularyDriftAcrossSurfaces,
    /// A shared component is not adopted by at least two distinct consumers.
    InlineComponentReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow note.
    NarrowNoteMissing,
    /// A narrow note's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow note's next action does not match the required next action.
    NarrowNextActionMismatch,
    /// A narrow note is missing its preserved-vocabulary note.
    NarrowNotePreservedVocabularyMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A full-desktop binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit remote-source note is missing it.
    RemoteSourceNoteMissing,
    /// A binding that needs an explicit export-evidence note is missing it.
    ExportEvidenceNoteMissing,
    /// A binding encodes inline state by color alone.
    StateEncodedByColorAlone,
    /// A binding lets a comment anchor or evidence pointer silently drift.
    AnchorOrEvidencePointerDrift,
    /// A binding blurs outdated and resolved review state.
    OutdatedResolvedBlurred,
    /// A binding presents an inferred fix as exact.
    InferredFixShownAsExact,
    /// A binding hides an evidence timeline in an opaque log.
    EvidenceHiddenInOpaqueLog,
    /// A binding rewords the inline vocabulary per surface.
    VocabularyRewordedPerSurface,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared component appears among the bindings.
    ComponentCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5EditorInlineSharedConsumersViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::VocabularyFacetIncomplete => "vocabulary_facet_incomplete",
            Self::StateWordOutsideVocabulary => "state_word_outside_vocabulary",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::VocabularyDriftAcrossSurfaces => "vocabulary_drift_across_surfaces",
            Self::InlineComponentReuseUnproven => "inline_component_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedVocabularyMissing => {
                "narrow_note_preserved_vocabulary_missing"
            }
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::RemoteSourceNoteMissing => "remote_source_note_missing",
            Self::ExportEvidenceNoteMissing => "export_evidence_note_missing",
            Self::StateEncodedByColorAlone => "state_encoded_by_color_alone",
            Self::AnchorOrEvidencePointerDrift => "anchor_or_evidence_pointer_drift",
            Self::OutdatedResolvedBlurred => "outdated_resolved_blurred",
            Self::InferredFixShownAsExact => "inferred_fix_shown_as_exact",
            Self::EvidenceHiddenInOpaqueLog => "evidence_hidden_in_opaque_log",
            Self::VocabularyRewordedPerSurface => "vocabulary_reworded_per_surface",
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable editor-inline shared-consumer export.
pub fn current_stable_m5_editor_inline_shared_consumers_export(
) -> Result<M5EditorInlineSharedConsumersPacket, M5EditorInlineSharedConsumersArtifactError> {
    let packet: M5EditorInlineSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-editor-inline-shared-consumers-proof/support_export.json"
    )))
    .map_err(M5EditorInlineSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5EditorInlineSharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5EditorInlineSharedConsumersPacket,
    violations: &mut Vec<M5EditorInlineSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_EDITOR_INLINE_SHARED_CONSUMERS_SCHEMA_REF,
        M5_EDITOR_INLINE_SHARED_CONSUMERS_DOC_REF,
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
        M5_EDITOR_INLINE_COMPONENT_DOC_REF,
    ];
    for family in M5EditorInlineComponentFamily::ALL {
        required.push(family.canonical_component_schema_ref());
    }
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5EditorInlineSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5EditorInlineSharedConsumersPacket,
    violations: &mut Vec<M5EditorInlineSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5EditorInlineSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One vocabulary: the facet values must be identical for every binding that renders
    // the same inline object.
    let mut object_facets: BTreeMap<&str, &EditorInlineStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each component must be adopted by at least two distinct consumers.
    let mut component_consumers: BTreeMap<
        M5EditorInlineComponentFamily,
        BTreeSet<M5EditorInlineConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5EditorInlineConsumerSurface> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5EditorInlineComponentFamily> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.inline_object_id.trim().is_empty()
            || binding.inline_object_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5EditorInlineSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5EditorInlineSharedConsumersViolation::VocabularyFacetIncomplete);
        }
        if !binding.state_facets.state_word_in_vocabulary() {
            violations.push(M5EditorInlineSharedConsumersViolation::StateWordOutsideVocabulary);
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5EditorInlineSharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5EditorInlineSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5EditorInlineSharedConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations
                            .push(M5EditorInlineSharedConsumersViolation::NarrowNextActionMismatch);
                    }
                    if note.preserved_vocabulary_note.trim().is_empty() {
                        violations.push(
                            M5EditorInlineSharedConsumersViolation::NarrowNotePreservedVocabularyMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5EditorInlineSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5EditorInlineSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5EditorInlineSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_evidence_note && binding.export_evidence_note.trim().is_empty() {
            violations.push(M5EditorInlineSharedConsumersViolation::ExportEvidenceNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.encodes_state_by_color_alone {
            violations.push(M5EditorInlineSharedConsumersViolation::StateEncodedByColorAlone);
        }
        if binding.lets_anchor_or_evidence_pointer_silently_drift {
            violations.push(M5EditorInlineSharedConsumersViolation::AnchorOrEvidencePointerDrift);
        }
        if binding.blurs_outdated_and_resolved_review_state {
            violations.push(M5EditorInlineSharedConsumersViolation::OutdatedResolvedBlurred);
        }
        if binding.presents_inferred_fix_as_exact {
            violations.push(M5EditorInlineSharedConsumersViolation::InferredFixShownAsExact);
        }
        if binding.hides_evidence_in_opaque_log {
            violations.push(M5EditorInlineSharedConsumersViolation::EvidenceHiddenInOpaqueLog);
        }
        if binding.rewords_inline_vocabulary_per_surface {
            violations.push(M5EditorInlineSharedConsumersViolation::VocabularyRewordedPerSurface);
        }

        // Support / export consumers must map inline state back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5EditorInlineSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Vocabulary-drift accumulation.
        match object_facets.get(binding.inline_object_id.as_str()) {
            None => {
                object_facets.insert(binding.inline_object_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5EditorInlineSharedConsumersViolation::VocabularyDriftAcrossSurfaces,
                    );
                    drift_reported = true;
                }
            }
        }

        component_consumers
            .entry(binding.component)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_components.insert(binding.component);
    }

    // Coverage: every consumer surface and every component must appear.
    for consumer in M5EditorInlineConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5EditorInlineSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for component in M5EditorInlineComponentFamily::ALL {
        if !seen_components.contains(&component) {
            violations.push(M5EditorInlineSharedConsumersViolation::ComponentCoverageMissing);
            break;
        }
    }

    // Reuse: every present component must be adopted by two or more distinct consumers.
    for consumers in component_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5EditorInlineSharedConsumersViolation::InlineComponentReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
