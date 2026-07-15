//! Shared build-farm, cache-service, release-center, shiproom, provenance-service, diagnostics,
//! docs / help, CLI / export, and support / export consumers that keep the B144 build-lane-trust families —
//! the contributor / PR lane, the protected-merge lane, the release lane, and the emergency-hotfix lane — at
//! **one canonical registry** across every claimed M5 release-bearing surface.
//!
//! This module is the consumer-adoption capstone for the four governed build lanes frozen in
//! [`crate::m5_build_lane_trust_matrix`] and implemented by the build-lane-descriptor / reproducibility-proof
//! lane ([`crate::m5_build_lane_descriptor_and_reproducibility_proof_registries`]), the verified-input /
//! sidecar-completeness lane
//! ([`crate::m5_verified_input_manifest_and_sidecar_completeness_registries`]), the clean-room-rebuild /
//! artifact-diff lane ([`crate::m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries`]), the
//! remote-cache-integrity / cache-bypass-drill lane
//! ([`crate::m5_remote_cache_integrity_and_cache_bypass_drill_registries`]), and the
//! exact-build-symbolication / mirror-offline-parity lane
//! ([`crate::m5_exact_build_symbolication_and_mirror_offline_parity_registries`]).
//!
//! It binds each shared build-lane-trust family to the concrete About / provenance, Help, service-health,
//! release-center, and support-export consumers — projected here through the build-farm, cache-service,
//! release-center, shiproom, provenance-service, diagnostics, docs / help, CLI / export, and support-export
//! surfaces — that render it, and proves — by fixtures, not screenshots — that the same build profile presents
//! the same build-lane-trust-role, family, registry-reference, build-context, surface-context, and
//! replay-continuity grammar wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the four shared build-lane-trust families must be adopted by at least two distinct
//!    consumers, so a lane is proven to be shared build-lane infrastructure rather than a one-surface,
//!    feature-local fork of build-lane-descriptor or reproducibility-proof copy.
//! 2. **One registry / no drift.** For a given build profile every consumer surface must present identical
//!    [`BuildLaneTrustStateFacetValues`] — the same build-lane-trust-role word, the same family word, the same
//!    registry-reference word, the same build-context word, the same surface-context word, and the same
//!    replay-continuity word. The build-lane-trust-role word must be a token from the frozen
//!    [`M5BuildLaneTrustRole`] vocabulary, so no surface rewrites `cache_posture`, `publication_authority`,
//!    `credential_boundary`, `hermetic_input`, `reproducibility_proof`, `artifact_convergence`, or
//!    `support_identity` in its own words. A surface may narrow *how much* it shows across desktop, compact,
//!    remote, and exported representations, but it may never reword the underlying grammar per surface, and a
//!    role that carries cache-posture, publication-authority, reproducibility-proof, or artifact-convergence
//!    meaning may never let a PR cache publish release artifacts, treat a remote-cache hit as reproducibility
//!    proof, drift a docs / schema / SBOM / symbol sidecar from the binary build identity, or overclaim
//!    clean-room parity on a partial rebuild.
//! 3. **Map back to one family.** Support and CLI/export consumers must point at the canonical per-domain
//!    schema and the frozen matrix by id, so an exported packet can always map a release-center / provenance /
//!    diagnostics / support surface back to one shared contract family.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an explicit
//! [`BuildLaneTrustNarrowNote`] naming the reason, the preserved grammar, and the next action, and an exported
//! representation additionally names its export-safe detail boundary rather than collapsing the profile out of
//! view.
//!
//! The packet references upstream build-lane-trust contracts by id rather than embedding their content. Raw
//! secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/release/m5-build-lane-trust-shared-consumers.schema.json`](../../../../schemas/release/m5-build-lane-trust-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/release/m5_build_lane_trust_shared_consumers_one_registry.md`](../../../../docs/release/m5_build_lane_trust_shared_consumers_one_registry.md).
//! The protected fixture directory is
//! [`fixtures/release/m5-build-lane-trust-shared-consumers/`](../../../../fixtures/release/m5-build-lane-trust-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_build_lane_trust_shared_consumers,
    seeded_m5_build_lane_trust_shared_consumers_compact_remote_narrowed,
    seeded_m5_build_lane_trust_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_build_lane_trust_matrix::{
    M5BuildLaneConsumerSurface, M5BuildLaneFamily, M5BuildLaneTrustRole,
    M5_BUILD_LANE_TRUST_MATRIX_DOC_REF, M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5BuildLaneTrustSharedConsumersPacket`].
pub const M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_build_lane_trust_shared_consumer_registry_parity";

/// Schema version for build-lane-trust shared-consumer parity records.
pub const M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-build-lane-trust-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/release/m5-build-lane-trust-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/release/m5_build_lane_trust_shared_consumers_one_registry.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-build-lane-trust-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-build-lane-trust-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-build-lane-trust-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/release/m5-build-lane-trust-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Replay-continuity sentinel words a cache-posture / publication-authority / reproducibility-proof /
/// artifact-convergence role may never fall back to; a trust-carrying role that changes surface presentation
/// must always keep a real replay-proven-and-build-identity-converged continuity, never publishing release
/// artifacts from a PR cache, treating a remote-cache hit as reproducibility proof, drifting a sidecar from
/// the binary build identity, or overclaiming clean-room parity on a partial rebuild.
const REPLAY_CONTINUITY_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "published_release_artifacts_from_a_pr_cache",
    "treated_a_remote_cache_hit_as_reproducibility_proof",
    "drifted_a_sidecar_from_the_binary_build_identity",
    "overclaimed_clean_room_parity_on_a_partial_rebuild",
];

/// Whether a consumer surface is an export / support path that must map a family back to its canonical
/// contract by id.
pub const fn consumer_must_reference_canonical(consumer: M5BuildLaneConsumerSurface) -> bool {
    matches!(
        consumer,
        M5BuildLaneConsumerSurface::SupportExport | M5BuildLaneConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5BuildLaneTrustRole`] vocabulary.
