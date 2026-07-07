//! Frozen M5 release-candidate-card, version-bump-row, publish-target-row,
//! artifact-provenance-bundle-card, and promotion-timeline component matrix.
//!
//! This module locks Aureline's reusable release-center and publication
//! components into one export-safe packet. Every component family M5 claims that
//! still drifts too easily by pipeline or admin page — the release candidate card,
//! the version-bump row, the publish target row/review sheet, the artifact
//! provenance bundle card, the promotion timeline step, and the rollback/revocation
//! row — is named once here and constrained by the same candidate-scope,
//! blocker-freshness, target visibility/mutability/auth-source, signature/
//! attestation/SBOM, immutable-digest-lineage, rollout-ring, and rollback-blast-
//! radius rules regardless of the surface family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components*
//! themselves: the component families, the candidate scope classes and blocker
//! states, the version-bump classes and compatibility impacts, the publish-target
//! visibilities, mutabilities, auth sources, and dry-run availabilities, the
//! signature/attestation/SBOM statuses and digest-lineage states, the rollout
//! rings and promotion stage states, the rollback blast radii and revocation
//! scopes, the deployment lines every component must survive, the non-visual
//! accessibility routes, and the mandatory labels every component must be able to
//! show. It does not re-architect the artifact graph, promotion pipeline, or
//! mirror transport that already own those records — it is the shared component
//! contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 release or
//! publication component may publish a candidate, version-bump, publish-target,
//! provenance, promotion, or rollback claim. Release-center, update-center,
//! registry, mirror, enterprise-evaluation, support, docs, and admin surfaces all
//! consume this packet so one candidate card carries scope and blocker freshness,
//! one version-bump row states its compatibility impact, one publish-target row
//! names the target's auth source and mutability, one provenance card shows
//! signature, attestation, and SBOM truth over an immutable digest lineage, one
//! promotion timeline step names its rollout ring and stage, and one
//! rollback/revocation row states its blast radius before any promotion. No M5
//! lane invents a second release-status grammar, masks a target auth source or
//! mutability, conflates signed with unsigned provenance, or overstates rollback
//! reversibility.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5ReleaseCenterVocabularySet`] rather than minted per surface. Raw URLs, raw
//! signing keys, raw tokens, credentials, private endpoints, and user text bodies
//! stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-release-center-components.schema.json`](../../../../schemas/ui/m5-release-center-components.schema.json)
//! and the contract doc is
//! [`docs/release/m5_release_center_components_contract.md`](../../../../docs/release/m5_release_center_components_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-release-center-components/`](../../../../fixtures/ui/m5-release-center-components/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_release_center_component_matrix,
    seeded_m5_release_center_component_matrix_promotion_timeline_step_beta_narrowed,
    seeded_m5_release_center_component_matrix_rollback_revocation_row_preview_narrowed,
    M5_RELEASE_CENTER_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ReleaseCenterMatrixPacket`].
pub const M5_RELEASE_CENTER_MATRIX_RECORD_KIND: &str =
    "freeze_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix";

/// Schema version for M5 release-center-component-matrix records.
pub const M5_RELEASE_CENTER_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the release-center-components boundary schema.
pub const M5_RELEASE_CENTER_SCHEMA_REF: &str =
    "schemas/ui/m5-release-center-components.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_RELEASE_CENTER_DOC_REF: &str = "docs/release/m5_release_center_components_contract.md";

/// Repo-relative path of the release-center object-model contract this matrix
/// binds against.
pub const M5_RELEASE_CENTER_OBJECT_MODEL_REF: &str =
    "docs/release/release_center_object_model_contract.md";

/// Repo-relative path of the update-and-rollback contract this matrix binds
/// against.
pub const M5_RELEASE_CENTER_ROLLBACK_CONTRACT_REF: &str =
    "docs/release/update_and_rollback_contract.md";

/// Repo-relative path of the artifact-verification (signature/attestation/SBOM)
/// contract this matrix binds against.
pub const M5_RELEASE_CENTER_PROVENANCE_CONTRACT_REF: &str =
    "docs/release/artifact_verification_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_RELEASE_CENTER_FIXTURE_DIR: &str = "fixtures/ui/m5-release-center-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RELEASE_CENTER_ARTIFACT_REF: &str =
    "artifacts/release/m5-release-center-component-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_RELEASE_CENTER_CSV_REF: &str =
    "artifacts/release/m5-release-center-component-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_RELEASE_CENTER_REPORT_REF: &str =
    "artifacts/components/m5-release-center-components.md";

/// One of the six governed release-center component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseCenterComponentFamily {
    /// A release candidate card carrying candidate scope and blocker freshness.
    ReleaseCandidateCard,
    /// A version-bump row carrying the proposed bump class and compatibility
    /// impact.
    VersionBumpRow,
    /// A publish target row / review sheet naming target visibility, mutability,
    /// and auth source.
    PublishTargetRow,
    /// An artifact provenance bundle card carrying signature, attestation, SBOM,
    /// and digest-lineage truth.
    ArtifactProvenanceBundleCard,
    /// A promotion timeline step naming its rollout ring and stage state.
    PromotionTimelineStep,
    /// A rollback / revocation row naming its blast radius and revocation scope.
    RollbackRevocationRow,
}

impl M5ReleaseCenterComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReleaseCandidateCard,
        Self::VersionBumpRow,
        Self::PublishTargetRow,
        Self::ArtifactProvenanceBundleCard,
        Self::PromotionTimelineStep,
        Self::RollbackRevocationRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCandidateCard => "release_candidate_card",
            Self::VersionBumpRow => "version_bump_row",
            Self::PublishTargetRow => "publish_target_row",
            Self::ArtifactProvenanceBundleCard => "artifact_provenance_bundle_card",
            Self::PromotionTimelineStep => "promotion_timeline_step",
            Self::RollbackRevocationRow => "rollback_revocation_row",
        }
    }

    /// `true` when this family is a release candidate card and must therefore
    /// declare its candidate scope classes and blocker states.
    pub const fn is_candidate(self) -> bool {
        matches!(self, Self::ReleaseCandidateCard)
    }

    /// `true` when this family is a version-bump row and must therefore declare its
    /// version-bump classes and compatibility impacts.
    pub const fn is_version_bump(self) -> bool {
        matches!(self, Self::VersionBumpRow)
    }

    /// `true` when this family is a publish target row and must therefore declare
    /// its target visibilities, mutabilities, auth sources, and dry-run
    /// availabilities.
    pub const fn is_publish_target(self) -> bool {
        matches!(self, Self::PublishTargetRow)
    }

    /// `true` when this family is a provenance bundle card and must therefore
    /// declare its signature, attestation, SBOM, and digest-lineage vocabulary.
    pub const fn is_provenance(self) -> bool {
        matches!(self, Self::ArtifactProvenanceBundleCard)
    }

    /// `true` when this family is a promotion timeline step and must therefore
    /// declare its rollout rings and promotion stage states.
    pub const fn is_promotion(self) -> bool {
        matches!(self, Self::PromotionTimelineStep)
    }

    /// `true` when this family is a rollback / revocation row and must therefore
    /// declare its blast radii and revocation scopes.
    pub const fn is_rollback(self) -> bool {
        matches!(self, Self::RollbackRevocationRow)
    }
}

/// Controlled candidate scope class — how wide a release candidate reaches, so a
/// candidate card never leaves its scope implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CandidateScopeClass {
    /// One feature family only.
    SingleFamilyCandidate,
    /// Several feature families together.
    MultiFamilyCandidate,
    /// A full release train.
    FullTrainCandidate,
    /// An emergency hotfix candidate.
    HotfixCandidate,
    /// A supported-line backport candidate.
    BackportLineCandidate,
    /// A preview / experimental channel candidate.
    PreviewChannelCandidate,
}

impl M5CandidateScopeClass {
    /// Every candidate scope class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleFamilyCandidate,
        Self::MultiFamilyCandidate,
        Self::FullTrainCandidate,
        Self::HotfixCandidate,
        Self::BackportLineCandidate,
        Self::PreviewChannelCandidate,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleFamilyCandidate => "single_family_candidate",
            Self::MultiFamilyCandidate => "multi_family_candidate",
            Self::FullTrainCandidate => "full_train_candidate",
            Self::HotfixCandidate => "hotfix_candidate",
            Self::BackportLineCandidate => "backport_line_candidate",
            Self::PreviewChannelCandidate => "preview_channel_candidate",
        }
    }
}

/// Controlled candidate blocker state — the blocker posture of a candidate, so a
/// candidate card never shows a stale or unknown blocker state as clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CandidateBlockerState {
    /// No blockers are open.
    NoBlockers,
    /// Only soft (non-gating) blockers are open.
    SoftBlockersOnly,
    /// A hard, promotion-gating blocker is open.
    HardBlockerOpen,
    /// A blocker is held under a disclosed waiver.
    BlockerWaived,
    /// A blocker was resolved but is pending re-verification.
    BlockerResolvedPendingReverify,
    /// The blocker state is unknown / not yet evaluated.
    BlockerStateUnknown,
}

impl M5CandidateBlockerState {
    /// Every candidate blocker state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoBlockers,
        Self::SoftBlockersOnly,
        Self::HardBlockerOpen,
        Self::BlockerWaived,
        Self::BlockerResolvedPendingReverify,
        Self::BlockerStateUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoBlockers => "no_blockers",
            Self::SoftBlockersOnly => "soft_blockers_only",
            Self::HardBlockerOpen => "hard_blocker_open",
            Self::BlockerWaived => "blocker_waived",
            Self::BlockerResolvedPendingReverify => "blocker_resolved_pending_reverify",
            Self::BlockerStateUnknown => "blocker_state_unknown",
        }
    }
}

/// Controlled version-bump class — the kind of version change a bump row proposes,
/// so the bump magnitude is always explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VersionBumpClass {
    /// A major version bump.
    Major,
    /// A minor version bump.
    Minor,
    /// A patch version bump.
    Patch,
    /// A prerelease version bump.
    Prerelease,
    /// A build-metadata-only change.
    BuildMetadataOnly,
    /// A republish with no version change.
    RepublishNoVersionChange,
}

impl M5VersionBumpClass {
    /// Every version-bump class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Major,
        Self::Minor,
        Self::Patch,
        Self::Prerelease,
        Self::BuildMetadataOnly,
        Self::RepublishNoVersionChange,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Patch => "patch",
            Self::Prerelease => "prerelease",
            Self::BuildMetadataOnly => "build_metadata_only",
            Self::RepublishNoVersionChange => "republish_no_version_change",
        }
    }
}

/// Controlled compatibility impact — the compatibility consequence a bump carries,
/// so a version-bump row never hides a breaking change behind a version number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityImpact {
    /// Backward compatible.
    BackwardCompatible,
    /// A breaking change.
    BreakingChange,
    /// Forward incompatible (older readers cannot read newer output).
    ForwardIncompatible,
    /// Runtime behavior changes only, no interface change.
    RuntimeBehaviorOnly,
    /// A schema migration is required.
    SchemaMigrationRequired,
}

impl M5CompatibilityImpact {
    /// Every compatibility impact, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::BackwardCompatible,
        Self::BreakingChange,
        Self::ForwardIncompatible,
        Self::RuntimeBehaviorOnly,
        Self::SchemaMigrationRequired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackwardCompatible => "backward_compatible",
            Self::BreakingChange => "breaking_change",
            Self::ForwardIncompatible => "forward_incompatible",
            Self::RuntimeBehaviorOnly => "runtime_behavior_only",
            Self::SchemaMigrationRequired => "schema_migration_required",
        }
    }
}

/// Controlled publish-target visibility — how visible a publish target is, so a
/// publish-target row never implies a target is more or less public than it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublishTargetVisibility {
    /// Publicly listed and discoverable.
    PublicListed,
    /// Public but unlisted (reachable only by exact reference).
    PublicUnlisted,
    /// Private to a tenant / organization.
    PrivateTenant,
    /// Internal-only, never externally reachable.
    InternalOnly,
    /// Replicated to a mirror.
    MirrorReplicated,
}

impl M5PublishTargetVisibility {
    /// Every publish-target visibility, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PublicListed,
        Self::PublicUnlisted,
        Self::PrivateTenant,
        Self::InternalOnly,
        Self::MirrorReplicated,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicListed => "public_listed",
            Self::PublicUnlisted => "public_unlisted",
            Self::PrivateTenant => "private_tenant",
            Self::InternalOnly => "internal_only",
            Self::MirrorReplicated => "mirror_replicated",
        }
    }
}

/// Controlled target mutability — whether a published artifact at a target can
/// change, so a publish-target row never shows a mutable target as immutable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TargetMutability {
    /// Immutable once published.
    ImmutableOncePublished,
    /// A mutable tag that can be repointed.
    MutableTagRepointable,
    /// Overwrite of an existing version is allowed.
    OverwriteAllowed,
    /// Retraction / yanking is allowed.
    RetractionAllowed,
    /// Append-only (new versions only, no overwrite).
    AppendOnly,
}

impl M5TargetMutability {
    /// Every target mutability, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ImmutableOncePublished,
        Self::MutableTagRepointable,
        Self::OverwriteAllowed,
        Self::RetractionAllowed,
        Self::AppendOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImmutableOncePublished => "immutable_once_published",
            Self::MutableTagRepointable => "mutable_tag_repointable",
            Self::OverwriteAllowed => "overwrite_allowed",
            Self::RetractionAllowed => "retraction_allowed",
            Self::AppendOnly => "append_only",
        }
    }
}

/// Controlled target auth source — the identity authorized to publish to a target,
/// so a publish-target row never masks who or what is authorized to publish.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TargetAuthSource {
    /// A CI federated (workload) identity.
    CiFederatedIdentity,
    /// An individual maintainer key.
    MaintainerKey,
    /// An organization-managed identity.
    OrgManagedIdentity,
    /// A hardware-token signer.
    HardwareTokenSigner,
    /// A delegated bot / automation identity.
    DelegatedBotIdentity,
    /// An unauthenticated mirror (read-only replication, no publish identity).
    UnauthenticatedMirror,
}

impl M5TargetAuthSource {
    /// Every target auth source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CiFederatedIdentity,
        Self::MaintainerKey,
        Self::OrgManagedIdentity,
        Self::HardwareTokenSigner,
        Self::DelegatedBotIdentity,
        Self::UnauthenticatedMirror,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CiFederatedIdentity => "ci_federated_identity",
            Self::MaintainerKey => "maintainer_key",
            Self::OrgManagedIdentity => "org_managed_identity",
            Self::HardwareTokenSigner => "hardware_token_signer",
            Self::DelegatedBotIdentity => "delegated_bot_identity",
            Self::UnauthenticatedMirror => "unauthenticated_mirror",
        }
    }
}

/// Controlled dry-run availability — whether a publish can be previewed without
/// mutating the target, so a publish-target row is honest about preview support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DryRunAvailability {
    /// A full dry-run preview is supported.
    DryRunSupported,
    /// A partial dry-run preview is supported.
    DryRunPartial,
    /// No dry-run preview is available.
    DryRunUnavailable,
    /// A dry-run is required before any real publish.
    DryRunRequiredBeforePublish,
}

impl M5DryRunAvailability {
    /// Every dry-run availability, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DryRunSupported,
        Self::DryRunPartial,
        Self::DryRunUnavailable,
        Self::DryRunRequiredBeforePublish,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DryRunSupported => "dry_run_supported",
            Self::DryRunPartial => "dry_run_partial",
            Self::DryRunUnavailable => "dry_run_unavailable",
            Self::DryRunRequiredBeforePublish => "dry_run_required_before_publish",
        }
    }
}

/// Controlled signature status — the signing posture of an artifact bundle, so a
/// provenance card never shows an unsigned or broken signature as verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SignatureStatus {
    /// Signed and the signature verified against a trusted key.
    SignedVerified,
    /// Signed, but the signing key is not yet verified / trusted.
    SignedUnverifiedKey,
    /// Unsigned.
    Unsigned,
    /// A signature is present but broken / does not verify.
    SignatureBroken,
    /// Signing is pending.
    SignaturePending,
}

impl M5SignatureStatus {
    /// Every signature status, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SignedVerified,
        Self::SignedUnverifiedKey,
        Self::Unsigned,
        Self::SignatureBroken,
        Self::SignaturePending,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedVerified => "signed_verified",
            Self::SignedUnverifiedKey => "signed_unverified_key",
            Self::Unsigned => "unsigned",
            Self::SignatureBroken => "signature_broken",
            Self::SignaturePending => "signature_pending",
        }
    }
}

/// Controlled attestation status — the build-attestation posture of an artifact
/// bundle, so a provenance card never claims unverified provenance as attested.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AttestationStatus {
    /// Attested and the attestation verified.
    AttestedVerified,
    /// Attested, but the attestation is not yet verified.
    AttestedUnverified,
    /// No attestation is present.
    NoAttestation,
    /// The attestation has expired.
    AttestationExpired,
    /// Attestation is pending.
    AttestationPending,
}

impl M5AttestationStatus {
    /// Every attestation status, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AttestedVerified,
        Self::AttestedUnverified,
        Self::NoAttestation,
        Self::AttestationExpired,
        Self::AttestationPending,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AttestedVerified => "attested_verified",
            Self::AttestedUnverified => "attested_unverified",
            Self::NoAttestation => "no_attestation",
            Self::AttestationExpired => "attestation_expired",
            Self::AttestationPending => "attestation_pending",
        }
    }
}

/// Controlled SBOM status — the software-bill-of-materials posture of an artifact
/// bundle, so a provenance card never shows a missing or partial SBOM as complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SbomStatus {
    /// A complete SBOM is attached.
    SbomComplete,
    /// A partial SBOM is attached.
    SbomPartial,
    /// No SBOM is attached.
    SbomMissing,
    /// The SBOM is stale relative to the built artifact.
    SbomStale,
    /// The SBOM is being generated.
    SbomGenerating,
}

impl M5SbomStatus {
    /// Every SBOM status, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SbomComplete,
        Self::SbomPartial,
        Self::SbomMissing,
        Self::SbomStale,
        Self::SbomGenerating,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SbomComplete => "sbom_complete",
            Self::SbomPartial => "sbom_partial",
            Self::SbomMissing => "sbom_missing",
            Self::SbomStale => "sbom_stale",
            Self::SbomGenerating => "sbom_generating",
        }
    }
}

/// Controlled digest-lineage state — the immutable-digest lineage posture of an
/// artifact bundle, so a provenance card never hides a broken or unverified
/// lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DigestLineageState {
    /// The immutable digest is pinned.
    ImmutableDigestPinned,
    /// The digest lineage is continuous back to source.
    DigestLineageContinuous,
    /// The digest lineage is broken.
    DigestLineageBroken,
    /// The digest is not yet verified.
    DigestUnverified,
    /// A clean-room rebuild reproduced the digest exactly.
    RebuildDigestMatched,
}

impl M5DigestLineageState {
    /// Every digest-lineage state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ImmutableDigestPinned,
        Self::DigestLineageContinuous,
        Self::DigestLineageBroken,
        Self::DigestUnverified,
        Self::RebuildDigestMatched,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImmutableDigestPinned => "immutable_digest_pinned",
            Self::DigestLineageContinuous => "digest_lineage_continuous",
            Self::DigestLineageBroken => "digest_lineage_broken",
            Self::DigestUnverified => "digest_unverified",
            Self::RebuildDigestMatched => "rebuild_digest_matched",
        }
    }
}

/// Controlled rollout ring — the promotion ring a timeline step targets, so a
/// promotion timeline step never leaves its rollout ring implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RolloutRing {
    /// A canary ring.
    CanaryRing,
    /// A pilot ring.
    PilotRing,
    /// An early-access ring.
    EarlyAccessRing,
    /// A broad ring.
    BroadRing,
    /// General availability.
    GeneralAvailability,
    /// Held, not promoted to any ring.
    HeldNotPromoted,
}

impl M5RolloutRing {
    /// Every rollout ring, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CanaryRing,
        Self::PilotRing,
        Self::EarlyAccessRing,
        Self::BroadRing,
        Self::GeneralAvailability,
        Self::HeldNotPromoted,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanaryRing => "canary_ring",
            Self::PilotRing => "pilot_ring",
            Self::EarlyAccessRing => "early_access_ring",
            Self::BroadRing => "broad_ring",
            Self::GeneralAvailability => "general_availability",
            Self::HeldNotPromoted => "held_not_promoted",
        }
    }
}

/// Controlled promotion stage state — the state of a promotion timeline step, so a
/// promotion timeline never shows a blocked or rolled-back stage as promoted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PromotionStageState {
    /// The stage is pending.
    StagePending,
    /// The stage is in progress.
    StageInProgress,
    /// The stage promoted successfully.
    StagePromoted,
    /// The stage is blocked.
    StageBlocked,
    /// The stage was rolled back.
    StageRolledBack,
}

impl M5PromotionStageState {
    /// Every promotion stage state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::StagePending,
        Self::StageInProgress,
        Self::StagePromoted,
        Self::StageBlocked,
        Self::StageRolledBack,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StagePending => "stage_pending",
            Self::StageInProgress => "stage_in_progress",
            Self::StagePromoted => "stage_promoted",
            Self::StageBlocked => "stage_blocked",
            Self::StageRolledBack => "stage_rolled_back",
        }
    }
}

/// Controlled rollback blast radius — how far a rollback / revocation reaches, so a
/// rollback row never understates what a rollback will affect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackBlastRadius {
    /// A single artifact.
    SingleArtifact,
    /// One feature family.
    FamilyScoped,
    /// One release train.
    TrainScoped,
    /// Multiple trains.
    CrossTrainScoped,
    /// The whole fleet.
    FleetWide,
}

impl M5RollbackBlastRadius {
    /// Every rollback blast radius, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SingleArtifact,
        Self::FamilyScoped,
        Self::TrainScoped,
        Self::CrossTrainScoped,
        Self::FleetWide,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleArtifact => "single_artifact",
            Self::FamilyScoped => "family_scoped",
            Self::TrainScoped => "train_scoped",
            Self::CrossTrainScoped => "cross_train_scoped",
            Self::FleetWide => "fleet_wide",
        }
    }
}

/// Controlled revocation scope — what a rollback / revocation actually revokes, so
/// a rollback row is honest about whether trust material is being rotated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RevocationScope {
    /// No revocation (a soft rollback only).
    NoRevocation,
    /// A mutable tag is repointed to an earlier version only.
    TagRepointOnly,
    /// The artifact is revoked / yanked.
    ArtifactRevoked,
    /// A signing key is revoked.
    SigningKeyRevoked,
    /// The trust root is rotated.
    TrustRootRotated,
}

impl M5RevocationScope {
    /// Every revocation scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoRevocation,
        Self::TagRepointOnly,
        Self::ArtifactRevoked,
        Self::SigningKeyRevoked,
        Self::TrustRootRotated,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRevocation => "no_revocation",
            Self::TagRepointOnly => "tag_repoint_only",
            Self::ArtifactRevoked => "artifact_revoked",
            Self::SigningKeyRevoked => "signing_key_revoked",
            Self::TrustRootRotated => "trust_root_rotated",
        }
    }
}

/// Claimed M5 publication surface family that renders / consumes a release-center
/// component. This is the release-center analog of the shell-zone surface family:
/// no component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationSurfaceFamily {
    /// The release-center / shiproom surface.
    ReleaseCenter,
    /// The update-center surface.
    UpdateCenter,
    /// Registry publication surfaces.
    RegistryPublication,
    /// Mirror publication surfaces.
    MirrorPublication,
    /// Enterprise evaluation / procurement surfaces.
    EnterpriseEvaluation,
    /// Support-desk surfaces.
    SupportDesk,
    /// Docs / Help surfaces.
    DocsHelp,
    /// Admin review surfaces.
    AdminReview,
}

impl M5PublicationSurfaceFamily {
    /// Every publication surface family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ReleaseCenter,
        Self::UpdateCenter,
        Self::RegistryPublication,
        Self::MirrorPublication,
        Self::EnterpriseEvaluation,
        Self::SupportDesk,
        Self::DocsHelp,
        Self::AdminReview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::UpdateCenter => "update_center",
            Self::RegistryPublication => "registry_publication",
            Self::MirrorPublication => "mirror_publication",
            Self::EnterpriseEvaluation => "enterprise_evaluation",
            Self::SupportDesk => "support_desk",
            Self::DocsHelp => "docs_help",
            Self::AdminReview => "admin_review",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's
/// scope never silently narrows or widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5DeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Release-center subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseCenterConsumerSurface {
    /// The release-center UI.
    ReleaseCenterUi,
    /// The Help / About surface.
    HelpAbout,
    /// The service-health surface.
    ServiceHealth,
    /// The docs portal.
    DocsPortal,
    /// The admin console.
    AdminConsole,
    /// The evaluation pack.
    EvaluationPack,
    /// The mirror console.
    MirrorConsole,
    /// The support export.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The general product UI.
    ProductUi,
}

impl M5ReleaseCenterConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ReleaseCenterUi,
        Self::HelpAbout,
        Self::ServiceHealth,
        Self::DocsPortal,
        Self::AdminConsole,
        Self::EvaluationPack,
        Self::MirrorConsole,
        Self::SupportExport,
        Self::CliInspect,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenterUi => "release_center_ui",
            Self::HelpAbout => "help_about",
            Self::ServiceHealth => "service_health",
            Self::DocsPortal => "docs_portal",
            Self::AdminConsole => "admin_console",
            Self::EvaluationPack => "evaluation_pack",
            Self::MirrorConsole => "mirror_console",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no release truth
/// is hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseCenterAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5ReleaseCenterAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed release-center component must be able to show. The
/// first three are hard requirements on every component; the remaining three close
/// the acceptance-criteria ambiguity about evidence freshness, target auth source,
/// and rollback vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseCenterRequiredLabel {
    /// The component's stable identity / what release object it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The evidence-freshness reading behind the component's claim.
    EvidenceFreshness,
    /// The auth source authorized for the component's target / action.
    AuthSource,
    /// The rollback / revocation vocabulary for the component's action.
    RollbackVocabulary,
}

impl M5ReleaseCenterRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::EvidenceFreshness,
        Self::AuthSource,
        Self::RollbackVocabulary,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::EvidenceFreshness => "evidence_freshness",
            Self::AuthSource => "auth_source",
            Self::RollbackVocabulary => "rollback_vocabulary",
        }
    }
}

/// Qualification class for an M5 release-center-component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseCenterQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5ReleaseCenterQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a release-center component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseCenterDowngradeTrigger {
    /// A candidate card left its scope unstated.
    CandidateScopeUnstated,
    /// A candidate card hid blocker freshness.
    BlockerFreshnessHidden,
    /// A version-bump row left the compatibility impact unstated.
    VersionBumpImpactUnstated,
    /// A publish-target row masked the target auth source.
    TargetAuthSourceMasked,
    /// A publish-target row hid target mutability.
    TargetMutabilityHidden,
    /// A publish-target row left dry-run availability unstated.
    DryRunAvailabilityUnstated,
    /// A provenance card showed an unverified signature or attestation as clean.
    SignatureOrAttestationOverclaimed,
    /// A provenance card overstated SBOM completeness.
    SbomCompletenessOverstated,
    /// A provenance card hid a broken digest lineage.
    DigestLineageBrokenHidden,
    /// A promotion timeline step left its rollout ring unstated.
    RolloutRingUnstated,
    /// A rollback row understated its blast radius.
    RollbackBlastRadiusUnderstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5ReleaseCenterDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::CandidateScopeUnstated,
        Self::BlockerFreshnessHidden,
        Self::VersionBumpImpactUnstated,
        Self::TargetAuthSourceMasked,
        Self::TargetMutabilityHidden,
        Self::DryRunAvailabilityUnstated,
        Self::SignatureOrAttestationOverclaimed,
        Self::SbomCompletenessOverstated,
        Self::DigestLineageBrokenHidden,
        Self::RolloutRingUnstated,
        Self::RollbackBlastRadiusUnderstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CandidateScopeUnstated => "candidate_scope_unstated",
            Self::BlockerFreshnessHidden => "blocker_freshness_hidden",
            Self::VersionBumpImpactUnstated => "version_bump_impact_unstated",
            Self::TargetAuthSourceMasked => "target_auth_source_masked",
            Self::TargetMutabilityHidden => "target_mutability_hidden",
            Self::DryRunAvailabilityUnstated => "dry_run_availability_unstated",
            Self::SignatureOrAttestationOverclaimed => "signature_or_attestation_overclaimed",
            Self::SbomCompletenessOverstated => "sbom_completeness_overstated",
            Self::DigestLineageBrokenHidden => "digest_lineage_broken_hidden",
            Self::RolloutRingUnstated => "rollout_ring_unstated",
            Self::RollbackBlastRadiusUnderstated => "rollback_blast_radius_understated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed release-center component family bound to the
/// surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCenterComponentRow {
    /// Governed component family.
    pub component_family: M5ReleaseCenterComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5ReleaseCenterQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 publication surface families that render / consume this
    /// component.
    pub surface_families: Vec<M5PublicationSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5ReleaseCenterRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5ReleaseCenterRequiredLabel>,
    /// Candidate scope classes this component names (candidate only).
    pub candidate_scope_classes: Vec<M5CandidateScopeClass>,
    /// Candidate blocker states this component distinguishes (candidate only).
    pub candidate_blocker_states: Vec<M5CandidateBlockerState>,
    /// Version-bump classes this component names (version-bump only).
    pub version_bump_classes: Vec<M5VersionBumpClass>,
    /// Compatibility impacts this component discloses (version-bump only).
    pub compatibility_impacts: Vec<M5CompatibilityImpact>,
    /// Publish-target visibilities this component names (publish-target only).
    pub target_visibilities: Vec<M5PublishTargetVisibility>,
    /// Publish-target mutabilities this component names (publish-target only).
    pub target_mutabilities: Vec<M5TargetMutability>,
    /// Publish-target auth sources this component names (publish-target only).
    pub target_auth_sources: Vec<M5TargetAuthSource>,
    /// Dry-run availabilities this component discloses (publish-target only).
    pub dry_run_availabilities: Vec<M5DryRunAvailability>,
    /// Signature statuses this component distinguishes (provenance only).
    pub signature_statuses: Vec<M5SignatureStatus>,
    /// Attestation statuses this component distinguishes (provenance only).
    pub attestation_statuses: Vec<M5AttestationStatus>,
    /// SBOM statuses this component distinguishes (provenance only).
    pub sbom_statuses: Vec<M5SbomStatus>,
    /// Digest-lineage states this component distinguishes (provenance only).
    pub digest_lineage_states: Vec<M5DigestLineageState>,
    /// Rollout rings this component names (promotion only).
    pub rollout_rings: Vec<M5RolloutRing>,
    /// Promotion stage states this component distinguishes (promotion only).
    pub promotion_stage_states: Vec<M5PromotionStageState>,
    /// Rollback blast radii this component discloses (rollback only).
    pub rollback_blast_radii: Vec<M5RollbackBlastRadius>,
    /// Revocation scopes this component discloses (rollback only).
    pub revocation_scopes: Vec<M5RevocationScope>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5ReleaseCenterAccessibilityRoute>,
    /// Release-center subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5ReleaseCenterConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5ReleaseCenterDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks a target auth source or
    /// mutability. MUST be `false`.
    pub masks_target_auth_source_or_mutability: bool,
    /// Hard invariant: this component never conflates signed with unsigned
    /// provenance. MUST be `false`.
    pub conflates_signed_and_unsigned_provenance: bool,
    /// Hard invariant: this component never invents a private release-status
    /// grammar. MUST be `false`.
    pub invents_private_release_status_grammar: bool,
    /// Hard invariant: this component never overstates rollback reversibility or
    /// drops evidence-freshness truth. MUST be `false`.
    pub overstates_rollback_reversibility_or_drops_evidence_freshness: bool,
}

impl M5ReleaseCenterComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ReleaseCenterRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ReleaseCenterRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_target_auth_source_or_mutability
            && !self.conflates_signed_and_unsigned_provenance
            && !self.invents_private_release_status_grammar
            && !self.overstates_rollback_reversibility_or_drops_evidence_freshness
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCenterVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Candidate-scope-class tokens.
    pub candidate_scope_classes: Vec<String>,
    /// Candidate-blocker-state tokens.
    pub candidate_blocker_states: Vec<String>,
    /// Version-bump-class tokens.
    pub version_bump_classes: Vec<String>,
    /// Compatibility-impact tokens.
    pub compatibility_impacts: Vec<String>,
    /// Publish-target-visibility tokens.
    pub target_visibilities: Vec<String>,
    /// Target-mutability tokens.
    pub target_mutabilities: Vec<String>,
    /// Target-auth-source tokens.
    pub target_auth_sources: Vec<String>,
    /// Dry-run-availability tokens.
    pub dry_run_availabilities: Vec<String>,
    /// Signature-status tokens.
    pub signature_statuses: Vec<String>,
    /// Attestation-status tokens.
    pub attestation_statuses: Vec<String>,
    /// SBOM-status tokens.
    pub sbom_statuses: Vec<String>,
    /// Digest-lineage-state tokens.
    pub digest_lineage_states: Vec<String>,
    /// Rollout-ring tokens.
    pub rollout_rings: Vec<String>,
    /// Promotion-stage-state tokens.
    pub promotion_stage_states: Vec<String>,
    /// Rollback-blast-radius tokens.
    pub rollback_blast_radii: Vec<String>,
    /// Revocation-scope tokens.
    pub revocation_scopes: Vec<String>,
    /// Publication-surface-family tokens.
    pub publication_surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5ReleaseCenterVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5ReleaseCenterComponentFamily::ALL, |v| v.as_str()),
            candidate_scope_classes: tokens(&M5CandidateScopeClass::ALL, |v| v.as_str()),
            candidate_blocker_states: tokens(&M5CandidateBlockerState::ALL, |v| v.as_str()),
            version_bump_classes: tokens(&M5VersionBumpClass::ALL, |v| v.as_str()),
            compatibility_impacts: tokens(&M5CompatibilityImpact::ALL, |v| v.as_str()),
            target_visibilities: tokens(&M5PublishTargetVisibility::ALL, |v| v.as_str()),
            target_mutabilities: tokens(&M5TargetMutability::ALL, |v| v.as_str()),
            target_auth_sources: tokens(&M5TargetAuthSource::ALL, |v| v.as_str()),
            dry_run_availabilities: tokens(&M5DryRunAvailability::ALL, |v| v.as_str()),
            signature_statuses: tokens(&M5SignatureStatus::ALL, |v| v.as_str()),
            attestation_statuses: tokens(&M5AttestationStatus::ALL, |v| v.as_str()),
            sbom_statuses: tokens(&M5SbomStatus::ALL, |v| v.as_str()),
            digest_lineage_states: tokens(&M5DigestLineageState::ALL, |v| v.as_str()),
            rollout_rings: tokens(&M5RolloutRing::ALL, |v| v.as_str()),
            promotion_stage_states: tokens(&M5PromotionStageState::ALL, |v| v.as_str()),
            rollback_blast_radii: tokens(&M5RollbackBlastRadius::ALL, |v| v.as_str()),
            revocation_scopes: tokens(&M5RevocationScope::ALL, |v| v.as_str()),
            publication_surface_families: tokens(&M5PublicationSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5DeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ReleaseCenterConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ReleaseCenterAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ReleaseCenterRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5ReleaseCenterGovernanceReview {
    /// The candidate card shows candidate scope and blocker freshness.
    pub candidate_card_shows_scope_and_blocker_freshness: bool,
    /// The version-bump row shows its compatibility impact.
    pub version_bump_row_shows_compatibility_impact: bool,
    /// The publish-target row shows auth source and mutability.
    pub publish_target_row_shows_auth_source_and_mutability: bool,
    /// The provenance card shows signature, attestation, and SBOM truth.
    pub provenance_card_shows_signature_attestation_sbom: bool,
    /// The promotion timeline shows rollout ring and stage state.
    pub promotion_timeline_shows_ring_and_stage: bool,
    /// The rollback row shows blast radius and revocation scope.
    pub rollback_row_shows_blast_radius_and_revocation_scope: bool,
    /// Signed and unsigned provenance are never conflated.
    pub signed_versus_unsigned_never_conflated: bool,
    /// No component invents a second release-status grammar.
    pub no_component_invents_second_status_grammar: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel release-center vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCenterConsumerProjection {
    /// Candidate and version surfaces consume the shared candidate/bump vocabulary.
    pub candidate_and_version_surfaces_consume_matrix: bool,
    /// Publish-target surfaces consume the auth-source / mutability vocabulary.
    pub publish_target_surfaces_consume_auth_vocabulary: bool,
    /// Provenance surfaces consume the signature / attestation / SBOM vocabulary.
    pub provenance_surfaces_consume_signature_vocabulary: bool,
    /// Promotion and rollback surfaces consume the ring / blast-radius vocabulary.
    pub promotion_and_rollback_surfaces_consume_ring_and_blast_vocabulary: bool,
    /// Support / export reads a single canonical release-center source.
    pub support_export_reads_single_source: bool,
    /// Evaluation and mirror surfaces read a single canonical release-center
    /// source.
    pub evaluation_and_mirror_surfaces_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCenterProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the release-center lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCenterReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting release-center audit for the lane.
    pub release_center_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ReleaseCenterMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReleaseCenterMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ReleaseCenterComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ReleaseCenterVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ReleaseCenterGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReleaseCenterConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReleaseCenterProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ReleaseCenterReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 release-center-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCenterMatrixPacket {
    /// Record kind; must equal [`M5_RELEASE_CENTER_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RELEASE_CENTER_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ReleaseCenterComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ReleaseCenterVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ReleaseCenterGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReleaseCenterConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReleaseCenterProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ReleaseCenterReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ReleaseCenterMatrixPacket {
    /// Builds an M5 release-center-component matrix packet from stable-lane input.
    pub fn new(input: M5ReleaseCenterMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_RELEASE_CENTER_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_RELEASE_CENTER_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 release-center-component matrix invariants.
    pub fn validate(&self) -> Vec<M5ReleaseCenterMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RELEASE_CENTER_MATRIX_RECORD_KIND {
            violations.push(M5ReleaseCenterMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RELEASE_CENTER_MATRIX_SCHEMA_VERSION {
            violations.push(M5ReleaseCenterMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ReleaseCenterMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 release center matrix packet serializes"),
        ) {
            violations.push(M5ReleaseCenterMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 release center matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Release-Candidate-Card, Version-Bump-Row, Publish-Target-Row, Artifact-Provenance-Bundle-Card, and Promotion-Timeline Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Target auth sources: {}\n",
            self.vocabulary_set.target_auth_sources.join(", ")
        ));
        out.push_str(&format!(
            "- Rollback blast radii: {}\n",
            self.vocabulary_set.rollback_blast_radii.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 release-center matrix export.
#[derive(Debug)]
pub enum M5ReleaseCenterMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ReleaseCenterMatrixViolation>),
}

impl fmt::Display for M5ReleaseCenterMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 release center matrix export parse failed: {error}"
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
                    "m5 release center matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ReleaseCenterMatrixArtifactError {}

/// Validation failures emitted by [`M5ReleaseCenterMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ReleaseCenterMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A candidate component declares no candidate scope classes.
    CandidateScopeClassMissing,
    /// A candidate component declares no blocker states.
    CandidateBlockerStateMissing,
    /// A version-bump component declares no version-bump classes.
    VersionBumpClassMissing,
    /// A version-bump component declares no compatibility impacts.
    CompatibilityImpactMissing,
    /// A publish-target component declares no target visibilities.
    TargetVisibilityMissing,
    /// A publish-target component declares no target mutabilities.
    TargetMutabilityMissing,
    /// A publish-target component declares no target auth sources.
    TargetAuthSourceMissing,
    /// A publish-target component declares no dry-run availabilities.
    DryRunAvailabilityMissing,
    /// A provenance component declares no signature statuses.
    SignatureStatusMissing,
    /// A provenance component declares no attestation statuses.
    AttestationStatusMissing,
    /// A provenance component declares no SBOM statuses.
    SbomStatusMissing,
    /// A provenance component declares no digest-lineage states.
    DigestLineageStateMissing,
    /// A promotion component declares no rollout rings.
    RolloutRingMissing,
    /// A promotion component declares no promotion stage states.
    PromotionStageStateMissing,
    /// A rollback component declares no blast radii.
    RollbackBlastRadiusMissing,
    /// A rollback component declares no revocation scopes.
    RevocationScopeMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (masked auth/mutability, conflated
    /// signed/unsigned provenance, private status grammar, or overstated rollback
    /// reversibility / dropped evidence freshness).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ReleaseCenterMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::CandidateScopeClassMissing => "candidate_scope_class_missing",
            Self::CandidateBlockerStateMissing => "candidate_blocker_state_missing",
            Self::VersionBumpClassMissing => "version_bump_class_missing",
            Self::CompatibilityImpactMissing => "compatibility_impact_missing",
            Self::TargetVisibilityMissing => "target_visibility_missing",
            Self::TargetMutabilityMissing => "target_mutability_missing",
            Self::TargetAuthSourceMissing => "target_auth_source_missing",
            Self::DryRunAvailabilityMissing => "dry_run_availability_missing",
            Self::SignatureStatusMissing => "signature_status_missing",
            Self::AttestationStatusMissing => "attestation_status_missing",
            Self::SbomStatusMissing => "sbom_status_missing",
            Self::DigestLineageStateMissing => "digest_lineage_state_missing",
            Self::RolloutRingMissing => "rollout_ring_missing",
            Self::PromotionStageStateMissing => "promotion_stage_state_missing",
            Self::RollbackBlastRadiusMissing => "rollback_blast_radius_missing",
            Self::RevocationScopeMissing => "revocation_scope_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 release-center matrix export.
pub fn current_stable_m5_release_center_component_matrix_export(
) -> Result<M5ReleaseCenterMatrixPacket, M5ReleaseCenterMatrixArtifactError> {
    let packet: M5ReleaseCenterMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-release-center-component-proof/support_export.json"
    )))
    .map_err(M5ReleaseCenterMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ReleaseCenterMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ReleaseCenterMatrixPacket,
    violations: &mut Vec<M5ReleaseCenterMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RELEASE_CENTER_SCHEMA_REF,
        M5_RELEASE_CENTER_DOC_REF,
        M5_RELEASE_CENTER_OBJECT_MODEL_REF,
        M5_RELEASE_CENTER_ROLLBACK_CONTRACT_REF,
        M5_RELEASE_CENTER_PROVENANCE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ReleaseCenterMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ReleaseCenterMatrixPacket,
    violations: &mut Vec<M5ReleaseCenterMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ReleaseCenterMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5ReleaseCenterMatrixPacket,
    violations: &mut Vec<M5ReleaseCenterMatrixViolation>,
) {
    let present: BTreeSet<M5ReleaseCenterComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5ReleaseCenterComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5ReleaseCenterMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5ReleaseCenterMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5ReleaseCenterMatrixViolation::MandatoryLabelMissing);
        }
        if family.is_candidate() && row.candidate_scope_classes.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::CandidateScopeClassMissing);
        }
        if family.is_candidate() && row.candidate_blocker_states.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::CandidateBlockerStateMissing);
        }
        if family.is_version_bump() && row.version_bump_classes.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::VersionBumpClassMissing);
        }
        if family.is_version_bump() && row.compatibility_impacts.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::CompatibilityImpactMissing);
        }
        if family.is_publish_target() && row.target_visibilities.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::TargetVisibilityMissing);
        }
        if family.is_publish_target() && row.target_mutabilities.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::TargetMutabilityMissing);
        }
        if family.is_publish_target() && row.target_auth_sources.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::TargetAuthSourceMissing);
        }
        if family.is_publish_target() && row.dry_run_availabilities.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::DryRunAvailabilityMissing);
        }
        if family.is_provenance() && row.signature_statuses.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::SignatureStatusMissing);
        }
        if family.is_provenance() && row.attestation_statuses.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::AttestationStatusMissing);
        }
        if family.is_provenance() && row.sbom_statuses.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::SbomStatusMissing);
        }
        if family.is_provenance() && row.digest_lineage_states.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::DigestLineageStateMissing);
        }
        if family.is_promotion() && row.rollout_rings.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::RolloutRingMissing);
        }
        if family.is_promotion() && row.promotion_stage_states.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::PromotionStageStateMissing);
        }
        if family.is_rollback() && row.rollback_blast_radii.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::RollbackBlastRadiusMissing);
        }
        if family.is_rollback() && row.revocation_scopes.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::RevocationScopeMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ReleaseCenterMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ReleaseCenterMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ReleaseCenterMatrixPacket,
    violations: &mut Vec<M5ReleaseCenterMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.candidate_card_shows_scope_and_blocker_freshness,
        review.version_bump_row_shows_compatibility_impact,
        review.publish_target_row_shows_auth_source_and_mutability,
        review.provenance_card_shows_signature_attestation_sbom,
        review.promotion_timeline_shows_ring_and_stage,
        review.rollback_row_shows_blast_radius_and_revocation_scope,
        review.signed_versus_unsigned_never_conflated,
        review.no_component_invents_second_status_grammar,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ReleaseCenterMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ReleaseCenterMatrixPacket,
    violations: &mut Vec<M5ReleaseCenterMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.candidate_and_version_surfaces_consume_matrix,
        projection.publish_target_surfaces_consume_auth_vocabulary,
        projection.provenance_surfaces_consume_signature_vocabulary,
        projection.promotion_and_rollback_surfaces_consume_ring_and_blast_vocabulary,
        projection.support_export_reads_single_source,
        projection.evaluation_and_mirror_surfaces_read_single_source,
    ] {
        if !ok {
            violations.push(M5ReleaseCenterMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ReleaseCenterMatrixPacket,
    violations: &mut Vec<M5ReleaseCenterMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ReleaseCenterMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ReleaseCenterMatrixPacket,
    violations: &mut Vec<M5ReleaseCenterMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.release_center_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ReleaseCenterMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
