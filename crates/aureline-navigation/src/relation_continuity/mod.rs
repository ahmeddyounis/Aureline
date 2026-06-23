//! Relation-aware peek/history continuity and the replay-safe support/export packet.
//!
//! The [`target_model`](crate::target_model) freezes the relation vocabulary —
//! [`RelationKind`](crate::target_model::RelationKind),
//! [`ContinuityState`](crate::target_model::ContinuityState),
//! [`AmbiguityClass`](crate::target_model::AmbiguityClass), and the proof/freshness
//! classes — and the [`bookmark_history_and_drift_continuity`](crate::bookmark_history_and_drift_continuity)
//! module froze a *generic* continuity packet whose anchors carry drift state but **no
//! relation semantics**. What was still implicit is the *relation-aware* continuity
//! model: a peek, temporary reveal, open-in-split, back/forward-history, or
//! recent-location entry that remembers **which relation kind** it navigated, **which
//! surface** it came from, **where to return**, and **whether the captured target still
//! resolves** — so a return jump or a replay never degenerates into generic open
//! behavior or a silent retarget to a nearby guess.
//!
//! This module is that model. [`build_relation_continuity_packet`] is a pure function
//! over a typed [`RelationContinuityInput`] that produces a [`RelationContinuityPacket`]:
//!
//! 1. **Relation-aware entries.** Each [`RelationNavEntryInput`] becomes a
//!    [`RelationNavigationEntry`] that preserves its [`RelationNavEntryKind`], its
//!    [`RelationKind`](crate::target_model::RelationKind), its origin surface, a
//!    [`ReturnAnchor`] that restores the origin selection/viewport, and a
//!    [`RelationTargetSnapshot`] for **both** the captured target and the
//!    currently-resolved one — so peek/reveal/split/history entries never lose relation
//!    semantics or return context.
//! 2. **Current-versus-captured truth.** A [`RelationContinuityCounts`] separates the
//!    entries that still resolve against the current scope from those carried only by a
//!    captured snapshot, trace, or imported pack, and tallies entries by kind and by
//!    drift state, so the packet always states what is live and what is captured.
//! 3. **No silent jump.** An entry only auto-opens when it is [`Bound`] with live
//!    semantic evidence. A [`Remapped`] entry cites stable remap evidence (never a
//!    nearby fallback) and offers an explicit open action; a [`Drifted`],
//!    [`MissingTarget`], [`ScopeUnavailable`], or [`Archived`] entry carries no current
//!    target, keeps its drift reason and recovery choices visible, and routes ambiguity
//!    through a disambiguation set — so the surface shows the drift state before any jump.
//! 4. **Replay-safe support/export.** Every entry and every [`RenamePreviewEvidence`]
//!    row carries a stable replay-safe target id derived from its own id, names its
//!    [`RelationContinuityEvidenceClass`] so a lexical/grep fallback never reads as
//!    semantic certainty, and keeps ambiguity, drift, return anchors, and relation kind
//!    visible — so a support or debug packet can replay or explain symbol navigation and
//!    rename evidence without retargeting.
//! 5. **Consumer parity.** Each packet projects to every
//!    [`ConsumerSurface`](crate::target_model::ConsumerSurface) with a
//!    [`RelationContinuityProjection`] that preserves relation kind, origin surface,
//!    return anchor, current-versus-captured truth, drift state, fallback class,
//!    ambiguity, and replay ids, never silently retargets, and never exports code bodies.
//!
//! [`relation_continuity_set`] freezes a deterministic corpus of packets whose
//! [`RelationContinuityInvariant`] flags are computed from the builder's own output, so
//! the checked-in fixture and the freeze gate pin the contract byte-for-byte and any
//! regression in [`build_relation_continuity_packet`] flips an invariant or drifts the
//! fixture rather than silently passing. The records carry no source bodies, raw paths,
//! provider payloads, identifiers, URLs, hostnames, or credentials — only opaque object
//! handles, stable tokens, and short reviewable sentences — so they are safe for support
//! export.
//!
//! [`Bound`]: crate::target_model::ContinuityState::Bound
//! [`Remapped`]: crate::target_model::ContinuityState::Remapped
//! [`Drifted`]: crate::target_model::ContinuityState::Drifted
//! [`MissingTarget`]: crate::target_model::ContinuityState::MissingTarget
//! [`ScopeUnavailable`]: crate::target_model::ContinuityState::ScopeUnavailable
//! [`Archived`]: crate::target_model::ContinuityState::Archived

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::target_model::{
    AccessKind, AmbiguityClass, ConsumerSurface, ContinuityState, DowngradeReason,
    ExportRedactionClass, FreshnessClass, GeneratedOrExternalState, NavigationConfidence,
    ProofClass, ProviderClass, RelationKind, RenameApplyPosture, ScopeCompleteness,
    REQUIRED_CONSUMER_SURFACES,
};

#[cfg(test)]
mod tests;

/// Schema version for the relation-continuity corpus.
pub const RELATION_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the relation-continuity corpus.
pub const RELATION_CONTINUITY_SCHEMA_REF: &str =
    "schemas/navigation/relation_continuity.schema.json";

/// Stable record-kind tag for the relation-continuity corpus.
pub const RELATION_CONTINUITY_RECORD_KIND: &str = "relation_continuity_set";

/// Stable id for the canonical relation-continuity corpus.
pub const RELATION_CONTINUITY_SET_ID: &str = "relation-continuity:set:0001";

/// Evaluation stamp for the canonical corpus. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const RELATION_CONTINUITY_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the corpus binding current. Stable promotion runs this
/// gate; it fails when the in-code corpus drifts from the checked-in fixture or any
/// invariant flips.
pub const RELATION_CONTINUITY_FREEZE_GATE_REF: &str =
    "crates/aureline-navigation/tests/relation_continuity.rs";

/// Reviewer doc for the relation-continuity contract.
pub const RELATION_CONTINUITY_DOC_REF: &str = "docs/navigation/relation_continuity.md";

/// Evidence companion for the relation-continuity corpus.
pub const RELATION_CONTINUITY_ARTIFACT_REF: &str = "artifacts/navigation/relation_continuity.md";

/// Repo-relative path of the checked-in canonical corpus.
pub const RELATION_CONTINUITY_FIXTURE_REF: &str =
    "fixtures/navigation/relation_continuity/canonical_continuity.json";

/// The canonical order navigation entries are listed in within a packet.
pub const RELATION_NAV_ENTRY_ORDER: [RelationNavEntryKind; 6] = [
    RelationNavEntryKind::Peek,
    RelationNavEntryKind::TemporaryReveal,
    RelationNavEntryKind::OpenInSplit,
    RelationNavEntryKind::BackHistory,
    RelationNavEntryKind::ForwardHistory,
    RelationNavEntryKind::RecentLocation,
];

/// The drift states every relation-continuity packet must keep visible.
pub const RELATION_CONTINUITY_DRIFT_STATES: [ContinuityState; 6] = [
    ContinuityState::Bound,
    ContinuityState::Remapped,
    ContinuityState::Drifted,
    ContinuityState::MissingTarget,
    ContinuityState::ScopeUnavailable,
    ContinuityState::Archived,
];

// ---------------------------------------------------------------------------
// Entry kinds and vocabularies.
// ---------------------------------------------------------------------------

/// The navigation surface affordance a continuity entry was created by.
///
/// Peek, temporary reveal, and open-in-split keep the origin context live; back,
/// forward, and recent-location move the active context but still carry a return anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationNavEntryKind {
    /// Inline peek overlay that returns to the invocation site.
    Peek,
    /// Temporary reveal of a definition/reference that auto-dismisses.
    TemporaryReveal,
    /// Open-in-split that keeps the origin pane and its selection.
    OpenInSplit,
    /// Back-stack history entry.
    BackHistory,
    /// Forward-stack history entry.
    ForwardHistory,
    /// Recent-location entry.
    RecentLocation,
}

impl RelationNavEntryKind {
    /// All entry kinds, in canonical order.
    pub const ALL: [Self; 6] = RELATION_NAV_ENTRY_ORDER;

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Peek => "peek",
            Self::TemporaryReveal => "temporary_reveal",
            Self::OpenInSplit => "open_in_split",
            Self::BackHistory => "back_history",
            Self::ForwardHistory => "forward_history",
            Self::RecentLocation => "recent_location",
        }
    }

    /// Returns a human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Peek => "Peek",
            Self::TemporaryReveal => "Temporary reveal",
            Self::OpenInSplit => "Open in split",
            Self::BackHistory => "Back",
            Self::ForwardHistory => "Forward",
            Self::RecentLocation => "Recent location",
        }
    }

    /// Returns true when the entry keeps the origin context live beside the target
    /// rather than replacing it (peek, temporary reveal, open-in-split).
    pub const fn keeps_origin_context_live(self) -> bool {
        matches!(self, Self::Peek | Self::TemporaryReveal | Self::OpenInSplit)
    }
}

/// The evidence class for a continuity entry or rename-evidence row.
///
/// Answers the support/debug question "was this navigation semantic, framework-derived,
/// runtime-observed, imported, or a lexical fallback?" so a grep match is never replayed
/// as semantic certainty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationContinuityEvidenceClass {
    /// Direct or indexed semantic evidence over current source.
    Semantic,
    /// Framework pack, route metadata, or generator evidence.
    FrameworkDerived,
    /// Runtime trace, debugger, or observed-dispatch evidence.
    RuntimeObserved,
    /// Imported snapshot, docs pack, or provider overlay evidence.
    ImportedSnapshot,
    /// Lexical or grep fallback evidence.
    LexicalFallback,
    /// Syntax-tree-only fallback evidence.
    SyntaxFallback,
    /// No admissible evidence for the entry.
    Unavailable,
}

impl RelationContinuityEvidenceClass {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Semantic => "semantic",
            Self::FrameworkDerived => "framework_derived",
            Self::RuntimeObserved => "runtime_observed",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::LexicalFallback => "lexical_fallback",
            Self::SyntaxFallback => "syntax_fallback",
            Self::Unavailable => "unavailable",
        }
    }

    /// Returns true when this evidence class must render with a visible caveat rather
    /// than as plain semantic certainty.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Semantic)
    }

    /// Returns true when this evidence class rests on a lexical or syntax fallback.
    pub const fn is_fallback(self) -> bool {
        matches!(self, Self::LexicalFallback | Self::SyntaxFallback)
    }

    /// Maps a [`ProofClass`] onto the continuity evidence class.
    pub const fn from_proof(proof: ProofClass) -> Self {
        match proof {
            ProofClass::DirectSemantic | ProofClass::IndexedSemantic => Self::Semantic,
            ProofClass::FrameworkDerived => Self::FrameworkDerived,
            ProofClass::RuntimeObserved => Self::RuntimeObserved,
            ProofClass::ImportedEvidence => Self::ImportedSnapshot,
            ProofClass::LexicalFallback => Self::LexicalFallback,
            ProofClass::SyntaxFallback => Self::SyntaxFallback,
            ProofClass::AiInferred | ProofClass::Unavailable => Self::Unavailable,
        }
    }
}

/// A recovery choice a surface offers before navigating a non-bound entry.
///
/// The presence of recovery choices is what keeps a drifted, missing, unavailable, or
/// remapped entry from silently jumping to a nearby guess.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationRecoveryChoice {
    /// Open the stable-remap target after disclosing the remap.
    OpenRemappedTarget,
    /// Inspect the drift before doing anything.
    InspectDrift,
    /// Choose a successor from the disambiguation set.
    ChooseFromDisambiguation,
    /// Restore only the return anchor, not the drifted target.
    RestoreReturnAnchorOnly,
    /// Widen the workset or scope to bring the target back.
    WidenScope,
    /// Refresh the provider or index before retrying.
    RefreshProvider,
    /// Keep the archived reference as metadata without opening.
    KeepArchivedReference,
}

