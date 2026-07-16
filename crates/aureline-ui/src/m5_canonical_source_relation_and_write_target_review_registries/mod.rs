//! Implemented M5 canonical-source-relation and write-target-review registries.
//!
//! The frozen [historical-reference matrix][matrix] names Aureline's governed non-live-evidence object classes and
//! locks their controlled vocabulary. This module makes the first descriptor-bearing of those classes — a retirement
//! snapshot and a captured support / export evidence packet — operable so CLI, docs / help, review / incident, and
//! support surfaces inherit one canonical snapshot descriptor rather than a hand-authored parallel prose that has to
//! be kept consistent: it turns the *canonical-source-relation* grammar (one machine-readable descriptor per
//! preserved object — one typed field per descriptor section: the canonical object ID and source class, the capture
//! time, the producer / build identity, the provenance lineage and trust class, the retention / removal state, and
//! whether the object is analysis-only, reopenable, or metadata-only — each bound to one object-class identity with
//! its capture-context joins) and the *write-target-review* grammar (the diff scope a descriptor change sits in —
//! a producer-build change, a target-link change, or a retention-state change) into registry resolvers that produce
//! export-safe, honest projections. Every preserved object then resolves to one typed canonical-source-relation
//! object — the fields it publishes, its capture-context joins, its live-target reference, and its retention state,
//! all attributable to its capture context so a snapshot never reads as a current live object — and to one
//! write-target-review object — the resolved object identity, the affected descriptor field, the
//! previous-versus-current descriptor-state reference, the diff-scope state, and the active diff reason — that the
//! CLI, docs / help, support, and review / incident surfaces can inspect without manual reconstruction, so a changed
//! producer build, target link, or retention state stays visible as a typed diff instead of a silent mutation, and a
//! descriptor that cannot bind a field to its capture context degrades honestly instead of leaving a snapshot to read
//! as live.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Emit one machine-readable snapshot descriptor per preserved object, carrying every descriptor field.**
//!   [`resolve_canonical_source_relation_entry`] refuses to read as a clean, registry-bound descriptor entry
//!   unless it names a canonical registry token, a classified [descriptor field][M5CanonicalSourceRelationKind],
//!   a historical-reference role, covers every [resolution form][M5CanonicalSourceRelationResolutionForm] (the
//!   canonical object, the accessible summary, and the audit record), publishes every descriptor field (canonical
//!   object ID and source class, capture time, producer / build identity, provenance lineage and trust class,
//!   retention / removal state, and analysis-only / reopenable / metadata-only disposition bound to capture-context
//!   joins), keeps its non-live evidence attributable to its capture context, and keeps a public-facing field bound to
//!   its provenance; otherwise it degrades.
//! * **Expose live-target and provenance truth without hand-authored parallel prose staying consistent.**
//!   [`line_preserves_rollback_and_diagnostics_before_widening`] rejects a descriptor entry whose
//!   live-target / provenance join is not preserved (a snapshot presenting itself as a current object without one) so
//!   it degrades to
//!   [`M5CanonicalSourceRelationEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof`],
//!   and a public-facing live-target / provenance field whose language outruns its capture context degrades the same
//!   way — the structured blocker reason a present-as-live attempt must surface.
//! * **Turn a changed producer build, target link, or retention state into a visible, typed diff instead of a silent
//!   mutation.** [`resolve_write_target_review_entry`] names a classified [diff scope][M5WriteTargetReviewScope]
//!   (producer-build-change, target-link-change, or retention-state-change), requires the full
//!   object-identity / affected-descriptor-field / previous-versus-current-descriptor-state / diff-scope / active-reason
//!   diff object, covers every resolution form, and degrades to
//!   [`M5WriteTargetReviewEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence`]
//!   when the diff would present stale capture context as current, hide the diff, or let a gap masquerade as covered,
//!   so a descriptor change can never read as trustworthy when it has quietly dropped the reason a snapshot changed
//!   state.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5ConstrainedFileStateRole`] historical-reference
//! role vocabulary and the [`M5ConstrainedFileStateConsumerSurface`] consumer-surface taxonomy — so the shell / archive
//! viewer, help / docs, support, review / incident, runbook-archive, and companion / export surfaces can never fork
//! their own non-live-evidence meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_constrained_file_state_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_canonical_source_relation_and_write_target_review_registries,
    seeded_m5_canonical_source_relation_and_write_target_review_registries_canonical_source_relation_beta_narrowed,
    seeded_m5_canonical_source_relation_and_write_target_review_registries_write_target_review_preview_narrowed,
    M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_constrained_file_state_matrix::{
    M5ConstrainedFileStateAccessibilityRoute, M5ConstrainedFileStateClassificationStage,
    M5ConstrainedFileStateConsumerSurface, M5ConstrainedFileStateDowngradeTrigger,
    M5ConstrainedFileStateObject, M5ConstrainedFileStateQualificationClass,
    M5ConstrainedFileStateRequiredLabel, M5ConstrainedFileStateRole,
    M5_CANONICAL_SOURCE_RELATION_DOMAIN_SCHEMA_REF, M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF,
    M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket`].
pub const M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_canonical_source_relation_and_write_target_review_packet_registries";

/// Schema version for M5 line-canonical_source_relation / line-downgrade-packet registry records.
pub const M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_SCHEMA_REF: &str =
    "schemas/program/m5-canonical-source-relation-and-write-target-review-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_DOC_REF: &str =
    "docs/support/m5_canonical_source_relation_and_write_target_review_registries.md";

/// Repo-relative path of the write-target-review domain schema this lane consumes (reused from the frozen
/// M5 constrained-file-state matrix, which mints it). Registry-B binds each constrained current object to the
/// exact write target the chosen action would or would not change — the current pane, the canonical source,
/// a duplicate copy, an overlay patch, or a no-write path — so a save or write-review surface never leaves a
/// user to infer where authoritative bytes land.
pub const M5_WRITE_TARGET_REVIEW_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-write-target-review.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/support/m5-canonical-source-relation-and-write-target-review-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_CSV_REF: &str =
    "artifacts/support/m5-canonical-source-relation-and-write-target-review-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_REPORT_REF: &str =
    "artifacts/support/m5-canonical-source-relation-and-write-target-review-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/editor/m5-canonical-source-relation-and-write-target-review-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// line invents a parallel surface set.
pub type M5CanonicalSourceRelationWriteTargetReviewRegistriesConsumerSurface =
    M5ConstrainedFileStateConsumerSurface;

/// One of the three resolution forms every line-canonical_source_relation or line-downgrade-packet entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// line-canonical_source_relation and line-downgrade *domains* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CanonicalSourceRelationResolutionForm {
    /// The canonical resolved line-canonical_source_relation / line-downgrade-packet object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved line discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved line inspectable off-renderer.
    AuditRecord,
}

impl M5CanonicalSourceRelationResolutionForm {
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

/// Controlled manifest field a canonical-source-relation entry publishes for one retiring object-class identity, so every
/// retiring class shares one typed registry rather than a hand-copied per-class assumption. Minted by this lane
/// because the frozen matrix names the retiring classes but not the concrete fields a retirement manifest carries —
/// the last-supported version / channel, the retirement trigger, the cutoff date, the successor reference, the
/// disable path, or the export / rollback route. Every classified field carries its canonical mode, and the
/// successor-reference and disable-path fields are public-facing (they surface directly in the user-facing exit
/// language a retiring class publishes) so their claim must stay matched to the closed support note and a current
/// rollback / export route before the class widens its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CanonicalSourceRelationKind {
    /// The last-supported version / channel field: the last supported version or channel pinned to an exact build
    /// before retirement.
    ReadOnlyPathObject,
    /// The retirement-trigger field: the trigger (deprecation-window close, policy decision, successor readiness)
    /// that moves the class to Retired.
    GeneratedArtifactObject,
    /// The cutoff-date field: the date after which the class is retired and can no longer be newly selected.
    PolicyLockedObject,
    /// The successor-reference field (public-facing; the successor / replacement a retiring class routes forward to,
    /// whose claim must match the closed support note and a current rollback / export route).
    ManagedMirroredObject,
    /// The disable-path field (public-facing; the disable path a retiring class exposes so admins can exit safely,
    /// whose claim must match the closed support note and a current rollback / export route).
    ProjectionObject,
    /// The export / rollback-route field: the export / rollback route preserved through retirement so last-supported
    /// state stays recoverable.
    CapturedSnapshotObject,
    /// The manifest field is unclassified, which is disallowed.
    ObjectClassUnclassified,
}

impl M5CanonicalSourceRelationKind {
    /// Every public-proof-ledger kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReadOnlyPathObject,
        Self::GeneratedArtifactObject,
        Self::PolicyLockedObject,
        Self::ManagedMirroredObject,
        Self::ProjectionObject,
        Self::CapturedSnapshotObject,
        Self::ObjectClassUnclassified,
    ];

    /// The six canonical public-proof-ledger kinds every claimed M5 supported line records for its bundles.
    pub const CANONICAL_JOURNEYS: [Self; 6] = [
        Self::ReadOnlyPathObject,
        Self::GeneratedArtifactObject,
        Self::PolicyLockedObject,
        Self::ManagedMirroredObject,
        Self::ProjectionObject,
        Self::CapturedSnapshotObject,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyPathObject => "read_only_path_object",
            Self::GeneratedArtifactObject => "generated_artifact_object",
            Self::PolicyLockedObject => "policy_locked_object",
            Self::ManagedMirroredObject => "managed_mirrored_object",
            Self::ProjectionObject => "projection_object",
            Self::CapturedSnapshotObject => "captured_snapshot_object",
            Self::ObjectClassUnclassified => "object_class_unclassified",
        }
    }

    /// Whether the item is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ObjectClassUnclassified)
    }

    /// The canonical mode for this public-proof-ledger kind.
    pub const fn canonical_report_section_mode(self) -> &'static str {
        match self {
            Self::ReadOnlyPathObject => "read_only_path_object_mode",
            Self::GeneratedArtifactObject => "generated_artifact_object_mode",
            Self::PolicyLockedObject => "policy_locked_object_mode",
            Self::ManagedMirroredObject => "managed_mirrored_object_mode",
            Self::ProjectionObject => "projection_object_mode",
            Self::CapturedSnapshotObject => "captured_snapshot_object_mode",
            Self::ObjectClassUnclassified => "",
        }
    }

    /// Whether this public-proof-ledger kind is public-facing and so must keep its onboarding / migration /
    /// support claim matched to a current, reversible bundle before the line widens.
    pub const fn is_public_facing_line(self) -> bool {
        matches!(self, Self::ManagedMirroredObject | Self::ProjectionObject)
    }
}

/// Controlled write-target-review scope a canonical-source-relation change sits in, so a changed cutoff date or
/// producer build, target link, or retention state becomes a visible, typed diff event rather than a silent
/// mutation and shares one registry rather than a hand-copied per-record assumption. Minted by this lane,
/// tracking whether a snapshot descriptor changed its producing build (producer-build-change), changed the
/// current live-target link it points at (target-link-change), or changed its retention / removal state
/// (retention-state-change). Each scope maps directly to the acceptance-criteria change the implementation
/// requirement names — a changed producer build, a changed target link, or a changed retention state — so a
/// preserved snapshot never mutates silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteTargetReviewScope {
    /// Producer-build-change: a descriptor changed the producer / build identity it was captured from.
    StateClassChange,
    /// Target-link-change: a descriptor changed the current live-target reference it hands off to.
    CanonicalSourceChange,
    /// Retention-state-change: a descriptor changed its retention / removal (expiry / cleanup) state.
    WriteTargetChange,
    /// The write-target-review scope is unclassified, which is disallowed.
    DiffScopeUnclassified,
}

impl M5WriteTargetReviewScope {
    /// Every comparison scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StateClassChange,
        Self::CanonicalSourceChange,
        Self::WriteTargetChange,
        Self::DiffScopeUnclassified,
    ];

    /// The three canonical comparison scopes every claim-history-diff report must stay distinct across.
    pub const CANONICAL_SCOPES: [Self; 3] = [
        Self::StateClassChange,
        Self::CanonicalSourceChange,
        Self::WriteTargetChange,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateClassChange => "state_class_change",
            Self::CanonicalSourceChange => "canonical_source_change",
            Self::WriteTargetChange => "write_target_change",
            Self::DiffScopeUnclassified => "diff_scope_unclassified",
        }
    }

    /// Whether the comparison scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::DiffScopeUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a line-canonical_source_relation or
/// line-downgrade-packet token's meaning stays stable whether it appears in the release-center, shiproom,
/// executive-steering, program-governance, or a support / export form. Minted by this lane, tracking the
/// first-consumer surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CanonicalSourceRelationSurfaceContext {
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

impl M5CanonicalSourceRelationSurfaceContext {
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

/// One mandatory rendered part a line-canonical_source_relation or line-downgrade-packet entry must be able to show, so no
/// line journey, repo / bundle / toolchain / deployment row, known-limits packet, rollback target,
/// line-downgrade field, or registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CanonicalSourceRelationAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The line journey the entry classifies (line-canonical_source_relation entry).
    CohortArchetype,
    /// The exact repo / journey rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles the entry publishes (line-canonical_source_relation entry).
    RepoBundleToolchainAndDeploymentRows,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The known-limits packet and rollback target the entry preserves before widening (line-canonical_source_relation
    /// entry).
    KnownLimitsAndRollbackTarget,
    /// The line-downgrade fields (line identity, known-limits ledger, rollback target, rehearsal currency,
    /// readiness signoff, support language) the entry publishes (line-downgrade-packet entry).
    CohortEvidenceFields,
    /// The support-identity hint the entry publishes (line-downgrade-packet entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved line canonical_source_relation or line downgrade (both entries).
    PlainLanguageMeaning,
}

impl M5CanonicalSourceRelationAnatomyPart {
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
            Self::CohortArchetype => "report_section",
            Self::RepoBundleToolchainAndDeploymentRows => {
                "repo_bundle_toolchain_and_deployment_rows"
            }
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::KnownLimitsAndRollbackTarget => "known_limits_and_rollback_target",
            Self::CohortEvidenceFields => "write_target_review_fields",
            Self::SupportIdentityHint => "support_identity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// line canonical_source_relation, a line-downgrade packet, or a degraded line-canonical_source_relation / line-downgrade-packet entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CanonicalSourceRelationNextAction {
    /// Expand the resolved line canonical_source_relation's or line-downgrade packet's plain-language meaning.
    ExpandCohortMeaning,
    /// Inspect the line journey or downgrade scope the entry resolves.
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

impl M5CanonicalSourceRelationNextAction {
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
            Self::ExpandCohortMeaning => "expand_line_meaning",
            Self::InspectArchetypeOrScope => "inspect_journey_or_scope",
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
pub enum M5CanonicalSourceRelationExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The line families covered.
    CohortFamilies,
    /// The line journeys carried.
    CohortArchetypes,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The downgrade scopes carried.
    EvidenceScopes,
    /// The render / surface context.
    SurfaceContext,
    /// The line-journey modes carried.
    CohortArchetypeModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5CanonicalSourceRelationExportField {
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
            Self::CohortFamilies => "line_families",
            Self::CohortArchetypes => "report_sections",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::EvidenceScopes => "comparison_scopes",
            Self::SurfaceContext => "surface_context",
            Self::CohortArchetypeModes => "report_section_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a line-canonical_source_relation entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, widen-without-rollback, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CanonicalSourceRelationEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the canonical_source_relation means.
    DescriptorTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The line journey is unclassified (not in the resolved taxonomy).
    CohortObjectClassUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    DescriptorNotBoundToRegistry,
    /// The resolved line-canonical_source_relation object is incomplete: the exact repo / journey rows, bundle IDs, install
    /// topology, toolchain envelope, known limits, rollback target, or diagnostics posture is unstated.
    CohortDescriptorObjectIncomplete,
    /// The line's rollback and diagnostics posture is not preserved before widening (a line widening without
    /// a rollback target and diagnostics posture), or a public-facing line ran its support language ahead of
    /// line proof.
    DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A public-facing line did not keep its support language matched to line proof before widening.
    RollbackOrDiagnosticsNotPreservedForPublicCohort,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CanonicalSourceRelationEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::DescriptorTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::CohortObjectClassUnclassified,
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
            Self::DescriptorTokenUnstated => "canonical_source_relation_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::CohortObjectClassUnclassified => "line_object_class_unclassified",
            Self::DescriptorNotBoundToRegistry => "canonical_source_relation_not_bound_to_registry",
            Self::CohortDescriptorObjectIncomplete => "canonical_source_relation_object_incomplete",
            Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof => {
                "canonical_source_relation_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::RollbackOrDiagnosticsNotPreservedForPublicCohort => {
                "rollback_or_diagnostics_not_preserved_for_public_line"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5CanonicalSourceRelationNextAction {
        match self {
            Self::DescriptorTokenUnstated | Self::DescriptorNotBoundToRegistry => {
                M5CanonicalSourceRelationNextAction::TraceCanonicalRegistry
            }
            Self::CohortObjectClassUnclassified
            | Self::CohortDescriptorObjectIncomplete
            | Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof => {
                M5CanonicalSourceRelationNextAction::InspectArchetypeOrScope
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5CanonicalSourceRelationNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::RollbackOrDiagnosticsNotPreservedForPublicCohort
            | Self::ProofStale => M5CanonicalSourceRelationNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5ConstrainedFileStateDowngradeTrigger {
        match self {
            Self::DescriptorTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::DescriptorNotBoundToRegistry => {
                M5ConstrainedFileStateDowngradeTrigger::CanonicalSourceUnstated
            }
            Self::CohortObjectClassUnclassified | Self::CohortDescriptorObjectIncomplete => {
                M5ConstrainedFileStateDowngradeTrigger::StateBadgeMissing
            }
            Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof
            | Self::RollbackOrDiagnosticsNotPreservedForPublicCohort => {
                M5ConstrainedFileStateDowngradeTrigger::ExactWriteTargetUnstated
            }
            Self::ProofStale => {
                M5ConstrainedFileStateDowngradeTrigger::ConstrainedFileStateDescriptorStale
            }
        }
    }
}

/// Reason a line-downgrade-packet entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteTargetReviewEntryDegradeReason {
    /// The canonical registry token name is unstated.
    EvidenceTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The downgrade scope is unclassified (not in the resolved taxonomy).
    EvidenceScopeUnclassified,
    /// The line downgrade would run partner / public support language ahead of line proof, hide the line
    /// downgrade, let a known-limits gap masquerade as covered, or it dropped one of the required line-downgrade
    /// fields (line identity, known-limits ledger, rollback target, rehearsal currency, readiness signoff,
    /// support language, last widening revision).
    CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
    /// The canonical / accessible / audit resolution-form coverage of the downgrade is incomplete.
    EvidenceFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5WriteTargetReviewEntryDegradeReason {
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
            Self::EvidenceTokenUnstated => "comparison_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::EvidenceScopeUnclassified => "comparison_diff_scope_unclassified",
            Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                "write_target_review_runs_support_ahead_of_proof_or_drops_write_target_review"
            }
            Self::EvidenceFormCoverageIncomplete => "comparison_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5CanonicalSourceRelationNextAction {
        match self {
            Self::EvidenceTokenUnstated => {
                M5CanonicalSourceRelationNextAction::TraceCanonicalRegistry
            }
            Self::EvidenceScopeUnclassified
            | Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                M5CanonicalSourceRelationNextAction::InspectArchetypeOrScope
            }
            Self::EvidenceFormCoverageIncomplete => {
                M5CanonicalSourceRelationNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5CanonicalSourceRelationNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5ConstrainedFileStateDowngradeTrigger {
        match self {
            Self::EvidenceTokenUnstated => {
                M5ConstrainedFileStateDowngradeTrigger::CanonicalSourceUnstated
            }
            Self::SurfaceContextUnresolved | Self::EvidenceScopeUnclassified => {
                M5ConstrainedFileStateDowngradeTrigger::PreservedVersusLostSyncUnstated
            }
            Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                M5ConstrainedFileStateDowngradeTrigger::BlockedWriteReasonMissing
            }
            Self::EvidenceFormCoverageIncomplete => {
                M5ConstrainedFileStateDowngradeTrigger::NearestSafeActionMissing
            }
            Self::ProofStale => {
                M5ConstrainedFileStateDowngradeTrigger::ConstrainedFileStateDescriptorStale
            }
        }
    }
}

/// Input to [`resolve_canonical_source_relation_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CanonicalSourceRelationEntryResolutionInput {
    /// Stable identity of the line-canonical_source_relation-registry entry.
    pub entry_id: String,
    /// The stable line-binding ID this canonical_source_relation binds to (e.g. `launch.line.public-preview`); empty means
    /// unstated.
    pub line_binding_id: String,
    /// The canonical registry token name (e.g. `line.canonical_source_relation.projection_object`); empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5ConstrainedFileStateRole,
    /// The line journey this entry classifies.
    pub report_section: M5CanonicalSourceRelationKind,
    /// The render / surface context.
    pub surface_context: M5CanonicalSourceRelationSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5CanonicalSourceRelationResolutionForm>,
    /// The published exact repo / journey rows; empty means unstated.
    pub exact_repo_journey_rows: String,
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
    /// True when the behavior traces to the line-canonical_source_relation registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the line's rollback and diagnostics posture is preserved before widening (a hard invariant
    /// when `false`).
    pub rollback_and_diagnostics_bounded: bool,
    /// True when this line's journey is public-facing.
    pub is_public_facing_line: bool,
    /// True when partner / public support language is matched to line proof before a public-facing line
    /// widens.
    pub support_language_matches_line_proof: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe line-canonical_source_relation-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCanonicalSourceRelationEntry {
    /// Stable identity of the line-canonical_source_relation-registry entry.
    pub entry_id: String,
    /// The stable line-binding ID this canonical_source_relation binds to.
    pub line_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the downgrade snapshot and signoff before widening.
    pub semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: bool,
    /// The line-journey token named by the entry.
    pub report_section: String,
    /// Whether the line journey is classified into the resolved taxonomy.
    pub report_section_is_classified: bool,
    /// The canonical mode for the entry's line journey.
    pub canonical_report_section_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published exact repo / journey rows.
    pub exact_repo_journey_rows: String,
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
    /// Whether the resolved line-canonical_source_relation object publishes every required field.
    pub canonical_source_relation_object_complete: bool,
    /// Whether the entry traces to the line-canonical_source_relation registry.
    pub bound_to_registry: bool,
    /// Whether the line's rollback and diagnostics posture stays preserved before widening.
    pub rollback_and_diagnostics_bounded: bool,
    /// Whether this line's journey is public-facing.
    pub is_public_facing_line: bool,
    /// Whether partner / public support language is matched to line proof before widening.
    pub support_language_matches_line_proof: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5CanonicalSourceRelationEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5CanonicalSourceRelationNextAction,
    /// Whether the canonical_source_relation resolves to one typed object across every claimed line (clean entry naming every
    /// fact).
    pub canonical_source_relation_resolves_across_lines: bool,
}

impl M5ResolvedCanonicalSourceRelationEntry {
    /// Whether this line-canonical_source_relation entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_write_target_review_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WriteTargetReviewEntryResolutionInput {
    /// Stable identity of the line-downgrade-packet entry.
    pub entry_id: String,
    /// The stable downgrade-ref this record binds to; empty means unstated.
    pub comparison_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5ConstrainedFileStateRole,
    /// The downgrade scope this record must resolve its line proof from.
    pub comparison_scope: M5WriteTargetReviewScope,
    /// The render / surface context.
    pub surface_context: M5CanonicalSourceRelationSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5CanonicalSourceRelationResolutionForm>,
    /// The published resolved line identity; empty means missing.
    pub resolved_line_identity: String,
    /// The published known-limits ledger; empty means missing.
    pub known_limits_ledger: String,
    /// The published rollback-target reference; empty means missing.
    pub rollback_target_reference: String,
    /// The published rehearsal-currency state; empty means missing.
    pub rehearsal_currency_state: String,
    /// The published readiness-signoff state; empty means missing.
    pub readiness_signoff_state: String,
    /// The published line-bound support-language reference; empty means missing.
    pub support_language_reference: String,
    /// The published last widening revision; empty means missing.
    pub last_widening_revision: String,
    /// True when the record keeps the line downgrade visible.
    pub keeps_write_target_review_visible: bool,
    /// True when the downgrade is truthful (never claims a clean packet over hidden line downgrade).
    pub comparison_is_truthful: bool,
    /// True when partner / public support language is present on this record.
    pub support_language_present: bool,
    /// True when the support language is bound to line proof rather than running ahead of it.
    pub support_language_bound_to_proof: bool,
    /// True when a known-limits gap is present on this record.
    pub known_limits_gap_present: bool,
    /// True when a known-limits gap is flagged rather than masquerading as covered.
    pub known_limits_gap_flagged: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe line-downgrade-packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWriteTargetReviewEntry {
    /// Stable identity of the line-downgrade-packet entry.
    pub entry_id: String,
    /// The stable downgrade-ref this record binds to.
    pub comparison_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the downgrade snapshot and signoff before widening.
    pub semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: bool,
    /// The downgrade-scope token named by the entry.
    pub comparison_scope: String,
    /// Whether the downgrade scope is classified into the resolved taxonomy.
    pub comparison_scope_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published resolved line identity.
    pub resolved_line_identity: String,
    /// The published known-limits ledger.
    pub known_limits_ledger: String,
    /// The published rollback-target reference.
    pub rollback_target_reference: String,
    /// The published rehearsal-currency state.
    pub rehearsal_currency_state: String,
    /// The published readiness-signoff state.
    pub readiness_signoff_state: String,
    /// The published line-bound support-language reference.
    pub support_language_reference: String,
    /// The published last widening revision.
    pub last_widening_revision: String,
    /// Whether the record keeps the line downgrade visible.
    pub keeps_write_target_review_visible: bool,
    /// Whether the downgrade is truthful.
    pub comparison_is_truthful: bool,
    /// Whether partner / public support language is present on this build.
    pub support_language_present: bool,
    /// Whether the support language is bound to line proof rather than running ahead of it.
    pub support_language_bound_to_proof: bool,
    /// Whether a known-limits gap is present on this record.
    pub known_limits_gap_present: bool,
    /// Whether a known-limits gap is flagged rather than masquerading as covered.
    pub known_limits_gap_flagged: bool,
    /// Whether the record stays honest (line downgrade visible, support language bound to proof, known-limits
    /// gap flagged).
    pub write_target_review_stays_honest: bool,
    /// Whether the entry provides the complete line-downgrade object (line identity, known-limits ledger,
    /// rollback target, rehearsal currency, readiness signoff, support language, last widening revision).
    pub provides_complete_write_target_review: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5WriteTargetReviewEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5CanonicalSourceRelationNextAction,
    /// Whether the line downgrade is safe on every claimed line (clean entry naming every fact).
    pub comparison_safe_on_every_line: bool,
}

impl M5ResolvedWriteTargetReviewEntry {
    /// Whether this line-downgrade-packet entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5CanonicalSourceRelationResolutionError {
    /// The line-canonical_source_relation-entry id was empty.
    EmptyCohortDescriptorEntryId,
    /// The line-downgrade-packet-entry id was empty.
    EmptyCohortEvidencePacketEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5CanonicalSourceRelationResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCohortDescriptorEntryId => "empty_canonical_source_relation_entry_id",
            Self::EmptyCohortEvidencePacketEntryId => "empty_write_target_review_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5CanonicalSourceRelationResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 line-canonical_source_relation / line-downgrade-packet registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5CanonicalSourceRelationResolutionError {}

fn form_tokens(forms: &[M5CanonicalSourceRelationResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5CanonicalSourceRelationResolutionForm]) -> bool {
    let present: BTreeSet<M5CanonicalSourceRelationResolutionForm> =
        forms.iter().copied().collect();
    M5CanonicalSourceRelationResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved line-canonical_source_relation object publishes every required field: classified line journey,
/// exact repo / journey rows, bundle IDs, install topology, toolchain envelope, known limits, rollback target,
/// and diagnostics posture. An unclassified journey or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn canonical_source_relation_object_is_complete(
    journey: M5CanonicalSourceRelationKind,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> bool {
    journey.is_classified()
        && !exact_repo_journey_rows.trim().is_empty()
        && !bundle_ids.trim().is_empty()
        && !install_topology.trim().is_empty()
        && !toolchain_envelope.trim().is_empty()
        && !known_limits.trim().is_empty()
        && !rollback_target.trim().is_empty()
        && !diagnostics_posture.trim().is_empty()
}

/// Whether the line canonical_source_relation keeps a line from widening without preserving its rollback and diagnostics
/// posture: the journey must be classified, the rollback and diagnostics posture must be preserved before
/// widening, and a public-facing line must keep its support language matched to line proof. An unclassified
/// journey, an unpreserved rollback / diagnostics posture, or partner / public support language running ahead
/// of proof never matches.
pub fn line_preserves_rollback_and_diagnostics_before_widening(
    journey: M5CanonicalSourceRelationKind,
    rollback_and_diagnostics_bounded: bool,
    is_public_facing_line: bool,
    support_language_matches_line_proof: bool,
) -> bool {
    journey.is_classified()
        && rollback_and_diagnostics_bounded
        && (!is_public_facing_line || support_language_matches_line_proof)
}

/// Whether a line-downgrade packet stays honest: the scope must be classified, the downgrade must be truthful,
/// it must keep the line downgrade visible, any partner / public support language must be bound to line proof
/// rather than running ahead of it, and any known-limits gap must be flagged rather than masquerade as covered.
pub fn write_target_review_stays_honest(
    scope: M5WriteTargetReviewScope,
    comparison_is_truthful: bool,
    keeps_write_target_review_visible: bool,
    support_language_present: bool,
    support_language_bound_to_proof: bool,
    known_limits_gap_present: bool,
    known_limits_gap_flagged: bool,
) -> bool {
    scope.is_classified()
        && comparison_is_truthful
        && keeps_write_target_review_visible
        && (!support_language_present || support_language_bound_to_proof)
        && (!known_limits_gap_present || known_limits_gap_flagged)
}

/// Resolves a line-canonical_source_relation-registry entry so it stays bound to the line-canonical_source_relation registry: the entry
/// names its canonical token, semantic role, and line journey, covers all three resolution forms, publishes
/// a complete canonical_source_relation object (exact repo / journey rows, bundle IDs, install topology, toolchain envelope,
/// known limits, rollback target, diagnostics posture), preserves its rollback and diagnostics posture before
/// widening so a line never widens without it, and keeps a public-facing line's support language matched to
/// line proof.
pub fn resolve_canonical_source_relation_entry(
    input: M5CanonicalSourceRelationEntryResolutionInput,
) -> Result<M5ResolvedCanonicalSourceRelationEntry, M5CanonicalSourceRelationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5CanonicalSourceRelationResolutionError::EmptyCohortDescriptorEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.line_binding_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.exact_repo_journey_rows)
        || string_is_forbidden(&input.bundle_ids)
        || string_is_forbidden(&input.install_topology)
        || string_is_forbidden(&input.toolchain_envelope)
        || string_is_forbidden(&input.known_limits)
        || string_is_forbidden(&input.rollback_target)
        || string_is_forbidden(&input.diagnostics_posture)
    {
        return Err(M5CanonicalSourceRelationResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = canonical_source_relation_object_is_complete(
        input.report_section,
        &input.exact_repo_journey_rows,
        &input.bundle_ids,
        &input.install_topology,
        &input.toolchain_envelope,
        &input.known_limits,
        &input.rollback_target,
        &input.diagnostics_posture,
    );
    let preserve_ok = line_preserves_rollback_and_diagnostics_before_widening(
        input.report_section,
        input.rollback_and_diagnostics_bounded,
        input.is_public_facing_line,
        input.support_language_matches_line_proof,
    );
    let support_undisclosed =
        input.is_public_facing_line && !input.support_language_matches_line_proof;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5CanonicalSourceRelationEntryDegradeReason::DescriptorTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5CanonicalSourceRelationEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.report_section.is_classified() {
        Some(M5CanonicalSourceRelationEntryDegradeReason::CohortObjectClassUnclassified)
    } else if !input.bound_to_registry {
        Some(M5CanonicalSourceRelationEntryDegradeReason::DescriptorNotBoundToRegistry)
    } else if !object_complete {
        Some(M5CanonicalSourceRelationEntryDegradeReason::CohortDescriptorObjectIncomplete)
    } else if !preserve_ok {
        Some(M5CanonicalSourceRelationEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof)
    } else if !all_forms {
        Some(M5CanonicalSourceRelationEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if support_undisclosed {
        Some(
            M5CanonicalSourceRelationEntryDegradeReason::RollbackOrDiagnosticsNotPreservedForPublicCohort,
        )
    } else if !input.proof_fresh {
        Some(M5CanonicalSourceRelationEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5CanonicalSourceRelationNextAction::ExpandCohortMeaning,
    };

    Ok(M5ResolvedCanonicalSourceRelationEntry {
        entry_id: input.entry_id,
        line_binding_id: input.line_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_be_present_before_surfacing_as_constrained_object(),
        report_section: input.report_section.as_str().to_owned(),
        report_section_is_classified: input.report_section.is_classified(),
        canonical_report_section_mode: input
            .report_section
            .canonical_report_section_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        exact_repo_journey_rows: input.exact_repo_journey_rows,
        bundle_ids: input.bundle_ids,
        install_topology: input.install_topology,
        toolchain_envelope: input.toolchain_envelope,
        known_limits: input.known_limits,
        rollback_target: input.rollback_target,
        diagnostics_posture: input.diagnostics_posture,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        canonical_source_relation_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        rollback_and_diagnostics_bounded: input.rollback_and_diagnostics_bounded,
        is_public_facing_line: input.is_public_facing_line,
        support_language_matches_line_proof: input.support_language_matches_line_proof,
        degrade_reason,
        next_action,
        canonical_source_relation_resolves_across_lines: degrade_reason.is_none(),
    })
}

/// Resolves a line-downgrade-packet entry so its downgrade stays safe: the entry names its canonical token,
/// semantic role, and downgrade scope, covers all three resolution forms, provides the complete line-identity /
/// known-limits-ledger / rollback-target / rehearsal-currency / readiness-signoff / support-language /
/// last-widening-revision line-downgrade object, and degrades honestly when the downgrade would run partner /
/// public support language ahead of line proof, hide the line downgrade, or let a known-limits gap masquerade
/// as covered.
pub fn resolve_write_target_review_entry(
    input: M5WriteTargetReviewEntryResolutionInput,
) -> Result<M5ResolvedWriteTargetReviewEntry, M5CanonicalSourceRelationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5CanonicalSourceRelationResolutionError::EmptyCohortEvidencePacketEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.comparison_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_line_identity)
        || string_is_forbidden(&input.known_limits_ledger)
        || string_is_forbidden(&input.rollback_target_reference)
        || string_is_forbidden(&input.rehearsal_currency_state)
        || string_is_forbidden(&input.readiness_signoff_state)
        || string_is_forbidden(&input.support_language_reference)
        || string_is_forbidden(&input.last_widening_revision)
    {
        return Err(M5CanonicalSourceRelationResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = write_target_review_stays_honest(
        input.comparison_scope,
        input.comparison_is_truthful,
        input.keeps_write_target_review_visible,
        input.support_language_present,
        input.support_language_bound_to_proof,
        input.known_limits_gap_present,
        input.known_limits_gap_flagged,
    );
    let provides_record = input.comparison_scope.is_classified()
        && !input.resolved_line_identity.trim().is_empty()
        && !input.known_limits_ledger.trim().is_empty()
        && !input.rollback_target_reference.trim().is_empty()
        && !input.rehearsal_currency_state.trim().is_empty()
        && !input.readiness_signoff_state.trim().is_empty()
        && !input.support_language_reference.trim().is_empty()
        && !input.last_widening_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5WriteTargetReviewEntryDegradeReason::EvidenceTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5WriteTargetReviewEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.comparison_scope.is_classified() {
        Some(M5WriteTargetReviewEntryDegradeReason::EvidenceScopeUnclassified)
    } else if !provides_record {
        Some(M5WriteTargetReviewEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    } else if !all_forms {
        Some(M5WriteTargetReviewEntryDegradeReason::EvidenceFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5WriteTargetReviewEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5CanonicalSourceRelationNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedWriteTargetReviewEntry {
        entry_id: input.entry_id,
        comparison_ref: input.comparison_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_be_present_before_surfacing_as_constrained_object(),
        comparison_scope: input.comparison_scope.as_str().to_owned(),
        comparison_scope_is_classified: input.comparison_scope.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        resolved_line_identity: input.resolved_line_identity,
        known_limits_ledger: input.known_limits_ledger,
        rollback_target_reference: input.rollback_target_reference,
        rehearsal_currency_state: input.rehearsal_currency_state,
        readiness_signoff_state: input.readiness_signoff_state,
        support_language_reference: input.support_language_reference,
        last_widening_revision: input.last_widening_revision,
        keeps_write_target_review_visible: input.keeps_write_target_review_visible,
        comparison_is_truthful: input.comparison_is_truthful,
        support_language_present: input.support_language_present,
        support_language_bound_to_proof: input.support_language_bound_to_proof,
        known_limits_gap_present: input.known_limits_gap_present,
        known_limits_gap_flagged: input.known_limits_gap_flagged,
        write_target_review_stays_honest: record_stays_honest,
        provides_complete_write_target_review: provides_record,
        degrade_reason,
        next_action,
        comparison_safe_on_every_line: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved line-canonical_source_relation and line-downgrade-packet
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CanonicalSourceRelationWriteTargetReviewRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5CanonicalSourceRelationWriteTargetReviewRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5ConstrainedFileStateQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Widening stages this row keeps the same truth across.
    pub capture_lifecycle_stages: Vec<M5ConstrainedFileStateClassificationStage>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5ConstrainedFileStateRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5ConstrainedFileStateAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5CanonicalSourceRelationAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5CanonicalSourceRelationExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5ConstrainedFileStateDowngradeTrigger>,
    /// Resolved line-canonical_source_relation-registry examples.
    pub canonical_source_relation_entries: Vec<M5ResolvedCanonicalSourceRelationEntry>,
    /// Resolved line-downgrade-packet examples.
    pub write_target_review_entries: Vec<M5ResolvedWriteTargetReviewEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the line-canonical_source_relation and
    /// line-downgrade-packet domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never widens a line without current rollback and diagnostics downgrade. MUST be
    /// `false`.
    pub widens_a_line_without_current_rollback_and_diagnostics_downgrade: bool,
    /// Hard invariant: this row never runs partner or public support language ahead of line proof. MUST be
    /// `false`.
    pub runs_partner_or_public_support_language_ahead_of_line_proof: bool,
    /// Hard invariant: this row never hides the rollback target or diagnostics posture before widening. MUST be
    /// `false`.
    pub hides_the_rollback_target_or_diagnostics_posture_before_widening: bool,
    /// Hard invariant: this row never collapses distinct line downgrade classes into one lane. MUST be `false`.
    pub collapses_distinct_write_target_review_classes_into_one_lane: bool,
}

impl M5CanonicalSourceRelationWriteTargetReviewRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5CanonicalSourceRelationAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5CanonicalSourceRelationAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5CanonicalSourceRelationExportField> =
            self.export_fields.iter().copied().collect();
        M5CanonicalSourceRelationExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.widens_a_line_without_current_rollback_and_diagnostics_downgrade
            && !self.runs_partner_or_public_support_language_ahead_of_line_proof
            && !self.hides_the_rollback_target_or_diagnostics_posture_before_widening
            && !self.collapses_distinct_write_target_review_classes_into_one_lane
    }

    /// True when a clean line-canonical_source_relation entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified line journey, publishes a complete canonical_source_relation object, preserves its rollback and
    /// diagnostics posture, covers all three resolution forms, and keeps a public-facing line's support
    /// language matched to proof.
    fn canonical_source_relation_is_honest(ex: &M5ResolvedCanonicalSourceRelationEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.report_section_is_classified
                && ex.canonical_source_relation_object_complete
                && ex.rollback_and_diagnostics_bounded
                && ex.covers_all_resolution_forms
                && (!ex.is_public_facing_line || ex.support_language_matches_line_proof))
    }

    /// True when a clean line-downgrade-packet entry preserves a safe packet: it keeps a classified downgrade
    /// scope, provides the complete line-downgrade object, stays honest, and covers all three resolution forms.
    fn downgrade_is_honest(ex: &M5ResolvedWriteTargetReviewEntry) -> bool {
        !ex.is_clean()
            || (ex.comparison_scope_is_classified
                && ex.provides_complete_write_target_review
                && ex.write_target_review_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.canonical_source_relation_entries
            .iter()
            .all(Self::canonical_source_relation_is_honest)
            && self
                .write_target_review_entries
                .iter()
                .all(Self::downgrade_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CanonicalSourceRelationWriteTargetReviewRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Cohort-journey tokens (minted by this lane).
    pub report_section_kinds: Vec<String>,
    /// Evidence-scope tokens (minted by this lane).
    pub comparison_scopes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Cohort-canonical_source_relation-entry degrade-reason tokens.
    pub canonical_source_relation_degrade_reasons: Vec<String>,
    /// Cohort-downgrade-packet-entry degrade-reason tokens.
    pub write_target_review_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5CanonicalSourceRelationWriteTargetReviewRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5ConstrainedFileStateRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5CanonicalSourceRelationResolutionForm::ALL, |v| {
                v.as_str()
            }),
            report_section_kinds: tokens(&M5CanonicalSourceRelationKind::ALL, |v| v.as_str()),
            comparison_scopes: tokens(&M5WriteTargetReviewScope::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5CanonicalSourceRelationSurfaceContext::ALL, |v| {
                v.as_str()
            }),
            canonical_source_relation_degrade_reasons: tokens(
                &M5CanonicalSourceRelationEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            write_target_review_degrade_reasons: tokens(
                &M5WriteTargetReviewEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5CanonicalSourceRelationAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5CanonicalSourceRelationNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5CanonicalSourceRelationExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ConstrainedFileStateConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5CanonicalSourceRelationWriteTargetReviewRegistriesGovernanceReview {
    /// The canonical_source_relation registry names a canonical token, semantic role, and line journey for every entry.
    pub canonical_source_relation_registry_names_token_role_and_journey: bool,
    /// Every claimed line resolves to one typed line-canonical_source_relation object from the shared registry, not
    /// per-entry reconstruction.
    pub line_resolves_to_typed_canonical_source_relation_from_shared_registry: bool,
    /// The exact repo / journey rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles are published for every resolved canonical_source_relation.
    pub repo_bundle_toolchain_and_deployment_rows_published: bool,
    /// Cohorts cannot widen without preserving rollback and diagnostics posture before widening.
    pub lines_cannot_widen_without_rollback_and_diagnostics: bool,
    /// The line downgrade keeps the line proof visible and binds partner / public support language to it.
    pub write_target_review_keeps_proof_visible_and_binds_support_language: bool,
    /// Partner / public support language stays matched to line proof for every public-facing line.
    pub support_language_matched_to_line_proof_for_public_lines: bool,
    /// Every line-canonical_source_relation and line-downgrade-packet entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Cohort-canonical_source_relation and line-downgrade-packet behavior stay bound to the shared registries rather than
    /// hand-copied per line.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shiproom, release center, executive steering, and program governance read a single line source.
    pub shiproom_release_center_executive_steering_and_program_governance_read_single_source: bool,
    /// A widen-without-rollback attempt, an incomplete object, or hidden line downgrade is caught by fixtures
    /// before release downgrade turns green.
    pub canonical_source_relation_or_downgrade_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CanonicalSourceRelationWriteTargetReviewRegistriesConsumerProjection {
    /// Shiproom and release center consume the shared line-canonical_source_relation registry.
    pub shiproom_and_release_center_consume_shared_registries: bool,
    /// Executive steering and program governance consume the shared line-downgrade registry.
    pub executive_steering_and_program_governance_consume_shared_registries: bool,
    /// Diagnostics and public proof consume the shared registries.
    pub diagnostics_and_public_proof_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical line-canonical_source_relation and line-downgrade-packet domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical line-canonical_source_relation / line-downgrade-packet registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CanonicalSourceRelationWriteTargetReviewRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CanonicalSourceRelationWriteTargetReviewRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting line audit for the lane.
    pub line_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CanonicalSourceRelationWriteTargetReviewRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5CanonicalSourceRelationWriteTargetReviewRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CanonicalSourceRelationWriteTargetReviewRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CanonicalSourceRelationWriteTargetReviewRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CanonicalSourceRelationWriteTargetReviewRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CanonicalSourceRelationWriteTargetReviewRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CanonicalSourceRelationWriteTargetReviewRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 line-canonical_source_relation and line-downgrade-packet registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket {
    /// Record kind; must equal [`M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5CanonicalSourceRelationWriteTargetReviewRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CanonicalSourceRelationWriteTargetReviewRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CanonicalSourceRelationWriteTargetReviewRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CanonicalSourceRelationWriteTargetReviewRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CanonicalSourceRelationWriteTargetReviewRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CanonicalSourceRelationWriteTargetReviewRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5CanonicalSourceRelationWriteTargetReviewRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_RECORD_KIND
                .to_owned(),
            schema_version:
                M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind
            != M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_RECORD_KIND
        {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::WrongRecordKind,
            );
        }
        if self.schema_version
            != M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::MissingIdentity,
            );
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::VocabularySetDrift,
            );
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(&serde_json::to_value(self).expect(
            "m5 line-canonical_source_relation / line-downgrade-packet registries packet serializes",
        )) {
            violations.push(M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::RawMaterialInExport);
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
            "m5 line-canonical_source_relation / line-downgrade-packet registries packet serializes",
        )
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,canonical_source_relation_entries,write_target_review_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .canonical_source_relation_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.write_target_review_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.canonical_source_relation_entries.len(),
                row.write_target_review_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Canonical-Source-Relation and Write-Target-Review Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Report sections: {}\n",
            self.vocabulary_set.report_section_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Resolution forms: {}\n",
            self.vocabulary_set.resolution_forms.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last audit: {})\n",
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
                "  - Correction-report entries: {} / claim-history-diff entries: {}\n",
                row.canonical_source_relation_entries.len(),
                row.write_target_review_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry line reference table generated from the registry, so docs and shiproom runbooks
    /// render the same journey-mode / repo-rows / bundle-ids / install-topology / toolchain-envelope /
    /// rollback-target truth the resolvers produced rather than a hand-copied line table. Only clean,
    /// registry-bound line-canonical_source_relation entries are listed.
    pub fn render_canonical_source_relation_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| line_binding_id | journey_mode | exact_repo_journey_rows | bundle_ids | install_topology | toolchain_envelope | rollback_target |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.canonical_source_relation_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.line_binding_id,
                    ex.canonical_report_section_mode,
                    ex.exact_repo_journey_rows,
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
pub enum M5CanonicalSourceRelationWriteTargetReviewRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation>),
}

impl fmt::Display for M5CanonicalSourceRelationWriteTargetReviewRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 line-canonical_source_relation / line-downgrade-packet registries export parse failed: {error}"
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
                    "m5 line-canonical_source_relation / line-downgrade-packet registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CanonicalSourceRelationWriteTargetReviewRegistriesArtifactError {}

/// Validation failures emitted by [`M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation {
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
    /// A registry row does not point at both the line-canonical_source_relation and line-downgrade-packet domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, widen-without-rollback, field-incomplete,
    /// form-incomplete, or a line-downgrade entry missing the complete downgrade object).
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
    /// Cohort-canonical_source_relation-resolution is not proven: clean canonical_source_relation entries do not cover the canonical line
    /// journeys or the first release-center / shiproom / executive-steering / program-governance / support
    /// surfaces, no object-incomplete example degrades, or a clean canonical_source_relation entry published an incomplete
    /// object.
    CohortDescriptorResolutionNotProven,
    /// Rollback-and-diagnostics-preservation is not proven: no widen-without-rollback example and no unbound
    /// example degrade, no clean bounded canonical_source_relation entry is present, or a clean canonical_source_relation entry is unbounded
    /// or unbound.
    RollbackAndDiagnosticsPreservationNotProven,
    /// Cohort-downgrade-integrity is not proven: clean downgrade entries do not cover the canonical Retest-pending /
    /// Evidence-stale / narrowed-support scopes with full resolution-form coverage while providing the
    /// complete downgrade object, no support-ahead or form-incomplete example degrades, or a clean downgrade entry
    /// is missing the complete downgrade object.
    CohortEvidenceIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation {
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
            Self::CohortDescriptorResolutionNotProven => {
                "canonical_source_relation_resolution_not_proven"
            }
            Self::RollbackAndDiagnosticsPreservationNotProven => {
                "rollback_and_diagnostics_preservation_not_proven"
            }
            Self::CohortEvidenceIntegrityNotProven => "write_target_review_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_canonical_source_relation_and_write_target_review_registries_export(
) -> Result<
    M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket,
    M5CanonicalSourceRelationWriteTargetReviewRegistriesArtifactError,
> {
    let packet: M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/support/m5-canonical-source-relation-and-write-target-review-registries-proof/support_export.json"
        )
    ))
    .map_err(M5CanonicalSourceRelationWriteTargetReviewRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(
            M5CanonicalSourceRelationWriteTargetReviewRegistriesArtifactError::Validation(
                violations,
            ),
        )
    }
}

fn validate_source_contracts(
    packet: &M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket,
    violations: &mut Vec<M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_SCHEMA_REF,
        M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_DOC_REF,
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
        M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF,
        M5_CANONICAL_SOURCE_RELATION_DOMAIN_SCHEMA_REF,
        M5_WRITE_TARGET_REVIEW_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket,
    violations: &mut Vec<M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations
            .push(M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::NoRegistryRows);
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.capture_lifecycle_stages.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_CANONICAL_SOURCE_RELATION_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_WRITE_TARGET_REVIEW_DOMAIN_SCHEMA_REF)
        {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.canonical_source_relation_entries.is_empty()
            || row.write_target_review_entries.is_empty()
        {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::ExamplesMissing,
            );
        }
        if !row.examples_are_honest() {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::DishonestExample,
            );
        }
        if !row.honours_invariants() {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket,
    violations: &mut Vec<M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.canonical_source_relation_registry_names_token_role_and_journey,
        review.line_resolves_to_typed_canonical_source_relation_from_shared_registry,
        review.repo_bundle_toolchain_and_deployment_rows_published,
        review.lines_cannot_widen_without_rollback_and_diagnostics,
        review.write_target_review_keeps_proof_visible_and_binds_support_language,
        review.support_language_matched_to_line_proof_for_public_lines,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shiproom_release_center_executive_steering_and_program_governance_read_single_source,
        review.canonical_source_relation_or_downgrade_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket,
    violations: &mut Vec<M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation>,
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
                M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket,
    violations: &mut Vec<M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket,
    violations: &mut Vec<M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.line_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket,
    violations: &mut Vec<M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation>,
) {
    let canonical_source_relations = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.canonical_source_relation_entries.iter())
    };
    let downgrade = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.write_target_review_entries.iter())
    };

    // AC1: every active line can be inspected by exact rows, bundles, toolchains, and deployment profiles.
    // Clean canonical_source_relation entries cover the canonical line journeys and the first release-center / shiproom /
    // executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no
    // clean canonical_source_relation entry published an incomplete object.
    let clean_journeys: BTreeSet<String> = canonical_source_relations()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.report_section.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = canonical_source_relations()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let journeys_covered = M5CanonicalSourceRelationKind::CANONICAL_JOURNEYS
        .iter()
        .all(|k| clean_journeys.contains(k.as_str()));
    let first_surfaces_covered = M5CanonicalSourceRelationSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = canonical_source_relations().any(|ex| {
        ex.degrade_reason
            == Some(M5CanonicalSourceRelationEntryDegradeReason::CohortDescriptorObjectIncomplete)
    });
    let no_clean_incomplete = !canonical_source_relations()
        .any(|ex| ex.is_clean() && !ex.canonical_source_relation_object_complete);
    if !(journeys_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::CohortDescriptorResolutionNotProven,
        );
    }

    // AC2: line packets preserve rollback and diagnostics posture before widening. A widen-without-rollback
    // example degrades, an unbound example degrades, at least one clean bounded canonical_source_relation entry is present, and
    // no clean canonical_source_relation entry is unbounded or unbound.
    let widen_fold_degrades = canonical_source_relations().any(|ex| {
        ex.degrade_reason
            == Some(
                M5CanonicalSourceRelationEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
            )
    });
    let unbound_degrades = canonical_source_relations().any(|ex| {
        ex.degrade_reason
            == Some(M5CanonicalSourceRelationEntryDegradeReason::DescriptorNotBoundToRegistry)
    });
    let bounded_clean_canonical_source_relation =
        canonical_source_relations().any(|ex| ex.is_clean() && ex.rollback_and_diagnostics_bounded);
    let no_clean_unbound =
        !canonical_source_relations().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded = !canonical_source_relations()
        .any(|ex| ex.is_clean() && !ex.rollback_and_diagnostics_bounded);
    if !(widen_fold_degrades
        && unbound_degrades
        && bounded_clean_canonical_source_relation
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations.push(
            M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::RollbackAndDiagnosticsPreservationNotProven,
        );
    }

    // AC3: claim publication can prove which line downgrade backs each launch-bearing lane. Clean downgrade
    // entries cover every canonical Retest-pending / Evidence-stale / narrowed-support scope with full
    // resolution-form coverage while providing the complete downgrade object, a support-ahead example degrades, a
    // form-incomplete example degrades, and no clean downgrade entry is missing the complete object.
    let clean_comparison_scopes: BTreeSet<String> = downgrade()
        .filter(|ex| {
            ex.is_clean()
                && ex.comparison_scope_is_classified
                && ex.provides_complete_write_target_review
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.comparison_scope.clone())
        .collect();
    let comparison_scopes_covered = M5WriteTargetReviewScope::CANONICAL_SCOPES
        .iter()
        .all(|m| clean_comparison_scopes.contains(m.as_str()));
    let support_ahead_degrades = downgrade().any(|ex| {
        ex.degrade_reason
            == Some(
                M5WriteTargetReviewEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
            )
    });
    let form_incomplete_degrades = downgrade().any(|ex| {
        ex.degrade_reason
            == Some(M5WriteTargetReviewEntryDegradeReason::EvidenceFormCoverageIncomplete)
    });
    let no_clean_missing_downgrade =
        !downgrade().any(|ex| ex.is_clean() && !ex.provides_complete_write_target_review);
    if !(comparison_scopes_covered
        && support_ahead_degrades
        && form_incomplete_degrades
        && no_clean_missing_downgrade)
    {
        violations.push(
            M5CanonicalSourceRelationWriteTargetReviewRegistriesViolation::CohortEvidenceIntegrityNotProven,
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

/// The launch-bearing lines this lane implements, for downstream reference: the line-canonical_source_relation registry
/// covers the core-team canary, design-partner preview, extension-author, public preview, and certified-journey
/// lines the frozen matrix froze, and the line-downgrade-packet registry binds the downgrade that backs each.
pub const IMPLEMENTED_LINES: [M5ConstrainedFileStateObject; 6] = M5ConstrainedFileStateObject::ALL;
