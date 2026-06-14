//! Scope-compatible selection objects, portable selector parity across UI / CLI /
//! AI / support packets, and widened-selection review for the M5 rerun and
//! triage lanes.
//!
//! Where [`crate::durable_test_items_and_partial_discovery`] lands the durable
//! discovery objects (nodes and snapshots) the framework, notebook, and
//! test-tree consumers normalize onto, this module lands the **selection objects**
//! that make those durable targets safely *selectable* and *re-runnable* across
//! surfaces. A selection is a durable product object, not an ad hoc display-name
//! match:
//!
//! * a [`SelectionObject`] ties a chosen set of [`SelectionTarget`]s to the
//!   [`SnapshotFingerprint`] it was resolved against, an include / exclude
//!   [`SelectionQuery`] with an optional `changed_since` ref, and an
//!   [`ExpansionPolicy`] that says whether re-resolving the selection may widen
//!   beyond the originally pinned targets;
//! * rerun, rerun-failed, CLI selectors, AI test plans, and support / export
//!   packets all normalize onto the *same* [`SelectionObject`] through a shared
//!   [`SelectorChannel`] and [`SelectionIntentKind`] vocabulary rather than each
//!   surface re-deriving a brittle name list;
//! * before a rerun executes, [`SelectionObject::assess_against`] compares the
//!   selection to the *current* discovery state and produces a
//!   [`SelectionCompatibilityAssessment`]. When the snapshot drifted, a target
//!   fingerprint changed, or re-resolution would widen / narrow the set, the
//!   assessment reports a [`TargetCompatibilityClass`] other than
//!   [`TargetCompatibilityClass::Compatible`] and forces a
//!   [`WidenedSelectionReviewState`] decision — so a rerun can never silently
//!   expand beyond what the user or packet originally meant.
//!
//! [`PortableSelectionPacket::validate`] refuses a packet that lets a display
//! label / id stand in for a target's durable fingerprint, collapses a
//! parameterized template into a concrete invocation, lets an imported /
//! provider-backed selection read as a local rerun, hides a widening or snapshot
//! drift behind a no-review-required state, or records an assessment whose
//! selection / target fingerprint cannot be reconstructed from the export.
//!
//! Raw test source, raw provider payloads, raw query bodies, provider cursors,
//! credentials, and raw artifact bodies never cross this boundary; the packet
//! carries only typed class tokens, booleans, opaque ids, fingerprint digests,
//! and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/testing/scope-compatible-selection-objects-and-widened-selection-review.schema.json`](../../../../schemas/testing/scope-compatible-selection-objects-and-widened-selection-review.schema.json).
//! The contract doc is
//! [`docs/testing/m5/scope-compatible-selection-objects-and-widened-selection-review.md`](../../../../docs/testing/m5/scope-compatible-selection-objects-and-widened-selection-review.md).
//! The protected fixture directory is
//! [`fixtures/testing/m5/scope-compatible-selection-objects-and-widened-selection-review/`](../../../../fixtures/testing/m5/scope-compatible-selection-objects-and-widened-selection-review/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use crate::testing_identity::TestItemIdentityClass;

/// Stable record-kind tag carried by [`PortableSelectionPacket`].
pub const PORTABLE_SELECTION_RECORD_KIND: &str = "portable_test_selection_packet";

/// Schema version for the portable selection packet.
pub const PORTABLE_SELECTION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PORTABLE_SELECTION_SCHEMA_REF: &str =
    "schemas/testing/scope-compatible-selection-objects-and-widened-selection-review.schema.json";

/// Repo-relative path of the contract doc.
pub const PORTABLE_SELECTION_DOC_REF: &str =
    "docs/testing/m5/scope-compatible-selection-objects-and-widened-selection-review.md";

/// Repo-relative path of the checked support-export artifact.
pub const PORTABLE_SELECTION_ARTIFACT_REF: &str =
    "artifacts/testing/m5/scope-compatible-selection-objects-and-widened-selection-review/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PORTABLE_SELECTION_SUMMARY_REF: &str =
    "artifacts/testing/m5/scope-compatible-selection-objects-and-widened-selection-review.md";

/// Repo-relative path of the protected fixture directory.
pub const PORTABLE_SELECTION_FIXTURE_DIR: &str =
    "fixtures/testing/m5/scope-compatible-selection-objects-and-widened-selection-review";

/// Stable token for an imported-CI snapshot consumer, matched against a
/// [`SnapshotFingerprint::consumer_token`].
pub const IMPORTED_CI_CONSUMER_TOKEN: &str = "imported_ci";

/// Closed vocabulary for the portability channels a selection object travels
/// across. The same selection object normalizes rerun, rerun-failed, CLI
/// selectors, AI test plans, and support / export packets so no surface re-derives
/// a brittle display-name list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectorChannel {
    /// An interactive UI surface (test tree, command palette, review sheet).
    Ui,
    /// A CLI / headless selector.
    Cli,
    /// An AI test-plan proposal.
    Ai,
    /// A support / export packet reconstructing a selection.
    Support,
}

