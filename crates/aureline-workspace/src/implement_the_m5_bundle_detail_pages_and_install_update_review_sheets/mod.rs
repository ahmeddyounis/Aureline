//! Implements the reusable bundle detail / review primitive: a bundle detail page
//! and an install / update review sheet that both resolve from one review context and
//! share one review identity, so start-center, workspace, bundle, extension, migration,
//! diagnostics, and support surfaces show the *same diffed bundle truth* — what will
//! change in the workspace, profile, and installed package set — *before* a user adopts,
//! updates, or removes a supported workflow bundle.
//!
//! Where
//! [`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix`]
//! *freezes* the reusable workflow-bundle component families as a governed contract, and
//! [`crate::implement_the_m5_start_center_bundle_cards_and_certified_archetype_badge_groups`]
//! narrows the start-center-card / certified-archetype-badge families, this module *narrows*
//! two more of those families —
//! [`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::M5WorkflowBundleComponentFamily::BundleDetailPage`]
//! and
//! [`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::M5WorkflowBundleComponentFamily::BundleInstallUpdateReviewSheet`]
//! — into one working primitive with a real **resolver**. A single review context projects
//! onto two surfaces that share one review identity, so the bundle's component inventory,
//! diff scope, dependency markers, side effects, mirror/offline posture, and rollback
//! checkpoint never blur across the detail page and the install / update review sheet.
//!
//! The resolver reuses the canonical review vocabulary already carried by
//! [`crate::m5_bundle_review_and_rollback`] ([`ComponentDiffEntry`], [`RollbackCheckpoint`],
//! [`BundleReviewOperation`], [`DiffAction`], [`AssetOwnership`], [`ResolutionChoice`]) and
//! the manifest / scorecard / governance vocabulary — never a bespoke per-flow diff or
//! rollback model.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — bundle adoption no longer hides what will change.** The detail page lists the
//!   full component inventory (extensions, presets, tasks, docs / tour packs, templates /
//!   scaffolds, migration maps) plus dependency markers and changelog, and the review sheet
//!   enumerates every added / removed / changed component with its ownership, resolution,
//!   and side effects — a review can never claim "no change" while a real diff exists.
//! - **AC2 — review sheets stay intelligible under mirror / offline and policy-constrained
//!   conditions.** The sheet carries its mirror/offline truth mode and any narrowing block,
//!   keeps a blocked-by-policy asset's honest (compare / keep-local) resolution, and derives
//!   a review posture (`ready_to_apply` / `constrained_by_policy` / `read_only_comparison`)
//!   so a constrained review still reads truthfully.
//! - **AC3 — every claimed stack-entry surface points to the same diffed bundle truth.** The
//!   detail page and the review sheet share one review identity and one diff model, so a
//!   mutating install / update / remove surface and a read-only drift-review surface never
//!   tell two different change stories.
//!
//! Raw manifest bytes, credentials, entitlement tokens, mirror URLs, and provider cursors
//! never cross this boundary; the resolver carries only opaque refs, typed class tokens,
//! booleans, and redacted labels, so support and diagnostics exports reconstruct exactly what
//! a surface would have shown without leaking source or live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-bundle-detail-review-primitive.schema.json`](../../../../schemas/ui/m5-bundle-detail-review-primitive.schema.json).
//! The contract doc is
//! [`docs/bundles/m5_bundle_detail_review_primitive.md`](../../../../docs/bundles/m5_bundle_detail_review_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the primitive binds to the freeze matrix's
// truth-mode, downgrade-trigger, and degraded-state tokens rather than mint parallel ones.
use crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::{
    DegradedState, M5BundleComponentDowngradeTrigger, M5BundleTruthMode,
};
// Reused canonical review / rollback vocabulary — the review sheet binds to the same diff
// and checkpoint model the install / update / remove / drift review flows already carry.
use crate::m5_bundle_review_and_rollback::{
    AssetOwnership, BundleReviewOperation, ComponentDiffEntry, DiffAction, ResolutionChoice,
    RollbackCheckpoint,
};
// Reused canonical bundle / scorecard / governance vocabulary already carried by the frozen
// bundle-manifest, scorecard, and entry-governance contracts.
use crate::m5_bundle_scorecards::{
    BundleScorecardClass, EvidenceFreshness, ImportedVsNativeConfidence,
};
use crate::m5_entry_and_bundle_governance::{BundleClass, SourceTrust};
use crate::m5_workflow_bundle_manifests::{
    BundleComponentKind, CertificationTarget, LifecycleStage,
};

/// Stable record-kind tag carried by [`M5BundleDetailReviewPacket`].
pub const M5_BUNDLE_REVIEW_RECORD_KIND: &str = "m5_bundle_detail_review_primitive";

/// Schema version for the bundle detail / review primitive packet.
pub const M5_BUNDLE_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_BUNDLE_REVIEW_SCHEMA_REF: &str =
    "schemas/ui/m5-bundle-detail-review-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BUNDLE_REVIEW_DOC_REF: &str = "docs/bundles/m5_bundle_detail_review_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive narrows.
pub const M5_BUNDLE_REVIEW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-workflow-bundle-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_BUNDLE_REVIEW_FIXTURE_DIR: &str = "fixtures/ui/m5-bundle-detail-review-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const M5_BUNDLE_REVIEW_ARTIFACT_REF: &str =
    "artifacts/release/m5-bundle-detail-review-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_BUNDLE_REVIEW_CSV_REF: &str =
    "artifacts/release/m5-bundle-detail-review-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_BUNDLE_REVIEW_REPORT_REF: &str =
    "artifacts/release/m5-bundle-detail-review-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed bundle-review surface family. Each family is one parity surface that ingests the
