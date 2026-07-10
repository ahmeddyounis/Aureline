//! Shared consumers for the reusable M5 governance-dashboard components, so
//! fitness dashboard tiles, governance report rows, waiver-expiry queue items,
//! release-gate banners, mitigation note cards, service-ownership cards, on-call
//! strips, decision-right cards, and milestone dashboard rows keep readiness,
//! evidence-freshness, waiver, owner-coverage, and decision-forum language aligned
//! across every claimed M5 surface that summarizes readiness or routes a
//! governance decision.
//!
//! Aureline's frozen governance-dashboard component matrix
//! ([`crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix`])
//! names the nine governed component families, and four sibling `implement_*` lanes
//! narrow those nine families into working primitives with their own canonical
//! schema, contract doc, and support-export artifact:
//!
//! * the fitness dashboard tile / governance report row
//!   ([`crate::implement_fitness_dashboard_tiles_and_governance_report_rows_with_protected_metric_identity_threshold_state_provenance_evidence_freshness_owner_and_report_continuity`]),
//! * the waiver-expiry queue item / release-gate banner / mitigation note card
//!   ([`crate::implement_waiver_expiry_queue_items_release_gate_banners_and_mitigation_note_cards_with_owner_expiry_milestone_impact_blocked_waived_evidence_stale_vocabulary_and_user_facing_mitigation_truth`]),
//! * the service-ownership card / on-call strip
//!   ([`crate::implement_service_ownership_cards_and_on_call_strips_with_role_based_owner_escalation_aliases_support_class_freshness_backup_coverage_and_export_safe_continuity`]),
//!   and
//! * the decision-right card / milestone dashboard row
//!   ([`crate::implement_decision_right_cards_and_milestone_dashboard_rows_with_required_forum_reason_satisfied_pending_state_blocker_and_waiver_counts_nearest_gate_and_next_review_continuity`]).
//!
//! This module is the *adoption* lane over those primitives. It proves the nine
//! families are reusable components — not one governance pipeline plus a few
//! admin-only dashboards — by binding every claimed M5 governance-dashboard
//! consumer (the assurance center, the release center, the operator dashboard, the
//! shiproom summary, the support export, the About/help surface, the docs portal,
//! and the CLI inspect / headless surface) to the same canonical component schemas
//! and the same governance vocabulary. Each consumer points at the primitive's
//! canonical schema and support-export artifact rather than re-wording readiness,
//! evidence-freshness, waiver, owner, or decision-forum facts in local prose, and
//! each keeps that vocabulary truthful even when it renders under stale evidence,
//! an expiring waiver, missing owner coverage, an unresolved forum, or a
//! not-evaluated-here context outside the authoritative assurance center.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_governance_consumer_binding`] — that takes one
//!    consumer's adoption of one component family, the descriptor set it surfaces,
//!    the governance evidence state it renders under, and its shared readiness
//!    vocabulary, and produces one [`M5ResolvedGovernanceBinding`] carrying the
//!    derived descriptor-parity state and — whenever the binding renders under
//!    narrowed evidence — a self-contained [`M5GovernanceNarrowBanner`] that names
//!    the exact reason (evidence stale, waiver expiring, owner coverage missing,
//!    forum unresolved, or not evaluated here), the descriptors that stay
//!    preserved, and the next action, rather than a generic "degraded" note. The
//!    resolver never lets a narrowed context drop a required descriptor, never
//!    reads waived or stale evidence as a clean pass, and never invents a second
//!    status grammar.
//! 2. A parity matrix — [`M5GovernanceComponentConsumerPacket`] — that binds one
//!    row per claimed M5 governance-dashboard consumer to the nine canonical
//!    component families, the one shared governance vocabulary, the same evidence
//!    states, narrow reasons, next actions, export fields, and non-visual
//!    accessibility routes, so governance facts stop diverging between the product
//!    UI, the CLI, the docs, and the support artifact.
//!
//! The governance surface families, deployment lines, consumer surfaces,
//! accessibility routes, qualification classes, downgrade triggers, and readiness
//! vocabulary are reused verbatim from the frozen governance-dashboard component
//! matrix. This module mints new vocabulary only for what the adoption lane itself
//! needs: its governance-dashboard consumers, the shared descriptor vocabulary, the
//! governance evidence states, the projection modes, the descriptor-parity states,
//! the narrow reasons and next actions, the consumer anatomy parts, and the export
//! fields.
//!
//! Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and
//! user text bodies stay outside the support boundary; every label is carried only
//! as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-governance-dashboard-component-consumer.schema.json`](../../../../schemas/ui/m5-governance-dashboard-component-consumer.schema.json)
//! and the contract doc is
//! [`docs/help/m5_governance_dashboard_component_consumer_contract.md`](../../../../docs/help/m5_governance_dashboard_component_consumer_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-governance-dashboard-component-consumers/`](../../../../fixtures/ui/m5-governance-dashboard-component-consumers/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_governance_component_consumer_docs_stale_narrowed,
    seeded_m5_governance_component_consumer_operator_ownership_narrowed,
    seeded_m5_governance_component_consumer_packet, M5_GOVERNANCE_COMPONENT_CONSUMER_PACKET_ID,
};

// The governance surface families, deployment lines, consumer surfaces,
// accessibility routes, qualification classes, downgrade triggers, and readiness
// vocabulary are frozen once, in the governance-dashboard component matrix. This
// adoption lane reuses them verbatim so it never invents a parallel governance
// vocabulary.
pub use crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix::{
    M5DeploymentLine, M5GovernanceAccessibilityRoute, M5GovernanceConsumerSurface,
    M5GovernanceDashboardComponentFamily, M5GovernanceDowngradeTrigger,
    M5GovernanceQualificationClass, M5GovernanceReadinessState, M5GovernanceSurfaceFamily,
};

