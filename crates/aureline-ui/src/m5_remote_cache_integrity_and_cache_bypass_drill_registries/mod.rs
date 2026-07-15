//! Implemented M5 clean-room-rebuild-lane and artifact-diff-packet registries.
//!
//! The frozen [build-lane-trust matrix][matrix] names Aureline's four governed build lanes — the contributor /
//! PR lane, the protected-merge lane, the release lane, and the emergency-hotfix lane — and locks their
//! controlled vocabulary. This module is the input-materialization and artifact-diff implement lane over
//! that matrix: it turns the *clean-room-rebuild-lane* grammar (how a lane captures the build-config digest, the
//! materialized-input receipt, the input provenance ledger, the verification authority it is bounded to, the
//! artifact families it expects, the hermetic-input posture, and the re-materialization rule so an unverified or
//! non-materialized input never silently enters a protected lane) and the *artifact-diff-packet* grammar
//! (how a release or emergency-hotfix lane proves that binaries, packages, docs packs, schemas, SBOMs, symbols,
//! source maps, and rollback metadata are all present and bound to one exact build identity, so a missing or
//! mismatched sidecar is a blocker rather than a warning) into registry resolvers that produce export-safe,
//! honest projections. Every claimed M5 build lane then resolves to one typed clean-room-rebuild-lane object —
//! the input source it classifies, the build-config digest, the materialized-input receipt, the input provenance
//! ledger, the verification authority it is bounded to (never admitting an unverified or non-materialized input
//! into a protected lane), the expected artifact families, the hermetic-input posture, and the re-materialization
//! rule — and to one artifact-diff-packet object — the resolved exact build identity, the claimed
//! artifact families, the sidecar-family ledger, the binding-identity check, the missing-or-mismatched reference,
//! the attestation state, and the last convergence revision — that the release-center, shiproom, diagnostics,
//! provenance, and support / export surfaces can inspect without manual reconstruction, so an unverified input can
//! never enter a protected lane, a missing or mismatched sidecar is never treated as a warning-only state, the
//! build-config-digest / receipt / verification boundary stays visible before promotion, and a build lane that
//! cannot explain the manifest it declared or prove its sidecars converge on one build identity degrades honestly
//! instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one typed clean-room-rebuild-lane object per lane.** [`resolve_remote_cache_integrity_finding_entry`]
//!   refuses to read as a clean, registry-bound manifest entry unless it names a canonical registry token, a
//!   classified [input source][M5RemoteCacheOriginKind], a build-lane-trust role, covers every [resolution
//!   form][M5CacheDisciplineResolutionForm] (the canonical object, the accessible summary, and the audit record),
//!   publishes every manifest field (build-config digest, materialized-input receipt, input provenance ledger,
//!   verification authority, expected artifact families, hermetic-input posture, and re-materialization rule),
//!   bounds its verification authority so an unverified input never enters a protected lane, and discloses the
//!   input-trust marker before a trust-risk input is admitted; otherwise it degrades.
//! * **Keep an unverified input from entering a protected lane.** [`shared_cache_cannot_authorize_rebuild`]
//!   rejects a manifest entry whose verification authority is unbounded (an unverified or non-materialized input
//!   claiming protected-lane admission) so it degrades to
//!   [`M5RemoteCacheIntegrityFindingEntryDegradeReason::LaneReliesOnSharedCacheOrHidesReplayReceipt`], and a trust-risk
//!   input source that hides its input-trust marker degrades the same way — the structured blocker reason an
//!   admit-unclean-room-rebuild attempt must surface.
//! * **Keep the artifact-diff packet from omitting a family or drifting the build identity.**
//!   [`resolve_cache_bypass_drill_entry`] names a classified [convergence
//!   scope][M5CacheBypassDrillScope], requires the full build-identity / claimed-families / sidecar-ledger /
//!   binding-identity / missing-or-mismatched / attestation / last-convergence-revision artifact-diff
//!   object, covers every resolution form, and degrades to
//!   [`M5CacheBypassDrillEntryDegradeReason::ArtifactDiffDivergesOrOmitsFamilyOrDriftsBuildIdentity`]
//!   when the manifest would let a green build omit a claimed sidecar family, bind a sidecar to a different build
//!   identity, or treat a missing or mismatched sidecar as warning-only, so a artifact-diff packet can
//!   never read as trustworthy when it has quietly dropped a docs, schema, SBOM, symbol, source-map, or
//!   rollback-metadata sidecar from the exact-build story.
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
    seeded_m5_remote_cache_integrity_and_cache_bypass_drill_registries,
    seeded_m5_remote_cache_integrity_and_cache_bypass_drill_registries_artifact_diff_preview_narrowed,
    seeded_m5_remote_cache_integrity_and_cache_bypass_drill_registries_hermetic_rebuild_beta_narrowed,
    M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_build_lane_trust_matrix::{
    M5BuildLaneAccessibilityRoute, M5BuildLaneConsumerSurface, M5BuildLaneDeploymentLine,
    M5BuildLaneDowngradeTrigger, M5BuildLaneFamily, M5BuildLaneQualificationClass,
    M5BuildLaneRequiredLabel, M5BuildLaneTrustRole, M5_BUILD_LANE_TRUST_MATRIX_DOC_REF,
    M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5CacheIntegrityBypassRegistriesPacket`].
pub const M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_remote_cache_integrity_and_cache_bypass_drill_registries";

/// Schema version for M5 clean-room-rebuild-lane / artifact-diff-packet registry records.
pub const M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_SCHEMA_REF: &str =
    "schemas/release/m5-remote-cache-integrity-and-cache-bypass-drill-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_DOC_REF: &str =
    "docs/release/m5_remote_cache_integrity_and_cache_bypass_drill_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-remote-cache-integrity-and-cache-bypass-drill-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-remote-cache-integrity-and-cache-bypass-drill-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-remote-cache-integrity-and-cache-bypass-drill-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/release/m5-remote-cache-integrity-and-cache-bypass-drill-registries";

/// Repo-relative path of the canonical clean-room-rebuild-lane domain schema (build-config digest,
/// materialized-input receipt, input provenance ledger, and verification authority of a lane's inputs).
pub const M5_REMOTE_CACHE_INTEGRITY_FINDING_DOMAIN_SCHEMA_REF: &str =
    "schemas/release/m5-remote-cache-integrity-finding.schema.json";

/// Repo-relative path of the canonical artifact-diff-packet domain schema (claimed artifact families,
/// sidecar-family ledger, and convergence of every sidecar on one exact build identity).
pub const M5_CACHE_BYPASS_DRILL_DOMAIN_SCHEMA_REF: &str =
    "schemas/release/m5-cache-bypass-drill.schema.json";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5CacheIntegrityBypassRegistriesConsumerSurface = M5BuildLaneConsumerSurface;

/// One of the three resolution forms every clean-room-rebuild-lane or artifact-diff-packet entry must
/// hold across so its truth keeps whether it is shown as the canonical resolved object, announced as an
/// accessible summary, or written to the audit / support record. Minted by this lane because the frozen matrix
/// names the build lanes but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CacheDisciplineResolutionForm {
    /// The canonical resolved clean-room-rebuild-lane / artifact-diff-packet object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved lane discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved lane inspectable off-renderer.
    AuditRecord,
}

impl M5CacheDisciplineResolutionForm {
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

/// Controlled input-source class a clean-room-rebuild-lane entry declares, so the typed manifest model shares
/// one registry rather than a hand-copied per-lane assumption. Minted by this lane because the frozen matrix
/// carries the build lanes but not the concrete re-materialized / verified-cache / pinned-digest /
/// unverified-external / non-materialized input source a manifest classifies against. Every classified source
/// carries its canonical mode, and the unverified-external and non-materialized sources are trust-risk-bearing so
/// their input-trust marker must be disclosed before the input is admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RemoteCacheOriginKind {
    /// A fully re-materialized input rebuilt from source with a build-config digest.
    HermeticCleanRoomRebuild,
    /// A verified remote-cache input whose digest is checked before use.
    RematerializedInputReplay,
    /// An input pinned to a checked-in build-config digest and materialized-input receipt.
    PinnedDigestReplay,
    /// An external input that is not yet verified (trust-risk; it may hide a missing or poisoned input).
    SharedCacheShortcut,
    /// A referenced input that has not been materialized (trust-risk; the receipt is not yet proven).
    UnreplayableReference,
    /// The input source is unclassified, which is disallowed.
    SourceUnclassified,
}

impl M5RemoteCacheOriginKind {
    /// Every input source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HermeticCleanRoomRebuild,
        Self::RematerializedInputReplay,
        Self::PinnedDigestReplay,
        Self::SharedCacheShortcut,
        Self::UnreplayableReference,
        Self::SourceUnclassified,
    ];

    /// The five canonical input sources every claimed M5 build lane classifies against.
    pub const CANONICAL_SOURCES: [Self; 5] = [
        Self::HermeticCleanRoomRebuild,
        Self::RematerializedInputReplay,
        Self::PinnedDigestReplay,
        Self::SharedCacheShortcut,
        Self::UnreplayableReference,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HermeticCleanRoomRebuild => "hermetic_clean_room_rebuild",
            Self::RematerializedInputReplay => "rematerialized_input_replay",
            Self::PinnedDigestReplay => "pinned_digest_replay",
            Self::SharedCacheShortcut => "shared_cache_shortcut",
            Self::UnreplayableReference => "unreplayable_reference",
            Self::SourceUnclassified => "source_unclassified",
        }
    }

    /// Whether the source is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::SourceUnclassified)
    }

    /// The canonical mode for this input source.
    pub const fn canonical_rebuild_source_mode(self) -> &'static str {
        match self {
            Self::HermeticCleanRoomRebuild => "hermetic_clean_room_rebuild_mode",
            Self::RematerializedInputReplay => "rematerialized_input_replay_mode",
            Self::PinnedDigestReplay => "pinned_digest_replay_mode",
            Self::SharedCacheShortcut => "shared_cache_shortcut_mode",
            Self::UnreplayableReference => "unreplayable_reference_mode",
            Self::SourceUnclassified => "",
        }
    }

    /// Whether this source is trust-risk-bearing and so must disclose the input-trust marker before the input
    /// is admitted.
    pub const fn is_replay_trust_risk_source(self) -> bool {
        matches!(
            self,
            Self::SharedCacheShortcut | Self::UnreplayableReference
        )
    }
}

/// Controlled convergence scope a artifact-diff-packet entry must resolve its sidecar families from,
/// so a manifest shares one registry rather than a hand-copied per-record assumption. Minted by this lane,
/// tracking whether the sidecars converged on the binary build identity, were reconciled against the
/// materialized-input receipt, or came from a hermetic rebuild the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CacheBypassDrillScope {
    /// Every claimed sidecar converged on the exact binary build identity.
    ByteIdenticalDiff,
    /// Every claimed sidecar was reconciled against the materialized-input receipt.
    NormalizedEquivalentDiff,
    /// Every claimed sidecar came from a hermetic clean-room rebuild.
    HermeticRebuildDiff,
    /// The convergence scope is unclassified, which is disallowed.
    ScopeUnclassified,
}

impl M5CacheBypassDrillScope {
    /// Every convergence scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ByteIdenticalDiff,
        Self::NormalizedEquivalentDiff,
        Self::HermeticRebuildDiff,
        Self::ScopeUnclassified,
    ];

    /// The three canonical convergence scopes every artifact-diff packet must stay distinct across.
    pub const CANONICAL_SCOPES: [Self; 3] = [
        Self::ByteIdenticalDiff,
        Self::NormalizedEquivalentDiff,
        Self::HermeticRebuildDiff,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ByteIdenticalDiff => "byte_identical_diff",
            Self::NormalizedEquivalentDiff => "normalized_equivalent_diff",
            Self::HermeticRebuildDiff => "hermetic_rebuild_diff",
            Self::ScopeUnclassified => "scope_unclassified",
        }
    }

    /// Whether the convergence scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ScopeUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a clean-room-rebuild-lane
/// or artifact-diff-packet token's meaning stays stable whether it appears in the release-center,
/// shiproom, diagnostics, provenance, or a support / export form. Minted by this lane, tracking the
/// first-consumer surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CacheDisciplineSurfaceContext {
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

impl M5CacheDisciplineSurfaceContext {
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

/// One mandatory rendered part a clean-room-rebuild-lane or artifact-diff-packet entry must be able to
/// show, so no input source, build-config digest, receipt, artifact family, artifact-diff field, or
/// registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CacheDisciplineAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The input source the entry classifies (clean-room-rebuild-lane entry).
    RebuildSource,
    /// The build-config digest, materialized-input receipt, verification authority, and expected artifact
    /// families the entry publishes (clean-room-rebuild-lane entry).
    RebuildConfigDigestAndReplayBoundaries,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The expected artifact families the entry publishes (clean-room-rebuild-lane entry).
    ExpectedArtifactFamilies,
    /// The artifact-diff fields (build identity, claimed families, sidecar ledger, binding-identity
    /// check, missing-or-mismatched reference, attestation) the entry publishes (artifact-diff entry).
    ArtifactDiffFields,
    /// The support-identity hint the entry publishes (artifact-diff entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved clean-room rebuild lane or artifact-diff packet (both
    /// entries).
    PlainLanguageMeaning,
}

impl M5CacheDisciplineAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::RebuildSource,
        Self::RebuildConfigDigestAndReplayBoundaries,
        Self::ResolutionFormCoverage,
        Self::ExpectedArtifactFamilies,
        Self::ArtifactDiffFields,
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
            Self::RebuildSource => "rebuild_source",
            Self::RebuildConfigDigestAndReplayBoundaries => {
                "rerebuild_config_digest_and_replay_boundaries"
            }
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::ExpectedArtifactFamilies => "expected_artifact_families",
            Self::ArtifactDiffFields => "artifact_diff_fields",
            Self::SupportIdentityHint => "support_identity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// clean-room rebuild lane, a artifact-diff packet, or a degraded clean-room-rebuild-lane /
/// artifact-diff-packet entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CacheDisciplineNextAction {
    /// Expand the resolved clean-room rebuild lane's or artifact-diff packet's plain-language meaning.
    ExpandRebuildMeaning,
    /// Inspect the input source or convergence scope the entry resolves.
    InspectSourceOrScope,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5CacheDisciplineNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandRebuildMeaning,
        Self::InspectSourceOrScope,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandRebuildMeaning => "expand_rebuild_meaning",
            Self::InspectSourceOrScope => "inspect_source_or_scope",
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
pub enum M5CacheDisciplineExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The build-lane families covered.
    BuildLaneFamilies,
    /// The input source kinds carried.
    RebuildSourceKinds,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The convergence scopes carried.
    DiffScopes,
    /// The render / surface context.
    SurfaceContext,
    /// The input-source modes carried.
    RebuildSourceModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5CacheDisciplineExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::BuildLaneFamilies,
        Self::RebuildSourceKinds,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::DiffScopes,
        Self::SurfaceContext,
        Self::RebuildSourceModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::BuildLaneFamilies,
        Self::RebuildSourceKinds,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::BuildLaneFamilies => "build_lane_families",
            Self::RebuildSourceKinds => "rebuild_source_kinds",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::DiffScopes => "diff_scopes",
            Self::SurfaceContext => "surface_context",
            Self::RebuildSourceModes => "rebuild_source_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a clean-room-rebuild-lane entry degraded below a clean, registry-bound state. The degrade-first
/// ladder returns one of these instead of ever letting a hand-copied, admit-unclean-room-rebuild, field-incomplete,
/// or form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RemoteCacheIntegrityFindingEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the manifest means.
    RegistryTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The input source is unclassified (not in the resolved taxonomy).
    RebuildSourceUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    LaneNotBoundToRegistry,
    /// The resolved clean-room-rebuild-lane object is incomplete: the build-config digest, materialized-input
    /// receipt, input provenance ledger, verification authority, expected artifact families, hermetic-input
    /// posture, or re-materialization rule is unstated.
    RemoteCacheIntegrityFindingObjectIncomplete,
    /// The lane's verification authority is unbounded (an unverified or non-materialized input claiming
    /// protected-lane admission), or a trust-risk input source hid its input-trust marker.
    LaneReliesOnSharedCacheOrHidesReplayReceipt,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A trust-risk input source did not disclose the input-trust marker before the input was admitted.
    CacheTrustNotDisclosedForSharedCacheSource,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RemoteCacheIntegrityFindingEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RegistryTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::RebuildSourceUnclassified,
        Self::LaneNotBoundToRegistry,
        Self::RemoteCacheIntegrityFindingObjectIncomplete,
        Self::LaneReliesOnSharedCacheOrHidesReplayReceipt,
        Self::ResolutionFormCoverageIncomplete,
        Self::CacheTrustNotDisclosedForSharedCacheSource,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegistryTokenUnstated => "registry_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::RebuildSourceUnclassified => "rebuild_source_unclassified",
            Self::LaneNotBoundToRegistry => "lane_not_bound_to_registry",
            Self::RemoteCacheIntegrityFindingObjectIncomplete => {
                "remote_cache_integrity_finding_object_incomplete"
            }
            Self::LaneReliesOnSharedCacheOrHidesReplayReceipt => {
                "lane_relies_on_shared_cache_or_hides_replay_receipt"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::CacheTrustNotDisclosedForSharedCacheSource => {
                "cache_trust_not_disclosed_for_shared_cache_source"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5CacheDisciplineNextAction {
        match self {
            Self::RegistryTokenUnstated | Self::LaneNotBoundToRegistry => {
                M5CacheDisciplineNextAction::TraceCanonicalRegistry
            }
            Self::RebuildSourceUnclassified
            | Self::RemoteCacheIntegrityFindingObjectIncomplete
            | Self::LaneReliesOnSharedCacheOrHidesReplayReceipt => {
                M5CacheDisciplineNextAction::InspectSourceOrScope
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5CacheDisciplineNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::CacheTrustNotDisclosedForSharedCacheSource
            | Self::ProofStale => M5CacheDisciplineNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5BuildLaneDowngradeTrigger {
        match self {
            Self::RegistryTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::LaneNotBoundToRegistry => {
                M5BuildLaneDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::RebuildSourceUnclassified | Self::RemoteCacheIntegrityFindingObjectIncomplete => {
                M5BuildLaneDowngradeTrigger::CachePostureUnstated
            }
            Self::LaneReliesOnSharedCacheOrHidesReplayReceipt
            | Self::CacheTrustNotDisclosedForSharedCacheSource => {
                M5BuildLaneDowngradeTrigger::HidNonHermeticInputsCachePoisoningOrUnreplayableArtifacts
            }
            Self::ProofStale => M5BuildLaneDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a artifact-diff-packet entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CacheBypassDrillEntryDegradeReason {
    /// The canonical registry token name is unstated.
    RegistryTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The convergence scope is unclassified (not in the resolved taxonomy).
    DiffScopeUnclassified,
    /// The artifact-diff packet would let a green build omit a claimed sidecar family, bind a sidecar
    /// to a different build identity, or treat a missing or mismatched sidecar as warning-only, or it dropped
    /// one of the required artifact-diff fields (build identity, claimed families, sidecar ledger,
    /// binding-identity check, missing-or-mismatched reference, attestation, last convergence revision).
    ArtifactDiffDivergesOrOmitsFamilyOrDriftsBuildIdentity,
    /// The canonical / accessible / audit resolution-form coverage of the manifest is incomplete.
    PacketFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CacheBypassDrillEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RegistryTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::DiffScopeUnclassified,
        Self::ArtifactDiffDivergesOrOmitsFamilyOrDriftsBuildIdentity,
        Self::PacketFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegistryTokenUnstated => "registry_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::DiffScopeUnclassified => "diff_scope_unclassified",
            Self::ArtifactDiffDivergesOrOmitsFamilyOrDriftsBuildIdentity => {
                "artifact_diff_diverges_or_omits_family_or_drifts_build_identity"
            }
            Self::PacketFormCoverageIncomplete => "packet_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5CacheDisciplineNextAction {
        match self {
            Self::RegistryTokenUnstated => M5CacheDisciplineNextAction::TraceCanonicalRegistry,
            Self::DiffScopeUnclassified
            | Self::ArtifactDiffDivergesOrOmitsFamilyOrDriftsBuildIdentity => {
                M5CacheDisciplineNextAction::InspectSourceOrScope
            }
            Self::PacketFormCoverageIncomplete => {
                M5CacheDisciplineNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5CacheDisciplineNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5BuildLaneDowngradeTrigger {
        match self {
            Self::RegistryTokenUnstated => M5BuildLaneDowngradeTrigger::RegistryReferenceUnstated,
            Self::SurfaceContextUnresolved | Self::DiffScopeUnclassified => {
                M5BuildLaneDowngradeTrigger::BuildIdentityUnstated
            }
            Self::ArtifactDiffDivergesOrOmitsFamilyOrDriftsBuildIdentity => {
                M5BuildLaneDowngradeTrigger::DriftedASidecarFromTheBinaryBuildIdentity
            }
            Self::PacketFormCoverageIncomplete => {
                M5BuildLaneDowngradeTrigger::CleanRoomProofRuleUnstated
            }
            Self::ProofStale => M5BuildLaneDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_remote_cache_integrity_finding_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RemoteCacheIntegrityFindingEntryResolutionInput {
    /// Stable identity of the clean-room-rebuild-lane-registry entry.
    pub entry_id: String,
    /// The stable lane-binding ID this manifest binds to (e.g. `release.lane.protected-merge`); empty means
    /// unstated.
    pub lane_binding_id: String,
    /// The canonical registry token name (e.g. `verified.input.manifest.protected_merge`); empty means unstated.
    pub token_name: String,
    /// The high-level build-lane-trust role (from the frozen matrix vocabulary).
    pub semantic_role: M5BuildLaneTrustRole,
    /// The input source this entry classifies.
    pub rebuild_source: M5RemoteCacheOriginKind,
    /// The render / surface context.
    pub surface_context: M5CacheDisciplineSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5CacheDisciplineResolutionForm>,
    /// The published build-config digest; empty means unstated.
    pub rebuild_config_digest: String,
    /// The published materialized-input receipt; empty means unstated.
    pub replay_receipt: String,
    /// The published input provenance ledger; empty means unstated.
    pub protected_input_ledger: String,
    /// The published verification authority; empty means unstated.
    pub rebuild_authority: String,
    /// The published expected artifact families; empty means unstated.
    pub expected_artifact_families: String,
    /// The published hermetic-input posture; empty means unstated.
    pub hermetic_rebuild_posture: String,
    /// The published re-materialization rule; empty means unstated.
    pub shared_cache_isolation_rule: String,
    /// True when the behavior traces to the clean-room-rebuild-lane registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the lane's verification authority is honestly bounded so an unverified or non-materialized
    /// input never enters a protected lane (a hard invariant when `false`).
    pub rebuild_authority_bounded: bool,
    /// True when this lane's input source is trust-risk-bearing.
    pub is_replay_trust_risk_source: bool,
    /// True when the input-trust marker is disclosed before a trust-risk input is admitted.
    pub cache_trust_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe clean-room-rebuild-lane-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRemoteCacheIntegrityFindingEntry {
    /// Stable identity of the clean-room-rebuild-lane-registry entry.
    pub entry_id: String,
    /// The stable lane-binding ID this manifest binds to.
    pub lane_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must verify inputs and prove replay before promotion.
    pub semantic_role_must_verify_inputs_and_prove_replay_before_promotion: bool,
    /// The input-source token named by the entry.
    pub rebuild_source: String,
    /// Whether the input source is classified into the resolved taxonomy.
    pub rebuild_source_is_classified: bool,
    /// The canonical mode for the entry's input source.
    pub canonical_rebuild_source_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published build-config digest.
    pub rebuild_config_digest: String,
    /// The published materialized-input receipt.
    pub replay_receipt: String,
    /// The published input provenance ledger.
    pub protected_input_ledger: String,
    /// The published verification authority.
    pub rebuild_authority: String,
    /// The published expected artifact families.
    pub expected_artifact_families: String,
    /// The published hermetic-input posture.
    pub hermetic_rebuild_posture: String,
    /// The published re-materialization rule.
    pub shared_cache_isolation_rule: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved clean-room-rebuild-lane object publishes every required field.
    pub remote_cache_integrity_finding_object_complete: bool,
    /// Whether the entry traces to the clean-room-rebuild-lane registry.
    pub bound_to_registry: bool,
    /// Whether the lane's verification authority stays bounded (an unverified input never enters a protected
    /// lane).
    pub rebuild_authority_bounded: bool,
    /// Whether this lane's input source is trust-risk-bearing.
    pub is_replay_trust_risk_source: bool,
    /// Whether the input-trust marker is disclosed before a trust-risk input is admitted.
    pub cache_trust_disclosed: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5RemoteCacheIntegrityFindingEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5CacheDisciplineNextAction,
    /// Whether the manifest resolves to one typed object across every claimed lane (clean entry naming every
    /// fact).
    pub lane_resolves_across_lanes: bool,
}

impl M5ResolvedRemoteCacheIntegrityFindingEntry {
    /// Whether this clean-room-rebuild-lane entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_cache_bypass_drill_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CacheBypassDrillEntryResolutionInput {
    /// Stable identity of the artifact-diff-packet entry.
    pub entry_id: String,
    /// The stable manifest-ref this record binds to; empty means unstated.
    pub diff_packet_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level build-lane-trust role (from the frozen matrix vocabulary).
    pub semantic_role: M5BuildLaneTrustRole,
    /// The convergence scope this record must resolve its sidecar families from.
    pub diff_scope: M5CacheBypassDrillScope,
    /// The render / surface context.
    pub surface_context: M5CacheDisciplineSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5CacheDisciplineResolutionForm>,
    /// The published resolved exact build identity; empty means missing.
    pub resolved_build_identity: String,
    /// The published claimed artifact families; empty means missing.
    pub compared_artifact_families: String,
    /// The published sidecar-family ledger; empty means missing.
    pub deterministic_diff_ledger: String,
    /// The published binding-identity check; empty means missing.
    pub candidate_vs_rebuild_check: String,
    /// The published missing-or-mismatched reference; empty means missing.
    pub divergence_or_missing_reference: String,
    /// The published attestation state; empty means missing.
    pub attestation_state: String,
    /// The published last convergence revision; empty means missing.
    pub last_diff_revision: String,
    /// True when the record keeps the sidecar-family ledger visible.
    pub keeps_diff_ledger_visible: bool,
    /// True when the manifest is truthful (never claims a clean manifest over a hidden missing family).
    pub packet_is_truthful: bool,
    /// True when a claimed sidecar family is missing from this build.
    pub omitted_family_present: bool,
    /// True when a missing sidecar family is flagged as a blocker rather than silently omitted.
    pub omitted_family_flagged: bool,
    /// True when a sidecar is bound to a different build identity than the binary.
    pub material_divergence_present: bool,
    /// True when a mismatched-identity sidecar is flagged as a blocker rather than treated as warning-only.
    pub material_divergence_flagged: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe artifact-diff-packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCacheBypassDrillEntry {
    /// Stable identity of the artifact-diff-packet entry.
    pub entry_id: String,
    /// The stable manifest-ref this record binds to.
    pub diff_packet_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must verify inputs and prove replay before promotion.
    pub semantic_role_must_verify_inputs_and_prove_replay_before_promotion: bool,
    /// The convergence-scope token named by the entry.
    pub diff_scope: String,
    /// Whether the convergence scope is classified into the resolved taxonomy.
    pub diff_scope_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published resolved exact build identity.
    pub resolved_build_identity: String,
    /// The published claimed artifact families.
    pub compared_artifact_families: String,
    /// The published sidecar-family ledger.
    pub deterministic_diff_ledger: String,
    /// The published binding-identity check.
    pub candidate_vs_rebuild_check: String,
    /// The published missing-or-mismatched reference.
    pub divergence_or_missing_reference: String,
    /// The published attestation state.
    pub attestation_state: String,
    /// The published last convergence revision.
    pub last_diff_revision: String,
    /// Whether the record keeps the sidecar-family ledger visible.
    pub keeps_diff_ledger_visible: bool,
    /// Whether the manifest is truthful.
    pub packet_is_truthful: bool,
    /// Whether a claimed sidecar family is missing from this build.
    pub omitted_family_present: bool,
    /// Whether a missing sidecar family is flagged as a blocker.
    pub omitted_family_flagged: bool,
    /// Whether a sidecar is bound to a different build identity than the binary.
    pub material_divergence_present: bool,
    /// Whether a mismatched-identity sidecar is flagged as a blocker.
    pub material_divergence_flagged: bool,
    /// Whether the record stays converged (family ledger visible, missing family flagged, mismatched identity
    /// flagged).
    pub artifact_diff_stays_deterministic: bool,
    /// Whether the entry provides the complete artifact-diff object (build identity, claimed families,
    /// sidecar ledger, binding-identity check, missing-or-mismatched reference, attestation, last convergence
    /// revision).
    pub provides_complete_artifact_diff: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5CacheBypassDrillEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5CacheDisciplineNextAction,
    /// Whether the artifact-diff packet is safe on every claimed lane (clean entry naming every fact).
    pub packet_safe_on_every_lane: bool,
}

impl M5ResolvedCacheBypassDrillEntry {
    /// Whether this artifact-diff-packet entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5CacheDisciplineResolutionError {
    /// The clean-room-rebuild-lane-entry id was empty.
    EmptyRemoteCacheIntegrityFindingEntryId,
    /// The artifact-diff-packet-entry id was empty.
    EmptyCacheBypassDrillEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5CacheDisciplineResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyRemoteCacheIntegrityFindingEntryId => {
                "empty_remote_cache_integrity_finding_entry_id"
            }
            Self::EmptyCacheBypassDrillEntryId => "empty_cache_bypass_drill_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5CacheDisciplineResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 clean-room-rebuild-lane / artifact-diff-packet registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5CacheDisciplineResolutionError {}

fn form_tokens(forms: &[M5CacheDisciplineResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5CacheDisciplineResolutionForm]) -> bool {
    let present: BTreeSet<M5CacheDisciplineResolutionForm> = forms.iter().copied().collect();
    M5CacheDisciplineResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved clean-room-rebuild-lane object publishes every required field: classified input source,
/// build-config digest, materialized-input receipt, input provenance ledger, verification authority, expected
/// artifact families, hermetic-input posture, and re-materialization rule. An unclassified source or any empty
/// field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn remote_cache_integrity_finding_object_is_complete(
    source: M5RemoteCacheOriginKind,
    rebuild_config_digest: &str,
    replay_receipt: &str,
    protected_input_ledger: &str,
    rebuild_authority: &str,
    expected_artifact_families: &str,
    hermetic_rebuild_posture: &str,
    shared_cache_isolation_rule: &str,
) -> bool {
    source.is_classified()
        && !rebuild_config_digest.trim().is_empty()
        && !replay_receipt.trim().is_empty()
        && !protected_input_ledger.trim().is_empty()
        && !rebuild_authority.trim().is_empty()
        && !expected_artifact_families.trim().is_empty()
        && !hermetic_rebuild_posture.trim().is_empty()
        && !shared_cache_isolation_rule.trim().is_empty()
}

/// Whether the clean-room rebuild lane keeps an unverified input from entering a protected lane: the source
/// must be classified, the verification authority must be bounded (an unverified or non-materialized input never
/// claims protected-lane admission), and a trust-risk input source must disclose its input-trust marker before
/// the input is admitted. An unclassified source, an unbounded verification authority, or a hidden input-trust
/// marker never matches.
pub fn shared_cache_cannot_authorize_rebuild(
    source: M5RemoteCacheOriginKind,
    rebuild_authority_bounded: bool,
    is_replay_trust_risk_source: bool,
    cache_trust_disclosed: bool,
) -> bool {
    source.is_classified()
        && rebuild_authority_bounded
        && (!is_replay_trust_risk_source || cache_trust_disclosed)
}

/// Whether a artifact-diff packet stays converged: the scope must be classified, the manifest must be
/// truthful, it must keep the sidecar-family ledger visible, any missing sidecar family must be flagged as a
/// blocker rather than silently omitted, and any mismatched-identity sidecar must be flagged as a blocker rather
/// than treated as warning-only.
pub fn artifact_diff_stays_deterministic(
    scope: M5CacheBypassDrillScope,
    packet_is_truthful: bool,
    keeps_diff_ledger_visible: bool,
    omitted_family_present: bool,
    omitted_family_flagged: bool,
    material_divergence_present: bool,
    material_divergence_flagged: bool,
) -> bool {
    scope.is_classified()
        && packet_is_truthful
        && keeps_diff_ledger_visible
        && (!omitted_family_present || omitted_family_flagged)
        && (!material_divergence_present || material_divergence_flagged)
}

/// Resolves a clean-room-rebuild-lane-registry entry so it stays bound to the clean-room-rebuild-lane registry:
/// the entry names its canonical token, semantic role, and input source, covers all three resolution forms,
/// publishes a complete manifest object (build-config digest, materialized-input receipt, input provenance
/// ledger, verification authority, expected artifact families, hermetic-input posture, re-materialization rule),
/// bounds its verification authority so an unverified input never enters a protected lane, and discloses the
/// input-trust marker before a trust-risk input is admitted.
pub fn resolve_remote_cache_integrity_finding_entry(
    input: M5RemoteCacheIntegrityFindingEntryResolutionInput,
) -> Result<M5ResolvedRemoteCacheIntegrityFindingEntry, M5CacheDisciplineResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5CacheDisciplineResolutionError::EmptyRemoteCacheIntegrityFindingEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.lane_binding_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.rebuild_config_digest)
        || string_is_forbidden(&input.replay_receipt)
        || string_is_forbidden(&input.protected_input_ledger)
        || string_is_forbidden(&input.rebuild_authority)
        || string_is_forbidden(&input.expected_artifact_families)
        || string_is_forbidden(&input.hermetic_rebuild_posture)
        || string_is_forbidden(&input.shared_cache_isolation_rule)
    {
        return Err(M5CacheDisciplineResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = remote_cache_integrity_finding_object_is_complete(
        input.rebuild_source,
        &input.rebuild_config_digest,
        &input.replay_receipt,
        &input.protected_input_ledger,
        &input.rebuild_authority,
        &input.expected_artifact_families,
        &input.hermetic_rebuild_posture,
        &input.shared_cache_isolation_rule,
    );
    let admission_ok = shared_cache_cannot_authorize_rebuild(
        input.rebuild_source,
        input.rebuild_authority_bounded,
        input.is_replay_trust_risk_source,
        input.cache_trust_disclosed,
    );
    let input_trust_undisclosed = input.is_replay_trust_risk_source && !input.cache_trust_disclosed;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5RemoteCacheIntegrityFindingEntryDegradeReason::RegistryTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5RemoteCacheIntegrityFindingEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.rebuild_source.is_classified() {
        Some(M5RemoteCacheIntegrityFindingEntryDegradeReason::RebuildSourceUnclassified)
    } else if !input.bound_to_registry {
        Some(M5RemoteCacheIntegrityFindingEntryDegradeReason::LaneNotBoundToRegistry)
    } else if !object_complete {
        Some(M5RemoteCacheIntegrityFindingEntryDegradeReason::RemoteCacheIntegrityFindingObjectIncomplete)
    } else if !admission_ok {
        Some(M5RemoteCacheIntegrityFindingEntryDegradeReason::LaneReliesOnSharedCacheOrHidesReplayReceipt)
    } else if !all_forms {
        Some(M5RemoteCacheIntegrityFindingEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if input_trust_undisclosed {
        Some(M5RemoteCacheIntegrityFindingEntryDegradeReason::CacheTrustNotDisclosedForSharedCacheSource)
    } else if !input.proof_fresh {
        Some(M5RemoteCacheIntegrityFindingEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5CacheDisciplineNextAction::ExpandRebuildMeaning,
    };

    Ok(M5ResolvedRemoteCacheIntegrityFindingEntry {
        entry_id: input.entry_id,
        lane_binding_id: input.lane_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_verify_inputs_and_prove_replay_before_promotion: input
            .semantic_role
            .must_verify_inputs_and_prove_replay_before_promotion(),
        rebuild_source: input.rebuild_source.as_str().to_owned(),
        rebuild_source_is_classified: input.rebuild_source.is_classified(),
        canonical_rebuild_source_mode: input
            .rebuild_source
            .canonical_rebuild_source_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        rebuild_config_digest: input.rebuild_config_digest,
        replay_receipt: input.replay_receipt,
        protected_input_ledger: input.protected_input_ledger,
        rebuild_authority: input.rebuild_authority,
        expected_artifact_families: input.expected_artifact_families,
        hermetic_rebuild_posture: input.hermetic_rebuild_posture,
        shared_cache_isolation_rule: input.shared_cache_isolation_rule,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        remote_cache_integrity_finding_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        rebuild_authority_bounded: input.rebuild_authority_bounded,
        is_replay_trust_risk_source: input.is_replay_trust_risk_source,
        cache_trust_disclosed: input.cache_trust_disclosed,
        degrade_reason,
        next_action,
        lane_resolves_across_lanes: degrade_reason.is_none(),
    })
}

/// Resolves a artifact-diff-packet entry so its manifest stays safe: the entry names its canonical
/// token, semantic role, and convergence scope, covers all three resolution forms, provides the complete
/// build-identity / claimed-families / sidecar-ledger / binding-identity / missing-or-mismatched / attestation /
/// last-convergence-revision artifact-diff object, and degrades honestly when the manifest would let a
/// green build omit a claimed sidecar family, bind a sidecar to a different build identity, or treat a missing
/// or mismatched sidecar as warning-only.
pub fn resolve_cache_bypass_drill_entry(
    input: M5CacheBypassDrillEntryResolutionInput,
) -> Result<M5ResolvedCacheBypassDrillEntry, M5CacheDisciplineResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5CacheDisciplineResolutionError::EmptyCacheBypassDrillEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.diff_packet_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_build_identity)
        || string_is_forbidden(&input.compared_artifact_families)
        || string_is_forbidden(&input.deterministic_diff_ledger)
        || string_is_forbidden(&input.candidate_vs_rebuild_check)
        || string_is_forbidden(&input.divergence_or_missing_reference)
        || string_is_forbidden(&input.attestation_state)
        || string_is_forbidden(&input.last_diff_revision)
    {
        return Err(M5CacheDisciplineResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_converged = artifact_diff_stays_deterministic(
        input.diff_scope,
        input.packet_is_truthful,
        input.keeps_diff_ledger_visible,
        input.omitted_family_present,
        input.omitted_family_flagged,
        input.material_divergence_present,
        input.material_divergence_flagged,
    );
    let provides_record = input.diff_scope.is_classified()
        && !input.resolved_build_identity.trim().is_empty()
        && !input.compared_artifact_families.trim().is_empty()
        && !input.deterministic_diff_ledger.trim().is_empty()
        && !input.candidate_vs_rebuild_check.trim().is_empty()
        && !input.divergence_or_missing_reference.trim().is_empty()
        && !input.attestation_state.trim().is_empty()
        && !input.last_diff_revision.trim().is_empty()
        && record_stays_converged;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5CacheBypassDrillEntryDegradeReason::RegistryTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5CacheBypassDrillEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.diff_scope.is_classified() {
        Some(M5CacheBypassDrillEntryDegradeReason::DiffScopeUnclassified)
    } else if !provides_record {
        Some(M5CacheBypassDrillEntryDegradeReason::ArtifactDiffDivergesOrOmitsFamilyOrDriftsBuildIdentity)
    } else if !all_forms {
        Some(M5CacheBypassDrillEntryDegradeReason::PacketFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5CacheBypassDrillEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5CacheDisciplineNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedCacheBypassDrillEntry {
        entry_id: input.entry_id,
        diff_packet_ref: input.diff_packet_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_verify_inputs_and_prove_replay_before_promotion: input
            .semantic_role
            .must_verify_inputs_and_prove_replay_before_promotion(),
        diff_scope: input.diff_scope.as_str().to_owned(),
        diff_scope_is_classified: input.diff_scope.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        resolved_build_identity: input.resolved_build_identity,
        compared_artifact_families: input.compared_artifact_families,
        deterministic_diff_ledger: input.deterministic_diff_ledger,
        candidate_vs_rebuild_check: input.candidate_vs_rebuild_check,
        divergence_or_missing_reference: input.divergence_or_missing_reference,
        attestation_state: input.attestation_state,
        last_diff_revision: input.last_diff_revision,
        keeps_diff_ledger_visible: input.keeps_diff_ledger_visible,
        packet_is_truthful: input.packet_is_truthful,
        omitted_family_present: input.omitted_family_present,
        omitted_family_flagged: input.omitted_family_flagged,
        material_divergence_present: input.material_divergence_present,
        material_divergence_flagged: input.material_divergence_flagged,
        artifact_diff_stays_deterministic: record_stays_converged,
        provides_complete_artifact_diff: provides_record,
        degrade_reason,
        next_action,
        packet_safe_on_every_lane: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved clean-room-rebuild-lane and
/// artifact-diff-packet entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CacheIntegrityBypassRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5CacheIntegrityBypassRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5CacheDisciplineAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5CacheDisciplineExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5BuildLaneDowngradeTrigger>,
    /// Resolved clean-room-rebuild-lane-registry examples.
    pub remote_cache_integrity_finding_entries: Vec<M5ResolvedRemoteCacheIntegrityFindingEntry>,
    /// Resolved artifact-diff-packet examples.
    pub cache_bypass_drill_entries: Vec<M5ResolvedCacheBypassDrillEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the clean-room-rebuild-lane and
    /// artifact-diff-packet domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never lets a green build omit a claimed artifact family or sidecar. MUST be
    /// `false`.
    pub overclaims_clean_room_parity_on_a_partial_artifact_family_rebuild: bool,
    /// Hard invariant: this row never binds a claimed sidecar to a different build identity. MUST be `false`.
    pub lets_a_clean_room_rebuild_rely_on_a_shared_remote_cache_as_authority: bool,
    /// Hard invariant: this row never treats a missing or mismatched sidecar as warning-only. MUST be `false`.
    pub treats_a_material_artifact_diff_divergence_as_warning_only: bool,
    /// Hard invariant: this row never admits an unverified or non-materialized input into a protected lane. MUST
    /// be `false`.
    pub publishes_rc_or_stable_when_clean_room_parity_is_stale_or_incomplete: bool,
}

impl M5CacheIntegrityBypassRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5CacheDisciplineAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5CacheDisciplineAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5CacheDisciplineExportField> =
            self.export_fields.iter().copied().collect();
        M5CacheDisciplineExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.overclaims_clean_room_parity_on_a_partial_artifact_family_rebuild
            && !self.lets_a_clean_room_rebuild_rely_on_a_shared_remote_cache_as_authority
            && !self.treats_a_material_artifact_diff_divergence_as_warning_only
            && !self.publishes_rc_or_stable_when_clean_room_parity_is_stale_or_incomplete
    }

    /// True when a clean clean-room-rebuild-lane entry preserves registry-bound truth: it traces to the
    /// registry, keeps a classified input source, publishes a complete manifest object, bounds its verification
    /// authority, covers all three resolution forms, and discloses the input-trust marker for a trust-risk
    /// source.
    fn manifest_is_honest(ex: &M5ResolvedRemoteCacheIntegrityFindingEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.rebuild_source_is_classified
                && ex.remote_cache_integrity_finding_object_complete
                && ex.rebuild_authority_bounded
                && ex.covers_all_resolution_forms
                && (!ex.is_replay_trust_risk_source || ex.cache_trust_disclosed))
    }

    /// True when a clean artifact-diff-packet entry preserves a safe manifest: it keeps a classified
    /// convergence scope, provides the complete artifact-diff object, stays converged, and covers all
    /// three resolution forms.
    fn sidecar_manifest_is_honest(ex: &M5ResolvedCacheBypassDrillEntry) -> bool {
        !ex.is_clean()
            || (ex.diff_scope_is_classified
                && ex.provides_complete_artifact_diff
                && ex.artifact_diff_stays_deterministic
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.remote_cache_integrity_finding_entries
            .iter()
            .all(Self::manifest_is_honest)
            && self
                .cache_bypass_drill_entries
                .iter()
                .all(Self::sidecar_manifest_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CacheIntegrityBypassRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Input-source tokens (minted by this lane).
    pub rebuild_source_kinds: Vec<String>,
    /// Convergence-scope tokens (minted by this lane).
    pub diff_scopes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Verified-input-manifest-entry degrade-reason tokens.
    pub remote_cache_integrity_finding_degrade_reasons: Vec<String>,
    /// Sidecar-completeness-manifest-entry degrade-reason tokens.
    pub cache_bypass_drill_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5CacheIntegrityBypassRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5BuildLaneTrustRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5CacheDisciplineResolutionForm::ALL, |v| v.as_str()),
            rebuild_source_kinds: tokens(&M5RemoteCacheOriginKind::ALL, |v| v.as_str()),
            diff_scopes: tokens(&M5CacheBypassDrillScope::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5CacheDisciplineSurfaceContext::ALL, |v| v.as_str()),
            remote_cache_integrity_finding_degrade_reasons: tokens(
                &M5RemoteCacheIntegrityFindingEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            cache_bypass_drill_degrade_reasons: tokens(
                &M5CacheBypassDrillEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5CacheDisciplineAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5CacheDisciplineNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5CacheDisciplineExportField::ALL, |v| v.as_str()),
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
pub struct M5CacheIntegrityBypassRegistriesGovernanceReview {
    /// The clean-room-rebuild registry names a canonical token, semantic role, and input source for every entry.
    pub remote_cache_integrity_registry_names_token_role_and_source: bool,
    /// Every claimed lane resolves to one typed clean-room-rebuild-lane object from the shared registry, not
    /// per-entry reconstruction.
    pub lane_resolves_to_typed_rebuild_lane_from_shared_registry: bool,
    /// The build-config digest, materialized-input receipt, verification authority, and expected artifact
    /// families are published for every resolved manifest.
    pub rebuild_config_digest_receipt_and_artifact_families_published: bool,
    /// Unverified inputs cannot enter protected lanes; an unverified or non-materialized input never claims
    /// protected-lane admission.
    pub shared_cache_cannot_authorize_rebuild_lanes: bool,
    /// The artifact-diff packet keeps the sidecar-family ledger visible and flags missing or mismatched
    /// sidecars as blockers.
    pub sidecar_manifest_keeps_diff_ledger_visible_and_flags_missing_or_mismatched: bool,
    /// The input-trust marker is disclosed before any trust-risk input is admitted.
    pub cache_trust_disclosed_for_trust_risk_sources: bool,
    /// Every clean-room-rebuild-lane and artifact-diff-packet entry covers the canonical / accessible /
    /// audit resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Verified-input-manifest and artifact-diff-packet behavior stay bound to the shared registries
    /// rather than hand-copied per lane.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Release center, shiproom, diagnostics, and provenance read a single build-lane source.
    pub release_center_shiproom_diagnostics_and_provenance_read_single_source: bool,
    /// An admit-unclean-room-rebuild attempt, an incomplete object, or a missing sidecar is caught by fixtures
    /// before release evidence turns green.
    pub rebuild_or_diff_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CacheIntegrityBypassRegistriesConsumerProjection {
    /// Release center and shiproom consume the shared clean-room-rebuild-lane registry.
    pub release_center_and_shiproom_consume_shared_registries: bool,
    /// Diagnostics and provenance consume the shared artifact-diff-packet registry.
    pub diagnostics_and_provenance_consume_shared_registries: bool,
    /// Build farm and cache service consume the shared registries.
    pub build_farm_and_cache_service_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical clean-room-rebuild-lane and artifact-diff-packet domain
    /// contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical clean-room-rebuild-lane / artifact-diff-packet
    /// registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CacheIntegrityBypassRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CacheIntegrityBypassRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting build-lane audit for the lane.
    pub build_lane_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CacheIntegrityBypassRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CacheIntegrityBypassRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5CacheIntegrityBypassRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CacheIntegrityBypassRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CacheIntegrityBypassRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CacheIntegrityBypassRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CacheIntegrityBypassRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CacheIntegrityBypassRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 clean-room-rebuild-lane and artifact-diff-packet registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CacheIntegrityBypassRegistriesPacket {
    /// Record kind; must equal [`M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5CacheIntegrityBypassRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CacheIntegrityBypassRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CacheIntegrityBypassRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CacheIntegrityBypassRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CacheIntegrityBypassRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CacheIntegrityBypassRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CacheIntegrityBypassRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5CacheIntegrityBypassRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5CacheIntegrityBypassRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_RECORD_KIND {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(&serde_json::to_value(self).expect(
            "m5 clean-room-rebuild-lane / artifact-diff-packet registries packet serializes",
        )) {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect(
            "m5 clean-room-rebuild-lane / artifact-diff-packet registries packet serializes",
        )
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,remote_cache_integrity_finding_entries,cache_bypass_drill_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .remote_cache_integrity_finding_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.cache_bypass_drill_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.remote_cache_integrity_finding_entries.len(),
                row.cache_bypass_drill_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Verified-Input-Manifest and Sidecar-Completeness-Manifest Registries\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Input sources: {}\n",
            self.vocabulary_set.rebuild_source_kinds.join(", ")
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
                "  - Verified-input-manifest entries: {} / artifact-diff-packet entries: {}\n",
                row.remote_cache_integrity_finding_entries.len(),
                row.cache_bypass_drill_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry manifest reference table generated from the registry, so docs and shiproom
    /// runbooks render the same input-source-mode / build-config-digest / receipt / verification-authority /
    /// artifact-families truth the resolvers produced rather than a hand-copied lane table. Only clean,
    /// registry-bound clean-room-rebuild-lane entries are listed.
    pub fn render_remote_cache_integrity_finding_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| lane_binding_id | source_mode | rebuild_config_digest | replay_receipt | rebuild_authority | expected_artifact_families |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.remote_cache_integrity_finding_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.lane_binding_id,
                    ex.canonical_rebuild_source_mode,
                    ex.rebuild_config_digest,
                    ex.replay_receipt,
                    ex.rebuild_authority,
                    ex.expected_artifact_families
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5CacheIntegrityBypassRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CacheIntegrityBypassRegistriesViolation>),
}

impl fmt::Display for M5CacheIntegrityBypassRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 clean-room-rebuild-lane / artifact-diff-packet registries export parse failed: {error}"
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
                    "m5 clean-room-rebuild-lane / artifact-diff-packet registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CacheIntegrityBypassRegistriesArtifactError {}

/// Validation failures emitted by [`M5CacheIntegrityBypassRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CacheIntegrityBypassRegistriesViolation {
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
    /// A registry row does not point at both the clean-room-rebuild-lane and artifact-diff-packet
    /// domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, admit-unclean-room-rebuild, field-incomplete,
    /// form-incomplete, or a artifact-diff entry missing the complete manifest object).
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
    /// Verified-input-manifest-resolution is not proven: clean manifest entries do not cover the canonical
    /// input sources or the first release-center / shiproom / diagnostics / provenance / support surfaces, no
    /// object-incomplete example degrades, or a clean manifest entry published an incomplete object.
    RemoteCacheIntegrityFindingResolutionNotProven,
    /// Input-verification-boundary-preservation is not proven: no admit-unverified example and no unbound
    /// example degrade, no clean bounded manifest entry is present, or a clean manifest entry is unbounded or
    /// unbound.
    SharedCacheAuthorityBoundaryNotProven,
    /// Sidecar-completeness-integrity is not proven: clean sidecar manifests do not cover the canonical
    /// binary-identity / receipt-reconciled / hermetic-rebuild convergence scopes with full resolution-form
    /// coverage while providing the complete manifest object, no missing-sidecar or form-incomplete example
    /// degrades, or a clean sidecar manifest is missing the complete manifest object.
    ArtifactDiffDeterminismNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5CacheIntegrityBypassRegistriesViolation {
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
            Self::RemoteCacheIntegrityFindingResolutionNotProven => {
                "remote_cache_integrity_finding_resolution_not_proven"
            }
            Self::SharedCacheAuthorityBoundaryNotProven => {
                "shared_cache_authority_boundary_not_proven"
            }
            Self::ArtifactDiffDeterminismNotProven => "artifact_diff_determinism_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_remote_cache_integrity_and_cache_bypass_drill_registries_export(
) -> Result<M5CacheIntegrityBypassRegistriesPacket, M5CacheIntegrityBypassRegistriesArtifactError> {
    let packet: M5CacheIntegrityBypassRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-remote-cache-integrity-and-cache-bypass-drill-registries-proof/support_export.json"
        )
    ))
    .map_err(M5CacheIntegrityBypassRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CacheIntegrityBypassRegistriesArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5CacheIntegrityBypassRegistriesPacket,
    violations: &mut Vec<M5CacheIntegrityBypassRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_SCHEMA_REF,
        M5_CACHE_INTEGRITY_BYPASS_REGISTRIES_DOC_REF,
        M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
        M5_BUILD_LANE_TRUST_MATRIX_DOC_REF,
        M5_REMOTE_CACHE_INTEGRITY_FINDING_DOMAIN_SCHEMA_REF,
        M5_CACHE_BYPASS_DRILL_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5CacheIntegrityBypassRegistriesPacket,
    violations: &mut Vec<M5CacheIntegrityBypassRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5CacheIntegrityBypassRegistriesViolation::NoRegistryRows);
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
            violations.push(M5CacheIntegrityBypassRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_REMOTE_CACHE_INTEGRITY_FINDING_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_CACHE_BYPASS_DRILL_DOMAIN_SCHEMA_REF)
        {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.remote_cache_integrity_finding_entries.is_empty()
            || row.cache_bypass_drill_entries.is_empty()
        {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5CacheIntegrityBypassRegistriesPacket,
    violations: &mut Vec<M5CacheIntegrityBypassRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.remote_cache_integrity_registry_names_token_role_and_source,
        review.lane_resolves_to_typed_rebuild_lane_from_shared_registry,
        review.rebuild_config_digest_receipt_and_artifact_families_published,
        review.shared_cache_cannot_authorize_rebuild_lanes,
        review.sidecar_manifest_keeps_diff_ledger_visible_and_flags_missing_or_mismatched,
        review.cache_trust_disclosed_for_trust_risk_sources,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.release_center_shiproom_diagnostics_and_provenance_read_single_source,
        review.rebuild_or_diff_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5CacheIntegrityBypassRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CacheIntegrityBypassRegistriesPacket,
    violations: &mut Vec<M5CacheIntegrityBypassRegistriesViolation>,
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
            violations
                .push(M5CacheIntegrityBypassRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CacheIntegrityBypassRegistriesPacket,
    violations: &mut Vec<M5CacheIntegrityBypassRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CacheIntegrityBypassRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CacheIntegrityBypassRegistriesPacket,
    violations: &mut Vec<M5CacheIntegrityBypassRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.build_lane_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CacheIntegrityBypassRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5CacheIntegrityBypassRegistriesPacket,
    violations: &mut Vec<M5CacheIntegrityBypassRegistriesViolation>,
) {
    let manifests = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.remote_cache_integrity_finding_entries.iter())
    };
    let sidecars = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.cache_bypass_drill_entries.iter())
    };

    // AC1: every lane exposes a typed manifest with build-config-digest, receipt, and verification boundaries.
    // Clean manifest entries cover the canonical input sources and the first release-center / shiproom /
    // diagnostics / provenance / support surfaces, an object-incomplete example degrades, and no clean manifest
    // entry published an incomplete object.
    let clean_sources: BTreeSet<String> = manifests()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.rebuild_source.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = manifests()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let sources_covered = M5RemoteCacheOriginKind::CANONICAL_SOURCES
        .iter()
        .all(|k| clean_sources.contains(k.as_str()));
    let first_surfaces_covered = M5CacheDisciplineSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = manifests().any(|ex| {
        ex.degrade_reason
            == Some(M5RemoteCacheIntegrityFindingEntryDegradeReason::RemoteCacheIntegrityFindingObjectIncomplete)
    });
    let no_clean_incomplete =
        !manifests().any(|ex| ex.is_clean() && !ex.remote_cache_integrity_finding_object_complete);
    if !(sources_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5CacheIntegrityBypassRegistriesViolation::RemoteCacheIntegrityFindingResolutionNotProven,
        );
    }

    // AC2: attempting to admit an unverified input into a protected lane fails with a structured blocker reason.
    // An admit-unverified example degrades, an unbound example degrades, at least one clean bounded manifest
    // entry is present, and no clean manifest entry is unbounded or unbound.
    let admit_fold_degrades = manifests().any(|ex| {
        ex.degrade_reason
            == Some(M5RemoteCacheIntegrityFindingEntryDegradeReason::LaneReliesOnSharedCacheOrHidesReplayReceipt)
    });
    let unbound_degrades = manifests().any(|ex| {
        ex.degrade_reason
            == Some(M5RemoteCacheIntegrityFindingEntryDegradeReason::LaneNotBoundToRegistry)
    });
    let bounded_clean_manifest =
        manifests().any(|ex| ex.is_clean() && ex.rebuild_authority_bounded);
    let no_clean_unbound = !manifests().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded = !manifests().any(|ex| ex.is_clean() && !ex.rebuild_authority_bounded);
    if !(admit_fold_degrades
        && unbound_degrades
        && bounded_clean_manifest
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations
            .push(M5CacheIntegrityBypassRegistriesViolation::SharedCacheAuthorityBoundaryNotProven);
    }

    // AC3: release packets can prove which lane produced each claimed artifact family and that every sidecar
    // converges on one exact build identity. Clean sidecar manifests cover every canonical binary-identity /
    // receipt-reconciled / hermetic-rebuild convergence scope with full resolution-form coverage while providing
    // the complete manifest object, a missing-sidecar example degrades, a form-incomplete example degrades, and
    // no clean sidecar manifest is missing the complete object.
    let clean_sidecar_scopes: BTreeSet<String> = sidecars()
        .filter(|ex| {
            ex.is_clean()
                && ex.diff_scope_is_classified
                && ex.provides_complete_artifact_diff
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.diff_scope.clone())
        .collect();
    let sidecar_scopes_covered = M5CacheBypassDrillScope::CANONICAL_SCOPES
        .iter()
        .all(|m| clean_sidecar_scopes.contains(m.as_str()));
    let missing_sidecar_degrades = sidecars().any(|ex| {
        ex.degrade_reason
            == Some(M5CacheBypassDrillEntryDegradeReason::ArtifactDiffDivergesOrOmitsFamilyOrDriftsBuildIdentity)
    });
    let form_incomplete_degrades = sidecars().any(|ex| {
        ex.degrade_reason
            == Some(M5CacheBypassDrillEntryDegradeReason::PacketFormCoverageIncomplete)
    });
    let no_clean_missing_sidecar =
        !sidecars().any(|ex| ex.is_clean() && !ex.provides_complete_artifact_diff);
    if !(sidecar_scopes_covered
        && missing_sidecar_degrades
        && form_incomplete_degrades
        && no_clean_missing_sidecar)
    {
        violations
            .push(M5CacheIntegrityBypassRegistriesViolation::ArtifactDiffDeterminismNotProven);
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

/// The build lanes this lane implements, for downstream reference: the clean-room-rebuild-lane registry covers
/// the contributor / PR and protected-merge lanes, and the artifact-diff-packet registry covers the
/// release and emergency-hotfix lanes.
pub const IMPLEMENTED_FAMILIES: [M5BuildLaneFamily; 4] = M5BuildLaneFamily::ALL;
