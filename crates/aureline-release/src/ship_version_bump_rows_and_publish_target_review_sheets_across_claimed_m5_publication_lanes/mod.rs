//! One reusable M5 version-bump-row / publish-target-review-sheet primitive:
//! previous-versus-next version, delta kind, public-surface impact, review-evidence
//! actions, publish-target class, visibility, mutability, auth source, dry-run
//! availability, and rollout ring, projected the same way across every claimed M5
//! publication lane.
//!
//! Aureline's frozen release-center component matrix
//! ([`crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix`])
//! names the version-bump row and the publish-target row as two governed component
//! families and freezes their controlled vocabulary — the version-bump classes, the
//! compatibility impacts, the publish-target visibilities, the target mutabilities,
//! the target auth sources, the dry-run availabilities, the rollout rings, the
//! publication surface families, the deployment lines, the accessibility routes, the
//! qualification classes, and the downgrade triggers. This module *implements* that
//! version-bump-row and publish-target contract as one reusable review-sheet
//! primitive so a user can tell — from the row and its review sheet alone — exactly
//! what will be changed, where it will be published, what can still mutate, and which
//! credentials or dry-run paths apply, *before* pushing a target or widening a
//! channel, instead of that truth drifting by pipeline page or admin log.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_publication_review`] — that takes one publication's
//!    proposal label, prior and next version, version-bump class, compatibility
//!    impact, changed artifact set, publish-target class, visibility, mutability,
//!    auth source, auth-disclosure state, dry-run availability, rollout ring, and
//!    public-surface impact-analysis state, and produces one
//!    [`M5ResolvedPublicationReview`] carrying the derived public-surface impact, the
//!    destination reversibility, the derived publication readiness (publishable
//!    versus publishable-with-review versus narrowed versus blocked), and — whenever
//!    the publication is blocked or narrowed — a self-contained
//!    [`M5PublicationBlockedBanner`] that names the exact reason, the blocked
//!    destination, and the next action rather than a generic `cannot publish`. The
//!    resolver never collapses the public-surface impact into a single semver
//!    string, never masks the target auth source or destination class, and never
//!    lets a mutable target read as an immutable publication step.
//! 2. A parity matrix — [`M5PublicationReviewPrimitivePacket`] — that binds one row
//!    per claimed M5 publication consumer (the release-center publish sheet, the
//!    update-center publish row, the CLI publish inspect, the admin publish report,
//!    and the support / evaluation export) to the shared row anatomy, the same
//!    version-bump classes, compatibility impacts, public-surface impacts, target
//!    classes, visibilities, mutabilities, auth sources, dry-run availabilities,
//!    rollout rings, readiness postures, block reasons, next actions, the same
//!    export fields, and the same non-visual accessibility routes, so the
//!    version-bump / publish-target vocabulary stays identical across the release
//!    center, the CLI, admin/reporting, and support/evaluation.
//!
//! The version-bump class ([`M5VersionBumpClass`]), compatibility impact
//! ([`M5CompatibilityImpact`]), publish-target visibility
//! ([`M5PublishTargetVisibility`]), target mutability ([`M5TargetMutability`]),
//! target auth source ([`M5TargetAuthSource`]), dry-run availability
//! ([`M5DryRunAvailability`]), rollout ring ([`M5RolloutRing`]), publication surface
//! family ([`M5PublicationSurfaceFamily`]), deployment line ([`M5DeploymentLine`]),
//! release-center consumer surface ([`M5ReleaseCenterConsumerSurface`]),
//! accessibility route ([`M5ReleaseCenterAccessibilityRoute`]), qualification class
//! ([`M5ReleaseCenterQualificationClass`]), and downgrade trigger
//! ([`M5ReleaseCenterDowngradeTrigger`]) are reused verbatim from the frozen
//! release-center component matrix. This module mints new vocabulary only for what
//! that matrix left implicit about the version-bump row and publish-target review
//! sheet themselves: its publication consumer families, its row anatomy parts, its
//! publish-target classes, its public-surface impact classes, its auth-disclosure
//! states, its surface-impact-analysis states, its destination reversibilities, its
//! readiness postures, its block reasons, its next actions, and its export fields.
//! No M5 publication surface invents a second version-bump or publish-target grammar.
//!
//! Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user
//! text bodies stay outside the support boundary; every proposal label, version,
//! artifact id, and target descriptor is carried only as an opaque, export-safe
//! representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-publish-target-review-sheet.schema.json`](../../../../schemas/ui/m5-publish-target-review-sheet.schema.json)
//! and the contract doc is
//! [`docs/release/m5_version_bump_and_publish_target_primitive_contract.md`](../../../../docs/release/m5_version_bump_and_publish_target_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-publish-target-review-sheet-primitive/`](../../../../fixtures/ui/m5-publish-target-review-sheet-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_publication_review_primitive_cli_publish_inspect_preview_narrowed,
    seeded_m5_publication_review_primitive_packet,
    seeded_m5_publication_review_primitive_update_center_publish_row_beta_narrowed,
    M5_PUBLICATION_REVIEW_PRIMITIVE_PACKET_ID,
};

