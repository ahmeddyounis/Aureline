//! Two reusable M5 governance-dashboard primitives implemented as one controls packet:
//! the **service-ownership card** (service or surface identity, owning role/team, support
//! class, escalation path, owner freshness, and backup-coverage state) and the **on-call
//! strip** (role alias, current availability state, primary/secondary distinction,
//! escalation route, and export-safe handoff continuity), projected the same way across
//! every claimed M5 operator and release surface.
//!
//! Aureline's frozen governance-dashboard component matrix
//! ([`crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix`])
//! names the service-ownership card and the on-call strip as two governed component
//! families and freezes their shared readiness-state vocabulary, the ownership-coverage
//! states, the on-call-coverage states, and the escalation-route classes. This module
//! *implements* those two contracts as one reusable controls packet so a support agent,
//! an operator, or a release reviewer can tell — from the card and the strip alone — who
//! actually owns a service or protected surface, what its support class is, whether a
//! backup exists, how fresh the owner record is, who is on call right now, and how a page
//! escalates, before that truth silently inherits the last interacting team as false
//! ownership.
//!
//! The packet has two resolver halves:
//!
//! 1. [`resolve_service_ownership_card`] takes one service's identity, owning role, support
//!    class, ownership-coverage state, owner source, backup owner, escalation route, and
//!    owner freshness, and produces one [`M5ResolvedServiceOwnershipCard`] carrying the
//!    *derived* readiness state drawn from the frozen [`M5GovernanceReadinessState`]
//!    vocabulary. An ownerless or backup-missing protected surface never resolves to
//!    `passing`, and an owner that is only an inference from the last interacting team is
//!    never rendered as a resolved owner: it reads `owner_unresolved`.
//! 2. [`resolve_on_call_strip`] takes one strip's role alias, on-call-coverage state,
//!    current availability, primary/secondary role tier, escalation route, handoff
//!    continuity, and roster freshness, and produces one [`M5ResolvedOnCallStrip`]
//!    carrying the derived readiness state, an always-explicit escalation route, and the
//!    export-safe handoff continuity a reviewer needs. An on-call gap or a missing
//!    escalation path never reads as covered.
//!
//! A parity matrix — [`M5ServiceOwnershipOnCallControlsPacket`] — binds one row per
//! claimed M5 governance consumer (the operator board, the release center, the
//! service-health surface, the support export, and the CLI inspect) to the shared
//! card and strip anatomy, the same readiness states, ownership-coverage states,
//! on-call-coverage states, escalation-route classes, support classes, freshness
//! readings, degrade reasons, next actions, and export fields, plus worked resolution
//! cases that must reproduce the resolver output exactly, so the ownership/escalation
//! vocabulary stays identical — one role-based model — across support, operator, and
//! release surfaces rather than cloned prose.
//!
//! The frozen readiness-state vocabulary ([`M5GovernanceReadinessState`]), the
//! ownership-coverage state ([`M5OwnershipCoverageState`]), the on-call-coverage state
//! ([`M5OnCallCoverageState`]), the escalation-route class ([`M5EscalationRouteClass`]),
//! the deployment line ([`M5DeploymentLine`]), the governance surface family
//! ([`M5GovernanceSurfaceFamily`]), the governance consumer surface
//! ([`M5GovernanceConsumerSurface`]), the accessibility route
//! ([`M5GovernanceAccessibilityRoute`]), the required label
//! ([`M5GovernanceRequiredLabel`]), the qualification class
//! ([`M5GovernanceQualificationClass`]), and the downgrade trigger
//! ([`M5GovernanceDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the card and
//! the strip themselves: their governance consumer families, their anatomy parts, the
//! support classes, the owner-source classes, the owner-freshness readings, the on-call
//! availability states, the on-call role tiers, the degrade reasons, the next actions, the
//! card and strip actions, and the export fields. No M5 governance surface invents a
//! second ownership or escalation grammar.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, and user text bodies stay outside
//! the support boundary; every service id, surface id, owner alias, backup alias, and
//! handoff representation is carried only as an opaque, export-safe representation, and an
//! owner or on-call role alias is a role alias, never a personal contact detail.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_service_ownership_on_call_controls_operator_board_preview_narrowed,
    seeded_m5_service_ownership_on_call_controls_packet,
    seeded_m5_service_ownership_on_call_controls_service_health_beta_narrowed,
    M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_PACKET_ID,
};

