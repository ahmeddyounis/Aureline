//! One diff-and-checkpoint model for workflow-bundle install, update, remove, and
//! drift review across the claimed M5 stacks.
//!
//! This module sits above the workflow-bundle manifests
//! ([`crate::m5_workflow_bundle_manifests`]) and turns a lifecycle action on a
//! bundle into a single reviewable [`BundleReviewRecord`]: a per-component diff
//! preview, an explicit asset-ownership classification, a per-asset resolution
//! choice, lifecycle-sensitive dependency markers, and a one-step rollback
//! checkpoint that exists before any mutation commits.
//!
//! The same packet drives every consumer. Start Center, bundle-detail pages,
//! CLI/headless install, diagnostics, support export, and docs/help all ingest
//! the [`M5BundleReviewAndRollbackPacket`] verbatim rather than re-deriving drift
//! or rollback state per surface, so the desktop UI, the CLI, and a support
//! export always explain the same thing.
//!
//! The contract the record guarantees:
//!
//! - **One diff-and-checkpoint model.** Install, update, remove, and drift review
//!   share one [`BundleReviewOperation`] vocabulary and one
//!   [`ComponentDiffEntry`] shape covering every component category the bundle
//!   composes — extensions, profile/layout/settings presets, task/launch/debug
//!   recipes, docs and tour packs, template and scaffold refs, and migration
//!   mappings.
//! - **Asset ownership is explicit.** Every diff entry is classified
//!   [`AssetOwnership::BundleOwned`], [`AssetOwnership::LocallyOverridden`],
//!   [`AssetOwnership::Adopted`], [`AssetOwnership::Removable`],
//!   [`AssetOwnership::BlockedByPolicy`], or [`AssetOwnership::BlockedByLifecycle`],
//!   so removal never silently destroys user-owned or locally-authored state.
//! - **Resolutions stay distinct.** Keep local, adopt bundle, rebase to bundle,
//!   compare, and remove-bundle-owned remain separate verbs
//!   ([`ResolutionChoice`]); a user-protected asset can never be paired with a
//!   destructive remove, and a blocked asset can never be silently adopted.
//! - **Dependency-marker honesty.** Any component whose capability sits on a
//!   non-stable [`LifecycleStage`] is review-gated and rolled up into the
//!   record's [`BundleReviewRecord::dependency_markers`]; a record depending on a
//!   non-stable capability MUST disclose it.
//! - **One-step rollback.** Every mutating operation carries a
//!   [`RollbackCheckpoint`] minted before the mutation commits, reversible in one
//!   step, so an install/update/remove can always be undone.
//! - **Diffable, never opaque.** Every record and entry stays diffable,
//!   mirrorable, and export-safe; opaque binary bundle state is forbidden.
//!
//! The packet is checked in at
//! `artifacts/workspace/m5/m5-bundle-review-and-rollback.json` and embedded here.
//! It is metadata-only: every field is a typed state, a count, or an opaque ref,
//! and it carries no credential bodies, raw provider payloads, raw local paths, or
//! bundle binary contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::m5_admission_and_routing::M5Wedge;
pub use crate::m5_workflow_bundle_manifests::{
    BundleComponentKind, CertificationTarget, LifecycleStage,
};

/// Supported packet schema version.
pub const M5_BUNDLE_REVIEW_AND_ROLLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_BUNDLE_REVIEW_AND_ROLLBACK_RECORD_KIND: &str = "m5_bundle_review_and_rollback_packet";

/// Repo-relative path to the checked-in packet.
pub const M5_BUNDLE_REVIEW_AND_ROLLBACK_PATH: &str =
    "artifacts/workspace/m5/m5-bundle-review-and-rollback.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_BUNDLE_REVIEW_AND_ROLLBACK_SCHEMA_REF: &str =
    "schemas/workspace/m5-bundle-review-and-rollback.schema.json";

/// Repo-relative path to the companion document.
pub const M5_BUNDLE_REVIEW_AND_ROLLBACK_DOC_REF: &str =
    "docs/workspace/m5/m5-bundle-review-and-rollback.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_BUNDLE_REVIEW_AND_ROLLBACK_FIXTURE_DIR: &str =
    "fixtures/workspace/m5/m5-bundle-review-and-rollback";

/// Embedded checked-in packet JSON.
pub const M5_BUNDLE_REVIEW_AND_ROLLBACK_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-bundle-review-and-rollback.json"
));

/// The lifecycle action a [`BundleReviewRecord`] reviews.
///
/// All four share one diff-and-checkpoint model: install, update, and remove
/// mutate local state and carry a one-step rollback checkpoint; drift review is a
/// read-only comparison of current local state against the bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleReviewOperation {
    /// Install a bundle into a workspace.
    Install,
    /// Update an installed bundle to a newer revision.
    Update,
    /// Remove an installed bundle.
    Remove,
    /// Compare current local state against the bundle without mutating.
    DriftReview,
}

impl BundleReviewOperation {
    /// Every operation, in declaration order.
    pub const ALL: [Self; 4] = [Self::Install, Self::Update, Self::Remove, Self::DriftReview];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Install => "install",
            Self::Update => "update",
            Self::Remove => "remove",
            Self::DriftReview => "drift_review",
        }
    }

    /// Whether the operation mutates local state and therefore requires a
    /// one-step rollback checkpoint minted before the mutation commits.
    pub const fn is_mutating(self) -> bool {
        matches!(self, Self::Install | Self::Update | Self::Remove)
    }
}

