//! Implemented M5 build-lane-descriptor and reproducibility-proof registries.
//!
//! The frozen [build-lane-trust matrix][matrix] names Aureline's four governed build lanes — the contributor /
//! PR lane, the protected-merge lane, the release lane, and the emergency-hotfix lane — and locks their
//! controlled vocabulary. This module is the first implement lane for the concrete build-lane trust flows: it
//! turns the *build-lane-descriptor* grammar (how a lane declares its allowed cache reads / writes, its
//! controlled credential class, its publication rights, and the artifact families it is expected to produce)
//! and the *reproducibility-proof* grammar (how a release or emergency-hotfix lane proves its inputs came from
//! a verified cache or were re-materialized and that binaries, packages, SBOMs, symbols, docs, and rollback
//! metadata converge on one exact build identity) into registry resolvers that produce export-safe, honest
//! projections. Every claimed M5 build lane then resolves to one typed build-lane-descriptor object — the
//! cache posture it classifies, the cache read scope, the cache write scope, the controlled credential class,
//! the publication rights it is bounded to (never allowing a contributor / PR lane to publish release
//! artifacts), the expected artifact families, the hermetic-input posture, and the clean-room rebuild rule —
//! and to one reproducibility-proof object — the resolved exact build identity, the verified-versus-
//! re-materialized input-source ledger, the clean-room rebuild diff reference, the sidecar-convergence state,
//! the attestation state, the rollback-metadata reference, and the last rebuild revision — that the
//! release-center, shiproom, diagnostics, provenance, and support / export surfaces can inspect without manual
//! reconstruction, so an untrusted lane can never publish a release artifact, a remote-cache hit is never
//! treated as reproducibility proof, the cache / credential / publication boundary stays visible before
//! promotion, and a build lane that cannot explain the descriptor it declared or the build identity it
//! converged on degrades honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one typed build-lane-descriptor object per lane.** [`resolve_build_lane_descriptor_entry`]
//!   refuses to read as a clean, registry-bound descriptor entry unless it names a canonical registry token, a
//!   classified [cache posture][M5BuildLaneCachePostureKind], a build-lane-trust role, covers every [resolution
//!   form][M5BuildLaneResolutionForm] (the canonical object, the accessible summary, and the audit record),
//!   publishes every descriptor field (cache read scope, cache write scope, credential class, publication
//!   rights, expected artifact families, hermetic-input posture, and clean-room rebuild rule), bounds its
//!   publication authority so a contributor / PR lane never publishes a release artifact, and discloses the
//!   cache-trust posture before a trust-risk cache is read; otherwise it degrades.
//! * **Keep an untrusted lane from publishing release artifacts.** [`untrusted_lane_cannot_publish`] rejects a
//!   descriptor entry whose publication authority is unbounded (a PR / contributor lane claiming publish
//!   rights) so it degrades to
//!   [`M5BuildLaneDescriptorEntryDegradeReason::DescriptorLetsUntrustedLanePublishOrHidesCacheTrust`], and a
//!   trust-risk cache posture that hides its cache-trust marker degrades the same way — the structured blocker
//!   reason a publish-from-untrusted-lane attempt must surface.
//! * **Keep the reproducibility proof from treating a cache hit as proof or drifting the build identity.**
//!   [`resolve_reproducibility_proof_entry`] names a classified [convergence scope][M5ReproducibilityConvergenceScope],
//!   requires the full build-identity / input-source-ledger / clean-room-diff / sidecar-convergence /
//!   attestation / rollback-metadata / last-rebuild-revision reproducibility-proof object, covers every
//!   resolution form, and degrades to
//!   [`M5ReproducibilityProofEntryDegradeReason::ReproducibilityProofTreatsCacheHitAsProofOrDriftsBuildIdentity`]
//!   when the proof would treat a remote-cache hit as reproducibility proof, hide the input source, or let a
//!   non-hermetic input masquerade as hermetic, so a reproducibility proof can never read as trustworthy when
//!   it has quietly dropped the reason its binaries are actually replayable.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5BuildLaneTrustRole`] role vocabulary
//! and the [`M5BuildLaneConsumerSurface`] consumer-surface taxonomy — so the build-farm, cache-service,
//! release-center, shiproom, provenance, diagnostics, docs, CLI, and support surfaces can never fork their own
//! build-lane meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_build_lane_trust_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries,
    seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries_build_lane_descriptor_beta_narrowed,
    seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries_reproducibility_proof_preview_narrowed,
    M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_build_lane_trust_matrix::{
    M5BuildLaneAccessibilityRoute, M5BuildLaneConsumerSurface, M5BuildLaneDeploymentLine,
    M5BuildLaneDowngradeTrigger, M5BuildLaneFamily, M5BuildLaneQualificationClass,
    M5BuildLaneRequiredLabel, M5BuildLaneTrustRole, M5_BUILD_LANE_DESCRIPTOR_DOMAIN_SCHEMA_REF,
    M5_BUILD_LANE_TRUST_MATRIX_DOC_REF, M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
    M5_REPRODUCIBILITY_PROOF_DOMAIN_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5BuildLaneDescriptorReproducibilityProofRegistriesPacket`].
pub const M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_build_lane_descriptor_and_reproducibility_proof_registries";

/// Schema version for M5 build-lane-descriptor / reproducibility-proof registry records.
pub const M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_SCHEMA_REF: &str =
    "schemas/release/m5-build-lane-descriptor-and-reproducibility-proof-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_DOC_REF: &str =
    "docs/release/m5_build_lane_descriptor_and_reproducibility_proof_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-build-lane-descriptor-and-reproducibility-proof-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-build-lane-descriptor-and-reproducibility-proof-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-build-lane-descriptor-and-reproducibility-proof-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/release/m5-build-lane-descriptor-and-reproducibility-proof-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5BuildLaneDescriptorReproducibilityProofRegistriesConsumerSurface =
    M5BuildLaneConsumerSurface;

/// One of the three resolution forms every build-lane-descriptor or reproducibility-proof entry must hold
/// across so its truth keeps whether it is shown as the canonical resolved object, announced as an accessible
/// summary, or written to the audit / support record. Minted by this lane because the frozen matrix names the
/// build-lane-descriptor and reproducibility-proof *domains* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneResolutionForm {
    /// The canonical resolved build-lane-descriptor / reproducibility-proof object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved lane discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved lane inspectable off-renderer.
    AuditRecord,
}

impl M5BuildLaneResolutionForm {
    /// Every resolution form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::CanonicalObject,
        Self::AccessibleSummary,
        Self::AuditRecord,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalObject => "canonical_object",
            Self::AccessibleSummary => "accessible_summary",
            Self::AuditRecord => "audit_record",
        }
    }
}

/// Controlled remote-cache trust posture a build-lane-descriptor entry declares, so the typed descriptor model
/// shares one registry rather than a hand-copied per-lane assumption. Minted by this lane because the frozen
/// matrix carries the build lanes but not the concrete hermetic / verified / shared-untrusted / remote-
/// publishing / mirror-replay cache posture a descriptor classifies against. Every classified posture carries
/// its canonical mode, and the shared-untrusted and remote-publishing postures are trust-risk-bearing so their
/// cache-trust marker must be disclosed before the cache is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneCachePostureKind {
    /// A fully hermetic lane that reads no remote cache and re-materializes every input.
    HermeticNoCache,
    /// A lane that reads verified cache inputs only.
    VerifiedInputsOnly,
    /// A lane that reads a shared remote cache marked untrusted (trust-risk; a hit may hide missing inputs).
    SharedReadableUntrusted,
    /// A publishing lane whose remote cache is write-bearing (trust-risk; a poisoned entry can propagate).
    RemotePublishingCache,
    /// A lane that replays from an offline or air-gapped mirror.
    MirrorReplayCache,
    /// The cache posture is unclassified, which is disallowed.
    PostureUnclassified,
}

impl M5BuildLaneCachePostureKind {
    /// Every cache posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HermeticNoCache,
        Self::VerifiedInputsOnly,
        Self::SharedReadableUntrusted,
        Self::RemotePublishingCache,
        Self::MirrorReplayCache,
        Self::PostureUnclassified,
    ];

    /// The five canonical cache postures every claimed M5 build lane classifies against.
    pub const CANONICAL_POSTURES: [Self; 5] = [
        Self::HermeticNoCache,
        Self::VerifiedInputsOnly,
        Self::SharedReadableUntrusted,
        Self::RemotePublishingCache,
        Self::MirrorReplayCache,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HermeticNoCache => "hermetic_no_cache",
            Self::VerifiedInputsOnly => "verified_inputs_only",
            Self::SharedReadableUntrusted => "shared_readable_untrusted",
            Self::RemotePublishingCache => "remote_publishing_cache",
            Self::MirrorReplayCache => "mirror_replay_cache",
            Self::PostureUnclassified => "posture_unclassified",
        }
    }

    /// Whether the posture is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::PostureUnclassified)
    }

    /// The canonical mode for this cache posture.
    pub const fn canonical_cache_posture_mode(self) -> &'static str {
        match self {
            Self::HermeticNoCache => "hermetic_no_cache_posture",
            Self::VerifiedInputsOnly => "verified_inputs_only_posture",
            Self::SharedReadableUntrusted => "shared_readable_untrusted_posture",
            Self::RemotePublishingCache => "remote_publishing_cache_posture",
            Self::MirrorReplayCache => "mirror_replay_cache_posture",
            Self::PostureUnclassified => "",
        }
    }

    /// Whether this posture is trust-risk-bearing and so must disclose the cache-trust marker before the cache
    /// is read.
    pub const fn is_trust_risk_posture(self) -> bool {
        matches!(
            self,
            Self::SharedReadableUntrusted | Self::RemotePublishingCache
        )
    }
}

/// Controlled convergence scope a reproducibility-proof entry must resolve its build identity from, so a proof
/// shares one registry rather than a hand-copied per-record assumption. Minted by this lane, tracking whether
/// the inputs came from a verified cache, were re-materialized, or came from a hermetic rebuild the acceptance
/// criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReproducibilityConvergenceScope {
    /// The inputs converged from a verified cache.
    VerifiedCacheInputs,
    /// The inputs were re-materialized from source.
    RematerializedInputs,
    /// The inputs came from a hermetic clean-room rebuild.
    HermeticRebuildInputs,
    /// The convergence scope is unclassified, which is disallowed.
    ScopeUnclassified,
}

impl M5ReproducibilityConvergenceScope {
    /// Every convergence scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::VerifiedCacheInputs,
        Self::RematerializedInputs,
        Self::HermeticRebuildInputs,
        Self::ScopeUnclassified,
    ];

    /// The three canonical convergence scopes every reproducibility proof must stay distinct across.
    pub const CANONICAL_SCOPES: [Self; 3] = [
        Self::VerifiedCacheInputs,
        Self::RematerializedInputs,
        Self::HermeticRebuildInputs,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCacheInputs => "verified_cache_inputs",
            Self::RematerializedInputs => "rematerialized_inputs",
            Self::HermeticRebuildInputs => "hermetic_rebuild_inputs",
            Self::ScopeUnclassified => "scope_unclassified",
        }
    }

    /// Whether the convergence scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ScopeUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a build-lane-descriptor
/// or reproducibility-proof token's meaning stays stable whether it appears in the release-center, shiproom,
/// diagnostics, provenance, or a support / export form. Minted by this lane, tracking the first-consumer
/// surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneSurfaceContext {
    /// The release-center surface.
    ReleaseCenterSurface,
    /// The shiproom surface.
    ShiproomSurface,
    /// The diagnostics surface.
    DiagnosticsSurface,
    /// The provenance surface.
    ProvenanceSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5BuildLaneSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReleaseCenterSurface,
        Self::ShiproomSurface,
        Self::DiagnosticsSurface,
        Self::ProvenanceSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::ReleaseCenterSurface,
        Self::ShiproomSurface,
        Self::DiagnosticsSurface,
        Self::ProvenanceSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenterSurface => "release_center_surface",
            Self::ShiproomSurface => "shiproom_surface",
            Self::DiagnosticsSurface => "diagnostics_surface",
            Self::ProvenanceSurface => "provenance_surface",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a build-lane-descriptor or reproducibility-proof entry must be able to show, so
/// no cache posture, credential class, publication right, artifact family, reproducibility-proof field, or
/// registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The cache posture the entry classifies (build-lane-descriptor entry).
    CachePosture,
    /// The cache read / write scopes, credential class, and publication rights the entry publishes
    /// (build-lane-descriptor entry).
    CacheCredentialAndPublicationBoundaries,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The expected artifact families the entry publishes (build-lane-descriptor entry).
    ExpectedArtifactFamilies,
    /// The reproducibility-proof fields (build identity, input-source ledger, clean-room diff, sidecar
    /// convergence, attestation, rollback metadata) the entry publishes (reproducibility-proof entry).
    ReproducibilityProofFields,
    /// The support-identity hint the entry publishes (reproducibility-proof entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved build-lane descriptor or reproducibility proof (both
    /// entries).
    PlainLanguageMeaning,
}

impl M5BuildLaneAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::CachePosture,
        Self::CacheCredentialAndPublicationBoundaries,
        Self::ResolutionFormCoverage,
        Self::ExpectedArtifactFamilies,
        Self::ReproducibilityProofFields,
        Self::SupportIdentityHint,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::CachePosture => "cache_posture",
            Self::CacheCredentialAndPublicationBoundaries => {
                "cache_credential_and_publication_boundaries"
            }
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::ExpectedArtifactFamilies => "expected_artifact_families",
            Self::ReproducibilityProofFields => "reproducibility_proof_fields",
            Self::SupportIdentityHint => "support_identity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// build-lane descriptor, a reproducibility proof, or a degraded build-lane-descriptor / reproducibility-proof
/// entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneNextAction {
    /// Expand the resolved build-lane descriptor's or reproducibility proof's plain-language meaning.
    ExpandLaneMeaning,
    /// Inspect the cache posture or convergence scope the entry resolves.
    InspectPostureOrScope,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5BuildLaneNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandLaneMeaning,
        Self::InspectPostureOrScope,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandLaneMeaning => "expand_lane_meaning",
            Self::InspectPostureOrScope => "inspect_posture_or_scope",
            Self::CompleteResolutionFormCoverage => "complete_resolution_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The build-lane families covered.
    BuildLaneFamilies,
    /// The cache postures carried.
    CachePostures,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The convergence scopes carried.
    ConvergenceScopes,
    /// The render / surface context.
    SurfaceContext,
    /// The cache-posture modes carried.
    CachePostureModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5BuildLaneExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::BuildLaneFamilies,
        Self::CachePostures,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::ConvergenceScopes,
        Self::SurfaceContext,
        Self::CachePostureModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::BuildLaneFamilies,
        Self::CachePostures,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::BuildLaneFamilies => "build_lane_families",
            Self::CachePostures => "cache_postures",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::ConvergenceScopes => "convergence_scopes",
            Self::SurfaceContext => "surface_context",
            Self::CachePostureModes => "cache_posture_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a build-lane-descriptor entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, publish-from-untrusted, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildLaneDescriptorEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the descriptor means.
    DescriptorTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The cache posture is unclassified (not in the resolved taxonomy).
    CachePostureUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    DescriptorNotBoundToRegistry,
    /// The resolved build-lane-descriptor object is incomplete: the cache read scope, cache write scope,
    /// credential class, publication rights, expected artifact families, hermetic-input posture, or clean-room
    /// rebuild rule is unstated.
    BuildLaneDescriptorObjectIncomplete,
    /// The lane's publication authority is unbounded (a PR / contributor lane claiming publish rights), or a
    /// trust-risk cache posture hid its cache-trust marker.
    DescriptorLetsUntrustedLanePublishOrHidesCacheTrust,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A trust-risk cache posture did not disclose the cache-trust marker before the cache was read.
    CacheTrustNotDisclosedForTrustRiskPosture,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5BuildLaneDescriptorEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::DescriptorTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::CachePostureUnclassified,
        Self::DescriptorNotBoundToRegistry,
        Self::BuildLaneDescriptorObjectIncomplete,
        Self::DescriptorLetsUntrustedLanePublishOrHidesCacheTrust,
        Self::ResolutionFormCoverageIncomplete,
        Self::CacheTrustNotDisclosedForTrustRiskPosture,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorTokenUnstated => "descriptor_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::CachePostureUnclassified => "cache_posture_unclassified",
            Self::DescriptorNotBoundToRegistry => "descriptor_not_bound_to_registry",
            Self::BuildLaneDescriptorObjectIncomplete => "build_lane_descriptor_object_incomplete",
            Self::DescriptorLetsUntrustedLanePublishOrHidesCacheTrust => {
                "descriptor_lets_untrusted_lane_publish_or_hides_cache_trust"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::CacheTrustNotDisclosedForTrustRiskPosture => {
                "cache_trust_not_disclosed_for_trust_risk_posture"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5BuildLaneNextAction {
        match self {
            Self::DescriptorTokenUnstated | Self::DescriptorNotBoundToRegistry => {
                M5BuildLaneNextAction::TraceCanonicalRegistry
            }
            Self::CachePostureUnclassified
            | Self::BuildLaneDescriptorObjectIncomplete
            | Self::DescriptorLetsUntrustedLanePublishOrHidesCacheTrust => {
                M5BuildLaneNextAction::InspectPostureOrScope
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5BuildLaneNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::CacheTrustNotDisclosedForTrustRiskPosture
            | Self::ProofStale => M5BuildLaneNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5BuildLaneDowngradeTrigger {
        match self {
            Self::DescriptorTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::DescriptorNotBoundToRegistry => {
                M5BuildLaneDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::CachePostureUnclassified | Self::BuildLaneDescriptorObjectIncomplete => {
                M5BuildLaneDowngradeTrigger::CachePostureUnstated
            }
            Self::DescriptorLetsUntrustedLanePublishOrHidesCacheTrust
            | Self::CacheTrustNotDisclosedForTrustRiskPosture => {
                M5BuildLaneDowngradeTrigger::PublishedReleaseArtifactsFromAPrCache
            }
            Self::ProofStale => M5BuildLaneDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a reproducibility-proof entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReproducibilityProofEntryDegradeReason {
    /// The canonical registry token name is unstated.
    ProofTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The convergence scope is unclassified (not in the resolved taxonomy).
    ConvergenceScopeUnclassified,
    /// The reproducibility proof would treat a remote-cache hit as reproducibility proof, hide the input
    /// source of the build, let a non-hermetic input masquerade as hermetic, or it dropped one of the required
    /// reproducibility-proof fields (build identity, input-source ledger, clean-room diff, sidecar
    /// convergence, attestation, rollback metadata, last rebuild revision).
    ReproducibilityProofTreatsCacheHitAsProofOrDriftsBuildIdentity,
    /// The canonical / accessible / audit resolution-form coverage of the proof is incomplete.
    ProofFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ReproducibilityProofEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::ConvergenceScopeUnclassified,
        Self::ReproducibilityProofTreatsCacheHitAsProofOrDriftsBuildIdentity,
        Self::ProofFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofTokenUnstated => "proof_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ConvergenceScopeUnclassified => "convergence_scope_unclassified",
            Self::ReproducibilityProofTreatsCacheHitAsProofOrDriftsBuildIdentity => {
                "reproducibility_proof_treats_cache_hit_as_proof_or_drifts_build_identity"
            }
            Self::ProofFormCoverageIncomplete => "proof_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5BuildLaneNextAction {
        match self {
            Self::ProofTokenUnstated => M5BuildLaneNextAction::TraceCanonicalRegistry,
            Self::ConvergenceScopeUnclassified
            | Self::ReproducibilityProofTreatsCacheHitAsProofOrDriftsBuildIdentity => {
                M5BuildLaneNextAction::InspectPostureOrScope
            }
            Self::ProofFormCoverageIncomplete => {
                M5BuildLaneNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5BuildLaneNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5BuildLaneDowngradeTrigger {
        match self {
            Self::ProofTokenUnstated => M5BuildLaneDowngradeTrigger::RegistryReferenceUnstated,
            Self::SurfaceContextUnresolved | Self::ConvergenceScopeUnclassified => {
                M5BuildLaneDowngradeTrigger::BuildIdentityUnstated
            }
            Self::ReproducibilityProofTreatsCacheHitAsProofOrDriftsBuildIdentity => {
                M5BuildLaneDowngradeTrigger::TreatedARemoteCacheHitAsReproducibilityProof
            }
            Self::ProofFormCoverageIncomplete => {
                M5BuildLaneDowngradeTrigger::CleanRoomProofRuleUnstated
            }
            Self::ProofStale => M5BuildLaneDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_build_lane_descriptor_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BuildLaneDescriptorEntryResolutionInput {
    /// Stable identity of the build-lane-descriptor-registry entry.
    pub entry_id: String,
    /// The stable lane-binding ID this descriptor binds to (e.g. `release.lane.protected-merge`); empty means
    /// unstated.
    pub lane_binding_id: String,
    /// The canonical registry token name (e.g. `build.lane.descriptor.protected_merge`); empty means unstated.
    pub token_name: String,
    /// The high-level build-lane-trust role (from the frozen matrix vocabulary).
    pub semantic_role: M5BuildLaneTrustRole,
    /// The cache posture this entry classifies.
    pub cache_posture: M5BuildLaneCachePostureKind,
    /// The render / surface context.
    pub surface_context: M5BuildLaneSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5BuildLaneResolutionForm>,
    /// The published cache read scope; empty means unstated.
    pub cache_read_scope: String,
    /// The published cache write scope; empty means unstated.
    pub cache_write_scope: String,
    /// The published controlled credential class; empty means unstated.
    pub credential_class: String,
    /// The published publication rights; empty means unstated.
    pub publication_rights: String,
    /// The published expected artifact families; empty means unstated.
    pub expected_artifact_families: String,
    /// The published hermetic-input posture; empty means unstated.
    pub hermetic_input_posture: String,
    /// The published clean-room rebuild rule; empty means unstated.
    pub clean_room_rebuild_rule: String,
    /// True when the behavior traces to the build-lane-descriptor registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the lane's publication authority is honestly bounded so a contributor / PR lane never
    /// publishes a release artifact (a hard invariant when `false`).
    pub publication_authority_bounded: bool,
    /// True when this lane's cache posture is trust-risk-bearing.
    pub is_trust_risk_posture: bool,
    /// True when the cache-trust marker is disclosed before a trust-risk cache is read.
    pub cache_trust_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe build-lane-descriptor-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBuildLaneDescriptorEntry {
    /// Stable identity of the build-lane-descriptor-registry entry.
    pub entry_id: String,
    /// The stable lane-binding ID this descriptor binds to.
    pub lane_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must verify inputs and prove replay before promotion.
    pub semantic_role_must_verify_inputs_and_prove_replay_before_promotion: bool,
    /// The cache-posture token named by the entry.
    pub cache_posture: String,
    /// Whether the cache posture is classified into the resolved taxonomy.
    pub cache_posture_is_classified: bool,
    /// The canonical mode for the entry's cache posture.
    pub canonical_cache_posture_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published cache read scope.
    pub cache_read_scope: String,
    /// The published cache write scope.
    pub cache_write_scope: String,
    /// The published controlled credential class.
    pub credential_class: String,
    /// The published publication rights.
    pub publication_rights: String,
    /// The published expected artifact families.
    pub expected_artifact_families: String,
    /// The published hermetic-input posture.
    pub hermetic_input_posture: String,
    /// The published clean-room rebuild rule.
    pub clean_room_rebuild_rule: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved build-lane-descriptor object publishes every required field.
    pub build_lane_descriptor_object_complete: bool,
    /// Whether the entry traces to the build-lane-descriptor registry.
    pub bound_to_registry: bool,
    /// Whether the lane's publication authority stays bounded (an untrusted lane never publishes).
    pub publication_authority_bounded: bool,
    /// Whether this lane's cache posture is trust-risk-bearing.
    pub is_trust_risk_posture: bool,
    /// Whether the cache-trust marker is disclosed before a trust-risk cache is read.
    pub cache_trust_disclosed: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5BuildLaneDescriptorEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5BuildLaneNextAction,
    /// Whether the descriptor resolves to one typed object across every claimed lane (clean entry naming every
    /// fact).
    pub descriptor_resolves_across_lanes: bool,
}

impl M5ResolvedBuildLaneDescriptorEntry {
    /// Whether this build-lane-descriptor entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_reproducibility_proof_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReproducibilityProofEntryResolutionInput {
    /// Stable identity of the reproducibility-proof entry.
    pub entry_id: String,
    /// The stable proof-ref this record binds to; empty means unstated.
    pub proof_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level build-lane-trust role (from the frozen matrix vocabulary).
    pub semantic_role: M5BuildLaneTrustRole,
    /// The convergence scope this record must resolve its build identity from.
    pub convergence_scope: M5ReproducibilityConvergenceScope,
    /// The render / surface context.
    pub surface_context: M5BuildLaneSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5BuildLaneResolutionForm>,
    /// The published resolved exact build identity; empty means missing.
    pub resolved_build_identity: String,
    /// The published verified-versus-re-materialized input-source ledger; empty means missing.
    pub input_source_ledger: String,
    /// The published clean-room rebuild diff reference; empty means missing.
    pub clean_room_diff_reference: String,
    /// The published sidecar-convergence state; empty means missing.
    pub sidecar_convergence_state: String,
    /// The published attestation state; empty means missing.
    pub attestation_state: String,
    /// The published rollback-metadata reference; empty means missing.
    pub rollback_metadata_reference: String,
    /// The published last rebuild revision; empty means missing.
    pub last_rebuild_revision: String,
    /// True when the record keeps the verified-versus-re-materialized input source visible.
    pub keeps_input_source_visible: bool,
    /// True when the proof is truthful (never claims a clean proof over a hidden input source).
    pub proof_is_truthful: bool,
    /// True when a remote-cache hit contributed to this build.
    pub remote_cache_hit_present: bool,
    /// True when a remote-cache hit is marked as never being reproducibility proof.
    pub remote_cache_hit_marked_not_proof: bool,
    /// True when the build consumed a non-hermetic input.
    pub non_hermetic_input_present: bool,
    /// True when a non-hermetic input is flagged rather than masquerading as hermetic.
    pub non_hermetic_input_flagged: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe reproducibility-proof projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedReproducibilityProofEntry {
    /// Stable identity of the reproducibility-proof entry.
    pub entry_id: String,
    /// The stable proof-ref this record binds to.
    pub proof_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must verify inputs and prove replay before promotion.
    pub semantic_role_must_verify_inputs_and_prove_replay_before_promotion: bool,
    /// The convergence-scope token named by the entry.
    pub convergence_scope: String,
    /// Whether the convergence scope is classified into the resolved taxonomy.
    pub convergence_scope_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published resolved exact build identity.
    pub resolved_build_identity: String,
    /// The published verified-versus-re-materialized input-source ledger.
    pub input_source_ledger: String,
    /// The published clean-room rebuild diff reference.
    pub clean_room_diff_reference: String,
    /// The published sidecar-convergence state.
    pub sidecar_convergence_state: String,
    /// The published attestation state.
    pub attestation_state: String,
    /// The published rollback-metadata reference.
    pub rollback_metadata_reference: String,
    /// The published last rebuild revision.
    pub last_rebuild_revision: String,
    /// Whether the record keeps the input source visible.
    pub keeps_input_source_visible: bool,
    /// Whether the proof is truthful.
    pub proof_is_truthful: bool,
    /// Whether a remote-cache hit contributed to this build.
    pub remote_cache_hit_present: bool,
    /// Whether a remote-cache hit is marked as never being reproducibility proof.
    pub remote_cache_hit_marked_not_proof: bool,
    /// Whether the build consumed a non-hermetic input.
    pub non_hermetic_input_present: bool,
    /// Whether a non-hermetic input is flagged rather than masquerading as hermetic.
    pub non_hermetic_input_flagged: bool,
    /// Whether the record stays honest (input source visible, cache hit marked not proof, non-hermetic input
    /// flagged).
    pub reproducibility_proof_stays_honest: bool,
    /// Whether the entry provides the complete reproducibility-proof object (build identity, input-source
    /// ledger, clean-room diff, sidecar convergence, attestation, rollback metadata, last rebuild revision).
    pub provides_complete_reproducibility_proof: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5ReproducibilityProofEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5BuildLaneNextAction,
    /// Whether the reproducibility proof is safe on every claimed lane (clean entry naming every fact).
    pub proof_safe_on_every_lane: bool,
}

impl M5ResolvedReproducibilityProofEntry {
    /// Whether this reproducibility-proof entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5BuildLaneResolutionError {
    /// The build-lane-descriptor-entry id was empty.
    EmptyBuildLaneDescriptorEntryId,
    /// The reproducibility-proof-entry id was empty.
    EmptyReproducibilityProofEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5BuildLaneResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyBuildLaneDescriptorEntryId => "empty_build_lane_descriptor_entry_id",
            Self::EmptyReproducibilityProofEntryId => "empty_reproducibility_proof_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5BuildLaneResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 build-lane-descriptor / reproducibility-proof registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5BuildLaneResolutionError {}

fn form_tokens(forms: &[M5BuildLaneResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5BuildLaneResolutionForm]) -> bool {
    let present: BTreeSet<M5BuildLaneResolutionForm> = forms.iter().copied().collect();
    M5BuildLaneResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved build-lane-descriptor object publishes every required field: classified cache posture,
/// cache read scope, cache write scope, credential class, publication rights, expected artifact families,
/// hermetic-input posture, and clean-room rebuild rule. An unclassified posture or any empty field never
/// resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn build_lane_descriptor_object_is_complete(
    posture: M5BuildLaneCachePostureKind,
    cache_read_scope: &str,
    cache_write_scope: &str,
    credential_class: &str,
    publication_rights: &str,
    expected_artifact_families: &str,
    hermetic_input_posture: &str,
    clean_room_rebuild_rule: &str,
) -> bool {
    posture.is_classified()
        && !cache_read_scope.trim().is_empty()
        && !cache_write_scope.trim().is_empty()
        && !credential_class.trim().is_empty()
        && !publication_rights.trim().is_empty()
        && !expected_artifact_families.trim().is_empty()
        && !hermetic_input_posture.trim().is_empty()
        && !clean_room_rebuild_rule.trim().is_empty()
}

/// Whether the build-lane descriptor keeps an untrusted lane from publishing: the posture must be classified,
/// the publication authority must be bounded (a PR / contributor lane never claims release publish rights), and
/// a trust-risk cache posture must disclose its cache-trust marker before the cache is read. An unclassified
/// posture, an unbounded publication authority, or a hidden cache-trust marker never matches.
pub fn untrusted_lane_cannot_publish(
    posture: M5BuildLaneCachePostureKind,
    publication_authority_bounded: bool,
    is_trust_risk_posture: bool,
    cache_trust_disclosed: bool,
) -> bool {
    posture.is_classified()
        && publication_authority_bounded
        && (!is_trust_risk_posture || cache_trust_disclosed)
}

/// Whether a reproducibility proof stays honest: the scope must be classified, the proof must be truthful, it
/// must keep the verified-versus-re-materialized input source visible, any remote-cache hit must be marked as
/// never being reproducibility proof rather than treated as proof, and any non-hermetic input must be flagged
/// rather than masquerade as hermetic.
pub fn reproducibility_proof_stays_honest(
    scope: M5ReproducibilityConvergenceScope,
    proof_is_truthful: bool,
    keeps_input_source_visible: bool,
    remote_cache_hit_present: bool,
    remote_cache_hit_marked_not_proof: bool,
    non_hermetic_input_present: bool,
    non_hermetic_input_flagged: bool,
) -> bool {
    scope.is_classified()
        && proof_is_truthful
        && keeps_input_source_visible
        && (!remote_cache_hit_present || remote_cache_hit_marked_not_proof)
        && (!non_hermetic_input_present || non_hermetic_input_flagged)
}

/// Resolves a build-lane-descriptor-registry entry so it stays bound to the build-lane-descriptor registry: the
/// entry names its canonical token, semantic role, and cache posture, covers all three resolution forms,
/// publishes a complete descriptor object (cache read scope, cache write scope, credential class, publication
/// rights, expected artifact families, hermetic-input posture, clean-room rebuild rule), bounds its publication
/// authority so an untrusted lane never publishes, and discloses the cache-trust posture before a trust-risk
/// cache is read.
pub fn resolve_build_lane_descriptor_entry(
    input: M5BuildLaneDescriptorEntryResolutionInput,
) -> Result<M5ResolvedBuildLaneDescriptorEntry, M5BuildLaneResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5BuildLaneResolutionError::EmptyBuildLaneDescriptorEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.lane_binding_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.cache_read_scope)
        || string_is_forbidden(&input.cache_write_scope)
        || string_is_forbidden(&input.credential_class)
        || string_is_forbidden(&input.publication_rights)
        || string_is_forbidden(&input.expected_artifact_families)
        || string_is_forbidden(&input.hermetic_input_posture)
        || string_is_forbidden(&input.clean_room_rebuild_rule)
    {
        return Err(M5BuildLaneResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = build_lane_descriptor_object_is_complete(
        input.cache_posture,
        &input.cache_read_scope,
        &input.cache_write_scope,
        &input.credential_class,
        &input.publication_rights,
        &input.expected_artifact_families,
        &input.hermetic_input_posture,
        &input.clean_room_rebuild_rule,
    );
    let cannot_publish_ok = untrusted_lane_cannot_publish(
        input.cache_posture,
        input.publication_authority_bounded,
        input.is_trust_risk_posture,
        input.cache_trust_disclosed,
    );
    let cache_trust_undisclosed = input.is_trust_risk_posture && !input.cache_trust_disclosed;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5BuildLaneDescriptorEntryDegradeReason::DescriptorTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5BuildLaneDescriptorEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.cache_posture.is_classified() {
        Some(M5BuildLaneDescriptorEntryDegradeReason::CachePostureUnclassified)
    } else if !input.bound_to_registry {
        Some(M5BuildLaneDescriptorEntryDegradeReason::DescriptorNotBoundToRegistry)
    } else if !object_complete {
        Some(M5BuildLaneDescriptorEntryDegradeReason::BuildLaneDescriptorObjectIncomplete)
    } else if !cannot_publish_ok {
        Some(M5BuildLaneDescriptorEntryDegradeReason::DescriptorLetsUntrustedLanePublishOrHidesCacheTrust)
    } else if !all_forms {
        Some(M5BuildLaneDescriptorEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if cache_trust_undisclosed {
        Some(M5BuildLaneDescriptorEntryDegradeReason::CacheTrustNotDisclosedForTrustRiskPosture)
    } else if !input.proof_fresh {
        Some(M5BuildLaneDescriptorEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5BuildLaneNextAction::ExpandLaneMeaning,
    };

    Ok(M5ResolvedBuildLaneDescriptorEntry {
        entry_id: input.entry_id,
        lane_binding_id: input.lane_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_verify_inputs_and_prove_replay_before_promotion: input
            .semantic_role
            .must_verify_inputs_and_prove_replay_before_promotion(),
        cache_posture: input.cache_posture.as_str().to_owned(),
        cache_posture_is_classified: input.cache_posture.is_classified(),
        canonical_cache_posture_mode: input
            .cache_posture
            .canonical_cache_posture_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        cache_read_scope: input.cache_read_scope,
        cache_write_scope: input.cache_write_scope,
        credential_class: input.credential_class,
        publication_rights: input.publication_rights,
        expected_artifact_families: input.expected_artifact_families,
        hermetic_input_posture: input.hermetic_input_posture,
        clean_room_rebuild_rule: input.clean_room_rebuild_rule,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        build_lane_descriptor_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        publication_authority_bounded: input.publication_authority_bounded,
        is_trust_risk_posture: input.is_trust_risk_posture,
        cache_trust_disclosed: input.cache_trust_disclosed,
        degrade_reason,
        next_action,
        descriptor_resolves_across_lanes: degrade_reason.is_none(),
    })
}

/// Resolves a reproducibility-proof entry so its proof stays safe: the entry names its canonical token,
/// semantic role, and convergence scope, covers all three resolution forms, provides the complete build-identity
/// / input-source-ledger / clean-room-diff / sidecar-convergence / attestation / rollback-metadata /
/// last-rebuild-revision reproducibility-proof object, and degrades honestly when the proof would treat a
/// remote-cache hit as reproducibility proof, hide the input source, or let a non-hermetic input masquerade as
/// hermetic.
pub fn resolve_reproducibility_proof_entry(
    input: M5ReproducibilityProofEntryResolutionInput,
) -> Result<M5ResolvedReproducibilityProofEntry, M5BuildLaneResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5BuildLaneResolutionError::EmptyReproducibilityProofEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.proof_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_build_identity)
        || string_is_forbidden(&input.input_source_ledger)
        || string_is_forbidden(&input.clean_room_diff_reference)
        || string_is_forbidden(&input.sidecar_convergence_state)
        || string_is_forbidden(&input.attestation_state)
        || string_is_forbidden(&input.rollback_metadata_reference)
        || string_is_forbidden(&input.last_rebuild_revision)
    {
        return Err(M5BuildLaneResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = reproducibility_proof_stays_honest(
        input.convergence_scope,
        input.proof_is_truthful,
        input.keeps_input_source_visible,
        input.remote_cache_hit_present,
        input.remote_cache_hit_marked_not_proof,
        input.non_hermetic_input_present,
        input.non_hermetic_input_flagged,
    );
    let provides_record = input.convergence_scope.is_classified()
        && !input.resolved_build_identity.trim().is_empty()
        && !input.input_source_ledger.trim().is_empty()
        && !input.clean_room_diff_reference.trim().is_empty()
        && !input.sidecar_convergence_state.trim().is_empty()
        && !input.attestation_state.trim().is_empty()
        && !input.rollback_metadata_reference.trim().is_empty()
        && !input.last_rebuild_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5ReproducibilityProofEntryDegradeReason::ProofTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ReproducibilityProofEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.convergence_scope.is_classified() {
        Some(M5ReproducibilityProofEntryDegradeReason::ConvergenceScopeUnclassified)
    } else if !provides_record {
        Some(M5ReproducibilityProofEntryDegradeReason::ReproducibilityProofTreatsCacheHitAsProofOrDriftsBuildIdentity)
    } else if !all_forms {
        Some(M5ReproducibilityProofEntryDegradeReason::ProofFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5ReproducibilityProofEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5BuildLaneNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedReproducibilityProofEntry {
        entry_id: input.entry_id,
        proof_ref: input.proof_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_verify_inputs_and_prove_replay_before_promotion: input
            .semantic_role
            .must_verify_inputs_and_prove_replay_before_promotion(),
        convergence_scope: input.convergence_scope.as_str().to_owned(),
        convergence_scope_is_classified: input.convergence_scope.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        resolved_build_identity: input.resolved_build_identity,
        input_source_ledger: input.input_source_ledger,
        clean_room_diff_reference: input.clean_room_diff_reference,
        sidecar_convergence_state: input.sidecar_convergence_state,
        attestation_state: input.attestation_state,
        rollback_metadata_reference: input.rollback_metadata_reference,
        last_rebuild_revision: input.last_rebuild_revision,
        keeps_input_source_visible: input.keeps_input_source_visible,
        proof_is_truthful: input.proof_is_truthful,
        remote_cache_hit_present: input.remote_cache_hit_present,
        remote_cache_hit_marked_not_proof: input.remote_cache_hit_marked_not_proof,
        non_hermetic_input_present: input.non_hermetic_input_present,
        non_hermetic_input_flagged: input.non_hermetic_input_flagged,
        reproducibility_proof_stays_honest: record_stays_honest,
        provides_complete_reproducibility_proof: provides_record,
        degrade_reason,
        next_action,
        proof_safe_on_every_lane: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved build-lane-descriptor and reproducibility-proof
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneDescriptorReproducibilityProofRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5BuildLaneDescriptorReproducibilityProofRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5BuildLaneQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Build contexts this row keeps the same truth across.
    pub deployment_lines: Vec<M5BuildLaneDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5BuildLaneRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5BuildLaneAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5BuildLaneAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5BuildLaneExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5BuildLaneDowngradeTrigger>,
    /// Resolved build-lane-descriptor-registry examples.
    pub build_lane_descriptor_entries: Vec<M5ResolvedBuildLaneDescriptorEntry>,
    /// Resolved reproducibility-proof examples.
    pub reproducibility_proof_entries: Vec<M5ResolvedReproducibilityProofEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the build-lane-descriptor and
    /// reproducibility-proof domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never lets a PR / contributor lane publish release artifacts. MUST be `false`.
    pub lets_a_pr_or_contributor_lane_publish_release_artifacts: bool,
    /// Hard invariant: this row never treats a remote-cache hit as reproducibility proof. MUST be `false`.
    pub treats_a_remote_cache_hit_as_reproducibility_proof: bool,
    /// Hard invariant: this row never hides the cache / credential / publication boundary before promotion.
    /// MUST be `false`.
    pub hides_the_cache_credential_or_publication_boundary_before_promotion: bool,
    /// Hard invariant: this row never collapses distinct build-lane input sources into one path. MUST be
    /// `false`.
    pub collapses_distinct_build_lane_input_sources_into_one_path: bool,
}

impl M5BuildLaneDescriptorReproducibilityProofRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5BuildLaneAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5BuildLaneAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5BuildLaneExportField> =
            self.export_fields.iter().copied().collect();
        M5BuildLaneExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.lets_a_pr_or_contributor_lane_publish_release_artifacts
            && !self.treats_a_remote_cache_hit_as_reproducibility_proof
            && !self.hides_the_cache_credential_or_publication_boundary_before_promotion
            && !self.collapses_distinct_build_lane_input_sources_into_one_path
    }

    /// True when a clean build-lane-descriptor entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified cache posture, publishes a complete descriptor object, bounds its publication
    /// authority, covers all three resolution forms, and discloses the cache-trust marker for a trust-risk
    /// posture.
    fn descriptor_is_honest(ex: &M5ResolvedBuildLaneDescriptorEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.cache_posture_is_classified
                && ex.build_lane_descriptor_object_complete
                && ex.publication_authority_bounded
                && ex.covers_all_resolution_forms
                && (!ex.is_trust_risk_posture || ex.cache_trust_disclosed))
    }

    /// True when a clean reproducibility-proof entry preserves a safe proof: it keeps a classified convergence
    /// scope, provides the complete reproducibility-proof object, stays honest, and covers all three resolution
    /// forms.
    fn proof_is_honest(ex: &M5ResolvedReproducibilityProofEntry) -> bool {
        !ex.is_clean()
            || (ex.convergence_scope_is_classified
                && ex.provides_complete_reproducibility_proof
                && ex.reproducibility_proof_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.build_lane_descriptor_entries
            .iter()
            .all(Self::descriptor_is_honest)
            && self
                .reproducibility_proof_entries
                .iter()
                .all(Self::proof_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneDescriptorReproducibilityProofRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Cache-posture tokens (minted by this lane).
    pub cache_posture_kinds: Vec<String>,
    /// Convergence-scope tokens (minted by this lane).
    pub convergence_scopes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Build-lane-descriptor-entry degrade-reason tokens.
    pub build_lane_descriptor_degrade_reasons: Vec<String>,
    /// Reproducibility-proof-entry degrade-reason tokens.
    pub reproducibility_proof_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5BuildLaneDescriptorReproducibilityProofRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5BuildLaneTrustRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5BuildLaneResolutionForm::ALL, |v| v.as_str()),
            cache_posture_kinds: tokens(&M5BuildLaneCachePostureKind::ALL, |v| v.as_str()),
            convergence_scopes: tokens(&M5ReproducibilityConvergenceScope::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5BuildLaneSurfaceContext::ALL, |v| v.as_str()),
            build_lane_descriptor_degrade_reasons: tokens(
                &M5BuildLaneDescriptorEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            reproducibility_proof_degrade_reasons: tokens(
                &M5ReproducibilityProofEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5BuildLaneAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5BuildLaneNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5BuildLaneExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5BuildLaneConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5BuildLaneDescriptorReproducibilityProofRegistriesGovernanceReview {
    /// The descriptor registry names a canonical token, semantic role, and cache posture for every entry.
    pub descriptor_registry_names_token_role_and_posture: bool,
    /// Every claimed lane resolves to one typed build-lane-descriptor object from the shared registry, not
    /// per-entry reconstruction.
    pub lane_resolves_to_typed_descriptor_from_shared_registry: bool,
    /// The cache read / write scopes, credential class, publication rights, and expected artifact families are
    /// published for every resolved descriptor.
    pub cache_credential_publication_and_artifact_families_published: bool,
    /// Untrusted lanes cannot publish release artifacts; a PR / contributor lane never claims publish rights.
    pub untrusted_lanes_cannot_publish_release_artifacts: bool,
    /// The reproducibility proof keeps the verified-versus-re-materialized input source visible and marks
    /// remote-cache hits as never being proof.
    pub reproducibility_proof_keeps_input_source_visible_and_marks_cache_hits_not_proof: bool,
    /// The cache-trust marker is disclosed before any trust-risk cache is read.
    pub cache_trust_disclosed_for_trust_risk_postures: bool,
    /// Every build-lane-descriptor and reproducibility-proof entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Build-lane-descriptor and reproducibility-proof behavior stay bound to the shared registries rather than
    /// hand-copied per lane.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Release center, shiproom, diagnostics, and provenance read a single build-lane source.
    pub release_center_shiproom_diagnostics_and_provenance_read_single_source: bool,
    /// A publish-from-untrusted-lane attempt, an incomplete object, or a hidden input source is caught by
    /// fixtures before release evidence turns green.
    pub descriptor_or_proof_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneDescriptorReproducibilityProofRegistriesConsumerProjection {
    /// Release center and shiproom consume the shared build-lane-descriptor registry.
    pub release_center_and_shiproom_consume_shared_registries: bool,
    /// Diagnostics and provenance consume the shared reproducibility-proof registry.
    pub diagnostics_and_provenance_consume_shared_registries: bool,
    /// Build farm and cache service consume the shared registries.
    pub build_farm_and_cache_service_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical build-lane-descriptor and reproducibility-proof domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical build-lane-descriptor / reproducibility-proof registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneDescriptorReproducibilityProofRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneDescriptorReproducibilityProofRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting build-lane audit for the lane.
    pub build_lane_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BuildLaneDescriptorReproducibilityProofRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BuildLaneDescriptorReproducibilityProofRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5BuildLaneDescriptorReproducibilityProofRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BuildLaneDescriptorReproducibilityProofRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BuildLaneDescriptorReproducibilityProofRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BuildLaneDescriptorReproducibilityProofRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BuildLaneDescriptorReproducibilityProofRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BuildLaneDescriptorReproducibilityProofRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 build-lane-descriptor and reproducibility-proof registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildLaneDescriptorReproducibilityProofRegistriesPacket {
    /// Record kind; must equal
    /// [`M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5BuildLaneDescriptorReproducibilityProofRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BuildLaneDescriptorReproducibilityProofRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BuildLaneDescriptorReproducibilityProofRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BuildLaneDescriptorReproducibilityProofRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BuildLaneDescriptorReproducibilityProofRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BuildLaneDescriptorReproducibilityProofRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BuildLaneDescriptorReproducibilityProofRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5BuildLaneDescriptorReproducibilityProofRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_RECORD_KIND
                .to_owned(),
            schema_version:
                M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registries_label: input.registries_label,
            registry_rows: input.registry_rows,
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

    /// Validates the registries-packet invariants.
    pub fn validate(&self) -> Vec<M5BuildLaneDescriptorReproducibilityProofRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_RECORD_KIND
        {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::WrongRecordKind,
            );
        }
        if self.schema_version
            != M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::MissingIdentity,
            );
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::VocabularySetDrift,
            );
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(&serde_json::to_value(self).expect(
            "m5 build-lane-descriptor / reproducibility-proof registries packet serializes",
        )) {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::RawMaterialInExport,
            );
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
            .expect("m5 build-lane-descriptor / reproducibility-proof registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,build_lane_descriptor_entries,reproducibility_proof_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .build_lane_descriptor_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.reproducibility_proof_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.build_lane_descriptor_entries.len(),
                row.reproducibility_proof_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Build-Lane-Descriptor and Reproducibility-Proof Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Cache postures: {}\n",
            self.vocabulary_set.cache_posture_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Resolution forms: {}\n",
            self.vocabulary_set.resolution_forms.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.registry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Build-lane-descriptor entries: {} / reproducibility-proof entries: {}\n",
                row.build_lane_descriptor_entries.len(),
                row.reproducibility_proof_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry build-lane reference table generated from the registry, so docs and shiproom
    /// runbooks render the same posture-mode / cache-read / cache-write / credential-class / publication-rights
    /// / artifact-families truth the resolvers produced rather than a hand-copied lane table. Only clean,
    /// registry-bound build-lane-descriptor entries are listed.
    pub fn render_build_lane_descriptor_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| lane_binding_id | posture_mode | cache_read_scope | cache_write_scope | credential_class | publication_rights | expected_artifact_families |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.build_lane_descriptor_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.lane_binding_id,
                    ex.canonical_cache_posture_mode,
                    ex.cache_read_scope,
                    ex.cache_write_scope,
                    ex.credential_class,
                    ex.publication_rights,
                    ex.expected_artifact_families
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5BuildLaneDescriptorReproducibilityProofRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BuildLaneDescriptorReproducibilityProofRegistriesViolation>),
}

