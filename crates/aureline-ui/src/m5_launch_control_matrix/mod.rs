//! Frozen M5 dogfood-ring, certification-cohort, ORR, rehearsal, freeze-exception, and go/no-go control matrix.
//!
//! This module locks Aureline's concrete launch-control model — its cohort taxonomy, readiness events,
//! rehearsal cadence, freeze-exception packets, and explicit go/no-go decisions — into one export-safe packet.
//! Every claimed M5 launch-bearing lane — the core-team canary cohort, the design-partner preview cohort, the
//! extension-author cohort, the public preview cohort, and the certified-archetype cohort — is named once here
//! and constrained by the same shared launch-control-role taxonomy (cohort_membership, readiness_event,
//! rehearsal_currency, freeze_exception_authority, go_no_go_authority, rollback_stop, regression_asset), the
//! same no-stable-claim-widens-without-current-cohort-and-rehearsal-evidence rule, the same
//! freeze-exceptions-are-documented-not-implicit-scope-widening rule, the same
//! Sev-1/Sev-2-incidents-generate-a-regression-asset-before-close-out rule, the same
//! shiproom-dashboards-never-imply-green-when-go-no-go-or-orr-state-is-stale rule, and the same
//! partner-and-public-support-language-never-outruns-current-cohort-proof rule regardless of the surface that
//! renders it.
//!
//! The matrix does not redesign generic dashboard chrome or bundle / certification UI — it is the shared
//! reusable cohort, rehearsal, freeze-exception, and go/no-go control engine contract those already-governed
//! surfaces consume, and it binds back to the already-landed cohort-scoreboard and freeze-exception packets
//! instead of leaving launch-control truth split across scattered meeting notes. The controlled vocabularies
//! are frozen in one self-describing [`M5LaunchControlVocabularySet`] rather than minted per surface. The
//! single controlled launch-control-role vocabulary consumers bind to — cohort_membership, readiness_event,
//! rehearsal_currency, freeze_exception_authority, go_no_go_authority, rollback_stop, and regression_asset —
//! keeps every stable claim entering scope through a cohort and rehearsal gate; keeps ring widening dependent
//! on current known-limits and rollback-stop rules; keeps Sev-1/Sev-2 incidents generating a linked regression
//! asset before close-out; keeps ORR, publish/rollback, mixed-version, advisory/revocation, and support-handoff
//! drills current; keeps freeze exceptions documented rather than implicit scope widening; keeps go/no-go
//! decisions preserving the exact evidence snapshot and named on-call / signoff roster; and keeps partner and
//! public support language from outrunning current cohort proof rather than reading as green. Raw secret values
//! and private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_launch_control_matrix,
    seeded_m5_launch_control_matrix_certified_archetype_preview_narrowed,
    seeded_m5_launch_control_matrix_public_preview_beta_narrowed,
    M5_LAUNCH_CONTROL_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5LaunchControlMatrixPacket`].
pub const M5_LAUNCH_CONTROL_MATRIX_RECORD_KIND: &str =
    "freeze_m5_dogfood_ring_certification_cohort_orr_rehearsal_freeze_exception_and_go_no_go_control_matrix";

/// Schema version for M5 launch-control matrix records.
pub const M5_LAUNCH_CONTROL_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined launch-control matrix schema.
pub const M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF: &str =
    "schemas/program/m5-launch-control-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_LAUNCH_CONTROL_MATRIX_DOC_REF: &str = "docs/release/m5_launch_control_contract.md";

/// Repo-relative path of the canonical cohort-descriptor domain schema (core-team canary and design-partner
/// preview cohorts: cohort class, entry gate, known-limits publication, and rollback-stop rule of a cohort).
pub const M5_COHORT_DESCRIPTOR_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-cohort-descriptor.schema.json";

/// Repo-relative path of the canonical freeze-exception-packet domain schema (extension-author and public
/// preview cohorts: the documented freeze exception, its scope, its expiry, and its rehearsal linkage).
pub const M5_FREEZE_EXCEPTION_PACKET_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-freeze-exception-packet.schema.json";

/// Repo-relative path of the canonical go/no-go-decision domain schema (certified-archetype cohort: the ORR
/// signoff, the explicit go/no-go record, and the preserved evidence snapshot and on-call roster).
pub const M5_GO_NO_GO_DECISION_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-go-no-go-decision.schema.json";

/// Repo-relative path of the already-landed cohort-scoreboard schema the matrix binds back to.
pub const M5_COHORT_SCOREBOARD_LANDED_SCHEMA_REF: &str =
    "schemas/release/cohort_scoreboards.schema.json";

/// Repo-relative path of the already-landed freeze-exception packet schema the launch-control matrix binds
/// back to.
pub const M5_FREEZE_EXCEPTION_LANDED_SCHEMA_REF: &str =
    "schemas/governance/freeze_exception_packet.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_LAUNCH_CONTROL_FIXTURE_DIR: &str = "fixtures/release/m5-launch-control";

/// Repo-relative path of the checked support-export artifact.
pub const M5_LAUNCH_CONTROL_ARTIFACT_REF: &str =
    "artifacts/release/m5-orr-rehearsal-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_LAUNCH_CONTROL_CSV_REF: &str = "artifacts/release/m5-orr-rehearsal-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_LAUNCH_CONTROL_REPORT_REF: &str = "artifacts/program/m5-launch-control-matrix.md";

/// Repo-relative path of the checked launch-control dashboard.
pub const M5_LAUNCH_CONTROL_DASHBOARD_REF: &str = "dashboards/m5-launch-control-dashboard.json";

/// One of the five governed launch-bearing cohorts this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchControlCohort {
    /// The core-team canary cohort: internal dogfood ring with an armed rollback-stop rule.
    CoreTeamCanary,
    /// The design-partner preview cohort: enrolled partners whose feedback triages to requirements.
    DesignPartnerPreview,
    /// The extension-author cohort: compatibility rehearsals current and freeze exceptions documented.
    ExtensionAuthor,
    /// The public preview cohort: publish/rollback, advisory/revocation, and support-handoff drills current.
    PublicPreview,
    /// The certified-archetype cohort: ORR signed and an explicit go/no-go decision recorded.
    CertifiedArchetype,
}

impl M5LaunchControlCohort {
    /// Every governed cohort, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CoreTeamCanary,
        Self::DesignPartnerPreview,
        Self::ExtensionAuthor,
        Self::PublicPreview,
        Self::CertifiedArchetype,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreTeamCanary => "core_team_canary",
            Self::DesignPartnerPreview => "design_partner_preview",
            Self::ExtensionAuthor => "extension_author",
            Self::PublicPreview => "public_preview",
            Self::CertifiedArchetype => "certified_archetype",
        }
    }

    /// The canonical per-domain schema ref a downstream surface points at instead of restating this cohort's
    /// cohort-descriptor, freeze-exception, or go/no-go meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::CoreTeamCanary | Self::DesignPartnerPreview => {
                M5_COHORT_DESCRIPTOR_DOMAIN_SCHEMA_REF
            }
            Self::ExtensionAuthor | Self::PublicPreview => {
                M5_FREEZE_EXCEPTION_PACKET_DOMAIN_SCHEMA_REF
            }
            Self::CertifiedArchetype => M5_GO_NO_GO_DECISION_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this cohort must name a controlled core-team-canary role.
    pub const fn declares_core_team_canary_roles(self) -> bool {
        matches!(self, Self::CoreTeamCanary)
    }

    /// `true` when this cohort must name a controlled design-partner-preview role.
    pub const fn declares_design_partner_preview_roles(self) -> bool {
        matches!(self, Self::DesignPartnerPreview)
    }

    /// `true` when this cohort must name a controlled extension-author role.
    pub const fn declares_extension_author_roles(self) -> bool {
        matches!(self, Self::ExtensionAuthor)
    }

    /// `true` when this cohort must name a controlled public-preview role.
    pub const fn declares_public_preview_roles(self) -> bool {
        matches!(self, Self::PublicPreview)
    }

    /// `true` when this cohort must name a controlled certified-archetype role.
    pub const fn declares_certified_archetype_roles(self) -> bool {
        matches!(self, Self::CertifiedArchetype)
    }
}

/// The single controlled launch-control-role vocabulary every shiproom, release-center, executive-steering,
/// program-governance, docs, or support consumer binds to. These are the exact acceptance-criteria tokens that
/// keep `cohort_membership`, `readiness_event`, `rehearsal_currency`, `freeze_exception_authority`,
/// `go_no_go_authority`, `rollback_stop`, and `regression_asset` meaning the same thing everywhere the
/// launch-control grammar ships. No surface invents a parallel word for any of these roles, and the
/// cohort-membership / readiness-event / go-no-go-authority / freeze-exception-authority roles may never let a
/// stable claim widen without current cohort and rehearsal evidence, imply green while go/no-go or ORR state is
/// stale, leave a freeze exception undocumented, or preserve an evidence snapshot that no longer justifies the
/// widening.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchControlRole {
    /// Cohort-membership role (which cohort a launch-bearing lane must enter before widening).
    CohortMembership,
    /// Readiness-event role (the ORR and readiness gate a lane must clear).
    ReadinessEvent,
    /// Rehearsal-currency role (the currency of publish/rollback, mixed-version, and handoff drills).
    RehearsalCurrency,
    /// Freeze-exception-authority role (the documented, scoped, expiring freeze exception a lane carries).
    FreezeExceptionAuthority,
    /// Go/no-go-authority role (the explicit stable go/no-go decision and its preserved evidence snapshot).
    GoNoGoAuthority,
    /// Rollback-stop role (the ring-progression and rollback-stop rule that bounds widening).
    RollbackStop,
    /// Regression-asset role (the linked regression asset a closed Sev-1/Sev-2 incident must generate).
    RegressionAsset,
}

impl M5LaunchControlRole {
    /// Every launch-control-role token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::CohortMembership,
        Self::ReadinessEvent,
        Self::RehearsalCurrency,
        Self::FreezeExceptionAuthority,
        Self::GoNoGoAuthority,
        Self::RollbackStop,
        Self::RegressionAsset,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CohortMembership => "cohort_membership",
            Self::ReadinessEvent => "readiness_event",
            Self::RehearsalCurrency => "rehearsal_currency",
            Self::FreezeExceptionAuthority => "freeze_exception_authority",
            Self::GoNoGoAuthority => "go_no_go_authority",
            Self::RollbackStop => "rollback_stop",
            Self::RegressionAsset => "regression_asset",
        }
    }

    /// Whether this role carries cohort-membership, readiness-event, go-no-go-authority, or
    /// freeze-exception-authority truth whose per-cohort behavior must never let a stable claim widen without
    /// current cohort and rehearsal evidence, imply green while go/no-go or ORR state is stale, leave a freeze
    /// exception undocumented, or preserve a stale evidence snapshot (`cohort_membership`, `readiness_event`,
    /// `go_no_go_authority`, `freeze_exception_authority`). The descriptive structure roles
    /// (`rehearsal_currency`, `rollback_stop`, `regression_asset`) are inspectable descriptors rather than
    /// widening-authority truth and so do not carry this requirement.
    pub const fn must_preserve_evidence_snapshot_and_signoff_before_widening(self) -> bool {
        matches!(
            self,
            Self::CohortMembership
                | Self::ReadinessEvent
                | Self::GoNoGoAuthority
                | Self::FreezeExceptionAuthority
        )
    }
}

/// Controlled core-team-canary role — how the core-team canary cohort is named, so the internal dogfood ring
/// entered, the known limits published before widening, the armed rollback-stop rule, and the reviewed dogfood
/// telemetry follow one launch-control registry rather than widening on tribal memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreTeamCanaryRole {
    /// Internal canary ring entered.
    InternalCanaryRingEntered,
    /// Known limits published before widening.
    KnownLimitsPublishedBeforeWidening,
    /// Rollback-stop rule armed.
    RollbackStopRuleArmed,
    /// Dogfood telemetry reviewed.
    DogfoodTelemetryReviewed,
    /// A role bound to the single launch-control registry.
    BoundToLaunchControlRegistry,
    /// A stable claim widened without current cohort evidence, which is disallowed.
    WidenedWithoutCurrentCohortEvidenceDisallowed,
}

impl M5CoreTeamCanaryRole {
    /// Every core-team-canary role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InternalCanaryRingEntered,
        Self::KnownLimitsPublishedBeforeWidening,
        Self::RollbackStopRuleArmed,
        Self::DogfoodTelemetryReviewed,
        Self::BoundToLaunchControlRegistry,
        Self::WidenedWithoutCurrentCohortEvidenceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InternalCanaryRingEntered => "internal_canary_ring_entered",
            Self::KnownLimitsPublishedBeforeWidening => "known_limits_published_before_widening",
            Self::RollbackStopRuleArmed => "rollback_stop_rule_armed",
            Self::DogfoodTelemetryReviewed => "dogfood_telemetry_reviewed",
            Self::BoundToLaunchControlRegistry => "bound_to_launch_control_registry",
            Self::WidenedWithoutCurrentCohortEvidenceDisallowed => {
                "widened_without_current_cohort_evidence_disallowed"
            }
        }
    }
}

/// Controlled design-partner-preview role — how the design-partner preview cohort is named, so the partners
/// enrolled under NDA, the preview feedback triaged to requirements, the partner support language matched to
/// cohort proof, and the ring widening gated on known limits follow one launch-control registry rather than
/// running partner language ahead of proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DesignPartnerPreviewRole {
    /// Design partners enrolled under NDA.
    DesignPartnersEnrolledUnderNda,
    /// Preview feedback triaged to requirements.
    PreviewFeedbackTriagedToRequirements,
    /// Partner support language matches cohort proof.
    PartnerSupportLanguageMatchesCohortProof,
    /// Ring widening gated on known limits.
    RingWideningGatedOnKnownLimits,
    /// A role bound to the single launch-control registry.
    BoundToLaunchControlRegistry,
    /// Partner support language outrunning proof, which is disallowed.
    PartnerSupportLanguageOutrunningProofDisallowed,
}

impl M5DesignPartnerPreviewRole {
    /// Every design-partner-preview role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesignPartnersEnrolledUnderNda,
        Self::PreviewFeedbackTriagedToRequirements,
        Self::PartnerSupportLanguageMatchesCohortProof,
        Self::RingWideningGatedOnKnownLimits,
        Self::BoundToLaunchControlRegistry,
        Self::PartnerSupportLanguageOutrunningProofDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesignPartnersEnrolledUnderNda => "design_partners_enrolled_under_nda",
            Self::PreviewFeedbackTriagedToRequirements => {
                "preview_feedback_triaged_to_requirements"
            }
            Self::PartnerSupportLanguageMatchesCohortProof => {
                "partner_support_language_matches_cohort_proof"
            }
            Self::RingWideningGatedOnKnownLimits => "ring_widening_gated_on_known_limits",
            Self::BoundToLaunchControlRegistry => "bound_to_launch_control_registry",
            Self::PartnerSupportLanguageOutrunningProofDisallowed => {
                "partner_support_language_outrunning_proof_disallowed"
            }
        }
    }
}

/// Controlled extension-author role — how the extension-author cohort is named, so the cohort admitted, the
/// compatibility rehearsal kept current, the freeze exception documented not implicit, and the mixed-version
/// drill passed follow one launch-control registry rather than letting a freeze exception become undocumented
/// scope widening.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExtensionAuthorRole {
    /// Extension-author cohort admitted.
    ExtensionAuthorCohortAdmitted,
    /// Compatibility rehearsal current.
    CompatibilityRehearsalCurrent,
    /// Freeze exception documented, not implicit.
    FreezeExceptionDocumentedNotImplicit,
    /// Mixed-version drill passed.
    MixedVersionDrillPassed,
    /// A role bound to the single launch-control registry.
    BoundToLaunchControlRegistry,
    /// Undocumented scope widening, which is disallowed.
    UndocumentedScopeWideningDisallowed,
}

impl M5ExtensionAuthorRole {
    /// Every extension-author role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExtensionAuthorCohortAdmitted,
        Self::CompatibilityRehearsalCurrent,
        Self::FreezeExceptionDocumentedNotImplicit,
        Self::MixedVersionDrillPassed,
        Self::BoundToLaunchControlRegistry,
        Self::UndocumentedScopeWideningDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionAuthorCohortAdmitted => "extension_author_cohort_admitted",
            Self::CompatibilityRehearsalCurrent => "compatibility_rehearsal_current",
            Self::FreezeExceptionDocumentedNotImplicit => {
                "freeze_exception_documented_not_implicit"
            }
            Self::MixedVersionDrillPassed => "mixed_version_drill_passed",
            Self::BoundToLaunchControlRegistry => "bound_to_launch_control_registry",
            Self::UndocumentedScopeWideningDisallowed => "undocumented_scope_widening_disallowed",
        }
    }
}

/// Controlled public-preview role — how the public preview cohort is named, so the public preview ring opened,
/// the publish/rollback drill kept current, the advisory/revocation rehearsal kept current, and the public
/// support-handoff drill kept current follow one launch-control registry rather than running public proof ahead
/// of cohort evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicPreviewRole {
    /// Public preview ring opened.
    PublicPreviewRingOpened,
    /// Publish/rollback drill current.
    PublishRollbackDrillCurrent,
    /// Advisory/revocation rehearsal current.
    AdvisoryRevocationRehearsalCurrent,
    /// Public support-handoff drill current.
    PublicSupportHandoffDrillCurrent,
    /// A role bound to the single launch-control registry.
    BoundToLaunchControlRegistry,
    /// Public proof outrunning cohort evidence, which is disallowed.
    PublicProofOutrunningCohortEvidenceDisallowed,
}

impl M5PublicPreviewRole {
    /// Every public-preview role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PublicPreviewRingOpened,
        Self::PublishRollbackDrillCurrent,
        Self::AdvisoryRevocationRehearsalCurrent,
        Self::PublicSupportHandoffDrillCurrent,
        Self::BoundToLaunchControlRegistry,
        Self::PublicProofOutrunningCohortEvidenceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicPreviewRingOpened => "public_preview_ring_opened",
            Self::PublishRollbackDrillCurrent => "publish_rollback_drill_current",
            Self::AdvisoryRevocationRehearsalCurrent => "advisory_revocation_rehearsal_current",
            Self::PublicSupportHandoffDrillCurrent => "public_support_handoff_drill_current",
            Self::BoundToLaunchControlRegistry => "bound_to_launch_control_registry",
            Self::PublicProofOutrunningCohortEvidenceDisallowed => {
                "public_proof_outrunning_cohort_evidence_disallowed"
            }
        }
    }
}

/// Controlled certified-archetype role — how the certified-archetype cohort is named, so the cohort validated,
/// the operational-readiness review signed, the go/no-go decision recorded, and the evidence snapshot and
/// on-call roster preserved follow one launch-control registry rather than widening to stable without a go/no-go
/// record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CertifiedArchetypeRole {
    /// Certified-archetype cohort validated.
    CertifiedArchetypeCohortValidated,
    /// Operational-readiness review signed.
    OperationalReadinessReviewSigned,
    /// Go/no-go decision recorded.
    GoNoGoDecisionRecorded,
    /// Evidence snapshot and on-call roster preserved.
    EvidenceSnapshotAndOnCallRosterPreserved,
    /// A role bound to the single launch-control registry.
    BoundToLaunchControlRegistry,
    /// Stable widening without a go/no-go decision, which is disallowed.
    StableWideningWithoutGoNoGoDisallowed,
}

impl M5CertifiedArchetypeRole {
    /// Every certified-archetype role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CertifiedArchetypeCohortValidated,
        Self::OperationalReadinessReviewSigned,
        Self::GoNoGoDecisionRecorded,
        Self::EvidenceSnapshotAndOnCallRosterPreserved,
        Self::BoundToLaunchControlRegistry,
        Self::StableWideningWithoutGoNoGoDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedArchetypeCohortValidated => "certified_archetype_cohort_validated",
            Self::OperationalReadinessReviewSigned => "operational_readiness_review_signed",
            Self::GoNoGoDecisionRecorded => "go_no_go_decision_recorded",
            Self::EvidenceSnapshotAndOnCallRosterPreserved => {
                "evidence_snapshot_and_on_call_roster_preserved"
            }
            Self::BoundToLaunchControlRegistry => "bound_to_launch_control_registry",
            Self::StableWideningWithoutGoNoGoDisallowed => {
                "stable_widening_without_go_no_go_disallowed"
            }
        }
    }
}

/// Claimed M5 surface family that renders / consumes a cohort. No cohort may invent a parallel surface
/// taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchControlSurfaceFamily {
    /// The shiproom surface.
    Shiproom,
    /// The release-center surface.
    ReleaseCenter,
    /// The executive-steering surface.
    ExecutiveSteering,
    /// The public-proof surface.
    PublicProof,
    /// The docs / help surface.
    DocsHelp,
    /// The support export.
    SupportExport,
}

impl M5LaunchControlSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Shiproom,
        Self::ReleaseCenter,
        Self::ExecutiveSteering,
        Self::PublicProof,
        Self::DocsHelp,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shiproom => "shiproom",
            Self::ReleaseCenter => "release_center",
            Self::ExecutiveSteering => "executive_steering",
            Self::PublicProof => "public_proof",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }
}

/// Widening stage a cohort must gate before it may claim the next channel, so the acceptance-criteria question
/// of which cohort or readiness event is required before alpha, beta, RC, stable, and LTS widening is answered
/// once rather than left to meeting folklore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchControlWideningStage {
    /// The alpha widening stage.
    Alpha,
    /// The beta widening stage.
    Beta,
    /// The release-candidate widening stage.
    ReleaseCandidate,
    /// The stable widening stage.
    Stable,
    /// The long-term-support widening stage.
    LongTermSupport,
}

impl M5LaunchControlWideningStage {
    /// Every widening stage, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Alpha,
        Self::Beta,
        Self::ReleaseCandidate,
        Self::Stable,
        Self::LongTermSupport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Alpha => "alpha",
            Self::Beta => "beta",
            Self::ReleaseCandidate => "release_candidate",
            Self::Stable => "stable",
            Self::LongTermSupport => "long_term_support",
        }
    }
}

/// Subsystem that consumes a cohort's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchControlConsumerSurface {
    /// The shiproom.
    Shiproom,
    /// The release center.
    ReleaseCenter,
    /// The executive-steering scorecard.
    ExecutiveSteering,
    /// The program-governance review.
    ProgramGovernance,
    /// The diagnostics surface.
    Diagnostics,
    /// The docs / help surface.
    DocsHelp,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
    /// The public-proof surface.
    PublicProof,
}

impl M5LaunchControlConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Shiproom,
        Self::ReleaseCenter,
        Self::ExecutiveSteering,
        Self::ProgramGovernance,
        Self::Diagnostics,
        Self::DocsHelp,
        Self::CliExport,
        Self::SupportExport,
        Self::PublicProof,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shiproom => "shiproom",
            Self::ReleaseCenter => "release_center",
            Self::ExecutiveSteering => "executive_steering",
            Self::ProgramGovernance => "program_governance",
            Self::Diagnostics => "diagnostics",
            Self::DocsHelp => "docs_help",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
            Self::PublicProof => "public_proof",
        }
    }
}

/// Non-visual / accessibility route every cohort must offer so no launch-control meaning disappears under
/// zoom, high contrast, keyboard-only use, or export. Records the keyboard, screen-reader, high-zoom,
/// high-contrast, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchControlAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5LaunchControlAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a cohort has degraded below its qualified state. Required on every row so a stale, unresolved, or
/// narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchControlDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The cohort-descriptor registry source is unavailable.
    CohortDescriptorSourceUnavailable,
    /// The freeze-exception-packet source is unavailable.
    FreezeExceptionSourceUnavailable,
    /// The go/no-go-decision source is unavailable.
    GoNoGoDecisionSourceUnavailable,
    /// Readiness / ORR evidence is unverified.
    ReadinessEvidenceUnverified,
    /// Rehearsal cadence has gone stale.
    RehearsalCadenceStale,
}

impl M5LaunchControlDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::CohortDescriptorSourceUnavailable,
        Self::FreezeExceptionSourceUnavailable,
        Self::GoNoGoDecisionSourceUnavailable,
        Self::ReadinessEvidenceUnverified,
        Self::RehearsalCadenceStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::CohortDescriptorSourceUnavailable => "cohort_descriptor_source_unavailable",
            Self::FreezeExceptionSourceUnavailable => "freeze_exception_source_unavailable",
            Self::GoNoGoDecisionSourceUnavailable => "go_no_go_decision_source_unavailable",
            Self::ReadinessEvidenceUnverified => "readiness_evidence_unverified",
            Self::RehearsalCadenceStale => "rehearsal_cadence_stale",
        }
    }
}

/// Mandatory label a claimed cohort must be able to show. The first three are hard requirements on every
/// cohort; the remaining three close the acceptance-criteria ambiguity about the cohort membership, the
/// readiness state, and the go/no-go state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchControlRequiredLabel {
    /// The cohort's stable identity.
    Identity,
    /// The cohort's launch-control role.
    ControlRole,
    /// The canonical registry reference the cohort points at.
    RegistryReference,
    /// The cohort membership the lane must enter.
    CohortMembership,
    /// The readiness / ORR state the cohort holds.
    ReadinessState,
    /// The go/no-go state the cohort converges on.
    GoNoGoState,
}

impl M5LaunchControlRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::ControlRole,
        Self::RegistryReference,
        Self::CohortMembership,
        Self::ReadinessState,
        Self::GoNoGoState,
    ];

    /// The three labels every claimed cohort must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::ControlRole, Self::RegistryReference];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::ControlRole => "control_role",
            Self::RegistryReference => "registry_reference",
            Self::CohortMembership => "cohort_membership",
            Self::ReadinessState => "readiness_state",
            Self::GoNoGoState => "go_no_go_state",
        }
    }
}

/// Qualification class for an M5 launch-control row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchControlQualificationClass {
    /// Cohort qualifies for the Stable claim.
    Stable,
    /// Cohort is narrowed to Beta.
    Beta,
    /// Cohort is narrowed to Preview.
    Preview,
    /// Cohort is experimental and not claimed.
    Experimental,
    /// Cohort is unavailable on this build.
    Unavailable,
    /// Cohort is held pending upstream resolution.
    Held,
}

impl M5LaunchControlQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the cohort may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a cohort below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchControlDowngradeTrigger {
    /// A stable claim widened without current cohort evidence.
    WidenedWithoutCurrentCohortEvidence,
    /// A stable claim widened without current rehearsal evidence.
    WidenedWithoutCurrentRehearsalEvidence,
    /// A freeze exception was left undocumented.
    LeftAFreezeExceptionUndocumented,
    /// A Sev-1/Sev-2 incident was closed without a regression asset.
    ClosedASevIncidentWithoutARegressionAsset,
    /// A surface implied green while go/no-go or ORR state was stale.
    ImpliedGreenWhileGoNoGoOrOrrWasStale,
    /// Partner or public support language ran ahead of cohort proof.
    RanPartnerOrPublicLanguageAheadOfCohortProof,
    /// A cohort left its cohort membership unstated.
    CohortMembershipUnstated,
    /// A cohort left its readiness state unstated.
    ReadinessStateUnstated,
    /// A cohort left its go/no-go state unstated.
    GoNoGoStateUnstated,
    /// A cohort left its canonical registry reference unstated.
    RegistryReferenceUnstated,
    /// A cohort left its rollback-stop rule unstated.
    RollbackStopRuleUnstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5LaunchControlDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::WidenedWithoutCurrentCohortEvidence,
        Self::WidenedWithoutCurrentRehearsalEvidence,
        Self::LeftAFreezeExceptionUndocumented,
        Self::ClosedASevIncidentWithoutARegressionAsset,
        Self::ImpliedGreenWhileGoNoGoOrOrrWasStale,
        Self::RanPartnerOrPublicLanguageAheadOfCohortProof,
        Self::CohortMembershipUnstated,
        Self::ReadinessStateUnstated,
        Self::GoNoGoStateUnstated,
        Self::RegistryReferenceUnstated,
        Self::RollbackStopRuleUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WidenedWithoutCurrentCohortEvidence => "widened_without_current_cohort_evidence",
            Self::WidenedWithoutCurrentRehearsalEvidence => {
                "widened_without_current_rehearsal_evidence"
            }
            Self::LeftAFreezeExceptionUndocumented => "left_a_freeze_exception_undocumented",
            Self::ClosedASevIncidentWithoutARegressionAsset => {
                "closed_a_sev_incident_without_a_regression_asset"
            }
            Self::ImpliedGreenWhileGoNoGoOrOrrWasStale => {
                "implied_green_while_go_no_go_or_orr_was_stale"
            }
            Self::RanPartnerOrPublicLanguageAheadOfCohortProof => {
                "ran_partner_or_public_language_ahead_of_cohort_proof"
            }
            Self::CohortMembershipUnstated => "cohort_membership_unstated",
            Self::ReadinessStateUnstated => "readiness_state_unstated",
            Self::GoNoGoStateUnstated => "go_no_go_state_unstated",
            Self::RegistryReferenceUnstated => "registry_reference_unstated",
            Self::RollbackStopRuleUnstated => "rollback_stop_rule_unstated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed cohort bound to the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchControlRow {
    /// Governed cohort.
    pub cohort_class: M5LaunchControlCohort,
    /// Qualification class earned by this cohort.
    pub qualification: M5LaunchControlQualificationClass,
    /// Owner role accountable for keeping this cohort governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this cohort.
    pub surface_families: Vec<M5LaunchControlSurfaceFamily>,
    /// Widening stages this cohort must gate before claiming the next channel.
    pub widening_stages: Vec<M5LaunchControlWideningStage>,
    /// Mandatory labels this cohort must be able to show (must include the three
    /// [`M5LaunchControlRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5LaunchControlRequiredLabel>,
    /// Launch-control roles this cohort can carry (the frozen AC vocabulary; required on every cohort).
    pub semantic_roles: Vec<M5LaunchControlRole>,
    /// Core-team-canary roles this cohort names (core-team canary cohort only).
    pub core_team_canary_roles: Vec<M5CoreTeamCanaryRole>,
    /// Design-partner-preview roles this cohort names (design-partner preview cohort only).
    pub design_partner_preview_roles: Vec<M5DesignPartnerPreviewRole>,
    /// Extension-author roles this cohort names (extension-author cohort only).
    pub extension_author_roles: Vec<M5ExtensionAuthorRole>,
    /// Public-preview roles this cohort names (public preview cohort only).
    pub public_preview_roles: Vec<M5PublicPreviewRole>,
    /// Certified-archetype roles this cohort names (certified-archetype cohort only).
    pub certified_archetype_roles: Vec<M5CertifiedArchetypeRole>,
    /// Degraded reasons this cohort can name (required on every cohort).
    pub degraded_reasons: Vec<M5LaunchControlDegradedReason>,
    /// Non-visual accessibility routes this cohort offers.
    pub accessibility_routes: Vec<M5LaunchControlAccessibilityRoute>,
    /// Subsystems that consume this cohort's projection.
    pub consumer_surfaces: Vec<M5LaunchControlConsumerSurface>,
    /// Downgrade triggers that apply to this cohort.
    pub downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    /// Proof packet refs that keep this cohort current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this cohort (must include its own canonical domain schema so
    /// downstream surfaces have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this cohort never widens a stable claim without current cohort and rehearsal evidence.
    /// MUST be `false`.
    pub widens_a_stable_claim_without_current_cohort_and_rehearsal_evidence: bool,
    /// Hard invariant: this cohort never lets a freeze exception become undocumented scope widening. MUST be
    /// `false`.
    pub lets_a_freeze_exception_become_undocumented_scope_widening: bool,
    /// Hard invariant: this cohort never closes a Sev-1/Sev-2 incident without a regression asset. MUST be
    /// `false`.
    pub closes_a_sev_one_or_sev_two_incident_without_a_regression_asset: bool,
    /// Hard invariant: this cohort never implies green when go/no-go records or ORR packets are stale. MUST be
    /// `false`.
    pub implies_green_when_go_no_go_records_or_orr_packets_are_stale: bool,
    /// Hard invariant: this cohort never maintains partner or public support language that outruns current
    /// cohort proof. MUST be `false`.
    pub maintains_partner_or_public_support_language_that_outruns_current_cohort_proof: bool,
}