/// How one component differs between the bundle and the current local state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffAction {
    /// The bundle introduces a component not present locally.
    Added,
    /// The component is present locally but not in the bundle's target state.
    Removed,
    /// The component differs between the bundle and local state.
    Modified,
    /// The component is identical in the bundle and local state.
    Unchanged,
}

impl DiffAction {
    /// Every diff action, in declaration order.
    pub const ALL: [Self; 4] = [Self::Added, Self::Removed, Self::Modified, Self::Unchanged];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Modified => "modified",
            Self::Unchanged => "unchanged",
        }
    }

    /// Whether the action requires a review decision (anything but unchanged).
    pub const fn requires_decision(self) -> bool {
        !matches!(self, Self::Unchanged)
    }
}

/// Who owns a component's current local state, governing whether it is safe to
/// adopt, rebase, or remove.
///
/// The classes stay distinct so removal never silently destroys user-owned or
/// locally-authored state, and a policy- or lifecycle-blocked asset is never
/// silently pulled in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetOwnership {
    /// Created and owned by the bundle; safe to remove with the bundle.
    BundleOwned,
    /// A bundle asset the user has locally overridden; must not be erased.
    LocallyOverridden,
    /// A previously bundle-provided asset the user has adopted as their own.
    Adopted,
    /// A bundle-owned asset that may be removed on uninstall.
    Removable,
    /// An asset whose adoption is blocked by org policy.
    BlockedByPolicy,
    /// An asset whose adoption is blocked by a lifecycle-sensitive dependency.
    BlockedByLifecycle,
}

impl AssetOwnership {
    /// Every ownership class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BundleOwned,
        Self::LocallyOverridden,
        Self::Adopted,
        Self::Removable,
        Self::BlockedByPolicy,
        Self::BlockedByLifecycle,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundleOwned => "bundle_owned",
            Self::LocallyOverridden => "locally_overridden",
            Self::Adopted => "adopted",
            Self::Removable => "removable",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::BlockedByLifecycle => "blocked_by_lifecycle",
        }
    }

    /// Whether the asset carries durable user state that removal must preserve.
    ///
    /// A locally-overridden or adopted asset is user-protected: bundle removal or
    /// rebase must never erase it under the banner of cleanup.
    pub const fn is_user_protected(self) -> bool {
        matches!(self, Self::LocallyOverridden | Self::Adopted)
    }

    /// Whether adoption of the asset is blocked by policy or a lifecycle
    /// dependency.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::BlockedByPolicy | Self::BlockedByLifecycle)
    }
}

/// The decision recorded for a component diff during review.
///
/// The verbs stay distinct: keep the local version, adopt the bundle version,
/// rebase the local override onto the bundle, compare the two, remove a
/// bundle-owned asset, or leave it untouched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionChoice {
    /// Keep the local version; do not adopt the bundle's.
    KeepLocal,
    /// Adopt the bundle's version, replacing local state.
    AdoptBundle,
    /// Rebase the local override onto the bundle's new base.
    Rebase,
    /// Compare the two versions without committing a choice.
    Compare,
    /// Remove a bundle-owned asset.
    RemoveBundleOwned,
    /// No decision is required (an unchanged or blocked component).
    NotApplicable,
}

impl ResolutionChoice {
    /// Every resolution choice, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeepLocal,
        Self::AdoptBundle,
        Self::Rebase,
        Self::Compare,
        Self::RemoveBundleOwned,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeepLocal => "keep_local",
            Self::AdoptBundle => "adopt_bundle",
            Self::Rebase => "rebase",
            Self::Compare => "compare",
            Self::RemoveBundleOwned => "remove_bundle_owned",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether the choice pulls bundle state into local state.
    pub const fn pulls_bundle_state(self) -> bool {
        matches!(self, Self::AdoptBundle | Self::Rebase)
    }
}

/// The high-level relationship between local state and the bundle's target state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftState {
    /// Local state matches the bundle.
    InSync,
    /// Local state carries changes the bundle does not.
    LocalAhead,
    /// The bundle carries changes local state does not.
    BundleAhead,
    /// Both local and bundle state have diverged.
    Diverged,
    /// Drift could not be determined.
    Unknown,
}

impl DriftState {
    /// Every drift state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InSync,
        Self::LocalAhead,
        Self::BundleAhead,
        Self::Diverged,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSync => "in_sync",
            Self::LocalAhead => "local_ahead",
            Self::BundleAhead => "bundle_ahead",
            Self::Diverged => "diverged",
            Self::Unknown => "unknown",
        }
    }
}