// The version-bump class, compatibility impact, publish-target visibility, target
// mutability, target auth source, dry-run availability, rollout ring, publication
// surface family, deployment line, release-center consumer surface, accessibility
// routes, qualification classes, and downgrade triggers are frozen once, in the
// release-center component matrix. This primitive reuses them verbatim so it never
// invents a parallel version-bump or publish-target vocabulary.
pub use crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix::{
    M5CompatibilityImpact, M5DeploymentLine, M5DryRunAvailability, M5PublicationSurfaceFamily,
    M5PublishTargetVisibility, M5ReleaseCenterAccessibilityRoute, M5ReleaseCenterConsumerSurface,
    M5ReleaseCenterDowngradeTrigger, M5ReleaseCenterQualificationClass, M5RolloutRing,
    M5TargetAuthSource, M5TargetMutability, M5VersionBumpClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5PublicationReviewPrimitivePacket`].
pub const M5_PUBLICATION_REVIEW_PRIMITIVE_RECORD_KIND: &str =
    "ship_m5_version_bump_rows_and_publish_target_review_sheets_across_claimed_m5_publication_lanes";

/// Schema version for M5 publication-review-primitive records.
pub const M5_PUBLICATION_REVIEW_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the publish-target-review-sheet boundary schema.
pub const M5_PUBLICATION_REVIEW_SCHEMA_REF: &str =
    "schemas/ui/m5-publish-target-review-sheet.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PUBLICATION_REVIEW_DOC_REF: &str =
    "docs/release/m5_version_bump_and_publish_target_primitive_contract.md";

/// Repo-relative path of the frozen release-center component matrix this primitive
/// narrows from.
pub const M5_PUBLICATION_REVIEW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-release-center-components.schema.json";

/// Repo-relative path of the release-center object-model contract this primitive
/// binds against.
pub const M5_PUBLICATION_REVIEW_OBJECT_MODEL_REF: &str =
    "docs/release/release_center_object_model_contract.md";

/// Repo-relative path of the artifact-verification contract this primitive projects
/// publish-target and provenance truth from.
pub const M5_PUBLICATION_REVIEW_VERIFICATION_CONTRACT_REF: &str =
    "docs/release/artifact_verification_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_PUBLICATION_REVIEW_FIXTURE_DIR: &str =
    "fixtures/ui/m5-publish-target-review-sheet-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PUBLICATION_REVIEW_ARTIFACT_REF: &str =
    "artifacts/release/m5-publish-target-review-sheet-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PUBLICATION_REVIEW_CSV_REF: &str =
    "artifacts/release/m5-publish-target-review-sheet-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_PUBLICATION_REVIEW_REPORT_REF: &str =
    "artifacts/components/m5-version-bump-and-publish-target-primitive.md";

/// One claimed M5 publication consumer that renders the shared version-bump row and
/// its publish-target review sheet. These are the consumers the acceptance criteria
/// name — the release center, the CLI, admin/reporting, and support/evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationReviewConsumerSurface {
    /// The release-center / shiproom publish review sheet.
    ReleaseCenterPublishSheet,
    /// The update-center publish row.
    UpdateCenterPublishRow,
    /// The CLI publish-inspect / headless surface.
    CliPublishInspect,
    /// The admin publish report.
    AdminPublishReport,
    /// The support / evaluation export.
    SupportEvaluationExport,
}

impl M5PublicationReviewConsumerSurface {
    /// Every claimed publication consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReleaseCenterPublishSheet,
        Self::UpdateCenterPublishRow,
        Self::CliPublishInspect,
        Self::AdminPublishReport,
        Self::SupportEvaluationExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenterPublishSheet => "release_center_publish_sheet",
            Self::UpdateCenterPublishRow => "update_center_publish_row",
            Self::CliPublishInspect => "cli_publish_inspect",
            Self::AdminPublishReport => "admin_publish_report",
            Self::SupportEvaluationExport => "support_evaluation_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseCenterPublishSheet => "Release-Center Publish Sheet",
            Self::UpdateCenterPublishRow => "Update-Center Publish Row",
            Self::CliPublishInspect => "CLI Publish Inspect",
            Self::AdminPublishReport => "Admin Publish Report",
            Self::SupportEvaluationExport => "Support / Evaluation Export",
        }
    }
}

/// One anatomy part the shared version-bump row / publish-target review sheet
/// surfaces. The parts in [`M5PublicationReviewAnatomyPart::MANDATORY`] are required
/// on every sheet so a user can review scope and destination risk before mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationReviewAnatomyPart {
    /// The version-bump identity: prior version and next version.
    VersionBumpIdentity,
    /// The version delta kind (bump class).
    VersionDeltaKind,
    /// The public-surface impact note.
    PublicSurfaceImpactNote,
    /// The review-evidence actions.
    ReviewEvidenceActions,
    /// The publish-target class.
    PublishTargetClass,
    /// The target-visibility badge.
    TargetVisibilityBadge,
    /// The target-mutability badge.
    TargetMutabilityBadge,
    /// The auth-source disclosure.
    AuthSourceDisclosure,
    /// The dry-run availability cue.
    DryRunAvailabilityCue,
    /// The rollout-ring badge.
    RolloutRingBadge,
    /// The derived destination-risk verdict.
    DestinationRiskVerdict,
    /// The publication-blocked banner (shown when blocked or narrowed).
    PublicationBlockedBanner,
}

impl M5PublicationReviewAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::VersionBumpIdentity,
        Self::VersionDeltaKind,
        Self::PublicSurfaceImpactNote,
        Self::ReviewEvidenceActions,
        Self::PublishTargetClass,
        Self::TargetVisibilityBadge,
        Self::TargetMutabilityBadge,
        Self::AuthSourceDisclosure,
        Self::DryRunAvailabilityCue,
        Self::RolloutRingBadge,
        Self::DestinationRiskVerdict,
        Self::PublicationBlockedBanner,
    ];

    /// The anatomy parts every publish sheet must render before mutation.
    pub const MANDATORY: [Self; 4] = [
        Self::VersionBumpIdentity,
        Self::PublishTargetClass,
        Self::AuthSourceDisclosure,
        Self::DestinationRiskVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VersionBumpIdentity => "version_bump_identity",
            Self::VersionDeltaKind => "version_delta_kind",
            Self::PublicSurfaceImpactNote => "public_surface_impact_note",
            Self::ReviewEvidenceActions => "review_evidence_actions",
            Self::PublishTargetClass => "publish_target_class",
            Self::TargetVisibilityBadge => "target_visibility_badge",
            Self::TargetMutabilityBadge => "target_mutability_badge",
            Self::AuthSourceDisclosure => "auth_source_disclosure",
            Self::DryRunAvailabilityCue => "dry_run_availability_cue",
            Self::RolloutRingBadge => "rollout_ring_badge",
            Self::DestinationRiskVerdict => "destination_risk_verdict",
            Self::PublicationBlockedBanner => "publication_blocked_banner",
        }
    }
}

/// Controlled publish-target class — what kind of destination a publish target is,
/// so a publish-target row never leaves its destination class implicit. This is the
/// destination-class taxonomy the frozen matrix left implicit about the target row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublishTargetClass {
    /// A package / artifact registry.
    RegistryTarget,
    /// A mirror destination.
    MirrorTarget,
    /// A mutable channel pointer / tag.
    ChannelPointerTarget,
    /// A managed control-plane destination.
    ManagedControlPlaneTarget,
    /// A local artifact store.
    LocalArtifactStoreTarget,
}

impl M5PublishTargetClass {
    /// Every publish-target class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RegistryTarget,
        Self::MirrorTarget,
        Self::ChannelPointerTarget,
        Self::ManagedControlPlaneTarget,
        Self::LocalArtifactStoreTarget,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegistryTarget => "registry_target",
            Self::MirrorTarget => "mirror_target",
            Self::ChannelPointerTarget => "channel_pointer_target",
            Self::ManagedControlPlaneTarget => "managed_control_plane_target",
            Self::LocalArtifactStoreTarget => "local_artifact_store_target",
        }
    }
}

/// Controlled public-surface impact — the derived consequence a version bump has on
/// the public surface, so a version-bump row discloses public-surface impact instead
/// of collapsing everything into a single semver string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicSurfaceImpact {
    /// No public-surface change.
    NoPublicSurfaceChange,
    /// An additive, backward-compatible public-surface change.
    AdditivePublicSurface,
    /// A breaking public-surface change.
    BreakingPublicSurface,
    /// A runtime-behavior shift with no interface change.
    RuntimeBehaviorShift,
    /// A public-surface change that requires a schema migration.
    MigrationRequiredPublicSurface,
}

impl M5PublicSurfaceImpact {
    /// Every public-surface impact, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoPublicSurfaceChange,
        Self::AdditivePublicSurface,
        Self::BreakingPublicSurface,
        Self::RuntimeBehaviorShift,
        Self::MigrationRequiredPublicSurface,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPublicSurfaceChange => "no_public_surface_change",
            Self::AdditivePublicSurface => "additive_public_surface",
            Self::BreakingPublicSurface => "breaking_public_surface",
            Self::RuntimeBehaviorShift => "runtime_behavior_shift",
            Self::MigrationRequiredPublicSurface => "migration_required_public_surface",
        }
    }
}

/// Controlled auth-disclosure state — whether the identity authorized to publish is
/// disclosed and scoped before mutation, so a publish never silently inherits
/// ambient credentials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthDisclosureState {
    /// The auth source is disclosed and scoped to this target.
    AuthScopedDisclosed,
    /// The auth source is disclosed but broadly scoped.
    AuthBroadDisclosed,
    /// The publish would inherit ambient credentials (never allowed).
    AmbientCredentialInherited,
    /// The auth source is disclosed and held under a disclosed waiver.
    AuthDisclosedUnderWaiver,
    /// The auth-disclosure review is pending sign-off.
    AuthDisclosurePendingReview,
    /// The auth-disclosure state is unknown / not yet evaluated.
    AuthDisclosureUnknown,
}

impl M5AuthDisclosureState {
    /// Every auth-disclosure state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AuthScopedDisclosed,
        Self::AuthBroadDisclosed,
        Self::AmbientCredentialInherited,
        Self::AuthDisclosedUnderWaiver,
        Self::AuthDisclosurePendingReview,
        Self::AuthDisclosureUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthScopedDisclosed => "auth_scoped_disclosed",
            Self::AuthBroadDisclosed => "auth_broad_disclosed",
            Self::AmbientCredentialInherited => "ambient_credential_inherited",
            Self::AuthDisclosedUnderWaiver => "auth_disclosed_under_waiver",
            Self::AuthDisclosurePendingReview => "auth_disclosure_pending_review",
            Self::AuthDisclosureUnknown => "auth_disclosure_unknown",
        }
    }
}

/// Controlled public-surface impact-analysis state, so a publish sheet never shows a
/// stale or missing surface-impact analysis as clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceImpactAnalysis {
    /// The surface-impact analysis is fresh within its window.
    SurfaceImpactFresh,
    /// The surface-impact analysis is aging but still within tolerance.
    SurfaceImpactAging,
    /// The surface-impact analysis is stale relative to the changed artifacts.
    SurfaceImpactStale,
    /// The required surface-impact analysis is missing.
    SurfaceImpactMissing,
    /// The surface-impact analysis is unknown / not yet run.
    SurfaceImpactUnknown,
}

impl M5SurfaceImpactAnalysis {
    /// Every surface-impact-analysis state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SurfaceImpactFresh,
        Self::SurfaceImpactAging,
        Self::SurfaceImpactStale,
        Self::SurfaceImpactMissing,
        Self::SurfaceImpactUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceImpactFresh => "surface_impact_fresh",
            Self::SurfaceImpactAging => "surface_impact_aging",
            Self::SurfaceImpactStale => "surface_impact_stale",
            Self::SurfaceImpactMissing => "surface_impact_missing",
            Self::SurfaceImpactUnknown => "surface_impact_unknown",
        }
    }
}

/// The derived destination reversibility of a publish target, so a mutable target
/// with no proven dry-run is never confused with an immutable publication step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DestinationReversibility {
    /// A dry-run preview proves the change before it mutates the target.
    DryRunProven,
    /// The target is immutable by design; the publish cannot be undone in place.
    ImmutableByDesign,
    /// The target is mutable but reversibility is unproven (no dry-run available).
    ReversibilityUnproven,
}

impl M5DestinationReversibility {
    /// Every destination reversibility, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::DryRunProven,
        Self::ImmutableByDesign,
        Self::ReversibilityUnproven,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DryRunProven => "dry_run_proven",
            Self::ImmutableByDesign => "immutable_by_design",
            Self::ReversibilityUnproven => "reversibility_unproven",
        }
    }
}

/// The derived headline publication readiness of a review — the resolver's verdict
/// about whether the publication is publishable, publishable with disclosed review,
/// narrowed, or blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationReadiness {
    /// Publishable: auth disclosed and scoped, fresh surface analysis, reversible.
    Publishable,
    /// Publishable with disclosed review (broad auth scope or aging analysis).
    PublishableWithReview,
    /// Publishable, but a dry-run is required first under a disclosed waiver.
    PublishableDryRunFirst,
    /// Narrowed: the public-surface review is pending sign-off.
    NarrowedSurfaceReviewPending,
    /// Narrowed: destination reversibility is unproven.
    NarrowedReversibilityUnproven,
    /// Blocked: the publish would inherit ambient credentials.
    BlockedAmbientCredential,
    /// Blocked: the public-surface impact analysis is stale.
    BlockedSurfaceImpactStale,
    /// Blocked: the public-surface impact analysis is missing.
    BlockedSurfaceImpactMissing,
    /// Blocked: the publication review state is unknown / not yet evaluated.
    BlockedUnknownState,
}

impl M5PublicationReadiness {
    /// Every readiness posture, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Publishable,
        Self::PublishableWithReview,
        Self::PublishableDryRunFirst,
        Self::NarrowedSurfaceReviewPending,
        Self::NarrowedReversibilityUnproven,
        Self::BlockedAmbientCredential,
        Self::BlockedSurfaceImpactStale,
        Self::BlockedSurfaceImpactMissing,
        Self::BlockedUnknownState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Publishable => "publishable",
            Self::PublishableWithReview => "publishable_with_review",
            Self::PublishableDryRunFirst => "publishable_dry_run_first",
            Self::NarrowedSurfaceReviewPending => "narrowed_surface_review_pending",
            Self::NarrowedReversibilityUnproven => "narrowed_reversibility_unproven",
            Self::BlockedAmbientCredential => "blocked_ambient_credential",
            Self::BlockedSurfaceImpactStale => "blocked_surface_impact_stale",
            Self::BlockedSurfaceImpactMissing => "blocked_surface_impact_missing",
            Self::BlockedUnknownState => "blocked_unknown_state",
        }
    }

    /// True when the publication may proceed (possibly with disclosed review or a
    /// required dry-run).
    pub const fn is_publishable(self) -> bool {
        matches!(
            self,
            Self::Publishable | Self::PublishableWithReview | Self::PublishableDryRunFirst
        )
    }

    /// True when the publication is hard-blocked.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::BlockedAmbientCredential
                | Self::BlockedSurfaceImpactStale
                | Self::BlockedSurfaceImpactMissing
                | Self::BlockedUnknownState
        )
    }

    /// True when the publication is narrowed below a clean publishable claim.
    pub const fn is_narrowed(self) -> bool {
        matches!(
            self,
            Self::NarrowedSurfaceReviewPending | Self::NarrowedReversibilityUnproven
        )
    }

    /// The specific block reason for a blocked or narrowed posture, if any. Returns
    /// `None` for a publishable posture.
    pub const fn block_reason(self) -> Option<M5PublicationBlockReason> {
        Some(match self {
            Self::BlockedAmbientCredential => {
                M5PublicationBlockReason::AmbientCredentialInheritance
            }
            Self::BlockedSurfaceImpactStale => M5PublicationBlockReason::SurfaceImpactStale,
            Self::BlockedSurfaceImpactMissing => M5PublicationBlockReason::SurfaceImpactMissing,
            Self::BlockedUnknownState => M5PublicationBlockReason::ReviewStateUnknown,
            Self::NarrowedSurfaceReviewPending => M5PublicationBlockReason::SurfaceReviewPending,
            Self::NarrowedReversibilityUnproven => {
                M5PublicationBlockReason::DestinationReversibilityUnproven
            }
            Self::Publishable | Self::PublishableWithReview | Self::PublishableDryRunFirst => {
                return None
            }
        })
    }
}

/// The exact reason a publication is blocked or narrowed, so a publication-blocked
/// banner never reads like a generic `cannot publish`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationBlockReason {
    /// The publish would inherit ambient credentials.
    AmbientCredentialInheritance,
    /// The public-surface impact analysis is stale.
    SurfaceImpactStale,
    /// The public-surface impact analysis is missing.
    SurfaceImpactMissing,
    /// The publication review state is unknown / not yet evaluated.
    ReviewStateUnknown,
    /// The public-surface review is pending sign-off.
    SurfaceReviewPending,
    /// The destination is mutable with no proven dry-run.
    DestinationReversibilityUnproven,
}

impl M5PublicationBlockReason {
    /// Every block reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AmbientCredentialInheritance,
        Self::SurfaceImpactStale,
        Self::SurfaceImpactMissing,
        Self::ReviewStateUnknown,
        Self::SurfaceReviewPending,
        Self::DestinationReversibilityUnproven,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AmbientCredentialInheritance => "ambient_credential_inheritance",
            Self::SurfaceImpactStale => "surface_impact_stale",
            Self::SurfaceImpactMissing => "surface_impact_missing",
            Self::ReviewStateUnknown => "review_state_unknown",
            Self::SurfaceReviewPending => "surface_review_pending",
            Self::DestinationReversibilityUnproven => "destination_reversibility_unproven",
        }
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::AmbientCredentialInheritance => "the publish would inherit ambient credentials",
            Self::SurfaceImpactStale => "the public-surface impact analysis is stale",
            Self::SurfaceImpactMissing => "the public-surface impact analysis is missing",
            Self::ReviewStateUnknown => "the publication review state is not yet evaluated",
            Self::SurfaceReviewPending => "the public-surface review is pending sign-off",
            Self::DestinationReversibilityUnproven => {
                "the destination is mutable with no proven dry-run"
            }
        }
    }

    /// The next action a reviewer should take to clear this reason.
    pub const fn next_action(self) -> M5PublicationNextAction {
        match self {
            Self::AmbientCredentialInheritance => M5PublicationNextAction::DiscloseAuthSource,
            Self::SurfaceImpactStale => M5PublicationNextAction::RefreshSurfaceImpact,
            Self::SurfaceImpactMissing => M5PublicationNextAction::ProvideSurfaceImpact,
            Self::ReviewStateUnknown => M5PublicationNextAction::RunPublicationReview,
            Self::SurfaceReviewPending => M5PublicationNextAction::CompleteSurfaceReview,
            Self::DestinationReversibilityUnproven => M5PublicationNextAction::EnableDryRunPreview,
        }
    }
}

/// The next action named on a publication-blocked banner, so a blocked state is
/// actionable from the banner itself rather than from a secondary pipeline page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationNextAction {
    /// Disclose the scoped auth source before mutation.
    DiscloseAuthSource,
    /// Refresh the stale surface-impact analysis.
    RefreshSurfaceImpact,
    /// Provide the missing surface-impact analysis.
    ProvideSurfaceImpact,
    /// Run the publication review.
    RunPublicationReview,
    /// Complete the pending public-surface review.
    CompleteSurfaceReview,
    /// Enable a dry-run preview before mutating the target.
    EnableDryRunPreview,
}

impl M5PublicationNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DiscloseAuthSource,
        Self::RefreshSurfaceImpact,
        Self::ProvideSurfaceImpact,
        Self::RunPublicationReview,
        Self::CompleteSurfaceReview,
        Self::EnableDryRunPreview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiscloseAuthSource => "disclose_auth_source",
            Self::RefreshSurfaceImpact => "refresh_surface_impact",
            Self::ProvideSurfaceImpact => "provide_surface_impact",
            Self::RunPublicationReview => "run_publication_review",
            Self::CompleteSurfaceReview => "complete_surface_review",
            Self::EnableDryRunPreview => "enable_dry_run_preview",
        }
    }
}

/// A field the support / export packet carries so version-bump and publish-target
/// truth is reconstructable from the shared model. The fields in
/// [`M5PublicationExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationExportField {
    /// The opaque prior-version representation.
    PriorVersion,
    /// The opaque next-version representation.
    NextVersion,
    /// The version-bump class.
    VersionBumpClass,
    /// The derived public-surface impact.
    PublicSurfaceImpact,
    /// The changed artifact set.
    ChangedArtifactSet,
    /// The publish-target class.
    TargetClass,
    /// The target visibility.
    TargetVisibility,
    /// The target mutability.
    TargetMutability,
    /// The target auth source.
    AuthSource,
    /// The dry-run availability.
    DryRunAvailability,
    /// The rollout ring.
    RolloutRing,
    /// The derived readiness posture.
    Readiness,
    /// The block reason (when blocked or narrowed).
    BlockReason,
}

impl M5PublicationExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::PriorVersion,
        Self::NextVersion,
        Self::VersionBumpClass,
        Self::PublicSurfaceImpact,
        Self::ChangedArtifactSet,
        Self::TargetClass,
        Self::TargetVisibility,
        Self::TargetMutability,
        Self::AuthSource,
        Self::DryRunAvailability,
        Self::RolloutRing,
        Self::Readiness,
        Self::BlockReason,
    ];

    /// The export fields every publish-sheet export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::NextVersion,
        Self::PublicSurfaceImpact,
        Self::TargetClass,
        Self::AuthSource,
        Self::Readiness,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PriorVersion => "prior_version",
            Self::NextVersion => "next_version",
            Self::VersionBumpClass => "version_bump_class",
            Self::PublicSurfaceImpact => "public_surface_impact",
            Self::ChangedArtifactSet => "changed_artifact_set",
            Self::TargetClass => "target_class",
            Self::TargetVisibility => "target_visibility",
            Self::TargetMutability => "target_mutability",
            Self::AuthSource => "auth_source",
            Self::DryRunAvailability => "dry_run_availability",
            Self::RolloutRing => "rollout_ring",
            Self::Readiness => "readiness",
            Self::BlockReason => "block_reason",
        }
    }
}

