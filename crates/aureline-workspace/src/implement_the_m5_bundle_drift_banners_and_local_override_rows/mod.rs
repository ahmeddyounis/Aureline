//! Implements the reusable bundle drift primitive: a bundle drift banner, a set of
//! local-override rows at field / package / task granularity, and a rollback / remove card
//! that all resolve from one drift context and share one drift identity, so workspace, bundle,
//! extension, migration, diagnostics, and support surfaces explain the *same drift truth* —
//! what in the user's current state no longer matches a declared workflow bundle, at the right
//! level of detail — before a user rebases, keeps local changes, compares, or removes a bundle.
//!
//! Where
//! [`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix`]
//! *freezes* the reusable workflow-bundle component families as a governed contract,
//! [`crate::implement_the_m5_start_center_bundle_cards_and_certified_archetype_badge_groups`]
//! narrows the start-center-card / certified-archetype-badge families, and
//! [`crate::implement_the_m5_bundle_detail_pages_and_install_update_review_sheets`]
//! narrows the detail-page / install-update-review-sheet families, this module *narrows* the
//! remaining three review-time families —
//! [`M5WorkflowBundleComponentFamily::BundleDriftBanner`],
//! [`M5WorkflowBundleComponentFamily::BundleLocalOverrideRow`], and
//! [`M5WorkflowBundleComponentFamily::BundleRollbackRemoveCard`]
//! ([`crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::M5WorkflowBundleComponentFamily`])
//! — into one working primitive with a real **resolver**. A single drift context projects onto
//! a drift banner, a list of local-override rows, and a rollback / remove card that share one
//! drift identity, so a bundle's drift state, per-override detail, missing artifacts, recommended
//! choices, and rollback path never blur across the three surfaces.
//!
//! The resolver reuses the canonical review / rollback vocabulary already carried by
//! [`crate::m5_bundle_review_and_rollback`] ([`DriftState`], [`BundleReviewOperation`],
//! [`DiffAction`], [`AssetOwnership`], [`ResolutionChoice`], [`RollbackCheckpoint`]), the
//! side-effect vocabulary already minted by the bundle-review primitive
//! ([`M5BundleSideEffectClass`]), and the manifest / scorecard / governance vocabulary — never
//! a bespoke per-flow drift or rollback model. It adds only the drift-specific vocabulary the
//! resolver needs: the shared drift vocabulary ([`M5DriftKind`]), the field / package / task
//! granularity ([`M5DriftGranularity`]), and the harmless-versus-support-significant separation
//! ([`M5DriftSignificance`]).
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — bundle drift becomes reviewable at the right level of detail.** The banner
//!   enumerates the distinct drift kinds it is reporting (local-only edits, bundle version
//!   drift, missing artifacts, imported gaps, stale certification, policy / entitlement
//!   narrowing) and never collapses into a generic package-update warning; a banner that reads
//!   like a generic update is rejected.
//! - **AC2 — users can see harmless local preference versus support-significant drift.** Every
//!   local-override row carries a [`M5DriftSignificance`] derived from its drift kind, so a
//!   harmless preference is never painted as support-significant and a support-significant drift
//!   is never hidden as harmless.
//! - **AC3 — local overrides remain attributable and exportable without forcing a bundle
//!   reset.** Every override row is attributable at field / package / task granularity (never one
//!   opaque `customized` label), user-protected overrides are preserved rather than silently
//!   discarded, and the rollback / remove card never forces a reset to make drift exportable.
//!
//! Raw manifest bytes, credentials, entitlement tokens, mirror URLs, and provider cursors never
//! cross this boundary; the resolver carries only opaque refs, typed class tokens, booleans, and
//! redacted labels, so support and diagnostics exports reconstruct exactly what a surface would
//! have shown without leaking source or live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-bundle-drift-override-primitive.schema.json`](../../../../schemas/ui/m5-bundle-drift-override-primitive.schema.json).
//! The contract doc is
//! [`docs/bundles/m5_bundle_drift_override_primitive.md`](../../../../docs/bundles/m5_bundle_drift_override_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the primitive binds to the freeze matrix's truth-mode,
// downgrade-trigger, and degraded-state tokens rather than mint parallel ones.
use crate::freeze_the_m5_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_matrix::{
    DegradedState, M5BundleComponentDowngradeTrigger, M5BundleTruthMode,
};
// Reused side-effect vocabulary already minted by the bundle-review primitive: the rollback /
// remove card names removal side effects with the same closed class set the review sheet uses.
use crate::implement_the_m5_bundle_detail_pages_and_install_update_review_sheets::M5BundleSideEffectClass;
// Reused canonical review / rollback vocabulary — the drift primitive binds to the same drift,
// diff, ownership, resolution, and checkpoint model the install / update / remove / drift review
// flows already carry.
use crate::m5_bundle_review_and_rollback::{
    AssetOwnership, BundleReviewOperation, DiffAction, DriftState, ResolutionChoice,
    RollbackCheckpoint,
};
// Reused canonical bundle / scorecard / governance vocabulary already carried by the frozen
// bundle-manifest, scorecard, and entry-governance contracts.
use crate::m5_bundle_scorecards::{
    BundleScorecardClass, EvidenceFreshness, ImportedVsNativeConfidence,
};
use crate::m5_entry_and_bundle_governance::{BundleClass, SourceTrust};
use crate::m5_workflow_bundle_manifests::{BundleComponentKind, CertificationTarget, LifecycleStage};

/// Stable record-kind tag carried by [`M5BundleDriftOverridePacket`].
pub const M5_BUNDLE_DRIFT_RECORD_KIND: &str = "m5_bundle_drift_override_primitive";

/// Schema version for the bundle drift / override primitive packet.
pub const M5_BUNDLE_DRIFT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_BUNDLE_DRIFT_SCHEMA_REF: &str =
    "schemas/ui/m5-bundle-drift-override-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BUNDLE_DRIFT_DOC_REF: &str = "docs/bundles/m5_bundle_drift_override_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive narrows.