// The readiness state vocabulary, the ownership-coverage states, the on-call-coverage
// states, the escalation-route classes, the deployment lines, the surface families, the
// consumer surfaces, the accessibility routes, the required labels, the qualification
// classes, and the downgrade triggers are frozen once, in the governance-dashboard
// component matrix. This controls packet reuses them verbatim so it never invents a
// parallel vocabulary.
pub use crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix::{
    M5DeploymentLine, M5EscalationRouteClass, M5GovernanceAccessibilityRoute,
    M5GovernanceConsumerSurface, M5GovernanceDowngradeTrigger, M5GovernanceQualificationClass,
    M5GovernanceReadinessState, M5GovernanceRequiredLabel, M5GovernanceSurfaceFamily,
    M5OnCallCoverageState, M5OwnershipCoverageState,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ServiceOwnershipOnCallControlsPacket`].
pub const M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_RECORD_KIND: &str =
    "implement_m5_service_ownership_cards_and_on_call_strips_across_claimed_m5_operator_and_release_surfaces";

/// Schema version for M5 service-ownership / on-call controls records.
pub const M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-service-ownership-on-call-controls.schema.json";

/// Repo-relative path of the controls contract doc.
pub const M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_DOC_REF: &str =
    "docs/help/m5_service_ownership_card_and_on_call_strip_controls.md";

/// Repo-relative path of the frozen governance-dashboard component matrix schema this
/// controls packet narrows from.
pub const M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-governance-dashboard-component-matrix.schema.json";

/// Repo-relative path of the frozen governance-dashboard component matrix doc.
pub const M5_GOVERNANCE_DASHBOARD_MATRIX_DOC_REF: &str =
    "docs/help/m5_governance_dashboard_components_contract.md";

/// Repo-relative path of the per-component service-ownership-card contract schema.
pub const M5_SERVICE_OWNERSHIP_CARD_CONTRACT_REF: &str =
    "schemas/ui/m5-service-ownership-card.schema.json";

/// Repo-relative path of the per-component on-call-strip contract schema.
pub const M5_ON_CALL_STRIP_CONTRACT_REF: &str = "schemas/ui/m5-on-call-strip.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-service-ownership-on-call-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-service-ownership-on-call-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-service-ownership-on-call-controls-proof/summary.md";

// ---------------------------------------------------------------------------
// Minted vocabulary
// ---------------------------------------------------------------------------

/// One claimed M5 governance consumer that renders the shared service-ownership card and
/// on-call strip. The operator, release, and support surfaces are all named so they can be
/// proven to reuse one role-based ownership/escalation model rather than cloning prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OwnershipConsumerSurface {
    /// The operator overview board.
    OperatorBoard,
    /// The release-center surface.
    ReleaseCenter,
    /// The service-health surface.
    ServiceHealth,
    /// The support / export packet.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
}

impl M5OwnershipConsumerSurface {
    /// Every claimed governance consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OperatorBoard,
        Self::ReleaseCenter,
        Self::ServiceHealth,
        Self::SupportExport,
        Self::CliInspect,
    ];

    /// The three surfaces that must share one role-based ownership/escalation model.
    pub const SHARED_MODEL_REQUIRED: [Self; 3] = [
        Self::OperatorBoard,
        Self::ReleaseCenter,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OperatorBoard => "operator_board",
            Self::ReleaseCenter => "release_center",
            Self::ServiceHealth => "service_health",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OperatorBoard => "Operator Board",
            Self::ReleaseCenter => "Release Center",
            Self::ServiceHealth => "Service Health",
            Self::SupportExport => "Support / Export",
            Self::CliInspect => "CLI Inspect",
        }
    }
}

/// One anatomy part the shared card / strip surfaces. The parts in
/// [`M5OwnershipAnatomyPart::MANDATORY`] are required on every row so a reviewer can orient
/// before trusting an ownership or on-call claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OwnershipAnatomyPart {
    /// The service or surface identity (card identity).
    ServiceIdentity,
    /// The owning role / team cue.
    OwningRole,
    /// The support-class cue.
    SupportClass,
    /// The escalation-path cue.
    EscalationPath,
    /// The owner-freshness cue.
    OwnerFreshness,
    /// The backup-coverage cue.
    BackupCoverage,
    /// The open-ownership-roster action.
    OpenRosterAction,
    /// The on-call role alias (strip identity).
    RoleAlias,
    /// The current on-call availability cue.
    AvailabilityState,
    /// The primary/secondary distinction cue.
    PrimarySecondary,
    /// The escalation-route cue.
    EscalationRoute,
    /// The export-safe handoff-continuity cue.
    HandoffContinuity,
    /// The page-escalation action.
    PageAction,
}

impl M5OwnershipAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::ServiceIdentity,
        Self::OwningRole,
        Self::SupportClass,
        Self::EscalationPath,
        Self::OwnerFreshness,
        Self::BackupCoverage,
        Self::OpenRosterAction,
        Self::RoleAlias,
        Self::AvailabilityState,
        Self::PrimarySecondary,
        Self::EscalationRoute,
        Self::HandoffContinuity,
        Self::PageAction,
    ];

    /// The anatomy parts every row must render before ownership or on-call is trusted.
    pub const MANDATORY: [Self; 4] = [
        Self::ServiceIdentity,
        Self::OwningRole,
        Self::BackupCoverage,
        Self::AvailabilityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServiceIdentity => "service_identity",
            Self::OwningRole => "owning_role",
            Self::SupportClass => "support_class",
            Self::EscalationPath => "escalation_path",
            Self::OwnerFreshness => "owner_freshness",
            Self::BackupCoverage => "backup_coverage",
            Self::OpenRosterAction => "open_roster_action",
            Self::RoleAlias => "role_alias",
            Self::AvailabilityState => "availability_state",
            Self::PrimarySecondary => "primary_secondary",
            Self::EscalationRoute => "escalation_route",
            Self::HandoffContinuity => "handoff_continuity",
            Self::PageAction => "page_action",
        }
    }
}

/// The support class a service or protected surface carries, so a service-ownership card
/// never leaves its supportability posture implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ServiceSupportClass {
    /// Tier-1, critical, fully supported.
    Tier1Critical,
    /// Tier-2, standard support.
    Tier2Standard,
    /// Tier-3, best-effort support.
    Tier3BestEffort,
    /// Community-supported only.
    CommunitySupported,
    /// Explicitly unsupported.
    Unsupported,
}

impl M5ServiceSupportClass {
    /// Every support class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Tier1Critical,
        Self::Tier2Standard,
        Self::Tier3BestEffort,
        Self::CommunitySupported,
        Self::Unsupported,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tier1Critical => "tier1_critical",
            Self::Tier2Standard => "tier2_standard",
            Self::Tier3BestEffort => "tier3_best_effort",
            Self::CommunitySupported => "community_supported",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Where a service-ownership claim came from, so an owner that is only an inference from
/// the last interacting team is never rendered as a resolved owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OwnerSource {
    /// The owner is drawn from the authoritative ownership roster.
    AuthoritativeRoster,
    /// The owner is a declared owning role on this surface.
    DeclaredOwnerRole,
    /// The owner is only an inference from the last interacting team.
    LastInteractingTeamInference,
    /// No owner is recorded for this service.
    OwnerUnrecorded,
}

impl M5OwnerSource {
    /// Every owner source, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AuthoritativeRoster,
        Self::DeclaredOwnerRole,
        Self::LastInteractingTeamInference,
        Self::OwnerUnrecorded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeRoster => "authoritative_roster",
            Self::DeclaredOwnerRole => "declared_owner_role",
            Self::LastInteractingTeamInference => "last_interacting_team_inference",
            Self::OwnerUnrecorded => "owner_unrecorded",
        }
    }

    /// `true` only when the owner is drawn from an authoritative or explicitly declared
    /// source. An inference from the last interacting team, or an unrecorded owner, is
    /// never an authoritative owner.
    pub const fn is_authoritative(self) -> bool {
        matches!(self, Self::AuthoritativeRoster | Self::DeclaredOwnerRole)
    }
}

/// The owner-record / roster freshness reading shared by both resolvers, so a card or a
/// strip never shows a stale or missing owner record as fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OwnerFreshness {
    /// The owner record is fresh within its freshness window.
    OwnerFresh,
    /// The owner record is aging but still within tolerance.
    OwnerAging,
    /// The owner record is stale relative to the roster.
    OwnerStale,
    /// The owner record is missing.
    OwnerMissing,
    /// The owner-freshness reading is unknown / not yet evaluated.
    OwnerUnknown,
}

impl M5OwnerFreshness {
    /// Every owner-freshness reading, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OwnerFresh,
        Self::OwnerAging,
        Self::OwnerStale,
        Self::OwnerMissing,
        Self::OwnerUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerFresh => "owner_fresh",
            Self::OwnerAging => "owner_aging",
            Self::OwnerStale => "owner_stale",
            Self::OwnerMissing => "owner_missing",
            Self::OwnerUnknown => "owner_unknown",
        }
    }
}

/// The current availability of the named on-call responder, so an on-call strip never
/// leaves the live availability implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OnCallAvailabilityState {
    /// The responder is available now.
    AvailableNow,
    /// The responder is off-shift.
    OffShift,
    /// A handoff is pending between responders.
    HandoffPending,
    /// There is no current coverage.
    NoCoverage,
    /// The availability is unknown.
    AvailabilityUnknown,
}

impl M5OnCallAvailabilityState {
    /// Every availability state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AvailableNow,
        Self::OffShift,
        Self::HandoffPending,
        Self::NoCoverage,
        Self::AvailabilityUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AvailableNow => "available_now",
            Self::OffShift => "off_shift",
            Self::HandoffPending => "handoff_pending",
            Self::NoCoverage => "no_coverage",
            Self::AvailabilityUnknown => "availability_unknown",
        }
    }
}

/// The primary/secondary distinction of the named on-call responder, so an on-call strip
/// never masks who is actually primary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OnCallRoleTier {
    /// The primary on-call.
    PrimaryOnCall,
    /// The secondary on-call.
    SecondaryOnCall,
    /// The manager escalation contact.
    ManagerEscalation,
    /// The incident commander.
    IncidentCommander,
    /// No named responder.
    NoNamedResponder,
}

impl M5OnCallRoleTier {
    /// Every role tier, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PrimaryOnCall,
        Self::SecondaryOnCall,
        Self::ManagerEscalation,
        Self::IncidentCommander,
        Self::NoNamedResponder,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryOnCall => "primary_on_call",
            Self::SecondaryOnCall => "secondary_on_call",
            Self::ManagerEscalation => "manager_escalation",
            Self::IncidentCommander => "incident_commander",
            Self::NoNamedResponder => "no_named_responder",
        }
    }
}

/// The next action named on a degraded card or strip, so a non-passing reading is
/// actionable rather than a dead end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OwnershipNextAction {
    /// Open the ownership roster.
    OpenRoster,
    /// Assign a resolved owner.
    AssignOwner,
    /// Add a backup owner.
    AddBackupOwner,
    /// Refresh the stale or missing owner record.
    RefreshOwnerRecord,
    /// Page the on-call responder.
    PageOnCall,
    /// Define the missing escalation path.
    DefineEscalationPath,
}

impl M5OwnershipNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenRoster,
        Self::AssignOwner,
        Self::AddBackupOwner,
        Self::RefreshOwnerRecord,
        Self::PageOnCall,
        Self::DefineEscalationPath,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRoster => "open_roster",
            Self::AssignOwner => "assign_owner",
            Self::AddBackupOwner => "add_backup_owner",
            Self::RefreshOwnerRecord => "refresh_owner_record",
            Self::PageOnCall => "page_on_call",
            Self::DefineEscalationPath => "define_escalation_path",
        }
    }
}

/// The exact reason a service-ownership card degraded below a clean, fully-covered pass,
/// so an ownerless or backup-missing surface never reads as covered and an owner inferred
/// from the last interacting team is never presented as truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OwnershipDegradeReason {
    /// The card has not been evaluated on this build.
    NotYetEvaluated,
    /// The owner is only an inference from the last interacting team, or is unrecorded.
    InheritedOrUnresolvedOwner,
    /// The service has no resolved owner in the roster.
    OwnerUnresolvedForService,
    /// The owner record is stale relative to the roster.
    OwnerRecordStale,
    /// The owner record is missing.
    OwnerRecordMissing,
    /// Ownership is hidden by policy on this surface.
    OwnershipPolicyHidden,
    /// The service has a primary owner but no named backup.
    BackupMissingForService,
    /// The owner record is aging and should be refreshed.
    OwnerRecordAging,
}

impl M5OwnershipDegradeReason {
    /// Every ownership degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotYetEvaluated,
        Self::InheritedOrUnresolvedOwner,
        Self::OwnerUnresolvedForService,
        Self::OwnerRecordStale,
        Self::OwnerRecordMissing,
        Self::OwnershipPolicyHidden,
        Self::BackupMissingForService,
        Self::OwnerRecordAging,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "not_yet_evaluated",
            Self::InheritedOrUnresolvedOwner => "inherited_or_unresolved_owner",
            Self::OwnerUnresolvedForService => "owner_unresolved_for_service",
            Self::OwnerRecordStale => "owner_record_stale",
            Self::OwnerRecordMissing => "owner_record_missing",
            Self::OwnershipPolicyHidden => "ownership_policy_hidden",
            Self::BackupMissingForService => "backup_missing_for_service",
            Self::OwnerRecordAging => "owner_record_aging",
        }
    }

    /// The frozen readiness state this degrade reason resolves to.
    pub const fn readiness_state(self) -> M5GovernanceReadinessState {
        match self {
            Self::NotYetEvaluated => M5GovernanceReadinessState::NotEvaluated,
            Self::InheritedOrUnresolvedOwner => M5GovernanceReadinessState::OwnerUnresolved,
            Self::OwnerUnresolvedForService => M5GovernanceReadinessState::OwnerUnresolved,
            Self::OwnerRecordStale => M5GovernanceReadinessState::EvidenceStale,
            Self::OwnerRecordMissing => M5GovernanceReadinessState::Blocked,
            Self::OwnershipPolicyHidden => M5GovernanceReadinessState::Warning,
            Self::BackupMissingForService => M5GovernanceReadinessState::Warning,
            Self::OwnerRecordAging => M5GovernanceReadinessState::Warning,
        }
    }

    /// The next action a reviewer should take to clear this degrade.
    pub const fn next_action(self) -> M5OwnershipNextAction {
        match self {
            Self::NotYetEvaluated => M5OwnershipNextAction::OpenRoster,
            Self::InheritedOrUnresolvedOwner => M5OwnershipNextAction::AssignOwner,
            Self::OwnerUnresolvedForService => M5OwnershipNextAction::AssignOwner,
            Self::OwnerRecordStale => M5OwnershipNextAction::RefreshOwnerRecord,
            Self::OwnerRecordMissing => M5OwnershipNextAction::RefreshOwnerRecord,
            Self::OwnershipPolicyHidden => M5OwnershipNextAction::OpenRoster,
            Self::BackupMissingForService => M5OwnershipNextAction::AddBackupOwner,
            Self::OwnerRecordAging => M5OwnershipNextAction::RefreshOwnerRecord,
        }
    }

    /// Review-safe reason phrase for the card's degrade note.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "the ownership card has not been evaluated on this build",
            Self::InheritedOrUnresolvedOwner => {
                "the owner is only inferred from the last interacting team and is not a resolved owner"
            }
            Self::OwnerUnresolvedForService => "the service has no resolved owner in the roster",
            Self::OwnerRecordStale => "the owner record is stale relative to the roster",
            Self::OwnerRecordMissing => "the owner record is missing",
            Self::OwnershipPolicyHidden => "ownership is hidden by policy on this surface",
            Self::BackupMissingForService => "the service has a primary owner but no named backup",
            Self::OwnerRecordAging => "the owner record is aging and should be refreshed",
        }
    }
}

/// The exact reason an on-call strip degraded below a clean, fully-covered pass, so an
/// on-call gap or a missing escalation path never reads as covered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OnCallDegradeReason {
    /// The strip has not been evaluated on this build.
    NotYetEvaluated,
    /// No escalation path exists for this strip.
    EscalationPathMissing,
    /// There is an open on-call gap (gap, no coverage, or a missing roster).
    OnCallGapOpen,
    /// There is no named responder.
    NoNamedResponder,
    /// The on-call roster is stale relative to the current build.
    RosterStale,
    /// The on-call posture is unknown / not yet evaluated.
    OnCallPostureUnknown,
    /// Only an escalation path is available, or the primary is off-shift.
    EscalationOnlyCoverage,
    /// A handoff is pending between responders.
    HandoffPending,
}

impl M5OnCallDegradeReason {
    /// Every on-call degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotYetEvaluated,
        Self::EscalationPathMissing,
        Self::OnCallGapOpen,
        Self::NoNamedResponder,
        Self::RosterStale,
        Self::OnCallPostureUnknown,
        Self::EscalationOnlyCoverage,
        Self::HandoffPending,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "not_yet_evaluated",
            Self::EscalationPathMissing => "escalation_path_missing",
            Self::OnCallGapOpen => "on_call_gap_open",
            Self::NoNamedResponder => "no_named_responder",
            Self::RosterStale => "roster_stale",
            Self::OnCallPostureUnknown => "on_call_posture_unknown",
            Self::EscalationOnlyCoverage => "escalation_only_coverage",
            Self::HandoffPending => "handoff_pending",
        }
    }

    /// The frozen readiness state this degrade reason resolves to.
    pub const fn readiness_state(self) -> M5GovernanceReadinessState {
        match self {
            Self::NotYetEvaluated => M5GovernanceReadinessState::NotEvaluated,
            Self::EscalationPathMissing => M5GovernanceReadinessState::Blocked,
            Self::OnCallGapOpen => M5GovernanceReadinessState::Blocked,
            Self::NoNamedResponder => M5GovernanceReadinessState::OwnerUnresolved,
            Self::RosterStale => M5GovernanceReadinessState::EvidenceStale,
            Self::OnCallPostureUnknown => M5GovernanceReadinessState::NotEvaluated,
            Self::EscalationOnlyCoverage => M5GovernanceReadinessState::Warning,
            Self::HandoffPending => M5GovernanceReadinessState::Warning,
        }
    }

    /// The next action a reviewer should take to clear this degrade.
    pub const fn next_action(self) -> M5OwnershipNextAction {
        match self {
            Self::NotYetEvaluated => M5OwnershipNextAction::OpenRoster,
            Self::EscalationPathMissing => M5OwnershipNextAction::DefineEscalationPath,
            Self::OnCallGapOpen => M5OwnershipNextAction::PageOnCall,
            Self::NoNamedResponder => M5OwnershipNextAction::AssignOwner,
            Self::RosterStale => M5OwnershipNextAction::RefreshOwnerRecord,
            Self::OnCallPostureUnknown => M5OwnershipNextAction::OpenRoster,
            Self::EscalationOnlyCoverage => M5OwnershipNextAction::PageOnCall,
            Self::HandoffPending => M5OwnershipNextAction::OpenRoster,
        }
    }

    /// Review-safe reason phrase for the strip's degrade note.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "the on-call strip has not been evaluated on this build",
            Self::EscalationPathMissing => "no escalation path exists for this strip",
            Self::OnCallGapOpen => "there is an open on-call gap with no current coverage",
            Self::NoNamedResponder => "there is no named on-call responder",
            Self::RosterStale => "the on-call roster is stale relative to this build",
            Self::OnCallPostureUnknown => "the on-call posture has not been evaluated",
            Self::EscalationOnlyCoverage => {
                "only an escalation path is available or the primary is off-shift"
            }
            Self::HandoffPending => "a handoff is pending between responders",
        }
    }
}

/// An action a service-ownership card offers. The actions in
/// [`M5OwnershipCardAction::MANDATORY`] are required on every row so a reviewer can always
/// open the ownership roster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OwnershipCardAction {
    /// Open the ownership roster.
    OpenOwnershipRoster,
    /// Compare the ownership history.
    CompareOwnershipHistory,
    /// Export the ownership ledger.
    ExportOwnershipLedger,
    /// Escalate the ownership gap.
    EscalateOwnership,
}

impl M5OwnershipCardAction {
    /// Every card action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpenOwnershipRoster,
        Self::CompareOwnershipHistory,
        Self::ExportOwnershipLedger,
        Self::EscalateOwnership,
    ];

    /// The card actions every row must offer.
    pub const MANDATORY: [Self; 1] = [Self::OpenOwnershipRoster];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenOwnershipRoster => "open_ownership_roster",
            Self::CompareOwnershipHistory => "compare_ownership_history",
            Self::ExportOwnershipLedger => "export_ownership_ledger",
            Self::EscalateOwnership => "escalate_ownership",
        }
    }
}

/// An action an on-call strip offers. The actions in [`M5OnCallStripAction::MANDATORY`]
/// are required on every row so a reviewer can always open the on-call schedule and page
/// the escalation path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OnCallStripAction {
    /// Open the on-call schedule.
    OpenOnCallSchedule,
    /// Page the stated escalation path.
    PageEscalationPath,
    /// Compare the on-call history.
    CompareOnCallHistory,
    /// Export the on-call ledger.
    ExportOnCallLedger,
}

impl M5OnCallStripAction {
    /// Every strip action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpenOnCallSchedule,
        Self::PageEscalationPath,
        Self::CompareOnCallHistory,
        Self::ExportOnCallLedger,
    ];

    /// The strip actions every row must offer.
    pub const MANDATORY: [Self; 2] = [Self::OpenOnCallSchedule, Self::PageEscalationPath];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenOnCallSchedule => "open_on_call_schedule",
            Self::PageEscalationPath => "page_escalation_path",
            Self::CompareOnCallHistory => "compare_on_call_history",
            Self::ExportOnCallLedger => "export_on_call_ledger",
        }
    }
}

/// A field the support / export packet carries so card and strip truth is reconstructable
/// from the shared model. The fields in [`M5OwnershipExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OwnershipExportField {
    /// The opaque service id.
    ServiceId,
    /// The owning role alias.
    OwningRole,
    /// The support class.
    SupportClass,
    /// The ownership-coverage state.
    OwnershipCoverage,
    /// The owner-freshness reading.
    OwnerFreshness,
    /// The escalation-route class.
    EscalationRoute,
    /// The on-call-coverage state.
    OnCallCoverage,
    /// The current availability state.
    AvailabilityState,
    /// The on-call role tier.
    RoleTier,
    /// The derived readiness state.
    ReadinessState,
    /// The backup owner alias.
    BackupOwner,
}

impl M5OwnershipExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ServiceId,
        Self::OwningRole,
        Self::SupportClass,
        Self::OwnershipCoverage,
        Self::OwnerFreshness,
        Self::EscalationRoute,
        Self::OnCallCoverage,
        Self::AvailabilityState,
        Self::RoleTier,
        Self::ReadinessState,
        Self::BackupOwner,
    ];

    /// The export fields every controls export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ServiceId,
        Self::OwningRole,
        Self::OwnershipCoverage,
        Self::OnCallCoverage,
        Self::ReadinessState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServiceId => "service_id",
            Self::OwningRole => "owning_role",
            Self::SupportClass => "support_class",
            Self::OwnershipCoverage => "ownership_coverage",
            Self::OwnerFreshness => "owner_freshness",
            Self::EscalationRoute => "escalation_route",
            Self::OnCallCoverage => "on_call_coverage",
            Self::AvailabilityState => "availability_state",
            Self::RoleTier => "role_tier",
            Self::ReadinessState => "readiness_state",
            Self::BackupOwner => "backup_owner",
        }
    }
}

