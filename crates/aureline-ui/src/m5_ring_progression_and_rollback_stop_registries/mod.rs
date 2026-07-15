//! Implemented M5 ring-progression and rollback-stop registries.
//!
//! The frozen [launch-control matrix][matrix] names Aureline's governed launch-bearing cohorts and locks their
//! controlled vocabulary. This module governs *ring widening by explicit stop conditions rather than schedule
//! optimism*: it turns the *ring-progression* grammar (how each widening transition — canary, broad internal
//! dogfood, design-partner preview, public preview, and certified stable — declares its minimum entry evidence,
//! soak-window expectation, why widening is allowed, its known-limits packet, issue-template linkage,
//! claim-narrowing action, and the rollback-stop reference that immediately stops it) and the *rollback-stop*
//! grammar (how a launch-bearing lane records the rollback-stop condition — a crash / data-loss / trust defect,
//! a repeated protected-metric regression, or a stale readiness packet — that halts ring progression while it is
//! active) into registry resolvers that produce export-safe, honest projections. Every claimed M5 ring
//! transition then resolves to one typed ring-progression object — the widening transition it classifies, the
//! minimum entry evidence, the soak-window expectation, the widening-allow rationale, the known-limits packet,
//! the issue-template ref, the claim-narrowing action, and the rollback-stop reference, all visible before
//! widening so a ring never advances without its known-limits and rollback-stop posture and so partner / public
//! support language never outruns current ring proof — and to one rollback-stop object — the resolved transition
//! identity, the active stop-condition ledger, the rollback-stop target reference, the protected-metric
//! regression state, the packet-freshness state, the crash / data-loss / trust reference, and the last
//! ring-transition revision — that the shiproom, release-center, executive-steering, program-governance, and
//! support / export surfaces can inspect without manual reconstruction, so every ring transition can state why
//! widening is allowed and what immediately stops it, known-limits and rollback posture stay visible before any
//! ring widens, ring progression can never advance on a claimed lane while a rollback-stop condition is active,
//! and a ring that cannot explain the progression rule it declared or the stop condition that backs it degrades
//! honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one typed ring-progression object per widening transition.** [`resolve_ring_progression_entry`]
//!   refuses to read as a clean, registry-bound progression entry unless it names a canonical registry token, a
//!   classified [ring widening transition][M5RingWideningTransitionKind], a launch-control role, covers every
//!   [resolution form][M5RingResolutionForm] (the canonical object, the accessible summary, and the audit
//!   record), publishes every progression field (minimum entry evidence, soak-window expectation, widening-allow
//!   rationale, issue-template ref, known limits, claim-narrowing action, and rollback-stop reference), keeps its
//!   known-limits and rollback-stop posture visible before widening, and keeps partner / public support language
//!   matched to ring proof; otherwise it degrades.
//! * **Keep a ring from advancing without a visible rollback-stop and known-limits posture.**
//!   [`ring_states_stop_and_rollback_before_widening`] rejects a progression entry whose rollback-stop and
//!   known-limits posture is not visible (a ring advancing without a rollback-stop reference and known limits) so
//!   it degrades to
//!   [`M5RingProgressionEntryDegradeReason::RingAdvancesWithoutRollbackStopOrRunsSupportAheadOfProof`],
//!   and a public-facing ring whose support language runs ahead of ring proof degrades the same way — the
//!   structured blocker reason a widen-without-stop attempt must surface.
//! * **Keep the rollback-stop record from advancing a ring while a stop condition is active.**
//!   [`resolve_rollback_stop_entry`] names a classified [rollback-stop condition][M5RollbackStopConditionKind],
//!   requires the full transition-identity / active-stop-condition-ledger / rollback-stop-target /
//!   protected-metric-regression / packet-freshness / crash-data-loss-or-trust / last-ring-transition-revision
//!   record, covers every resolution form, and degrades to
//!   [`M5RollbackStopEntryDegradeReason::RollbackStopAdvancesRingWhileActiveOrDropsStopEvidence`]
//!   when the record would advance a ring while a stop condition is active, hide the rollback-stop, or let a
//!   protected-metric regression masquerade as covered, so a rollback-stop record can never read as trustworthy
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
    seeded_m5_ring_progression_and_rollback_stop_registries,
    seeded_m5_ring_progression_and_rollback_stop_registries_ring_progression_beta_narrowed,
    seeded_m5_ring_progression_and_rollback_stop_registries_rollback_stop_preview_narrowed,
    M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_PACKET_ID,
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

/// Stable record-kind tag carried by [`M5RingProgressionRollbackStopRegistriesPacket`].
pub const M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_ring_progression_and_rollback_stop_registries";

/// Schema version for M5 ring-progression / rollback-stop registry records.
pub const M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_SCHEMA_REF: &str =
    "schemas/program/m5-ring-progression-and-rollback-stop-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_DOC_REF: &str =
    "docs/release/m5_ring_progression_and_rollback_stop_registries.md";

/// Repo-relative path of the canonical ring-progression domain schema minted by this lane (how a widening ring
/// transition declares its minimum entry evidence, soak-window expectation, why widening is allowed, its
/// known-limits packet, issue-template linkage, claim-narrowing action, and the rollback-stop reference that
/// immediately stops it).
pub const M5_RING_PROGRESSION_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-ring-progression.schema.json";

/// Repo-relative path of the canonical rollback-stop domain schema minted by this lane (how a launch-bearing lane
/// records the rollback-stop condition — a crash / data-loss / trust defect, a repeated protected-metric
/// regression, or a stale readiness packet — that halts ring progression while it is active).
pub const M5_ROLLBACK_STOP_DOMAIN_SCHEMA_REF: &str = "schemas/program/m5-rollback-stop.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-ring-progression-and-rollback-stop-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-ring-progression-and-rollback-stop-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-ring-progression-and-rollback-stop-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/release/m5-ring-progression-and-rollback-stop-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// cohort invents a parallel surface set.
pub type M5RingProgressionRollbackStopRegistriesConsumerSurface = M5LaunchControlConsumerSurface;

/// One of the three resolution forms every cohort-descriptor or cohort-evidence-packet entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// cohort-descriptor and cohort-evidence *domains* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RingResolutionForm {
    /// The canonical resolved cohort-descriptor / cohort-evidence-packet object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved cohort discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved cohort inspectable off-renderer.
    AuditRecord,
}