/// One per-component diff preview row.
///
/// A diff entry is a diffable reference, never an opaque blob: it names its kind,
/// the lifecycle stage of the capability it depends on, how it differs, who owns
/// the local state, the resolution recorded for it, and an opaque ref into the
/// rendered diff preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentDiffEntry {
    /// Which content category this component contributes.
    pub component_kind: BundleComponentKind,
    /// Stable component identifier within the bundle.
    pub component_id: String,
    /// The lifecycle stage of the capability this component depends on.
    pub lifecycle_stage: LifecycleStage,
    /// Whether the component is review-gated. Must be `true` for any non-stable stage.
    pub requires_review: bool,
    /// How the component differs between the bundle and local state.
    pub diff_action: DiffAction,
    /// Who owns the component's current local state.
    pub ownership: AssetOwnership,
    /// The resolution recorded for this component.
    pub resolution: ResolutionChoice,
    /// Guardrail: the diff is reviewable, never an opaque blob. Always `true`.
    pub diffable: bool,
    /// A human-readable, one-line component label.
    pub label: String,
    /// Opaque ref into the rendered diff preview.
    pub diff_preview_ref: String,
    /// Opaque ref to a preserved local override, when the local state diverges.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_override_ref: Option<String>,
}

impl ComponentDiffEntry {
    /// Whether a non-stable lifecycle stage is review-gated as required.
    pub fn lifecycle_gated(&self) -> bool {
        !self.lifecycle_stage.is_non_stable() || self.requires_review
    }

    /// Whether the resolution is safe for this entry's ownership and diff action.
    ///
    /// The safety rules that keep removal and adoption honest:
    ///
    /// - An unchanged component needs no decision ([`ResolutionChoice::NotApplicable`]).
    /// - A user-protected asset (locally overridden or adopted) may never be
    ///   paired with [`ResolutionChoice::RemoveBundleOwned`] — that would erase
    ///   user state under the banner of cleanup.
    /// - A blocked asset may never be adopted or rebased; its only honest
    ///   resolutions are to compare, keep local, or leave it untouched.
    /// - [`ResolutionChoice::RemoveBundleOwned`] only applies to bundle-owned or
    ///   removable assets.
    /// - Any component that requires a decision must record one (unless blocked,
    ///   which legitimately offers no actionable choice).
    pub fn resolution_safe(&self) -> bool {
        match self.diff_action {
            DiffAction::Unchanged => self.resolution == ResolutionChoice::NotApplicable,
            _ => {
                if self.ownership.is_user_protected()
                    && self.resolution == ResolutionChoice::RemoveBundleOwned
                {
                    return false;
                }
                if self.ownership.is_blocked() && self.resolution.pulls_bundle_state() {
                    return false;
                }
                if self.resolution == ResolutionChoice::RemoveBundleOwned
                    && !matches!(
                        self.ownership,
                        AssetOwnership::BundleOwned | AssetOwnership::Removable
                    )
                {
                    return false;
                }
                if self.ownership.is_blocked() {
                    // A blocked asset offers no actionable resolution; compare,
                    // keep-local, or not-applicable are the only honest choices.
                    return matches!(
                        self.resolution,
                        ResolutionChoice::Compare
                            | ResolutionChoice::KeepLocal
                            | ResolutionChoice::NotApplicable
                    );
                }
                // A non-blocked component that differs must record a real decision.
                self.resolution != ResolutionChoice::NotApplicable
            }
        }
    }

    /// Whether this entry is internally consistent and never an opaque blob.
    pub fn is_consistent(&self) -> bool {
        self.diffable
            && self.lifecycle_gated()
            && self.resolution_safe()
            && !self.component_id.trim().is_empty()
            && !self.label.trim().is_empty()
            && !self.diff_preview_ref.trim().is_empty()
    }
}

/// A one-step rollback checkpoint minted before a mutating operation commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackCheckpoint {
    /// Opaque ref to the captured checkpoint state.
    pub checkpoint_ref: String,
    /// Whether the operation can be undone in a single step.
    pub one_step: bool,
    /// Whether the operation is reversible.
    pub reversible: bool,
    /// Whether the checkpoint was captured before any mutation committed.
    pub captured_before_mutation: bool,
    /// Count of components captured in the checkpoint.
    pub captured_component_count: usize,
}

impl RollbackCheckpoint {
    /// Whether the checkpoint supports a one-step, pre-mutation rollback.
    pub fn supports_one_step_rollback(&self) -> bool {
        !self.checkpoint_ref.trim().is_empty()
            && self.one_step
            && self.reversible
            && self.captured_before_mutation
    }
}

/// One row of the drift comparison between local state and the bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DriftEntry {
    /// Stable drift-entry identifier.
    pub entry_id: String,
    /// The component this drift row concerns.
    pub component_id: String,
    /// Who owns the drifted local state.
    pub ownership: AssetOwnership,
    /// The resolution offered for this drift.
    pub resolution: ResolutionChoice,
    /// Opaque ref to a preserved local override, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_override_ref: Option<String>,
}

/// The drift comparison rolled up across a bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DriftRecord {
    /// High-level drift state.
    pub drift_state: DriftState,
    /// Per-component drift rows.
    #[serde(default)]
    pub entries: Vec<DriftEntry>,
    /// Whether user-protected local state is preserved across the comparison.
    pub preserves_local_overrides: bool,
}

impl DriftRecord {
    /// Whether every user-protected drift row preserves local state rather than
    /// being silently removed.
    pub fn user_state_preserved(&self) -> bool {
        self.preserves_local_overrides
            && self.entries.iter().all(|e| {
                !e.ownership.is_user_protected()
                    || e.resolution != ResolutionChoice::RemoveBundleOwned
            })
    }
}