// ---------------------------------------------------------------------------
// Service-ownership-card resolver
// ---------------------------------------------------------------------------

/// The full input to the service-ownership-card resolver for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ServiceOwnershipResolutionInput {
    /// The opaque, export-safe service id.
    pub service_id_repr: String,
    /// The opaque, export-safe service or surface identity.
    pub surface_identity_repr: String,
    /// The opaque owning role alias (never a personal contact detail).
    pub owning_role_alias: String,
    /// The support class of the service.
    pub support_class: M5ServiceSupportClass,
    /// The ownership-coverage state.
    pub coverage_state: M5OwnershipCoverageState,
    /// Where the ownership claim came from.
    pub owner_source: M5OwnerSource,
    /// The opaque backup owner alias (may be empty when no backup exists).
    pub backup_owner_alias: String,
    /// The escalation route for this service.
    pub escalation_route: M5EscalationRouteClass,
    /// The owner-record freshness reading.
    pub owner_freshness: M5OwnerFreshness,
}

/// The resolved service-ownership-card truth for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedServiceOwnershipCard {
    /// The opaque service id.
    pub service_id_repr: String,
    /// The opaque service or surface identity.
    pub surface_identity_repr: String,
    /// The opaque owning role alias.
    pub owning_role_alias: String,
    /// The support class.
    pub support_class: M5ServiceSupportClass,
    /// The ownership-coverage state.
    pub coverage_state: M5OwnershipCoverageState,
    /// Where the ownership claim came from.
    pub owner_source: M5OwnerSource,
    /// `true` when the owner is authoritative and coverage is not unresolved.
    pub owner_resolved: bool,
    /// The opaque backup owner alias.
    pub backup_owner_alias: String,
    /// `true` when a named backup owner is present.
    pub backup_present: bool,
    /// The escalation route.
    pub escalation_route: M5EscalationRouteClass,
    /// The owner-record freshness reading.
    pub owner_freshness: M5OwnerFreshness,
    /// The derived readiness state drawn from the frozen vocabulary.
    pub readiness_state: M5GovernanceReadinessState,
    /// `true` only when the derived readiness is a clean, fully-covered pass.
    pub is_clean_pass: bool,
    /// `true` always: the ownership-coverage state stays visible wherever the card is
    /// summarized.
    pub coverage_visible: bool,
    /// The card actions this row always offers (always includes open-roster).
    pub card_actions: Vec<M5OwnershipCardAction>,
    /// The degrade reason, present when the card is not a clean pass.
    pub degrade_reason: Option<M5OwnershipDegradeReason>,
    /// The next action, present when the card is degraded.
    pub next_action: Option<M5OwnershipNextAction>,
    /// A self-contained degrade note naming the reason and next action, present when the
    /// card is degraded.
    pub degrade_note: Option<String>,
}

