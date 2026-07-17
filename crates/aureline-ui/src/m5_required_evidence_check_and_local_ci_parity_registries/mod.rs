//! Implemented M5 required-evidence-check and local-CI-parity registries.
//!
//! The frozen [review-pack evaluator matrix][matrix] names Aureline's governed review-pack object classes and
//! locks their controlled vocabulary. This module makes the matrix's `required_evidence_check_row` and
//! `local_ci_parity_strip` classes operable so review, AI-review, provider-handoff, and support / export surfaces
//! inherit one canonical evidence-and-parity model rather than a hand-authored parallel prose that has to be kept
//! consistent: it turns the *required-evidence-check-row* grammar (one machine-readable row per required check —
//! a must-run test, scanner, docs / migration note, incident link, or rollout note — carrying the evaluation state
//! the check holds: required, optional, skipped, suppressed, timed out, ci-only, not evaluated here, or provider
//! unavailable, kept mechanically distinct so an unevaluated or provider-only check is never collapsed into one
//! success / failure bucket) and the *local-CI-parity-strip* grammar (the parity binding a strip sits in — a
//! local-parity-estimate binding, a provider-authoritative binding, or a capability-difference binding that names
//! environment, secrets, runner class, service dependencies, branch protections, or provider-only merge simulation)
//! into registry resolvers that produce export-safe, honest projections. Every required check then resolves to one
//! typed required-evidence-check row — its evidence-check state, whether Aureline evaluated it locally, imported it,
//! or could not evaluate it here, all attributable to the check so a skipped, suppressed, or ci-only check stays
//! distinguishable from a pass — and to one local-CI-parity strip — the local parity estimate, the
//! provider-authoritative state, and the named capability difference between them — that the review, AI-review,
//! support, and provider-handoff surfaces can inspect without manual reconstruction, so a local parity estimate can
//! never render as provider-authoritative or queue-eligible without an explicit state change backed by provider
//! evidence, and a strip that cannot classify its parity binding degrades honestly instead of implying mergeability
//! from one green summary state.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Emit one machine-readable required-evidence-check row per required check, preserving its evidence class and
//!   evaluation origin.** [`resolve_required_evidence_check_entry`] refuses to read as a clean, registry-bound row
//!   entry unless it names a canonical registry token, a classified [evidence-check state][M5RequiredEvidenceCheckKind],
//!   a disclosure role, covers every [resolution form][M5RequiredEvidenceCheckResolutionForm] (the
//!   canonical object, the accessible summary, and the audit record), publishes every field (the check identity, the
//!   evidence class, and whether Aureline evaluated it locally, imported it, or could not evaluate it here), keeps its
//!   evaluation origin attributable to the check, and keeps a provider-truth-sensitive check bound to its parity
//!   state; otherwise it degrades.
//! * **Compare the local parity estimate against provider-authoritative state rather than a hand-authored prose
//!   staying consistent.** [`line_preserves_rollback_and_diagnostics_before_widening`] rejects a row entry whose
//!   local-estimate / provider-authoritative join is not preserved (a local estimate presenting itself as provider
//!   authoritative without one) so it degrades to
//!   [`M5RequiredEvidenceCheckEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof`] —
//!   the structured blocker reason an estimate-as-authoritative attempt must surface.
//! * **Add compare actions that let users inspect the local-versus-provider delta instead of implying mergeability
//!   from one green summary state.** [`resolve_local_ci_parity_entry`] names a classified
//!   [parity binding][M5LocalCiParityScope]
//!   (local-parity-estimate binding, provider-authoritative binding, or capability-difference binding), requires the
//!   full capability-difference compare object with its named environment / secrets / runner-class / service-dependency /
//!   branch-protection / provider-only-merge-simulation deltas, covers every resolution form, and degrades to
//!   [`M5LocalCiParityEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence`]
//!   when the strip would widen a local estimate into provider-authoritative mergeability, hide a ci-only /
//!   not-evaluated-here / provider-unavailable state behind a green summary, or drop the capability difference, so a
//!   parity strip can never read as trustworthy when it has quietly implied provider truth it does not have.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5ReviewPackRole`] disclosure
//! role vocabulary and the [`M5ReviewPackConsumerSurface`] consumer-surface taxonomy — so the review detail,
//! AI-review panel, review-pack summary, local-CI parity strip, provider handoff, and support / export surfaces
//! can never fork their own review-pack meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_review_pack_evaluator_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_required_evidence_check_and_local_ci_parity_registries,
    seeded_m5_required_evidence_check_and_local_ci_parity_registries_local_ci_parity_preview_narrowed,
    seeded_m5_required_evidence_check_and_local_ci_parity_registries_required_evidence_check_beta_narrowed,
    M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_review_pack_evaluator_matrix::{
    M5ReviewPackAccessibilityRoute, M5ReviewPackClassificationStage, M5ReviewPackConsumerSurface,
    M5ReviewPackDowngradeTrigger, M5ReviewPackObject, M5ReviewPackQualificationClass,
    M5ReviewPackRequiredLabel, M5ReviewPackRole, M5_REVIEW_PACK_MATRIX_DOC_REF,
    M5_REVIEW_PACK_MATRIX_SCHEMA_REF, M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket`].
pub const M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_required_evidence_check_and_local_ci_parity_packet_registries";

/// Schema version for M5 line-required_evidence_check / line-downgrade-packet registry records.
pub const M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_SCHEMA_REF: &str =
    "schemas/review/m5-required-evidence-check-and-local-ci-parity-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_DOC_REF: &str =
    "docs/review/m5_required_evidence_check_and_local_ci_parity_registries.md";

/// Repo-relative path of the matrix's local-CI-parity domain schema, reused by this lane (the machine-readable
/// strip that compares the local parity estimate against the provider-authoritative state and names the capability
/// difference — environment, secrets, runner class, service dependencies, branch protections, or provider-only
/// merge simulation — between them). Declared locally so the row-level domain-schema-ref check stays self-contained.
pub const M5_LOCAL_CI_PARITY_DOMAIN_SCHEMA_REF: &str =
    "schemas/review/m5-local-ci-parity.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/review/m5-required-evidence-check-and-local-ci-parity-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_CSV_REF: &str =
    "artifacts/review/m5-required-evidence-check-and-local-ci-parity-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_REPORT_REF: &str =
    "artifacts/review/m5-required-evidence-check-and-local-ci-parity-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/review/m5-required-evidence-check-and-local-ci-parity-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// line invents a parallel surface set.
pub type M5RequiredEvidenceCheckAndLocalCiParityRegistriesConsumerSurface =
    M5ReviewPackConsumerSurface;

/// One of the three resolution forms every line-required_evidence_check or line-downgrade-packet entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// line-required_evidence_check and line-downgrade *domains* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RequiredEvidenceCheckResolutionForm {
    /// The canonical resolved line-required_evidence_check / line-downgrade-packet object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved line discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved line inspectable off-renderer.
    AuditRecord,
}

impl M5RequiredEvidenceCheckResolutionForm {
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

/// Controlled evaluation state a required-evidence-check row publishes for one must-run test, scanner, docs /
/// migration note, incident link, or rollout note, so every required check shares one typed registry rather than a
/// hand-copied per-check assumption. Minted by this lane because the frozen matrix names the required-evidence and
/// local-CI parity object classes but not the concrete execution states a check row can hold — required, optional,
/// skipped, suppressed, timed out, ci-only, not evaluated here, or provider unavailable — that keep an unevaluated
/// or provider-only check mechanically distinct from a passing one instead of collapsing them into one
/// success / failure bucket. Every classified state carries its canonical mode, and the ci-only and not-evaluated-here
/// states are provider-truth-sensitive (they surface directly in the user-facing local-versus-provider parity
/// language a check row publishes) so their claim must stay matched to explicit provider evidence before the row
/// reads as provider-authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RequiredEvidenceCheckKind {
    /// The check is required: a must-run test, scanner, docs / migration note, incident link, or rollout note that
    /// Aureline evaluated locally and that must pass before the review pack reads as satisfied.
    Required,
    /// The check is optional: it contributes evidence but does not gate the review pack.
    Optional,
    /// The check was skipped: it was not run for this evaluation (out of scope or explicitly deselected) and must
    /// never read as a silent pass.
    Skipped,
    /// The check was suppressed: an owner deliberately muted it, kept mechanically distinct from skipped so a muted
    /// check is never mistaken for one that was simply not selected.
    Suppressed,
    /// The check timed out: it started but did not finish in time, so its outcome is unknown rather than passing.
    TimedOut,
    /// The check is ci-only: it can only run on the provider's CI (environment, secrets, runner class, service
    /// dependencies, branch protections, or provider-only merge simulation) and was imported rather than evaluated
    /// here (provider-truth-sensitive).
    CiOnly,
    /// The check was not evaluated here: Aureline could not evaluate it in the local worktree and neither imported
    /// nor asserted an outcome, so it stays visibly unevaluated (provider-truth-sensitive).
    NotEvaluatedHere,
    /// The check's provider was unavailable: the provider-authoritative state could not be fetched, so no local
    /// parity estimate may stand in for it.
    ProviderUnavailable,
    /// The evidence-check state is unclassified, which is disallowed.
    EvidenceStateUnclassified,
}

impl M5RequiredEvidenceCheckKind {
    /// Every evidence-check state, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Required,
        Self::Optional,
        Self::Skipped,
        Self::Suppressed,
        Self::TimedOut,
        Self::CiOnly,
        Self::NotEvaluatedHere,
        Self::ProviderUnavailable,
        Self::EvidenceStateUnclassified,
    ];

    /// The eight canonical evidence-check states every claimed M5 review surface must keep distinct without
    /// collapsing them into one success / failure bucket.
    pub const CANONICAL_JOURNEYS: [Self; 8] = [
        Self::Required,
        Self::Optional,
        Self::Skipped,
        Self::Suppressed,
        Self::TimedOut,
        Self::CiOnly,
        Self::NotEvaluatedHere,
        Self::ProviderUnavailable,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::Skipped => "skipped",
            Self::Suppressed => "suppressed",
            Self::TimedOut => "timed_out",
            Self::CiOnly => "ci_only",
            Self::NotEvaluatedHere => "not_evaluated_here",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::EvidenceStateUnclassified => "evidence_state_unclassified",
        }
    }

    /// Whether the item is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::EvidenceStateUnclassified)
    }

    /// The canonical mode for this evidence-check state.
    pub const fn canonical_report_section_mode(self) -> &'static str {
        match self {
            Self::Required => "required_mode",
            Self::Optional => "optional_mode",
            Self::Skipped => "skipped_mode",
            Self::Suppressed => "suppressed_mode",
            Self::TimedOut => "timed_out_mode",
            Self::CiOnly => "ci_only_mode",
            Self::NotEvaluatedHere => "not_evaluated_here_mode",
            Self::ProviderUnavailable => "provider_unavailable_mode",
            Self::EvidenceStateUnclassified => "",
        }
    }

    /// Whether this evidence-check state is provider-truth-sensitive and so must keep its local-versus-provider
    /// parity claim matched to explicit provider evidence before the row reads as provider-authoritative.
    pub const fn is_public_facing_line(self) -> bool {
        matches!(self, Self::CiOnly | Self::NotEvaluatedHere)
    }
}

/// Controlled local-ci-parity scope a required-evidence-check change sits in, so a changed cutoff date or
/// producer build, target link, or retention state becomes a visible, typed diff event rather than a silent
/// mutation and shares one registry rather than a hand-copied per-record assumption. Minted by this lane,
/// tracking whether a snapshot descriptor changed its producing build (producer-build-change), changed the
/// current live-target link it points at (target-link-change), or changed its retention / removal state
/// (retention-state-change). Each scope maps directly to the acceptance-criteria change the implementation
/// requirement names — a changed producer build, a changed target link, or a changed retention state — so a
/// preserved snapshot never mutates silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalCiParityScope {
    /// Producer-build-change: a descriptor changed the producer / build identity it was captured from.
    LocalParityEstimateBinding,
    /// Target-link-change: a descriptor changed the current live-target reference it hands off to.
    ProviderAuthoritativeBinding,
    /// Retention-state-change: a descriptor changed its retention / removal (expiry / cleanup) state.
    CapabilityDifferenceBinding,
    /// The local-ci-parity scope is unclassified, which is disallowed.
    LocalCiParityUnclassified,
}

impl M5LocalCiParityScope {
    /// Every comparison scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalParityEstimateBinding,
        Self::ProviderAuthoritativeBinding,
        Self::CapabilityDifferenceBinding,
        Self::LocalCiParityUnclassified,
    ];

    /// The three canonical comparison scopes every claim-history-diff report must stay distinct across.
    pub const CANONICAL_SCOPES: [Self; 3] = [
        Self::LocalParityEstimateBinding,
        Self::ProviderAuthoritativeBinding,
        Self::CapabilityDifferenceBinding,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalParityEstimateBinding => "local_parity_estimate_binding",
            Self::ProviderAuthoritativeBinding => "provider_authoritative_binding",
            Self::CapabilityDifferenceBinding => "capability_difference_binding",
            Self::LocalCiParityUnclassified => "local_ci_parity_unclassified",
        }
    }

    /// Whether the comparison scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::LocalCiParityUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a line-required_evidence_check or
/// line-downgrade-packet token's meaning stays stable whether it appears in the release-center, shiproom,
/// executive-steering, program-governance, or a support / export form. Minted by this lane, tracking the
/// first-consumer surfaces the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RequiredEvidenceCheckSurfaceContext {
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

impl M5RequiredEvidenceCheckSurfaceContext {
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

/// One mandatory rendered part a line-required_evidence_check or line-downgrade-packet entry must be able to show, so no
/// line journey, repo / bundle / toolchain / deployment row, known-limits packet, rollback target,
/// line-downgrade field, or registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RequiredEvidenceCheckAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The line journey the entry classifies (line-required_evidence_check entry).
    CohortArchetype,
    /// The exact repo / journey rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles the entry publishes (line-required_evidence_check entry).
    RepoBundleToolchainAndDeploymentRows,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The known-limits packet and rollback target the entry preserves before widening (line-required_evidence_check
    /// entry).
    KnownLimitsAndRollbackTarget,
    /// The line-downgrade fields (line identity, known-limits ledger, rollback target, rehearsal currency,
    /// readiness signoff, support language) the entry publishes (line-downgrade-packet entry).
    CohortEvidenceFields,
    /// The support-identity hint the entry publishes (line-downgrade-packet entry).
    SupportIdentityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved line required_evidence_check or line downgrade (both entries).
    PlainLanguageMeaning,
}

impl M5RequiredEvidenceCheckAnatomyPart {
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
            Self::CohortEvidenceFields => "local_ci_parity_fields",
            Self::SupportIdentityHint => "support_identity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// line required_evidence_check, a line-downgrade packet, or a degraded line-required_evidence_check / line-downgrade-packet entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RequiredEvidenceCheckNextAction {
    /// Expand the resolved line required_evidence_check's or line-downgrade packet's plain-language meaning.
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

impl M5RequiredEvidenceCheckNextAction {
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
pub enum M5RequiredEvidenceCheckExportField {
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

impl M5RequiredEvidenceCheckExportField {
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

/// Reason a line-required_evidence_check entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, widen-without-rollback, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RequiredEvidenceCheckEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the required_evidence_check means.
    DescriptorTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The line journey is unclassified (not in the resolved taxonomy).
    CohortEvidenceStateUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    DescriptorNotBoundToRegistry,
    /// The resolved line-required_evidence_check object is incomplete: the exact repo / journey rows, bundle IDs, install
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

impl M5RequiredEvidenceCheckEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::DescriptorTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::CohortEvidenceStateUnclassified,
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
            Self::DescriptorTokenUnstated => "required_evidence_check_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::CohortEvidenceStateUnclassified => "line_evidence_state_unclassified",
            Self::DescriptorNotBoundToRegistry => "required_evidence_check_not_bound_to_registry",
            Self::CohortDescriptorObjectIncomplete => "required_evidence_check_object_incomplete",
            Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof => {
                "required_evidence_check_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::RollbackOrDiagnosticsNotPreservedForPublicCohort => {
                "rollback_or_diagnostics_not_preserved_for_public_line"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5RequiredEvidenceCheckNextAction {
        match self {
            Self::DescriptorTokenUnstated | Self::DescriptorNotBoundToRegistry => {
                M5RequiredEvidenceCheckNextAction::TraceCanonicalRegistry
            }
            Self::CohortEvidenceStateUnclassified
            | Self::CohortDescriptorObjectIncomplete
            | Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof => {
                M5RequiredEvidenceCheckNextAction::InspectArchetypeOrScope
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5RequiredEvidenceCheckNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::RollbackOrDiagnosticsNotPreservedForPublicCohort
            | Self::ProofStale => M5RequiredEvidenceCheckNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5ReviewPackDowngradeTrigger {
        match self {
            Self::DescriptorTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete
            | Self::DescriptorNotBoundToRegistry => {
                M5ReviewPackDowngradeTrigger::ParityStateUnstated
            }
            Self::CohortEvidenceStateUnclassified | Self::CohortDescriptorObjectIncomplete => {
                M5ReviewPackDowngradeTrigger::EvaluatorResultClassUnstated
            }
            Self::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof
            | Self::RollbackOrDiagnosticsNotPreservedForPublicCohort => {
                M5ReviewPackDowngradeTrigger::PackVersionDigestUnstated
            }
            Self::ProofStale => M5ReviewPackDowngradeTrigger::ReviewPackMatrixStale,
        }
    }
}

/// Reason a line-downgrade-packet entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalCiParityEntryDegradeReason {
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

impl M5LocalCiParityEntryDegradeReason {
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
            Self::EvidenceScopeUnclassified => "comparison_local_ci_parity_unclassified",
            Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                "local_ci_parity_runs_support_ahead_of_proof_or_drops_local_ci_parity"
            }
            Self::EvidenceFormCoverageIncomplete => "comparison_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5RequiredEvidenceCheckNextAction {
        match self {
            Self::EvidenceTokenUnstated => {
                M5RequiredEvidenceCheckNextAction::TraceCanonicalRegistry
            }
            Self::EvidenceScopeUnclassified
            | Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                M5RequiredEvidenceCheckNextAction::InspectArchetypeOrScope
            }
            Self::EvidenceFormCoverageIncomplete => {
                M5RequiredEvidenceCheckNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5RequiredEvidenceCheckNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5ReviewPackDowngradeTrigger {
        match self {
            Self::EvidenceTokenUnstated => M5ReviewPackDowngradeTrigger::ParityStateUnstated,
            Self::SurfaceContextUnresolved | Self::EvidenceScopeUnclassified => {
                M5ReviewPackDowngradeTrigger::PackFreshnessUnstated
            }
            Self::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence => {
                M5ReviewPackDowngradeTrigger::UnevaluatedCheckHiddenBehindGreenSummary
            }
            Self::EvidenceFormCoverageIncomplete => {
                M5ReviewPackDowngradeTrigger::PackVersionOrDigestDropped
            }
            Self::ProofStale => M5ReviewPackDowngradeTrigger::ReviewPackMatrixStale,
        }
    }
}

/// Input to [`resolve_required_evidence_check_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RequiredEvidenceCheckEntryResolutionInput {
    /// Stable identity of the line-required_evidence_check-registry entry.
    pub entry_id: String,
    /// The stable line-binding ID this required_evidence_check binds to (e.g. `launch.line.public-preview`); empty means
    /// unstated.
    pub line_binding_id: String,
    /// The canonical registry token name (e.g. `line.required_evidence_check.ci_only`); empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5ReviewPackRole,
    /// The line journey this entry classifies.
    pub report_section: M5RequiredEvidenceCheckKind,
    /// The render / surface context.
    pub surface_context: M5RequiredEvidenceCheckSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5RequiredEvidenceCheckResolutionForm>,
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
    /// True when the behavior traces to the line-required_evidence_check registry (never a hand-copied constant).
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

/// Resolved, export-safe line-required_evidence_check-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRequiredEvidenceCheckEntry {
    /// Stable identity of the line-required_evidence_check-registry entry.
    pub entry_id: String,
    /// The stable line-binding ID this required_evidence_check binds to.
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
    /// Whether the resolved line-required_evidence_check object publishes every required field.
    pub required_evidence_check_object_complete: bool,
    /// Whether the entry traces to the line-required_evidence_check registry.
    pub bound_to_registry: bool,
    /// Whether the line's rollback and diagnostics posture stays preserved before widening.
    pub rollback_and_diagnostics_bounded: bool,
    /// Whether this line's journey is public-facing.
    pub is_public_facing_line: bool,
    /// Whether partner / public support language is matched to line proof before widening.
    pub support_language_matches_line_proof: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5RequiredEvidenceCheckEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5RequiredEvidenceCheckNextAction,
    /// Whether the required_evidence_check resolves to one typed object across every claimed line (clean entry naming every
    /// fact).
    pub required_evidence_check_resolves_across_lines: bool,
}

impl M5ResolvedRequiredEvidenceCheckEntry {
    /// Whether this line-required_evidence_check entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_local_ci_parity_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LocalCiParityEntryResolutionInput {
    /// Stable identity of the line-downgrade-packet entry.
    pub entry_id: String,
    /// The stable downgrade-ref this record binds to; empty means unstated.
    pub comparison_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level launch-control role (from the frozen matrix vocabulary).
    pub semantic_role: M5ReviewPackRole,
    /// The downgrade scope this record must resolve its line proof from.
    pub comparison_scope: M5LocalCiParityScope,
    /// The render / surface context.
    pub surface_context: M5RequiredEvidenceCheckSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5RequiredEvidenceCheckResolutionForm>,
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
    pub keeps_local_ci_parity_visible: bool,
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
pub struct M5ResolvedLocalCiParityEntry {
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
    pub keeps_local_ci_parity_visible: bool,
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
    pub local_ci_parity_stays_honest: bool,
    /// Whether the entry provides the complete line-downgrade object (line identity, known-limits ledger,
    /// rollback target, rehearsal currency, readiness signoff, support language, last widening revision).
    pub provides_complete_local_ci_parity: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5LocalCiParityEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5RequiredEvidenceCheckNextAction,
    /// Whether the line downgrade is safe on every claimed line (clean entry naming every fact).
    pub comparison_safe_on_every_line: bool,
}

impl M5ResolvedLocalCiParityEntry {
    /// Whether this line-downgrade-packet entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5RequiredEvidenceCheckResolutionError {
    /// The line-required_evidence_check-entry id was empty.
    EmptyCohortDescriptorEntryId,
    /// The line-downgrade-packet-entry id was empty.
    EmptyCohortEvidencePacketEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5RequiredEvidenceCheckResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCohortDescriptorEntryId => "empty_required_evidence_check_entry_id",
            Self::EmptyCohortEvidencePacketEntryId => "empty_local_ci_parity_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5RequiredEvidenceCheckResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 line-required_evidence_check / line-downgrade-packet registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RequiredEvidenceCheckResolutionError {}

fn form_tokens(forms: &[M5RequiredEvidenceCheckResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5RequiredEvidenceCheckResolutionForm]) -> bool {
    let present: BTreeSet<M5RequiredEvidenceCheckResolutionForm> = forms.iter().copied().collect();
    M5RequiredEvidenceCheckResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved line-required_evidence_check object publishes every required field: classified line journey,
/// exact repo / journey rows, bundle IDs, install topology, toolchain envelope, known limits, rollback target,
/// and diagnostics posture. An unclassified journey or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn required_evidence_check_object_is_complete(
    journey: M5RequiredEvidenceCheckKind,
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

/// Whether the line required_evidence_check keeps a line from widening without preserving its rollback and diagnostics
/// posture: the journey must be classified, the rollback and diagnostics posture must be preserved before
/// widening, and a public-facing line must keep its support language matched to line proof. An unclassified
/// journey, an unpreserved rollback / diagnostics posture, or partner / public support language running ahead
/// of proof never matches.
pub fn line_preserves_rollback_and_diagnostics_before_widening(
    journey: M5RequiredEvidenceCheckKind,
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
pub fn local_ci_parity_stays_honest(
    scope: M5LocalCiParityScope,
    comparison_is_truthful: bool,
    keeps_local_ci_parity_visible: bool,
    support_language_present: bool,
    support_language_bound_to_proof: bool,
    known_limits_gap_present: bool,
    known_limits_gap_flagged: bool,
) -> bool {
    scope.is_classified()
        && comparison_is_truthful
        && keeps_local_ci_parity_visible
        && (!support_language_present || support_language_bound_to_proof)
        && (!known_limits_gap_present || known_limits_gap_flagged)
}

/// Resolves a line-required_evidence_check-registry entry so it stays bound to the line-required_evidence_check registry: the entry
/// names its canonical token, semantic role, and line journey, covers all three resolution forms, publishes
/// a complete required_evidence_check object (exact repo / journey rows, bundle IDs, install topology, toolchain envelope,
/// known limits, rollback target, diagnostics posture), preserves its rollback and diagnostics posture before
/// widening so a line never widens without it, and keeps a public-facing line's support language matched to
/// line proof.
pub fn resolve_required_evidence_check_entry(
    input: M5RequiredEvidenceCheckEntryResolutionInput,
) -> Result<M5ResolvedRequiredEvidenceCheckEntry, M5RequiredEvidenceCheckResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5RequiredEvidenceCheckResolutionError::EmptyCohortDescriptorEntryId);
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
        return Err(M5RequiredEvidenceCheckResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = required_evidence_check_object_is_complete(
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
        Some(M5RequiredEvidenceCheckEntryDegradeReason::DescriptorTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5RequiredEvidenceCheckEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.report_section.is_classified() {
        Some(M5RequiredEvidenceCheckEntryDegradeReason::CohortEvidenceStateUnclassified)
    } else if !input.bound_to_registry {
        Some(M5RequiredEvidenceCheckEntryDegradeReason::DescriptorNotBoundToRegistry)
    } else if !object_complete {
        Some(M5RequiredEvidenceCheckEntryDegradeReason::CohortDescriptorObjectIncomplete)
    } else if !preserve_ok {
        Some(M5RequiredEvidenceCheckEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof)
    } else if !all_forms {
        Some(M5RequiredEvidenceCheckEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if support_undisclosed {
        Some(M5RequiredEvidenceCheckEntryDegradeReason::RollbackOrDiagnosticsNotPreservedForPublicCohort)
    } else if !input.proof_fresh {
        Some(M5RequiredEvidenceCheckEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RequiredEvidenceCheckNextAction::ExpandCohortMeaning,
    };

    Ok(M5ResolvedRequiredEvidenceCheckEntry {
        entry_id: input.entry_id,
        line_binding_id: input.line_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_be_present_before_surfacing_as_a_review_pack_result(),
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
        required_evidence_check_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        rollback_and_diagnostics_bounded: input.rollback_and_diagnostics_bounded,
        is_public_facing_line: input.is_public_facing_line,
        support_language_matches_line_proof: input.support_language_matches_line_proof,
        degrade_reason,
        next_action,
        required_evidence_check_resolves_across_lines: degrade_reason.is_none(),
    })
}

/// Resolves a line-downgrade-packet entry so its downgrade stays safe: the entry names its canonical token,
/// semantic role, and downgrade scope, covers all three resolution forms, provides the complete line-identity /
/// known-limits-ledger / rollback-target / rehearsal-currency / readiness-signoff / support-language /
/// last-widening-revision line-downgrade object, and degrades honestly when the downgrade would run partner /
/// public support language ahead of line proof, hide the line downgrade, or let a known-limits gap masquerade
/// as covered.
pub fn resolve_local_ci_parity_entry(
    input: M5LocalCiParityEntryResolutionInput,
) -> Result<M5ResolvedLocalCiParityEntry, M5RequiredEvidenceCheckResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5RequiredEvidenceCheckResolutionError::EmptyCohortEvidencePacketEntryId);
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
        return Err(M5RequiredEvidenceCheckResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = local_ci_parity_stays_honest(
        input.comparison_scope,
        input.comparison_is_truthful,
        input.keeps_local_ci_parity_visible,
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
        Some(M5LocalCiParityEntryDegradeReason::EvidenceTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5LocalCiParityEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.comparison_scope.is_classified() {
        Some(M5LocalCiParityEntryDegradeReason::EvidenceScopeUnclassified)
    } else if !provides_record {
        Some(M5LocalCiParityEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    } else if !all_forms {
        Some(M5LocalCiParityEntryDegradeReason::EvidenceFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5LocalCiParityEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RequiredEvidenceCheckNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedLocalCiParityEntry {
        entry_id: input.entry_id,
        comparison_ref: input.comparison_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_downgrade_snapshot_and_signoff_before_widening: input
            .semantic_role
            .must_be_present_before_surfacing_as_a_review_pack_result(),
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
        keeps_local_ci_parity_visible: input.keeps_local_ci_parity_visible,
        comparison_is_truthful: input.comparison_is_truthful,
        support_language_present: input.support_language_present,
        support_language_bound_to_proof: input.support_language_bound_to_proof,
        known_limits_gap_present: input.known_limits_gap_present,
        known_limits_gap_flagged: input.known_limits_gap_flagged,
        local_ci_parity_stays_honest: record_stays_honest,
        provides_complete_local_ci_parity: provides_record,
        degrade_reason,
        next_action,
        comparison_safe_on_every_line: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved line-required_evidence_check and line-downgrade-packet
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RequiredEvidenceCheckAndLocalCiParityRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5RequiredEvidenceCheckAndLocalCiParityRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5ReviewPackQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Widening stages this row keeps the same truth across.
    pub capture_lifecycle_stages: Vec<M5ReviewPackClassificationStage>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5ReviewPackRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5ReviewPackAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5RequiredEvidenceCheckAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5RequiredEvidenceCheckExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5ReviewPackDowngradeTrigger>,
    /// Resolved line-required_evidence_check-registry examples.
    pub required_evidence_check_entries: Vec<M5ResolvedRequiredEvidenceCheckEntry>,
    /// Resolved line-downgrade-packet examples.
    pub local_ci_parity_entries: Vec<M5ResolvedLocalCiParityEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the line-required_evidence_check and
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
    pub collapses_distinct_local_ci_parity_classes_into_one_lane: bool,
}

impl M5RequiredEvidenceCheckAndLocalCiParityRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5RequiredEvidenceCheckAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5RequiredEvidenceCheckAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5RequiredEvidenceCheckExportField> =
            self.export_fields.iter().copied().collect();
        M5RequiredEvidenceCheckExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.widens_a_line_without_current_rollback_and_diagnostics_downgrade
            && !self.runs_partner_or_public_support_language_ahead_of_line_proof
            && !self.hides_the_rollback_target_or_diagnostics_posture_before_widening
            && !self.collapses_distinct_local_ci_parity_classes_into_one_lane
    }

    /// True when a clean line-required_evidence_check entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified line journey, publishes a complete required_evidence_check object, preserves its rollback and
    /// diagnostics posture, covers all three resolution forms, and keeps a public-facing line's support
    /// language matched to proof.
    fn required_evidence_check_is_honest(ex: &M5ResolvedRequiredEvidenceCheckEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.report_section_is_classified
                && ex.required_evidence_check_object_complete
                && ex.rollback_and_diagnostics_bounded
                && ex.covers_all_resolution_forms
                && (!ex.is_public_facing_line || ex.support_language_matches_line_proof))
    }

    /// True when a clean line-downgrade-packet entry preserves a safe packet: it keeps a classified downgrade
    /// scope, provides the complete line-downgrade object, stays honest, and covers all three resolution forms.
    fn downgrade_is_honest(ex: &M5ResolvedLocalCiParityEntry) -> bool {
        !ex.is_clean()
            || (ex.comparison_scope_is_classified
                && ex.provides_complete_local_ci_parity
                && ex.local_ci_parity_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.required_evidence_check_entries
            .iter()
            .all(Self::required_evidence_check_is_honest)
            && self
                .local_ci_parity_entries
                .iter()
                .all(Self::downgrade_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RequiredEvidenceCheckAndLocalCiParityRegistriesVocabularySet {
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
    /// Cohort-required_evidence_check-entry degrade-reason tokens.
    pub required_evidence_check_degrade_reasons: Vec<String>,
    /// Cohort-downgrade-packet-entry degrade-reason tokens.
    pub local_ci_parity_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5RequiredEvidenceCheckAndLocalCiParityRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5ReviewPackRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5RequiredEvidenceCheckResolutionForm::ALL, |v| v.as_str()),
            report_section_kinds: tokens(&M5RequiredEvidenceCheckKind::ALL, |v| v.as_str()),
            comparison_scopes: tokens(&M5LocalCiParityScope::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5RequiredEvidenceCheckSurfaceContext::ALL, |v| v.as_str()),
            required_evidence_check_degrade_reasons: tokens(
                &M5RequiredEvidenceCheckEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            local_ci_parity_degrade_reasons: tokens(&M5LocalCiParityEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5RequiredEvidenceCheckAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5RequiredEvidenceCheckNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RequiredEvidenceCheckExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ReviewPackConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5RequiredEvidenceCheckAndLocalCiParityRegistriesGovernanceReview {
    /// The required_evidence_check registry names a canonical token, semantic role, and line journey for every entry.
    pub required_evidence_check_registry_names_token_role_and_journey: bool,
    /// Every claimed line resolves to one typed line-required_evidence_check object from the shared registry, not
    /// per-entry reconstruction.
    pub line_resolves_to_typed_required_evidence_check_from_shared_registry: bool,
    /// The exact repo / journey rows, bundle IDs, install topology, toolchain envelope, and deployment
    /// profiles are published for every resolved required_evidence_check.
    pub repo_bundle_toolchain_and_deployment_rows_published: bool,
    /// Cohorts cannot widen without preserving rollback and diagnostics posture before widening.
    pub lines_cannot_widen_without_rollback_and_diagnostics: bool,
    /// The line downgrade keeps the line proof visible and binds partner / public support language to it.
    pub local_ci_parity_keeps_proof_visible_and_binds_support_language: bool,
    /// Partner / public support language stays matched to line proof for every public-facing line.
    pub support_language_matched_to_line_proof_for_public_lines: bool,
    /// Every line-required_evidence_check and line-downgrade-packet entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Cohort-required_evidence_check and line-downgrade-packet behavior stay bound to the shared registries rather than
    /// hand-copied per line.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shiproom, release center, executive steering, and program governance read a single line source.
    pub shiproom_release_center_executive_steering_and_program_governance_read_single_source: bool,
    /// A widen-without-rollback attempt, an incomplete object, or hidden line downgrade is caught by fixtures
    /// before release downgrade turns green.
    pub required_evidence_check_or_downgrade_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RequiredEvidenceCheckAndLocalCiParityRegistriesConsumerProjection {
    /// Shiproom and release center consume the shared line-required_evidence_check registry.
    pub shiproom_and_release_center_consume_shared_registries: bool,
    /// Executive steering and program governance consume the shared line-downgrade registry.
    pub executive_steering_and_program_governance_consume_shared_registries: bool,
    /// Diagnostics and public proof consume the shared registries.
    pub diagnostics_and_public_proof_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical line-required_evidence_check and line-downgrade-packet domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical line-required_evidence_check / line-downgrade-packet registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RequiredEvidenceCheckAndLocalCiParityRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RequiredEvidenceCheckAndLocalCiParityRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting line audit for the lane.
    pub line_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5RequiredEvidenceCheckAndLocalCiParityRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RequiredEvidenceCheckAndLocalCiParityRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RequiredEvidenceCheckAndLocalCiParityRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RequiredEvidenceCheckAndLocalCiParityRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RequiredEvidenceCheckAndLocalCiParityRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RequiredEvidenceCheckAndLocalCiParityRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 line-required_evidence_check and line-downgrade-packet registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket {
    /// Record kind; must equal [`M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5RequiredEvidenceCheckAndLocalCiParityRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RequiredEvidenceCheckAndLocalCiParityRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RequiredEvidenceCheckAndLocalCiParityRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RequiredEvidenceCheckAndLocalCiParityRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RequiredEvidenceCheckAndLocalCiParityRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RequiredEvidenceCheckAndLocalCiParityRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_RECORD_KIND
                .to_owned(),
            schema_version:
                M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_RECORD_KIND
        {
            violations
                .push(M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version
            != M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(
                M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations
                .push(M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::VocabularySetDrift,
            );
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(&serde_json::to_value(self).expect(
            "m5 line-required_evidence_check / line-downgrade-packet registries packet serializes",
        )) {
            violations.push(
                M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::RawMaterialInExport,
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
        serde_json::to_string_pretty(self).expect(
            "m5 line-required_evidence_check / line-downgrade-packet registries packet serializes",
        )
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,required_evidence_check_entries,local_ci_parity_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .required_evidence_check_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.local_ci_parity_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.required_evidence_check_entries.len(),
                row.local_ci_parity_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Required-Evidence-Check and Local-CI-Parity Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Evidence-check states: {}\n",
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
                "  - Required-evidence-check entries: {} / local-CI-parity entries: {}\n",
                row.required_evidence_check_entries.len(),
                row.local_ci_parity_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry line reference table generated from the registry, so docs and shiproom runbooks
    /// render the same journey-mode / repo-rows / bundle-ids / install-topology / toolchain-envelope /
    /// rollback-target truth the resolvers produced rather than a hand-copied line table. Only clean,
    /// registry-bound line-required_evidence_check entries are listed.
    pub fn render_required_evidence_check_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| line_binding_id | journey_mode | exact_repo_journey_rows | bundle_ids | install_topology | toolchain_envelope | rollback_target |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.required_evidence_check_entries {
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
pub enum M5RequiredEvidenceCheckAndLocalCiParityRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation>),
}

impl fmt::Display for M5RequiredEvidenceCheckAndLocalCiParityRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 line-required_evidence_check / line-downgrade-packet registries export parse failed: {error}"
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
                    "m5 line-required_evidence_check / line-downgrade-packet registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RequiredEvidenceCheckAndLocalCiParityRegistriesArtifactError {}

/// Validation failures emitted by [`M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation {
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
    /// A registry row does not point at both the line-required_evidence_check and line-downgrade-packet domain schemas.
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
    /// Cohort-required_evidence_check-resolution is not proven: clean required_evidence_check entries do not cover the canonical line
    /// journeys or the first release-center / shiproom / executive-steering / program-governance / support
    /// surfaces, no object-incomplete example degrades, or a clean required_evidence_check entry published an incomplete
    /// object.
    CohortDescriptorResolutionNotProven,
    /// Rollback-and-diagnostics-preservation is not proven: no widen-without-rollback example and no unbound
    /// example degrade, no clean bounded required_evidence_check entry is present, or a clean required_evidence_check entry is unbounded
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

impl M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation {
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
                "required_evidence_check_resolution_not_proven"
            }
            Self::RollbackAndDiagnosticsPreservationNotProven => {
                "rollback_and_diagnostics_preservation_not_proven"
            }
            Self::CohortEvidenceIntegrityNotProven => "local_ci_parity_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_required_evidence_check_and_local_ci_parity_registries_export() -> Result<
    M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket,
    M5RequiredEvidenceCheckAndLocalCiParityRegistriesArtifactError,
> {
    let packet: M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/review/m5-required-evidence-check-and-local-ci-parity-registries-proof/support_export.json"
        )
    ))
    .map_err(M5RequiredEvidenceCheckAndLocalCiParityRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RequiredEvidenceCheckAndLocalCiParityRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket,
    violations: &mut Vec<M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_SCHEMA_REF,
        M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_DOC_REF,
        M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
        M5_REVIEW_PACK_MATRIX_DOC_REF,
        M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF,
        M5_LOCAL_CI_PARITY_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket,
    violations: &mut Vec<M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::NoRegistryRows);
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
                M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_LOCAL_CI_PARITY_DOMAIN_SCHEMA_REF)
        {
            violations.push(
                M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.required_evidence_check_entries.is_empty() || row.local_ci_parity_entries.is_empty()
        {
            violations
                .push(M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations
                .push(M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(
                M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket,
    violations: &mut Vec<M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.required_evidence_check_registry_names_token_role_and_journey,
        review.line_resolves_to_typed_required_evidence_check_from_shared_registry,
        review.repo_bundle_toolchain_and_deployment_rows_published,
        review.lines_cannot_widen_without_rollback_and_diagnostics,
        review.local_ci_parity_keeps_proof_visible_and_binds_support_language,
        review.support_language_matched_to_line_proof_for_public_lines,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shiproom_release_center_executive_steering_and_program_governance_read_single_source,
        review.required_evidence_check_or_downgrade_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket,
    violations: &mut Vec<M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation>,
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
                .push(M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket,
    violations: &mut Vec<M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket,
    violations: &mut Vec<M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.line_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket,
    violations: &mut Vec<M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation>,
) {
    let required_evidence_checks = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.required_evidence_check_entries.iter())
    };
    let downgrade = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.local_ci_parity_entries.iter())
    };

    // AC1: every active line can be inspected by exact rows, bundles, toolchains, and deployment profiles.
    // Clean required_evidence_check entries cover the canonical line journeys and the first release-center / shiproom /
    // executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no
    // clean required_evidence_check entry published an incomplete object.
    let clean_journeys: BTreeSet<String> = required_evidence_checks()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.report_section.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = required_evidence_checks()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let journeys_covered = M5RequiredEvidenceCheckKind::CANONICAL_JOURNEYS
        .iter()
        .all(|k| clean_journeys.contains(k.as_str()));
    let first_surfaces_covered = M5RequiredEvidenceCheckSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = required_evidence_checks().any(|ex| {
        ex.degrade_reason
            == Some(M5RequiredEvidenceCheckEntryDegradeReason::CohortDescriptorObjectIncomplete)
    });
    let no_clean_incomplete = !required_evidence_checks()
        .any(|ex| ex.is_clean() && !ex.required_evidence_check_object_complete);
    if !(journeys_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::CohortDescriptorResolutionNotProven,
        );
    }

    // AC2: line packets preserve rollback and diagnostics posture before widening. A widen-without-rollback
    // example degrades, an unbound example degrades, at least one clean bounded required_evidence_check entry is present, and
    // no clean required_evidence_check entry is unbounded or unbound.
    let widen_fold_degrades = required_evidence_checks().any(|ex| {
        ex.degrade_reason
            == Some(
                M5RequiredEvidenceCheckEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
            )
    });
    let unbound_degrades = required_evidence_checks().any(|ex| {
        ex.degrade_reason
            == Some(M5RequiredEvidenceCheckEntryDegradeReason::DescriptorNotBoundToRegistry)
    });
    let bounded_clean_required_evidence_check =
        required_evidence_checks().any(|ex| ex.is_clean() && ex.rollback_and_diagnostics_bounded);
    let no_clean_unbound =
        !required_evidence_checks().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unbounded =
        !required_evidence_checks().any(|ex| ex.is_clean() && !ex.rollback_and_diagnostics_bounded);
    if !(widen_fold_degrades
        && unbound_degrades
        && bounded_clean_required_evidence_check
        && no_clean_unbound
        && no_clean_unbounded)
    {
        violations.push(
            M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::RollbackAndDiagnosticsPreservationNotProven,
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
                && ex.provides_complete_local_ci_parity
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.comparison_scope.clone())
        .collect();
    let comparison_scopes_covered = M5LocalCiParityScope::CANONICAL_SCOPES
        .iter()
        .all(|m| clean_comparison_scopes.contains(m.as_str()));
    let support_ahead_degrades = downgrade().any(|ex| {
        ex.degrade_reason
            == Some(
                M5LocalCiParityEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
            )
    });
    let form_incomplete_degrades = downgrade().any(|ex| {
        ex.degrade_reason == Some(M5LocalCiParityEntryDegradeReason::EvidenceFormCoverageIncomplete)
    });
    let no_clean_missing_downgrade =
        !downgrade().any(|ex| ex.is_clean() && !ex.provides_complete_local_ci_parity);
    if !(comparison_scopes_covered
        && support_ahead_degrades
        && form_incomplete_degrades
        && no_clean_missing_downgrade)
    {
        violations
            .push(M5RequiredEvidenceCheckAndLocalCiParityRegistriesViolation::CohortEvidenceIntegrityNotProven);
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

/// The launch-bearing lines this lane implements, for downstream reference: the line-required_evidence_check registry
/// covers the core-team canary, design-partner preview, extension-author, public preview, and certified-journey
/// lines the frozen matrix froze, and the line-downgrade-packet registry binds the downgrade that backs each.
pub const IMPLEMENTED_LINES: [M5ReviewPackObject; 6] = M5ReviewPackObject::ALL;