/// A policy- or lifecycle-sensitive warning surfaced on the review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyLifecycleWarning {
    /// Stable warning identifier.
    pub warning_id: String,
    /// The lifecycle stage the warning concerns.
    pub lifecycle_stage: LifecycleStage,
    /// Whether the warning blocks applying the reviewed change.
    pub blocks_apply: bool,
    /// A human-readable, one-line warning summary.
    pub summary_label: String,
}

/// One bundle-review record: a complete diff-and-checkpoint review of a lifecycle
/// action on one bundle for one M5 stack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleReviewRecord {
    /// Stable review identifier.
    pub review_id: String,
    /// The M5 launch wedge this review concerns.
    pub wedge: M5Wedge,
    /// The bundle being reviewed.
    pub bundle_id: String,
    /// The currently installed bundle version, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_version: Option<String>,
    /// The target bundle version this review resolves to.
    pub target_version: String,
    /// The lifecycle action being reviewed.
    pub operation: BundleReviewOperation,
    /// The certification posture the target bundle claims.
    pub certification_target: CertificationTarget,
    /// The per-component diff preview.
    pub component_diffs: Vec<ComponentDiffEntry>,
    /// The drift comparison between local state and the bundle.
    pub drift_record: DriftRecord,
    /// The one-step rollback checkpoint for the operation.
    pub rollback_checkpoint: RollbackCheckpoint,
    /// Distinct non-stable lifecycle stages depended on, sorted. Must equal the
    /// set computed from [`Self::component_diffs`].
    pub dependency_markers: Vec<LifecycleStage>,
    /// Whether the review discloses a non-stable dependency. Must equal whether
    /// [`Self::dependency_markers`] is non-empty.
    pub discloses_non_stable_dependencies: bool,
    /// Policy- and lifecycle-sensitive warnings.
    #[serde(default)]
    pub policy_lifecycle_warnings: Vec<PolicyLifecycleWarning>,
    /// Guardrail: the review is diffable. Always `true`.
    pub diffable: bool,
    /// Guardrail: the review is mirrorable. Always `true`.
    pub mirrorable: bool,
    /// Guardrail: the review is export-safe. Always `true`.
    pub export_safe: bool,
    /// Guardrail: opaque binary bundle state is forbidden. Always `false`.
    pub opaque_binary_state: bool,
    /// Opaque review-provenance ref.
    pub review_provenance_ref: String,
    /// Accountable owner.
    pub owner: String,
    /// Caveats shown on the review.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// A reviewer note summarizing the review.
    pub note: String,
}

impl BundleReviewRecord {
    /// Diff entries of the given kind.
    pub fn diffs_of_kind(
        &self,
        kind: BundleComponentKind,
    ) -> impl Iterator<Item = &ComponentDiffEntry> {
        self.component_diffs
            .iter()
            .filter(move |c| c.component_kind == kind)
    }

    /// Whether any component depends on a non-stable capability.
    pub fn has_non_stable_components(&self) -> bool {
        self.component_diffs
            .iter()
            .any(|c| c.lifecycle_stage.is_non_stable())
    }

    /// Whether any component is review-gated.
    pub fn has_review_required_components(&self) -> bool {
        self.component_diffs.iter().any(|c| c.requires_review)
    }

    /// The distinct non-stable lifecycle stages this review depends on, sorted.
    pub fn computed_dependency_markers(&self) -> Vec<LifecycleStage> {
        let mut set = BTreeSet::new();
        for entry in &self.component_diffs {
            if entry.lifecycle_stage.is_non_stable() {
                set.insert(entry.lifecycle_stage);
            }
        }
        set.into_iter().collect()
    }

    /// Whether the guardrails that keep a review diffable and non-opaque hold.
    pub fn guards_correct(&self) -> bool {
        self.diffable && self.mirrorable && self.export_safe && !self.opaque_binary_state
    }

    /// Whether the non-stable disclosure flag matches the computed markers.
    pub fn disclosure_consistent(&self) -> bool {
        self.discloses_non_stable_dependencies == self.has_non_stable_components()
            && self.dependency_markers == self.computed_dependency_markers()
    }

    /// Whether a mutating operation carries a one-step rollback checkpoint, and a
    /// read-only drift review carries at least a non-empty checkpoint ref.
    pub fn rollback_correct(&self) -> bool {
        if self.operation.is_mutating() {
            self.rollback_checkpoint.supports_one_step_rollback()
        } else {
            !self.rollback_checkpoint.checkpoint_ref.trim().is_empty()
        }
    }

    /// Whether removal preserves user-protected assets.
    ///
    /// A remove operation must never resolve a locally-overridden or adopted
    /// component to [`ResolutionChoice::RemoveBundleOwned`], and its drift record
    /// must preserve local overrides.
    pub fn removal_preserves_user_assets(&self) -> bool {
        if self.operation != BundleReviewOperation::Remove {
            return true;
        }
        self.component_diffs.iter().all(|c| {
            !c.ownership.is_user_protected() || c.resolution != ResolutionChoice::RemoveBundleOwned
        }) && self.drift_record.user_state_preserved()
    }

    /// Whether every policy- or lifecycle-blocked component carries a matching
    /// warning, so a blocked asset is never silently dropped from the diff.
    pub fn blocked_components_warned(&self) -> bool {
        let has_blocked = self
            .component_diffs
            .iter()
            .any(|c| c.ownership.is_blocked());
        if !has_blocked {
            return true;
        }
        !self.policy_lifecycle_warnings.is_empty()
    }

