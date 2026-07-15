//! Implemented M5 stable-line protection-plan and correction-lane queue registries.
//!
//! The frozen [stable-line-protection matrix][matrix] names Aureline's governed post-stable lines — the fresh
//! stable line, the evidence-refresh line, the correction / backport line, the bundle-currentness line, and the
//! LTS-candidate line — and locks their controlled vocabulary. This module is the first implement lane for the
//! concrete first-30-day operating model: it turns the *protection-plan* grammar (how a supported line binds
//! each protected journey — crash recovery, rollback / update, support export, and migration / import, plus
//! other named launch-bearing flows — to its regression queue, publishing the queued-regression issue IDs, the
//! release line, the correction packet, the rollback target, and the delayed-breadth ledger it is auditable by)
//! and the *correction-lane queue* grammar (how a supported line proves which protected-path regression is
//! queued for correction and which breadth work is intentionally delayed while it stays open, keeping every
//! delayed-breadth claim bound to a recorded override or claim-narrowing action rather than to hand-edited
//! prose) into registry resolvers that produce export-safe, honest projections. Every claimed M5 supported line
//! then resolves to one typed protection-plan object — the protected journey it classifies, its queued
//! regressions, release line, correction packet, rollback target, delayed-breadth ledger, and diagnostics
//! posture, all preserved before breadth work resumes so a line never lets breadth silently outrank a crash /
//! rollback / support-export / migration regression — and to one correction-lane queue object — the resolved
//! line identity, the queued-regression ledger, the rollback-target reference, the correction-packet state, the
//! backport-decision state, the delayed-breadth reference, and the last correction revision — that the shiproom,
//! release-center, executive-steering, program-governance, help, and support / export surfaces can inspect
//! without manual reconstruction, so protected journeys stay guarded, breadth work cannot silently outrank an
//! open regression, exact issue / release-line / correction-packet / rollback linkage stays visible, and a line
//! that cannot explain the protection plan it declared or the correction queue that backs it degrades honestly
//! instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one typed protection-plan object per supported line.** [`resolve_protection_plan_entry`] refuses
//!   to read as a clean, registry-bound protection-plan entry unless it names a canonical registry token, a
//!   classified [protected journey][M5ProtectedJourneyKind], a stable-line-protection role, covers every
//!   [resolution form][M5StableLinePlanResolutionForm] (the canonical object, the accessible summary, and the
//!   audit record), publishes every protection-plan field (protected-journey rows, queued-regression issue IDs,
//!   release line, correction packet, delayed-breadth ledger, rollback target, and diagnostics posture),
//!   preserves its rollback and diagnostics posture before widening, and keeps any delayed breadth work bound to
//!   a recorded override or claim-narrowing action; otherwise it degrades.
//! * **Keep breadth work from silently outranking a protected-journey regression.**
//!   [`line_preserves_rollback_and_diagnostics_before_widening`] rejects a protection-plan entry whose rollback
//!   and diagnostics posture is not preserved (a line resuming breadth work without a rollback target and
//!   diagnostics posture) so it degrades to
//!   [`M5ProtectionPlanEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof`],
//!   and a line whose breadth work outranks an open regression without a recorded override degrades the same way
//!   — the structured blocker reason a breadth-over-regression attempt must surface.
//! * **Keep the correction queue from running breadth ahead of an open regression or dropping the queue.**
//!   [`resolve_correction_queue_entry`] names a classified [correction scope][M5CorrectionQueueScope], requires
//!   the full line-identity / queued-regression-ledger / rollback-target / correction-packet /
//!   backport-decision / delayed-breadth / last-correction-revision queue object, covers every resolution form,
//!   and degrades to
//!   [`M5CorrectionQueueEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence`] when the
//!   queue would run breadth work ahead of an open regression, hide the correction queue, or let a queued
//!   regression masquerade as covered, so a correction-lane queue can never read as trustworthy when it has
//!   quietly dropped the reason a lane is actually holding breadth work.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5StableLineProtectionRole`] role
//! vocabulary and the [`M5StableLineProtectionConsumerSurface`] consumer-surface taxonomy — so the shiproom,
//! release-center, executive-steering, program-governance, diagnostics, docs, CLI, support, and public-proof
//! surfaces can never fork their own line meaning. Raw secret values and private endpoints stay outside the
//! export boundary.
//!
//! [matrix]: crate::m5_stable_line_protection_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_stable_line_protection_plan_and_correction_queue_registries,
    seeded_m5_stable_line_protection_plan_and_correction_queue_registries_correction_queue_preview_narrowed,
    seeded_m5_stable_line_protection_plan_and_correction_queue_registries_protection_plan_beta_narrowed,
    M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_stable_line_protection_matrix::{
    M5StableLineProtectionAccessibilityRoute, M5StableLineProtectionConsumerSurface,
    M5StableLineProtectionDowngradeTrigger, M5StableLineProtectionLine,
    M5StableLineProtectionQualificationClass, M5StableLineProtectionRequiredLabel,
    M5StableLineProtectionRole, M5StableLineProtectionWideningStage,
    M5_STABLE_LINE_PROTECTION_MATRIX_DOC_REF, M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
};

/// Repo-relative path of the canonical stable-line protection-plan domain schema minted by this lane (how a
/// supported line binds each protected journey — crash recovery, rollback / update, support export, and
/// migration / import — to its regression queue, issue IDs, release line, correction packet, and rollback
/// target). Minted locally because the frozen stable-line-protection matrix names the refresh-policy,
/// defect-ledger, and LTS-readiness domains but not the first-30-day protection-plan grammar.
pub const M5_STABLE_LINE_PROTECTION_PLAN_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-stable-line-protection-plan.schema.json";

/// Stable record-kind tag carried by [`M5StableLineProtectionPlanCorrectionQueueRegistriesPacket`].
pub const M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_stable_line_protection_plan_and_correction_lane_queue_registries";

/// Schema version for M5 line-protection_plan / line-correction-packet registry records.
pub const M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_SCHEMA_REF: &str =
    "schemas/program/m5-stable-line-protection-plan-and-correction-queue-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_DOC_REF: &str =
    "docs/release/m5_stable_line_protection_plan_and_correction_queue_registries.md";