pub const M5_BUNDLE_DRIFT_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-workflow-bundle-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_BUNDLE_DRIFT_FIXTURE_DIR: &str = "fixtures/ui/m5-bundle-drift-override-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const M5_BUNDLE_DRIFT_ARTIFACT_REF: &str =
    "artifacts/release/m5-bundle-drift-override-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_BUNDLE_DRIFT_CSV_REF: &str =
    "artifacts/release/m5-bundle-drift-override-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_BUNDLE_DRIFT_REPORT_REF: &str =
    "artifacts/release/m5-bundle-drift-override-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed bundle-drift surface family. Each family is one parity surface that ingests the shared
/// primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleDriftSurfaceFamily {
    /// The workspace drift banner shown when local state has diverged from a bundle.
    WorkspaceDriftBanner,
    /// The bundle detail drift panel reviewing drift for one bundle in full.
    BundleDetailDriftPanel,
    /// The extension drift row shown in an extension / capability list.
    ExtensionDriftRow,
    /// The migration drift view reconstructing an imported bundle's drift.
    MigrationDriftView,
    /// The diagnostics drift report used for triage / support handoff.
    DiagnosticsDriftReport,
    /// The support / export replay surface reconstructing drift truth offline.
    SupportExportReplay,
}

impl M5BundleDriftSurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WorkspaceDriftBanner,
        Self::BundleDetailDriftPanel,
        Self::ExtensionDriftRow,
        Self::MigrationDriftView,
        Self::DiagnosticsDriftReport,
        Self::SupportExportReplay,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceDriftBanner => "workspace_drift_banner",
            Self::BundleDetailDriftPanel => "bundle_detail_drift_panel",
            Self::ExtensionDriftRow => "extension_drift_row",
            Self::MigrationDriftView => "migration_drift_view",
            Self::DiagnosticsDriftReport => "diagnostics_drift_report",
            Self::SupportExportReplay => "support_export_replay",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceDriftBanner => "Workspace drift banner",
            Self::BundleDetailDriftPanel => "Bundle detail drift panel",
            Self::ExtensionDriftRow => "Extension drift row",
            Self::MigrationDriftView => "Migration drift view",
            Self::DiagnosticsDriftReport => "Diagnostics drift report",
            Self::SupportExportReplay => "Support / export replay",
        }
    }
}

/// The one shared drift vocabulary. Every surface — banner, override row, rollback card, docs,
/// help, and export — names drift with the same closed set rather than coining per-flow wording:
/// a missing artifact, a local-only edit, a bundle version drift, an imported gap, a stale
/// certification, or a policy / entitlement narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DriftKind {
    /// A user has locally edited a bundle-owned asset (a harmless local preference).
    LocalOnlyEdit,
    /// The installed bundle version differs from the declared bundle version.
    BundleVersionDrift,
    /// A bundle-declared artifact is absent from the current state.
    MissingArtifact,
    /// An imported bundle is missing a declared component (an imported gap).
    ImportedGap,
    /// The bundle's certification is stale, narrowing its claim.
    StaleCertification,
    /// A policy or entitlement dependency narrows the bundle.
    PolicyEntitlementNarrowing,
}

impl M5DriftKind {
    /// Every drift kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalOnlyEdit,
        Self::BundleVersionDrift,
        Self::MissingArtifact,
        Self::ImportedGap,
        Self::StaleCertification,
        Self::PolicyEntitlementNarrowing,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyEdit => "local_only_edit",
            Self::BundleVersionDrift => "bundle_version_drift",
            Self::MissingArtifact => "missing_artifact",
            Self::ImportedGap => "imported_gap",
            Self::StaleCertification => "stale_certification",
            Self::PolicyEntitlementNarrowing => "policy_entitlement_narrowing",
        }
    }
}

/// The granularity at which a local override is attributed. The override row is reported at
/// field / package / task level rather than as one opaque `customized` label (AC3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DriftGranularity {
    /// One overridden field within a settings / profile asset.
    Field,
    /// One overridden package / extension.
    Package,
    /// One overridden task / launch / debug recipe.
    Task,
}

impl M5DriftGranularity {
    /// Every granularity, in declaration order.
    pub const ALL: [Self; 3] = [Self::Field, Self::Package, Self::Task];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Field => "field",
            Self::Package => "package",
            Self::Task => "task",
        }
    }
}

/// Whether a drift row is a harmless local preference or a support-significant difference (AC2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DriftSignificance {
    /// A harmless local preference the user can keep without narrowing support.
    HarmlessLocalPreference,
    /// A support-significant difference that narrows the bundle's claim or support.
    SupportSignificant,
}

impl M5DriftSignificance {
    /// Every significance, in declaration order.
    pub const ALL: [Self; 2] = [Self::HarmlessLocalPreference, Self::SupportSignificant];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HarmlessLocalPreference => "harmless_local_preference",
            Self::SupportSignificant => "support_significant",
        }
    }

    /// The significance a drift kind implies: a local-only edit is a harmless preference; every
    /// other kind (missing artifact, version drift, imported gap, stale certification, policy /
    /// entitlement narrowing) is support-significant.
    pub const fn for_kind(kind: M5DriftKind) -> Self {
        match kind {
            M5DriftKind::LocalOnlyEdit => Self::HarmlessLocalPreference,
            M5DriftKind::BundleVersionDrift
            | M5DriftKind::MissingArtifact
            | M5DriftKind::ImportedGap
            | M5DriftKind::StaleCertification
            | M5DriftKind::PolicyEntitlementNarrowing => Self::SupportSignificant,
        }
    }

    /// A rank used to derive the banner's highest significance. Higher wins.
    pub const fn rank(self) -> u8 {
        match self {
            Self::HarmlessLocalPreference => 0,
            Self::SupportSignificant => 1,
        }
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must carry per
/// surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleDriftExportField {
    /// The stable drift identity shared across surfaces.
    DriftId,
    /// The opaque bundle identity ref and human name.
    BundleIdentity,
    /// The high-level drift state.
    DriftState,
    /// The enumerated distinct drift kinds the banner reports.
    DriftKinds,
    /// The per-override local-override rows at field / package / task granularity.
    LocalOverrides,
    /// The rollback checkpoint created before a mutating fix.
    RollbackCheckpoint,
    /// The mirror / offline posture of the source.
    MirrorOfflinePosture,
    /// The missing-artifact refs the drift reports.
    MissingArtifacts,
}