///
/// This is the "one registry" gate: a build profile's build-lane-trust-role word must be a controlled role
/// token rather than a per-surface synonym.
pub fn is_known_build_lane_trust_role_token(token: &str) -> bool {
    build_lane_trust_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5BuildLaneTrustRole`], if it is one.
pub fn build_lane_trust_role_from_token(token: &str) -> Option<M5BuildLaneTrustRole> {
    M5BuildLaneTrustRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared build-lane-trust family a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying grammar: a narrowed representation still carries
/// the same build-lane-trust-role, family, registry-reference, build-context, surface-context, and
/// replay-continuity words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildLaneTrustRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl BuildLaneTrustRepresentation {
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

/// A grammar axis whose word must stay identical across surfaces for one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildLaneTrustParityFacet {
    /// The frozen build-lane-trust-role word.
    BuildLaneTrustRoleWord,
    /// The build-lane-family word.
    FamilyWord,
    /// The canonical registry-reference word the family points at.
    RegistryReferenceWord,
    /// The build-context word (local developer build / continuous integration / offline or air-gapped mirror /
    /// protected release channel / emergency-hotfix channel) the profile ships.
    BuildContextWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The replay-continuity word paired with a cache-posture / publication-authority / reproducibility-proof /
    /// artifact-convergence role.
    ReplayContinuityWord,
}

impl BuildLaneTrustParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BuildLaneTrustRoleWord,
        Self::FamilyWord,
        Self::RegistryReferenceWord,
        Self::BuildContextWord,
        Self::SurfaceContextWord,
        Self::ReplayContinuityWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildLaneTrustRoleWord => "build_lane_trust_role_word",
            Self::FamilyWord => "family_word",
            Self::RegistryReferenceWord => "registry_reference_word",
            Self::BuildContextWord => "build_context_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::ReplayContinuityWord => "replay_continuity_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared build-lane-trust family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildLaneTrustNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl BuildLaneTrustNarrowReason {
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
pub enum BuildLaneTrustNarrowNextAction {
    /// Expand the family in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl BuildLaneTrustNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandInDesktop => "expand_in_desktop",
            Self::OpenRemoteSource => "open_remote_source",
            Self::OpenFullDetail => "open_full_detail",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildLaneTrustParityState {
    /// All grammar is preserved and shown in full.
    FacetsPreserved,
    /// All grammar is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl BuildLaneTrustParityState {
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
pub enum BuildLaneTrustSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Build-lane-trust grammar drifted between surfaces for the same profile.
    BuildLaneTrustGrammarDriftDetected,
    /// A trust-carrying role dropped its replay-proof or build-identity-convergence meaning.
    ReplayOrBuildIdentityDisclosureDropped,
    /// A surface let a PR cache publish release artifacts.
    PrCachesPublishReleaseArtifacts,
    /// A surface treated a remote-cache hit as reproducibility proof.
    TreatsRemoteCacheHitsAsReproducibilityProof,
    /// A surface let docs / schema / SBOM / symbol sidecars drift from the binary build identity.
    LetsDocsSchemaSbomOrSymbolSidecarsDriftFromBinaryBuildIdentity,
    /// A surface overclaimed clean-room parity when only partial artifact classes were rebuilt.
    OverclaimsCleanRoomParityWhenOnlyPartialArtifactClassesWereRebuilt,
    /// A surface hid non-hermetic inputs, cache poisoning, or unreplayable artifacts behind green rows.
    HidesNonHermeticInputsCachePoisoningOrUnreplayableArtifactsBehindGreenPublicationRows,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream shared build-lane-trust family narrowed.
    UpstreamBuildLaneTrustNarrowed,
}

impl BuildLaneTrustSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::BuildLaneTrustGrammarDriftDetected,
        Self::ReplayOrBuildIdentityDisclosureDropped,
        Self::PrCachesPublishReleaseArtifacts,
        Self::TreatsRemoteCacheHitsAsReproducibilityProof,
        Self::LetsDocsSchemaSbomOrSymbolSidecarsDriftFromBinaryBuildIdentity,
        Self::OverclaimsCleanRoomParityWhenOnlyPartialArtifactClassesWereRebuilt,
        Self::HidesNonHermeticInputsCachePoisoningOrUnreplayableArtifactsBehindGreenPublicationRows,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamBuildLaneTrustNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::BuildLaneTrustGrammarDriftDetected => "build_lane_trust_grammar_drift_detected",
            Self::ReplayOrBuildIdentityDisclosureDropped => {
                "replay_or_build_identity_disclosure_dropped"
            }
            Self::PrCachesPublishReleaseArtifacts => "pr_caches_publish_release_artifacts",
            Self::TreatsRemoteCacheHitsAsReproducibilityProof => {
                "treats_remote_cache_hits_as_reproducibility_proof"
            }
            Self::LetsDocsSchemaSbomOrSymbolSidecarsDriftFromBinaryBuildIdentity => {
                "lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity"
            }
            Self::OverclaimsCleanRoomParityWhenOnlyPartialArtifactClassesWereRebuilt => {
                "overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt"
            }
            Self::HidesNonHermeticInputsCachePoisoningOrUnreplayableArtifactsBehindGreenPublicationRows => {
                "hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows"
            }
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamBuildLaneTrustNarrowed => "upstream_build_lane_trust_narrowed",
        }
    }
}

/// The controlled grammar a build profile presents.
///
/// These six words must be identical across every consumer surface that shows the same build profile. The
/// build-lane-trust-role word must be a frozen role token; the rest are controlled words the profile's family
/// carries. A surface may narrow how much it renders, but it may never reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustStateFacetValues {
    /// Build-lane-trust-role word (must be a frozen [`M5BuildLaneTrustRole`] token).
    pub build_lane_trust_role_word: String,
    /// Build-lane-family word.
    pub family_word: String,
    /// Canonical registry-reference word the family points at.
    pub registry_reference_word: String,
    /// Build-context word (local developer build / continuous integration / offline or air-gapped mirror /
    /// protected release channel / emergency-hotfix channel) the profile ships.
    pub build_context_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Replay-continuity word paired with a cache-posture / publication-authority / reproducibility-proof /
    /// artifact-convergence role.
    pub replay_continuity_word: String,
}

impl BuildLaneTrustStateFacetValues {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.build_lane_trust_role_word.trim().is_empty()
            && !self.family_word.trim().is_empty()
            && !self.registry_reference_word.trim().is_empty()
            && !self.build_context_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.replay_continuity_word.trim().is_empty()
    }

    /// Whether the build-lane-trust-role word is a member of the frozen role vocabulary.
    pub fn build_lane_trust_role_word_in_vocabulary(&self) -> bool {
        is_known_build_lane_trust_role_token(self.build_lane_trust_role_word.trim())
    }

    /// Whether the profile honours the replay rule: a role that carries cache-posture, publication-authority,
    /// reproducibility-proof, or artifact-convergence meaning must pair its surface change with a real
    /// replay-proven-and-build-identity-converged continuity and never collapse to a
    /// published-release-artifacts-from-a-pr-cache, treated-a-remote-cache-hit-as-reproducibility-proof,
    /// drifted-a-sidecar-from-the-binary-build-identity, or
    /// overclaimed-clean-room-parity-on-a-partial-rebuild sentinel.
    pub fn replay_continuity_satisfied(&self) -> bool {
        match build_lane_trust_role_from_token(self.build_lane_trust_role_word.trim()) {
            Some(role) if role.must_verify_inputs_and_prove_replay_before_promotion() => {
                let continuity = self.replay_continuity_word.trim().to_lowercase();
                !continuity.is_empty()
                    && !REPLAY_CONTINUITY_ABSENT_SENTINELS.contains(&continuity.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustNarrowNote {
    /// Why the representation narrowed.
    pub reason: BuildLaneTrustNarrowReason,
    /// Note naming the preserved grammar (never omitted).
    pub preserved_grammar_note: String,
    /// The next action offered.
    pub next_action: BuildLaneTrustNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildLaneTrustRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: BuildLaneTrustParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<BuildLaneTrustNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<BuildLaneTrustNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit remote-source note.
    pub needs_remote_source_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its representation.
///
/// The full desktop representation renders at full parity. A compact representation narrows disclosure depth, a
/// remote-projected representation names its remote source, and an exported representation names its
/// export-safe-detail boundary — but all three keep every grammar word and disclose the narrowing through an
/// explicit note.
pub const fn resolve_build_lane_trust_render_disclosure(
    representation: BuildLaneTrustRepresentation,
) -> BuildLaneTrustRenderDisclosure {
    match representation {
        BuildLaneTrustRepresentation::DesktopFull => BuildLaneTrustRenderDisclosure {
            parity_state: BuildLaneTrustParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        BuildLaneTrustRepresentation::CompactNarrowed => BuildLaneTrustRenderDisclosure {
            parity_state: BuildLaneTrustParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(BuildLaneTrustNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(BuildLaneTrustNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        BuildLaneTrustRepresentation::RemoteProjected => BuildLaneTrustRenderDisclosure {
            parity_state: BuildLaneTrustParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(BuildLaneTrustNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(BuildLaneTrustNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        BuildLaneTrustRepresentation::ExportedRedacted => BuildLaneTrustRenderDisclosure {
            parity_state: BuildLaneTrustParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(BuildLaneTrustNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(BuildLaneTrustNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared build-lane-trust family rendered on one consumer surface in one
/// representation for one build profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable build-profile id (shared across surfaces that show the same profile).
    pub build_profile_id: String,
    /// Human-readable build-profile identity.
    pub build_profile_label: String,
    /// Which shared build-lane-trust family this binding renders.
    pub family: M5BuildLaneFamily,
    /// Which consumer surface renders it.
    pub consumer: M5BuildLaneConsumerSurface,
    /// Which representation this surface renders.
    pub representation: BuildLaneTrustRepresentation,
    /// The controlled grammar presented (identical across surfaces for one profile).
    pub state_facets: BuildLaneTrustStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: BuildLaneTrustParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<BuildLaneTrustNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface lets a PR cache publish release artifacts. MUST be `false`.
    pub pr_caches_publish_release_artifacts: bool,
    /// Guardrail: this surface treats a remote-cache hit as reproducibility proof. MUST be `false`.
    pub treats_remote_cache_hits_as_reproducibility_proof: bool,
    /// Guardrail: this surface lets docs / schema / SBOM / symbol sidecars drift from the binary build
    /// identity. MUST be `false`.
    pub lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity: bool,
    /// Guardrail: this surface overclaims clean-room parity when only partial artifact classes were rebuilt.
    /// MUST be `false`.
    pub overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt: bool,
    /// Guardrail: this surface hides non-hermetic inputs, cache poisoning, or unreplayable artifacts behind
    /// green publication rows. MUST be `false`.
    pub hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows:
        bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl BuildLaneTrustConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> BuildLaneTrustRenderDisclosure {
        resolve_build_lane_trust_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.pr_caches_publish_release_artifacts
            && !self.treats_remote_cache_hits_as_reproducibility_proof
            && !self.lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity
            && !self.overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt
            && !self
                .hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows
    }

    /// Whether this binding points at the canonical per-domain schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let domain_ref = self.family.canonical_domain_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == domain_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustSharedConsumersTrustReview {
    /// Family reuse is proven by fixtures rather than inferred from screenshots.
    pub family_reuse_proven_by_fixtures: bool,
    /// The same build profile presents the same grammar across surfaces.
    pub same_profile_same_build_lane_trust_across_surfaces: bool,
    /// Every build-lane-trust-role word is a frozen role token.
    pub build_lane_trust_role_words_stay_in_frozen_vocabulary: bool,
    /// Trust-carrying roles never publish untrusted or treat a cache hit as proof.
    pub trust_roles_never_publish_untrusted_or_treat_cache_as_proof: bool,
    /// A surface never lets a PR cache publish release artifacts.
    pub pr_cache_never_publishes_release_artifacts: bool,
    /// A surface never treats a remote-cache hit as reproducibility proof.
    pub remote_cache_hit_never_treated_as_reproducibility_proof: bool,
    /// A surface never lets docs / schema / SBOM / symbol sidecars drift from the binary build identity.
    pub sidecars_never_drift_from_binary_build_identity: bool,
    /// A surface never overclaims clean-room parity on a partial rebuild.
    pub clean_room_parity_never_overclaimed_on_partial_rebuild: bool,
    /// A surface never hides non-hermetic inputs, cache poisoning, or unreplayable artifacts.
    pub non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_never_hidden: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the family.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl BuildLaneTrustSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.family_reuse_proven_by_fixtures
            && self.same_profile_same_build_lane_trust_across_surfaces
            && self.build_lane_trust_role_words_stay_in_frozen_vocabulary
            && self.trust_roles_never_publish_untrusted_or_treat_cache_as_proof
            && self.pr_cache_never_publishes_release_artifacts
            && self.remote_cache_hit_never_treated_as_reproducibility_proof
            && self.sidecars_never_drift_from_binary_build_identity
            && self.clean_room_parity_never_overclaimed_on_partial_rebuild
            && self.non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_never_hidden
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustSharedConsumersProjection {
    /// The build farm consumes the shared build-lane-trust grammar.
    pub build_farm_consumes_shared_build_lane_trust: bool,
    /// The cache service consumes the shared build-lane-trust grammar.
    pub cache_service_consumes_shared_build_lane_trust: bool,
    /// The release center consumes the shared build-lane-trust grammar.
    pub release_center_consumes_shared_build_lane_trust: bool,
    /// The shiproom consumes the shared build-lane-trust grammar.
    pub shiproom_consumes_shared_build_lane_trust: bool,
    /// The provenance service consumes the shared build-lane-trust grammar.
    pub provenance_service_consumes_shared_build_lane_trust: bool,
    /// The diagnostics surface consumes the shared build-lane-trust grammar.
    pub diagnostics_consumes_shared_build_lane_trust: bool,
    /// The docs / help surface consumes the shared build-lane-trust grammar.
    pub docs_help_consumes_shared_build_lane_trust: bool,
    /// The CLI / export path consumes the shared build-lane-trust grammar.
    pub cli_export_consumes_shared_build_lane_trust: bool,
    /// The support / export path consumes the shared build-lane-trust grammar.
    pub support_export_consumes_shared_build_lane_trust: bool,
    /// Every family is adopted by two or more consumers.
    pub every_family_adopted_by_two_or_more_consumers: bool,
    /// Grammar is identical for the same build profile.
    pub build_lane_trust_identical_for_same_profile: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a family back to one shared contract family.
    pub export_maps_back_to_one_build_lane_family: bool,
}

impl BuildLaneTrustSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.build_farm_consumes_shared_build_lane_trust
            && self.cache_service_consumes_shared_build_lane_trust
            && self.release_center_consumes_shared_build_lane_trust
            && self.shiproom_consumes_shared_build_lane_trust
            && self.provenance_service_consumes_shared_build_lane_trust
            && self.diagnostics_consumes_shared_build_lane_trust
            && self.docs_help_consumes_shared_build_lane_trust
            && self.cli_export_consumes_shared_build_lane_trust
            && self.support_export_consumes_shared_build_lane_trust
            && self.every_family_adopted_by_two_or_more_consumers
            && self.build_lane_trust_identical_for_same_profile
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_build_lane_family
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildLaneTrustSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5BuildLaneTrustSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BuildLaneTrustSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<BuildLaneTrustConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<BuildLaneTrustSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5BuildLaneConsumerSurface>,
    /// Trust review block.
    pub trust_review: BuildLaneTrustSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: BuildLaneTrustSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: BuildLaneTrustSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe build-lane-trust shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneTrustSharedConsumersPacket {
    /// Record kind; must equal [`M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<BuildLaneTrustConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<BuildLaneTrustSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5BuildLaneConsumerSurface>,
    /// Trust review block.
    pub trust_review: BuildLaneTrustSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: BuildLaneTrustSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: BuildLaneTrustSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BuildLaneTrustSharedConsumersPacket {
    /// Builds a build-lane-trust shared-consumer packet from stable-lane input.
    pub fn new(input: M5BuildLaneTrustSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_SCHEMA_VERSION,
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

    /// Validates the build-lane-trust shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5BuildLaneTrustSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("build-lane-trust shared-consumer packet serializes"),
        ) {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::RawBoundaryMaterialInExport);
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
            .expect("build-lane-trust shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "family,consumer,representation,build_lane_trust_role_word,parity_state\n",
        );
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.build_lane_trust_role_word,
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
        out.push_str("# Shared Build-Lane-Trust Consumers: One Registry Across Surfaces\n\n");
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
                "- **{}** [`{}`]: family `{}` on `{}`, representation `{}`, role `{}`\n",
                binding.build_profile_label,
                binding.binding_id,
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.build_lane_trust_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in build-lane-trust shared-consumer export.
#[derive(Debug)]
pub enum M5BuildLaneTrustSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BuildLaneTrustSharedConsumersViolation>),
}