/// Repo-relative path of the canonical line-correction-packet domain schema minted by this lane (how a line
/// proves which correction class — dogfood-ring telemetry, rehearsal currency, or go/no-go signoff — backs it).
pub const M5_CORRECTION_LANE_QUEUE_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-correction-lane-queue.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-stable-line-protection-plan-and-correction-queue-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-stable-line-protection-plan-and-correction-queue-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-stable-line-protection-plan-and-correction-queue-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/release/m5-stable-line-protection-plan-and-correction-queue-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// line invents a parallel surface set.
pub type M5StableLineProtectionPlanCorrectionQueueRegistriesConsumerSurface =
    M5StableLineProtectionConsumerSurface;

/// One of the three resolution forms every line-protection_plan or line-correction-packet entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// line-protection_plan and line-correction *domains* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLinePlanResolutionForm {
    /// The canonical resolved line-protection_plan / line-correction-packet object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved line discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved line inspectable off-renderer.
    AuditRecord,
}

impl M5StableLinePlanResolutionForm {
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

/// Controlled line journey a line-protection_plan entry classifies, so the typed protection_plan model shares one
/// registry rather than a hand-copied per-line assumption. Minted by this lane because the frozen matrix
/// carries the launch-bearing lines but distinguishes the dogfood / migration-alpha / extension-author /
/// design-partner / public-preview / certified-journey journeys an auditable protection_plan classifies against
/// explicitly. Every classified journey carries its canonical mode, and the design-partner-preview and
/// public-preview journeys are public-facing so their partner / public support language must stay matched to
/// line proof before the line widens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProtectedJourneyKind {
    /// The internal dogfood core-team canary line.
    CrashRecoveryJourney,
    /// The migration alpha line (external alpha migrating from a prior toolchain).
    MigrationImportJourney,
    /// The extension-author line (compatibility rehearsals current, freeze exceptions documented).
    SupportExportJourney,
    /// The design-partner preview line (public-facing; support language must match line proof).
    RollbackUpdateJourney,
    /// The public preview line (public-facing; support language must match line proof).
    LaunchBearingFlowJourney,
    /// The certified-journey line (ORR signed and a go/no-go decision recorded).
    NamedProtectedJourney,
    /// The line journey is unclassified, which is disallowed.
    JourneyUnclassified,
}

impl M5ProtectedJourneyKind {
    /// Every line journey, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::CrashRecoveryJourney,
        Self::MigrationImportJourney,
        Self::SupportExportJourney,
        Self::RollbackUpdateJourney,
        Self::LaunchBearingFlowJourney,
        Self::NamedProtectedJourney,
        Self::JourneyUnclassified,
    ];

    /// The six canonical line journeys every claimed M5 launch-bearing line classifies against.
    pub const CANONICAL_JOURNEYS: [Self; 6] = [
        Self::CrashRecoveryJourney,
        Self::MigrationImportJourney,
        Self::SupportExportJourney,
        Self::RollbackUpdateJourney,
        Self::LaunchBearingFlowJourney,
        Self::NamedProtectedJourney,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashRecoveryJourney => "crash_recovery_journey",
            Self::MigrationImportJourney => "migration_import_journey",
            Self::SupportExportJourney => "support_export_journey",
            Self::RollbackUpdateJourney => "rollback_update_journey",
            Self::LaunchBearingFlowJourney => "launch_bearing_flow_journey",
            Self::NamedProtectedJourney => "named_protected_journey",
            Self::JourneyUnclassified => "journey_unclassified",
        }
    }

    /// Whether the journey is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::JourneyUnclassified)
    }

    /// The canonical mode for this line journey.
    pub const fn canonical_protected_journey_mode(self) -> &'static str {
        match self {
            Self::CrashRecoveryJourney => "crash_recovery_journey_mode",
            Self::MigrationImportJourney => "migration_import_journey_mode",
            Self::SupportExportJourney => "support_export_journey_mode",
            Self::RollbackUpdateJourney => "rollback_update_journey_mode",
            Self::LaunchBearingFlowJourney => "launch_bearing_flow_journey_mode",
            Self::NamedProtectedJourney => "named_protected_journey_mode",
            Self::JourneyUnclassified => "",
        }
    }

    /// Whether this journey is public-facing and so must keep partner / public support language matched to
    /// line proof before the line widens.
    pub const fn is_public_facing_line(self) -> bool {
        matches!(
            self,
            Self::RollbackUpdateJourney | Self::LaunchBearingFlowJourney
        )
    }
}

/// Controlled correction scope a line-correction-packet entry must resolve its line proof from, so an correction
/// packet shares one registry rather than a hand-copied per-record assumption. Minted by this lane, tracking
/// whether the correction came from dogfood-ring telemetry, current rehearsal cadence, or an explicit go/no-go
/// signoff the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CorrectionQueueScope {
    /// The correction came from internal dogfood-ring telemetry.
    QueuedRegressionScope,
    /// The correction came from current rehearsal cadence (publish/rollback, mixed-version, handoff drills).
    BackportDecisionScope,
    /// The correction came from an explicit go/no-go signoff with a preserved correction snapshot.
    CorrectionReportScope,
    /// The correction scope is unclassified, which is disallowed.
    ScopeUnclassified,
}

impl M5CorrectionQueueScope {
    /// Every correction scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::QueuedRegressionScope,
        Self::BackportDecisionScope,
        Self::CorrectionReportScope,
        Self::ScopeUnclassified,
    ];

    /// The three canonical correction scopes every line-correction packet must stay distinct across.
    pub const CANONICAL_SCOPES: [Self; 3] = [
        Self::QueuedRegressionScope,
        Self::BackportDecisionScope,
        Self::CorrectionReportScope,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueuedRegressionScope => "queued_regression_scope",
            Self::BackportDecisionScope => "backport_decision_scope",
            Self::CorrectionReportScope => "correction_report_scope",
            Self::ScopeUnclassified => "scope_unclassified",
        }
    }

    /// Whether the correction scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ScopeUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a line-protection_plan or
/// line-correction-packet token's meaning stays stable whether it appears in the release-center, shiproom,
/// executive-steering, program-governance, or a support / export form. Minted by this lane, tracking the
/// first-consumer surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLinePlanSurfaceContext {
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