/// A self-contained publication-blocked banner: the exact reason, the blocked
/// destination, and the next action, so a blocked publication state is understood
/// from the banner alone rather than from secondary logs or internal pipeline pages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationBlockedBanner {
    /// The exact block reason.
    pub reason: M5PublicationBlockReason,
    /// The next action a reviewer should take.
    pub next_action: M5PublicationNextAction,
    /// The publish-target class the block applies to.
    pub blocked_target_class: M5PublishTargetClass,
    /// The target visibility the block applies to.
    pub blocked_visibility: M5PublishTargetVisibility,
    /// The changed artifact set the block applies to.
    pub changed_artifact_set: Vec<String>,
    /// The derived public-surface impact of the blocked bump.
    pub public_surface_impact: M5PublicSurfaceImpact,
    /// A deterministic, self-contained headline naming the reason, the destination,
    /// the impact, and the next action — never a generic `cannot publish`.
    pub headline: String,
}

/// The full input to the publication-review resolver for one publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationReviewInput {
    /// The opaque, export-safe proposal label.
    pub proposal_label: String,
    /// The opaque, export-safe prior version.
    pub prior_version_repr: String,
    /// The opaque, export-safe next version.
    pub next_version_repr: String,
    /// The version-bump class.
    pub version_bump_class: M5VersionBumpClass,
    /// The declared compatibility impact.
    pub compatibility_impact: M5CompatibilityImpact,
    /// The changed artifact set. Must be non-empty so publication scope is explicit.
    pub changed_artifact_set: Vec<String>,
    /// The publish-target class.
    pub target_class: M5PublishTargetClass,
    /// The target visibility.
    pub visibility: M5PublishTargetVisibility,
    /// The target mutability.
    pub mutability: M5TargetMutability,
    /// The target auth source.
    pub auth_source: M5TargetAuthSource,
    /// The auth-disclosure state (never inherited silently from ambient credentials).
    pub auth_disclosure_state: M5AuthDisclosureState,
    /// The dry-run availability.
    pub dry_run: M5DryRunAvailability,
    /// The rollout ring the publish targets.
    pub rollout_ring: M5RolloutRing,
    /// The public-surface impact-analysis state.
    pub surface_impact_analysis: M5SurfaceImpactAnalysis,
}