impl SelectorChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 4] = [Self::Ui, Self::Cli, Self::Ai, Self::Support];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ui => "ui",
            Self::Cli => "cli",
            Self::Ai => "ai",
            Self::Support => "support",
        }
    }
}

/// Closed vocabulary naming what a selection means. Rerun, rerun-failed,
/// changed-since, and snapshot-scoped selectors all normalize onto one selection
/// object rather than ad hoc name matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionIntentKind {
    /// An explicit set of chosen targets.
    ExplicitItems,
    /// Rerun the entire originating selection.
    RerunAll,
    /// Rerun only the failed subset of the originating selection.
    RerunFailed,
    /// Select everything changed since a recorded ref.
    ChangedSince,
    /// Select everything in a discovery-snapshot scope matching the query.
    SnapshotScoped,
}

impl SelectionIntentKind {
    /// Every intent, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExplicitItems,
        Self::RerunAll,
        Self::RerunFailed,
        Self::ChangedSince,
        Self::SnapshotScoped,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitItems => "explicit_items",
            Self::RerunAll => "rerun_all",
            Self::RerunFailed => "rerun_failed",
            Self::ChangedSince => "changed_since",
            Self::SnapshotScoped => "snapshot_scoped",
        }
    }

    /// True when the intent requires a `changed_since` ref in its query.
    pub const fn requires_changed_since_ref(self) -> bool {
        matches!(self, Self::ChangedSince)
    }

    /// True when the intent is query-driven over a snapshot scope and so requires
    /// at least one include query token.
    pub const fn requires_include_query(self) -> bool {
        matches!(self, Self::SnapshotScoped)
    }
}

/// Closed vocabulary for how a selection may be re-resolved against a newer
/// discovery snapshot. The policy is the contract that decides whether a rerun
/// may widen, must stay pinned, or must fail closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpansionPolicy {
    /// Re-run only the exact pinned target fingerprints; never widen.
    PinnedExact,
    /// Re-resolve the query, but only within the same snapshot fingerprint.
    ReresolveWithinSnapshot,
    /// Re-resolution may widen the set, but any widening opens review first.
    AllowWidenWithReview,
    /// Imported / provider-owned; never re-dispatched as a local rerun.
    FrozenImportedReadOnly,
}

impl ExpansionPolicy {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinnedExact => "pinned_exact",
            Self::ReresolveWithinSnapshot => "reresolve_within_snapshot",
            Self::AllowWidenWithReview => "allow_widen_with_review",
            Self::FrozenImportedReadOnly => "frozen_imported_read_only",
        }
    }

    /// True when the policy marks the selection as imported / read-only.
    pub const fn is_frozen_imported(self) -> bool {
        matches!(self, Self::FrozenImportedReadOnly)
    }
}

/// Closed result vocabulary for assessing a selection against the current
/// discovery state before a rerun. Only [`Self::Compatible`] preserves the
/// originating selection without review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetCompatibilityClass {
    /// Snapshot and target fingerprints match; the originating selection is
    /// preserved exactly and may dispatch without review.
    Compatible,
    /// Re-resolution would add targets beyond the originally pinned set; the
    /// widening must be reviewed before execution.
    WidenedNeedsReview,
    /// Re-resolution would drop originally pinned targets; the narrowing must be
    /// reviewed so the loss is explicit.
    NarrowedNeedsReview,
    /// The discovery snapshot fingerprint changed since the selection was made.
    SnapshotDrifted,
    /// A target's fingerprint changed (e.g. its parameter set) even though its id
    /// matched.
    TargetFingerprintMismatch,
    /// The selection is imported / provider-owned and cannot be a local rerun.
    ImportedNotRerunnable,
}

impl TargetCompatibilityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::WidenedNeedsReview => "widened_needs_review",
            Self::NarrowedNeedsReview => "narrowed_needs_review",
            Self::SnapshotDrifted => "snapshot_drifted",
            Self::TargetFingerprintMismatch => "target_fingerprint_mismatch",
            Self::ImportedNotRerunnable => "imported_not_rerunnable",
        }
    }

    /// True when the class preserves the originating selection without review.
    pub const fn preserves_origin_without_review(self) -> bool {
        matches!(self, Self::Compatible)
    }
}

/// Closed vocabulary for the state of a widened-selection review. A drifted or
/// widened rerun cannot dispatch while the review is [`Self::Pending`] and must
/// fail closed when [`Self::Blocked`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidenedSelectionReviewState {
    /// No review is required; the selection is compatible and preserved.
    NotRequired,
    /// A review sheet is open and awaiting a decision; dispatch is blocked.
    Pending,
    /// The reviewer accepted the re-resolved (wider / narrower) scope.
    ApprovedAsAdjusted,
    /// The reviewer rejected the change and kept the original pinned scope.
    RejectedKeepOriginal,
    /// The selection cannot be re-dispatched locally (imported / provider-owned).
    Blocked,
}

