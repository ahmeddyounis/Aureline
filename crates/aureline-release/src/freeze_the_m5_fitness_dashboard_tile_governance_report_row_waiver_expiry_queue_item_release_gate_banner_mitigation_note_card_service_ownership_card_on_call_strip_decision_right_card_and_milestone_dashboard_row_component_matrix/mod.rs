//! Frozen M5 fitness-dashboard-tile, governance-report-row,
//! waiver-expiry-queue-item, release-gate-banner, mitigation-note-card,
//! service-ownership-card, on-call-strip, decision-right-card, and
//! milestone-dashboard-row component matrix.
//!
//! This module locks Aureline's reusable governance-dashboard components into one
//! export-safe packet. Every component family M5 claims that still drifts too
//! easily by assurance dashboard, operator board, or shiproom spreadsheet — the
//! fitness dashboard tile, the governance report row, the waiver-expiry queue
//! item, the release-gate banner, the mitigation note card, the service-ownership
//! card, the on-call strip, the decision-right card, and the milestone dashboard
//! row — is named once here and constrained by the same passing/blocked/waived/
//! evidence-stale readiness vocabulary, corpus/profile provenance, waiver-expiry,
//! owner-freshness, escalation-route, decision-forum, and mitigation-language rules
//! regardless of the surface family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components*
//! themselves: the component families, the shared governance readiness states, the
//! fitness provenance classes, the governance report scopes, the waiver expiry
//! states, the release-gate decisions, the mitigation postures, the ownership
//! coverage states, the on-call coverage states and escalation route classes, the
//! decision forum classes and decision-right states, the milestone gate states,
//! the surface families and deployment lines every component must survive, the
//! non-visual accessibility routes, and the mandatory labels every component must
//! be able to show. It does not re-architect the fitness feeds, waiver ledgers,
//! ownership maps, or decision-forum manifests that already own those records — it
//! is the shared component contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 assurance,
//! operator, or shiproom component may render a fitness, governance, waiver,
//! release-gate, mitigation, ownership, on-call, decision-right, or milestone
//! claim. Assurance-center, operator-overview, release-center, shiproom,
//! service-health, support, docs, and admin surfaces all consume this packet so one
//! fitness tile carries its corpus/profile provenance and evidence freshness, one
//! governance report row states its readiness, one waiver-expiry item names when a
//! waiver lapses, one release-gate banner states its ship/no-ship reason, one
//! mitigation note card carries user-facing mitigation language, one
//! service-ownership card names owner coverage and freshness, one on-call strip
//! names the escalation route, one decision-right card names the forum that can
//! actually approve the next move, and one milestone dashboard row states its exit
//! gate. No M5 lane invents a second governance-status grammar, renders a waived or
//! stale reading as a clean pass, lets an ownerless or forumless blocker read as
//! resolved, or hides mitigation text behind internal jargon.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5GovernanceDashboardVocabularySet`] rather than minted per surface. Raw URLs,
//! raw tokens, credentials, private endpoints, and user text bodies stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-governance-dashboard-component-matrix.schema.json`](../../../../schemas/ui/m5-governance-dashboard-component-matrix.schema.json)
//! and the contract doc is
//! [`docs/help/m5_governance_dashboard_components_contract.md`](../../../../docs/help/m5_governance_dashboard_components_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-governance-dashboard-components/`](../../../../fixtures/ui/m5-governance-dashboard-components/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_governance_dashboard_component_matrix,
    seeded_m5_governance_dashboard_component_matrix_release_gate_banner_preview_narrowed,
    seeded_m5_governance_dashboard_component_matrix_service_ownership_card_beta_narrowed,
    M5_GOVERNANCE_DASHBOARD_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5GovernanceDashboardMatrixPacket`].
pub const M5_GOVERNANCE_DASHBOARD_MATRIX_RECORD_KIND: &str =
    "freeze_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix";

/// Schema version for M5 governance-dashboard-component-matrix records.
pub const M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the governance-dashboard-component-matrix boundary schema.
pub const M5_GOVERNANCE_DASHBOARD_SCHEMA_REF: &str =
    "schemas/ui/m5-governance-dashboard-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_GOVERNANCE_DASHBOARD_DOC_REF: &str =
    "docs/help/m5_governance_dashboard_components_contract.md";

/// Per-component boundary schema: fitness dashboard tile.
pub const M5_FITNESS_DASHBOARD_TILE_SCHEMA_REF: &str =
    "schemas/ui/m5-fitness-dashboard-tile.schema.json";
/// Per-component boundary schema: governance report row.
pub const M5_GOVERNANCE_REPORT_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-governance-report-row.schema.json";
/// Per-component boundary schema: waiver-expiry queue item.
pub const M5_WAIVER_EXPIRY_QUEUE_ITEM_SCHEMA_REF: &str =
    "schemas/ui/m5-waiver-expiry-queue-item.schema.json";
/// Per-component boundary schema: release-gate banner.
pub const M5_RELEASE_GATE_BANNER_SCHEMA_REF: &str = "schemas/ui/m5-release-gate-banner.schema.json";
/// Per-component boundary schema: mitigation note card.
pub const M5_MITIGATION_NOTE_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-mitigation-note-card.schema.json";
/// Per-component boundary schema: service-ownership card.
pub const M5_SERVICE_OWNERSHIP_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-service-ownership-card.schema.json";
/// Per-component boundary schema: on-call strip.
pub const M5_ON_CALL_STRIP_SCHEMA_REF: &str = "schemas/ui/m5-on-call-strip.schema.json";
/// Per-component boundary schema: decision-right card.
pub const M5_DECISION_RIGHT_CARD_SCHEMA_REF: &str = "schemas/ui/m5-decision-right-card.schema.json";
/// Per-component boundary schema: milestone dashboard row.
pub const M5_MILESTONE_DASHBOARD_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-milestone-dashboard-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_GOVERNANCE_DASHBOARD_FIXTURE_DIR: &str =
    "fixtures/ui/m5-governance-dashboard-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_GOVERNANCE_DASHBOARD_ARTIFACT_REF: &str =
    "artifacts/release/m5-governance-dashboard-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_GOVERNANCE_DASHBOARD_CSV_REF: &str =
    "artifacts/release/m5-governance-dashboard-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_GOVERNANCE_DASHBOARD_REPORT_REF: &str =
    "artifacts/release/m5-governance-dashboard-proof/summary.md";

/// Repo-relative path of the design contract narrative.
pub const M5_GOVERNANCE_DASHBOARD_DESIGN_REF: &str =
    "artifacts/design/m5-governance-dashboard-component-matrix.md";

/// One of the nine governed governance-dashboard component families this matrix
/// freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceDashboardComponentFamily {
    /// A fitness dashboard tile carrying a fitness-function reading and its
    /// corpus/profile provenance.
    FitnessDashboardTile,
    /// A governance report row carrying a lane's readiness and its evidence.
    GovernanceReportRow,
    /// A waiver-expiry queue item naming when a waiver lapses.
    WaiverExpiryQueueItem,
    /// A release-gate banner naming a ship/no-ship decision and its reason.
    ReleaseGateBanner,
    /// A mitigation note card carrying user-facing mitigation language.
    MitigationNoteCard,
    /// A service-ownership card naming owner coverage and freshness.
    ServiceOwnershipCard,
    /// An on-call strip naming on-call coverage and the escalation route.
    OnCallStrip,
    /// A decision-right card naming the forum authorized to approve the next move.
    DecisionRightCard,
    /// A milestone dashboard row naming a milestone's exit gate state.
    MilestoneDashboardRow,
}

impl M5GovernanceDashboardComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::FitnessDashboardTile,
        Self::GovernanceReportRow,
        Self::WaiverExpiryQueueItem,
        Self::ReleaseGateBanner,
        Self::MitigationNoteCard,
        Self::ServiceOwnershipCard,
        Self::OnCallStrip,
        Self::DecisionRightCard,
        Self::MilestoneDashboardRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FitnessDashboardTile => "fitness_dashboard_tile",
            Self::GovernanceReportRow => "governance_report_row",
            Self::WaiverExpiryQueueItem => "waiver_expiry_queue_item",
            Self::ReleaseGateBanner => "release_gate_banner",
            Self::MitigationNoteCard => "mitigation_note_card",
            Self::ServiceOwnershipCard => "service_ownership_card",
            Self::OnCallStrip => "on_call_strip",
            Self::DecisionRightCard => "decision_right_card",
            Self::MilestoneDashboardRow => "milestone_dashboard_row",
        }
    }

    /// `true` when this family is a fitness dashboard tile and must therefore
    /// declare its fitness provenance classes.
    pub const fn is_fitness_tile(self) -> bool {
        matches!(self, Self::FitnessDashboardTile)
    }

    /// `true` when this family is a governance report row and must therefore declare
    /// its report scopes.
    pub const fn is_report_row(self) -> bool {
        matches!(self, Self::GovernanceReportRow)
    }

    /// `true` when this family is a waiver-expiry queue item and must therefore
    /// declare its waiver expiry states.
    pub const fn is_waiver_item(self) -> bool {
        matches!(self, Self::WaiverExpiryQueueItem)
    }

    /// `true` when this family is a release-gate banner and must therefore declare
    /// its release-gate decisions.
    pub const fn is_release_gate(self) -> bool {
        matches!(self, Self::ReleaseGateBanner)
    }

    /// `true` when this family is a mitigation note card and must therefore declare
    /// its mitigation postures.
    pub const fn is_mitigation_card(self) -> bool {
        matches!(self, Self::MitigationNoteCard)
    }

    /// `true` when this family is a service-ownership card and must therefore
    /// declare its ownership coverage states.
    pub const fn is_ownership_card(self) -> bool {
        matches!(self, Self::ServiceOwnershipCard)
    }

    /// `true` when this family is an on-call strip and must therefore declare its
    /// on-call coverage states and escalation route classes.
    pub const fn is_on_call(self) -> bool {
        matches!(self, Self::OnCallStrip)
    }

    /// `true` when this family is a decision-right card and must therefore declare
    /// its decision forum classes and decision-right states.
    pub const fn is_decision_right(self) -> bool {
        matches!(self, Self::DecisionRightCard)
    }

    /// `true` when this family is a milestone dashboard row and must therefore
    /// declare its milestone gate states.
    pub const fn is_milestone_row(self) -> bool {
        matches!(self, Self::MilestoneDashboardRow)
    }
}

/// The frozen governance readiness state vocabulary shared by every component so no
/// surface invents a dashboard-local status word. This is the one acceptance-
/// criteria vocabulary the matrix locks: passing-versus-warning-versus-blocked-
/// versus-waived-versus-expired-versus-stale-versus-unresolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceReadinessState {
    /// The lane is passing.
    Passing,
    /// The lane is passing but degraded / at warning.
    Warning,
    /// The lane is blocked by a hard, gating problem.
    Blocked,
    /// A blocker is held under a disclosed, still-valid waiver.
    Waived,
    /// A waiver that previously held the lane has expired.
    ExpiredWaiver,
    /// The lane's evidence is stale relative to the current build.
    EvidenceStale,
    /// The lane has no resolved owner.
    OwnerUnresolved,
    /// The lane has no authorized decision forum.
    ForumUnresolved,
    /// The lane has not been evaluated on this build.
    NotEvaluated,
}

impl M5GovernanceReadinessState {
    /// Every readiness state, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Passing,
        Self::Warning,
        Self::Blocked,
        Self::Waived,
        Self::ExpiredWaiver,
        Self::EvidenceStale,
        Self::OwnerUnresolved,
        Self::ForumUnresolved,
        Self::NotEvaluated,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passing => "passing",
            Self::Warning => "warning",
            Self::Blocked => "blocked",
            Self::Waived => "waived",
            Self::ExpiredWaiver => "expired_waiver",
            Self::EvidenceStale => "evidence_stale",
            Self::OwnerUnresolved => "owner_unresolved",
            Self::ForumUnresolved => "forum_unresolved",
            Self::NotEvaluated => "not_evaluated",
        }
    }

    /// `true` only for [`Self::Passing`]. Every other state must never be rendered
    /// as a clean pass.
    pub const fn is_clean_pass(self) -> bool {
        matches!(self, Self::Passing)
    }
}

/// Controlled fitness provenance class — which corpus/profile a fitness reading came
/// from, so a fitness dashboard tile never leaves its provenance implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FitnessProvenanceClass {
    /// The canonical, pinned corpus.
    CanonicalCorpus,
    /// A pinned execution profile.
    ProfilePinned,
    /// A sampled subset of the corpus.
    SampledCorpus,
    /// A synthetic / generated corpus.
    SyntheticCorpus,
    /// The provenance is unknown / unrecorded.
    ProvenanceUnknown,
}

impl M5FitnessProvenanceClass {
    /// Every fitness provenance class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CanonicalCorpus,
        Self::ProfilePinned,
        Self::SampledCorpus,
        Self::SyntheticCorpus,
        Self::ProvenanceUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalCorpus => "canonical_corpus",
            Self::ProfilePinned => "profile_pinned",
            Self::SampledCorpus => "sampled_corpus",
            Self::SyntheticCorpus => "synthetic_corpus",
            Self::ProvenanceUnknown => "provenance_unknown",
        }
    }
}

/// Controlled governance report scope — how wide a governance report row reaches, so
/// a report row never leaves its aggregation scope implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceReportScope {
    /// A single service.
    ServiceScope,
    /// One feature family.
    FamilyScope,
    /// One release train.
    TrainScope,
    /// The whole fleet.
    FleetScope,
    /// The waiver ledger.
    WaiverLedgerScope,
}

impl M5GovernanceReportScope {
    /// Every governance report scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ServiceScope,
        Self::FamilyScope,
        Self::TrainScope,
        Self::FleetScope,
        Self::WaiverLedgerScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServiceScope => "service_scope",
            Self::FamilyScope => "family_scope",
            Self::TrainScope => "train_scope",
            Self::FleetScope => "fleet_scope",
            Self::WaiverLedgerScope => "waiver_ledger_scope",
        }
    }
}

/// Controlled waiver expiry state — the lifecycle posture of a waiver, so a
/// waiver-expiry queue item never shows an expired or revoked waiver as active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WaiverExpiryState {
    /// The waiver is active and still valid.
    ActiveWaiver,
    /// The waiver is active but expiring soon.
    ExpiringSoon,
    /// The waiver has expired.
    ExpiredWaiver,
    /// The waiver was revoked before its natural expiry.
    RevokedWaiver,
    /// No waiver applies to this item.
    NoWaiver,
}

impl M5WaiverExpiryState {
    /// Every waiver expiry state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ActiveWaiver,
        Self::ExpiringSoon,
        Self::ExpiredWaiver,
        Self::RevokedWaiver,
        Self::NoWaiver,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveWaiver => "active_waiver",
            Self::ExpiringSoon => "expiring_soon",
            Self::ExpiredWaiver => "expired_waiver",
            Self::RevokedWaiver => "revoked_waiver",
            Self::NoWaiver => "no_waiver",
        }
    }
}

/// Controlled release-gate decision — the ship/no-ship posture a release-gate banner
/// names, so a gate is never shown as go while a blocker, owner, or forum is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseGateDecision {
    /// Cleared to ship.
    Go,
    /// Held from shipping.
    NoGo,
    /// Cleared to ship under a stated condition.
    ConditionalGo,
    /// Held pending fresh evidence.
    HeldPendingEvidence,
    /// Held by an unresolved owner or decision forum.
    BlockedByOwnerOrForum,
}

impl M5ReleaseGateDecision {
    /// Every release-gate decision, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Go,
        Self::NoGo,
        Self::ConditionalGo,
        Self::HeldPendingEvidence,
        Self::BlockedByOwnerOrForum,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::NoGo => "no_go",
            Self::ConditionalGo => "conditional_go",
            Self::HeldPendingEvidence => "held_pending_evidence",
            Self::BlockedByOwnerOrForum => "blocked_by_owner_or_forum",
        }
    }
}

/// Controlled mitigation posture — how far a risk is mitigated, so a mitigation note
/// card never shows an unmitigated or merely accepted risk as resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MitigationPosture {
    /// Fully mitigated.
    Mitigated,
    /// Partially mitigated.
    PartiallyMitigated,
    /// Not mitigated.
    Unmitigated,
    /// The risk is accepted without mitigation.
    RiskAccepted,
    /// The mitigation posture is unknown.
    MitigationUnknown,
}

impl M5MitigationPosture {
    /// Every mitigation posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Mitigated,
        Self::PartiallyMitigated,
        Self::Unmitigated,
        Self::RiskAccepted,
        Self::MitigationUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mitigated => "mitigated",
            Self::PartiallyMitigated => "partially_mitigated",
            Self::Unmitigated => "unmitigated",
            Self::RiskAccepted => "risk_accepted",
            Self::MitigationUnknown => "mitigation_unknown",
        }
    }
}

/// Controlled ownership coverage state — the owner posture of a service, so a
/// service-ownership card never shows a backup-missing or unresolved owner as
/// covered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OwnershipCoverageState {
    /// A primary owner with a named backup.
    OwnedWithBackup,
    /// A primary owner only, with no backup.
    PrimaryOnlyNoBackup,
    /// No resolved owner.
    OwnerUnresolved,
    /// The owner record is stale relative to the roster.
    OwnerStale,
    /// Ownership is hidden by policy on this surface.
    PolicyHidden,
}

impl M5OwnershipCoverageState {
    /// Every ownership coverage state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OwnedWithBackup,
        Self::PrimaryOnlyNoBackup,
        Self::OwnerUnresolved,
        Self::OwnerStale,
        Self::PolicyHidden,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnedWithBackup => "owned_with_backup",
            Self::PrimaryOnlyNoBackup => "primary_only_no_backup",
            Self::OwnerUnresolved => "owner_unresolved",
            Self::OwnerStale => "owner_stale",
            Self::PolicyHidden => "policy_hidden",
        }
    }

    /// `true` only for [`Self::OwnedWithBackup`]. Every other state must never be
    /// rendered as fully covered.
    pub const fn is_fully_covered(self) -> bool {
        matches!(self, Self::OwnedWithBackup)
    }
}

/// Controlled on-call coverage state — the on-call posture of a service, so an
/// on-call strip never shows an on-call gap as covered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OnCallCoverageState {
    /// On-call is covered.
    OnCallCovered,
    /// There is an on-call gap.
    OnCallGap,
    /// Only an escalation path is available, no primary on-call.
    EscalationOnly,
    /// Follow-the-sun coverage across regions.
    FollowTheSun,
    /// The on-call posture is unknown.
    OnCallUnknown,
}

impl M5OnCallCoverageState {
    /// Every on-call coverage state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OnCallCovered,
        Self::OnCallGap,
        Self::EscalationOnly,
        Self::FollowTheSun,
        Self::OnCallUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnCallCovered => "on_call_covered",
            Self::OnCallGap => "on_call_gap",
            Self::EscalationOnly => "escalation_only",
            Self::FollowTheSun => "follow_the_sun",
            Self::OnCallUnknown => "on_call_unknown",
        }
    }
}

/// Controlled escalation route class — how a page escalates, so an on-call strip
/// never leaves the escalation route implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EscalationRouteClass {
    /// Page the primary on-call.
    PagePrimary,
    /// Page the backup on-call.
    PageBackup,
    /// Escalate to the manager.
    EscalateToManager,
    /// Open an incident bridge.
    IncidentBridge,
    /// No escalation path exists.
    NoEscalationPath,
}

impl M5EscalationRouteClass {
    /// Every escalation route class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PagePrimary,
        Self::PageBackup,
        Self::EscalateToManager,
        Self::IncidentBridge,
        Self::NoEscalationPath,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PagePrimary => "page_primary",
            Self::PageBackup => "page_backup",
            Self::EscalateToManager => "escalate_to_manager",
            Self::IncidentBridge => "incident_bridge",
            Self::NoEscalationPath => "no_escalation_path",
        }
    }
}

/// Controlled decision forum class — which forum can approve a governance move, so a
/// decision-right card never masks who is authorized to decide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionForumClass {
    /// The release council.
    ReleaseCouncil,
    /// The standing service owner.
    ServiceOwner,
    /// The security review board.
    SecurityReviewBoard,
    /// The architecture forum.
    ArchitectureForum,
    /// No authorized forum exists.
    NoAuthorizedForum,
}

impl M5DecisionForumClass {
    /// Every decision forum class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReleaseCouncil,
        Self::ServiceOwner,
        Self::SecurityReviewBoard,
        Self::ArchitectureForum,
        Self::NoAuthorizedForum,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCouncil => "release_council",
            Self::ServiceOwner => "service_owner",
            Self::SecurityReviewBoard => "security_review_board",
            Self::ArchitectureForum => "architecture_forum",
            Self::NoAuthorizedForum => "no_authorized_forum",
        }
    }
}

/// Controlled decision-right state — whether the named forum can actually decide, so
/// a decision-right card never lets an advisory or unresolved forum read as
/// authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionRightState {
    /// The forum is authoritative for this decision.
    AuthoritativeForum,
    /// The forum is advisory only.
    AdvisoryOnly,
    /// No authorized forum is resolved for this decision.
    ForumUnresolved,
    /// The decision is delegated to another forum.
    DelegatedDecision,
    /// The decision right has not been evaluated here.
    NotEvaluatedHere,
}

impl M5DecisionRightState {
    /// Every decision-right state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AuthoritativeForum,
        Self::AdvisoryOnly,
        Self::ForumUnresolved,
        Self::DelegatedDecision,
        Self::NotEvaluatedHere,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeForum => "authoritative_forum",
            Self::AdvisoryOnly => "advisory_only",
            Self::ForumUnresolved => "forum_unresolved",
            Self::DelegatedDecision => "delegated_decision",
            Self::NotEvaluatedHere => "not_evaluated_here",
        }
    }

    /// `true` only for [`Self::AuthoritativeForum`]. Every other state must never be
    /// rendered as an authoritative approval.
    pub const fn is_authoritative(self) -> bool {
        matches!(self, Self::AuthoritativeForum)
    }
}

/// Controlled milestone gate state — the exit-gate posture of a milestone, so a
/// milestone dashboard row never shows a blocked, waived, or stale gate as met.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MilestoneGateState {
    /// The exit gate is met.
    ExitGateMet,
    /// The exit gate is pending.
    ExitGatePending,
    /// The exit gate is blocked.
    ExitGateBlocked,
    /// The exit gate is held under a waiver.
    ExitGateWaived,
    /// The exit-gate evidence is stale.
    ExitGateStale,
}

impl M5MilestoneGateState {
    /// Every milestone gate state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExitGateMet,
        Self::ExitGatePending,
        Self::ExitGateBlocked,
        Self::ExitGateWaived,
        Self::ExitGateStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExitGateMet => "exit_gate_met",
            Self::ExitGatePending => "exit_gate_pending",
            Self::ExitGateBlocked => "exit_gate_blocked",
            Self::ExitGateWaived => "exit_gate_waived",
            Self::ExitGateStale => "exit_gate_stale",
        }
    }
}

/// Claimed M5 governance surface family that renders / consumes a governance-
/// dashboard component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceSurfaceFamily {
    /// The assurance-center surface.
    AssuranceCenter,
    /// The operator-overview surface.
    OperatorOverview,
    /// The release-center surface.
    ReleaseCenter,
    /// The shiproom surface.
    Shiproom,
    /// The service-health surface.
    ServiceHealth,
    /// Support-desk surfaces.
    SupportDesk,
    /// Docs / Help surfaces.
    DocsHelp,
    /// Admin review surfaces.
    AdminReview,
}

impl M5GovernanceSurfaceFamily {
    /// Every governance surface family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::AssuranceCenter,
        Self::OperatorOverview,
        Self::ReleaseCenter,
        Self::Shiproom,
        Self::ServiceHealth,
        Self::SupportDesk,
        Self::DocsHelp,
        Self::AdminReview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssuranceCenter => "assurance_center",
            Self::OperatorOverview => "operator_overview",
            Self::ReleaseCenter => "release_center",
            Self::Shiproom => "shiproom",
            Self::ServiceHealth => "service_health",
            Self::SupportDesk => "support_desk",
            Self::DocsHelp => "docs_help",
            Self::AdminReview => "admin_review",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's
/// scope never silently narrows or widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5DeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Governance subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceConsumerSurface {
    /// The assurance dashboard.
    AssuranceDashboard,
    /// The operator board.
    OperatorBoard,
    /// The release-center UI.
    ReleaseCenterUi,
    /// The shiproom packet.
    ShiproomPacket,
    /// The service-health surface.
    ServiceHealth,
    /// The Help / About surface.
    HelpAbout,
    /// The support export.
    SupportExport,
    /// The docs portal.
    DocsPortal,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The general product UI.
    ProductUi,
}

impl M5GovernanceConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::AssuranceDashboard,
        Self::OperatorBoard,
        Self::ReleaseCenterUi,
        Self::ShiproomPacket,
        Self::ServiceHealth,
        Self::HelpAbout,
        Self::SupportExport,
        Self::DocsPortal,
        Self::CliInspect,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssuranceDashboard => "assurance_dashboard",
            Self::OperatorBoard => "operator_board",
            Self::ReleaseCenterUi => "release_center_ui",
            Self::ShiproomPacket => "shiproom_packet",
            Self::ServiceHealth => "service_health",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
            Self::DocsPortal => "docs_portal",
            Self::CliInspect => "cli_inspect",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no governance
/// truth is hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5GovernanceAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed governance-dashboard component must be able to show.
/// The first three are hard requirements on every component; the remaining three
/// close the acceptance-criteria ambiguity about evidence freshness, owner and
/// escalation route, and decision forum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceRequiredLabel {
    /// The component's stable identity / what governance object it represents.
    Identity,
    /// The component's current readiness state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The evidence-freshness reading behind the component's claim.
    EvidenceFreshness,
    /// The owner and escalation route behind the component's claim.
    OwnerAndEscalation,
    /// The decision forum authorized for the component's move.
    DecisionForum,
}

impl M5GovernanceRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::EvidenceFreshness,
        Self::OwnerAndEscalation,
        Self::DecisionForum,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::EvidenceFreshness => "evidence_freshness",
            Self::OwnerAndEscalation => "owner_and_escalation",
            Self::DecisionForum => "decision_forum",
        }
    }
}

/// Qualification class for an M5 governance-dashboard-component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5GovernanceQualificationClass {
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

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a governance-dashboard component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceDowngradeTrigger {
    /// A fitness tile left its corpus/profile provenance unstated.
    FitnessProvenanceUnstated,
    /// A component hid stale evidence behind a clean reading.
    EvidenceStaleHidden,
    /// A waiver-expiry item hid when a waiver lapses.
    WaiverExpiryHidden,
    /// A release-gate banner gave a generic ship/no-ship reason.
    ReleaseGateReasonGeneric,
    /// A mitigation note card hid its mitigation behind internal jargon.
    MitigationHiddenBehindJargon,
    /// A service-ownership card overstated owner coverage.
    OwnerCoverageOverstated,
    /// An on-call strip hid an on-call gap.
    OnCallGapHidden,
    /// An on-call strip left the escalation route unstated.
    EscalationRouteUnstated,
    /// A decision-right card masked the decision forum.
    DecisionForumMasked,
    /// A decision-right card let an advisory forum read as authoritative.
    AdvisoryForumReadsAuthoritative,
    /// A milestone dashboard row overstated its exit gate.
    MilestoneGateOverstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5GovernanceDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::FitnessProvenanceUnstated,
        Self::EvidenceStaleHidden,
        Self::WaiverExpiryHidden,
        Self::ReleaseGateReasonGeneric,
        Self::MitigationHiddenBehindJargon,
        Self::OwnerCoverageOverstated,
        Self::OnCallGapHidden,
        Self::EscalationRouteUnstated,
        Self::DecisionForumMasked,
        Self::AdvisoryForumReadsAuthoritative,
        Self::MilestoneGateOverstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FitnessProvenanceUnstated => "fitness_provenance_unstated",
            Self::EvidenceStaleHidden => "evidence_stale_hidden",
            Self::WaiverExpiryHidden => "waiver_expiry_hidden",
            Self::ReleaseGateReasonGeneric => "release_gate_reason_generic",
            Self::MitigationHiddenBehindJargon => "mitigation_hidden_behind_jargon",
            Self::OwnerCoverageOverstated => "owner_coverage_overstated",
            Self::OnCallGapHidden => "on_call_gap_hidden",
            Self::EscalationRouteUnstated => "escalation_route_unstated",
            Self::DecisionForumMasked => "decision_forum_masked",
            Self::AdvisoryForumReadsAuthoritative => "advisory_forum_reads_authoritative",
            Self::MilestoneGateOverstated => "milestone_gate_overstated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed governance-dashboard component family bound to
/// the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceDashboardComponentRow {
    /// Governed component family.
    pub component_family: M5GovernanceDashboardComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5GovernanceQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 governance surface families that render / consume this component.
    pub surface_families: Vec<M5GovernanceSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5GovernanceRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5GovernanceRequiredLabel>,
    /// Shared governance readiness states this component distinguishes (every
    /// component).
    pub readiness_states: Vec<M5GovernanceReadinessState>,
    /// Fitness provenance classes this component names (fitness tile only).
    pub fitness_provenance_classes: Vec<M5FitnessProvenanceClass>,
    /// Governance report scopes this component names (report row only).
    pub report_scopes: Vec<M5GovernanceReportScope>,
    /// Waiver expiry states this component distinguishes (waiver item only).
    pub waiver_expiry_states: Vec<M5WaiverExpiryState>,
    /// Release-gate decisions this component names (release-gate banner only).
    pub release_gate_decisions: Vec<M5ReleaseGateDecision>,
    /// Mitigation postures this component distinguishes (mitigation card only).
    pub mitigation_postures: Vec<M5MitigationPosture>,
    /// Ownership coverage states this component distinguishes (ownership card only).
    pub ownership_coverage_states: Vec<M5OwnershipCoverageState>,
    /// On-call coverage states this component distinguishes (on-call strip only).
    pub on_call_coverage_states: Vec<M5OnCallCoverageState>,
    /// Escalation route classes this component names (on-call strip only).
    pub escalation_route_classes: Vec<M5EscalationRouteClass>,
    /// Decision forum classes this component names (decision-right card only).
    pub decision_forum_classes: Vec<M5DecisionForumClass>,
    /// Decision-right states this component distinguishes (decision-right card only).
    pub decision_right_states: Vec<M5DecisionRightState>,
    /// Milestone gate states this component distinguishes (milestone row only).
    pub milestone_gate_states: Vec<M5MilestoneGateState>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5GovernanceAccessibilityRoute>,
    /// Governance subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5GovernanceConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5GovernanceDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never renders a waived or stale reading as a
    /// clean pass. MUST be `false`.
    pub renders_waived_or_stale_as_clean_pass: bool,
    /// Hard invariant: this component never lets an ownerless or forumless blocker
    /// read as resolved. MUST be `false`.
    pub lets_ownerless_or_forumless_blocker_read_resolved: bool,
    /// Hard invariant: this component never hides mitigation text behind internal
    /// jargon. MUST be `false`.
    pub hides_mitigation_behind_internal_jargon: bool,
    /// Hard invariant: this component never invents a private governance-status
    /// grammar. MUST be `false`.
    pub invents_private_governance_status_grammar: bool,
}

impl M5GovernanceDashboardComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5GovernanceRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5GovernanceRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.renders_waived_or_stale_as_clean_pass
            && !self.lets_ownerless_or_forumless_blocker_read_resolved
            && !self.hides_mitigation_behind_internal_jargon
            && !self.invents_private_governance_status_grammar
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceDashboardVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Governance-readiness-state tokens (the frozen acceptance-criteria vocabulary).
    pub readiness_states: Vec<String>,
    /// Fitness-provenance-class tokens.
    pub fitness_provenance_classes: Vec<String>,
    /// Governance-report-scope tokens.
    pub report_scopes: Vec<String>,
    /// Waiver-expiry-state tokens.
    pub waiver_expiry_states: Vec<String>,
    /// Release-gate-decision tokens.
    pub release_gate_decisions: Vec<String>,
    /// Mitigation-posture tokens.
    pub mitigation_postures: Vec<String>,
    /// Ownership-coverage-state tokens.
    pub ownership_coverage_states: Vec<String>,
    /// On-call-coverage-state tokens.
    pub on_call_coverage_states: Vec<String>,
    /// Escalation-route-class tokens.
    pub escalation_route_classes: Vec<String>,
    /// Decision-forum-class tokens.
    pub decision_forum_classes: Vec<String>,
    /// Decision-right-state tokens.
    pub decision_right_states: Vec<String>,
    /// Milestone-gate-state tokens.
    pub milestone_gate_states: Vec<String>,
    /// Governance-surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5GovernanceDashboardVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5GovernanceDashboardComponentFamily::ALL, |v| v.as_str()),
            readiness_states: tokens(&M5GovernanceReadinessState::ALL, |v| v.as_str()),
            fitness_provenance_classes: tokens(&M5FitnessProvenanceClass::ALL, |v| v.as_str()),
            report_scopes: tokens(&M5GovernanceReportScope::ALL, |v| v.as_str()),
            waiver_expiry_states: tokens(&M5WaiverExpiryState::ALL, |v| v.as_str()),
            release_gate_decisions: tokens(&M5ReleaseGateDecision::ALL, |v| v.as_str()),
            mitigation_postures: tokens(&M5MitigationPosture::ALL, |v| v.as_str()),
            ownership_coverage_states: tokens(&M5OwnershipCoverageState::ALL, |v| v.as_str()),
            on_call_coverage_states: tokens(&M5OnCallCoverageState::ALL, |v| v.as_str()),
            escalation_route_classes: tokens(&M5EscalationRouteClass::ALL, |v| v.as_str()),
            decision_forum_classes: tokens(&M5DecisionForumClass::ALL, |v| v.as_str()),
            decision_right_states: tokens(&M5DecisionRightState::ALL, |v| v.as_str()),
            milestone_gate_states: tokens(&M5MilestoneGateState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5GovernanceSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5DeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5GovernanceConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5GovernanceAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5GovernanceRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5GovernanceDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5GovernanceDashboardGovernanceReview {
    /// The fitness tile shows its corpus/profile provenance and evidence freshness.
    pub fitness_tile_shows_provenance_and_freshness: bool,
    /// The governance report row states its readiness with evidence.
    pub report_row_states_readiness_with_evidence: bool,
    /// The waiver-expiry item states when a waiver lapses.
    pub waiver_item_states_expiry: bool,
    /// The release-gate banner states a specific ship/no-ship reason.
    pub release_gate_banner_states_specific_reason: bool,
    /// The mitigation note card carries reusable, jargon-free mitigation language.
    pub mitigation_card_carries_reusable_language: bool,
    /// The service-ownership card states owner coverage and freshness.
    pub ownership_card_states_coverage_and_freshness: bool,
    /// The on-call strip states on-call coverage and the escalation route.
    pub on_call_strip_states_coverage_and_route: bool,
    /// The decision-right card names the authorized forum and never widens advisory
    /// to authoritative.
    pub decision_right_card_names_authorized_forum: bool,
    /// A waived or stale reading is never rendered as a clean pass.
    pub waived_or_stale_never_clean_pass: bool,
    /// No component invents a second governance-status grammar.
    pub no_component_invents_second_status_grammar: bool,
    /// Later M5 rows cannot invent parallel governance-dashboard vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceDashboardConsumerProjection {
    /// Assurance and operator surfaces consume the shared readiness vocabulary.
    pub assurance_and_operator_surfaces_consume_readiness_vocabulary: bool,
    /// Waiver and mitigation surfaces consume the waiver/mitigation vocabulary.
    pub waiver_and_mitigation_surfaces_consume_matrix: bool,
    /// Ownership and on-call surfaces consume the coverage/escalation vocabulary.
    pub ownership_and_on_call_surfaces_consume_coverage_vocabulary: bool,
    /// Release-gate and decision-right surfaces consume the gate/forum vocabulary.
    pub release_gate_and_decision_right_surfaces_consume_forum_vocabulary: bool,
    /// Support / export reads a single canonical governance-dashboard source.
    pub support_export_reads_single_source: bool,
    /// Shiproom and milestone surfaces read a single canonical source.
    pub shiproom_and_milestone_surfaces_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceDashboardProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the governance-dashboard lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceDashboardReleasePosture {
    /// Ref of the supporting governance packet for the lane.
    pub governance_packet_ref: String,
    /// Ref of the supporting assurance audit for the lane.
    pub assurance_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5GovernanceDashboardMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5GovernanceDashboardMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5GovernanceDashboardComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5GovernanceDashboardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5GovernanceDashboardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5GovernanceDashboardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5GovernanceDashboardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5GovernanceDashboardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 governance-dashboard-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceDashboardMatrixPacket {
    /// Record kind; must equal [`M5_GOVERNANCE_DASHBOARD_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5GovernanceDashboardComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5GovernanceDashboardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5GovernanceDashboardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5GovernanceDashboardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5GovernanceDashboardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5GovernanceDashboardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5GovernanceDashboardMatrixPacket {
    /// Builds an M5 governance-dashboard-component matrix packet from stable-lane
    /// input.
    pub fn new(input: M5GovernanceDashboardMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_GOVERNANCE_DASHBOARD_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 governance-dashboard-component matrix invariants.
    pub fn validate(&self) -> Vec<M5GovernanceDashboardMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_GOVERNANCE_DASHBOARD_MATRIX_RECORD_KIND {
            violations.push(M5GovernanceDashboardMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_VERSION {
            violations.push(M5GovernanceDashboardMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5GovernanceDashboardMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 governance dashboard matrix packet serializes"),
        ) {
            violations.push(M5GovernanceDashboardMatrixViolation::RawMaterialInExport);
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
            .expect("m5 governance dashboard matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,readiness_states,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.readiness_states, |v| v.as_str()),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Fitness-Dashboard-Tile, Governance-Report-Row, Waiver-Expiry-Queue-Item, Release-Gate-Banner, Mitigation-Note-Card, Service-Ownership-Card, On-Call-Strip, Decision-Right-Card, and Milestone-Dashboard-Row Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Readiness states: {}\n",
            self.vocabulary_set.readiness_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Readiness states: {}\n",
                row.readiness_states
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
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

/// Errors emitted when reading the checked-in M5 governance-dashboard matrix export.
#[derive(Debug)]
pub enum M5GovernanceDashboardMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5GovernanceDashboardMatrixViolation>),
}

impl fmt::Display for M5GovernanceDashboardMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 governance dashboard matrix export parse failed: {error}"
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
                    "m5 governance dashboard matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5GovernanceDashboardMatrixArtifactError {}

/// Validation failures emitted by [`M5GovernanceDashboardMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5GovernanceDashboardMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component declares no shared readiness states.
    ReadinessStateMissing,
    /// A fitness component declares no fitness provenance classes.
    FitnessProvenanceMissing,
    /// A report component declares no report scopes.
    ReportScopeMissing,
    /// A waiver component declares no waiver expiry states.
    WaiverExpiryStateMissing,
    /// A release-gate component declares no release-gate decisions.
    ReleaseGateDecisionMissing,
    /// A mitigation component declares no mitigation postures.
    MitigationPostureMissing,
    /// An ownership component declares no ownership coverage states.
    OwnershipCoverageStateMissing,
    /// An on-call component declares no on-call coverage states.
    OnCallCoverageStateMissing,
    /// An on-call component declares no escalation route classes.
    EscalationRouteClassMissing,
    /// A decision-right component declares no decision forum classes.
    DecisionForumClassMissing,
    /// A decision-right component declares no decision-right states.
    DecisionRightStateMissing,
    /// A milestone component declares no milestone gate states.
    MilestoneGateStateMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (waived/stale rendered as clean pass,
    /// ownerless/forumless blocker read as resolved, mitigation hidden behind
    /// jargon, or private status grammar).
    ComponentInvariantViolated,
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

impl M5GovernanceDashboardMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ReadinessStateMissing => "readiness_state_missing",
            Self::FitnessProvenanceMissing => "fitness_provenance_missing",
            Self::ReportScopeMissing => "report_scope_missing",
            Self::WaiverExpiryStateMissing => "waiver_expiry_state_missing",
            Self::ReleaseGateDecisionMissing => "release_gate_decision_missing",
            Self::MitigationPostureMissing => "mitigation_posture_missing",
            Self::OwnershipCoverageStateMissing => "ownership_coverage_state_missing",
            Self::OnCallCoverageStateMissing => "on_call_coverage_state_missing",
            Self::EscalationRouteClassMissing => "escalation_route_class_missing",
            Self::DecisionForumClassMissing => "decision_forum_class_missing",
            Self::DecisionRightStateMissing => "decision_right_state_missing",
            Self::MilestoneGateStateMissing => "milestone_gate_state_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 governance-dashboard matrix export.
pub fn current_stable_m5_governance_dashboard_component_matrix_export(
) -> Result<M5GovernanceDashboardMatrixPacket, M5GovernanceDashboardMatrixArtifactError> {
    let packet: M5GovernanceDashboardMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-governance-dashboard-proof/support_export.json"
    )))
    .map_err(M5GovernanceDashboardMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5GovernanceDashboardMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5GovernanceDashboardMatrixPacket,
    violations: &mut Vec<M5GovernanceDashboardMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_GOVERNANCE_DASHBOARD_SCHEMA_REF,
        M5_GOVERNANCE_DASHBOARD_DOC_REF,
        M5_FITNESS_DASHBOARD_TILE_SCHEMA_REF,
        M5_GOVERNANCE_REPORT_ROW_SCHEMA_REF,
        M5_WAIVER_EXPIRY_QUEUE_ITEM_SCHEMA_REF,
        M5_RELEASE_GATE_BANNER_SCHEMA_REF,
        M5_MITIGATION_NOTE_CARD_SCHEMA_REF,
        M5_SERVICE_OWNERSHIP_CARD_SCHEMA_REF,
        M5_ON_CALL_STRIP_SCHEMA_REF,
        M5_DECISION_RIGHT_CARD_SCHEMA_REF,
        M5_MILESTONE_DASHBOARD_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5GovernanceDashboardMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5GovernanceDashboardMatrixPacket,
    violations: &mut Vec<M5GovernanceDashboardMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5GovernanceDashboardMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5GovernanceDashboardMatrixPacket,
    violations: &mut Vec<M5GovernanceDashboardMatrixViolation>,
) {
    let present: BTreeSet<M5GovernanceDashboardComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5GovernanceDashboardComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5GovernanceDashboardMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5GovernanceDashboardMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5GovernanceDashboardMatrixViolation::MandatoryLabelMissing);
        }
        if row.readiness_states.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::ReadinessStateMissing);
        }
        if family.is_fitness_tile() && row.fitness_provenance_classes.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::FitnessProvenanceMissing);
        }
        if family.is_report_row() && row.report_scopes.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::ReportScopeMissing);
        }
        if family.is_waiver_item() && row.waiver_expiry_states.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::WaiverExpiryStateMissing);
        }
        if family.is_release_gate() && row.release_gate_decisions.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::ReleaseGateDecisionMissing);
        }
        if family.is_mitigation_card() && row.mitigation_postures.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::MitigationPostureMissing);
        }
        if family.is_ownership_card() && row.ownership_coverage_states.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::OwnershipCoverageStateMissing);
        }
        if family.is_on_call() && row.on_call_coverage_states.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::OnCallCoverageStateMissing);
        }
        if family.is_on_call() && row.escalation_route_classes.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::EscalationRouteClassMissing);
        }
        if family.is_decision_right() && row.decision_forum_classes.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::DecisionForumClassMissing);
        }
        if family.is_decision_right() && row.decision_right_states.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::DecisionRightStateMissing);
        }
        if family.is_milestone_row() && row.milestone_gate_states.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::MilestoneGateStateMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5GovernanceDashboardMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5GovernanceDashboardMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5GovernanceDashboardMatrixPacket,
    violations: &mut Vec<M5GovernanceDashboardMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.fitness_tile_shows_provenance_and_freshness,
        review.report_row_states_readiness_with_evidence,
        review.waiver_item_states_expiry,
        review.release_gate_banner_states_specific_reason,
        review.mitigation_card_carries_reusable_language,
        review.ownership_card_states_coverage_and_freshness,
        review.on_call_strip_states_coverage_and_route,
        review.decision_right_card_names_authorized_forum,
        review.waived_or_stale_never_clean_pass,
        review.no_component_invents_second_status_grammar,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5GovernanceDashboardMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5GovernanceDashboardMatrixPacket,
    violations: &mut Vec<M5GovernanceDashboardMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.assurance_and_operator_surfaces_consume_readiness_vocabulary,
        projection.waiver_and_mitigation_surfaces_consume_matrix,
        projection.ownership_and_on_call_surfaces_consume_coverage_vocabulary,
        projection.release_gate_and_decision_right_surfaces_consume_forum_vocabulary,
        projection.support_export_reads_single_source,
        projection.shiproom_and_milestone_surfaces_read_single_source,
    ] {
        if !ok {
            violations.push(M5GovernanceDashboardMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5GovernanceDashboardMatrixPacket,
    violations: &mut Vec<M5GovernanceDashboardMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5GovernanceDashboardMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5GovernanceDashboardMatrixPacket,
    violations: &mut Vec<M5GovernanceDashboardMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.governance_packet_ref.trim().is_empty()
        || posture.assurance_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5GovernanceDashboardMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