impl M5BundleDriftExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DriftId,
        Self::BundleIdentity,
        Self::DriftState,
        Self::DriftKinds,
        Self::LocalOverrides,
        Self::RollbackCheckpoint,
        Self::MirrorOfflinePosture,
        Self::MissingArtifacts,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::DriftId,
        Self::BundleIdentity,
        Self::DriftState,
        Self::LocalOverrides,
        Self::MirrorOfflinePosture,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DriftId => "drift_id",
            Self::BundleIdentity => "bundle_identity",
            Self::DriftState => "drift_state",
            Self::DriftKinds => "drift_kinds",
            Self::LocalOverrides => "local_overrides",
            Self::RollbackCheckpoint => "rollback_checkpoint",
            Self::MirrorOfflinePosture => "mirror_offline_posture",
            Self::MissingArtifacts => "missing_artifacts",
        }
    }
}

/// One local-override row: one overridden bundle-owned asset attributed at field / package /
/// task granularity, never one opaque `customized` label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleLocalOverride {
    /// The granularity at which the override is attributed.
    pub granularity: M5DriftGranularity,
    /// Opaque ref to the overridden field / package / task; never raw content.
    pub target_ref: String,
    /// A human-readable, one-line override label.
    pub label: String,
    /// The drift kind this override represents.
    pub drift_kind: M5DriftKind,
    /// How the override differs between the bundle and local state.
    pub diff_action: DiffAction,
    /// Who owns the overridden local state.
    pub ownership: AssetOwnership,
    /// The resolution recorded for this override (keep local / adopt / rebase / compare / remove).
    pub resolution: ResolutionChoice,
    /// The significance of this override; must equal [`M5DriftSignificance::for_kind`].
    pub significance: M5DriftSignificance,
}

impl M5BundleLocalOverride {
    /// Whether the resolution is safe for this override's ownership and diff action.
    ///
    /// Mirrors the canonical [`crate::m5_bundle_review_and_rollback::ComponentDiffEntry`] safety
    /// rules so a user-protected override is never removed under the banner of cleanup and a
    /// blocked asset is never silently pulled in.
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
                    return matches!(
                        self.resolution,
                        ResolutionChoice::Compare
                            | ResolutionChoice::KeepLocal
                            | ResolutionChoice::NotApplicable
                    );
                }
                self.resolution != ResolutionChoice::NotApplicable
            }
        }
    }

    /// Whether the significance matches the drift kind — a harmless preference is never painted
    /// as support-significant and a support-significant drift is never hidden as harmless.
    pub fn significance_honest(&self) -> bool {
        self.significance == M5DriftSignificance::for_kind(self.drift_kind)
    }

    /// Whether the row preserves a user-protected override rather than silently discarding it.
    pub fn preserves_local_override(&self) -> bool {
        !self.ownership.is_user_protected()
            || self.resolution != ResolutionChoice::RemoveBundleOwned
    }

    /// Whether this override row is internally consistent and attributable.
    pub fn is_consistent(&self) -> bool {
        !self.target_ref.trim().is_empty()
            && !self.label.trim().is_empty()
            && self.resolution_safe()
            && self.significance_honest()
    }
}

/// One bundle-declared artifact absent from the current state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MissingArtifact {
    /// Which content category the missing artifact belongs to.
    pub component_kind: BundleComponentKind,
    /// Opaque ref to the missing artifact; never empty.
    pub artifact_ref: String,
    /// A human-readable, one-line label for the missing artifact.
    pub label: String,
}

impl M5MissingArtifact {
    /// Whether this missing-artifact row is internally consistent.
    pub fn is_consistent(&self) -> bool {
        !self.artifact_ref.trim().is_empty() && !self.label.trim().is_empty()
    }
}

// --- resolver input ---

/// The full input to the bundle drift resolver for one drift context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDriftInput {
    /// The stable drift identity that must survive across the banner, override list, and card.
    pub drift_id: String,
    /// Human-readable surface / context label.
    pub surface_label: String,
    /// Opaque ref to the bundle id under drift review; never raw manifest bytes.
    pub bundle_id_ref: String,
    /// Human-readable bundle name shown on the banner.
    pub bundle_name: String,
    /// The bundle class under drift review.
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
    /// Imported-vs-native confidence contributing to the drift story.
    pub imported_confidence: ImportedVsNativeConfidence,
    /// Compatible Aureline range the bundle declares (opaque range token).
    pub compatible_aureline_range: String,
    /// The provenance / freshness truth class the drift binds to.
    pub truth_mode: M5BundleTruthMode,
    /// The high-level drift state between local state and the bundle.
    pub drift_state: DriftState,
    /// The operation the rollback / remove card reviews (drift review / remove / update rebase).
    pub operation: BundleReviewOperation,
    /// The per-override rows at field / package / task granularity.
    pub local_overrides: Vec<M5BundleLocalOverride>,
    /// The bundle-declared artifacts absent from the current state.
    pub missing_artifacts: Vec<M5MissingArtifact>,
    /// The recommended choices the banner offers (rebase / keep local / compare / remove); each
    /// must be an actionable choice.
    pub recommended_choices: Vec<ResolutionChoice>,
    /// The toolchain / scaffold / settings / docs side effects a rollback / remove carries.
    pub side_effects: Vec<M5BundleSideEffectClass>,
    /// The one-step rollback checkpoint created before a mutating fix. Required for a mutating
    /// remove / update; may be absent for a read-only drift review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint: Option<RollbackCheckpoint>,
    /// The banner reads like a generic package update; must be `false` (AC1).
    pub reads_like_generic_update: bool,
    /// A support-significant drift is claimed harmless; must be `false` (AC2).
    pub claims_harmless_despite_significant: bool,
    /// A stale / missing certification is claimed as current; must be `false`.
    pub claims_current_despite_stale: bool,
    /// The card forces a bundle reset to make drift exportable; must be `false` (AC3).
    pub forces_reset_to_export: bool,
    /// An externally-observed narrowing carried through onto the drift before action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved bundle drift banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDriftBanner {
    /// The drift identity — identical to the override list and card.
    pub drift_id: String,
    /// The opaque bundle id ref.
    pub bundle_id_ref: String,
    /// The human-readable bundle name.
    pub bundle_name: String,
    /// The high-level drift state.
    pub drift_state: DriftState,
    /// The provenance / freshness truth class.
    pub truth_mode: M5BundleTruthMode,
    /// The recommended choices the banner offers (rebase / keep local / compare / remove).
    pub recommended_choices: Vec<ResolutionChoice>,
    /// The distinct drift kinds the banner reports, sorted — proves it is not a generic warning.
    pub distinct_drift_kinds: Vec<M5DriftKind>,
    /// The banner reports missing artifacts.
    pub has_missing_artifacts: bool,
    /// The highest significance across the reported drift.
    pub highest_significance: M5DriftSignificance,
    /// The banner reads like a generic package update (AC1); always `false`.
    pub reads_like_generic_update: bool,
    /// The banner discloses local-override state; always `true`.
    pub discloses_override_state: bool,
}