impl M5RingResolutionForm {
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
pub enum M5RingWideningTransitionKind {
    /// The internal dogfood core-team canary cohort.
    CanaryWidening,
    /// The migration alpha cohort (external alpha migrating from a prior toolchain).
    BroadInternalDogfoodWidening,
    /// The extension-author cohort (compatibility rehearsals current, freeze exceptions documented).
    ExtensionAuthorWidening,
    /// The design-partner preview cohort (public-facing; support language must match cohort proof).
    DesignPartnerPreviewWidening,
    /// The public preview cohort (public-facing; support language must match cohort proof).
    PublicPreviewWidening,
    /// The certified-archetype cohort (ORR signed and a go/no-go decision recorded).
    CertifiedStableWidening,
    /// The cohort archetype is unclassified, which is disallowed.
    TransitionUnclassified,
}

impl M5RingWideningTransitionKind {
    /// Every cohort archetype, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::CanaryWidening,
        Self::BroadInternalDogfoodWidening,
        Self::ExtensionAuthorWidening,
        Self::DesignPartnerPreviewWidening,
        Self::PublicPreviewWidening,
        Self::CertifiedStableWidening,
        Self::TransitionUnclassified,
    ];

    /// The six canonical cohort archetypes every claimed M5 launch-bearing cohort classifies against.
    pub const CANONICAL_TRANSITIONS: [Self; 6] = [
        Self::CanaryWidening,
        Self::BroadInternalDogfoodWidening,
        Self::ExtensionAuthorWidening,
        Self::DesignPartnerPreviewWidening,
        Self::PublicPreviewWidening,
        Self::CertifiedStableWidening,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanaryWidening => "canary_widening",
            Self::BroadInternalDogfoodWidening => "broad_internal_dogfood_widening",
            Self::ExtensionAuthorWidening => "extension_author_widening",
            Self::DesignPartnerPreviewWidening => "design_partner_preview_widening",
            Self::PublicPreviewWidening => "public_preview_widening",
            Self::CertifiedStableWidening => "certified_stable_widening",
            Self::TransitionUnclassified => "transition_unclassified",
        }
    }

    /// Whether the archetype is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::TransitionUnclassified)
    }

    /// The canonical mode for this cohort archetype.
    pub const fn canonical_ring_widening_transition_mode(self) -> &'static str {
        match self {
            Self::CanaryWidening => "canary_widening_transition",
            Self::BroadInternalDogfoodWidening => "broad_internal_dogfood_widening_transition",
            Self::ExtensionAuthorWidening => "extension_author_widening_widening_transition",
            Self::DesignPartnerPreviewWidening => {
                "design_partner_preview_widening_widening_transition"
            }
            Self::PublicPreviewWidening => "public_preview_widening_widening_transition",
            Self::CertifiedStableWidening => "certified_stable_widening_transition",
            Self::TransitionUnclassified => "",
        }
    }

    /// Whether this archetype is public-facing and so must keep partner / public support language matched to
    /// cohort proof before the cohort widens.
    pub const fn is_public_facing_ring(self) -> bool {
        matches!(
            self,
            Self::DesignPartnerPreviewWidening | Self::PublicPreviewWidening
        )
    }
}

/// Controlled evidence scope a cohort-evidence-packet entry must resolve its cohort proof from, so an evidence
/// packet shares one registry rather than a hand-copied per-record assumption. Minted by this lane, tracking
/// whether the evidence came from dogfood-ring telemetry, current rehearsal cadence, or an explicit go/no-go
/// signoff the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackStopConditionKind {
    /// The evidence came from internal dogfood-ring telemetry.
    CrashDataLossOrTrustDefect,
    /// The evidence came from current rehearsal cadence (publish/rollback, mixed-version, handoff drills).
    RepeatedProtectedMetricRegression,
    /// The evidence came from an explicit go/no-go signoff with a preserved evidence snapshot.
    StaleReadinessPacket,
    /// The evidence scope is unclassified, which is disallowed.
    ConditionUnclassified,
}

impl M5RollbackStopConditionKind {
    /// Every evidence scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CrashDataLossOrTrustDefect,
        Self::RepeatedProtectedMetricRegression,
        Self::StaleReadinessPacket,
        Self::ConditionUnclassified,
    ];

    /// The three canonical evidence scopes every cohort-evidence packet must stay distinct across.
    pub const CANONICAL_CONDITIONS: [Self; 3] = [
        Self::CrashDataLossOrTrustDefect,
        Self::RepeatedProtectedMetricRegression,
        Self::StaleReadinessPacket,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashDataLossOrTrustDefect => "crash_data_loss_or_trust_defect",
            Self::RepeatedProtectedMetricRegression => "repeated_protected_metric_regression",
            Self::StaleReadinessPacket => "stale_readiness_packet",
            Self::ConditionUnclassified => "condition_unclassified",
        }
    }

    /// Whether the evidence scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ConditionUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a cohort-descriptor or
/// cohort-evidence-packet token's meaning stays stable whether it appears in the release-center, shiproom,
/// executive-steering, program-governance, or a support / export form. Minted by this lane, tracking the
/// first-consumer surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RingSurfaceContext {
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