// The four canonical primitive schema / doc / artifact refs this adoption lane
// points every consumer at, rather than re-wording their facts in local prose.
use crate::implement_decision_right_cards_and_milestone_dashboard_rows_with_required_forum_reason_satisfied_pending_state_blocker_and_waiver_counts_nearest_gate_and_next_review_continuity::{
    M5_DECISION_RIGHT_MILESTONE_CONTROLS_ARTIFACT_REF, M5_DECISION_RIGHT_MILESTONE_CONTROLS_DOC_REF,
    M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_REF,
};
use crate::implement_fitness_dashboard_tiles_and_governance_report_rows_with_protected_metric_identity_threshold_state_provenance_evidence_freshness_owner_and_report_continuity::{
    M5_FITNESS_GOVERNANCE_CONTROLS_ARTIFACT_REF, M5_FITNESS_GOVERNANCE_CONTROLS_DOC_REF,
    M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_REF,
};
use crate::implement_service_ownership_cards_and_on_call_strips_with_role_based_owner_escalation_aliases_support_class_freshness_backup_coverage_and_export_safe_continuity::{
    M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_ARTIFACT_REF, M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_DOC_REF,
    M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_REF,
};
use crate::implement_waiver_expiry_queue_items_release_gate_banners_and_mitigation_note_cards_with_owner_expiry_milestone_impact_blocked_waived_evidence_stale_vocabulary_and_user_facing_mitigation_truth::{
    M5_WAIVER_GATE_CONTROLS_ARTIFACT_REF, M5_WAIVER_GATE_CONTROLS_DOC_REF,
    M5_WAIVER_GATE_CONTROLS_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5GovernanceComponentConsumerPacket`].
pub const M5_GOVERNANCE_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_assurance_center_release_center_operator_dashboard_support_export_shiproom_about_help_consumers_so_governance_dashboard_components_keep_fitness_ownership_waiver_and_decision_language_aligned";

/// Schema version for M5 governance-dashboard-component-consumer records.
pub const M5_GOVERNANCE_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the governance-dashboard-component-consumer boundary schema.
pub const M5_GOVERNANCE_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-governance-dashboard-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_GOVERNANCE_CONSUMER_DOC_REF: &str =
    "docs/help/m5_governance_dashboard_component_consumer_contract.md";

/// Repo-relative path of the frozen governance-dashboard component matrix schema
/// this lane adopts from.
pub const M5_GOVERNANCE_CONSUMER_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-governance-dashboard-component-matrix.schema.json";

/// Repo-relative path of the governance-dashboard component matrix contract doc.
pub const M5_GOVERNANCE_CONSUMER_MATRIX_DOC_REF: &str =
    "docs/help/m5_governance_dashboard_components_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_GOVERNANCE_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-governance-dashboard-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_GOVERNANCE_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-governance-dashboard-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_GOVERNANCE_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-governance-dashboard-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_GOVERNANCE_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-governance-dashboard-component-consumer-proof/summary.md";

/// One claimed M5 governance-dashboard consumer that adopts the shared governance
/// components. These are the consumers the acceptance criteria name — the assurance
/// center, the release center, the operator dashboard, the shiproom summary, the
/// support export, About/help, the docs portal, and the CLI inspect surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceDashboardConsumer {
    /// The assurance-center dashboard.
    AssuranceCenter,
    /// The release-center surface.
    ReleaseCenter,
    /// The operator-overview dashboard.
    OperatorDashboard,
    /// The shiproom summary / packet.
    ShiproomSummary,
    /// The support export.
    SupportExport,
    /// The About/help surface.
    AboutHelp,
    /// The docs portal.
    DocsPortal,
    /// The CLI inspect / headless surface.
    CliInspect,
}

impl M5GovernanceDashboardConsumer {
    /// Every claimed governance-dashboard consumer, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::AssuranceCenter,
        Self::ReleaseCenter,
        Self::OperatorDashboard,
        Self::ShiproomSummary,
        Self::SupportExport,
        Self::AboutHelp,
        Self::DocsPortal,
        Self::CliInspect,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssuranceCenter => "assurance_center",
            Self::ReleaseCenter => "release_center",
            Self::OperatorDashboard => "operator_dashboard",
            Self::ShiproomSummary => "shiproom_summary",
            Self::SupportExport => "support_export",
            Self::AboutHelp => "about_help",
            Self::DocsPortal => "docs_portal",
            Self::CliInspect => "cli_inspect",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AssuranceCenter => "Assurance Center",
            Self::ReleaseCenter => "Release Center",
            Self::OperatorDashboard => "Operator Dashboard",
            Self::ShiproomSummary => "Shiproom Summary",
            Self::SupportExport => "Support Export",
            Self::AboutHelp => "About / Help",
            Self::DocsPortal => "Docs Portal",
            Self::CliInspect => "CLI Inspect",
        }
    }

    /// True when this consumer is a docs/help surface — the surfaces the acceptance
    /// criteria single out for a canonical-schema reference so their prose can never
    /// drift from the product truth.
    pub const fn is_docs_or_help(self) -> bool {
        matches!(self, Self::AboutHelp | Self::DocsPortal)
    }
}

/// The canonical boundary schema ref of the narrowed primitive that owns a
/// governance-dashboard component family. A consumer that adopts a family must point
/// at this schema, not at a local re-description. The nine matrix families narrow
/// into four canonical controls packets.
pub const fn component_canonical_schema_ref(
    family: M5GovernanceDashboardComponentFamily,
) -> &'static str {
    use M5GovernanceDashboardComponentFamily as Family;
    match family {
        Family::FitnessDashboardTile | Family::GovernanceReportRow => {
            M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_REF
        }
        Family::WaiverExpiryQueueItem | Family::ReleaseGateBanner | Family::MitigationNoteCard => {
            M5_WAIVER_GATE_CONTROLS_SCHEMA_REF
        }
        Family::ServiceOwnershipCard | Family::OnCallStrip => {
            M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_REF
        }
        Family::DecisionRightCard | Family::MilestoneDashboardRow => {
            M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_REF
        }
    }
}

/// The canonical contract-doc ref of the narrowed primitive that owns a family.
pub const fn component_canonical_doc_ref(
    family: M5GovernanceDashboardComponentFamily,
) -> &'static str {
    use M5GovernanceDashboardComponentFamily as Family;
    match family {
        Family::FitnessDashboardTile | Family::GovernanceReportRow => {
            M5_FITNESS_GOVERNANCE_CONTROLS_DOC_REF
        }
        Family::WaiverExpiryQueueItem | Family::ReleaseGateBanner | Family::MitigationNoteCard => {
            M5_WAIVER_GATE_CONTROLS_DOC_REF
        }
        Family::ServiceOwnershipCard | Family::OnCallStrip => {
            M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_DOC_REF
        }
        Family::DecisionRightCard | Family::MilestoneDashboardRow => {
            M5_DECISION_RIGHT_MILESTONE_CONTROLS_DOC_REF
        }
    }
}

/// The canonical support-export artifact ref of the narrowed primitive that owns a
/// family.
pub const fn component_canonical_artifact_ref(
    family: M5GovernanceDashboardComponentFamily,
) -> &'static str {
    use M5GovernanceDashboardComponentFamily as Family;
    match family {
        Family::FitnessDashboardTile | Family::GovernanceReportRow => {
            M5_FITNESS_GOVERNANCE_CONTROLS_ARTIFACT_REF
        }
        Family::WaiverExpiryQueueItem | Family::ReleaseGateBanner | Family::MitigationNoteCard => {
            M5_WAIVER_GATE_CONTROLS_ARTIFACT_REF
        }
        Family::ServiceOwnershipCard | Family::OnCallStrip => {
            M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_ARTIFACT_REF
        }
        Family::DecisionRightCard | Family::MilestoneDashboardRow => {
            M5_DECISION_RIGHT_MILESTONE_CONTROLS_ARTIFACT_REF
        }
    }
}

/// The one shared governance vocabulary every component keeps aligned across
/// surfaces, so no consumer invents a new badge or stale wording. The descriptors
/// in [`M5GovernanceDescriptor::REQUIRED`] must be present on every binding — the
/// track invariant that readiness, evidence freshness, waiver state, owner
/// coverage, and decision forum stay explicit everywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceDescriptor {
    /// The readiness descriptor (passing / warning / blocked / waived / stale / …).
    Readiness,
    /// The evidence-freshness descriptor.
    EvidenceFreshness,
    /// The waiver-state descriptor (active / expiring / expired).
    WaiverState,
    /// The owner-coverage descriptor (owner + backup + escalation route).
    OwnerCoverage,
    /// The decision-forum descriptor (which forum can approve the next move).
    DecisionForum,
}

impl M5GovernanceDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Readiness,
        Self::EvidenceFreshness,
        Self::WaiverState,
        Self::OwnerCoverage,
        Self::DecisionForum,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 5] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Readiness => "readiness",
            Self::EvidenceFreshness => "evidence_freshness",
            Self::WaiverState => "waiver_state",
            Self::OwnerCoverage => "owner_coverage",
            Self::DecisionForum => "decision_forum",
        }
    }
}

/// The governance evidence state a consumer renders a component under. A narrowed
/// state still keeps the descriptor vocabulary — it only discloses that the rendered
/// truth is narrowed relative to the authoritative assurance center, and which
/// governance fact drove the narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceEvidenceState {
    /// Full, fresh, provider-authoritative truth: the authoritative rendering.
    FullTruthFresh,
    /// The evidence is stale relative to the current build.
    EvidenceStale,
    /// A waiver behind the reading is expiring or has expired.
    WaiverExpiringOrExpired,
    /// Owner coverage (owner and/or backup) is missing.
    OwnerCoverageMissing,
    /// No authorized decision forum is resolved.
    ForumUnresolved,
    /// The component has not been evaluated on this build / surface.
    NotEvaluatedHere,
}

impl M5GovernanceEvidenceState {
    /// Every evidence state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullTruthFresh,
        Self::EvidenceStale,
        Self::WaiverExpiringOrExpired,
        Self::OwnerCoverageMissing,
        Self::ForumUnresolved,
        Self::NotEvaluatedHere,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullTruthFresh => "full_truth_fresh",
            Self::EvidenceStale => "evidence_stale",
            Self::WaiverExpiringOrExpired => "waiver_expiring_or_expired",
            Self::OwnerCoverageMissing => "owner_coverage_missing",
            Self::ForumUnresolved => "forum_unresolved",
            Self::NotEvaluatedHere => "not_evaluated_here",
        }
    }

    /// True when the state renders below full, fresh truth and so must disclose a
    /// self-contained narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullTruthFresh)
    }

    /// The projection mode this evidence state resolves to.
    pub const fn projection_mode(self) -> M5GovernanceProjectionMode {
        match self {
            Self::FullTruthFresh => M5GovernanceProjectionMode::FullParity,
            Self::EvidenceStale => M5GovernanceProjectionMode::StaleNarrowed,
            Self::WaiverExpiringOrExpired => M5GovernanceProjectionMode::WaiverNarrowed,
            Self::OwnerCoverageMissing => M5GovernanceProjectionMode::OwnershipNarrowed,
            Self::ForumUnresolved => M5GovernanceProjectionMode::ForumNarrowed,
            Self::NotEvaluatedHere => M5GovernanceProjectionMode::NotEvaluatedNarrowed,
        }
    }

    /// The narrow reason a narrowed state discloses, if any.
    pub const fn narrow_reason(self) -> Option<M5GovernanceNarrowReason> {
        Some(match self {
            Self::EvidenceStale => M5GovernanceNarrowReason::EvidenceStale,
            Self::WaiverExpiringOrExpired => M5GovernanceNarrowReason::WaiverExpiring,
            Self::OwnerCoverageMissing => M5GovernanceNarrowReason::OwnerCoverageMissing,
            Self::ForumUnresolved => M5GovernanceNarrowReason::ForumUnresolved,
            Self::NotEvaluatedHere => M5GovernanceNarrowReason::NotEvaluatedHere,
            Self::FullTruthFresh => return None,
        })
    }
}

/// The derived projection mode of a binding — the parity verdict a consumer renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceProjectionMode {
    /// Full descriptor parity at authoritative truth.
    FullParity,
    /// Parity preserved, evidence disclosed stale.
    StaleNarrowed,
    /// Parity preserved, waiver disclosed expiring / expired.
    WaiverNarrowed,
    /// Parity preserved, owner coverage disclosed missing.
    OwnershipNarrowed,
    /// Parity preserved, decision forum disclosed unresolved.
    ForumNarrowed,
    /// Parity preserved, evaluation disclosed not-run-here.
    NotEvaluatedNarrowed,
}

impl M5GovernanceProjectionMode {
    /// Every projection mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullParity,
        Self::StaleNarrowed,
        Self::WaiverNarrowed,
        Self::OwnershipNarrowed,
        Self::ForumNarrowed,
        Self::NotEvaluatedNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::StaleNarrowed => "stale_narrowed",
            Self::WaiverNarrowed => "waiver_narrowed",
            Self::OwnershipNarrowed => "ownership_narrowed",
            Self::ForumNarrowed => "forum_narrowed",
            Self::NotEvaluatedNarrowed => "not_evaluated_narrowed",
        }
    }

    /// True when the mode is a narrowed projection.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }
}

/// The exact reason a binding renders narrowed, so a narrow banner never reads like
/// a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceNarrowReason {
    /// The component's evidence is stale relative to the current build.
    EvidenceStale,
    /// A waiver behind the reading is expiring or has expired.
    WaiverExpiring,
    /// Owner coverage is missing.
    OwnerCoverageMissing,
    /// No authorized decision forum is resolved.
    ForumUnresolved,
    /// The component has not been evaluated here.
    NotEvaluatedHere,
}

impl M5GovernanceNarrowReason {
    /// Every narrow reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EvidenceStale,
        Self::WaiverExpiring,
        Self::OwnerCoverageMissing,
        Self::ForumUnresolved,
        Self::NotEvaluatedHere,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceStale => "evidence_stale",
            Self::WaiverExpiring => "waiver_expiring",
            Self::OwnerCoverageMissing => "owner_coverage_missing",
            Self::ForumUnresolved => "forum_unresolved",
            Self::NotEvaluatedHere => "not_evaluated_here",
        }
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::EvidenceStale => "the evidence is stale relative to the current build",
            Self::WaiverExpiring => "a waiver behind this reading is expiring or expired",
            Self::OwnerCoverageMissing => "owner coverage is missing for this lane",
            Self::ForumUnresolved => "no authorized decision forum is resolved",
            Self::NotEvaluatedHere => "this component has not been evaluated here",
        }
    }

    /// The readiness state this narrowing must never read past as a clean pass.
    pub const fn readiness_floor(self) -> M5GovernanceReadinessState {
        match self {
            Self::EvidenceStale => M5GovernanceReadinessState::EvidenceStale,
            Self::WaiverExpiring => M5GovernanceReadinessState::ExpiredWaiver,
            Self::OwnerCoverageMissing => M5GovernanceReadinessState::OwnerUnresolved,
            Self::ForumUnresolved => M5GovernanceReadinessState::ForumUnresolved,
            Self::NotEvaluatedHere => M5GovernanceReadinessState::NotEvaluated,
        }
    }

    /// The next action a reader should take to reach authoritative truth.
    pub const fn next_action(self) -> M5GovernanceNextAction {
        match self {
            Self::EvidenceStale => M5GovernanceNextAction::RefreshEvidence,
            Self::WaiverExpiring => M5GovernanceNextAction::RenewOrEscalateWaiver,
            Self::OwnerCoverageMissing => M5GovernanceNextAction::AssignOwnerAndBackup,
            Self::ForumUnresolved => M5GovernanceNextAction::RouteToAuthorizedForum,
            Self::NotEvaluatedHere => M5GovernanceNextAction::RequestEvaluation,
        }
    }
}

/// The next action named on a narrow banner, so a narrowed rendering is actionable
/// from the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceNextAction {
    /// Refresh the evidence from its canonical source.
    RefreshEvidence,
    /// Renew or escalate the expiring / expired waiver.
    RenewOrEscalateWaiver,
    /// Assign an owner and a backup with an escalation route.
    AssignOwnerAndBackup,
    /// Route the decision to an authorized forum.
    RouteToAuthorizedForum,
    /// Request an evaluation on this build / surface.
    RequestEvaluation,
}

impl M5GovernanceNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RefreshEvidence,
        Self::RenewOrEscalateWaiver,
        Self::AssignOwnerAndBackup,
        Self::RouteToAuthorizedForum,
        Self::RequestEvaluation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshEvidence => "refresh_evidence",
            Self::RenewOrEscalateWaiver => "renew_or_escalate_waiver",
            Self::AssignOwnerAndBackup => "assign_owner_and_backup",
            Self::RouteToAuthorizedForum => "route_to_authorized_forum",
            Self::RequestEvaluation => "request_evaluation",
        }
    }
}

/// The derived descriptor-parity state of a binding — whether the shared descriptor
/// vocabulary is preserved as-is or preserved with a disclosed narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceDescriptorParityState {
    /// The descriptor vocabulary is preserved at full truth.
    DescriptorsPreserved,
    /// The descriptor vocabulary is preserved, with a disclosed narrowing.
    DescriptorsDisclosedNarrowed,
}

impl M5GovernanceDescriptorParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [
        Self::DescriptorsPreserved,
        Self::DescriptorsDisclosedNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorsPreserved => "descriptors_preserved",
            Self::DescriptorsDisclosedNarrowed => "descriptors_disclosed_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5GovernanceConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceConsumerAnatomyPart {
    /// The adopted component identity.
    ComponentIdentity,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The shared descriptor set.
    DescriptorSet,
    /// The evidence-state cue.
    EvidenceStateCue,
    /// The readiness-state vocabulary.
    ReadinessVocabulary,
    /// The derived descriptor-parity verdict.
    DescriptorParityVerdict,
    /// The narrow banner (shown when narrowed).
    NarrowBanner,
}

impl M5GovernanceConsumerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::EvidenceStateCue,
        Self::ReadinessVocabulary,
        Self::DescriptorParityVerdict,
        Self::NarrowBanner,
    ];

    /// The anatomy parts every consumer projection must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::DescriptorParityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentIdentity => "component_identity",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::EvidenceStateCue => "evidence_state_cue",
            Self::ReadinessVocabulary => "readiness_vocabulary",
            Self::DescriptorParityVerdict => "descriptor_parity_verdict",
            Self::NarrowBanner => "narrow_banner",
        }
    }
}

/// A field the support / export packet carries so consumer parity is reconstructable
/// from the shared model. The fields in
/// [`M5GovernanceConsumerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceConsumerExportField {
    /// The consumer identity.
    Consumer,
    /// The adopted component family.
    ComponentFamily,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The descriptor set.
    DescriptorSet,
    /// The evidence state.
    EvidenceState,
    /// The projection mode.
    ProjectionMode,
    /// The descriptor-parity state.
    DescriptorParityState,
    /// The narrow reason (when narrowed).
    NarrowReason,
}

impl M5GovernanceConsumerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::EvidenceState,
        Self::ProjectionMode,
        Self::DescriptorParityState,
        Self::NarrowReason,
    ];

    /// The export fields every consumer export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::DescriptorParityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consumer => "consumer",
            Self::ComponentFamily => "component_family",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::EvidenceState => "evidence_state",
            Self::ProjectionMode => "projection_mode",
            Self::DescriptorParityState => "descriptor_parity_state",
            Self::NarrowReason => "narrow_reason",
        }
    }
}