/// The resolved local-override list at field / package / task granularity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedLocalOverrideList {
    /// The drift identity — identical to the banner and card.
    pub drift_id: String,
    /// The per-override rows.
    pub overrides: Vec<M5BundleLocalOverride>,
    /// The granularities present across the rows, sorted — proves field / package / task detail.
    pub granularities_present: Vec<M5DriftGranularity>,
    /// The list preserves user-protected overrides (AC3); always `true`.
    pub preserves_local_overrides: bool,
    /// Every override is attributable and exportable without a reset (AC3); always `true`.
    pub attributable_and_exportable: bool,
    /// The list collapses to one opaque `customized` label; always `false`.
    pub collapses_to_opaque_customized: bool,
}

/// The resolved rollback / remove card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRollbackRemoveCard {
    /// The drift identity — identical to the banner and override list.
    pub drift_id: String,
    /// The operation the card reviews.
    pub operation: BundleReviewOperation,
    /// The side effects a rollback / remove carries.
    pub side_effects: Vec<M5BundleSideEffectClass>,
    /// The one-step rollback checkpoint created before a mutating fix, when any.
    pub rollback_checkpoint: Option<RollbackCheckpoint>,
    /// The card creates a rollback checkpoint before a mutating fix (true for mutating, false
    /// for a read-only drift review).
    pub creates_rollback_checkpoint: bool,
    /// The card discloses the rollback path; always `true`.
    pub discloses_rollback_path: bool,
    /// The card discloses removal side effects; always `true`.
    pub discloses_side_effects: bool,
    /// The card forces a bundle reset to export drift (AC3); always `false`.
    pub forces_reset: bool,
}

/// The resolved bundle drift truth shared across the banner, override list, and card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBundleDrift {
    /// The stable drift identity.
    pub drift_id: String,
    /// The resolved drift banner.
    pub banner: M5ResolvedDriftBanner,
    /// The resolved local-override list.
    pub override_list: M5ResolvedLocalOverrideList,
    /// The resolved rollback / remove card.
    pub rollback_remove_card: M5ResolvedRollbackRemoveCard,
    /// Drift is reviewable at the right level of detail, not a generic warning (AC1).
    pub reviewable_at_detail: bool,
    /// Harmless local preference is distinguished from support-significant drift (AC2).
    pub significance_distinguished: bool,
    /// Local overrides remain attributable and exportable without a reset (AC3).
    pub overrides_attributable_without_reset: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedBundleDrift {
    /// True when the drift identity is identical across the banner, override list, and card.
    pub fn identity_consistent(&self) -> bool {
        self.banner.drift_id == self.drift_id
            && self.override_list.drift_id == self.drift_id
            && self.rollback_remove_card.drift_id == self.drift_id
    }

    /// True when the drift is reviewable at the right level of detail (AC1).
    pub fn reviewable_at_detail(&self) -> bool {
        self.reviewable_at_detail
    }

    /// True when significance is distinguished (AC2).
    pub fn significance_distinguished(&self) -> bool {
        self.significance_distinguished
    }

    /// True when local overrides remain attributable without a reset (AC3).
    pub fn overrides_attributable_without_reset(&self) -> bool {
        self.overrides_attributable_without_reset
    }

    /// True when the banner reports more than one distinct drift kind — the drift is a real,
    /// enumerated set of differences rather than a single generic warning.
    pub fn has_enumerated_drift(&self) -> bool {
        self.banner.distinct_drift_kinds.len() >= 2
    }

    /// True when the override list carries both a harmless and a support-significant row.
    pub fn separates_harmless_from_significant(&self) -> bool {
        let has_harmless = self
            .override_list
            .overrides
            .iter()
            .any(|o| o.significance == M5DriftSignificance::HarmlessLocalPreference);
        let has_significant = self
            .override_list
            .overrides
            .iter()
            .any(|o| o.significance == M5DriftSignificance::SupportSignificant);
        has_harmless && has_significant
    }
}