impl RelationRecoveryChoice {
    /// All recovery choices, in canonical order.
    pub const ALL: [Self; 7] = [
        Self::OpenRemappedTarget,
        Self::InspectDrift,
        Self::ChooseFromDisambiguation,
        Self::RestoreReturnAnchorOnly,
        Self::WidenScope,
        Self::RefreshProvider,
        Self::KeepArchivedReference,
    ];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRemappedTarget => "open_remapped_target",
            Self::InspectDrift => "inspect_drift",
            Self::ChooseFromDisambiguation => "choose_from_disambiguation",
            Self::RestoreReturnAnchorOnly => "restore_return_anchor_only",
            Self::WidenScope => "widen_scope",
            Self::RefreshProvider => "refresh_provider",
            Self::KeepArchivedReference => "keep_archived_reference",
        }
    }
}

/// A user-visible label a continuity entry attaches so a degraded, captured, drifted, or
/// fallback entry never reads as a plain live jump.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationContinuityLabel {
    /// The entry is carried only by a captured snapshot/trace/pack, not current source.
    CapturedScopeOnly,
    /// The entry rests on runtime-observed evidence.
    RuntimeObserved,
    /// The entry rests on framework-derived evidence.
    FrameworkDerived,
    /// The entry rests on imported-snapshot evidence.
    ImportedSnapshot,
    /// The entry rests on a lexical/grep fallback.
    LexicalFallback,
    /// The entry rests on a syntax-only fallback.
    SyntaxFallback,
    /// The entry's target is generated or a paired artifact.
    Generated,
    /// The entry's target is external-dependency source.
    External,
    /// The entry's target is read-only or protected source.
    ReadOnly,
    /// The entry's target is test-only.
    TestOnly,
    /// The entry's target moved and was remapped via stable evidence.
    Remapped,
    /// The entry's target drifted and cannot auto-open.
    Drifted,
    /// The entry's target is missing from the current scope.
    MissingTarget,
    /// The entry's target is outside the active workset, branch, policy, or shard.
    ScopeUnavailable,
    /// The entry is retained only as an archive/tombstone.
    Archived,
    /// The entry needs an explicit disambiguation selection.
    AmbiguousNeedsSelection,
}

impl RelationContinuityLabel {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapturedScopeOnly => "captured_scope_only",
            Self::RuntimeObserved => "runtime_observed",
            Self::FrameworkDerived => "framework_derived",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::LexicalFallback => "lexical_fallback",
            Self::SyntaxFallback => "syntax_fallback",
            Self::Generated => "generated",
            Self::External => "external",
            Self::ReadOnly => "read_only",
            Self::TestOnly => "test_only",
            Self::Remapped => "remapped",
            Self::Drifted => "drifted",
            Self::MissingTarget => "missing_target",
            Self::ScopeUnavailable => "scope_unavailable",
            Self::Archived => "archived",
            Self::AmbiguousNeedsSelection => "ambiguous_needs_selection",
        }
    }
}

// ---------------------------------------------------------------------------
// Return anchor and target snapshot.
// ---------------------------------------------------------------------------

/// The return context a continuity entry restores when the user returns from a jump.
///
/// Every entry keeps a return anchor, so a peek, split, or back/forward jump returns to
/// where the user was rather than stranding them at the target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnAnchor {
    /// Stable return-anchor id.
    pub return_anchor_ref: String,
    /// Surface the navigation originated from.
    pub origin_surface: ConsumerSurface,
    /// Stable object ref the user was on when they navigated.
    pub origin_object_ref: String,
    /// Stable scope ref active at the origin.
    pub origin_scope_ref: String,
    /// Always true: returning restores the origin selection.
    pub restores_selection: bool,
    /// Always true: returning restores the origin viewport.
    pub restores_viewport: bool,
}

/// A stable, replay-safe snapshot of a navigation target — captured or current.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationTargetSnapshot {
    /// Replay-safe stable target id.
    pub target_id: String,
    /// Relation kind the snapshot represents.
    pub relation_kind: RelationKind,
    /// Stable object ref.
    pub object_ref: String,
    /// Stable anchor ref.
    pub anchor_ref: String,
    /// Stable scope ref the snapshot was resolved against.
    pub scope_ref: String,
}

// ---------------------------------------------------------------------------
// Entry input and built entry.
// ---------------------------------------------------------------------------

/// The typed seed the builder turns into a [`RelationNavigationEntry`].
///
/// The builder never invents relation kind, drift state, proof class, or remap evidence;
/// it derives the entry deterministically from this seed, so a provider cannot smuggle a
/// drifted target into a live jump or a grep match into a semantic one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavEntryInput {
    /// Stable entry id.
    pub entry_id: String,
    /// The navigation affordance that created the entry.
    pub entry_kind: RelationNavEntryKind,
    /// The relation kind the entry navigated.
    pub relation_kind: RelationKind,
    /// Surface the navigation originated from.
    pub origin_surface: ConsumerSurface,
    /// Stable object ref the user was on at the origin.
    pub origin_object_ref: String,
    /// Stable scope ref active at the origin.
    pub origin_scope_ref: String,
    /// Replay-safe id of the target as captured at navigation time.
    pub captured_target_id: String,
    /// Stable object ref of the captured target.
    pub captured_object_ref: String,
    /// Stable anchor ref of the captured target.
    pub captured_anchor_ref: String,
    /// Stable scope ref the captured target belonged to.
    pub captured_scope_ref: String,
    /// Current drift state of the captured target.
    pub drift_state: ContinuityState,
    /// Stable object ref of the remapped target, when [`ContinuityState::Remapped`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remapped_object_ref: Option<String>,
    /// Stable anchor ref of the remapped target, when [`ContinuityState::Remapped`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remapped_anchor_ref: Option<String>,
    /// Stable remap evidence refs; required for a remap, and never a nearby fallback.
    #[serde(default)]
    pub remap_evidence_refs: Vec<String>,
    /// Access kind of the captured target.
    pub access_kind: AccessKind,
    /// Proof class for the entry's evidence.
    pub proof_class: ProofClass,
    /// Provider family that admitted the entry.
    pub provider_class: ProviderClass,
    /// Confidence class for the entry.
    pub confidence: NavigationConfidence,
    /// Freshness class for the entry.
    pub freshness: FreshnessClass,
    /// Ambiguity class for the entry.
    pub ambiguity_class: AmbiguityClass,
    /// Completeness of the entry's materialized scope.
    pub scope_completeness: ScopeCompleteness,
    /// Authorship, generated, external, read-only, or imported posture.
    pub generated_or_external_state: GeneratedOrExternalState,
    /// Disambiguation set ref when the entry needs an explicit selection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disambiguation_set_ref: Option<String>,
    /// Explicit drift reason; required for any non-bound state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift_reason: Option<String>,
    /// Downgrade reasons that must stay visible on consumers.
    #[serde(default)]
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Evidence refs safe for support, review, AI, and CLI consumers.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

/// One relation-aware continuity entry: a peek, reveal, split, or history location that
/// remembers its relation kind, origin, return anchor, and current-versus-captured truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavigationEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// The navigation affordance that created the entry.
    pub entry_kind: RelationNavEntryKind,
    /// The relation kind the entry navigated.
    pub relation_kind: RelationKind,
    /// The return context the entry restores.
    pub return_anchor: ReturnAnchor,
    /// The target as captured at navigation time.
    pub captured_target: RelationTargetSnapshot,
    /// The currently-resolved target, present only for bound or remapped entries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_target: Option<RelationTargetSnapshot>,
    /// Current drift state of the captured target.
    pub drift_state: ContinuityState,
    /// Explicit drift reason, present for any non-bound state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift_reason: Option<String>,
    /// Stable remap evidence refs cited by a remapped entry.
    #[serde(default)]
    pub remap_evidence_refs: Vec<String>,
    /// Always false: a remap never rests on a nearest-line or nearest-symbol fallback.
    pub used_nearby_fallback: bool,
    /// Whether the entry may auto-open without an explicit recovery action.
    pub auto_open_allowed: bool,
    /// Proof class for the entry's evidence.
    pub proof_class: ProofClass,
    /// The entry's evidence class.
    pub evidence_class: RelationContinuityEvidenceClass,
    /// Provider family that admitted the entry.
    pub provider_class: ProviderClass,
    /// Confidence class for the entry.
    pub confidence: NavigationConfidence,
    /// Freshness class for the entry.
    pub freshness: FreshnessClass,
    /// Ambiguity class for the entry.
    pub ambiguity_class: AmbiguityClass,
    /// Completeness of the entry's materialized scope.
    pub scope_completeness: ScopeCompleteness,
    /// True when the entry still resolves against the current scope with live evidence.
    pub current_scope: bool,
    /// Disambiguation set ref when the entry needs an explicit selection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disambiguation_set_ref: Option<String>,
    /// Replay-safe id a support/export packet uses to reconstruct the navigation.
    pub replay_target_id: String,
    /// Recovery choices a surface offers before navigating a non-bound entry.
    #[serde(default)]
    pub recovery_choices: Vec<RelationRecoveryChoice>,
    /// Visible labels for the entry.
    #[serde(default)]
    pub labels: Vec<RelationContinuityLabel>,
    /// Downgrade reasons that must stay visible on consumers.
    #[serde(default)]
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Fallback notes describing lexical/syntax/runtime/framework/imported evidence.
    #[serde(default)]
    pub fallback_notes: Vec<String>,
    /// Evidence refs safe for support, review, AI, and CLI consumers.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