/// The resolved version-bump / publish-target truth for one publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPublicationReview {
    /// The opaque proposal label.
    pub proposal_label: String,
    /// The opaque prior version.
    pub prior_version_repr: String,
    /// The opaque next version.
    pub next_version_repr: String,
    /// The version-bump class.
    pub version_bump_class: M5VersionBumpClass,
    /// The declared compatibility impact.
    pub compatibility_impact: M5CompatibilityImpact,
    /// The derived public-surface impact.
    pub public_surface_impact: M5PublicSurfaceImpact,
    /// The changed artifact set.
    pub changed_artifact_set: Vec<String>,
    /// The count of changed artifacts.
    pub changed_artifact_count: usize,
    /// The publish-target class.
    pub target_class: M5PublishTargetClass,
    /// The target visibility.
    pub visibility: M5PublishTargetVisibility,
    /// The target mutability.
    pub mutability: M5TargetMutability,
    /// The target auth source.
    pub auth_source: M5TargetAuthSource,
    /// The auth-disclosure state.
    pub auth_disclosure_state: M5AuthDisclosureState,
    /// The dry-run availability.
    pub dry_run: M5DryRunAvailability,
    /// The rollout ring.
    pub rollout_ring: M5RolloutRing,
    /// The public-surface impact-analysis state.
    pub surface_impact_analysis: M5SurfaceImpactAnalysis,
    /// The derived destination reversibility.
    pub destination_reversibility: M5DestinationReversibility,
    /// The derived publication readiness.
    pub readiness: M5PublicationReadiness,
    /// True when the publication may proceed.
    pub is_publishable: bool,
    /// True when the publication is hard-blocked.
    pub is_blocked: bool,
    /// True when the publication is narrowed.
    pub is_narrowed: bool,
    /// The publication-blocked banner, present when blocked or narrowed.
    pub publication_banner: Option<M5PublicationBlockedBanner>,
}