impl WidenedSelectionReviewState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Pending => "pending",
            Self::ApprovedAsAdjusted => "approved_as_adjusted",
            Self::RejectedKeepOriginal => "rejected_keep_original",
            Self::Blocked => "blocked",
        }
    }

    /// True when the state is a review that is open or has been decided (i.e. a
    /// drift / widening actually opened review) rather than `not_required` or
    /// `blocked`.
    pub const fn is_open_or_decided(self) -> bool {
        matches!(
            self,
            Self::Pending | Self::ApprovedAsAdjusted | Self::RejectedKeepOriginal
        )
    }

    /// True when the decided state allows a rerun to dispatch.
    pub const fn allows_dispatch(self) -> bool {
        matches!(self, Self::ApprovedAsAdjusted | Self::RejectedKeepOriginal)
    }
}

/// Non-display fingerprint of the discovery snapshot a selection was resolved
/// against. Carries only opaque ids and a digest — never raw enumeration bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotFingerprint {
    /// Discovery snapshot id the selection was resolved against.
    pub snapshot_id: String,
    /// Non-display digest of the snapshot's enumerated node set at resolution
    /// time. Drift in this digest forces a widened-selection review.
    pub snapshot_digest: String,
    /// Stable consumer token (`framework_pack`, `notebook`, `test_tree`, or
    /// `imported_ci`) recording which consumer produced the snapshot.
    pub consumer_token: String,
}

impl SnapshotFingerprint {
    /// Whether the fingerprint carries the ids and digest a rerun or support
    /// export needs.
    pub fn is_valid(&self) -> bool {
        !self.snapshot_id.trim().is_empty()
            && !self.snapshot_digest.trim().is_empty()
            && !self.consumer_token.trim().is_empty()
    }

    /// Whether this fingerprint is for an imported / provider-owned snapshot.
    pub fn is_imported(&self) -> bool {
        self.consumer_token == IMPORTED_CI_CONSUMER_TOKEN
    }

    /// Whether this fingerprint matches another by both snapshot id and digest.
    pub fn matches(&self, other: &SnapshotFingerprint) -> bool {
        self.snapshot_id == other.snapshot_id && self.snapshot_digest == other.snapshot_digest
    }
}

/// One resolved target inside a selection, addressed by a durable node id and a
/// non-display fingerprint rather than a display label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionTarget {
    /// Durable node id of the selected target.
    pub target_id: String,
    /// Node kind, reusing the frozen durable-discovery vocabulary so a
    /// parameterized template never collapses into a concrete invocation.
    pub node_kind: DurableTestNodeKind,
    /// Non-display fingerprint token of the resolved target (e.g. a parameter
    /// set + source-anchor digest). Must differ from
    /// [`target_id`](SelectionTarget::target_id) so neither a label nor a bare id
    /// stands in for the durable fingerprint.
    pub target_fingerprint_token: String,
    /// Identity stability, reusing the frozen identity vocabulary.
    pub identity_class: TestItemIdentityClass,
}

impl SelectionTarget {
    /// Whether this target is imported / provider-owned and read-only.
    pub fn is_imported(&self) -> bool {
        self.identity_class == TestItemIdentityClass::ImportedReadOnly
    }

    /// Whether the target's fingerprint is a real non-display basis distinct from
    /// its id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.target_fingerprint_token.trim();
        !token.is_empty() && token != self.target_id.trim()
    }

    /// Whether the target carries the durable identity a portable rerun needs.
    pub fn is_valid(&self) -> bool {
        !self.target_id.trim().is_empty()
            && self.fingerprint_independent_of_id()
            && self.identity_class != TestItemIdentityClass::DisplayTextOnlyDenied
    }
}

/// Include / exclude query and optional changed-since ref captured with a
/// selection so a re-resolution is reproducible.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SelectionQuery {
    /// Include query tokens (redaction-safe, never raw query bodies).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include_query_tokens: Vec<String>,
    /// Exclude query tokens.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_query_tokens: Vec<String>,
    /// Changed-since ref used by a `changed_since` selection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_since_ref: Option<String>,
}

impl SelectionQuery {
    /// Whether every recorded token is non-empty.
    pub fn well_formed_tokens(&self) -> bool {
        self.include_query_tokens
            .iter()
            .chain(self.exclude_query_tokens.iter())
            .all(|t| !t.trim().is_empty())
            && self
                .changed_since_ref
                .as_ref()
                .map_or(true, |r| !r.trim().is_empty())
    }
}

/// A durable selection object that ties chosen targets to the snapshot they were
/// resolved against, the query that produced them, and an expansion policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionObject {
    /// Stable selection id.
    pub selection_id: String,
    /// Human-readable selection label. Never the identity basis.
    pub label: String,
    /// Channel that produced this selection.
    pub origin_channel: SelectorChannel,
    /// What the selection means.
    pub intent: SelectionIntentKind,
    /// Re-resolution / expansion policy.
    pub expansion_policy: ExpansionPolicy,
    /// Fingerprint of the discovery snapshot the selection was resolved against.
    pub snapshot_fingerprint: SnapshotFingerprint,
    /// Include / exclude query and optional changed-since ref.
    pub query: SelectionQuery,
    /// Targets pinned at selection time.
    pub pinned_targets: Vec<SelectionTarget>,
    /// Evidence packet refs backing this selection.
    pub evidence_refs: Vec<String>,
}