impl RelationNavigationEntry {
    /// Returns true when the entry must render with a visible caveat rather than as a
    /// plain live jump.
    pub fn requires_disclosure(&self) -> bool {
        self.drift_state.requires_user_review()
            || self.drift_state == ContinuityState::Remapped
            || !self.current_scope
            || self.evidence_class.requires_disclosure()
            || self.ambiguity_class.requires_disambiguation()
            || !self.downgrade_reasons.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Rename-preview evidence.
// ---------------------------------------------------------------------------

/// The typed seed the builder turns into a [`RenamePreviewEvidence`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenamePreviewEvidenceInput {
    /// Stable evidence id.
    pub evidence_id: String,
    /// Stable ref to the rename-preview the evidence replays.
    pub rename_preview_ref: String,
    /// Surface the rename originated from.
    pub origin_surface: ConsumerSurface,
    /// Stable object ref the user was on when the rename was previewed.
    pub origin_object_ref: String,
    /// Stable scope ref active at the origin.
    pub origin_scope_ref: String,
    /// Proof class for the rename evidence.
    pub proof_class: ProofClass,
    /// Apply posture captured from the rename preview.
    pub apply_posture: RenameApplyPosture,
    /// Ambiguity class for the rename root.
    pub ambiguity_class: AmbiguityClass,
    /// Drift state of the rename root since the preview was captured.
    pub drift_state: ContinuityState,
    /// Candidates the rename would change.
    pub changed_count: usize,
    /// Candidates the rename would hold rather than change.
    pub held_count: usize,
    /// Disambiguation set ref when the rename root needs an explicit selection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disambiguation_set_ref: Option<String>,
    /// Explicit drift reason, required for any non-bound state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift_reason: Option<String>,
    /// Downgrade reasons that must stay visible on consumers.
    #[serde(default)]
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Evidence refs safe for support, review, AI, and CLI consumers.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

/// Replay-safe rename-preview evidence carried in a continuity packet.
///
/// Retains the relation kind, fallback class, ambiguity state, return anchor, apply
/// posture, change-versus-held counts, and a replay-safe id, so a support or debug
/// packet can replay or explain rename evidence without silent retargeting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenamePreviewEvidence {
    /// Stable evidence id.
    pub evidence_id: String,
    /// Stable ref to the rename-preview the evidence replays.
    pub rename_preview_ref: String,
    /// The relation kind of the renamed root (always [`RelationKind::Definition`]).
    pub root_relation_kind: RelationKind,
    /// The return context the rename evidence restores.
    pub return_anchor: ReturnAnchor,
    /// The evidence class for the rename root.
    pub evidence_class: RelationContinuityEvidenceClass,
    /// Apply posture captured from the rename preview.
    pub apply_posture: RenameApplyPosture,
    /// Ambiguity class for the rename root.
    pub ambiguity_class: AmbiguityClass,
    /// Drift state of the rename root since the preview was captured.
    pub drift_state: ContinuityState,
    /// Explicit drift reason, present for any non-bound state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift_reason: Option<String>,
    /// Candidates the rename would change.
    pub changed_count: usize,
    /// Candidates the rename would hold rather than change.
    pub held_count: usize,
    /// Disambiguation set ref when the rename root needs an explicit selection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disambiguation_set_ref: Option<String>,
    /// Replay-safe id a support/export packet uses to reconstruct the rename.
    pub replay_target_id: String,
    /// Downgrade reasons that must stay visible on consumers.
    #[serde(default)]
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Fallback notes describing the rename evidence's weakest class.
    #[serde(default)]
    pub fallback_notes: Vec<String>,
    /// Evidence refs safe for support, review, AI, and CLI consumers.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Counts.
// ---------------------------------------------------------------------------

/// Current-versus-captured and by-kind/by-drift counts for a continuity packet.
///
/// The by-kind tallies and the by-drift tallies each sum to `total_count`, and
/// `current_scope_count + captured_scope_count == total_count`, so the packet always
/// states what is live and what is captured.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationContinuityCounts {
    /// Total entries in the packet.
    pub total_count: usize,
    /// Peek entries.
    pub peek_count: usize,
    /// Temporary-reveal entries.
    pub temporary_reveal_count: usize,
    /// Open-in-split entries.
    pub open_in_split_count: usize,
    /// Back-history entries.
    pub back_history_count: usize,
    /// Forward-history entries.
    pub forward_history_count: usize,
    /// Recent-location entries.
    pub recent_location_count: usize,
    /// Bound entries.
    pub bound_count: usize,
    /// Remapped entries.
    pub remapped_count: usize,
    /// Drifted entries.
    pub drifted_count: usize,
    /// Missing-target entries.
    pub missing_target_count: usize,
    /// Scope-unavailable entries.
    pub scope_unavailable_count: usize,
    /// Archived entries.
    pub archived_count: usize,
    /// Entries still resolving against the current scope with live evidence.
    pub current_scope_count: usize,
    /// Entries carried only by a captured snapshot, trace, or imported pack.
    pub captured_scope_count: usize,
    /// Entries that must disclose their evidence or drift before any jump (every entry
    /// that does not auto-open).
    pub requires_disclosure_count: usize,
    /// Entries that need an explicit disambiguation selection.
    pub ambiguous_count: usize,
}

impl RelationContinuityCounts {
    /// Returns true when the by-kind, by-drift, and current/captured tallies all
    /// reconcile with the total.
    pub const fn reconciles(&self) -> bool {
        self.peek_count
            + self.temporary_reveal_count
            + self.open_in_split_count
            + self.back_history_count
            + self.forward_history_count
            + self.recent_location_count
            == self.total_count
            && self.bound_count
                + self.remapped_count
                + self.drifted_count
                + self.missing_target_count
                + self.scope_unavailable_count
                + self.archived_count
                == self.total_count
            && self.current_scope_count + self.captured_scope_count == self.total_count
    }

    fn add(&mut self, entry: &RelationNavigationEntry) {
        self.total_count += 1;
        match entry.entry_kind {
            RelationNavEntryKind::Peek => self.peek_count += 1,
            RelationNavEntryKind::TemporaryReveal => self.temporary_reveal_count += 1,
            RelationNavEntryKind::OpenInSplit => self.open_in_split_count += 1,
            RelationNavEntryKind::BackHistory => self.back_history_count += 1,
            RelationNavEntryKind::ForwardHistory => self.forward_history_count += 1,
            RelationNavEntryKind::RecentLocation => self.recent_location_count += 1,
        }
        match entry.drift_state {
            ContinuityState::Bound => self.bound_count += 1,
            ContinuityState::Remapped => self.remapped_count += 1,
            ContinuityState::Drifted => self.drifted_count += 1,
            ContinuityState::MissingTarget => self.missing_target_count += 1,
            ContinuityState::ScopeUnavailable => self.scope_unavailable_count += 1,
            ContinuityState::Archived => self.archived_count += 1,
        }
        if entry.current_scope {
            self.current_scope_count += 1;
        } else {
            self.captured_scope_count += 1;
        }
        if !entry.auto_open_allowed {
            self.requires_disclosure_count += 1;
        }
        if entry.ambiguity_class.requires_disambiguation() {
            self.ambiguous_count += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// Consumer projection.
// ---------------------------------------------------------------------------

/// A surface-level projection proving the packet survives review, support, AI, graph,
/// docs, editor, and CLI consumers without losing relation semantics or retargeting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationContinuityProjection {
    /// The consumer surface.
    pub consumer_surface: ConsumerSurface,
    /// Number of navigation entries projected to this surface.
    pub projected_entry_count: usize,
    /// Number of rename-evidence rows projected to this surface.
    pub projected_rename_evidence_count: usize,
    /// True when relation-kind labels are preserved.
    pub preserves_relation_kind: bool,
    /// True when origin surfaces are preserved.
    pub preserves_origin_surface: bool,
    /// True when return anchors are preserved.
    pub preserves_return_anchor: bool,
    /// True when the current-versus-captured split is preserved.
    pub preserves_current_vs_captured: bool,
    /// True when drift states are preserved.
    pub preserves_drift_state: bool,
    /// True when fallback/evidence classes are preserved.
    pub preserves_fallback_class: bool,
    /// True when ambiguity state and disambiguation paths are preserved.
    pub preserves_ambiguity_state: bool,
    /// True when replay-safe target ids are preserved.
    pub preserves_replay_target_ids: bool,
    /// True when the projection silently retargets a drifted entry (must be false).
    pub silently_retargets: bool,
    /// True when the projection exports raw code bodies (must be false).
    pub exports_code_bodies: bool,
    /// Redaction class for this projection.
    pub redaction_class: ExportRedactionClass,
    /// Export-safe summary.
    pub summary: String,
}

impl RelationContinuityProjection {
    /// Returns true when the projection preserves the packet's typed truth without
    /// retargeting or leaking code bodies.
    pub const fn preserves_truth(&self) -> bool {
        self.preserves_relation_kind
            && self.preserves_origin_surface
            && self.preserves_return_anchor
            && self.preserves_current_vs_captured
            && self.preserves_drift_state
            && self.preserves_fallback_class
            && self.preserves_ambiguity_state
            && self.preserves_replay_target_ids
            && !self.silently_retargets
            && !self.exports_code_bodies
    }
}

// ---------------------------------------------------------------------------
// Packet and input.
// ---------------------------------------------------------------------------

/// A relation-aware, replay-safe continuity packet: navigation entries and rename
/// evidence with their relation kind, origin, return anchor, drift state, fallback
/// class, ambiguity, and replay ids preserved, plus consumer projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationContinuityPacket {
    /// Stable packet id.
    pub packet_id: String,
    /// Surface the packet was assembled from.
    pub origin_surface: ConsumerSurface,
    /// The current scope ref the packet was resolved against.
    pub scope_ref: String,
    /// The captured snapshot/trace/pack scope ref, when any entry is captured-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_scope_ref: Option<String>,
    /// Relation-aware navigation entries, in canonical order.
    pub entries: Vec<RelationNavigationEntry>,
    /// Rename-preview evidence rows.
    #[serde(default)]
    pub rename_evidence: Vec<RenamePreviewEvidence>,
    /// Current-versus-captured and by-kind/by-drift counts.
    pub counts: RelationContinuityCounts,
    /// The distinct drift states present in the packet.
    pub covered_drift_states: Vec<ContinuityState>,
    /// Consumer projections proving cross-surface parity.
    pub consumer_projections: Vec<RelationContinuityProjection>,
    /// Redaction class for review/support/AI export.
    pub redaction_class: ExportRedactionClass,
    /// Downgrade reasons that must stay visible on consumers.
    #[serde(default)]
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Fallback notes describing the packet's weakest evidence.
    #[serde(default)]
    pub fallback_notes: Vec<String>,
    /// Always true: every entry and rename-evidence row carries a replay-safe target id.
    pub replay_safe: bool,
    /// Export-safe summary.
    pub summary: String,
}

impl RelationContinuityPacket {
    /// Returns the entry with a given id, if present.
    pub fn entry(&self, entry_id: &str) -> Option<&RelationNavigationEntry> {
        self.entries.iter().find(|entry| entry.entry_id == entry_id)
    }

    /// Returns true when the packet holds any entry that cannot auto-open.
    pub const fn has_disclosure_entries(&self) -> bool {
        self.counts.requires_disclosure_count > 0
    }

