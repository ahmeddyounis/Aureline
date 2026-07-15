//! Implemented M5 operational-readiness-review (ORR) and rehearsal-drill registries.
//!
//! The frozen [launch-control matrix][matrix] names Aureline's governed launch-bearing lanes and locks their
//! controlled vocabulary. This module governs *exercising launch-bearing lanes before widening rather than
//! discovering gaps in production*: it turns the *ORR / rehearsal-packet* grammar (how each packet kind — a
//! monthly ORR, a release-candidate ORR, a publish/rollback drill, a mixed-version drill, an advisory/revocation
//! drill, or a support/incident handoff drill — names its readiness scope, its release / advisory / support-room
//! / docs-comms / backup-signer role roster, and its rehearsal-freshness expiry so a stable claim can never widen
//! on a stale, skipped, or contradictory rehearsal packet) and the *rehearsal-drill readiness* grammar (how a
//! launch-bearing lane records the roster coverage — a full roster, a backup roster, or a conditional roster —
//! with the preserved ORR signoff, the named on-call roster state, the rehearsal-freshness state, and the
//! authorized widening stage that justified widening) into registry resolvers that produce export-safe, honest
//! projections. Every claimed launch-bearing lane then resolves to one typed ORR-packet object — the packet kind
//! it classifies, the readiness scope, the release / advisory / support-room / docs-comms / backup-signer roster,
//! and the rehearsal-freshness expiry, all current before widening so a lane never widens on a stale rehearsal
//! packet and so partner / public support language never outruns current rehearsal proof — and to one
//! rehearsal-drill object — the resolved coverage identity, the rehearsal evidence ledger, the ORR-signoff
//! reference, the named on-call roster state, the rehearsal-freshness state, the authorized widening stage, and
//! the last rehearsal-drill revision — that the shiproom, release-center, executive-steering,
//! program-governance, and support / export surfaces can inspect without manual reconstruction, so every claimed
//! launch-bearing lane points at current ORR and rehearsal packets, rehearsal freshness and role coverage read as
//! first-class shiproom and release blockers, stable/LTS promotion halts automatically when a lane's rehearsal
//! state is red or stale, and a lane that cannot show the rehearsal packet it ran or the roster that covered it
//! degrades honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one typed ORR-packet object per packet kind.** [`resolve_orr_review_entry`]
//!   refuses to read as a clean, registry-bound readiness entry unless it names a canonical registry token, a
//!   classified [packet kind][M5OrrReviewPacketKind], a launch-control role, covers every
//!   [resolution form][M5OrrReviewResolutionForm] (the canonical object, the accessible summary, and the audit
//!   record), publishes every readiness field (readiness scope, release / advisory / support-room / docs-comms /
//!   backup-signer roster, and rehearsal-freshness expiry), keeps its rehearsal packet current before widening,
//!   and keeps partner / public support language matched to rehearsal proof; otherwise it degrades.
//! * **Fail widening when a rehearsal packet is stale, skipped, or missing roster coverage.**
//!   [`orr_review_stays_documented_before_widening`] rejects a readiness entry whose rehearsal packet is not
//!   current before widening (a lane widening on a stale or skipped rehearsal packet) so it degrades to
//!   [`M5OrrReviewEntryDegradeReason::OrrReviewWidensScopeUndocumentedOrRunsClaimAheadOfProof`],
//!   and a public-facing lane whose support language runs ahead of rehearsal proof degrades the same way — the
//!   structured blocker reason a widen-on-stale-rehearsal attempt must surface.
//! * **Keep the rehearsal-drill record from implying green while rehearsal state is stale.**
//!   [`resolve_rehearsal_drill_entry`] names a classified [roster coverage][M5RehearsalDrillCoverageKind],
//!   requires the full resolved-coverage-identity / rehearsal-evidence-ledger / ORR-signoff / on-call-roster /
//!   rehearsal-freshness / widening-stage / last-rehearsal-drill-revision record, covers every resolution form,
//!   and degrades to
//!   [`M5RehearsalDrillEntryDegradeReason::RehearsalDrillDropsEvidenceOrImpliesGreenWhileStale`]
//!   when the record would imply green while the rehearsal packet is stale, hide the rehearsal evidence, or let a
//!   roster-coverage gap masquerade as covered, so a rehearsal-drill record can never read as trustworthy when it
//!   has quietly dropped the reason a lane's widening is actually gated.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5LaunchControlRole`] role vocabulary and
//! the [`M5LaunchControlConsumerSurface`] consumer-surface taxonomy — so the shiproom, release-center,
//! executive-steering, program-governance, diagnostics, docs, CLI, support, and public-proof surfaces can never
//! fork their own launch-control meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_launch_control_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_orr_review_and_rehearsal_drill_registries,
    seeded_m5_orr_review_and_rehearsal_drill_registries_orr_review_beta_narrowed,
    seeded_m5_orr_review_and_rehearsal_drill_registries_rehearsal_drill_preview_narrowed,
    M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_PACKET_ID,
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

/// Stable record-kind tag carried by [`M5OrrReviewRehearsalDrillRegistriesPacket`].
pub const M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_orr_review_and_rehearsal_drill_registries";

/// Schema version for M5 orr-review / rehearsal-drill registry records.
pub const M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_SCHEMA_REF: &str =
    "schemas/program/m5-orr-review-and-rehearsal-drill-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_DOC_REF: &str =
    "docs/release/m5_orr_review_and_rehearsal_drill_registries.md";

/// Repo-relative path of the canonical orr-review domain schema minted by this lane (how a widening ring
/// transition declares its minimum entry evidence, soak-window expectation, why widening is allowed, its
/// known-limits packet, issue-template linkage, claim-narrowing action, and the rehearsal-drill reference that
/// immediately stops it).
pub const M5_ORR_REVIEW_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-orr-review-packet.schema.json";

/// Repo-relative path of the canonical rehearsal-drill domain schema minted by this lane (how a launch-bearing lane
/// records the rehearsal-drill condition — a crash / data-loss / trust defect, a repeated protected-metric
/// regression, or a stale readiness packet — that halts regression asset while it is active).
pub const M5_REHEARSAL_DRILL_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-rehearsal-drill.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-orr-review-and-rehearsal-drill-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-orr-review-and-rehearsal-drill-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-orr-review-and-rehearsal-drill-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/release/m5-orr-review-and-rehearsal-drill-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// cohort invents a parallel surface set.
pub type M5OrrReviewRehearsalDrillRegistriesConsumerSurface = M5LaunchControlConsumerSurface;

/// One of the three resolution forms every orr-review or rehearsal-drill entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// orr-review and rehearsal-drill *domains* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OrrReviewResolutionForm {
    /// The canonical resolved orr-review / rehearsal-drill object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved cohort discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved cohort inspectable off-renderer.
    AuditRecord,
}