    /// The certified presentation computed from the certification target.
    pub fn presents_as_certified(&self) -> bool {
        self.certification_target.presents_as_certified()
    }

    /// Whether a caveat is required on this review.
    ///
    /// Anything not certified, anything depending on a non-stable capability,
    /// anything carrying a review-gated component, a remove operation, or any
    /// blocking warning must carry a caveat so the weaker posture is never silent.
    pub fn caveats_required(&self) -> bool {
        self.certification_target != CertificationTarget::Certified
            || self.has_non_stable_components()
            || self.has_review_required_components()
            || self.operation == BundleReviewOperation::Remove
            || self
                .policy_lifecycle_warnings
                .iter()
                .any(|w| w.blocks_apply)
    }

    /// Whether the review is internally consistent against the gate.
    pub fn is_consistent(&self) -> bool {
        self.guards_correct()
            && self.disclosure_consistent()
            && self.rollback_correct()
            && self.removal_preserves_user_assets()
            && self.blocked_components_warned()
            && !self.bundle_id.trim().is_empty()
            && !self.target_version.trim().is_empty()
            && !self.review_provenance_ref.trim().is_empty()
            && self
                .component_diffs
                .iter()
                .all(ComponentDiffEntry::is_consistent)
            && (!self.caveats_required() || self.caveats.iter().any(|c| !c.trim().is_empty()))
    }
}

/// Summary counts rolled up across every bundle review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5BundleReviewAndRollbackSummary {
    /// Total reviews.
    pub total_reviews: usize,
    /// Install reviews.
    pub install_reviews: usize,
    /// Update reviews.
    pub update_reviews: usize,
    /// Remove reviews.
    pub remove_reviews: usize,
    /// Drift-review reviews.
    pub drift_review_reviews: usize,
    /// Reviews that depend on a non-stable capability.
    pub reviews_with_non_stable_dependencies: usize,
    /// Reviews that disclose a non-stable dependency.
    pub reviews_disclosing_non_stable: usize,
    /// Reviews carrying a one-step rollback checkpoint.
    pub reviews_with_one_step_rollback: usize,
    /// Reviews carrying a policy- or lifecycle-blocking warning.
    pub reviews_with_blocking_warnings: usize,
    /// Total component diffs across all reviews.
    pub total_component_diffs: usize,
    /// Added component diffs.
    pub added_diffs: usize,
    /// Removed component diffs.
    pub removed_diffs: usize,
    /// Modified component diffs.
    pub modified_diffs: usize,
    /// Unchanged component diffs.
    pub unchanged_diffs: usize,
    /// Bundle-owned diffs.
    pub bundle_owned_diffs: usize,
    /// Locally-overridden diffs.
    pub locally_overridden_diffs: usize,
    /// Adopted diffs.
    pub adopted_diffs: usize,
    /// Removable diffs.
    pub removable_diffs: usize,
    /// Policy-blocked diffs.
    pub blocked_by_policy_diffs: usize,
    /// Lifecycle-blocked diffs.
    pub blocked_by_lifecycle_diffs: usize,
    /// Non-stable component diffs.
    pub non_stable_diffs: usize,
    /// Review-gated component diffs.
    pub review_required_diffs: usize,
}

/// One redaction-safe export row projected from a review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleReviewAndRollbackExportRow {
    /// Review id.
    pub review_id: String,
    /// Wedge token.
    pub wedge: M5Wedge,
    /// Bundle id.
    pub bundle_id: String,
    /// Target version.
    pub target_version: String,
    /// Operation token.
    pub operation: BundleReviewOperation,
    /// Certification target token.
    pub certification_target: CertificationTarget,
    /// Drift state token.
    pub drift_state: DriftState,
    /// Component kinds diffed.
    pub component_kinds: Vec<BundleComponentKind>,
    /// Distinct non-stable dependency markers.
    pub dependency_markers: Vec<LifecycleStage>,
    /// Whether the review discloses a non-stable dependency.
    pub discloses_non_stable_dependencies: bool,
    /// Whether the review carries a one-step rollback checkpoint.
    pub one_step_rollback: bool,
    /// Whether the review presents as certified.
    pub presents_as_certified: bool,
    /// Whether removal preserves user-owned assets.
    pub removal_preserves_user_assets: bool,
    /// Review provenance ref.
    pub review_provenance_ref: String,
}

/// A redaction-safe export projection of the whole packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleReviewAndRollbackExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected review rows.
    pub reviews: Vec<M5BundleReviewAndRollbackExportRow>,
    /// Whether every review is gate-consistent.
    pub all_reviews_consistent: bool,
    /// Reviews that depend on a non-stable capability.
    pub reviews_with_non_stable_dependencies: usize,
    /// Reviews carrying a one-step rollback checkpoint.
    pub reviews_with_one_step_rollback: usize,
}