impl SelectionObject {
    /// Pinned target ids.
    pub fn target_ids(&self) -> BTreeSet<&str> {
        self.pinned_targets
            .iter()
            .map(|t| t.target_id.as_str())
            .collect()
    }

    /// Whether every pinned target id is unique.
    pub fn target_ids_unique(&self) -> bool {
        self.target_ids().len() == self.pinned_targets.len()
    }

    /// Whether this selection is imported / provider-owned (by snapshot consumer,
    /// expansion policy, or any pinned target).
    pub fn is_imported(&self) -> bool {
        self.snapshot_fingerprint.is_imported()
            || self.expansion_policy.is_frozen_imported()
            || self.pinned_targets.iter().any(SelectionTarget::is_imported)
    }

    /// Whether the intent's query requirements hold.
    pub fn intent_query_consistent(&self) -> bool {
        let changed_since_ok =
            !self.intent.requires_changed_since_ref() || self.query.changed_since_ref.is_some();
        let include_ok =
            !self.intent.requires_include_query() || !self.query.include_query_tokens.is_empty();
        changed_since_ok && include_ok
    }

    /// Whether an imported selection is frozen read-only, so an imported result
    /// never normalizes into a re-dispatchable local rerun.
    pub fn imported_policy_ok(&self) -> bool {
        if self.is_imported() {
            self.expansion_policy.is_frozen_imported()
        } else {
            !self.expansion_policy.is_frozen_imported()
        }
    }

    /// Whether every field required to record this selection is present and its
    /// invariants hold.
    pub fn is_valid(&self) -> bool {
        !self.selection_id.trim().is_empty()
            && !self.label.trim().is_empty()
            && self.snapshot_fingerprint.is_valid()
            && self.query.well_formed_tokens()
            && !self.pinned_targets.is_empty()
            && self.pinned_targets.iter().all(SelectionTarget::is_valid)
            && self.target_ids_unique()
            && self.intent_query_consistent()
            && self.imported_policy_ok()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Assesses this selection against the current discovery state and produces a
    /// [`SelectionCompatibilityAssessment`]. The assessment preserves the
    /// originating selection only when the snapshot and every target fingerprint
    /// still match; any drift, widening, or narrowing forces a review state so a
    /// rerun cannot silently expand beyond what was originally meant.
    pub fn assess_against(
        &self,
        assessment_id: String,
        current_snapshot: SnapshotFingerprint,
        current_targets: &[SelectionTarget],
        evidence_refs: Vec<String>,
    ) -> SelectionCompatibilityAssessment {
        let original_ids: BTreeSet<&str> = self.target_ids();
        let current_ids: BTreeSet<&str> = current_targets
            .iter()
            .map(|t| t.target_id.as_str())
            .collect();

        let added_target_ids: Vec<String> = current_ids
            .difference(&original_ids)
            .map(|id| (*id).to_owned())
            .collect();
        let removed_target_ids: Vec<String> = original_ids
            .difference(&current_ids)
            .map(|id| (*id).to_owned())
            .collect();
        let mut fingerprint_mismatch_target_ids: Vec<String> = self
            .pinned_targets
            .iter()
            .filter_map(|original| {
                current_targets
                    .iter()
                    .find(|cur| cur.target_id == original.target_id)
                    .filter(|cur| cur.target_fingerprint_token != original.target_fingerprint_token)
                    .map(|_| original.target_id.clone())
            })
            .collect();
        fingerprint_mismatch_target_ids.sort();

        let snapshot_drifted = !self.snapshot_fingerprint.matches(&current_snapshot);
        let imported = self.is_imported()
            || current_snapshot.is_imported()
            || current_targets.iter().any(SelectionTarget::is_imported);

        let (compatibility_class, review_state) = if imported {
            (
                TargetCompatibilityClass::ImportedNotRerunnable,
                WidenedSelectionReviewState::Blocked,
            )
        } else if snapshot_drifted {
            (
                TargetCompatibilityClass::SnapshotDrifted,
                WidenedSelectionReviewState::Pending,
            )
        } else if !fingerprint_mismatch_target_ids.is_empty() {
            (
                TargetCompatibilityClass::TargetFingerprintMismatch,
                WidenedSelectionReviewState::Pending,
            )
        } else if !added_target_ids.is_empty() {
            (
                TargetCompatibilityClass::WidenedNeedsReview,
                WidenedSelectionReviewState::Pending,
            )
        } else if !removed_target_ids.is_empty() {
            (
                TargetCompatibilityClass::NarrowedNeedsReview,
                WidenedSelectionReviewState::Pending,
            )
        } else {
            (
                TargetCompatibilityClass::Compatible,
                WidenedSelectionReviewState::NotRequired,
            )
        };

        let preserves_origin = compatibility_class.preserves_origin_without_review();

        SelectionCompatibilityAssessment {
            assessment_id,
            selection_ref: self.selection_id.clone(),
            current_snapshot_fingerprint: current_snapshot,
            compatibility_class,
            review_state,
            snapshot_drifted,
            added_target_ids,
            removed_target_ids,
            fingerprint_mismatch_target_ids,
            preserves_origin,
            evidence_refs,
        }
    }
}

/// The outcome of assessing a [`SelectionObject`] against the current discovery
/// state before a rerun or triage path executes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionCompatibilityAssessment {
    /// Stable assessment id.
    pub assessment_id: String,
    /// Selection id this assessment evaluates. Lets support reconstruct the exact
    /// selection used for a rerun or triage path.
    pub selection_ref: String,
    /// Fingerprint of the snapshot the selection was re-resolved against.
    pub current_snapshot_fingerprint: SnapshotFingerprint,
    /// Compatibility class of the re-resolution.
    pub compatibility_class: TargetCompatibilityClass,
    /// Widened-selection review state.
    pub review_state: WidenedSelectionReviewState,
    /// True when the snapshot fingerprint drifted from the selection's original.
    pub snapshot_drifted: bool,
    /// Target ids a re-resolution would add beyond the pinned set.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added_target_ids: Vec<String>,
    /// Pinned target ids a re-resolution would drop.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed_target_ids: Vec<String>,
    /// Pinned target ids whose fingerprint changed even though the id matched.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fingerprint_mismatch_target_ids: Vec<String>,
    /// True when the dispatched scope equals the originating selection.
    pub preserves_origin: bool,
    /// Evidence packet refs backing this assessment.
    pub evidence_refs: Vec<String>,
}