impl M5LaunchControlRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5LaunchControlRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5LaunchControlRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.widens_a_stable_claim_without_current_cohort_and_rehearsal_evidence
            && !self.lets_a_freeze_exception_become_undocumented_scope_widening
            && !self.closes_a_sev_one_or_sev_two_incident_without_a_regression_asset
            && !self.implies_green_when_go_no_go_records_or_orr_packets_are_stale
            && !self.maintains_partner_or_public_support_language_that_outruns_current_cohort_proof
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchControlVocabularySet {
    /// Cohort-class tokens.
    pub cohort_classes: Vec<String>,
    /// Launch-control-role tokens.
    pub semantic_roles: Vec<String>,
    /// Core-team-canary-role tokens.
    pub core_team_canary_roles: Vec<String>,
    /// Design-partner-preview-role tokens.
    pub design_partner_preview_roles: Vec<String>,
    /// Extension-author-role tokens.
    pub extension_author_roles: Vec<String>,
    /// Public-preview-role tokens.
    pub public_preview_roles: Vec<String>,
    /// Certified-archetype-role tokens.
    pub certified_archetype_roles: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Widening-stage tokens.
    pub widening_stages: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5LaunchControlVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            cohort_classes: tokens(&M5LaunchControlCohort::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5LaunchControlRole::ALL, |v| v.as_str()),
            core_team_canary_roles: tokens(&M5CoreTeamCanaryRole::ALL, |v| v.as_str()),
            design_partner_preview_roles: tokens(&M5DesignPartnerPreviewRole::ALL, |v| v.as_str()),
            extension_author_roles: tokens(&M5ExtensionAuthorRole::ALL, |v| v.as_str()),
            public_preview_roles: tokens(&M5PublicPreviewRole::ALL, |v| v.as_str()),
            certified_archetype_roles: tokens(&M5CertifiedArchetypeRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5LaunchControlSurfaceFamily::ALL, |v| v.as_str()),
            widening_stages: tokens(&M5LaunchControlWideningStage::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5LaunchControlConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5LaunchControlAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5LaunchControlDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5LaunchControlRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5LaunchControlDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5LaunchControlGovernanceReview {
    /// No stable claim skips cohorts.
    pub no_stable_claim_skips_cohorts: bool,
    /// Every committed item enters with a requirement row, evidence class, matrix row, and rollback path.
    pub every_committed_item_enters_with_requirement_row_evidence_class_and_rollback_path: bool,
    /// Ring widening depends on current known-limits and rollback-stop rules.
    pub ring_widening_depends_on_current_known_limits_and_rollback_stop_rules: bool,
    /// Sev-1/Sev-2 incidents generate linked regression assets before close-out.
    pub sev_one_and_sev_two_incidents_generate_linked_regression_assets_before_close_out: bool,
    /// ORR, publish/rollback, mixed-version, and advisory/revocation drills stay current.
    pub orr_publish_rollback_mixed_version_and_advisory_revocation_drills_stay_current: bool,
    /// Support-handoff drills stay current.
    pub support_handoff_drills_stay_current: bool,
    /// Stable go/no-go decisions preserve the evidence snapshot and signoff roster.
    pub stable_go_no_go_decisions_preserve_evidence_snapshot_and_signoff_roster: bool,
    /// Freeze exceptions are documented, not implicit scope widening.
    pub freeze_exceptions_are_documented_not_implicit_scope_widening: bool,
    /// Every cohort keeps the same truth across every widening stage.
    pub every_cohort_declares_widening_stages: bool,
    /// Every cohort declares a non-visual accessibility route.
    pub every_cohort_declares_accessibility_route: bool,
    /// Support / export reads a single canonical launch-control source.
    pub support_export_reads_single_launch_control_source: bool,
    /// Shiproom, release center, and executive steering bind to a single canonical launch-control source.
    pub shiproom_release_center_and_executive_steering_bind_to_single_launch_control_source: bool,
    /// Later M5 rows cannot invent parallel launch-control vocabulary.
    pub later_rows_cannot_invent_parallel_launch_control_vocabulary: bool,
    /// Launch-control truth survives zoom and high contrast.
    pub launch_control_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when the registry is missing, stale, or not yet qualified.
    pub claims_narrow_automatically_when_registry_missing_or_stale: bool,
    /// Partner and public support language never outruns cohort proof.
    pub partner_and_public_support_language_never_outruns_cohort_proof: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchControlConsumerProjection {
    /// Shiproom and release center consume the shared launch-control truth.
    pub shiproom_and_release_center_consume_shared_launch_control_truth: bool,
    /// Executive steering and program governance consume the shared cohort and readiness truth.
    pub executive_steering_and_program_governance_consume_shared_cohort_and_readiness_truth: bool,
    /// Diagnostics and CLI/export consume the shared rehearsal and rollback truth.
    pub diagnostics_and_cli_export_consume_shared_rehearsal_and_rollback_truth: bool,
    /// Docs, help, and screenshots read a single launch-control source.
    pub docs_help_and_screenshots_read_single_launch_control_source: bool,
    /// Go/no-go and ORR proofs bind to the shared evidence snapshot.
    pub go_no_go_and_orr_proofs_bind_to_shared_evidence_snapshot: bool,
    /// Support / export reads a single canonical launch-control source.
    pub support_export_reads_single_launch_control_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchControlProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the cohort.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the launch-control lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchControlReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting launch-control audit for the lane.
    pub launch_control_audit_ref: String,
    /// True when support/export parity is required for every cohort.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every cohort.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5LaunchControlMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LaunchControlMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Launch-control rows.
    pub launch_control_rows: Vec<M5LaunchControlRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LaunchControlVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LaunchControlGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LaunchControlConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LaunchControlProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LaunchControlReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 launch-control matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaunchControlMatrixPacket {
    /// Record kind; must equal [`M5_LAUNCH_CONTROL_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_LAUNCH_CONTROL_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Launch-control rows.
    pub launch_control_rows: Vec<M5LaunchControlRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LaunchControlVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LaunchControlGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LaunchControlConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LaunchControlProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LaunchControlReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5LaunchControlMatrixPacket {
    /// Builds an M5 launch-control matrix packet from stable-cohort input.
    pub fn new(input: M5LaunchControlMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_LAUNCH_CONTROL_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_LAUNCH_CONTROL_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            launch_control_rows: input.launch_control_rows,
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

    /// Validates the M5 launch-control matrix invariants.
    pub fn validate(&self) -> Vec<M5LaunchControlMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_LAUNCH_CONTROL_MATRIX_RECORD_KIND {
            violations.push(M5LaunchControlMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_LAUNCH_CONTROL_MATRIX_SCHEMA_VERSION {
            violations.push(M5LaunchControlMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5LaunchControlMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_launch_control_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 launch-control matrix serializes"),
        ) {
            violations.push(M5LaunchControlMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 launch-control matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed cohort.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "cohort_class,qualification,owner,canonical_schema,surface_families,widening_stages,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.launch_control_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.cohort_class.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.cohort_class.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.widening_stages, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic launch-control dashboard JSON that shiproom and public-proof surfaces render from one
    /// canonical matrix instead of hand-authoring readiness chrome.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn render_dashboard_json(&self) -> String {
        let cohorts: Vec<serde_json::Value> = self
            .launch_control_rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "cohort": row.cohort_class.as_str(),
                    "qualification": row.qualification.as_str(),
                    "canonical_schema": row.cohort_class.canonical_domain_schema_ref(),
                    "widening_stages": row
                        .widening_stages
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                    "consumer_surfaces": row
                        .consumer_surfaces
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                })
            })
            .collect();
        let dashboard = serde_json::json!({
            "record_kind": "m5_launch_control_dashboard",
            "packet_id": self.packet_id,
            "matrix_label": self.matrix_label,
            "matrix_schema_ref": M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
            "support_export_ref": M5_LAUNCH_CONTROL_ARTIFACT_REF,
            "widening_stages": self.vocabulary_set.widening_stages,
            "downgrade_triggers": self.vocabulary_set.downgrade_triggers,
            "cohorts": cohorts,
        });
        serde_json::to_string_pretty(&dashboard).expect("m5 launch-control dashboard serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_cohorts = self
            .launch_control_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Dogfood-Ring, Certification-Cohort, ORR, Rehearsal, Freeze-Exception, and Go/No-Go Control Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Cohorts: {} ({} stable)\n",
            self.launch_control_rows.len(),
            stable_cohorts
        ));
        out.push_str(&format!(
            "- Launch-control roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Widening stages: {}\n",
            self.vocabulary_set.widening_stages.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Cohorts\n\n");
        for row in &self.launch_control_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.cohort_class.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.cohort_class.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 launch-control matrix export.
#[derive(Debug)]
pub enum M5LaunchControlMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5LaunchControlMatrixViolation>),
}

impl fmt::Display for M5LaunchControlMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 launch-control matrix export parse failed: {error}"
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
                    "m5 launch-control matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5LaunchControlMatrixArtifactError {}

/// Validation failures emitted by [`M5LaunchControlMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5LaunchControlMatrixViolation {
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
    /// A required governed cohort is missing from the matrix.
    RequiredCohortMissing,
    /// A launch-control row is incomplete.
    LaunchControlRowIncomplete,
    /// A launch-control row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A launch-control row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A cohort declares no launch-control roles.
    SemanticRoleMissing,
    /// The core-team canary cohort declares no core-team-canary roles.
    CoreTeamCanaryRoleMissing,
    /// The design-partner preview cohort declares no design-partner-preview roles.
    DesignPartnerPreviewRoleMissing,
    /// The extension-author cohort declares no extension-author roles.
    ExtensionAuthorRoleMissing,
    /// The public preview cohort declares no public-preview roles.
    PublicPreviewRoleMissing,
    /// The certified-archetype cohort declares no certified-archetype roles.
    CertifiedArchetypeRoleMissing,
    /// A cohort declares no degraded reasons.
    DegradedReasonMissing,
    /// A cohort declares no surface families.
    SurfaceFamilyMissing,
    /// A cohort declares no widening stages.
    WideningStageMissing,
    /// A cohort declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A cohort declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A cohort declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A cohort claiming Stable is missing required proof packet refs.
    StableCohortMissingProof,
    /// A cohort violates a hard invariant (widening a stable claim without current cohort and rehearsal
    /// evidence, letting a freeze exception become undocumented scope widening, closing a Sev-1/Sev-2 incident
    /// without a regression asset, implying green when go/no-go records or ORR packets are stale, or
    /// maintaining partner or public support language that outruns current cohort proof).
    LaunchControlInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5LaunchControlMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredCohortMissing => "required_cohort_missing",
            Self::LaunchControlRowIncomplete => "launch_control_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::CoreTeamCanaryRoleMissing => "core_team_canary_role_missing",
            Self::DesignPartnerPreviewRoleMissing => "design_partner_preview_role_missing",
            Self::ExtensionAuthorRoleMissing => "extension_author_role_missing",
            Self::PublicPreviewRoleMissing => "public_preview_role_missing",
            Self::CertifiedArchetypeRoleMissing => "certified_archetype_role_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::WideningStageMissing => "widening_stage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableCohortMissingProof => "stable_cohort_missing_proof",
            Self::LaunchControlInvariantViolated => "launch_control_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 launch-control matrix export.
pub fn current_stable_m5_launch_control_matrix_export(
) -> Result<M5LaunchControlMatrixPacket, M5LaunchControlMatrixArtifactError> {
    let packet: M5LaunchControlMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-orr-rehearsal-proof/support_export.json"
    )))
    .map_err(M5LaunchControlMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5LaunchControlMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5LaunchControlMatrixPacket,
    violations: &mut Vec<M5LaunchControlMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_COHORT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
        M5_FREEZE_EXCEPTION_PACKET_DOMAIN_SCHEMA_REF,
        M5_GO_NO_GO_DECISION_DOMAIN_SCHEMA_REF,
        M5_COHORT_SCOREBOARD_LANDED_SCHEMA_REF,
        M5_FREEZE_EXCEPTION_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5LaunchControlMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5LaunchControlMatrixPacket,
    violations: &mut Vec<M5LaunchControlMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5LaunchControlMatrixViolation::VocabularySetDrift);
    }
}

fn validate_launch_control_rows(
    packet: &M5LaunchControlMatrixPacket,
    violations: &mut Vec<M5LaunchControlMatrixViolation>,
) {
    let present: BTreeSet<M5LaunchControlCohort> = packet
        .launch_control_rows
        .iter()
        .map(|row| row.cohort_class)
        .collect();
    for required in M5LaunchControlCohort::ALL {
        if !present.contains(&required) {
            violations.push(M5LaunchControlMatrixViolation::RequiredCohortMissing);
            return;
        }
    }

    for row in &packet.launch_control_rows {
        let cohort = row.cohort_class;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5LaunchControlMatrixViolation::LaunchControlRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5LaunchControlMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == cohort.canonical_domain_schema_ref())
        {
            violations.push(M5LaunchControlMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5LaunchControlMatrixViolation::SemanticRoleMissing);
        }
        if cohort.declares_core_team_canary_roles() && row.core_team_canary_roles.is_empty() {
            violations.push(M5LaunchControlMatrixViolation::CoreTeamCanaryRoleMissing);
        }
        if cohort.declares_design_partner_preview_roles()
            && row.design_partner_preview_roles.is_empty()
        {
            violations.push(M5LaunchControlMatrixViolation::DesignPartnerPreviewRoleMissing);
        }
        if cohort.declares_extension_author_roles() && row.extension_author_roles.is_empty() {
            violations.push(M5LaunchControlMatrixViolation::ExtensionAuthorRoleMissing);
        }
        if cohort.declares_public_preview_roles() && row.public_preview_roles.is_empty() {
            violations.push(M5LaunchControlMatrixViolation::PublicPreviewRoleMissing);
        }
        if cohort.declares_certified_archetype_roles() && row.certified_archetype_roles.is_empty() {
            violations.push(M5LaunchControlMatrixViolation::CertifiedArchetypeRoleMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5LaunchControlMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5LaunchControlMatrixViolation::SurfaceFamilyMissing);
        }
        if row.widening_stages.is_empty() {
            violations.push(M5LaunchControlMatrixViolation::WideningStageMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5LaunchControlMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5LaunchControlMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5LaunchControlMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5LaunchControlMatrixViolation::StableCohortMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5LaunchControlMatrixViolation::LaunchControlInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5LaunchControlMatrixPacket,
    violations: &mut Vec<M5LaunchControlMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.no_stable_claim_skips_cohorts,
        review.every_committed_item_enters_with_requirement_row_evidence_class_and_rollback_path,
        review.ring_widening_depends_on_current_known_limits_and_rollback_stop_rules,
        review.sev_one_and_sev_two_incidents_generate_linked_regression_assets_before_close_out,
        review.orr_publish_rollback_mixed_version_and_advisory_revocation_drills_stay_current,
        review.support_handoff_drills_stay_current,
        review.stable_go_no_go_decisions_preserve_evidence_snapshot_and_signoff_roster,
        review.freeze_exceptions_are_documented_not_implicit_scope_widening,
        review.every_cohort_declares_widening_stages,
        review.every_cohort_declares_accessibility_route,
        review.support_export_reads_single_launch_control_source,
        review.shiproom_release_center_and_executive_steering_bind_to_single_launch_control_source,
        review.later_rows_cannot_invent_parallel_launch_control_vocabulary,
        review.launch_control_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_registry_missing_or_stale,
        review.partner_and_public_support_language_never_outruns_cohort_proof,
    ] {
        if !ok {
            violations.push(M5LaunchControlMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5LaunchControlMatrixPacket,
    violations: &mut Vec<M5LaunchControlMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shiproom_and_release_center_consume_shared_launch_control_truth,
        projection
            .executive_steering_and_program_governance_consume_shared_cohort_and_readiness_truth,
        projection.diagnostics_and_cli_export_consume_shared_rehearsal_and_rollback_truth,
        projection.docs_help_and_screenshots_read_single_launch_control_source,
        projection.go_no_go_and_orr_proofs_bind_to_shared_evidence_snapshot,
        projection.support_export_reads_single_launch_control_source,
    ] {
        if !ok {
            violations.push(M5LaunchControlMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5LaunchControlMatrixPacket,
    violations: &mut Vec<M5LaunchControlMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5LaunchControlMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5LaunchControlMatrixPacket,
    violations: &mut Vec<M5LaunchControlMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.launch_control_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5LaunchControlMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses cohort / rehearsal / readiness / go-no-go / freeze-exception words; what is rejected is a
/// raw secret *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