/// The typed M5 bundle-review-and-rollback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5BundleReviewAndRollbackPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed wedge vocabulary.
    pub wedges: Vec<M5Wedge>,
    /// Closed operation vocabulary.
    pub operations: Vec<BundleReviewOperation>,
    /// Closed diff-action vocabulary.
    pub diff_actions: Vec<DiffAction>,
    /// Closed asset-ownership vocabulary.
    pub ownership_classes: Vec<AssetOwnership>,
    /// Closed resolution-choice vocabulary.
    pub resolution_choices: Vec<ResolutionChoice>,
    /// Closed drift-state vocabulary.
    pub drift_states: Vec<DriftState>,
    /// Closed component-kind vocabulary.
    pub component_kinds: Vec<BundleComponentKind>,
    /// Closed lifecycle-stage vocabulary.
    pub lifecycle_stages: Vec<LifecycleStage>,
    /// Closed certification-target vocabulary.
    pub certification_targets: Vec<CertificationTarget>,
    /// Closed set of consumer surfaces that ingest this packet.
    pub consumer_surfaces: Vec<String>,
    /// Bundle reviews, one or more per M5 launch wedge.
    #[serde(default)]
    pub reviews: Vec<BundleReviewRecord>,
    /// Summary counts.
    pub summary: M5BundleReviewAndRollbackSummary,
}

impl M5BundleReviewAndRollbackPacket {
    /// Returns the review with the given id.
    pub fn review(&self, review_id: &str) -> Option<&BundleReviewRecord> {
        self.reviews.iter().find(|r| r.review_id == review_id)
    }

    /// Reviews concerning the given wedge.
    pub fn reviews_for_wedge(&self, wedge: M5Wedge) -> impl Iterator<Item = &BundleReviewRecord> {
        self.reviews.iter().filter(move |r| r.wedge == wedge)
    }

    /// Reviews for the given operation.
    pub fn reviews_with_operation(
        &self,
        operation: BundleReviewOperation,
    ) -> impl Iterator<Item = &BundleReviewRecord> {
        self.reviews
            .iter()
            .filter(move |r| r.operation == operation)
    }

    /// Whether every review operation is exercised by at least one review.
    pub fn covers_every_operation(&self) -> bool {
        BundleReviewOperation::ALL
            .iter()
            .all(|op| self.reviews_with_operation(*op).next().is_some())
    }

    /// Whether every M5 wedge is covered by at least one review.
    pub fn covers_every_wedge(&self) -> bool {
        M5Wedge::ALL
            .iter()
            .all(|wedge| self.reviews_for_wedge(*wedge).next().is_some())
    }

    /// Whether every review is internally consistent against the gate.
    pub fn all_reviews_consistent(&self) -> bool {
        self.reviews.iter().all(BundleReviewRecord::is_consistent)
    }

    /// Recomputes the summary from the reviews.
    pub fn computed_summary(&self) -> M5BundleReviewAndRollbackSummary {
        let count_op = |op: BundleReviewOperation| self.reviews_with_operation(op).count();
        let diffs = || self.reviews.iter().flat_map(|r| r.component_diffs.iter());
        let count_action = |action: DiffAction| diffs().filter(|d| d.diff_action == action).count();
        let count_ownership =
            |owner: AssetOwnership| diffs().filter(|d| d.ownership == owner).count();
        M5BundleReviewAndRollbackSummary {
            total_reviews: self.reviews.len(),
            install_reviews: count_op(BundleReviewOperation::Install),
            update_reviews: count_op(BundleReviewOperation::Update),
            remove_reviews: count_op(BundleReviewOperation::Remove),
            drift_review_reviews: count_op(BundleReviewOperation::DriftReview),
            reviews_with_non_stable_dependencies: self
                .reviews
                .iter()
                .filter(|r| r.has_non_stable_components())
                .count(),
            reviews_disclosing_non_stable: self
                .reviews
                .iter()
                .filter(|r| r.discloses_non_stable_dependencies)
                .count(),
            reviews_with_one_step_rollback: self
                .reviews
                .iter()
                .filter(|r| r.rollback_checkpoint.supports_one_step_rollback())
                .count(),
            reviews_with_blocking_warnings: self
                .reviews
                .iter()
                .filter(|r| r.policy_lifecycle_warnings.iter().any(|w| w.blocks_apply))
                .count(),
            total_component_diffs: diffs().count(),
            added_diffs: count_action(DiffAction::Added),
            removed_diffs: count_action(DiffAction::Removed),
            modified_diffs: count_action(DiffAction::Modified),
            unchanged_diffs: count_action(DiffAction::Unchanged),
            bundle_owned_diffs: count_ownership(AssetOwnership::BundleOwned),
            locally_overridden_diffs: count_ownership(AssetOwnership::LocallyOverridden),
            adopted_diffs: count_ownership(AssetOwnership::Adopted),
            removable_diffs: count_ownership(AssetOwnership::Removable),
            blocked_by_policy_diffs: count_ownership(AssetOwnership::BlockedByPolicy),
            blocked_by_lifecycle_diffs: count_ownership(AssetOwnership::BlockedByLifecycle),
            non_stable_diffs: diffs()
                .filter(|d| d.lifecycle_stage.is_non_stable())
                .count(),
            review_required_diffs: diffs().filter(|d| d.requires_review).count(),
        }
    }

