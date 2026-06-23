//! Governed rename preview: the typed preview-and-apply model over rename candidates.
//!
//! The [`target_model`](crate::target_model) freezes the
//! [`RenamePreviewSet`](crate::target_model::RenamePreviewSet) object — the candidate
//! occurrences a rename would touch, with blocked, conflicting, generated, read-only,
//! and partial-scope candidates named — and the
//! [`RenameApplyPosture`](crate::target_model::RenameApplyPosture) that says whether a
//! preview may be applied. The [`relation-navigation matrix`](crate::m5_relation_navigation)
//! names that object family and pins its rename-omission vocabulary. What was still
//! implicit is the *preview-and-apply model*: how Aureline turns a flat list of rename
//! candidates into a preview that shows **what the rename would change**, **what it
//! would not change and why** (blocked/conflict/generated/read-only/partial-scope
//! groupings), and an **inspect-before-mutate apply gate** that never lets a broad
//! rename collapse into one opaque apply action that silently drops the candidates it
//! cannot or will not touch.
//!
//! This module is that model. [`build_rename_preview`] is a pure function over a typed
//! [`RenamePreviewInput`] that produces a [`GovernedRenamePreview`]:
//!
//! 1. **Disjoint candidate grouping.** Candidates are grouped into
//!    [`RenameCandidateGroup`]s keyed by [`RenameCandidateGroupKind`] in a canonical
//!    order — editable, blocked-for-review, conflict, generated-boundary,
//!    read-only-or-external, partial-scope-omitted — by a fixed precedence so a
//!    blocked, generated, or read-only candidate is never silently counted as
//!    editable, and every candidate lands in exactly one group.
//! 2. **Exact change-versus-held counts.** Each group and the preview carry a
//!    [`RenameCandidateCounts`] separating the editable set the rename *will change*
//!    from the held set it *will not* (with per-reason and current-versus-captured
//!    tallies), so the preview can always state what changes, what does not, and why.
//! 3. **Omission and conflict truth.** Every non-editable candidate keeps a visible
//!    [`RenameOmissionReason`] and [`RenameCandidateLabel`]; conflict candidates keep
//!    their conflict notes; and any group resting on a lexical/grep fallback carries a
//!    fallback note and a downgrade reason, so an omitted candidate never disappears
//!    and a grep fallback never masquerades as semantic certainty.
//! 4. **Inspect-before-mutate apply gate.** Each preview carries a [`RenameApplyGate`]
//!    that always requires inspection before mutation, always blocks a blind apply,
//!    derives a [`RenameApplyPosture`](crate::target_model::RenameApplyPosture) and the
//!    [`RenameApplyPrecondition`]s the user must clear, keeps omitted and redacted
//!    candidates visible, and binds an undo checkpoint that preserves the preview.
//! 5. **Consumer parity.** Each preview projects to every
//!    [`ConsumerSurface`](crate::target_model::ConsumerSurface) with a
//!    [`RenamePreviewProjection`] that preserves the groups, counts, omission reasons,
//!    conflict notes, and apply gate, never flattens the rename into a single apply
//!    action, never exports code bodies, and preserves the undo checkpoint — so review,
//!    support, AI, graph, docs, and editor consumers can reconstruct the rename.
//!
//! [`rename_preview_set`] freezes a deterministic corpus of previews whose
//! [`RenamePreviewInvariant`] flags are computed from the builder's own output, so the
//! checked-in fixture and the freeze gate pin the contract byte-for-byte and any
//! regression in [`build_rename_preview`] flips an invariant or drifts the fixture
//! rather than silently passing. The records carry no source bodies, raw paths,
//! provider payloads, identifiers, URLs, hostnames, or credentials — only opaque object
//! handles, stable tokens, and short reviewable sentences — so they are safe for
//! support export.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::target_model::{
    AccessKind, ConsumerSurface, DowngradeReason, ExportRedactionClass, FreshnessClass,
    GeneratedOrExternalState, NavigationConfidence, NavigationTargetCountSummary, ProofClass,
    ProviderClass, RelationKind, RenameApplyPosture, RenamePreviewSet, ScopeCompleteness,
    REQUIRED_CONSUMER_SURFACES,
};

#[cfg(test)]
mod tests;

/// Schema version for the rename-preview corpus.
pub const RENAME_PREVIEW_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the rename-preview corpus.
///
/// Distinct from the language-wedge `rename_preview_record` schema; this corpus is the
/// governed preview-and-apply model over the [`RenamePreviewSet`] object.
pub const RENAME_PREVIEW_SCHEMA_REF: &str =
    "schemas/navigation/governed_rename_preview.schema.json";

/// Stable record-kind tag for the rename-preview corpus.
pub const RENAME_PREVIEW_RECORD_KIND: &str = "rename_preview_governance_set";

/// Stable id for the canonical rename-preview corpus.
pub const RENAME_PREVIEW_SET_ID: &str = "rename-preview:set:0001";

/// Evaluation stamp for the canonical corpus. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const RENAME_PREVIEW_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the corpus binding current. Stable promotion runs this
/// gate; it fails when the in-code corpus drifts from the checked-in fixture or any
/// invariant flips.
pub const RENAME_PREVIEW_FREEZE_GATE_REF: &str =
    "crates/aureline-navigation/tests/rename_preview.rs";

/// Reviewer doc for the rename-preview contract.
pub const RENAME_PREVIEW_DOC_REF: &str = "docs/navigation/governed_rename_preview.md";

/// Evidence companion for the rename-preview corpus.
pub const RENAME_PREVIEW_ARTIFACT_REF: &str = "artifacts/navigation/governed_rename_preview.md";

/// Repo-relative path of the checked-in canonical corpus.
pub const RENAME_PREVIEW_FIXTURE_REF: &str =
    "fixtures/navigation/governed_rename_preview/canonical_previews.json";

/// The canonical group order for a rename preview.
///
/// A preview lists the editable set first, then the held groups in
/// blocked → conflict → generated → read-only → partial-scope order, so the change
/// set is never confused with the held set and a held candidate is never folded into
/// the editable count.
pub const RENAME_GROUP_ORDER: [RenameCandidateGroupKind; 6] = [
    RenameCandidateGroupKind::Editable,
    RenameCandidateGroupKind::BlockedForReview,
    RenameCandidateGroupKind::Conflict,
    RenameCandidateGroupKind::GeneratedBoundary,
    RenameCandidateGroupKind::ReadOnlyOrExternal,
    RenameCandidateGroupKind::PartialScopeOmitted,
];

// ---------------------------------------------------------------------------
// Group kinds.
// ---------------------------------------------------------------------------

/// The group a rename candidate lands in.
///
/// Grouping is disjoint and precedence-ordered: a candidate that is blocked, in
/// conflict, generated, read-only, or out of scope is held under that group rather
/// than the editable one, so only the [`Editable`](RenameCandidateGroupKind::Editable)
/// group is ever changed by an apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenameCandidateGroupKind {
    /// The candidate will be renamed when the preview is applied.
    Editable,
    /// The candidate is blocked by policy or protection and held for review.
    BlockedForReview,
    /// The candidate collides, shadows, or aliases and is held for resolution.
    Conflict,
    /// The candidate sits across a generated or paired-artifact boundary.
    GeneratedBoundary,
    /// The candidate is read-only, external, or imported and cannot be mutated.
    ReadOnlyOrExternal,
    /// The candidate is out of scope, unresolved, sparse, or partially loaded.
    PartialScopeOmitted,
}

impl RenameCandidateGroupKind {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editable => "editable",
            Self::BlockedForReview => "blocked_for_review",
            Self::Conflict => "conflict",
            Self::GeneratedBoundary => "generated_boundary",
            Self::ReadOnlyOrExternal => "read_only_or_external",
            Self::PartialScopeOmitted => "partial_scope_omitted",
        }
    }

    /// Returns a human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Editable => "Will rename",
            Self::BlockedForReview => "Blocked — pending review",
            Self::Conflict => "Conflict — pending resolution",
            Self::GeneratedBoundary => "Generated boundary — held",
            Self::ReadOnlyOrExternal => "Read-only / external — held",
            Self::PartialScopeOmitted => "Out of scope / partial — omitted",
        }
    }

    /// Returns true when applying the preview mutates candidates in this group.
    pub const fn mutates_on_apply(self) -> bool {
        matches!(self, Self::Editable)
    }

    /// Returns true when this group blocks a direct apply (must clear first).
    pub const fn blocks_apply(self) -> bool {
        matches!(self, Self::BlockedForReview | Self::Conflict)
    }
}

// ---------------------------------------------------------------------------
// Omission reasons, labels, evidence.
// ---------------------------------------------------------------------------

/// Why a rename candidate is held out of the editable set.
///
/// Every non-editable candidate keeps at least one of these, so a preview can always
/// answer *why* a candidate would not be changed instead of dropping it silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenameOmissionReason {
    /// Held by an explicit block, pending review before any broad mutation.
    BlockedPendingReview,
    /// Held by policy, trust, or protected-source limits.
    PolicyLimited,
    /// Held because the candidate collides, shadows, or aliases another symbol.
    ConflictPendingResolution,
    /// Held because the candidate sits across a generated or paired-artifact boundary.
    GeneratedBoundary,
    /// Held because the candidate is read-only or protected source.
    ReadOnlyOrProtected,
    /// Held because the candidate is external-dependency or imported-snapshot source.
    ExternalDependency,
    /// Held because the current workset or sparse slice omits the candidate's scope.
    OutOfScopeSparse,
    /// Held because the candidate's scope is only partially loaded.
    PartiallyLoaded,
    /// Held because the candidate's scope is stale and must be refreshed first.
    StaleScope,
    /// Held because the candidate's anchor could not be resolved.
    UnresolvedAnchor,
}

impl RenameOmissionReason {
    /// All omission reasons, in vocabulary order.
    pub const ALL: [Self; 10] = [
        Self::BlockedPendingReview,
        Self::PolicyLimited,
        Self::ConflictPendingResolution,
        Self::GeneratedBoundary,
        Self::ReadOnlyOrProtected,
        Self::ExternalDependency,
        Self::OutOfScopeSparse,
        Self::PartiallyLoaded,
        Self::StaleScope,
        Self::UnresolvedAnchor,
    ];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockedPendingReview => "blocked_pending_review",
            Self::PolicyLimited => "policy_limited",
            Self::ConflictPendingResolution => "conflict_pending_resolution",
            Self::GeneratedBoundary => "generated_boundary",
            Self::ReadOnlyOrProtected => "read_only_or_protected",
            Self::ExternalDependency => "external_dependency",
            Self::OutOfScopeSparse => "out_of_scope_sparse",
            Self::PartiallyLoaded => "partially_loaded",
            Self::StaleScope => "stale_scope",
            Self::UnresolvedAnchor => "unresolved_anchor",
        }
    }

    /// Returns true when this reason is a policy or protection block — the class that
    /// routes the apply posture through a policy/protected review gate.
    pub const fn is_policy_or_protected(self) -> bool {
        matches!(
            self,
            Self::PolicyLimited | Self::ReadOnlyOrProtected | Self::BlockedPendingReview
        )
    }
}