/// shared primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleReviewSurfaceFamily {
    /// The bundle detail page describing one workflow bundle in full.
    BundleDetailPage,
    /// The install review sheet shown before a bundle is first adopted.
    InstallReviewSheet,
    /// The update review sheet shown before an installed bundle is changed.
    UpdateReviewSheet,
    /// The drift-review sheet comparing local state against the bundle read-only.
    DriftReviewSheet,
    /// The migration review view reconstructing an imported bundle's diffed truth.
    MigrationReviewView,
    /// The support / export replay surface reconstructing review truth offline.
    SupportExportReplay,
}

impl M5BundleReviewSurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BundleDetailPage,
        Self::InstallReviewSheet,
        Self::UpdateReviewSheet,
        Self::DriftReviewSheet,
        Self::MigrationReviewView,
        Self::SupportExportReplay,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundleDetailPage => "bundle_detail_page",
            Self::InstallReviewSheet => "install_review_sheet",
            Self::UpdateReviewSheet => "update_review_sheet",
            Self::DriftReviewSheet => "drift_review_sheet",
            Self::MigrationReviewView => "migration_review_view",
            Self::SupportExportReplay => "support_export_replay",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::BundleDetailPage => "Bundle detail page",
            Self::InstallReviewSheet => "Install review sheet",
            Self::UpdateReviewSheet => "Update review sheet",
            Self::DriftReviewSheet => "Drift-review sheet",
            Self::MigrationReviewView => "Migration review view",
            Self::SupportExportReplay => "Support / export replay",
        }
    }
}

/// Closed review posture. Names whether an install / update review is ready to apply,
/// constrained by policy, or a read-only comparison — the AC2 legibility a review sheet must
/// derive so a constrained or read-only review never reads as an ordinary "apply now".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleReviewPosture {
    /// A mutating review whose diff carries no blocked assets: ready to apply after review.
    ReadyToApply,
    /// A mutating review whose diff carries a policy- or lifecycle-blocked asset: still
    /// intelligible, but a decision is constrained.
    ConstrainedByPolicy,
    /// A read-only drift comparison that mutates nothing.
    ReadOnlyComparison,
}

impl M5BundleReviewPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ReadyToApply,
        Self::ConstrainedByPolicy,
        Self::ReadOnlyComparison,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToApply => "ready_to_apply",
            Self::ConstrainedByPolicy => "constrained_by_policy",
            Self::ReadOnlyComparison => "read_only_comparison",
        }
    }

    /// Derives the posture a review sheet publishes from the operation and whether any diff
    /// row is blocked by policy / lifecycle: a read-only drift review is always a comparison,
    /// a mutating review with a blocked asset is constrained, and an unblocked mutating review
    /// is ready to apply.
    pub const fn for_review(operation: BundleReviewOperation, has_blocked_asset: bool) -> Self {
        if !operation.is_mutating() {
            Self::ReadOnlyComparison
        } else if has_blocked_asset {
            Self::ConstrainedByPolicy
        } else {
            Self::ReadyToApply
        }
    }

    /// True when the posture describes a durable, mutating apply.
    pub const fn is_mutating(self) -> bool {
        matches!(self, Self::ReadyToApply | Self::ConstrainedByPolicy)
    }
}

/// Closed bundle dependency marker. Names an entitlement / policy / lifecycle / platform
/// dependency the detail page and review sheet must disclose rather than pull in silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleDependencyMarker {
    /// The bundle requires an entitlement to install or run.
    EntitlementRequired,
    /// The bundle is gated by org policy.
    PolicyGated,
    /// The bundle depends on a preview-stage capability.
    PreviewCapability,
    /// The bundle depends on a labs / experimental capability.
    LabsCapability,
    /// The bundle is available only through a mirror.
    MirrorOnlySource,
    /// The bundle is bounded to specific platforms.
    BoundedPlatform,
}

impl M5BundleDependencyMarker {
    /// Every dependency marker, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EntitlementRequired,
        Self::PolicyGated,
        Self::PreviewCapability,
        Self::LabsCapability,
        Self::MirrorOnlySource,
        Self::BoundedPlatform,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntitlementRequired => "entitlement_required",
            Self::PolicyGated => "policy_gated",
            Self::PreviewCapability => "preview_capability",
            Self::LabsCapability => "labs_capability",
            Self::MirrorOnlySource => "mirror_only_source",
            Self::BoundedPlatform => "bounded_platform",
        }
    }

    /// The lifecycle-derived marker for a non-stable capability stage, when any. Stable
    /// capabilities need no lifecycle dependency marker.
    pub const fn for_lifecycle_stage(stage: LifecycleStage) -> Option<Self> {
        match stage {
            LifecycleStage::Stable => None,
            LifecycleStage::Preview => Some(Self::PreviewCapability),
            LifecycleStage::Labs => Some(Self::LabsCapability),
            LifecycleStage::PolicyGated => Some(Self::PolicyGated),
            LifecycleStage::MirrorOnly => Some(Self::MirrorOnlySource),
            LifecycleStage::BoundedPlatform => Some(Self::BoundedPlatform),
        }
    }
}

/// Closed side-effect class a bundle install / update carries. Names the toolchain / scaffold
/// / settings / task / docs side effects a review sheet discloses before applying.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleSideEffectClass {
    /// Installs a workspace extension.
    ExtensionInstall,
    /// Writes a settings / profile / layout preset.
    SettingsProfileWrite,
    /// Registers a task / launch / debug recipe.
    TaskRecipeRegistration,
    /// Installs a docs / tour pack.
    DocsTourPackInstall,
    /// Runs a toolchain install or update.
    ToolchainInstall,
    /// Runs a scaffold / template generation.
    ScaffoldGeneration,
}