/// A self-contained narrow banner: the exact reason, the descriptors that stay
/// preserved, the readiness floor the narrowing must never read past, and the next
/// action, so a narrowed rendering is understood from the banner alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceNarrowBanner {
    /// The exact narrow reason.
    pub reason: M5GovernanceNarrowReason,
    /// The next action a reader should take.
    pub next_action: M5GovernanceNextAction,
    /// The consumer the banner applies to.
    pub consumer: M5GovernanceDashboardConsumer,
    /// The component family the banner applies to.
    pub component_family: M5GovernanceDashboardComponentFamily,
    /// The readiness state this narrowing must never read past as a clean pass.
    pub readiness_floor: M5GovernanceReadinessState,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5GovernanceDescriptor>,
    /// A deterministic, self-contained headline naming the reason, the preserved
    /// descriptors, and the next action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the governance-binding resolver for one consumer/family
/// adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5GovernanceDashboardConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5GovernanceDashboardComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor
    /// so readiness, evidence freshness, waiver, owner, and decision forum stay
    /// explicit.
    pub descriptor_families: Vec<M5GovernanceDescriptor>,
    /// The governance evidence state the binding renders under.
    pub evidence_state: M5GovernanceEvidenceState,
    /// The shared readiness vocabulary this binding keeps aligned. Must be non-empty
    /// so the consumer reads the frozen readiness lexicon rather than a local one.
    pub readiness_vocab: Vec<M5GovernanceReadinessState>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved descriptor-parity / narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedGovernanceBinding {
    /// The consumer.
    pub consumer: M5GovernanceDashboardConsumer,
    /// The component family.
    pub component_family: M5GovernanceDashboardComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5GovernanceDescriptor>,
    /// The evidence state.
    pub evidence_state: M5GovernanceEvidenceState,
    /// The derived projection mode.
    pub projection_mode: M5GovernanceProjectionMode,
    /// The readiness vocabulary the binding keeps aligned.
    pub readiness_vocab: Vec<M5GovernanceReadinessState>,
    /// The derived descriptor-parity state.
    pub descriptor_parity_state: M5GovernanceDescriptorParityState,
    /// True when the binding renders under narrowed evidence.
    pub is_narrowed: bool,
    /// The narrow banner, present when narrowed.
    pub narrow_banner: Option<M5GovernanceNarrowBanner>,
}