    /// Returns true when the packet holds any captured-only entry.
    pub const fn has_captured_scope(&self) -> bool {
        self.counts.captured_scope_count > 0
    }
}

/// The typed input the builder turns into a relation-continuity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationContinuityInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Surface the packet was assembled from.
    pub origin_surface: ConsumerSurface,
    /// The current scope ref.
    pub scope_ref: String,
    /// The captured snapshot/trace/pack scope ref, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_scope_ref: Option<String>,
    /// Redaction class for review/support/AI export.
    pub redaction_class: ExportRedactionClass,
    /// The navigation entries to preserve.
    pub entries: Vec<RelationNavEntryInput>,
    /// The rename-preview evidence rows to preserve.
    #[serde(default)]
    pub rename_evidence: Vec<RenamePreviewEvidenceInput>,
}

// ---------------------------------------------------------------------------
// Builder.
// ---------------------------------------------------------------------------

/// Builds a relation-continuity packet from a typed input.
///
/// Deterministic: the same input yields the same packet. Entries are listed in canonical
/// [`RELATION_NAV_ENTRY_ORDER`]; each entry preserves relation kind, origin, return
/// anchor, captured-versus-current target truth, drift state, evidence class, ambiguity,
/// recovery choices, labels, and a replay-safe id; rename-evidence rows preserve the same
/// dimensions; and the counts, covered drift states, downgrade reasons, fallback notes,
/// and consumer projections are derived from the entries themselves — so a drifted,
/// captured, or fallback entry can never lose its caveat or silently retarget.
pub fn build_relation_continuity_packet(
    input: &RelationContinuityInput,
) -> RelationContinuityPacket {
    // Entries are emitted in canonical kind order, preserving input order within a kind.
    let mut ordered: Vec<&RelationNavEntryInput> = Vec::new();
    for kind in RELATION_NAV_ENTRY_ORDER {
        for entry in input.entries.iter().filter(|seed| seed.entry_kind == kind) {
            ordered.push(entry);
        }
    }

    let entries: Vec<RelationNavigationEntry> =
        ordered.iter().map(|seed| build_entry(seed)).collect();
    let rename_evidence: Vec<RenamePreviewEvidence> = input
        .rename_evidence
        .iter()
        .map(build_rename_evidence)
        .collect();

    let mut counts = RelationContinuityCounts::default();
    for entry in &entries {
        counts.add(entry);
    }

    let covered_drift_states = covered_drift_states(&entries);
    let downgrade_reasons = packet_downgrade_reasons(&entries, &rename_evidence);
    let fallback_notes = packet_fallback_notes(&entries, &rename_evidence);

    let consumer_projections = REQUIRED_CONSUMER_SURFACES
        .iter()
        .map(|surface| {
            build_projection(
                *surface,
                entries.len(),
                rename_evidence.len(),
                input.redaction_class,
            )
        })
        .collect();

    let summary = format!(
        "Relation-continuity packet from {}: {} entr(y/ies) — {} current, {} captured, {} need \
         disclosure before a jump across {} drift state(s); {} rename-evidence row(s); replay-safe.",
        input.origin_surface.as_str(),
        counts.total_count,
        counts.current_scope_count,
        counts.captured_scope_count,
        counts.requires_disclosure_count,
        covered_drift_states.len(),
        rename_evidence.len(),
    );

    RelationContinuityPacket {
        packet_id: input.packet_id.clone(),
        origin_surface: input.origin_surface,
        scope_ref: input.scope_ref.clone(),
        captured_scope_ref: input.captured_scope_ref.clone(),
        entries,
        rename_evidence,
        counts,
        covered_drift_states,
        consumer_projections,
        redaction_class: input.redaction_class,
        downgrade_reasons,
        fallback_notes,
        replay_safe: true,
        summary,
    }
}

fn build_entry(seed: &RelationNavEntryInput) -> RelationNavigationEntry {
    let evidence_class = RelationContinuityEvidenceClass::from_proof(seed.proof_class);
    // Live current scope only when the entry is bound with fresh, semantic evidence; a
    // remapped, drifted, imported, runtime, framework, lexical, or stale entry is carried
    // only by a captured scope.
    let current_scope = seed.drift_state == ContinuityState::Bound
        && evidence_class == RelationContinuityEvidenceClass::Semantic
        && !matches!(
            seed.freshness,
            FreshnessClass::Stale | FreshnessClass::Unverified
        );
    let captured_only = !current_scope;
    // Only a bound, live, unambiguous entry may auto-open; everything else discloses its
    // state and offers recovery, so a return jump never lands on a nearby guess.
    let auto_open_allowed = current_scope && !seed.ambiguity_class.requires_disambiguation();

    let current_target = match seed.drift_state {
        ContinuityState::Bound => Some(RelationTargetSnapshot {
            target_id: seed.captured_target_id.clone(),
            relation_kind: seed.relation_kind,
            object_ref: seed.captured_object_ref.clone(),
            anchor_ref: seed.captured_anchor_ref.clone(),
            scope_ref: seed.captured_scope_ref.clone(),
        }),
        ContinuityState::Remapped => Some(RelationTargetSnapshot {
            target_id: format!("{}::remapped", seed.captured_target_id),
            // The relation kind is preserved across a remap; a remap never relabels it.
            relation_kind: seed.relation_kind,
            object_ref: seed
                .remapped_object_ref
                .clone()
                .unwrap_or_else(|| seed.captured_object_ref.clone()),
            anchor_ref: seed
                .remapped_anchor_ref
                .clone()
                .unwrap_or_else(|| seed.captured_anchor_ref.clone()),
            scope_ref: seed.captured_scope_ref.clone(),
        }),
        _ => None,
    };

    let drift_reason = if seed.drift_state == ContinuityState::Bound {
        None
    } else {
        Some(
            seed.drift_reason
                .clone()
                .unwrap_or_else(|| default_drift_reason(seed.drift_state, seed.relation_kind)),
        )
    };

    let recovery_choices = recovery_choices_for(seed);
    let labels = labels_for(seed, captured_only);
    let downgrade_reasons = entry_downgrade_reasons(seed);
    let fallback_notes = entry_fallback_notes(seed, captured_only);

    let return_anchor = ReturnAnchor {
        return_anchor_ref: format!("aureline://return/{}", seed.entry_id),
        origin_surface: seed.origin_surface,
        origin_object_ref: seed.origin_object_ref.clone(),
        origin_scope_ref: seed.origin_scope_ref.clone(),
        restores_selection: true,
        restores_viewport: true,
    };

    let captured_target = RelationTargetSnapshot {
        target_id: seed.captured_target_id.clone(),
        relation_kind: seed.relation_kind,
        object_ref: seed.captured_object_ref.clone(),
        anchor_ref: seed.captured_anchor_ref.clone(),
        scope_ref: seed.captured_scope_ref.clone(),
    };

    let summary = format!(
        "{} via {} ({}): {} — {} scope, evidence {}{}.",
        seed.entry_kind.label(),
        seed.relation_kind.as_str(),
        seed.origin_surface.as_str(),
        drift_token(seed.drift_state),
        if current_scope { "current" } else { "captured" },
        evidence_class.as_str(),
        if auto_open_allowed {
            String::new()
        } else {
            "; disclosed before any jump".to_owned()
        },
    );

    RelationNavigationEntry {
        entry_id: seed.entry_id.clone(),
        entry_kind: seed.entry_kind,
        relation_kind: seed.relation_kind,
        return_anchor,
        captured_target,
        current_target,
        drift_state: seed.drift_state,
        drift_reason,
        remap_evidence_refs: seed.remap_evidence_refs.clone(),
        used_nearby_fallback: false,
        auto_open_allowed,
        proof_class: seed.proof_class,
        evidence_class,
        provider_class: seed.provider_class,
        confidence: seed.confidence,
        freshness: seed.freshness,
        ambiguity_class: seed.ambiguity_class,
        scope_completeness: seed.scope_completeness,
        current_scope,
        disambiguation_set_ref: seed.disambiguation_set_ref.clone(),
        replay_target_id: format!("aureline://replay/relation-nav/{}", seed.entry_id),
        recovery_choices,
        labels,
        downgrade_reasons,
        fallback_notes,
        evidence_refs: seed.evidence_refs.clone(),
        summary,
    }
}

fn build_rename_evidence(seed: &RenamePreviewEvidenceInput) -> RenamePreviewEvidence {
    let evidence_class = RelationContinuityEvidenceClass::from_proof(seed.proof_class);
    let drift_reason =
        if seed.drift_state == ContinuityState::Bound {
            None
        } else {
            Some(seed.drift_reason.clone().unwrap_or_else(|| {
                default_drift_reason(seed.drift_state, RelationKind::Definition)
            }))
        };
    let mut fallback_notes = Vec::new();
    if let Some(note) = fallback_note_for_proof(seed.proof_class) {
        fallback_notes.push(note);
    }
    let return_anchor = ReturnAnchor {
        return_anchor_ref: format!("aureline://return/rename/{}", seed.evidence_id),
        origin_surface: seed.origin_surface,
        origin_object_ref: seed.origin_object_ref.clone(),
        origin_scope_ref: seed.origin_scope_ref.clone(),
        restores_selection: true,
        restores_viewport: true,
    };
    let summary = format!(
        "Rename evidence for a definition root ({}): posture {}, {} changed / {} held, drift {}, \
         evidence {} — replayable.",
        seed.origin_surface.as_str(),
        apply_posture_token(seed.apply_posture),
        seed.changed_count,
        seed.held_count,
        drift_token(seed.drift_state),
        evidence_class.as_str(),
    );
    RenamePreviewEvidence {
        evidence_id: seed.evidence_id.clone(),
        rename_preview_ref: seed.rename_preview_ref.clone(),
        root_relation_kind: RelationKind::Definition,
        return_anchor,
        evidence_class,
        apply_posture: seed.apply_posture,
        ambiguity_class: seed.ambiguity_class,
        drift_state: seed.drift_state,
        drift_reason,
        changed_count: seed.changed_count,
        held_count: seed.held_count,
        disambiguation_set_ref: seed.disambiguation_set_ref.clone(),
        replay_target_id: format!("aureline://replay/rename/{}", seed.evidence_id),
        downgrade_reasons: seed.downgrade_reasons.clone(),
        fallback_notes,
        evidence_refs: seed.evidence_refs.clone(),
        summary,
    }
}

fn build_projection(
    surface: ConsumerSurface,
    entry_count: usize,
    rename_evidence_count: usize,
    redaction_class: ExportRedactionClass,
) -> RelationContinuityProjection {
    RelationContinuityProjection {
        consumer_surface: surface,
        projected_entry_count: entry_count,
        projected_rename_evidence_count: rename_evidence_count,
        preserves_relation_kind: true,
        preserves_origin_surface: true,
        preserves_return_anchor: true,
        preserves_current_vs_captured: true,
        preserves_drift_state: true,
        preserves_fallback_class: true,
        preserves_ambiguity_state: true,
        preserves_replay_target_ids: true,
        silently_retargets: false,
        exports_code_bodies: false,
        redaction_class,
        summary: format!(
            "{} consumes the packet with relation kind, origin surface, return anchor, \
             current-versus-captured truth, drift state, fallback class, ambiguity, and replay ids \
             preserved; never silently retargets and never exports code bodies.",
            surface.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Derivations.
// ---------------------------------------------------------------------------

fn recovery_choices_for(seed: &RelationNavEntryInput) -> Vec<RelationRecoveryChoice> {
    let mut out = Vec::new();
    match seed.drift_state {
        ContinuityState::Bound => {}
        ContinuityState::Remapped => {
            out.push(RelationRecoveryChoice::OpenRemappedTarget);
            out.push(RelationRecoveryChoice::RestoreReturnAnchorOnly);
        }
        ContinuityState::Drifted => {
            out.push(RelationRecoveryChoice::InspectDrift);
            out.push(RelationRecoveryChoice::RestoreReturnAnchorOnly);
        }
        ContinuityState::MissingTarget => {
            out.push(RelationRecoveryChoice::InspectDrift);
            out.push(RelationRecoveryChoice::RestoreReturnAnchorOnly);
        }
        ContinuityState::ScopeUnavailable => {
            out.push(RelationRecoveryChoice::WidenScope);
            out.push(RelationRecoveryChoice::RestoreReturnAnchorOnly);
        }
        ContinuityState::Archived => {
            out.push(RelationRecoveryChoice::KeepArchivedReference);
            out.push(RelationRecoveryChoice::RestoreReturnAnchorOnly);
        }
    }
    if seed.ambiguity_class.requires_disambiguation() {
        push_unique(&mut out, RelationRecoveryChoice::ChooseFromDisambiguation);
    }
    if needs_provider_refresh(seed) {
        push_unique(&mut out, RelationRecoveryChoice::RefreshProvider);
    }
    out
}

fn needs_provider_refresh(seed: &RelationNavEntryInput) -> bool {
    matches!(
        seed.freshness,
        FreshnessClass::DegradedCached | FreshnessClass::Stale | FreshnessClass::Unverified
    ) || seed.scope_completeness == ScopeCompleteness::StaleForDeclaredScope
        || seed.downgrade_reasons.iter().any(|reason| {
            matches!(
                reason,
                DowngradeReason::MissingProvider
                    | DowngradeReason::ProviderUnavailable
                    | DowngradeReason::StaleShard
            )
        })
}

fn labels_for(seed: &RelationNavEntryInput, captured_only: bool) -> Vec<RelationContinuityLabel> {
    let mut labels = Vec::new();
    match seed.proof_class {
        ProofClass::ImportedEvidence => {
            push_unique(&mut labels, RelationContinuityLabel::ImportedSnapshot)
        }
        ProofClass::RuntimeObserved => {
            push_unique(&mut labels, RelationContinuityLabel::RuntimeObserved)
        }
        ProofClass::FrameworkDerived => {
            push_unique(&mut labels, RelationContinuityLabel::FrameworkDerived)
        }
        ProofClass::LexicalFallback => {
            push_unique(&mut labels, RelationContinuityLabel::LexicalFallback)
        }
        ProofClass::SyntaxFallback => {
            push_unique(&mut labels, RelationContinuityLabel::SyntaxFallback)
        }
        _ => {}
    }
    match seed.generated_or_external_state {
        GeneratedOrExternalState::GeneratedSource => {
            push_unique(&mut labels, RelationContinuityLabel::Generated)
        }
        GeneratedOrExternalState::ExternalDependency => {
            push_unique(&mut labels, RelationContinuityLabel::External)
        }
        GeneratedOrExternalState::ReadOnlySource => {
            push_unique(&mut labels, RelationContinuityLabel::ReadOnly)
        }
        GeneratedOrExternalState::ImportedSnapshot => {
            push_unique(&mut labels, RelationContinuityLabel::ImportedSnapshot)
        }
        GeneratedOrExternalState::AuthoredSource => {}
    }
    if seed.access_kind == AccessKind::TestOnly {
        push_unique(&mut labels, RelationContinuityLabel::TestOnly);
    }
    match seed.drift_state {
        ContinuityState::Bound => {}
        ContinuityState::Remapped => push_unique(&mut labels, RelationContinuityLabel::Remapped),
        ContinuityState::Drifted => push_unique(&mut labels, RelationContinuityLabel::Drifted),
        ContinuityState::MissingTarget => {
            push_unique(&mut labels, RelationContinuityLabel::MissingTarget)
        }
        ContinuityState::ScopeUnavailable => {
            push_unique(&mut labels, RelationContinuityLabel::ScopeUnavailable)
        }
        ContinuityState::Archived => push_unique(&mut labels, RelationContinuityLabel::Archived),
    }
    if seed.ambiguity_class.requires_disambiguation() {
        push_unique(
            &mut labels,
            RelationContinuityLabel::AmbiguousNeedsSelection,
        );
    }
    if captured_only {
        push_unique(&mut labels, RelationContinuityLabel::CapturedScopeOnly);
    }
    labels
}

fn entry_downgrade_reasons(seed: &RelationNavEntryInput) -> Vec<DowngradeReason> {
    let mut reasons = seed.downgrade_reasons.clone();
    match seed.proof_class {
        ProofClass::LexicalFallback => {
            push_unique(&mut reasons, DowngradeReason::LexicalFallbackOnly)
        }
        ProofClass::SyntaxFallback => {
            push_unique(&mut reasons, DowngradeReason::SyntaxFallbackOnly)
        }
        ProofClass::RuntimeObserved | ProofClass::FrameworkDerived => {
            push_unique(&mut reasons, DowngradeReason::RuntimeOrFrameworkOnly)
        }
        _ => {}
    }
    if seed.generated_or_external_state == GeneratedOrExternalState::GeneratedSource {
        push_unique(&mut reasons, DowngradeReason::GeneratedBoundary);
    }
    match seed.drift_state {
        ContinuityState::Drifted
        | ContinuityState::MissingTarget
        | ContinuityState::Remapped
        | ContinuityState::Archived => {
            push_unique(&mut reasons, DowngradeReason::BookmarkOrHistoryDrift)
        }
        ContinuityState::ScopeUnavailable => {
            push_unique(&mut reasons, DowngradeReason::SparseWorkset)
        }
        ContinuityState::Bound => {}
    }
    if seed.ambiguity_class.requires_disambiguation() {
        push_unique(&mut reasons, DowngradeReason::AmbiguousCandidates);
    }
    reasons
}

fn entry_fallback_notes(seed: &RelationNavEntryInput, captured_only: bool) -> Vec<String> {
    let mut notes = Vec::new();
    if let Some(note) = fallback_note_for_proof(seed.proof_class) {
        notes.push(note);
    }
    // A captured-only entry whose proof carries no fallback note — a bound semantic entry
    // gone stale, or a drifted/missing/scope-unavailable target — still discloses that it
    // is carried only by a captured scope.
    if captured_only && notes.is_empty() {
        notes.push(
            "Carried only by a captured scope; not re-proven against current source.".to_owned(),
        );
    }
    notes
}

fn fallback_note_for_proof(proof: ProofClass) -> Option<String> {
    match proof {
        ProofClass::LexicalFallback => Some(
            "Rests on a lexical/grep fallback; disclosed as such, never replayed as semantic certainty."
                .to_owned(),
        ),
        ProofClass::SyntaxFallback => {
            Some("Rests on a syntax-only fallback and stays labeled as a fallback.".to_owned())
        }
        ProofClass::RuntimeObserved => {
            Some("Runtime-observed from a captured trace, not static source.".to_owned())
        }
        ProofClass::FrameworkDerived => {
            Some("Framework-derived from route/generator metadata.".to_owned())
        }
        ProofClass::ImportedEvidence => {
            Some("From an imported snapshot and captured-scope only.".to_owned())
        }
        _ => None,
    }
}

fn covered_drift_states(entries: &[RelationNavigationEntry]) -> Vec<ContinuityState> {
    let mut out = Vec::new();
    for state in RELATION_CONTINUITY_DRIFT_STATES {
        if entries.iter().any(|entry| entry.drift_state == state) {
            out.push(state);
        }
    }
    out
}

fn packet_downgrade_reasons(
    entries: &[RelationNavigationEntry],
    rename_evidence: &[RenamePreviewEvidence],
) -> Vec<DowngradeReason> {
    let mut reasons = Vec::new();
    for entry in entries {
        for reason in &entry.downgrade_reasons {
            push_unique(&mut reasons, *reason);
        }
    }
    for evidence in rename_evidence {
        for reason in &evidence.downgrade_reasons {
            push_unique(&mut reasons, *reason);
        }
    }
    reasons
}

fn packet_fallback_notes(
    entries: &[RelationNavigationEntry],
    rename_evidence: &[RenamePreviewEvidence],
) -> Vec<String> {
    let mut notes = Vec::new();
    for entry in entries {
        for note in &entry.fallback_notes {
            push_unique_string(&mut notes, note);
        }
    }
    for evidence in rename_evidence {
        for note in &evidence.fallback_notes {
            push_unique_string(&mut notes, note);
        }
    }
    notes
}

fn default_drift_reason(state: ContinuityState, relation_kind: RelationKind) -> String {
    let relation = relation_kind.as_str();
    match state {
        ContinuityState::Bound => format!("{relation} target still resolves exactly."),
        ContinuityState::Remapped => {
            format!("{relation} target moved; remapped via stable evidence, not a nearby guess.")
        }
        ContinuityState::Drifted => {
            format!("{relation} target drifted and must be inspected before a jump.")
        }
        ContinuityState::MissingTarget => {
            format!("{relation} target is missing from the current scope.")
        }
        ContinuityState::ScopeUnavailable => {
            format!("{relation} target is outside the active workset, branch, or shard.")
        }
        ContinuityState::Archived => {
            format!("{relation} target is retained only as an archived reference.")
        }
    }
}

fn drift_token(state: ContinuityState) -> &'static str {
    match state {
        ContinuityState::Bound => "bound",
        ContinuityState::Remapped => "remapped",
        ContinuityState::Drifted => "drifted",
        ContinuityState::MissingTarget => "missing_target",
        ContinuityState::ScopeUnavailable => "scope_unavailable",
        ContinuityState::Archived => "archived",
    }
}

fn apply_posture_token(posture: RenameApplyPosture) -> &'static str {
    match posture {
        RenameApplyPosture::ReadyForApplyAfterPreview => "ready_for_apply_after_preview",
        RenameApplyPosture::BlockedPendingScopeReview => "blocked_pending_scope_review",
        RenameApplyPosture::BlockedPendingRefresh => "blocked_pending_refresh",
        RenameApplyPosture::BlockedPendingPolicyOrProtectedReview => {
            "blocked_pending_policy_or_protected_review"
        }
        RenameApplyPosture::InspectOnlyUnavailable => "inspect_only_unavailable",
    }
}

fn push_unique<T: PartialEq>(items: &mut Vec<T>, item: T) {
    if !items.contains(&item) {
        items.push(item);
    }
}

fn push_unique_string(items: &mut Vec<String>, item: &str) {
    if !items.iter().any(|existing| existing == item) {
        items.push(item.to_owned());
    }
}

// ---------------------------------------------------------------------------
// Frozen corpus.
// ---------------------------------------------------------------------------

/// One frozen scenario: an input, the packet the builder produces for it, and the
/// property the scenario proves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationContinuityScenario {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Plain-language title.
    pub title: String,
    /// The packet-building input.
    pub input: RelationContinuityInput,
    /// The packet `build_relation_continuity_packet` produces for the input.
    pub packet: RelationContinuityPacket,
    /// One reviewable sentence stating what the scenario proves.
    pub expectation_note: String,
}

/// One frozen invariant over the corpus, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationContinuityInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built corpus satisfies the invariant.
    pub holds: bool,
}