impl M5BundleSideEffectClass {
    /// Every side-effect class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExtensionInstall,
        Self::SettingsProfileWrite,
        Self::TaskRecipeRegistration,
        Self::DocsTourPackInstall,
        Self::ToolchainInstall,
        Self::ScaffoldGeneration,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionInstall => "extension_install",
            Self::SettingsProfileWrite => "settings_profile_write",
            Self::TaskRecipeRegistration => "task_recipe_registration",
            Self::DocsTourPackInstall => "docs_tour_pack_install",
            Self::ToolchainInstall => "toolchain_install",
            Self::ScaffoldGeneration => "scaffold_generation",
        }
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must carry per
/// surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleReviewExportField {
    /// The stable review identity shared across surfaces.
    ReviewId,
    /// The opaque bundle identity ref and human name.
    BundleIdentity,
    /// The shared source class (certified / managed / community / imported / draft).
    SourceClass,
    /// The diff scope: the enumerated added / removed / changed components.
    DiffScope,
    /// The rollback checkpoint created before a mutating change.
    RollbackCheckpoint,
    /// The entitlement / policy / lifecycle / platform dependency markers.
    DependencyMarkers,
    /// The mirror / offline posture of the source.
    MirrorOfflinePosture,
    /// The changelog reference.
    ChangelogRef,
}

impl M5BundleReviewExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ReviewId,
        Self::BundleIdentity,
        Self::SourceClass,
        Self::DiffScope,
        Self::RollbackCheckpoint,
        Self::DependencyMarkers,
        Self::MirrorOfflinePosture,
        Self::ChangelogRef,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ReviewId,
        Self::BundleIdentity,
        Self::DiffScope,
        Self::RollbackCheckpoint,
        Self::MirrorOfflinePosture,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewId => "review_id",
            Self::BundleIdentity => "bundle_identity",
            Self::SourceClass => "source_class",
            Self::DiffScope => "diff_scope",
            Self::RollbackCheckpoint => "rollback_checkpoint",
            Self::DependencyMarkers => "dependency_markers",
            Self::MirrorOfflinePosture => "mirror_offline_posture",
            Self::ChangelogRef => "changelog_ref",
        }
    }
}

/// One component the detail page inventories: an extension, preset, recipe, docs / tour pack,
/// template / scaffold ref, or migration mapping the bundle composes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleComponentSummary {
    /// Which content category this component contributes.
    pub component_kind: BundleComponentKind,
    /// Stable component identifier within the bundle.
    pub component_id: String,
    /// A human-readable, one-line component label.
    pub label: String,
    /// The lifecycle stage of the capability this component depends on.
    pub lifecycle_stage: LifecycleStage,
}

impl M5BundleComponentSummary {
    /// Whether this summary is internally consistent.
    pub fn is_consistent(&self) -> bool {
        !self.component_id.trim().is_empty() && !self.label.trim().is_empty()
    }
}

// --- resolver input ---

/// The full input to the bundle detail / review resolver for one review context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleReviewInput {
    /// The stable review identity that must survive across the detail page and the review sheet.
    pub review_id: String,
    /// Human-readable surface / context label.
    pub surface_label: String,
    /// Opaque ref to the bundle id under review; never raw manifest bytes.
    pub bundle_id_ref: String,
    /// Human-readable bundle name shown on the page.
    pub bundle_name: String,
    /// The bundle class under review.
    pub bundle_class: BundleClass,
    /// Signer / source trust of the bundle.
    pub signer_source: SourceTrust,
    /// Support-class / lifecycle stage of the bundle.
    pub support_class: LifecycleStage,
    /// The shared source class (certified / managed / community / imported / draft).
    pub source_class: CertificationTarget,
    /// The scorecard class the bundle carries.
    pub scorecard_class: BundleScorecardClass,
    /// Certification freshness of the claim.
    pub certification_freshness: EvidenceFreshness,
    /// Imported-vs-native confidence contributing to the assurance story.
    pub imported_confidence: ImportedVsNativeConfidence,
    /// Compatible Aureline range the bundle declares (opaque range token).
    pub compatible_aureline_range: String,
    /// The provenance / freshness truth class the review binds to.
    pub truth_mode: M5BundleTruthMode,
    /// Opaque ref to the bundle changelog; never empty.
    pub changelog_ref: String,
    /// Opaque refs to evidence / proof links the detail page surfaces.
    pub evidence_link_refs: Vec<String>,
    /// The detail page component inventory (extensions, presets, recipes, docs, templates,
    /// migration maps); never empty.
    pub component_inventory: Vec<M5BundleComponentSummary>,
    /// The entitlement / policy / lifecycle / platform dependency markers the bundle declares.
    pub dependency_markers: Vec<M5BundleDependencyMarker>,
    /// The review operation the sheet reviews.
    pub operation: BundleReviewOperation,
    /// The enumerated per-component diff rows (added / removed / changed).
    pub diff_rows: Vec<ComponentDiffEntry>,
    /// The toolchain / scaffold / settings / docs side effects the change carries.
    pub side_effects: Vec<M5BundleSideEffectClass>,
    /// The one-step rollback checkpoint created before a mutating change. Required for a
    /// mutating install / update / remove; may be absent for a read-only drift review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint: Option<RollbackCheckpoint>,
    /// The review claims no change despite an enumerated diff; must be `false`.
    pub claims_no_change_despite_diff: bool,
    /// The review claims a current certification despite stale / missing freshness; must be
    /// `false`.
    pub claims_current_despite_stale: bool,
    /// An externally-observed narrowing (stale mirror, offline cache, imported-not-native)
    /// carried through onto the review before action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved bundle detail page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBundleDetailPage {
    /// The review identity — identical to the review sheet.
    pub review_id: String,
    /// The opaque bundle id ref.
    pub bundle_id_ref: String,
    /// The human-readable bundle name.
    pub bundle_name: String,
    /// The bundle class under review.
    pub bundle_class: BundleClass,
    /// The signer / source trust of the bundle.
    pub signer_source: SourceTrust,
    /// The support / lifecycle class of the bundle.
    pub support_class: LifecycleStage,
    /// The shared source class named explicitly on the page.
    pub source_class: CertificationTarget,
    /// The scorecard class the bundle carries.
    pub scorecard_class: BundleScorecardClass,
    /// The certification freshness of the claim.
    pub certification_freshness: EvidenceFreshness,
    /// The compatible Aureline range the bundle declares.
    pub compatible_aureline_range: String,
    /// The provenance / freshness truth class.
    pub truth_mode: M5BundleTruthMode,
    /// The opaque changelog ref.
    pub changelog_ref: String,
    /// The opaque evidence / proof link refs.
    pub evidence_link_refs: Vec<String>,
    /// The full component inventory.
    pub component_inventory: Vec<M5BundleComponentSummary>,
    /// The declared dependency markers.
    pub dependency_markers: Vec<M5BundleDependencyMarker>,
    /// The page lists the full component inventory (AC1); always holds.
    pub lists_full_inventory: bool,
    /// The page discloses mirror / offline posture; always holds.
    pub discloses_mirror_offline_posture: bool,
    /// The page discloses dependency markers; always holds.
    pub discloses_dependency_markers: bool,
}

