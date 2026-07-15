//! Frozen M5 stable-line protection, evidence-refresh, correction-line, and LTS-readiness matrix.
//!
//! This module locks Aureline's concrete post-stable operating model — its stable-line taxonomy, support
//! windows, correction-line owners, backport-decision SLAs, evidence-refresh cadences, bundle-refresh
//! obligations, and LTS-eligibility state — into one export-safe packet. Every active stable or
//! stable-candidate line — the fresh stable line, the evidence-refresh line, the correction/backport line, the
//! launch-bundle-currentness line, and the LTS-candidate line — is named once here and constrained by the same
//! shared stable-line-protection-role taxonomy (support_window, correction_ownership, evidence_refresh,
//! backport_decision, lts_eligibility, bundle_currentness, defect_ledger), the same
//! no-support-language-widens-without-current-refresh-and-correction-evidence rule, the same
//! no-shipping-line-drifts-on-stale-evidence-or-frozen-launch-bundles rule, the same
//! backport-decisions-are-documented-not-tribal-memory rule, the same
//! supported-line-defects-stay-owned-and-resolved-within-SLA rule, and the same
//! LTS-is-a-checked-in-decision-packet-backed-by-current-rollback-and-support-evidence rule regardless of the
//! surface that renders it.
//!
//! The matrix does not redesign generic dashboard chrome or bundle / release-center UI — it is the shared
//! reusable stable-line protection, refresh, correction, and LTS-readiness engine contract those
//! already-governed surfaces consume, and it binds back to the already-landed claim-manifest and release-center
//! packets instead of leaving post-stable truth split across scattered shiproom notes. The controlled
//! vocabularies are frozen in one self-describing [`M5StableLineProtectionVocabularySet`] rather than minted per
//! surface. The single controlled stable-line-protection-role vocabulary consumers bind to — support_window,
//! correction_ownership, evidence_refresh, backport_decision, lts_eligibility, bundle_currentness, and
//! defect_ledger — keeps every support claim entering scope through a refresh and correction gate; keeps
//! bundle-currentness dependent on current refresh audits; keeps supported-line defects owned and resolved
//! within SLA; keeps evidence-refresh cadence ordinary release ops rather than launch-only heroics; keeps
//! backport decisions documented rather than tribal memory; keeps LTS decisions preserving the exact rollback
//! and support evidence snapshot; and keeps support language from outrunning current refresh and correction
//! proof rather than reading as green. Raw secret values and private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_stable_line_protection_matrix,
    seeded_m5_stable_line_protection_matrix_bundle_currentness_beta_narrowed,
    seeded_m5_stable_line_protection_matrix_lts_candidate_preview_narrowed,
    M5_STABLE_LINE_PROTECTION_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5StableLineProtectionMatrixPacket`].
pub const M5_STABLE_LINE_PROTECTION_MATRIX_RECORD_KIND: &str =
    "freeze_m5_stable_line_protection_evidence_refresh_correction_line_and_lts_readiness_matrix";

/// Schema version for M5 stable-line-protection matrix records.
pub const M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined stable-line-protection matrix schema.
pub const M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF: &str =
    "schemas/program/m5-stable-line-protection-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_STABLE_LINE_PROTECTION_MATRIX_DOC_REF: &str = "docs/release/m5-stable-line-ops.md";

/// Repo-relative path of the canonical stable-line-refresh-policy domain schema (fresh stable line and
/// evidence-refresh line: the support window, the evidence-refresh cadence, and the refresh obligations of a
/// line).
pub const M5_STABLE_LINE_REFRESH_POLICY_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-stable-line-refresh-policy.schema.json";

/// Repo-relative path of the canonical supported-line-defect-ledger domain schema (correction/backport line
/// and launch-bundle-currentness line: the defect entry, its correction owner, its backport-decision SLA, and
/// its bundle-currentness posture).
pub const M5_SUPPORTED_LINE_DEFECT_LEDGER_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-supported-line-defect-ledger.schema.json";

/// Repo-relative path of the canonical lts-readiness-decision domain schema (LTS-candidate line: the LTS
/// decision, its rollback/support discipline evidence, and the preserved evidence snapshot).
pub const M5_LTS_READINESS_DECISION_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-lts-readiness-decision.schema.json";

/// Repo-relative path of the already-landed claim-manifest schema the matrix binds back to.
pub const M5_STABLE_CLAIM_MANIFEST_LANDED_SCHEMA_REF: &str =
    "schemas/release/stable_claim_manifest.schema.json";

/// Repo-relative path of the already-landed release-center schema the stable-line-protection matrix binds
/// back to.
pub const M5_RELEASE_CENTER_LANDED_SCHEMA_REF: &str = "schemas/release/release_center.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_STABLE_LINE_PROTECTION_FIXTURE_DIR: &str =
    "fixtures/release/m5-stable-line-protection";

/// Repo-relative path of the checked support-export artifact.
pub const M5_STABLE_LINE_PROTECTION_ARTIFACT_REF: &str =
    "artifacts/release/m5-stable-line-correction-reports/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_STABLE_LINE_PROTECTION_CSV_REF: &str =
    "artifacts/release/m5-stable-line-correction-reports/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_STABLE_LINE_PROTECTION_REPORT_REF: &str =
    "artifacts/program/m5-stable-line-protection-matrix.md";

/// Repo-relative path of the checked stable-line-protection dashboard.
pub const M5_STABLE_LINE_PROTECTION_DASHBOARD_REF: &str = "dashboards/m5-stable-line-health.json";

/// One of the five governed active stable / stable-candidate lines this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLineProtectionLine {
    /// The fresh stable line: the just-shipped stable line whose crash / rollback / support-export / migration
    /// flows are protected through the first 30 days after stable.
    FreshStableLine,
    /// The evidence-refresh line: certified-archetype, compatibility, and known-limits evidence kept refreshed
    /// on an ordinary release-ops cadence.
    EvidenceRefreshLine,
    /// The correction / backport line: the first correction / backport path exercised, with backport decisions
    /// recorded within SLA and post-launch correction reports published.
    CorrectionBackportLine,
    /// The launch-bundle-currentness line: launch-bundle freshness re-checked and the bundle-refresh obligation
    /// met on the shipping line.
    BundleCurrentnessLine,
    /// The LTS-candidate line: backport / rollback / support discipline demonstrated and an LTS decision packet
    /// recorded before any LTS commitment.
    LtsCandidateLine,
}

impl M5StableLineProtectionLine {
    /// Every governed line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FreshStableLine,
        Self::EvidenceRefreshLine,
        Self::CorrectionBackportLine,
        Self::BundleCurrentnessLine,
        Self::LtsCandidateLine,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshStableLine => "fresh_stable_line",
            Self::EvidenceRefreshLine => "evidence_refresh_line",
            Self::CorrectionBackportLine => "correction_backport_line",
            Self::BundleCurrentnessLine => "bundle_currentness_line",
            Self::LtsCandidateLine => "lts_candidate_line",
        }
    }

    /// The canonical per-domain schema ref a downstream surface points at instead of restating this line's
    /// refresh-policy, defect-ledger, or LTS-readiness meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::FreshStableLine | Self::EvidenceRefreshLine => {
                M5_STABLE_LINE_REFRESH_POLICY_DOMAIN_SCHEMA_REF
            }
            Self::CorrectionBackportLine | Self::BundleCurrentnessLine => {
                M5_SUPPORTED_LINE_DEFECT_LEDGER_DOMAIN_SCHEMA_REF
            }
            Self::LtsCandidateLine => M5_LTS_READINESS_DECISION_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this line must name a controlled fresh-stable-line role.
    pub const fn declares_fresh_stable_line_roles(self) -> bool {
        matches!(self, Self::FreshStableLine)
    }

    /// `true` when this line must name a controlled evidence-refresh-line role.
    pub const fn declares_evidence_refresh_line_roles(self) -> bool {
        matches!(self, Self::EvidenceRefreshLine)
    }

    /// `true` when this line must name a controlled correction/backport-line role.
    pub const fn declares_correction_backport_line_roles(self) -> bool {
        matches!(self, Self::CorrectionBackportLine)
    }

    /// `true` when this line must name a controlled launch-bundle-currentness-line role.
    pub const fn declares_bundle_currentness_line_roles(self) -> bool {
        matches!(self, Self::BundleCurrentnessLine)
    }

    /// `true` when this line must name a controlled LTS-candidate-line role.
    pub const fn declares_lts_candidate_line_roles(self) -> bool {
        matches!(self, Self::LtsCandidateLine)
    }
}

/// The single controlled stable-line-protection-role vocabulary every release, help, support, public-proof,
/// shiproom, or program-governance consumer binds to. These are the exact acceptance-criteria tokens that
/// keep `support_window`, `correction_ownership`, `evidence_refresh`, `backport_decision`,
/// `lts_eligibility`, `bundle_currentness`, and `defect_ledger` meaning the same thing everywhere the
/// stable-line-protection grammar ships. No surface invents a parallel word for any of these roles, and the
/// support-window / correction-ownership / lts-eligibility / backport-decision roles may never let support
/// language widen without current refresh and correction evidence, claim LTS without current rollback and
/// support evidence, leave a supported-line defect unowned, or preserve an evidence snapshot that no longer
/// justifies the widening.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLineProtectionRole {
    /// Support-window role (the declared support window an active stable line must publish before widening).
    SupportWindow,
    /// Correction-ownership role (the named correction-line owner accountable for supported-line defects).
    CorrectionOwnership,
    /// Evidence-refresh role (the evidence-refresh cadence kept ordinary release ops).
    EvidenceRefresh,
    /// Backport-decision role (the documented backport decision recorded within its SLA).
    BackportDecision,
    /// LTS-eligibility role (the LTS decision packet and its preserved rollback/support evidence snapshot).
    LtsEligibility,
    /// Bundle-currentness role (the launch-bundle-refresh obligation and shipping-line bundle audit).
    BundleCurrentness,
    /// Defect-ledger role (the supported-line defect ledger that keeps defects owned and resolved within SLA).
    DefectLedger,
}

impl M5StableLineProtectionRole {
    /// Every stable-line-protection-role token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SupportWindow,
        Self::CorrectionOwnership,
        Self::EvidenceRefresh,
        Self::BackportDecision,
        Self::LtsEligibility,
        Self::BundleCurrentness,
        Self::DefectLedger,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportWindow => "support_window",
            Self::CorrectionOwnership => "correction_ownership",
            Self::EvidenceRefresh => "evidence_refresh",
            Self::BackportDecision => "backport_decision",
            Self::LtsEligibility => "lts_eligibility",
            Self::BundleCurrentness => "bundle_currentness",
            Self::DefectLedger => "defect_ledger",
        }
    }

    /// Whether this role carries support-window, correction-ownership, lts-eligibility, or backport-decision
    /// truth whose per-line behavior must never let support language widen without current refresh and
    /// correction evidence, claim LTS without current rollback and support evidence, leave a supported-line
    /// defect unowned, or preserve a stale evidence snapshot (`support_window`, `correction_ownership`,
    /// `lts_eligibility`, `backport_decision`). The descriptive structure roles
    /// (`evidence_refresh`, `bundle_currentness`, `defect_ledger`) are inspectable descriptors rather than
    /// widening-authority truth and so do not carry this requirement.
    pub const fn must_preserve_evidence_snapshot_and_signoff_before_widening(self) -> bool {
        matches!(
            self,
            Self::SupportWindow
                | Self::CorrectionOwnership
                | Self::LtsEligibility
                | Self::BackportDecision
        )
    }
}

/// Controlled fresh-stable-line role — how the just-shipped stable line is protected through its first 30
/// days, so the crash/rollback flow, the support-export flow, and the migration flow stay protected under one
/// stable-line-protection registry rather than drifting on stale evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FreshStableLineRole {
    /// Crash / rollback flow protected in the first 30 days after stable.
    CrashRollbackFlowProtected,
    /// Support-export flow protected in the first 30 days after stable.
    SupportExportFlowProtected,
    /// Migration flow protected in the first 30 days after stable.
    MigrationFlowProtected,
    /// First-thirty-day watch active over the fresh stable line.
    FirstThirtyDayWatchActive,
    /// A role bound to the single stable-line-protection registry.
    BoundToStableLineRegistry,
    /// A shipping line drifted on stale evidence, which is disallowed.
    DriftedOnStaleEvidenceDisallowed,
}

impl M5FreshStableLineRole {
    /// Every fresh-stable-line role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CrashRollbackFlowProtected,
        Self::SupportExportFlowProtected,
        Self::MigrationFlowProtected,
        Self::FirstThirtyDayWatchActive,
        Self::BoundToStableLineRegistry,
        Self::DriftedOnStaleEvidenceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashRollbackFlowProtected => "crash_rollback_flow_protected",
            Self::SupportExportFlowProtected => "support_export_flow_protected",
            Self::MigrationFlowProtected => "migration_flow_protected",
            Self::FirstThirtyDayWatchActive => "first_thirty_day_watch_active",
            Self::BoundToStableLineRegistry => "bound_to_stable_line_registry",
            Self::DriftedOnStaleEvidenceDisallowed => "drifted_on_stale_evidence_disallowed",
        }
    }
}

/// Controlled evidence-refresh-line role — how the certified-archetype, compatibility, and known-limits
/// evidence is kept refreshed on an ordinary release-ops cadence under one stable-line-protection registry
/// rather than letting support language outrun current refresh proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceRefreshLineRole {
    /// Certified-archetype evidence refreshed.
    ArchetypeEvidenceRefreshed,
    /// Compatibility evidence refreshed.
    CompatibilityEvidenceRefreshed,
    /// Known-limits evidence refreshed.
    KnownLimitsEvidenceRefreshed,
    /// Refresh cadence kept ordinary release ops rather than launch-only heroics.
    RefreshCadenceIsOrdinaryReleaseOps,
    /// A role bound to the single stable-line-protection registry.
    BoundToStableLineRegistry,
    /// Support language outrunning current refresh proof, which is disallowed.
    SupportLanguageOutrunningRefreshDisallowed,
}

impl M5EvidenceRefreshLineRole {
    /// Every evidence-refresh-line role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ArchetypeEvidenceRefreshed,
        Self::CompatibilityEvidenceRefreshed,
        Self::KnownLimitsEvidenceRefreshed,
        Self::RefreshCadenceIsOrdinaryReleaseOps,
        Self::BoundToStableLineRegistry,
        Self::SupportLanguageOutrunningRefreshDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArchetypeEvidenceRefreshed => "archetype_evidence_refreshed",
            Self::CompatibilityEvidenceRefreshed => "compatibility_evidence_refreshed",
            Self::KnownLimitsEvidenceRefreshed => "known_limits_evidence_refreshed",
            Self::RefreshCadenceIsOrdinaryReleaseOps => "refresh_cadence_is_ordinary_release_ops",
            Self::BoundToStableLineRegistry => "bound_to_stable_line_registry",
            Self::SupportLanguageOutrunningRefreshDisallowed => {
                "support_language_outrunning_refresh_disallowed"
            }
        }
    }
}

/// Controlled correction/backport-line role — how the first correction and backport path is exercised, so the
/// backport decision recorded within SLA, the may-slip item shipped or narrowed, and the post-launch correction
/// report published follow one stable-line-protection registry rather than resting on tribal backport memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CorrectionBackportLineRole {
    /// First correction / backport path exercised.
    CorrectionPathExercised,
    /// Backport decision recorded within its SLA.
    BackportDecisionRecordedWithinSla,
    /// A bounded may-slip item shipped as a correction or narrowed to a smaller public claim.
    MaySlipItemShippedOrNarrowed,
    /// Post-launch correction report published.
    PostLaunchCorrectionReportPublished,
    /// A role bound to the single stable-line-protection registry.
    BoundToStableLineRegistry,
    /// Relying on tribal backport memory instead of a documented correction packet, which is disallowed.
    TribalBackportMemoryDisallowed,
}

impl M5CorrectionBackportLineRole {
    /// Every correction/backport-line role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CorrectionPathExercised,
        Self::BackportDecisionRecordedWithinSla,
        Self::MaySlipItemShippedOrNarrowed,
        Self::PostLaunchCorrectionReportPublished,
        Self::BoundToStableLineRegistry,
        Self::TribalBackportMemoryDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorrectionPathExercised => "correction_path_exercised",
            Self::BackportDecisionRecordedWithinSla => "backport_decision_recorded_within_sla",
            Self::MaySlipItemShippedOrNarrowed => "may_slip_item_shipped_or_narrowed",
            Self::PostLaunchCorrectionReportPublished => "post_launch_correction_report_published",
            Self::BoundToStableLineRegistry => "bound_to_stable_line_registry",
            Self::TribalBackportMemoryDisallowed => "tribal_backport_memory_disallowed",
        }
    }
}

/// Controlled launch-bundle-currentness-line role — how launch-bundle freshness is re-checked on the shipping
/// line, so the bundle-refresh obligation met, the shipping-line bundle audited, and any frozen-bundle drift
/// detected follow one stable-line-protection registry rather than shipping a stale launch bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BundleCurrentnessLineRole {
    /// Launch-bundle freshness re-checked on the shipping line.
    LaunchBundleFreshnessRechecked,
    /// Bundle-refresh obligation met.
    BundleRefreshObligationMet,
    /// Shipping-line bundle audited for currentness.
    ShippingLineBundleAudited,
    /// Frozen-bundle drift detected before it ships.
    FrozenBundleDriftDetected,
    /// A role bound to the single stable-line-protection registry.
    BoundToStableLineRegistry,
    /// Shipping a stale launch bundle without a currentness audit, which is disallowed.
    StaleLaunchBundleShippedDisallowed,
}

impl M5BundleCurrentnessLineRole {
    /// Every launch-bundle-currentness-line role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LaunchBundleFreshnessRechecked,
        Self::BundleRefreshObligationMet,
        Self::ShippingLineBundleAudited,
        Self::FrozenBundleDriftDetected,
        Self::BoundToStableLineRegistry,
        Self::StaleLaunchBundleShippedDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchBundleFreshnessRechecked => "launch_bundle_freshness_rechecked",
            Self::BundleRefreshObligationMet => "bundle_refresh_obligation_met",
            Self::ShippingLineBundleAudited => "shipping_line_bundle_audited",
            Self::FrozenBundleDriftDetected => "frozen_bundle_drift_detected",
            Self::BoundToStableLineRegistry => "bound_to_stable_line_registry",
            Self::StaleLaunchBundleShippedDisallowed => "stale_launch_bundle_shipped_disallowed",
        }
    }
}

/// Controlled LTS-candidate-line role — how the stable line justifies an LTS commitment, so the backport
/// discipline demonstrated, the rollback discipline demonstrated, the LTS decision packet recorded, and the
/// support-evidence snapshot preserved follow one stable-line-protection registry rather than claiming LTS on a
/// marketing label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LtsCandidateLineRole {
    /// Backport discipline demonstrated on the stable line.
    BackportDisciplineDemonstrated,
    /// Rollback discipline demonstrated on the stable line.
    RollbackDisciplineDemonstrated,
    /// LTS decision packet recorded.
    LtsDecisionPacketRecorded,
    /// Support-evidence snapshot preserved with the LTS decision.
    SupportEvidenceSnapshotPreserved,
    /// A role bound to the single stable-line-protection registry.
    BoundToStableLineRegistry,
    /// Claiming LTS without current rollback and support evidence, which is disallowed.
    LtsClaimedWithoutEvidenceDisallowed,
}

impl M5LtsCandidateLineRole {
    /// Every LTS-candidate-line role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BackportDisciplineDemonstrated,
        Self::RollbackDisciplineDemonstrated,
        Self::LtsDecisionPacketRecorded,
        Self::SupportEvidenceSnapshotPreserved,
        Self::BoundToStableLineRegistry,
        Self::LtsClaimedWithoutEvidenceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackportDisciplineDemonstrated => "backport_discipline_demonstrated",
            Self::RollbackDisciplineDemonstrated => "rollback_discipline_demonstrated",
            Self::LtsDecisionPacketRecorded => "lts_decision_packet_recorded",
            Self::SupportEvidenceSnapshotPreserved => "support_evidence_snapshot_preserved",
            Self::BoundToStableLineRegistry => "bound_to_stable_line_registry",
            Self::LtsClaimedWithoutEvidenceDisallowed => "lts_claimed_without_evidence_disallowed",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a line. No line may invent a parallel surface
/// taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLineProtectionSurfaceFamily {
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

impl M5StableLineProtectionSurfaceFamily {
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

/// Widening stage a line must gate before it may claim the next channel, so the acceptance-criteria question
/// of which line-protection gate is required before alpha, beta, RC, stable, and LTS widening is answered
/// once rather than left to meeting folklore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLineProtectionWideningStage {
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

impl M5StableLineProtectionWideningStage {
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

/// Subsystem that consumes a line's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLineProtectionConsumerSurface {
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

impl M5StableLineProtectionConsumerSurface {
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

/// Non-visual / accessibility route every line must offer so no stable-line-protection meaning disappears under
/// zoom, high contrast, keyboard-only use, or export. Records the keyboard, screen-reader, high-zoom,
/// high-contrast, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLineProtectionAccessibilityRoute {
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

impl M5StableLineProtectionAccessibilityRoute {
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

/// Reason a line has degraded below its qualified state. Required on every row so a stale, unresolved, or
/// narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLineProtectionDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The stable-line-refresh-policy source is unavailable.
    RefreshPolicySourceUnavailable,
    /// The supported-line-defect-ledger source is unavailable.
    DefectLedgerSourceUnavailable,
    /// The LTS-readiness-decision source is unavailable.
    LtsReadinessSourceUnavailable,
    /// Evidence-refresh proof is unverified.
    RefreshEvidenceUnverified,
    /// The correction-line owner is unknown.
    CorrectionOwnershipUnknown,
}

impl M5StableLineProtectionDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::RefreshPolicySourceUnavailable,
        Self::DefectLedgerSourceUnavailable,
        Self::LtsReadinessSourceUnavailable,
        Self::RefreshEvidenceUnverified,
        Self::CorrectionOwnershipUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::RefreshPolicySourceUnavailable => "refresh_policy_source_unavailable",
            Self::DefectLedgerSourceUnavailable => "defect_ledger_source_unavailable",
            Self::LtsReadinessSourceUnavailable => "lts_readiness_source_unavailable",
            Self::RefreshEvidenceUnverified => "refresh_evidence_unverified",
            Self::CorrectionOwnershipUnknown => "correction_ownership_unknown",
        }
    }
}

/// Mandatory label a claimed line must be able to show. The first three are hard requirements on every
/// line; the remaining three close the acceptance-criteria ambiguity about the support window, the
/// refresh state, and the LTS posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLineProtectionRequiredLabel {
    /// The line's stable identity.
    Identity,
    /// The line's stable-line-protection role.
    ProtectionRole,
    /// The canonical registry reference the line points at.
    RegistryReference,
    /// The support window the line must publish.
    SupportWindow,
    /// The refresh state the line holds.
    RefreshState,
    /// The LTS posture the line converges on.
    LtsPosture,
}

impl M5StableLineProtectionRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::ProtectionRole,
        Self::RegistryReference,
        Self::SupportWindow,
        Self::RefreshState,
        Self::LtsPosture,
    ];

    /// The three labels every claimed line must be able to show.
    pub const MANDATORY: [Self; 3] = [
        Self::Identity,
        Self::ProtectionRole,
        Self::RegistryReference,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::ProtectionRole => "protection_role",
            Self::RegistryReference => "registry_reference",
            Self::SupportWindow => "support_window",
            Self::RefreshState => "refresh_state",
            Self::LtsPosture => "lts_posture",
        }
    }
}

/// Qualification class for an M5 stable-line-protection row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLineProtectionQualificationClass {
    /// Line qualifies for the Stable claim.
    Stable,
    /// Line is narrowed to Beta.
    Beta,
    /// Line is narrowed to Preview.
    Preview,
    /// Line is experimental and not claimed.
    Experimental,
    /// Line is unavailable on this build.
    Unavailable,
    /// Line is held pending upstream resolution.
    Held,
}

impl M5StableLineProtectionQualificationClass {
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

    /// Whether the line may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a line below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLineProtectionDowngradeTrigger {
    /// A stable claim widened without current line evidence.
    WidenedSupportWithoutCurrentRefreshEvidence,
    /// Support language widened without current correction evidence.
    WidenedSupportWithoutCurrentCorrectionEvidence,
    /// A backport rested on tribal memory instead of a documented correction packet.
    ReliedOnTribalBackportMemory,
    /// A supported-line defect was left unowned or unresolved past its SLA.
    LeftASupportedLineDefectUnownedPastSla,
    /// A surface implied green while refresh or defect-ledger state was stale.
    ImpliedGreenWhileRefreshOrLedgerWasStale,
    /// Partner or public support language ran ahead of line proof.
    RanSupportLanguageAheadOfRefreshProof,
    /// A line left its line membership unstated.
    SupportWindowUnstated,
    /// A line left its refresh state unstated.
    RefreshStateUnstated,
    /// A line left its LTS posture unstated.
    LtsPostureUnstated,
    /// A line left its canonical registry reference unstated.
    RegistryReferenceUnstated,
    /// A line left its rollback-stop rule unstated.
    BundleCurrentnessUnstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5StableLineProtectionDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::WidenedSupportWithoutCurrentRefreshEvidence,
        Self::WidenedSupportWithoutCurrentCorrectionEvidence,
        Self::ReliedOnTribalBackportMemory,
        Self::LeftASupportedLineDefectUnownedPastSla,
        Self::ImpliedGreenWhileRefreshOrLedgerWasStale,
        Self::RanSupportLanguageAheadOfRefreshProof,
        Self::SupportWindowUnstated,
        Self::RefreshStateUnstated,
        Self::LtsPostureUnstated,
        Self::RegistryReferenceUnstated,
        Self::BundleCurrentnessUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WidenedSupportWithoutCurrentRefreshEvidence => {
                "widened_support_without_current_refresh_evidence"
            }
            Self::WidenedSupportWithoutCurrentCorrectionEvidence => {
                "widened_support_without_current_correction_evidence"
            }
            Self::ReliedOnTribalBackportMemory => "relied_on_tribal_backport_memory",
            Self::LeftASupportedLineDefectUnownedPastSla => {
                "left_a_supported_line_defect_unowned_past_sla"
            }
            Self::ImpliedGreenWhileRefreshOrLedgerWasStale => {
                "implied_green_while_refresh_or_ledger_was_stale"
            }
            Self::RanSupportLanguageAheadOfRefreshProof => {
                "ran_support_language_ahead_of_refresh_proof"
            }
            Self::SupportWindowUnstated => "support_window_unstated",
            Self::RefreshStateUnstated => "refresh_state_unstated",
            Self::LtsPostureUnstated => "lts_posture_unstated",
            Self::RegistryReferenceUnstated => "registry_reference_unstated",
            Self::BundleCurrentnessUnstated => "bundle_currentness_unstated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed line bound to the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableLineProtectionRow {
    /// Governed line.
    pub line_class: M5StableLineProtectionLine,
    /// Qualification class earned by this line.
    pub qualification: M5StableLineProtectionQualificationClass,
    /// Owner role accountable for keeping this line governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this line.
    pub surface_families: Vec<M5StableLineProtectionSurfaceFamily>,
    /// Widening stages this line must gate before claiming the next channel.
    pub widening_stages: Vec<M5StableLineProtectionWideningStage>,
    /// Mandatory labels this line must be able to show (must include the three
    /// [`M5StableLineProtectionRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5StableLineProtectionRequiredLabel>,
    /// Stable-line-protection roles this line can carry (the frozen AC vocabulary; required on every line).
    pub semantic_roles: Vec<M5StableLineProtectionRole>,
    /// Fresh-stable-line roles this line names (fresh stable line only).
    pub fresh_stable_line_roles: Vec<M5FreshStableLineRole>,
    /// Evidence-refresh-line roles this line names (evidence-refresh line only).
    pub evidence_refresh_line_roles: Vec<M5EvidenceRefreshLineRole>,
    /// Correction/backport-line roles this line names (correction/backport line only).
    pub correction_backport_line_roles: Vec<M5CorrectionBackportLineRole>,
    /// Launch-bundle-currentness-line roles this line names (launch-bundle-currentness line only).
    pub bundle_currentness_line_roles: Vec<M5BundleCurrentnessLineRole>,
    /// LTS-candidate-line roles this line names (LTS-candidate line only).
    pub lts_candidate_line_roles: Vec<M5LtsCandidateLineRole>,
    /// Degraded reasons this line can name (required on every line).
    pub degraded_reasons: Vec<M5StableLineProtectionDegradedReason>,
    /// Non-visual accessibility routes this line offers.
    pub accessibility_routes: Vec<M5StableLineProtectionAccessibilityRoute>,
    /// Subsystems that consume this line's projection.
    pub consumer_surfaces: Vec<M5StableLineProtectionConsumerSurface>,
    /// Downgrade triggers that apply to this line.
    pub downgrade_triggers: Vec<M5StableLineProtectionDowngradeTrigger>,
    /// Proof packet refs that keep this line current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this line (must include its own canonical domain schema so
    /// downstream surfaces have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this line never widens support language without current refresh and correction evidence.
    /// MUST be `false`.
    pub widens_support_language_without_current_refresh_and_correction_evidence: bool,
    /// Hard invariant: this line never drifts a shipping line on stale evidence or frozen launch bundles. MUST be
    /// `false`.
    pub drifts_a_shipping_line_on_stale_evidence_or_frozen_launch_bundles: bool,
    /// Hard invariant: this line never relies on tribal backport memory instead of a documented correction packet. MUST be
    /// `false`.
    pub relies_on_tribal_backport_memory_instead_of_a_documented_correction_packet: bool,
    /// Hard invariant: this line never claims LTS eligibility without current rollback and support evidence. MUST be
    /// `false`.
    pub claims_lts_eligibility_without_current_rollback_and_support_evidence: bool,
    /// Hard invariant: this line never leaves a supported-line defect unowned or unresolved past its SLA. MUST
    /// line proof. MUST be `false`.
    pub leaves_a_supported_line_defect_unowned_or_unresolved_past_its_sla: bool,
}