/// The frozen relation-continuity corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationContinuitySet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub relation_continuity_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable corpus id.
    pub set_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The freeze gate that keeps the corpus binding current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the corpus.
    pub summary: String,
    /// The frozen scenarios.
    pub scenarios: Vec<RelationContinuityScenario>,
    /// The computed invariants.
    pub invariants: Vec<RelationContinuityInvariant>,
    /// Whether raw source bodies and payloads are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the corpus fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationContinuityValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for RelationContinuityValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "relation-continuity corpus invalid: {}", self.reason)
    }
}

impl std::error::Error for RelationContinuityValidationError {}

impl RelationContinuitySet {
    /// Returns the scenario with a given id, if present.
    pub fn scenario(&self, scenario_id: &str) -> Option<&RelationContinuityScenario> {
        self.scenarios
            .iter()
            .find(|scenario| scenario.scenario_id == scenario_id)
    }

    /// Returns true when every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the corpus is safe to place in a support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded && self.all_refs().into_iter().all(is_export_safe_ref)
    }

    fn all_refs(&self) -> Vec<&str> {
        let mut refs: Vec<&str> = vec![self.schema_ref.as_str(), self.freeze_gate_ref.as_str()];
        for scenario in &self.scenarios {
            let packet = &scenario.packet;
            refs.push(packet.scope_ref.as_str());
            if let Some(captured) = &packet.captured_scope_ref {
                refs.push(captured.as_str());
            }
            for entry in &packet.entries {
                refs.push(entry.return_anchor.return_anchor_ref.as_str());
                refs.push(entry.return_anchor.origin_object_ref.as_str());
                refs.push(entry.return_anchor.origin_scope_ref.as_str());
                refs.push(entry.captured_target.target_id.as_str());
                refs.push(entry.captured_target.object_ref.as_str());
                refs.push(entry.captured_target.anchor_ref.as_str());
                refs.push(entry.captured_target.scope_ref.as_str());
                refs.push(entry.replay_target_id.as_str());
                if let Some(current) = &entry.current_target {
                    refs.push(current.target_id.as_str());
                    refs.push(current.object_ref.as_str());
                    refs.push(current.anchor_ref.as_str());
                }
                if let Some(set) = &entry.disambiguation_set_ref {
                    refs.push(set.as_str());
                }
                refs.extend(entry.remap_evidence_refs.iter().map(String::as_str));
                refs.extend(entry.evidence_refs.iter().map(String::as_str));
            }
            for evidence in &packet.rename_evidence {
                refs.push(evidence.rename_preview_ref.as_str());
                refs.push(evidence.return_anchor.return_anchor_ref.as_str());
                refs.push(evidence.return_anchor.origin_object_ref.as_str());
                refs.push(evidence.return_anchor.origin_scope_ref.as_str());
                refs.push(evidence.replay_target_id.as_str());
                if let Some(set) = &evidence.disambiguation_set_ref {
                    refs.push(set.as_str());
                }
                refs.extend(evidence.evidence_refs.iter().map(String::as_str));
            }
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), RelationContinuityValidationError> {
        let fail = |reason: String| Err(RelationContinuityValidationError { reason });

        if self.record_kind != RELATION_CONTINUITY_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != RELATION_CONTINUITY_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.scenarios.is_empty() {
            return fail("corpus must carry at least one scenario".to_owned());
        }
        if !all_unique(self.scenarios.iter().map(|s| s.scenario_id.as_str())) {
            return fail("scenario ids are not unique".to_owned());
        }

        // Every scenario's stored packet equals what the builder produces, so the fixture
        // cannot drift from the builder.
        for scenario in &self.scenarios {
            let produced = build_relation_continuity_packet(&scenario.input);
            if produced != scenario.packet {
                return fail(format!(
                    "scenario {} packet drifted from builder output",
                    scenario.scenario_id
                ));
            }
        }

        if !self.is_support_export_safe() {
            return fail("corpus is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|invariant| !invariant.holds)
                .map(|invariant| invariant.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

/// Builds the canonical relation-continuity corpus.
///
/// Deterministic: the same bytes every call. Each scenario's packet is the builder's own
/// output, and the invariant `holds` flags are computed from those packets, so a
/// regression in [`build_relation_continuity_packet`] flips an invariant or drifts the
/// fixture rather than silently passing.
pub fn relation_continuity_set() -> RelationContinuitySet {
    let scenarios = build_scenarios();
    let invariants = compute_invariants(&scenarios);

    RelationContinuitySet {
        record_kind: RELATION_CONTINUITY_RECORD_KIND.to_owned(),
        relation_continuity_schema_version: RELATION_CONTINUITY_SCHEMA_VERSION,
        schema_ref: RELATION_CONTINUITY_SCHEMA_REF.to_owned(),
        set_id: RELATION_CONTINUITY_SET_ID.to_owned(),
        as_of: RELATION_CONTINUITY_AS_OF.to_owned(),
        freeze_gate_ref: RELATION_CONTINUITY_FREEZE_GATE_REF.to_owned(),
        summary: "Frozen relation-continuity corpus: every peek, temporary reveal, open-in-split, \
                  back/forward-history, and recent-location entry preserves its relation kind, origin \
                  surface, return anchor, and current-versus-captured target truth; a remapped, \
                  drifted, missing-target, scope-unavailable, or archived entry keeps its drift state, \
                  reason, and recovery choices visible and never silently jumps to a nearby guess; \
                  every entry and rename-evidence row names its evidence class so a grep fallback is \
                  never replayed as semantic, and carries a replay-safe target id; and every consumer \
                  surface preserves that truth without retargeting or exporting code bodies."
            .to_owned(),
        scenarios,
        invariants,
        raw_payload_excluded: true,
    }
}

/// Renders the corpus as human-readable lines for CLI/headless and support.
pub fn relation_continuity_lines(set: &RelationContinuitySet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Relation-continuity corpus — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Scenarios: {}  Invariants: {}",
        set.scenarios.len(),
        set.invariants.len()
    ));

    lines.push("Scenarios:".to_owned());
    for scenario in &set.scenarios {
        let packet = &scenario.packet;
        lines.push(format!("  - {} [{}]", scenario.scenario_id, scenario.title));
        lines.push(format!(
            "      entries={} current={} captured={} disclose={} rename_evidence={} drift_states={}",
            packet.counts.total_count,
            packet.counts.current_scope_count,
            packet.counts.captured_scope_count,
            packet.counts.requires_disclosure_count,
            packet.rename_evidence.len(),
            packet.covered_drift_states.len(),
        ));
        for entry in &packet.entries {
            lines.push(format!(
                "      · {} {}/{} drift={} {} auto_open={} replay={}",
                entry.entry_kind.as_str(),
                entry.relation_kind.as_str(),
                entry.evidence_class.as_str(),
                drift_token(entry.drift_state),
                if entry.current_scope {
                    "current"
                } else {
                    "captured"
                },
                entry.auto_open_allowed,
                entry.replay_target_id,
            ));
        }
        for evidence in &packet.rename_evidence {
            lines.push(format!(
                "      ⟳ rename {} posture={} drift={} changed={} held={} replay={}",
                evidence.evidence_class.as_str(),
                apply_posture_token(evidence.apply_posture),
                drift_token(evidence.drift_state),
                evidence.changed_count,
                evidence.held_count,
                evidence.replay_target_id,
            ));
        }
    }

    lines.push("Invariants:".to_owned());
    for invariant in &set.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if invariant.holds { "ok" } else { "FAIL" },
            invariant.invariant_id
        ));
    }

    lines
}