/// The resolved install / update review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedInstallUpdateReviewSheet {
    /// The review identity — identical to the detail page.
    pub review_id: String,
    /// The review operation.
    pub operation: BundleReviewOperation,
    /// The enumerated per-component diff rows.
    pub diff_rows: Vec<ComponentDiffEntry>,
    /// The side effects the change carries.
    pub side_effects: Vec<M5BundleSideEffectClass>,
    /// The dependency markers — identical to the detail page.
    pub dependency_markers: Vec<M5BundleDependencyMarker>,
    /// The rollback checkpoint created before a mutating change, when any.
    pub rollback_checkpoint: Option<RollbackCheckpoint>,
    /// The provenance / freshness truth class the sheet binds to.
    pub truth_mode: M5BundleTruthMode,
    /// The derived review posture (AC2).
    pub review_posture: M5BundleReviewPosture,
    /// The sheet enumerates every added / removed / changed component (AC1); always holds.
    pub enumerates_every_change: bool,
    /// The sheet creates a rollback checkpoint before a mutating change (true for mutating,
    /// false for a read-only drift review).
    pub creates_rollback_checkpoint: bool,
    /// The sheet reviews before applying a durable change; always holds.
    pub reviewed_before_apply: bool,
    /// The sheet discloses diff scope; always holds.
    pub discloses_diff_scope: bool,
}

/// The resolved bundle review truth shared across the detail page and the review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBundleReview {
    /// The stable review identity.
    pub review_id: String,
    /// The resolved bundle detail page.
    pub detail_page: M5ResolvedBundleDetailPage,
    /// The resolved install / update review sheet.
    pub review_sheet: M5ResolvedInstallUpdateReviewSheet,
    /// Bundle adoption discloses every change (AC1).
    pub change_fully_disclosed: bool,
    /// The review stays intelligible under mirror / offline / policy constraint (AC2).
    pub intelligible_under_constraint: bool,
    /// The detail page and review sheet share one diffed truth (AC3).
    pub shared_diff_truth: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedBundleReview {
    /// True when the review identity is identical across the detail page and review sheet.
    pub fn identity_consistent(&self) -> bool {
        self.detail_page.review_id == self.review_id
            && self.review_sheet.review_id == self.review_id
    }

    /// True when the detail page and review sheet name the same dependency markers — the
    /// review never tells two dependency stories.
    pub fn dependency_markers_consistent(&self) -> bool {
        self.detail_page.dependency_markers == self.review_sheet.dependency_markers
    }

    /// True when bundle adoption discloses every change (AC1).
    pub fn change_fully_disclosed(&self) -> bool {
        self.change_fully_disclosed
    }

    /// True when the review stays intelligible under constraint (AC2).
    pub fn intelligible_under_constraint(&self) -> bool {
        self.intelligible_under_constraint
    }

    /// True when the detail page and review sheet point to the same diffed truth (AC3).
    pub fn shared_diff_truth(&self) -> bool {
        self.shared_diff_truth
    }

    /// True when at least one diff row requires a real review decision.
    pub fn has_decision_requiring_change(&self) -> bool {
        self.review_sheet
            .diff_rows
            .iter()
            .any(|row| row.diff_action.requires_decision())
    }
}

/// Errors returned by [`resolve_bundle_review`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BundleReviewResolutionError {
    /// The review identity was empty.
    EmptyReviewId,
    /// The bundle id ref was empty.
    EmptyBundleIdRef,
    /// The bundle name was empty.
    EmptyBundleName,
    /// The compatible Aureline range was empty.
    EmptyCompatibleRange,
    /// The changelog ref was empty.
    EmptyChangelogRef,
    /// The component inventory was empty.
    EmptyComponentInventory,
    /// A component-inventory summary was incomplete.
    ComponentSummaryIncomplete,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// A mutating install / update / remove offered no one-step rollback checkpoint.
    MutatingOpWithoutCheckpoint,
    /// A diff row recorded a resolution unsafe for its ownership and diff action.
    UnsafeResolution,
    /// A diff row was internally inconsistent (opaque blob, ungated lifecycle, blank ids).
    InconsistentDiffRow,
    /// A review claimed no change despite an enumerated, decision-requiring diff.
    HiddenChange,
    /// A non-stable capability was inventoried without a disclosed dependency marker.
    DependencyMarkerHidden,
    /// A stale / missing certification was claimed as current instead of narrowing.
    StaleClaimShownAsCurrent,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5BundleReviewResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyReviewId => "empty_review_id",
            Self::EmptyBundleIdRef => "empty_bundle_id_ref",
            Self::EmptyBundleName => "empty_bundle_name",
            Self::EmptyCompatibleRange => "empty_compatible_range",
            Self::EmptyChangelogRef => "empty_changelog_ref",
            Self::EmptyComponentInventory => "empty_component_inventory",
            Self::ComponentSummaryIncomplete => "component_summary_incomplete",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::MutatingOpWithoutCheckpoint => "mutating_op_without_checkpoint",
            Self::UnsafeResolution => "unsafe_resolution",
            Self::InconsistentDiffRow => "inconsistent_diff_row",
            Self::HiddenChange => "hidden_change",
            Self::DependencyMarkerHidden => "dependency_marker_hidden",
            Self::StaleClaimShownAsCurrent => "stale_claim_shown_as_current",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5BundleReviewResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "bundle-review resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5BundleReviewResolutionError {}