/// Errors returned by [`resolve_publication_review`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5PublicationReviewError {
    /// The proposal label was empty.
    EmptyProposalLabel,
    /// A prior or next version was empty.
    EmptyVersion,
    /// The changed artifact set was empty (publication scope must be explicit).
    EmptyChangedArtifactSet,
    /// The next version equals the prior version for a non-republish bump.
    NextVersionEqualsPriorForBump,
    /// A proposal label, version, or artifact id carried forbidden material.
    ForbiddenPublicationMaterial,
}

impl M5PublicationReviewError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyProposalLabel => "empty_proposal_label",
            Self::EmptyVersion => "empty_version",
            Self::EmptyChangedArtifactSet => "empty_changed_artifact_set",
            Self::NextVersionEqualsPriorForBump => "next_version_equals_prior_for_bump",
            Self::ForbiddenPublicationMaterial => "forbidden_publication_material",
        }
    }
}

impl fmt::Display for M5PublicationReviewError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "publication-review resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5PublicationReviewError {}

/// Resolves one publication review from its declared version-bump and publish-target
/// state.
///
/// The derived readiness posture is the headline verdict, computed in a fixed
/// blocking-first order: an unknown auth-disclosure or surface-impact reading blocks
/// first, then missing surface analysis, then stale surface analysis, then an
/// ambient-credential inheritance, then a pending surface review narrows, then an
/// unproven destination reversibility narrows, then a disclosed waiver requires a
/// dry-run first, then a broad auth scope or aging analysis carries disclosed
/// review, and only a publication with a scoped auth source, fresh analysis, and a
/// reversible destination is cleanly publishable. The public-surface impact is
/// derived from the bump class and compatibility impact — never collapsed into the
/// semver string — and a blocked or narrowed publication always produces a
/// self-contained banner.
pub fn resolve_publication_review(
    input: &M5PublicationReviewInput,
) -> Result<M5ResolvedPublicationReview, M5PublicationReviewError> {
    if input.proposal_label.trim().is_empty() {
        return Err(M5PublicationReviewError::EmptyProposalLabel);
    }
    if input.prior_version_repr.trim().is_empty() || input.next_version_repr.trim().is_empty() {
        return Err(M5PublicationReviewError::EmptyVersion);
    }
    if input.changed_artifact_set.is_empty() {
        return Err(M5PublicationReviewError::EmptyChangedArtifactSet);
    }
    if value_repr_is_forbidden(&input.proposal_label)
        || value_repr_is_forbidden(&input.prior_version_repr)
        || value_repr_is_forbidden(&input.next_version_repr)
    {
        return Err(M5PublicationReviewError::ForbiddenPublicationMaterial);
    }
    for artifact in &input.changed_artifact_set {
        if value_repr_is_forbidden(artifact) {
            return Err(M5PublicationReviewError::ForbiddenPublicationMaterial);
        }
    }
    if input.next_version_repr == input.prior_version_repr
        && !matches!(
            input.version_bump_class,
            M5VersionBumpClass::RepublishNoVersionChange
        )
    {
        return Err(M5PublicationReviewError::NextVersionEqualsPriorForBump);
    }

    let public_surface_impact =
        derive_public_surface_impact(input.version_bump_class, input.compatibility_impact);
    let destination_reversibility =
        derive_destination_reversibility(input.mutability, input.dry_run);
    let readiness = derive_readiness(
        input.auth_disclosure_state,
        input.surface_impact_analysis,
        destination_reversibility,
    );

    let is_publishable = readiness.is_publishable();
    let is_blocked = readiness.is_blocked();
    let is_narrowed = readiness.is_narrowed();

    let publication_banner = readiness.block_reason().map(|reason| {
        let next_action = reason.next_action();
        let headline = format!(
            "Publication held: {} — target {} ({} visibility), {} impact across {} changed artifact(s); next: {}",
            reason.phrase(),
            input.target_class.as_str(),
            input.visibility.as_str(),
            public_surface_impact.as_str(),
            input.changed_artifact_set.len(),
            next_action.as_str()
        );
        M5PublicationBlockedBanner {
            reason,
            next_action,
            blocked_target_class: input.target_class,
            blocked_visibility: input.visibility,
            changed_artifact_set: input.changed_artifact_set.clone(),
            public_surface_impact,
            headline,
        }
    });

    Ok(M5ResolvedPublicationReview {
        proposal_label: input.proposal_label.clone(),
        prior_version_repr: input.prior_version_repr.clone(),
        next_version_repr: input.next_version_repr.clone(),
        version_bump_class: input.version_bump_class,
        compatibility_impact: input.compatibility_impact,
        public_surface_impact,
        changed_artifact_set: input.changed_artifact_set.clone(),
        changed_artifact_count: input.changed_artifact_set.len(),
        target_class: input.target_class,
        visibility: input.visibility,
        mutability: input.mutability,
        auth_source: input.auth_source,
        auth_disclosure_state: input.auth_disclosure_state,
        dry_run: input.dry_run,
        rollout_ring: input.rollout_ring,
        surface_impact_analysis: input.surface_impact_analysis,
        destination_reversibility,
        readiness,
        is_publishable,
        is_blocked,
        is_narrowed,
        publication_banner,
    })
}

/// Derives the public-surface impact from the bump class and compatibility impact,
/// so the impact is never collapsed into a single semver string.
fn derive_public_surface_impact(
    bump: M5VersionBumpClass,
    compat: M5CompatibilityImpact,
) -> M5PublicSurfaceImpact {
    match compat {
        M5CompatibilityImpact::SchemaMigrationRequired => {
            M5PublicSurfaceImpact::MigrationRequiredPublicSurface
        }
        M5CompatibilityImpact::BreakingChange | M5CompatibilityImpact::ForwardIncompatible => {
            M5PublicSurfaceImpact::BreakingPublicSurface
        }
        M5CompatibilityImpact::RuntimeBehaviorOnly => M5PublicSurfaceImpact::RuntimeBehaviorShift,
        M5CompatibilityImpact::BackwardCompatible => {
            if matches!(
                bump,
                M5VersionBumpClass::RepublishNoVersionChange
                    | M5VersionBumpClass::BuildMetadataOnly
            ) {
                M5PublicSurfaceImpact::NoPublicSurfaceChange
            } else {
                M5PublicSurfaceImpact::AdditivePublicSurface
            }
        }
    }
}

/// Derives the destination reversibility from target mutability and dry-run
/// availability, so a mutable target with no dry-run is never confused with an
/// immutable publication step.
fn derive_destination_reversibility(
    mutability: M5TargetMutability,
    dry_run: M5DryRunAvailability,
) -> M5DestinationReversibility {
    if !matches!(dry_run, M5DryRunAvailability::DryRunUnavailable) {
        M5DestinationReversibility::DryRunProven
    } else if matches!(
        mutability,
        M5TargetMutability::ImmutableOncePublished | M5TargetMutability::AppendOnly
    ) {
        M5DestinationReversibility::ImmutableByDesign
    } else {
        M5DestinationReversibility::ReversibilityUnproven
    }
}

