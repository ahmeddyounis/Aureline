//! Frozen M5 supported-line public-proof, transparency-report, migration-scoreboard, ORR-history, and
//! correction-train-archive matrix.
//!
//! This module locks Aureline's durable post-launch external-proof object model — its supported-line public-proof
//! ledgers, upstream/compatibility transparency reports, versioned migration scoreboards, supported-line
//! ORR-history events, and correction-train archive packets, plus their public-safe versus internal-only
//! visibility posture — into one export-safe packet. Every governed supported-line proof object — the
//! public-proof ledger, the transparency report, the migration scoreboard, the ORR-history event, and the
//! correction-train archive — is named once here and constrained by the same shared transparency-role taxonomy
//! (freshness_window, transparency_disclosure, migration_scoreboard_currency, orr_history_retention,
//! correction_archive_retention, public_proof_freshness, correction_history_join), the same
//! no-claim-widens-because-a-report-once-existed-without-current-freshness rule, the same
//! no-supported-line-stays-green-on-stale-external-proof-or-opaque-upstream-health rule, the same
//! no-internal-only-detail-leaks-into-public-safe-feeds rule, the same
//! public-proof-migration-and-history-stay-joined-to-build-and-release-line-identity rule, and the same
//! migration-pain-and-ORR-and-correction-history-stay-retained rule regardless of the surface that renders it.
//!
//! The matrix does not redesign generic dashboard chrome or update-center / release-center UI — it is the shared
//! reusable public-proof, transparency, migration-scoreboard, ORR-history, and correction-archive engine contract
//! those already-governed surfaces consume, and it binds back to the already-landed stable-proof-index and
//! migration-task-row packets instead of leaving post-launch external truth split across scattered internal
//! notes. The controlled vocabularies are frozen in one self-describing
//! [`M5SupportedLineTransparencyVocabularySet`] rather than minted per surface. The single controlled
//! transparency-role vocabulary consumers bind to — freshness_window, transparency_disclosure,
//! migration_scoreboard_currency, orr_history_retention, correction_archive_retention, public_proof_freshness,
//! and correction_history_join — keeps every external claim entering scope through a freshness gate; keeps
//! migration scoreboards versioned and current; keeps ORR and correction history retained and archived; keeps
//! transparency reports export-safe with no internal-only leakage; keeps public-proof, migration, and history
//! joined to exact build and release-line identity; and keeps support language from outrunning current
//! public proof rather than reading as green. Raw secret values and private endpoints stay outside the export
//! boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_supported_line_transparency_matrix,
    seeded_m5_supported_line_transparency_matrix_correction_train_archive_preview_narrowed,
    seeded_m5_supported_line_transparency_matrix_orr_history_event_beta_narrowed,
    M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5SupportedLineTransparencyMatrixPacket`].
pub const M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_RECORD_KIND: &str =
    "freeze_m5_supported_line_public_proof_transparency_migration_scoreboard_orr_history_and_correction_train_archive_matrix";

/// Schema version for M5 stable-line-protection matrix records.
pub const M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined stable-line-protection matrix schema.
pub const M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF: &str =
    "schemas/program/m5-supported-line-transparency-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_DOC_REF: &str =
    "docs/release/m5-supported-line-transparency-ops.md";

/// Repo-relative path of the canonical public-proof-freshness-ledger domain schema (public-proof ledger and
/// transparency report: the public-claim proof, the compatibility/upstream-health report, the freshness window,
/// and the export-safe public view of a supported line).
pub const M5_PUBLIC_PROOF_FRESHNESS_LEDGER_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-public-proof-freshness-ledger.schema.json";

/// Repo-relative path of the canonical migration-scoreboard domain schema (migration scoreboard: the scored
/// migration path, tracked blockers, recorded migration-pain deltas, and the versioned scoreboard of a
/// supported line).
pub const M5_MIGRATION_SCOREBOARD_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-migration-scoreboard.schema.json";

/// Repo-relative path of the canonical supported-line-orr-history domain schema (ORR-history event: the
/// recorded ORR / go-no-go decision events, retained support-window decisions, and archived history of a
/// supported line).
pub const M5_SUPPORTED_LINE_ORR_HISTORY_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-supported-line-orr-history.schema.json";

/// Repo-relative path of the canonical correction-train-archive domain schema (correction-train archive: the
/// archived correction-train / hotfix-backport / advisory packets bound to exact build identity for a supported
/// line).
pub const M5_CORRECTION_TRAIN_ARCHIVE_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-correction-train-archive.schema.json";

/// Repo-relative path of the already-landed stable-proof-index schema the matrix binds back to.
pub const M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF: &str =
    "schemas/release/stable_proof_index.schema.json";

/// Repo-relative path of the already-landed migration-task-row schema the transparency matrix binds
/// back to.
pub const M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF: &str =
    "schemas/release/m5-migration-task-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SUPPORTED_LINE_TRANSPARENCY_FIXTURE_DIR: &str =
    "fixtures/release/m5-supported-line-transparency";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SUPPORTED_LINE_TRANSPARENCY_ARTIFACT_REF: &str =
    "artifacts/release/m5-supported-line-transparency/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SUPPORTED_LINE_TRANSPARENCY_CSV_REF: &str =
    "artifacts/release/m5-supported-line-transparency/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SUPPORTED_LINE_TRANSPARENCY_REPORT_REF: &str =
    "artifacts/program/m5-supported-line-transparency-matrix.md";

/// Repo-relative path of the checked stable-line-protection dashboard.
pub const M5_SUPPORTED_LINE_TRANSPARENCY_DASHBOARD_REF: &str =
    "dashboards/m5-supported-line-public-proof.json";

/// One of the five governed supported-line proof objects this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTransparencyObject {
    /// The public-proof ledger: the current public-claim, compatibility-report, and support-window proof kept
    /// fresh within its freshness window so external claims inherit current rather than tribal truth.
    PublicProofLedger,
    /// The transparency report: the export-safe upstream-health, compatibility-health, and maintainer-durability
    /// report that never leaks internal-only incident detail into a public-safe feed.
    TransparencyReport,
    /// The migration scoreboard: the versioned, scored migration path with tracked blockers and recorded
    /// migration-pain deltas so migration pain is never forgotten.
    MigrationScoreboard,
    /// The ORR-history event: the retained ORR / go-no-go / support-window decision history archived per
    /// supported line so decisions are never lost to memory.
    OrrHistoryEvent,
    /// The correction-train archive: the archived correction-train, hotfix-backport, and advisory packets bound
    /// to exact build identity so correction history stays durable and inspectable.
    CorrectionTrainArchive,
}

impl M5SupportedLineTransparencyObject {
    /// Every governed line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PublicProofLedger,
        Self::TransparencyReport,
        Self::MigrationScoreboard,
        Self::OrrHistoryEvent,
        Self::CorrectionTrainArchive,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicProofLedger => "public_proof_ledger",
            Self::TransparencyReport => "transparency_report",
            Self::MigrationScoreboard => "migration_scoreboard",
            Self::OrrHistoryEvent => "orr_history_event",
            Self::CorrectionTrainArchive => "correction_train_archive",
        }
    }

    /// The canonical per-domain schema ref a downstream surface points at instead of restating this object's
    /// public-proof, migration-scoreboard, ORR-history, or correction-archive meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::PublicProofLedger | Self::TransparencyReport => {
                M5_PUBLIC_PROOF_FRESHNESS_LEDGER_DOMAIN_SCHEMA_REF
            }
            Self::MigrationScoreboard => M5_MIGRATION_SCOREBOARD_DOMAIN_SCHEMA_REF,
            Self::OrrHistoryEvent => M5_SUPPORTED_LINE_ORR_HISTORY_DOMAIN_SCHEMA_REF,
            Self::CorrectionTrainArchive => M5_CORRECTION_TRAIN_ARCHIVE_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this object must name a controlled public-proof-ledger role.
    pub const fn declares_public_proof_ledger_roles(self) -> bool {
        matches!(self, Self::PublicProofLedger)
    }

    /// `true` when this object must name a controlled transparency-report role.
    pub const fn declares_transparency_report_roles(self) -> bool {
        matches!(self, Self::TransparencyReport)
    }

    /// `true` when this object must name a controlled migration-scoreboard role.
    pub const fn declares_migration_scoreboard_roles(self) -> bool {
        matches!(self, Self::MigrationScoreboard)
    }

    /// `true` when this object must name a controlled ORR-history-event role.
    pub const fn declares_orr_history_event_roles(self) -> bool {
        matches!(self, Self::OrrHistoryEvent)
    }

    /// `true` when this object must name a controlled correction-train-archive role.
    pub const fn declares_correction_train_archive_roles(self) -> bool {
        matches!(self, Self::CorrectionTrainArchive)
    }
}

/// The single controlled transparency-role vocabulary every release, help, docs, support, public-proof, or
/// partner/procurement consumer binds to. These are the exact acceptance-criteria tokens that keep
/// `freshness_window`, `transparency_disclosure`, `migration_scoreboard_currency`, `orr_history_retention`,
/// `correction_archive_retention`, `public_proof_freshness`, and `correction_history_join` meaning the same thing
/// everywhere the supported-line transparency grammar ships. No surface invents a parallel word for any of these
/// roles, and the freshness-window / transparency-disclosure / correction-archive-retention / orr-history-retention
/// roles may never let a claim widen because a report once existed without current freshness, leak internal-only
/// detail into a public feed, drop ORR or correction history, or unjoin proof from build and release-line
/// identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTransparencyRole {
    /// Freshness-window role (the declared freshness window a public-proof object must publish before widening).
    FreshnessWindow,
    /// Transparency-disclosure role (the export-safe upstream/compatibility disclosure with no internal leakage).
    TransparencyDisclosure,
    /// Migration-scoreboard-currency role (the versioned, current migration scoreboard state).
    MigrationScoreboardCurrency,
    /// ORR-history-retention role (the retained, archived ORR / go-no-go decision history).
    OrrHistoryRetention,
    /// Correction-archive-retention role (the archived correction-train packets bound to exact build identity).
    CorrectionArchiveRetention,
    /// Public-proof-freshness role (the current public-claim / compatibility-report / support-window proof).
    PublicProofFreshness,
    /// Correction-history-join role (the join that keeps proof and history bound to build/release-line identity).
    CorrectionHistoryJoin,
}

impl M5SupportedLineTransparencyRole {
    /// Every transparency-role token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FreshnessWindow,
        Self::TransparencyDisclosure,
        Self::MigrationScoreboardCurrency,
        Self::OrrHistoryRetention,
        Self::CorrectionArchiveRetention,
        Self::PublicProofFreshness,
        Self::CorrectionHistoryJoin,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshnessWindow => "freshness_window",
            Self::TransparencyDisclosure => "transparency_disclosure",
            Self::MigrationScoreboardCurrency => "migration_scoreboard_currency",
            Self::OrrHistoryRetention => "orr_history_retention",
            Self::CorrectionArchiveRetention => "correction_archive_retention",
            Self::PublicProofFreshness => "public_proof_freshness",
            Self::CorrectionHistoryJoin => "correction_history_join",
        }
    }

    /// Whether this role carries freshness-window, transparency-disclosure, correction-archive-retention, or
    /// orr-history-retention truth whose per-object behavior must never let a claim widen because a report once
    /// existed without current freshness, leak internal-only detail into a public feed, drop ORR or correction
    /// history, or unjoin proof from build and release-line identity (`freshness_window`,
    /// `transparency_disclosure`, `correction_archive_retention`, `orr_history_retention`). The descriptive
    /// structure roles (`migration_scoreboard_currency`, `public_proof_freshness`, `correction_history_join`) are
    /// inspectable descriptors rather than widening-authority truth and so do not carry this requirement.
    pub const fn must_preserve_evidence_snapshot_and_signoff_before_widening(self) -> bool {
        matches!(
            self,
            Self::FreshnessWindow
                | Self::TransparencyDisclosure
                | Self::CorrectionArchiveRetention
                | Self::OrrHistoryRetention
        )
    }
}

/// Controlled public-proof-ledger role — how a supported line's public claims stay current, so the public-claim
/// proof, the published compatibility report, the deprecation-notice proof, and the freshness window met stay
/// under one transparency registry rather than reading as green on stale external proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicProofLedgerRole {
    /// Current public-claim proof recorded in the ledger.
    PublicClaimProofCurrent,
    /// Compatibility report published for the line.
    CompatibilityReportPublished,
    /// Deprecation / support-window notice proof current.
    DeprecationNoticeProofCurrent,
    /// Freshness window met for the public-proof ledger.
    ProofFreshnessWindowMet,
    /// A role bound to the single transparency registry.
    BoundToTransparencyRegistry,
    /// Stale public proof published without current freshness, which is disallowed.
    StalePublicProofPublishedDisallowed,
}

impl M5PublicProofLedgerRole {
    /// Every public-proof-ledger role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PublicClaimProofCurrent,
        Self::CompatibilityReportPublished,
        Self::DeprecationNoticeProofCurrent,
        Self::ProofFreshnessWindowMet,
        Self::BoundToTransparencyRegistry,
        Self::StalePublicProofPublishedDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicClaimProofCurrent => "public_claim_proof_current",
            Self::CompatibilityReportPublished => "compatibility_report_published",
            Self::DeprecationNoticeProofCurrent => "deprecation_notice_proof_current",
            Self::ProofFreshnessWindowMet => "proof_freshness_window_met",
            Self::BoundToTransparencyRegistry => "bound_to_transparency_registry",
            Self::StalePublicProofPublishedDisallowed => "stale_public_proof_published_disallowed",
        }
    }
}

/// Controlled transparency-report role — how the upstream-health, compatibility-health, and maintainer-durability
/// report stays export-safe under one transparency registry rather than leaking internal-only incident or
/// security detail into a public-safe or partner/procurement feed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TransparencyReportRole {
    /// Upstream health reported for the line.
    UpstreamHealthReported,
    /// Compatibility health reported for the line.
    CompatibilityHealthReported,
    /// Maintainer durability reported for the line.
    MaintainerDurabilityReported,
    /// Report kept an export-safe public view.
    ReportIsExportSafePublicView,
    /// A role bound to the single transparency registry.
    BoundToTransparencyRegistry,
    /// Internal-only incident or security detail leaked into a public feed, which is disallowed.
    InternalIncidentDetailLeakedDisallowed,
}

impl M5TransparencyReportRole {
    /// Every transparency-report role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UpstreamHealthReported,
        Self::CompatibilityHealthReported,
        Self::MaintainerDurabilityReported,
        Self::ReportIsExportSafePublicView,
        Self::BoundToTransparencyRegistry,
        Self::InternalIncidentDetailLeakedDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpstreamHealthReported => "upstream_health_reported",
            Self::CompatibilityHealthReported => "compatibility_health_reported",
            Self::MaintainerDurabilityReported => "maintainer_durability_reported",
            Self::ReportIsExportSafePublicView => "report_is_export_safe_public_view",
            Self::BoundToTransparencyRegistry => "bound_to_transparency_registry",
            Self::InternalIncidentDetailLeakedDisallowed => {
                "internal_incident_detail_leaked_disallowed"
            }
        }
    }
}

/// Controlled migration-scoreboard role — how a supported line's migration pain stays scored and versioned, so
/// the migration path scored, the blockers tracked, the migration-pain deltas recorded, and the scoreboard
/// versioned follow one transparency registry rather than letting migration pain be forgotten between trains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MigrationScoreboardRole {
    /// Migration path scored on the scoreboard.
    MigrationPathScored,
    /// Migration blockers tracked on the scoreboard.
    MigrationBlockerTracked,
    /// Migration-pain deltas recorded across trains.
    MigrationPainDeltaRecorded,
    /// Scoreboard versioned and current.
    MigrationScoreboardVersioned,
    /// A role bound to the single transparency registry.
    BoundToTransparencyRegistry,
    /// Migration pain forgotten between release trains, which is disallowed.
    ForgottenMigrationPainDisallowed,
}

impl M5MigrationScoreboardRole {
    /// Every migration-scoreboard role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MigrationPathScored,
        Self::MigrationBlockerTracked,
        Self::MigrationPainDeltaRecorded,
        Self::MigrationScoreboardVersioned,
        Self::BoundToTransparencyRegistry,
        Self::ForgottenMigrationPainDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MigrationPathScored => "migration_path_scored",
            Self::MigrationBlockerTracked => "migration_blocker_tracked",
            Self::MigrationPainDeltaRecorded => "migration_pain_delta_recorded",
            Self::MigrationScoreboardVersioned => "migration_scoreboard_versioned",
            Self::BoundToTransparencyRegistry => "bound_to_transparency_registry",
            Self::ForgottenMigrationPainDisallowed => "forgotten_migration_pain_disallowed",
        }
    }
}

/// Controlled ORR-history-event role — how a supported line's ORR and support-window decisions stay retained,
/// so the ORR decision event recorded, the go/no-go outcome preserved, the deprecation decision retained, and
/// the history event archived follow one transparency registry rather than being lost to memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OrrHistoryEventRole {
    /// ORR decision event recorded in history.
    OrrDecisionEventRecorded,
    /// Go/no-go outcome preserved with the event.
    GoNoGoOutcomePreserved,
    /// Deprecation / support-window decision retained.
    DeprecationDecisionRetained,
    /// ORR-history event archived per supported line.
    OrrHistoryEventArchived,
    /// A role bound to the single transparency registry.
    BoundToTransparencyRegistry,
    /// ORR / support-window history left unretained, which is disallowed.
    MissingOrrHistoryDisallowed,
}

impl M5OrrHistoryEventRole {
    /// Every ORR-history-event role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OrrDecisionEventRecorded,
        Self::GoNoGoOutcomePreserved,
        Self::DeprecationDecisionRetained,
        Self::OrrHistoryEventArchived,
        Self::BoundToTransparencyRegistry,
        Self::MissingOrrHistoryDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrrDecisionEventRecorded => "orr_decision_event_recorded",
            Self::GoNoGoOutcomePreserved => "go_no_go_outcome_preserved",
            Self::DeprecationDecisionRetained => "deprecation_decision_retained",
            Self::OrrHistoryEventArchived => "orr_history_event_archived",
            Self::BoundToTransparencyRegistry => "bound_to_transparency_registry",
            Self::MissingOrrHistoryDisallowed => "missing_orr_history_disallowed",
        }
    }
}

/// Controlled correction-train-archive role — how a supported line's correction history stays durable, so the
/// correction-train packet archived, the hotfix/backport packet archived, the advisory packet archived, and the
/// archive packet bound to exact build identity follow one transparency registry rather than being scattered as
/// internal notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CorrectionTrainArchiveRole {
    /// Correction-train packet archived.
    CorrectionTrainPacketArchived,
    /// Hotfix / backport packet archived.
    HotfixBackportPacketArchived,
    /// Advisory packet archived.
    AdvisoryPacketArchived,
    /// Archive packet bound to exact build identity.
    ArchivePacketBoundToBuildIdentity,
    /// A role bound to the single transparency registry.
    BoundToTransparencyRegistry,
    /// Correction history left unarchived or unbound to build identity, which is disallowed.
    MissingCorrectionArchiveDisallowed,
}

impl M5CorrectionTrainArchiveRole {
    /// Every correction-train-archive role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CorrectionTrainPacketArchived,
        Self::HotfixBackportPacketArchived,
        Self::AdvisoryPacketArchived,
        Self::ArchivePacketBoundToBuildIdentity,
        Self::BoundToTransparencyRegistry,
        Self::MissingCorrectionArchiveDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorrectionTrainPacketArchived => "correction_train_packet_archived",
            Self::HotfixBackportPacketArchived => "hotfix_backport_packet_archived",
            Self::AdvisoryPacketArchived => "advisory_packet_archived",
            Self::ArchivePacketBoundToBuildIdentity => "archive_packet_bound_to_build_identity",
            Self::BoundToTransparencyRegistry => "bound_to_transparency_registry",
            Self::MissingCorrectionArchiveDisallowed => "missing_correction_archive_disallowed",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a line. No line may invent a parallel surface
/// taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTransparencySurfaceFamily {
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

impl M5SupportedLineTransparencySurfaceFamily {
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
pub enum M5SupportedLineTransparencyWideningStage {
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

impl M5SupportedLineTransparencyWideningStage {
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
pub enum M5SupportedLineTransparencyConsumerSurface {
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

impl M5SupportedLineTransparencyConsumerSurface {
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
pub enum M5SupportedLineTransparencyAccessibilityRoute {
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

impl M5SupportedLineTransparencyAccessibilityRoute {
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
pub enum M5SupportedLineTransparencyDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The public-proof-freshness-ledger source is unavailable.
    PublicProofSourceUnavailable,
    /// The migration-scoreboard source is unavailable.
    MigrationScoreboardSourceUnavailable,
    /// The correction-train-archive / ORR-history source is unavailable.
    CorrectionArchiveSourceUnavailable,
    /// Transparency-report evidence is unverified.
    TransparencyEvidenceUnverified,
    /// The public-proof owner is unknown.
    ProofOwnershipUnknown,
}

impl M5SupportedLineTransparencyDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::PublicProofSourceUnavailable,
        Self::MigrationScoreboardSourceUnavailable,
        Self::CorrectionArchiveSourceUnavailable,
        Self::TransparencyEvidenceUnverified,
        Self::ProofOwnershipUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PublicProofSourceUnavailable => "public_proof_source_unavailable",
            Self::MigrationScoreboardSourceUnavailable => "migration_scoreboard_source_unavailable",
            Self::CorrectionArchiveSourceUnavailable => "correction_archive_source_unavailable",
            Self::TransparencyEvidenceUnverified => "transparency_evidence_unverified",
            Self::ProofOwnershipUnknown => "proof_ownership_unknown",
        }
    }
}

/// Mandatory label a claimed proof object must be able to show. The first three are hard requirements on every
/// object; the remaining three close the acceptance-criteria ambiguity about the freshness window, the export
/// class, and the supported-line association.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTransparencyRequiredLabel {
    /// The object's stable identity.
    Identity,
    /// The object's transparency role.
    TransparencyRole,
    /// The canonical registry reference the object points at.
    RegistryReference,
    /// The freshness window the object must publish.
    FreshnessWindow,
    /// The public-safe versus internal-only export class the object holds.
    ExportClass,
    /// The supported-line association the object joins to.
    LineAssociation,
}

impl M5SupportedLineTransparencyRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::TransparencyRole,
        Self::RegistryReference,
        Self::FreshnessWindow,
        Self::ExportClass,
        Self::LineAssociation,
    ];

    /// The three labels every claimed line must be able to show.
    pub const MANDATORY: [Self; 3] = [
        Self::Identity,
        Self::TransparencyRole,
        Self::RegistryReference,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::TransparencyRole => "transparency_role",
            Self::RegistryReference => "registry_reference",
            Self::FreshnessWindow => "freshness_window",
            Self::ExportClass => "export_class",
            Self::LineAssociation => "line_association",
        }
    }
}

/// Qualification class for an M5 stable-line-protection row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTransparencyQualificationClass {
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

impl M5SupportedLineTransparencyQualificationClass {
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

/// Downgrade trigger that narrows a proof object below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTransparencyDowngradeTrigger {
    /// A claim widened because a report once existed, on stale public proof.
    WidenedClaimOnStalePublicProof,
    /// A claim widened without a current transparency report.
    WidenedClaimWithoutCurrentTransparencyReport,
    /// Internal-only detail leaked into a public-safe or partner/procurement feed.
    LeakedInternalDetailIntoPublicProof,
    /// Migration pain was left unscored on the scoreboard.
    LeftMigrationPainUnscored,
    /// A surface implied green while public proof or the correction archive was stale.
    ImpliedGreenWhileProofOrArchiveWasStale,
    /// Partner or public support language ran ahead of current public proof.
    RanSupportLanguageAheadOfPublicProof,
    /// An object left its freshness window unstated.
    FreshnessWindowUnstated,
    /// An object left its export class unstated.
    ExportClassUnstated,
    /// An object left its supported-line association unstated.
    LineAssociationUnstated,
    /// An object left its canonical registry reference unstated.
    RegistryReferenceUnstated,
    /// ORR / correction history was left unretained.
    OrrHistoryUnretained,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5SupportedLineTransparencyDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::WidenedClaimOnStalePublicProof,
        Self::WidenedClaimWithoutCurrentTransparencyReport,
        Self::LeakedInternalDetailIntoPublicProof,
        Self::LeftMigrationPainUnscored,
        Self::ImpliedGreenWhileProofOrArchiveWasStale,
        Self::RanSupportLanguageAheadOfPublicProof,
        Self::FreshnessWindowUnstated,
        Self::ExportClassUnstated,
        Self::LineAssociationUnstated,
        Self::RegistryReferenceUnstated,
        Self::OrrHistoryUnretained,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WidenedClaimOnStalePublicProof => "widened_claim_on_stale_public_proof",
            Self::WidenedClaimWithoutCurrentTransparencyReport => {
                "widened_claim_without_current_transparency_report"
            }
            Self::LeakedInternalDetailIntoPublicProof => "leaked_internal_detail_into_public_proof",
            Self::LeftMigrationPainUnscored => "left_migration_pain_unscored",
            Self::ImpliedGreenWhileProofOrArchiveWasStale => {
                "implied_green_while_proof_or_archive_was_stale"
            }
            Self::RanSupportLanguageAheadOfPublicProof => {
                "ran_support_language_ahead_of_public_proof"
            }
            Self::FreshnessWindowUnstated => "freshness_window_unstated",
            Self::ExportClassUnstated => "export_class_unstated",
            Self::LineAssociationUnstated => "line_association_unstated",
            Self::RegistryReferenceUnstated => "registry_reference_unstated",
            Self::OrrHistoryUnretained => "orr_history_unretained",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed proof object bound to the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTransparencyRow {
    /// Governed proof object.
    pub proof_object: M5SupportedLineTransparencyObject,
    /// Qualification class earned by this object.
    pub qualification: M5SupportedLineTransparencyQualificationClass,
    /// Owner role accountable for keeping this object governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this object.
    pub surface_families: Vec<M5SupportedLineTransparencySurfaceFamily>,
    /// Widening stages this object must gate before claiming the next channel.
    pub widening_stages: Vec<M5SupportedLineTransparencyWideningStage>,
    /// Mandatory labels this object must be able to show (must include the three
    /// [`M5SupportedLineTransparencyRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5SupportedLineTransparencyRequiredLabel>,
    /// Transparency roles this object can carry (the frozen AC vocabulary; required on every object).
    pub semantic_roles: Vec<M5SupportedLineTransparencyRole>,
    /// Public-proof-ledger roles this object names (public-proof ledger only).
    pub public_proof_ledger_roles: Vec<M5PublicProofLedgerRole>,
    /// Transparency-report roles this object names (transparency report only).
    pub transparency_report_roles: Vec<M5TransparencyReportRole>,
    /// Migration-scoreboard roles this object names (migration scoreboard only).
    pub migration_scoreboard_roles: Vec<M5MigrationScoreboardRole>,
    /// ORR-history-event roles this object names (ORR-history event only).
    pub orr_history_event_roles: Vec<M5OrrHistoryEventRole>,
    /// Correction-train-archive roles this object names (correction-train archive only).
    pub correction_train_archive_roles: Vec<M5CorrectionTrainArchiveRole>,
    /// Degraded reasons this object can name (required on every object).
    pub degraded_reasons: Vec<M5SupportedLineTransparencyDegradedReason>,
    /// Non-visual accessibility routes this object offers.
    pub accessibility_routes: Vec<M5SupportedLineTransparencyAccessibilityRoute>,
    /// Subsystems that consume this object's projection.
    pub consumer_surfaces: Vec<M5SupportedLineTransparencyConsumerSurface>,
    /// Downgrade triggers that apply to this object.
    pub downgrade_triggers: Vec<M5SupportedLineTransparencyDowngradeTrigger>,
    /// Proof packet refs that keep this object current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this object (must include its own canonical domain schema so
    /// downstream surfaces have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this object never widens a claim because a report once existed without current freshness.
    /// MUST be `false`.
    pub widens_a_claim_because_a_report_once_existed_without_current_freshness: bool,
    /// Hard invariant: this object never stays green on stale external proof or opaque upstream health. MUST be
    /// `false`.
    pub stays_green_on_stale_external_proof_or_opaque_upstream_health: bool,
    /// Hard invariant: this object never leaks internal-only incident or security detail into public-safe feeds.
    /// MUST be `false`.
    pub leaks_internal_only_incident_or_security_detail_into_public_safe_feeds: bool,
    /// Hard invariant: this object never leaves public-proof, migration, or history unjoined to build and
    /// release-line identity. MUST be `false`.
    pub leaves_public_proof_migration_or_history_unjoined_to_build_and_release_line_identity: bool,
    /// Hard invariant: this object never leaves migration pain or ORR / correction history unretained. MUST be
    /// `false`.
    pub leaves_migration_pain_or_orr_and_correction_history_unretained: bool,
}