/// Resolves one bundle review context into its shared detail page and install / update review
/// sheet.
///
/// The two surfaces share one review identity, so the bundle's component inventory, diff
/// scope, dependency markers, side effects, mirror/offline posture, and rollback checkpoint
/// never blur across them. The detail page always lists the full inventory and dependency
/// markers; the review sheet always enumerates every added / removed / changed component,
/// creates a one-step rollback checkpoint before any mutating change, and derives a review
/// posture so a policy-constrained or read-only review still reads truthfully; a review can
/// never claim "no change" while a real diff exists, and a stale certification never reads as
/// current.
pub fn resolve_bundle_review(
    input: &M5BundleReviewInput,
) -> Result<M5ResolvedBundleReview, M5BundleReviewResolutionError> {
    if input.review_id.trim().is_empty() {
        return Err(M5BundleReviewResolutionError::EmptyReviewId);
    }
    if input.bundle_id_ref.trim().is_empty() {
        return Err(M5BundleReviewResolutionError::EmptyBundleIdRef);
    }
    if input.bundle_name.trim().is_empty() {
        return Err(M5BundleReviewResolutionError::EmptyBundleName);
    }
    if input.compatible_aureline_range.trim().is_empty() {
        return Err(M5BundleReviewResolutionError::EmptyCompatibleRange);
    }
    if input.changelog_ref.trim().is_empty() {
        return Err(M5BundleReviewResolutionError::EmptyChangelogRef);
    }
    if input.component_inventory.is_empty() {
        return Err(M5BundleReviewResolutionError::EmptyComponentInventory);
    }
    if input
        .component_inventory
        .iter()
        .any(|summary| !summary.is_consistent())
    {
        return Err(M5BundleReviewResolutionError::ComponentSummaryIncomplete);
    }

    if input_carries_forbidden_material(input) {
        return Err(M5BundleReviewResolutionError::ForbiddenMaterial);
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5BundleReviewResolutionError::DegradedLabelGeneric);
        }
    }

    // AC1: a review never claims "no change" while a decision-requiring diff exists.
    let has_decision_requiring_change = input
        .diff_rows
        .iter()
        .any(|row| row.diff_action.requires_decision());
    if input.claims_no_change_despite_diff && has_decision_requiring_change {
        return Err(M5BundleReviewResolutionError::HiddenChange);
    }

    // Every enumerated diff row is a reviewable, safely-resolved reference — never an opaque
    // blob and never a resolution that would erase user-protected state or adopt a blocked
    // asset.
    for row in &input.diff_rows {
        if !row.resolution_safe() {
            return Err(M5BundleReviewResolutionError::UnsafeResolution);
        }
        if !row.is_consistent() {
            return Err(M5BundleReviewResolutionError::InconsistentDiffRow);
        }
    }

    // The detail page must disclose a dependency marker whenever it inventories a non-stable
    // capability; a preview / labs / policy-gated / mirror-only / bounded component can never
    // be pulled in as if it were stable.
    for summary in &input.component_inventory {
        if let Some(required) =
            M5BundleDependencyMarker::for_lifecycle_stage(summary.lifecycle_stage)
        {
            if !input.dependency_markers.contains(&required) {
                return Err(M5BundleReviewResolutionError::DependencyMarkerHidden);
            }
        }
    }

    // AC: a mutating install / update / remove must create a one-step rollback checkpoint
    // captured before the mutation commits.
    if input.operation.is_mutating() {
        let has_checkpoint = input
            .rollback_checkpoint
            .as_ref()
            .is_some_and(RollbackCheckpoint::supports_one_step_rollback);
        if !has_checkpoint {
            return Err(M5BundleReviewResolutionError::MutatingOpWithoutCheckpoint);
        }
    }

    // AC2: a stale / missing certification narrows the claim rather than being shown as
    // current.
    let freshness_is_stale = matches!(
        input.certification_freshness,
        EvidenceFreshness::Stale | EvidenceFreshness::Missing
    );
    if input.claims_current_despite_stale && freshness_is_stale {
        return Err(M5BundleReviewResolutionError::StaleClaimShownAsCurrent);
    }

    let has_blocked_asset = input.diff_rows.iter().any(|row| row.ownership.is_blocked());
    let review_posture = M5BundleReviewPosture::for_review(input.operation, has_blocked_asset);
    let creates_rollback_checkpoint = input.operation.is_mutating();

    let detail_page = M5ResolvedBundleDetailPage {
        review_id: input.review_id.clone(),
        bundle_id_ref: input.bundle_id_ref.clone(),
        bundle_name: input.bundle_name.clone(),
        bundle_class: input.bundle_class,
        signer_source: input.signer_source,
        support_class: input.support_class,
        source_class: input.source_class,
        scorecard_class: input.scorecard_class,
        certification_freshness: input.certification_freshness,
        compatible_aureline_range: input.compatible_aureline_range.clone(),
        truth_mode: input.truth_mode,
        changelog_ref: input.changelog_ref.clone(),
        evidence_link_refs: input.evidence_link_refs.clone(),
        component_inventory: input.component_inventory.clone(),
        dependency_markers: input.dependency_markers.clone(),
        lists_full_inventory: true,
        discloses_mirror_offline_posture: true,
        discloses_dependency_markers: true,
    };

    let review_sheet = M5ResolvedInstallUpdateReviewSheet {
        review_id: input.review_id.clone(),
        operation: input.operation,
        diff_rows: input.diff_rows.clone(),
        side_effects: input.side_effects.clone(),
        dependency_markers: input.dependency_markers.clone(),
        rollback_checkpoint: input.rollback_checkpoint.clone(),
        truth_mode: input.truth_mode,
        review_posture,
        enumerates_every_change: true,
        creates_rollback_checkpoint,
        reviewed_before_apply: true,
        discloses_diff_scope: true,
    };

    Ok(M5ResolvedBundleReview {
        review_id: input.review_id.clone(),
        detail_page,
        review_sheet,
        change_fully_disclosed: true,
        intelligible_under_constraint: true,
        shared_diff_truth: true,
        degraded: input.degraded.clone(),
    })
}