/// Errors returned by [`resolve_bundle_drift`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BundleDriftResolutionError {
    /// The drift identity was empty.
    EmptyDriftId,
    /// The bundle id ref was empty.
    EmptyBundleIdRef,
    /// The bundle name was empty.
    EmptyBundleName,
    /// The compatible Aureline range was empty.
    EmptyCompatibleRange,
    /// The drift carried neither a local override nor a missing artifact.
    EmptyDriftSignals,
    /// A local-override row was incomplete or unsafe.
    OverrideRowIncomplete,
    /// A missing-artifact row was incomplete.
    MissingArtifactIncomplete,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// The banner read like a generic package update instead of enumerated drift (AC1).
    ReadsLikeGenericUpdate,
    /// The banner offered no actionable recommended choice.
    MissingRecommendedChoices,
    /// A recommended choice was not an actionable resolution.
    NonActionableRecommendedChoice,
    /// A support-significant drift was claimed harmless (AC2).
    SignificanceMislabeled,
    /// A user-protected override was marked for removal (AC3).
    LocalOverrideNotPreserved,
    /// A mutating remove / update offered no one-step rollback checkpoint.
    MutatingOpWithoutCheckpoint,
    /// A stale / missing certification was claimed as current instead of narrowing.
    StaleClaimShownAsCurrent,
    /// The card forced a bundle reset to make drift exportable (AC3).
    ForcesResetToExport,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5BundleDriftResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDriftId => "empty_drift_id",
            Self::EmptyBundleIdRef => "empty_bundle_id_ref",
            Self::EmptyBundleName => "empty_bundle_name",
            Self::EmptyCompatibleRange => "empty_compatible_range",
            Self::EmptyDriftSignals => "empty_drift_signals",
            Self::OverrideRowIncomplete => "override_row_incomplete",
            Self::MissingArtifactIncomplete => "missing_artifact_incomplete",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::ReadsLikeGenericUpdate => "reads_like_generic_update",
            Self::MissingRecommendedChoices => "missing_recommended_choices",
            Self::NonActionableRecommendedChoice => "non_actionable_recommended_choice",
            Self::SignificanceMislabeled => "significance_mislabeled",
            Self::LocalOverrideNotPreserved => "local_override_not_preserved",
            Self::MutatingOpWithoutCheckpoint => "mutating_op_without_checkpoint",
            Self::StaleClaimShownAsCurrent => "stale_claim_shown_as_current",
            Self::ForcesResetToExport => "forces_reset_to_export",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5BundleDriftResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "bundle-drift resolution error: {}", self.as_str())
    }
}

impl Error for M5BundleDriftResolutionError {}

/// Resolves one bundle drift context into its shared drift banner, local-override list, and
/// rollback / remove card.
///
/// The three surfaces share one drift identity, so the bundle's drift state, per-override detail,
/// missing artifacts, recommended choices, and rollback path never blur across them. The banner
/// enumerates the distinct drift kinds it reports and never reads as a generic package update;
/// every override row is attributed at field / package / task granularity and carries a
/// significance derived from its drift kind; user-protected overrides are preserved; a mutating
/// remove / update creates a one-step rollback checkpoint before it commits; a stale
/// certification never reads as current; and the card never forces a reset to make drift
/// exportable.
pub fn resolve_bundle_drift(
    input: &M5BundleDriftInput,
) -> Result<M5ResolvedBundleDrift, M5BundleDriftResolutionError> {
    if input.drift_id.trim().is_empty() {
        return Err(M5BundleDriftResolutionError::EmptyDriftId);
    }
    if input.bundle_id_ref.trim().is_empty() {
        return Err(M5BundleDriftResolutionError::EmptyBundleIdRef);
    }
    if input.bundle_name.trim().is_empty() {
        return Err(M5BundleDriftResolutionError::EmptyBundleName);
    }
    if input.compatible_aureline_range.trim().is_empty() {
        return Err(M5BundleDriftResolutionError::EmptyCompatibleRange);
    }
    if input.local_overrides.is_empty() && input.missing_artifacts.is_empty() {
        return Err(M5BundleDriftResolutionError::EmptyDriftSignals);
    }

    if input_carries_forbidden_material(input) {
        return Err(M5BundleDriftResolutionError::ForbiddenMaterial);
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5BundleDriftResolutionError::DegradedLabelGeneric);
        }
    }

    // AC1: the banner never reads like a generic package update.
    if input.reads_like_generic_update {
        return Err(M5BundleDriftResolutionError::ReadsLikeGenericUpdate);
    }

    // Every override row is complete, safely resolved, and honestly attributed; a user-protected
    // override is never silently discarded.
    for override_row in &input.local_overrides {
        if !override_row.resolution_safe() || !override_row.is_consistent() {
            return Err(M5BundleDriftResolutionError::OverrideRowIncomplete);
        }
        if !override_row.preserves_local_override() {
            return Err(M5BundleDriftResolutionError::LocalOverrideNotPreserved);
        }
    }
    for artifact in &input.missing_artifacts {
        if !artifact.is_consistent() {
            return Err(M5BundleDriftResolutionError::MissingArtifactIncomplete);
        }
    }

    // The banner must offer at least one actionable recommended choice, each drawn from the
    // shared resolution vocabulary (never a non-actionable `not_applicable`).
    if input.recommended_choices.is_empty() {
        return Err(M5BundleDriftResolutionError::MissingRecommendedChoices);
    }
    if input
        .recommended_choices
        .iter()
        .any(|choice| *choice == ResolutionChoice::NotApplicable)
    {
        return Err(M5BundleDriftResolutionError::NonActionableRecommendedChoice);
    }

    // AC2: a support-significant drift is never claimed harmless.
    let has_support_significant = input
        .local_overrides
        .iter()
        .any(|o| o.significance == M5DriftSignificance::SupportSignificant)
        || !input.missing_artifacts.is_empty();
    if input.claims_harmless_despite_significant && has_support_significant {
        return Err(M5BundleDriftResolutionError::SignificanceMislabeled);
    }

    // A mutating remove / update must create a one-step rollback checkpoint captured before the
    // mutation commits.
    if input.operation.is_mutating() {
        let has_checkpoint = input
            .rollback_checkpoint
            .as_ref()
            .is_some_and(RollbackCheckpoint::supports_one_step_rollback);
        if !has_checkpoint {
            return Err(M5BundleDriftResolutionError::MutatingOpWithoutCheckpoint);
        }
    }

    // A stale / missing certification narrows the claim rather than being shown as current.
    let freshness_is_stale = matches!(
        input.certification_freshness,
        EvidenceFreshness::Stale | EvidenceFreshness::Missing
    );
    if input.claims_current_despite_stale && freshness_is_stale {
        return Err(M5BundleDriftResolutionError::StaleClaimShownAsCurrent);
    }

    // AC3: the card never forces a reset to make drift exportable.
    if input.forces_reset_to_export {
        return Err(M5BundleDriftResolutionError::ForcesResetToExport);
    }

    // Enumerate the distinct drift kinds across overrides and missing artifacts, sorted.
    let mut drift_kinds: BTreeSet<M5DriftKind> =
        input.local_overrides.iter().map(|o| o.drift_kind).collect();
    if !input.missing_artifacts.is_empty() {
        drift_kinds.insert(M5DriftKind::MissingArtifact);
    }
    let distinct_drift_kinds: Vec<M5DriftKind> = drift_kinds.into_iter().collect();

    // Enumerate the granularities present across overrides, sorted.
    let granularities: BTreeSet<M5DriftGranularity> =
        input.local_overrides.iter().map(|o| o.granularity).collect();
    let granularities_present: Vec<M5DriftGranularity> = granularities.into_iter().collect();

    let highest_significance = input
        .local_overrides
        .iter()
        .map(|o| o.significance)
        .chain(
            (!input.missing_artifacts.is_empty())
                .then_some(M5DriftSignificance::SupportSignificant),
        )
        .max_by_key(|s| s.rank())
        .unwrap_or(M5DriftSignificance::HarmlessLocalPreference);

    let creates_rollback_checkpoint = input.operation.is_mutating();

    let banner = M5ResolvedDriftBanner {
        drift_id: input.drift_id.clone(),
        bundle_id_ref: input.bundle_id_ref.clone(),
        bundle_name: input.bundle_name.clone(),
        drift_state: input.drift_state,
        truth_mode: input.truth_mode,
        recommended_choices: input.recommended_choices.clone(),
        distinct_drift_kinds,
        has_missing_artifacts: !input.missing_artifacts.is_empty(),
        highest_significance,
        reads_like_generic_update: false,
        discloses_override_state: true,
    };

    let override_list = M5ResolvedLocalOverrideList {
        drift_id: input.drift_id.clone(),
        overrides: input.local_overrides.clone(),
        granularities_present,
        preserves_local_overrides: true,
        attributable_and_exportable: true,
        collapses_to_opaque_customized: false,
    };

    let rollback_remove_card = M5ResolvedRollbackRemoveCard {
        drift_id: input.drift_id.clone(),
        operation: input.operation,
        side_effects: input.side_effects.clone(),
        rollback_checkpoint: input.rollback_checkpoint.clone(),
        creates_rollback_checkpoint,
        discloses_rollback_path: true,
        discloses_side_effects: true,
        forces_reset: false,
    };

    Ok(M5ResolvedBundleDrift {
        drift_id: input.drift_id.clone(),
        banner,
        override_list,
        rollback_remove_card,
        reviewable_at_detail: true,
        significance_distinguished: true,
        overrides_attributable_without_reset: true,
        degraded: input.degraded.clone(),
    })
}