impl SelectionCompatibilityAssessment {
    /// Whether a rerun may dispatch from this assessment without further review:
    /// only a preserved compatible selection or an explicit review decision.
    pub fn dispatch_allowed(&self) -> bool {
        match self.compatibility_class {
            TargetCompatibilityClass::Compatible
                if self.review_state == WidenedSelectionReviewState::NotRequired =>
            {
                true
            }
            _ => self.review_state.allows_dispatch(),
        }
    }

    /// Whether the recorded class, review state, and target deltas are mutually
    /// consistent — so a widening or drift can never hide behind a no-review
    /// state, and only an explicit decision flips `preserves_origin`.
    pub fn is_consistent(&self) -> bool {
        let added = !self.added_target_ids.is_empty();
        let removed = !self.removed_target_ids.is_empty();
        let mismatch = !self.fingerprint_mismatch_target_ids.is_empty();

        let expected_preserves = self.compatibility_class.preserves_origin_without_review()
            || self.review_state == WidenedSelectionReviewState::RejectedKeepOriginal;
        if self.preserves_origin != expected_preserves {
            return false;
        }

        match self.compatibility_class {
            TargetCompatibilityClass::Compatible => {
                !self.snapshot_drifted
                    && !added
                    && !removed
                    && !mismatch
                    && self.review_state == WidenedSelectionReviewState::NotRequired
            }
            TargetCompatibilityClass::WidenedNeedsReview => {
                !self.snapshot_drifted && added && self.review_state.is_open_or_decided()
            }
            TargetCompatibilityClass::NarrowedNeedsReview => {
                !self.snapshot_drifted
                    && removed
                    && !added
                    && self.review_state.is_open_or_decided()
            }
            TargetCompatibilityClass::TargetFingerprintMismatch => {
                !self.snapshot_drifted && mismatch && self.review_state.is_open_or_decided()
            }
            TargetCompatibilityClass::SnapshotDrifted => {
                self.snapshot_drifted && self.review_state.is_open_or_decided()
            }
            TargetCompatibilityClass::ImportedNotRerunnable => {
                self.review_state == WidenedSelectionReviewState::Blocked
            }
        }
    }

    /// Records a reviewer's decision, recomputing
    /// [`preserves_origin`](SelectionCompatibilityAssessment::preserves_origin).
    /// Keeping the original scope preserves the origin; accepting the adjusted
    /// scope does not.
    pub fn with_decision(mut self, decision: WidenedSelectionReviewState) -> Self {
        self.review_state = decision;
        self.preserves_origin = self.compatibility_class.preserves_origin_without_review()
            || decision == WidenedSelectionReviewState::RejectedKeepOriginal;
        self
    }