impl M5RingSurfaceContext {
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
pub enum M5RingAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The cohort archetype the entry classifies (cohort-descriptor entry).
    RingWideningTransition,
    /// The exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles the entry publishes (cohort-descriptor entry).
    RingEvidenceAndSoakRows,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The known-limits packet and rollback target the entry preserves before widening (cohort-descriptor
    /// entry).
    KnownLimitsAndRollbackStop,
    /// The cohort-evidence fields (cohort identity, known-limits ledger, rollback target, rehearsal currency,
    /// readiness signoff, support language) the entry publishes (cohort-evidence-packet entry).
    RollbackStopFields,
    /// The support-identity hint the entry publishes (cohort-evidence-packet entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved cohort descriptor or cohort evidence (both entries).
    PlainLanguageMeaning,
}

impl M5RingAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::RingWideningTransition,
        Self::RingEvidenceAndSoakRows,
        Self::ResolutionFormCoverage,
        Self::KnownLimitsAndRollbackStop,
        Self::RollbackStopFields,
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
            Self::RingWideningTransition => "ring_widening_transition",
            Self::RingEvidenceAndSoakRows => "ring_evidence_and_soak_rows",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::KnownLimitsAndRollbackStop => "known_limits_and_claim_narrowing_action",
            Self::RollbackStopFields => "rollback_stop_fields",
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
pub enum M5RingNextAction {
    /// Expand the resolved cohort descriptor's or cohort-evidence packet's plain-language meaning.
    ExpandRingMeaning,
    /// Inspect the cohort archetype or evidence scope the entry resolves.
    InspectTransitionOrCondition,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5RingNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandRingMeaning,
        Self::InspectTransitionOrCondition,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandRingMeaning => "expand_ring_meaning",
            Self::InspectTransitionOrCondition => "inspect_transition_or_condition",
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
pub enum M5RingExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The cohort families covered.
    RingFamilies,
    /// The cohort archetypes carried.
    RingWideningTransitions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The evidence scopes carried.
    RollbackStopConditions,
    /// The render / surface context.
    SurfaceContext,
    /// The cohort-archetype modes carried.
    RingWideningTransitionModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5RingExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::RingFamilies,
        Self::RingWideningTransitions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::RollbackStopConditions,
        Self::SurfaceContext,
        Self::RingWideningTransitionModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::RingFamilies,
        Self::RingWideningTransitions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::RingFamilies => "ring_families",
            Self::RingWideningTransitions => "ring_widening_transitions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::RollbackStopConditions => "rollback_stop_conditions",
            Self::SurfaceContext => "surface_context",
            Self::RingWideningTransitionModes => "ring_widening_transition_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a cohort-descriptor entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, widen-without-rollback, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RingProgressionEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the descriptor means.
    RingTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The cohort archetype is unclassified (not in the resolved taxonomy).
    RingWideningTransitionUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    RingProgressionNotBoundToRegistry,
    /// The resolved cohort-descriptor object is incomplete: the exact repo / archetype rows, bundle IDs, install
    /// topology, toolchain envelope, known limits, rollback target, or diagnostics posture is unstated.
    RingProgressionObjectIncomplete,
    /// The cohort's rollback and diagnostics posture is not preserved before widening (a cohort widening without
    /// a rollback target and diagnostics posture), or a public-facing cohort ran its support language ahead of
    /// cohort proof.
    RingAdvancesWithoutRollbackStopOrRunsSupportAheadOfProof,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A public-facing cohort did not keep its support language matched to cohort proof before widening.
    RollbackStopNotVisibleForPublicRing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RingProgressionEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RingTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::RingWideningTransitionUnclassified,
        Self::RingProgressionNotBoundToRegistry,
        Self::RingProgressionObjectIncomplete,
        Self::RingAdvancesWithoutRollbackStopOrRunsSupportAheadOfProof,
        Self::ResolutionFormCoverageIncomplete,
        Self::RollbackStopNotVisibleForPublicRing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RingTokenUnstated => "ring_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::RingWideningTransitionUnclassified => "ring_widening_transition_unclassified",
            Self::RingProgressionNotBoundToRegistry => "ring_progression_not_bound_to_registry",
            Self::RingProgressionObjectIncomplete => "ring_progression_object_incomplete",
            Self::RingAdvancesWithoutRollbackStopOrRunsSupportAheadOfProof => {
                "ring_advances_without_rollback_stop_or_runs_support_ahead_of_proof"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::RollbackStopNotVisibleForPublicRing => {
                "rollback_stop_not_visible_for_public_ring"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5RingNextAction {
        match self {
            Self::RingTokenUnstated | Self::RingProgressionNotBoundToRegistry => {
                M5RingNextAction::TraceCanonicalRegistry
            }
            Self::RingWideningTransitionUnclassified
            | Self::RingProgressionObjectIncomplete
            | Self::RingAdvancesWithoutRollbackStopOrRunsSupportAheadOfProof => {
                M5RingNextAction::InspectTransitionOrCondition
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5RingNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::RollbackStopNotVisibleForPublicRing
            | Self::ProofStale => M5RingNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5LaunchControlDowngradeTrigger {
        match self {
            Self::RingTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::RingProgressionNotBoundToRegistry => {
                M5LaunchControlDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::RingWideningTransitionUnclassified | Self::RingProgressionObjectIncomplete => {
                M5LaunchControlDowngradeTrigger::CohortMembershipUnstated
            }
            Self::RingAdvancesWithoutRollbackStopOrRunsSupportAheadOfProof
            | Self::RollbackStopNotVisibleForPublicRing => {
                M5LaunchControlDowngradeTrigger::WidenedWithoutCurrentCohortEvidence
            }
            Self::ProofStale => M5LaunchControlDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a cohort-evidence-packet entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackStopEntryDegradeReason {
    /// The canonical registry token name is unstated.
    RollbackStopTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The evidence scope is unclassified (not in the resolved taxonomy).
    RollbackStopConditionUnclassified,
    /// The cohort evidence would run partner / public support language ahead of cohort proof, hide the cohort
    /// evidence, let a known-limits gap masquerade as covered, or it dropped one of the required cohort-evidence
    /// fields (cohort identity, known-limits ledger, rollback target, rehearsal currency, readiness signoff,
    /// support language, last widening revision).
    RollbackStopAdvancesRingWhileActiveOrDropsStopEvidence,
    /// The canonical / accessible / audit resolution-form coverage of the evidence is incomplete.
    RollbackStopFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RollbackStopEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RollbackStopTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::RollbackStopConditionUnclassified,
        Self::RollbackStopAdvancesRingWhileActiveOrDropsStopEvidence,
        Self::RollbackStopFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RollbackStopTokenUnstated => "rollback_stop_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::RollbackStopConditionUnclassified => "rollback_stop_condition_unclassified",
            Self::RollbackStopAdvancesRingWhileActiveOrDropsStopEvidence => {
                "rollback_stop_advances_ring_while_active_or_drops_stop_evidence"
            }
            Self::RollbackStopFormCoverageIncomplete => "rollback_stop_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5RingNextAction {
        match self {
            Self::RollbackStopTokenUnstated => M5RingNextAction::TraceCanonicalRegistry,
            Self::RollbackStopConditionUnclassified
            | Self::RollbackStopAdvancesRingWhileActiveOrDropsStopEvidence => {
                M5RingNextAction::InspectTransitionOrCondition
            }
            Self::RollbackStopFormCoverageIncomplete => {
                M5RingNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5RingNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5LaunchControlDowngradeTrigger {
        match self {
            Self::RollbackStopTokenUnstated => {
                M5LaunchControlDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::RollbackStopConditionUnclassified => {
                M5LaunchControlDowngradeTrigger::ReadinessStateUnstated
            }
            Self::RollbackStopAdvancesRingWhileActiveOrDropsStopEvidence => {
                M5LaunchControlDowngradeTrigger::RanPartnerOrPublicLanguageAheadOfCohortProof
            }
            Self::RollbackStopFormCoverageIncomplete => {
                M5LaunchControlDowngradeTrigger::ImpliedGreenWhileGoNoGoOrOrrWasStale
            }
            Self::ProofStale => M5LaunchControlDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_ring_progression_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RingProgressionEntryResolutionInput {
    /// Stable identity of the cohort-descriptor-registry entry.
    pub entry_id: String,
    /// The stable cohort-binding ID this descriptor binds to (e.g. `launch.ring.public-preview`); empty means
    /// unstated.
    pub transition_binding_id: String,
    /// The canonical registry token name (e.g. `ring.progression.public_preview_widening`); empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5LaunchControlRole,
    /// The cohort archetype this entry classifies.
    pub ring_widening_transition: M5RingWideningTransitionKind,
    /// The render / surface context.
    pub surface_context: M5RingSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5RingResolutionForm>,
    /// The published exact repo / archetype rows; empty means unstated.
    pub entry_evidence_minimum: String,
    /// The published bundle IDs; empty means unstated.
    pub soak_window_expectation: String,
    /// The published install topology; empty means unstated.
    pub widening_allow_rationale: String,
    /// The published toolchain envelope; empty means unstated.
    pub issue_template_ref: String,
    /// The published known limits; empty means unstated.
    pub known_limits: String,
    /// The published rollback target; empty means unstated.
    pub claim_narrowing_action: String,
    /// The published diagnostics posture; empty means unstated.
    pub rollback_stop_reference: String,
    /// True when the behavior traces to the cohort-descriptor registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the cohort's rollback and diagnostics posture is preserved before widening (a hard invariant
    /// when `false`).
    pub stop_and_rollback_visible_before_widening: bool,
    /// True when this cohort's archetype is public-facing.
    pub is_public_facing_ring: bool,
    /// True when partner / public support language is matched to cohort proof before a public-facing cohort
    /// widens.
    pub support_language_matches_ring_proof: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe cohort-descriptor-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRingProgressionEntry {
    /// Stable identity of the cohort-descriptor-registry entry.
    pub entry_id: String,
    /// The stable cohort-binding ID this descriptor binds to.
    pub transition_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the evidence snapshot and signoff before widening.
    pub semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: bool,
    /// The cohort-archetype token named by the entry.
    pub ring_widening_transition: String,
    /// Whether the cohort archetype is classified into the resolved taxonomy.
    pub ring_widening_transition_is_classified: bool,
    /// The canonical mode for the entry's cohort archetype.
    pub canonical_ring_widening_transition_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published exact repo / archetype rows.
    pub entry_evidence_minimum: String,
    /// The published bundle IDs.
    pub soak_window_expectation: String,
    /// The published install topology.
    pub widening_allow_rationale: String,
    /// The published toolchain envelope.
    pub issue_template_ref: String,
    /// The published known limits.
    pub known_limits: String,
    /// The published rollback target.
    pub claim_narrowing_action: String,
    /// The published diagnostics posture.
    pub rollback_stop_reference: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved cohort-descriptor object publishes every required field.
    pub ring_progression_object_complete: bool,
    /// Whether the entry traces to the cohort-descriptor registry.
    pub bound_to_registry: bool,
    /// Whether the cohort's rollback and diagnostics posture stays preserved before widening.
    pub stop_and_rollback_visible_before_widening: bool,
    /// Whether this cohort's archetype is public-facing.
    pub is_public_facing_ring: bool,
    /// Whether partner / public support language is matched to cohort proof before widening.
    pub support_language_matches_ring_proof: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5RingProgressionEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5RingNextAction,
    /// Whether the descriptor resolves to one typed object across every claimed cohort (clean entry naming every
    /// fact).
    pub ring_progression_resolves_across_transitions: bool,
}

impl M5ResolvedRingProgressionEntry {
    /// Whether this cohort-descriptor entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_rollback_stop_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RollbackStopEntryResolutionInput {
    /// Stable identity of the cohort-evidence-packet entry.
    pub entry_id: String,
    /// The stable evidence-ref this record binds to; empty means unstated.
    pub stop_condition_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5LaunchControlRole,
    /// The evidence scope this record must resolve its cohort proof from.
    pub rollback_stop_condition: M5RollbackStopConditionKind,
    /// The render / surface context.
    pub surface_context: M5RingSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5RingResolutionForm>,
    /// The published resolved cohort identity; empty means missing.
    pub resolved_transition_identity: String,
    /// The published known-limits ledger; empty means missing.
    pub active_stop_condition_ledger: String,
    /// The published rollback-target reference; empty means missing.
    pub rollback_stop_target_reference: String,
    /// The published rehearsal-currency state; empty means missing.
    pub protected_metric_regression_state: String,
    /// The published readiness-signoff state; empty means missing.
    pub packet_freshness_state: String,
    /// The published cohort-bound support-language reference; empty means missing.
    pub crash_data_loss_or_trust_reference: String,
    /// The published last widening revision; empty means missing.
    pub last_ring_transition_revision: String,
    /// True when the record keeps the cohort evidence visible.
    pub keeps_rollback_stop_visible: bool,
    /// True when the evidence is truthful (never claims a clean packet over hidden cohort evidence).
    pub stop_state_is_truthful: bool,
    /// True when partner / public support language is present on this record.
    pub stop_condition_active: bool,
    /// True when the support language is bound to cohort proof rather than running ahead of it.
    pub ring_progression_halted_when_stop_active: bool,
    /// True when a known-limits gap is present on this record.
    pub protected_metric_regression_present: bool,
    /// True when a known-limits gap is flagged rather than masquerading as covered.
    pub protected_metric_regression_flagged: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe cohort-evidence-packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRollbackStopEntry {
    /// Stable identity of the cohort-evidence-packet entry.
    pub entry_id: String,
    /// The stable evidence-ref this record binds to.
    pub stop_condition_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve the evidence snapshot and signoff before widening.
    pub semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: bool,
    /// The evidence-scope token named by the entry.
    pub rollback_stop_condition: String,
    /// Whether the evidence scope is classified into the resolved taxonomy.
    pub rollback_stop_condition_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published resolved cohort identity.
    pub resolved_transition_identity: String,
    /// The published known-limits ledger.
    pub active_stop_condition_ledger: String,
    /// The published rollback-target reference.
    pub rollback_stop_target_reference: String,
    /// The published rehearsal-currency state.
    pub protected_metric_regression_state: String,
    /// The published readiness-signoff state.
    pub packet_freshness_state: String,
    /// The published cohort-bound support-language reference.
    pub crash_data_loss_or_trust_reference: String,
    /// The published last widening revision.
    pub last_ring_transition_revision: String,
    /// Whether the record keeps the cohort evidence visible.
    pub keeps_rollback_stop_visible: bool,
    /// Whether the evidence is truthful.
    pub stop_state_is_truthful: bool,
    /// Whether partner / public support language is present on this build.
    pub stop_condition_active: bool,
    /// Whether the support language is bound to cohort proof rather than running ahead of it.
    pub ring_progression_halted_when_stop_active: bool,
    /// Whether a known-limits gap is present on this record.
    pub protected_metric_regression_present: bool,
    /// Whether a known-limits gap is flagged rather than masquerading as covered.
    pub protected_metric_regression_flagged: bool,
    /// Whether the record stays honest (cohort evidence visible, support language bound to proof, known-limits
    /// gap flagged).
    pub rollback_stop_stays_honest: bool,
    /// Whether the entry provides the complete cohort-evidence object (cohort identity, known-limits ledger,
    /// rollback target, rehearsal currency, readiness signoff, support language, last widening revision).
    pub provides_complete_rollback_stop_record: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5RollbackStopEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5RingNextAction,
    /// Whether the cohort evidence is safe on every claimed cohort (clean entry naming every fact).
    pub rollback_stop_safe_on_every_transition: bool,
}

impl M5ResolvedRollbackStopEntry {
    /// Whether this cohort-evidence-packet entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5RingResolutionError {
    /// The cohort-descriptor-entry id was empty.
    EmptyRingProgressionEntryId,
    /// The cohort-evidence-packet-entry id was empty.
    EmptyRollbackStopEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5RingResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyRingProgressionEntryId => "empty_ring_progression_entry_id",
            Self::EmptyRollbackStopEntryId => "empty_rollback_stop_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5RingResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 cohort-descriptor / cohort-evidence-packet registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RingResolutionError {}

fn form_tokens(forms: &[M5RingResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5RingResolutionForm]) -> bool {
    let present: BTreeSet<M5RingResolutionForm> = forms.iter().copied().collect();
    M5RingResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved cohort-descriptor object publishes every required field: classified cohort archetype,
/// exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, known limits, rollback target,
/// and diagnostics posture. An unclassified archetype or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn ring_progression_object_is_complete(
    archetype: M5RingWideningTransitionKind,
    entry_evidence_minimum: &str,
    soak_window_expectation: &str,
    widening_allow_rationale: &str,
    issue_template_ref: &str,
    known_limits: &str,
    claim_narrowing_action: &str,
    rollback_stop_reference: &str,
) -> bool {
    archetype.is_classified()
        && !entry_evidence_minimum.trim().is_empty()
        && !soak_window_expectation.trim().is_empty()
        && !widening_allow_rationale.trim().is_empty()
        && !issue_template_ref.trim().is_empty()
        && !known_limits.trim().is_empty()
        && !claim_narrowing_action.trim().is_empty()
        && !rollback_stop_reference.trim().is_empty()
}

/// Whether the cohort descriptor keeps a cohort from widening without preserving its rollback and diagnostics
/// posture: the archetype must be classified, the rollback and diagnostics posture must be preserved before
/// widening, and a public-facing cohort must keep its support language matched to cohort proof. An unclassified
/// archetype, an unpreserved rollback / diagnostics posture, or partner / public support language running ahead
/// of proof never matches.
pub fn ring_states_stop_and_rollback_before_widening(
    archetype: M5RingWideningTransitionKind,
    stop_and_rollback_visible_before_widening: bool,
    is_public_facing_ring: bool,
    support_language_matches_ring_proof: bool,
) -> bool {
    archetype.is_classified()
        && stop_and_rollback_visible_before_widening
        && (!is_public_facing_ring || support_language_matches_ring_proof)
}

/// Whether a cohort-evidence packet stays honest: the scope must be classified, the evidence must be truthful,
/// it must keep the cohort evidence visible, any partner / public support language must be bound to cohort proof
/// rather than running ahead of it, and any known-limits gap must be flagged rather than masquerade as covered.
pub fn rollback_stop_stays_honest(
    scope: M5RollbackStopConditionKind,
    stop_state_is_truthful: bool,
    keeps_rollback_stop_visible: bool,
    stop_condition_active: bool,
    ring_progression_halted_when_stop_active: bool,
    protected_metric_regression_present: bool,
    protected_metric_regression_flagged: bool,
) -> bool {
    scope.is_classified()
        && stop_state_is_truthful
        && keeps_rollback_stop_visible
        && (!stop_condition_active || ring_progression_halted_when_stop_active)
        && (!protected_metric_regression_present || protected_metric_regression_flagged)
}

/// Resolves a cohort-descriptor-registry entry so it stays bound to the cohort-descriptor registry: the entry
/// names its canonical token, semantic role, and cohort archetype, covers all three resolution forms, publishes
/// a complete descriptor object (exact repo / archetype rows, bundle IDs, install topology, toolchain envelope,
/// known limits, rollback target, diagnostics posture), preserves its rollback and diagnostics posture before
/// widening so a cohort never widens without it, and keeps a public-facing cohort's support language matched to
/// cohort proof.
pub fn resolve_ring_progression_entry(
    input: M5RingProgressionEntryResolutionInput,
) -> Result<M5ResolvedRingProgressionEntry, M5RingResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5RingResolutionError::EmptyRingProgressionEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.transition_binding_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.entry_evidence_minimum)
        || string_is_forbidden(&input.soak_window_expectation)
        || string_is_forbidden(&input.widening_allow_rationale)
        || string_is_forbidden(&input.issue_template_ref)
        || string_is_forbidden(&input.known_limits)
        || string_is_forbidden(&input.claim_narrowing_action)
        || string_is_forbidden(&input.rollback_stop_reference)
    {
        return Err(M5RingResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = ring_progression_object_is_complete(
        input.ring_widening_transition,
        &input.entry_evidence_minimum,
        &input.soak_window_expectation,
        &input.widening_allow_rationale,
        &input.issue_template_ref,
        &input.known_limits,
        &input.claim_narrowing_action,
        &input.rollback_stop_reference,
    );
    let preserve_ok = ring_states_stop_and_rollback_before_widening(
        input.ring_widening_transition,
        input.stop_and_rollback_visible_before_widening,
        input.is_public_facing_ring,
        input.support_language_matches_ring_proof,
    );
    let support_undisclosed =
        input.is_public_facing_ring && !input.support_language_matches_ring_proof;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5RingProgressionEntryDegradeReason::RingTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5RingProgressionEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.ring_widening_transition.is_classified() {
        Some(M5RingProgressionEntryDegradeReason::RingWideningTransitionUnclassified)
    } else if !input.bound_to_registry {
        Some(M5RingProgressionEntryDegradeReason::RingProgressionNotBoundToRegistry)
    } else if !object_complete {
        Some(M5RingProgressionEntryDegradeReason::RingProgressionObjectIncomplete)
    } else if !preserve_ok {
        Some(M5RingProgressionEntryDegradeReason::RingAdvancesWithoutRollbackStopOrRunsSupportAheadOfProof)
    } else if !all_forms {
        Some(M5RingProgressionEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if support_undisclosed {
        Some(M5RingProgressionEntryDegradeReason::RollbackStopNotVisibleForPublicRing)
    } else if !input.proof_fresh {
        Some(M5RingProgressionEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RingNextAction::ExpandRingMeaning,
    };

    Ok(M5ResolvedRingProgressionEntry {
        entry_id: input.entry_id,
        transition_binding_id: input.transition_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        ring_widening_transition: input.ring_widening_transition.as_str().to_owned(),
        ring_widening_transition_is_classified: input.ring_widening_transition.is_classified(),
        canonical_ring_widening_transition_mode: input
            .ring_widening_transition
            .canonical_ring_widening_transition_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        entry_evidence_minimum: input.entry_evidence_minimum,
        soak_window_expectation: input.soak_window_expectation,
        widening_allow_rationale: input.widening_allow_rationale,
        issue_template_ref: input.issue_template_ref,
        known_limits: input.known_limits,
        claim_narrowing_action: input.claim_narrowing_action,
        rollback_stop_reference: input.rollback_stop_reference,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        ring_progression_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        stop_and_rollback_visible_before_widening: input.stop_and_rollback_visible_before_widening,
        is_public_facing_ring: input.is_public_facing_ring,
        support_language_matches_ring_proof: input.support_language_matches_ring_proof,
        degrade_reason,
        next_action,
        ring_progression_resolves_across_transitions: degrade_reason.is_none(),
    })
}

/// Resolves a cohort-evidence-packet entry so its evidence stays safe: the entry names its canonical token,
/// semantic role, and evidence scope, covers all three resolution forms, provides the complete cohort-identity /
/// known-limits-ledger / rollback-target / rehearsal-currency / readiness-signoff / support-language /
/// last-widening-revision cohort-evidence object, and degrades honestly when the evidence would run partner /
/// public support language ahead of cohort proof, hide the cohort evidence, or let a known-limits gap masquerade
/// as covered.
pub fn resolve_rollback_stop_entry(
    input: M5RollbackStopEntryResolutionInput,
) -> Result<M5ResolvedRollbackStopEntry, M5RingResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5RingResolutionError::EmptyRollbackStopEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.stop_condition_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_transition_identity)
        || string_is_forbidden(&input.active_stop_condition_ledger)
        || string_is_forbidden(&input.rollback_stop_target_reference)
        || string_is_forbidden(&input.protected_metric_regression_state)
        || string_is_forbidden(&input.packet_freshness_state)
        || string_is_forbidden(&input.crash_data_loss_or_trust_reference)
        || string_is_forbidden(&input.last_ring_transition_revision)
    {
        return Err(M5RingResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = rollback_stop_stays_honest(
        input.rollback_stop_condition,
        input.stop_state_is_truthful,
        input.keeps_rollback_stop_visible,
        input.stop_condition_active,
        input.ring_progression_halted_when_stop_active,
        input.protected_metric_regression_present,
        input.protected_metric_regression_flagged,
    );
    let provides_record = input.rollback_stop_condition.is_classified()
        && !input.resolved_transition_identity.trim().is_empty()
        && !input.active_stop_condition_ledger.trim().is_empty()
        && !input.rollback_stop_target_reference.trim().is_empty()
        && !input.protected_metric_regression_state.trim().is_empty()
        && !input.packet_freshness_state.trim().is_empty()
        && !input.crash_data_loss_or_trust_reference.trim().is_empty()
        && !input.last_ring_transition_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5RollbackStopEntryDegradeReason::RollbackStopTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5RollbackStopEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.rollback_stop_condition.is_classified() {
        Some(M5RollbackStopEntryDegradeReason::RollbackStopConditionUnclassified)
    } else if !provides_record {
        Some(M5RollbackStopEntryDegradeReason::RollbackStopAdvancesRingWhileActiveOrDropsStopEvidence)
    } else if !all_forms {
        Some(M5RollbackStopEntryDegradeReason::RollbackStopFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5RollbackStopEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RingNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedRollbackStopEntry {
        entry_id: input.entry_id,
        stop_condition_ref: input.stop_condition_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_preserve_evidence_snapshot_and_signoff_before_widening(),
        rollback_stop_condition: input.rollback_stop_condition.as_str().to_owned(),
        rollback_stop_condition_is_classified: input.rollback_stop_condition.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        resolved_transition_identity: input.resolved_transition_identity,
        active_stop_condition_ledger: input.active_stop_condition_ledger,
        rollback_stop_target_reference: input.rollback_stop_target_reference,
        protected_metric_regression_state: input.protected_metric_regression_state,
        packet_freshness_state: input.packet_freshness_state,
        crash_data_loss_or_trust_reference: input.crash_data_loss_or_trust_reference,
        last_ring_transition_revision: input.last_ring_transition_revision,
        keeps_rollback_stop_visible: input.keeps_rollback_stop_visible,
        stop_state_is_truthful: input.stop_state_is_truthful,
        stop_condition_active: input.stop_condition_active,
        ring_progression_halted_when_stop_active: input.ring_progression_halted_when_stop_active,
        protected_metric_regression_present: input.protected_metric_regression_present,
        protected_metric_regression_flagged: input.protected_metric_regression_flagged,
        rollback_stop_stays_honest: record_stays_honest,
        provides_complete_rollback_stop_record: provides_record,
        degrade_reason,
        next_action,
        rollback_stop_safe_on_every_transition: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved cohort-descriptor and cohort-evidence-packet
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RingProgressionRollbackStopRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5RingProgressionRollbackStopRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5RingAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5RingExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    /// Resolved cohort-descriptor-registry examples.
    pub ring_progression_entries: Vec<M5ResolvedRingProgressionEntry>,
    /// Resolved cohort-evidence-packet examples.
    pub rollback_stop_entries: Vec<M5ResolvedRollbackStopEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the cohort-descriptor and
    /// cohort-evidence-packet domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never widens a cohort without current rollback and diagnostics evidence. MUST be
    /// `false`.
    pub advances_a_ring_without_current_known_limits_and_rollback_stop_evidence: bool,
    /// Hard invariant: this row never runs partner or public support language ahead of cohort proof. MUST be
    /// `false`.
    pub runs_partner_or_public_support_language_ahead_of_ring_proof: bool,
    /// Hard invariant: this row never hides the rollback target or diagnostics posture before widening. MUST be
    /// `false`.
    pub hides_the_known_limits_or_rollback_stop_posture_before_widening: bool,
    /// Hard invariant: this row never collapses distinct cohort evidence classes into one lane. MUST be `false`.
    pub collapses_distinct_rollback_stop_condition_classes_into_one_lane: bool,
}

impl M5RingProgressionRollbackStopRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5RingAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5RingAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5RingExportField> = self.export_fields.iter().copied().collect();
        M5RingExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.advances_a_ring_without_current_known_limits_and_rollback_stop_evidence
            && !self.runs_partner_or_public_support_language_ahead_of_ring_proof
            && !self.hides_the_known_limits_or_rollback_stop_posture_before_widening
            && !self.collapses_distinct_rollback_stop_condition_classes_into_one_lane
    }

    /// True when a clean cohort-descriptor entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified cohort archetype, publishes a complete descriptor object, preserves its rollback and
    /// diagnostics posture, covers all three resolution forms, and keeps a public-facing cohort's support
    /// language matched to proof.
    fn descriptor_is_honest(ex: &M5ResolvedRingProgressionEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.ring_widening_transition_is_classified
                && ex.ring_progression_object_complete
                && ex.stop_and_rollback_visible_before_widening
                && ex.covers_all_resolution_forms
                && (!ex.is_public_facing_ring || ex.support_language_matches_ring_proof))
    }

    /// True when a clean cohort-evidence-packet entry preserves a safe packet: it keeps a classified evidence
    /// scope, provides the complete cohort-evidence object, stays honest, and covers all three resolution forms.
    fn evidence_is_honest(ex: &M5ResolvedRollbackStopEntry) -> bool {
        !ex.is_clean()
            || (ex.rollback_stop_condition_is_classified
                && ex.provides_complete_rollback_stop_record
                && ex.rollback_stop_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.ring_progression_entries
            .iter()
            .all(Self::descriptor_is_honest)
            && self
                .rollback_stop_entries
                .iter()
                .all(Self::evidence_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RingProgressionRollbackStopRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Cohort-archetype tokens (minted by this lane).
    pub ring_widening_transition_kinds: Vec<String>,
    /// Evidence-scope tokens (minted by this lane).
    pub rollback_stop_conditions: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Cohort-descriptor-entry degrade-reason tokens.
    pub ring_progression_degrade_reasons: Vec<String>,
    /// Cohort-evidence-packet-entry degrade-reason tokens.
    pub rollback_stop_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5RingProgressionRollbackStopRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5LaunchControlRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5RingResolutionForm::ALL, |v| v.as_str()),
            ring_widening_transition_kinds: tokens(&M5RingWideningTransitionKind::ALL, |v| {
                v.as_str()
            }),
            rollback_stop_conditions: tokens(&M5RollbackStopConditionKind::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5RingSurfaceContext::ALL, |v| v.as_str()),
            ring_progression_degrade_reasons: tokens(
                &M5RingProgressionEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            rollback_stop_degrade_reasons: tokens(&M5RollbackStopEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5RingAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5RingNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RingExportField::ALL, |v| v.as_str()),
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
pub struct M5RingProgressionRollbackStopRegistriesGovernanceReview {
    /// The descriptor registry names a canonical token, semantic role, and cohort archetype for every entry.
    pub ring_progression_registry_names_token_role_and_transition: bool,
    /// Every claimed cohort resolves to one typed cohort-descriptor object from the shared registry, not
    /// per-entry reconstruction.
    pub transition_resolves_to_typed_ring_progression_from_shared_registry: bool,
    /// The exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles are published for every resolved descriptor.
    pub ring_evidence_and_soak_rows_published: bool,
    /// Cohorts cannot widen without preserving rollback and diagnostics posture before widening.
    pub rings_cannot_advance_without_rollback_stop_and_known_limits: bool,
    /// The cohort evidence keeps the cohort proof visible and binds partner / public support language to it.
    pub rollback_stop_keeps_condition_visible_and_halts_active_ring: bool,
    /// Partner / public support language stays matched to cohort proof for every public-facing cohort.
    pub support_language_matched_to_ring_proof_for_public_rings: bool,
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
    pub ring_or_stop_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RingProgressionRollbackStopRegistriesConsumerProjection {
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
pub struct M5RingProgressionRollbackStopRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RingProgressionRollbackStopRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting cohort audit for the lane.
    pub ring_control_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RingProgressionRollbackStopRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RingProgressionRollbackStopRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5RingProgressionRollbackStopRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RingProgressionRollbackStopRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RingProgressionRollbackStopRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RingProgressionRollbackStopRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RingProgressionRollbackStopRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RingProgressionRollbackStopRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 cohort-descriptor and cohort-evidence-packet registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RingProgressionRollbackStopRegistriesPacket {
    /// Record kind; must equal [`M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5RingProgressionRollbackStopRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RingProgressionRollbackStopRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RingProgressionRollbackStopRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RingProgressionRollbackStopRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RingProgressionRollbackStopRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RingProgressionRollbackStopRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RingProgressionRollbackStopRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5RingProgressionRollbackStopRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5RingProgressionRollbackStopRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_RECORD_KIND {
            violations.push(M5RingProgressionRollbackStopRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5RingProgressionRollbackStopRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RingProgressionRollbackStopRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5RingProgressionRollbackStopRegistriesViolation::VocabularySetDrift);
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
            violations.push(M5RingProgressionRollbackStopRegistriesViolation::RawMaterialInExport);
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
            "consumer_surface,qualification,owner,ring_progression_entries,rollback_stop_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .ring_progression_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.rollback_stop_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.ring_progression_entries.len(),
                row.rollback_stop_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Ring-Progression and Rollback-Stop Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Ring widening transitions: {}\n",
            self.vocabulary_set
                .ring_widening_transition_kinds
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
                "  - Ring-progression entries: {} / cohort-evidence-packet entries: {}\n",
                row.ring_progression_entries.len(),
                row.rollback_stop_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry cohort reference table generated from the registry, so docs and shiproom runbooks
    /// render the same archetype-mode / repo-rows / bundle-ids / install-topology / toolchain-envelope /
    /// rollback-target truth the resolvers produced rather than a hand-copied cohort table. Only clean,
    /// registry-bound cohort-descriptor entries are listed.
    pub fn render_ring_progression_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| transition_binding_id | transition_mode | entry_evidence_minimum | soak_window_expectation | widening_allow_rationale | issue_template_ref | claim_narrowing_action |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.ring_progression_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.transition_binding_id,
                    ex.canonical_ring_widening_transition_mode,
                    ex.entry_evidence_minimum,
                    ex.soak_window_expectation,
                    ex.widening_allow_rationale,
                    ex.issue_template_ref,
                    ex.claim_narrowing_action
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5RingProgressionRollbackStopRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RingProgressionRollbackStopRegistriesViolation>),
}

impl fmt::Display for M5RingProgressionRollbackStopRegistriesArtifactError {
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

impl Error for M5RingProgressionRollbackStopRegistriesArtifactError {}

/// Validation failures emitted by [`M5RingProgressionRollbackStopRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RingProgressionRollbackStopRegistriesViolation {
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
    RingProgressionResolutionNotProven,
    /// Rollback-and-diagnostics-preservation is not proven: no widen-without-rollback example and no unbound
    /// example degrade, no clean bounded descriptor entry is present, or a clean descriptor entry is unbounded
    /// or unbound.
    RollbackStopVisibilityNotProven,
    /// Cohort-evidence-integrity is not proven: clean evidence entries do not cover the canonical dogfood-ring /
    /// rehearsal-currency / go-no-go-signoff scopes with full resolution-form coverage while providing the
    /// complete evidence object, no support-ahead or form-incomplete example degrades, or a clean evidence entry
    /// is missing the complete evidence object.
    RollbackStopIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5RingProgressionRollbackStopRegistriesViolation {
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
            Self::RingProgressionResolutionNotProven => "ring_progression_resolution_not_proven",
            Self::RollbackStopVisibilityNotProven => "rollback_stop_visibility_not_proven",
            Self::RollbackStopIntegrityNotProven => "rollback_stop_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_ring_progression_and_rollback_stop_registries_export() -> Result<
    M5RingProgressionRollbackStopRegistriesPacket,
    M5RingProgressionRollbackStopRegistriesArtifactError,
> {
    let packet: M5RingProgressionRollbackStopRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-ring-progression-and-rollback-stop-registries-proof/support_export.json"
        )
    ))
    .map_err(M5RingProgressionRollbackStopRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RingProgressionRollbackStopRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5RingProgressionRollbackStopRegistriesPacket,
    violations: &mut Vec<M5RingProgressionRollbackStopRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_SCHEMA_REF,
        M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_DOC_REF,
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_RING_PROGRESSION_DOMAIN_SCHEMA_REF,
        M5_ROLLBACK_STOP_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5RingProgressionRollbackStopRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5RingProgressionRollbackStopRegistriesPacket,
    violations: &mut Vec<M5RingProgressionRollbackStopRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5RingProgressionRollbackStopRegistriesViolation::NoRegistryRows);
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
                .push(M5RingProgressionRollbackStopRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations
                .push(M5RingProgressionRollbackStopRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5RingProgressionRollbackStopRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_RING_PROGRESSION_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_ROLLBACK_STOP_DOMAIN_SCHEMA_REF)
        {
            violations
                .push(M5RingProgressionRollbackStopRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.ring_progression_entries.is_empty() || row.rollback_stop_entries.is_empty() {
            violations.push(M5RingProgressionRollbackStopRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5RingProgressionRollbackStopRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5RingProgressionRollbackStopRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5RingProgressionRollbackStopRegistriesPacket,
    violations: &mut Vec<M5RingProgressionRollbackStopRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.ring_progression_registry_names_token_role_and_transition,
        review.transition_resolves_to_typed_ring_progression_from_shared_registry,
        review.ring_evidence_and_soak_rows_published,
        review.rings_cannot_advance_without_rollback_stop_and_known_limits,
        review.rollback_stop_keeps_condition_visible_and_halts_active_ring,
        review.support_language_matched_to_ring_proof_for_public_rings,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shiproom_release_center_executive_steering_and_program_governance_read_single_source,
        review.ring_or_stop_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5RingProgressionRollbackStopRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RingProgressionRollbackStopRegistriesPacket,
    violations: &mut Vec<M5RingProgressionRollbackStopRegistriesViolation>,
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
                M5RingProgressionRollbackStopRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RingProgressionRollbackStopRegistriesPacket,
    violations: &mut Vec<M5RingProgressionRollbackStopRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RingProgressionRollbackStopRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5RingProgressionRollbackStopRegistriesPacket,
    violations: &mut Vec<M5RingProgressionRollbackStopRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.ring_control_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RingProgressionRollbackStopRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5RingProgressionRollbackStopRegistriesPacket,
    violations: &mut Vec<M5RingProgressionRollbackStopRegistriesViolation>,
) {
    let descriptors = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.ring_progression_entries.iter())
    };
    let evidence = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.rollback_stop_entries.iter())
    };

    // AC1: every active cohort can be inspected by exact rows, bundles, toolchains, and deployment profiles.
    // Clean descriptor entries cover the canonical cohort archetypes and the first release-center / shiproom /
    // executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no
    // clean descriptor entry published an incomplete object.
    let clean_archetypes: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.ring_widening_transition.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = descriptors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let archetypes_covered = M5RingWideningTransitionKind::CANONICAL_TRANSITIONS
        .iter()
        .all(|k| clean_archetypes.contains(k.as_str()));
    let first_surfaces_covered = M5RingSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(M5RingProgressionEntryDegradeReason::RingProgressionObjectIncomplete)
    });
    let no_clean_incomplete =
        !descriptors().any(|ex| ex.is_clean() && !ex.ring_progression_object_complete);
    if !(archetypes_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5RingProgressionRollbackStopRegistriesViolation::RingProgressionResolutionNotProven,
        );
    }

    // AC2: cohort packets preserve rollback and diagnostics posture before widening. A widen-without-rollback
    // example degrades, an unbound example degrades, at least one clean bounded descriptor entry is present, and
    // no clean descriptor entry is unbounded or unbound.
    let widen_fold_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(
                M5RingProgressionEntryDegradeReason::RingAdvancesWithoutRollbackStopOrRunsSupportAheadOfProof,
            )
    });
    let unbound_degrades = descriptors().any(|ex| {
        ex.degrade_reason
            == Some(M5RingProgressionEntryDegradeReason::RingProgressionNotBoundToRegistry)
    });
    let bounded_clean_descriptor =
        descriptors().any(|ex| ex.is_clean() && ex.stop_and_rollback_visible_before_widening);
    let no_clean_unbound = !descriptors().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded =
        !descriptors().any(|ex| ex.is_clean() && !ex.stop_and_rollback_visible_before_widening);
    if !(widen_fold_degrades
        && unbound_degrades
        && bounded_clean_descriptor
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations.push(
            M5RingProgressionRollbackStopRegistriesViolation::RollbackStopVisibilityNotProven,
        );
    }

    // AC3: claim publication can prove which cohort evidence backs each launch-bearing lane. Clean evidence
    // entries cover every canonical dogfood-ring / rehearsal-currency / go-no-go-signoff scope with full
    // resolution-form coverage while providing the complete evidence object, a support-ahead example degrades, a
    // form-incomplete example degrades, and no clean evidence entry is missing the complete object.
    let clean_rollback_stop_conditions: BTreeSet<String> = evidence()
        .filter(|ex| {
            ex.is_clean()
                && ex.rollback_stop_condition_is_classified
                && ex.provides_complete_rollback_stop_record
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.rollback_stop_condition.clone())
        .collect();
    let rollback_stop_conditions_covered = M5RollbackStopConditionKind::CANONICAL_CONDITIONS
        .iter()
        .all(|m| clean_rollback_stop_conditions.contains(m.as_str()));
    let support_ahead_degrades = evidence().any(|ex| {
        ex.degrade_reason
            == Some(
                M5RollbackStopEntryDegradeReason::RollbackStopAdvancesRingWhileActiveOrDropsStopEvidence,
            )
    });
    let form_incomplete_degrades = evidence().any(|ex| {
        ex.degrade_reason
            == Some(M5RollbackStopEntryDegradeReason::RollbackStopFormCoverageIncomplete)
    });
    let no_clean_missing_evidence =
        !evidence().any(|ex| ex.is_clean() && !ex.provides_complete_rollback_stop_record);
    if !(rollback_stop_conditions_covered
        && support_ahead_degrades
        && form_incomplete_degrades
        && no_clean_missing_evidence)
    {
        violations
            .push(M5RingProgressionRollbackStopRegistriesViolation::RollbackStopIntegrityNotProven);
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

/// The widening stages this lane governs, for downstream reference: the ring-progression registry defines the
/// minimum evidence and soak expectations that let a lane advance across the alpha, beta, release-candidate,
/// stable, and long-term-support widening stages, and the rollback-stop registry records the conditions that
/// immediately stop that progression.
pub const IMPLEMENTED_RING_STAGES: [M5LaunchControlWideningStage; 5] =
    M5LaunchControlWideningStage::ALL;
