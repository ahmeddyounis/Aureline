//! Implemented M5 stable go/no-go widening-decision and ring-history registries.
//!
//! The frozen [launch-control matrix][matrix] names Aureline's governed launch-bearing lanes and locks their
//! controlled vocabulary. This module governs *making every claimed stable-widening event reconstructible after
//! the fact rather than trusting tribal meeting memory*: it turns the *stable go/no-go decision-record* grammar
//! (how each widening event — an alpha, beta, release-candidate, stable, long-term-support, or correction-reissue
//! widening — records its final go/no-go decision, its open risks, its narrowed claims, its named on-call and
//! signoff roster, its exact evidence snapshot, and its decision-freshness expiry so a stable claim can never
//! widen on a stale, dropped, or undocumented decision) and the *ring-history snapshot* grammar (how a
//! launch-bearing lane preserves the ring history — a ring-history scope, a prior-blocker scope, or a
//! packet-freshness scope — with the preserved evidence snapshot, the preserved signoff, the named on-call roster
//! state, the previous packet-freshness state, and the authorized widening stage that justified widening) into
//! registry resolvers that produce export-safe, honest projections. Every claimed widening event then resolves to
//! one durable go/no-go record — the decision kind it classifies, the final decision, the open risks, the
//! narrowed claims, the on-call / signoff roster, the exact evidence snapshot, and the decision-freshness expiry,
//! all preserved before widening so a lane never widens on a stale or dropped record and so partner / public
//! support language never outruns current proof — and to one ring-history snapshot — the resolved coverage
//! identity, the ring-history ledger, the preserved signoff, the named on-call roster state, the previous
//! packet-freshness state, the authorized widening stage, and the last ring-history revision — that the shiproom,
//! release-center, executive-steering, program-governance, correction-line, and support / export surfaces can
//! inspect without manual reconstruction, so every claimed widening event points at one durable go/no-go record
//! tied to exact evidence and roster state, later incident or support review can reconstruct why a lane widened,
//! shiproom and correction-line flows consume the same record rather than duplicating decision state, and a
//! record that cannot show the evidence snapshot, roster, or ring history behind its widening degrades honestly
//! instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one durable go/no-go record per widening event.** [`resolve_widening_decision_entry`]
//!   refuses to read as a clean, registry-bound decision entry unless it names a canonical registry token, a
//!   classified [decision kind][M5WideningDecisionPacketKind], a launch-control role, covers every
//!   [resolution form][M5WideningDecisionResolutionForm] (the canonical object, the accessible summary, and the audit
//!   record), publishes every decision field (final decision, open risks, narrowed claims, on-call / signoff
//!   roster, evidence snapshot, and decision-freshness expiry), keeps the record documented before widening,
//!   and keeps partner / public support language matched to proof; otherwise it degrades.
//! * **Fail widening when a go/no-go record is stale, dropped, or missing its roster.**
//!   [`widening_decision_stays_documented_before_widening`] rejects a decision entry whose record is not
//!   documented before widening (a lane widening on a stale or dropped record) so it degrades to
//!   [`M5WideningDecisionEntryDegradeReason::WideningDecisionWidensScopeUndocumentedOrRunsClaimAheadOfProof`],
//!   and a public-facing lane whose support language runs ahead of proof degrades the same way — the
//!   structured blocker reason a widen-on-stale-record attempt must surface.
//! * **Keep the ring-history snapshot from implying green while its preserved evidence is stale.**
//!   [`resolve_ring_history_entry`] names a classified [snapshot scope][M5RingHistoryCoverageKind],
//!   requires the full resolved-coverage-identity / ring-history-ledger / signoff / on-call-roster /
//!   packet-freshness / widening-stage / last-ring-history-revision record, covers every resolution form,
//!   and degrades to
//!   [`M5RingHistoryEntryDegradeReason::RingHistoryDropsEvidenceOrImpliesGreenWhileStale`]
//!   when the snapshot would imply green while its preserved evidence is stale, hide the ring-history evidence, or
//!   let a roster-coverage gap masquerade as covered, so a ring-history snapshot can never read as trustworthy
//!   when it has quietly dropped the reason a lane's widening is actually gated.
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
    seeded_m5_widening_decision_and_ring_history_registries,
    seeded_m5_widening_decision_and_ring_history_registries_ring_history_preview_narrowed,
    seeded_m5_widening_decision_and_ring_history_registries_widening_decision_beta_narrowed,
    M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_PACKET_ID,
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

/// Stable record-kind tag carried by [`M5WideningDecisionRingHistoryRegistriesPacket`].
pub const M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_widening_decision_and_ring_history_registries";

/// Schema version for M5 widening-decision / ring-history registry records.
pub const M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_SCHEMA_REF: &str =
    "schemas/program/m5-widening-decision-and-ring-history-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_DOC_REF: &str =
    "docs/release/m5_widening_decision_and_ring_history_registries.md";

/// Repo-relative path of the canonical widening-decision domain schema minted by this lane (how a widening ring
/// transition declares its minimum entry evidence, soak-window expectation, why widening is allowed, its
/// known-limits packet, issue-template linkage, claim-narrowing action, and the ring-history reference that
/// immediately stops it).
pub const M5_WIDENING_DECISION_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-widening-decision-packet.schema.json";

/// Repo-relative path of the canonical ring-history domain schema minted by this lane (how a launch-bearing lane
/// records the ring-history condition — a crash / data-loss / trust defect, a repeated protected-metric
/// regression, or a stale readiness packet — that halts regression asset while it is active).
pub const M5_RING_HISTORY_DOMAIN_SCHEMA_REF: &str = "schemas/program/m5-ring-history.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-widening-decision-and-ring-history-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-widening-decision-and-ring-history-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-widening-decision-and-ring-history-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/release/m5-widening-decision-and-ring-history-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// cohort invents a parallel surface set.
pub type M5WideningDecisionRingHistoryRegistriesConsumerSurface = M5LaunchControlConsumerSurface;

/// One of the three resolution forms every widening-decision or ring-history entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// widening-decision and ring-history *domains* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WideningDecisionResolutionForm {
    /// The canonical resolved widening-decision / ring-history object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved cohort discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved cohort inspectable off-renderer.
    AuditRecord,
}

impl M5WideningDecisionResolutionForm {
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

/// Controlled cohort archetype a widening-decision entry classifies, so the typed descriptor model shares one
/// registry rather than a hand-copied per-cohort assumption. Minted by this lane because the frozen matrix
/// carries the launch-bearing cohorts but distinguishes the dogfood / migration-alpha / extension-author /
/// design-partner / public-preview / certified-archetype archetypes an auditable descriptor classifies against
/// explicitly. Every classified archetype carries its canonical mode, and the design-partner-preview and
/// public-preview archetypes are public-facing so their partner / public support language must stay matched to
/// cohort proof before the cohort widens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WideningDecisionPacketKind {
    /// The internal dogfood core-team canary cohort.
    AlphaWideningDecision,
    /// The migration alpha cohort (external alpha migrating from a prior toolchain).
    BetaWideningDecision,
    /// The extension-author cohort (compatibility rehearsals current, freeze exceptions documented).
    ReleaseCandidateWideningDecision,
    /// The design-partner preview cohort (public-facing; support language must match cohort proof).
    StableWideningDecision,
    /// The public preview cohort (public-facing; support language must match cohort proof).
    LongTermSupportWideningDecision,
    /// The certified-archetype cohort (ORR signed and a go/no-go coverage recorded).
    CorrectionReissueDecision,
    /// The cohort archetype is unclassified, which is disallowed.
    PacketKindUnclassified,
}