    /// Whether every field required to record this assessment is present and the
    /// assessment is internally consistent.
    pub fn is_valid(&self) -> bool {
        !self.assessment_id.trim().is_empty()
            && !self.selection_ref.trim().is_empty()
            && self.current_snapshot_fingerprint.is_valid()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && self.is_consistent()
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionGuardrails {
    /// Parameterized templates stay distinct from their concrete invocations.
    pub templates_distinct_from_invocations: bool,
    /// Imported / provider-backed selections never re-dispatch as local reruns.
    pub imported_never_rerun_as_local: bool,
    /// Any widening of a re-resolved selection opens review first.
    pub widening_opens_review: bool,
    /// Any snapshot drift opens review first.
    pub snapshot_drift_opens_review: bool,
    /// A rerun either preserves the originating selection or is explicitly
    /// reviewed.
    pub origin_preserved_or_reviewed: bool,
    /// The exact selection and target fingerprints are reconstructable from the
    /// export.
    pub selection_reconstructable_from_export: bool,
}

impl SelectionGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.templates_distinct_from_invocations
            && self.imported_never_rerun_as_local
            && self.widening_opens_review
            && self.snapshot_drift_opens_review
            && self.origin_preserved_or_reviewed
            && self.selection_reconstructable_from_export
    }
}

/// Channel projection block: the surfaces that consume the same selection object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionChannelProjection {
    /// The UI consumes these selection objects for rerun / rerun-failed.
    pub ui_consumes_selection: bool,
    /// The CLI consumes these selection objects.
    pub cli_consumes_selection: bool,
    /// AI test plans consume these selection objects.
    pub ai_plan_consumes_selection: bool,
    /// Support export reconstructs selections and target fingerprints.
    pub support_export_reconstructs_selection: bool,
}

impl SelectionChannelProjection {
    /// Whether every channel-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.ui_consumes_selection
            && self.cli_consumes_selection
            && self.ai_plan_consumes_selection
            && self.support_export_reconstructs_selection
    }
}