// ---------------------------------------------------------------------------
// Scenario builders.
// ---------------------------------------------------------------------------

/// Compact seed for a [`RelationNavEntryInput`], so each scenario reads as a small table.
struct EntrySeed {
    entry_id: &'static str,
    kind: RelationNavEntryKind,
    relation: RelationKind,
    origin: ConsumerSurface,
    drift: ContinuityState,
    access: AccessKind,
    proof: ProofClass,
    provider: ProviderClass,
    confidence: NavigationConfidence,
    freshness: FreshnessClass,
    ambiguity: AmbiguityClass,
    scope: ScopeCompleteness,
    generated: GeneratedOrExternalState,
    remapped: bool,
    disambiguation: Option<&'static str>,
    downgrades: &'static [DowngradeReason],
    summary: &'static str,
}

fn entry(seed: EntrySeed) -> RelationNavEntryInput {
    let remapped = seed.drift == ContinuityState::Remapped || seed.remapped;
    RelationNavEntryInput {
        entry_id: seed.entry_id.to_owned(),
        entry_kind: seed.kind,
        relation_kind: seed.relation,
        origin_surface: seed.origin,
        origin_object_ref: format!("aureline://object/origin/{}", seed.entry_id),
        origin_scope_ref: "aureline://scope/workspace".to_owned(),
        captured_target_id: format!("aureline://target/{}", seed.entry_id),
        captured_object_ref: format!("aureline://object/target/{}", seed.entry_id),
        captured_anchor_ref: format!("aureline://anchor/{}", seed.entry_id),
        captured_scope_ref: "aureline://scope/workspace".to_owned(),
        drift_state: seed.drift,
        remapped_object_ref: remapped
            .then(|| format!("aureline://object/target/{}/moved", seed.entry_id)),
        remapped_anchor_ref: remapped.then(|| format!("aureline://anchor/{}/moved", seed.entry_id)),
        remap_evidence_refs: if seed.drift == ContinuityState::Remapped {
            vec![format!(
                "aureline://evidence/remap/{}/stable-symbol-id",
                seed.entry_id
            )]
        } else {
            Vec::new()
        },
        access_kind: seed.access,
        proof_class: seed.proof,
        provider_class: seed.provider,
        confidence: seed.confidence,
        freshness: seed.freshness,
        ambiguity_class: seed.ambiguity,
        scope_completeness: seed.scope,
        generated_or_external_state: seed.generated,
        disambiguation_set_ref: seed.disambiguation.map(str::to_owned),
        drift_reason: None,
        downgrade_reasons: seed.downgrades.to_vec(),
        evidence_refs: vec![format!("aureline://evidence/{}", seed.entry_id)],
        summary: seed.summary.to_owned(),
    }
}

/// Compact seed for a [`RenamePreviewEvidenceInput`].
struct RenameSeed {
    evidence_id: &'static str,
    origin: ConsumerSurface,
    proof: ProofClass,
    posture: RenameApplyPosture,
    ambiguity: AmbiguityClass,
    drift: ContinuityState,
    changed: usize,
    held: usize,
    disambiguation: Option<&'static str>,
    downgrades: &'static [DowngradeReason],
    summary: &'static str,
}

fn rename_evidence(seed: RenameSeed) -> RenamePreviewEvidenceInput {
    RenamePreviewEvidenceInput {
        evidence_id: seed.evidence_id.to_owned(),
        rename_preview_ref: format!("aureline://rename-preview/{}", seed.evidence_id),
        origin_surface: seed.origin,
        origin_object_ref: format!("aureline://object/origin/{}", seed.evidence_id),
        origin_scope_ref: "aureline://scope/workspace".to_owned(),
        proof_class: seed.proof,
        apply_posture: seed.posture,
        ambiguity_class: seed.ambiguity,
        drift_state: seed.drift,
        changed_count: seed.changed,
        held_count: seed.held,
        disambiguation_set_ref: seed.disambiguation.map(str::to_owned),
        drift_reason: None,
        downgrade_reasons: seed.downgrades.to_vec(),
        evidence_refs: vec![format!("aureline://evidence/rename/{}", seed.evidence_id)],
        summary: seed.summary.to_owned(),
    }
}

fn packet_input(
    packet_id: &str,
    origin: ConsumerSurface,
    captured_scope_ref: Option<&str>,
    redaction_class: ExportRedactionClass,
    entries: Vec<RelationNavEntryInput>,
    rename_evidence: Vec<RenamePreviewEvidenceInput>,
) -> RelationContinuityInput {
    RelationContinuityInput {
        packet_id: packet_id.to_owned(),
        origin_surface: origin,
        scope_ref: "aureline://scope/workspace".to_owned(),
        captured_scope_ref: captured_scope_ref.map(str::to_owned),
        redaction_class,
        entries,
        rename_evidence,
    }
}

fn scenario(
    scenario_id: &str,
    title: &str,
    input: RelationContinuityInput,
    expectation_note: &str,
) -> RelationContinuityScenario {
    let packet = build_relation_continuity_packet(&input);
    RelationContinuityScenario {
        scenario_id: scenario_id.to_owned(),
        title: title.to_owned(),
        input,
        packet,
        expectation_note: expectation_note.to_owned(),
    }
}