    /// Projects a redaction-safe export view of the packet.
    pub fn export_projection(&self) -> M5BundleReviewAndRollbackExportProjection {
        let reviews = self
            .reviews
            .iter()
            .map(|r| M5BundleReviewAndRollbackExportRow {
                review_id: r.review_id.clone(),
                wedge: r.wedge,
                bundle_id: r.bundle_id.clone(),
                target_version: r.target_version.clone(),
                operation: r.operation,
                certification_target: r.certification_target,
                drift_state: r.drift_record.drift_state,
                component_kinds: r.component_diffs.iter().map(|c| c.component_kind).collect(),
                dependency_markers: r.dependency_markers.clone(),
                discloses_non_stable_dependencies: r.discloses_non_stable_dependencies,
                one_step_rollback: r.rollback_checkpoint.supports_one_step_rollback(),
                presents_as_certified: r.presents_as_certified(),
                removal_preserves_user_assets: r.removal_preserves_user_assets(),
                review_provenance_ref: r.review_provenance_ref.clone(),
            })
            .collect();
        let summary = self.computed_summary();
        M5BundleReviewAndRollbackExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            reviews,
            all_reviews_consistent: self.all_reviews_consistent(),
            reviews_with_non_stable_dependencies: summary.reviews_with_non_stable_dependencies,
            reviews_with_one_step_rollback: summary.reviews_with_one_step_rollback,
        }
    }

    /// Validates the packet against its honesty contract, returning every violation.
    pub fn validate(&self) -> Vec<M5BundleReviewAndRollbackViolation> {
        use M5BundleReviewAndRollbackViolation as V;
        let mut violations = Vec::new();

        if self.schema_version != M5_BUNDLE_REVIEW_AND_ROLLBACK_SCHEMA_VERSION {
            violations.push(V::SchemaVersionMismatch {
                found: self.schema_version,
            });
        }
        if self.record_kind != M5_BUNDLE_REVIEW_AND_ROLLBACK_RECORD_KIND {
            violations.push(V::RecordKindMismatch {
                found: self.record_kind.clone(),
            });
        }
        if !self.vocabularies_complete() {
            violations.push(V::VocabularyMismatch);
        }
        if self.reviews.is_empty() {
            violations.push(V::NoReviews);
        }

        let mut seen_ids = BTreeSet::new();
        for review in &self.reviews {
            if !seen_ids.insert(review.review_id.clone()) {
                violations.push(V::DuplicateReviewId {
                    review_id: review.review_id.clone(),
                });
            }
            self.validate_review(review, &mut violations);
        }

        for wedge in M5Wedge::ALL {
            if self.reviews_for_wedge(wedge).next().is_none() {
                violations.push(V::MissingWedgeCoverage { wedge });
            }
        }
        for op in BundleReviewOperation::ALL {
            if self.reviews_with_operation(op).next().is_none() {
                violations.push(V::MissingOperationCoverage { operation: op });
            }
        }
        if self.summary != self.computed_summary() {
            violations.push(V::SummaryMismatch);
        }

        violations
    }

    fn vocabularies_complete(&self) -> bool {
        self.wedges == M5Wedge::ALL.to_vec()
            && self.operations == BundleReviewOperation::ALL.to_vec()
            && self.diff_actions == DiffAction::ALL.to_vec()
            && self.ownership_classes == AssetOwnership::ALL.to_vec()
            && self.resolution_choices == ResolutionChoice::ALL.to_vec()
            && self.drift_states == DriftState::ALL.to_vec()
            && self.component_kinds == BundleComponentKind::ALL.to_vec()
            && self.lifecycle_stages == LifecycleStage::ALL.to_vec()
            && self.certification_targets == CertificationTarget::ALL.to_vec()
            && !self.consumer_surfaces.is_empty()
    }

    fn validate_review(
        &self,
        review: &BundleReviewRecord,
        violations: &mut Vec<M5BundleReviewAndRollbackViolation>,
    ) {
        use M5BundleReviewAndRollbackViolation as V;
        let id = review.review_id.clone();

        if review.review_id.trim().is_empty() {
            violations.push(V::EmptyReviewId);
        }
        if review.bundle_id.trim().is_empty() || review.target_version.trim().is_empty() {
            violations.push(V::EmptyBundleIdentity {
                review_id: id.clone(),
            });
        }
        if !review.guards_correct() {
            violations.push(V::OpaqueOrNonDiffable {
                review_id: id.clone(),
            });
        }
        if !review.disclosure_consistent() {
            violations.push(V::UndisclosedNonStableDependency {
                review_id: id.clone(),
            });
        }
        if !review.rollback_correct() {
            violations.push(V::MissingRollbackCheckpoint {
                review_id: id.clone(),
            });
        }
        if !review.removal_preserves_user_assets() {
            violations.push(V::RemovalErasesUserAssets {
                review_id: id.clone(),
            });
        }
        if !review.blocked_components_warned() {
            violations.push(V::BlockedComponentNotWarned {
                review_id: id.clone(),
            });
        }
        if review.review_provenance_ref.trim().is_empty() {
            violations.push(V::MissingProvenance {
                review_id: id.clone(),
            });
        }
        for entry in &review.component_diffs {
            if !entry.is_consistent() {
                violations.push(V::InconsistentDiffEntry {
                    review_id: id.clone(),
                    component_id: entry.component_id.clone(),
                });
            }
        }
        if review.caveats_required() && !review.caveats.iter().any(|c| !c.trim().is_empty()) {
            violations.push(V::MissingCaveat {
                review_id: id.clone(),
            });
        }
        if review.presents_as_certified()
            && review.certification_target != CertificationTarget::Certified
        {
            violations.push(V::CertificationOverreach { review_id: id });
        }
    }
}