impl M5OrrReviewResolutionForm {
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

/// Controlled cohort archetype a orr-review entry classifies, so the typed descriptor model shares one
/// registry rather than a hand-copied per-cohort assumption. Minted by this lane because the frozen matrix
/// carries the launch-bearing cohorts but distinguishes the dogfood / migration-alpha / extension-author /
/// design-partner / public-preview / certified-archetype archetypes an auditable descriptor classifies against
/// explicitly. Every classified archetype carries its canonical mode, and the design-partner-preview and
/// public-preview archetypes are public-facing so their partner / public support language must stay matched to
/// cohort proof before the cohort widens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OrrReviewPacketKind {
    /// The internal dogfood core-team canary cohort.
    MonthlyOrrPacket,
    /// The migration alpha cohort (external alpha migrating from a prior toolchain).
    ReleaseCandidateOrrPacket,
    /// The extension-author cohort (compatibility rehearsals current, freeze exceptions documented).
    PublishRollbackDrill,
    /// The design-partner preview cohort (public-facing; support language must match cohort proof).
    MixedVersionDrill,
    /// The public preview cohort (public-facing; support language must match cohort proof).
    AdvisoryRevocationDrill,
    /// The certified-archetype cohort (ORR signed and a go/no-go coverage recorded).
    SupportIncidentHandoffDrill,
    /// The cohort archetype is unclassified, which is disallowed.
    PacketKindUnclassified,
}

impl M5OrrReviewPacketKind {
    /// Every cohort archetype, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::MonthlyOrrPacket,
        Self::ReleaseCandidateOrrPacket,
        Self::PublishRollbackDrill,
        Self::MixedVersionDrill,
        Self::AdvisoryRevocationDrill,
        Self::SupportIncidentHandoffDrill,
        Self::PacketKindUnclassified,
    ];

    /// The six canonical cohort archetypes every claimed M5 launch-bearing cohort classifies against.
    pub const CANONICAL_PACKET_KINDS: [Self; 6] = [
        Self::MonthlyOrrPacket,
        Self::ReleaseCandidateOrrPacket,
        Self::PublishRollbackDrill,
        Self::MixedVersionDrill,
        Self::AdvisoryRevocationDrill,
        Self::SupportIncidentHandoffDrill,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MonthlyOrrPacket => "monthly_orr_packet",
            Self::ReleaseCandidateOrrPacket => "release_candidate_orr_packet",
            Self::PublishRollbackDrill => "publish_rollback_drill",
            Self::MixedVersionDrill => "mixed_version_drill",
            Self::AdvisoryRevocationDrill => "advisory_revocation_drill",
            Self::SupportIncidentHandoffDrill => "support_incident_handoff_drill",
            Self::PacketKindUnclassified => "packet_kind_unclassified",
        }
    }

    /// Whether the archetype is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::PacketKindUnclassified)
    }

    /// The canonical mode for this cohort archetype.
    pub const fn canonical_orr_review_packet_kind_mode(self) -> &'static str {
        match self {
            Self::MonthlyOrrPacket => "monthly_orr_packet_kind",
            Self::ReleaseCandidateOrrPacket => "release_candidate_orr_packet_kind",
            Self::PublishRollbackDrill => "publish_rollback_drill_kind",
            Self::MixedVersionDrill => "mixed_version_drill_kind",
            Self::AdvisoryRevocationDrill => "advisory_revocation_drill_kind",
            Self::SupportIncidentHandoffDrill => "support_incident_handoff_drill_kind",
            Self::PacketKindUnclassified => "",
        }
    }

    /// Whether this archetype is public-facing and so must keep partner / public support language matched to
    /// cohort proof before the cohort widens.
    pub const fn requires_documented_exception(self) -> bool {
        matches!(
            self,
            Self::MixedVersionDrill | Self::AdvisoryRevocationDrill
        )
    }
}

/// Controlled evidence scope a rehearsal-drill entry must resolve its cohort proof from, so an evidence
/// packet shares one registry rather than a hand-copied per-record assumption. Minted by this lane, tracking
/// whether the evidence came from dogfood-ring telemetry, current rehearsal cadence, or an explicit go/no-go
/// signoff the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RehearsalDrillCoverageKind {
    /// The evidence came from internal dogfood-ring telemetry.
    FullRosterCoverage,
    /// The evidence came from current rehearsal cadence (publish/rollback, mixed-version, handoff drills).
    BackupRosterCoverage,
    /// The evidence came from an explicit go/no-go signoff with a preserved evidence snapshot.
    ConditionalRosterCoverage,
    /// The evidence scope is unclassified, which is disallowed.
    CoverageUnclassified,
}

impl M5RehearsalDrillCoverageKind {
    /// Every evidence scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullRosterCoverage,
        Self::BackupRosterCoverage,
        Self::ConditionalRosterCoverage,
        Self::CoverageUnclassified,
    ];

    /// The three canonical evidence scopes every rehearsal-drill packet must stay distinct across.
    pub const CANONICAL_COVERAGES: [Self; 3] = [
        Self::FullRosterCoverage,
        Self::BackupRosterCoverage,
        Self::ConditionalRosterCoverage,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullRosterCoverage => "full_roster_coverage",
            Self::BackupRosterCoverage => "backup_roster_coverage",
            Self::ConditionalRosterCoverage => "conditional_roster_coverage",
            Self::CoverageUnclassified => "coverage_unclassified",
        }
    }

    /// Whether the evidence scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::CoverageUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a orr-review or
/// rehearsal-drill token's meaning stays stable whether it appears in the release-center, shiproom,
/// executive-steering, program-governance, or a support / export form. Minted by this lane, tracking the
/// first-consumer surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OrrReviewSurfaceContext {
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