impl M5WideningDecisionPacketKind {
    /// Every cohort archetype, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AlphaWideningDecision,
        Self::BetaWideningDecision,
        Self::ReleaseCandidateWideningDecision,
        Self::StableWideningDecision,
        Self::LongTermSupportWideningDecision,
        Self::CorrectionReissueDecision,
        Self::PacketKindUnclassified,
    ];

    /// The six canonical cohort archetypes every claimed M5 launch-bearing cohort classifies against.
    pub const CANONICAL_PACKET_KINDS: [Self; 6] = [
        Self::AlphaWideningDecision,
        Self::BetaWideningDecision,
        Self::ReleaseCandidateWideningDecision,
        Self::StableWideningDecision,
        Self::LongTermSupportWideningDecision,
        Self::CorrectionReissueDecision,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AlphaWideningDecision => "alpha_widening_decision",
            Self::BetaWideningDecision => "beta_widening_decision",
            Self::ReleaseCandidateWideningDecision => "release_candidate_widening_decision",
            Self::StableWideningDecision => "stable_widening_decision",
            Self::LongTermSupportWideningDecision => "long_term_support_widening_decision",
            Self::CorrectionReissueDecision => "correction_reissue_decision",
            Self::PacketKindUnclassified => "packet_kind_unclassified",
        }
    }

    /// Whether the archetype is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::PacketKindUnclassified)
    }

    /// The canonical mode for this cohort archetype.
    pub const fn canonical_widening_decision_packet_kind_mode(self) -> &'static str {
        match self {
            Self::AlphaWideningDecision => "alpha_widening_decision_kind",
            Self::BetaWideningDecision => "beta_widening_decision_kind",
            Self::ReleaseCandidateWideningDecision => "release_candidate_widening_decision_kind",
            Self::StableWideningDecision => "stable_widening_decision_kind",
            Self::LongTermSupportWideningDecision => "long_term_support_widening_decision_kind",
            Self::CorrectionReissueDecision => "correction_reissue_decision_kind",
            Self::PacketKindUnclassified => "",
        }
    }

    /// Whether this archetype is public-facing and so must keep partner / public support language matched to
    /// cohort proof before the cohort widens.
    pub const fn requires_documented_exception(self) -> bool {
        matches!(
            self,
            Self::StableWideningDecision | Self::LongTermSupportWideningDecision
        )
    }
}

/// Controlled evidence scope a ring-history entry must resolve its cohort proof from, so an evidence
/// packet shares one registry rather than a hand-copied per-record assumption. Minted by this lane, tracking
/// whether the evidence came from dogfood-ring telemetry, current rehearsal cadence, or an explicit go/no-go
/// signoff the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RingHistoryCoverageKind {
    /// The evidence came from internal dogfood-ring telemetry.
    RingHistoryScope,
    /// The evidence came from current rehearsal cadence (publish/rollback, mixed-version, handoff drills).
    PriorBlockerScope,
    /// The evidence came from an explicit go/no-go signoff with a preserved evidence snapshot.
    PacketFreshnessScope,
    /// The evidence scope is unclassified, which is disallowed.
    CoverageUnclassified,
}

impl M5RingHistoryCoverageKind {
    /// Every evidence scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RingHistoryScope,
        Self::PriorBlockerScope,
        Self::PacketFreshnessScope,
        Self::CoverageUnclassified,
    ];

    /// The three canonical evidence scopes every ring-history packet must stay distinct across.
    pub const CANONICAL_COVERAGES: [Self; 3] = [
        Self::RingHistoryScope,
        Self::PriorBlockerScope,
        Self::PacketFreshnessScope,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RingHistoryScope => "ring_history_scope",
            Self::PriorBlockerScope => "prior_blocker_scope",
            Self::PacketFreshnessScope => "packet_freshness_scope",
            Self::CoverageUnclassified => "coverage_unclassified",
        }
    }

    /// Whether the evidence scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::CoverageUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a widening-decision or
/// ring-history token's meaning stays stable whether it appears in the release-center, shiproom,
/// executive-steering, program-governance, or a support / export form. Minted by this lane, tracking the
/// first-consumer surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WideningDecisionSurfaceContext {
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