/// True when any label, ref, or note on the input carries obviously forbidden material.
fn input_carries_forbidden_material(input: &M5BundleReviewInput) -> bool {
    let mut values: Vec<&str> = vec![
        input.review_id.as_str(),
        input.surface_label.as_str(),
        input.bundle_id_ref.as_str(),
        input.bundle_name.as_str(),
        input.compatible_aureline_range.as_str(),
        input.changelog_ref.as_str(),
    ];
    values.extend(input.evidence_link_refs.iter().map(String::as_str));
    for summary in &input.component_inventory {
        values.push(summary.component_id.as_str());
        values.push(summary.label.as_str());
    }
    for row in &input.diff_rows {
        values.push(row.component_id.as_str());
        values.push(row.label.as_str());
        values.push(row.diff_preview_ref.as_str());
        if let Some(local_override_ref) = &row.local_override_ref {
            values.push(local_override_ref.as_str());
        }
    }
    if let Some(checkpoint) = &input.rollback_checkpoint {
        values.push(checkpoint.checkpoint_ref.as_str());
    }
    values.into_iter().any(value_is_forbidden)
}

/// True when a label, ref, or note carries obviously forbidden material.
fn value_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs bundle-review truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleReviewCase {
    /// The resolver input.
    pub input: M5BundleReviewInput,
    /// The resolved bundle-review truth. Must equal `resolve_bundle_review(&input)`.
    pub resolved: M5ResolvedBundleReview,
}

impl M5BundleReviewCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5BundleReviewInput) -> Self {
        let resolved = resolve_bundle_review(&input).expect("seed bundle-review case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_bundle_review(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one bundle-review surface family bound to the shared
/// review contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleReviewSurfaceRow {
    /// The bundle-review surface family.
    pub surface_family: M5BundleReviewSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Review operations this surface can review (must be non-empty).
    pub operations: Vec<BundleReviewOperation>,
    /// Source classes this surface can disclose (must be non-empty).
    pub source_classes: Vec<CertificationTarget>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<M5BundleTruthMode>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5BundleReviewExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5BundleComponentDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be non-empty).
    pub example_reviews: Vec<M5BundleReviewCase>,
    /// Hard invariant: this row never hides diff scope. MUST be `false`.
    pub hides_diff_scope: bool,
    /// Hard invariant: this row never applies before review. MUST be `false`.
    pub applies_before_review: bool,
    /// Hard invariant: this row never hides a dependency marker. MUST be `false`.
    pub hides_dependency_markers: bool,
}