/// Errors returned by [`resolve_service_ownership_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ServiceOwnershipResolutionError {
    /// The service id was empty.
    EmptyServiceId,
    /// The owning role alias was empty.
    EmptyOwningRole,
    /// An owner or backup alias carried a personal contact detail (an `@`), not a role
    /// alias.
    PersonContactDetailInAlias,
    /// A service id, surface id, owner alias, backup alias carried forbidden material.
    ForbiddenOwnershipMaterial,
}

impl M5ServiceOwnershipResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyServiceId => "empty_service_id",
            Self::EmptyOwningRole => "empty_owning_role",
            Self::PersonContactDetailInAlias => "person_contact_detail_in_alias",
            Self::ForbiddenOwnershipMaterial => "forbidden_ownership_material",
        }
    }
}

impl fmt::Display for M5ServiceOwnershipResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "service-ownership-card resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ServiceOwnershipResolutionError {}

/// Resolves one service-ownership card from its declared state.
///
/// The derived readiness state is computed in a fixed degrade-first order: an unknown
/// freshness reading is `not_evaluated`; an owner that is only an inference from the last
/// interacting team, or an unrecorded owner, is `owner_unresolved` (never presented as a
/// resolved owner); an unresolved coverage state is `owner_unresolved`; a stale owner
/// record is `evidence_stale`; a missing owner record blocks; policy-hidden ownership and
/// a missing backup are `warning`; and an aging owner record is `warning`. Only a service
/// with an authoritative owner, a named backup (`owned_with_backup`), and a fresh owner
/// record is a clean pass. An ownerless or backup-missing protected surface therefore
/// never reads as covered.
pub fn resolve_service_ownership_card(
    input: &M5ServiceOwnershipResolutionInput,
) -> Result<M5ResolvedServiceOwnershipCard, M5ServiceOwnershipResolutionError> {
    if input.service_id_repr.trim().is_empty() {
        return Err(M5ServiceOwnershipResolutionError::EmptyServiceId);
    }
    if input.owning_role_alias.trim().is_empty() {
        return Err(M5ServiceOwnershipResolutionError::EmptyOwningRole);
    }
    if input.owning_role_alias.contains('@') || input.backup_owner_alias.contains('@') {
        return Err(M5ServiceOwnershipResolutionError::PersonContactDetailInAlias);
    }
    if value_repr_is_forbidden(&input.service_id_repr)
        || value_repr_is_forbidden(&input.surface_identity_repr)
        || value_repr_is_forbidden(&input.owning_role_alias)
        || value_repr_is_forbidden(&input.backup_owner_alias)
    {
        return Err(M5ServiceOwnershipResolutionError::ForbiddenOwnershipMaterial);
    }

    let backup_present = !input.backup_owner_alias.trim().is_empty();
    let owner_resolved = input.owner_source.is_authoritative()
        && !matches!(
            input.coverage_state,
            M5OwnershipCoverageState::OwnerUnresolved
        );
    let degrade_reason = derive_ownership_degrade(
        input.coverage_state,
        input.owner_source,
        input.owner_freshness,
    );
    let readiness_state = match degrade_reason {
        Some(reason) => reason.readiness_state(),
        None => M5GovernanceReadinessState::Passing,
    };
    let next_action = degrade_reason.map(M5OwnershipDegradeReason::next_action);
    let degrade_note = degrade_reason.map(|reason| {
        format!(
            "Service-ownership card degraded: {} — state `{}`; next: {}",
            reason.phrase(),
            readiness_state.as_str(),
            reason.next_action().as_str()
        )
    });

    Ok(M5ResolvedServiceOwnershipCard {
        service_id_repr: input.service_id_repr.clone(),
        surface_identity_repr: input.surface_identity_repr.clone(),
        owning_role_alias: input.owning_role_alias.clone(),
        support_class: input.support_class,
        coverage_state: input.coverage_state,
        owner_source: input.owner_source,
        owner_resolved,
        backup_owner_alias: input.backup_owner_alias.clone(),
        backup_present,
        escalation_route: input.escalation_route,
        owner_freshness: input.owner_freshness,
        readiness_state,
        is_clean_pass: readiness_state.is_clean_pass(),
        coverage_visible: true,
        card_actions: vec![
            M5OwnershipCardAction::OpenOwnershipRoster,
            M5OwnershipCardAction::CompareOwnershipHistory,
            M5OwnershipCardAction::ExportOwnershipLedger,
        ],
        degrade_reason,
        next_action,
        degrade_note,
    })
}