/// A user-visible label a preview attaches to a candidate, group, or the whole preview.
///
/// Labels keep generated, external, read-only, imported, test-only, fallback, runtime,
/// framework, conflicting, blocked, out-of-scope, unresolved, and captured-scope
/// candidates visible instead of folding them into one editable count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenameCandidateLabel {
    /// The candidate is generated or a paired artifact.
    Generated,
    /// The candidate is external-dependency source.
    External,
    /// The candidate is read-only or protected source.
    ReadOnly,
    /// The candidate is imported-snapshot source.
    ImportedSnapshot,
    /// The candidate is test-only.
    TestOnly,
    /// The candidate rests on a lexical/grep fallback.
    LexicalFallback,
    /// The candidate rests on a syntax-only fallback.
    SyntaxFallback,
    /// The candidate is runtime-observed.
    RuntimeObserved,
    /// The candidate is framework-derived.
    FrameworkDerived,
    /// The candidate collides, shadows, or aliases another symbol.
    Conflict,
    /// The candidate is blocked and held for review.
    Blocked,
    /// The candidate is out of scope, sparse, or partially loaded.
    OutOfScope,
    /// The candidate's anchor could not be resolved.
    Unresolved,
    /// The candidate is carried only by a captured scope, not the current scope.
    CapturedScopeOnly,
}

impl RenameCandidateLabel {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Generated => "generated",
            Self::External => "external",
            Self::ReadOnly => "read_only",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::TestOnly => "test_only",
            Self::LexicalFallback => "lexical_fallback",
            Self::SyntaxFallback => "syntax_fallback",
            Self::RuntimeObserved => "runtime_observed",
            Self::FrameworkDerived => "framework_derived",
            Self::Conflict => "conflict",
            Self::Blocked => "blocked",
            Self::OutOfScope => "out_of_scope",
            Self::Unresolved => "unresolved",
            Self::CapturedScopeOnly => "captured_scope_only",
        }
    }
}

/// The evidence class for a rename group or preview.
///
/// Answers the support/debug question "was this candidate set semantic,
/// framework-derived, runtime-observed, imported, or a lexical fallback?". A group
/// whose members rest on more than one evidence class resolves to [`Mixed`].
///
/// [`Mixed`]: RenameEvidenceClass::Mixed
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenameEvidenceClass {
    /// Direct or indexed semantic evidence over the declared scope.
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
    /// The group mixes more than one evidence class.
    Mixed,
    /// No admissible evidence for the group.
    Unavailable,
}

impl RenameEvidenceClass {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Semantic => "semantic",
            Self::FrameworkDerived => "framework_derived",
            Self::RuntimeObserved => "runtime_observed",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::LexicalFallback => "lexical_fallback",
            Self::SyntaxFallback => "syntax_fallback",
            Self::Mixed => "mixed",
            Self::Unavailable => "unavailable",
        }
    }

    /// Returns true when this evidence class must render with a visible caveat rather
    /// than as plain semantic certainty.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Semantic)
    }

    /// Returns true when this evidence class rests on a lexical/syntax fallback.
    pub const fn is_fallback(self) -> bool {
        matches!(self, Self::LexicalFallback | Self::SyntaxFallback)
    }
}

// ---------------------------------------------------------------------------
// Counts.
// ---------------------------------------------------------------------------

/// Change-versus-held counts for a rename group or preview.
///
/// The disjoint group tallies — `will_change`, `blocked`, `conflict`, `generated`,
/// `read_only`, `partial_scope_omitted` — always sum to `total_count`, so the preview
/// can state exactly what the rename changes and what it holds. `unresolved_count`,
/// `current_scope_count`, and `captured_scope_count` cross-cut those groups, with
/// current and captured always summing to the total.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenameCandidateCounts {
    /// Total candidates in the group or preview.
    pub total_count: usize,
    /// Candidates the rename will change (the editable set).
    pub will_change_count: usize,
    /// Candidates held by an explicit block, pending review.
    pub blocked_count: usize,
    /// Candidates held by a conflict, pending resolution.
    pub conflict_count: usize,
    /// Candidates held across a generated or paired-artifact boundary.
    pub generated_count: usize,
    /// Candidates held because they are read-only, external, or imported.
    pub read_only_count: usize,
    /// Candidates held because they are out of scope, sparse, or partially loaded.
    pub partial_scope_omitted_count: usize,
    /// Candidates whose anchor could not be resolved (cross-cuts the held groups).
    pub unresolved_count: usize,
    /// Candidates proven against the current scope.
    pub current_scope_count: usize,
    /// Candidates carried only by a captured snapshot, trace, or imported pack.
    pub captured_scope_count: usize,
}

impl RenameCandidateCounts {
    /// Candidates the rename would hold rather than change.
    pub const fn held_count(&self) -> usize {
        self.total_count - self.will_change_count
    }

    /// Returns true when the disjoint group counts and the current/captured split both
    /// reconcile with the total.
    pub const fn reconciles(&self) -> bool {
        self.will_change_count
            + self.blocked_count
            + self.conflict_count
            + self.generated_count
            + self.read_only_count
            + self.partial_scope_omitted_count
            == self.total_count
            && self.current_scope_count + self.captured_scope_count == self.total_count
    }

    fn add(&mut self, candidate: &RenameCandidate, group_kind: RenameCandidateGroupKind) {
        self.total_count += 1;
        match group_kind {
            RenameCandidateGroupKind::Editable => self.will_change_count += 1,
            RenameCandidateGroupKind::BlockedForReview => self.blocked_count += 1,
            RenameCandidateGroupKind::Conflict => self.conflict_count += 1,
            RenameCandidateGroupKind::GeneratedBoundary => self.generated_count += 1,
            RenameCandidateGroupKind::ReadOnlyOrExternal => self.read_only_count += 1,
            RenameCandidateGroupKind::PartialScopeOmitted => self.partial_scope_omitted_count += 1,
        }
        if !candidate.anchor_resolved {
            self.unresolved_count += 1;
        }
        if is_captured_only(candidate) {
            self.captured_scope_count += 1;
        } else {
            self.current_scope_count += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// Candidate input.
// ---------------------------------------------------------------------------

/// One rename candidate the builder groups and tallies.
///
/// Mirrors the shape of a [`ReferenceOccurrence`](crate::target_model::ReferenceOccurrence)
/// — access kind, proof class, provider, freshness, scope completeness, authorship —
/// plus the rename-specific facts the builder needs to group it: an explicit block
/// reason, a conflict note, and whether its anchor resolved. The builder never invents
/// these; it groups deterministically from them so a provider cannot smuggle a blocked
/// or unresolved candidate into the editable set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenameCandidate {
    /// Stable candidate id.
    pub candidate_id: String,
    /// The root symbol ref this candidate occurrence references.
    pub target_ref: String,
    /// Stable source anchor ref for the candidate.
    pub anchor_ref: String,
    /// Access kind for the candidate occurrence.
    pub access_kind: AccessKind,
    /// Scope containing the candidate.
    pub scope_ref: String,
    /// Authorship, generated, external, read-only, or imported posture.
    pub generated_or_external_state: GeneratedOrExternalState,
    /// Proof class for the candidate.
    pub proof_class: ProofClass,
    /// Provider family that admitted the candidate.
    pub provider_class: ProviderClass,
    /// Confidence class for the candidate.
    pub confidence: NavigationConfidence,
    /// Freshness class for the candidate.
    pub freshness: FreshnessClass,
    /// Completeness of the candidate's materialized scope.
    pub scope_completeness: ScopeCompleteness,
    /// Explicit block held by policy or protection, if any.
    pub block_reason: Option<RenameOmissionReason>,
    /// Conflict note such as shadowing or alias ambiguity, if any.
    pub conflict_note: Option<String>,
    /// Whether the candidate's anchor resolved against current scope.
    pub anchor_resolved: bool,
    /// Downgrade reasons that must stay visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Evidence refs safe for support, review, AI, and CLI consumers.
    pub evidence_refs: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

impl RenameCandidate {
    /// Returns the group this candidate lands in, by fixed precedence.
    ///
    /// Precedence: explicit block → conflict → generated boundary → read-only/external
    /// → out-of-scope/unresolved/partial/stale → editable. The first matching rule
    /// wins, so a blocked candidate is never editable and an unresolved candidate is
    /// never silently renamed.
    pub fn group_kind(&self) -> RenameCandidateGroupKind {
        if self.block_reason.is_some() {
            return RenameCandidateGroupKind::BlockedForReview;
        }
        if self.conflict_note.is_some() {
            return RenameCandidateGroupKind::Conflict;
        }
        if is_generated(self) {
            return RenameCandidateGroupKind::GeneratedBoundary;
        }
        if is_read_only_or_external(self) {
            return RenameCandidateGroupKind::ReadOnlyOrExternal;
        }
        if !self.anchor_resolved || self.is_out_of_scope() {
            return RenameCandidateGroupKind::PartialScopeOmitted;
        }
        RenameCandidateGroupKind::Editable
    }

    /// Returns true when the candidate's scope is partial, sparse, or stale.
    fn is_out_of_scope(&self) -> bool {
        self.scope_completeness != ScopeCompleteness::CompleteForDeclaredScope
            || self
                .downgrade_reasons
                .iter()
                .any(|reason| matches!(reason, DowngradeReason::SparseWorkset))
            || matches!(
                self.freshness,
                FreshnessClass::Stale | FreshnessClass::Unverified
            )
    }
}

// ---------------------------------------------------------------------------
// Group, apply gate, projection, preview.
// ---------------------------------------------------------------------------

/// One disjoint group inside a rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenameCandidateGroup {
    /// The group kind.
    pub group_kind: RenameCandidateGroupKind,
    /// Candidate ids in this group, in preview order.
    pub candidate_refs: Vec<String>,
    /// Change-versus-held counts for the group.
    pub counts: RenameCandidateCounts,
    /// The evidence class for the group.
    pub evidence_class: RenameEvidenceClass,
    /// Why these candidates are held (empty for the editable group).
    pub omission_reasons: Vec<RenameOmissionReason>,
    /// Conflict notes preserved from conflict candidates.
    pub conflict_notes: Vec<String>,
    /// Provider families that admitted candidates in this group.
    pub provider_classes: Vec<ProviderClass>,
    /// Visible labels for the group.
    pub labels: Vec<RenameCandidateLabel>,
    /// Downgrade reasons that must stay visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Fallback notes describing lexical/syntax/runtime/framework/imported evidence.
    pub fallback_notes: Vec<String>,
    /// Whether applying the preview mutates candidates in this group.
    pub mutates_on_apply: bool,
    /// Always true: held candidates stay visible even when their content is redacted.
    pub remains_visible_when_redacted: bool,
    /// Export-safe summary.
    pub summary: String,
}

/// An apply precondition the user must clear before a held rename can proceed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenameApplyPrecondition {
    /// Review the blocked candidates before applying.
    ReviewBlockedCandidates,
    /// Resolve the conflicting candidates before applying.
    ResolveConflicts,
    /// Acknowledge the generated-boundary candidates that will not be changed.
    AcknowledgeGeneratedBoundary,
    /// Acknowledge the read-only/external candidates that will not be changed.
    AcknowledgeReadOnlyOmission,
    /// Acknowledge the out-of-scope, sparse, or partial candidates that are omitted.
    AcknowledgePartialScope,
    /// Refresh the stale scope before applying.
    RefreshStaleScope,
    /// Widen the sparse workset before applying for full coverage.
    WidenSparseScope,
}

impl RenameApplyPrecondition {
    /// All preconditions, in canonical order.
    pub const ALL: [Self; 7] = [
        Self::ReviewBlockedCandidates,
        Self::ResolveConflicts,
        Self::AcknowledgeGeneratedBoundary,
        Self::AcknowledgeReadOnlyOmission,
        Self::AcknowledgePartialScope,
        Self::RefreshStaleScope,
        Self::WidenSparseScope,
    ];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewBlockedCandidates => "review_blocked_candidates",
            Self::ResolveConflicts => "resolve_conflicts",
            Self::AcknowledgeGeneratedBoundary => "acknowledge_generated_boundary",
            Self::AcknowledgeReadOnlyOmission => "acknowledge_read_only_omission",
            Self::AcknowledgePartialScope => "acknowledge_partial_scope",
            Self::RefreshStaleScope => "refresh_stale_scope",
            Self::WidenSparseScope => "widen_sparse_scope",
        }
    }
}