impl M5StableLinePlanSurfaceContext {
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

/// One mandatory rendered part a line-protection_plan or line-correction-packet entry must be able to show, so no
/// line journey, repo / bundle / toolchain / deployment row, known-limits packet, rollback target,
/// line-correction field, or registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLinePlanAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The line journey the entry classifies (line-protection_plan entry).
    CohortArchetype,
    /// The exact repo / journey rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles the entry publishes (line-protection_plan entry).
    RepoBundleToolchainAndDeploymentRows,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The known-limits packet and rollback target the entry preserves before widening (line-protection_plan
    /// entry).
    KnownLimitsAndRollbackTarget,
    /// The line-correction fields (line identity, known-limits ledger, rollback target, rehearsal currency,
    /// readiness signoff, support language) the entry publishes (line-correction-packet entry).
    CohortEvidenceFields,
    /// The support-identity hint the entry publishes (line-correction-packet entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved line protection_plan or line correction (both entries).
    PlainLanguageMeaning,
}

impl M5StableLinePlanAnatomyPart {
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
            Self::CohortArchetype => "protected_journey",
            Self::RepoBundleToolchainAndDeploymentRows => {
                "repo_bundle_toolchain_and_deployment_rows"
            }
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::KnownLimitsAndRollbackTarget => "known_limits_and_rollback_target",
            Self::CohortEvidenceFields => "correction_queue_fields",
            Self::SupportIdentityHint => "support_identity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// line protection_plan, a line-correction packet, or a degraded line-protection_plan / line-correction-packet entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLinePlanNextAction {
    /// Expand the resolved line protection_plan's or line-correction packet's plain-language meaning.
    ExpandCohortMeaning,
    /// Inspect the line journey or correction scope the entry resolves.
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

impl M5StableLinePlanNextAction {
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
pub enum M5StableLinePlanExportField {
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
    /// The correction scopes carried.
    EvidenceScopes,
    /// The render / surface context.
    SurfaceContext,
    /// The line-journey modes carried.
    CohortArchetypeModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5StableLinePlanExportField {
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
            Self::CohortArchetypes => "protected_journeys",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::EvidenceScopes => "correction_scopes",
            Self::SurfaceContext => "surface_context",
            Self::CohortArchetypeModes => "protected_journey_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a line-protection_plan entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, widen-without-rollback, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProtectionPlanEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the protection_plan means.
    DescriptorTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The line journey is unclassified (not in the resolved taxonomy).
    CohortJourneyUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    DescriptorNotBoundToRegistry,
    /// The resolved line-protection_plan object is incomplete: the exact repo / journey rows, bundle IDs, install
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

impl M5ProtectionPlanEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::DescriptorTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::CohortJourneyUnclassified,
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
            Self::DescriptorTokenUnstated => "protection_plan_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::CohortJourneyUnclassified => "line_journey_unclassified",
            Self::DescriptorNotBoundToRegistry => "protection_plan_not_bound_to_registry",
            Self::CohortDescriptorObjectIncomplete => "protection_plan_object_incomplete",
            Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof => {
                "protection_plan_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::RollbackOrDiagnosticsNotPreservedForPublicCohort => {
                "rollback_or_diagnostics_not_preserved_for_public_line"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5StableLinePlanNextAction {
        match self {
            Self::DescriptorTokenUnstated | Self::DescriptorNotBoundToRegistry => {
                M5StableLinePlanNextAction::TraceCanonicalRegistry
            }
            Self::CohortJourneyUnclassified
            | Self::CohortDescriptorObjectIncomplete
            | Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof => {
                M5StableLinePlanNextAction::InspectArchetypeOrScope
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5StableLinePlanNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::RollbackOrDiagnosticsNotPreservedForPublicCohort
            | Self::ProofStale => M5StableLinePlanNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5StableLineProtectionDowngradeTrigger {
        match self {
            Self::DescriptorTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::DescriptorNotBoundToRegistry => {
                M5StableLineProtectionDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::CohortJourneyUnclassified | Self::CohortDescriptorObjectIncomplete => {
                M5StableLineProtectionDowngradeTrigger::SupportWindowUnstated
            }
            Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof
            | Self::RollbackOrDiagnosticsNotPreservedForPublicCohort => {
                M5StableLineProtectionDowngradeTrigger::WidenedSupportWithoutCurrentRefreshEvidence
            }
            Self::ProofStale => M5StableLineProtectionDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a line-correction-packet entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CorrectionQueueEntryDegradeReason {
    /// The canonical registry token name is unstated.
    EvidenceTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The correction scope is unclassified (not in the resolved taxonomy).
    EvidenceScopeUnclassified,
    /// The line correction would run partner / public support language ahead of line proof, hide the line
    /// correction, let a known-limits gap masquerade as covered, or it dropped one of the required line-correction
    /// fields (line identity, known-limits ledger, rollback target, rehearsal currency, readiness signoff,
    /// support language, last widening revision).
    CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
    /// The canonical / accessible / audit resolution-form coverage of the correction is incomplete.
    EvidenceFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CorrectionQueueEntryDegradeReason {
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
            Self::EvidenceTokenUnstated => "correction_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::EvidenceScopeUnclassified => "correction_scope_unclassified",
            Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                "correction_queue_runs_support_ahead_of_proof_or_drops_correction_queue"
            }
            Self::EvidenceFormCoverageIncomplete => "correction_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5StableLinePlanNextAction {
        match self {
            Self::EvidenceTokenUnstated => M5StableLinePlanNextAction::TraceCanonicalRegistry,
            Self::EvidenceScopeUnclassified
            | Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                M5StableLinePlanNextAction::InspectArchetypeOrScope
            }
            Self::EvidenceFormCoverageIncomplete => {
                M5StableLinePlanNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5StableLinePlanNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5StableLineProtectionDowngradeTrigger {
        match self {
            Self::EvidenceTokenUnstated => {
                M5StableLineProtectionDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::EvidenceScopeUnclassified => {
                M5StableLineProtectionDowngradeTrigger::RefreshStateUnstated
            }
            Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                M5StableLineProtectionDowngradeTrigger::RanSupportLanguageAheadOfRefreshProof
            }
            Self::EvidenceFormCoverageIncomplete => {
                M5StableLineProtectionDowngradeTrigger::ImpliedGreenWhileRefreshOrLedgerWasStale
            }
            Self::ProofStale => M5StableLineProtectionDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_protection_plan_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ProtectionPlanEntryResolutionInput {
    /// Stable identity of the line-protection_plan-registry entry.
    pub entry_id: String,
    /// The stable line-binding ID this protection_plan binds to (e.g. `launch.line.public-preview`); empty means
    /// unstated.
    pub line_binding_id: String,
    /// The canonical registry token name (e.g. `line.protection_plan.launch_bearing_flow_journey`); empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5StableLineProtectionRole,
    /// The line journey this entry classifies.
    pub protected_journey: M5ProtectedJourneyKind,
    /// The render / surface context.
    pub surface_context: M5StableLinePlanSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5StableLinePlanResolutionForm>,
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
    /// True when the behavior traces to the line-protection_plan registry (never a hand-copied constant).
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

/// Resolved, export-safe line-protection_plan-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedProtectionPlanEntry {
    /// Stable identity of the line-protection_plan-registry entry.
    pub entry_id: String,
    /// The stable line-binding ID this protection_plan binds to.
    pub line_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the correction snapshot and signoff before widening.
    pub semantic_role_must_preserve_correction_snapshot_and_signoff_before_widening: bool,
    /// The line-journey token named by the entry.
    pub protected_journey: String,
    /// Whether the line journey is classified into the resolved taxonomy.
    pub protected_journey_is_classified: bool,
    /// The canonical mode for the entry's line journey.
    pub canonical_protected_journey_mode: String,
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
    /// Whether the resolved line-protection_plan object publishes every required field.
    pub protection_plan_object_complete: bool,
    /// Whether the entry traces to the line-protection_plan registry.
    pub bound_to_registry: bool,
    /// Whether the line's rollback and diagnostics posture stays preserved before widening.
    pub rollback_and_diagnostics_bounded: bool,
    /// Whether this line's journey is public-facing.
    pub is_public_facing_line: bool,
    /// Whether partner / public support language is matched to line proof before widening.
    pub support_language_matches_line_proof: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5ProtectionPlanEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5StableLinePlanNextAction,
    /// Whether the protection_plan resolves to one typed object across every claimed line (clean entry naming every
    /// fact).
    pub protection_plan_resolves_across_lines: bool,
}

impl M5ResolvedProtectionPlanEntry {
    /// Whether this line-protection_plan entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_correction_queue_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CorrectionQueueEntryResolutionInput {
    /// Stable identity of the line-correction-packet entry.
    pub entry_id: String,
    /// The stable correction-ref this record binds to; empty means unstated.
    pub correction_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5StableLineProtectionRole,
    /// The correction scope this record must resolve its line proof from.
    pub correction_scope: M5CorrectionQueueScope,
    /// The render / surface context.
    pub surface_context: M5StableLinePlanSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5StableLinePlanResolutionForm>,
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
    /// True when the record keeps the line correction visible.
    pub keeps_correction_queue_visible: bool,
    /// True when the correction is truthful (never claims a clean packet over hidden line correction).
    pub correction_is_truthful: bool,
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

/// Resolved, export-safe line-correction-packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCorrectionQueueEntry {
    /// Stable identity of the line-correction-packet entry.
    pub entry_id: String,
    /// The stable correction-ref this record binds to.
    pub correction_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the correction snapshot and signoff before widening.
    pub semantic_role_must_preserve_correction_snapshot_and_signoff_before_widening: bool,
    /// The correction-scope token named by the entry.
    pub correction_scope: String,
    /// Whether the correction scope is classified into the resolved taxonomy.
    pub correction_scope_is_classified: bool,
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
    /// Whether the record keeps the line correction visible.
    pub keeps_correction_queue_visible: bool,
    /// Whether the correction is truthful.
    pub correction_is_truthful: bool,
    /// Whether partner / public support language is present on this build.
    pub support_language_present: bool,
    /// Whether the support language is bound to line proof rather than running ahead of it.
    pub support_language_bound_to_proof: bool,
    /// Whether a known-limits gap is present on this record.
    pub known_limits_gap_present: bool,
    /// Whether a known-limits gap is flagged rather than masquerading as covered.
    pub known_limits_gap_flagged: bool,
    /// Whether the record stays honest (line correction visible, support language bound to proof, known-limits
    /// gap flagged).
    pub correction_queue_stays_honest: bool,
    /// Whether the entry provides the complete line-correction object (line identity, known-limits ledger,
    /// rollback target, rehearsal currency, readiness signoff, support language, last widening revision).
    pub provides_complete_correction_queue: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5CorrectionQueueEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5StableLinePlanNextAction,
    /// Whether the line correction is safe on every claimed line (clean entry naming every fact).
    pub correction_safe_on_every_line: bool,
}

impl M5ResolvedCorrectionQueueEntry {
    /// Whether this line-correction-packet entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5StableLinePlanResolutionError {
    /// The line-protection_plan-entry id was empty.
    EmptyCohortDescriptorEntryId,
    /// The line-correction-packet-entry id was empty.
    EmptyCohortEvidencePacketEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5StableLinePlanResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCohortDescriptorEntryId => "empty_protection_plan_entry_id",
            Self::EmptyCohortEvidencePacketEntryId => "empty_correction_queue_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5StableLinePlanResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 line-protection_plan / line-correction-packet registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5StableLinePlanResolutionError {}

fn form_tokens(forms: &[M5StableLinePlanResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5StableLinePlanResolutionForm]) -> bool {
    let present: BTreeSet<M5StableLinePlanResolutionForm> = forms.iter().copied().collect();
    M5StableLinePlanResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved line-protection_plan object publishes every required field: classified line journey,
/// exact repo / journey rows, bundle IDs, install topology, toolchain envelope, known limits, rollback target,
/// and diagnostics posture. An unclassified journey or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn protection_plan_object_is_complete(
    journey: M5ProtectedJourneyKind,
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

/// Whether the line protection_plan keeps a line from widening without preserving its rollback and diagnostics
/// posture: the journey must be classified, the rollback and diagnostics posture must be preserved before
/// widening, and a public-facing line must keep its support language matched to line proof. An unclassified
/// journey, an unpreserved rollback / diagnostics posture, or partner / public support language running ahead
/// of proof never matches.
pub fn line_preserves_rollback_and_diagnostics_before_widening(
    journey: M5ProtectedJourneyKind,
    rollback_and_diagnostics_bounded: bool,
    is_public_facing_line: bool,
    support_language_matches_line_proof: bool,
) -> bool {
    journey.is_classified()
        && rollback_and_diagnostics_bounded
        && (!is_public_facing_line || support_language_matches_line_proof)
}

/// Whether a line-correction packet stays honest: the scope must be classified, the correction must be truthful,
/// it must keep the line correction visible, any partner / public support language must be bound to line proof
/// rather than running ahead of it, and any known-limits gap must be flagged rather than masquerade as covered.
pub fn correction_queue_stays_honest(
    scope: M5CorrectionQueueScope,
    correction_is_truthful: bool,
    keeps_correction_queue_visible: bool,
    support_language_present: bool,
    support_language_bound_to_proof: bool,
    known_limits_gap_present: bool,
    known_limits_gap_flagged: bool,
) -> bool {
    scope.is_classified()
        && correction_is_truthful
        && keeps_correction_queue_visible
        && (!support_language_present || support_language_bound_to_proof)
        && (!known_limits_gap_present || known_limits_gap_flagged)
}

/// Resolves a line-protection_plan-registry entry so it stays bound to the line-protection_plan registry: the entry
/// names its canonical token, semantic role, and line journey, covers all three resolution forms, publishes
/// a complete protection_plan object (exact repo / journey rows, bundle IDs, install topology, toolchain envelope,
/// known limits, rollback target, diagnostics posture), preserves its rollback and diagnostics posture before
/// widening so a line never widens without it, and keeps a public-facing line's support language matched to
/// line proof.
pub fn resolve_protection_plan_entry(
    input: M5ProtectionPlanEntryResolutionInput,
) -> Result<M5ResolvedProtectionPlanEntry, M5StableLinePlanResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5StableLinePlanResolutionError::EmptyCohortDescriptorEntryId);
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
        return Err(M5StableLinePlanResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = protection_plan_object_is_complete(
        input.protected_journey,
        &input.exact_repo_journey_rows,
        &input.bundle_ids,
        &input.install_topology,
        &input.toolchain_envelope,
        &input.known_limits,
        &input.rollback_target,
        &input.diagnostics_posture,
    );
    let preserve_ok = line_preserves_rollback_and_diagnostics_before_widening(
        input.protected_journey,
        input.rollback_and_diagnostics_bounded,
        input.is_public_facing_line,
        input.support_language_matches_line_proof,
    );
    let support_undisclosed =
        input.is_public_facing_line && !input.support_language_matches_line_proof;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5ProtectionPlanEntryDegradeReason::DescriptorTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ProtectionPlanEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.protected_journey.is_classified() {
        Some(M5ProtectionPlanEntryDegradeReason::CohortJourneyUnclassified)
    } else if !input.bound_to_registry {
        Some(M5ProtectionPlanEntryDegradeReason::DescriptorNotBoundToRegistry)
    } else if !object_complete {
        Some(M5ProtectionPlanEntryDegradeReason::CohortDescriptorObjectIncomplete)
    } else if !preserve_ok {
        Some(M5ProtectionPlanEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof)
    } else if !all_forms {
        Some(M5ProtectionPlanEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if support_undisclosed {
        Some(M5ProtectionPlanEntryDegradeReason::RollbackOrDiagnosticsNotPreservedForPublicCohort)
    } else if !input.proof_fresh {
        Some(M5ProtectionPlanEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5StableLinePlanNextAction::ExpandCohortMeaning,
    };

    Ok(M5ResolvedProtectionPlanEntry {
        entry_id: input.entry_id,
        line_binding_id: input.line_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_correction_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        protected_journey: input.protected_journey.as_str().to_owned(),
        protected_journey_is_classified: input.protected_journey.is_classified(),
        canonical_protected_journey_mode: input
            .protected_journey
            .canonical_protected_journey_mode()
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
        protection_plan_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        rollback_and_diagnostics_bounded: input.rollback_and_diagnostics_bounded,
        is_public_facing_line: input.is_public_facing_line,
        support_language_matches_line_proof: input.support_language_matches_line_proof,
        degrade_reason,
        next_action,
        protection_plan_resolves_across_lines: degrade_reason.is_none(),
    })
}

/// Resolves a line-correction-packet entry so its correction stays safe: the entry names its canonical token,
/// semantic role, and correction scope, covers all three resolution forms, provides the complete line-identity /
/// known-limits-ledger / rollback-target / rehearsal-currency / readiness-signoff / support-language /
/// last-widening-revision line-correction object, and degrades honestly when the correction would run partner /
/// public support language ahead of line proof, hide the line correction, or let a known-limits gap masquerade
/// as covered.
pub fn resolve_correction_queue_entry(
    input: M5CorrectionQueueEntryResolutionInput,
) -> Result<M5ResolvedCorrectionQueueEntry, M5StableLinePlanResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5StableLinePlanResolutionError::EmptyCohortEvidencePacketEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.correction_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_line_identity)
        || string_is_forbidden(&input.known_limits_ledger)
        || string_is_forbidden(&input.rollback_target_reference)
        || string_is_forbidden(&input.rehearsal_currency_state)
        || string_is_forbidden(&input.readiness_signoff_state)
        || string_is_forbidden(&input.support_language_reference)
        || string_is_forbidden(&input.last_widening_revision)
    {
        return Err(M5StableLinePlanResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = correction_queue_stays_honest(
        input.correction_scope,
        input.correction_is_truthful,
        input.keeps_correction_queue_visible,
        input.support_language_present,
        input.support_language_bound_to_proof,
        input.known_limits_gap_present,
        input.known_limits_gap_flagged,
    );
    let provides_record = input.correction_scope.is_classified()
        && !input.resolved_line_identity.trim().is_empty()
        && !input.known_limits_ledger.trim().is_empty()
        && !input.rollback_target_reference.trim().is_empty()
        && !input.rehearsal_currency_state.trim().is_empty()
        && !input.readiness_signoff_state.trim().is_empty()
        && !input.support_language_reference.trim().is_empty()
        && !input.last_widening_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5CorrectionQueueEntryDegradeReason::EvidenceTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5CorrectionQueueEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.correction_scope.is_classified() {
        Some(M5CorrectionQueueEntryDegradeReason::EvidenceScopeUnclassified)
    } else if !provides_record {
        Some(M5CorrectionQueueEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    } else if !all_forms {
        Some(M5CorrectionQueueEntryDegradeReason::EvidenceFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5CorrectionQueueEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5StableLinePlanNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedCorrectionQueueEntry {
        entry_id: input.entry_id,
        correction_ref: input.correction_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_correction_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        correction_scope: input.correction_scope.as_str().to_owned(),
        correction_scope_is_classified: input.correction_scope.is_classified(),
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
        keeps_correction_queue_visible: input.keeps_correction_queue_visible,
        correction_is_truthful: input.correction_is_truthful,
        support_language_present: input.support_language_present,
        support_language_bound_to_proof: input.support_language_bound_to_proof,
        known_limits_gap_present: input.known_limits_gap_present,
        known_limits_gap_flagged: input.known_limits_gap_flagged,
        correction_queue_stays_honest: record_stays_honest,
        provides_complete_correction_queue: provides_record,
        degrade_reason,
        next_action,
        correction_safe_on_every_line: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved line-protection_plan and line-correction-packet
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableLineProtectionPlanCorrectionQueueRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5StableLineProtectionPlanCorrectionQueueRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5StableLineProtectionQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Widening stages this row keeps the same truth across.
    pub widening_stages: Vec<M5StableLineProtectionWideningStage>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5StableLineProtectionRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5StableLineProtectionAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5StableLinePlanAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5StableLinePlanExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5StableLineProtectionDowngradeTrigger>,
    /// Resolved line-protection_plan-registry examples.
    pub protection_plan_entries: Vec<M5ResolvedProtectionPlanEntry>,
    /// Resolved line-correction-packet examples.
    pub correction_queue_entries: Vec<M5ResolvedCorrectionQueueEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the line-protection_plan and
    /// line-correction-packet domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never widens a line without current rollback and diagnostics correction. MUST be
    /// `false`.
    pub widens_a_line_without_current_rollback_and_diagnostics_correction: bool,
    /// Hard invariant: this row never runs partner or public support language ahead of line proof. MUST be
    /// `false`.
    pub runs_partner_or_public_support_language_ahead_of_line_proof: bool,
    /// Hard invariant: this row never hides the rollback target or diagnostics posture before widening. MUST be
    /// `false`.
    pub hides_the_rollback_target_or_diagnostics_posture_before_widening: bool,
    /// Hard invariant: this row never collapses distinct line correction classes into one lane. MUST be `false`.
    pub collapses_distinct_correction_queue_classes_into_one_lane: bool,
}

impl M5StableLineProtectionPlanCorrectionQueueRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5StableLinePlanAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5StableLinePlanAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5StableLinePlanExportField> =
            self.export_fields.iter().copied().collect();
        M5StableLinePlanExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.widens_a_line_without_current_rollback_and_diagnostics_correction
            && !self.runs_partner_or_public_support_language_ahead_of_line_proof
            && !self.hides_the_rollback_target_or_diagnostics_posture_before_widening
            && !self.collapses_distinct_correction_queue_classes_into_one_lane
    }

    /// True when a clean line-protection_plan entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified line journey, publishes a complete protection_plan object, preserves its rollback and
    /// diagnostics posture, covers all three resolution forms, and keeps a public-facing line's support
    /// language matched to proof.
    fn protection_plan_is_honest(ex: &M5ResolvedProtectionPlanEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.protected_journey_is_classified
                && ex.protection_plan_object_complete
                && ex.rollback_and_diagnostics_bounded
                && ex.covers_all_resolution_forms
                && (!ex.is_public_facing_line || ex.support_language_matches_line_proof))
    }

    /// True when a clean line-correction-packet entry preserves a safe packet: it keeps a classified correction
    /// scope, provides the complete line-correction object, stays honest, and covers all three resolution forms.
    fn correction_is_honest(ex: &M5ResolvedCorrectionQueueEntry) -> bool {
        !ex.is_clean()
            || (ex.correction_scope_is_classified
                && ex.provides_complete_correction_queue
                && ex.correction_queue_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.protection_plan_entries
            .iter()
            .all(Self::protection_plan_is_honest)
            && self
                .correction_queue_entries
                .iter()
                .all(Self::correction_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableLineProtectionPlanCorrectionQueueRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Cohort-journey tokens (minted by this lane).
    pub protected_journey_kinds: Vec<String>,
    /// Evidence-scope tokens (minted by this lane).
    pub correction_scopes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Cohort-protection_plan-entry degrade-reason tokens.
    pub protection_plan_degrade_reasons: Vec<String>,
    /// Cohort-correction-packet-entry degrade-reason tokens.
    pub correction_queue_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5StableLineProtectionPlanCorrectionQueueRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5StableLineProtectionRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5StableLinePlanResolutionForm::ALL, |v| v.as_str()),
            protected_journey_kinds: tokens(&M5ProtectedJourneyKind::ALL, |v| v.as_str()),
            correction_scopes: tokens(&M5CorrectionQueueScope::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5StableLinePlanSurfaceContext::ALL, |v| v.as_str()),
            protection_plan_degrade_reasons: tokens(
                &M5ProtectionPlanEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            correction_queue_degrade_reasons: tokens(
                &M5CorrectionQueueEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5StableLinePlanAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5StableLinePlanNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5StableLinePlanExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5StableLineProtectionConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5StableLineProtectionPlanCorrectionQueueRegistriesGovernanceReview {
    /// The protection_plan registry names a canonical token, semantic role, and line journey for every entry.
    pub protection_plan_registry_names_token_role_and_journey: bool,
    /// Every claimed line resolves to one typed line-protection_plan object from the shared registry, not
    /// per-entry reconstruction.
    pub line_resolves_to_typed_protection_plan_from_shared_registry: bool,
    /// The exact repo / journey rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles are published for every resolved protection_plan.
    pub repo_bundle_toolchain_and_deployment_rows_published: bool,
    /// Cohorts cannot widen without preserving rollback and diagnostics posture before widening.
    pub lines_cannot_widen_without_rollback_and_diagnostics: bool,
    /// The line correction keeps the line proof visible and binds partner / public support language to it.
    pub correction_queue_keeps_proof_visible_and_binds_support_language: bool,
    /// Partner / public support language stays matched to line proof for every public-facing line.
    pub support_language_matched_to_line_proof_for_public_lines: bool,
    /// Every line-protection_plan and line-correction-packet entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Cohort-protection_plan and line-correction-packet behavior stay bound to the shared registries rather than
    /// hand-copied per line.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shiproom, release center, executive steering, and program governance read a single line source.
    pub shiproom_release_center_executive_steering_and_program_governance_read_single_source: bool,
    /// A widen-without-rollback attempt, an incomplete object, or hidden line correction is caught by fixtures
    /// before release correction turns green.
    pub protection_plan_or_correction_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableLineProtectionPlanCorrectionQueueRegistriesConsumerProjection {
    /// Shiproom and release center consume the shared line-protection_plan registry.
    pub shiproom_and_release_center_consume_shared_registries: bool,
    /// Executive steering and program governance consume the shared line-correction registry.
    pub executive_steering_and_program_governance_consume_shared_registries: bool,
    /// Diagnostics and public proof consume the shared registries.
    pub diagnostics_and_public_proof_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical line-protection_plan and line-correction-packet domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical line-protection_plan / line-correction-packet registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableLineProtectionPlanCorrectionQueueRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableLineProtectionPlanCorrectionQueueRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting line audit for the lane.
    pub line_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5StableLineProtectionPlanCorrectionQueueRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5StableLineProtectionPlanCorrectionQueueRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5StableLineProtectionPlanCorrectionQueueRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5StableLineProtectionPlanCorrectionQueueRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5StableLineProtectionPlanCorrectionQueueRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5StableLineProtectionPlanCorrectionQueueRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5StableLineProtectionPlanCorrectionQueueRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5StableLineProtectionPlanCorrectionQueueRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 line-protection_plan and line-correction-packet registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableLineProtectionPlanCorrectionQueueRegistriesPacket {
    /// Record kind; must equal [`M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5StableLineProtectionPlanCorrectionQueueRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5StableLineProtectionPlanCorrectionQueueRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5StableLineProtectionPlanCorrectionQueueRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5StableLineProtectionPlanCorrectionQueueRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5StableLineProtectionPlanCorrectionQueueRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5StableLineProtectionPlanCorrectionQueueRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5StableLineProtectionPlanCorrectionQueueRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5StableLineProtectionPlanCorrectionQueueRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_RECORD_KIND
                .to_owned(),
            schema_version:
                M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5StableLineProtectionPlanCorrectionQueueRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind
            != M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_RECORD_KIND
        {
            violations.push(
                M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::WrongRecordKind,
            );
        }
        if self.schema_version
            != M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(
                M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(
                M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::MissingIdentity,
            );
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::VocabularySetDrift,
            );
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(&serde_json::to_value(self).expect(
            "m5 line-protection_plan / line-correction-packet registries packet serializes",
        )) {
            violations.push(
                M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::RawMaterialInExport,
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
            .expect("m5 line-protection_plan / line-correction-packet registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,protection_plan_entries,correction_queue_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .protection_plan_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.correction_queue_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.protection_plan_entries.len(),
                row.correction_queue_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Stable-Line Protection-Plan and Correction-Lane Queue Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Protected journeys: {}\n",
            self.vocabulary_set.protected_journey_kinds.join(", ")
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
                "  - Protection-plan entries: {} / correction-queue entries: {}\n",
                row.protection_plan_entries.len(),
                row.correction_queue_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry line reference table generated from the registry, so docs and shiproom runbooks
    /// render the same journey-mode / repo-rows / bundle-ids / install-topology / toolchain-envelope /
    /// rollback-target truth the resolvers produced rather than a hand-copied line table. Only clean,
    /// registry-bound line-protection_plan entries are listed.
    pub fn render_protection_plan_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| line_binding_id | journey_mode | exact_repo_journey_rows | bundle_ids | install_topology | toolchain_envelope | rollback_target |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.protection_plan_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.line_binding_id,
                    ex.canonical_protected_journey_mode,
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
pub enum M5StableLineProtectionPlanCorrectionQueueRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5StableLineProtectionPlanCorrectionQueueRegistriesViolation>),
}

impl fmt::Display for M5StableLineProtectionPlanCorrectionQueueRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 line-protection_plan / line-correction-packet registries export parse failed: {error}"
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
                    "m5 line-protection_plan / line-correction-packet registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5StableLineProtectionPlanCorrectionQueueRegistriesArtifactError {}

/// Validation failures emitted by [`M5StableLineProtectionPlanCorrectionQueueRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5StableLineProtectionPlanCorrectionQueueRegistriesViolation {
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
    /// A registry row does not point at both the line-protection_plan and line-correction-packet domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, widen-without-rollback, field-incomplete,
    /// form-incomplete, or a line-correction entry missing the complete correction object).
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
    /// Cohort-protection_plan-resolution is not proven: clean protection_plan entries do not cover the canonical line
    /// journeys or the first release-center / shiproom / executive-steering / program-governance / support
    /// surfaces, no object-incomplete example degrades, or a clean protection_plan entry published an incomplete
    /// object.
    CohortDescriptorResolutionNotProven,
    /// Rollback-and-diagnostics-preservation is not proven: no widen-without-rollback example and no unbound
    /// example degrade, no clean bounded protection_plan entry is present, or a clean protection_plan entry is unbounded
    /// or unbound.
    RollbackAndDiagnosticsPreservationNotProven,
    /// Cohort-correction-integrity is not proven: clean correction entries do not cover the canonical dogfood-ring /
    /// rehearsal-currency / go-no-go-signoff scopes with full resolution-form coverage while providing the
    /// complete correction object, no support-ahead or form-incomplete example degrades, or a clean correction entry
    /// is missing the complete correction object.
    CohortEvidenceIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5StableLineProtectionPlanCorrectionQueueRegistriesViolation {
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
            Self::CohortDescriptorResolutionNotProven => "protection_plan_resolution_not_proven",
            Self::RollbackAndDiagnosticsPreservationNotProven => {
                "rollback_and_diagnostics_preservation_not_proven"
            }
            Self::CohortEvidenceIntegrityNotProven => "correction_queue_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_stable_line_protection_plan_and_correction_queue_registries_export(
) -> Result<
    M5StableLineProtectionPlanCorrectionQueueRegistriesPacket,
    M5StableLineProtectionPlanCorrectionQueueRegistriesArtifactError,
> {
    let packet: M5StableLineProtectionPlanCorrectionQueueRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-stable-line-protection-plan-and-correction-queue-registries-proof/support_export.json"
        )
    ))
    .map_err(M5StableLineProtectionPlanCorrectionQueueRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(
            M5StableLineProtectionPlanCorrectionQueueRegistriesArtifactError::Validation(
                violations,
            ),
        )
    }
}

fn validate_source_contracts(
    packet: &M5StableLineProtectionPlanCorrectionQueueRegistriesPacket,
    violations: &mut Vec<M5StableLineProtectionPlanCorrectionQueueRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_SCHEMA_REF,
        M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_DOC_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_DOC_REF,
        M5_STABLE_LINE_PROTECTION_PLAN_DOMAIN_SCHEMA_REF,
        M5_CORRECTION_LANE_QUEUE_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5StableLineProtectionPlanCorrectionQueueRegistriesPacket,
    violations: &mut Vec<M5StableLineProtectionPlanCorrectionQueueRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations
            .push(M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::NoRegistryRows);
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
            violations.push(
                M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations
                .push(M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_STABLE_LINE_PROTECTION_PLAN_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_CORRECTION_LANE_QUEUE_DOMAIN_SCHEMA_REF)
        {
            violations
                .push(M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.protection_plan_entries.is_empty() || row.correction_queue_entries.is_empty() {
            violations.push(
                M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::ExamplesMissing,
            );
        }
        if !row.examples_are_honest() {
            violations.push(
                M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::DishonestExample,
            );
        }
        if !row.honours_invariants() {
            violations.push(
                M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5StableLineProtectionPlanCorrectionQueueRegistriesPacket,
    violations: &mut Vec<M5StableLineProtectionPlanCorrectionQueueRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.protection_plan_registry_names_token_role_and_journey,
        review.line_resolves_to_typed_protection_plan_from_shared_registry,
        review.repo_bundle_toolchain_and_deployment_rows_published,
        review.lines_cannot_widen_without_rollback_and_diagnostics,
        review.correction_queue_keeps_proof_visible_and_binds_support_language,
        review.support_language_matched_to_line_proof_for_public_lines,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shiproom_release_center_executive_steering_and_program_governance_read_single_source,
        review.protection_plan_or_correction_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5StableLineProtectionPlanCorrectionQueueRegistriesPacket,
    violations: &mut Vec<M5StableLineProtectionPlanCorrectionQueueRegistriesViolation>,
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
                M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5StableLineProtectionPlanCorrectionQueueRegistriesPacket,
    violations: &mut Vec<M5StableLineProtectionPlanCorrectionQueueRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5StableLineProtectionPlanCorrectionQueueRegistriesPacket,
    violations: &mut Vec<M5StableLineProtectionPlanCorrectionQueueRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.line_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5StableLineProtectionPlanCorrectionQueueRegistriesPacket,
    violations: &mut Vec<M5StableLineProtectionPlanCorrectionQueueRegistriesViolation>,
) {
    let protection_plans = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.protection_plan_entries.iter())
    };
    let correction = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.correction_queue_entries.iter())
    };

    // AC1: every active line can be inspected by exact rows, bundles, toolchains, and deployment profiles.
    // Clean protection_plan entries cover the canonical line journeys and the first release-center / shiproom /
    // executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no
    // clean protection_plan entry published an incomplete object.
    let clean_journeys: BTreeSet<String> = protection_plans()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.protected_journey.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = protection_plans()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let journeys_covered = M5ProtectedJourneyKind::CANONICAL_JOURNEYS
        .iter()
        .all(|k| clean_journeys.contains(k.as_str()));
    let first_surfaces_covered = M5StableLinePlanSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = protection_plans().any(|ex| {
        ex.degrade_reason
            == Some(M5ProtectionPlanEntryDegradeReason::CohortDescriptorObjectIncomplete)
    });
    let no_clean_incomplete =
        !protection_plans().any(|ex| ex.is_clean() && !ex.protection_plan_object_complete);
    if !(journeys_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::CohortDescriptorResolutionNotProven,
        );
    }

    // AC2: line packets preserve rollback and diagnostics posture before widening. A widen-without-rollback
    // example degrades, an unbound example degrades, at least one clean bounded protection_plan entry is present, and
    // no clean protection_plan entry is unbounded or unbound.
    let widen_fold_degrades = protection_plans().any(|ex| {
        ex.degrade_reason
            == Some(
                M5ProtectionPlanEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
            )
    });
    let unbound_degrades = protection_plans().any(|ex| {
        ex.degrade_reason == Some(M5ProtectionPlanEntryDegradeReason::DescriptorNotBoundToRegistry)
    });
    let bounded_clean_protection_plan =
        protection_plans().any(|ex| ex.is_clean() && ex.rollback_and_diagnostics_bounded);
    let no_clean_unbound = !protection_plans().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded =
        !protection_plans().any(|ex| ex.is_clean() && !ex.rollback_and_diagnostics_bounded);
    if !(widen_fold_degrades
        && unbound_degrades
        && bounded_clean_protection_plan
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations.push(
            M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::RollbackAndDiagnosticsPreservationNotProven,
        );
    }

    // AC3: claim publication can prove which line correction backs each launch-bearing lane. Clean correction
    // entries cover every canonical dogfood-ring / rehearsal-currency / go-no-go-signoff scope with full
    // resolution-form coverage while providing the complete correction object, a support-ahead example degrades, a
    // form-incomplete example degrades, and no clean correction entry is missing the complete object.
    let clean_correction_scopes: BTreeSet<String> = correction()
        .filter(|ex| {
            ex.is_clean()
                && ex.correction_scope_is_classified
                && ex.provides_complete_correction_queue
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.correction_scope.clone())
        .collect();
    let correction_scopes_covered = M5CorrectionQueueScope::CANONICAL_SCOPES
        .iter()
        .all(|m| clean_correction_scopes.contains(m.as_str()));
    let support_ahead_degrades = correction().any(|ex| {
        ex.degrade_reason
            == Some(
                M5CorrectionQueueEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
            )
    });
    let form_incomplete_degrades = correction().any(|ex| {
        ex.degrade_reason
            == Some(M5CorrectionQueueEntryDegradeReason::EvidenceFormCoverageIncomplete)
    });
    let no_clean_missing_correction =
        !correction().any(|ex| ex.is_clean() && !ex.provides_complete_correction_queue);
    if !(correction_scopes_covered
        && support_ahead_degrades
        && form_incomplete_degrades
        && no_clean_missing_correction)
    {
        violations.push(
            M5StableLineProtectionPlanCorrectionQueueRegistriesViolation::CohortEvidenceIntegrityNotProven,
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

/// The launch-bearing lines this lane implements, for downstream reference: the line-protection_plan registry
/// covers the core-team canary, design-partner preview, extension-author, public preview, and certified-journey
/// lines the frozen matrix froze, and the line-correction-packet registry binds the correction that backs each.
pub const IMPLEMENTED_LINES: [M5StableLineProtectionLine; 5] = M5StableLineProtectionLine::ALL;
