//! Implemented M5 freeze-exception and go-no-go registries.
//!
//! The frozen [launch-control matrix][matrix] names Aureline's governed launch-bearing cohorts and locks their
//! controlled vocabulary. This module governs *ring widening by explicit stop conditions rather than schedule
//! optimism*: it turns the *freeze-exception* grammar (how each regression asset type — canary, broad internal
//! dogfood, design-partner preview, public preview, and certified stable — declares its minimum entry evidence,
//! soak-window expectation, why widening is allowed, its known-limits packet, issue-template linkage,
//! claim-narrowing action, and the go-no-go reference that immediately stops it) and the *go-no-go*
//! grammar (how a launch-bearing lane records the go-no-go condition — a crash / data-loss / trust defect,
//! a repeated protected-metric regression, or a stale readiness packet — that halts regression asset while it is
//! active) into registry resolvers that produce export-safe, honest projections. Every claimed M5 ring
//! transition then resolves to one typed freeze-exception object — the regression asset type it classifies, the
//! minimum entry evidence, the soak-window expectation, the widening-allow rationale, the known-limits packet,
//! the issue-template ref, the claim-narrowing action, and the go-no-go reference, all visible before
//! widening so a ring never advances without its known-limits and go-no-go posture and so partner / public
//! support language never outruns current ring proof — and to one go-no-go object — the resolved transition
//! identity, the active stop-condition ledger, the go-no-go target reference, the protected-metric
//! regression state, the packet-freshness state, the crash / data-loss / trust reference, and the last
//! ring-transition revision — that the shiproom, release-center, executive-steering, program-governance, and
//! support / export surfaces can inspect without manual reconstruction, so every ring transition can state why
//! widening is allowed and what immediately stops it, known-limits and rollback posture stay visible before any
//! ring widens, regression asset can never advance on a claimed lane while a go-no-go condition is active,
//! and a ring that cannot explain the progression rule it declared or the stop condition that backs it degrades
//! honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one typed freeze-exception object per regression asset type.** [`resolve_freeze_exception_entry`]
//!   refuses to read as a clean, registry-bound progression entry unless it names a canonical registry token, a
//!   classified [regression asset type][M5FreezeExceptionChangeClass], a launch-control role, covers every
//!   [resolution form][M5FreezeExceptionResolutionForm] (the canonical object, the accessible summary, and the audit
//!   record), publishes every progression field (minimum entry evidence, soak-window expectation, widening-allow
//!   rationale, issue-template ref, known limits, claim-narrowing action, and go-no-go reference), keeps its
//!   known-limits and go-no-go posture visible before widening, and keeps partner / public support language
//!   matched to ring proof; otherwise it degrades.
//! * **Keep a ring from advancing without a visible go-no-go and known-limits posture.**
//!   [`freeze_exception_stays_documented_before_widening`] rejects a progression entry whose go-no-go and
//!   known-limits posture is not visible (a ring advancing without a go-no-go reference and known limits) so
//!   it degrades to
//!   [`M5FreezeExceptionEntryDegradeReason::FreezeExceptionWidensScopeUndocumentedOrRunsClaimAheadOfProof`],
//!   and a public-facing ring whose support language runs ahead of ring proof degrades the same way — the
//!   structured blocker reason a widen-without-stop attempt must surface.
//! * **Keep the go-no-go record from advancing a ring while a stop condition is active.**
//!   [`resolve_go_no_go_entry`] names a classified [go-no-go condition][M5GoNoGoDecisionKind],
//!   requires the full transition-identity / active-stop-condition-ledger / go-no-go-target /
//!   protected-metric-regression / packet-freshness / crash-data-loss-or-trust / last-ring-transition-revision
//!   record, covers every resolution form, and degrades to
//!   [`M5GoNoGoEntryDegradeReason::GoNoGoDropsEvidenceOrImpliesGreenWhileStale`]
//!   when the record would advance a ring while a stop condition is active, hide the go-no-go, or let a
//!   protected-metric regression masquerade as covered, so a go-no-go record can never read as trustworthy
//!   when it has quietly dropped the reason a lane's progression is actually gated.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5LaunchControlRole`] role vocabulary and
//! the [`M5LaunchControlConsumerSurface`] consumer-surface taxonomy — so the shiproom, release-center,
//! executive-steering, program-governance, diagnostics, docs, CLI, support, and public-proof surfaces can never
//! fork their own ring-control meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_launch_control_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_freeze_exception_and_go_no_go_registries,
    seeded_m5_freeze_exception_and_go_no_go_registries_freeze_exception_beta_narrowed,
    seeded_m5_freeze_exception_and_go_no_go_registries_go_no_go_preview_narrowed,
    M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_launch_control_matrix::{
    M5LaunchControlAccessibilityRoute, M5LaunchControlConsumerSurface,
    M5LaunchControlDowngradeTrigger, M5LaunchControlQualificationClass,
    M5LaunchControlRequiredLabel, M5LaunchControlRole, M5LaunchControlWideningStage,
    M5_LAUNCH_CONTROL_MATRIX_DOC_REF, M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5FreezeExceptionGoNoGoRegistriesPacket`].
pub const M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_freeze_exception_and_go_no_go_registries";

/// Schema version for M5 freeze-exception / go-no-go registry records.
pub const M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_SCHEMA_REF: &str =
    "schemas/program/m5-freeze-exception-and-go-no-go-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_DOC_REF: &str =
    "docs/release/m5_freeze_exception_and_go_no_go_registries.md";

/// Repo-relative path of the canonical freeze-exception domain schema minted by this lane (how a widening ring
/// transition declares its minimum entry evidence, soak-window expectation, why widening is allowed, its
/// known-limits packet, issue-template linkage, claim-narrowing action, and the go-no-go reference that
/// immediately stops it).
pub const M5_FREEZE_EXCEPTION_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-freeze-exception-packet.schema.json";

/// Repo-relative path of the canonical go-no-go domain schema minted by this lane (how a launch-bearing lane
/// records the go-no-go condition — a crash / data-loss / trust defect, a repeated protected-metric
/// regression, or a stale readiness packet — that halts regression asset while it is active).
pub const M5_GO_NO_GO_DOMAIN_SCHEMA_REF: &str = "schemas/program/m5-go-no-go-decision.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-freeze-exception-and-go-no-go-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-freeze-exception-and-go-no-go-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-freeze-exception-and-go-no-go-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/release/m5-freeze-exception-and-go-no-go-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// cohort invents a parallel surface set.
pub type M5FreezeExceptionGoNoGoRegistriesConsumerSurface = M5LaunchControlConsumerSurface;

/// One of the three resolution forms every freeze-exception or go-no-go entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// freeze-exception and go-no-go *domains* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FreezeExceptionResolutionForm {
    /// The canonical resolved freeze-exception / go-no-go object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved cohort discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved cohort inspectable off-renderer.
    AuditRecord,
}

impl M5FreezeExceptionResolutionForm {
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

/// Controlled cohort archetype a freeze-exception entry classifies, so the typed descriptor model shares one
/// registry rather than a hand-copied per-cohort assumption. Minted by this lane because the frozen matrix
/// carries the launch-bearing cohorts but distinguishes the dogfood / migration-alpha / extension-author /
/// design-partner / public-preview / certified-archetype archetypes an auditable descriptor classifies against
/// explicitly. Every classified archetype carries its canonical mode, and the design-partner-preview and
/// public-preview archetypes are public-facing so their partner / public support language must stay matched to
/// cohort proof before the cohort widens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FreezeExceptionChangeClass {
    /// The internal dogfood core-team canary cohort.
    PhaseAllowedChange,
    /// The migration alpha cohort (external alpha migrating from a prior toolchain).
    ExceptionRequiredChange,
    /// The extension-author cohort (compatibility rehearsals current, freeze exceptions documented).
    ApiOrContractChange,
    /// The design-partner preview cohort (public-facing; support language must match cohort proof).
    ScopeWideningChange,
    /// The public preview cohort (public-facing; support language must match cohort proof).
    MigrationOrDataChange,
    /// The certified-archetype cohort (ORR signed and a go/no-go decision recorded).
    DependencyOrToolchainChange,
    /// The cohort archetype is unclassified, which is disallowed.
    ChangeClassUnclassified,
}

impl M5FreezeExceptionChangeClass {
    /// Every cohort archetype, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::PhaseAllowedChange,
        Self::ExceptionRequiredChange,
        Self::ApiOrContractChange,
        Self::ScopeWideningChange,
        Self::MigrationOrDataChange,
        Self::DependencyOrToolchainChange,
        Self::ChangeClassUnclassified,
    ];

    /// The six canonical cohort archetypes every claimed M5 launch-bearing cohort classifies against.
    pub const CANONICAL_CHANGE_CLASSES: [Self; 6] = [
        Self::PhaseAllowedChange,
        Self::ExceptionRequiredChange,
        Self::ApiOrContractChange,
        Self::ScopeWideningChange,
        Self::MigrationOrDataChange,
        Self::DependencyOrToolchainChange,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhaseAllowedChange => "phase_allowed_change",
            Self::ExceptionRequiredChange => "exception_required_change",
            Self::ApiOrContractChange => "api_or_contract_change",
            Self::ScopeWideningChange => "scope_widening_change",
            Self::MigrationOrDataChange => "migration_or_data_change",
            Self::DependencyOrToolchainChange => "dependency_or_toolchain_change",
            Self::ChangeClassUnclassified => "change_class_unclassified",
        }
    }

    /// Whether the archetype is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ChangeClassUnclassified)
    }

    /// The canonical mode for this cohort archetype.
    pub const fn canonical_freeze_exception_change_class_mode(self) -> &'static str {
        match self {
            Self::PhaseAllowedChange => "phase_allowed_change_class",
            Self::ExceptionRequiredChange => "exception_required_change_class",
            Self::ApiOrContractChange => "api_or_contract_change_class",
            Self::ScopeWideningChange => "scope_widening_change_class",
            Self::MigrationOrDataChange => "migration_or_data_change_class",
            Self::DependencyOrToolchainChange => "dependency_or_toolchain_change_class",
            Self::ChangeClassUnclassified => "",
        }
    }

    /// Whether this archetype is public-facing and so must keep partner / public support language matched to
    /// cohort proof before the cohort widens.
    pub const fn requires_documented_exception(self) -> bool {
        matches!(
            self,
            Self::ScopeWideningChange | Self::MigrationOrDataChange
        )
    }
}

/// Controlled evidence scope a go-no-go entry must resolve its cohort proof from, so an evidence
/// packet shares one registry rather than a hand-copied per-record assumption. Minted by this lane, tracking
/// whether the evidence came from dogfood-ring telemetry, current rehearsal cadence, or an explicit go/no-go
/// signoff the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GoNoGoDecisionKind {
    /// The evidence came from internal dogfood-ring telemetry.
    GoDecision,
    /// The evidence came from current rehearsal cadence (publish/rollback, mixed-version, handoff drills).
    NoGoDecision,
    /// The evidence came from an explicit go/no-go signoff with a preserved evidence snapshot.
    ConditionalGoDecision,
    /// The evidence scope is unclassified, which is disallowed.
    DecisionUnclassified,
}

impl M5GoNoGoDecisionKind {
    /// Every evidence scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::GoDecision,
        Self::NoGoDecision,
        Self::ConditionalGoDecision,
        Self::DecisionUnclassified,
    ];

    /// The three canonical evidence scopes every go-no-go packet must stay distinct across.
    pub const CANONICAL_DECISIONS: [Self; 3] = [
        Self::GoDecision,
        Self::NoGoDecision,
        Self::ConditionalGoDecision,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GoDecision => "go_decision",
            Self::NoGoDecision => "no_go_decision",
            Self::ConditionalGoDecision => "conditional_go_decision",
            Self::DecisionUnclassified => "decision_unclassified",
        }
    }

    /// Whether the evidence scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::DecisionUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a freeze-exception or
/// go-no-go token's meaning stays stable whether it appears in the release-center, shiproom,
/// executive-steering, program-governance, or a support / export form. Minted by this lane, tracking the
/// first-consumer surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FreezeExceptionSurfaceContext {
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

impl M5FreezeExceptionSurfaceContext {
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

/// One mandatory rendered part a freeze-exception or go-no-go entry must be able to show, so no
/// cohort archetype, repo / bundle / toolchain / deployment row, known-limits packet, rollback target,
/// go-no-go field, or registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FreezeExceptionAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The cohort archetype the entry classifies (freeze-exception entry).
    FreezeExceptionType,
    /// The exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles the entry publishes (freeze-exception entry).
    IncidentLineageRows,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The known-limits packet and rollback target the entry preserves before widening (freeze-exception
    /// entry).
    BuildAndCohortLineage,
    /// The go-no-go fields (cohort identity, known-limits ledger, rollback target, rehearsal currency,
    /// readiness signoff, support language) the entry publishes (go-no-go entry).
    GoNoGoFields,
    /// The support-identity hint the entry publishes (go-no-go entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved cohort descriptor or cohort evidence (both entries).
    PlainLanguageMeaning,
}

impl M5FreezeExceptionAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::FreezeExceptionType,
        Self::IncidentLineageRows,
        Self::ResolutionFormCoverage,
        Self::BuildAndCohortLineage,
        Self::GoNoGoFields,
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
            Self::FreezeExceptionType => "freeze_exception_change_class",
            Self::IncidentLineageRows => "incident_lineage_rows",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::BuildAndCohortLineage => "build_and_cohort_lineage",
            Self::GoNoGoFields => "go_no_go_fields",
            Self::SupportIdentityHint => "support_identity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// cohort descriptor, a go-no-go packet, or a degraded freeze-exception / go-no-go entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FreezeExceptionNextAction {
    /// Expand the resolved cohort descriptor's or go-no-go packet's plain-language meaning.
    ExpandFreezeExceptionMeaning,
    /// Inspect the cohort archetype or evidence scope the entry resolves.
    InspectChangeClassOrDecision,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5FreezeExceptionNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandFreezeExceptionMeaning,
        Self::InspectChangeClassOrDecision,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandFreezeExceptionMeaning => "expand_freeze_exception_meaning",
            Self::InspectChangeClassOrDecision => "inspect_change_class_or_decision",
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
pub enum M5FreezeExceptionExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The cohort families covered.
    FreezeExceptionFamilies,
    /// The cohort archetypes carried.
    FreezeExceptionChangeClasses,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The evidence scopes carried.
    GoNoGoDecisions,
    /// The render / surface context.
    SurfaceContext,
    /// The cohort-archetype modes carried.
    FreezeExceptionChangeClassModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5FreezeExceptionExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::FreezeExceptionFamilies,
        Self::FreezeExceptionChangeClasses,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::GoNoGoDecisions,
        Self::SurfaceContext,
        Self::FreezeExceptionChangeClassModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::FreezeExceptionFamilies,
        Self::FreezeExceptionChangeClasses,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::FreezeExceptionFamilies => "freeze_exception_families",
            Self::FreezeExceptionChangeClasses => "freeze_exception_change_classes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::GoNoGoDecisions => "go_no_go_decisions",
            Self::SurfaceContext => "surface_context",
            Self::FreezeExceptionChangeClassModes => "freeze_exception_change_class_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a freeze-exception entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, widen-without-rollback, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FreezeExceptionEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the descriptor means.
    FreezeExceptionTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The cohort archetype is unclassified (not in the resolved taxonomy).
    FreezeExceptionChangeClassUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    FreezeExceptionNotBoundToRegistry,
    /// The resolved freeze-exception object is incomplete: the exact repo / archetype rows, bundle IDs, install
    /// topology, toolchain envelope, known limits, rollback target, or diagnostics posture is unstated.
    FreezeExceptionObjectIncomplete,
    /// The cohort's rollback and diagnostics posture is not preserved before widening (a cohort widening without
    /// a rollback target and diagnostics posture), or a public-facing cohort ran its support language ahead of
    /// cohort proof.
    FreezeExceptionWidensScopeUndocumentedOrRunsClaimAheadOfProof,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A public-facing cohort did not keep its support language matched to cohort proof before widening.
    FreezeExceptionUndocumentedForScopeWidening,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5FreezeExceptionEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::FreezeExceptionTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::FreezeExceptionChangeClassUnclassified,
        Self::FreezeExceptionNotBoundToRegistry,
        Self::FreezeExceptionObjectIncomplete,
        Self::FreezeExceptionWidensScopeUndocumentedOrRunsClaimAheadOfProof,
        Self::ResolutionFormCoverageIncomplete,
        Self::FreezeExceptionUndocumentedForScopeWidening,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreezeExceptionTokenUnstated => "freeze_exception_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::FreezeExceptionChangeClassUnclassified => {
                "freeze_exception_change_class_unclassified"
            }
            Self::FreezeExceptionNotBoundToRegistry => "freeze_exception_not_bound_to_registry",
            Self::FreezeExceptionObjectIncomplete => "freeze_exception_object_incomplete",
            Self::FreezeExceptionWidensScopeUndocumentedOrRunsClaimAheadOfProof => {
                "freeze_exception_widens_scope_undocumented_or_runs_claim_ahead_of_proof"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::FreezeExceptionUndocumentedForScopeWidening => {
                "freeze_exception_undocumented_for_scope_widening"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5FreezeExceptionNextAction {
        match self {
            Self::FreezeExceptionTokenUnstated | Self::FreezeExceptionNotBoundToRegistry => {
                M5FreezeExceptionNextAction::TraceCanonicalRegistry
            }
            Self::FreezeExceptionChangeClassUnclassified
            | Self::FreezeExceptionObjectIncomplete
            | Self::FreezeExceptionWidensScopeUndocumentedOrRunsClaimAheadOfProof => {
                M5FreezeExceptionNextAction::InspectChangeClassOrDecision
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5FreezeExceptionNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::FreezeExceptionUndocumentedForScopeWidening
            | Self::ProofStale => M5FreezeExceptionNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5LaunchControlDowngradeTrigger {
        match self {
            Self::FreezeExceptionTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::FreezeExceptionNotBoundToRegistry => {
                M5LaunchControlDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::FreezeExceptionChangeClassUnclassified
            | Self::FreezeExceptionObjectIncomplete => {
                M5LaunchControlDowngradeTrigger::CohortMembershipUnstated
            }
            Self::FreezeExceptionWidensScopeUndocumentedOrRunsClaimAheadOfProof
            | Self::FreezeExceptionUndocumentedForScopeWidening => {
                M5LaunchControlDowngradeTrigger::WidenedWithoutCurrentCohortEvidence
            }
            Self::ProofStale => M5LaunchControlDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a go-no-go entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GoNoGoEntryDegradeReason {
    /// The canonical registry token name is unstated.
    GoNoGoTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The evidence scope is unclassified (not in the resolved taxonomy).
    GoNoGoDecisionUnclassified,
    /// The cohort evidence would run partner / public support language ahead of cohort proof, hide the cohort
    /// evidence, let a known-limits gap masquerade as covered, or it dropped one of the required go-no-go
    /// fields (cohort identity, known-limits ledger, rollback target, rehearsal currency, readiness signoff,
    /// support language, last widening revision).
    GoNoGoDropsEvidenceOrImpliesGreenWhileStale,
    /// The canonical / accessible / audit resolution-form coverage of the evidence is incomplete.
    GoNoGoFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5GoNoGoEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GoNoGoTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::GoNoGoDecisionUnclassified,
        Self::GoNoGoDropsEvidenceOrImpliesGreenWhileStale,
        Self::GoNoGoFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GoNoGoTokenUnstated => "go_no_go_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::GoNoGoDecisionUnclassified => "go_no_go_decision_unclassified",
            Self::GoNoGoDropsEvidenceOrImpliesGreenWhileStale => {
                "go_no_go_drops_evidence_or_implies_green_while_stale"
            }
            Self::GoNoGoFormCoverageIncomplete => "go_no_go_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5FreezeExceptionNextAction {
        match self {
            Self::GoNoGoTokenUnstated => M5FreezeExceptionNextAction::TraceCanonicalRegistry,
            Self::GoNoGoDecisionUnclassified
            | Self::GoNoGoDropsEvidenceOrImpliesGreenWhileStale => {
                M5FreezeExceptionNextAction::InspectChangeClassOrDecision
            }
            Self::GoNoGoFormCoverageIncomplete => {
                M5FreezeExceptionNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5FreezeExceptionNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5LaunchControlDowngradeTrigger {
        match self {
            Self::GoNoGoTokenUnstated => M5LaunchControlDowngradeTrigger::RegistryReferenceUnstated,
            Self::SurfaceContextUnresolved | Self::GoNoGoDecisionUnclassified => {
                M5LaunchControlDowngradeTrigger::ReadinessStateUnstated
            }
            Self::GoNoGoDropsEvidenceOrImpliesGreenWhileStale => {
                M5LaunchControlDowngradeTrigger::RanPartnerOrPublicLanguageAheadOfCohortProof
            }
            Self::GoNoGoFormCoverageIncomplete => {
                M5LaunchControlDowngradeTrigger::ImpliedGreenWhileGoNoGoOrOrrWasStale
            }
            Self::ProofStale => M5LaunchControlDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_freeze_exception_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5FreezeExceptionEntryResolutionInput {
    /// Stable identity of the freeze-exception-registry entry.
    pub entry_id: String,
    /// The stable cohort-binding ID this descriptor binds to (e.g. `incident.lane.public-preview`); empty means
    /// unstated.
    pub exception_binding_id: String,
    /// The canonical registry token name (e.g. `freeze.exception.migration_or_data_change`); empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5LaunchControlRole,
    /// The cohort archetype this entry classifies.
    pub freeze_exception_change_class: M5FreezeExceptionChangeClass,
    /// The render / surface context.
    pub surface_context: M5FreezeExceptionSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5FreezeExceptionResolutionForm>,
    /// The published exact repo / archetype rows; empty means unstated.
    pub exception_scope_reference: String,
    /// The published bundle IDs; empty means unstated.
    pub rollback_or_narrowing_reference: String,
    /// The published install topology; empty means unstated.
    pub docs_support_migration_reference: String,
    /// The published toolchain envelope; empty means unstated.
    pub owner_capture_reference: String,
    /// The published known limits; empty means unstated.
    pub risk_capture_reference: String,
    /// The published rollback target; empty means unstated.
    pub change_budget_reference: String,
    /// The published diagnostics posture; empty means unstated.
    pub expiry_reference: String,
    /// True when the behavior traces to the freeze-exception registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the cohort's rollback and diagnostics posture is preserved before widening (a hard invariant
    /// when `false`).
    pub freeze_exception_documented_before_widening: bool,
    /// True when this cohort's archetype is public-facing.
    pub requires_documented_exception: bool,
    /// True when partner / public support language is matched to cohort proof before a public-facing cohort
    /// widens.
    pub attributable_asset_or_approved_exception: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe freeze-exception-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedFreezeExceptionEntry {
    /// Stable identity of the freeze-exception-registry entry.
    pub entry_id: String,
    /// The stable cohort-binding ID this descriptor binds to.
    pub exception_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the evidence snapshot and signoff before widening.
    pub semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: bool,
    /// The cohort-archetype token named by the entry.
    pub freeze_exception_change_class: String,
    /// Whether the cohort archetype is classified into the resolved taxonomy.
    pub freeze_exception_change_class_is_classified: bool,
    /// The canonical mode for the entry's cohort archetype.
    pub canonical_freeze_exception_change_class_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published exact repo / archetype rows.
    pub exception_scope_reference: String,
    /// The published bundle IDs.
    pub rollback_or_narrowing_reference: String,
    /// The published install topology.
    pub docs_support_migration_reference: String,
    /// The published toolchain envelope.
    pub owner_capture_reference: String,
    /// The published known limits.
    pub risk_capture_reference: String,
    /// The published rollback target.
    pub change_budget_reference: String,
    /// The published diagnostics posture.
    pub expiry_reference: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved freeze-exception object publishes every required field.
    pub freeze_exception_object_complete: bool,
    /// Whether the entry traces to the freeze-exception registry.
    pub bound_to_registry: bool,
    /// Whether the cohort's rollback and diagnostics posture stays preserved before widening.
    pub freeze_exception_documented_before_widening: bool,
    /// Whether this cohort's archetype is public-facing.
    pub requires_documented_exception: bool,
    /// Whether partner / public support language is matched to cohort proof before widening.
    pub attributable_asset_or_approved_exception: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5FreezeExceptionEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5FreezeExceptionNextAction,
    /// Whether the descriptor resolves to one typed object across every claimed cohort (clean entry naming every
    /// fact).
    pub freeze_exception_resolves_across_classes: bool,
}

impl M5ResolvedFreezeExceptionEntry {
    /// Whether this freeze-exception entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_go_no_go_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5GoNoGoEntryResolutionInput {
    /// Stable identity of the go-no-go entry.
    pub entry_id: String,
    /// The stable evidence-ref this record binds to; empty means unstated.
    pub go_no_go_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5LaunchControlRole,
    /// The evidence scope this record must resolve its cohort proof from.
    pub go_no_go_decision: M5GoNoGoDecisionKind,
    /// The render / surface context.
    pub surface_context: M5FreezeExceptionSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5FreezeExceptionResolutionForm>,
    /// The published resolved cohort identity; empty means missing.
    pub resolved_decision_identity: String,
    /// The published known-limits ledger; empty means missing.
    pub evidence_snapshot_ledger: String,
    /// The published rollback-target reference; empty means missing.
    pub orr_signoff_reference: String,
    /// The published rehearsal-currency state; empty means missing.
    pub on_call_roster_state: String,
    /// The published readiness-signoff state; empty means missing.
    pub go_no_go_freshness_state: String,
    /// The published cohort-bound support-language reference; empty means missing.
    pub widening_stage_reference: String,
    /// The published last widening revision; empty means missing.
    pub last_go_no_go_revision: String,
    /// True when the record keeps the cohort evidence visible.
    pub keeps_evidence_snapshot_visible: bool,
    /// True when the evidence is truthful (never claims a clean packet over hidden cohort evidence).
    pub go_no_go_lineage_is_truthful: bool,
    /// True when partner / public support language is present on this record.
    pub override_without_evidence_requested: bool,
    /// True when the support language is bound to cohort proof rather than running ahead of it.
    pub blocked_until_evidence_linked: bool,
    /// True when a known-limits gap is present on this record.
    pub lineage_gap_present: bool,
    /// True when a known-limits gap is flagged rather than masquerading as covered.
    pub lineage_gap_flagged: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe go-no-go projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedGoNoGoEntry {
    /// Stable identity of the go-no-go entry.
    pub entry_id: String,
    /// The stable evidence-ref this record binds to.
    pub go_no_go_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the evidence snapshot and signoff before widening.
    pub semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: bool,
    /// The evidence-scope token named by the entry.
    pub go_no_go_decision: String,
    /// Whether the evidence scope is classified into the resolved taxonomy.
    pub go_no_go_decision_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published resolved cohort identity.
    pub resolved_decision_identity: String,
    /// The published known-limits ledger.
    pub evidence_snapshot_ledger: String,
    /// The published rollback-target reference.
    pub orr_signoff_reference: String,
    /// The published rehearsal-currency state.
    pub on_call_roster_state: String,
    /// The published readiness-signoff state.
    pub go_no_go_freshness_state: String,
    /// The published cohort-bound support-language reference.
    pub widening_stage_reference: String,
    /// The published last widening revision.
    pub last_go_no_go_revision: String,
    /// Whether the record keeps the cohort evidence visible.
    pub keeps_evidence_snapshot_visible: bool,
    /// Whether the evidence is truthful.
    pub go_no_go_lineage_is_truthful: bool,
    /// Whether partner / public support language is present on this build.
    pub override_without_evidence_requested: bool,
    /// Whether the support language is bound to cohort proof rather than running ahead of it.
    pub blocked_until_evidence_linked: bool,
    /// Whether a known-limits gap is present on this record.
    pub lineage_gap_present: bool,
    /// Whether a known-limits gap is flagged rather than masquerading as covered.
    pub lineage_gap_flagged: bool,
    /// Whether the record stays honest (cohort evidence visible, support language bound to proof, known-limits
    /// gap flagged).
    pub go_no_go_stays_honest: bool,
    /// Whether the entry provides the complete go-no-go object (cohort identity, known-limits ledger,
    /// rollback target, rehearsal currency, readiness signoff, support language, last widening revision).
    pub provides_complete_go_no_go_record: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5GoNoGoEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5FreezeExceptionNextAction,
    /// Whether the cohort evidence is safe on every claimed cohort (clean entry naming every fact).
    pub go_no_go_safe_on_every_decision: bool,
}

impl M5ResolvedGoNoGoEntry {
    /// Whether this go-no-go entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5FreezeExceptionResolutionError {
    /// The freeze-exception-entry id was empty.
    EmptyFreezeExceptionEntryId,
    /// The go-no-go-entry id was empty.
    EmptyGoNoGoEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5FreezeExceptionResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyFreezeExceptionEntryId => "empty_freeze_exception_entry_id",
            Self::EmptyGoNoGoEntryId => "empty_go_no_go_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5FreezeExceptionResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 freeze-exception / go-no-go registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5FreezeExceptionResolutionError {}

fn form_tokens(forms: &[M5FreezeExceptionResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5FreezeExceptionResolutionForm]) -> bool {
    let present: BTreeSet<M5FreezeExceptionResolutionForm> = forms.iter().copied().collect();
    M5FreezeExceptionResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved freeze-exception object publishes every required field: classified cohort archetype,
/// exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, known limits, rollback target,
/// and diagnostics posture. An unclassified archetype or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn freeze_exception_object_is_complete(
    archetype: M5FreezeExceptionChangeClass,
    exception_scope_reference: &str,
    rollback_or_narrowing_reference: &str,
    docs_support_migration_reference: &str,
    owner_capture_reference: &str,
    risk_capture_reference: &str,
    change_budget_reference: &str,
    expiry_reference: &str,
) -> bool {
    archetype.is_classified()
        && !exception_scope_reference.trim().is_empty()
        && !rollback_or_narrowing_reference.trim().is_empty()
        && !docs_support_migration_reference.trim().is_empty()
        && !owner_capture_reference.trim().is_empty()
        && !risk_capture_reference.trim().is_empty()
        && !change_budget_reference.trim().is_empty()
        && !expiry_reference.trim().is_empty()
}

/// Whether the cohort descriptor keeps a cohort from widening without preserving its rollback and diagnostics
/// posture: the archetype must be classified, the rollback and diagnostics posture must be preserved before
/// widening, and a public-facing cohort must keep its support language matched to cohort proof. An unclassified
/// archetype, an unpreserved rollback / diagnostics posture, or partner / public support language running ahead
/// of proof never matches.
pub fn freeze_exception_stays_documented_before_widening(
    archetype: M5FreezeExceptionChangeClass,
    freeze_exception_documented_before_widening: bool,
    requires_documented_exception: bool,
    attributable_asset_or_approved_exception: bool,
) -> bool {
    archetype.is_classified()
        && freeze_exception_documented_before_widening
        && (!requires_documented_exception || attributable_asset_or_approved_exception)
}

/// Whether a go-no-go packet stays honest: the scope must be classified, the evidence must be truthful,
/// it must keep the cohort evidence visible, any partner / public support language must be bound to cohort proof
/// rather than running ahead of it, and any known-limits gap must be flagged rather than masquerade as covered.
pub fn go_no_go_stays_honest(
    scope: M5GoNoGoDecisionKind,
    go_no_go_lineage_is_truthful: bool,
    keeps_evidence_snapshot_visible: bool,
    override_without_evidence_requested: bool,
    blocked_until_evidence_linked: bool,
    lineage_gap_present: bool,
    lineage_gap_flagged: bool,
) -> bool {
    scope.is_classified()
        && go_no_go_lineage_is_truthful
        && keeps_evidence_snapshot_visible
        && (!override_without_evidence_requested || blocked_until_evidence_linked)
        && (!lineage_gap_present || lineage_gap_flagged)
}

/// Resolves a freeze-exception-registry entry so it stays bound to the freeze-exception registry: the entry
/// names its canonical token, semantic role, and cohort archetype, covers all three resolution forms, publishes
/// a complete descriptor object (exact repo / archetype rows, bundle IDs, install topology, toolchain envelope,
/// known limits, rollback target, diagnostics posture), preserves its rollback and diagnostics posture before
/// widening so a cohort never widens without it, and keeps a public-facing cohort's support language matched to
/// cohort proof.
pub fn resolve_freeze_exception_entry(
    input: M5FreezeExceptionEntryResolutionInput,
) -> Result<M5ResolvedFreezeExceptionEntry, M5FreezeExceptionResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5FreezeExceptionResolutionError::EmptyFreezeExceptionEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.exception_binding_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.exception_scope_reference)
        || string_is_forbidden(&input.rollback_or_narrowing_reference)
        || string_is_forbidden(&input.docs_support_migration_reference)
        || string_is_forbidden(&input.owner_capture_reference)
        || string_is_forbidden(&input.risk_capture_reference)
        || string_is_forbidden(&input.change_budget_reference)
        || string_is_forbidden(&input.expiry_reference)
    {
        return Err(M5FreezeExceptionResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = freeze_exception_object_is_complete(
        input.freeze_exception_change_class,
        &input.exception_scope_reference,
        &input.rollback_or_narrowing_reference,
        &input.docs_support_migration_reference,
        &input.owner_capture_reference,
        &input.risk_capture_reference,
        &input.change_budget_reference,
        &input.expiry_reference,
    );
    let preserve_ok = freeze_exception_stays_documented_before_widening(
        input.freeze_exception_change_class,
        input.freeze_exception_documented_before_widening,
        input.requires_documented_exception,
        input.attributable_asset_or_approved_exception,
    );
    let support_undisclosed =
        input.requires_documented_exception && !input.attributable_asset_or_approved_exception;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5FreezeExceptionEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.freeze_exception_change_class.is_classified() {
        Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionChangeClassUnclassified)
    } else if !input.bound_to_registry {
        Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionNotBoundToRegistry)
    } else if !object_complete {
        Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionObjectIncomplete)
    } else if !preserve_ok {
        Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionWidensScopeUndocumentedOrRunsClaimAheadOfProof)
    } else if !all_forms {
        Some(M5FreezeExceptionEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if support_undisclosed {
        Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionUndocumentedForScopeWidening)
    } else if !input.proof_fresh {
        Some(M5FreezeExceptionEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5FreezeExceptionNextAction::ExpandFreezeExceptionMeaning,
    };

    Ok(M5ResolvedFreezeExceptionEntry {
        entry_id: input.entry_id,
        exception_binding_id: input.exception_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        freeze_exception_change_class: input.freeze_exception_change_class.as_str().to_owned(),
        freeze_exception_change_class_is_classified: input
            .freeze_exception_change_class
            .is_classified(),
        canonical_freeze_exception_change_class_mode: input
            .freeze_exception_change_class
            .canonical_freeze_exception_change_class_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        exception_scope_reference: input.exception_scope_reference,
        rollback_or_narrowing_reference: input.rollback_or_narrowing_reference,
        docs_support_migration_reference: input.docs_support_migration_reference,
        owner_capture_reference: input.owner_capture_reference,
        risk_capture_reference: input.risk_capture_reference,
        change_budget_reference: input.change_budget_reference,
        expiry_reference: input.expiry_reference,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        freeze_exception_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        freeze_exception_documented_before_widening: input
            .freeze_exception_documented_before_widening,
        requires_documented_exception: input.requires_documented_exception,
        attributable_asset_or_approved_exception: input.attributable_asset_or_approved_exception,
        degrade_reason,
        next_action,
        freeze_exception_resolves_across_classes: degrade_reason.is_none(),
    })
}

/// Resolves a go-no-go entry so its evidence stays safe: the entry names its canonical token,
/// semantic role, and evidence scope, covers all three resolution forms, provides the complete cohort-identity /
/// known-limits-ledger / rollback-target / rehearsal-currency / readiness-signoff / support-language /
/// last-widening-revision go-no-go object, and degrades honestly when the evidence would run partner /
/// public support language ahead of cohort proof, hide the cohort evidence, or let a known-limits gap masquerade
/// as covered.
pub fn resolve_go_no_go_entry(
    input: M5GoNoGoEntryResolutionInput,
) -> Result<M5ResolvedGoNoGoEntry, M5FreezeExceptionResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5FreezeExceptionResolutionError::EmptyGoNoGoEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.go_no_go_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_decision_identity)
        || string_is_forbidden(&input.evidence_snapshot_ledger)
        || string_is_forbidden(&input.orr_signoff_reference)
        || string_is_forbidden(&input.on_call_roster_state)
        || string_is_forbidden(&input.go_no_go_freshness_state)
        || string_is_forbidden(&input.widening_stage_reference)
        || string_is_forbidden(&input.last_go_no_go_revision)
    {
        return Err(M5FreezeExceptionResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = go_no_go_stays_honest(
        input.go_no_go_decision,
        input.go_no_go_lineage_is_truthful,
        input.keeps_evidence_snapshot_visible,
        input.override_without_evidence_requested,
        input.blocked_until_evidence_linked,
        input.lineage_gap_present,
        input.lineage_gap_flagged,
    );
    let provides_record = input.go_no_go_decision.is_classified()
        && !input.resolved_decision_identity.trim().is_empty()
        && !input.evidence_snapshot_ledger.trim().is_empty()
        && !input.orr_signoff_reference.trim().is_empty()
        && !input.on_call_roster_state.trim().is_empty()
        && !input.go_no_go_freshness_state.trim().is_empty()
        && !input.widening_stage_reference.trim().is_empty()
        && !input.last_go_no_go_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5GoNoGoEntryDegradeReason::GoNoGoTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5GoNoGoEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.go_no_go_decision.is_classified() {
        Some(M5GoNoGoEntryDegradeReason::GoNoGoDecisionUnclassified)
    } else if !provides_record {
        Some(M5GoNoGoEntryDegradeReason::GoNoGoDropsEvidenceOrImpliesGreenWhileStale)
    } else if !all_forms {
        Some(M5GoNoGoEntryDegradeReason::GoNoGoFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5GoNoGoEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5FreezeExceptionNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedGoNoGoEntry {
        entry_id: input.entry_id,
        go_no_go_ref: input.go_no_go_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        go_no_go_decision: input.go_no_go_decision.as_str().to_owned(),
        go_no_go_decision_is_classified: input.go_no_go_decision.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        resolved_decision_identity: input.resolved_decision_identity,
        evidence_snapshot_ledger: input.evidence_snapshot_ledger,
        orr_signoff_reference: input.orr_signoff_reference,
        on_call_roster_state: input.on_call_roster_state,
        go_no_go_freshness_state: input.go_no_go_freshness_state,
        widening_stage_reference: input.widening_stage_reference,
        last_go_no_go_revision: input.last_go_no_go_revision,
        keeps_evidence_snapshot_visible: input.keeps_evidence_snapshot_visible,
        go_no_go_lineage_is_truthful: input.go_no_go_lineage_is_truthful,
        override_without_evidence_requested: input.override_without_evidence_requested,
        blocked_until_evidence_linked: input.blocked_until_evidence_linked,
        lineage_gap_present: input.lineage_gap_present,
        lineage_gap_flagged: input.lineage_gap_flagged,
        go_no_go_stays_honest: record_stays_honest,
        provides_complete_go_no_go_record: provides_record,
        degrade_reason,
        next_action,
        go_no_go_safe_on_every_decision: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved freeze-exception and go-no-go
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FreezeExceptionGoNoGoRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5FreezeExceptionGoNoGoRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5FreezeExceptionAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5FreezeExceptionExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    /// Resolved freeze-exception-registry examples.
    pub freeze_exception_entries: Vec<M5ResolvedFreezeExceptionEntry>,
    /// Resolved go-no-go examples.
    pub go_no_go_entries: Vec<M5ResolvedGoNoGoEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the freeze-exception and
    /// go-no-go domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never widens a cohort without current rollback and diagnostics evidence. MUST be
    /// `false`.
    pub widens_scope_without_a_documented_freeze_exception: bool,
    /// Hard invariant: this row never runs partner or public support language ahead of cohort proof. MUST be
    /// `false`.
    pub lets_a_freeze_exception_become_undocumented_scope_widening: bool,
    /// Hard invariant: this row never hides the rollback target or diagnostics posture before widening. MUST be
    /// `false`.
    pub hides_the_change_budget_or_owner_risk_on_the_freeze_exception: bool,
    /// Hard invariant: this row never collapses distinct cohort evidence classes into one lane. MUST be `false`.
    pub collapses_distinct_go_no_go_decision_classes_into_one_lane: bool,
}

impl M5FreezeExceptionGoNoGoRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5FreezeExceptionAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5FreezeExceptionAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5FreezeExceptionExportField> =
            self.export_fields.iter().copied().collect();
        M5FreezeExceptionExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.widens_scope_without_a_documented_freeze_exception
            && !self.lets_a_freeze_exception_become_undocumented_scope_widening
            && !self.hides_the_change_budget_or_owner_risk_on_the_freeze_exception
            && !self.collapses_distinct_go_no_go_decision_classes_into_one_lane
    }

    /// True when a clean freeze-exception entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified cohort archetype, publishes a complete descriptor object, preserves its rollback and
    /// diagnostics posture, covers all three resolution forms, and keeps a public-facing cohort's support
    /// language matched to proof.
    fn descriptor_is_honest(ex: &M5ResolvedFreezeExceptionEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.freeze_exception_change_class_is_classified
                && ex.freeze_exception_object_complete
                && ex.freeze_exception_documented_before_widening
                && ex.covers_all_resolution_forms
                && (!ex.requires_documented_exception
                    || ex.attributable_asset_or_approved_exception))
    }

    /// True when a clean go-no-go entry preserves a safe packet: it keeps a classified evidence
    /// scope, provides the complete go-no-go object, stays honest, and covers all three resolution forms.
    fn evidence_is_honest(ex: &M5ResolvedGoNoGoEntry) -> bool {
        !ex.is_clean()
            || (ex.go_no_go_decision_is_classified
                && ex.provides_complete_go_no_go_record
                && ex.go_no_go_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.freeze_exception_entries
            .iter()
            .all(Self::descriptor_is_honest)
            && self.go_no_go_entries.iter().all(Self::evidence_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FreezeExceptionGoNoGoRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Cohort-archetype tokens (minted by this lane).
    pub freeze_exception_change_class_kinds: Vec<String>,
    /// Evidence-scope tokens (minted by this lane).
    pub go_no_go_decisions: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Cohort-descriptor-entry degrade-reason tokens.
    pub freeze_exception_degrade_reasons: Vec<String>,
    /// Cohort-evidence-packet-entry degrade-reason tokens.
    pub go_no_go_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5FreezeExceptionGoNoGoRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5LaunchControlRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5FreezeExceptionResolutionForm::ALL, |v| v.as_str()),
            freeze_exception_change_class_kinds: tokens(&M5FreezeExceptionChangeClass::ALL, |v| {
                v.as_str()
            }),
            go_no_go_decisions: tokens(&M5GoNoGoDecisionKind::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5FreezeExceptionSurfaceContext::ALL, |v| v.as_str()),
            freeze_exception_degrade_reasons: tokens(
                &M5FreezeExceptionEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            go_no_go_degrade_reasons: tokens(&M5GoNoGoEntryDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5FreezeExceptionAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5FreezeExceptionNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5FreezeExceptionExportField::ALL, |v| v.as_str()),
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
pub struct M5FreezeExceptionGoNoGoRegistriesGovernanceReview {
    /// The descriptor registry names a canonical token, semantic role, and cohort archetype for every entry.
    pub freeze_exception_registry_names_token_role_and_type: bool,
    /// Every claimed cohort resolves to one typed freeze-exception object from the shared registry, not
    /// per-entry reconstruction.
    pub type_resolves_to_typed_freeze_exception_from_shared_registry: bool,
    /// The exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles are published for every resolved descriptor.
    pub build_row_and_cohort_lineage_published: bool,
    /// Cohorts cannot widen without preserving rollback and diagnostics posture before widening.
    pub scope_cannot_widen_without_documented_freeze_exception: bool,
    /// The cohort evidence keeps the cohort proof visible and binds partner / public support language to it.
    pub go_no_go_keeps_evidence_visible_and_blocks_stale_green: bool,
    /// Partner / public support language stays matched to cohort proof for every public-facing cohort.
    pub approved_exception_matched_to_scope_for_widening: bool,
    /// Every freeze-exception and go-no-go entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Cohort-descriptor and go-no-go behavior stay bound to the shared registries rather than
    /// hand-copied per cohort.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shiproom, release center, executive steering, and program governance read a single cohort source.
    pub shiproom_release_center_executive_steering_and_program_governance_read_single_source: bool,
    /// A widen-without-rollback attempt, an incomplete object, or hidden cohort evidence is caught by fixtures
    /// before release evidence turns green.
    pub exception_or_go_no_go_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FreezeExceptionGoNoGoRegistriesConsumerProjection {
    /// Shiproom and release center consume the shared freeze-exception registry.
    pub shiproom_and_release_center_consume_shared_registries: bool,
    /// Executive steering and program governance consume the shared go-no-go registry.
    pub executive_steering_and_program_governance_consume_shared_registries: bool,
    /// Diagnostics and public proof consume the shared registries.
    pub diagnostics_and_public_proof_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical freeze-exception and go-no-go domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical freeze-exception / go-no-go registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FreezeExceptionGoNoGoRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FreezeExceptionGoNoGoRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting cohort audit for the lane.
    pub go_no_go_control_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5FreezeExceptionGoNoGoRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5FreezeExceptionGoNoGoRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5FreezeExceptionGoNoGoRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5FreezeExceptionGoNoGoRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5FreezeExceptionGoNoGoRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FreezeExceptionGoNoGoRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5FreezeExceptionGoNoGoRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5FreezeExceptionGoNoGoRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 freeze-exception and go-no-go registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FreezeExceptionGoNoGoRegistriesPacket {
    /// Record kind; must equal [`M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5FreezeExceptionGoNoGoRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5FreezeExceptionGoNoGoRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5FreezeExceptionGoNoGoRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FreezeExceptionGoNoGoRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5FreezeExceptionGoNoGoRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5FreezeExceptionGoNoGoRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5FreezeExceptionGoNoGoRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5FreezeExceptionGoNoGoRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5FreezeExceptionGoNoGoRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_RECORD_KIND {
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 freeze-exception / go-no-go registries packet serializes"),
        ) {
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 freeze-exception / go-no-go registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,freeze_exception_entries,go_no_go_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .freeze_exception_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.go_no_go_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.freeze_exception_entries.len(),
                row.go_no_go_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Freeze-Exception and Go-No-Go Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Freeze exception change classes: {}\n",
            self.vocabulary_set
                .freeze_exception_change_class_kinds
                .join(", ")
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
                "  - Regression-asset entries: {} / go-no-go entries: {}\n",
                row.freeze_exception_entries.len(),
                row.go_no_go_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry cohort reference table generated from the registry, so docs and shiproom runbooks
    /// render the same archetype-mode / repo-rows / bundle-ids / install-topology / toolchain-envelope /
    /// rollback-target truth the resolvers produced rather than a hand-copied cohort table. Only clean,
    /// registry-bound freeze-exception entries are listed.
    pub fn render_freeze_exception_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| exception_binding_id | change_class_mode | exception_scope_reference | rollback_or_narrowing_reference | docs_support_migration_reference | owner_capture_reference | change_budget_reference |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.freeze_exception_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.exception_binding_id,
                    ex.canonical_freeze_exception_change_class_mode,
                    ex.exception_scope_reference,
                    ex.rollback_or_narrowing_reference,
                    ex.docs_support_migration_reference,
                    ex.owner_capture_reference,
                    ex.change_budget_reference
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5FreezeExceptionGoNoGoRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5FreezeExceptionGoNoGoRegistriesViolation>),
}