/// Errors returned by [`resolve_governance_consumer_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5GovernanceBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// The readiness vocabulary was empty.
    EmptyReadinessVocab,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5GovernanceBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::EmptyReadinessVocab => "empty_readiness_vocab",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5GovernanceBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "governance binding error: {}", self.as_str())
    }
}

impl Error for M5GovernanceBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the track invariant that readiness,
/// evidence freshness, waiver state, owner coverage, and decision forum stay
/// explicit on every surface. The descriptor-parity state is preserved at full truth
/// and disclosed-narrowed under any narrowed evidence, and a narrowed evidence state
/// always produces a self-contained banner naming the exact reason, the readiness
/// floor it must never read past, and the next action while keeping the descriptor
/// vocabulary intact.
pub fn resolve_governance_consumer_binding(
    input: &M5GovernanceBindingInput,
) -> Result<M5ResolvedGovernanceBinding, M5GovernanceBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5GovernanceBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5GovernanceDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5GovernanceDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5GovernanceBindingError::MissingRequiredDescriptor);
        }
    }
    if input.readiness_vocab.is_empty() {
        return Err(M5GovernanceBindingError::EmptyReadinessVocab);
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5GovernanceBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.evidence_state.is_narrowed();
    let projection_mode = input.evidence_state.projection_mode();
    let descriptor_parity_state = if is_narrowed {
        M5GovernanceDescriptorParityState::DescriptorsDisclosedNarrowed
    } else {
        M5GovernanceDescriptorParityState::DescriptorsPreserved
    };

    let narrow_banner = input.evidence_state.narrow_reason().map(|reason| {
        let next_action = reason.next_action();
        let headline = format!(
            "Governance narrowed: {} — {} renders {} with {} descriptor(s) preserved; never a clean pass past `{}`; next: {}",
            reason.phrase(),
            input.consumer.as_str(),
            input.component_family.as_str(),
            input.descriptor_families.len(),
            reason.readiness_floor().as_str(),
            next_action.as_str()
        );
        M5GovernanceNarrowBanner {
            reason,
            next_action,
            consumer: input.consumer,
            component_family: input.component_family,
            readiness_floor: reason.readiness_floor(),
            preserved_descriptors: input.descriptor_families.clone(),
            headline,
        }
    });

    Ok(M5ResolvedGovernanceBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: component_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        evidence_state: input.evidence_state,
        projection_mode,
        readiness_vocab: input.readiness_vocab.clone(),
        descriptor_parity_state,
        is_narrowed,
        narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet
/// reconstructs consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceBindingCase {
    /// The resolver input.
    pub input: M5GovernanceBindingInput,
    /// The resolved truth. Must equal `resolve_governance_consumer_binding(&input)`.
    pub resolved: M5ResolvedGovernanceBinding,
}

impl M5GovernanceBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5GovernanceBindingInput) -> Self {
        let resolved =
            resolve_governance_consumer_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_governance_consumer_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the
/// consumer points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5GovernanceDashboardComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's
    /// canonical schema ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal
    /// the family's canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local
    /// re-description of its facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5GovernanceBindingCase>,
}

