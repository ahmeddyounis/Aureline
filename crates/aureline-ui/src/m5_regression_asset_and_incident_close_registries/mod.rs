//! Implemented M5 regression-asset and incident-close registries.
//!
//! The frozen [launch-control matrix][matrix] names Aureline's governed launch-bearing cohorts and locks their
//! controlled vocabulary. This module governs *ring widening by explicit stop conditions rather than schedule
//! optimism*: it turns the *regression-asset* grammar (how each regression asset type — canary, broad internal
//! dogfood, design-partner preview, public preview, and certified stable — declares its minimum entry evidence,
//! soak-window expectation, why widening is allowed, its known-limits packet, issue-template linkage,
//! claim-narrowing action, and the incident-close reference that immediately stops it) and the *incident-close*
//! grammar (how a launch-bearing lane records the incident-close condition — a crash / data-loss / trust defect,
//! a repeated protected-metric regression, or a stale readiness packet — that halts regression asset while it is
//! active) into registry resolvers that produce export-safe, honest projections. Every claimed M5 ring
//! transition then resolves to one typed regression-asset object — the regression asset type it classifies, the
//! minimum entry evidence, the soak-window expectation, the widening-allow rationale, the known-limits packet,
//! the issue-template ref, the claim-narrowing action, and the incident-close reference, all visible before
//! widening so a ring never advances without its known-limits and incident-close posture and so partner / public
//! support language never outruns current ring proof — and to one incident-close object — the resolved transition
//! identity, the active stop-condition ledger, the incident-close target reference, the protected-metric
//! regression state, the packet-freshness state, the crash / data-loss / trust reference, and the last
//! ring-transition revision — that the shiproom, release-center, executive-steering, program-governance, and
//! support / export surfaces can inspect without manual reconstruction, so every ring transition can state why
//! widening is allowed and what immediately stops it, known-limits and rollback posture stay visible before any
//! ring widens, regression asset can never advance on a claimed lane while a incident-close condition is active,
//! and a ring that cannot explain the progression rule it declared or the stop condition that backs it degrades
//! honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one typed regression-asset object per regression asset type.** [`resolve_regression_asset_entry`]
//!   refuses to read as a clean, registry-bound progression entry unless it names a canonical registry token, a
//!   classified [regression asset type][M5RegressionAssetTypeKind], a launch-control role, covers every
//!   [resolution form][M5RegressionAssetResolutionForm] (the canonical object, the accessible summary, and the audit
//!   record), publishes every progression field (minimum entry evidence, soak-window expectation, widening-allow
//!   rationale, issue-template ref, known limits, claim-narrowing action, and incident-close reference), keeps its
//!   known-limits and incident-close posture visible before widening, and keeps partner / public support language
//!   matched to ring proof; otherwise it degrades.
//! * **Keep a ring from advancing without a visible incident-close and known-limits posture.**
//!   [`regression_asset_attributable_before_closure`] rejects a progression entry whose incident-close and
//!   known-limits posture is not visible (a ring advancing without a incident-close reference and known limits) so
//!   it degrades to
//!   [`M5RegressionAssetEntryDegradeReason::IncidentClosesWithoutRegressionAssetOrRunsClaimAheadOfProof`],
//!   and a public-facing ring whose support language runs ahead of ring proof degrades the same way — the
//!   structured blocker reason a widen-without-stop attempt must surface.
//! * **Keep the incident-close record from advancing a ring while a stop condition is active.**
//!   [`resolve_incident_close_entry`] names a classified [incident-close condition][M5IncidentSeverityKind],
//!   requires the full transition-identity / active-stop-condition-ledger / incident-close-target /
//!   protected-metric-regression / packet-freshness / crash-data-loss-or-trust / last-ring-transition-revision
//!   record, covers every resolution form, and degrades to
//!   [`M5IncidentCloseEntryDegradeReason::IncidentCloseDropsLineageOrClosesWithoutRegressionAsset`]
//!   when the record would advance a ring while a stop condition is active, hide the incident-close, or let a
//!   protected-metric regression masquerade as covered, so a incident-close record can never read as trustworthy
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
    seeded_m5_regression_asset_and_incident_close_registries,
    seeded_m5_regression_asset_and_incident_close_registries_incident_close_preview_narrowed,
    seeded_m5_regression_asset_and_incident_close_registries_regression_asset_beta_narrowed,
    M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_PACKET_ID,
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

/// Stable record-kind tag carried by [`M5RegressionAssetIncidentCloseRegistriesPacket`].
pub const M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_regression_asset_and_incident_close_registries";

/// Schema version for M5 regression-asset / incident-close registry records.
pub const M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_SCHEMA_REF: &str =
    "schemas/program/m5-regression-asset-and-incident-close-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_DOC_REF: &str =
    "docs/release/m5_regression_asset_and_incident_close_registries.md";

/// Repo-relative path of the canonical regression-asset domain schema minted by this lane (how a widening ring
/// transition declares its minimum entry evidence, soak-window expectation, why widening is allowed, its
/// known-limits packet, issue-template linkage, claim-narrowing action, and the incident-close reference that
/// immediately stops it).
pub const M5_REGRESSION_ASSET_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-regression-asset.schema.json";

/// Repo-relative path of the canonical incident-close domain schema minted by this lane (how a launch-bearing lane
/// records the incident-close condition — a crash / data-loss / trust defect, a repeated protected-metric
/// regression, or a stale readiness packet — that halts regression asset while it is active).
pub const M5_INCIDENT_CLOSE_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-incident-close.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-regression-asset-and-incident-close-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-regression-asset-and-incident-close-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-regression-asset-and-incident-close-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/release/m5-regression-asset-and-incident-close-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// cohort invents a parallel surface set.
pub type M5RegressionAssetIncidentCloseRegistriesConsumerSurface = M5LaunchControlConsumerSurface;

/// One of the three resolution forms every regression-asset or incident-close entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// regression-asset and incident-close *domains* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RegressionAssetResolutionForm {
    /// The canonical resolved regression-asset / incident-close object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved cohort discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved cohort inspectable off-renderer.
    AuditRecord,
}

impl M5RegressionAssetResolutionForm {
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

/// Controlled cohort archetype a regression-asset entry classifies, so the typed descriptor model shares one
/// registry rather than a hand-copied per-cohort assumption. Minted by this lane because the frozen matrix
/// carries the launch-bearing cohorts but distinguishes the dogfood / migration-alpha / extension-author /
/// design-partner / public-preview / certified-archetype archetypes an auditable descriptor classifies against
/// explicitly. Every classified archetype carries its canonical mode, and the design-partner-preview and
/// public-preview archetypes are public-facing so their partner / public support language must stay matched to
/// cohort proof before the cohort widens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RegressionAssetTypeKind {
    /// The internal dogfood core-team canary cohort.
    AutomatedTest,
    /// The migration alpha cohort (external alpha migrating from a prior toolchain).
    FixtureRepository,
    /// The extension-author cohort (compatibility rehearsals current, freeze exceptions documented).
    RecoveryDrill,
    /// The design-partner preview cohort (public-facing; support language must match cohort proof).
    ProtectedCorpusCase,
    /// The public preview cohort (public-facing; support language must match cohort proof).
    SchemaPolicyGuard,
    /// The certified-archetype cohort (ORR signed and a go/no-go decision recorded).
    MonitoringRegressionCheck,
    /// The cohort archetype is unclassified, which is disallowed.
    AssetTypeUnclassified,
}