/// The fixed degrade-first ownership ladder. Returns `None` for a clean, fully-covered
/// pass.
fn derive_ownership_degrade(
    coverage: M5OwnershipCoverageState,
    owner_source: M5OwnerSource,
    freshness: M5OwnerFreshness,
) -> Option<M5OwnershipDegradeReason> {
    if matches!(freshness, M5OwnerFreshness::OwnerUnknown) {
        Some(M5OwnershipDegradeReason::NotYetEvaluated)
    } else if !owner_source.is_authoritative() {
        // An inference from the last interacting team, or an unrecorded owner, never reads
        // as a resolved owner.
        Some(M5OwnershipDegradeReason::InheritedOrUnresolvedOwner)
    } else if matches!(coverage, M5OwnershipCoverageState::OwnerUnresolved) {
        Some(M5OwnershipDegradeReason::OwnerUnresolvedForService)
    } else if matches!(freshness, M5OwnerFreshness::OwnerMissing) {
        Some(M5OwnershipDegradeReason::OwnerRecordMissing)
    } else if matches!(coverage, M5OwnershipCoverageState::OwnerStale)
        || matches!(freshness, M5OwnerFreshness::OwnerStale)
    {
        Some(M5OwnershipDegradeReason::OwnerRecordStale)
    } else if matches!(coverage, M5OwnershipCoverageState::PolicyHidden) {
        Some(M5OwnershipDegradeReason::OwnershipPolicyHidden)
    } else if matches!(coverage, M5OwnershipCoverageState::PrimaryOnlyNoBackup) {
        Some(M5OwnershipDegradeReason::BackupMissingForService)
    } else if matches!(freshness, M5OwnerFreshness::OwnerAging) {
        Some(M5OwnershipDegradeReason::OwnerRecordAging)
    } else {
        // OwnedWithBackup, authoritative owner, fresh record.
        None
    }
}

/// One worked service-ownership-card resolution case carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ServiceOwnershipCardCase {
    /// The resolver input.
    pub input: M5ServiceOwnershipResolutionInput,
    /// The resolved truth. Must equal `resolve_service_ownership_card(&input)`.
    pub resolved: M5ResolvedServiceOwnershipCard,
}

impl M5ServiceOwnershipCardCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ServiceOwnershipResolutionInput) -> Self {
        let resolved = resolve_service_ownership_card(&input)
            .expect("seed service-ownership-card case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_service_ownership_card(&self.input).as_ref() == Ok(&self.resolved)
    }
}

// ---------------------------------------------------------------------------
// On-call-strip resolver
// ---------------------------------------------------------------------------

/// The full input to the on-call-strip resolver for one strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OnCallStripResolutionInput {
    /// The opaque, export-safe strip id.
    pub strip_id_repr: String,
    /// The opaque on-call role alias (never a personal contact detail).
    pub role_alias: String,
    /// The on-call-coverage state.
    pub coverage_state: M5OnCallCoverageState,
    /// The current availability of the named responder.
    pub availability_state: M5OnCallAvailabilityState,
    /// The primary/secondary role tier.
    pub role_tier: M5OnCallRoleTier,
    /// The escalation route for this strip.
    pub escalation_route: M5EscalationRouteClass,
    /// The opaque, export-safe handoff-continuity representation.
    pub handoff_repr: String,
    /// The on-call roster freshness reading.
    pub roster_freshness: M5OwnerFreshness,
}

/// The resolved on-call-strip truth for one strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedOnCallStrip {
    /// The opaque strip id.
    pub strip_id_repr: String,
    /// The opaque on-call role alias.
    pub role_alias: String,
    /// The on-call-coverage state.
    pub coverage_state: M5OnCallCoverageState,
    /// The current availability state.
    pub availability_state: M5OnCallAvailabilityState,
    /// The primary/secondary role tier.
    pub role_tier: M5OnCallRoleTier,
    /// The escalation route.
    pub escalation_route: M5EscalationRouteClass,
    /// `true` when an escalation route is stated (never `no_escalation_path`).
    pub escalation_route_explicit: bool,
    /// The opaque handoff-continuity representation.
    pub handoff_repr: String,
    /// The on-call roster freshness reading.
    pub roster_freshness: M5OwnerFreshness,
    /// The derived readiness state drawn from the frozen vocabulary.
    pub readiness_state: M5GovernanceReadinessState,
    /// `true` only when the derived readiness is a clean, fully-covered pass.
    pub is_clean_pass: bool,
    /// `true` always: on-call handoff truth is reconstructable from the packet/export.
    pub handoff_continuity: bool,
    /// The strip actions this row always offers (always includes open + page).
    pub strip_actions: Vec<M5OnCallStripAction>,
    /// The degrade reason, present when the strip is not a clean pass.
    pub degrade_reason: Option<M5OnCallDegradeReason>,
    /// The next action, present when the strip is degraded.
    pub next_action: Option<M5OwnershipNextAction>,
    /// A self-contained handoff note naming the escalation route, present always.
    pub handoff_note: String,
    /// A self-contained degrade note, present when the strip is degraded.
    pub degrade_note: Option<String>,
}

/// Errors returned by [`resolve_on_call_strip`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5OnCallStripResolutionError {
    /// The strip id was empty.
    EmptyStripId,
    /// The handoff representation was empty.
    EmptyHandoff,
    /// The role alias carried a personal contact detail (an `@`), not a role alias.
    PersonContactDetailInAlias,
    /// A strip id, role alias, or handoff repr carried forbidden material.
    ForbiddenOnCallMaterial,
}

impl M5OnCallStripResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyStripId => "empty_strip_id",
            Self::EmptyHandoff => "empty_handoff",
            Self::PersonContactDetailInAlias => "person_contact_detail_in_alias",
            Self::ForbiddenOnCallMaterial => "forbidden_on_call_material",
        }
    }
}

impl fmt::Display for M5OnCallStripResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "on-call-strip resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5OnCallStripResolutionError {}

/// Resolves one on-call strip from its declared state.
///
/// The derived readiness state is computed in a fixed degrade-first order: an unknown
/// freshness reading is `not_evaluated`; a missing escalation path blocks; an on-call gap,
/// no current coverage, or a missing roster blocks; no named responder is
/// `owner_unresolved`; a stale roster is `evidence_stale`; an unknown posture is
/// `not_evaluated`; an escalation-only or off-shift posture is `warning`; and a pending
/// handoff is `warning`. Only a strip with covered (or follow-the-sun) coverage, an
/// available named responder, a stated escalation route, and a fresh-or-aging roster is a
/// clean pass. An on-call gap therefore never reads as covered.
pub fn resolve_on_call_strip(
    input: &M5OnCallStripResolutionInput,
) -> Result<M5ResolvedOnCallStrip, M5OnCallStripResolutionError> {
    if input.strip_id_repr.trim().is_empty() {
        return Err(M5OnCallStripResolutionError::EmptyStripId);
    }
    if input.handoff_repr.trim().is_empty() {
        return Err(M5OnCallStripResolutionError::EmptyHandoff);
    }
    if input.role_alias.contains('@') {
        return Err(M5OnCallStripResolutionError::PersonContactDetailInAlias);
    }
    if value_repr_is_forbidden(&input.strip_id_repr)
        || value_repr_is_forbidden(&input.role_alias)
        || value_repr_is_forbidden(&input.handoff_repr)
    {
        return Err(M5OnCallStripResolutionError::ForbiddenOnCallMaterial);
    }

    let escalation_route_explicit = !matches!(
        input.escalation_route,
        M5EscalationRouteClass::NoEscalationPath
    );
    let degrade_reason = derive_on_call_degrade(
        input.coverage_state,
        input.availability_state,
        input.role_tier,
        input.escalation_route,
        input.roster_freshness,
    );
    let readiness_state = match degrade_reason {
        Some(reason) => reason.readiness_state(),
        None => M5GovernanceReadinessState::Passing,
    };
    let next_action = degrade_reason.map(M5OnCallDegradeReason::next_action);
    let handoff_note = format!(
        "On-call handoff: escalation route `{}` ({})",
        input.escalation_route.as_str(),
        if escalation_route_explicit {
            "explicit escalation path"
        } else {
            "no escalation path — page cannot escalate"
        }
    );
    let degrade_note = degrade_reason.map(|reason| {
        format!(
            "On-call strip degraded: {} — state `{}`; next: {}",
            reason.phrase(),
            readiness_state.as_str(),
            reason.next_action().as_str()
        )
    });

    Ok(M5ResolvedOnCallStrip {
        strip_id_repr: input.strip_id_repr.clone(),
        role_alias: input.role_alias.clone(),
        coverage_state: input.coverage_state,
        availability_state: input.availability_state,
        role_tier: input.role_tier,
        escalation_route: input.escalation_route,
        escalation_route_explicit,
        handoff_repr: input.handoff_repr.clone(),
        roster_freshness: input.roster_freshness,
        readiness_state,
        is_clean_pass: readiness_state.is_clean_pass(),
        handoff_continuity: true,
        strip_actions: vec![
            M5OnCallStripAction::OpenOnCallSchedule,
            M5OnCallStripAction::PageEscalationPath,
            M5OnCallStripAction::ExportOnCallLedger,
        ],
        degrade_reason,
        next_action,
        handoff_note,
        degrade_note,
    })
}