impl M5GovernanceComponentBinding {
    /// True when the binding points at the family's canonical refs and references the
    /// canonical family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == component_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref
                == component_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one governance-dashboard consumer bound to the
/// nine canonical component families it adopts, the shared governance vocabulary, the
/// evidence states, narrow reasons, parity states, next actions, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceConsumerRow {
    /// Governance-dashboard consumer.
    pub consumer: M5GovernanceDashboardConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5GovernanceQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 governance surface families that render / consume this projection.
    pub surface_families: Vec<M5GovernanceSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5GovernanceConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5GovernanceDescriptor>,
    /// The shared readiness vocabulary this consumer keeps aligned (non-empty).
    pub readiness_vocab: Vec<M5GovernanceReadinessState>,
    /// Evidence states this consumer distinguishes.
    pub evidence_states: Vec<M5GovernanceEvidenceState>,
    /// Projection modes this consumer distinguishes.
    pub projection_modes: Vec<M5GovernanceProjectionMode>,
    /// Descriptor-parity states this consumer distinguishes.
    pub descriptor_parity_states: Vec<M5GovernanceDescriptorParityState>,
    /// Narrow reasons this consumer names.
    pub narrow_reasons: Vec<M5GovernanceNarrowReason>,
    /// Next actions this consumer names.
    pub next_actions: Vec<M5GovernanceNextAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5GovernanceConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5GovernanceAccessibilityRoute>,
    /// Governance subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5GovernanceConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5GovernanceDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5GovernanceComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never renders waived or stale evidence as a
    /// clean pass. MUST be `false`.
    pub renders_waived_or_stale_as_clean_pass: bool,
    /// Hard invariant: this consumer never lets an ownerless or forumless blocker
    /// read as resolved. MUST be `false`.
    pub lets_ownerless_or_forumless_blocker_read_resolved: bool,
    /// Hard invariant: this consumer never hides mitigation text behind internal
    /// jargon. MUST be `false`.
    pub hides_mitigation_behind_internal_jargon: bool,
    /// Hard invariant: this consumer never re-words the governance vocabulary per
    /// surface. MUST be `false`.
    pub rewords_governance_vocabulary_per_surface: bool,
    /// Hard invariant: this consumer never invents a new dashboard-local status word.
    /// MUST be `false`.
    pub invents_new_dashboard_local_status: bool,
}