impl M5RegressionAssetTypeKind {
    /// Every cohort archetype, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AutomatedTest,
        Self::FixtureRepository,
        Self::RecoveryDrill,
        Self::ProtectedCorpusCase,
        Self::SchemaPolicyGuard,
        Self::MonitoringRegressionCheck,
        Self::AssetTypeUnclassified,
    ];

    /// The six canonical cohort archetypes every claimed M5 launch-bearing cohort classifies against.
    pub const CANONICAL_TRANSITIONS: [Self; 6] = [
        Self::AutomatedTest,
        Self::FixtureRepository,
        Self::RecoveryDrill,
        Self::ProtectedCorpusCase,
        Self::SchemaPolicyGuard,
        Self::MonitoringRegressionCheck,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutomatedTest => "automated_test",
            Self::FixtureRepository => "fixture_repository",
            Self::RecoveryDrill => "recovery_drill",
            Self::ProtectedCorpusCase => "protected_corpus_case",
            Self::SchemaPolicyGuard => "schema_policy_guard",
            Self::MonitoringRegressionCheck => "monitoring_regression_check",
            Self::AssetTypeUnclassified => "asset_type_unclassified",
        }
    }

    /// Whether the archetype is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::AssetTypeUnclassified)
    }

    /// The canonical mode for this cohort archetype.
    pub const fn canonical_regression_asset_type_mode(self) -> &'static str {
        match self {
            Self::AutomatedTest => "automated_test_type",
            Self::FixtureRepository => "fixture_repository_type",
            Self::RecoveryDrill => "recovery_drill_type",
            Self::ProtectedCorpusCase => "protected_corpus_case_type",
            Self::SchemaPolicyGuard => "schema_policy_guard_type",
            Self::MonitoringRegressionCheck => "monitoring_regression_check_type",
            Self::AssetTypeUnclassified => "",
        }
    }

    /// Whether this archetype is public-facing and so must keep partner / public support language matched to
    /// cohort proof before the cohort widens.
    pub const fn is_severe_incident(self) -> bool {
        matches!(self, Self::ProtectedCorpusCase | Self::SchemaPolicyGuard)
    }
}

/// Controlled evidence scope a incident-close entry must resolve its cohort proof from, so an evidence
/// packet shares one registry rather than a hand-copied per-record assumption. Minted by this lane, tracking
/// whether the evidence came from dogfood-ring telemetry, current rehearsal cadence, or an explicit go/no-go
/// signoff the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IncidentSeverityKind {
    /// The evidence came from internal dogfood-ring telemetry.
    SevOneIncident,
    /// The evidence came from current rehearsal cadence (publish/rollback, mixed-version, handoff drills).
    SevTwoIncident,
    /// The evidence came from an explicit go/no-go signoff with a preserved evidence snapshot.
    LaunchBearingFailure,
    /// The evidence scope is unclassified, which is disallowed.
    SeverityUnclassified,
}

impl M5IncidentSeverityKind {
    /// Every evidence scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SevOneIncident,
        Self::SevTwoIncident,
        Self::LaunchBearingFailure,
        Self::SeverityUnclassified,
    ];

    /// The three canonical evidence scopes every incident-close packet must stay distinct across.
    pub const CANONICAL_CONDITIONS: [Self; 3] = [
        Self::SevOneIncident,
        Self::SevTwoIncident,
        Self::LaunchBearingFailure,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SevOneIncident => "sev_one_incident",
            Self::SevTwoIncident => "sev_two_incident",
            Self::LaunchBearingFailure => "launch_bearing_failure",
            Self::SeverityUnclassified => "severity_unclassified",
        }
    }

    /// Whether the evidence scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::SeverityUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a regression-asset or
/// incident-close token's meaning stays stable whether it appears in the release-center, shiproom,
/// executive-steering, program-governance, or a support / export form. Minted by this lane, tracking the
/// first-consumer surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RegressionAssetSurfaceContext {
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

