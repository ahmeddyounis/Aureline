//! Frozen M5 build-farm, cache-trust-domain, clean-room-rebuild, and exact-build-supportability matrix.
//!
//! This module locks Aureline's concrete build-lane trust domains, remote-cache discipline, clean-room
//! rebuild proof, and exact-build supportability into one export-safe packet. Every claimed M5 release lane
//! — the contributor / PR lane, the protected-merge lane, the release lane, and the emergency-hotfix lane —
//! is named once here and constrained by the same shared build-lane-trust-role taxonomy (cache_posture,
//! publication_authority, credential_boundary, hermetic_input, reproducibility_proof, artifact_convergence,
//! support_identity), the same contributor-lanes-read-caches-but-never-publish rule, the same
//! remote-cache-hits-are-never-reproducibility-proof rule, the same
//! docs-schema-sbom-and-symbol-sidecars-stay-pinned-to-the-binary-build-identity rule, the same
//! clean-room-parity-is-never-overclaimed-on-partial-rebuilds rule, and the same
//! non-hermetic-inputs-cache-poisoning-and-unreplayable-artifacts-block-promotion rule regardless of the
//! surface that renders it.
//!
//! The matrix does not redesign generic publish-target UI or marketing release notes — it is the shared
//! reusable build-lane, remote-cache, and clean-room proof engine contract those already-governed surfaces
//! consume, and it binds back to the already-landed artifact-publication and reproducible-RC packets instead
//! of leaving build-lane truth split across scattered CI prose. The controlled vocabularies are frozen in
//! one self-describing [`M5BuildLaneTrustVocabularySet`] rather than minted per surface. The single
//! controlled build-lane-trust-role vocabulary consumers bind to — cache_posture, publication_authority,
//! credential_boundary, hermetic_input, reproducibility_proof, artifact_convergence, and support_identity —
//! keeps contributor lanes reading shared caches but never publishing release artifacts; keeps
//! protected-merge lanes on controlled credentials and verified caches; keeps release and emergency-hotfix
//! lanes on verified or re-materialized inputs converging on one exact build identity; keeps remote-cache
//! hits from being treated as reproducibility proof; keeps docs, schema, SBOM, and symbol sidecars pinned to
//! the binary build identity; keeps clean-room parity honest on partial rebuilds; and keeps non-hermetic
//! inputs, cache poisoning, and unreplayable artifacts blocking promotion rather than hiding behind green
//! publication rows. Raw secret values and private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_build_lane_trust_matrix,
    seeded_m5_build_lane_trust_matrix_emergency_hotfix_preview_narrowed,
    seeded_m5_build_lane_trust_matrix_release_beta_narrowed, M5_BUILD_LANE_TRUST_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5BuildLaneTrustMatrixPacket`].
pub const M5_BUILD_LANE_TRUST_MATRIX_RECORD_KIND: &str =
    "freeze_m5_build_farm_cache_trust_clean_room_rebuild_and_exact_build_supportability_matrix";

/// Schema version for M5 build-lane-trust matrix records.
pub const M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined build-lane-trust matrix schema.
pub const M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF: &str =
    "schemas/release/m5-build-lane-trust-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BUILD_LANE_TRUST_MATRIX_DOC_REF: &str = "docs/release/m5_build_lane_trust_contract.md";

/// Repo-relative path of the canonical build-lane-descriptor domain schema (contributor / PR and
/// protected-merge lanes: allowed cache posture, publication authority, credential boundary, and
/// exact-build expectation of a lane).
pub const M5_BUILD_LANE_DESCRIPTOR_DOMAIN_SCHEMA_REF: &str =
    "schemas/release/m5-build-lane-descriptor.schema.json";

/// Repo-relative path of the canonical reproducibility-proof domain schema (release and emergency-hotfix
/// lanes: verified or re-materialized inputs, clean-room rebuild diff, and exact-build supportability
/// convergence on one build identity).
pub const M5_REPRODUCIBILITY_PROOF_DOMAIN_SCHEMA_REF: &str =
    "schemas/release/m5-reproducibility-proof.schema.json";

/// Repo-relative path of the already-landed artifact-publication row schema the matrix binds back to.
pub const M5_ARTIFACT_PUBLICATION_LANDED_SCHEMA_REF: &str =
    "schemas/release/artifact_publication_row.schema.json";

/// Repo-relative path of the already-landed reproducible-RC packet schema the build-lane-trust matrix binds
/// back to.
pub const M5_REPRODUCIBLE_RC_LANDED_SCHEMA_REF: &str =
    "schemas/release/reproducible_rc_packet.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_BUILD_LANE_TRUST_FIXTURE_DIR: &str = "fixtures/release/m5-clean-room-rebuild";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BUILD_LANE_TRUST_ARTIFACT_REF: &str =
    "artifacts/release/m5-exact-build-supportability-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_BUILD_LANE_TRUST_CSV_REF: &str =
    "artifacts/release/m5-exact-build-supportability-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_BUILD_LANE_TRUST_REPORT_REF: &str = "artifacts/release/m5-build-lane-trust-matrix.md";

/// One of the four governed build lanes this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneFamily {
    /// The contributor / PR lane: may read shared caches but never publishes release artifacts.
    ContributorPr,
    /// The protected-merge lane: controlled credentials and verified caches only.
    ProtectedMerge,
    /// The release lane: verified or re-materialized inputs converging on one exact build identity.
    Release,
    /// The emergency-hotfix lane: expedited yet still verified inputs and one exact build identity.
    EmergencyHotfix,
}

impl M5BuildLaneFamily {
    /// Every governed build lane, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ContributorPr,
        Self::ProtectedMerge,
        Self::Release,
        Self::EmergencyHotfix,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContributorPr => "contributor_pr",
            Self::ProtectedMerge => "protected_merge",
            Self::Release => "release",
            Self::EmergencyHotfix => "emergency_hotfix",
        }
    }

    /// The canonical per-domain schema ref a downstream surface points at instead of restating this lane's
    /// build-lane-descriptor or reproducibility-proof meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::ContributorPr | Self::ProtectedMerge => {
                M5_BUILD_LANE_DESCRIPTOR_DOMAIN_SCHEMA_REF
            }
            Self::Release | Self::EmergencyHotfix => M5_REPRODUCIBILITY_PROOF_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this lane must name a controlled contributor / PR role.
    pub const fn declares_contributor_pr_roles(self) -> bool {
        matches!(self, Self::ContributorPr)
    }

    /// `true` when this lane must name a controlled protected-merge role.
    pub const fn declares_protected_merge_roles(self) -> bool {
        matches!(self, Self::ProtectedMerge)
    }

    /// `true` when this lane must name a controlled release role.
    pub const fn declares_release_roles(self) -> bool {
        matches!(self, Self::Release)
    }

    /// `true` when this lane must name a controlled emergency-hotfix role.
    pub const fn declares_emergency_hotfix_roles(self) -> bool {
        matches!(self, Self::EmergencyHotfix)
    }
}

/// The single controlled build-lane-trust-role vocabulary every release-center, shiproom, diagnostics,
/// admin, docs, or support consumer binds to. These are the exact acceptance-criteria tokens that keep
/// `cache_posture`, `publication_authority`, `credential_boundary`, `hermetic_input`,
/// `reproducibility_proof`, `artifact_convergence`, and `support_identity` meaning the same thing everywhere
/// the build-lane-trust grammar ships. No surface invents a parallel word for any of these roles, and the
/// cache-posture / publication-authority / reproducibility-proof / artifact-convergence roles may never let
/// a PR cache publish, treat a remote-cache hit as reproducibility proof, drift a sidecar from the binary
/// build identity, or overclaim clean-room parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneTrustRole {
    /// Cache-posture role (the allowed remote-cache trust posture of a lane).
    CachePosture,
    /// Publication-authority role (whether a lane may publish release artifacts).
    PublicationAuthority,
    /// Credential-boundary role (the controlled credential scope a lane runs under).
    CredentialBoundary,
    /// Hermetic-input role (the hermeticity of the inputs a lane consumes).
    HermeticInput,
    /// Reproducibility-proof role (the clean-room rebuild diff and replay proof of a build).
    ReproducibilityProof,
    /// Artifact-convergence role (binaries, packages, SBOMs, symbols, and docs on one build identity).
    ArtifactConvergence,
    /// Support-identity role (the one exact build identity support and symbolication converge on).
    SupportIdentity,
}

impl M5BuildLaneTrustRole {
    /// Every build-lane-trust role token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::CachePosture,
        Self::PublicationAuthority,
        Self::CredentialBoundary,
        Self::HermeticInput,
        Self::ReproducibilityProof,
        Self::ArtifactConvergence,
        Self::SupportIdentity,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CachePosture => "cache_posture",
            Self::PublicationAuthority => "publication_authority",
            Self::CredentialBoundary => "credential_boundary",
            Self::HermeticInput => "hermetic_input",
            Self::ReproducibilityProof => "reproducibility_proof",
            Self::ArtifactConvergence => "artifact_convergence",
            Self::SupportIdentity => "support_identity",
        }
    }

    /// Whether this role carries cache-posture, publication-authority, reproducibility-proof, or
    /// artifact-convergence truth whose per-lane behavior must never let a PR cache publish release
    /// artifacts, treat a remote-cache hit as reproducibility proof, drift a docs / schema / SBOM / symbol
    /// sidecar from the binary build identity, or overclaim clean-room parity (`cache_posture`,
    /// `publication_authority`, `reproducibility_proof`, `artifact_convergence`). The descriptive structure
    /// roles (`credential_boundary`, `hermetic_input`, `support_identity`) are inspectable descriptors
    /// rather than trust-carrying truth and so do not carry this requirement.
    pub const fn must_verify_inputs_and_prove_replay_before_promotion(self) -> bool {
        matches!(
            self,
            Self::CachePosture
                | Self::PublicationAuthority
                | Self::ReproducibilityProof
                | Self::ArtifactConvergence
        )
    }
}

/// Controlled contributor / PR role — how the contributor / PR lane is named, so a shared cache read without
/// publication authority, the withheld release-artifact publication, the untrusted-cache posture, and the
/// PR-scoped credentials follow one build-lane-trust registry rather than publishing a release artifact from
/// a PR cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContributorPrRole {
    /// Shared cache is readable, never publishing.
    SharedCacheReadableNeverPublishing,
    /// Release-artifact publication withheld from this lane.
    ReleaseArtifactPublicationWithheld,
    /// Unverified cache marked untrusted.
    UnverifiedCacheMarkedUntrusted,
    /// PR-scoped credentials only.
    PrScopedCredentialsOnly,
    /// A role bound to the single build-lane-trust registry.
    BoundToBuildLaneTrustRegistry,
    /// A release artifact published from a PR lane, which is disallowed.
    ReleaseArtifactPublishedFromPrLaneDisallowed,
}

impl M5ContributorPrRole {
    /// Every contributor / PR role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SharedCacheReadableNeverPublishing,
        Self::ReleaseArtifactPublicationWithheld,
        Self::UnverifiedCacheMarkedUntrusted,
        Self::PrScopedCredentialsOnly,
        Self::BoundToBuildLaneTrustRegistry,
        Self::ReleaseArtifactPublishedFromPrLaneDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedCacheReadableNeverPublishing => "shared_cache_readable_never_publishing",
            Self::ReleaseArtifactPublicationWithheld => "release_artifact_publication_withheld",
            Self::UnverifiedCacheMarkedUntrusted => "unverified_cache_marked_untrusted",
            Self::PrScopedCredentialsOnly => "pr_scoped_credentials_only",
            Self::BoundToBuildLaneTrustRegistry => "bound_to_build_lane_trust_registry",
            Self::ReleaseArtifactPublishedFromPrLaneDisallowed => {
                "release_artifact_published_from_pr_lane_disallowed"
            }
        }
    }
}

/// Controlled protected-merge role — how the protected-merge lane is named, so controlled credentials scoped
/// to the lane, verified cache inputs only, cache posture verified before promotion, and a missing digest
/// blocking promotion follow one build-lane-trust registry rather than promoting from an untrusted cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProtectedMergeRole {
    /// Controlled credentials scoped to the lane.
    ControlledCredentialsScopedToLane,
    /// Verified cache inputs only.
    VerifiedCacheInputsOnly,
    /// Cache posture verified before promotion.
    CachePostureVerifiedBeforePromotion,
    /// A missing digest blocks promotion.
    MissingDigestBlocksPromotion,
    /// A role bound to the single build-lane-trust registry.
    BoundToBuildLaneTrustRegistry,
    /// An untrusted cache used for promotion, which is disallowed.
    UntrustedCacheUsedForPromotionDisallowed,
}

impl M5ProtectedMergeRole {
    /// Every protected-merge role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ControlledCredentialsScopedToLane,
        Self::VerifiedCacheInputsOnly,
        Self::CachePostureVerifiedBeforePromotion,
        Self::MissingDigestBlocksPromotion,
        Self::BoundToBuildLaneTrustRegistry,
        Self::UntrustedCacheUsedForPromotionDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlledCredentialsScopedToLane => "controlled_credentials_scoped_to_lane",
            Self::VerifiedCacheInputsOnly => "verified_cache_inputs_only",
            Self::CachePostureVerifiedBeforePromotion => "cache_posture_verified_before_promotion",
            Self::MissingDigestBlocksPromotion => "missing_digest_blocks_promotion",
            Self::BoundToBuildLaneTrustRegistry => "bound_to_build_lane_trust_registry",
            Self::UntrustedCacheUsedForPromotionDisallowed => {
                "untrusted_cache_used_for_promotion_disallowed"
            }
        }
    }
}

/// Controlled release role — how the release lane is named, so verified or re-materialized inputs only,
/// artifacts converging on one exact build identity, a fresh clean-room rebuild proof, and sidecars pinned to
/// the binary build identity follow one build-lane-trust registry rather than treating a remote-cache hit as
/// reproducibility proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseRole {
    /// Verified or re-materialized inputs only.
    VerifiedOrRematerializedInputsOnly,
    /// Artifacts converge on one exact build identity.
    ArtifactsConvergeOnOneExactBuildIdentity,
    /// Clean-room rebuild proof stays fresh.
    CleanRoomRebuildProofFresh,
    /// Sidecars pinned to the binary build identity.
    SidecarsPinnedToBinaryBuildIdentity,
    /// A role bound to the single build-lane-trust registry.
    BoundToBuildLaneTrustRegistry,
    /// A cache hit treated as reproducibility proof, which is disallowed.
    CacheHitTreatedAsReproducibilityProofDisallowed,
}

impl M5ReleaseRole {
    /// Every release role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::VerifiedOrRematerializedInputsOnly,
        Self::ArtifactsConvergeOnOneExactBuildIdentity,
        Self::CleanRoomRebuildProofFresh,
        Self::SidecarsPinnedToBinaryBuildIdentity,
        Self::BoundToBuildLaneTrustRegistry,
        Self::CacheHitTreatedAsReproducibilityProofDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedOrRematerializedInputsOnly => "verified_or_rematerialized_inputs_only",
            Self::ArtifactsConvergeOnOneExactBuildIdentity => {
                "artifacts_converge_on_one_exact_build_identity"
            }
            Self::CleanRoomRebuildProofFresh => "clean_room_rebuild_proof_fresh",
            Self::SidecarsPinnedToBinaryBuildIdentity => "sidecars_pinned_to_binary_build_identity",
            Self::BoundToBuildLaneTrustRegistry => "bound_to_build_lane_trust_registry",
            Self::CacheHitTreatedAsReproducibilityProofDisallowed => {
                "cache_hit_treated_as_reproducibility_proof_disallowed"
            }
        }
    }
}

/// Controlled emergency-hotfix role — how the emergency-hotfix lane is named, so re-materialized inputs under
/// controlled credentials, the exact build identity preserved under expedite, rollback metadata and the
/// support packet converged, and hermetic inputs verified despite urgency follow one build-lane-trust
/// registry rather than waiving non-hermetic inputs for speed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmergencyHotfixRole {
    /// Re-materialized inputs under controlled credentials.
    RematerializedInputsUnderControlledCredentials,
    /// Exact build identity preserved under expedite.
    ExactBuildIdentityPreservedUnderExpedite,
    /// Rollback metadata and the support packet converged.
    RollbackMetadataAndSupportPacketConverged,
    /// Hermetic inputs verified despite urgency.
    HermeticInputsVerifiedDespiteUrgency,
    /// A role bound to the single build-lane-trust registry.
    BoundToBuildLaneTrustRegistry,
    /// Non-hermetic inputs waived for speed, which is disallowed.
    NonHermeticInputsWaivedForSpeedDisallowed,
}

impl M5EmergencyHotfixRole {
    /// Every emergency-hotfix role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RematerializedInputsUnderControlledCredentials,
        Self::ExactBuildIdentityPreservedUnderExpedite,
        Self::RollbackMetadataAndSupportPacketConverged,
        Self::HermeticInputsVerifiedDespiteUrgency,
        Self::BoundToBuildLaneTrustRegistry,
        Self::NonHermeticInputsWaivedForSpeedDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RematerializedInputsUnderControlledCredentials => {
                "rematerialized_inputs_under_controlled_credentials"
            }
            Self::ExactBuildIdentityPreservedUnderExpedite => {
                "exact_build_identity_preserved_under_expedite"
            }
            Self::RollbackMetadataAndSupportPacketConverged => {
                "rollback_metadata_and_support_packet_converged"
            }
            Self::HermeticInputsVerifiedDespiteUrgency => {
                "hermetic_inputs_verified_despite_urgency"
            }
            Self::BoundToBuildLaneTrustRegistry => "bound_to_build_lane_trust_registry",
            Self::NonHermeticInputsWaivedForSpeedDisallowed => {
                "non_hermetic_inputs_waived_for_speed_disallowed"
            }
        }
    }
}

/// Claimed M5 surface family that renders / consumes a build lane. No lane may invent a parallel surface
/// taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneSurfaceFamily {
    /// The release-center surface.
    ReleaseCenter,
    /// The shiproom surface.
    Shiproom,
    /// The diagnostics surface.
    Diagnostics,
    /// The admin surface.
    Admin,
    /// The docs / help surface.
    DocsHelp,
    /// The support export.
    SupportExport,
}

impl M5BuildLaneSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReleaseCenter,
        Self::Shiproom,
        Self::Diagnostics,
        Self::Admin,
        Self::DocsHelp,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::Shiproom => "shiproom",
            Self::Diagnostics => "diagnostics",
            Self::Admin => "admin",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }
}

/// Build / publication context a lane must survive with the same truth, so a lane's cache-posture,
/// publication-authority, credential-boundary, hermetic-input, reproducibility-proof, artifact-convergence,
/// or support-identity meaning never silently narrows or widens between build shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneDeploymentLine {
    /// A local developer build.
    LocalDeveloperBuild,
    /// A continuous-integration build.
    ContinuousIntegration,
    /// An offline or air-gapped mirror build.
    OfflineOrAirGappedMirror,
    /// A protected release channel.
    ProtectedReleaseChannel,
    /// An emergency-hotfix channel.
    EmergencyHotfixChannel,
}

impl M5BuildLaneDeploymentLine {
    /// Every build context, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalDeveloperBuild,
        Self::ContinuousIntegration,
        Self::OfflineOrAirGappedMirror,
        Self::ProtectedReleaseChannel,
        Self::EmergencyHotfixChannel,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDeveloperBuild => "local_developer_build",
            Self::ContinuousIntegration => "continuous_integration",
            Self::OfflineOrAirGappedMirror => "offline_or_air_gapped_mirror",
            Self::ProtectedReleaseChannel => "protected_release_channel",
            Self::EmergencyHotfixChannel => "emergency_hotfix_channel",
        }
    }
}

/// Subsystem that consumes a lane's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneConsumerSurface {
    /// The build farm.
    BuildFarm,
    /// The cache service.
    CacheService,
    /// The release center.
    ReleaseCenter,
    /// The shiproom.
    Shiproom,
    /// The provenance service.
    ProvenanceService,
    /// The diagnostics surface.
    Diagnostics,
    /// The docs / help surface.
    DocsHelp,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
}

impl M5BuildLaneConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::BuildFarm,
        Self::CacheService,
        Self::ReleaseCenter,
        Self::Shiproom,
        Self::ProvenanceService,
        Self::Diagnostics,
        Self::DocsHelp,
        Self::CliExport,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildFarm => "build_farm",
            Self::CacheService => "cache_service",
            Self::ReleaseCenter => "release_center",
            Self::Shiproom => "shiproom",
            Self::ProvenanceService => "provenance_service",
            Self::Diagnostics => "diagnostics",
            Self::DocsHelp => "docs_help",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Non-visual / accessibility route every lane must offer so no build-lane-trust meaning disappears under
/// zoom, high contrast, keyboard-only use, or export. Records the keyboard, screen-reader, high-zoom,
/// high-contrast, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5BuildLaneAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a build lane has degraded below its qualified state. Required on every row so a stale, unresolved,
/// or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The build-lane-descriptor registry source is unavailable.
    BuildLaneDescriptorSourceUnavailable,
    /// The reproducibility-proof source is unavailable.
    ReproducibilityProofSourceUnavailable,
    /// Cache-posture evidence is unverified.
    CachePostureEvidenceUnverified,
    /// Clean-room proof evidence is unverified.
    CleanRoomProofEvidenceUnverified,
    /// Exact-build supportability evidence is unavailable.
    ExactBuildSupportabilityEvidenceUnavailable,
}

impl M5BuildLaneDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::BuildLaneDescriptorSourceUnavailable,
        Self::ReproducibilityProofSourceUnavailable,
        Self::CachePostureEvidenceUnverified,
        Self::CleanRoomProofEvidenceUnverified,
        Self::ExactBuildSupportabilityEvidenceUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::BuildLaneDescriptorSourceUnavailable => {
                "build_lane_descriptor_source_unavailable"
            }
            Self::ReproducibilityProofSourceUnavailable => {
                "reproducibility_proof_source_unavailable"
            }
            Self::CachePostureEvidenceUnverified => "cache_posture_evidence_unverified",
            Self::CleanRoomProofEvidenceUnverified => "clean_room_proof_evidence_unverified",
            Self::ExactBuildSupportabilityEvidenceUnavailable => {
                "exact_build_supportability_evidence_unavailable"
            }
        }
    }
}

/// Mandatory label a claimed build lane must be able to show. The first three are hard requirements on every
/// lane; the remaining three close the acceptance-criteria ambiguity about the cache posture, the
/// publication authority, and the exact build identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneRequiredLabel {
    /// The lane's stable identity.
    Identity,
    /// The lane's build-lane-trust role.
    SemanticRole,
    /// The canonical registry reference the lane points at.
    RegistryReference,
    /// The cache posture the lane is allowed.
    CachePosture,
    /// The publication authority the lane carries.
    PublicationAuthority,
    /// The exact build identity the lane converges on.
    BuildIdentity,
}

impl M5BuildLaneRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::CachePosture,
        Self::PublicationAuthority,
        Self::BuildIdentity,
    ];

    /// The three labels every claimed lane must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::CachePosture => "cache_posture",
            Self::PublicationAuthority => "publication_authority",
            Self::BuildIdentity => "build_identity",
        }
    }
}

/// Qualification class for an M5 build-lane row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneQualificationClass {
    /// Lane qualifies for the Stable claim.
    Stable,
    /// Lane is narrowed to Beta.
    Beta,
    /// Lane is narrowed to Preview.
    Preview,
    /// Lane is experimental and not claimed.
    Experimental,
    /// Lane is unavailable on this build.
    Unavailable,
    /// Lane is held pending upstream resolution.
    Held,
}

impl M5BuildLaneQualificationClass {
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

    /// Whether the lane may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a build lane below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneDowngradeTrigger {
    /// An untrusted cache was used.
    UsedAnUntrustedCache,
    /// A remote-cache hit was treated as reproducibility proof.
    TreatedARemoteCacheHitAsReproducibilityProof,
    /// A sidecar drifted from the binary build identity.
    DriftedASidecarFromTheBinaryBuildIdentity,
    /// Clean-room parity was overclaimed on a partial rebuild.
    OverclaimedCleanRoomParityOnPartialRebuild,
    /// Non-hermetic inputs, cache poisoning, or unreplayable artifacts were hidden.
    HidNonHermeticInputsCachePoisoningOrUnreplayableArtifacts,
    /// Release artifacts were published from a PR cache.
    PublishedReleaseArtifactsFromAPrCache,
    /// A lane left its cache posture unstated.
    CachePostureUnstated,
    /// A lane left its publication authority unstated.
    PublicationAuthorityUnstated,
    /// A lane left its exact build identity unstated.
    BuildIdentityUnstated,
    /// A lane left its canonical registry reference unstated.
    RegistryReferenceUnstated,
    /// A lane left its clean-room proof rule unstated.
    CleanRoomProofRuleUnstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5BuildLaneDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::UsedAnUntrustedCache,
        Self::TreatedARemoteCacheHitAsReproducibilityProof,
        Self::DriftedASidecarFromTheBinaryBuildIdentity,
        Self::OverclaimedCleanRoomParityOnPartialRebuild,
        Self::HidNonHermeticInputsCachePoisoningOrUnreplayableArtifacts,
        Self::PublishedReleaseArtifactsFromAPrCache,
        Self::CachePostureUnstated,
        Self::PublicationAuthorityUnstated,
        Self::BuildIdentityUnstated,
        Self::RegistryReferenceUnstated,
        Self::CleanRoomProofRuleUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UsedAnUntrustedCache => "used_an_untrusted_cache",
            Self::TreatedARemoteCacheHitAsReproducibilityProof => {
                "treated_a_remote_cache_hit_as_reproducibility_proof"
            }
            Self::DriftedASidecarFromTheBinaryBuildIdentity => {
                "drifted_a_sidecar_from_the_binary_build_identity"
            }
            Self::OverclaimedCleanRoomParityOnPartialRebuild => {
                "overclaimed_clean_room_parity_on_partial_rebuild"
            }
            Self::HidNonHermeticInputsCachePoisoningOrUnreplayableArtifacts => {
                "hid_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts"
            }
            Self::PublishedReleaseArtifactsFromAPrCache => {
                "published_release_artifacts_from_a_pr_cache"
            }
            Self::CachePostureUnstated => "cache_posture_unstated",
            Self::PublicationAuthorityUnstated => "publication_authority_unstated",
            Self::BuildIdentityUnstated => "build_identity_unstated",
            Self::RegistryReferenceUnstated => "registry_reference_unstated",
            Self::CleanRoomProofRuleUnstated => "clean_room_proof_rule_unstated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed build lane bound to the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneRow {
    /// Governed build lane.
    pub build_lane_family: M5BuildLaneFamily,
    /// Qualification class earned by this lane.
    pub qualification: M5BuildLaneQualificationClass,
    /// Owner role accountable for keeping this lane governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this lane.
    pub surface_families: Vec<M5BuildLaneSurfaceFamily>,
    /// Build contexts this lane keeps the same truth across.
    pub deployment_lines: Vec<M5BuildLaneDeploymentLine>,
    /// Mandatory labels this lane must be able to show (must include the three
    /// [`M5BuildLaneRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5BuildLaneRequiredLabel>,
    /// Build-lane-trust roles this lane can carry (the frozen AC vocabulary; required on every lane).
    pub semantic_roles: Vec<M5BuildLaneTrustRole>,
    /// Contributor / PR roles this lane names (contributor / PR lane only).
    pub contributor_pr_roles: Vec<M5ContributorPrRole>,
    /// Protected-merge roles this lane names (protected-merge lane only).
    pub protected_merge_roles: Vec<M5ProtectedMergeRole>,
    /// Release roles this lane names (release lane only).
    pub release_roles: Vec<M5ReleaseRole>,
    /// Emergency-hotfix roles this lane names (emergency-hotfix lane only).
    pub emergency_hotfix_roles: Vec<M5EmergencyHotfixRole>,
    /// Degraded reasons this lane can name (required on every lane).
    pub degraded_reasons: Vec<M5BuildLaneDegradedReason>,
    /// Non-visual accessibility routes this lane offers.
    pub accessibility_routes: Vec<M5BuildLaneAccessibilityRoute>,
    /// Subsystems that consume this lane's projection.
    pub consumer_surfaces: Vec<M5BuildLaneConsumerSurface>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5BuildLaneDowngradeTrigger>,
    /// Proof packet refs that keep this lane current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this lane (must include its own canonical domain schema so
    /// downstream surfaces have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this lane never lets a PR cache publish release artifacts. MUST be `false`.
    pub pr_caches_publish_release_artifacts: bool,
    /// Hard invariant: this lane never treats a remote-cache hit as reproducibility proof. MUST be `false`.
    pub treats_remote_cache_hits_as_reproducibility_proof: bool,
    /// Hard invariant: this lane never lets docs / schema / SBOM / symbol sidecars drift from the binary
    /// build identity. MUST be `false`.
    pub lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity: bool,
    /// Hard invariant: this lane never overclaims clean-room parity when only partial artifact classes were
    /// rebuilt. MUST be `false`.
    pub overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt: bool,
    /// Hard invariant: this lane never hides non-hermetic inputs, cache poisoning, or unreplayable artifacts
    /// behind green publication rows. MUST be `false`.
    pub hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows:
        bool,
}