/// True when any label, ref, or note on the input carries obviously forbidden material.
fn input_carries_forbidden_material(input: &M5BundleDriftInput) -> bool {
    let mut values: Vec<&str> = vec![
        input.drift_id.as_str(),
        input.surface_label.as_str(),
        input.bundle_id_ref.as_str(),
        input.bundle_name.as_str(),
        input.compatible_aureline_range.as_str(),
    ];
    for override_row in &input.local_overrides {
        values.push(override_row.target_ref.as_str());
        values.push(override_row.label.as_str());
    }
    for artifact in &input.missing_artifacts {
        values.push(artifact.artifact_ref.as_str());
        values.push(artifact.label.as_str());
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

/// One worked resolution case carried in the packet so the support / export packet reconstructs
/// bundle-drift truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDriftCase {
    /// The resolver input.
    pub input: M5BundleDriftInput,
    /// The resolved bundle-drift truth. Must equal `resolve_bundle_drift(&input)`.
    pub resolved: M5ResolvedBundleDrift,
}

impl M5BundleDriftCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5BundleDriftInput) -> Self {
        let resolved = resolve_bundle_drift(&input).expect("seed bundle-drift case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_bundle_drift(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one bundle-drift surface family bound to the shared drift
/// contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDriftSurfaceRow {
    /// The bundle-drift surface family.
    pub surface_family: M5BundleDriftSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Drift operations this surface can review (must be non-empty).
    pub operations: Vec<BundleReviewOperation>,
    /// Source classes this surface can disclose (must be non-empty).
    pub source_classes: Vec<CertificationTarget>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<M5BundleTruthMode>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5BundleDriftExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5BundleComponentDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be non-empty).
    pub example_drifts: Vec<M5BundleDriftCase>,
    /// Hard invariant: this row never reads like a generic update. MUST be `false`.
    pub reads_like_generic_update: bool,
    /// Hard invariant: this row never collapses overrides to one opaque label. MUST be `false`.
    pub collapses_to_opaque_customized: bool,
    /// Hard invariant: this row never forces a reset to export drift. MUST be `false`.
    pub forces_reset_to_export: bool,
}