impl fmt::Display for M5FreezeExceptionGoNoGoRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 freeze-exception / go-no-go registries export parse failed: {error}"
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
                    "m5 freeze-exception / go-no-go registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5FreezeExceptionGoNoGoRegistriesArtifactError {}

/// Validation failures emitted by [`M5FreezeExceptionGoNoGoRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5FreezeExceptionGoNoGoRegistriesViolation {
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
    /// A registry row does not point at both the freeze-exception and go-no-go domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, widen-without-rollback, field-incomplete,
    /// form-incomplete, or a go-no-go entry missing the complete evidence object).
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
    FreezeExceptionResolutionNotProven,
    /// Rollback-and-diagnostics-preservation is not proven: no widen-without-rollback example and no unbound
    /// example degrade, no clean bounded descriptor entry is present, or a clean descriptor entry is unbounded
    /// or unbound.
    GoNoGoAttributionNotProven,
    /// Cohort-evidence-integrity is not proven: clean evidence entries do not cover the canonical dogfood-ring /
    /// rehearsal-currency / go-no-go-signoff scopes with full resolution-form coverage while providing the
    /// complete evidence object, no support-ahead or form-incomplete example degrades, or a clean evidence entry
    /// is missing the complete evidence object.
    GoNoGoIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5FreezeExceptionGoNoGoRegistriesViolation {
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
            Self::FreezeExceptionResolutionNotProven => "freeze_exception_resolution_not_proven",
            Self::GoNoGoAttributionNotProven => "go_no_go_attribution_not_proven",
            Self::GoNoGoIntegrityNotProven => "go_no_go_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_freeze_exception_and_go_no_go_registries_export(
) -> Result<M5FreezeExceptionGoNoGoRegistriesPacket, M5FreezeExceptionGoNoGoRegistriesArtifactError>
{
    let packet: M5FreezeExceptionGoNoGoRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-freeze-exception-and-go-no-go-registries-proof/support_export.json"
        )
    ))
    .map_err(M5FreezeExceptionGoNoGoRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5FreezeExceptionGoNoGoRegistriesArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5FreezeExceptionGoNoGoRegistriesPacket,
    violations: &mut Vec<M5FreezeExceptionGoNoGoRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_SCHEMA_REF,
        M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_DOC_REF,
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_FREEZE_EXCEPTION_DOMAIN_SCHEMA_REF,
        M5_GO_NO_GO_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5FreezeExceptionGoNoGoRegistriesPacket,
    violations: &mut Vec<M5FreezeExceptionGoNoGoRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::NoRegistryRows);
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
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5FreezeExceptionGoNoGoRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_FREEZE_EXCEPTION_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_GO_NO_GO_DOMAIN_SCHEMA_REF)
        {
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.freeze_exception_entries.is_empty() || row.go_no_go_entries.is_empty() {
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5FreezeExceptionGoNoGoRegistriesPacket,
    violations: &mut Vec<M5FreezeExceptionGoNoGoRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.freeze_exception_registry_names_token_role_and_type,
        review.type_resolves_to_typed_freeze_exception_from_shared_registry,
        review.build_row_and_cohort_lineage_published,
        review.scope_cannot_widen_without_documented_freeze_exception,
        review.go_no_go_keeps_evidence_visible_and_blocks_stale_green,
        review.approved_exception_matched_to_scope_for_widening,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shiproom_release_center_executive_steering_and_program_governance_read_single_source,
        review.exception_or_go_no_go_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5FreezeExceptionGoNoGoRegistriesPacket,
    violations: &mut Vec<M5FreezeExceptionGoNoGoRegistriesViolation>,
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
            violations
                .push(M5FreezeExceptionGoNoGoRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5FreezeExceptionGoNoGoRegistriesPacket,
    violations: &mut Vec<M5FreezeExceptionGoNoGoRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5FreezeExceptionGoNoGoRegistriesPacket,
    violations: &mut Vec<M5FreezeExceptionGoNoGoRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.go_no_go_control_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5FreezeExceptionGoNoGoRegistriesPacket,
    violations: &mut Vec<M5FreezeExceptionGoNoGoRegistriesViolation>,
) {
    let descriptors = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.freeze_exception_entries.iter())
    };
    let evidence = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.go_no_go_entries.iter())
    };

    // AC1: every active cohort can be inspected by exact rows, bundles, toolchains, and deployment profiles.
    // Clean descriptor entries cover the canonical cohort archetypes and the first release-center / shiproom /
    // executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no
    // clean descriptor entry published an incomplete object.
    let clean_archetypes: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.freeze_exception_change_class.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let archetypes_covered = M5FreezeExceptionChangeClass::CANONICAL_CHANGE_CLASSES
        .iter()
        .all(|k| clean_archetypes.contains(k.as_str()));
    let first_surfaces_covered = M5FreezeExceptionSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionObjectIncomplete)
    });
    let no_clean_incomplete =
        !descriptors().any(|ex| ex.is_clean() && !ex.freeze_exception_object_complete);
    if !(archetypes_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations
            .push(M5FreezeExceptionGoNoGoRegistriesViolation::FreezeExceptionResolutionNotProven);
    }

    // AC2: cohort packets preserve rollback and diagnostics posture before widening. A widen-without-rollback
    // example degrades, an unbound example degrades, at least one clean bounded descriptor entry is present, and
    // no clean descriptor entry is unbounded or unbound.
    let widen_fold_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(
                M5FreezeExceptionEntryDegradeReason::FreezeExceptionWidensScopeUndocumentedOrRunsClaimAheadOfProof,
            )
    });
    let unbound_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionNotBoundToRegistry)
    });
    let bounded_clean_descriptor =
        descriptors().any(|ex| ex.is_clean() && ex.freeze_exception_documented_before_widening);
    let no_clean_unbound = !descriptors().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded =
        !descriptors().any(|ex| ex.is_clean() && !ex.freeze_exception_documented_before_widening);
    if !(widen_fold_degrades
        && unbound_degrades
        && bounded_clean_descriptor
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::GoNoGoAttributionNotProven);
    }

    // AC3: claim publication can prove which cohort evidence backs each launch-bearing lane. Clean evidence
    // entries cover every canonical dogfood-ring / rehearsal-currency / go-no-go-signoff scope with full
    // resolution-form coverage while providing the complete evidence object, a support-ahead example degrades, a
    // form-incomplete example degrades, and no clean evidence entry is missing the complete object.
    let clean_go_no_go_decisions: BTreeSet<String> = evidence()
        .filter(|ex| {
            ex.is_clean()
                && ex.go_no_go_decision_is_classified
                && ex.provides_complete_go_no_go_record
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.go_no_go_decision.clone())
        .collect();
    let go_no_go_decisions_covered = M5GoNoGoDecisionKind::CANONICAL_DECISIONS
        .iter()
        .all(|m| clean_go_no_go_decisions.contains(m.as_str()));
    let support_ahead_degrades = evidence().any(|ex| {
        ex.degrade_reason
            == Some(M5GoNoGoEntryDegradeReason::GoNoGoDropsEvidenceOrImpliesGreenWhileStale)
    });
    let form_incomplete_degrades = evidence().any(|ex| {
        ex.degrade_reason == Some(M5GoNoGoEntryDegradeReason::GoNoGoFormCoverageIncomplete)
    });
    let no_clean_missing_evidence =
        !evidence().any(|ex| ex.is_clean() && !ex.provides_complete_go_no_go_record);
    if !(go_no_go_decisions_covered
        && support_ahead_degrades
        && form_incomplete_degrades
        && no_clean_missing_evidence)
    {
        violations.push(M5FreezeExceptionGoNoGoRegistriesViolation::GoNoGoIntegrityNotProven);
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

/// The widening stages this lane governs, for downstream reference: the freeze-exception registry defines the
/// minimum evidence and soak expectations that let a lane advance across the alpha, beta, release-candidate,
/// stable, and long-term-support widening stages, and the go-no-go registry records the conditions that
/// immediately stop that progression.
pub const IMPLEMENTED_FREEZE_EXCEPTION_STAGES: [M5LaunchControlWideningStage; 5] =
    M5LaunchControlWideningStage::ALL;