impl M5BuildLaneRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5BuildLaneRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5BuildLaneRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.pr_caches_publish_release_artifacts
            && !self.treats_remote_cache_hits_as_reproducibility_proof
            && !self.lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity
            && !self.overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt
            && !self
                .hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneTrustVocabularySet {
    /// Build-lane-family tokens.
    pub build_lane_families: Vec<String>,
    /// Build-lane-trust-role tokens.
    pub semantic_roles: Vec<String>,
    /// Contributor / PR-role tokens.
    pub contributor_pr_roles: Vec<String>,
    /// Protected-merge-role tokens.
    pub protected_merge_roles: Vec<String>,
    /// Release-role tokens.
    pub release_roles: Vec<String>,
    /// Emergency-hotfix-role tokens.
    pub emergency_hotfix_roles: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Build-context tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5BuildLaneTrustVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            build_lane_families: tokens(&M5BuildLaneFamily::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5BuildLaneTrustRole::ALL, |v| v.as_str()),
            contributor_pr_roles: tokens(&M5ContributorPrRole::ALL, |v| v.as_str()),
            protected_merge_roles: tokens(&M5ProtectedMergeRole::ALL, |v| v.as_str()),
            release_roles: tokens(&M5ReleaseRole::ALL, |v| v.as_str()),
            emergency_hotfix_roles: tokens(&M5EmergencyHotfixRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5BuildLaneSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5BuildLaneDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5BuildLaneConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5BuildLaneAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5BuildLaneDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5BuildLaneRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5BuildLaneDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5BuildLaneTrustGovernanceReview {
    /// Contributor lanes read shared caches but never publish release artifacts.
    pub contributor_lanes_read_caches_but_never_publish_release_artifacts: bool,
    /// Protected-merge lanes use controlled credentials and verified caches.
    pub protected_merge_lanes_use_controlled_credentials_and_verified_caches: bool,
    /// Release and emergency-hotfix lanes use verified or re-materialized inputs.
    pub release_and_hotfix_lanes_use_verified_or_rematerialized_inputs: bool,
    /// Release artifacts converge on one exact build identity.
    pub release_artifacts_converge_on_one_exact_build_identity: bool,
    /// Remote-cache hits are never treated as reproducibility proof.
    pub remote_cache_hits_are_never_treated_as_reproducibility_proof: bool,
    /// Docs, schema, SBOM, and symbol sidecars stay pinned to the binary build identity.
    pub docs_schema_sbom_and_symbol_sidecars_stay_pinned_to_binary_build_identity: bool,
    /// Clean-room parity is never overclaimed on partial rebuilds.
    pub clean_room_parity_is_never_overclaimed_on_partial_rebuilds: bool,
    /// Non-hermetic inputs, cache poisoning, and unreplayable artifacts block promotion.
    pub non_hermetic_inputs_cache_poisoning_and_unreplayable_artifacts_block_promotion: bool,
    /// Missing digests block protected promotion.
    pub missing_digests_block_protected_promotion: bool,
    /// Every lane keeps the same truth across every build context.
    pub every_lane_declares_deployment_contexts: bool,
    /// Every lane declares a non-visual accessibility route.
    pub every_lane_declares_accessibility_route: bool,
    /// Support / export reads a single canonical build-lane source.
    pub support_export_reads_single_build_lane_source: bool,
    /// Release center, shiproom, and diagnostics bind to a single canonical build-lane source.
    pub release_center_shiproom_and_diagnostics_bind_to_single_build_lane_source: bool,
    /// Later M5 rows cannot invent parallel build-lane vocabulary.
    pub later_rows_cannot_invent_parallel_build_lane_vocabulary: bool,
    /// Build-lane truth survives zoom and high contrast.
    pub build_lane_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when the registry is missing, stale, or not yet qualified.
    pub claims_narrow_automatically_when_registry_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneTrustConsumerProjection {
    /// Release center and shiproom consume the shared build-lane truth.
    pub release_center_and_shiproom_consume_shared_build_lane_truth: bool,
    /// Diagnostics and admin consume the shared cache and credential boundaries.
    pub diagnostics_and_admin_consume_shared_cache_and_credential_boundaries: bool,
    /// Build farm and cache service consume the shared cache posture and publication authority.
    pub build_farm_and_cache_service_consume_shared_cache_posture_and_publication_authority: bool,
    /// Docs, help, and screenshots read a single build-lane source.
    pub docs_help_and_screenshots_read_single_build_lane_source: bool,
    /// Reproducibility and clean-room proofs bind to the shared exact-build identity.
    pub reproducibility_and_clean_room_proofs_bind_to_shared_exact_build_identity: bool,
    /// Support / export reads a single canonical build-lane source.
    pub support_export_reads_single_build_lane_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneTrustProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the build-lane-trust lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneTrustReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting build-lane audit for the lane.
    pub build_lane_audit_ref: String,
    /// True when support/export parity is required for every lane.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every lane.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BuildLaneTrustMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BuildLaneTrustMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Build-lane rows.
    pub build_lane_rows: Vec<M5BuildLaneRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BuildLaneTrustVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BuildLaneTrustGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BuildLaneTrustConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BuildLaneTrustProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BuildLaneTrustReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 build-lane-trust matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneTrustMatrixPacket {
    /// Record kind; must equal [`M5_BUILD_LANE_TRUST_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Build-lane rows.
    pub build_lane_rows: Vec<M5BuildLaneRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BuildLaneTrustVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BuildLaneTrustGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BuildLaneTrustConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BuildLaneTrustProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BuildLaneTrustReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BuildLaneTrustMatrixPacket {
    /// Builds an M5 build-lane-trust matrix packet from stable-lane input.
    pub fn new(input: M5BuildLaneTrustMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_BUILD_LANE_TRUST_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            build_lane_rows: input.build_lane_rows,
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

    /// Validates the M5 build-lane-trust matrix invariants.
    pub fn validate(&self) -> Vec<M5BuildLaneTrustMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_BUILD_LANE_TRUST_MATRIX_RECORD_KIND {
            violations.push(M5BuildLaneTrustMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_VERSION {
            violations.push(M5BuildLaneTrustMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BuildLaneTrustMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_build_lane_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 build-lane-trust matrix serializes"),
        ) {
            violations.push(M5BuildLaneTrustMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 build-lane-trust matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed lane.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "build_lane_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.build_lane_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.build_lane_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.build_lane_family.canonical_domain_schema_ref(),
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
        let stable_lanes = self
            .build_lane_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Build-Farm, Cache-Trust, Clean-Room-Rebuild, and Exact-Build-Supportability Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Build lanes: {} ({} stable)\n",
            self.build_lane_rows.len(),
            stable_lanes
        ));
        out.push_str(&format!(
            "- Build-lane-trust roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Contributor / PR roles: {}\n",
            self.vocabulary_set.contributor_pr_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Build lanes\n\n");
        for row in &self.build_lane_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.build_lane_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.build_lane_family.canonical_domain_schema_ref()
            ));
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

/// Errors emitted when reading the checked-in M5 build-lane-trust matrix export.
#[derive(Debug)]
pub enum M5BuildLaneTrustMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BuildLaneTrustMatrixViolation>),
}

impl fmt::Display for M5BuildLaneTrustMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 build-lane-trust matrix export parse failed: {error}"
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
                    "m5 build-lane-trust matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BuildLaneTrustMatrixArtifactError {}

/// Validation failures emitted by [`M5BuildLaneTrustMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BuildLaneTrustMatrixViolation {
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
    /// A required governed build lane is missing from the matrix.
    RequiredFamilyMissing,
    /// A build-lane row is incomplete.
    BuildLaneRowIncomplete,
    /// A build-lane row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A build-lane row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A lane declares no build-lane-trust roles.
    SemanticRoleMissing,
    /// The contributor / PR lane declares no contributor / PR roles.
    ContributorPrRoleMissing,
    /// The protected-merge lane declares no protected-merge roles.
    ProtectedMergeRoleMissing,
    /// The release lane declares no release roles.
    ReleaseRoleMissing,
    /// The emergency-hotfix lane declares no emergency-hotfix roles.
    EmergencyHotfixRoleMissing,
    /// A lane declares no degraded reasons.
    DegradedReasonMissing,
    /// A lane declares no surface families.
    SurfaceFamilyMissing,
    /// A lane declares no build contexts.
    DeploymentLineMissing,
    /// A lane declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A lane declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A lane declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A lane claiming Stable is missing required proof packet refs.
    StableFamilyMissingProof,
    /// A lane violates a hard invariant (letting a PR cache publish release artifacts, treating a
    /// remote-cache hit as reproducibility proof, drifting a docs / schema / SBOM / symbol sidecar from the
    /// binary build identity, overclaiming clean-room parity on a partial rebuild, or hiding non-hermetic
    /// inputs, cache poisoning, or unreplayable artifacts behind green publication rows).
    BuildLaneInvariantViolated,
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

impl M5BuildLaneTrustMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::BuildLaneRowIncomplete => "build_lane_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::ContributorPrRoleMissing => "contributor_pr_role_missing",
            Self::ProtectedMergeRoleMissing => "protected_merge_role_missing",
            Self::ReleaseRoleMissing => "release_role_missing",
            Self::EmergencyHotfixRoleMissing => "emergency_hotfix_role_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableFamilyMissingProof => "stable_family_missing_proof",
            Self::BuildLaneInvariantViolated => "build_lane_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 build-lane-trust matrix export.
pub fn current_stable_m5_build_lane_trust_matrix_export(
) -> Result<M5BuildLaneTrustMatrixPacket, M5BuildLaneTrustMatrixArtifactError> {
    let packet: M5BuildLaneTrustMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-exact-build-supportability-proof/support_export.json"
    )))
    .map_err(M5BuildLaneTrustMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BuildLaneTrustMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BuildLaneTrustMatrixPacket,
    violations: &mut Vec<M5BuildLaneTrustMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
        M5_BUILD_LANE_TRUST_MATRIX_DOC_REF,
        M5_BUILD_LANE_DESCRIPTOR_DOMAIN_SCHEMA_REF,
        M5_REPRODUCIBILITY_PROOF_DOMAIN_SCHEMA_REF,
        M5_ARTIFACT_PUBLICATION_LANDED_SCHEMA_REF,
        M5_REPRODUCIBLE_RC_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BuildLaneTrustMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BuildLaneTrustMatrixPacket,
    violations: &mut Vec<M5BuildLaneTrustMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BuildLaneTrustMatrixViolation::VocabularySetDrift);
    }
}

fn validate_build_lane_rows(
    packet: &M5BuildLaneTrustMatrixPacket,
    violations: &mut Vec<M5BuildLaneTrustMatrixViolation>,
) {
    let present: BTreeSet<M5BuildLaneFamily> = packet
        .build_lane_rows
        .iter()
        .map(|row| row.build_lane_family)
        .collect();
    for required in M5BuildLaneFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5BuildLaneTrustMatrixViolation::RequiredFamilyMissing);
            return;
        }
    }

    for row in &packet.build_lane_rows {
        let family = row.build_lane_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5BuildLaneTrustMatrixViolation::BuildLaneRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5BuildLaneTrustMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_domain_schema_ref())
        {
            violations.push(M5BuildLaneTrustMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5BuildLaneTrustMatrixViolation::SemanticRoleMissing);
        }
        if family.declares_contributor_pr_roles() && row.contributor_pr_roles.is_empty() {
            violations.push(M5BuildLaneTrustMatrixViolation::ContributorPrRoleMissing);
        }
        if family.declares_protected_merge_roles() && row.protected_merge_roles.is_empty() {
            violations.push(M5BuildLaneTrustMatrixViolation::ProtectedMergeRoleMissing);
        }
        if family.declares_release_roles() && row.release_roles.is_empty() {
            violations.push(M5BuildLaneTrustMatrixViolation::ReleaseRoleMissing);
        }
        if family.declares_emergency_hotfix_roles() && row.emergency_hotfix_roles.is_empty() {
            violations.push(M5BuildLaneTrustMatrixViolation::EmergencyHotfixRoleMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5BuildLaneTrustMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5BuildLaneTrustMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5BuildLaneTrustMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5BuildLaneTrustMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BuildLaneTrustMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5BuildLaneTrustMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5BuildLaneTrustMatrixViolation::StableFamilyMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5BuildLaneTrustMatrixViolation::BuildLaneInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5BuildLaneTrustMatrixPacket,
    violations: &mut Vec<M5BuildLaneTrustMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.contributor_lanes_read_caches_but_never_publish_release_artifacts,
        review.protected_merge_lanes_use_controlled_credentials_and_verified_caches,
        review.release_and_hotfix_lanes_use_verified_or_rematerialized_inputs,
        review.release_artifacts_converge_on_one_exact_build_identity,
        review.remote_cache_hits_are_never_treated_as_reproducibility_proof,
        review.docs_schema_sbom_and_symbol_sidecars_stay_pinned_to_binary_build_identity,
        review.clean_room_parity_is_never_overclaimed_on_partial_rebuilds,
        review.non_hermetic_inputs_cache_poisoning_and_unreplayable_artifacts_block_promotion,
        review.missing_digests_block_protected_promotion,
        review.every_lane_declares_deployment_contexts,
        review.every_lane_declares_accessibility_route,
        review.support_export_reads_single_build_lane_source,
        review.release_center_shiproom_and_diagnostics_bind_to_single_build_lane_source,
        review.later_rows_cannot_invent_parallel_build_lane_vocabulary,
        review.build_lane_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_registry_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5BuildLaneTrustMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BuildLaneTrustMatrixPacket,
    violations: &mut Vec<M5BuildLaneTrustMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.release_center_and_shiproom_consume_shared_build_lane_truth,
        projection.diagnostics_and_admin_consume_shared_cache_and_credential_boundaries,
        projection
            .build_farm_and_cache_service_consume_shared_cache_posture_and_publication_authority,
        projection.docs_help_and_screenshots_read_single_build_lane_source,
        projection.reproducibility_and_clean_room_proofs_bind_to_shared_exact_build_identity,
        projection.support_export_reads_single_build_lane_source,
    ] {
        if !ok {
            violations.push(M5BuildLaneTrustMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5BuildLaneTrustMatrixPacket,
    violations: &mut Vec<M5BuildLaneTrustMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5BuildLaneTrustMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5BuildLaneTrustMatrixPacket,
    violations: &mut Vec<M5BuildLaneTrustMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.build_lane_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BuildLaneTrustMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses build / cache / lane / credential / hermetic / reproducibility words; what is rejected
/// is a raw secret *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key
/// block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