impl M5GovernanceConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5GovernanceConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5GovernanceConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5GovernanceConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5GovernanceConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5GovernanceDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5GovernanceDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5GovernanceComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5GovernanceDashboardComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.renders_waived_or_stale_as_clean_pass
            && !self.lets_ownerless_or_forumless_blocker_read_resolved
            && !self.hides_mitigation_behind_internal_jargon
            && !self.rewords_governance_vocabulary_per_surface
            && !self.invents_new_dashboard_local_status
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceConsumerVocabularySet {
    /// Governance-dashboard-consumer tokens.
    pub consumers: Vec<String>,
    /// Component-family tokens (reused from the frozen matrix).
    pub component_families: Vec<String>,
    /// Descriptor tokens.
    pub descriptors: Vec<String>,
    /// Readiness-state tokens (reused from the frozen matrix).
    pub readiness_states: Vec<String>,
    /// Evidence-state tokens.
    pub evidence_states: Vec<String>,
    /// Projection-mode tokens.
    pub projection_modes: Vec<String>,
    /// Narrow-reason tokens.
    pub narrow_reasons: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Descriptor-parity-state tokens.
    pub descriptor_parity_states: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5GovernanceConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5GovernanceDashboardConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5GovernanceDashboardComponentFamily::ALL, |v| v.as_str()),
            descriptors: tokens(&M5GovernanceDescriptor::ALL, |v| v.as_str()),
            readiness_states: tokens(&M5GovernanceReadinessState::ALL, |v| v.as_str()),
            evidence_states: tokens(&M5GovernanceEvidenceState::ALL, |v| v.as_str()),
            projection_modes: tokens(&M5GovernanceProjectionMode::ALL, |v| v.as_str()),
            narrow_reasons: tokens(&M5GovernanceNarrowReason::ALL, |v| v.as_str()),
            next_actions: tokens(&M5GovernanceNextAction::ALL, |v| v.as_str()),
            descriptor_parity_states: tokens(&M5GovernanceDescriptorParityState::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5GovernanceConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5GovernanceConsumerExportField::ALL, |v| v.as_str()),
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
pub struct M5GovernanceConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The governance vocabulary is shared, never re-worded per surface.
    pub governance_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new dashboard-local status word.
    pub no_consumer_invents_new_status: bool,
    /// Readiness, evidence freshness, waiver, owner, and forum stay explicit
    /// everywhere.
    pub descriptors_explicit_on_every_surface: bool,
    /// No consumer renders waived or stale evidence as a clean pass.
    pub waived_or_stale_never_reads_clean: bool,
    /// A narrowed rendering always shows a self-contained narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and next action, never a generic note.
    pub banner_names_exact_reason_and_next_action: bool,
    /// The support / export packet reconstructs consumer parity.
    pub support_export_reconstructs_consumer_parity: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceConsumerProjection {
    /// Assurance, release, operator, shiproom, support, About/help, docs, and CLI
    /// consumers all adopt the shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The readiness descriptor reads a single canonical source.
    pub readiness_reads_single_source: bool,
    /// The evidence-freshness descriptor reads a single canonical source.
    pub evidence_freshness_reads_single_source: bool,
    /// The waiver-state descriptor reads a single canonical source.
    pub waiver_state_reads_single_source: bool,
    /// The owner-coverage descriptor reads a single canonical source.
    pub owner_coverage_reads_single_source: bool,
    /// The decision-forum descriptor reads a single canonical source.
    pub decision_forum_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting consumer audit.
    pub consumer_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5GovernanceComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5GovernanceComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5GovernanceConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5GovernanceConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5GovernanceConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5GovernanceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5GovernanceConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5GovernanceConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 governance-dashboard-component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceComponentConsumerPacket {
    /// Record kind; must equal [`M5_GOVERNANCE_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_GOVERNANCE_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5GovernanceConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5GovernanceConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5GovernanceConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5GovernanceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5GovernanceConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5GovernanceConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5GovernanceComponentConsumerPacket {
    /// Builds an M5 governance-dashboard-component-consumer packet from stable-lane input.
    pub fn new(input: M5GovernanceComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_GOVERNANCE_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_GOVERNANCE_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
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

    /// Validates the M5 governance-dashboard-component-consumer invariants.
    pub fn validate(&self) -> Vec<M5GovernanceConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_GOVERNANCE_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5GovernanceConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_GOVERNANCE_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5GovernanceConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5GovernanceConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_docs_help_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 governance-dashboard-component consumer packet serializes"),
        ) {
            violations.push(M5GovernanceConsumerViolation::RawMaterialInExport);
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
            .expect("m5 governance-dashboard-component consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,qualification,owner,adopted_families,evidence_states,descriptor_parity_states,narrow_reasons,export_fields,binding_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.component_bindings, |b| b.component_family.as_str()),
                join_tokens(&row.evidence_states, |v| v.as_str()),
                join_tokens(&row.descriptor_parity_states, |v| v.as_str()),
                join_tokens(&row.narrow_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.component_bindings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Governance-Dashboard Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Governance-dashboard consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Component families: {}\n",
            self.vocabulary_set.component_families.join(", ")
        ));
        out.push_str(&format!(
            "- Descriptors: {}\n",
            self.vocabulary_set.descriptors.join(", ")
        ));
        out.push_str(&format!(
            "- Evidence states: {}\n",
            self.vocabulary_set.evidence_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Governance-dashboard consumers\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Adopted families: {}\n",
                row.component_bindings.len()
            ));
            for binding in &row.component_bindings {
                out.push_str(&format!(
                    "    - `{}` → `{}` ({} worked binding(s))\n",
                    binding.component_family.as_str(),
                    binding.canonical_schema_ref,
                    binding.example_bindings.len()
                ));
                for case in &binding.example_bindings {
                    let banner = match &case.resolved.narrow_banner {
                        Some(banner) => banner.reason.as_str(),
                        None => "full",
                    };
                    out.push_str(&format!(
                        "      - `{}` → `{}` (banner `{}`)\n",
                        case.resolved.evidence_state.as_str(),
                        case.resolved.descriptor_parity_state.as_str(),
                        banner
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 governance-dashboard-component-consumer export.
#[derive(Debug)]
pub enum M5GovernanceConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5GovernanceConsumerViolation>),
}

impl fmt::Display for M5GovernanceConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 governance-dashboard-component consumer export parse failed: {error}"
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
                    "m5 governance-dashboard-component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5GovernanceConsumerArtifactError {}

/// Validation failures emitted by [`M5GovernanceComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5GovernanceConsumerViolation {
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
    /// A required governance-dashboard consumer is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A consumer row does not keep every required descriptor.
    RequiredDescriptorMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no readiness vocabulary.
    ReadinessVocabMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no component bindings.
    ComponentBindingMissing,
    /// A component binding does not point to its canonical family.
    CanonicalRefMismatch,
    /// A component binding declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// A required component family is never adopted, or is adopted by only one
    /// consumer (reuse across surfaces unproven).
    ComponentFamilyReuseUnproven,
    /// No worked binding proves a narrowed rendering with a self-contained banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-scope rendering with preserved parity and no
    /// banner.
    ScopePreservedUnproven,
    /// A docs/help consumer does not reference the canonical component schema.
    DocsHelpReferenceMissing,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
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

impl M5GovernanceConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::RequiredDescriptorMissing => "required_descriptor_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ReadinessVocabMissing => "readiness_vocab_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ComponentBindingMissing => "component_binding_missing",
            Self::CanonicalRefMismatch => "canonical_ref_mismatch",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ComponentFamilyReuseUnproven => "component_family_reuse_unproven",
            Self::NarrowingDisclosureUnproven => "narrowing_disclosure_unproven",
            Self::ScopePreservedUnproven => "scope_preserved_unproven",
            Self::DocsHelpReferenceMissing => "docs_help_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 governance-dashboard-component-consumer export.
pub fn current_stable_m5_governance_component_consumer_export(
) -> Result<M5GovernanceComponentConsumerPacket, M5GovernanceConsumerArtifactError> {
    let packet: M5GovernanceComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-governance-dashboard-component-consumer-proof/support_export.json"
    )))
    .map_err(M5GovernanceConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5GovernanceConsumerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5GovernanceComponentConsumerPacket,
    violations: &mut Vec<M5GovernanceConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_GOVERNANCE_CONSUMER_SCHEMA_REF,
        M5_GOVERNANCE_CONSUMER_DOC_REF,
        M5_GOVERNANCE_CONSUMER_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_CONSUMER_MATRIX_DOC_REF,
        M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_REF,
        M5_WAIVER_GATE_CONTROLS_SCHEMA_REF,
        M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_REF,
        M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5GovernanceConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5GovernanceComponentConsumerPacket,
    violations: &mut Vec<M5GovernanceConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5GovernanceConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5GovernanceComponentConsumerPacket,
    violations: &mut Vec<M5GovernanceConsumerViolation>,
) {
    let present: BTreeSet<M5GovernanceDashboardConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5GovernanceDashboardConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5GovernanceConsumerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.evidence_states.is_empty()
            || row.projection_modes.is_empty()
            || row.descriptor_parity_states.is_empty()
            || row.narrow_reasons.is_empty()
            || row.next_actions.is_empty()
        {
            violations.push(M5GovernanceConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5GovernanceConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5GovernanceConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5GovernanceConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.readiness_vocab.is_empty() {
            violations.push(M5GovernanceConsumerViolation::ReadinessVocabMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5GovernanceAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5GovernanceConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5GovernanceConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5GovernanceConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5GovernanceConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5GovernanceConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5GovernanceConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5GovernanceConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5GovernanceConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5GovernanceConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct
/// consumers — the acceptance-criterion proof that the families are reusable
/// components rather than one governance pipeline plus a few admin-only dashboards.
fn validate_family_reuse(
    packet: &M5GovernanceComponentConsumerPacket,
    violations: &mut Vec<M5GovernanceConsumerViolation>,
) {
    for family in M5GovernanceDashboardComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5GovernanceConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering
/// whose banner carries a specific reason, a next action, and a non-empty set of
/// preserved descriptors — the acceptance-criterion example that governance
/// components stay truthful when evidence or ownership state is stale.
fn validate_narrowing_disclosure(
    packet: &M5GovernanceComponentConsumerPacket,
    violations: &mut Vec<M5GovernanceConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        case.resolved.is_narrowed
            && case.resolved.narrow_banner.as_ref().is_some_and(|banner| {
                !banner.headline.trim().is_empty() && !banner.preserved_descriptors.is_empty()
            })
    });
    if !proven {
        violations.push(M5GovernanceConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-truth rendering
/// with preserved parity and no banner — the acceptance-criterion example that
/// full-truth consumers keep the descriptor vocabulary without a spurious narrowing
/// note.
fn validate_scope_preserved(
    packet: &M5GovernanceComponentConsumerPacket,
    violations: &mut Vec<M5GovernanceConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.narrow_banner.is_none()
            && case.resolved.descriptor_parity_state
                == M5GovernanceDescriptorParityState::DescriptorsPreserved
    });
    if !proven {
        violations.push(M5GovernanceConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every docs/help consumer must reference the canonical component schema for each
/// family it adopts — the acceptance-criterion that docs/help prose can never drift
/// from the product truth.
fn validate_docs_help_reference(
    packet: &M5GovernanceComponentConsumerPacket,
    violations: &mut Vec<M5GovernanceConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_docs_or_help() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5GovernanceComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5GovernanceConsumerViolation::DocsHelpReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5GovernanceComponentConsumerPacket,
    violations: &mut Vec<M5GovernanceConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.governance_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_status,
        review.descriptors_explicit_on_every_surface,
        review.waived_or_stale_never_reads_clean,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_next_action,
        review.support_export_reconstructs_consumer_parity,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5GovernanceConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5GovernanceComponentConsumerPacket,
    violations: &mut Vec<M5GovernanceConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.readiness_reads_single_source,
        projection.evidence_freshness_reads_single_source,
        projection.waiver_state_reads_single_source,
        projection.owner_coverage_reads_single_source,
        projection.decision_forum_reads_single_source,
    ] {
        if !ok {
            violations.push(M5GovernanceConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5GovernanceComponentConsumerPacket,
    violations: &mut Vec<M5GovernanceConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5GovernanceConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5GovernanceComponentConsumerPacket,
    violations: &mut Vec<M5GovernanceConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.consumer_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5GovernanceConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5GovernanceComponentConsumerPacket,
) -> impl Iterator<Item = &M5GovernanceBindingCase> {
    packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|binding| binding.example_bindings.iter())
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
