//! Implemented M5 cohort-descriptor and cohort-evidence-packet registries.
//!
//! The frozen [launch-control matrix][matrix] names Aureline's governed launch-bearing cohorts — the core-team
//! canary, design-partner preview, extension-author, public preview, and certified-archetype cohorts — and
//! locks their controlled vocabulary. This module is the first implement lane for the concrete cohort model: it
//! turns the *cohort-descriptor* grammar (how a widening cohort declares the exact repo / archetype rows, bundle
//! IDs, install topology, toolchain envelope, known limits, rollback target, and diagnostics posture it is
//! auditable by) and the *cohort-evidence-packet* grammar (how a launch-bearing lane proves which cohort
//! evidence — dogfood-ring telemetry, current rehearsal cadence, or an explicit go/no-go signoff — backs it,
//! keeping partner / public support language and known-limits packets bound to that proof rather than to
//! hand-edited prose) into registry resolvers that produce export-safe, honest projections. Every claimed M5
//! launch-bearing cohort then resolves to one typed cohort-descriptor object — the archetype it classifies, the
//! exact repo / archetype rows, the bundle IDs, the install topology, the toolchain envelope, the known limits,
//! the rollback target, and the diagnostics posture, all preserved before widening so a cohort never widens
//! without its rollback and diagnostics posture and so partner / public support language never outruns current
//! cohort proof — and to one cohort-evidence-packet object — the resolved cohort identity, the known-limits
//! ledger, the rollback-target reference, the rehearsal-currency state, the readiness-signoff state, the
//! cohort-bound support-language reference, and the last widening revision — that the shiproom, release-center,
//! executive-steering, program-governance, and support / export surfaces can inspect without manual
//! reconstruction, so a cohort can never widen without preserving rollback and diagnostics, partner / public
//! support language never runs ahead of cohort proof, the exact rows / bundles / toolchains / deployment
//! profiles stay visible before widening, and a cohort that cannot explain the descriptor it declared or the
//! evidence that backs it degrades honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one typed cohort-descriptor object per cohort.** [`resolve_cohort_descriptor_entry`] refuses to
//!   read as a clean, registry-bound descriptor entry unless it names a canonical registry token, a classified
//!   [cohort archetype][M5CohortArchetypeKind], a launch-control role, covers every [resolution
//!   form][M5CohortResolutionForm] (the canonical object, the accessible summary, and the audit record),
//!   publishes every descriptor field (exact repo / archetype rows, bundle IDs, install topology, toolchain
//!   envelope, known limits, rollback target, and diagnostics posture), preserves its rollback and diagnostics
//!   posture before widening, and keeps partner / public support language matched to cohort proof; otherwise it
//!   degrades.
//! * **Keep a cohort from widening without preserving rollback and diagnostics.**
//!   [`cohort_preserves_rollback_and_diagnostics_before_widening`] rejects a descriptor entry whose rollback and
//!   diagnostics posture is not preserved (a cohort widening without a rollback target and diagnostics posture)
//!   so it degrades to
//!   [`M5CohortDescriptorEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof`],
//!   and a public-facing cohort whose support language runs ahead of cohort proof degrades the same way — the
//!   structured blocker reason a widen-without-rollback attempt must surface.
//! * **Keep the cohort evidence from running support language ahead of proof or dropping cohort evidence.**
//!   [`resolve_cohort_evidence_packet_entry`] names a classified [evidence scope][M5CohortEvidenceScope],
//!   requires the full cohort-identity / known-limits-ledger / rollback-target / rehearsal-currency /
//!   readiness-signoff / support-language / last-widening-revision cohort-evidence object, covers every
//!   resolution form, and degrades to
//!   [`M5CohortEvidencePacketEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence`]
//!   when the evidence would run partner / public support language ahead of cohort proof, hide the cohort
//!   evidence, or let a known-limits gap masquerade as covered, so a cohort-evidence packet can never read as
//!   trustworthy when it has quietly dropped the reason a lane is actually backed by cohort proof.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5LaunchControlRole`] role vocabulary and
//! the [`M5LaunchControlConsumerSurface`] consumer-surface taxonomy — so the shiproom, release-center,
//! executive-steering, program-governance, diagnostics, docs, CLI, support, and public-proof surfaces can never
//! fork their own cohort meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_launch_control_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_cohort_descriptor_and_evidence_packet_registries,
    seeded_m5_cohort_descriptor_and_evidence_packet_registries_cohort_descriptor_beta_narrowed,
    seeded_m5_cohort_descriptor_and_evidence_packet_registries_cohort_evidence_preview_narrowed,
    M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_launch_control_matrix::{
    M5LaunchControlAccessibilityRoute, M5LaunchControlCohort, M5LaunchControlConsumerSurface,
    M5LaunchControlDowngradeTrigger, M5LaunchControlQualificationClass,
    M5LaunchControlRequiredLabel, M5LaunchControlRole, M5LaunchControlWideningStage,
    M5_COHORT_DESCRIPTOR_DOMAIN_SCHEMA_REF, M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
    M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5CohortDescriptorEvidencePacketRegistriesPacket`].
pub const M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_cohort_descriptor_and_cohort_evidence_packet_registries";

/// Schema version for M5 cohort-descriptor / cohort-evidence-packet registry records.
pub const M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_SCHEMA_REF: &str =
    "schemas/program/m5-cohort-descriptor-and-evidence-packet-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_DOC_REF: &str =
    "docs/release/m5_cohort_descriptor_and_evidence_packet_registries.md";

/// Repo-relative path of the canonical cohort-evidence-packet domain schema minted by this lane (how a cohort
/// proves which evidence class — dogfood-ring telemetry, rehearsal currency, or go/no-go signoff — backs it).
pub const M5_COHORT_EVIDENCE_PACKET_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-cohort-evidence-packet.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-cohort-descriptor-and-evidence-packet-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-cohort-descriptor-and-evidence-packet-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-cohort-descriptor-and-evidence-packet-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/release/m5-cohort-descriptor-and-evidence-packet-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// cohort invents a parallel surface set.
pub type M5CohortDescriptorEvidencePacketRegistriesConsumerSurface = M5LaunchControlConsumerSurface;

/// One of the three resolution forms every cohort-descriptor or cohort-evidence-packet entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// cohort-descriptor and cohort-evidence *domains* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CohortResolutionForm {
    /// The canonical resolved cohort-descriptor / cohort-evidence-packet object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved cohort discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved cohort inspectable off-renderer.
    AuditRecord,
}

impl M5CohortResolutionForm {
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

/// Controlled cohort archetype a cohort-descriptor entry classifies, so the typed descriptor model shares one
/// registry rather than a hand-copied per-cohort assumption. Minted by this lane because the frozen matrix
/// carries the launch-bearing cohorts but distinguishes the dogfood / migration-alpha / extension-author /
/// design-partner / public-preview / certified-archetype archetypes an auditable descriptor classifies against
/// explicitly. Every classified archetype carries its canonical mode, and the design-partner-preview and
/// public-preview archetypes are public-facing so their partner / public support language must stay matched to
/// cohort proof before the cohort widens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CohortArchetypeKind {
    /// The internal dogfood core-team canary cohort.
    DogfoodCoreTeamCanary,
    /// The migration alpha cohort (external alpha migrating from a prior toolchain).
    MigrationAlpha,
    /// The extension-author cohort (compatibility rehearsals current, freeze exceptions documented).
    ExtensionAuthor,
    /// The design-partner preview cohort (public-facing; support language must match cohort proof).
    DesignPartnerPreview,
    /// The public preview cohort (public-facing; support language must match cohort proof).
    PublicPreview,
    /// The certified-archetype cohort (ORR signed and a go/no-go decision recorded).
    CertifiedArchetype,
    /// The cohort archetype is unclassified, which is disallowed.
    ArchetypeUnclassified,
}

impl M5CohortArchetypeKind {
    /// Every cohort archetype, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DogfoodCoreTeamCanary,
        Self::MigrationAlpha,
        Self::ExtensionAuthor,
        Self::DesignPartnerPreview,
        Self::PublicPreview,
        Self::CertifiedArchetype,
        Self::ArchetypeUnclassified,
    ];

    /// The six canonical cohort archetypes every claimed M5 launch-bearing cohort classifies against.
    pub const CANONICAL_ARCHETYPES: [Self; 6] = [
        Self::DogfoodCoreTeamCanary,
        Self::MigrationAlpha,
        Self::ExtensionAuthor,
        Self::DesignPartnerPreview,
        Self::PublicPreview,
        Self::CertifiedArchetype,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DogfoodCoreTeamCanary => "dogfood_core_team_canary",
            Self::MigrationAlpha => "migration_alpha",
            Self::ExtensionAuthor => "extension_author",
            Self::DesignPartnerPreview => "design_partner_preview",
            Self::PublicPreview => "public_preview",
            Self::CertifiedArchetype => "certified_archetype",
            Self::ArchetypeUnclassified => "archetype_unclassified",
        }
    }

    /// Whether the archetype is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ArchetypeUnclassified)
    }

    /// The canonical mode for this cohort archetype.
    pub const fn canonical_cohort_archetype_mode(self) -> &'static str {
        match self {
            Self::DogfoodCoreTeamCanary => "dogfood_core_team_canary_archetype",
            Self::MigrationAlpha => "migration_alpha_archetype",
            Self::ExtensionAuthor => "extension_author_archetype",
            Self::DesignPartnerPreview => "design_partner_preview_archetype",
            Self::PublicPreview => "public_preview_archetype",
            Self::CertifiedArchetype => "certified_archetype_archetype",
            Self::ArchetypeUnclassified => "",
        }
    }

    /// Whether this archetype is public-facing and so must keep partner / public support language matched to
    /// cohort proof before the cohort widens.
    pub const fn is_public_facing_cohort(self) -> bool {
        matches!(self, Self::DesignPartnerPreview | Self::PublicPreview)
    }
}

/// Controlled evidence scope a cohort-evidence-packet entry must resolve its cohort proof from, so an evidence
/// packet shares one registry rather than a hand-copied per-record assumption. Minted by this lane, tracking
/// whether the evidence came from dogfood-ring telemetry, current rehearsal cadence, or an explicit go/no-go
/// signoff the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CohortEvidenceScope {
    /// The evidence came from internal dogfood-ring telemetry.
    DogfoodRingEvidence,
    /// The evidence came from current rehearsal cadence (publish/rollback, mixed-version, handoff drills).
    RehearsalCurrencyEvidence,
    /// The evidence came from an explicit go/no-go signoff with a preserved evidence snapshot.
    GoNoGoSignoffEvidence,
    /// The evidence scope is unclassified, which is disallowed.
    ScopeUnclassified,
}

impl M5CohortEvidenceScope {
    /// Every evidence scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DogfoodRingEvidence,
        Self::RehearsalCurrencyEvidence,
        Self::GoNoGoSignoffEvidence,
        Self::ScopeUnclassified,
    ];

    /// The three canonical evidence scopes every cohort-evidence packet must stay distinct across.
    pub const CANONICAL_SCOPES: [Self; 3] = [
        Self::DogfoodRingEvidence,
        Self::RehearsalCurrencyEvidence,
        Self::GoNoGoSignoffEvidence,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DogfoodRingEvidence => "dogfood_ring_evidence",
            Self::RehearsalCurrencyEvidence => "rehearsal_currency_evidence",
            Self::GoNoGoSignoffEvidence => "go_no_go_signoff_evidence",
            Self::ScopeUnclassified => "scope_unclassified",
        }
    }

    /// Whether the evidence scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ScopeUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a cohort-descriptor or
/// cohort-evidence-packet token's meaning stays stable whether it appears in the release-center, shiproom,
/// executive-steering, program-governance, or a support / export form. Minted by this lane, tracking the
/// first-consumer surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CohortSurfaceContext {
    /// The release-center surface.
    ReleaseCenterSurface,
    /// The shiproom surface.
    ShiproomSurface,
    /// The executive-steering surface.
    ExecutiveSteeringSurface,
    /// The program-governance surface.
    ProgramGovernanceSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5CohortSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReleaseCenterSurface,
        Self::ShiproomSurface,
        Self::ExecutiveSteeringSurface,
        Self::ProgramGovernanceSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::ReleaseCenterSurface,
        Self::ShiproomSurface,
        Self::ExecutiveSteeringSurface,
        Self::ProgramGovernanceSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenterSurface => "release_center_surface",
            Self::ShiproomSurface => "shiproom_surface",
            Self::ExecutiveSteeringSurface => "executive_steering_surface",
            Self::ProgramGovernanceSurface => "program_governance_surface",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a cohort-descriptor or cohort-evidence-packet entry must be able to show, so no
/// cohort archetype, repo / bundle / toolchain / deployment row, known-limits packet, rollback target,
/// cohort-evidence field, or registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CohortAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The cohort archetype the entry classifies (cohort-descriptor entry).
    CohortArchetype,
    /// The exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles the entry publishes (cohort-descriptor entry).
    RepoBundleToolchainAndDeploymentRows,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The known-limits packet and rollback target the entry preserves before widening (cohort-descriptor
    /// entry).
    KnownLimitsAndRollbackTarget,
    /// The cohort-evidence fields (cohort identity, known-limits ledger, rollback target, rehearsal currency,
    /// readiness signoff, support language) the entry publishes (cohort-evidence-packet entry).
    CohortEvidenceFields,
    /// The support-identity hint the entry publishes (cohort-evidence-packet entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved cohort descriptor or cohort evidence (both entries).
    PlainLanguageMeaning,
}

impl M5CohortAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::CohortArchetype,
        Self::RepoBundleToolchainAndDeploymentRows,
        Self::ResolutionFormCoverage,
        Self::KnownLimitsAndRollbackTarget,
        Self::CohortEvidenceFields,
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
            Self::CohortArchetype => "cohort_archetype",
            Self::RepoBundleToolchainAndDeploymentRows => {
                "repo_bundle_toolchain_and_deployment_rows"
            }
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::KnownLimitsAndRollbackTarget => "known_limits_and_rollback_target",
            Self::CohortEvidenceFields => "cohort_evidence_fields",
            Self::SupportIdentityHint => "support_identity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// cohort descriptor, a cohort-evidence packet, or a degraded cohort-descriptor / cohort-evidence-packet entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CohortNextAction {
    /// Expand the resolved cohort descriptor's or cohort-evidence packet's plain-language meaning.
    ExpandCohortMeaning,
    /// Inspect the cohort archetype or evidence scope the entry resolves.
    InspectArchetypeOrScope,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5CohortNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandCohortMeaning,
        Self::InspectArchetypeOrScope,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandCohortMeaning => "expand_cohort_meaning",
            Self::InspectArchetypeOrScope => "inspect_archetype_or_scope",
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
pub enum M5CohortExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The cohort families covered.
    CohortFamilies,
    /// The cohort archetypes carried.
    CohortArchetypes,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The evidence scopes carried.
    EvidenceScopes,
    /// The render / surface context.
    SurfaceContext,
    /// The cohort-archetype modes carried.
    CohortArchetypeModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5CohortExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::CohortFamilies,
        Self::CohortArchetypes,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::EvidenceScopes,
        Self::SurfaceContext,
        Self::CohortArchetypeModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::CohortFamilies,
        Self::CohortArchetypes,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::CohortFamilies => "cohort_families",
            Self::CohortArchetypes => "cohort_archetypes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::EvidenceScopes => "evidence_scopes",
            Self::SurfaceContext => "surface_context",
            Self::CohortArchetypeModes => "cohort_archetype_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a cohort-descriptor entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, widen-without-rollback, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CohortDescriptorEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the descriptor means.
    DescriptorTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The cohort archetype is unclassified (not in the resolved taxonomy).
    CohortArchetypeUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    DescriptorNotBoundToRegistry,
    /// The resolved cohort-descriptor object is incomplete: the exact repo / archetype rows, bundle IDs, install
    /// topology, toolchain envelope, known limits, rollback target, or diagnostics posture is unstated.
    CohortDescriptorObjectIncomplete,
    /// The cohort's rollback and diagnostics posture is not preserved before widening (a cohort widening without
    /// a rollback target and diagnostics posture), or a public-facing cohort ran its support language ahead of
    /// cohort proof.
    DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A public-facing cohort did not keep its support language matched to cohort proof before widening.
    RollbackOrDiagnosticsNotPreservedForPublicCohort,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CohortDescriptorEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::DescriptorTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::CohortArchetypeUnclassified,
        Self::DescriptorNotBoundToRegistry,
        Self::CohortDescriptorObjectIncomplete,
        Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
        Self::ResolutionFormCoverageIncomplete,
        Self::RollbackOrDiagnosticsNotPreservedForPublicCohort,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorTokenUnstated => "descriptor_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::CohortArchetypeUnclassified => "cohort_archetype_unclassified",
            Self::DescriptorNotBoundToRegistry => "descriptor_not_bound_to_registry",
            Self::CohortDescriptorObjectIncomplete => "cohort_descriptor_object_incomplete",
            Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof => {
                "descriptor_lets_cohort_widen_without_rollback_or_runs_support_ahead_of_proof"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::RollbackOrDiagnosticsNotPreservedForPublicCohort => {
                "rollback_or_diagnostics_not_preserved_for_public_cohort"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5CohortNextAction {
        match self {
            Self::DescriptorTokenUnstated | Self::DescriptorNotBoundToRegistry => {
                M5CohortNextAction::TraceCanonicalRegistry
            }
            Self::CohortArchetypeUnclassified
            | Self::CohortDescriptorObjectIncomplete
            | Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof => {
                M5CohortNextAction::InspectArchetypeOrScope
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5CohortNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::RollbackOrDiagnosticsNotPreservedForPublicCohort
            | Self::ProofStale => M5CohortNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5LaunchControlDowngradeTrigger {
        match self {
            Self::DescriptorTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::DescriptorNotBoundToRegistry => {
                M5LaunchControlDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::CohortArchetypeUnclassified | Self::CohortDescriptorObjectIncomplete => {
                M5LaunchControlDowngradeTrigger::CohortMembershipUnstated
            }
            Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof
            | Self::RollbackOrDiagnosticsNotPreservedForPublicCohort => {
                M5LaunchControlDowngradeTrigger::WidenedWithoutCurrentCohortEvidence
            }
            Self::ProofStale => M5LaunchControlDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a cohort-evidence-packet entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CohortEvidencePacketEntryDegradeReason {
    /// The canonical registry token name is unstated.
    EvidenceTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The evidence scope is unclassified (not in the resolved taxonomy).
    EvidenceScopeUnclassified,
    /// The cohort evidence would run partner / public support language ahead of cohort proof, hide the cohort
    /// evidence, let a known-limits gap masquerade as covered, or it dropped one of the required cohort-evidence
    /// fields (cohort identity, known-limits ledger, rollback target, rehearsal currency, readiness signoff,
    /// support language, last widening revision).
    CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
    /// The canonical / accessible / audit resolution-form coverage of the evidence is incomplete.
    EvidenceFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CohortEvidencePacketEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EvidenceTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::EvidenceScopeUnclassified,
        Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
        Self::EvidenceFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceTokenUnstated => "evidence_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::EvidenceScopeUnclassified => "evidence_scope_unclassified",
            Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                "cohort_evidence_runs_support_ahead_of_proof_or_drops_cohort_evidence"
            }
            Self::EvidenceFormCoverageIncomplete => "evidence_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5CohortNextAction {
        match self {
            Self::EvidenceTokenUnstated => M5CohortNextAction::TraceCanonicalRegistry,
            Self::EvidenceScopeUnclassified
            | Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                M5CohortNextAction::InspectArchetypeOrScope
            }
            Self::EvidenceFormCoverageIncomplete => {
                M5CohortNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5CohortNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5LaunchControlDowngradeTrigger {
        match self {
            Self::EvidenceTokenUnstated => {
                M5LaunchControlDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::EvidenceScopeUnclassified => {
                M5LaunchControlDowngradeTrigger::ReadinessStateUnstated
            }
            Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                M5LaunchControlDowngradeTrigger::RanPartnerOrPublicLanguageAheadOfCohortProof
            }
            Self::EvidenceFormCoverageIncomplete => {
                M5LaunchControlDowngradeTrigger::ImpliedGreenWhileGoNoGoOrOrrWasStale
            }
            Self::ProofStale => M5LaunchControlDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_cohort_descriptor_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CohortDescriptorEntryResolutionInput {
    /// Stable identity of the cohort-descriptor-registry entry.
    pub entry_id: String,
    /// The stable cohort-binding ID this descriptor binds to (e.g. `launch.cohort.public-preview`); empty means
    /// unstated.
    pub cohort_binding_id: String,
    /// The canonical registry token name (e.g. `cohort.descriptor.public_preview`); empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5LaunchControlRole,
    /// The cohort archetype this entry classifies.
    pub cohort_archetype: M5CohortArchetypeKind,
    /// The render / surface context.
    pub surface_context: M5CohortSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5CohortResolutionForm>,
    /// The published exact repo / archetype rows; empty means unstated.
    pub exact_repo_archetype_rows: String,
    /// The published bundle IDs; empty means unstated.
    pub bundle_ids: String,
    /// The published install topology; empty means unstated.
    pub install_topology: String,
    /// The published toolchain envelope; empty means unstated.
    pub toolchain_envelope: String,
    /// The published known limits; empty means unstated.
    pub known_limits: String,
    /// The published rollback target; empty means unstated.
    pub rollback_target: String,
    /// The published diagnostics posture; empty means unstated.
    pub diagnostics_posture: String,
    /// True when the behavior traces to the cohort-descriptor registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the cohort's rollback and diagnostics posture is preserved before widening (a hard invariant
    /// when `false`).
    pub rollback_and_diagnostics_bounded: bool,
    /// True when this cohort's archetype is public-facing.
    pub is_public_facing_cohort: bool,
    /// True when partner / public support language is matched to cohort proof before a public-facing cohort
    /// widens.
    pub support_language_matches_cohort_proof: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe cohort-descriptor-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCohortDescriptorEntry {
    /// Stable identity of the cohort-descriptor-registry entry.
    pub entry_id: String,
    /// The stable cohort-binding ID this descriptor binds to.
    pub cohort_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the evidence snapshot and signoff before widening.
    pub semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: bool,
    /// The cohort-archetype token named by the entry.
    pub cohort_archetype: String,
    /// Whether the cohort archetype is classified into the resolved taxonomy.
    pub cohort_archetype_is_classified: bool,
    /// The canonical mode for the entry's cohort archetype.
    pub canonical_cohort_archetype_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published exact repo / archetype rows.
    pub exact_repo_archetype_rows: String,
    /// The published bundle IDs.
    pub bundle_ids: String,
    /// The published install topology.
    pub install_topology: String,
    /// The published toolchain envelope.
    pub toolchain_envelope: String,
    /// The published known limits.
    pub known_limits: String,
    /// The published rollback target.
    pub rollback_target: String,
    /// The published diagnostics posture.
    pub diagnostics_posture: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved cohort-descriptor object publishes every required field.
    pub cohort_descriptor_object_complete: bool,
    /// Whether the entry traces to the cohort-descriptor registry.
    pub bound_to_registry: bool,
    /// Whether the cohort's rollback and diagnostics posture stays preserved before widening.
    pub rollback_and_diagnostics_bounded: bool,
    /// Whether this cohort's archetype is public-facing.
    pub is_public_facing_cohort: bool,
    /// Whether partner / public support language is matched to cohort proof before widening.
    pub support_language_matches_cohort_proof: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5CohortDescriptorEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5CohortNextAction,
    /// Whether the descriptor resolves to one typed object across every claimed cohort (clean entry naming every
    /// fact).
    pub descriptor_resolves_across_cohorts: bool,
}

impl M5ResolvedCohortDescriptorEntry {
    /// Whether this cohort-descriptor entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_cohort_evidence_packet_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CohortEvidencePacketEntryResolutionInput {
    /// Stable identity of the cohort-evidence-packet entry.
    pub entry_id: String,
    /// The stable evidence-ref this record binds to; empty means unstated.
    pub evidence_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5LaunchControlRole,
    /// The evidence scope this record must resolve its cohort proof from.
    pub evidence_scope: M5CohortEvidenceScope,
    /// The render / surface context.
    pub surface_context: M5CohortSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5CohortResolutionForm>,
    /// The published resolved cohort identity; empty means missing.
    pub resolved_cohort_identity: String,
    /// The published known-limits ledger; empty means missing.
    pub known_limits_ledger: String,
    /// The published rollback-target reference; empty means missing.
    pub rollback_target_reference: String,
    /// The published rehearsal-currency state; empty means missing.
    pub rehearsal_currency_state: String,
    /// The published readiness-signoff state; empty means missing.
    pub readiness_signoff_state: String,
    /// The published cohort-bound support-language reference; empty means missing.
    pub support_language_reference: String,
    /// The published last widening revision; empty means missing.
    pub last_widening_revision: String,
    /// True when the record keeps the cohort evidence visible.
    pub keeps_cohort_evidence_visible: bool,
    /// True when the evidence is truthful (never claims a clean packet over hidden cohort evidence).
    pub evidence_is_truthful: bool,
    /// True when partner / public support language is present on this record.
    pub support_language_present: bool,
    /// True when the support language is bound to cohort proof rather than running ahead of it.
    pub support_language_bound_to_proof: bool,
    /// True when a known-limits gap is present on this record.
    pub known_limits_gap_present: bool,
    /// True when a known-limits gap is flagged rather than masquerading as covered.
    pub known_limits_gap_flagged: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe cohort-evidence-packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCohortEvidencePacketEntry {
    /// Stable identity of the cohort-evidence-packet entry.
    pub entry_id: String,
    /// The stable evidence-ref this record binds to.
    pub evidence_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the evidence snapshot and signoff before widening.
    pub semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: bool,
    /// The evidence-scope token named by the entry.
    pub evidence_scope: String,
    /// Whether the evidence scope is classified into the resolved taxonomy.
    pub evidence_scope_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published resolved cohort identity.
    pub resolved_cohort_identity: String,
    /// The published known-limits ledger.
    pub known_limits_ledger: String,
    /// The published rollback-target reference.
    pub rollback_target_reference: String,
    /// The published rehearsal-currency state.
    pub rehearsal_currency_state: String,
    /// The published readiness-signoff state.
    pub readiness_signoff_state: String,
    /// The published cohort-bound support-language reference.
    pub support_language_reference: String,
    /// The published last widening revision.
    pub last_widening_revision: String,
    /// Whether the record keeps the cohort evidence visible.
    pub keeps_cohort_evidence_visible: bool,
    /// Whether the evidence is truthful.
    pub evidence_is_truthful: bool,
    /// Whether partner / public support language is present on this build.
    pub support_language_present: bool,
    /// Whether the support language is bound to cohort proof rather than running ahead of it.
    pub support_language_bound_to_proof: bool,
    /// Whether a known-limits gap is present on this record.
    pub known_limits_gap_present: bool,
    /// Whether a known-limits gap is flagged rather than masquerading as covered.
    pub known_limits_gap_flagged: bool,
    /// Whether the record stays honest (cohort evidence visible, support language bound to proof, known-limits
    /// gap flagged).
    pub cohort_evidence_stays_honest: bool,
    /// Whether the entry provides the complete cohort-evidence object (cohort identity, known-limits ledger,
    /// rollback target, rehearsal currency, readiness signoff, support language, last widening revision).
    pub provides_complete_cohort_evidence: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5CohortEvidencePacketEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5CohortNextAction,
    /// Whether the cohort evidence is safe on every claimed cohort (clean entry naming every fact).
    pub evidence_safe_on_every_cohort: bool,
}

impl M5ResolvedCohortEvidencePacketEntry {
    /// Whether this cohort-evidence-packet entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5CohortResolutionError {
    /// The cohort-descriptor-entry id was empty.
    EmptyCohortDescriptorEntryId,
    /// The cohort-evidence-packet-entry id was empty.
    EmptyCohortEvidencePacketEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5CohortResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCohortDescriptorEntryId => "empty_cohort_descriptor_entry_id",
            Self::EmptyCohortEvidencePacketEntryId => "empty_cohort_evidence_packet_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5CohortResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 cohort-descriptor / cohort-evidence-packet registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5CohortResolutionError {}

fn form_tokens(forms: &[M5CohortResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5CohortResolutionForm]) -> bool {
    let present: BTreeSet<M5CohortResolutionForm> = forms.iter().copied().collect();
    M5CohortResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved cohort-descriptor object publishes every required field: classified cohort archetype,
/// exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, known limits, rollback target,
/// and diagnostics posture. An unclassified archetype or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn cohort_descriptor_object_is_complete(
    archetype: M5CohortArchetypeKind,
    exact_repo_archetype_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> bool {
    archetype.is_classified()
        && !exact_repo_archetype_rows.trim().is_empty()
        && !bundle_ids.trim().is_empty()
        && !install_topology.trim().is_empty()
        && !toolchain_envelope.trim().is_empty()
        && !known_limits.trim().is_empty()
        && !rollback_target.trim().is_empty()
        && !diagnostics_posture.trim().is_empty()
}

/// Whether the cohort descriptor keeps a cohort from widening without preserving its rollback and diagnostics
/// posture: the archetype must be classified, the rollback and diagnostics posture must be preserved before
/// widening, and a public-facing cohort must keep its support language matched to cohort proof. An unclassified
/// archetype, an unpreserved rollback / diagnostics posture, or partner / public support language running ahead
/// of proof never matches.
pub fn cohort_preserves_rollback_and_diagnostics_before_widening(
    archetype: M5CohortArchetypeKind,
    rollback_and_diagnostics_bounded: bool,
    is_public_facing_cohort: bool,
    support_language_matches_cohort_proof: bool,
) -> bool {
    archetype.is_classified()
        && rollback_and_diagnostics_bounded
        && (!is_public_facing_cohort || support_language_matches_cohort_proof)
}

/// Whether a cohort-evidence packet stays honest: the scope must be classified, the evidence must be truthful,
/// it must keep the cohort evidence visible, any partner / public support language must be bound to cohort proof
/// rather than running ahead of it, and any known-limits gap must be flagged rather than masquerade as covered.
pub fn cohort_evidence_stays_honest(
    scope: M5CohortEvidenceScope,
    evidence_is_truthful: bool,
    keeps_cohort_evidence_visible: bool,
    support_language_present: bool,
    support_language_bound_to_proof: bool,
    known_limits_gap_present: bool,
    known_limits_gap_flagged: bool,
) -> bool {
    scope.is_classified()
        && evidence_is_truthful
        && keeps_cohort_evidence_visible
        && (!support_language_present || support_language_bound_to_proof)
        && (!known_limits_gap_present || known_limits_gap_flagged)
}

/// Resolves a cohort-descriptor-registry entry so it stays bound to the cohort-descriptor registry: the entry
/// names its canonical token, semantic role, and cohort archetype, covers all three resolution forms, publishes
/// a complete descriptor object (exact repo / archetype rows, bundle IDs, install topology, toolchain envelope,
/// known limits, rollback target, diagnostics posture), preserves its rollback and diagnostics posture before
/// widening so a cohort never widens without it, and keeps a public-facing cohort's support language matched to
/// cohort proof.
pub fn resolve_cohort_descriptor_entry(
    input: M5CohortDescriptorEntryResolutionInput,
) -> Result<M5ResolvedCohortDescriptorEntry, M5CohortResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5CohortResolutionError::EmptyCohortDescriptorEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.cohort_binding_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.exact_repo_archetype_rows)
        || string_is_forbidden(&input.bundle_ids)
        || string_is_forbidden(&input.install_topology)
        || string_is_forbidden(&input.toolchain_envelope)
        || string_is_forbidden(&input.known_limits)
        || string_is_forbidden(&input.rollback_target)
        || string_is_forbidden(&input.diagnostics_posture)
    {
        return Err(M5CohortResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = cohort_descriptor_object_is_complete(
        input.cohort_archetype,
        &input.exact_repo_archetype_rows,
        &input.bundle_ids,
        &input.install_topology,
        &input.toolchain_envelope,
        &input.known_limits,
        &input.rollback_target,
        &input.diagnostics_posture,
    );
    let preserve_ok = cohort_preserves_rollback_and_diagnostics_before_widening(
        input.cohort_archetype,
        input.rollback_and_diagnostics_bounded,
        input.is_public_facing_cohort,
        input.support_language_matches_cohort_proof,
    );
    let support_undisclosed =
        input.is_public_facing_cohort && !input.support_language_matches_cohort_proof;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5CohortDescriptorEntryDegradeReason::DescriptorTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5CohortDescriptorEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.cohort_archetype.is_classified() {
        Some(M5CohortDescriptorEntryDegradeReason::CohortArchetypeUnclassified)
    } else if !input.bound_to_registry {
        Some(M5CohortDescriptorEntryDegradeReason::DescriptorNotBoundToRegistry)
    } else if !object_complete {
        Some(M5CohortDescriptorEntryDegradeReason::CohortDescriptorObjectIncomplete)
    } else if !preserve_ok {
        Some(M5CohortDescriptorEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof)
    } else if !all_forms {
        Some(M5CohortDescriptorEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if support_undisclosed {
        Some(M5CohortDescriptorEntryDegradeReason::RollbackOrDiagnosticsNotPreservedForPublicCohort)
    } else if !input.proof_fresh {
        Some(M5CohortDescriptorEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5CohortNextAction::ExpandCohortMeaning,
    };

    Ok(M5ResolvedCohortDescriptorEntry {
        entry_id: input.entry_id,
        cohort_binding_id: input.cohort_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        cohort_archetype: input.cohort_archetype.as_str().to_owned(),
        cohort_archetype_is_classified: input.cohort_archetype.is_classified(),
        canonical_cohort_archetype_mode: input
            .cohort_archetype
            .canonical_cohort_archetype_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        exact_repo_archetype_rows: input.exact_repo_archetype_rows,
        bundle_ids: input.bundle_ids,
        install_topology: input.install_topology,
        toolchain_envelope: input.toolchain_envelope,
        known_limits: input.known_limits,
        rollback_target: input.rollback_target,
        diagnostics_posture: input.diagnostics_posture,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        cohort_descriptor_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        rollback_and_diagnostics_bounded: input.rollback_and_diagnostics_bounded,
        is_public_facing_cohort: input.is_public_facing_cohort,
        support_language_matches_cohort_proof: input.support_language_matches_cohort_proof,
        degrade_reason,
        next_action,
        descriptor_resolves_across_cohorts: degrade_reason.is_none(),
    })
}

/// Resolves a cohort-evidence-packet entry so its evidence stays safe: the entry names its canonical token,
/// semantic role, and evidence scope, covers all three resolution forms, provides the complete cohort-identity /
/// known-limits-ledger / rollback-target / rehearsal-currency / readiness-signoff / support-language /
/// last-widening-revision cohort-evidence object, and degrades honestly when the evidence would run partner /
/// public support language ahead of cohort proof, hide the cohort evidence, or let a known-limits gap masquerade
/// as covered.
pub fn resolve_cohort_evidence_packet_entry(
    input: M5CohortEvidencePacketEntryResolutionInput,
) -> Result<M5ResolvedCohortEvidencePacketEntry, M5CohortResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5CohortResolutionError::EmptyCohortEvidencePacketEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.evidence_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_cohort_identity)
        || string_is_forbidden(&input.known_limits_ledger)
        || string_is_forbidden(&input.rollback_target_reference)
        || string_is_forbidden(&input.rehearsal_currency_state)
        || string_is_forbidden(&input.readiness_signoff_state)
        || string_is_forbidden(&input.support_language_reference)
        || string_is_forbidden(&input.last_widening_revision)
    {
        return Err(M5CohortResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = cohort_evidence_stays_honest(
        input.evidence_scope,
        input.evidence_is_truthful,
        input.keeps_cohort_evidence_visible,
        input.support_language_present,
        input.support_language_bound_to_proof,
        input.known_limits_gap_present,
        input.known_limits_gap_flagged,
    );
    let provides_record = input.evidence_scope.is_classified()
        && !input.resolved_cohort_identity.trim().is_empty()
        && !input.known_limits_ledger.trim().is_empty()
        && !input.rollback_target_reference.trim().is_empty()
        && !input.rehearsal_currency_state.trim().is_empty()
        && !input.readiness_signoff_state.trim().is_empty()
        && !input.support_language_reference.trim().is_empty()
        && !input.last_widening_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5CohortEvidencePacketEntryDegradeReason::EvidenceTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5CohortEvidencePacketEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.evidence_scope.is_classified() {
        Some(M5CohortEvidencePacketEntryDegradeReason::EvidenceScopeUnclassified)
    } else if !provides_record {
        Some(M5CohortEvidencePacketEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    } else if !all_forms {
        Some(M5CohortEvidencePacketEntryDegradeReason::EvidenceFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5CohortEvidencePacketEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5CohortNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedCohortEvidencePacketEntry {
        entry_id: input.entry_id,
        evidence_ref: input.evidence_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        evidence_scope: input.evidence_scope.as_str().to_owned(),
        evidence_scope_is_classified: input.evidence_scope.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        resolved_cohort_identity: input.resolved_cohort_identity,
        known_limits_ledger: input.known_limits_ledger,
        rollback_target_reference: input.rollback_target_reference,
        rehearsal_currency_state: input.rehearsal_currency_state,
        readiness_signoff_state: input.readiness_signoff_state,
        support_language_reference: input.support_language_reference,
        last_widening_revision: input.last_widening_revision,
        keeps_cohort_evidence_visible: input.keeps_cohort_evidence_visible,
        evidence_is_truthful: input.evidence_is_truthful,
        support_language_present: input.support_language_present,
        support_language_bound_to_proof: input.support_language_bound_to_proof,
        known_limits_gap_present: input.known_limits_gap_present,
        known_limits_gap_flagged: input.known_limits_gap_flagged,
        cohort_evidence_stays_honest: record_stays_honest,
        provides_complete_cohort_evidence: provides_record,
        degrade_reason,
        next_action,
        evidence_safe_on_every_cohort: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved cohort-descriptor and cohort-evidence-packet
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CohortDescriptorEvidencePacketRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5CohortDescriptorEvidencePacketRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5LaunchControlQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Widening stages this row keeps the same truth across.
    pub widening_stages: Vec<M5LaunchControlWideningStage>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5LaunchControlRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5LaunchControlAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5CohortAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5CohortExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    /// Resolved cohort-descriptor-registry examples.
    pub cohort_descriptor_entries: Vec<M5ResolvedCohortDescriptorEntry>,
    /// Resolved cohort-evidence-packet examples.
    pub cohort_evidence_packet_entries: Vec<M5ResolvedCohortEvidencePacketEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the cohort-descriptor and
    /// cohort-evidence-packet domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never widens a cohort without current rollback and diagnostics evidence. MUST be
    /// `false`.
    pub widens_a_cohort_without_current_rollback_and_diagnostics_evidence: bool,
    /// Hard invariant: this row never runs partner or public support language ahead of cohort proof. MUST be
    /// `false`.
    pub runs_partner_or_public_support_language_ahead_of_cohort_proof: bool,
    /// Hard invariant: this row never hides the rollback target or diagnostics posture before widening. MUST be
    /// `false`.
    pub hides_the_rollback_target_or_diagnostics_posture_before_widening: bool,
    /// Hard invariant: this row never collapses distinct cohort evidence classes into one lane. MUST be `false`.
    pub collapses_distinct_cohort_evidence_classes_into_one_lane: bool,
}

impl M5CohortDescriptorEvidencePacketRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5CohortAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5CohortAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5CohortExportField> = self.export_fields.iter().copied().collect();
        M5CohortExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.widens_a_cohort_without_current_rollback_and_diagnostics_evidence
            && !self.runs_partner_or_public_support_language_ahead_of_cohort_proof
            && !self.hides_the_rollback_target_or_diagnostics_posture_before_widening
            && !self.collapses_distinct_cohort_evidence_classes_into_one_lane
    }

    /// True when a clean cohort-descriptor entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified cohort archetype, publishes a complete descriptor object, preserves its rollback and
    /// diagnostics posture, covers all three resolution forms, and keeps a public-facing cohort's support
    /// language matched to proof.
    fn descriptor_is_honest(ex: &M5ResolvedCohortDescriptorEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.cohort_archetype_is_classified
                && ex.cohort_descriptor_object_complete
                && ex.rollback_and_diagnostics_bounded
                && ex.covers_all_resolution_forms
                && (!ex.is_public_facing_cohort || ex.support_language_matches_cohort_proof))
    }

    /// True when a clean cohort-evidence-packet entry preserves a safe packet: it keeps a classified evidence
    /// scope, provides the complete cohort-evidence object, stays honest, and covers all three resolution forms.
    fn evidence_is_honest(ex: &M5ResolvedCohortEvidencePacketEntry) -> bool {
        !ex.is_clean()
            || (ex.evidence_scope_is_classified
                && ex.provides_complete_cohort_evidence
                && ex.cohort_evidence_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.cohort_descriptor_entries
            .iter()
            .all(Self::descriptor_is_honest)
            && self
                .cohort_evidence_packet_entries
                .iter()
                .all(Self::evidence_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CohortDescriptorEvidencePacketRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Cohort-archetype tokens (minted by this lane).
    pub cohort_archetype_kinds: Vec<String>,
    /// Evidence-scope tokens (minted by this lane).
    pub evidence_scopes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Cohort-descriptor-entry degrade-reason tokens.
    pub cohort_descriptor_degrade_reasons: Vec<String>,
    /// Cohort-evidence-packet-entry degrade-reason tokens.
    pub cohort_evidence_packet_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5CohortDescriptorEvidencePacketRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5LaunchControlRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5CohortResolutionForm::ALL, |v| v.as_str()),
            cohort_archetype_kinds: tokens(&M5CohortArchetypeKind::ALL, |v| v.as_str()),
            evidence_scopes: tokens(&M5CohortEvidenceScope::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5CohortSurfaceContext::ALL, |v| v.as_str()),
            cohort_descriptor_degrade_reasons: tokens(
                &M5CohortDescriptorEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            cohort_evidence_packet_degrade_reasons: tokens(
                &M5CohortEvidencePacketEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5CohortAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5CohortNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5CohortExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5LaunchControlConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5CohortDescriptorEvidencePacketRegistriesGovernanceReview {
    /// The descriptor registry names a canonical token, semantic role, and cohort archetype for every entry.
    pub descriptor_registry_names_token_role_and_archetype: bool,
    /// Every claimed cohort resolves to one typed cohort-descriptor object from the shared registry, not
    /// per-entry reconstruction.
    pub cohort_resolves_to_typed_descriptor_from_shared_registry: bool,
    /// The exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles are published for every resolved descriptor.
    pub repo_bundle_toolchain_and_deployment_rows_published: bool,
    /// Cohorts cannot widen without preserving rollback and diagnostics posture before widening.
    pub cohorts_cannot_widen_without_rollback_and_diagnostics: bool,
    /// The cohort evidence keeps the cohort proof visible and binds partner / public support language to it.
    pub cohort_evidence_keeps_proof_visible_and_binds_support_language: bool,
    /// Partner / public support language stays matched to cohort proof for every public-facing cohort.
    pub support_language_matched_to_cohort_proof_for_public_cohorts: bool,
    /// Every cohort-descriptor and cohort-evidence-packet entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Cohort-descriptor and cohort-evidence-packet behavior stay bound to the shared registries rather than
    /// hand-copied per cohort.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shiproom, release center, executive steering, and program governance read a single cohort source.
    pub shiproom_release_center_executive_steering_and_program_governance_read_single_source: bool,
    /// A widen-without-rollback attempt, an incomplete object, or hidden cohort evidence is caught by fixtures
    /// before release evidence turns green.
    pub descriptor_or_evidence_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CohortDescriptorEvidencePacketRegistriesConsumerProjection {
    /// Shiproom and release center consume the shared cohort-descriptor registry.
    pub shiproom_and_release_center_consume_shared_registries: bool,
    /// Executive steering and program governance consume the shared cohort-evidence registry.
    pub executive_steering_and_program_governance_consume_shared_registries: bool,
    /// Diagnostics and public proof consume the shared registries.
    pub diagnostics_and_public_proof_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical cohort-descriptor and cohort-evidence-packet domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical cohort-descriptor / cohort-evidence-packet registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CohortDescriptorEvidencePacketRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CohortDescriptorEvidencePacketRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting cohort audit for the lane.
    pub cohort_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CohortDescriptorEvidencePacketRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CohortDescriptorEvidencePacketRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5CohortDescriptorEvidencePacketRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CohortDescriptorEvidencePacketRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CohortDescriptorEvidencePacketRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CohortDescriptorEvidencePacketRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CohortDescriptorEvidencePacketRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CohortDescriptorEvidencePacketRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 cohort-descriptor and cohort-evidence-packet registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CohortDescriptorEvidencePacketRegistriesPacket {
    /// Record kind; must equal [`M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5CohortDescriptorEvidencePacketRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CohortDescriptorEvidencePacketRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CohortDescriptorEvidencePacketRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CohortDescriptorEvidencePacketRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CohortDescriptorEvidencePacketRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CohortDescriptorEvidencePacketRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CohortDescriptorEvidencePacketRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5CohortDescriptorEvidencePacketRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5CohortDescriptorEvidencePacketRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_RECORD_KIND {
            violations.push(M5CohortDescriptorEvidencePacketRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_SCHEMA_VERSION {
            violations
                .push(M5CohortDescriptorEvidencePacketRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CohortDescriptorEvidencePacketRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations
                .push(M5CohortDescriptorEvidencePacketRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect(
                "m5 cohort-descriptor / cohort-evidence-packet registries packet serializes",
            ),
        ) {
            violations
                .push(M5CohortDescriptorEvidencePacketRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 cohort-descriptor / cohort-evidence-packet registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,cohort_descriptor_entries,cohort_evidence_packet_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .cohort_descriptor_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.cohort_evidence_packet_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.cohort_descriptor_entries.len(),
                row.cohort_evidence_packet_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Cohort-Descriptor and Cohort-Evidence-Packet Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Cohort archetypes: {}\n",
            self.vocabulary_set.cohort_archetype_kinds.join(", ")
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
                "  - Cohort-descriptor entries: {} / cohort-evidence-packet entries: {}\n",
                row.cohort_descriptor_entries.len(),
                row.cohort_evidence_packet_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry cohort reference table generated from the registry, so docs and shiproom runbooks
    /// render the same archetype-mode / repo-rows / bundle-ids / install-topology / toolchain-envelope /
    /// rollback-target truth the resolvers produced rather than a hand-copied cohort table. Only clean,
    /// registry-bound cohort-descriptor entries are listed.
    pub fn render_cohort_descriptor_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| cohort_binding_id | archetype_mode | exact_repo_archetype_rows | bundle_ids | install_topology | toolchain_envelope | rollback_target |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.cohort_descriptor_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.cohort_binding_id,
                    ex.canonical_cohort_archetype_mode,
                    ex.exact_repo_archetype_rows,
                    ex.bundle_ids,
                    ex.install_topology,
                    ex.toolchain_envelope,
                    ex.rollback_target
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5CohortDescriptorEvidencePacketRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CohortDescriptorEvidencePacketRegistriesViolation>),
}

impl fmt::Display for M5CohortDescriptorEvidencePacketRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 cohort-descriptor / cohort-evidence-packet registries export parse failed: {error}"
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
                    "m5 cohort-descriptor / cohort-evidence-packet registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CohortDescriptorEvidencePacketRegistriesArtifactError {}

/// Validation failures emitted by [`M5CohortDescriptorEvidencePacketRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CohortDescriptorEvidencePacketRegistriesViolation {
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
    /// A registry row does not point at both the cohort-descriptor and cohort-evidence-packet domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, widen-without-rollback, field-incomplete,
    /// form-incomplete, or a cohort-evidence entry missing the complete evidence object).
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
    /// Cohort-descriptor-resolution is not proven: clean descriptor entries do not cover the canonical cohort
    /// archetypes or the first release-center / shiproom / executive-steering / program-governance / support
    /// surfaces, no object-incomplete example degrades, or a clean descriptor entry published an incomplete
    /// object.
    CohortDescriptorResolutionNotProven,
    /// Rollback-and-diagnostics-preservation is not proven: no widen-without-rollback example and no unbound
    /// example degrade, no clean bounded descriptor entry is present, or a clean descriptor entry is unbounded
    /// or unbound.
    RollbackAndDiagnosticsPreservationNotProven,
    /// Cohort-evidence-integrity is not proven: clean evidence entries do not cover the canonical dogfood-ring /
    /// rehearsal-currency / go-no-go-signoff scopes with full resolution-form coverage while providing the
    /// complete evidence object, no support-ahead or form-incomplete example degrades, or a clean evidence entry
    /// is missing the complete evidence object.
    CohortEvidenceIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5CohortDescriptorEvidencePacketRegistriesViolation {
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
            Self::CohortDescriptorResolutionNotProven => "cohort_descriptor_resolution_not_proven",
            Self::RollbackAndDiagnosticsPreservationNotProven => {
                "rollback_and_diagnostics_preservation_not_proven"
            }
            Self::CohortEvidenceIntegrityNotProven => "cohort_evidence_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_cohort_descriptor_and_evidence_packet_registries_export() -> Result<
    M5CohortDescriptorEvidencePacketRegistriesPacket,
    M5CohortDescriptorEvidencePacketRegistriesArtifactError,
> {
    let packet: M5CohortDescriptorEvidencePacketRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-cohort-descriptor-and-evidence-packet-registries-proof/support_export.json"
        )
    ))
    .map_err(M5CohortDescriptorEvidencePacketRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CohortDescriptorEvidencePacketRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5CohortDescriptorEvidencePacketRegistriesPacket,
    violations: &mut Vec<M5CohortDescriptorEvidencePacketRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_SCHEMA_REF,
        M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_DOC_REF,
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_COHORT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
        M5_COHORT_EVIDENCE_PACKET_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5CohortDescriptorEvidencePacketRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5CohortDescriptorEvidencePacketRegistriesPacket,
    violations: &mut Vec<M5CohortDescriptorEvidencePacketRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5CohortDescriptorEvidencePacketRegistriesViolation::NoRegistryRows);
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.widening_stages.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations
                .push(M5CohortDescriptorEvidencePacketRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations
                .push(M5CohortDescriptorEvidencePacketRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5CohortDescriptorEvidencePacketRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_COHORT_DESCRIPTOR_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_COHORT_EVIDENCE_PACKET_DOMAIN_SCHEMA_REF)
        {
            violations
                .push(M5CohortDescriptorEvidencePacketRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.cohort_descriptor_entries.is_empty() || row.cohort_evidence_packet_entries.is_empty()
        {
            violations.push(M5CohortDescriptorEvidencePacketRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5CohortDescriptorEvidencePacketRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations
                .push(M5CohortDescriptorEvidencePacketRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5CohortDescriptorEvidencePacketRegistriesPacket,
    violations: &mut Vec<M5CohortDescriptorEvidencePacketRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.descriptor_registry_names_token_role_and_archetype,
        review.cohort_resolves_to_typed_descriptor_from_shared_registry,
        review.repo_bundle_toolchain_and_deployment_rows_published,
        review.cohorts_cannot_widen_without_rollback_and_diagnostics,
        review.cohort_evidence_keeps_proof_visible_and_binds_support_language,
        review.support_language_matched_to_cohort_proof_for_public_cohorts,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shiproom_release_center_executive_steering_and_program_governance_read_single_source,
        review.descriptor_or_evidence_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5CohortDescriptorEvidencePacketRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CohortDescriptorEvidencePacketRegistriesPacket,
    violations: &mut Vec<M5CohortDescriptorEvidencePacketRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shiproom_and_release_center_consume_shared_registries,
        projection.executive_steering_and_program_governance_consume_shared_registries,
        projection.diagnostics_and_public_proof_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5CohortDescriptorEvidencePacketRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CohortDescriptorEvidencePacketRegistriesPacket,
    violations: &mut Vec<M5CohortDescriptorEvidencePacketRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations
            .push(M5CohortDescriptorEvidencePacketRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CohortDescriptorEvidencePacketRegistriesPacket,
    violations: &mut Vec<M5CohortDescriptorEvidencePacketRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.cohort_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations
            .push(M5CohortDescriptorEvidencePacketRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5CohortDescriptorEvidencePacketRegistriesPacket,
    violations: &mut Vec<M5CohortDescriptorEvidencePacketRegistriesViolation>,
) {
    let descriptors = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.cohort_descriptor_entries.iter())
    };
    let evidence = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.cohort_evidence_packet_entries.iter())
    };

    // AC1: every active cohort can be inspected by exact rows, bundles, toolchains, and deployment profiles.
    // Clean descriptor entries cover the canonical cohort archetypes and the first release-center / shiproom /
    // executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no
    // clean descriptor entry published an incomplete object.
    let clean_archetypes: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.cohort_archetype.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let archetypes_covered = M5CohortArchetypeKind::CANONICAL_ARCHETYPES
        .iter()
        .all(|k| clean_archetypes.contains(k.as_str()));
    let first_surfaces_covered = M5CohortSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(M5CohortDescriptorEntryDegradeReason::CohortDescriptorObjectIncomplete)
    });
    let no_clean_incomplete =
        !descriptors().any(|ex| ex.is_clean() && !ex.cohort_descriptor_object_complete);
    if !(archetypes_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5CohortDescriptorEvidencePacketRegistriesViolation::CohortDescriptorResolutionNotProven,
        );
    }

    // AC2: cohort packets preserve rollback and diagnostics posture before widening. A widen-without-rollback
    // example degrades, an unbound example degrades, at least one clean bounded descriptor entry is present, and
    // no clean descriptor entry is unbounded or unbound.
    let widen_fold_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(
                M5CohortDescriptorEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
            )
    });
    let unbound_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(M5CohortDescriptorEntryDegradeReason::DescriptorNotBoundToRegistry)
    });
    let bounded_clean_descriptor =
        descriptors().any(|ex| ex.is_clean() && ex.rollback_and_diagnostics_bounded);
    let no_clean_unbound = !descriptors().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded =
        !descriptors().any(|ex| ex.is_clean() && !ex.rollback_and_diagnostics_bounded);
    if !(widen_fold_degrades
        && unbound_degrades
        && bounded_clean_descriptor
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations.push(
            M5CohortDescriptorEvidencePacketRegistriesViolation::RollbackAndDiagnosticsPreservationNotProven,
        );
    }

    // AC3: claim publication can prove which cohort evidence backs each launch-bearing lane. Clean evidence
    // entries cover every canonical dogfood-ring / rehearsal-currency / go-no-go-signoff scope with full
    // resolution-form coverage while providing the complete evidence object, a support-ahead example degrades, a
    // form-incomplete example degrades, and no clean evidence entry is missing the complete object.
    let clean_evidence_scopes: BTreeSet<String> = evidence()
        .filter(|ex| {
            ex.is_clean()
                && ex.evidence_scope_is_classified
                && ex.provides_complete_cohort_evidence
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.evidence_scope.clone())
        .collect();
    let evidence_scopes_covered = M5CohortEvidenceScope::CANONICAL_SCOPES
        .iter()
        .all(|m| clean_evidence_scopes.contains(m.as_str()));
    let support_ahead_degrades = evidence().any(|ex| {
        ex.degrade_reason
            == Some(
                M5CohortEvidencePacketEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
            )
    });
    let form_incomplete_degrades = evidence().any(|ex| {
        ex.degrade_reason
            == Some(M5CohortEvidencePacketEntryDegradeReason::EvidenceFormCoverageIncomplete)
    });
    let no_clean_missing_evidence =
        !evidence().any(|ex| ex.is_clean() && !ex.provides_complete_cohort_evidence);
    if !(evidence_scopes_covered
        && support_ahead_degrades
        && form_incomplete_degrades
        && no_clean_missing_evidence)
    {
        violations.push(
            M5CohortDescriptorEvidencePacketRegistriesViolation::CohortEvidenceIntegrityNotProven,
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

/// The launch-bearing cohorts this lane implements, for downstream reference: the cohort-descriptor registry
/// covers the core-team canary, design-partner preview, extension-author, public preview, and certified-archetype
/// cohorts the frozen matrix froze, and the cohort-evidence-packet registry binds the evidence that backs each.
pub const IMPLEMENTED_COHORTS: [M5LaunchControlCohort; 5] = M5LaunchControlCohort::ALL;