impl M5WideningDecisionSurfaceContext {
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

/// One mandatory rendered part a widening-decision or ring-history entry must be able to show, so no
/// cohort archetype, repo / bundle / toolchain / deployment row, known-limits packet, rollback target,
/// ring-history field, or registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WideningDecisionAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The cohort archetype the entry classifies (widening-decision entry).
    WideningDecisionType,
    /// The exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles the entry publishes (widening-decision entry).
    IncidentLineageRows,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The known-limits packet and rollback target the entry preserves before widening (widening-decision
    /// entry).
    BuildAndCohortLineage,
    /// The ring-history fields (cohort identity, known-limits ledger, rollback target, rehearsal currency,
    /// readiness signoff, support language) the entry publishes (ring-history entry).
    RingHistoryFields,
    /// The support-identity hint the entry publishes (ring-history entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved cohort descriptor or cohort evidence (both entries).
    PlainLanguageMeaning,
}

impl M5WideningDecisionAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::WideningDecisionType,
        Self::IncidentLineageRows,
        Self::ResolutionFormCoverage,
        Self::BuildAndCohortLineage,
        Self::RingHistoryFields,
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
            Self::WideningDecisionType => "widening_decision_packet_kind",
            Self::IncidentLineageRows => "incident_lineage_rows",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::BuildAndCohortLineage => "build_and_cohort_lineage",
            Self::RingHistoryFields => "ring_history_fields",
            Self::SupportIdentityHint => "support_identity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// cohort descriptor, a ring-history packet, or a degraded widening-decision / ring-history entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WideningDecisionNextAction {
    /// Expand the resolved cohort descriptor's or ring-history packet's plain-language meaning.
    ExpandWideningDecisionMeaning,
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

impl M5WideningDecisionNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandWideningDecisionMeaning,
        Self::InspectPacketKindOrCoverage,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandWideningDecisionMeaning => "expand_widening_decision_meaning",
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
pub enum M5WideningDecisionExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The cohort families covered.
    WideningDecisionFamilies,
    /// The cohort archetypes carried.
    WideningDecisionPacketKinds,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The evidence scopes carried.
    RingHistoryCoverages,
    /// The render / surface context.
    SurfaceContext,
    /// The cohort-archetype modes carried.
    WideningDecisionPacketKindModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5WideningDecisionExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::WideningDecisionFamilies,
        Self::WideningDecisionPacketKinds,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::RingHistoryCoverages,
        Self::SurfaceContext,
        Self::WideningDecisionPacketKindModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::WideningDecisionFamilies,
        Self::WideningDecisionPacketKinds,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::WideningDecisionFamilies => "widening_decision_families",
            Self::WideningDecisionPacketKinds => "widening_decision_packet_kinds",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::RingHistoryCoverages => "ring_history_coverages",
            Self::SurfaceContext => "surface_context",
            Self::WideningDecisionPacketKindModes => "widening_decision_packet_kind_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a widening-decision entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, widen-without-rollback, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WideningDecisionEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the descriptor means.
    WideningDecisionTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The cohort archetype is unclassified (not in the resolved taxonomy).
    WideningDecisionPacketKindUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    WideningDecisionNotBoundToRegistry,
    /// The resolved widening-decision object is incomplete: the exact repo / archetype rows, bundle IDs, install
    /// topology, toolchain envelope, known limits, rollback target, or diagnostics posture is unstated.
    WideningDecisionObjectIncomplete,
    /// The cohort's rollback and diagnostics posture is not preserved before widening (a cohort widening without
    /// a rollback target and diagnostics posture), or a public-facing cohort ran its support language ahead of
    /// cohort proof.
    WideningDecisionWidensScopeUndocumentedOrRunsClaimAheadOfProof,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A public-facing cohort did not keep its support language matched to cohort proof before widening.
    WideningDecisionUndocumentedForScopeWidening,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5WideningDecisionEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::WideningDecisionTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::WideningDecisionPacketKindUnclassified,
        Self::WideningDecisionNotBoundToRegistry,
        Self::WideningDecisionObjectIncomplete,
        Self::WideningDecisionWidensScopeUndocumentedOrRunsClaimAheadOfProof,
        Self::ResolutionFormCoverageIncomplete,
        Self::WideningDecisionUndocumentedForScopeWidening,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WideningDecisionTokenUnstated => "widening_decision_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::WideningDecisionPacketKindUnclassified => {
                "widening_decision_packet_kind_unclassified"
            }
            Self::WideningDecisionNotBoundToRegistry => "widening_decision_not_bound_to_registry",
            Self::WideningDecisionObjectIncomplete => "widening_decision_object_incomplete",
            Self::WideningDecisionWidensScopeUndocumentedOrRunsClaimAheadOfProof => {
                "widening_decision_widens_scope_undocumented_or_runs_claim_ahead_of_proof"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::WideningDecisionUndocumentedForScopeWidening => {
                "widening_decision_undocumented_for_scope_widening"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5WideningDecisionNextAction {
        match self {
            Self::WideningDecisionTokenUnstated | Self::WideningDecisionNotBoundToRegistry => {
                M5WideningDecisionNextAction::TraceCanonicalRegistry
            }
            Self::WideningDecisionPacketKindUnclassified
            | Self::WideningDecisionObjectIncomplete
            | Self::WideningDecisionWidensScopeUndocumentedOrRunsClaimAheadOfProof => {
                M5WideningDecisionNextAction::InspectPacketKindOrCoverage
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5WideningDecisionNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::WideningDecisionUndocumentedForScopeWidening
            | Self::ProofStale => M5WideningDecisionNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5LaunchControlDowngradeTrigger {
        match self {
            Self::WideningDecisionTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::WideningDecisionNotBoundToRegistry => {
                M5LaunchControlDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::WideningDecisionPacketKindUnclassified
            | Self::WideningDecisionObjectIncomplete => {
                M5LaunchControlDowngradeTrigger::CohortMembershipUnstated
            }
            Self::WideningDecisionWidensScopeUndocumentedOrRunsClaimAheadOfProof
            | Self::WideningDecisionUndocumentedForScopeWidening => {
                M5LaunchControlDowngradeTrigger::WidenedWithoutCurrentCohortEvidence
            }
            Self::ProofStale => M5LaunchControlDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a ring-history entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RingHistoryEntryDegradeReason {
    /// The canonical registry token name is unstated.
    RingHistoryTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The evidence scope is unclassified (not in the resolved taxonomy).
    RingHistoryCoverageUnclassified,
    /// The cohort evidence would run partner / public support language ahead of cohort proof, hide the cohort
    /// evidence, let a known-limits gap masquerade as covered, or it dropped one of the required ring-history
    /// fields (cohort identity, known-limits ledger, rollback target, rehearsal currency, readiness signoff,
    /// support language, last widening revision).
    RingHistoryDropsEvidenceOrImpliesGreenWhileStale,
    /// The canonical / accessible / audit resolution-form coverage of the evidence is incomplete.
    RingHistoryFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RingHistoryEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RingHistoryTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::RingHistoryCoverageUnclassified,
        Self::RingHistoryDropsEvidenceOrImpliesGreenWhileStale,
        Self::RingHistoryFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RingHistoryTokenUnstated => "ring_history_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::RingHistoryCoverageUnclassified => "ring_history_coverage_unclassified",
            Self::RingHistoryDropsEvidenceOrImpliesGreenWhileStale => {
                "ring_history_drops_evidence_or_implies_green_while_stale"
            }
            Self::RingHistoryFormCoverageIncomplete => "ring_history_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5WideningDecisionNextAction {
        match self {
            Self::RingHistoryTokenUnstated => M5WideningDecisionNextAction::TraceCanonicalRegistry,
            Self::RingHistoryCoverageUnclassified
            | Self::RingHistoryDropsEvidenceOrImpliesGreenWhileStale => {
                M5WideningDecisionNextAction::InspectPacketKindOrCoverage
            }
            Self::RingHistoryFormCoverageIncomplete => {
                M5WideningDecisionNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5WideningDecisionNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5LaunchControlDowngradeTrigger {
        match self {
            Self::RingHistoryTokenUnstated => {
                M5LaunchControlDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::RingHistoryCoverageUnclassified => {
                M5LaunchControlDowngradeTrigger::ReadinessStateUnstated
            }
            Self::RingHistoryDropsEvidenceOrImpliesGreenWhileStale => {
                M5LaunchControlDowngradeTrigger::RanPartnerOrPublicLanguageAheadOfCohortProof
            }
            Self::RingHistoryFormCoverageIncomplete => {
                M5LaunchControlDowngradeTrigger::ImpliedGreenWhileGoNoGoOrOrrWasStale
            }
            Self::ProofStale => M5LaunchControlDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_widening_decision_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WideningDecisionEntryResolutionInput {
    /// Stable identity of the widening-decision-registry entry.
    pub entry_id: String,
    /// The stable cohort-binding ID this descriptor binds to (e.g. `incident.lane.public-preview`); empty means
    /// unstated.
    pub widening_event_binding_id: String,
    /// The canonical registry token name (e.g. `freeze.exception.long_term_support_widening_decision`); empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5LaunchControlRole,
    /// The cohort archetype this entry classifies.
    pub widening_decision_packet_kind: M5WideningDecisionPacketKind,
    /// The render / surface context.
    pub surface_context: M5WideningDecisionSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5WideningDecisionResolutionForm>,
    /// The published exact repo / archetype rows; empty means unstated.
    pub final_decision_reference: String,
    /// The published bundle IDs; empty means unstated.
    pub open_risks_reference: String,
    /// The published install topology; empty means unstated.
    pub narrowed_claims_reference: String,
    /// The published toolchain envelope; empty means unstated.
    pub on_call_roster_reference: String,
    /// The published known limits; empty means unstated.
    pub signoff_roster_reference: String,
    /// The published rollback target; empty means unstated.
    pub evidence_snapshot_reference: String,
    /// The published diagnostics posture; empty means unstated.
    pub decision_freshness_reference: String,
    /// True when the behavior traces to the widening-decision registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the cohort's rollback and diagnostics posture is preserved before widening (a hard invariant
    /// when `false`).
    pub widening_decision_documented_before_widening: bool,
    /// True when this cohort's archetype is public-facing.
    pub requires_documented_exception: bool,
    /// True when partner / public support language is matched to cohort proof before a public-facing cohort
    /// widens.
    pub attributable_asset_or_approved_exception: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe widening-decision-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWideningDecisionEntry {
    /// Stable identity of the widening-decision-registry entry.
    pub entry_id: String,
    /// The stable cohort-binding ID this descriptor binds to.
    pub widening_event_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the evidence snapshot and signoff before widening.
    pub semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: bool,
    /// The cohort-archetype token named by the entry.
    pub widening_decision_packet_kind: String,
    /// Whether the cohort archetype is classified into the resolved taxonomy.
    pub widening_decision_packet_kind_is_classified: bool,
    /// The canonical mode for the entry's cohort archetype.
    pub canonical_widening_decision_packet_kind_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published exact repo / archetype rows.
    pub final_decision_reference: String,
    /// The published bundle IDs.
    pub open_risks_reference: String,
    /// The published install topology.
    pub narrowed_claims_reference: String,
    /// The published toolchain envelope.
    pub on_call_roster_reference: String,
    /// The published known limits.
    pub signoff_roster_reference: String,
    /// The published rollback target.
    pub evidence_snapshot_reference: String,
    /// The published diagnostics posture.
    pub decision_freshness_reference: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved widening-decision object publishes every required field.
    pub widening_decision_object_complete: bool,
    /// Whether the entry traces to the widening-decision registry.
    pub bound_to_registry: bool,
    /// Whether the cohort's rollback and diagnostics posture stays preserved before widening.
    pub widening_decision_documented_before_widening: bool,
    /// Whether this cohort's archetype is public-facing.
    pub requires_documented_exception: bool,
    /// Whether partner / public support language is matched to cohort proof before widening.
    pub attributable_asset_or_approved_exception: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5WideningDecisionEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5WideningDecisionNextAction,
    /// Whether the descriptor resolves to one typed object across every claimed cohort (clean entry naming every
    /// fact).
    pub widening_decision_resolves_across_classes: bool,
}

impl M5ResolvedWideningDecisionEntry {
    /// Whether this widening-decision entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_ring_history_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RingHistoryEntryResolutionInput {
    /// Stable identity of the ring-history entry.
    pub entry_id: String,
    /// The stable evidence-ref this record binds to; empty means unstated.
    pub ring_history_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5LaunchControlRole,
    /// The evidence scope this record must resolve its cohort proof from.
    pub ring_history_coverage: M5RingHistoryCoverageKind,
    /// The render / surface context.
    pub surface_context: M5WideningDecisionSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5WideningDecisionResolutionForm>,
    /// The published resolved cohort identity; empty means missing.
    pub resolved_coverage_identity: String,
    /// The published known-limits ledger; empty means missing.
    pub evidence_snapshot_ledger: String,
    /// The published rollback-target reference; empty means missing.
    pub orr_signoff_reference: String,
    /// The published rehearsal-currency state; empty means missing.
    pub on_call_roster_state: String,
    /// The published readiness-signoff state; empty means missing.
    pub ring_history_freshness_state: String,
    /// The published cohort-bound support-language reference; empty means missing.
    pub widening_stage_reference: String,
    /// The published last widening revision; empty means missing.
    pub last_ring_history_revision: String,
    /// True when the record keeps the cohort evidence visible.
    pub keeps_evidence_snapshot_visible: bool,
    /// True when the evidence is truthful (never claims a clean packet over hidden cohort evidence).
    pub ring_history_lineage_is_truthful: bool,
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

/// Resolved, export-safe ring-history projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRingHistoryEntry {
    /// Stable identity of the ring-history entry.
    pub entry_id: String,
    /// The stable evidence-ref this record binds to.
    pub ring_history_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the evidence snapshot and signoff before widening.
    pub semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: bool,
    /// The evidence-scope token named by the entry.
    pub ring_history_coverage: String,
    /// Whether the evidence scope is classified into the resolved taxonomy.
    pub ring_history_coverage_is_classified: bool,
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
    pub ring_history_freshness_state: String,
    /// The published cohort-bound support-language reference.
    pub widening_stage_reference: String,
    /// The published last widening revision.
    pub last_ring_history_revision: String,
    /// Whether the record keeps the cohort evidence visible.
    pub keeps_evidence_snapshot_visible: bool,
    /// Whether the evidence is truthful.
    pub ring_history_lineage_is_truthful: bool,
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
    pub ring_history_stays_honest: bool,
    /// Whether the entry provides the complete ring-history object (cohort identity, known-limits ledger,
    /// rollback target, rehearsal currency, readiness signoff, support language, last widening revision).
    pub provides_complete_ring_history_record: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5RingHistoryEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5WideningDecisionNextAction,
    /// Whether the cohort evidence is safe on every claimed cohort (clean entry naming every fact).
    pub ring_history_safe_on_every_coverage: bool,
}

impl M5ResolvedRingHistoryEntry {
    /// Whether this ring-history entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5WideningDecisionResolutionError {
    /// The widening-decision-entry id was empty.
    EmptyWideningDecisionEntryId,
    /// The ring-history-entry id was empty.
    EmptyRingHistoryEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5WideningDecisionResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyWideningDecisionEntryId => "empty_widening_decision_entry_id",
            Self::EmptyRingHistoryEntryId => "empty_ring_history_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5WideningDecisionResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 widening-decision / ring-history registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5WideningDecisionResolutionError {}

fn form_tokens(forms: &[M5WideningDecisionResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5WideningDecisionResolutionForm]) -> bool {
    let present: BTreeSet<M5WideningDecisionResolutionForm> = forms.iter().copied().collect();
    M5WideningDecisionResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved widening-decision object publishes every required field: classified cohort archetype,
/// exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, known limits, rollback target,
/// and diagnostics posture. An unclassified archetype or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn widening_decision_object_is_complete(
    archetype: M5WideningDecisionPacketKind,
    final_decision_reference: &str,
    open_risks_reference: &str,
    narrowed_claims_reference: &str,
    on_call_roster_reference: &str,
    signoff_roster_reference: &str,
    evidence_snapshot_reference: &str,
    decision_freshness_reference: &str,
) -> bool {
    archetype.is_classified()
        && !final_decision_reference.trim().is_empty()
        && !open_risks_reference.trim().is_empty()
        && !narrowed_claims_reference.trim().is_empty()
        && !on_call_roster_reference.trim().is_empty()
        && !signoff_roster_reference.trim().is_empty()
        && !evidence_snapshot_reference.trim().is_empty()
        && !decision_freshness_reference.trim().is_empty()
}

/// Whether the cohort descriptor keeps a cohort from widening without preserving its rollback and diagnostics
/// posture: the archetype must be classified, the rollback and diagnostics posture must be preserved before
/// widening, and a public-facing cohort must keep its support language matched to cohort proof. An unclassified
/// archetype, an unpreserved rollback / diagnostics posture, or partner / public support language running ahead
/// of proof never matches.
pub fn widening_decision_stays_documented_before_widening(
    archetype: M5WideningDecisionPacketKind,
    widening_decision_documented_before_widening: bool,
    requires_documented_exception: bool,
    attributable_asset_or_approved_exception: bool,
) -> bool {
    archetype.is_classified()
        && widening_decision_documented_before_widening
        && (!requires_documented_exception || attributable_asset_or_approved_exception)
}

/// Whether a ring-history packet stays honest: the scope must be classified, the evidence must be truthful,
/// it must keep the cohort evidence visible, any partner / public support language must be bound to cohort proof
/// rather than running ahead of it, and any known-limits gap must be flagged rather than masquerade as covered.
pub fn ring_history_stays_honest(
    scope: M5RingHistoryCoverageKind,
    ring_history_lineage_is_truthful: bool,
    keeps_evidence_snapshot_visible: bool,
    override_without_evidence_requested: bool,
    blocked_until_evidence_linked: bool,
    lineage_gap_present: bool,
    lineage_gap_flagged: bool,
) -> bool {
    scope.is_classified()
        && ring_history_lineage_is_truthful
        && keeps_evidence_snapshot_visible
        && (!override_without_evidence_requested || blocked_until_evidence_linked)
        && (!lineage_gap_present || lineage_gap_flagged)
}

/// Resolves a widening-decision-registry entry so it stays bound to the widening-decision registry: the entry
/// names its canonical token, semantic role, and cohort archetype, covers all three resolution forms, publishes
/// a complete descriptor object (exact repo / archetype rows, bundle IDs, install topology, toolchain envelope,
/// known limits, rollback target, diagnostics posture), preserves its rollback and diagnostics posture before
/// widening so a cohort never widens without it, and keeps a public-facing cohort's support language matched to
/// cohort proof.
pub fn resolve_widening_decision_entry(
    input: M5WideningDecisionEntryResolutionInput,
) -> Result<M5ResolvedWideningDecisionEntry, M5WideningDecisionResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5WideningDecisionResolutionError::EmptyWideningDecisionEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.widening_event_binding_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.final_decision_reference)
        || string_is_forbidden(&input.open_risks_reference)
        || string_is_forbidden(&input.narrowed_claims_reference)
        || string_is_forbidden(&input.on_call_roster_reference)
        || string_is_forbidden(&input.signoff_roster_reference)
        || string_is_forbidden(&input.evidence_snapshot_reference)
        || string_is_forbidden(&input.decision_freshness_reference)
    {
        return Err(M5WideningDecisionResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = widening_decision_object_is_complete(
        input.widening_decision_packet_kind,
        &input.final_decision_reference,
        &input.open_risks_reference,
        &input.narrowed_claims_reference,
        &input.on_call_roster_reference,
        &input.signoff_roster_reference,
        &input.evidence_snapshot_reference,
        &input.decision_freshness_reference,
    );
    let preserve_ok = widening_decision_stays_documented_before_widening(
        input.widening_decision_packet_kind,
        input.widening_decision_documented_before_widening,
        input.requires_documented_exception,
        input.attributable_asset_or_approved_exception,
    );
    let support_undisclosed =
        input.requires_documented_exception && !input.attributable_asset_or_approved_exception;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5WideningDecisionEntryDegradeReason::WideningDecisionTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5WideningDecisionEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.widening_decision_packet_kind.is_classified() {
        Some(M5WideningDecisionEntryDegradeReason::WideningDecisionPacketKindUnclassified)
    } else if !input.bound_to_registry {
        Some(M5WideningDecisionEntryDegradeReason::WideningDecisionNotBoundToRegistry)
    } else if !object_complete {
        Some(M5WideningDecisionEntryDegradeReason::WideningDecisionObjectIncomplete)
    } else if !preserve_ok {
        Some(M5WideningDecisionEntryDegradeReason::WideningDecisionWidensScopeUndocumentedOrRunsClaimAheadOfProof)
    } else if !all_forms {
        Some(M5WideningDecisionEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if support_undisclosed {
        Some(M5WideningDecisionEntryDegradeReason::WideningDecisionUndocumentedForScopeWidening)
    } else if !input.proof_fresh {
        Some(M5WideningDecisionEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5WideningDecisionNextAction::ExpandWideningDecisionMeaning,
    };

    Ok(M5ResolvedWideningDecisionEntry {
        entry_id: input.entry_id,
        widening_event_binding_id: input.widening_event_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        widening_decision_packet_kind: input.widening_decision_packet_kind.as_str().to_owned(),
        widening_decision_packet_kind_is_classified: input
            .widening_decision_packet_kind
            .is_classified(),
        canonical_widening_decision_packet_kind_mode: input
            .widening_decision_packet_kind
            .canonical_widening_decision_packet_kind_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        final_decision_reference: input.final_decision_reference,
        open_risks_reference: input.open_risks_reference,
        narrowed_claims_reference: input.narrowed_claims_reference,
        on_call_roster_reference: input.on_call_roster_reference,
        signoff_roster_reference: input.signoff_roster_reference,
        evidence_snapshot_reference: input.evidence_snapshot_reference,
        decision_freshness_reference: input.decision_freshness_reference,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        widening_decision_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        widening_decision_documented_before_widening: input
            .widening_decision_documented_before_widening,
        requires_documented_exception: input.requires_documented_exception,
        attributable_asset_or_approved_exception: input.attributable_asset_or_approved_exception,
        degrade_reason,
        next_action,
        widening_decision_resolves_across_classes: degrade_reason.is_none(),
    })
}

/// Resolves a ring-history entry so its evidence stays safe: the entry names its canonical token,
/// semantic role, and evidence scope, covers all three resolution forms, provides the complete cohort-identity /
/// known-limits-ledger / rollback-target / rehearsal-currency / readiness-signoff / support-language /
/// last-widening-revision ring-history object, and degrades honestly when the evidence would run partner /
/// public support language ahead of cohort proof, hide the cohort evidence, or let a known-limits gap masquerade
/// as covered.
pub fn resolve_ring_history_entry(
    input: M5RingHistoryEntryResolutionInput,
) -> Result<M5ResolvedRingHistoryEntry, M5WideningDecisionResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5WideningDecisionResolutionError::EmptyRingHistoryEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.ring_history_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_coverage_identity)
        || string_is_forbidden(&input.evidence_snapshot_ledger)
        || string_is_forbidden(&input.orr_signoff_reference)
        || string_is_forbidden(&input.on_call_roster_state)
        || string_is_forbidden(&input.ring_history_freshness_state)
        || string_is_forbidden(&input.widening_stage_reference)
        || string_is_forbidden(&input.last_ring_history_revision)
    {
        return Err(M5WideningDecisionResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = ring_history_stays_honest(
        input.ring_history_coverage,
        input.ring_history_lineage_is_truthful,
        input.keeps_evidence_snapshot_visible,
        input.override_without_evidence_requested,
        input.blocked_until_evidence_linked,
        input.lineage_gap_present,
        input.lineage_gap_flagged,
    );
    let provides_record = input.ring_history_coverage.is_classified()
        && !input.resolved_coverage_identity.trim().is_empty()
        && !input.evidence_snapshot_ledger.trim().is_empty()
        && !input.orr_signoff_reference.trim().is_empty()
        && !input.on_call_roster_state.trim().is_empty()
        && !input.ring_history_freshness_state.trim().is_empty()
        && !input.widening_stage_reference.trim().is_empty()
        && !input.last_ring_history_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5RingHistoryEntryDegradeReason::RingHistoryTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5RingHistoryEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.ring_history_coverage.is_classified() {
        Some(M5RingHistoryEntryDegradeReason::RingHistoryCoverageUnclassified)
    } else if !provides_record {
        Some(M5RingHistoryEntryDegradeReason::RingHistoryDropsEvidenceOrImpliesGreenWhileStale)
    } else if !all_forms {
        Some(M5RingHistoryEntryDegradeReason::RingHistoryFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5RingHistoryEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5WideningDecisionNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedRingHistoryEntry {
        entry_id: input.entry_id,
        ring_history_ref: input.ring_history_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        ring_history_coverage: input.ring_history_coverage.as_str().to_owned(),
        ring_history_coverage_is_classified: input.ring_history_coverage.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        resolved_coverage_identity: input.resolved_coverage_identity,
        evidence_snapshot_ledger: input.evidence_snapshot_ledger,
        orr_signoff_reference: input.orr_signoff_reference,
        on_call_roster_state: input.on_call_roster_state,
        ring_history_freshness_state: input.ring_history_freshness_state,
        widening_stage_reference: input.widening_stage_reference,
        last_ring_history_revision: input.last_ring_history_revision,
        keeps_evidence_snapshot_visible: input.keeps_evidence_snapshot_visible,
        ring_history_lineage_is_truthful: input.ring_history_lineage_is_truthful,
        override_without_evidence_requested: input.override_without_evidence_requested,
        blocked_until_evidence_linked: input.blocked_until_evidence_linked,
        lineage_gap_present: input.lineage_gap_present,
        lineage_gap_flagged: input.lineage_gap_flagged,
        ring_history_stays_honest: record_stays_honest,
        provides_complete_ring_history_record: provides_record,
        degrade_reason,
        next_action,
        ring_history_safe_on_every_coverage: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved widening-decision and ring-history
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WideningDecisionRingHistoryRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5WideningDecisionRingHistoryRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5WideningDecisionAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5WideningDecisionExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    /// Resolved widening-decision-registry examples.
    pub widening_decision_entries: Vec<M5ResolvedWideningDecisionEntry>,
    /// Resolved ring-history examples.
    pub ring_history_entries: Vec<M5ResolvedRingHistoryEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the widening-decision and
    /// ring-history domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never widens a cohort without current rollback and diagnostics evidence. MUST be
    /// `false`.
    pub widens_a_stable_claim_without_a_durable_go_no_go_record: bool,
    /// Hard invariant: this row never runs partner or public support language ahead of cohort proof. MUST be
    /// `false`.
    pub drops_the_evidence_snapshot_or_roster_from_a_widening_record: bool,
    /// Hard invariant: this row never hides the rollback target or diagnostics posture before widening. MUST be
    /// `false`.
    pub hides_the_ring_history_or_prior_blockers_before_widening: bool,
    /// Hard invariant: this row never collapses distinct cohort evidence classes into one lane. MUST be `false`.
    pub implies_green_when_go_no_go_records_or_evidence_snapshots_are_stale: bool,
}

impl M5WideningDecisionRingHistoryRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5WideningDecisionAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5WideningDecisionAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5WideningDecisionExportField> =
            self.export_fields.iter().copied().collect();
        M5WideningDecisionExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.widens_a_stable_claim_without_a_durable_go_no_go_record
            && !self.drops_the_evidence_snapshot_or_roster_from_a_widening_record
            && !self.hides_the_ring_history_or_prior_blockers_before_widening
            && !self.implies_green_when_go_no_go_records_or_evidence_snapshots_are_stale
    }

    /// True when a clean widening-decision entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified cohort archetype, publishes a complete descriptor object, preserves its rollback and
    /// diagnostics posture, covers all three resolution forms, and keeps a public-facing cohort's support
    /// language matched to proof.
    fn descriptor_is_honest(ex: &M5ResolvedWideningDecisionEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.widening_decision_packet_kind_is_classified
                && ex.widening_decision_object_complete
                && ex.widening_decision_documented_before_widening
                && ex.covers_all_resolution_forms
                && (!ex.requires_documented_exception
                    || ex.attributable_asset_or_approved_exception))
    }

    /// True when a clean ring-history entry preserves a safe packet: it keeps a classified evidence
    /// scope, provides the complete ring-history object, stays honest, and covers all three resolution forms.
    fn evidence_is_honest(ex: &M5ResolvedRingHistoryEntry) -> bool {
        !ex.is_clean()
            || (ex.ring_history_coverage_is_classified
                && ex.provides_complete_ring_history_record
                && ex.ring_history_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.widening_decision_entries
            .iter()
            .all(Self::descriptor_is_honest)
            && self
                .ring_history_entries
                .iter()
                .all(Self::evidence_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WideningDecisionRingHistoryRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Cohort-archetype tokens (minted by this lane).
    pub widening_decision_packet_kinds: Vec<String>,
    /// Evidence-scope tokens (minted by this lane).
    pub ring_history_coverages: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Cohort-descriptor-entry degrade-reason tokens.
    pub widening_decision_degrade_reasons: Vec<String>,
    /// Cohort-evidence-packet-entry degrade-reason tokens.
    pub ring_history_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5WideningDecisionRingHistoryRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5LaunchControlRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5WideningDecisionResolutionForm::ALL, |v| v.as_str()),
            widening_decision_packet_kinds: tokens(&M5WideningDecisionPacketKind::ALL, |v| {
                v.as_str()
            }),
            ring_history_coverages: tokens(&M5RingHistoryCoverageKind::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5WideningDecisionSurfaceContext::ALL, |v| v.as_str()),
            widening_decision_degrade_reasons: tokens(
                &M5WideningDecisionEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            ring_history_degrade_reasons: tokens(&M5RingHistoryEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5WideningDecisionAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5WideningDecisionNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5WideningDecisionExportField::ALL, |v| v.as_str()),
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
pub struct M5WideningDecisionRingHistoryRegistriesGovernanceReview {
    /// The descriptor registry names a canonical token, semantic role, and cohort archetype for every entry.
    pub widening_decision_registry_names_token_role_and_type: bool,
    /// Every claimed cohort resolves to one typed widening-decision object from the shared registry, not
    /// per-entry reconstruction.
    pub type_resolves_to_typed_widening_decision_from_shared_registry: bool,
    /// The exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles are published for every resolved descriptor.
    pub build_row_and_cohort_lineage_published: bool,
    /// Cohorts cannot widen without preserving rollback and diagnostics posture before widening.
    pub scope_cannot_widen_without_documented_widening_decision: bool,
    /// The cohort evidence keeps the cohort proof visible and binds partner / public support language to it.
    pub ring_history_keeps_evidence_visible_and_blocks_stale_green: bool,
    /// Partner / public support language stays matched to cohort proof for every public-facing cohort.
    pub approved_exception_matched_to_scope_for_widening: bool,
    /// Every widening-decision and ring-history entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Cohort-descriptor and ring-history behavior stay bound to the shared registries rather than
    /// hand-copied per cohort.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shiproom, release center, executive steering, and program governance read a single cohort source.
    pub shiproom_release_center_executive_steering_and_program_governance_read_single_source: bool,
    /// A widen-without-rollback attempt, an incomplete object, or hidden cohort evidence is caught by fixtures
    /// before release evidence turns green.
    pub exception_or_ring_history_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WideningDecisionRingHistoryRegistriesConsumerProjection {
    /// Shiproom and release center consume the shared widening-decision registry.
    pub shiproom_and_release_center_consume_shared_registries: bool,
    /// Executive steering and program governance consume the shared ring-history registry.
    pub executive_steering_and_program_governance_consume_shared_registries: bool,
    /// Diagnostics and public proof consume the shared registries.
    pub diagnostics_and_public_proof_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical widening-decision and ring-history domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical widening-decision / ring-history registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WideningDecisionRingHistoryRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WideningDecisionRingHistoryRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting cohort audit for the lane.
    pub ring_history_control_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5WideningDecisionRingHistoryRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WideningDecisionRingHistoryRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5WideningDecisionRingHistoryRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WideningDecisionRingHistoryRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WideningDecisionRingHistoryRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WideningDecisionRingHistoryRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WideningDecisionRingHistoryRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WideningDecisionRingHistoryRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 widening-decision and ring-history registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WideningDecisionRingHistoryRegistriesPacket {
    /// Record kind; must equal [`M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5WideningDecisionRingHistoryRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WideningDecisionRingHistoryRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WideningDecisionRingHistoryRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WideningDecisionRingHistoryRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WideningDecisionRingHistoryRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WideningDecisionRingHistoryRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5WideningDecisionRingHistoryRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5WideningDecisionRingHistoryRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5WideningDecisionRingHistoryRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_RECORD_KIND {
            violations.push(M5WideningDecisionRingHistoryRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5WideningDecisionRingHistoryRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5WideningDecisionRingHistoryRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5WideningDecisionRingHistoryRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 widening-decision / ring-history registries packet serializes"),
        ) {
            violations.push(M5WideningDecisionRingHistoryRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 widening-decision / ring-history registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,widening_decision_entries,ring_history_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .widening_decision_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.ring_history_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.widening_decision_entries.len(),
                row.ring_history_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Stable Go/No-Go Widening-Decision and Ring-History Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Widening-decision kinds: {}\n",
            self.vocabulary_set
                .widening_decision_packet_kinds
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
                "  - Widening-decision entries: {} / ring-history entries: {}\n",
                row.widening_decision_entries.len(),
                row.ring_history_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry cohort reference table generated from the registry, so docs and shiproom runbooks
    /// render the same archetype-mode / repo-rows / bundle-ids / install-topology / toolchain-envelope /
    /// rollback-target truth the resolvers produced rather than a hand-copied cohort table. Only clean,
    /// registry-bound widening-decision entries are listed.
    pub fn render_widening_decision_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| widening_event_binding_id | packet_kind_mode | final_decision_reference | open_risks_reference | narrowed_claims_reference | on_call_roster_reference | evidence_snapshot_reference |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.widening_decision_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.widening_event_binding_id,
                    ex.canonical_widening_decision_packet_kind_mode,
                    ex.final_decision_reference,
                    ex.open_risks_reference,
                    ex.narrowed_claims_reference,
                    ex.on_call_roster_reference,
                    ex.evidence_snapshot_reference
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5WideningDecisionRingHistoryRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5WideningDecisionRingHistoryRegistriesViolation>),
}

impl fmt::Display for M5WideningDecisionRingHistoryRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 widening-decision / ring-history registries export parse failed: {error}"
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
                    "m5 widening-decision / ring-history registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5WideningDecisionRingHistoryRegistriesArtifactError {}

/// Validation failures emitted by [`M5WideningDecisionRingHistoryRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5WideningDecisionRingHistoryRegistriesViolation {
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
    /// A registry row does not point at both the widening-decision and ring-history domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, widen-without-rollback, field-incomplete,
    /// form-incomplete, or a ring-history entry missing the complete evidence object).
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
    WideningDecisionResolutionNotProven,
    /// Rollback-and-diagnostics-preservation is not proven: no widen-without-rollback example and no unbound
    /// example degrade, no clean bounded descriptor entry is present, or a clean descriptor entry is unbounded
    /// or unbound.
    RingHistoryAttributionNotProven,
    /// Cohort-evidence-integrity is not proven: clean evidence entries do not cover the canonical dogfood-ring /
    /// rehearsal-currency / ring-history-signoff scopes with full resolution-form coverage while providing the
    /// complete evidence object, no support-ahead or form-incomplete example degrades, or a clean evidence entry
    /// is missing the complete evidence object.
    RingHistoryIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5WideningDecisionRingHistoryRegistriesViolation {
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
            Self::WideningDecisionResolutionNotProven => "widening_decision_resolution_not_proven",
            Self::RingHistoryAttributionNotProven => "ring_history_attribution_not_proven",
            Self::RingHistoryIntegrityNotProven => "ring_history_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_widening_decision_and_ring_history_registries_export() -> Result<
    M5WideningDecisionRingHistoryRegistriesPacket,
    M5WideningDecisionRingHistoryRegistriesArtifactError,
> {
    let packet: M5WideningDecisionRingHistoryRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-widening-decision-and-ring-history-registries-proof/support_export.json"
        )
    ))
    .map_err(M5WideningDecisionRingHistoryRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5WideningDecisionRingHistoryRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5WideningDecisionRingHistoryRegistriesPacket,
    violations: &mut Vec<M5WideningDecisionRingHistoryRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_SCHEMA_REF,
        M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_DOC_REF,
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_WIDENING_DECISION_DOMAIN_SCHEMA_REF,
        M5_RING_HISTORY_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5WideningDecisionRingHistoryRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5WideningDecisionRingHistoryRegistriesPacket,
    violations: &mut Vec<M5WideningDecisionRingHistoryRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5WideningDecisionRingHistoryRegistriesViolation::NoRegistryRows);
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
                .push(M5WideningDecisionRingHistoryRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations
                .push(M5WideningDecisionRingHistoryRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5WideningDecisionRingHistoryRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_WIDENING_DECISION_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_RING_HISTORY_DOMAIN_SCHEMA_REF)
        {
            violations
                .push(M5WideningDecisionRingHistoryRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.widening_decision_entries.is_empty() || row.ring_history_entries.is_empty() {
            violations.push(M5WideningDecisionRingHistoryRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5WideningDecisionRingHistoryRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5WideningDecisionRingHistoryRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5WideningDecisionRingHistoryRegistriesPacket,
    violations: &mut Vec<M5WideningDecisionRingHistoryRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.widening_decision_registry_names_token_role_and_type,
        review.type_resolves_to_typed_widening_decision_from_shared_registry,
        review.build_row_and_cohort_lineage_published,
        review.scope_cannot_widen_without_documented_widening_decision,
        review.ring_history_keeps_evidence_visible_and_blocks_stale_green,
        review.approved_exception_matched_to_scope_for_widening,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shiproom_release_center_executive_steering_and_program_governance_read_single_source,
        review.exception_or_ring_history_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5WideningDecisionRingHistoryRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5WideningDecisionRingHistoryRegistriesPacket,
    violations: &mut Vec<M5WideningDecisionRingHistoryRegistriesViolation>,
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
                M5WideningDecisionRingHistoryRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5WideningDecisionRingHistoryRegistriesPacket,
    violations: &mut Vec<M5WideningDecisionRingHistoryRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5WideningDecisionRingHistoryRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5WideningDecisionRingHistoryRegistriesPacket,
    violations: &mut Vec<M5WideningDecisionRingHistoryRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.ring_history_control_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5WideningDecisionRingHistoryRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5WideningDecisionRingHistoryRegistriesPacket,
    violations: &mut Vec<M5WideningDecisionRingHistoryRegistriesViolation>,
) {
    let descriptors = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.widening_decision_entries.iter())
    };
    let evidence = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.ring_history_entries.iter())
    };

    // AC1: every active cohort can be inspected by exact rows, bundles, toolchains, and deployment profiles.
    // Clean descriptor entries cover the canonical cohort archetypes and the first release-center / shiproom /
    // executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no
    // clean descriptor entry published an incomplete object.
    let clean_archetypes: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.widening_decision_packet_kind.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let archetypes_covered = M5WideningDecisionPacketKind::CANONICAL_PACKET_KINDS
        .iter()
        .all(|k| clean_archetypes.contains(k.as_str()));
    let first_surfaces_covered = M5WideningDecisionSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(M5WideningDecisionEntryDegradeReason::WideningDecisionObjectIncomplete)
    });
    let no_clean_incomplete =
        !descriptors().any(|ex| ex.is_clean() && !ex.widening_decision_object_complete);
    if !(archetypes_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5WideningDecisionRingHistoryRegistriesViolation::WideningDecisionResolutionNotProven,
        );
    }

    // AC2: cohort packets preserve rollback and diagnostics posture before widening. A widen-without-rollback
    // example degrades, an unbound example degrades, at least one clean bounded descriptor entry is present, and
    // no clean descriptor entry is unbounded or unbound.
    let widen_fold_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(
                M5WideningDecisionEntryDegradeReason::WideningDecisionWidensScopeUndocumentedOrRunsClaimAheadOfProof,
            )
    });
    let unbound_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(M5WideningDecisionEntryDegradeReason::WideningDecisionNotBoundToRegistry)
    });
    let bounded_clean_descriptor =
        descriptors().any(|ex| ex.is_clean() && ex.widening_decision_documented_before_widening);
    let no_clean_unbound = !descriptors().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded =
        !descriptors().any(|ex| ex.is_clean() && !ex.widening_decision_documented_before_widening);
    if !(widen_fold_degrades
        && unbound_degrades
        && bounded_clean_descriptor
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations.push(
            M5WideningDecisionRingHistoryRegistriesViolation::RingHistoryAttributionNotProven,
        );
    }

    // AC3: claim publication can prove which cohort evidence backs each launch-bearing lane. Clean evidence
    // entries cover every canonical dogfood-ring / rehearsal-currency / ring-history-signoff scope with full
    // resolution-form coverage while providing the complete evidence object, a support-ahead example degrades, a
    // form-incomplete example degrades, and no clean evidence entry is missing the complete object.
    let clean_ring_history_coverages: BTreeSet<String> = evidence()
        .filter(|ex| {
            ex.is_clean()
                && ex.ring_history_coverage_is_classified
                && ex.provides_complete_ring_history_record
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.ring_history_coverage.clone())
        .collect();
    let ring_history_coverages_covered = M5RingHistoryCoverageKind::CANONICAL_COVERAGES
        .iter()
        .all(|m| clean_ring_history_coverages.contains(m.as_str()));
    let support_ahead_degrades = evidence().any(|ex| {
        ex.degrade_reason
            == Some(
                M5RingHistoryEntryDegradeReason::RingHistoryDropsEvidenceOrImpliesGreenWhileStale,
            )
    });
    let form_incomplete_degrades = evidence().any(|ex| {
        ex.degrade_reason
            == Some(M5RingHistoryEntryDegradeReason::RingHistoryFormCoverageIncomplete)
    });
    let no_clean_missing_evidence =
        !evidence().any(|ex| ex.is_clean() && !ex.provides_complete_ring_history_record);
    if !(ring_history_coverages_covered
        && support_ahead_degrades
        && form_incomplete_degrades
        && no_clean_missing_evidence)
    {
        violations
            .push(M5WideningDecisionRingHistoryRegistriesViolation::RingHistoryIntegrityNotProven);
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

/// The widening stages this lane governs, for downstream reference: the widening-decision registry defines the
/// minimum evidence and soak expectations that let a lane advance across the alpha, beta, release-candidate,
/// stable, and long-term-support widening stages, and the ring-history registry records the conditions that
/// immediately stop that progression.
pub const IMPLEMENTED_WIDENING_DECISION_STAGES: [M5LaunchControlWideningStage; 5] =
    M5LaunchControlWideningStage::ALL;