/// Constructor input for [`PortableSelectionPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableSelectionPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Selection objects across channels and intents.
    pub selections: Vec<SelectionObject>,
    /// Compatibility assessments referencing the selections.
    pub assessments: Vec<SelectionCompatibilityAssessment>,
    /// Guardrail invariants block.
    pub guardrails: SelectionGuardrails,
    /// Channel projection block.
    pub channel_projection: SelectionChannelProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe portable selection packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableSelectionPacket {
    /// Record kind; must equal [`PORTABLE_SELECTION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PORTABLE_SELECTION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Selection objects across channels and intents.
    pub selections: Vec<SelectionObject>,
    /// Compatibility assessments referencing the selections.
    pub assessments: Vec<SelectionCompatibilityAssessment>,
    /// Guardrail invariants block.
    pub guardrails: SelectionGuardrails,
    /// Channel projection block.
    pub channel_projection: SelectionChannelProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PortableSelectionPacket {
    /// Builds a portable selection packet.
    pub fn new(input: PortableSelectionPacketInput) -> Self {
        Self {
            record_kind: PORTABLE_SELECTION_RECORD_KIND.to_owned(),
            schema_version: PORTABLE_SELECTION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            selections: input.selections,
            assessments: input.assessments,
            guardrails: input.guardrails,
            channel_projection: input.channel_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Channels represented by some selection in this packet.
    pub fn represented_channels(&self) -> BTreeSet<SelectorChannel> {
        self.selections.iter().map(|s| s.origin_channel).collect()
    }

    /// Intents represented by some selection in this packet.
    pub fn represented_intents(&self) -> BTreeSet<SelectionIntentKind> {
        self.selections.iter().map(|s| s.intent).collect()
    }

    /// Target node kinds represented across every selection.
    pub fn represented_target_kinds(&self) -> BTreeSet<DurableTestNodeKind> {
        self.selections
            .iter()
            .flat_map(|s| s.pinned_targets.iter().map(|t| t.node_kind))
            .collect()
    }

    /// Count of assessments that opened a widened-selection review.
    pub fn drift_review_count(&self) -> usize {
        self.assessments
            .iter()
            .filter(|a| a.review_state.is_open_or_decided())
            .count()
    }

    /// Resolves a selection by its id.
    pub fn selection(&self, selection_id: &str) -> Option<&SelectionObject> {
        self.selections
            .iter()
            .find(|s| s.selection_id == selection_id)
    }

    /// Validates the portable selection invariants.
    pub fn validate(&self) -> Vec<PortableSelectionViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PORTABLE_SELECTION_RECORD_KIND {
            violations.push(PortableSelectionViolation::WrongRecordKind);
        }
        if self.schema_version != PORTABLE_SELECTION_SCHEMA_VERSION {
            violations.push(PortableSelectionViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PortableSelectionViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_selections(self, &mut violations);
        validate_assessments(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(PortableSelectionViolation::GuardrailsIncomplete);
        }
        if !self.channel_projection.all_hold() {
            violations.push(PortableSelectionViolation::ChannelProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("portable selection packet serializes"),
        ) {
            violations.push(PortableSelectionViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("portable selection packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Scope-Compatible Selection Objects\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Selections: {} across {} / {} channels\n",
            self.selections.len(),
            self.represented_channels().len(),
            SelectorChannel::ALL.len()
        ));
        out.push_str(&format!(
            "- Intents present: {} / {}\n",
            self.represented_intents().len(),
            SelectionIntentKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Assessments: {} ({} opened review)\n",
            self.assessments.len(),
            self.drift_review_count()
        ));
        out.push_str("\n## Selections\n\n");
        for selection in &self.selections {
            out.push_str(&format!(
                "- **{}** ({} / {}): policy `{}`\n",
                selection.selection_id,
                selection.origin_channel.as_str(),
                selection.intent.as_str(),
                selection.expansion_policy.as_str()
            ));
            out.push_str(&format!("  - {}\n", selection.label));
            out.push_str(&format!(
                "  - snapshot `{}` digest `{}` ({} targets)\n",
                selection.snapshot_fingerprint.snapshot_id,
                selection.snapshot_fingerprint.snapshot_digest,
                selection.pinned_targets.len()
            ));
            for target in &selection.pinned_targets {
                out.push_str(&format!(
                    "    - `{}` [{}] fingerprint=`{}` identity=`{}`\n",
                    target.target_id,
                    target.node_kind.as_str(),
                    target.target_fingerprint_token,
                    target.identity_class.as_str()
                ));
            }
        }
        out.push_str("\n## Assessments\n\n");
        for assessment in &self.assessments {
            out.push_str(&format!(
                "- **{}** → `{}`: class `{}`, review `{}` (dispatch {})\n",
                assessment.assessment_id,
                assessment.selection_ref,
                assessment.compatibility_class.as_str(),
                assessment.review_state.as_str(),
                if assessment.dispatch_allowed() {
                    "allowed"
                } else {
                    "blocked"
                }
            ));
            if !assessment.added_target_ids.is_empty() {
                out.push_str(&format!(
                    "  - would add: {}\n",
                    assessment.added_target_ids.join(", ")
                ));
            }
            if !assessment.removed_target_ids.is_empty() {
                out.push_str(&format!(
                    "  - would drop: {}\n",
                    assessment.removed_target_ids.join(", ")
                ));
            }
            if !assessment.fingerprint_mismatch_target_ids.is_empty() {
                out.push_str(&format!(
                    "  - fingerprint changed: {}\n",
                    assessment.fingerprint_mismatch_target_ids.join(", ")
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in selection export.
#[derive(Debug)]
pub enum PortableSelectionArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PortableSelectionViolation>),
}

impl fmt::Display for PortableSelectionArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "portable selection export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "portable selection export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PortableSelectionArtifactError {}

/// Validation failures emitted by [`PortableSelectionPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortableSelectionViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required portability channel is represented by no selection.
    ChannelMissing,
    /// A required selector intent is represented by no selection.
    IntentCoverageMissing,
    /// No assessment demonstrates a preserved compatible selection.
    CompatibleCaseMissing,
    /// No assessment demonstrates a widening that opened review.
    WidenReviewCaseMissing,
    /// No assessment demonstrates an imported selection blocked from local rerun.
    ImportedRerunCaseMissing,
    /// A selection is incomplete.
    SelectionInvalid,
    /// Selection target ids are not unique.
    TargetIdsNotUnique,
    /// A target's fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A parameterized template was collapsed into its concrete invocation.
    TemplateCollapsedWithInvocation,
    /// An imported / provider-backed selection was treated as a local rerun.
    ImportedRerunAsLocal,
    /// An assessment is incomplete.
    AssessmentInvalid,
    /// An assessment references a selection absent from the packet.
    AssessmentSelectionUnresolved,
    /// An assessment's class, review, and deltas are inconsistent.
    AssessmentInconsistent,
    /// An assessment's snapshot-drift flag disagrees with the referenced
    /// selection's snapshot fingerprint.
    SnapshotDriftFlagMismatch,
    /// A widening or drift hid behind a no-review state.
    WideningHidesReview,
    /// A selection or assessment lacks evidence refs.
    EvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Channel projection does not satisfy required invariants.
    ChannelProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PortableSelectionViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ChannelMissing => "channel_missing",
            Self::IntentCoverageMissing => "intent_coverage_missing",
            Self::CompatibleCaseMissing => "compatible_case_missing",
            Self::WidenReviewCaseMissing => "widen_review_case_missing",
            Self::ImportedRerunCaseMissing => "imported_rerun_case_missing",
            Self::SelectionInvalid => "selection_invalid",
            Self::TargetIdsNotUnique => "target_ids_not_unique",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::TemplateCollapsedWithInvocation => "template_collapsed_with_invocation",
            Self::ImportedRerunAsLocal => "imported_rerun_as_local",
            Self::AssessmentInvalid => "assessment_invalid",
            Self::AssessmentSelectionUnresolved => "assessment_selection_unresolved",
            Self::AssessmentInconsistent => "assessment_inconsistent",
            Self::SnapshotDriftFlagMismatch => "snapshot_drift_flag_mismatch",
            Self::WideningHidesReview => "widening_hides_review",
            Self::EvidenceMissing => "evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ChannelProjectionIncomplete => "channel_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable selection export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_portable_selection_export(
) -> Result<PortableSelectionPacket, PortableSelectionArtifactError> {
    let packet: PortableSelectionPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/testing/m5/scope-compatible-selection-objects-and-widened-selection-review/support_export.json"
    )))
    .map_err(PortableSelectionArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PortableSelectionArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &PortableSelectionPacket,
    violations: &mut Vec<PortableSelectionViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PORTABLE_SELECTION_SCHEMA_REF,
        PORTABLE_SELECTION_DOC_REF,
        PORTABLE_SELECTION_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PortableSelectionViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &PortableSelectionPacket,
    violations: &mut Vec<PortableSelectionViolation>,
) {
    let channels = packet.represented_channels();
    for required in SelectorChannel::ALL {
        if !channels.contains(&required) {
            violations.push(PortableSelectionViolation::ChannelMissing);
            break;
        }
    }

    let intents = packet.represented_intents();
    for required in [
        SelectionIntentKind::RerunAll,
        SelectionIntentKind::RerunFailed,
        SelectionIntentKind::ChangedSince,
        SelectionIntentKind::SnapshotScoped,
    ] {
        if !intents.contains(&required) {
            violations.push(PortableSelectionViolation::IntentCoverageMissing);
            break;
        }
    }

    let target_kinds = packet.represented_target_kinds();
    if !(target_kinds.contains(&DurableTestNodeKind::ParameterizedTemplate)
        && target_kinds.contains(&DurableTestNodeKind::ConcreteInvocation))
    {
        violations.push(PortableSelectionViolation::TemplateCollapsedWithInvocation);
    }

    if !packet.assessments.iter().any(|a| {
        a.compatibility_class == TargetCompatibilityClass::Compatible && a.preserves_origin
    }) {
        violations.push(PortableSelectionViolation::CompatibleCaseMissing);
    }

    if !packet.assessments.iter().any(|a| {
        a.compatibility_class == TargetCompatibilityClass::WidenedNeedsReview
            && a.review_state.is_open_or_decided()
    }) {
        violations.push(PortableSelectionViolation::WidenReviewCaseMissing);
    }

    if !packet
        .assessments
        .iter()
        .any(|a| a.compatibility_class == TargetCompatibilityClass::ImportedNotRerunnable)
    {
        violations.push(PortableSelectionViolation::ImportedRerunCaseMissing);
    }
}

fn validate_selections(
    packet: &PortableSelectionPacket,
    violations: &mut Vec<PortableSelectionViolation>,
) {
    for selection in &packet.selections {
        if !selection.is_valid() {
            violations.push(PortableSelectionViolation::SelectionInvalid);
        }
        if !selection.target_ids_unique() {
            violations.push(PortableSelectionViolation::TargetIdsNotUnique);
        }
        if !selection.imported_policy_ok() {
            violations.push(PortableSelectionViolation::ImportedRerunAsLocal);
        }
        if selection.evidence_refs.is_empty()
            || selection.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(PortableSelectionViolation::EvidenceMissing);
        }
        for target in &selection.pinned_targets {
            if !target.fingerprint_independent_of_id() {
                violations.push(PortableSelectionViolation::FingerprintSubstitutesIdentity);
            }
        }
    }
}

fn validate_assessments(
    packet: &PortableSelectionPacket,
    violations: &mut Vec<PortableSelectionViolation>,
) {
    for assessment in &packet.assessments {
        if !assessment.is_valid() {
            violations.push(PortableSelectionViolation::AssessmentInvalid);
        }
        if !assessment.is_consistent() {
            violations.push(PortableSelectionViolation::AssessmentInconsistent);
        }
        if assessment.evidence_refs.is_empty()
            || assessment.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(PortableSelectionViolation::EvidenceMissing);
        }

        // A widening or narrowing that does not open a review is silently
        // expanding beyond the originating selection.
        let drifted_or_changed = assessment.snapshot_drifted
            || !assessment.added_target_ids.is_empty()
            || !assessment.removed_target_ids.is_empty()
            || !assessment.fingerprint_mismatch_target_ids.is_empty();
        if drifted_or_changed && assessment.review_state == WidenedSelectionReviewState::NotRequired
        {
            violations.push(PortableSelectionViolation::WideningHidesReview);
        }

        match packet.selection(&assessment.selection_ref) {
            None => {
                violations.push(PortableSelectionViolation::AssessmentSelectionUnresolved);
            }
            Some(selection) => {
                let recomputed_drift = !selection
                    .snapshot_fingerprint
                    .matches(&assessment.current_snapshot_fingerprint);
                if recomputed_drift != assessment.snapshot_drifted {
                    violations.push(PortableSelectionViolation::SnapshotDriftFlagMismatch);
                }
                if selection.is_imported()
                    && assessment.compatibility_class
                        != TargetCompatibilityClass::ImportedNotRerunnable
                {
                    violations.push(PortableSelectionViolation::ImportedRerunAsLocal);
                }
            }
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
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