impl M5StableLineProtectionRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5StableLineProtectionRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5StableLineProtectionRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.widens_support_language_without_current_refresh_and_correction_evidence
            && !self.drifts_a_shipping_line_on_stale_evidence_or_frozen_launch_bundles
            && !self.relies_on_tribal_backport_memory_instead_of_a_documented_correction_packet
            && !self.claims_lts_eligibility_without_current_rollback_and_support_evidence
            && !self.leaves_a_supported_line_defect_unowned_or_unresolved_past_its_sla
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableLineProtectionVocabularySet {
    /// Line-class tokens.
    pub line_classes: Vec<String>,
    /// Stable-line-protection-role tokens.
    pub semantic_roles: Vec<String>,
    /// Fresh-stable-line-role tokens.
    pub fresh_stable_line_roles: Vec<String>,
    /// Evidence-refresh-line-role tokens.
    pub evidence_refresh_line_roles: Vec<String>,
    /// Correction/backport-line-role tokens.
    pub correction_backport_line_roles: Vec<String>,
    /// Launch-bundle-currentness-line-role tokens.
    pub bundle_currentness_line_roles: Vec<String>,
    /// LTS-candidate-line-role tokens.
    pub lts_candidate_line_roles: Vec<String>,
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

impl M5StableLineProtectionVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            line_classes: tokens(&M5StableLineProtectionLine::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5StableLineProtectionRole::ALL, |v| v.as_str()),
            fresh_stable_line_roles: tokens(&M5FreshStableLineRole::ALL, |v| v.as_str()),
            evidence_refresh_line_roles: tokens(&M5EvidenceRefreshLineRole::ALL, |v| v.as_str()),
            correction_backport_line_roles: tokens(&M5CorrectionBackportLineRole::ALL, |v| {
                v.as_str()
            }),
            bundle_currentness_line_roles: tokens(&M5BundleCurrentnessLineRole::ALL, |v| {
                v.as_str()
            }),
            lts_candidate_line_roles: tokens(&M5LtsCandidateLineRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5StableLineProtectionSurfaceFamily::ALL, |v| v.as_str()),
            widening_stages: tokens(&M5StableLineProtectionWideningStage::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5StableLineProtectionConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5StableLineProtectionAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5StableLineProtectionDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5StableLineProtectionRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5StableLineProtectionDowngradeTrigger::ALL, |v| {
                v.as_str()
            }),
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
pub struct M5StableLineProtectionGovernanceReview {
    /// No stable claim skips lines.
    pub no_shipping_line_drifts_on_stale_evidence: bool,
    /// Every committed item enters with a requirement row, evidence class, matrix row, and rollback path.
    pub every_active_line_names_support_window_correction_owner_and_refresh_cadence: bool,
    /// Ring widening depends on current known-limits and rollback-stop rules.
    pub bundle_currentness_depends_on_current_refresh_audits: bool,
    /// Supported-line defects stay owned and resolved within SLA.
    pub supported_line_defects_stay_owned_and_resolved_within_sla: bool,
    /// Evidence-refresh cadence stays ordinary release ops.
    pub evidence_refresh_cadence_is_ordinary_release_ops: bool,
    /// Support-handoff drills stay current.
    pub first_correction_and_backport_path_exercised: bool,
    /// LTS decisions preserve the rollback and support evidence snapshot.
    pub lts_decisions_preserve_rollback_and_support_evidence_snapshot: bool,
    /// Freeze exceptions are documented, not implicit scope widening.
    pub backport_decisions_are_documented_not_tribal_memory: bool,
    /// Every line keeps the same truth across every widening stage.
    pub every_line_declares_widening_stages: bool,
    /// Every line declares a non-visual accessibility route.
    pub every_line_declares_accessibility_route: bool,
    /// Support / export reads a single canonical stable-line-protection source.
    pub support_export_reads_single_stable_line_source: bool,
    /// Shiproom, release center, and executive steering bind to a single canonical stable-line-protection source.
    pub release_help_and_support_bind_to_single_stable_line_source: bool,
    /// Later M5 rows cannot invent parallel stable-line-protection vocabulary.
    pub later_rows_cannot_invent_parallel_stable_line_vocabulary: bool,
    /// Stable-line-protection truth survives zoom and high contrast.
    pub stable_line_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when the registry is missing, stale, or not yet qualified.
    pub claims_narrow_automatically_when_registry_missing_or_stale: bool,
    /// Partner and public support language never outruns line proof.
    pub support_language_never_outruns_current_refresh_and_correction_proof: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableLineProtectionConsumerProjection {
    /// Shiproom and release center consume the shared stable-line-protection truth.
    pub release_and_help_consume_shared_stable_line_truth: bool,
    /// Support and public-proof consume the shared support-window and refresh truth.
    pub support_and_public_proof_consume_shared_support_window_and_refresh_truth: bool,
    /// Diagnostics and CLI/export consume the shared correction and bundle truth.
    pub diagnostics_and_cli_export_consume_shared_correction_and_bundle_truth: bool,
    /// Docs, help, and screenshots read a single stable-line-protection source.
    pub docs_help_and_screenshots_read_single_stable_line_source: bool,
    /// LTS and refresh proofs bind to the shared evidence snapshot.
    pub lts_and_refresh_proofs_bind_to_shared_evidence_snapshot: bool,
    /// Support / export reads a single canonical stable-line-protection source.
    pub support_export_reads_single_stable_line_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableLineProtectionProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the line.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the stable-line-protection lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableLineProtectionReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting stable-line-protection audit for the lane.
    pub stable_line_protection_audit_ref: String,
    /// True when support/export parity is required for every line.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every line.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5StableLineProtectionMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5StableLineProtectionMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Stable-line-protection rows.
    pub stable_line_protection_rows: Vec<M5StableLineProtectionRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5StableLineProtectionVocabularySet,
    /// Governance-review block.
    pub governance_review: M5StableLineProtectionGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5StableLineProtectionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5StableLineProtectionProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5StableLineProtectionReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 stable-line-protection matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableLineProtectionMatrixPacket {
    /// Record kind; must equal [`M5_STABLE_LINE_PROTECTION_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Stable-line-protection rows.
    pub stable_line_protection_rows: Vec<M5StableLineProtectionRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5StableLineProtectionVocabularySet,
    /// Governance-review block.
    pub governance_review: M5StableLineProtectionGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5StableLineProtectionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5StableLineProtectionProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5StableLineProtectionReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5StableLineProtectionMatrixPacket {
    /// Builds an M5 stable-line-protection matrix packet from stable-line input.
    pub fn new(input: M5StableLineProtectionMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_STABLE_LINE_PROTECTION_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            stable_line_protection_rows: input.stable_line_protection_rows,
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

    /// Validates the M5 stable-line-protection matrix invariants.
    pub fn validate(&self) -> Vec<M5StableLineProtectionMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_STABLE_LINE_PROTECTION_MATRIX_RECORD_KIND {
            violations.push(M5StableLineProtectionMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_VERSION {
            violations.push(M5StableLineProtectionMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5StableLineProtectionMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_stable_line_protection_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 stable-line-protection matrix serializes"),
        ) {
            violations.push(M5StableLineProtectionMatrixViolation::RawMaterialInExport);
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
            .expect("m5 stable-line-protection matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed line.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "line_class,qualification,owner,canonical_schema,surface_families,widening_stages,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.stable_line_protection_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.line_class.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.line_class.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.widening_stages, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic stable-line-protection dashboard JSON that shiproom and public-proof surfaces render from one
    /// canonical matrix instead of hand-authoring readiness chrome.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn render_dashboard_json(&self) -> String {
        let lines: Vec<serde_json::Value> = self
            .stable_line_protection_rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "line": row.line_class.as_str(),
                    "qualification": row.qualification.as_str(),
                    "canonical_schema": row.line_class.canonical_domain_schema_ref(),
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
            "record_kind": "m5_stable_line_health",
            "packet_id": self.packet_id,
            "matrix_label": self.matrix_label,
            "matrix_schema_ref": M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
            "support_export_ref": M5_STABLE_LINE_PROTECTION_ARTIFACT_REF,
            "widening_stages": self.vocabulary_set.widening_stages,
            "downgrade_triggers": self.vocabulary_set.downgrade_triggers,
            "lines": lines,
        });
        serde_json::to_string_pretty(&dashboard)
            .expect("m5 stable-line-protection dashboard serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_lines = self
            .stable_line_protection_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Stable-Line Protection, Evidence-Refresh, Correction-Line, and LTS-Readiness Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Lines: {} ({} stable)\n",
            self.stable_line_protection_rows.len(),
            stable_lines
        ));
        out.push_str(&format!(
            "- Stable-line-protection roles: {}\n",
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
        out.push_str("\n## Lines\n\n");
        for row in &self.stable_line_protection_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.line_class.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.line_class.canonical_domain_schema_ref()
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

/// Errors emitted when reading the checked-in M5 stable-line-protection matrix export.
#[derive(Debug)]
pub enum M5StableLineProtectionMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5StableLineProtectionMatrixViolation>),
}

impl fmt::Display for M5StableLineProtectionMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 stable-line-protection matrix export parse failed: {error}"
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
                    "m5 stable-line-protection matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5StableLineProtectionMatrixArtifactError {}

/// Validation failures emitted by [`M5StableLineProtectionMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5StableLineProtectionMatrixViolation {
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
    /// A required governed line is missing from the matrix.
    RequiredLineMissing,
    /// A stable-line-protection row is incomplete.
    StableLineProtectionRowIncomplete,
    /// A stable-line-protection row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A stable-line-protection row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A line declares no stable-line-protection roles.
    SemanticRoleMissing,
    /// The fresh stable line declares no fresh-stable-line roles.
    FreshStableLineRoleMissing,
    /// The evidence-refresh line declares no evidence-refresh-line roles.
    EvidenceRefreshLineRoleMissing,
    /// The correction/backport line declares no correction/backport-line roles.
    CorrectionBackportLineRoleMissing,
    /// The launch-bundle-currentness line declares no launch-bundle-currentness-line roles.
    BundleCurrentnessLineRoleMissing,
    /// The LTS-candidate line declares no LTS-candidate-line roles.
    LtsCandidateLineRoleMissing,
    /// A line declares no degraded reasons.
    DegradedReasonMissing,
    /// A line declares no surface families.
    SurfaceFamilyMissing,
    /// A line declares no widening stages.
    WideningStageMissing,
    /// A line declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A line declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A line declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A line claiming Stable is missing required proof packet refs.
    StableLineMissingProof,
    /// A line violates a hard invariant (widening support language without current refresh and correction
    /// evidence, drifting a shipping line on stale evidence or frozen launch bundles, relying on tribal backport
    /// memory instead of a documented correction packet, claiming LTS eligibility without current rollback and
    /// support evidence, or leaving a supported-line defect unowned or unresolved past its SLA).
    StableLineProtectionInvariantViolated,
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

impl M5StableLineProtectionMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredLineMissing => "required_line_missing",
            Self::StableLineProtectionRowIncomplete => "stable_line_protection_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::FreshStableLineRoleMissing => "fresh_stable_line_role_missing",
            Self::EvidenceRefreshLineRoleMissing => "evidence_refresh_line_role_missing",
            Self::CorrectionBackportLineRoleMissing => "correction_backport_line_role_missing",
            Self::BundleCurrentnessLineRoleMissing => "bundle_currentness_line_role_missing",
            Self::LtsCandidateLineRoleMissing => "lts_candidate_line_role_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::WideningStageMissing => "widening_stage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableLineMissingProof => "stable_line_missing_proof",
            Self::StableLineProtectionInvariantViolated => {
                "stable_line_protection_invariant_violated"
            }
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 stable-line-protection matrix export.
pub fn current_stable_m5_stable_line_protection_matrix_export(
) -> Result<M5StableLineProtectionMatrixPacket, M5StableLineProtectionMatrixArtifactError> {
    let packet: M5StableLineProtectionMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-stable-line-correction-reports/support_export.json"
    )))
    .map_err(M5StableLineProtectionMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5StableLineProtectionMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5StableLineProtectionMatrixPacket,
    violations: &mut Vec<M5StableLineProtectionMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_DOC_REF,
        M5_STABLE_LINE_REFRESH_POLICY_DOMAIN_SCHEMA_REF,
        M5_SUPPORTED_LINE_DEFECT_LEDGER_DOMAIN_SCHEMA_REF,
        M5_LTS_READINESS_DECISION_DOMAIN_SCHEMA_REF,
        M5_STABLE_CLAIM_MANIFEST_LANDED_SCHEMA_REF,
        M5_RELEASE_CENTER_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5StableLineProtectionMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5StableLineProtectionMatrixPacket,
    violations: &mut Vec<M5StableLineProtectionMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5StableLineProtectionMatrixViolation::VocabularySetDrift);
    }
}

fn validate_stable_line_protection_rows(
    packet: &M5StableLineProtectionMatrixPacket,
    violations: &mut Vec<M5StableLineProtectionMatrixViolation>,
) {
    let present: BTreeSet<M5StableLineProtectionLine> = packet
        .stable_line_protection_rows
        .iter()
        .map(|row| row.line_class)
        .collect();
    for required in M5StableLineProtectionLine::ALL {
        if !present.contains(&required) {
            violations.push(M5StableLineProtectionMatrixViolation::RequiredLineMissing);
            return;
        }
    }

    for row in &packet.stable_line_protection_rows {
        let line = row.line_class;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations
                .push(M5StableLineProtectionMatrixViolation::StableLineProtectionRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5StableLineProtectionMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == line.canonical_domain_schema_ref())
        {
            violations.push(M5StableLineProtectionMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5StableLineProtectionMatrixViolation::SemanticRoleMissing);
        }
        if line.declares_fresh_stable_line_roles() && row.fresh_stable_line_roles.is_empty() {
            violations.push(M5StableLineProtectionMatrixViolation::FreshStableLineRoleMissing);
        }
        if line.declares_evidence_refresh_line_roles() && row.evidence_refresh_line_roles.is_empty()
        {
            violations.push(M5StableLineProtectionMatrixViolation::EvidenceRefreshLineRoleMissing);
        }
        if line.declares_correction_backport_line_roles()
            && row.correction_backport_line_roles.is_empty()
        {
            violations
                .push(M5StableLineProtectionMatrixViolation::CorrectionBackportLineRoleMissing);
        }
        if line.declares_bundle_currentness_line_roles()
            && row.bundle_currentness_line_roles.is_empty()
        {
            violations
                .push(M5StableLineProtectionMatrixViolation::BundleCurrentnessLineRoleMissing);
        }
        if line.declares_lts_candidate_line_roles() && row.lts_candidate_line_roles.is_empty() {
            violations.push(M5StableLineProtectionMatrixViolation::LtsCandidateLineRoleMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5StableLineProtectionMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5StableLineProtectionMatrixViolation::SurfaceFamilyMissing);
        }
        if row.widening_stages.is_empty() {
            violations.push(M5StableLineProtectionMatrixViolation::WideningStageMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5StableLineProtectionMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5StableLineProtectionMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5StableLineProtectionMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5StableLineProtectionMatrixViolation::StableLineMissingProof);
        }
        if !row.honours_invariants() {
            violations
                .push(M5StableLineProtectionMatrixViolation::StableLineProtectionInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5StableLineProtectionMatrixPacket,
    violations: &mut Vec<M5StableLineProtectionMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.no_shipping_line_drifts_on_stale_evidence,
        review.every_active_line_names_support_window_correction_owner_and_refresh_cadence,
        review.bundle_currentness_depends_on_current_refresh_audits,
        review.supported_line_defects_stay_owned_and_resolved_within_sla,
        review.evidence_refresh_cadence_is_ordinary_release_ops,
        review.first_correction_and_backport_path_exercised,
        review.lts_decisions_preserve_rollback_and_support_evidence_snapshot,
        review.backport_decisions_are_documented_not_tribal_memory,
        review.every_line_declares_widening_stages,
        review.every_line_declares_accessibility_route,
        review.support_export_reads_single_stable_line_source,
        review.release_help_and_support_bind_to_single_stable_line_source,
        review.later_rows_cannot_invent_parallel_stable_line_vocabulary,
        review.stable_line_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_registry_missing_or_stale,
        review.support_language_never_outruns_current_refresh_and_correction_proof,
    ] {
        if !ok {
            violations.push(M5StableLineProtectionMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5StableLineProtectionMatrixPacket,
    violations: &mut Vec<M5StableLineProtectionMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.release_and_help_consume_shared_stable_line_truth,
        projection.support_and_public_proof_consume_shared_support_window_and_refresh_truth,
        projection.diagnostics_and_cli_export_consume_shared_correction_and_bundle_truth,
        projection.docs_help_and_screenshots_read_single_stable_line_source,
        projection.lts_and_refresh_proofs_bind_to_shared_evidence_snapshot,
        projection.support_export_reads_single_stable_line_source,
    ] {
        if !ok {
            violations.push(M5StableLineProtectionMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5StableLineProtectionMatrixPacket,
    violations: &mut Vec<M5StableLineProtectionMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5StableLineProtectionMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5StableLineProtectionMatrixPacket,
    violations: &mut Vec<M5StableLineProtectionMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.stable_line_protection_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5StableLineProtectionMatrixViolation::ReleasePostureIncomplete);
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
/// deliberately uses stable-line / refresh / correction / backport / LTS words; what is rejected is a
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