/// The inspect-before-mutate apply gate for a rename preview.
///
/// The gate always requires inspection before mutation and always blocks a blind
/// apply, so a broad rename can never collapse into one opaque action. It carries the
/// derived [`RenameApplyPosture`](crate::target_model::RenameApplyPosture), the
/// preconditions to clear, the change-versus-held counts, and an undo checkpoint, and
/// states that omitted and redacted candidates stay visible through the apply flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenameApplyGate {
    /// The derived apply posture.
    pub apply_posture: RenameApplyPosture,
    /// Always true: a broad rename must be inspected before it mutates anything.
    pub inspect_before_mutate_required: bool,
    /// Always true: a blind apply that skips the preview is never allowed.
    pub blind_apply_blocked: bool,
    /// Whether the editable set may be applied once the preview is inspected.
    pub apply_allowed_after_preview: bool,
    /// Candidates the rename will change.
    pub will_change_count: usize,
    /// Candidates the rename will hold rather than change.
    pub held_count: usize,
    /// Preconditions the user must clear, in canonical order.
    pub preconditions: Vec<RenameApplyPrecondition>,
    /// Always true: omitted candidates stay visible through the apply flow.
    pub omitted_candidates_remain_visible: bool,
    /// Always true: candidates stay visible even when their content is redacted.
    pub redacted_candidates_remain_visible: bool,
    /// Undo checkpoint ref that preserves the preview for replay and rollback.
    pub undo_checkpoint_ref: String,
    /// Export-safe summary.
    pub summary: String,
}

/// A surface-level projection proving the preview survives review, support, AI, graph,
/// docs, and editor consumers without flattening into one apply action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenamePreviewProjection {
    /// The consumer surface.
    pub consumer_surface: ConsumerSurface,
    /// Number of groups projected to this surface.
    pub projected_group_count: usize,
    /// True when the disjoint candidate grouping is preserved.
    pub preserves_candidate_grouping: bool,
    /// True when the change-versus-held counts are preserved.
    pub preserves_counts: bool,
    /// True when the omission reasons stay visible.
    pub preserves_omission_reasons: bool,
    /// True when conflict notes are preserved.
    pub preserves_conflict_notes: bool,
    /// True when the inspect-before-mutate apply gate is preserved.
    pub preserves_apply_gate: bool,
    /// True when the undo checkpoint is preserved.
    pub preserves_undo_checkpoint: bool,
    /// True when omitted candidates stay visible (must be true).
    pub omitted_candidates_remain_visible: bool,
    /// True when the projection flattens the rename into one apply action (must be false).
    pub flattens_to_single_apply_action: bool,
    /// True when the projection exports raw code bodies (must be false).
    pub exports_code_bodies: bool,
    /// Redaction class for this projection.
    pub redaction_class: ExportRedactionClass,
    /// Export-safe summary.
    pub summary: String,
}

impl RenamePreviewProjection {
    /// Returns true when the projection preserves the preview's typed truth without
    /// flattening into one apply action or leaking code bodies.
    pub const fn preserves_truth(&self) -> bool {
        self.preserves_candidate_grouping
            && self.preserves_counts
            && self.preserves_omission_reasons
            && self.preserves_conflict_notes
            && self.preserves_apply_gate
            && self.preserves_undo_checkpoint
            && self.omitted_candidates_remain_visible
            && !self.flattens_to_single_apply_action
            && !self.exports_code_bodies
    }
}

/// A governed rename preview: candidates grouped by editability, change-versus-held
/// counts, omission and conflict truth, an inspect-before-mutate apply gate, the frozen
/// [`RenamePreviewSet`](crate::target_model::RenamePreviewSet) object, and consumer
/// projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedRenamePreview {
    /// Stable preview id.
    pub preview_id: String,
    /// The root symbol ref being renamed.
    pub root_target_ref: String,
    /// The relation kind of the renamed root (always [`RelationKind::Definition`]).
    pub root_relation_kind: RelationKind,
    /// Opaque handle for the symbol's current name (never the raw identifier).
    pub current_name_ref: String,
    /// Opaque handle for the proposed new name (never the raw identifier).
    pub proposed_name_ref: String,
    /// The current scope ref the rename was resolved against.
    pub scope_ref: String,
    /// The captured snapshot/trace/pack scope ref, when any candidate is captured-only.
    pub captured_scope_ref: Option<String>,
    /// Disjoint candidate groups, in canonical order.
    pub groups: Vec<RenameCandidateGroup>,
    /// Aggregate change-versus-held counts across all groups.
    pub totals: RenameCandidateCounts,
    /// The aggregate evidence class for the preview.
    pub preview_evidence_class: RenameEvidenceClass,
    /// The union of group labels.
    pub labels: Vec<RenameCandidateLabel>,
    /// The union of omission reasons across held groups.
    pub omission_reasons: Vec<RenameOmissionReason>,
    /// Conflict notes preserved across the preview.
    pub conflict_notes: Vec<String>,
    /// The inspect-before-mutate apply gate.
    pub apply_gate: RenameApplyGate,
    /// The frozen rename-preview-set object this governed preview projects.
    pub preview_set: RenamePreviewSet,
    /// Consumer projections proving cross-surface parity.
    pub consumer_projections: Vec<RenamePreviewProjection>,
    /// Downgrade reasons that must stay visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Fallback notes describing the preview's weakest evidence.
    pub fallback_notes: Vec<String>,
    /// Redaction class for review/support/AI export.
    pub redaction_class: ExportRedactionClass,
    /// Export-safe summary.
    pub summary: String,
}

impl GovernedRenamePreview {
    /// Returns the group for a kind, if present.
    pub fn group(&self, group_kind: RenameCandidateGroupKind) -> Option<&RenameCandidateGroup> {
        self.groups
            .iter()
            .find(|group| group.group_kind == group_kind)
    }

    /// Returns true when the preview has any held (non-editable) candidate.
    pub const fn has_held_candidates(&self) -> bool {
        self.totals.held_count() > 0
    }

    /// Returns true when the preview has any captured-only candidate.
    pub const fn has_captured_scope(&self) -> bool {
        self.totals.captured_scope_count > 0
    }

    /// Returns true when the preview must render with a visible caveat or cannot be
    /// directly applied.
    pub fn requires_disclosure(&self) -> bool {
        self.apply_gate.apply_posture.blocks_apply()
            || self.has_held_candidates()
            || self.has_captured_scope()
            || self.preview_evidence_class.requires_disclosure()
            || !self.downgrade_reasons.is_empty()
    }
}

/// The typed input the builder turns into a governed rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenamePreviewInput {
    /// Stable preview id.
    pub preview_id: String,
    /// The root symbol ref being renamed.
    pub root_target_ref: String,
    /// Opaque handle for the symbol's current name.
    pub current_name_ref: String,
    /// Opaque handle for the proposed new name.
    pub proposed_name_ref: String,
    /// The current scope ref.
    pub scope_ref: String,
    /// The captured snapshot/trace/pack scope ref, if any.
    pub captured_scope_ref: Option<String>,
    /// Redaction class for review/support/AI export.
    pub redaction_class: ExportRedactionClass,
    /// The rename candidates to group and tally.
    pub candidates: Vec<RenameCandidate>,
}

// ---------------------------------------------------------------------------
// Builder.
// ---------------------------------------------------------------------------

/// Builds a governed rename preview from a typed input.
///
/// Deterministic: the same input yields the same preview. Candidates are grouped by
/// the fixed [`RenameCandidate::group_kind`] precedence in [`RENAME_GROUP_ORDER`]; each
/// group and the preview carry computed change-versus-held counts, an evidence class,
/// omission reasons, conflict notes, provider attribution, labels, downgrade reasons,
/// and fallback notes; the inspect-before-mutate apply gate, the frozen
/// [`RenamePreviewSet`](crate::target_model::RenamePreviewSet) object, and the consumer
/// projections are derived from the candidates themselves, so a held, fallback, or
/// captured candidate cannot lose its caveat.
pub fn build_rename_preview(input: &RenamePreviewInput) -> GovernedRenamePreview {
    // Assign each candidate to its group, preserving input order within a group.
    let mut grouped: Vec<(RenameCandidateGroupKind, Vec<&RenameCandidate>)> = Vec::new();
    for group_kind in RENAME_GROUP_ORDER {
        let members: Vec<&RenameCandidate> = input
            .candidates
            .iter()
            .filter(|candidate| candidate.group_kind() == group_kind)
            .collect();
        if !members.is_empty() {
            grouped.push((group_kind, members));
        }
    }

    let groups: Vec<RenameCandidateGroup> = grouped
        .iter()
        .map(|(kind, members)| build_group(*kind, members))
        .collect();

    let all: Vec<&RenameCandidate> = input.candidates.iter().collect();
    let mut totals = RenameCandidateCounts::default();
    for (kind, members) in &grouped {
        for candidate in members {
            totals.add(candidate, *kind);
        }
    }

    let preview_evidence_class = evidence_class_for(&all);
    let labels = labels_for(&all);
    let omission_reasons = union_omission_reasons(&groups);
    let conflict_notes = union_conflict_notes(&all);
    let downgrade_reasons = downgrade_reasons_for(&all);
    let fallback_notes = fallback_notes_for(&all);

    let apply_gate = build_apply_gate(&input.preview_id, &totals, &all);
    let preview_set = build_preview_set(input, &totals, &all, &apply_gate, &conflict_notes);

    let consumer_projections = REQUIRED_CONSUMER_SURFACES
        .iter()
        .map(|surface| build_projection(*surface, groups.len(), input.redaction_class))
        .collect();

    let summary = format!(
        "Rename preview for {} root: {} candidate(s) — {} will change, {} held across {} group(s); \
         {} current, {} captured-scope; evidence {}; apply posture {}.",
        input.root_target_ref,
        totals.total_count,
        totals.will_change_count,
        totals.held_count(),
        groups.len(),
        totals.current_scope_count,
        totals.captured_scope_count,
        preview_evidence_class.as_str(),
        apply_posture_token(apply_gate.apply_posture),
    );

    GovernedRenamePreview {
        preview_id: input.preview_id.clone(),
        root_target_ref: input.root_target_ref.clone(),
        root_relation_kind: RelationKind::Definition,
        current_name_ref: input.current_name_ref.clone(),
        proposed_name_ref: input.proposed_name_ref.clone(),
        scope_ref: input.scope_ref.clone(),
        captured_scope_ref: input.captured_scope_ref.clone(),
        groups,
        totals,
        preview_evidence_class,
        labels,
        omission_reasons,
        conflict_notes,
        apply_gate,
        preview_set,
        consumer_projections,
        downgrade_reasons,
        fallback_notes,
        redaction_class: input.redaction_class,
        summary,
    }
}