/// The fixed degrade-first on-call ladder. Returns `None` for a clean, fully-covered pass.
fn derive_on_call_degrade(
    coverage: M5OnCallCoverageState,
    availability: M5OnCallAvailabilityState,
    role_tier: M5OnCallRoleTier,
    escalation: M5EscalationRouteClass,
    freshness: M5OwnerFreshness,
) -> Option<M5OnCallDegradeReason> {
    if matches!(freshness, M5OwnerFreshness::OwnerUnknown) {
        Some(M5OnCallDegradeReason::NotYetEvaluated)
    } else if matches!(escalation, M5EscalationRouteClass::NoEscalationPath) {
        Some(M5OnCallDegradeReason::EscalationPathMissing)
    } else if matches!(coverage, M5OnCallCoverageState::OnCallGap)
        || matches!(availability, M5OnCallAvailabilityState::NoCoverage)
        || matches!(freshness, M5OwnerFreshness::OwnerMissing)
    {
        Some(M5OnCallDegradeReason::OnCallGapOpen)
    } else if matches!(role_tier, M5OnCallRoleTier::NoNamedResponder) {
        Some(M5OnCallDegradeReason::NoNamedResponder)
    } else if matches!(freshness, M5OwnerFreshness::OwnerStale) {
        Some(M5OnCallDegradeReason::RosterStale)
    } else if matches!(coverage, M5OnCallCoverageState::OnCallUnknown)
        || matches!(availability, M5OnCallAvailabilityState::AvailabilityUnknown)
    {
        Some(M5OnCallDegradeReason::OnCallPostureUnknown)
    } else if matches!(coverage, M5OnCallCoverageState::EscalationOnly)
        || matches!(availability, M5OnCallAvailabilityState::OffShift)
    {
        Some(M5OnCallDegradeReason::EscalationOnlyCoverage)
    } else if matches!(availability, M5OnCallAvailabilityState::HandoffPending) {
        Some(M5OnCallDegradeReason::HandoffPending)
    } else {
        // OnCallCovered or FollowTheSun, AvailableNow, named responder, escalation path,
        // fresh or aging roster.
        None
    }
}

/// One worked on-call-strip resolution case carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OnCallStripCase {
    /// The resolver input.
    pub input: M5OnCallStripResolutionInput,
    /// The resolved truth. Must equal `resolve_on_call_strip(&input)`.
    pub resolved: M5ResolvedOnCallStrip,
}

impl M5OnCallStripCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5OnCallStripResolutionInput) -> Self {
        let resolved = resolve_on_call_strip(&input).expect("seed on-call-strip case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_on_call_strip(&self.input).as_ref() == Ok(&self.resolved)
    }
}

// ---------------------------------------------------------------------------
// Parity matrix
// ---------------------------------------------------------------------------

/// One row in the controls matrix: one governance consumer bound to the shared card and
/// strip anatomy, readiness states, ownership-coverage states, on-call-coverage states,
/// escalation-route classes, support classes, freshness readings, degrade reasons,
/// actions, export fields, and accessibility routes, plus worked resolution cases for both
/// resolver halves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OwnershipRow {
    /// Governance consumer family.
    pub consumer_surface: M5OwnershipConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5GovernanceQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 governance surface families that render / consume these components.
    pub surface_families: Vec<M5GovernanceSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts these components render (must include the mandatory parts).
    pub anatomy_parts: Vec<M5OwnershipAnatomyPart>,
    /// Required labels these components can show (must include the mandatory labels).
    pub required_labels: Vec<M5GovernanceRequiredLabel>,
    /// Readiness states these components distinguish.
    pub readiness_states: Vec<M5GovernanceReadinessState>,
    /// Support classes these cards distinguish.
    pub support_classes: Vec<M5ServiceSupportClass>,
    /// Ownership-coverage states these cards distinguish.
    pub ownership_coverage_states: Vec<M5OwnershipCoverageState>,
    /// Owner sources these cards distinguish.
    pub owner_sources: Vec<M5OwnerSource>,
    /// Owner-freshness readings these components distinguish.
    pub owner_freshness_states: Vec<M5OwnerFreshness>,
    /// Ownership degrade reasons these cards name.
    pub ownership_degrade_reasons: Vec<M5OwnershipDegradeReason>,
    /// On-call-coverage states these strips distinguish.
    pub on_call_coverage_states: Vec<M5OnCallCoverageState>,
    /// On-call availability states these strips distinguish.
    pub availability_states: Vec<M5OnCallAvailabilityState>,
    /// On-call role tiers these strips distinguish.
    pub role_tiers: Vec<M5OnCallRoleTier>,
    /// Escalation route classes these strips name.
    pub escalation_route_classes: Vec<M5EscalationRouteClass>,
    /// On-call degrade reasons these strips name.
    pub on_call_degrade_reasons: Vec<M5OnCallDegradeReason>,
    /// Card actions these rows offer (must include the mandatory actions).
    pub card_actions: Vec<M5OwnershipCardAction>,
    /// Strip actions these rows offer (must include the mandatory actions).
    pub strip_actions: Vec<M5OnCallStripAction>,
    /// Next actions these components name.
    pub next_actions: Vec<M5OwnershipNextAction>,
    /// Export fields these components carry (must include the mandatory fields).
    pub export_fields: Vec<M5OwnershipExportField>,
    /// Non-visual accessibility routes these components offer.
    pub accessibility_routes: Vec<M5GovernanceAccessibilityRoute>,
    /// Governance subsystems that consume these components' projection.
    pub consumer_surfaces: Vec<M5GovernanceConsumerSurface>,
    /// Downgrade triggers that apply to these components.
    pub downgrade_triggers: Vec<M5GovernanceDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked service-ownership-card cases proving the card resolver on this consumer.
    pub ownership_examples: Vec<M5ServiceOwnershipCardCase>,
    /// Worked on-call-strip cases proving the strip resolver on this consumer.
    pub on_call_examples: Vec<M5OnCallStripCase>,
    /// Hard invariant: this row never renders an unowned or backup-missing surface as
    /// covered. MUST be `false`.
    pub renders_unowned_or_backup_missing_as_covered: bool,
    /// Hard invariant: this row never inherits the last interacting team as the owner.
    /// MUST be `false`.
    pub inherits_last_interacting_team_as_owner: bool,
    /// Hard invariant: this row never hides an on-call gap or the escalation route. MUST
    /// be `false`.
    pub hides_on_call_gap_or_escalation_route: bool,
    /// Hard invariant: this row never invents an ownership-local status word. MUST be
    /// `false`.
    pub invents_ownership_local_status_grammar: bool,
}

impl M5OwnershipRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5OwnershipAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5OwnershipAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory required label.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5GovernanceRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5GovernanceRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// True when the row declares every mandatory card action.
    fn declares_mandatory_card_actions(&self) -> bool {
        let present: BTreeSet<M5OwnershipCardAction> = self.card_actions.iter().copied().collect();
        M5OwnershipCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// True when the row declares every mandatory strip action.
    fn declares_mandatory_strip_actions(&self) -> bool {
        let present: BTreeSet<M5OnCallStripAction> = self.strip_actions.iter().copied().collect();
        M5OnCallStripAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5OwnershipExportField> =
            self.export_fields.iter().copied().collect();
        M5OwnershipExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.renders_unowned_or_backup_missing_as_covered
            && !self.inherits_last_interacting_team_as_owner
            && !self.hides_on_call_gap_or_escalation_route
            && !self.invents_ownership_local_status_grammar
    }
}