impl M5SupportedLineTransparencyRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5SupportedLineTransparencyRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5SupportedLineTransparencyRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.widens_a_claim_because_a_report_once_existed_without_current_freshness
            && !self.stays_green_on_stale_external_proof_or_opaque_upstream_health
            && !self.leaks_internal_only_incident_or_security_detail_into_public_safe_feeds
            && !self.leaves_public_proof_migration_or_history_unjoined_to_build_and_release_line_identity
            && !self.leaves_migration_pain_or_orr_and_correction_history_unretained
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTransparencyVocabularySet {
    /// Line-class tokens.
    pub proof_objectes: Vec<String>,
    /// Stable-line-protection-role tokens.
    pub semantic_roles: Vec<String>,
    /// Fresh-stable-line-role tokens.
    pub public_proof_ledger_roles: Vec<String>,
    /// Evidence-refresh-line-role tokens.
    pub transparency_report_roles: Vec<String>,
    /// Correction/backport-line-role tokens.
    pub migration_scoreboard_roles: Vec<String>,
    /// Launch-bundle-currentness-line-role tokens.
    pub orr_history_event_roles: Vec<String>,
    /// LTS-candidate-line-role tokens.
    pub correction_train_archive_roles: Vec<String>,
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

impl M5SupportedLineTransparencyVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            proof_objectes: tokens(&M5SupportedLineTransparencyObject::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5SupportedLineTransparencyRole::ALL, |v| v.as_str()),
            public_proof_ledger_roles: tokens(&M5PublicProofLedgerRole::ALL, |v| v.as_str()),
            transparency_report_roles: tokens(&M5TransparencyReportRole::ALL, |v| v.as_str()),
            migration_scoreboard_roles: tokens(&M5MigrationScoreboardRole::ALL, |v| v.as_str()),
            orr_history_event_roles: tokens(&M5OrrHistoryEventRole::ALL, |v| v.as_str()),
            correction_train_archive_roles: tokens(&M5CorrectionTrainArchiveRole::ALL, |v| {
                v.as_str()
            }),
            surface_families: tokens(&M5SupportedLineTransparencySurfaceFamily::ALL, |v| {
                v.as_str()
            }),
            widening_stages: tokens(&M5SupportedLineTransparencyWideningStage::ALL, |v| {
                v.as_str()
            }),
            consumer_surfaces: tokens(&M5SupportedLineTransparencyConsumerSurface::ALL, |v| {
                v.as_str()
            }),
            accessibility_routes: tokens(
                &M5SupportedLineTransparencyAccessibilityRoute::ALL,
                |v| v.as_str(),
            ),
            degraded_reasons: tokens(&M5SupportedLineTransparencyDegradedReason::ALL, |v| {
                v.as_str()
            }),
            required_labels: tokens(&M5SupportedLineTransparencyRequiredLabel::ALL, |v| {
                v.as_str()
            }),
            downgrade_triggers: tokens(&M5SupportedLineTransparencyDowngradeTrigger::ALL, |v| {
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
pub struct M5SupportedLineTransparencyGovernanceReview {
    /// No supported line stays green on stale external proof or opaque upstream health.
    pub no_supported_line_stays_green_on_stale_external_proof: bool,
    /// Every supported proof object names its owner, freshness window, and export class.
    pub every_supported_object_names_owner_freshness_window_and_export_class: bool,
    /// Migration scoreboards stay versioned and current.
    pub migration_scoreboard_stays_versioned_and_current: bool,
    /// ORR and correction history is retained, not forgotten.
    pub orr_and_correction_history_is_retained_not_forgotten: bool,
    /// Transparency reports stay an export-safe public view.
    pub transparency_reports_stay_export_safe_public_view: bool,
    /// Public-proof ledgers stay current on active lines.
    pub public_proof_ledgers_stay_current_on_active_lines: bool,
    /// Correction-train packets are archived and bound to exact build identity.
    pub correction_train_packets_are_archived_and_build_bound: bool,
    /// Internal-only incident detail never leaks into public feeds.
    pub internal_incident_detail_never_leaks_into_public_feeds: bool,
    /// Every object keeps the same truth across every widening stage.
    pub every_object_declares_widening_stages: bool,
    /// Every object declares a non-visual accessibility route.
    pub every_object_declares_accessibility_route: bool,
    /// Support / export reads a single canonical transparency source.
    pub support_export_reads_single_transparency_source: bool,
    /// Release, help, and support bind to a single canonical transparency source.
    pub release_help_and_support_bind_to_single_transparency_source: bool,
    /// Later M5 rows cannot invent parallel transparency vocabulary.
    pub later_rows_cannot_invent_parallel_transparency_vocabulary: bool,
    /// Transparency truth survives zoom and high contrast.
    pub transparency_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when the matrix row is missing, stale, or not yet qualified.
    pub claims_narrow_automatically_when_matrix_row_missing_or_stale: bool,
    /// Partner and public support language never outruns current public proof.
    pub support_language_never_outruns_current_public_proof: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTransparencyConsumerProjection {
    /// Release and help consume the shared transparency truth.
    pub release_and_help_consume_shared_transparency_truth: bool,
    /// Support and public-proof consume the shared public-proof and freshness truth.
    pub support_and_public_proof_consume_shared_public_proof_and_freshness_truth: bool,
    /// Diagnostics and CLI/export consume the shared migration and archive truth.
    pub diagnostics_and_cli_export_consume_shared_migration_and_archive_truth: bool,
    /// Docs, help, and screenshots read a single transparency source.
    pub docs_help_and_screenshots_read_single_transparency_source: bool,
    /// ORR and correction archives bind to the shared build identity.
    pub orr_and_correction_archives_bind_to_shared_build_identity: bool,
    /// Support / export reads a single canonical transparency source.
    pub support_export_reads_single_transparency_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTransparencyProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the line.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the stable-line-protection lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTransparencyReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting stable-line-protection audit for the lane.
    pub supported_line_transparency_audit_ref: String,
    /// True when support/export parity is required for every line.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every line.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SupportedLineTransparencyMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SupportedLineTransparencyMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Stable-line-protection rows.
    pub supported_line_transparency_rows: Vec<M5SupportedLineTransparencyRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SupportedLineTransparencyVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SupportedLineTransparencyGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SupportedLineTransparencyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SupportedLineTransparencyProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SupportedLineTransparencyReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 stable-line-protection matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportedLineTransparencyMatrixPacket {
    /// Record kind; must equal [`M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Stable-line-protection rows.
    pub supported_line_transparency_rows: Vec<M5SupportedLineTransparencyRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SupportedLineTransparencyVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SupportedLineTransparencyGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SupportedLineTransparencyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SupportedLineTransparencyProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SupportedLineTransparencyReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SupportedLineTransparencyMatrixPacket {
    /// Builds an M5 stable-line-protection matrix packet from stable-line input.
    pub fn new(input: M5SupportedLineTransparencyMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            supported_line_transparency_rows: input.supported_line_transparency_rows,
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
    pub fn validate(&self) -> Vec<M5SupportedLineTransparencyMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_RECORD_KIND {
            violations.push(M5SupportedLineTransparencyMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_VERSION {
            violations.push(M5SupportedLineTransparencyMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SupportedLineTransparencyMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_supported_line_transparency_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 stable-line-protection matrix serializes"),
        ) {
            violations.push(M5SupportedLineTransparencyMatrixViolation::RawMaterialInExport);
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
            "proof_object,qualification,owner,canonical_schema,surface_families,widening_stages,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.supported_line_transparency_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.proof_object.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.proof_object.canonical_domain_schema_ref(),
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
            .supported_line_transparency_rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "line": row.proof_object.as_str(),
                    "qualification": row.qualification.as_str(),
                    "canonical_schema": row.proof_object.canonical_domain_schema_ref(),
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
            "record_kind": "m5_supported_line_public_proof",
            "packet_id": self.packet_id,
            "matrix_label": self.matrix_label,
            "matrix_schema_ref": M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
            "support_export_ref": M5_SUPPORTED_LINE_TRANSPARENCY_ARTIFACT_REF,
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
            .supported_line_transparency_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Supported-Line Public-Proof, Transparency-Report, Migration-Scoreboard, ORR-History, and Correction-Train-Archive Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Lines: {} ({} stable)\n",
            self.supported_line_transparency_rows.len(),
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
        for row in &self.supported_line_transparency_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.proof_object.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.proof_object.canonical_domain_schema_ref()
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
pub enum M5SupportedLineTransparencyMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SupportedLineTransparencyMatrixViolation>),
}

impl fmt::Display for M5SupportedLineTransparencyMatrixArtifactError {
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

impl Error for M5SupportedLineTransparencyMatrixArtifactError {}

/// Validation failures emitted by [`M5SupportedLineTransparencyMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SupportedLineTransparencyMatrixViolation {
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
    SupportedLineTransparencyRowIncomplete,
    /// A stable-line-protection row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A stable-line-protection row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A line declares no stable-line-protection roles.
    SemanticRoleMissing,
    /// The fresh stable line declares no fresh-stable-line roles.
    PublicProofLedgerRoleMissing,
    /// The evidence-refresh line declares no evidence-refresh-line roles.
    TransparencyReportRoleMissing,
    /// The correction/backport line declares no correction/backport-line roles.
    MigrationScoreboardRoleMissing,
    /// The launch-bundle-currentness line declares no launch-bundle-currentness-line roles.
    OrrHistoryEventRoleMissing,
    /// The LTS-candidate line declares no LTS-candidate-line roles.
    CorrectionTrainArchiveRoleMissing,
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
    SupportedLineTransparencyInvariantViolated,
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

impl M5SupportedLineTransparencyMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredLineMissing => "required_line_missing",
            Self::SupportedLineTransparencyRowIncomplete => {
                "supported_line_transparency_row_incomplete"
            }
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::PublicProofLedgerRoleMissing => "public_proof_ledger_role_missing",
            Self::TransparencyReportRoleMissing => "transparency_report_role_missing",
            Self::MigrationScoreboardRoleMissing => "migration_scoreboard_role_missing",
            Self::OrrHistoryEventRoleMissing => "orr_history_event_role_missing",
            Self::CorrectionTrainArchiveRoleMissing => "correction_train_archive_role_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::WideningStageMissing => "widening_stage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableLineMissingProof => "stable_line_missing_proof",
            Self::SupportedLineTransparencyInvariantViolated => {
                "supported_line_transparency_invariant_violated"
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
pub fn current_stable_m5_supported_line_transparency_matrix_export(
) -> Result<M5SupportedLineTransparencyMatrixPacket, M5SupportedLineTransparencyMatrixArtifactError>
{
    let packet: M5SupportedLineTransparencyMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-supported-line-transparency/support_export.json"
        )))
        .map_err(M5SupportedLineTransparencyMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SupportedLineTransparencyMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5SupportedLineTransparencyMatrixPacket,
    violations: &mut Vec<M5SupportedLineTransparencyMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_DOC_REF,
        M5_PUBLIC_PROOF_FRESHNESS_LEDGER_DOMAIN_SCHEMA_REF,
        M5_MIGRATION_SCOREBOARD_DOMAIN_SCHEMA_REF,
        M5_SUPPORTED_LINE_ORR_HISTORY_DOMAIN_SCHEMA_REF,
        M5_CORRECTION_TRAIN_ARCHIVE_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SupportedLineTransparencyMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SupportedLineTransparencyMatrixPacket,
    violations: &mut Vec<M5SupportedLineTransparencyMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5SupportedLineTransparencyMatrixViolation::VocabularySetDrift);
    }
}

fn validate_supported_line_transparency_rows(
    packet: &M5SupportedLineTransparencyMatrixPacket,
    violations: &mut Vec<M5SupportedLineTransparencyMatrixViolation>,
) {
    let present: BTreeSet<M5SupportedLineTransparencyObject> = packet
        .supported_line_transparency_rows
        .iter()
        .map(|row| row.proof_object)
        .collect();
    for required in M5SupportedLineTransparencyObject::ALL {
        if !present.contains(&required) {
            violations.push(M5SupportedLineTransparencyMatrixViolation::RequiredLineMissing);
            return;
        }
    }

    for row in &packet.supported_line_transparency_rows {
        let line = row.proof_object;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(
                M5SupportedLineTransparencyMatrixViolation::SupportedLineTransparencyRowIncomplete,
            );
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5SupportedLineTransparencyMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == line.canonical_domain_schema_ref())
        {
            violations.push(M5SupportedLineTransparencyMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5SupportedLineTransparencyMatrixViolation::SemanticRoleMissing);
        }
        if line.declares_public_proof_ledger_roles() && row.public_proof_ledger_roles.is_empty() {
            violations
                .push(M5SupportedLineTransparencyMatrixViolation::PublicProofLedgerRoleMissing);
        }
        if line.declares_transparency_report_roles() && row.transparency_report_roles.is_empty() {
            violations
                .push(M5SupportedLineTransparencyMatrixViolation::TransparencyReportRoleMissing);
        }
        if line.declares_migration_scoreboard_roles() && row.migration_scoreboard_roles.is_empty() {
            violations
                .push(M5SupportedLineTransparencyMatrixViolation::MigrationScoreboardRoleMissing);
        }
        if line.declares_orr_history_event_roles() && row.orr_history_event_roles.is_empty() {
            violations.push(M5SupportedLineTransparencyMatrixViolation::OrrHistoryEventRoleMissing);
        }
        if line.declares_correction_train_archive_roles()
            && row.correction_train_archive_roles.is_empty()
        {
            violations.push(
                M5SupportedLineTransparencyMatrixViolation::CorrectionTrainArchiveRoleMissing,
            );
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5SupportedLineTransparencyMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5SupportedLineTransparencyMatrixViolation::SurfaceFamilyMissing);
        }
        if row.widening_stages.is_empty() {
            violations.push(M5SupportedLineTransparencyMatrixViolation::WideningStageMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5SupportedLineTransparencyMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5SupportedLineTransparencyMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5SupportedLineTransparencyMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5SupportedLineTransparencyMatrixViolation::StableLineMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(
                M5SupportedLineTransparencyMatrixViolation::SupportedLineTransparencyInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5SupportedLineTransparencyMatrixPacket,
    violations: &mut Vec<M5SupportedLineTransparencyMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.no_supported_line_stays_green_on_stale_external_proof,
        review.every_supported_object_names_owner_freshness_window_and_export_class,
        review.migration_scoreboard_stays_versioned_and_current,
        review.orr_and_correction_history_is_retained_not_forgotten,
        review.transparency_reports_stay_export_safe_public_view,
        review.public_proof_ledgers_stay_current_on_active_lines,
        review.correction_train_packets_are_archived_and_build_bound,
        review.internal_incident_detail_never_leaks_into_public_feeds,
        review.every_object_declares_widening_stages,
        review.every_object_declares_accessibility_route,
        review.support_export_reads_single_transparency_source,
        review.release_help_and_support_bind_to_single_transparency_source,
        review.later_rows_cannot_invent_parallel_transparency_vocabulary,
        review.transparency_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_matrix_row_missing_or_stale,
        review.support_language_never_outruns_current_public_proof,
    ] {
        if !ok {
            violations.push(M5SupportedLineTransparencyMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SupportedLineTransparencyMatrixPacket,
    violations: &mut Vec<M5SupportedLineTransparencyMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.release_and_help_consume_shared_transparency_truth,
        projection.support_and_public_proof_consume_shared_public_proof_and_freshness_truth,
        projection.diagnostics_and_cli_export_consume_shared_migration_and_archive_truth,
        projection.docs_help_and_screenshots_read_single_transparency_source,
        projection.orr_and_correction_archives_bind_to_shared_build_identity,
        projection.support_export_reads_single_transparency_source,
    ] {
        if !ok {
            violations
                .push(M5SupportedLineTransparencyMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SupportedLineTransparencyMatrixPacket,
    violations: &mut Vec<M5SupportedLineTransparencyMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SupportedLineTransparencyMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SupportedLineTransparencyMatrixPacket,
    violations: &mut Vec<M5SupportedLineTransparencyMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture
            .supported_line_transparency_audit_ref
            .trim()
            .is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SupportedLineTransparencyMatrixViolation::ReleasePostureIncomplete);
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