fn build_group(
    group_kind: RenameCandidateGroupKind,
    members: &[&RenameCandidate],
) -> RenameCandidateGroup {
    let mut counts = RenameCandidateCounts::default();
    for candidate in members {
        counts.add(candidate, group_kind);
    }
    let evidence_class = evidence_class_for(members);
    let omission_reasons = if group_kind == RenameCandidateGroupKind::Editable {
        Vec::new()
    } else {
        omission_reasons_for(group_kind, members)
    };
    let conflict_notes = union_conflict_notes(members);
    let provider_classes = provider_classes_for(members);
    let labels = labels_for(members);
    let downgrade_reasons = downgrade_reasons_for(members);
    let fallback_notes = fallback_notes_for(members);
    let candidate_refs = members
        .iter()
        .map(|candidate| candidate.candidate_id.clone())
        .collect();
    let summary = format!(
        "{} group: {} candidate(s) ({} current, {} captured); evidence {}; {}.",
        group_kind.as_str(),
        counts.total_count,
        counts.current_scope_count,
        counts.captured_scope_count,
        evidence_class.as_str(),
        if group_kind.mutates_on_apply() {
            "changed on apply".to_owned()
        } else {
            format!(
                "held: {}",
                omission_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        },
    );
    RenameCandidateGroup {
        group_kind,
        candidate_refs,
        counts,
        evidence_class,
        omission_reasons,
        conflict_notes,
        provider_classes,
        labels,
        downgrade_reasons,
        fallback_notes,
        mutates_on_apply: group_kind.mutates_on_apply(),
        remains_visible_when_redacted: true,
        summary,
    }
}

fn build_apply_gate(
    preview_id: &str,
    totals: &RenameCandidateCounts,
    candidates: &[&RenameCandidate],
) -> RenameApplyGate {
    let apply_posture = derive_apply_posture(totals, candidates);
    let preconditions = derive_preconditions(totals, candidates);
    let apply_allowed_after_preview = apply_posture
        == RenameApplyPosture::ReadyForApplyAfterPreview
        && totals.will_change_count > 0;
    let summary = format!(
        "Inspect before mutate: a blind apply is blocked, {} candidate(s) would change and {} are \
         held visibly; posture {}; {} precondition(s) to clear.",
        totals.will_change_count,
        totals.held_count(),
        apply_posture_token(apply_posture),
        preconditions.len(),
    );
    RenameApplyGate {
        apply_posture,
        inspect_before_mutate_required: true,
        blind_apply_blocked: true,
        apply_allowed_after_preview,
        will_change_count: totals.will_change_count,
        held_count: totals.held_count(),
        preconditions,
        omitted_candidates_remain_visible: true,
        redacted_candidates_remain_visible: true,
        undo_checkpoint_ref: format!("aureline://undo/rename/{preview_id}"),
        summary,
    }
}

fn build_projection(
    surface: ConsumerSurface,
    group_count: usize,
    redaction_class: ExportRedactionClass,
) -> RenamePreviewProjection {
    RenamePreviewProjection {
        consumer_surface: surface,
        projected_group_count: group_count,
        preserves_candidate_grouping: true,
        preserves_counts: true,
        preserves_omission_reasons: true,
        preserves_conflict_notes: true,
        preserves_apply_gate: true,
        preserves_undo_checkpoint: true,
        omitted_candidates_remain_visible: true,
        flattens_to_single_apply_action: false,
        exports_code_bodies: false,
        redaction_class,
        summary: format!(
            "{} consumes the preview with candidate grouping, change-versus-held counts, omission \
             reasons, conflict notes, the inspect-before-mutate apply gate, and the undo checkpoint \
             preserved; never flattened into one apply action.",
            surface.as_str()
        ),
    }
}

fn build_preview_set(
    input: &RenamePreviewInput,
    totals: &RenameCandidateCounts,
    candidates: &[&RenameCandidate],
    apply_gate: &RenameApplyGate,
    conflict_notes: &[String],
) -> RenamePreviewSet {
    let candidate_occurrence_refs: Vec<String> = candidates
        .iter()
        .map(|candidate| candidate.candidate_id.clone())
        .collect();
    // The held set — everything the rename would not change — is the honest answer to
    // "what would this rename not touch".
    let blocked_refs: Vec<String> = candidates
        .iter()
        .filter(|candidate| candidate.group_kind() != RenameCandidateGroupKind::Editable)
        .map(|candidate| candidate.candidate_id.clone())
        .collect();
    let sparse_or_partial_reasons = sparse_or_partial_reasons_for(candidates);
    let generated_scope_notes = generated_scope_notes_for(candidates);

    let count_summary = NavigationTargetCountSummary {
        changed_count: totals.will_change_count,
        unresolved_count: totals.unresolved_count,
        generated_count: totals.generated_count,
        protected_count: totals.read_only_count,
        skipped_count: totals.blocked_count
            + totals.conflict_count
            + (totals.partial_scope_omitted_count - totals.unresolved_count),
    };

    let proof_class = weakest_proof_class(candidates);
    let confidence = weakest_confidence(candidates);
    let freshness = weakest_freshness(candidates);
    let scope_completeness = weakest_scope_completeness(candidates);

    let evidence_refs = union_evidence_refs(candidates);

    let summary = format!(
        "Rename-preview set over {} candidate(s): {} changed, {} held; posture {}.",
        candidate_occurrence_refs.len(),
        count_summary.changed_count,
        blocked_refs.len(),
        apply_posture_token(apply_gate.apply_posture),
    );

    RenamePreviewSet {
        rename_preview_id: input.preview_id.clone(),
        root_target_ref: input.root_target_ref.clone(),
        candidate_occurrence_refs,
        blocked_refs,
        conflict_notes: conflict_notes.to_vec(),
        sparse_or_partial_reasons,
        generated_scope_notes,
        count_summary,
        proof_class,
        confidence,
        freshness,
        scope_completeness,
        apply_posture: apply_gate.apply_posture,
        redaction_class: input.redaction_class,
        evidence_refs,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Apply posture and preconditions.
// ---------------------------------------------------------------------------

fn derive_apply_posture(
    totals: &RenameCandidateCounts,
    candidates: &[&RenameCandidate],
) -> RenameApplyPosture {
    let blocking = totals.blocked_count > 0 || totals.conflict_count > 0;
    if blocking {
        let policy_or_protected = candidates.iter().any(|candidate| {
            candidate
                .block_reason
                .is_some_and(RenameOmissionReason::is_policy_or_protected)
        });
        return if policy_or_protected {
            RenameApplyPosture::BlockedPendingPolicyOrProtectedReview
        } else {
            RenameApplyPosture::BlockedPendingScopeReview
        };
    }
    if needs_refresh(candidates) {
        return RenameApplyPosture::BlockedPendingRefresh;
    }
    if totals.will_change_count == 0 {
        return RenameApplyPosture::InspectOnlyUnavailable;
    }
    RenameApplyPosture::ReadyForApplyAfterPreview
}

fn derive_preconditions(
    totals: &RenameCandidateCounts,
    candidates: &[&RenameCandidate],
) -> Vec<RenameApplyPrecondition> {
    let mut out = Vec::new();
    if totals.blocked_count > 0 {
        out.push(RenameApplyPrecondition::ReviewBlockedCandidates);
    }
    if totals.conflict_count > 0 {
        out.push(RenameApplyPrecondition::ResolveConflicts);
    }
    if totals.generated_count > 0 {
        out.push(RenameApplyPrecondition::AcknowledgeGeneratedBoundary);
    }
    if totals.read_only_count > 0 {
        out.push(RenameApplyPrecondition::AcknowledgeReadOnlyOmission);
    }
    if totals.partial_scope_omitted_count > 0 {
        out.push(RenameApplyPrecondition::AcknowledgePartialScope);
    }
    if needs_refresh(candidates) {
        out.push(RenameApplyPrecondition::RefreshStaleScope);
    }
    if candidates.iter().any(|candidate| {
        candidate
            .downgrade_reasons
            .iter()
            .any(|reason| matches!(reason, DowngradeReason::SparseWorkset))
    }) {
        out.push(RenameApplyPrecondition::WidenSparseScope);
    }
    out
}

fn needs_refresh(candidates: &[&RenameCandidate]) -> bool {
    candidates.iter().any(|candidate| {
        matches!(
            candidate.freshness,
            FreshnessClass::DegradedCached | FreshnessClass::Stale | FreshnessClass::Unverified
        ) || candidate.scope_completeness == ScopeCompleteness::StaleForDeclaredScope
    })
}

// ---------------------------------------------------------------------------
// Derivations.
// ---------------------------------------------------------------------------

/// Returns true when a candidate is carried only by a captured scope — a snapshot,
/// runtime trace, or imported pack — rather than re-proven against current source.
fn is_captured_only(candidate: &RenameCandidate) -> bool {
    matches!(
        candidate.proof_class,
        ProofClass::ImportedEvidence | ProofClass::RuntimeObserved
    ) || candidate.generated_or_external_state == GeneratedOrExternalState::ImportedSnapshot
        || matches!(
            candidate.freshness,
            FreshnessClass::Stale | FreshnessClass::Unverified
        )
}

/// Returns true when a candidate is generated or a paired artifact.
fn is_generated(candidate: &RenameCandidate) -> bool {
    candidate.access_kind == AccessKind::Generated
        || candidate.generated_or_external_state == GeneratedOrExternalState::GeneratedSource
}

/// Returns true when a candidate is read-only, external, or an imported snapshot.
fn is_read_only_or_external(candidate: &RenameCandidate) -> bool {
    matches!(
        candidate.generated_or_external_state,
        GeneratedOrExternalState::ReadOnlySource
            | GeneratedOrExternalState::ExternalDependency
            | GeneratedOrExternalState::ImportedSnapshot
    )
}

fn evidence_class_of_proof(proof: ProofClass) -> RenameEvidenceClass {
    match proof {
        ProofClass::DirectSemantic | ProofClass::IndexedSemantic => RenameEvidenceClass::Semantic,
        ProofClass::FrameworkDerived => RenameEvidenceClass::FrameworkDerived,
        ProofClass::RuntimeObserved => RenameEvidenceClass::RuntimeObserved,
        ProofClass::ImportedEvidence => RenameEvidenceClass::ImportedSnapshot,
        ProofClass::LexicalFallback => RenameEvidenceClass::LexicalFallback,
        ProofClass::SyntaxFallback => RenameEvidenceClass::SyntaxFallback,
        ProofClass::AiInferred | ProofClass::Unavailable => RenameEvidenceClass::Unavailable,
    }
}

fn evidence_class_for(members: &[&RenameCandidate]) -> RenameEvidenceClass {
    if members.is_empty() {
        return RenameEvidenceClass::Unavailable;
    }
    let mut classes = BTreeSet::new();
    for candidate in members {
        classes.insert(evidence_class_of_proof(candidate.proof_class));
    }
    if classes.len() == 1 {
        classes.into_iter().next().unwrap()
    } else {
        RenameEvidenceClass::Mixed
    }
}

fn omission_reasons_for(
    group_kind: RenameCandidateGroupKind,
    members: &[&RenameCandidate],
) -> Vec<RenameOmissionReason> {
    let mut reasons: Vec<RenameOmissionReason> = Vec::new();
    for candidate in members {
        match group_kind {
            RenameCandidateGroupKind::Editable => {}
            RenameCandidateGroupKind::BlockedForReview => {
                push_unique(
                    &mut reasons,
                    candidate
                        .block_reason
                        .unwrap_or(RenameOmissionReason::BlockedPendingReview),
                );
            }
            RenameCandidateGroupKind::Conflict => {
                push_unique(
                    &mut reasons,
                    RenameOmissionReason::ConflictPendingResolution,
                );
            }
            RenameCandidateGroupKind::GeneratedBoundary => {
                push_unique(&mut reasons, RenameOmissionReason::GeneratedBoundary);
            }
            RenameCandidateGroupKind::ReadOnlyOrExternal => {
                if candidate.generated_or_external_state
                    == GeneratedOrExternalState::ExternalDependency
                {
                    push_unique(&mut reasons, RenameOmissionReason::ExternalDependency);
                } else {
                    push_unique(&mut reasons, RenameOmissionReason::ReadOnlyOrProtected);
                }
            }
            RenameCandidateGroupKind::PartialScopeOmitted => {
                if !candidate.anchor_resolved {
                    push_unique(&mut reasons, RenameOmissionReason::UnresolvedAnchor);
                }
                if candidate
                    .downgrade_reasons
                    .iter()
                    .any(|reason| matches!(reason, DowngradeReason::SparseWorkset))
                {
                    push_unique(&mut reasons, RenameOmissionReason::OutOfScopeSparse);
                }
                if matches!(
                    candidate.scope_completeness,
                    ScopeCompleteness::PartialForDeclaredScope
                ) {
                    push_unique(&mut reasons, RenameOmissionReason::PartiallyLoaded);
                }
                if matches!(
                    candidate.freshness,
                    FreshnessClass::Stale | FreshnessClass::Unverified
                ) || candidate.scope_completeness == ScopeCompleteness::StaleForDeclaredScope
                {
                    push_unique(&mut reasons, RenameOmissionReason::StaleScope);
                }
                if reasons.is_empty() {
                    push_unique(&mut reasons, RenameOmissionReason::PartiallyLoaded);
                }
            }
        }
    }
    reasons
}

fn union_omission_reasons(groups: &[RenameCandidateGroup]) -> Vec<RenameOmissionReason> {
    let mut reasons: Vec<RenameOmissionReason> = Vec::new();
    for group in groups {
        for reason in &group.omission_reasons {
            push_unique(&mut reasons, *reason);
        }
    }
    reasons
}

fn union_conflict_notes(members: &[&RenameCandidate]) -> Vec<String> {
    let mut notes = Vec::new();
    for candidate in members {
        if let Some(note) = &candidate.conflict_note {
            if !notes.contains(note) {
                notes.push(note.clone());
            }
        }
    }
    notes
}

fn provider_classes_for(members: &[&RenameCandidate]) -> Vec<ProviderClass> {
    let mut providers: Vec<ProviderClass> = Vec::new();
    for candidate in members {
        if !providers.contains(&candidate.provider_class) {
            providers.push(candidate.provider_class);
        }
    }
    providers
}

fn labels_for(members: &[&RenameCandidate]) -> Vec<RenameCandidateLabel> {
    let mut labels = BTreeSet::new();
    for candidate in members {
        if is_generated(candidate) {
            labels.insert(RenameCandidateLabel::Generated);
        }
        match candidate.generated_or_external_state {
            GeneratedOrExternalState::ExternalDependency => {
                labels.insert(RenameCandidateLabel::External);
            }
            GeneratedOrExternalState::ReadOnlySource => {
                labels.insert(RenameCandidateLabel::ReadOnly);
            }
            GeneratedOrExternalState::ImportedSnapshot => {
                labels.insert(RenameCandidateLabel::ImportedSnapshot);
            }
            _ => {}
        }
        // Imported-snapshot evidence keeps a captured-scope label even when the
        // underlying source is authored, external, or read-only, so a candidate proven
        // only by an imported snapshot never reads as current.
        if candidate.proof_class == ProofClass::ImportedEvidence {
            labels.insert(RenameCandidateLabel::ImportedSnapshot);
        }
        if candidate.access_kind == AccessKind::TestOnly {
            labels.insert(RenameCandidateLabel::TestOnly);
        }
        match candidate.proof_class {
            ProofClass::LexicalFallback => {
                labels.insert(RenameCandidateLabel::LexicalFallback);
            }
            ProofClass::SyntaxFallback => {
                labels.insert(RenameCandidateLabel::SyntaxFallback);
            }
            ProofClass::RuntimeObserved => {
                labels.insert(RenameCandidateLabel::RuntimeObserved);
            }
            ProofClass::FrameworkDerived => {
                labels.insert(RenameCandidateLabel::FrameworkDerived);
            }
            _ => {}
        }
        if candidate.conflict_note.is_some() {
            labels.insert(RenameCandidateLabel::Conflict);
        }
        if candidate.block_reason.is_some() {
            labels.insert(RenameCandidateLabel::Blocked);
        }
        if candidate.is_out_of_scope() {
            labels.insert(RenameCandidateLabel::OutOfScope);
        }
        if !candidate.anchor_resolved {
            labels.insert(RenameCandidateLabel::Unresolved);
        }
    }
    if !members.is_empty() && members.iter().all(|candidate| is_captured_only(candidate)) {
        labels.insert(RenameCandidateLabel::CapturedScopeOnly);
    }
    labels.into_iter().collect()
}

fn downgrade_reasons_for(members: &[&RenameCandidate]) -> Vec<DowngradeReason> {
    let mut reasons: Vec<DowngradeReason> = Vec::new();
    for candidate in members {
        for reason in &candidate.downgrade_reasons {
            push_unique(&mut reasons, *reason);
        }
    }
    for candidate in members {
        match candidate.proof_class {
            ProofClass::LexicalFallback => {
                push_unique(&mut reasons, DowngradeReason::LexicalFallbackOnly);
            }
            ProofClass::SyntaxFallback => {
                push_unique(&mut reasons, DowngradeReason::SyntaxFallbackOnly);
            }
            ProofClass::RuntimeObserved | ProofClass::FrameworkDerived => {
                push_unique(&mut reasons, DowngradeReason::RuntimeOrFrameworkOnly);
            }
            _ => {}
        }
        if is_generated(candidate) {
            push_unique(&mut reasons, DowngradeReason::GeneratedBoundary);
        }
        if candidate.block_reason.is_some() {
            push_unique(&mut reasons, DowngradeReason::PolicyLimited);
        }
        if candidate.conflict_note.is_some() {
            push_unique(&mut reasons, DowngradeReason::AmbiguousCandidates);
        }
    }
    reasons
}

fn fallback_notes_for(members: &[&RenameCandidate]) -> Vec<String> {
    let mut notes = Vec::new();
    let count = |predicate: &dyn Fn(&RenameCandidate) -> bool| {
        members
            .iter()
            .filter(|candidate| predicate(candidate))
            .count()
    };

    let lexical = count(&|candidate| candidate.proof_class == ProofClass::LexicalFallback);
    if lexical > 0 {
        notes.push(format!(
            "{lexical} candidate(s) rest on a lexical/grep fallback and are disclosed as such, \
             never renamed as if semantic certainty."
        ));
    }
    let syntax = count(&|candidate| candidate.proof_class == ProofClass::SyntaxFallback);
    if syntax > 0 {
        notes.push(format!(
            "{syntax} candidate(s) rest on a syntax-only fallback and stay labeled as a fallback."
        ));
    }
    let runtime = count(&|candidate| candidate.proof_class == ProofClass::RuntimeObserved);
    if runtime > 0 {
        notes.push(format!(
            "{runtime} candidate(s) are runtime-observed from a captured trace, not static source."
        ));
    }
    let framework = count(&|candidate| candidate.proof_class == ProofClass::FrameworkDerived);
    if framework > 0 {
        notes.push(format!(
            "{framework} candidate(s) are framework-derived from route/generator metadata."
        ));
    }
    let imported = count(&|candidate| candidate.proof_class == ProofClass::ImportedEvidence);
    if imported > 0 {
        notes.push(format!(
            "{imported} candidate(s) come from an imported snapshot and are captured-scope only."
        ));
    }
    let captured = count(&|candidate| is_captured_only(candidate));
    if captured > 0 {
        notes.push(format!(
            "{captured} candidate(s) are carried only by a captured scope and are not re-proven \
             against current source."
        ));
    }
    notes
}

fn sparse_or_partial_reasons_for(members: &[&RenameCandidate]) -> Vec<String> {
    let mut notes = Vec::new();
    let unresolved = members
        .iter()
        .filter(|candidate| !candidate.anchor_resolved)
        .count();
    if unresolved > 0 {
        notes.push(format!(
            "{unresolved} candidate(s) could not be anchored and are held, not silently dropped."
        ));
    }
    let partial = members
        .iter()
        .filter(|candidate| {
            matches!(
                candidate.scope_completeness,
                ScopeCompleteness::PartialForDeclaredScope
            )
        })
        .count();
    if partial > 0 {
        notes.push(format!(
            "{partial} candidate(s) are partially loaded for the declared scope."
        ));
    }
    let sparse = members
        .iter()
        .filter(|candidate| {
            candidate
                .downgrade_reasons
                .iter()
                .any(|reason| matches!(reason, DowngradeReason::SparseWorkset))
        })
        .count();
    if sparse > 0 {
        notes.push(format!(
            "{sparse} candidate(s) sit outside the current sparse workset."
        ));
    }
    let stale = members
        .iter()
        .filter(|candidate| {
            matches!(
                candidate.freshness,
                FreshnessClass::Stale | FreshnessClass::Unverified
            ) || candidate.scope_completeness == ScopeCompleteness::StaleForDeclaredScope
        })
        .count();
    if stale > 0 {
        notes.push(format!(
            "{stale} candidate(s) rest on a stale scope and need a refresh before apply."
        ));
    }
    notes
}

fn generated_scope_notes_for(members: &[&RenameCandidate]) -> Vec<String> {
    let generated = members
        .iter()
        .filter(|candidate| is_generated(candidate))
        .count();
    if generated > 0 {
        vec![format!(
            "{generated} candidate(s) sit across a generated boundary; rename the source, not the \
             generated artifact."
        )]
    } else {
        Vec::new()
    }
}

fn union_evidence_refs(members: &[&RenameCandidate]) -> Vec<String> {
    let mut refs = Vec::new();
    for candidate in members {
        for evidence in &candidate.evidence_refs {
            if !refs.contains(evidence) {
                refs.push(evidence.clone());
            }
        }
    }
    refs
}

fn push_unique<T: PartialEq>(items: &mut Vec<T>, item: T) {
    if !items.contains(&item) {
        items.push(item);
    }
}

// Weakest-class reducers, so the projected `RenamePreviewSet` never claims stronger
// proof, confidence, freshness, or scope than its weakest candidate.

fn weakest_proof_class(members: &[&RenameCandidate]) -> ProofClass {
    let severity = |proof: ProofClass| match proof {
        ProofClass::DirectSemantic => 0,
        ProofClass::IndexedSemantic => 1,
        ProofClass::FrameworkDerived => 2,
        ProofClass::RuntimeObserved => 3,
        ProofClass::ImportedEvidence => 4,
        ProofClass::SyntaxFallback => 5,
        ProofClass::LexicalFallback => 6,
        ProofClass::AiInferred => 7,
        ProofClass::Unavailable => 8,
    };
    members
        .iter()
        .map(|candidate| candidate.proof_class)
        .max_by_key(|proof| severity(*proof))
        .unwrap_or(ProofClass::Unavailable)
}

fn weakest_confidence(members: &[&RenameCandidate]) -> NavigationConfidence {
    let severity = |confidence: NavigationConfidence| match confidence {
        NavigationConfidence::Exact => 0,
        NavigationConfidence::Indexed => 1,
        NavigationConfidence::Imported => 2,
        NavigationConfidence::Partial => 3,
        NavigationConfidence::WorkspaceSliceLimited => 4,
        NavigationConfidence::Heuristic => 5,
        NavigationConfidence::Stale => 6,
        NavigationConfidence::Unavailable => 7,
    };
    members
        .iter()
        .map(|candidate| candidate.confidence)
        .max_by_key(|confidence| severity(*confidence))
        .unwrap_or(NavigationConfidence::Unavailable)
}

fn weakest_freshness(members: &[&RenameCandidate]) -> FreshnessClass {
    let severity = |freshness: FreshnessClass| match freshness {
        FreshnessClass::AuthoritativeLive => 0,
        FreshnessClass::WarmCached => 1,
        FreshnessClass::DegradedCached => 2,
        FreshnessClass::Stale => 3,
        FreshnessClass::Unverified => 4,
    };
    members
        .iter()
        .map(|candidate| candidate.freshness)
        .max_by_key(|freshness| severity(*freshness))
        .unwrap_or(FreshnessClass::Unverified)
}

fn weakest_scope_completeness(members: &[&RenameCandidate]) -> ScopeCompleteness {
    let severity = |scope: ScopeCompleteness| match scope {
        ScopeCompleteness::CompleteForDeclaredScope => 0,
        ScopeCompleteness::PartialForDeclaredScope => 1,
        ScopeCompleteness::StaleForDeclaredScope => 2,
        ScopeCompleteness::UnavailableForDeclaredScope => 3,
    };
    members
        .iter()
        .map(|candidate| candidate.scope_completeness)
        .max_by_key(|scope| severity(*scope))
        .unwrap_or(ScopeCompleteness::UnavailableForDeclaredScope)
}

/// Returns the stable token for an apply posture, for summaries and the lines view.
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

// ---------------------------------------------------------------------------
// Frozen corpus.
// ---------------------------------------------------------------------------

/// One frozen preview scenario: an input, the preview the builder produces for it, and
/// the property the scenario proves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenamePreviewScenario {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Plain-language title.
    pub title: String,
    /// The preview-building input.
    pub input: RenamePreviewInput,
    /// The preview `build_rename_preview` produces for the input.
    pub preview: GovernedRenamePreview,
    /// One reviewable sentence stating what the scenario proves.
    pub expectation_note: String,
}

/// One frozen invariant over the corpus, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenamePreviewInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built corpus satisfies the invariant.
    pub holds: bool,
}

/// The frozen rename-preview corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenamePreviewGovernanceSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub rename_preview_schema_version: u32,
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
    /// The frozen preview scenarios.
    pub scenarios: Vec<RenamePreviewScenario>,
    /// The computed invariants.
    pub invariants: Vec<RenamePreviewInvariant>,
    /// Whether raw source bodies and payloads are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the corpus fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenamePreviewValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for RenamePreviewValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rename-preview corpus invalid: {}", self.reason)
    }
}