fn build_scenarios() -> Vec<RelationContinuityScenario> {
    use AccessKind::*;
    use ConsumerSurface::*;
    use ContinuityState::*;
    use ExportRedactionClass::*;
    use ProofClass::*;
    use ProviderClass::*;
    use RelationKind::{Declaration, Definition, Implementation, Reference, RouteBinding, Type};
    use ScopeCompleteness::*;

    let authored = GeneratedOrExternalState::AuthoredSource;

    vec![
        // 1. Bound peek/reveal/split keep relation kind and return context, auto-open.
        scenario(
            "continuity.bound_peek_reveal_split",
            "Bound peek, reveal, and split preserve relation kind and return context",
            packet_input(
                "continuity:peek:0001",
                EditorUi,
                None,
                MetadataSafeDefault,
                vec![
                    entry(EntrySeed {
                        entry_id: "entry.peek.definition",
                        kind: RelationNavEntryKind::Peek,
                        relation: Definition,
                        origin: EditorUi,
                        drift: Bound,
                        access: Read,
                        proof: DirectSemantic,
                        provider: LanguageServer,
                        confidence: NavigationConfidence::Exact,
                        freshness: FreshnessClass::AuthoritativeLive,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: CompleteForDeclaredScope,
                        generated: authored,
                        remapped: false,
                        disambiguation: None,
                        downgrades: &[],
                        summary: "Peek the definition from the editor.",
                    }),
                    entry(EntrySeed {
                        entry_id: "entry.reveal.reference",
                        kind: RelationNavEntryKind::TemporaryReveal,
                        relation: Reference,
                        origin: EditorUi,
                        drift: Bound,
                        access: Read,
                        proof: IndexedSemantic,
                        provider: ProjectGraph,
                        confidence: NavigationConfidence::Indexed,
                        freshness: FreshnessClass::WarmCached,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: CompleteForDeclaredScope,
                        generated: authored,
                        remapped: false,
                        disambiguation: None,
                        downgrades: &[],
                        summary: "Temporarily reveal a reference.",
                    }),
                    entry(EntrySeed {
                        entry_id: "entry.split.implementation",
                        kind: RelationNavEntryKind::OpenInSplit,
                        relation: Implementation,
                        origin: EditorUi,
                        drift: Bound,
                        access: Read,
                        proof: DirectSemantic,
                        provider: LanguageServer,
                        confidence: NavigationConfidence::Exact,
                        freshness: FreshnessClass::AuthoritativeLive,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: CompleteForDeclaredScope,
                        generated: authored,
                        remapped: false,
                        disambiguation: None,
                        downgrades: &[],
                        summary: "Open the implementation in a split.",
                    }),
                ],
                vec![rename_evidence(RenameSeed {
                    evidence_id: "rename.bound.ready",
                    origin: EditorUi,
                    proof: DirectSemantic,
                    posture: RenameApplyPosture::ReadyForApplyAfterPreview,
                    ambiguity: AmbiguityClass::Unambiguous,
                    drift: Bound,
                    changed: 4,
                    held: 0,
                    disambiguation: None,
                    downgrades: &[],
                    summary: "A clean rename preview captured beside the navigation.",
                })],
            ),
            "Three bound entries on temporary surfaces keep their relation kind, origin, and return \
             anchor, all resolve current-scope and auto-open, and a bound rename preview rides along \
             as replayable evidence.",
        ),
        // 2. Remapped history preserves relation and cites stable evidence — no silent jump.
        scenario(
            "continuity.remapped_history",
            "Remapped history preserves relation and cites stable evidence",
            packet_input(
                "continuity:history:0002",
                EditorUi,
                Some("aureline://scope/captured-index"),
                MetadataSafeDefault,
                vec![
                    entry(EntrySeed {
                        entry_id: "entry.back.definition.remapped",
                        kind: RelationNavEntryKind::BackHistory,
                        relation: Definition,
                        origin: EditorUi,
                        drift: Remapped,
                        access: Read,
                        proof: IndexedSemantic,
                        provider: ProjectGraph,
                        confidence: NavigationConfidence::Indexed,
                        freshness: FreshnessClass::WarmCached,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: CompleteForDeclaredScope,
                        generated: authored,
                        remapped: true,
                        disambiguation: None,
                        downgrades: &[],
                        summary: "Back to a definition that moved but remapped cleanly.",
                    }),
                    entry(EntrySeed {
                        entry_id: "entry.forward.reference.remapped",
                        kind: RelationNavEntryKind::ForwardHistory,
                        relation: Reference,
                        origin: ShellContinuity,
                        drift: Remapped,
                        access: Write,
                        proof: IndexedSemantic,
                        provider: ProjectGraph,
                        confidence: NavigationConfidence::Indexed,
                        freshness: FreshnessClass::WarmCached,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: CompleteForDeclaredScope,
                        generated: authored,
                        remapped: true,
                        disambiguation: None,
                        downgrades: &[],
                        summary: "Forward to a reference that moved but remapped cleanly.",
                    }),
                    entry(EntrySeed {
                        entry_id: "entry.recent.type.imported",
                        kind: RelationNavEntryKind::RecentLocation,
                        relation: Type,
                        origin: GraphOverlay,
                        drift: Bound,
                        access: Read,
                        proof: ImportedEvidence,
                        provider: ProviderClass::ImportedSnapshot,
                        confidence: NavigationConfidence::Imported,
                        freshness: FreshnessClass::WarmCached,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: CompleteForDeclaredScope,
                        generated: GeneratedOrExternalState::ImportedSnapshot,
                        remapped: false,
                        disambiguation: None,
                        downgrades: &[],
                        summary: "Recent type location proven only by an imported snapshot.",
                    }),
                ],
                Vec::new(),
            ),
            "Two remapped history entries keep their relation kind, cite stable remap evidence with no \
             nearby fallback, and offer an explicit open action instead of jumping; an imported-snapshot \
             recent location is captured-scope only — so current-versus-captured counts split.",
        ),
        // 3. Drifted / missing / scope-unavailable stay visible with reasons and recovery.
        scenario(
            "continuity.drifted_missing_scope",
            "Drifted, missing, and scope-unavailable entries stay visible before any jump",
            packet_input(
                "continuity:drift:0003",
                ReviewWorkspace,
                Some("aureline://scope/captured-branch"),
                InternalSupportRestricted,
                vec![
                    entry(EntrySeed {
                        entry_id: "entry.peek.reference.scope_unavailable",
                        kind: RelationNavEntryKind::Peek,
                        relation: Reference,
                        origin: ReviewWorkspace,
                        drift: ScopeUnavailable,
                        access: Read,
                        proof: IndexedSemantic,
                        provider: RemoteIndex,
                        confidence: NavigationConfidence::WorkspaceSliceLimited,
                        freshness: FreshnessClass::WarmCached,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: PartialForDeclaredScope,
                        generated: authored,
                        remapped: false,
                        disambiguation: None,
                        downgrades: &[DowngradeReason::SparseWorkset],
                        summary: "Peek a reference outside the active workset.",
                    }),
                    entry(EntrySeed {
                        entry_id: "entry.back.implementation.missing",
                        kind: RelationNavEntryKind::BackHistory,
                        relation: Implementation,
                        origin: EditorUi,
                        drift: MissingTarget,
                        access: Read,
                        proof: IndexedSemantic,
                        provider: ProjectGraph,
                        confidence: NavigationConfidence::Unavailable,
                        freshness: FreshnessClass::Stale,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: UnavailableForDeclaredScope,
                        generated: authored,
                        remapped: false,
                        disambiguation: None,
                        downgrades: &[DowngradeReason::StaleShard],
                        summary: "Back to an implementation that no longer resolves.",
                    }),
                    entry(EntrySeed {
                        entry_id: "entry.recent.definition.drifted",
                        kind: RelationNavEntryKind::RecentLocation,
                        relation: Definition,
                        origin: GraphOverlay,
                        drift: Drifted,
                        access: Read,
                        proof: IndexedSemantic,
                        provider: ProjectGraph,
                        confidence: NavigationConfidence::Stale,
                        freshness: FreshnessClass::DegradedCached,
                        ambiguity: AmbiguityClass::DriftedNeedsReview,
                        scope: StaleForDeclaredScope,
                        generated: authored,
                        remapped: false,
                        disambiguation: Some("aureline://disambiguation/drifted-definition"),
                        downgrades: &[DowngradeReason::BookmarkOrHistoryDrift],
                        summary: "Recent definition that drifted and needs review.",
                    }),
                ],
                Vec::new(),
            ),
            "A scope-unavailable peek, a missing-target back entry, and a drifted recent location each \
             carry no current target, keep a visible drift reason and recovery choices, route the \
             drifted entry's ambiguity through a disambiguation path, and never auto-open — so the \
             state shows before any jump.",
        ),
        // 4. Fallback / runtime / framework evidence is disclosed, never replayed as semantic.
        scenario(
            "continuity.fallback_runtime_framework",
            "Lexical, runtime, and framework evidence is disclosed, never replayed as semantic",
            packet_input(
                "continuity:fallback:0004",
                AiContext,
                Some("aureline://scope/captured-trace"),
                MetadataSafeDefault,
                vec![
                    entry(EntrySeed {
                        entry_id: "entry.peek.call.lexical",
                        kind: RelationNavEntryKind::Peek,
                        relation: RelationKind::Call,
                        origin: AiContext,
                        drift: Bound,
                        access: Call,
                        proof: LexicalFallback,
                        provider: Syntax,
                        confidence: NavigationConfidence::Heuristic,
                        freshness: FreshnessClass::WarmCached,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: PartialForDeclaredScope,
                        generated: authored,
                        remapped: false,
                        disambiguation: None,
                        downgrades: &[DowngradeReason::LexicalFallbackOnly],
                        summary: "Peek a call matched only by a lexical/grep fallback.",
                    }),
                    entry(EntrySeed {
                        entry_id: "entry.split.reference.runtime",
                        kind: RelationNavEntryKind::OpenInSplit,
                        relation: Reference,
                        origin: AiContext,
                        drift: Bound,
                        access: Call,
                        proof: RuntimeObserved,
                        provider: RuntimeObserver,
                        confidence: NavigationConfidence::Imported,
                        freshness: FreshnessClass::WarmCached,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: PartialForDeclaredScope,
                        generated: authored,
                        remapped: false,
                        disambiguation: None,
                        downgrades: &[DowngradeReason::RuntimeOrFrameworkOnly],
                        summary: "Split a reference observed only in a runtime trace.",
                    }),
                    entry(EntrySeed {
                        entry_id: "entry.forward.route.framework",
                        kind: RelationNavEntryKind::ForwardHistory,
                        relation: RouteBinding,
                        origin: AiContext,
                        drift: Remapped,
                        access: Read,
                        proof: FrameworkDerived,
                        provider: FrameworkPack,
                        confidence: NavigationConfidence::Imported,
                        freshness: FreshnessClass::WarmCached,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: CompleteForDeclaredScope,
                        generated: authored,
                        remapped: true,
                        disambiguation: None,
                        downgrades: &[DowngradeReason::RuntimeOrFrameworkOnly],
                        summary: "Forward to a framework route binding that remapped.",
                    }),
                ],
                Vec::new(),
            ),
            "A lexical-fallback call, a runtime-observed reference, and a framework-derived route binding \
             each name their evidence class, carry a fallback note and downgrade reason, and stay \
             captured-scope only — so a grep or runtime match never auto-opens or reads as semantic.",
        ),
        // 5. Archived and ambiguous entries keep a disambiguation path.
        scenario(
            "continuity.archived_and_ambiguous",
            "Archived and ambiguous entries keep their state and a disambiguation path",
            packet_input(
                "continuity:archive:0005",
                SupportExport,
                Some("aureline://scope/captured-archive"),
                OperatorOnlyRestricted,
                vec![
                    entry(EntrySeed {
                        entry_id: "entry.recent.definition.archived",
                        kind: RelationNavEntryKind::RecentLocation,
                        relation: Definition,
                        origin: SupportExport,
                        drift: Archived,
                        access: Read,
                        proof: ImportedEvidence,
                        provider: ProviderClass::ImportedSnapshot,
                        confidence: NavigationConfidence::Imported,
                        freshness: FreshnessClass::Unverified,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: UnavailableForDeclaredScope,
                        generated: GeneratedOrExternalState::ImportedSnapshot,
                        remapped: false,
                        disambiguation: None,
                        downgrades: &[DowngradeReason::MetadataOnlyExport],
                        summary: "Archived definition retained for support replay.",
                    }),
                    entry(EntrySeed {
                        entry_id: "entry.peek.declaration.ambiguous",
                        kind: RelationNavEntryKind::Peek,
                        relation: Declaration,
                        origin: SupportExport,
                        drift: Drifted,
                        access: Read,
                        proof: SyntaxFallback,
                        provider: Syntax,
                        confidence: NavigationConfidence::Heuristic,
                        freshness: FreshnessClass::DegradedCached,
                        ambiguity: AmbiguityClass::AmbiguousNeedsSelection,
                        scope: PartialForDeclaredScope,
                        generated: authored,
                        remapped: false,
                        disambiguation: Some("aureline://disambiguation/ambiguous-declaration"),
                        downgrades: &[DowngradeReason::SyntaxFallbackOnly],
                        summary: "Peek an ambiguous declaration that drifted.",
                    }),
                ],
                vec![rename_evidence(RenameSeed {
                    evidence_id: "rename.drifted.blocked",
                    origin: SupportExport,
                    proof: SyntaxFallback,
                    posture: RenameApplyPosture::BlockedPendingScopeReview,
                    ambiguity: AmbiguityClass::DriftedNeedsReview,
                    drift: Drifted,
                    changed: 0,
                    held: 6,
                    disambiguation: Some("aureline://disambiguation/rename-drifted"),
                    downgrades: &[DowngradeReason::BookmarkOrHistoryDrift],
                    summary: "A drifted rename preview held pending review, kept for support replay.",
                })],
            ),
            "An archived definition stays as metadata-only replay evidence, an ambiguous drifted \
             declaration keeps its disambiguation path, and a blocked drifted rename preview is \
             retained with its posture, ambiguity, and replay id — all support-export safe.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> RelationContinuityInvariant {
    RelationContinuityInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    scenarios: &[RelationContinuityScenario],
) -> Vec<RelationContinuityInvariant> {
    let packets: Vec<&RelationContinuityPacket> = scenarios.iter().map(|s| &s.packet).collect();

    let mut out = Vec::new();

    // Every entry preserves its relation context: relation kind, origin surface, a
    // non-empty return anchor, a captured target, and a relation-preserving current target.
    out.push(invariant(
        "relation_continuity.entry_preserves_relation_context",
        "Every entry keeps its entry kind, relation kind, origin surface, and a return anchor that \
         restores the origin selection and viewport, carries a captured target snapshot of the same \
         relation kind, and — when a current target exists — preserves that relation kind across the \
         remap, so a peek/reveal/split/history entry never degenerates into generic open behavior.",
        packets.iter().all(|packet| {
            packet.entries.iter().all(|entry| {
                !entry.return_anchor.return_anchor_ref.is_empty()
                    && entry.return_anchor.restores_selection
                    && entry.return_anchor.restores_viewport
                    && entry.captured_target.relation_kind == entry.relation_kind
                    && entry
                        .current_target
                        .as_ref()
                        .map_or(true, |current| current.relation_kind == entry.relation_kind)
            })
        }),
    ));

    // Counts reconcile and current-vs-captured is honest.
    out.push(invariant(
        "relation_continuity.current_vs_captured_separated",
        "Every packet reconciles its by-kind, by-drift, and current-versus-captured counts with its \
         total, an entry is current-scope only when it is bound with live semantic evidence, and an \
         entry carried by a captured snapshot/trace/import or any non-bound state counts as captured \
         — so the packet always states what is live and what is captured.",
        packets.iter().all(|packet| {
            packet.counts.reconciles()
                && packet.entries.iter().all(|entry| {
                    entry.current_scope == (entry.drift_state == ContinuityState::Bound
                        && entry.evidence_class == RelationContinuityEvidenceClass::Semantic
                        && !matches!(entry.freshness, FreshnessClass::Stale | FreshnessClass::Unverified))
                })
        }),
    ));

    // No silent jump: non-bound entries disclose state, carry recovery, and never auto-open.
    out.push(invariant(
        "relation_continuity.no_silent_jump",
        "Every non-bound entry has auto_open_allowed false, a visible drift reason, and at least one \
         recovery choice; a remapped entry cites stable remap evidence and never used a nearby \
         fallback; and a drifted, missing-target, scope-unavailable, or archived entry carries no \
         current target — so a return jump never lands on a nearby guess.",
        packets.iter().all(|packet| {
            packet.entries.iter().all(|entry| {
                if entry.drift_state == ContinuityState::Bound {
                    return true;
                }
                let base = !entry.auto_open_allowed
                    && entry.drift_reason.as_ref().is_some_and(|r| !r.is_empty())
                    && !entry.recovery_choices.is_empty()
                    && !entry.used_nearby_fallback;
                let drift_specific = match entry.drift_state {
                    ContinuityState::Remapped => {
                        !entry.remap_evidence_refs.is_empty() && entry.current_target.is_some()
                    }
                    ContinuityState::Drifted
                    | ContinuityState::MissingTarget
                    | ContinuityState::ScopeUnavailable
                    | ContinuityState::Archived => entry.current_target.is_none(),
                    ContinuityState::Bound => true,
                };
                base && drift_specific
            })
        }),
    ));

    // Drift states stay visible and ambiguity routes through a disambiguation set.
    out.push(invariant(
        "relation_continuity.drift_states_visible_with_disambiguation",
        "Every entry's drift state is one of the closed continuity vocabulary and is reflected in its \
         labels, and an entry whose ambiguity needs selection carries a disambiguation set ref and a \
         choose-from-disambiguation recovery choice, so drift and ambiguity stay visible before a jump.",
        packets.iter().all(|packet| {
            packet.entries.iter().all(|entry| {
                let labeled = match entry.drift_state {
                    ContinuityState::Bound => true,
                    ContinuityState::Remapped => entry.labels.contains(&RelationContinuityLabel::Remapped),
                    ContinuityState::Drifted => entry.labels.contains(&RelationContinuityLabel::Drifted),
                    ContinuityState::MissingTarget => {
                        entry.labels.contains(&RelationContinuityLabel::MissingTarget)
                    }
                    ContinuityState::ScopeUnavailable => {
                        entry.labels.contains(&RelationContinuityLabel::ScopeUnavailable)
                    }
                    ContinuityState::Archived => {
                        entry.labels.contains(&RelationContinuityLabel::Archived)
                    }
                };
                let ambiguity_ok = !entry.ambiguity_class.requires_disambiguation()
                    || (entry.disambiguation_set_ref.is_some()
                        && entry
                            .recovery_choices
                            .contains(&RelationRecoveryChoice::ChooseFromDisambiguation));
                labeled && ambiguity_ok
            })
        }),
    ));

    // Fallback class is honest: never a fallback that auto-opens or omits its note.
    out.push(invariant(
        "relation_continuity.fallback_class_honest",
        "Every entry names an evidence class; any entry on a lexical or syntax fallback carries a \
         fallback note and a downgrade reason and is never marked current-scope or auto-open; and any \
         runtime, framework, or imported entry stays captured-scope — so a grep fallback is never \
         replayed as semantic certainty.",
        packets.iter().all(|packet| {
            packet.entries.iter().all(|entry| {
                if entry.evidence_class.is_fallback() {
                    !entry.fallback_notes.is_empty()
                        && !entry.downgrade_reasons.is_empty()
                        && !entry.current_scope
                        && !entry.auto_open_allowed
                } else if matches!(
                    entry.evidence_class,
                    RelationContinuityEvidenceClass::RuntimeObserved
                        | RelationContinuityEvidenceClass::FrameworkDerived
                        | RelationContinuityEvidenceClass::ImportedSnapshot
                ) {
                    !entry.current_scope
                } else {
                    true
                }
            })
        }),
    ));

    // Replay ids are stable and derived from the entry/evidence id.
    out.push(invariant(
        "relation_continuity.replay_ids_stable",
        "Every entry and rename-evidence row carries a non-empty replay-safe target id derived from \
         its own stable id, and the packet declares itself replay-safe, so a support or export packet \
         can replay or explain each navigation without ambiguity.",
        packets.iter().all(|packet| {
            packet.replay_safe
                && packet.entries.iter().all(|entry| {
                    entry.replay_target_id
                        == format!("aureline://replay/relation-nav/{}", entry.entry_id)
                })
                && packet.rename_evidence.iter().all(|evidence| {
                    evidence.replay_target_id
                        == format!("aureline://replay/rename/{}", evidence.evidence_id)
                })
        }),
    ));

    // Rename evidence preserves its relation kind, posture, ambiguity, drift, and counts.
    out.push(invariant(
        "relation_continuity.rename_evidence_preserved",
        "Every rename-evidence row keeps a definition root relation, its apply posture, ambiguity, \
         drift state, return anchor, change-versus-held counts, and a replay id, and discloses a drift \
         reason and a disambiguation path whenever its root drifted or is ambiguous — so rename \
         evidence survives export without retargeting.",
        packets.iter().all(|packet| {
            packet.rename_evidence.iter().all(|evidence| {
                evidence.root_relation_kind == RelationKind::Definition
                    && !evidence.return_anchor.return_anchor_ref.is_empty()
                    && !evidence.replay_target_id.is_empty()
                    && (evidence.drift_state == ContinuityState::Bound
                        || evidence.drift_reason.as_ref().is_some_and(|r| !r.is_empty()))
                    && (!evidence.ambiguity_class.requires_disambiguation()
                        || evidence.disambiguation_set_ref.is_some())
            })
        }),
    ));

    // Consumers preserve the typed truth without retargeting.
    out.push(invariant(
        "relation_continuity.consumers_preserve_truth",
        "Every consumer projection preserves relation kind, origin surface, return anchor, \
         current-versus-captured truth, drift state, fallback class, ambiguity, and replay ids, never \
         silently retargets, and never exports raw code bodies, and every required consumer surface is \
         covered — so review, support, AI, graph, docs, editor, and CLI consumers see the same truth.",
        packets.iter().all(|packet| {
            !packet.consumer_projections.is_empty()
                && packet
                    .consumer_projections
                    .iter()
                    .all(RelationContinuityProjection::preserves_truth)
                && required_surfaces_covered(&packet.consumer_projections)
        }),
    ));

    // The corpus covers every entry kind, drift state, and evidence answer, plus rename evidence.
    out.push(invariant(
        "relation_continuity.corpus_covers_vocabulary",
        "The corpus exercises every navigation entry kind, every drift state, the semantic, \
         framework, runtime, imported, and lexical evidence answers, ambiguous and archived entries, \
         and rename-preview evidence — so the relation-continuity model is proven across its \
         vocabulary.",
        every_entry_kind_covered(&packets)
            && every_drift_state_covered(&packets)
            && every_evidence_answer_covered(&packets)
            && packets.iter().any(|packet| !packet.rename_evidence.is_empty()),
    ));

    out
}

// ---------------------------------------------------------------------------
// Invariant helpers.
// ---------------------------------------------------------------------------

fn required_surfaces_covered(projections: &[RelationContinuityProjection]) -> bool {
    REQUIRED_CONSUMER_SURFACES.iter().all(|surface| {
        projections
            .iter()
            .any(|projection| projection.consumer_surface == *surface)
    })
}

fn every_entry_kind_covered(packets: &[&RelationContinuityPacket]) -> bool {
    RELATION_NAV_ENTRY_ORDER.iter().all(|kind| {
        packets
            .iter()
            .any(|packet| packet.entries.iter().any(|entry| entry.entry_kind == *kind))
    })
}

fn every_drift_state_covered(packets: &[&RelationContinuityPacket]) -> bool {
    RELATION_CONTINUITY_DRIFT_STATES.iter().all(|state| {
        packets.iter().any(|packet| {
            packet
                .entries
                .iter()
                .any(|entry| entry.drift_state == *state)
        })
    })
}

fn every_evidence_answer_covered(packets: &[&RelationContinuityPacket]) -> bool {
    let answers = [
        RelationContinuityEvidenceClass::Semantic,
        RelationContinuityEvidenceClass::FrameworkDerived,
        RelationContinuityEvidenceClass::RuntimeObserved,
        RelationContinuityEvidenceClass::ImportedSnapshot,
        RelationContinuityEvidenceClass::LexicalFallback,
    ];
    answers.iter().all(|answer| {
        packets.iter().any(|packet| {
            packet
                .entries
                .iter()
                .any(|entry| entry.evidence_class == *answer)
        })
    })
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative path or opaque `aureline://` handle,
/// never a URL, host, credential, or absolute path.
fn is_export_safe_ref(r: &str) -> bool {
    if r.is_empty() || r.starts_with('/') || (r.contains("://") && !r.starts_with("aureline://")) {
        return false;
    }
    r.starts_with("schemas/")
        || r.starts_with("crates/")
        || r.starts_with("artifacts/")
        || r.starts_with("fixtures/")
        || r.starts_with("docs/")
        || r.starts_with("aureline://")
}