/// Every way a packet can violate the bundle-review-and-rollback contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BundleReviewAndRollbackViolation {
    /// The packet schema version is not the supported version.
    SchemaVersionMismatch {
        /// The version found.
        found: u32,
    },
    /// The record-kind discriminator is wrong.
    RecordKindMismatch {
        /// The record-kind found.
        found: String,
    },
    /// A closed vocabulary array does not list its full set.
    VocabularyMismatch,
    /// The packet carries no reviews.
    NoReviews,
    /// Two reviews share an id.
    DuplicateReviewId {
        /// The duplicated review id.
        review_id: String,
    },
    /// A review id is empty.
    EmptyReviewId,
    /// A review is missing its bundle id or target version.
    EmptyBundleIdentity {
        /// The offending review id.
        review_id: String,
    },
    /// A review is non-diffable or carries opaque binary state.
    OpaqueOrNonDiffable {
        /// The offending review id.
        review_id: String,
    },
    /// A review depends on a non-stable capability without disclosing it.
    UndisclosedNonStableDependency {
        /// The offending review id.
        review_id: String,
    },
    /// A mutating review lacks a one-step rollback checkpoint.
    MissingRollbackCheckpoint {
        /// The offending review id.
        review_id: String,
    },
    /// A remove operation would erase user-owned or locally-authored assets.
    RemovalErasesUserAssets {
        /// The offending review id.
        review_id: String,
    },
    /// A blocked component carries no policy/lifecycle warning.
    BlockedComponentNotWarned {
        /// The offending review id.
        review_id: String,
    },
    /// A review is missing its provenance ref.
    MissingProvenance {
        /// The offending review id.
        review_id: String,
    },
    /// A component diff entry is internally inconsistent.
    InconsistentDiffEntry {
        /// The offending review id.
        review_id: String,
        /// The offending component id.
        component_id: String,
    },
    /// A review requires a caveat but carries none.
    MissingCaveat {
        /// The offending review id.
        review_id: String,
    },
    /// A review presents as certified without a certified target.
    CertificationOverreach {
        /// The offending review id.
        review_id: String,
    },
    /// An M5 wedge has no review.
    MissingWedgeCoverage {
        /// The uncovered wedge.
        wedge: M5Wedge,
    },
    /// A review operation is never exercised.
    MissingOperationCoverage {
        /// The uncovered operation.
        operation: BundleReviewOperation,
    },
    /// The packet summary does not equal the recomputed summary.
    SummaryMismatch,
}

impl fmt::Display for M5BundleReviewAndRollbackViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersionMismatch { found } => {
                write!(f, "unexpected schema_version {found}")
            }
            Self::RecordKindMismatch { found } => write!(f, "unexpected record_kind {found}"),
            Self::VocabularyMismatch => write!(f, "a closed vocabulary array is incomplete"),
            Self::NoReviews => write!(f, "packet carries no reviews"),
            Self::DuplicateReviewId { review_id } => {
                write!(f, "duplicate review id {review_id}")
            }
            Self::EmptyReviewId => write!(f, "review id is empty"),
            Self::EmptyBundleIdentity { review_id } => {
                write!(
                    f,
                    "review {review_id} is missing bundle id or target version"
                )
            }
            Self::OpaqueOrNonDiffable { review_id } => {
                write!(
                    f,
                    "review {review_id} is non-diffable or carries opaque state"
                )
            }
            Self::UndisclosedNonStableDependency { review_id } => {
                write!(f, "review {review_id} hides a non-stable dependency")
            }
            Self::MissingRollbackCheckpoint { review_id } => {
                write!(f, "review {review_id} lacks a one-step rollback checkpoint")
            }
            Self::RemovalErasesUserAssets { review_id } => {
                write!(f, "review {review_id} would erase user-owned assets")
            }
            Self::BlockedComponentNotWarned { review_id } => {
                write!(
                    f,
                    "review {review_id} has a blocked component with no warning"
                )
            }
            Self::MissingProvenance { review_id } => {
                write!(f, "review {review_id} is missing provenance")
            }
            Self::InconsistentDiffEntry {
                review_id,
                component_id,
            } => write!(
                f,
                "review {review_id} has inconsistent diff entry {component_id}"
            ),
            Self::MissingCaveat { review_id } => {
                write!(f, "review {review_id} requires a caveat but carries none")
            }
            Self::CertificationOverreach { review_id } => {
                write!(
                    f,
                    "review {review_id} presents as certified without a certified target"
                )
            }
            Self::MissingWedgeCoverage { wedge } => {
                write!(f, "wedge {} has no review", wedge.as_str())
            }
            Self::MissingOperationCoverage { operation } => {
                write!(f, "operation {} is never exercised", operation.as_str())
            }
            Self::SummaryMismatch => write!(f, "summary does not equal recomputed summary"),
        }
    }
}

impl Error for M5BundleReviewAndRollbackViolation {}

/// Loads the embedded canonical M5 bundle-review-and-rollback packet.
///
/// # Errors
///
/// Returns a deserialization error if the embedded JSON does not parse into the
/// typed packet.
pub fn current_m5_bundle_review_and_rollback_packet(
) -> Result<M5BundleReviewAndRollbackPacket, serde_json::Error> {
    serde_json::from_str(M5_BUNDLE_REVIEW_AND_ROLLBACK_JSON)
}

#[cfg(test)]
mod tests;