/// Self-describing controlled-vocabulary set carried by this controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OwnershipVocabularySet {
    /// Governance consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Readiness-state tokens (reused from the frozen matrix).
    pub readiness_states: Vec<String>,
    /// Support-class tokens.
    pub support_classes: Vec<String>,
    /// Ownership-coverage-state tokens (reused from the frozen matrix).
    pub ownership_coverage_states: Vec<String>,
    /// Owner-source tokens.
    pub owner_sources: Vec<String>,
    /// Owner-freshness tokens.
    pub owner_freshness_states: Vec<String>,
    /// Ownership-degrade-reason tokens.
    pub ownership_degrade_reasons: Vec<String>,
    /// On-call-coverage-state tokens (reused from the frozen matrix).
    pub on_call_coverage_states: Vec<String>,
    /// On-call availability tokens.
    pub availability_states: Vec<String>,
    /// On-call role-tier tokens.
    pub role_tiers: Vec<String>,
    /// Escalation-route-class tokens (reused from the frozen matrix).
    pub escalation_route_classes: Vec<String>,
    /// On-call-degrade-reason tokens.
    pub on_call_degrade_reasons: Vec<String>,
    /// Card-action tokens.
    pub card_actions: Vec<String>,
    /// Strip-action tokens.
    pub strip_actions: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5OwnershipVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5OwnershipConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5OwnershipAnatomyPart::ALL, |v| v.as_str()),
            readiness_states: tokens(&M5GovernanceReadinessState::ALL, |v| v.as_str()),
            support_classes: tokens(&M5ServiceSupportClass::ALL, |v| v.as_str()),
            ownership_coverage_states: tokens(&M5OwnershipCoverageState::ALL, |v| v.as_str()),
            owner_sources: tokens(&M5OwnerSource::ALL, |v| v.as_str()),
            owner_freshness_states: tokens(&M5OwnerFreshness::ALL, |v| v.as_str()),
            ownership_degrade_reasons: tokens(&M5OwnershipDegradeReason::ALL, |v| v.as_str()),
            on_call_coverage_states: tokens(&M5OnCallCoverageState::ALL, |v| v.as_str()),
            availability_states: tokens(&M5OnCallAvailabilityState::ALL, |v| v.as_str()),
            role_tiers: tokens(&M5OnCallRoleTier::ALL, |v| v.as_str()),
            escalation_route_classes: tokens(&M5EscalationRouteClass::ALL, |v| v.as_str()),
            on_call_degrade_reasons: tokens(&M5OnCallDegradeReason::ALL, |v| v.as_str()),
            card_actions: tokens(&M5OwnershipCardAction::ALL, |v| v.as_str()),
            strip_actions: tokens(&M5OnCallStripAction::ALL, |v| v.as_str()),
            next_actions: tokens(&M5OwnershipNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5OwnershipExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5GovernanceAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5OwnershipReview {
    /// One controls packet carries ownership and on-call truth on every consumer.
    pub one_packet_carries_ownership_and_on_call_truth: bool,
    /// The service identity and owning role are shown before an ownership claim is trusted.
    pub service_identity_and_owning_role_always_shown: bool,
    /// An unowned or backup-missing surface never reads as covered.
    pub unowned_or_backup_missing_never_reads_covered: bool,
    /// The owner is never inherited from the last interacting team as false truth.
    pub owner_never_inherited_from_last_interacting_team: bool,
    /// The support class and owner freshness are always shown on the card.
    pub support_class_and_freshness_always_shown: bool,
    /// An on-call gap never reads as covered.
    pub on_call_gap_never_reads_covered: bool,
    /// The escalation route is always explicit.
    pub escalation_route_always_explicit: bool,
    /// The readiness state is drawn only from the frozen vocabulary.
    pub readiness_state_drawn_from_frozen_vocabulary: bool,
    /// Support, operator, and release surfaces reuse one role-based ownership/escalation
    /// model.
    pub support_operator_release_reuse_one_model: bool,
    /// The support / export packet reconstructs card and strip truth.
    pub support_export_reconstructs_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// An owner or on-call alias is a role alias, never a personal contact detail.
    pub owner_alias_is_role_not_person: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OwnershipConsumerProjection {
    /// Operator, release, service-health, support, and CLI consumers all consume the
    /// shared controls packet.
    pub surfaces_consume_shared_packet: bool,
    /// The ownership resolver reads a single canonical source.
    pub ownership_resolver_reads_single_source: bool,
    /// The on-call resolver reads a single canonical source.
    pub on_call_resolver_reads_single_source: bool,
    /// The escalation-route reading reads a single canonical source.
    pub escalation_route_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OwnershipProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the controls packet.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OwnershipReleasePosture {
    /// Ref of the supporting governance packet.
    pub governance_packet_ref: String,
    /// Ref of the supporting assurance audit.
    pub assurance_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ServiceOwnershipOnCallControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ServiceOwnershipOnCallControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5OwnershipRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5OwnershipVocabularySet,
    /// Governance-review block.
    pub governance_review: M5OwnershipReview,
    /// Consumer projection block.
    pub consumer_projection: M5OwnershipConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5OwnershipProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5OwnershipReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 service-ownership / on-call controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ServiceOwnershipOnCallControlsPacket {
    /// Record kind; must equal [`M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5OwnershipRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5OwnershipVocabularySet,
    /// Governance-review block.
    pub governance_review: M5OwnershipReview,
    /// Consumer projection block.
    pub consumer_projection: M5OwnershipConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5OwnershipProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5OwnershipReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ServiceOwnershipOnCallControlsPacket {
    /// Builds an M5 service-ownership / on-call controls packet from stable-lane input.
    pub fn new(input: M5ServiceOwnershipOnCallControlsPacketInput) -> Self {
        Self {
            record_kind: M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            controls_rows: input.controls_rows,
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

    /// Validates the M5 service-ownership / on-call controls invariants.
    pub fn validate(&self) -> Vec<M5ServiceOwnershipOnCallControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_RECORD_KIND {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_VERSION {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_controls_rows(self, &mut violations);
        validate_ownerless_or_backup_missing_degrades_proven(self, &mut violations);
        validate_shared_role_based_model_proven(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 service-ownership/on-call controls serializes"),
        ) {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::RawMaterialInExport);
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
            .expect("m5 service-ownership/on-call controls packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governance consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,readiness_states,support_classes,ownership_coverage_states,on_call_coverage_states,escalation_route_classes,card_actions,strip_actions,export_fields,ownership_example_count,on_call_example_count\n",
        );
        for row in &self.controls_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.readiness_states, |v| v.as_str()),
                join_tokens(&row.support_classes, |v| v.as_str()),
                join_tokens(&row.ownership_coverage_states, |v| v.as_str()),
                join_tokens(&row.on_call_coverage_states, |v| v.as_str()),
                join_tokens(&row.escalation_route_classes, |v| v.as_str()),
                join_tokens(&row.card_actions, |v| v.as_str()),
                join_tokens(&row.strip_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.ownership_examples.len(),
                row.on_call_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .controls_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Service-Ownership Card and On-Call Strip Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Governance consumers: {} ({} stable)\n",
            self.controls_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Readiness states: {}\n",
            self.vocabulary_set.readiness_states.join(", ")
        ));
        out.push_str(&format!(
            "- Ownership-coverage states: {}\n",
            self.vocabulary_set.ownership_coverage_states.join(", ")
        ));
        out.push_str(&format!(
            "- On-call-coverage states: {}\n",
            self.vocabulary_set.on_call_coverage_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Governance consumers\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked ownership cards: {}\n",
                row.ownership_examples.len()
            ));
            for case in &row.ownership_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (coverage `{}`, source `{}`, backup `{}`, covered-visible `{}`)\n",
                    case.resolved.service_id_repr,
                    case.resolved.readiness_state.as_str(),
                    case.resolved.coverage_state.as_str(),
                    case.resolved.owner_source.as_str(),
                    case.resolved.backup_present,
                    case.resolved.coverage_visible,
                ));
            }
            out.push_str(&format!(
                "  - Worked on-call strips: {}\n",
                row.on_call_examples.len()
            ));
            for case in &row.on_call_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (coverage `{}`, availability `{}`, tier `{}`, escalation `{}`)\n",
                    case.resolved.strip_id_repr,
                    case.resolved.readiness_state.as_str(),
                    case.resolved.coverage_state.as_str(),
                    case.resolved.availability_state.as_str(),
                    case.resolved.role_tier.as_str(),
                    case.resolved.escalation_route.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 service-ownership / on-call controls
/// export.
#[derive(Debug)]
pub enum M5ServiceOwnershipOnCallControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ServiceOwnershipOnCallControlsViolation>),
}

impl fmt::Display for M5ServiceOwnershipOnCallControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 service-ownership/on-call controls export parse failed: {error}"
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
                    "m5 service-ownership/on-call controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ServiceOwnershipOnCallControlsArtifactError {}

/// Validation failures emitted by [`M5ServiceOwnershipOnCallControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ServiceOwnershipOnCallControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governance consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory required labels.
    MandatoryLabelMissing,
    /// A controls row omits one of the mandatory card actions.
    MandatoryCardActionMissing,
    /// A controls row omits one of the mandatory strip actions.
    MandatoryStripActionMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A controls row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A controls row declares no service-ownership worked cases.
    OwnershipExampleMissing,
    /// A controls row declares no on-call worked cases.
    OnCallExampleMissing,
    /// A worked service-ownership case does not match a fresh resolve of its input.
    OwnershipExampleDrift,
    /// A worked on-call case does not match a fresh resolve of its input.
    OnCallExampleDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked ownership case proves an ownerless or backup-missing surface that never
    /// reads as covered, and an owner inferred from the last interacting team that reads
    /// as unresolved (the AC-1 example).
    OwnerlessOrBackupMissingDegradeUnproven,
    /// The operator, release, and support consumers do not all reuse the shared role-based
    /// ownership/escalation model with worked cases (the AC-2 example).
    SharedRoleBasedModelUnproven,
    /// A controls row violates a hard invariant.
    ControlsInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ServiceOwnershipOnCallControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::MandatoryCardActionMissing => "mandatory_card_action_missing",
            Self::MandatoryStripActionMissing => "mandatory_strip_action_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::OwnershipExampleMissing => "ownership_example_missing",
            Self::OnCallExampleMissing => "on_call_example_missing",
            Self::OwnershipExampleDrift => "ownership_example_drift",
            Self::OnCallExampleDrift => "on_call_example_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::OwnerlessOrBackupMissingDegradeUnproven => {
                "ownerless_or_backup_missing_degrade_unproven"
            }
            Self::SharedRoleBasedModelUnproven => "shared_role_based_model_unproven",
            Self::ControlsInvariantViolated => "controls_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 service-ownership / on-call controls
/// export.
pub fn current_stable_m5_service_ownership_on_call_controls_export(
) -> Result<M5ServiceOwnershipOnCallControlsPacket, M5ServiceOwnershipOnCallControlsArtifactError> {
    let packet: M5ServiceOwnershipOnCallControlsPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-service-ownership-on-call-controls-proof/support_export.json"
    )))
        .map_err(M5ServiceOwnershipOnCallControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ServiceOwnershipOnCallControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ServiceOwnershipOnCallControlsPacket,
    violations: &mut Vec<M5ServiceOwnershipOnCallControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_REF,
        M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_DOC_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_DOC_REF,
        M5_SERVICE_OWNERSHIP_CARD_CONTRACT_REF,
        M5_ON_CALL_STRIP_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ServiceOwnershipOnCallControlsPacket,
    violations: &mut Vec<M5ServiceOwnershipOnCallControlsViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ServiceOwnershipOnCallControlsViolation::VocabularySetDrift);
    }
}