impl M5OrrReviewSurfaceContext {
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

/// One mandatory rendered part a orr-review or rehearsal-drill entry must be able to show, so no
/// cohort archetype, repo / bundle / toolchain / deployment row, known-limits packet, rollback target,
/// rehearsal-drill field, or registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OrrReviewAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The cohort archetype the entry classifies (orr-review entry).
    OrrReviewType,
    /// The exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles the entry publishes (orr-review entry).
    IncidentLineageRows,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The known-limits packet and rollback target the entry preserves before widening (orr-review
    /// entry).
    BuildAndCohortLineage,
    /// The rehearsal-drill fields (cohort identity, known-limits ledger, rollback target, rehearsal currency,
    /// readiness signoff, support language) the entry publishes (rehearsal-drill entry).
    RehearsalDrillFields,
    /// The support-identity hint the entry publishes (rehearsal-drill entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved cohort descriptor or cohort evidence (both entries).
    PlainLanguageMeaning,
}

impl M5OrrReviewAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::OrrReviewType,
        Self::IncidentLineageRows,
        Self::ResolutionFormCoverage,
        Self::BuildAndCohortLineage,
        Self::RehearsalDrillFields,
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
            Self::OrrReviewType => "orr_review_packet_kind",
            Self::IncidentLineageRows => "incident_lineage_rows",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::BuildAndCohortLineage => "build_and_cohort_lineage",
            Self::RehearsalDrillFields => "rehearsal_drill_fields",
            Self::SupportIdentityHint => "support_identity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// cohort descriptor, a rehearsal-drill packet, or a degraded orr-review / rehearsal-drill entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OrrReviewNextAction {
    /// Expand the resolved cohort descriptor's or rehearsal-drill packet's plain-language meaning.
    ExpandOrrReviewMeaning,
    /// Inspect the cohort archetype or evidence scope the entry resolves.
    InspectPacketKindOrCoverage,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5OrrReviewNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandOrrReviewMeaning,
        Self::InspectPacketKindOrCoverage,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandOrrReviewMeaning => "expand_orr_review_meaning",
            Self::InspectPacketKindOrCoverage => "inspect_packet_kind_or_coverage",
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
pub enum M5OrrReviewExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The cohort families covered.
    OrrReviewFamilies,
    /// The cohort archetypes carried.
    OrrReviewPacketKinds,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The evidence scopes carried.
    RehearsalDrillCoverages,
    /// The render / surface context.
    SurfaceContext,
    /// The cohort-archetype modes carried.
    OrrReviewPacketKindModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5OrrReviewExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::OrrReviewFamilies,
        Self::OrrReviewPacketKinds,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::RehearsalDrillCoverages,
        Self::SurfaceContext,
        Self::OrrReviewPacketKindModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::OrrReviewFamilies,
        Self::OrrReviewPacketKinds,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::OrrReviewFamilies => "orr_review_families",
            Self::OrrReviewPacketKinds => "orr_review_packet_kinds",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::RehearsalDrillCoverages => "rehearsal_drill_coverages",
            Self::SurfaceContext => "surface_context",
            Self::OrrReviewPacketKindModes => "orr_review_packet_kind_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a orr-review entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, widen-without-rollback, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OrrReviewEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the descriptor means.
    OrrReviewTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The cohort archetype is unclassified (not in the resolved taxonomy).
    OrrReviewPacketKindUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    OrrReviewNotBoundToRegistry,
    /// The resolved orr-review object is incomplete: the exact repo / archetype rows, bundle IDs, install
    /// topology, toolchain envelope, known limits, rollback target, or diagnostics posture is unstated.
    OrrReviewObjectIncomplete,
    /// The cohort's rollback and diagnostics posture is not preserved before widening (a cohort widening without
    /// a rollback target and diagnostics posture), or a public-facing cohort ran its support language ahead of
    /// cohort proof.
    OrrReviewWidensScopeUndocumentedOrRunsClaimAheadOfProof,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A public-facing cohort did not keep its support language matched to cohort proof before widening.
    OrrReviewUndocumentedForScopeWidening,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5OrrReviewEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::OrrReviewTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::OrrReviewPacketKindUnclassified,
        Self::OrrReviewNotBoundToRegistry,
        Self::OrrReviewObjectIncomplete,
        Self::OrrReviewWidensScopeUndocumentedOrRunsClaimAheadOfProof,
        Self::ResolutionFormCoverageIncomplete,
        Self::OrrReviewUndocumentedForScopeWidening,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrrReviewTokenUnstated => "orr_review_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::OrrReviewPacketKindUnclassified => "orr_review_packet_kind_unclassified",
            Self::OrrReviewNotBoundToRegistry => "orr_review_not_bound_to_registry",
            Self::OrrReviewObjectIncomplete => "orr_review_object_incomplete",
            Self::OrrReviewWidensScopeUndocumentedOrRunsClaimAheadOfProof => {
                "orr_review_widens_scope_undocumented_or_runs_claim_ahead_of_proof"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::OrrReviewUndocumentedForScopeWidening => {
                "orr_review_undocumented_for_scope_widening"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5OrrReviewNextAction {
        match self {
            Self::OrrReviewTokenUnstated | Self::OrrReviewNotBoundToRegistry => {
                M5OrrReviewNextAction::TraceCanonicalRegistry
            }
            Self::OrrReviewPacketKindUnclassified
            | Self::OrrReviewObjectIncomplete
            | Self::OrrReviewWidensScopeUndocumentedOrRunsClaimAheadOfProof => {
                M5OrrReviewNextAction::InspectPacketKindOrCoverage
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5OrrReviewNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::OrrReviewUndocumentedForScopeWidening
            | Self::ProofStale => M5OrrReviewNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5LaunchControlDowngradeTrigger {
        match self {
            Self::OrrReviewTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::OrrReviewNotBoundToRegistry => {
                M5LaunchControlDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::OrrReviewPacketKindUnclassified | Self::OrrReviewObjectIncomplete => {
                M5LaunchControlDowngradeTrigger::CohortMembershipUnstated
            }
            Self::OrrReviewWidensScopeUndocumentedOrRunsClaimAheadOfProof
            | Self::OrrReviewUndocumentedForScopeWidening => {
                M5LaunchControlDowngradeTrigger::WidenedWithoutCurrentCohortEvidence
            }
            Self::ProofStale => M5LaunchControlDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a rehearsal-drill entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RehearsalDrillEntryDegradeReason {
    /// The canonical registry token name is unstated.
    RehearsalDrillTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The evidence scope is unclassified (not in the resolved taxonomy).
    RehearsalDrillCoverageUnclassified,
    /// The cohort evidence would run partner / public support language ahead of cohort proof, hide the cohort
    /// evidence, let a known-limits gap masquerade as covered, or it dropped one of the required rehearsal-drill
    /// fields (cohort identity, known-limits ledger, rollback target, rehearsal currency, readiness signoff,
    /// support language, last widening revision).
    RehearsalDrillDropsEvidenceOrImpliesGreenWhileStale,
    /// The canonical / accessible / audit resolution-form coverage of the evidence is incomplete.
    RehearsalDrillFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RehearsalDrillEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RehearsalDrillTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::RehearsalDrillCoverageUnclassified,
        Self::RehearsalDrillDropsEvidenceOrImpliesGreenWhileStale,
        Self::RehearsalDrillFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RehearsalDrillTokenUnstated => "rehearsal_drill_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::RehearsalDrillCoverageUnclassified => "rehearsal_drill_coverage_unclassified",
            Self::RehearsalDrillDropsEvidenceOrImpliesGreenWhileStale => {
                "rehearsal_drill_drops_evidence_or_implies_green_while_stale"
            }
            Self::RehearsalDrillFormCoverageIncomplete => {
                "rehearsal_drill_form_coverage_incomplete"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5OrrReviewNextAction {
        match self {
            Self::RehearsalDrillTokenUnstated => M5OrrReviewNextAction::TraceCanonicalRegistry,
            Self::RehearsalDrillCoverageUnclassified
            | Self::RehearsalDrillDropsEvidenceOrImpliesGreenWhileStale => {
                M5OrrReviewNextAction::InspectPacketKindOrCoverage
            }
            Self::RehearsalDrillFormCoverageIncomplete => {
                M5OrrReviewNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5OrrReviewNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5LaunchControlDowngradeTrigger {
        match self {
            Self::RehearsalDrillTokenUnstated => {
                M5LaunchControlDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::RehearsalDrillCoverageUnclassified => {
                M5LaunchControlDowngradeTrigger::ReadinessStateUnstated
            }
            Self::RehearsalDrillDropsEvidenceOrImpliesGreenWhileStale => {
                M5LaunchControlDowngradeTrigger::RanPartnerOrPublicLanguageAheadOfCohortProof
            }
            Self::RehearsalDrillFormCoverageIncomplete => {
                M5LaunchControlDowngradeTrigger::ImpliedGreenWhileGoNoGoOrOrrWasStale
            }
            Self::ProofStale => M5LaunchControlDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_orr_review_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5OrrReviewEntryResolutionInput {
    /// Stable identity of the orr-review-registry entry.
    pub entry_id: String,
    /// The stable cohort-binding ID this descriptor binds to (e.g. `incident.lane.public-preview`); empty means
    /// unstated.
    pub orr_packet_binding_id: String,
    /// The canonical registry token name (e.g. `freeze.exception.advisory_revocation_drill`); empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5LaunchControlRole,
    /// The cohort archetype this entry classifies.
    pub orr_review_packet_kind: M5OrrReviewPacketKind,
    /// The render / surface context.
    pub surface_context: M5OrrReviewSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5OrrReviewResolutionForm>,
    /// The published exact repo / archetype rows; empty means unstated.
    pub readiness_scope_reference: String,
    /// The published bundle IDs; empty means unstated.
    pub release_owner_reference: String,
    /// The published install topology; empty means unstated.
    pub advisory_owner_reference: String,
    /// The published toolchain envelope; empty means unstated.
    pub support_room_owner_reference: String,
    /// The published known limits; empty means unstated.
    pub docs_comms_owner_reference: String,
    /// The published rollback target; empty means unstated.
    pub backup_signer_reference: String,
    /// The published diagnostics posture; empty means unstated.
    pub rehearsal_freshness_reference: String,
    /// True when the behavior traces to the orr-review registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the cohort's rollback and diagnostics posture is preserved before widening (a hard invariant
    /// when `false`).
    pub orr_review_documented_before_widening: bool,
    /// True when this cohort's archetype is public-facing.
    pub requires_documented_exception: bool,
    /// True when partner / public support language is matched to cohort proof before a public-facing cohort
    /// widens.
    pub attributable_asset_or_approved_exception: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe orr-review-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedOrrReviewEntry {
    /// Stable identity of the orr-review-registry entry.
    pub entry_id: String,
    /// The stable cohort-binding ID this descriptor binds to.
    pub orr_packet_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the evidence snapshot and signoff before widening.
    pub semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: bool,
    /// The cohort-archetype token named by the entry.
    pub orr_review_packet_kind: String,
    /// Whether the cohort archetype is classified into the resolved taxonomy.
    pub orr_review_packet_kind_is_classified: bool,
    /// The canonical mode for the entry's cohort archetype.
    pub canonical_orr_review_packet_kind_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published exact repo / archetype rows.
    pub readiness_scope_reference: String,
    /// The published bundle IDs.
    pub release_owner_reference: String,
    /// The published install topology.
    pub advisory_owner_reference: String,
    /// The published toolchain envelope.
    pub support_room_owner_reference: String,
    /// The published known limits.
    pub docs_comms_owner_reference: String,
    /// The published rollback target.
    pub backup_signer_reference: String,
    /// The published diagnostics posture.
    pub rehearsal_freshness_reference: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved orr-review object publishes every required field.
    pub orr_review_object_complete: bool,
    /// Whether the entry traces to the orr-review registry.
    pub bound_to_registry: bool,
    /// Whether the cohort's rollback and diagnostics posture stays preserved before widening.
    pub orr_review_documented_before_widening: bool,
    /// Whether this cohort's archetype is public-facing.
    pub requires_documented_exception: bool,
    /// Whether partner / public support language is matched to cohort proof before widening.
    pub attributable_asset_or_approved_exception: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5OrrReviewEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5OrrReviewNextAction,
    /// Whether the descriptor resolves to one typed object across every claimed cohort (clean entry naming every
    /// fact).
    pub orr_review_resolves_across_classes: bool,
}

impl M5ResolvedOrrReviewEntry {
    /// Whether this orr-review entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_rehearsal_drill_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RehearsalDrillEntryResolutionInput {
    /// Stable identity of the rehearsal-drill entry.
    pub entry_id: String,
    /// The stable evidence-ref this record binds to; empty means unstated.
    pub rehearsal_drill_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5LaunchControlRole,
    /// The evidence scope this record must resolve its cohort proof from.
    pub rehearsal_drill_coverage: M5RehearsalDrillCoverageKind,
    /// The render / surface context.
    pub surface_context: M5OrrReviewSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5OrrReviewResolutionForm>,
    /// The published resolved cohort identity; empty means missing.
    pub resolved_coverage_identity: String,
    /// The published known-limits ledger; empty means missing.
    pub evidence_snapshot_ledger: String,
    /// The published rollback-target reference; empty means missing.
    pub orr_signoff_reference: String,
    /// The published rehearsal-currency state; empty means missing.
    pub on_call_roster_state: String,
    /// The published readiness-signoff state; empty means missing.
    pub rehearsal_drill_freshness_state: String,
    /// The published cohort-bound support-language reference; empty means missing.
    pub widening_stage_reference: String,
    /// The published last widening revision; empty means missing.
    pub last_rehearsal_drill_revision: String,
    /// True when the record keeps the cohort evidence visible.
    pub keeps_evidence_snapshot_visible: bool,
    /// True when the evidence is truthful (never claims a clean packet over hidden cohort evidence).
    pub rehearsal_drill_lineage_is_truthful: bool,
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

/// Resolved, export-safe rehearsal-drill projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRehearsalDrillEntry {
    /// Stable identity of the rehearsal-drill entry.
    pub entry_id: String,
    /// The stable evidence-ref this record binds to.
    pub rehearsal_drill_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the evidence snapshot and signoff before widening.
    pub semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: bool,
    /// The evidence-scope token named by the entry.
    pub rehearsal_drill_coverage: String,
    /// Whether the evidence scope is classified into the resolved taxonomy.
    pub rehearsal_drill_coverage_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published resolved cohort identity.
    pub resolved_coverage_identity: String,
    /// The published known-limits ledger.
    pub evidence_snapshot_ledger: String,
    /// The published rollback-target reference.
    pub orr_signoff_reference: String,
    /// The published rehearsal-currency state.
    pub on_call_roster_state: String,
    /// The published readiness-signoff state.
    pub rehearsal_drill_freshness_state: String,
    /// The published cohort-bound support-language reference.
    pub widening_stage_reference: String,
    /// The published last widening revision.
    pub last_rehearsal_drill_revision: String,
    /// Whether the record keeps the cohort evidence visible.
    pub keeps_evidence_snapshot_visible: bool,
    /// Whether the evidence is truthful.
    pub rehearsal_drill_lineage_is_truthful: bool,
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
    pub rehearsal_drill_stays_honest: bool,
    /// Whether the entry provides the complete rehearsal-drill object (cohort identity, known-limits ledger,
    /// rollback target, rehearsal currency, readiness signoff, support language, last widening revision).
    pub provides_complete_rehearsal_drill_record: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5RehearsalDrillEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5OrrReviewNextAction,
    /// Whether the cohort evidence is safe on every claimed cohort (clean entry naming every fact).
    pub rehearsal_drill_safe_on_every_coverage: bool,
}

impl M5ResolvedRehearsalDrillEntry {
    /// Whether this rehearsal-drill entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5OrrReviewResolutionError {
    /// The orr-review-entry id was empty.
    EmptyOrrReviewEntryId,
    /// The rehearsal-drill-entry id was empty.
    EmptyRehearsalDrillEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5OrrReviewResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyOrrReviewEntryId => "empty_orr_review_entry_id",
            Self::EmptyRehearsalDrillEntryId => "empty_rehearsal_drill_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5OrrReviewResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 orr-review / rehearsal-drill registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5OrrReviewResolutionError {}

fn form_tokens(forms: &[M5OrrReviewResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5OrrReviewResolutionForm]) -> bool {
    let present: BTreeSet<M5OrrReviewResolutionForm> = forms.iter().copied().collect();
    M5OrrReviewResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved orr-review object publishes every required field: classified cohort archetype,
/// exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, known limits, rollback target,
/// and diagnostics posture. An unclassified archetype or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn orr_review_object_is_complete(
    archetype: M5OrrReviewPacketKind,
    readiness_scope_reference: &str,
    release_owner_reference: &str,
    advisory_owner_reference: &str,
    support_room_owner_reference: &str,
    docs_comms_owner_reference: &str,
    backup_signer_reference: &str,
    rehearsal_freshness_reference: &str,
) -> bool {
    archetype.is_classified()
        && !readiness_scope_reference.trim().is_empty()
        && !release_owner_reference.trim().is_empty()
        && !advisory_owner_reference.trim().is_empty()
        && !support_room_owner_reference.trim().is_empty()
        && !docs_comms_owner_reference.trim().is_empty()
        && !backup_signer_reference.trim().is_empty()
        && !rehearsal_freshness_reference.trim().is_empty()
}

/// Whether the cohort descriptor keeps a cohort from widening without preserving its rollback and diagnostics
/// posture: the archetype must be classified, the rollback and diagnostics posture must be preserved before
/// widening, and a public-facing cohort must keep its support language matched to cohort proof. An unclassified
/// archetype, an unpreserved rollback / diagnostics posture, or partner / public support language running ahead
/// of proof never matches.
pub fn orr_review_stays_documented_before_widening(
    archetype: M5OrrReviewPacketKind,
    orr_review_documented_before_widening: bool,
    requires_documented_exception: bool,
    attributable_asset_or_approved_exception: bool,
) -> bool {
    archetype.is_classified()
        && orr_review_documented_before_widening
        && (!requires_documented_exception || attributable_asset_or_approved_exception)
}

/// Whether a rehearsal-drill packet stays honest: the scope must be classified, the evidence must be truthful,
/// it must keep the cohort evidence visible, any partner / public support language must be bound to cohort proof
/// rather than running ahead of it, and any known-limits gap must be flagged rather than masquerade as covered.
pub fn rehearsal_drill_stays_honest(
    scope: M5RehearsalDrillCoverageKind,
    rehearsal_drill_lineage_is_truthful: bool,
    keeps_evidence_snapshot_visible: bool,
    override_without_evidence_requested: bool,
    blocked_until_evidence_linked: bool,
    lineage_gap_present: bool,
    lineage_gap_flagged: bool,
) -> bool {
    scope.is_classified()
        && rehearsal_drill_lineage_is_truthful
        && keeps_evidence_snapshot_visible
        && (!override_without_evidence_requested || blocked_until_evidence_linked)
        && (!lineage_gap_present || lineage_gap_flagged)
}

/// Resolves a orr-review-registry entry so it stays bound to the orr-review registry: the entry
/// names its canonical token, semantic role, and cohort archetype, covers all three resolution forms, publishes
/// a complete descriptor object (exact repo / archetype rows, bundle IDs, install topology, toolchain envelope,
/// known limits, rollback target, diagnostics posture), preserves its rollback and diagnostics posture before
/// widening so a cohort never widens without it, and keeps a public-facing cohort's support language matched to
/// cohort proof.
pub fn resolve_orr_review_entry(
    input: M5OrrReviewEntryResolutionInput,
) -> Result<M5ResolvedOrrReviewEntry, M5OrrReviewResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5OrrReviewResolutionError::EmptyOrrReviewEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.orr_packet_binding_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.readiness_scope_reference)
        || string_is_forbidden(&input.release_owner_reference)
        || string_is_forbidden(&input.advisory_owner_reference)
        || string_is_forbidden(&input.support_room_owner_reference)
        || string_is_forbidden(&input.docs_comms_owner_reference)
        || string_is_forbidden(&input.backup_signer_reference)
        || string_is_forbidden(&input.rehearsal_freshness_reference)
    {
        return Err(M5OrrReviewResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = orr_review_object_is_complete(
        input.orr_review_packet_kind,
        &input.readiness_scope_reference,
        &input.release_owner_reference,
        &input.advisory_owner_reference,
        &input.support_room_owner_reference,
        &input.docs_comms_owner_reference,
        &input.backup_signer_reference,
        &input.rehearsal_freshness_reference,
    );
    let preserve_ok = orr_review_stays_documented_before_widening(
        input.orr_review_packet_kind,
        input.orr_review_documented_before_widening,
        input.requires_documented_exception,
        input.attributable_asset_or_approved_exception,
    );
    let support_undisclosed =
        input.requires_documented_exception && !input.attributable_asset_or_approved_exception;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5OrrReviewEntryDegradeReason::OrrReviewTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5OrrReviewEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.orr_review_packet_kind.is_classified() {
        Some(M5OrrReviewEntryDegradeReason::OrrReviewPacketKindUnclassified)
    } else if !input.bound_to_registry {
        Some(M5OrrReviewEntryDegradeReason::OrrReviewNotBoundToRegistry)
    } else if !object_complete {
        Some(M5OrrReviewEntryDegradeReason::OrrReviewObjectIncomplete)
    } else if !preserve_ok {
        Some(M5OrrReviewEntryDegradeReason::OrrReviewWidensScopeUndocumentedOrRunsClaimAheadOfProof)
    } else if !all_forms {
        Some(M5OrrReviewEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if support_undisclosed {
        Some(M5OrrReviewEntryDegradeReason::OrrReviewUndocumentedForScopeWidening)
    } else if !input.proof_fresh {
        Some(M5OrrReviewEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5OrrReviewNextAction::ExpandOrrReviewMeaning,
    };

    Ok(M5ResolvedOrrReviewEntry {
        entry_id: input.entry_id,
        orr_packet_binding_id: input.orr_packet_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        orr_review_packet_kind: input.orr_review_packet_kind.as_str().to_owned(),
        orr_review_packet_kind_is_classified: input.orr_review_packet_kind.is_classified(),
        canonical_orr_review_packet_kind_mode: input
            .orr_review_packet_kind
            .canonical_orr_review_packet_kind_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        readiness_scope_reference: input.readiness_scope_reference,
        release_owner_reference: input.release_owner_reference,
        advisory_owner_reference: input.advisory_owner_reference,
        support_room_owner_reference: input.support_room_owner_reference,
        docs_comms_owner_reference: input.docs_comms_owner_reference,
        backup_signer_reference: input.backup_signer_reference,
        rehearsal_freshness_reference: input.rehearsal_freshness_reference,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        orr_review_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        orr_review_documented_before_widening: input.orr_review_documented_before_widening,
        requires_documented_exception: input.requires_documented_exception,
        attributable_asset_or_approved_exception: input.attributable_asset_or_approved_exception,
        degrade_reason,
        next_action,
        orr_review_resolves_across_classes: degrade_reason.is_none(),
    })
}

/// Resolves a rehearsal-drill entry so its evidence stays safe: the entry names its canonical token,
/// semantic role, and evidence scope, covers all three resolution forms, provides the complete cohort-identity /
/// known-limits-ledger / rollback-target / rehearsal-currency / readiness-signoff / support-language /
/// last-widening-revision rehearsal-drill object, and degrades honestly when the evidence would run partner /
/// public support language ahead of cohort proof, hide the cohort evidence, or let a known-limits gap masquerade
/// as covered.
pub fn resolve_rehearsal_drill_entry(
    input: M5RehearsalDrillEntryResolutionInput,
) -> Result<M5ResolvedRehearsalDrillEntry, M5OrrReviewResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5OrrReviewResolutionError::EmptyRehearsalDrillEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.rehearsal_drill_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_coverage_identity)
        || string_is_forbidden(&input.evidence_snapshot_ledger)
        || string_is_forbidden(&input.orr_signoff_reference)
        || string_is_forbidden(&input.on_call_roster_state)
        || string_is_forbidden(&input.rehearsal_drill_freshness_state)
        || string_is_forbidden(&input.widening_stage_reference)
        || string_is_forbidden(&input.last_rehearsal_drill_revision)
    {
        return Err(M5OrrReviewResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = rehearsal_drill_stays_honest(
        input.rehearsal_drill_coverage,
        input.rehearsal_drill_lineage_is_truthful,
        input.keeps_evidence_snapshot_visible,
        input.override_without_evidence_requested,
        input.blocked_until_evidence_linked,
        input.lineage_gap_present,
        input.lineage_gap_flagged,
    );
    let provides_record = input.rehearsal_drill_coverage.is_classified()
        && !input.resolved_coverage_identity.trim().is_empty()
        && !input.evidence_snapshot_ledger.trim().is_empty()
        && !input.orr_signoff_reference.trim().is_empty()
        && !input.on_call_roster_state.trim().is_empty()
        && !input.rehearsal_drill_freshness_state.trim().is_empty()
        && !input.widening_stage_reference.trim().is_empty()
        && !input.last_rehearsal_drill_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5RehearsalDrillEntryDegradeReason::RehearsalDrillTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5RehearsalDrillEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.rehearsal_drill_coverage.is_classified() {
        Some(M5RehearsalDrillEntryDegradeReason::RehearsalDrillCoverageUnclassified)
    } else if !provides_record {
        Some(
            M5RehearsalDrillEntryDegradeReason::RehearsalDrillDropsEvidenceOrImpliesGreenWhileStale,
        )
    } else if !all_forms {
        Some(M5RehearsalDrillEntryDegradeReason::RehearsalDrillFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5RehearsalDrillEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5OrrReviewNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedRehearsalDrillEntry {
        entry_id: input.entry_id,
        rehearsal_drill_ref: input.rehearsal_drill_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        rehearsal_drill_coverage: input.rehearsal_drill_coverage.as_str().to_owned(),
        rehearsal_drill_coverage_is_classified: input.rehearsal_drill_coverage.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        resolved_coverage_identity: input.resolved_coverage_identity,
        evidence_snapshot_ledger: input.evidence_snapshot_ledger,
        orr_signoff_reference: input.orr_signoff_reference,
        on_call_roster_state: input.on_call_roster_state,
        rehearsal_drill_freshness_state: input.rehearsal_drill_freshness_state,
        widening_stage_reference: input.widening_stage_reference,
        last_rehearsal_drill_revision: input.last_rehearsal_drill_revision,
        keeps_evidence_snapshot_visible: input.keeps_evidence_snapshot_visible,
        rehearsal_drill_lineage_is_truthful: input.rehearsal_drill_lineage_is_truthful,
        override_without_evidence_requested: input.override_without_evidence_requested,
        blocked_until_evidence_linked: input.blocked_until_evidence_linked,
        lineage_gap_present: input.lineage_gap_present,
        lineage_gap_flagged: input.lineage_gap_flagged,
        rehearsal_drill_stays_honest: record_stays_honest,
        provides_complete_rehearsal_drill_record: provides_record,
        degrade_reason,
        next_action,
        rehearsal_drill_safe_on_every_coverage: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved orr-review and rehearsal-drill
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OrrReviewRehearsalDrillRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5OrrReviewRehearsalDrillRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5OrrReviewAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5OrrReviewExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    /// Resolved orr-review-registry examples.
    pub orr_review_entries: Vec<M5ResolvedOrrReviewEntry>,
    /// Resolved rehearsal-drill examples.
    pub rehearsal_drill_entries: Vec<M5ResolvedRehearsalDrillEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the orr-review and
    /// rehearsal-drill domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never widens a cohort without current rollback and diagnostics evidence. MUST be
    /// `false`.
    pub widens_a_stable_claim_without_current_orr_and_rehearsal_evidence: bool,
    /// Hard invariant: this row never runs partner or public support language ahead of cohort proof. MUST be
    /// `false`.
    pub lets_a_rehearsal_packet_go_stale_or_skipped_before_widening: bool,
    /// Hard invariant: this row never hides the rollback target or diagnostics posture before widening. MUST be
    /// `false`.
    pub hides_the_required_role_roster_or_on_call_coverage: bool,
    /// Hard invariant: this row never collapses distinct cohort evidence classes into one lane. MUST be `false`.
    pub implies_green_when_orr_or_rehearsal_packets_are_stale: bool,
}

impl M5OrrReviewRehearsalDrillRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5OrrReviewAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5OrrReviewAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5OrrReviewExportField> =
            self.export_fields.iter().copied().collect();
        M5OrrReviewExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.widens_a_stable_claim_without_current_orr_and_rehearsal_evidence
            && !self.lets_a_rehearsal_packet_go_stale_or_skipped_before_widening
            && !self.hides_the_required_role_roster_or_on_call_coverage
            && !self.implies_green_when_orr_or_rehearsal_packets_are_stale
    }

    /// True when a clean orr-review entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified cohort archetype, publishes a complete descriptor object, preserves its rollback and
    /// diagnostics posture, covers all three resolution forms, and keeps a public-facing cohort's support
    /// language matched to proof.
    fn descriptor_is_honest(ex: &M5ResolvedOrrReviewEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.orr_review_packet_kind_is_classified
                && ex.orr_review_object_complete
                && ex.orr_review_documented_before_widening
                && ex.covers_all_resolution_forms
                && (!ex.requires_documented_exception
                    || ex.attributable_asset_or_approved_exception))
    }

    /// True when a clean rehearsal-drill entry preserves a safe packet: it keeps a classified evidence
    /// scope, provides the complete rehearsal-drill object, stays honest, and covers all three resolution forms.
    fn evidence_is_honest(ex: &M5ResolvedRehearsalDrillEntry) -> bool {
        !ex.is_clean()
            || (ex.rehearsal_drill_coverage_is_classified
                && ex.provides_complete_rehearsal_drill_record
                && ex.rehearsal_drill_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.orr_review_entries
            .iter()
            .all(Self::descriptor_is_honest)
            && self
                .rehearsal_drill_entries
                .iter()
                .all(Self::evidence_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OrrReviewRehearsalDrillRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Cohort-archetype tokens (minted by this lane).
    pub orr_review_packet_kinds: Vec<String>,
    /// Evidence-scope tokens (minted by this lane).
    pub rehearsal_drill_coverages: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Cohort-descriptor-entry degrade-reason tokens.
    pub orr_review_degrade_reasons: Vec<String>,
    /// Cohort-evidence-packet-entry degrade-reason tokens.
    pub rehearsal_drill_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5OrrReviewRehearsalDrillRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5LaunchControlRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5OrrReviewResolutionForm::ALL, |v| v.as_str()),
            orr_review_packet_kinds: tokens(&M5OrrReviewPacketKind::ALL, |v| v.as_str()),
            rehearsal_drill_coverages: tokens(&M5RehearsalDrillCoverageKind::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5OrrReviewSurfaceContext::ALL, |v| v.as_str()),
            orr_review_degrade_reasons: tokens(&M5OrrReviewEntryDegradeReason::ALL, |v| v.as_str()),
            rehearsal_drill_degrade_reasons: tokens(
                &M5RehearsalDrillEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5OrrReviewAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5OrrReviewNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5OrrReviewExportField::ALL, |v| v.as_str()),
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
pub struct M5OrrReviewRehearsalDrillRegistriesGovernanceReview {
    /// The descriptor registry names a canonical token, semantic role, and cohort archetype for every entry.
    pub orr_review_registry_names_token_role_and_type: bool,
    /// Every claimed cohort resolves to one typed orr-review object from the shared registry, not
    /// per-entry reconstruction.
    pub type_resolves_to_typed_orr_review_from_shared_registry: bool,
    /// The exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles are published for every resolved descriptor.
    pub build_row_and_cohort_lineage_published: bool,
    /// Cohorts cannot widen without preserving rollback and diagnostics posture before widening.
    pub scope_cannot_widen_without_documented_orr_review: bool,
    /// The cohort evidence keeps the cohort proof visible and binds partner / public support language to it.
    pub rehearsal_drill_keeps_evidence_visible_and_blocks_stale_green: bool,
    /// Partner / public support language stays matched to cohort proof for every public-facing cohort.
    pub approved_exception_matched_to_scope_for_widening: bool,
    /// Every orr-review and rehearsal-drill entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Cohort-descriptor and rehearsal-drill behavior stay bound to the shared registries rather than
    /// hand-copied per cohort.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shiproom, release center, executive steering, and program governance read a single cohort source.
    pub shiproom_release_center_executive_steering_and_program_governance_read_single_source: bool,
    /// A widen-without-rollback attempt, an incomplete object, or hidden cohort evidence is caught by fixtures
    /// before release evidence turns green.
    pub exception_or_rehearsal_drill_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OrrReviewRehearsalDrillRegistriesConsumerProjection {
    /// Shiproom and release center consume the shared orr-review registry.
    pub shiproom_and_release_center_consume_shared_registries: bool,
    /// Executive steering and program governance consume the shared rehearsal-drill registry.
    pub executive_steering_and_program_governance_consume_shared_registries: bool,
    /// Diagnostics and public proof consume the shared registries.
    pub diagnostics_and_public_proof_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical orr-review and rehearsal-drill domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical orr-review / rehearsal-drill registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OrrReviewRehearsalDrillRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OrrReviewRehearsalDrillRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting cohort audit for the lane.
    pub rehearsal_drill_control_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5OrrReviewRehearsalDrillRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5OrrReviewRehearsalDrillRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5OrrReviewRehearsalDrillRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5OrrReviewRehearsalDrillRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5OrrReviewRehearsalDrillRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5OrrReviewRehearsalDrillRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5OrrReviewRehearsalDrillRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5OrrReviewRehearsalDrillRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 orr-review and rehearsal-drill registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OrrReviewRehearsalDrillRegistriesPacket {
    /// Record kind; must equal [`M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5OrrReviewRehearsalDrillRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5OrrReviewRehearsalDrillRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5OrrReviewRehearsalDrillRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5OrrReviewRehearsalDrillRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5OrrReviewRehearsalDrillRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5OrrReviewRehearsalDrillRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5OrrReviewRehearsalDrillRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5OrrReviewRehearsalDrillRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5OrrReviewRehearsalDrillRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_RECORD_KIND {
            violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 orr-review / rehearsal-drill registries packet serializes"),
        ) {
            violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 orr-review / rehearsal-drill registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,orr_review_entries,rehearsal_drill_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .orr_review_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.rehearsal_drill_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.orr_review_entries.len(),
                row.rehearsal_drill_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Operational-Readiness-Review and Rehearsal-Drill Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- ORR / rehearsal packet kinds: {}\n",
            self.vocabulary_set.orr_review_packet_kinds.join(", ")
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
                "  - ORR-packet entries: {} / rehearsal-drill entries: {}\n",
                row.orr_review_entries.len(),
                row.rehearsal_drill_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry cohort reference table generated from the registry, so docs and shiproom runbooks
    /// render the same archetype-mode / repo-rows / bundle-ids / install-topology / toolchain-envelope /
    /// rollback-target truth the resolvers produced rather than a hand-copied cohort table. Only clean,
    /// registry-bound orr-review entries are listed.
    pub fn render_orr_review_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| orr_packet_binding_id | packet_kind_mode | readiness_scope_reference | release_owner_reference | advisory_owner_reference | support_room_owner_reference | backup_signer_reference |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.orr_review_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.orr_packet_binding_id,
                    ex.canonical_orr_review_packet_kind_mode,
                    ex.readiness_scope_reference,
                    ex.release_owner_reference,
                    ex.advisory_owner_reference,
                    ex.support_room_owner_reference,
                    ex.backup_signer_reference
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5OrrReviewRehearsalDrillRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5OrrReviewRehearsalDrillRegistriesViolation>),
}

impl fmt::Display for M5OrrReviewRehearsalDrillRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 orr-review / rehearsal-drill registries export parse failed: {error}"
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
                    "m5 orr-review / rehearsal-drill registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5OrrReviewRehearsalDrillRegistriesArtifactError {}

/// Validation failures emitted by [`M5OrrReviewRehearsalDrillRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5OrrReviewRehearsalDrillRegistriesViolation {
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
    /// A registry row does not point at both the orr-review and rehearsal-drill domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, widen-without-rollback, field-incomplete,
    /// form-incomplete, or a rehearsal-drill entry missing the complete evidence object).
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
    OrrReviewResolutionNotProven,
    /// Rollback-and-diagnostics-preservation is not proven: no widen-without-rollback example and no unbound
    /// example degrade, no clean bounded descriptor entry is present, or a clean descriptor entry is unbounded
    /// or unbound.
    RehearsalDrillAttributionNotProven,
    /// Cohort-evidence-integrity is not proven: clean evidence entries do not cover the canonical dogfood-ring /
    /// rehearsal-currency / rehearsal-drill-signoff scopes with full resolution-form coverage while providing the
    /// complete evidence object, no support-ahead or form-incomplete example degrades, or a clean evidence entry
    /// is missing the complete evidence object.
    RehearsalDrillIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5OrrReviewRehearsalDrillRegistriesViolation {
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
            Self::OrrReviewResolutionNotProven => "orr_review_resolution_not_proven",
            Self::RehearsalDrillAttributionNotProven => "rehearsal_drill_attribution_not_proven",
            Self::RehearsalDrillIntegrityNotProven => "rehearsal_drill_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_orr_review_and_rehearsal_drill_registries_export() -> Result<
    M5OrrReviewRehearsalDrillRegistriesPacket,
    M5OrrReviewRehearsalDrillRegistriesArtifactError,
> {
    let packet: M5OrrReviewRehearsalDrillRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-orr-review-and-rehearsal-drill-registries-proof/support_export.json"
        )
    ))
    .map_err(M5OrrReviewRehearsalDrillRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5OrrReviewRehearsalDrillRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5OrrReviewRehearsalDrillRegistriesPacket,
    violations: &mut Vec<M5OrrReviewRehearsalDrillRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_SCHEMA_REF,
        M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_DOC_REF,
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_ORR_REVIEW_DOMAIN_SCHEMA_REF,
        M5_REHEARSAL_DRILL_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5OrrReviewRehearsalDrillRegistriesPacket,
    violations: &mut Vec<M5OrrReviewRehearsalDrillRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::NoRegistryRows);
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
            violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5OrrReviewRehearsalDrillRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_ORR_REVIEW_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_REHEARSAL_DRILL_DOMAIN_SCHEMA_REF)
        {
            violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.orr_review_entries.is_empty() || row.rehearsal_drill_entries.is_empty() {
            violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5OrrReviewRehearsalDrillRegistriesPacket,
    violations: &mut Vec<M5OrrReviewRehearsalDrillRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.orr_review_registry_names_token_role_and_type,
        review.type_resolves_to_typed_orr_review_from_shared_registry,
        review.build_row_and_cohort_lineage_published,
        review.scope_cannot_widen_without_documented_orr_review,
        review.rehearsal_drill_keeps_evidence_visible_and_blocks_stale_green,
        review.approved_exception_matched_to_scope_for_widening,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shiproom_release_center_executive_steering_and_program_governance_read_single_source,
        review.exception_or_rehearsal_drill_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5OrrReviewRehearsalDrillRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5OrrReviewRehearsalDrillRegistriesPacket,
    violations: &mut Vec<M5OrrReviewRehearsalDrillRegistriesViolation>,
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
                .push(M5OrrReviewRehearsalDrillRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5OrrReviewRehearsalDrillRegistriesPacket,
    violations: &mut Vec<M5OrrReviewRehearsalDrillRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5OrrReviewRehearsalDrillRegistriesPacket,
    violations: &mut Vec<M5OrrReviewRehearsalDrillRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.rehearsal_drill_control_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5OrrReviewRehearsalDrillRegistriesPacket,
    violations: &mut Vec<M5OrrReviewRehearsalDrillRegistriesViolation>,
) {
    let descriptors = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.orr_review_entries.iter())
    };
    let evidence = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.rehearsal_drill_entries.iter())
    };

    // AC1: every active cohort can be inspected by exact rows, bundles, toolchains, and deployment profiles.
    // Clean descriptor entries cover the canonical cohort archetypes and the first release-center / shiproom /
    // executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no
    // clean descriptor entry published an incomplete object.
    let clean_archetypes: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.orr_review_packet_kind.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let archetypes_covered = M5OrrReviewPacketKind::CANONICAL_PACKET_KINDS
        .iter()
        .all(|k| clean_archetypes.contains(k.as_str()));
    let first_surfaces_covered = M5OrrReviewSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = descriptors().any(|ex| {
        ex.degrade_reason == Some(M5OrrReviewEntryDegradeReason::OrrReviewObjectIncomplete)
    });
    let no_clean_incomplete =
        !descriptors().any(|ex| ex.is_clean() && !ex.orr_review_object_complete);
    if !(archetypes_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(M5OrrReviewRehearsalDrillRegistriesViolation::OrrReviewResolutionNotProven);
    }

    // AC2: cohort packets preserve rollback and diagnostics posture before widening. A widen-without-rollback
    // example degrades, an unbound example degrades, at least one clean bounded descriptor entry is present, and
    // no clean descriptor entry is unbounded or unbound.
    let widen_fold_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(
                M5OrrReviewEntryDegradeReason::OrrReviewWidensScopeUndocumentedOrRunsClaimAheadOfProof,
            )
    });
    let unbound_degrades = descriptors().any(|ex| {
        ex.degrade_reason == Some(M5OrrReviewEntryDegradeReason::OrrReviewNotBoundToRegistry)
    });
    let bounded_clean_descriptor =
        descriptors().any(|ex| ex.is_clean() && ex.orr_review_documented_before_widening);
    let no_clean_unbound = !descriptors().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded =
        !descriptors().any(|ex| ex.is_clean() && !ex.orr_review_documented_before_widening);
    if !(widen_fold_degrades
        && unbound_degrades
        && bounded_clean_descriptor
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations
            .push(M5OrrReviewRehearsalDrillRegistriesViolation::RehearsalDrillAttributionNotProven);
    }

    // AC3: claim publication can prove which cohort evidence backs each launch-bearing lane. Clean evidence
    // entries cover every canonical dogfood-ring / rehearsal-currency / rehearsal-drill-signoff scope with full
    // resolution-form coverage while providing the complete evidence object, a support-ahead example degrades, a
    // form-incomplete example degrades, and no clean evidence entry is missing the complete object.
    let clean_rehearsal_drill_coverages: BTreeSet<String> = evidence()
        .filter(|ex| {
            ex.is_clean()
                && ex.rehearsal_drill_coverage_is_classified
                && ex.provides_complete_rehearsal_drill_record
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.rehearsal_drill_coverage.clone())
        .collect();
    let rehearsal_drill_coverages_covered = M5RehearsalDrillCoverageKind::CANONICAL_COVERAGES
        .iter()
        .all(|m| clean_rehearsal_drill_coverages.contains(m.as_str()));
    let support_ahead_degrades = evidence().any(|ex| {
        ex.degrade_reason
            == Some(M5RehearsalDrillEntryDegradeReason::RehearsalDrillDropsEvidenceOrImpliesGreenWhileStale)
    });
    let form_incomplete_degrades = evidence().any(|ex| {
        ex.degrade_reason
            == Some(M5RehearsalDrillEntryDegradeReason::RehearsalDrillFormCoverageIncomplete)
    });
    let no_clean_missing_evidence =
        !evidence().any(|ex| ex.is_clean() && !ex.provides_complete_rehearsal_drill_record);
    if !(rehearsal_drill_coverages_covered
        && support_ahead_degrades
        && form_incomplete_degrades
        && no_clean_missing_evidence)
    {
        violations
            .push(M5OrrReviewRehearsalDrillRegistriesViolation::RehearsalDrillIntegrityNotProven);
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

/// The widening stages this lane governs, for downstream reference: the orr-review registry defines the
/// minimum evidence and soak expectations that let a lane advance across the alpha, beta, release-candidate,
/// stable, and long-term-support widening stages, and the rehearsal-drill registry records the conditions that
/// immediately stop that progression.
pub const IMPLEMENTED_ORR_REVIEW_STAGES: [M5LaunchControlWideningStage; 5] =
    M5LaunchControlWideningStage::ALL;