impl fmt::Display for M5BuildLaneDescriptorReproducibilityProofRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 build-lane-descriptor / reproducibility-proof registries export parse failed: {error}"
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
                    "m5 build-lane-descriptor / reproducibility-proof registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BuildLaneDescriptorReproducibilityProofRegistriesArtifactError {}

/// Validation failures emitted by
/// [`M5BuildLaneDescriptorReproducibilityProofRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BuildLaneDescriptorReproducibilityProofRegistriesViolation {
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
    /// The registries packet declares no rows.
    NoRegistryRows,
    /// A registry row is incomplete.
    RegistryRowIncomplete,
    /// A registry row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A registry row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A registry row does not point at both the build-lane-descriptor and reproducibility-proof domain
    /// schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, publish-from-untrusted, field-incomplete,
    /// form-incomplete, or a reproducibility-proof entry missing the complete proof object).
    DishonestExample,
    /// A registry row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Build-lane-descriptor-resolution is not proven: clean descriptor entries do not cover the canonical
    /// cache postures or the first release-center / shiproom / diagnostics / provenance / support surfaces, no
    /// object-incomplete example degrades, or a clean descriptor entry published an incomplete object.
    BuildLaneDescriptorResolutionNotProven,
    /// Publication-boundary-preservation is not proven: no publish-from-untrusted example and no unbound
    /// example degrade, no clean bounded descriptor entry is present, or a clean descriptor entry is unbounded
    /// or unbound.
    PublicationBoundaryPreservationNotProven,
    /// Reproducibility-proof-integrity is not proven: clean proof entries do not cover the canonical
    /// verified-cache / re-materialized / hermetic-rebuild scopes with full resolution-form coverage while
    /// providing the complete proof object, no cache-hit-as-proof or form-incomplete example degrades, or a
    /// clean proof entry is missing the complete proof object.
    ReproducibilityProofIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BuildLaneDescriptorReproducibilityProofRegistriesViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoRegistryRows => "no_registry_rows",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::BuildLaneDescriptorResolutionNotProven => {
                "build_lane_descriptor_resolution_not_proven"
            }
            Self::PublicationBoundaryPreservationNotProven => {
                "publication_boundary_preservation_not_proven"
            }
            Self::ReproducibilityProofIntegrityNotProven => {
                "reproducibility_proof_integrity_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_build_lane_descriptor_and_reproducibility_proof_registries_export(
) -> Result<
    M5BuildLaneDescriptorReproducibilityProofRegistriesPacket,
    M5BuildLaneDescriptorReproducibilityProofRegistriesArtifactError,