impl M5RegressionAssetSurfaceContext {
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

/// One mandatory rendered part a regression-asset or incident-close entry must be able to show, so no
/// cohort archetype, repo / bundle / toolchain / deployment row, known-limits packet, rollback target,
/// incident-close field, or registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RegressionAssetAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The cohort archetype the entry classifies (regression-asset entry).
    RegressionAssetType,
    /// The exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles the entry publishes (regression-asset entry).
    IncidentLineageRows,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The known-limits packet and rollback target the entry preserves before widening (regression-asset
    /// entry).
    BuildAndCohortLineage,
    /// The incident-close fields (cohort identity, known-limits ledger, rollback target, rehearsal currency,
    /// readiness signoff, support language) the entry publishes (incident-close entry).
    IncidentCloseFields,
    /// The support-identity hint the entry publishes (incident-close entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved cohort descriptor or cohort evidence (both entries).
    PlainLanguageMeaning,
}

impl M5RegressionAssetAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::RegressionAssetType,
        Self::IncidentLineageRows,
        Self::ResolutionFormCoverage,
        Self::BuildAndCohortLineage,
        Self::IncidentCloseFields,
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
            Self::RegressionAssetType => "regression_asset_type",
            Self::IncidentLineageRows => "incident_lineage_rows",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::BuildAndCohortLineage => "build_and_cohort_lineage",
            Self::IncidentCloseFields => "incident_close_fields",
            Self::SupportIdentityHint => "support_identity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// cohort descriptor, a incident-close packet, or a degraded regression-asset / incident-close entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RegressionAssetNextAction {
    /// Expand the resolved cohort descriptor's or incident-close packet's plain-language meaning.
    ExpandRegressionAssetMeaning,
    /// Inspect the cohort archetype or evidence scope the entry resolves.
    InspectAssetTypeOrSeverity,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5RegressionAssetNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandRegressionAssetMeaning,
        Self::InspectAssetTypeOrSeverity,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandRegressionAssetMeaning => "expand_regression_asset_meaning",
            Self::InspectAssetTypeOrSeverity => "inspect_asset_type_or_severity",
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
pub enum M5RegressionAssetExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The cohort families covered.
    RegressionAssetFamilies,
    /// The cohort archetypes carried.
    RegressionAssetTypes,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The evidence scopes carried.
    IncidentSeverities,
    /// The render / surface context.
    SurfaceContext,
    /// The cohort-archetype modes carried.
    RegressionAssetTypeModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5RegressionAssetExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::RegressionAssetFamilies,
        Self::RegressionAssetTypes,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::IncidentSeverities,
        Self::SurfaceContext,
        Self::RegressionAssetTypeModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::RegressionAssetFamilies,
        Self::RegressionAssetTypes,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::RegressionAssetFamilies => "regression_asset_families",
            Self::RegressionAssetTypes => "regression_asset_types",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::IncidentSeverities => "incident_severities",
            Self::SurfaceContext => "surface_context",
            Self::RegressionAssetTypeModes => "regression_asset_type_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a regression-asset entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, widen-without-rollback, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RegressionAssetEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the descriptor means.
    RegressionAssetTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The cohort archetype is unclassified (not in the resolved taxonomy).
    RegressionAssetTypeUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    RegressionAssetNotBoundToRegistry,
    /// The resolved regression-asset object is incomplete: the exact repo / archetype rows, bundle IDs, install
    /// topology, toolchain envelope, known limits, rollback target, or diagnostics posture is unstated.
    RegressionAssetObjectIncomplete,
    /// The cohort's rollback and diagnostics posture is not preserved before widening (a cohort widening without
    /// a rollback target and diagnostics posture), or a public-facing cohort ran its support language ahead of
    /// cohort proof.
    IncidentClosesWithoutRegressionAssetOrRunsClaimAheadOfProof,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A public-facing cohort did not keep its support language matched to cohort proof before widening.
    RegressionAssetNotAttributableForSevereIncident,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RegressionAssetEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RegressionAssetTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::RegressionAssetTypeUnclassified,
        Self::RegressionAssetNotBoundToRegistry,
        Self::RegressionAssetObjectIncomplete,
        Self::IncidentClosesWithoutRegressionAssetOrRunsClaimAheadOfProof,
        Self::ResolutionFormCoverageIncomplete,
        Self::RegressionAssetNotAttributableForSevereIncident,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegressionAssetTokenUnstated => "regression_asset_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::RegressionAssetTypeUnclassified => "regression_asset_type_unclassified",
            Self::RegressionAssetNotBoundToRegistry => "regression_asset_not_bound_to_registry",
            Self::RegressionAssetObjectIncomplete => "regression_asset_object_incomplete",
            Self::IncidentClosesWithoutRegressionAssetOrRunsClaimAheadOfProof => {
                "incident_closes_without_regression_asset_or_runs_claim_ahead_of_proof"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::RegressionAssetNotAttributableForSevereIncident => {
                "regression_asset_not_attributable_for_severe_incident"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5RegressionAssetNextAction {
        match self {
            Self::RegressionAssetTokenUnstated | Self::RegressionAssetNotBoundToRegistry => {
                M5RegressionAssetNextAction::TraceCanonicalRegistry
            }
            Self::RegressionAssetTypeUnclassified
            | Self::RegressionAssetObjectIncomplete
            | Self::IncidentClosesWithoutRegressionAssetOrRunsClaimAheadOfProof => {
                M5RegressionAssetNextAction::InspectAssetTypeOrSeverity
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5RegressionAssetNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::RegressionAssetNotAttributableForSevereIncident
            | Self::ProofStale => M5RegressionAssetNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5LaunchControlDowngradeTrigger {
        match self {
            Self::RegressionAssetTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::RegressionAssetNotBoundToRegistry => {
                M5LaunchControlDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::RegressionAssetTypeUnclassified | Self::RegressionAssetObjectIncomplete => {
                M5LaunchControlDowngradeTrigger::CohortMembershipUnstated
            }
            Self::IncidentClosesWithoutRegressionAssetOrRunsClaimAheadOfProof
            | Self::RegressionAssetNotAttributableForSevereIncident => {
                M5LaunchControlDowngradeTrigger::WidenedWithoutCurrentCohortEvidence
            }
            Self::ProofStale => M5LaunchControlDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a incident-close entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IncidentCloseEntryDegradeReason {
    /// The canonical registry token name is unstated.
    IncidentCloseTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The evidence scope is unclassified (not in the resolved taxonomy).
    IncidentSeverityUnclassified,
    /// The cohort evidence would run partner / public support language ahead of cohort proof, hide the cohort
    /// evidence, let a known-limits gap masquerade as covered, or it dropped one of the required incident-close
    /// fields (cohort identity, known-limits ledger, rollback target, rehearsal currency, readiness signoff,
    /// support language, last widening revision).
    IncidentCloseDropsLineageOrClosesWithoutRegressionAsset,
    /// The canonical / accessible / audit resolution-form coverage of the evidence is incomplete.
    IncidentCloseFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5IncidentCloseEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IncidentCloseTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::IncidentSeverityUnclassified,
        Self::IncidentCloseDropsLineageOrClosesWithoutRegressionAsset,
        Self::IncidentCloseFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncidentCloseTokenUnstated => "incident_close_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::IncidentSeverityUnclassified => "incident_severity_unclassified",
            Self::IncidentCloseDropsLineageOrClosesWithoutRegressionAsset => {
                "incident_close_drops_lineage_or_closes_without_regression_asset"
            }
            Self::IncidentCloseFormCoverageIncomplete => "incident_close_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5RegressionAssetNextAction {
        match self {
            Self::IncidentCloseTokenUnstated => M5RegressionAssetNextAction::TraceCanonicalRegistry,
            Self::IncidentSeverityUnclassified
            | Self::IncidentCloseDropsLineageOrClosesWithoutRegressionAsset => {
                M5RegressionAssetNextAction::InspectAssetTypeOrSeverity
            }
            Self::IncidentCloseFormCoverageIncomplete => {
                M5RegressionAssetNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5RegressionAssetNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5LaunchControlDowngradeTrigger {
        match self {
            Self::IncidentCloseTokenUnstated => {
                M5LaunchControlDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::IncidentSeverityUnclassified => {
                M5LaunchControlDowngradeTrigger::ReadinessStateUnstated
            }
            Self::IncidentCloseDropsLineageOrClosesWithoutRegressionAsset => {
                M5LaunchControlDowngradeTrigger::RanPartnerOrPublicLanguageAheadOfCohortProof
            }
            Self::IncidentCloseFormCoverageIncomplete => {
                M5LaunchControlDowngradeTrigger::ImpliedGreenWhileGoNoGoOrOrrWasStale
            }
            Self::ProofStale => M5LaunchControlDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_regression_asset_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RegressionAssetEntryResolutionInput {
    /// Stable identity of the regression-asset-registry entry.
    pub entry_id: String,
    /// The stable cohort-binding ID this descriptor binds to (e.g. `incident.lane.public-preview`); empty means
    /// unstated.
    pub asset_binding_id: String,
    /// The canonical registry token name (e.g. `regression.asset.schema_policy_guard`); empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5LaunchControlRole,
    /// The cohort archetype this entry classifies.
    pub regression_asset_type: M5RegressionAssetTypeKind,
    /// The render / surface context.
    pub surface_context: M5RegressionAssetSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5RegressionAssetResolutionForm>,
    /// The published exact repo / archetype rows; empty means unstated.
    pub exact_build_reference: String,
    /// The published bundle IDs; empty means unstated.
    pub affected_row_reference: String,
    /// The published install topology; empty means unstated.
    pub cohort_ring_reference: String,
    /// The published toolchain envelope; empty means unstated.
    pub workaround_lineage: String,
    /// The published known limits; empty means unstated.
    pub regression_asset_reference: String,
    /// The published rollback target; empty means unstated.
    pub approved_exception_reference: String,
    /// The published diagnostics posture; empty means unstated.
    pub close_blocker_reference: String,
    /// True when the behavior traces to the regression-asset registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the cohort's rollback and diagnostics posture is preserved before widening (a hard invariant
    /// when `false`).
    pub regression_asset_linked_before_closure: bool,
    /// True when this cohort's archetype is public-facing.
    pub is_severe_incident: bool,
    /// True when partner / public support language is matched to cohort proof before a public-facing cohort
    /// widens.
    pub attributable_asset_or_approved_exception: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe regression-asset-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRegressionAssetEntry {
    /// Stable identity of the regression-asset-registry entry.
    pub entry_id: String,
    /// The stable cohort-binding ID this descriptor binds to.
    pub asset_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the evidence snapshot and signoff before widening.
    pub semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: bool,
    /// The cohort-archetype token named by the entry.
    pub regression_asset_type: String,
    /// Whether the cohort archetype is classified into the resolved taxonomy.
    pub regression_asset_type_is_classified: bool,
    /// The canonical mode for the entry's cohort archetype.
    pub canonical_regression_asset_type_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published exact repo / archetype rows.
    pub exact_build_reference: String,
    /// The published bundle IDs.
    pub affected_row_reference: String,
    /// The published install topology.
    pub cohort_ring_reference: String,
    /// The published toolchain envelope.
    pub workaround_lineage: String,
    /// The published known limits.
    pub regression_asset_reference: String,
    /// The published rollback target.
    pub approved_exception_reference: String,
    /// The published diagnostics posture.
    pub close_blocker_reference: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved regression-asset object publishes every required field.
    pub regression_asset_object_complete: bool,
    /// Whether the entry traces to the regression-asset registry.
    pub bound_to_registry: bool,
    /// Whether the cohort's rollback and diagnostics posture stays preserved before widening.
    pub regression_asset_linked_before_closure: bool,
    /// Whether this cohort's archetype is public-facing.
    pub is_severe_incident: bool,
    /// Whether partner / public support language is matched to cohort proof before widening.
    pub attributable_asset_or_approved_exception: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5RegressionAssetEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5RegressionAssetNextAction,
    /// Whether the descriptor resolves to one typed object across every claimed cohort (clean entry naming every
    /// fact).
    pub regression_asset_resolves_across_types: bool,
}

impl M5ResolvedRegressionAssetEntry {
    /// Whether this regression-asset entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_incident_close_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5IncidentCloseEntryResolutionInput {
    /// Stable identity of the incident-close entry.
    pub entry_id: String,
    /// The stable evidence-ref this record binds to; empty means unstated.
    pub incident_close_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5LaunchControlRole,
    /// The evidence scope this record must resolve its cohort proof from.
    pub incident_severity: M5IncidentSeverityKind,
    /// The render / surface context.
    pub surface_context: M5RegressionAssetSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5RegressionAssetResolutionForm>,
    /// The published resolved cohort identity; empty means missing.
    pub resolved_incident_identity: String,
    /// The published known-limits ledger; empty means missing.
    pub linked_regression_asset_ledger: String,
    /// The published rollback-target reference; empty means missing.
    pub exact_build_and_row_reference: String,
    /// The published rehearsal-currency state; empty means missing.
    pub cohort_ring_lineage_state: String,
    /// The published readiness-signoff state; empty means missing.
    pub close_lineage_freshness_state: String,
    /// The published cohort-bound support-language reference; empty means missing.
    pub workaround_lineage_reference: String,
    /// The published last widening revision; empty means missing.
    pub last_incident_close_revision: String,
    /// True when the record keeps the cohort evidence visible.
    pub keeps_incident_lineage_visible: bool,
    /// True when the evidence is truthful (never claims a clean packet over hidden cohort evidence).
    pub close_lineage_is_truthful: bool,
    /// True when partner / public support language is present on this record.
    pub close_without_asset_requested: bool,
    /// True when the support language is bound to cohort proof rather than running ahead of it.
    pub close_blocked_until_asset_linked: bool,
    /// True when a known-limits gap is present on this record.
    pub lineage_gap_present: bool,
    /// True when a known-limits gap is flagged rather than masquerading as covered.
    pub lineage_gap_flagged: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe incident-close projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedIncidentCloseEntry {
    /// Stable identity of the incident-close entry.
    pub entry_id: String,
    /// The stable evidence-ref this record binds to.
    pub incident_close_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the evidence snapshot and signoff before widening.
    pub semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: bool,
    /// The evidence-scope token named by the entry.
    pub incident_severity: String,
    /// Whether the evidence scope is classified into the resolved taxonomy.
    pub incident_severity_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published resolved cohort identity.
    pub resolved_incident_identity: String,
    /// The published known-limits ledger.
    pub linked_regression_asset_ledger: String,
    /// The published rollback-target reference.
    pub exact_build_and_row_reference: String,
    /// The published rehearsal-currency state.
    pub cohort_ring_lineage_state: String,
    /// The published readiness-signoff state.
    pub close_lineage_freshness_state: String,
    /// The published cohort-bound support-language reference.
    pub workaround_lineage_reference: String,
    /// The published last widening revision.
    pub last_incident_close_revision: String,
    /// Whether the record keeps the cohort evidence visible.
    pub keeps_incident_lineage_visible: bool,
    /// Whether the evidence is truthful.
    pub close_lineage_is_truthful: bool,
    /// Whether partner / public support language is present on this build.
    pub close_without_asset_requested: bool,
    /// Whether the support language is bound to cohort proof rather than running ahead of it.
    pub close_blocked_until_asset_linked: bool,
    /// Whether a known-limits gap is present on this record.
    pub lineage_gap_present: bool,
    /// Whether a known-limits gap is flagged rather than masquerading as covered.
    pub lineage_gap_flagged: bool,
    /// Whether the record stays honest (cohort evidence visible, support language bound to proof, known-limits
    /// gap flagged).
    pub incident_close_stays_honest: bool,
    /// Whether the entry provides the complete incident-close object (cohort identity, known-limits ledger,
    /// rollback target, rehearsal currency, readiness signoff, support language, last widening revision).
    pub provides_complete_incident_close_record: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5IncidentCloseEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5RegressionAssetNextAction,
    /// Whether the cohort evidence is safe on every claimed cohort (clean entry naming every fact).
    pub incident_close_safe_on_every_severity: bool,
}

impl M5ResolvedIncidentCloseEntry {
    /// Whether this incident-close entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5RegressionAssetResolutionError {
    /// The regression-asset-entry id was empty.
    EmptyRegressionAssetEntryId,
    /// The incident-close-entry id was empty.
    EmptyIncidentCloseEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5RegressionAssetResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyRegressionAssetEntryId => "empty_regression_asset_entry_id",
            Self::EmptyIncidentCloseEntryId => "empty_incident_close_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5RegressionAssetResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 regression-asset / incident-close registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RegressionAssetResolutionError {}

fn form_tokens(forms: &[M5RegressionAssetResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5RegressionAssetResolutionForm]) -> bool {
    let present: BTreeSet<M5RegressionAssetResolutionForm> = forms.iter().copied().collect();
    M5RegressionAssetResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved regression-asset object publishes every required field: classified cohort archetype,
/// exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, known limits, rollback target,
/// and diagnostics posture. An unclassified archetype or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn regression_asset_object_is_complete(
    archetype: M5RegressionAssetTypeKind,
    exact_build_reference: &str,
    affected_row_reference: &str,
    cohort_ring_reference: &str,
    workaround_lineage: &str,
    regression_asset_reference: &str,
    approved_exception_reference: &str,
    close_blocker_reference: &str,
) -> bool {
    archetype.is_classified()
        && !exact_build_reference.trim().is_empty()
        && !affected_row_reference.trim().is_empty()
        && !cohort_ring_reference.trim().is_empty()
        && !workaround_lineage.trim().is_empty()
        && !regression_asset_reference.trim().is_empty()
        && !approved_exception_reference.trim().is_empty()
        && !close_blocker_reference.trim().is_empty()
}

/// Whether the cohort descriptor keeps a cohort from widening without preserving its rollback and diagnostics
/// posture: the archetype must be classified, the rollback and diagnostics posture must be preserved before
/// widening, and a public-facing cohort must keep its support language matched to cohort proof. An unclassified
/// archetype, an unpreserved rollback / diagnostics posture, or partner / public support language running ahead
/// of proof never matches.
pub fn regression_asset_attributable_before_closure(
    archetype: M5RegressionAssetTypeKind,
    regression_asset_linked_before_closure: bool,
    is_severe_incident: bool,
    attributable_asset_or_approved_exception: bool,
) -> bool {
    archetype.is_classified()
        && regression_asset_linked_before_closure
        && (!is_severe_incident || attributable_asset_or_approved_exception)
}

/// Whether a incident-close packet stays honest: the scope must be classified, the evidence must be truthful,
/// it must keep the cohort evidence visible, any partner / public support language must be bound to cohort proof
/// rather than running ahead of it, and any known-limits gap must be flagged rather than masquerade as covered.
pub fn incident_close_stays_honest(
    scope: M5IncidentSeverityKind,
    close_lineage_is_truthful: bool,
    keeps_incident_lineage_visible: bool,
    close_without_asset_requested: bool,
    close_blocked_until_asset_linked: bool,
    lineage_gap_present: bool,
    lineage_gap_flagged: bool,
) -> bool {
    scope.is_classified()
        && close_lineage_is_truthful
        && keeps_incident_lineage_visible
        && (!close_without_asset_requested || close_blocked_until_asset_linked)
        && (!lineage_gap_present || lineage_gap_flagged)
}

/// Resolves a regression-asset-registry entry so it stays bound to the regression-asset registry: the entry
/// names its canonical token, semantic role, and cohort archetype, covers all three resolution forms, publishes
/// a complete descriptor object (exact repo / archetype rows, bundle IDs, install topology, toolchain envelope,
/// known limits, rollback target, diagnostics posture), preserves its rollback and diagnostics posture before
/// widening so a cohort never widens without it, and keeps a public-facing cohort's support language matched to
/// cohort proof.
pub fn resolve_regression_asset_entry(
    input: M5RegressionAssetEntryResolutionInput,
) -> Result<M5ResolvedRegressionAssetEntry, M5RegressionAssetResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5RegressionAssetResolutionError::EmptyRegressionAssetEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.asset_binding_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.exact_build_reference)
        || string_is_forbidden(&input.affected_row_reference)
        || string_is_forbidden(&input.cohort_ring_reference)
        || string_is_forbidden(&input.workaround_lineage)
        || string_is_forbidden(&input.regression_asset_reference)
        || string_is_forbidden(&input.approved_exception_reference)
        || string_is_forbidden(&input.close_blocker_reference)
    {
        return Err(M5RegressionAssetResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = regression_asset_object_is_complete(
        input.regression_asset_type,
        &input.exact_build_reference,
        &input.affected_row_reference,
        &input.cohort_ring_reference,
        &input.workaround_lineage,
        &input.regression_asset_reference,
        &input.approved_exception_reference,
        &input.close_blocker_reference,
    );
    let preserve_ok = regression_asset_attributable_before_closure(
        input.regression_asset_type,
        input.regression_asset_linked_before_closure,
        input.is_severe_incident,
        input.attributable_asset_or_approved_exception,
    );
    let support_undisclosed =
        input.is_severe_incident && !input.attributable_asset_or_approved_exception;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5RegressionAssetEntryDegradeReason::RegressionAssetTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5RegressionAssetEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.regression_asset_type.is_classified() {
        Some(M5RegressionAssetEntryDegradeReason::RegressionAssetTypeUnclassified)
    } else if !input.bound_to_registry {
        Some(M5RegressionAssetEntryDegradeReason::RegressionAssetNotBoundToRegistry)
    } else if !object_complete {
        Some(M5RegressionAssetEntryDegradeReason::RegressionAssetObjectIncomplete)
    } else if !preserve_ok {
        Some(M5RegressionAssetEntryDegradeReason::IncidentClosesWithoutRegressionAssetOrRunsClaimAheadOfProof)
    } else if !all_forms {
        Some(M5RegressionAssetEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if support_undisclosed {
        Some(M5RegressionAssetEntryDegradeReason::RegressionAssetNotAttributableForSevereIncident)
    } else if !input.proof_fresh {
        Some(M5RegressionAssetEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RegressionAssetNextAction::ExpandRegressionAssetMeaning,
    };

    Ok(M5ResolvedRegressionAssetEntry {
        entry_id: input.entry_id,
        asset_binding_id: input.asset_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        regression_asset_type: input.regression_asset_type.as_str().to_owned(),
        regression_asset_type_is_classified: input.regression_asset_type.is_classified(),
        canonical_regression_asset_type_mode: input
            .regression_asset_type
            .canonical_regression_asset_type_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        exact_build_reference: input.exact_build_reference,
        affected_row_reference: input.affected_row_reference,
        cohort_ring_reference: input.cohort_ring_reference,
        workaround_lineage: input.workaround_lineage,
        regression_asset_reference: input.regression_asset_reference,
        approved_exception_reference: input.approved_exception_reference,
        close_blocker_reference: input.close_blocker_reference,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        regression_asset_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        regression_asset_linked_before_closure: input.regression_asset_linked_before_closure,
        is_severe_incident: input.is_severe_incident,
        attributable_asset_or_approved_exception: input.attributable_asset_or_approved_exception,
        degrade_reason,
        next_action,
        regression_asset_resolves_across_types: degrade_reason.is_none(),
    })
}

/// Resolves a incident-close entry so its evidence stays safe: the entry names its canonical token,
/// semantic role, and evidence scope, covers all three resolution forms, provides the complete cohort-identity /
/// known-limits-ledger / rollback-target / rehearsal-currency / readiness-signoff / support-language /
/// last-widening-revision incident-close object, and degrades honestly when the evidence would run partner /
/// public support language ahead of cohort proof, hide the cohort evidence, or let a known-limits gap masquerade
/// as covered.
pub fn resolve_incident_close_entry(
    input: M5IncidentCloseEntryResolutionInput,
) -> Result<M5ResolvedIncidentCloseEntry, M5RegressionAssetResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5RegressionAssetResolutionError::EmptyIncidentCloseEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.incident_close_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_incident_identity)
        || string_is_forbidden(&input.linked_regression_asset_ledger)
        || string_is_forbidden(&input.exact_build_and_row_reference)
        || string_is_forbidden(&input.cohort_ring_lineage_state)
        || string_is_forbidden(&input.close_lineage_freshness_state)
        || string_is_forbidden(&input.workaround_lineage_reference)
        || string_is_forbidden(&input.last_incident_close_revision)
    {
        return Err(M5RegressionAssetResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = incident_close_stays_honest(
        input.incident_severity,
        input.close_lineage_is_truthful,
        input.keeps_incident_lineage_visible,
        input.close_without_asset_requested,
        input.close_blocked_until_asset_linked,
        input.lineage_gap_present,
        input.lineage_gap_flagged,
    );
    let provides_record = input.incident_severity.is_classified()
        && !input.resolved_incident_identity.trim().is_empty()
        && !input.linked_regression_asset_ledger.trim().is_empty()
        && !input.exact_build_and_row_reference.trim().is_empty()
        && !input.cohort_ring_lineage_state.trim().is_empty()
        && !input.close_lineage_freshness_state.trim().is_empty()
        && !input.workaround_lineage_reference.trim().is_empty()
        && !input.last_incident_close_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5IncidentCloseEntryDegradeReason::IncidentCloseTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5IncidentCloseEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.incident_severity.is_classified() {
        Some(M5IncidentCloseEntryDegradeReason::IncidentSeverityUnclassified)
    } else if !provides_record {
        Some(M5IncidentCloseEntryDegradeReason::IncidentCloseDropsLineageOrClosesWithoutRegressionAsset)
    } else if !all_forms {
        Some(M5IncidentCloseEntryDegradeReason::IncidentCloseFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5IncidentCloseEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RegressionAssetNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedIncidentCloseEntry {
        entry_id: input.entry_id,
        incident_close_ref: input.incident_close_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        incident_severity: input.incident_severity.as_str().to_owned(),
        incident_severity_is_classified: input.incident_severity.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        resolved_incident_identity: input.resolved_incident_identity,
        linked_regression_asset_ledger: input.linked_regression_asset_ledger,
        exact_build_and_row_reference: input.exact_build_and_row_reference,
        cohort_ring_lineage_state: input.cohort_ring_lineage_state,
        close_lineage_freshness_state: input.close_lineage_freshness_state,
        workaround_lineage_reference: input.workaround_lineage_reference,
        last_incident_close_revision: input.last_incident_close_revision,
        keeps_incident_lineage_visible: input.keeps_incident_lineage_visible,
        close_lineage_is_truthful: input.close_lineage_is_truthful,
        close_without_asset_requested: input.close_without_asset_requested,
        close_blocked_until_asset_linked: input.close_blocked_until_asset_linked,
        lineage_gap_present: input.lineage_gap_present,
        lineage_gap_flagged: input.lineage_gap_flagged,
        incident_close_stays_honest: record_stays_honest,
        provides_complete_incident_close_record: provides_record,
        degrade_reason,
        next_action,
        incident_close_safe_on_every_severity: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved regression-asset and incident-close
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RegressionAssetIncidentCloseRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5RegressionAssetIncidentCloseRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5RegressionAssetAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5RegressionAssetExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    /// Resolved regression-asset-registry examples.
    pub regression_asset_entries: Vec<M5ResolvedRegressionAssetEntry>,
    /// Resolved incident-close examples.
    pub incident_close_entries: Vec<M5ResolvedIncidentCloseEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the regression-asset and
    /// incident-close domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never widens a cohort without current rollback and diagnostics evidence. MUST be
    /// `false`.
    pub closes_a_severe_incident_without_a_linked_regression_asset: bool,
    /// Hard invariant: this row never runs partner or public support language ahead of cohort proof. MUST be
    /// `false`.
    pub lets_an_approved_exception_become_an_untracked_close: bool,
    /// Hard invariant: this row never hides the rollback target or diagnostics posture before widening. MUST be
    /// `false`.
    pub hides_the_build_row_or_cohort_lineage_on_the_regression_asset: bool,
    /// Hard invariant: this row never collapses distinct cohort evidence classes into one lane. MUST be `false`.
    pub collapses_distinct_incident_severity_classes_into_one_lane: bool,
}

impl M5RegressionAssetIncidentCloseRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5RegressionAssetAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5RegressionAssetAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5RegressionAssetExportField> =
            self.export_fields.iter().copied().collect();
        M5RegressionAssetExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.closes_a_severe_incident_without_a_linked_regression_asset
            && !self.lets_an_approved_exception_become_an_untracked_close
            && !self.hides_the_build_row_or_cohort_lineage_on_the_regression_asset
            && !self.collapses_distinct_incident_severity_classes_into_one_lane
    }

    /// True when a clean regression-asset entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified cohort archetype, publishes a complete descriptor object, preserves its rollback and
    /// diagnostics posture, covers all three resolution forms, and keeps a public-facing cohort's support
    /// language matched to proof.
    fn descriptor_is_honest(ex: &M5ResolvedRegressionAssetEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.regression_asset_type_is_classified
                && ex.regression_asset_object_complete
                && ex.regression_asset_linked_before_closure
                && ex.covers_all_resolution_forms
                && (!ex.is_severe_incident || ex.attributable_asset_or_approved_exception))
    }

    /// True when a clean incident-close entry preserves a safe packet: it keeps a classified evidence
    /// scope, provides the complete incident-close object, stays honest, and covers all three resolution forms.
    fn evidence_is_honest(ex: &M5ResolvedIncidentCloseEntry) -> bool {
        !ex.is_clean()
            || (ex.incident_severity_is_classified
                && ex.provides_complete_incident_close_record
                && ex.incident_close_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.regression_asset_entries
            .iter()
            .all(Self::descriptor_is_honest)
            && self
                .incident_close_entries
                .iter()
                .all(Self::evidence_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RegressionAssetIncidentCloseRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Cohort-archetype tokens (minted by this lane).
    pub regression_asset_type_kinds: Vec<String>,
    /// Evidence-scope tokens (minted by this lane).
    pub incident_severities: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Cohort-descriptor-entry degrade-reason tokens.
    pub regression_asset_degrade_reasons: Vec<String>,
    /// Cohort-evidence-packet-entry degrade-reason tokens.
    pub incident_close_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5RegressionAssetIncidentCloseRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5LaunchControlRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5RegressionAssetResolutionForm::ALL, |v| v.as_str()),
            regression_asset_type_kinds: tokens(&M5RegressionAssetTypeKind::ALL, |v| v.as_str()),
            incident_severities: tokens(&M5IncidentSeverityKind::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5RegressionAssetSurfaceContext::ALL, |v| v.as_str()),
            regression_asset_degrade_reasons: tokens(
                &M5RegressionAssetEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            incident_close_degrade_reasons: tokens(&M5IncidentCloseEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5RegressionAssetAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5RegressionAssetNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RegressionAssetExportField::ALL, |v| v.as_str()),
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
pub struct M5RegressionAssetIncidentCloseRegistriesGovernanceReview {
    /// The descriptor registry names a canonical token, semantic role, and cohort archetype for every entry.
    pub regression_asset_registry_names_token_role_and_type: bool,
    /// Every claimed cohort resolves to one typed regression-asset object from the shared registry, not
    /// per-entry reconstruction.
    pub type_resolves_to_typed_regression_asset_from_shared_registry: bool,
    /// The exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles are published for every resolved descriptor.
    pub build_row_and_cohort_lineage_published: bool,
    /// Cohorts cannot widen without preserving rollback and diagnostics posture before widening.
    pub severe_incidents_cannot_close_without_regression_asset_and_lineage: bool,
    /// The cohort evidence keeps the cohort proof visible and binds partner / public support language to it.
    pub incident_close_keeps_lineage_visible_and_blocks_assetless_close: bool,
    /// Partner / public support language stays matched to cohort proof for every public-facing cohort.
    pub approved_exception_matched_to_asset_proof_for_severe_incidents: bool,
    /// Every regression-asset and incident-close entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Cohort-descriptor and incident-close behavior stay bound to the shared registries rather than
    /// hand-copied per cohort.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shiproom, release center, executive steering, and program governance read a single cohort source.
    pub shiproom_release_center_executive_steering_and_program_governance_read_single_source: bool,
    /// A widen-without-rollback attempt, an incomplete object, or hidden cohort evidence is caught by fixtures
    /// before release evidence turns green.
    pub asset_or_close_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RegressionAssetIncidentCloseRegistriesConsumerProjection {
    /// Shiproom and release center consume the shared regression-asset registry.
    pub shiproom_and_release_center_consume_shared_registries: bool,
    /// Executive steering and program governance consume the shared incident-close registry.
    pub executive_steering_and_program_governance_consume_shared_registries: bool,
    /// Diagnostics and public proof consume the shared registries.
    pub diagnostics_and_public_proof_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical regression-asset and incident-close domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical regression-asset / incident-close registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RegressionAssetIncidentCloseRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RegressionAssetIncidentCloseRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting cohort audit for the lane.
    pub incident_control_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RegressionAssetIncidentCloseRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RegressionAssetIncidentCloseRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5RegressionAssetIncidentCloseRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RegressionAssetIncidentCloseRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RegressionAssetIncidentCloseRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RegressionAssetIncidentCloseRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RegressionAssetIncidentCloseRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RegressionAssetIncidentCloseRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 regression-asset and incident-close registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RegressionAssetIncidentCloseRegistriesPacket {
    /// Record kind; must equal [`M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5RegressionAssetIncidentCloseRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RegressionAssetIncidentCloseRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RegressionAssetIncidentCloseRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RegressionAssetIncidentCloseRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RegressionAssetIncidentCloseRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RegressionAssetIncidentCloseRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RegressionAssetIncidentCloseRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5RegressionAssetIncidentCloseRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5RegressionAssetIncidentCloseRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_RECORD_KIND {
            violations.push(M5RegressionAssetIncidentCloseRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5RegressionAssetIncidentCloseRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RegressionAssetIncidentCloseRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5RegressionAssetIncidentCloseRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 regression-asset / incident-close registries packet serializes"),
        ) {
            violations.push(M5RegressionAssetIncidentCloseRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 regression-asset / incident-close registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,regression_asset_entries,incident_close_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .regression_asset_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.incident_close_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.regression_asset_entries.len(),
                row.incident_close_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Regression-Asset and Incident-Close Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Regression asset types: {}\n",
            self.vocabulary_set.regression_asset_type_kinds.join(", ")
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
                "  - Regression-asset entries: {} / incident-close entries: {}\n",
                row.regression_asset_entries.len(),
                row.incident_close_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry cohort reference table generated from the registry, so docs and shiproom runbooks
    /// render the same archetype-mode / repo-rows / bundle-ids / install-topology / toolchain-envelope /
    /// rollback-target truth the resolvers produced rather than a hand-copied cohort table. Only clean,
    /// registry-bound regression-asset entries are listed.
    pub fn render_regression_asset_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| asset_binding_id | asset_type_mode | exact_build_reference | affected_row_reference | cohort_ring_reference | workaround_lineage | approved_exception_reference |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.regression_asset_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.asset_binding_id,
                    ex.canonical_regression_asset_type_mode,
                    ex.exact_build_reference,
                    ex.affected_row_reference,
                    ex.cohort_ring_reference,
                    ex.workaround_lineage,
                    ex.approved_exception_reference
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5RegressionAssetIncidentCloseRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RegressionAssetIncidentCloseRegistriesViolation>),
}

impl fmt::Display for M5RegressionAssetIncidentCloseRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 regression-asset / incident-close registries export parse failed: {error}"
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
                    "m5 regression-asset / incident-close registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RegressionAssetIncidentCloseRegistriesArtifactError {}

/// Validation failures emitted by [`M5RegressionAssetIncidentCloseRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RegressionAssetIncidentCloseRegistriesViolation {
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
    /// A registry row does not point at both the regression-asset and incident-close domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, widen-without-rollback, field-incomplete,
    /// form-incomplete, or a incident-close entry missing the complete evidence object).
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
    RegressionAssetResolutionNotProven,
    /// Rollback-and-diagnostics-preservation is not proven: no widen-without-rollback example and no unbound
    /// example degrade, no clean bounded descriptor entry is present, or a clean descriptor entry is unbounded
    /// or unbound.
    IncidentCloseAttributionNotProven,
    /// Cohort-evidence-integrity is not proven: clean evidence entries do not cover the canonical dogfood-ring /
    /// rehearsal-currency / go-no-go-signoff scopes with full resolution-form coverage while providing the
    /// complete evidence object, no support-ahead or form-incomplete example degrades, or a clean evidence entry
    /// is missing the complete evidence object.
    IncidentCloseIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5RegressionAssetIncidentCloseRegistriesViolation {
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
            Self::RegressionAssetResolutionNotProven => "regression_asset_resolution_not_proven",
            Self::IncidentCloseAttributionNotProven => "incident_close_attribution_not_proven",
            Self::IncidentCloseIntegrityNotProven => "incident_close_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_regression_asset_and_incident_close_registries_export() -> Result<
    M5RegressionAssetIncidentCloseRegistriesPacket,
    M5RegressionAssetIncidentCloseRegistriesArtifactError,
> {
    let packet: M5RegressionAssetIncidentCloseRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-regression-asset-and-incident-close-registries-proof/support_export.json"
        )
    ))
    .map_err(M5RegressionAssetIncidentCloseRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RegressionAssetIncidentCloseRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5RegressionAssetIncidentCloseRegistriesPacket,
    violations: &mut Vec<M5RegressionAssetIncidentCloseRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_SCHEMA_REF,
        M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_DOC_REF,
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_REGRESSION_ASSET_DOMAIN_SCHEMA_REF,
        M5_INCIDENT_CLOSE_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5RegressionAssetIncidentCloseRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5RegressionAssetIncidentCloseRegistriesPacket,
    violations: &mut Vec<M5RegressionAssetIncidentCloseRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5RegressionAssetIncidentCloseRegistriesViolation::NoRegistryRows);
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
                .push(M5RegressionAssetIncidentCloseRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations
                .push(M5RegressionAssetIncidentCloseRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5RegressionAssetIncidentCloseRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_REGRESSION_ASSET_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_INCIDENT_CLOSE_DOMAIN_SCHEMA_REF)
        {
            violations
                .push(M5RegressionAssetIncidentCloseRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.regression_asset_entries.is_empty() || row.incident_close_entries.is_empty() {
            violations.push(M5RegressionAssetIncidentCloseRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5RegressionAssetIncidentCloseRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations
                .push(M5RegressionAssetIncidentCloseRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5RegressionAssetIncidentCloseRegistriesPacket,
    violations: &mut Vec<M5RegressionAssetIncidentCloseRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.regression_asset_registry_names_token_role_and_type,
        review.type_resolves_to_typed_regression_asset_from_shared_registry,
        review.build_row_and_cohort_lineage_published,
        review.severe_incidents_cannot_close_without_regression_asset_and_lineage,
        review.incident_close_keeps_lineage_visible_and_blocks_assetless_close,
        review.approved_exception_matched_to_asset_proof_for_severe_incidents,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shiproom_release_center_executive_steering_and_program_governance_read_single_source,
        review.asset_or_close_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5RegressionAssetIncidentCloseRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RegressionAssetIncidentCloseRegistriesPacket,
    violations: &mut Vec<M5RegressionAssetIncidentCloseRegistriesViolation>,
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
                M5RegressionAssetIncidentCloseRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RegressionAssetIncidentCloseRegistriesPacket,
    violations: &mut Vec<M5RegressionAssetIncidentCloseRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations
            .push(M5RegressionAssetIncidentCloseRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5RegressionAssetIncidentCloseRegistriesPacket,
    violations: &mut Vec<M5RegressionAssetIncidentCloseRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.incident_control_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations
            .push(M5RegressionAssetIncidentCloseRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5RegressionAssetIncidentCloseRegistriesPacket,
    violations: &mut Vec<M5RegressionAssetIncidentCloseRegistriesViolation>,
) {
    let descriptors = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.regression_asset_entries.iter())
    };
    let evidence = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.incident_close_entries.iter())
    };

    // AC1: every active cohort can be inspected by exact rows, bundles, toolchains, and deployment profiles.
    // Clean descriptor entries cover the canonical cohort archetypes and the first release-center / shiproom /
    // executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no
    // clean descriptor entry published an incomplete object.
    let clean_archetypes: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.regression_asset_type.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let archetypes_covered = M5RegressionAssetTypeKind::CANONICAL_TRANSITIONS
        .iter()
        .all(|k| clean_archetypes.contains(k.as_str()));
    let first_surfaces_covered = M5RegressionAssetSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(M5RegressionAssetEntryDegradeReason::RegressionAssetObjectIncomplete)
    });
    let no_clean_incomplete =
        !descriptors().any(|ex| ex.is_clean() && !ex.regression_asset_object_complete);
    if !(archetypes_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5RegressionAssetIncidentCloseRegistriesViolation::RegressionAssetResolutionNotProven,
        );
    }

    // AC2: cohort packets preserve rollback and diagnostics posture before widening. A widen-without-rollback
    // example degrades, an unbound example degrades, at least one clean bounded descriptor entry is present, and
    // no clean descriptor entry is unbounded or unbound.
    let widen_fold_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(
                M5RegressionAssetEntryDegradeReason::IncidentClosesWithoutRegressionAssetOrRunsClaimAheadOfProof,
            )
    });
    let unbound_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(M5RegressionAssetEntryDegradeReason::RegressionAssetNotBoundToRegistry)
    });
    let bounded_clean_descriptor =
        descriptors().any(|ex| ex.is_clean() && ex.regression_asset_linked_before_closure);
    let no_clean_unbound = !descriptors().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded =
        !descriptors().any(|ex| ex.is_clean() && !ex.regression_asset_linked_before_closure);
    if !(widen_fold_degrades
        && unbound_degrades
        && bounded_clean_descriptor
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations.push(
            M5RegressionAssetIncidentCloseRegistriesViolation::IncidentCloseAttributionNotProven,
        );
    }

    // AC3: claim publication can prove which cohort evidence backs each launch-bearing lane. Clean evidence
    // entries cover every canonical dogfood-ring / rehearsal-currency / go-no-go-signoff scope with full
    // resolution-form coverage while providing the complete evidence object, a support-ahead example degrades, a
    // form-incomplete example degrades, and no clean evidence entry is missing the complete object.
    let clean_incident_severities: BTreeSet<String> = evidence()
        .filter(|ex| {
            ex.is_clean()
                && ex.incident_severity_is_classified
                && ex.provides_complete_incident_close_record
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.incident_severity.clone())
        .collect();
    let incident_severities_covered = M5IncidentSeverityKind::CANONICAL_CONDITIONS
        .iter()
        .all(|m| clean_incident_severities.contains(m.as_str()));
    let support_ahead_degrades = evidence().any(|ex| {
        ex.degrade_reason
            == Some(
                M5IncidentCloseEntryDegradeReason::IncidentCloseDropsLineageOrClosesWithoutRegressionAsset,
            )
    });
    let form_incomplete_degrades = evidence().any(|ex| {
        ex.degrade_reason
            == Some(M5IncidentCloseEntryDegradeReason::IncidentCloseFormCoverageIncomplete)
    });
    let no_clean_missing_evidence =
        !evidence().any(|ex| ex.is_clean() && !ex.provides_complete_incident_close_record);
    if !(incident_severities_covered
        && support_ahead_degrades
        && form_incomplete_degrades
        && no_clean_missing_evidence)
    {
        violations.push(
            M5RegressionAssetIncidentCloseRegistriesViolation::IncidentCloseIntegrityNotProven,
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

/// The widening stages this lane governs, for downstream reference: the regression-asset registry defines the
/// minimum evidence and soak expectations that let a lane advance across the alpha, beta, release-candidate,
/// stable, and long-term-support widening stages, and the incident-close registry records the conditions that
/// immediately stop that progression.
pub const IMPLEMENTED_REGRESSION_STAGES: [M5LaunchControlWideningStage; 5] =
    M5LaunchControlWideningStage::ALL;