impl M5BundleDriftSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5BundleDriftExportField> =
            self.export_fields.iter().copied().collect();
        M5BundleDriftExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.reads_like_generic_update
            && !self.collapses_to_opaque_customized
            && !self.forces_reset_to_export
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDriftVocabularySet {
    /// Bundle-drift surface-family tokens.
    pub surface_families: Vec<String>,
    /// Drift-kind tokens.
    pub drift_kinds: Vec<String>,
    /// Drift-granularity tokens.
    pub granularities: Vec<String>,
    /// Drift-significance tokens.
    pub significances: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Side-effect-class tokens (reused from the bundle-review primitive).
    pub side_effect_classes: Vec<String>,
    /// Drift-state tokens (reused from the review / rollback contract).
    pub drift_states: Vec<String>,
    /// Review-operation tokens (reused from the review / rollback contract).
    pub review_operations: Vec<String>,
    /// Diff-action tokens (reused from the review / rollback contract).
    pub diff_actions: Vec<String>,
    /// Asset-ownership tokens (reused from the review / rollback contract).
    pub asset_ownerships: Vec<String>,
    /// Resolution-choice tokens (reused from the review / rollback contract).
    pub resolution_choices: Vec<String>,
    /// Component-kind tokens (reused from the bundle-manifest contract).
    pub component_kinds: Vec<String>,
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

impl M5BundleDriftVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(
                &M5BundleDriftSurfaceFamily::ALL,
                M5BundleDriftSurfaceFamily::as_str,
            ),
            drift_kinds: tokens(&M5DriftKind::ALL, M5DriftKind::as_str),
            granularities: tokens(&M5DriftGranularity::ALL, M5DriftGranularity::as_str),
            significances: tokens(&M5DriftSignificance::ALL, M5DriftSignificance::as_str),
            export_fields: tokens(
                &M5BundleDriftExportField::ALL,
                M5BundleDriftExportField::as_str,
            ),
            side_effect_classes: tokens(
                &M5BundleSideEffectClass::ALL,
                M5BundleSideEffectClass::as_str,
            ),
            drift_states: tokens(&DriftState::ALL, DriftState::as_str),
            review_operations: tokens(&BundleReviewOperation::ALL, BundleReviewOperation::as_str),
            diff_actions: tokens(&DiffAction::ALL, DiffAction::as_str),
            asset_ownerships: tokens(&AssetOwnership::ALL, AssetOwnership::as_str),
            resolution_choices: tokens(&ResolutionChoice::ALL, ResolutionChoice::as_str),
            component_kinds: tokens(&BundleComponentKind::ALL, BundleComponentKind::as_str),
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
pub struct M5BundleDriftGovernanceReview {
    /// One primitive carries banner, override-list, and rollback-card truth on every surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Drift identity is preserved across the banner, override list, and card.
    pub drift_identity_preserved_across_surfaces: bool,
    /// Drift is reviewable at field / package / task detail, not a generic warning.
    pub drift_reviewable_at_detail: bool,
    /// Harmless local preference is distinguished from support-significant drift.
    pub significance_distinguished: bool,
    /// Local overrides remain attributable and exportable without a reset.
    pub overrides_attributable_without_reset: bool,
    /// A mutating remove / update always creates a one-step rollback checkpoint.
    pub rollback_checkpoint_created_before_mutation: bool,
    /// The support / export packet reconstructs bundle-drift truth.
    pub support_export_reconstructs_drift: bool,
    /// Later M5 rows cannot invent parallel drift / override vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDriftConsumerProjection {
    /// Workspace / bundle / extension / migration / diagnostics / support surfaces all consume
    /// the shared primitive.
    pub drift_surfaces_consume_shared_primitive: bool,
    /// The drift resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The override list reads a single canonical drift / override source.
    pub override_list_reads_single_source: bool,
    /// Support / export reads a single canonical drift source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the bundle-drift primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDriftReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting drift audit.
    pub drift_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BundleDriftOverridePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BundleDriftOverridePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5BundleDriftSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BundleDriftVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BundleDriftGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BundleDriftConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5BundleDriftReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 bundle drift / override primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleDriftOverridePacket {
    /// Record kind; must equal [`M5_BUNDLE_DRIFT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BUNDLE_DRIFT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5BundleDriftSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BundleDriftVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BundleDriftGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BundleDriftConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5BundleDriftReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BundleDriftOverridePacket {
    /// Builds an M5 bundle drift / override primitive packet from stable-lane input.
    pub fn new(input: M5BundleDriftOverridePacketInput) -> Self {
        Self {
            record_kind: M5_BUNDLE_DRIFT_RECORD_KIND.to_owned(),
            schema_version: M5_BUNDLE_DRIFT_SCHEMA_VERSION,
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

    /// Validates the M5 bundle-drift primitive invariants.
    pub fn validate(&self) -> Vec<M5BundleDriftViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BUNDLE_DRIFT_RECORD_KIND {
            violations.push(M5BundleDriftViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BUNDLE_DRIFT_SCHEMA_VERSION {
            violations.push(M5BundleDriftViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BundleDriftViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 bundle-drift primitive packet serializes"),
        ) {
            violations.push(M5BundleDriftViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 bundle-drift primitive packet serializes")
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
                row.example_drifts.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Bundle Drift / Override Primitive: Drift Banner, Local-Override Rows, and Rollback / Remove Card\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Bundle-drift surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5BundleDriftSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Drift kinds: {}\n",
            self.vocabulary_set.drift_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Granularities: {}\n",
            self.vocabulary_set.granularities.join(", ")
        ));
        out.push_str(&format!(
            "- Significances: {}\n",
            self.vocabulary_set.significances.join(", ")
        ));
        out.push_str("\n## Bundle-drift surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Worked cases: {}\n", row.example_drifts.len()));
            for case in &row.example_drifts {
                out.push_str(&format!(
                    "    - `{}` → drift `{}` (op `{}`), {} override row(s), {} kind(s), significance `{}`\n",
                    case.resolved.drift_id,
                    case.resolved.banner.drift_state.as_str(),
                    case.resolved.rollback_remove_card.operation.as_str(),
                    case.resolved.override_list.overrides.len(),
                    case.resolved.banner.distinct_drift_kinds.len(),
                    case.resolved.banner.highest_significance.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 bundle-drift export.
#[derive(Debug)]
pub enum M5BundleDriftArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BundleDriftViolation>),
}

impl fmt::Display for M5BundleDriftArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 bundle-drift primitive export parse failed: {error}"
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
                    "m5 bundle-drift primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BundleDriftArtifactError {}

/// Validation failures emitted by [`M5BundleDriftOverridePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BundleDriftViolation {
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
    /// A required bundle-drift surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no drift operations.
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
    /// A surface row declares no worked drift cases.
    ExampleDriftsMissing,
    /// A worked drift case does not match a fresh resolve of its input.
    ExampleDriftDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves drift reviewable at detail with enumerated kinds (AC1).
    DetailReviewabilityUnproven,
    /// No worked case proves harmless-versus-significant separation (AC2).
    SignificanceSeparationUnproven,
    /// No worked case proves overrides attributable without a reset across ops (AC3).
    AttributabilityUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BundleDriftViolation {
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
            Self::ExampleDriftsMissing => "example_drifts_missing",
            Self::ExampleDriftDrift => "example_drift_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::DetailReviewabilityUnproven => "detail_reviewability_unproven",
            Self::SignificanceSeparationUnproven => "significance_separation_unproven",
            Self::AttributabilityUnproven => "attributability_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 bundle-drift export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_bundle_drift_override_export(
) -> Result<M5BundleDriftOverridePacket, M5BundleDriftArtifactError> {
    let packet: M5BundleDriftOverridePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-bundle-drift-override-primitive-proof/support_export.json"
    )))
    .map_err(M5BundleDriftArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BundleDriftArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BundleDriftOverridePacket,
    violations: &mut Vec<M5BundleDriftViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BUNDLE_DRIFT_SCHEMA_REF,
        M5_BUNDLE_DRIFT_DOC_REF,
        M5_BUNDLE_DRIFT_COMPONENT_MATRIX_REF,
        M5_BUNDLE_DRIFT_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BundleDriftViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BundleDriftOverridePacket,
    violations: &mut Vec<M5BundleDriftViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BundleDriftViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5BundleDriftOverridePacket,
    violations: &mut Vec<M5BundleDriftViolation>,
) {
    let present: BTreeSet<M5BundleDriftSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5BundleDriftSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5BundleDriftViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5BundleDriftViolation::SurfaceRowIncomplete);
        }
        if row.operations.is_empty() {
            violations.push(M5BundleDriftViolation::OperationMissing);
        }
        if row.source_classes.is_empty() {
            violations.push(M5BundleDriftViolation::SourceClassMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5BundleDriftViolation::TruthModeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5BundleDriftViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5BundleDriftViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BundleDriftViolation::ConsumerSurfacesMissing);
        }
        if row.example_drifts.is_empty() {
            violations.push(M5BundleDriftViolation::ExampleDriftsMissing);
        }
        if row.example_drifts.iter().any(|case| !case.is_self_consistent()) {
            violations.push(M5BundleDriftViolation::ExampleDriftDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5BundleDriftViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case across the
/// matrix: drift is reviewable at detail with enumerated kinds and never a generic warning
/// (AC1), the matrix separates harmless local preference from support-significant drift (AC2),
/// and overrides stay attributable and exportable without a reset across a read-only and a
/// mutating op (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5BundleDriftOverridePacket,
    violations: &mut Vec<M5BundleDriftViolation>,
) {
    let cases: Vec<&M5ResolvedBundleDrift> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_drifts.iter().map(|case| &case.resolved))
        .collect();

    // AC1: every case is reviewable at detail and never reads like a generic update; at least
    // one enumerates more than one distinct drift kind.
    let detail_proven = cases.iter().all(|resolved| {
        resolved.reviewable_at_detail() && !resolved.banner.reads_like_generic_update
    }) && cases.iter().any(|resolved| resolved.has_enumerated_drift());
    if !detail_proven {
        violations.push(M5BundleDriftViolation::DetailReviewabilityUnproven);
    }

    // AC2: every case distinguishes significance, the matrix spans a harmless preference and a
    // support-significant drift, and at least one case carries both on one drift.
    let has_harmless = cases.iter().any(|resolved| {
        resolved
            .override_list
            .overrides
            .iter()
            .any(|o| o.significance == M5DriftSignificance::HarmlessLocalPreference)
    });
    let has_significant = cases.iter().any(|resolved| {
        resolved.banner.highest_significance == M5DriftSignificance::SupportSignificant
    });
    let significance_proven = cases
        .iter()
        .all(|resolved| resolved.significance_distinguished())
        && has_harmless
        && has_significant
        && cases
            .iter()
            .any(|resolved| resolved.separates_harmless_from_significant());
    if !significance_proven {
        violations.push(M5BundleDriftViolation::SignificanceSeparationUnproven);
    }

    // AC3: every case keeps overrides attributable without a reset and shares one drift identity,
    // and the matrix spans a read-only drift review and a mutating remove / update fix.
    let attributability_proven = cases.iter().all(|resolved| {
        resolved.identity_consistent()
            && resolved.overrides_attributable_without_reset()
            && !resolved.override_list.collapses_to_opaque_customized
            && !resolved.rollback_remove_card.forces_reset
    }) && cases
        .iter()
        .any(|resolved| !resolved.rollback_remove_card.operation.is_mutating())
        && cases
            .iter()
            .any(|resolved| resolved.rollback_remove_card.operation.is_mutating());
    if !attributability_proven {
        violations.push(M5BundleDriftViolation::AttributabilityUnproven);
    }
}

fn validate_governance_review(
    packet: &M5BundleDriftOverridePacket,
    violations: &mut Vec<M5BundleDriftViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.drift_identity_preserved_across_surfaces,
        review.drift_reviewable_at_detail,
        review.significance_distinguished,
        review.overrides_attributable_without_reset,
        review.rollback_checkpoint_created_before_mutation,
        review.support_export_reconstructs_drift,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5BundleDriftViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BundleDriftOverridePacket,
    violations: &mut Vec<M5BundleDriftViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.drift_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.override_list_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5BundleDriftViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5BundleDriftOverridePacket,
    violations: &mut Vec<M5BundleDriftViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.drift_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BundleDriftViolation::ReleasePostureIncomplete);
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