fn validate_controls_rows(
    packet: &M5ServiceOwnershipOnCallControlsPacket,
    violations: &mut Vec<M5ServiceOwnershipOnCallControlsViolation>,
) {
    let present: BTreeSet<M5OwnershipConsumerSurface> = packet
        .controls_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5OwnershipConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.readiness_states.is_empty()
            || row.support_classes.is_empty()
            || row.ownership_coverage_states.is_empty()
            || row.owner_sources.is_empty()
            || row.owner_freshness_states.is_empty()
            || row.ownership_degrade_reasons.is_empty()
            || row.on_call_coverage_states.is_empty()
            || row.availability_states.is_empty()
            || row.role_tiers.is_empty()
            || row.escalation_route_classes.is_empty()
            || row.on_call_degrade_reasons.is_empty()
            || row.next_actions.is_empty()
        {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::MandatoryLabelMissing);
        }
        if !row.declares_mandatory_card_actions() {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::MandatoryCardActionMissing);
        }
        if !row.declares_mandatory_strip_actions() {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::MandatoryStripActionMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5GovernanceAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::AccessibilityRouteMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::DowngradeTriggersMissing);
        }
        if row.ownership_examples.is_empty() {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::OwnershipExampleMissing);
        }
        if row.on_call_examples.is_empty() {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::OnCallExampleMissing);
        }
        if row
            .ownership_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::OwnershipExampleDrift);
        }
        if row
            .on_call_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::OnCallExampleDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::ControlsInvariantViolated);
        }
    }
}

/// At least one worked ownership case across the matrix must prove both halves of AC-1:
/// a backup-missing surface (`primary_only_no_backup`) that never reads as covered, and an
/// owner inferred only from the last interacting team that reads as `owner_unresolved`
/// rather than inheriting the last team as false truth.
fn validate_ownerless_or_backup_missing_degrades_proven(
    packet: &M5ServiceOwnershipOnCallControlsPacket,
    violations: &mut Vec<M5ServiceOwnershipOnCallControlsViolation>,
) {
    let backup_missing_proven = packet.controls_rows.iter().any(|row| {
        row.ownership_examples.iter().any(|case| {
            matches!(
                case.resolved.coverage_state,
                M5OwnershipCoverageState::PrimaryOnlyNoBackup
            ) && !case.resolved.is_clean_pass
                && case.resolved.coverage_visible
        })
    });
    let inherited_owner_proven = packet.controls_rows.iter().any(|row| {
        row.ownership_examples.iter().any(|case| {
            matches!(
                case.resolved.owner_source,
                M5OwnerSource::LastInteractingTeamInference
            ) && !case.resolved.owner_resolved
                && case.resolved.readiness_state == M5GovernanceReadinessState::OwnerUnresolved
        })
    });
    if !backup_missing_proven || !inherited_owner_proven {
        violations.push(
            M5ServiceOwnershipOnCallControlsViolation::OwnerlessOrBackupMissingDegradeUnproven,
        );
    }
}

/// The operator, release, and support consumers must each be present and reuse the shared
/// role-based ownership/escalation model, each carrying at least one worked ownership card
/// and one worked on-call strip. This is the AC-2 example that support, operator, and
/// release surfaces reuse one model rather than cloning prose.
fn validate_shared_role_based_model_proven(
    packet: &M5ServiceOwnershipOnCallControlsPacket,
    violations: &mut Vec<M5ServiceOwnershipOnCallControlsViolation>,
) {
    let shared = M5OwnershipConsumerSurface::SHARED_MODEL_REQUIRED
        .iter()
        .all(|required| {
            packet.controls_rows.iter().any(|row| {
                row.consumer_surface == *required
                    && !row.ownership_examples.is_empty()
                    && !row.on_call_examples.is_empty()
            })
        });
    if !shared {
        violations.push(M5ServiceOwnershipOnCallControlsViolation::SharedRoleBasedModelUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ServiceOwnershipOnCallControlsPacket,
    violations: &mut Vec<M5ServiceOwnershipOnCallControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_packet_carries_ownership_and_on_call_truth,
        review.service_identity_and_owning_role_always_shown,
        review.unowned_or_backup_missing_never_reads_covered,
        review.owner_never_inherited_from_last_interacting_team,
        review.support_class_and_freshness_always_shown,
        review.on_call_gap_never_reads_covered,
        review.escalation_route_always_explicit,
        review.readiness_state_drawn_from_frozen_vocabulary,
        review.support_operator_release_reuse_one_model,
        review.support_export_reconstructs_truth,
        review.every_row_declares_accessibility_route,
        review.owner_alias_is_role_not_person,
    ] {
        if !ok {
            violations.push(M5ServiceOwnershipOnCallControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ServiceOwnershipOnCallControlsPacket,
    violations: &mut Vec<M5ServiceOwnershipOnCallControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.surfaces_consume_shared_packet,
        projection.ownership_resolver_reads_single_source,
        projection.on_call_resolver_reads_single_source,
        projection.escalation_route_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations
                .push(M5ServiceOwnershipOnCallControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ServiceOwnershipOnCallControlsPacket,
    violations: &mut Vec<M5ServiceOwnershipOnCallControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ServiceOwnershipOnCallControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ServiceOwnershipOnCallControlsPacket,
    violations: &mut Vec<M5ServiceOwnershipOnCallControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.governance_packet_ref.trim().is_empty()
        || posture.assurance_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ServiceOwnershipOnCallControlsViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