impl std::error::Error for RenamePreviewValidationError {}

impl RenamePreviewGovernanceSet {
    /// Returns the scenario with a given id, if present.
    pub fn scenario(&self, scenario_id: &str) -> Option<&RenamePreviewScenario> {
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
            let input = &scenario.input;
            refs.push(input.root_target_ref.as_str());
            refs.push(input.current_name_ref.as_str());
            refs.push(input.proposed_name_ref.as_str());
            refs.push(input.scope_ref.as_str());
            if let Some(captured) = &input.captured_scope_ref {
                refs.push(captured.as_str());
            }
            for candidate in &input.candidates {
                refs.push(candidate.target_ref.as_str());
                refs.push(candidate.anchor_ref.as_str());
                refs.push(candidate.scope_ref.as_str());
                refs.extend(candidate.evidence_refs.iter().map(String::as_str));
            }
            let preview = &scenario.preview;
            refs.push(preview.root_target_ref.as_str());
            refs.push(preview.current_name_ref.as_str());
            refs.push(preview.proposed_name_ref.as_str());
            refs.push(preview.scope_ref.as_str());
            if let Some(captured) = &preview.captured_scope_ref {
                refs.push(captured.as_str());
            }
            refs.push(preview.apply_gate.undo_checkpoint_ref.as_str());
            refs.push(preview.preview_set.root_target_ref.as_str());
            refs.extend(preview.preview_set.evidence_refs.iter().map(String::as_str));
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), RenamePreviewValidationError> {
        let fail = |reason: String| Err(RenamePreviewValidationError { reason });

        if self.record_kind != RENAME_PREVIEW_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != RENAME_PREVIEW_SCHEMA_REF {
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

        // Every scenario's stored preview equals what the builder produces, so the
        // fixture cannot drift from the builder.
        for scenario in &self.scenarios {
            let produced = build_rename_preview(&scenario.input);
            if produced != scenario.preview {
                return fail(format!(
                    "scenario {} preview drifted from builder output",
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

/// Builds the canonical rename-preview corpus.
///
/// Deterministic: the same bytes every call. Each scenario's preview is the builder's
/// own output, and the invariant `holds` flags are computed from those previews, so a
/// regression in [`build_rename_preview`] flips an invariant or drifts the fixture
/// rather than silently passing.
pub fn rename_preview_set() -> RenamePreviewGovernanceSet {
    let scenarios = build_scenarios();
    let invariants = compute_invariants(&scenarios);

    RenamePreviewGovernanceSet {
        record_kind: RENAME_PREVIEW_RECORD_KIND.to_owned(),
        rename_preview_schema_version: RENAME_PREVIEW_SCHEMA_VERSION,
        schema_ref: RENAME_PREVIEW_SCHEMA_REF.to_owned(),
        set_id: RENAME_PREVIEW_SET_ID.to_owned(),
        as_of: RENAME_PREVIEW_AS_OF.to_owned(),
        freeze_gate_ref: RENAME_PREVIEW_FREEZE_GATE_REF.to_owned(),
        summary: "Frozen rename-preview corpus: every broad rename is a governed preview that groups \
                  candidates into the editable set and the held set (blocked, conflict, generated, \
                  read-only, partial-scope), separates change-versus-held and current-versus-captured \
                  counts, keeps every omitted candidate visible with its omission reason and label, \
                  names whether evidence is semantic, framework-derived, runtime-observed, imported, \
                  or a lexical fallback, enforces an inspect-before-mutate apply gate that always \
                  blocks a blind apply and binds an undo checkpoint, projects the frozen \
                  rename-preview-set object, and reaches review, support, AI, graph, docs, and editor \
                  consumers without flattening into one generic apply action or exporting code bodies."
            .to_owned(),
        scenarios,
        invariants,
        raw_payload_excluded: true,
    }
}

/// Renders the corpus as human-readable lines for CLI/headless and support.
pub fn rename_preview_lines(set: &RenamePreviewGovernanceSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Rename-preview corpus — {} ({})",
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
        let preview = &scenario.preview;
        lines.push(format!("  - {} [{}]", scenario.scenario_id, scenario.title));
        lines.push(format!(
            "      groups={} total={} will_change={} held={} current={} captured={} evidence={}",
            preview.groups.len(),
            preview.totals.total_count,
            preview.totals.will_change_count,
            preview.totals.held_count(),
            preview.totals.current_scope_count,
            preview.totals.captured_scope_count,
            preview.preview_evidence_class.as_str(),
        ));
        let group_summary = preview
            .groups
            .iter()
            .map(|group| format!("{}:{}", group.group_kind.as_str(), group.counts.total_count))
            .collect::<Vec<_>>()
            .join(" ");
        lines.push(format!("      {group_summary}"));
        lines.push(format!(
            "      apply: posture={} inspect_before_mutate={} blind_blocked={} allowed_after_preview={} preconditions={}",
            apply_posture_token(preview.apply_gate.apply_posture),
            preview.apply_gate.inspect_before_mutate_required,
            preview.apply_gate.blind_apply_blocked,
            preview.apply_gate.apply_allowed_after_preview,
            preview
                .apply_gate
                .preconditions
                .iter()
                .map(|precondition| precondition.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ));
        if !preview.omission_reasons.is_empty() {
            lines.push(format!(
                "      omitted={}",
                preview
                    .omission_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
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

/// Compact seed for a [`RenameCandidate`], so each scenario reads as a small table
/// rather than a wall of struct fields.
struct CandSeed {
    candidate_id: &'static str,
    access: AccessKind,
    proof: ProofClass,
    provider: ProviderClass,
    confidence: NavigationConfidence,
    freshness: FreshnessClass,
    scope: ScopeCompleteness,
    generated: GeneratedOrExternalState,
    block: Option<RenameOmissionReason>,
    conflict: Option<&'static str>,
    anchor_resolved: bool,
    downgrades: &'static [DowngradeReason],
    summary: &'static str,
}

fn candidate(root: &str, seed: CandSeed) -> RenameCandidate {
    let anchor = if seed.anchor_resolved {
        format!("aureline://anchor/{}", seed.candidate_id)
    } else {
        format!("aureline://anchor/unresolved/{}", seed.candidate_id)
    };
    RenameCandidate {
        candidate_id: seed.candidate_id.to_owned(),
        target_ref: root.to_owned(),
        anchor_ref: anchor,
        access_kind: seed.access,
        scope_ref: "aureline://scope/workspace".to_owned(),
        generated_or_external_state: seed.generated,
        proof_class: seed.proof,
        provider_class: seed.provider,
        confidence: seed.confidence,
        freshness: seed.freshness,
        scope_completeness: seed.scope,
        block_reason: seed.block,
        conflict_note: seed.conflict.map(str::to_owned),
        anchor_resolved: seed.anchor_resolved,
        downgrade_reasons: seed.downgrades.to_vec(),
        evidence_refs: vec![format!("aureline://evidence/{}", seed.candidate_id)],
        summary: seed.summary.to_owned(),
    }
}

fn input(
    preview_id: &str,
    root: &str,
    captured_scope_ref: Option<&str>,
    redaction_class: ExportRedactionClass,
    candidates: Vec<RenameCandidate>,
) -> RenamePreviewInput {
    RenamePreviewInput {
        preview_id: preview_id.to_owned(),
        root_target_ref: format!("aureline://object/{root}"),
        current_name_ref: format!("aureline://rename/{preview_id}/current-name"),
        proposed_name_ref: format!("aureline://rename/{preview_id}/proposed-name"),
        scope_ref: "aureline://scope/workspace".to_owned(),
        captured_scope_ref: captured_scope_ref.map(str::to_owned),
        redaction_class,
        candidates,
    }
}

fn scenario(
    scenario_id: &str,
    title: &str,
    input: RenamePreviewInput,
    expectation_note: &str,
) -> RenamePreviewScenario {
    let preview = build_rename_preview(&input);
    RenamePreviewScenario {
        scenario_id: scenario_id.to_owned(),
        title: title.to_owned(),
        input,
        preview,
        expectation_note: expectation_note.to_owned(),
    }
}

fn build_scenarios() -> Vec<RenamePreviewScenario> {
    use AccessKind::*;
    use ExportRedactionClass::*;
    use FreshnessClass::*;
    use NavigationConfidence::*;
    use ProofClass::*;
    use ProviderClass::*;
    use ScopeCompleteness::*;

    let authored = GeneratedOrExternalState::AuthoredSource;

    vec![
        // 1. A clean, fully editable rename — still inspect-before-mutate.
        scenario(
            "rename.clean_editable",
            "Clean rename: every candidate is editable, apply only after preview",
            input(
                "rename:handler:0001",
                "symbol.handler",
                None,
                MetadataSafeDefault,
                vec![
                    candidate(
                        "aureline://object/symbol.handler",
                        CandSeed {
                            candidate_id: "cand.handler.decl",
                            access: Write,
                            proof: DirectSemantic,
                            provider: LanguageServer,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[],
                            summary: "Declaration site of the handler.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.handler",
                        CandSeed {
                            candidate_id: "cand.handler.ref.1",
                            access: Read,
                            proof: IndexedSemantic,
                            provider: ProjectGraph,
                            confidence: Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[],
                            summary: "Reference in the router.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.handler",
                        CandSeed {
                            candidate_id: "cand.handler.ref.2",
                            access: Call,
                            proof: DirectSemantic,
                            provider: LanguageServer,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[],
                            summary: "Call site in the dispatcher.",
                        },
                    ),
                ],
            ),
            "All candidates are editable with semantic proof, so the preview is ready to apply after \
             inspection, yet a blind apply is still blocked and the change set is named explicitly.",
        ),
        // 2. Blocked (policy), generated, and read-only omissions stay visible.
        scenario(
            "rename.blocked_generated_readonly",
            "Blocked, generated, and read-only candidates are held and stay visible",
            input(
                "rename:config:0002",
                "symbol.config",
                None,
                InternalSupportRestricted,
                vec![
                    candidate(
                        "aureline://object/symbol.config",
                        CandSeed {
                            candidate_id: "cand.config.edit",
                            access: Write,
                            proof: DirectSemantic,
                            provider: LanguageServer,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[],
                            summary: "Editable definition of config.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.config",
                        CandSeed {
                            candidate_id: "cand.config.policy",
                            access: Read,
                            proof: IndexedSemantic,
                            provider: ProjectGraph,
                            confidence: Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            block: Some(RenameOmissionReason::PolicyLimited),
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[DowngradeReason::PolicyLimited],
                            summary: "Reference held by a protected-path policy.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.config",
                        CandSeed {
                            candidate_id: "cand.config.blocked",
                            access: Write,
                            proof: DirectSemantic,
                            provider: LanguageServer,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            block: Some(RenameOmissionReason::BlockedPendingReview),
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[],
                            summary: "Write held for explicit review before a broad mutation.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.config",
                        CandSeed {
                            candidate_id: "cand.config.generated",
                            access: Generated,
                            proof: FrameworkDerived,
                            provider: GeneratedSourceBridge,
                            confidence: Imported,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: GeneratedOrExternalState::GeneratedSource,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[DowngradeReason::GeneratedBoundary],
                            summary: "Generated accessor in a build artifact.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.config",
                        CandSeed {
                            candidate_id: "cand.config.readonly",
                            access: Read,
                            proof: IndexedSemantic,
                            provider: ProjectGraph,
                            confidence: Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: GeneratedOrExternalState::ReadOnlySource,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[],
                            summary: "Reference in read-only vendored source.",
                        },
                    ),
                ],
            ),
            "A policy-blocked, a generated, and a read-only candidate are each held in their own \
             group with a visible omission reason, so the rename never silently changes or drops \
             them; the apply posture routes through policy/protected review.",
        ),
        // 3. Conflict / shadowing candidates are held for resolution.
        scenario(
            "rename.conflict_shadowing",
            "Shadowing and alias conflicts are held pending resolution",
            input(
                "rename:value:0003",
                "symbol.value",
                None,
                MetadataSafeDefault,
                vec![
                    candidate(
                        "aureline://object/symbol.value",
                        CandSeed {
                            candidate_id: "cand.value.edit",
                            access: Write,
                            proof: DirectSemantic,
                            provider: LanguageServer,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[],
                            summary: "Editable definition of value.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.value",
                        CandSeed {
                            candidate_id: "cand.value.shadow",
                            access: Read,
                            proof: DirectSemantic,
                            provider: LanguageServer,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            block: None,
                            conflict: Some(
                                "Proposed name shadows a local binding in the same scope.",
                            ),
                            anchor_resolved: true,
                            downgrades: &[],
                            summary: "Reference where the new name would shadow a local.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.value",
                        CandSeed {
                            candidate_id: "cand.value.alias",
                            access: Read,
                            proof: IndexedSemantic,
                            provider: ProjectGraph,
                            confidence: Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            block: None,
                            conflict: Some("Proposed name collides with an existing alias import."),
                            anchor_resolved: true,
                            downgrades: &[],
                            summary: "Reference where the new name collides with an alias.",
                        },
                    ),
                ],
            ),
            "Shadowing and alias-collision candidates are held in the conflict group with their \
             conflict notes preserved, and the apply posture blocks the rename pending scope review.",
        ),
        // 4. Stale / degraded scope and an unresolved anchor — refresh before apply.
        scenario(
            "rename.stale_unresolved_refresh",
            "Stale scope and an unresolved anchor require a refresh before apply",
            input(
                "rename:service:0004",
                "symbol.service",
                Some("aureline://scope/captured-trace"),
                MetadataSafeDefault,
                vec![
                    candidate(
                        "aureline://object/symbol.service",
                        CandSeed {
                            candidate_id: "cand.service.edit.degraded",
                            access: Write,
                            proof: IndexedSemantic,
                            provider: RemoteIndex,
                            confidence: Indexed,
                            freshness: DegradedCached,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[DowngradeReason::StaleShard],
                            summary: "Editable reference proven from a degraded cache.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.service",
                        CandSeed {
                            candidate_id: "cand.service.runtime.unresolved",
                            access: Call,
                            proof: RuntimeObserved,
                            provider: RuntimeObserver,
                            confidence: Imported,
                            freshness: FreshnessClass::Stale,
                            scope: PartialForDeclaredScope,
                            generated: authored,
                            block: None,
                            conflict: None,
                            anchor_resolved: false,
                            downgrades: &[DowngradeReason::RuntimeOrFrameworkOnly],
                            summary: "Call observed in a stale runtime trace whose anchor did not resolve.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.service",
                        CandSeed {
                            candidate_id: "cand.service.runtime.partial",
                            access: Call,
                            proof: RuntimeObserved,
                            provider: RuntimeObserver,
                            confidence: Imported,
                            freshness: DegradedCached,
                            scope: PartialForDeclaredScope,
                            generated: authored,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[DowngradeReason::RuntimeOrFrameworkOnly],
                            summary: "Call observed in a captured runtime trace for a partial scope.",
                        },
                    ),
                ],
            ),
            "A degraded-cache editable candidate sits alongside two runtime-observed candidates — one \
             with an unresolved anchor — so current-versus-captured counts split, the unresolved and \
             partial candidates are held visibly, and the apply posture demands a refresh before \
             mutation.",
        ),
        // 5. Lexical fallback and sparse scope omitted, but the editable set still applies.
        scenario(
            "rename.fallback_sparse_visible",
            "Lexical fallback and sparse candidates are omitted but stay visible",
            input(
                "rename:macro:0005",
                "symbol.macro_target",
                None,
                MetadataSafeDefault,
                vec![
                    candidate(
                        "aureline://object/symbol.macro_target",
                        CandSeed {
                            candidate_id: "cand.macro.edit",
                            access: Write,
                            proof: DirectSemantic,
                            provider: LanguageServer,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[],
                            summary: "Editable definition of the macro target.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.macro_target",
                        CandSeed {
                            candidate_id: "cand.macro.lexical_sparse",
                            access: Read,
                            proof: LexicalFallback,
                            provider: Syntax,
                            confidence: Heuristic,
                            freshness: WarmCached,
                            scope: PartialForDeclaredScope,
                            generated: authored,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[
                                DowngradeReason::LexicalFallbackOnly,
                                DowngradeReason::SparseWorkset,
                            ],
                            summary: "Lexical match for a macro-expanded reference outside the sparse workset.",
                        },
                    ),
                ],
            ),
            "A lexical/grep fallback candidate that also sits outside the sparse workset is held in \
             the partial-scope group with its fallback and sparse reasons disclosed, while the \
             editable definition still applies after preview — a grep match is never renamed as if \
             semantic.",
        ),
        // 6. Inspect-only: nothing is editable, but everything stays visible.
        scenario(
            "rename.inspect_only_nothing_editable",
            "When nothing is editable the rename is inspect-only, not silently empty",
            input(
                "rename:external:0006",
                "symbol.external_api",
                None,
                MetadataSafeDefault,
                vec![
                    candidate(
                        "aureline://object/symbol.external_api",
                        CandSeed {
                            candidate_id: "cand.external.dep",
                            access: Import,
                            proof: ImportedEvidence,
                            provider: ImportedSnapshot,
                            confidence: Imported,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: GeneratedOrExternalState::ExternalDependency,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[],
                            summary: "Import from an external dependency package.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.external_api",
                        CandSeed {
                            candidate_id: "cand.external.readonly",
                            access: Read,
                            proof: ImportedEvidence,
                            provider: ImportedSnapshot,
                            confidence: Imported,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: GeneratedOrExternalState::ReadOnlySource,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[],
                            summary: "Reference in read-only vendored source proven from an imported snapshot.",
                        },
                    ),
                    candidate(
                        "aureline://object/symbol.external_api",
                        CandSeed {
                            candidate_id: "cand.external.generated",
                            access: Generated,
                            proof: FrameworkDerived,
                            provider: GeneratedSourceBridge,
                            confidence: Imported,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: GeneratedOrExternalState::GeneratedSource,
                            block: None,
                            conflict: None,
                            anchor_resolved: true,
                            downgrades: &[DowngradeReason::GeneratedBoundary],
                            summary: "Generated binding for the external API.",
                        },
                    ),
                ],
            ),
            "Every candidate is external, read-only, or generated, so nothing can be safely renamed; \
             the preview is inspect-only-unavailable yet still lists every held candidate and why.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> RenamePreviewInvariant {
    RenamePreviewInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(scenarios: &[RenamePreviewScenario]) -> Vec<RenamePreviewInvariant> {
    let previews: Vec<&GovernedRenamePreview> = scenarios.iter().map(|s| &s.preview).collect();

    let mut out = Vec::new();

    // Disjoint grouping: every candidate lands in exactly one group keyed by its
    // precedence group kind, and groups are in canonical order.
    out.push(invariant(
        "rename_preview.candidate_grouping_disjoint",
        "Every preview groups its candidates by the fixed editable/blocked/conflict/generated/\
         read-only/partial-scope precedence in canonical order, places each candidate in exactly \
         the group its precedence selects, and never folds a held candidate into the editable set.",
        scenarios.iter().all(|scenario| {
            let preview = &scenario.preview;
            groups_in_canonical_order(preview)
                && total_grouped(preview) == scenario.input.candidates.len()
                && preview.groups.iter().all(|group| {
                    scenario.input.candidates.iter().all(|candidate| {
                        if candidate.group_kind() == group.group_kind {
                            group.candidate_refs.contains(&candidate.candidate_id)
                        } else {
                            !group.candidate_refs.contains(&candidate.candidate_id)
                        }
                    })
                })
        }),
    ));

    // Counts reconcile across groups and the preview.
    out.push(invariant(
        "rename_preview.counts_reconcile",
        "Every group and preview reconciles its disjoint change-versus-held counts and its \
         current-versus-captured split with its total, and the group totals sum to the preview \
         total, so the count of what changes and what is held is always internally consistent.",
        previews.iter().all(|preview| {
            preview.totals.reconciles()
                && preview.groups.iter().all(|group| group.counts.reconciles())
                && preview
                    .groups
                    .iter()
                    .map(|group| group.counts.total_count)
                    .sum::<usize>()
                    == preview.totals.total_count
                && preview
                    .groups
                    .iter()
                    .map(|group| group.counts.captured_scope_count)
                    .sum::<usize>()
                    == preview.totals.captured_scope_count
                && preview.totals.will_change_count + preview.totals.held_count()
                    == preview.totals.total_count
        }),
    ));

    // Omissions stay visible: no held candidate disappears, each carries a reason and label.
    out.push(invariant(
        "rename_preview.omissions_visible",
        "Every blocked, conflicting, generated, read-only, or partial-scope candidate stays in the \
         preview with a visible omission reason on its group and a visible label, so no held \
         candidate ever disappears silently from a broad rename.",
        scenarios.iter().all(|scenario| {
            let preview = &scenario.preview;
            scenario.input.candidates.iter().all(|candidate| {
                let kind = candidate.group_kind();
                if kind == RenameCandidateGroupKind::Editable {
                    return true;
                }
                preview.group(kind).is_some_and(|group| {
                    group.candidate_refs.contains(&candidate.candidate_id)
                        && !group.omission_reasons.is_empty()
                }) && expected_labels_for(candidate)
                    .iter()
                    .all(|label| preview.labels.contains(label))
            })
        }),
    ));

    // Evidence class is disclosed and fallbacks never masquerade as semantic.
    out.push(invariant(
        "rename_preview.evidence_class_disclosed_no_grep_as_semantic",
        "Every preview and group names its evidence class, and any group resting on a lexical or \
         syntax fallback carries a fallback note and a downgrade reason, so a grep fallback \
         candidate is never renamed as if it were semantic certainty.",
        previews.iter().all(|preview| {
            preview.groups.iter().all(|group| {
                !group.evidence_class.is_fallback()
                    || (!group.fallback_notes.is_empty() && !group.downgrade_reasons.is_empty())
            }) && (!preview.preview_evidence_class.is_fallback()
                || !preview.fallback_notes.is_empty())
        }),
    ));

    // Conflict notes are preserved on the group and preview.
    out.push(invariant(
        "rename_preview.conflict_notes_preserved",
        "Every conflicting candidate keeps its conflict note on the conflict group and on the \
         preview, so a shadowing or alias collision is never dropped before a broad rename.",
        scenarios.iter().all(|scenario| {
            let preview = &scenario.preview;
            scenario
                .input
                .candidates
                .iter()
                .filter_map(|candidate| candidate.conflict_note.as_ref())
                .all(|note| {
                    preview.conflict_notes.contains(note)
                        && preview
                            .group(RenameCandidateGroupKind::Conflict)
                            .is_some_and(|group| group.conflict_notes.contains(note))
                })
        }),
    ));

    // Inspect-before-mutate is enforced on every preview.
    out.push(invariant(
        "rename_preview.inspect_before_mutate_enforced",
        "Every preview's apply gate requires inspection before mutation, blocks a blind apply, keeps \
         omitted and redacted candidates visible, binds an undo checkpoint, and only ever mutates \
         the editable group, so a broad rename can never collapse into one opaque apply action.",
        previews.iter().all(|preview| {
            let gate = &preview.apply_gate;
            gate.inspect_before_mutate_required
                && gate.blind_apply_blocked
                && gate.omitted_candidates_remain_visible
                && gate.redacted_candidates_remain_visible
                && !gate.undo_checkpoint_ref.is_empty()
                && gate.will_change_count == preview.totals.will_change_count
                && gate.held_count == preview.totals.held_count()
                && preview
                    .groups
                    .iter()
                    .all(|group| group.mutates_on_apply == group.group_kind.mutates_on_apply())
        }),
    ));

    // Apply posture matches the group state.
    out.push(invariant(
        "rename_preview.apply_posture_matches_groups",
        "The apply posture blocks apply whenever a blocked or conflict group exists, a stale scope \
         needs refresh, or nothing is editable, and only reports ready-after-preview when an \
         editable set exists with no blocking group or refresh need — so the posture can never \
         claim a rename is safe to apply while candidates are held.",
        previews.iter().all(|preview| {
            let blocking = preview.totals.blocked_count > 0 || preview.totals.conflict_count > 0;
            let ready =
                preview.apply_gate.apply_posture == RenameApplyPosture::ReadyForApplyAfterPreview;
            let allowed = preview.apply_gate.apply_allowed_after_preview;
            // Ready iff allowed iff not blocking-or-refresh-or-empty.
            ready == allowed
                && (!ready || (!blocking && preview.totals.will_change_count > 0))
                && (ready || preview.apply_gate.apply_posture.blocks_apply())
        }),
    ));

    // Partial scope and captured divergence is always disclosed.
    out.push(invariant(
        "rename_preview.partial_scope_truth",
        "Whenever a preview holds partial-scope candidates or has captured-only candidates it \
         carries a captured scope ref or a downgrade reason, a partial/out-of-scope or captured \
         label, and fallback or sparse notes, so partial and captured truth is never hidden.",
        previews.iter().all(|preview| {
            let has_partial = preview.totals.partial_scope_omitted_count > 0;
            let has_captured = preview.totals.captured_scope_count > 0;
            if !has_partial && !has_captured {
                return true;
            }
            let disclosed_ref =
                preview.captured_scope_ref.is_some() || !preview.downgrade_reasons.is_empty();
            let disclosed_label = preview.labels.iter().any(|label| {
                matches!(
                    label,
                    RenameCandidateLabel::OutOfScope
                        | RenameCandidateLabel::Unresolved
                        | RenameCandidateLabel::CapturedScopeOnly
                        | RenameCandidateLabel::ImportedSnapshot
                        | RenameCandidateLabel::RuntimeObserved
                        | RenameCandidateLabel::LexicalFallback
                )
            });
            disclosed_ref && disclosed_label && !preview.fallback_notes.is_empty()
        }),
    ));

    // The projected RenamePreviewSet stays consistent with the governed preview.
    out.push(invariant(
        "rename_preview.preview_set_consistent",
        "The projected rename-preview-set object carries the same apply posture, the same changed \
         count, and a held set covering every non-editable candidate, so the frozen target-model \
         object and the governed preview can never disagree about what the rename would change.",
        scenarios.iter().all(|scenario| {
            let preview = &scenario.preview;
            let set = &preview.preview_set;
            let held: Vec<&String> = scenario
                .input
                .candidates
                .iter()
                .filter(|candidate| candidate.group_kind() != RenameCandidateGroupKind::Editable)
                .map(|candidate| &candidate.candidate_id)
                .collect();
            set.apply_posture == preview.apply_gate.apply_posture
                && set.count_summary.changed_count == preview.totals.will_change_count
                && set.candidate_occurrence_refs.len() == scenario.input.candidates.len()
                && held
                    .iter()
                    .all(|candidate| set.blocked_refs.contains(candidate))
                && set.blocked_refs.len() == held.len()
        }),
    ));

    // Consumers preserve the typed truth without flattening into one apply action.
    out.push(invariant(
        "rename_preview.consumers_preserve_truth",
        "Every consumer projection preserves the candidate grouping, counts, omission reasons, \
         conflict notes, apply gate, and undo checkpoint, keeps omitted candidates visible, never \
         flattens the rename into one apply action, and never exports raw code bodies, so review, \
         support, AI, graph, docs, and editor consumers see the governed rename rather than a button.",
        previews.iter().all(|preview| {
            !preview.consumer_projections.is_empty()
                && preview
                    .consumer_projections
                    .iter()
                    .all(RenamePreviewProjection::preserves_truth)
                && required_surfaces_covered(&preview.consumer_projections)
        }),
    ));

    // The corpus covers every group kind, posture, omission reason, and precondition.
    out.push(invariant(
        "rename_preview.corpus_covers_vocabulary",
        "The corpus exercises every candidate group kind, every apply posture, every omission \
         reason, every apply precondition, and the semantic, framework, runtime, imported, and \
         lexical evidence answers, so the governed rename model is proven across its whole vocabulary.",
        every_group_kind_covered(&previews)
            && every_posture_covered(&previews)
            && every_omission_reason_covered(&previews)
            && every_precondition_covered(&previews)
            && every_evidence_answer_covered(&previews),
    ));

    // The preview is replayable and answers the support question.
    out.push(invariant(
        "rename_preview.replayable_support_answer",
        "Every preview carries a non-empty id and summary, a named preview evidence class, a \
         definition root relation, and change-versus-held counts, so a support or debug packet can \
         reconstruct what the rename would change, what it would not, and why.",
        previews.iter().all(|preview| {
            !preview.preview_id.trim().is_empty()
                && !preview.summary.trim().is_empty()
                && preview.root_relation_kind == RelationKind::Definition
                && preview.totals.will_change_count + preview.totals.held_count()
                    == preview.totals.total_count
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Invariant helpers.
// ---------------------------------------------------------------------------

fn groups_in_canonical_order(preview: &GovernedRenamePreview) -> bool {
    let order = |group_kind: RenameCandidateGroupKind| {
        RENAME_GROUP_ORDER
            .iter()
            .position(|candidate| *candidate == group_kind)
            .unwrap_or(usize::MAX)
    };
    preview
        .groups
        .windows(2)
        .all(|pair| order(pair[0].group_kind) < order(pair[1].group_kind))
}

fn total_grouped(preview: &GovernedRenamePreview) -> usize {
    preview
        .groups
        .iter()
        .map(|group| group.candidate_refs.len())
        .sum()
}

fn expected_labels_for(candidate: &RenameCandidate) -> Vec<RenameCandidateLabel> {
    let mut labels = Vec::new();
    if is_generated(candidate) {
        labels.push(RenameCandidateLabel::Generated);
    }
    match candidate.generated_or_external_state {
        GeneratedOrExternalState::ExternalDependency => labels.push(RenameCandidateLabel::External),
        GeneratedOrExternalState::ReadOnlySource => labels.push(RenameCandidateLabel::ReadOnly),
        GeneratedOrExternalState::ImportedSnapshot => {
            labels.push(RenameCandidateLabel::ImportedSnapshot)
        }
        _ => {}
    }
    if candidate.conflict_note.is_some() {
        labels.push(RenameCandidateLabel::Conflict);
    }
    if candidate.block_reason.is_some() {
        labels.push(RenameCandidateLabel::Blocked);
    }
    if !candidate.anchor_resolved {
        labels.push(RenameCandidateLabel::Unresolved);
    }
    labels
}

fn every_group_kind_covered(previews: &[&GovernedRenamePreview]) -> bool {
    RENAME_GROUP_ORDER.iter().all(|group_kind| {
        previews
            .iter()
            .any(|preview| preview.group(*group_kind).is_some())
    })
}

fn every_posture_covered(previews: &[&GovernedRenamePreview]) -> bool {
    let postures = [
        RenameApplyPosture::ReadyForApplyAfterPreview,
        RenameApplyPosture::BlockedPendingScopeReview,
        RenameApplyPosture::BlockedPendingRefresh,
        RenameApplyPosture::BlockedPendingPolicyOrProtectedReview,
        RenameApplyPosture::InspectOnlyUnavailable,
    ];
    postures.iter().all(|posture| {
        previews
            .iter()
            .any(|preview| preview.apply_gate.apply_posture == *posture)
    })
}

fn every_omission_reason_covered(previews: &[&GovernedRenamePreview]) -> bool {
    RenameOmissionReason::ALL.iter().all(|reason| {
        previews
            .iter()
            .any(|preview| preview.omission_reasons.contains(reason))
    })
}

fn every_precondition_covered(previews: &[&GovernedRenamePreview]) -> bool {
    RenameApplyPrecondition::ALL.iter().all(|precondition| {
        previews
            .iter()
            .any(|preview| preview.apply_gate.preconditions.contains(precondition))
    })
}

fn every_evidence_answer_covered(previews: &[&GovernedRenamePreview]) -> bool {
    let answers = [
        RenameEvidenceClass::Semantic,
        RenameEvidenceClass::FrameworkDerived,
        RenameEvidenceClass::RuntimeObserved,
        RenameEvidenceClass::ImportedSnapshot,
        RenameEvidenceClass::LexicalFallback,
    ];
    answers.iter().all(|answer| {
        previews.iter().any(|preview| {
            preview.preview_evidence_class == *answer
                || preview
                    .groups
                    .iter()
                    .any(|group| group.evidence_class == *answer)
        })
    })
}

fn required_surfaces_covered(projections: &[RenamePreviewProjection]) -> bool {
    REQUIRED_CONSUMER_SURFACES.iter().all(|surface| {
        projections
            .iter()
            .any(|projection| projection.consumer_surface == *surface)
    })
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative path or opaque `aureline://`
/// handle, never a URL, host, credential, or absolute path.
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