> {
    let packet: M5BuildLaneDescriptorReproducibilityProofRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-build-lane-descriptor-and-reproducibility-proof-registries-proof/support_export.json"
        )
    ))
    .map_err(M5BuildLaneDescriptorReproducibilityProofRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(
            M5BuildLaneDescriptorReproducibilityProofRegistriesArtifactError::Validation(
                violations,
            ),
        )
    }
}

fn validate_source_contracts(
    packet: &M5BuildLaneDescriptorReproducibilityProofRegistriesPacket,
    violations: &mut Vec<M5BuildLaneDescriptorReproducibilityProofRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_SCHEMA_REF,
        M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_DOC_REF,
        M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
        M5_BUILD_LANE_TRUST_MATRIX_DOC_REF,
        M5_BUILD_LANE_DESCRIPTOR_DOMAIN_SCHEMA_REF,
        M5_REPRODUCIBILITY_PROOF_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5BuildLaneDescriptorReproducibilityProofRegistriesPacket,
    violations: &mut Vec<M5BuildLaneDescriptorReproducibilityProofRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations
            .push(M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::NoRegistryRows);
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_BUILD_LANE_DESCRIPTOR_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_REPRODUCIBILITY_PROOF_DOMAIN_SCHEMA_REF)
        {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.build_lane_descriptor_entries.is_empty()
            || row.reproducibility_proof_entries.is_empty()
        {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::ExamplesMissing,
            );
        }
        if !row.examples_are_honest() {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::DishonestExample,
            );
        }
        if !row.honours_invariants() {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5BuildLaneDescriptorReproducibilityProofRegistriesPacket,
    violations: &mut Vec<M5BuildLaneDescriptorReproducibilityProofRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.descriptor_registry_names_token_role_and_posture,
        review.lane_resolves_to_typed_descriptor_from_shared_registry,
        review.cache_credential_publication_and_artifact_families_published,
        review.untrusted_lanes_cannot_publish_release_artifacts,
        review.reproducibility_proof_keeps_input_source_visible_and_marks_cache_hits_not_proof,
        review.cache_trust_disclosed_for_trust_risk_postures,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.release_center_shiproom_diagnostics_and_provenance_read_single_source,
        review.descriptor_or_proof_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BuildLaneDescriptorReproducibilityProofRegistriesPacket,
    violations: &mut Vec<M5BuildLaneDescriptorReproducibilityProofRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.release_center_and_shiproom_consume_shared_registries,
        projection.diagnostics_and_provenance_consume_shared_registries,
        projection.build_farm_and_cache_service_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5BuildLaneDescriptorReproducibilityProofRegistriesPacket,
    violations: &mut Vec<M5BuildLaneDescriptorReproducibilityProofRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5BuildLaneDescriptorReproducibilityProofRegistriesPacket,
    violations: &mut Vec<M5BuildLaneDescriptorReproducibilityProofRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.build_lane_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5BuildLaneDescriptorReproducibilityProofRegistriesPacket,
    violations: &mut Vec<M5BuildLaneDescriptorReproducibilityProofRegistriesViolation>,
) {
    let descriptors = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.build_lane_descriptor_entries.iter())
    };
    let proofs = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.reproducibility_proof_entries.iter())
    };

    // AC1: every lane exposes a typed descriptor with cache, credential, and publication boundaries. Clean
    // descriptor entries cover the canonical cache postures and the first release-center / shiproom /
    // diagnostics / provenance / support surfaces, an object-incomplete example degrades, and no clean
    // descriptor entry published an incomplete object.
    let clean_postures: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.cache_posture.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let postures_covered = M5BuildLaneCachePostureKind::CANONICAL_POSTURES
        .iter()
        .all(|k| clean_postures.contains(k.as_str()));
    let first_surfaces_covered = M5BuildLaneSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(M5BuildLaneDescriptorEntryDegradeReason::BuildLaneDescriptorObjectIncomplete)
    });
    let no_clean_incomplete =
        !descriptors().any(|ex| ex.is_clean() && !ex.build_lane_descriptor_object_complete);
    if !(postures_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::BuildLaneDescriptorResolutionNotProven,
        );
    }

    // AC2: attempting to publish from an untrusted lane fails with a structured blocker reason. A
    // publish-from-untrusted example degrades, an unbound example degrades, at least one clean bounded
    // descriptor entry is present, and no clean descriptor entry is unbounded or unbound.
    let publish_fold_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(
                M5BuildLaneDescriptorEntryDegradeReason::DescriptorLetsUntrustedLanePublishOrHidesCacheTrust,
            )
    });
    let unbound_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(M5BuildLaneDescriptorEntryDegradeReason::DescriptorNotBoundToRegistry)
    });
    let bounded_clean_descriptor =
        descriptors().any(|ex| ex.is_clean() && ex.publication_authority_bounded);
    let no_clean_unbound = !descriptors().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded =
        !descriptors().any(|ex| ex.is_clean() && !ex.publication_authority_bounded);
    if !(publish_fold_degrades
        && unbound_degrades
        && bounded_clean_descriptor
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations.push(
            M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::PublicationBoundaryPreservationNotProven,
        );
    }

    // AC3: release packets can prove which lane produced each claimed artifact family. Clean proof entries
    // cover every canonical verified-cache / re-materialized / hermetic-rebuild convergence scope with full
    // resolution-form coverage while providing the complete proof object, a cache-hit-as-proof example
    // degrades, a form-incomplete example degrades, and no clean proof entry is missing the complete object.
    let clean_proof_scopes: BTreeSet<String> = proofs()
        .filter(|ex| {
            ex.is_clean()
                && ex.convergence_scope_is_classified
                && ex.provides_complete_reproducibility_proof
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.convergence_scope.clone())
        .collect();
    let proof_scopes_covered = M5ReproducibilityConvergenceScope::CANONICAL_SCOPES
        .iter()
        .all(|m| clean_proof_scopes.contains(m.as_str()));
    let cache_hit_degrades = proofs().any(|ex| {
        ex.degrade_reason
            == Some(
                M5ReproducibilityProofEntryDegradeReason::ReproducibilityProofTreatsCacheHitAsProofOrDriftsBuildIdentity,
            )
    });
    let form_incomplete_degrades = proofs().any(|ex| {
        ex.degrade_reason
            == Some(M5ReproducibilityProofEntryDegradeReason::ProofFormCoverageIncomplete)
    });
    let no_clean_missing_proof =
        !proofs().any(|ex| ex.is_clean() && !ex.provides_complete_reproducibility_proof);
    if !(proof_scopes_covered
        && cache_hit_degrades
        && form_incomplete_degrades
        && no_clean_missing_proof)
    {
        violations.push(
            M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::ReproducibilityProofIntegrityNotProven,
        );
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

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The build lanes this lane implements, for downstream reference: the build-lane-descriptor registry covers
/// the contributor / PR and protected-merge lanes, and the reproducibility-proof registry covers the release
/// and emergency-hotfix lanes.
pub const IMPLEMENTED_FAMILIES: [M5BuildLaneFamily; 4] = M5BuildLaneFamily::ALL;