impl M5BundleReviewSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5BundleReviewExportField> =
            self.export_fields.iter().copied().collect();
        M5BundleReviewExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_diff_scope && !self.applies_before_review && !self.hides_dependency_markers
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleReviewVocabularySet {
    /// Bundle-review surface-family tokens.
    pub surface_families: Vec<String>,
    /// Review-posture tokens.
    pub review_postures: Vec<String>,
    /// Dependency-marker tokens.
    pub dependency_markers: Vec<String>,
    /// Side-effect-class tokens.
    pub side_effect_classes: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Component-kind tokens (reused from the bundle-manifest contract).
    pub component_kinds: Vec<String>,
    /// Review-operation tokens (reused from the review / rollback contract).
    pub review_operations: Vec<String>,
    /// Diff-action tokens (reused from the review / rollback contract).
    pub diff_actions: Vec<String>,
    /// Asset-ownership tokens (reused from the review / rollback contract).
    pub asset_ownerships: Vec<String>,
    /// Resolution-choice tokens (reused from the review / rollback contract).
    pub resolution_choices: Vec<String>,
    /// Source-class tokens (reused from the bundle-manifest contract).
    pub source_classes: Vec<String>,
    /// Bundle-class tokens (reused from the entry-governance contract).
    pub bundle_classes: Vec<String>,
    /// Signer / source-trust tokens (reused from the entry-governance contract).
    pub signer_sources: Vec<String>,
    /// Support-class / lifecycle tokens (reused from the bundle-manifest contract).
    pub support_classes: Vec<String>,
    /// Scorecard-class tokens (reused from the scorecard contract).
    pub scorecard_classes: Vec<String>,
    /// Certification-freshness tokens (reused from the scorecard contract).
    pub freshness_states: Vec<String>,
    /// Imported-vs-native confidence tokens (reused from the scorecard contract).
    pub imported_confidences: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5BundleReviewVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(
                &M5BundleReviewSurfaceFamily::ALL,
                M5BundleReviewSurfaceFamily::as_str,
            ),
            review_postures: tokens(&M5BundleReviewPosture::ALL, M5BundleReviewPosture::as_str),
            dependency_markers: tokens(
                &M5BundleDependencyMarker::ALL,
                M5BundleDependencyMarker::as_str,
            ),
            side_effect_classes: tokens(
                &M5BundleSideEffectClass::ALL,
                M5BundleSideEffectClass::as_str,
            ),
            export_fields: tokens(
                &M5BundleReviewExportField::ALL,
                M5BundleReviewExportField::as_str,
            ),
            component_kinds: tokens(&BundleComponentKind::ALL, BundleComponentKind::as_str),
            review_operations: tokens(&BundleReviewOperation::ALL, BundleReviewOperation::as_str),
            diff_actions: tokens(&DiffAction::ALL, DiffAction::as_str),
            asset_ownerships: tokens(&AssetOwnership::ALL, AssetOwnership::as_str),
            resolution_choices: tokens(&ResolutionChoice::ALL, ResolutionChoice::as_str),
            source_classes: tokens(&CertificationTarget::ALL, CertificationTarget::as_str),
            bundle_classes: tokens(&BundleClass::ALL, BundleClass::as_str),
            signer_sources: tokens(&SourceTrust::ALL, SourceTrust::as_str),
            support_classes: tokens(&LifecycleStage::ALL, LifecycleStage::as_str),
            scorecard_classes: tokens(&BundleScorecardClass::ALL, BundleScorecardClass::as_str),
            freshness_states: tokens(&EvidenceFreshness::ALL, EvidenceFreshness::as_str),
            imported_confidences: tokens(
                &ImportedVsNativeConfidence::ALL,
                ImportedVsNativeConfidence::as_str,
            ),
            truth_modes: tokens(&M5BundleTruthMode::ALL, M5BundleTruthMode::as_str),
            downgrade_triggers: tokens(
                &DOWNGRADE_TRIGGER_ALL,
                M5BundleComponentDowngradeTrigger::as_str,
            ),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The downgrade triggers reused from the frozen matrix, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5BundleComponentDowngradeTrigger; 9] = [
    M5BundleComponentDowngradeTrigger::StaleCertification,
    M5BundleComponentDowngradeTrigger::MirrorStale,
    M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
    M5BundleComponentDowngradeTrigger::UnverifiedSigner,
    M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
    M5BundleComponentDowngradeTrigger::IncompatibleAureline,
    M5BundleComponentDowngradeTrigger::EntitlementDependencyUnmet,
    M5BundleComponentDowngradeTrigger::ImportedNotNative,
    M5BundleComponentDowngradeTrigger::RollbackOnlyPath,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleReviewGovernanceReview {
    /// One primitive carries detail-page and review-sheet truth on every surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Review identity is preserved across the detail page and review sheet.
    pub review_identity_preserved_across_surfaces: bool,
    /// Bundle adoption discloses what will change before it applies.
    pub change_disclosed_before_apply: bool,
    /// Review sheets stay intelligible under mirror / offline and policy constraint.
    pub review_intelligible_under_constraint: bool,
    /// A mutating change always creates a one-step rollback checkpoint.
    pub rollback_checkpoint_created_before_mutation: bool,
    /// The support / export packet reconstructs bundle-review truth.
    pub support_export_reconstructs_review: bool,
    /// Later M5 rows cannot invent parallel review / diff vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleReviewConsumerProjection {
    /// Detail / install / update / drift / migration / support surfaces all consume the shared
    /// primitive.
    pub review_surfaces_consume_shared_primitive: bool,
    /// The review resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The review sheet reads a single canonical diff / rollback source.
    pub review_sheet_reads_single_diff_source: bool,
    /// Support / export reads a single canonical review source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the bundle-review primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleReviewReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting review audit.
    pub review_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BundleDetailReviewPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BundleDetailReviewPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5BundleReviewSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BundleReviewVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BundleReviewGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BundleReviewConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5BundleReviewReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 bundle detail / review primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDetailReviewPacket {
    /// Record kind; must equal [`M5_BUNDLE_REVIEW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BUNDLE_REVIEW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5BundleReviewSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BundleReviewVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BundleReviewGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BundleReviewConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5BundleReviewReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BundleDetailReviewPacket {
    /// Builds an M5 bundle detail / review primitive packet from stable-lane input.
    pub fn new(input: M5BundleDetailReviewPacketInput) -> Self {
        Self {
            record_kind: M5_BUNDLE_REVIEW_RECORD_KIND.to_owned(),
            schema_version: M5_BUNDLE_REVIEW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 bundle-review primitive invariants.
    pub fn validate(&self) -> Vec<M5BundleReviewViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BUNDLE_REVIEW_RECORD_KIND {
            violations.push(M5BundleReviewViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BUNDLE_REVIEW_SCHEMA_VERSION {
            violations.push(M5BundleReviewViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BundleReviewViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 bundle-review primitive packet serializes"),
        ) {
            violations.push(M5BundleReviewViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 bundle-review primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,owner,operations,source_classes,truth_modes,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.operations, |v| v.as_str()),
                join_tokens(&row.source_classes, |v| v.as_str()),
                join_tokens(&row.truth_modes, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_reviews.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Bundle Detail / Review Primitive: Bundle Detail Page and Install / Update Review Sheet\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Bundle-review surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5BundleReviewSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Review operations: {}\n",
            self.vocabulary_set.review_operations.join(", ")
        ));
        out.push_str(&format!(
            "- Review postures: {}\n",
            self.vocabulary_set.review_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Dependency markers: {}\n",
            self.vocabulary_set.dependency_markers.join(", ")
        ));
        out.push_str("\n## Bundle-review surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cases: {}\n",
                row.example_reviews.len()
            ));
            for case in &row.example_reviews {
                out.push_str(&format!(
                    "    - `{}` → op `{}` (posture `{}`), {} diff row(s), source `{}`, range `{}`\n",
                    case.resolved.review_id,
                    case.resolved.review_sheet.operation.as_str(),
                    case.resolved.review_sheet.review_posture.as_str(),
                    case.resolved.review_sheet.diff_rows.len(),
                    case.resolved.detail_page.source_class.as_str(),
                    case.resolved.detail_page.compatible_aureline_range,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 bundle-review export.
#[derive(Debug)]
pub enum M5BundleReviewArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BundleReviewViolation>),
}

impl fmt::Display for M5BundleReviewArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 bundle-review primitive export parse failed: {error}"
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
                    "m5 bundle-review primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BundleReviewArtifactError {}

/// Validation failures emitted by [`M5BundleDetailReviewPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BundleReviewViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required bundle-review surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no review operations.
    OperationMissing,
    /// A surface row declares no source classes.
    SourceClassMissing,
    /// A surface row declares no truth classes.
    TruthModeMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked review cases.
    ExampleReviewsMissing,
    /// A worked review case does not match a fresh resolve of its input.
    ExampleReviewDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves the change fully disclosed with a real diff (AC1).
    ChangeDisclosureUnproven,
    /// No worked case proves the review stays intelligible under constraint (AC2).
    ConstraintIntelligibilityUnproven,
    /// No worked case proves the detail page and review sheet share one diffed truth (AC3).
    SharedDiffTruthUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BundleReviewViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::OperationMissing => "operation_missing",
            Self::SourceClassMissing => "source_class_missing",
            Self::TruthModeMissing => "truth_mode_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleReviewsMissing => "example_reviews_missing",
            Self::ExampleReviewDrift => "example_review_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::ChangeDisclosureUnproven => "change_disclosure_unproven",
            Self::ConstraintIntelligibilityUnproven => "constraint_intelligibility_unproven",
            Self::SharedDiffTruthUnproven => "shared_diff_truth_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 bundle-review export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_bundle_detail_review_export(
) -> Result<M5BundleDetailReviewPacket, M5BundleReviewArtifactError> {
    let packet: M5BundleDetailReviewPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-bundle-detail-review-primitive-proof/support_export.json"
    )))
    .map_err(M5BundleReviewArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BundleReviewArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BundleDetailReviewPacket,
    violations: &mut Vec<M5BundleReviewViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BUNDLE_REVIEW_SCHEMA_REF,
        M5_BUNDLE_REVIEW_DOC_REF,
        M5_BUNDLE_REVIEW_COMPONENT_MATRIX_REF,
        M5_BUNDLE_REVIEW_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BundleReviewViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BundleDetailReviewPacket,
    violations: &mut Vec<M5BundleReviewViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BundleReviewViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5BundleDetailReviewPacket,
    violations: &mut Vec<M5BundleReviewViolation>,
) {
    let present: BTreeSet<M5BundleReviewSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5BundleReviewSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5BundleReviewViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5BundleReviewViolation::SurfaceRowIncomplete);
        }
        if row.operations.is_empty() {
            violations.push(M5BundleReviewViolation::OperationMissing);
        }
        if row.source_classes.is_empty() {
            violations.push(M5BundleReviewViolation::SourceClassMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5BundleReviewViolation::TruthModeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5BundleReviewViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5BundleReviewViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BundleReviewViolation::ConsumerSurfacesMissing);
        }
        if row.example_reviews.is_empty() {
            violations.push(M5BundleReviewViolation::ExampleReviewsMissing);
        }
        if row
            .example_reviews
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5BundleReviewViolation::ExampleReviewDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5BundleReviewViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case across the
/// matrix: bundle adoption discloses every change with a real diff (AC1), the review stays
/// intelligible under mirror / offline / policy constraint (AC2), and the detail page and
/// review sheet share one diffed truth across a read-only and a mutating surface (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5BundleDetailReviewPacket,
    violations: &mut Vec<M5BundleReviewViolation>,
) {
    let cases: Vec<&M5ResolvedBundleReview> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_reviews.iter().map(|case| &case.resolved))
        .collect();

    // AC1: at least one case shows a real, decision-requiring diff and discloses the change;
    // and every case discloses its change fully.
    let change_disclosure_proven = cases.iter().any(|resolved| {
        resolved.change_fully_disclosed() && resolved.has_decision_requiring_change()
    }) && cases
        .iter()
        .all(|resolved| resolved.change_fully_disclosed());
    if !change_disclosure_proven {
        violations.push(M5BundleReviewViolation::ChangeDisclosureUnproven);
    }

    // AC2: at least one case is under a mirror / offline truth mode or carries a policy /
    // lifecycle constraint (constrained posture), and every case stays intelligible.
    let constraint_proven = cases.iter().any(|resolved| {
        !resolved.review_sheet.truth_mode.is_current_source()
            || resolved.review_sheet.review_posture == M5BundleReviewPosture::ConstrainedByPolicy
    }) && cases
        .iter()
        .all(|resolved| resolved.intelligible_under_constraint());
    if !constraint_proven {
        violations.push(M5BundleReviewViolation::ConstraintIntelligibilityUnproven);
    }

    // AC3: every case shares one diffed truth across its two projections, and the matrix spans
    // both a read-only comparison and a mutating apply pointing to that same model.
    let shared_truth_proven = cases
        .iter()
        .all(|resolved| resolved.identity_consistent() && resolved.shared_diff_truth())
        && cases
            .iter()
            .any(|resolved| resolved.review_sheet.review_posture.is_mutating())
        && cases.iter().any(|resolved| {
            resolved.review_sheet.review_posture == M5BundleReviewPosture::ReadOnlyComparison
        });
    if !shared_truth_proven {
        violations.push(M5BundleReviewViolation::SharedDiffTruthUnproven);
    }
}

fn validate_governance_review(
    packet: &M5BundleDetailReviewPacket,
    violations: &mut Vec<M5BundleReviewViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.review_identity_preserved_across_surfaces,
        review.change_disclosed_before_apply,
        review.review_intelligible_under_constraint,
        review.rollback_checkpoint_created_before_mutation,
        review.support_export_reconstructs_review,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5BundleReviewViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BundleDetailReviewPacket,
    violations: &mut Vec<M5BundleReviewViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.review_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.review_sheet_reads_single_diff_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5BundleReviewViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5BundleDetailReviewPacket,
    violations: &mut Vec<M5BundleReviewViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.review_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BundleReviewViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

include!("seed.rs");