impl fmt::Display for M5BuildLaneTrustSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "build-lane-trust shared-consumer export parse failed: {error}"
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
                    "build-lane-trust shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BuildLaneTrustSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5BuildLaneTrustSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BuildLaneTrustSharedConsumersViolation {
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
    /// A binding's grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's build-lane-trust-role word is not a frozen role token.
    BuildLaneTrustRoleWordOutsideVocabulary,
    /// A binding's trust-carrying role dropped its replay continuity.
    ReplayContinuityMissingForTrustRole,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same build profile with different grammar.
    BuildLaneTrustGrammarDriftAcrossSurfaces,
    /// A shared family is not adopted by at least two distinct consumers.
    FamilyReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow note.
    NarrowNoteMissing,
    /// A narrow note's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow note's next action does not match the required next action.
    NarrowNextActionMismatch,
    /// A narrow note is missing its preserved-grammar note.
    NarrowNotePreservedGrammarMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A full-desktop binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit remote-source note is missing it.
    RemoteSourceNoteMissing,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding lets a PR cache publish release artifacts.
    PrCachesPublishReleaseArtifacts,
    /// A binding treats a remote-cache hit as reproducibility proof.
    TreatsRemoteCacheHitsAsReproducibilityProof,
    /// A binding lets docs / schema / SBOM / symbol sidecars drift from the binary build identity.
    LetsDocsSchemaSbomOrSymbolSidecarsDriftFromBinaryBuildIdentity,
    /// A binding overclaims clean-room parity when only partial artifact classes were rebuilt.
    OverclaimsCleanRoomParityWhenOnlyPartialArtifactClassesWereRebuilt,
    /// A binding hides non-hermetic inputs, cache poisoning, or unreplayable artifacts behind green rows.
    HidesNonHermeticInputsCachePoisoningOrUnreplayableArtifactsBehindGreenPublicationRows,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared family appears among the bindings.
    FamilyCoverageMissing,
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

impl M5BuildLaneTrustSharedConsumersViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::GrammarFacetIncomplete => "grammar_facet_incomplete",
            Self::BuildLaneTrustRoleWordOutsideVocabulary => {
                "build_lane_trust_role_word_outside_vocabulary"
            }
            Self::ReplayContinuityMissingForTrustRole => {
                "replay_continuity_missing_for_trust_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::BuildLaneTrustGrammarDriftAcrossSurfaces => {
                "build_lane_trust_grammar_drift_across_surfaces"
            }
            Self::FamilyReuseUnproven => "family_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedGrammarMissing => "narrow_note_preserved_grammar_missing",
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::RemoteSourceNoteMissing => "remote_source_note_missing",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::PrCachesPublishReleaseArtifacts => "pr_caches_publish_release_artifacts",
            Self::TreatsRemoteCacheHitsAsReproducibilityProof => {
                "treats_remote_cache_hits_as_reproducibility_proof"
            }
            Self::LetsDocsSchemaSbomOrSymbolSidecarsDriftFromBinaryBuildIdentity => {
                "lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity"
            }
            Self::OverclaimsCleanRoomParityWhenOnlyPartialArtifactClassesWereRebuilt => {
                "overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt"
            }
            Self::HidesNonHermeticInputsCachePoisoningOrUnreplayableArtifactsBehindGreenPublicationRows => {
                "hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::FamilyCoverageMissing => "family_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable build-lane-trust shared-consumer export.
pub fn current_stable_m5_build_lane_trust_shared_consumers_export(
) -> Result<M5BuildLaneTrustSharedConsumersPacket, M5BuildLaneTrustSharedConsumersArtifactError> {
    let packet: M5BuildLaneTrustSharedConsumersPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-build-lane-trust-shared-consumers-proof/support_export.json"
    )))
        .map_err(M5BuildLaneTrustSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BuildLaneTrustSharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5BuildLaneTrustSharedConsumersPacket,
    violations: &mut Vec<M5BuildLaneTrustSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_SCHEMA_REF,
        M5_BUILD_LANE_TRUST_SHARED_CONSUMERS_DOC_REF,
        M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
        M5_BUILD_LANE_TRUST_MATRIX_DOC_REF,
    ];
    // The four families map to two canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5BuildLaneFamily::ALL {
        domains.insert(family.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5BuildLaneTrustSharedConsumersPacket,
    violations: &mut Vec<M5BuildLaneTrustSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5BuildLaneTrustSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One registry: the facet values must be identical for every binding that renders the same build
    // profile.
    let mut profile_facets: BTreeMap<&str, &BuildLaneTrustStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each family must be adopted by at least two distinct consumers.
    let mut family_consumers: BTreeMap<M5BuildLaneFamily, BTreeSet<M5BuildLaneConsumerSurface>> =
        BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5BuildLaneConsumerSurface> = BTreeSet::new();
    let mut seen_families: BTreeSet<M5BuildLaneFamily> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.build_profile_id.trim().is_empty()
            || binding.build_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::GrammarFacetIncomplete);
        }
        if !binding
            .state_facets
            .build_lane_trust_role_word_in_vocabulary()
        {
            violations.push(
                M5BuildLaneTrustSharedConsumersViolation::BuildLaneTrustRoleWordOutsideVocabulary,
            );
        }
        if !binding.state_facets.replay_continuity_satisfied() {
            violations.push(
                M5BuildLaneTrustSharedConsumersViolation::ReplayContinuityMissingForTrustRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5BuildLaneTrustSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5BuildLaneTrustSharedConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5BuildLaneTrustSharedConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_grammar_note.trim().is_empty() {
                        violations.push(
                            M5BuildLaneTrustSharedConsumersViolation::NarrowNotePreservedGrammarMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5BuildLaneTrustSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.pr_caches_publish_release_artifacts {
            violations
                .push(M5BuildLaneTrustSharedConsumersViolation::PrCachesPublishReleaseArtifacts);
        }
        if binding.treats_remote_cache_hits_as_reproducibility_proof {
            violations.push(
                M5BuildLaneTrustSharedConsumersViolation::TreatsRemoteCacheHitsAsReproducibilityProof,
            );
        }
        if binding.lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity {
            violations.push(
                M5BuildLaneTrustSharedConsumersViolation::LetsDocsSchemaSbomOrSymbolSidecarsDriftFromBinaryBuildIdentity,
            );
        }
        if binding.overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt {
            violations.push(
                M5BuildLaneTrustSharedConsumersViolation::OverclaimsCleanRoomParityWhenOnlyPartialArtifactClassesWereRebuilt,
            );
        }
        if binding
            .hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows
        {
            violations.push(
                M5BuildLaneTrustSharedConsumersViolation::HidesNonHermeticInputsCachePoisoningOrUnreplayableArtifactsBehindGreenPublicationRows,
            );
        }

        // Support / export consumers must map a family back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations
                .push(M5BuildLaneTrustSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match profile_facets.get(binding.build_profile_id.as_str()) {
            None => {
                profile_facets.insert(binding.build_profile_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5BuildLaneTrustSharedConsumersViolation::BuildLaneTrustGrammarDriftAcrossSurfaces,
                    );
                    drift_reported = true;
                }
            }
        }

        family_consumers
            .entry(binding.family)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_families.insert(binding.family);
    }

    // Coverage: every consumer surface and every family must appear.
    for consumer in M5BuildLaneConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for family in M5BuildLaneFamily::ALL {
        if !seen_families.contains(&family) {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::FamilyCoverageMissing);
            break;
        }
    }

    // Reuse: every present family must be adopted by two or more distinct consumers.
    for consumers in family_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5BuildLaneTrustSharedConsumersViolation::FamilyReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