/// The fixed blocking-first readiness ladder.
fn derive_readiness(
    auth: M5AuthDisclosureState,
    surface: M5SurfaceImpactAnalysis,
    reversibility: M5DestinationReversibility,
) -> M5PublicationReadiness {
    let state_unknown = matches!(auth, M5AuthDisclosureState::AuthDisclosureUnknown)
        || matches!(surface, M5SurfaceImpactAnalysis::SurfaceImpactUnknown);
    if state_unknown {
        M5PublicationReadiness::BlockedUnknownState
    } else if matches!(surface, M5SurfaceImpactAnalysis::SurfaceImpactMissing) {
        M5PublicationReadiness::BlockedSurfaceImpactMissing
    } else if matches!(surface, M5SurfaceImpactAnalysis::SurfaceImpactStale) {
        M5PublicationReadiness::BlockedSurfaceImpactStale
    } else if matches!(auth, M5AuthDisclosureState::AmbientCredentialInherited) {
        M5PublicationReadiness::BlockedAmbientCredential
    } else if matches!(auth, M5AuthDisclosureState::AuthDisclosurePendingReview) {
        M5PublicationReadiness::NarrowedSurfaceReviewPending
    } else if matches!(
        reversibility,
        M5DestinationReversibility::ReversibilityUnproven
    ) {
        M5PublicationReadiness::NarrowedReversibilityUnproven
    } else if matches!(auth, M5AuthDisclosureState::AuthDisclosedUnderWaiver) {
        M5PublicationReadiness::PublishableDryRunFirst
    } else if matches!(auth, M5AuthDisclosureState::AuthBroadDisclosed)
        || matches!(surface, M5SurfaceImpactAnalysis::SurfaceImpactAging)
    {
        M5PublicationReadiness::PublishableWithReview
    } else {
        M5PublicationReadiness::Publishable
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs version-bump and publish-target truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationReviewResolutionCase {
    /// The resolver input.
    pub input: M5PublicationReviewInput,
    /// The resolved truth. Must equal `resolve_publication_review(&input)`.
    pub resolved: M5ResolvedPublicationReview,
}

impl M5PublicationReviewResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5PublicationReviewInput) -> Self {
        let resolved = resolve_publication_review(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_publication_review(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one publication consumer bound to the shared row
/// anatomy, readiness postures, version-bump classes, public-surface impacts, target
/// classes, visibilities, mutabilities, auth sources, dry-run availabilities, rollout
/// rings, block reasons, next actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationReviewRow {
    /// Publication consumer family.
    pub consumer_surface: M5PublicationReviewConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5ReleaseCenterQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 publication surface families that render / consume this sheet.
    pub surface_families: Vec<M5PublicationSurfaceFamily>,
    /// Deployment lines this sheet keeps the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts this sheet renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5PublicationReviewAnatomyPart>,
    /// Version-bump classes this sheet names.
    pub version_bump_classes: Vec<M5VersionBumpClass>,
    /// Compatibility impacts this sheet distinguishes.
    pub compatibility_impacts: Vec<M5CompatibilityImpact>,
    /// Public-surface impacts this sheet distinguishes.
    pub public_surface_impacts: Vec<M5PublicSurfaceImpact>,
    /// Publish-target classes this sheet names.
    pub target_classes: Vec<M5PublishTargetClass>,
    /// Target visibilities this sheet distinguishes.
    pub target_visibilities: Vec<M5PublishTargetVisibility>,
    /// Target mutabilities this sheet distinguishes.
    pub target_mutabilities: Vec<M5TargetMutability>,
    /// Target auth sources this sheet discloses.
    pub target_auth_sources: Vec<M5TargetAuthSource>,
    /// Auth-disclosure states this sheet distinguishes.
    pub auth_disclosure_states: Vec<M5AuthDisclosureState>,
    /// Dry-run availabilities this sheet distinguishes.
    pub dry_run_availabilities: Vec<M5DryRunAvailability>,
    /// Rollout rings this sheet names.
    pub rollout_rings: Vec<M5RolloutRing>,
    /// Surface-impact-analysis states this sheet distinguishes.
    pub surface_impact_analyses: Vec<M5SurfaceImpactAnalysis>,
    /// Destination reversibilities this sheet distinguishes.
    pub destination_reversibilities: Vec<M5DestinationReversibility>,
    /// Readiness postures this sheet distinguishes.
    pub readiness_postures: Vec<M5PublicationReadiness>,
    /// Block reasons this sheet names.
    pub block_reasons: Vec<M5PublicationBlockReason>,
    /// Next actions this sheet names.
    pub next_actions: Vec<M5PublicationNextAction>,
    /// Export fields this sheet carries (must include the mandatory fields).
    pub export_fields: Vec<M5PublicationExportField>,
    /// Non-visual accessibility routes this sheet offers.
    pub accessibility_routes: Vec<M5ReleaseCenterAccessibilityRoute>,
    /// Release-center subsystems that consume this sheet's projection.
    pub consumer_surfaces: Vec<M5ReleaseCenterConsumerSurface>,
    /// Downgrade triggers that apply to this sheet.
    pub downgrade_triggers: Vec<M5ReleaseCenterDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5PublicationReviewResolutionCase>,
    /// Hard invariant: this sheet never collapses the public-surface impact into the
    /// semver string alone. MUST be `false`.
    pub collapses_impact_into_semver_string: bool,
    /// Hard invariant: this sheet never masks the target auth source or destination
    /// class. MUST be `false`.
    pub masks_target_auth_source_or_destination_class: bool,
    /// Hard invariant: this sheet never lets a mutable target read as immutable. MUST
    /// be `false`.
    pub confuses_mutable_with_immutable_publication: bool,
    /// Hard invariant: this sheet never inherits ambient credentials silently. MUST
    /// be `false`.
    pub inherits_ambient_credentials_silently: bool,
}

impl M5PublicationReviewRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5PublicationReviewAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5PublicationReviewAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5PublicationExportField> =
            self.export_fields.iter().copied().collect();
        M5PublicationExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_impact_into_semver_string
            && !self.masks_target_auth_source_or_destination_class
            && !self.confuses_mutable_with_immutable_publication
            && !self.inherits_ambient_credentials_silently
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationReviewVocabularySet {
    /// Publication consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Publish-target-class tokens.
    pub target_classes: Vec<String>,
    /// Public-surface-impact tokens.
    pub public_surface_impacts: Vec<String>,
    /// Auth-disclosure-state tokens.
    pub auth_disclosure_states: Vec<String>,
    /// Surface-impact-analysis tokens.
    pub surface_impact_analyses: Vec<String>,
    /// Destination-reversibility tokens.
    pub destination_reversibilities: Vec<String>,
    /// Readiness-posture tokens.
    pub readiness_postures: Vec<String>,
    /// Block-reason tokens.
    pub block_reasons: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Version-bump-class tokens (reused from the frozen matrix).
    pub version_bump_classes: Vec<String>,
    /// Compatibility-impact tokens (reused from the frozen matrix).
    pub compatibility_impacts: Vec<String>,
    /// Publish-target-visibility tokens (reused from the frozen matrix).
    pub target_visibilities: Vec<String>,
    /// Target-mutability tokens (reused from the frozen matrix).
    pub target_mutabilities: Vec<String>,
    /// Target-auth-source tokens (reused from the frozen matrix).
    pub target_auth_sources: Vec<String>,
    /// Dry-run-availability tokens (reused from the frozen matrix).
    pub dry_run_availabilities: Vec<String>,
    /// Rollout-ring tokens (reused from the frozen matrix).
    pub rollout_rings: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5PublicationReviewVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5PublicationReviewConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5PublicationReviewAnatomyPart::ALL, |v| v.as_str()),
            target_classes: tokens(&M5PublishTargetClass::ALL, |v| v.as_str()),
            public_surface_impacts: tokens(&M5PublicSurfaceImpact::ALL, |v| v.as_str()),
            auth_disclosure_states: tokens(&M5AuthDisclosureState::ALL, |v| v.as_str()),
            surface_impact_analyses: tokens(&M5SurfaceImpactAnalysis::ALL, |v| v.as_str()),
            destination_reversibilities: tokens(&M5DestinationReversibility::ALL, |v| v.as_str()),
            readiness_postures: tokens(&M5PublicationReadiness::ALL, |v| v.as_str()),
            block_reasons: tokens(&M5PublicationBlockReason::ALL, |v| v.as_str()),
            next_actions: tokens(&M5PublicationNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5PublicationExportField::ALL, |v| v.as_str()),
            version_bump_classes: tokens(&M5VersionBumpClass::ALL, |v| v.as_str()),
            compatibility_impacts: tokens(&M5CompatibilityImpact::ALL, |v| v.as_str()),
            target_visibilities: tokens(&M5PublishTargetVisibility::ALL, |v| v.as_str()),
            target_mutabilities: tokens(&M5TargetMutability::ALL, |v| v.as_str()),
            target_auth_sources: tokens(&M5TargetAuthSource::ALL, |v| v.as_str()),
            dry_run_availabilities: tokens(&M5DryRunAvailability::ALL, |v| v.as_str()),
            rollout_rings: tokens(&M5RolloutRing::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ReleaseCenterAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationReviewGovernanceReview {
    /// One publication primitive carries version-bump and publish-target truth on
    /// every consumer.
    pub one_primitive_carries_publication_truth: bool,
    /// The version-bump identity and public-surface impact are shown before mutation.
    pub version_identity_and_impact_always_shown: bool,
    /// The public-surface impact is never collapsed into the semver string.
    pub impact_never_collapsed_into_semver: bool,
    /// The target auth source and destination class are shown before mutation.
    pub auth_source_and_destination_shown_before_mutation: bool,
    /// Mutability and dry-run availability are never confused with immutable steps.
    pub mutability_and_dry_run_never_confused_with_immutable: bool,
    /// Ambient credentials are never inherited silently.
    pub ambient_credentials_never_inherited_silently: bool,
    /// A blocked or narrowed publication always shows a self-contained banner.
    pub blocked_state_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and next action, never a generic message.
    pub banner_names_exact_reason_and_next_action: bool,
    /// The support / export packet reconstructs version-bump and publish-target truth.
    pub support_export_reconstructs_publication_truth: bool,
    /// No consumer invents a second version-bump or publish-target grammar.
    pub no_surface_invents_second_publication_grammar: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel version-bump / publish-target vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationReviewConsumerProjection {
    /// Release-center, update-center, CLI, admin, and support/evaluation consumers
    /// all consume the shared primitive.
    pub publication_surfaces_consume_shared_primitive: bool,
    /// The readiness resolver reads a single canonical source.
    pub readiness_resolver_reads_single_source: bool,
    /// The public-surface impact cue reads a single canonical source.
    pub public_surface_impact_reads_single_source: bool,
    /// The auth-source disclosure reads a single canonical source.
    pub auth_source_disclosure_reads_single_source: bool,
    /// Support / export reads a single canonical publish-sheet source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationReviewProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the publication primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationReviewReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting publication audit.
    pub publication_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5PublicationReviewPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PublicationReviewPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Publication rows.
    pub publication_rows: Vec<M5PublicationReviewRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PublicationReviewVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PublicationReviewGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PublicationReviewConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PublicationReviewProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PublicationReviewReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 publication-review-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationReviewPrimitivePacket {
    /// Record kind; must equal [`M5_PUBLICATION_REVIEW_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PUBLICATION_REVIEW_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Publication rows.
    pub publication_rows: Vec<M5PublicationReviewRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PublicationReviewVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PublicationReviewGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PublicationReviewConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PublicationReviewProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PublicationReviewReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PublicationReviewPrimitivePacket {
    /// Builds an M5 publication-review-primitive packet from stable-lane input.
    pub fn new(input: M5PublicationReviewPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_PUBLICATION_REVIEW_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_PUBLICATION_REVIEW_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            publication_rows: input.publication_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 publication-review-primitive invariants.
    pub fn validate(&self) -> Vec<M5PublicationReviewPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PUBLICATION_REVIEW_PRIMITIVE_RECORD_KIND {
            violations.push(M5PublicationReviewPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PUBLICATION_REVIEW_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5PublicationReviewPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PublicationReviewPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_publication_rows(self, &mut violations);
        validate_publishability_coverage(self, &mut violations);
        validate_mutability_and_dry_run_explicit(self, &mut violations);
        validate_ambient_credential_surfaced(self, &mut violations);
        validate_blocked_banner_self_contained(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 publication-review primitive packet serializes"),
        ) {
            violations.push(M5PublicationReviewPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 publication-review primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per publication consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,readiness_postures,public_surface_impacts,target_classes,auth_disclosure_states,block_reasons,next_actions,export_fields,example_count\n",
        );
        for row in &self.publication_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.readiness_postures, |v| v.as_str()),
                join_tokens(&row.public_surface_impacts, |v| v.as_str()),
                join_tokens(&row.target_classes, |v| v.as_str()),
                join_tokens(&row.auth_disclosure_states, |v| v.as_str()),
                join_tokens(&row.block_reasons, |v| v.as_str()),
                join_tokens(&row.next_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .publication_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Version-Bump Row and Publish-Target Review-Sheet Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Publication consumers: {} ({} stable)\n",
            self.publication_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Readiness postures: {}\n",
            self.vocabulary_set.readiness_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Public-surface impacts: {}\n",
            self.vocabulary_set.public_surface_impacts.join(", ")
        ));
        out.push_str(&format!(
            "- Block reasons: {}\n",
            self.vocabulary_set.block_reasons.join(", ")
        ));
        out.push_str(&format!(
            "- Destination reversibilities: {}\n",
            self.vocabulary_set.destination_reversibilities.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Publication consumers\n\n");
        for row in &self.publication_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                let banner = match &case.resolved.publication_banner {
                    Some(banner) => banner.reason.as_str(),
                    None => "clear",
                };
                out.push_str(&format!(
                    "    - `{}` → `{}` on `{}` → `{}` ({} impact, {} reversibility, banner `{}`)\n",
                    case.resolved.prior_version_repr,
                    case.resolved.next_version_repr,
                    case.resolved.target_class.as_str(),
                    case.resolved.readiness.as_str(),
                    case.resolved.public_surface_impact.as_str(),
                    case.resolved.destination_reversibility.as_str(),
                    banner
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 publication-review-primitive export.
#[derive(Debug)]
pub enum M5PublicationReviewPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PublicationReviewPrimitiveViolation>),
}

impl fmt::Display for M5PublicationReviewPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 publication-review primitive export parse failed: {error}"
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
                    "m5 publication-review primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PublicationReviewPrimitiveArtifactError {}

/// Validation failures emitted by [`M5PublicationReviewPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PublicationReviewPrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required publication consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A publication row is incomplete.
    PublicationRowIncomplete,
    /// A publication row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A publication row declares no version-bump classes.
    VersionBumpClassMissing,
    /// A publication row declares no readiness postures.
    ReadinessPostureMissing,
    /// A publication row declares no public-surface impacts.
    PublicSurfaceImpactMissing,
    /// A publication row declares no destination reversibilities.
    DestinationReversibilityMissing,
    /// A publication row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A publication row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A publication row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A publication row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A publication row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A publication claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked resolution proves both a publishable and a blocked publication.
    PublishabilityCoverageUnproven,
    /// No worked resolution proves an immutable step distinguished from a dry-run one.
    MutabilityAndDryRunExplicitUnproven,
    /// No worked resolution proves an inherited ambient credential surfaced as blocked.
    AmbientCredentialSurfacedUnproven,
    /// No worked resolution proves a blocked publication with a self-contained banner.
    BlockedBannerSelfContainedUnproven,
    /// A publication row violates a hard invariant.
    PublicationInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5PublicationReviewPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::PublicationRowIncomplete => "publication_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::VersionBumpClassMissing => "version_bump_class_missing",
            Self::ReadinessPostureMissing => "readiness_posture_missing",
            Self::PublicSurfaceImpactMissing => "public_surface_impact_missing",
            Self::DestinationReversibilityMissing => "destination_reversibility_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::PublishabilityCoverageUnproven => "publishability_coverage_unproven",
            Self::MutabilityAndDryRunExplicitUnproven => "mutability_and_dry_run_explicit_unproven",
            Self::AmbientCredentialSurfacedUnproven => "ambient_credential_surfaced_unproven",
            Self::BlockedBannerSelfContainedUnproven => "blocked_banner_self_contained_unproven",
            Self::PublicationInvariantViolated => "publication_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 publication-review-primitive export.
pub fn current_stable_m5_publication_review_primitive_export(
) -> Result<M5PublicationReviewPrimitivePacket, M5PublicationReviewPrimitiveArtifactError> {
    let packet: M5PublicationReviewPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-publish-target-review-sheet-proof/support_export.json"
    )))
    .map_err(M5PublicationReviewPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PublicationReviewPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5PublicationReviewPrimitivePacket,
    violations: &mut Vec<M5PublicationReviewPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PUBLICATION_REVIEW_SCHEMA_REF,
        M5_PUBLICATION_REVIEW_DOC_REF,
        M5_PUBLICATION_REVIEW_COMPONENT_MATRIX_REF,
        M5_PUBLICATION_REVIEW_OBJECT_MODEL_REF,
        M5_PUBLICATION_REVIEW_VERIFICATION_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5PublicationReviewPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5PublicationReviewPrimitivePacket,
    violations: &mut Vec<M5PublicationReviewPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5PublicationReviewPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_publication_rows(
    packet: &M5PublicationReviewPrimitivePacket,
    violations: &mut Vec<M5PublicationReviewPrimitiveViolation>,
) {
    let present: BTreeSet<M5PublicationReviewConsumerSurface> = packet
        .publication_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5PublicationReviewConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5PublicationReviewPrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.publication_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.compatibility_impacts.is_empty()
            || row.target_classes.is_empty()
            || row.target_visibilities.is_empty()
            || row.target_mutabilities.is_empty()
            || row.target_auth_sources.is_empty()
            || row.auth_disclosure_states.is_empty()
            || row.dry_run_availabilities.is_empty()
            || row.rollout_rings.is_empty()
            || row.surface_impact_analyses.is_empty()
            || row.block_reasons.is_empty()
            || row.next_actions.is_empty()
        {
            violations.push(M5PublicationReviewPrimitiveViolation::PublicationRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5PublicationReviewPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.version_bump_classes.is_empty() {
            violations.push(M5PublicationReviewPrimitiveViolation::VersionBumpClassMissing);
        }
        if row.readiness_postures.is_empty() {
            violations.push(M5PublicationReviewPrimitiveViolation::ReadinessPostureMissing);
        }
        if row.public_surface_impacts.is_empty() {
            violations.push(M5PublicationReviewPrimitiveViolation::PublicSurfaceImpactMissing);
        }
        if row.destination_reversibilities.is_empty() {
            violations.push(M5PublicationReviewPrimitiveViolation::DestinationReversibilityMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5PublicationReviewPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ReleaseCenterAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5PublicationReviewPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5PublicationReviewPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5PublicationReviewPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5PublicationReviewPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5PublicationReviewPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5PublicationReviewPrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5PublicationReviewPrimitiveViolation::PublicationInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must prove a publishable
/// publication and at least one must prove a blocked publication — the
/// acceptance-criterion example that a user can tell publishable from blocked before
/// pushing a target.
fn validate_publishability_coverage(
    packet: &M5PublicationReviewPrimitivePacket,
    violations: &mut Vec<M5PublicationReviewPrimitiveViolation>,
) {
    let has_publishable = packet.publication_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_publishable)
    });
    let has_blocked = packet.publication_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_blocked)
    });
    if !(has_publishable && has_blocked) {
        violations.push(M5PublicationReviewPrimitiveViolation::PublishabilityCoverageUnproven);
    }
}

/// At least one worked resolution must prove an immutable-by-design destination and
/// at least one must prove a dry-run-proven destination — the acceptance-criterion
/// example that mutability and dry-run availability stay explicit and cannot be
/// confused with an immutable publication step.
fn validate_mutability_and_dry_run_explicit(
    packet: &M5PublicationReviewPrimitivePacket,
    violations: &mut Vec<M5PublicationReviewPrimitiveViolation>,
) {
    let has_immutable = packet.publication_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.destination_reversibility == M5DestinationReversibility::ImmutableByDesign
        })
    });
    let has_dry_run = packet.publication_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.destination_reversibility == M5DestinationReversibility::DryRunProven
        })
    });
    if !(has_immutable && has_dry_run) {
        violations.push(M5PublicationReviewPrimitiveViolation::MutabilityAndDryRunExplicitUnproven);
    }
}

/// At least one worked resolution must prove an inherited ambient credential being
/// surfaced as a blocked publication — the acceptance-criterion example that ambient
/// credential inheritance is never silent.
fn validate_ambient_credential_surfaced(
    packet: &M5PublicationReviewPrimitivePacket,
    violations: &mut Vec<M5PublicationReviewPrimitiveViolation>,
) {
    let proven = packet.publication_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.auth_disclosure_state == M5AuthDisclosureState::AmbientCredentialInherited
                && case.resolved.readiness == M5PublicationReadiness::BlockedAmbientCredential
        })
    });
    if !proven {
        violations.push(M5PublicationReviewPrimitiveViolation::AmbientCredentialSurfacedUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a blocked publication
/// whose banner carries a specific reason, a next action, the blocked destination,
/// and a non-empty changed artifact set — the acceptance-criterion example that a
/// blocked state is understood from the banner rather than a secondary log.
fn validate_blocked_banner_self_contained(
    packet: &M5PublicationReviewPrimitivePacket,
    violations: &mut Vec<M5PublicationReviewPrimitiveViolation>,
) {
    let proven = packet.publication_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_blocked
                && case
                    .resolved
                    .publication_banner
                    .as_ref()
                    .is_some_and(|banner| {
                        !banner.headline.trim().is_empty()
                            && !banner.changed_artifact_set.is_empty()
                    })
        })
    });
    if !proven {
        violations.push(M5PublicationReviewPrimitiveViolation::BlockedBannerSelfContainedUnproven);
    }
}

fn validate_governance_review(
    packet: &M5PublicationReviewPrimitivePacket,
    violations: &mut Vec<M5PublicationReviewPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_publication_truth,
        review.version_identity_and_impact_always_shown,
        review.impact_never_collapsed_into_semver,
        review.auth_source_and_destination_shown_before_mutation,
        review.mutability_and_dry_run_never_confused_with_immutable,
        review.ambient_credentials_never_inherited_silently,
        review.blocked_state_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_next_action,
        review.support_export_reconstructs_publication_truth,
        review.no_surface_invents_second_publication_grammar,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5PublicationReviewPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5PublicationReviewPrimitivePacket,
    violations: &mut Vec<M5PublicationReviewPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.publication_surfaces_consume_shared_primitive,
        projection.readiness_resolver_reads_single_source,
        projection.public_surface_impact_reads_single_source,
        projection.auth_source_disclosure_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5PublicationReviewPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5PublicationReviewPrimitivePacket,
    violations: &mut Vec<M5PublicationReviewPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5PublicationReviewPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5PublicationReviewPrimitivePacket,
    violations: &mut Vec<M5PublicationReviewPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.publication_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5PublicationReviewPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
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

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
