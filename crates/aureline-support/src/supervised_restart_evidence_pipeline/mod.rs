//! Supervised-restart evidence pipeline: one exportable packet that captures
//! restart lineage, fault-domain identity, host-lane identity, strike budget,
//! quarantine state, and reattach / no-rerun policy across local, remote,
//! extension, debug, and notebook fault domains.
//!
//! This module consumes the runtime topology inspector and the support
//! fault-domain view packet, then mints a [`SupervisedRestartEvidencePacket`]
//! that unifies restart evidence for Support Center, Diagnostics Center,
//! shiproom proof, support bundles, and known-limits publication.
//!
//! ## What this module owns
//!
//! - The [`SupervisedRestartEvidencePacket`] record — the canonical truth
//!   model for supervised restart evidence. It mirrors the boundary schema at
//!   `/schemas/support/supervised-restart-evidence-pipeline.schema.json`.
//! - The [`RestartLineageEntry`] row — one chronological restart event with
//!   fault-domain identity, host-lane ref, strike budget, and trigger class.
//! - The [`HostLaneIdentityRecord`] row — stable identity for a host lane
//!   including boundary badges, fault-domain ownership, and mutating capability.
//! - The [`SupervisedRestartReviewDecision`] row — the explicit review
//!   decision for a reattach or restart event (current, review required,
//!   reapproval required, rerun required, blocked).
//! - The [`NoRerunPolicyRecord`] row — the no-rerun policy that prevents
//!   mutating or externally routed lanes from silently re-running after crash,
//!   restart, or reattach.
//! - The [`FaultDomainRestartSummary`] row — per-fault-domain summary of
//!   restart count, quarantine state, and review posture.
//!
//! ## Invariants this module enforces
//!
//! - Every mutating or externally routed lane MUST carry a
//!   [`NoRerunPolicyRecord`] that requires explicit review or reapproval.
//! - Non-mutating lanes MAY rehydrate safely but still disclose their restart
//!   lineage and any stale or partial truth.
//! - The packet never embeds raw secret-bearing material; all evidence is
//!   metadata-only with opaque refs.
//! - Restart storms and quarantine are never hidden under generic reconnect
//!   copy; the packet surfaces explicit degraded state.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::FaultDomainViewPacket;

/// Frozen schema version for supervised-restart evidence records.
pub const SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the supervised-restart evidence packet.
pub const SUPERVISED_RESTART_EVIDENCE_PACKET_RECORD_KIND: &str =
    "supervised_restart_evidence_packet";

/// Stable record-kind tag for one restart lineage entry.
pub const RESTART_LINEAGE_ENTRY_RECORD_KIND: &str = "restart_lineage_entry_record";

/// Stable record-kind tag for one host-lane identity record.
pub const HOST_LANE_IDENTITY_RECORD_KIND: &str = "host_lane_identity_record";

/// Stable record-kind tag for one supervised restart review decision.
pub const SUPERVISED_RESTART_REVIEW_DECISION_RECORD_KIND: &str =
    "supervised_restart_review_decision_record";

/// Stable record-kind tag for one no-rerun policy record.
pub const NO_RERUN_POLICY_RECORD_KIND: &str = "no_rerun_policy_record";

/// Stable record-kind tag for one fault-domain restart summary.
pub const FAULT_DOMAIN_RESTART_SUMMARY_RECORD_KIND: &str = "fault_domain_restart_summary_record";

/// Repo-relative path of the boundary schema this module mirrors.
pub const SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF: &str =
    "schemas/support/supervised-restart-evidence-pipeline.schema.json";

/// Reviewer doc ref quoted verbatim by every emitted packet.
pub const SUPERVISED_RESTART_EVIDENCE_PIPELINE_DOC_REF: &str =
    "docs/help/support/supervised-restart-evidence-pipeline.md";

/// Closed fault-domain vocabulary covering the five required domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartDomainClass {
    /// Local shell and workspace lanes.
    Local,
    /// Remote workspace agent and connector lanes.
    Remote,
    /// Extension sandbox and tool host lanes.
    Extension,
    /// Debug adapter and task adapter lanes.
    Debug,
    /// Notebook and REPL kernel lanes.
    Notebook,
}

impl RestartDomainClass {
    /// All restart domain classes in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Local,
        Self::Remote,
        Self::Extension,
        Self::Debug,
        Self::Notebook,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Extension => "extension",
            Self::Debug => "debug",
            Self::Notebook => "notebook",
        }
    }

    /// Plain-language label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Local => "Local",
            Self::Remote => "Remote",
            Self::Extension => "Extension",
            Self::Debug => "Debug",
            Self::Notebook => "Notebook",
        }
    }

    /// Returns true when this domain is externally routed (remote or managed).
    pub const fn is_externally_routed(self) -> bool {
        matches!(self, Self::Remote)
    }

    /// Returns true when this domain can perform mutating execution.
    pub const fn can_mutate(self) -> bool {
        matches!(self, Self::Debug | Self::Notebook | Self::Remote)
    }
}

/// Closed restart-trigger vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartTriggerClass {
    /// Host process crashed.
    HostCrash,
    /// Host process was explicitly restarted by the user.
    UserInitiatedRestart,
    /// Host process was restarted by the supervisor inside budget.
    SupervisorRestart,
    /// Host disconnected and is attempting reattach.
    ReattachAttempt,
    /// Host entered quarantine after budget exhaustion.
    QuarantineEntered,
    /// Host was disabled by policy or emergency action.
    PolicyDisable,
}

impl RestartTriggerClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostCrash => "host_crash",
            Self::UserInitiatedRestart => "user_initiated_restart",
            Self::SupervisorRestart => "supervisor_restart",
            Self::ReattachAttempt => "reattach_attempt",
            Self::QuarantineEntered => "quarantine_entered",
            Self::PolicyDisable => "policy_disable",
        }
    }
}

/// Closed restart-budget-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartBudgetToken {
    /// Within automatic restart budget.
    WithinBudget,
    /// Warning: one strike from exhaustion.
    BudgetWarning,
    /// Budget exhausted; no more automatic restarts.
    BudgetExhausted,
    /// Quarantined after budget or policy escalation.
    Quarantined,
    /// No automatic restart budget configured.
    NoAutomaticRestart,
    /// Reattach review required before current.
    ReattachReviewRequired,
    /// Disabled.
    Disabled,
}

impl RestartBudgetToken {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinBudget => "within_budget",
            Self::BudgetWarning => "budget_warning",
            Self::BudgetExhausted => "budget_exhausted",
            Self::Quarantined => "quarantined",
            Self::NoAutomaticRestart => "no_automatic_restart",
            Self::ReattachReviewRequired => "reattach_review_required",
            Self::Disabled => "disabled",
        }
    }
}

/// Closed review-decision vocabulary for supervised restarts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupervisedRestartDecisionClass {
    /// Lane is current; no review required.
    Current,
    /// Non-mutating lane auto-reattached but stale results need refresh.
    AutoReattachedStaleRefresh,
    /// Review is required before claiming current.
    ReviewRequired,
    /// Reapproval is required before claiming current.
    ReapprovalRequired,
    /// Explicit rerun is required.
    RerunRequired,
    /// Manual repair blocks reattach.
    BlockedManualRepair,
}

impl SupervisedRestartDecisionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::AutoReattachedStaleRefresh => "auto_reattached_stale_refresh",
            Self::ReviewRequired => "review_required",
            Self::ReapprovalRequired => "reapproval_required",
            Self::RerunRequired => "rerun_required",
            Self::BlockedManualRepair => "blocked_manual_repair",
        }
    }

    /// True when the lane may be claimed as current.
    pub const fn allows_current_claim(self) -> bool {
        matches!(self, Self::Current | Self::AutoReattachedStaleRefresh)
    }

    /// True when explicit review or reapproval is required.
    pub const fn requires_explicit_review(self) -> bool {
        matches!(
            self,
            Self::ReviewRequired
                | Self::ReapprovalRequired
                | Self::RerunRequired
                | Self::BlockedManualRepair
        )
    }
}

/// Closed no-rerun-policy vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoRerunPolicyClass {
    /// Lane may safely rehydrate without explicit review.
    SafeRehydrate,
    /// Lane requires explicit rerun confirmation.
    ExplicitRerunRequired,
    /// Lane requires reapproval before rerun.
    ReapprovalRequired,
    /// Lane is blocked from rerun until manual repair.
    BlockedUntilRepair,
}

impl NoRerunPolicyClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeRehydrate => "safe_rehydrate",
            Self::ExplicitRerunRequired => "explicit_rerun_required",
            Self::ReapprovalRequired => "reapproval_required",
            Self::BlockedUntilRepair => "blocked_until_repair",
        }
    }

    /// True when the policy forbids silent rerun.
    pub const fn forbids_silent_rerun(self) -> bool {
        !matches!(self, Self::SafeRehydrate)
    }
}

/// One chronological restart lineage entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartLineageEntry {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable entry id.
    pub entry_id: String,
    /// Fault domain.
    pub domain: RestartDomainClass,
    /// Stable domain token.
    pub domain_token: String,
    /// Host lane reference.
    pub host_lane_ref: String,
    /// Host family label.
    pub host_family_label: String,
    /// UTC timestamp of the restart event.
    pub timestamp: String,
    /// Restart trigger.
    pub trigger: RestartTriggerClass,
    /// Stable trigger token.
    pub trigger_token: String,
    /// Strike count after this event.
    pub strike_count: u32,
    /// Restart budget in the window.
    pub restart_budget_in_window: u32,
    /// Restart budget state.
    pub budget_state: RestartBudgetToken,
    /// Stable budget-state token.
    pub budget_state_token: String,
    /// Whether the lane is quarantined.
    pub quarantined: bool,
    /// Opaque quarantine reference, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quarantine_ref: Option<String>,
    /// Exact-build identifier at the time of the event.
    pub build_id: String,
    /// Export-safe summary.
    pub summary: String,
}

/// Host-lane identity record for restart evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostLaneIdentityRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Host lane reference.
    pub host_lane_ref: String,
    /// Host family label.
    pub host_family_label: String,
    /// Fault-domain id.
    pub fault_domain_id: String,
    /// Fault-domain class token.
    pub fault_domain_token: String,
    /// Boundary badge tokens.
    pub boundary_badge_tokens: Vec<String>,
    /// Whether the lane can perform mutating work.
    pub can_mutate: bool,
    /// Whether the lane is externally routed.
    pub externally_routed: bool,
    /// Current health token.
    pub health_token: String,
    /// Restart budget reference.
    pub restart_budget_ref: String,
    /// Affected capability tokens.
    pub affected_capability_tokens: Vec<String>,
    /// Preserved checkpoint references.
    pub preserved_checkpoint_refs: Vec<String>,
    /// Partial-truth result references.
    pub partial_truth_result_refs: Vec<String>,
}

/// Supervised restart review decision for one lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupervisedRestartReviewDecision {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Decision id.
    pub decision_id: String,
    /// Host lane reference.
    pub host_lane_ref: String,
    /// Decision class.
    pub decision: SupervisedRestartDecisionClass,
    /// Stable decision token.
    pub decision_token: String,
    /// Whether the lane may claim current status.
    pub current_lane_accepted: bool,
    /// Whether explicit review is required.
    pub explicit_review_required: bool,
    /// Preserved state references.
    pub preserved_state_refs: Vec<String>,
    /// Lost state references.
    pub lost_state_refs: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

/// No-rerun policy record for one lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoRerunPolicyRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Policy id.
    pub policy_id: String,
    /// Host lane reference.
    pub host_lane_ref: String,
    /// Policy class.
    pub policy: NoRerunPolicyClass,
    /// Stable policy token.
    pub policy_token: String,
    /// Whether silent rerun is forbidden.
    pub forbids_silent_rerun: bool,
    /// Reason summary.
    pub reason: String,
}

/// Per-fault-domain restart summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultDomainRestartSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Summary id.
    pub summary_id: String,
    /// Fault domain.
    pub domain: RestartDomainClass,
    /// Stable domain token.
    pub domain_token: String,
    /// Number of restart lineage entries in this domain.
    pub restart_entry_count: u32,
    /// Number of lanes in this domain.
    pub lane_count: u32,
    /// Number of quarantined lanes.
    pub quarantined_lane_count: u32,
    /// Number of lanes requiring explicit review.
    pub review_required_lane_count: u32,
    /// Number of mutating lanes.
    pub mutating_lane_count: u32,
    /// Number of externally routed lanes.
    pub externally_routed_lane_count: u32,
    /// Whether any lane in this domain blocks a healthy claim.
    pub blocks_healthy_claim: bool,
}

/// Validation violation emitted by the packet validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupervisedRestartViolation {
    /// Field or path that failed validation.
    pub path: String,
    /// Subject record reference.
    pub subject_ref: String,
    /// Export-safe validation summary.
    pub summary: String,
}

fn push_violation(
    violations: &mut Vec<SupervisedRestartViolation>,
    path: impl Into<String>,
    subject_ref: impl Into<String>,
    summary: impl Into<String>,
) {
    violations.push(SupervisedRestartViolation {
        path: path.into(),
        subject_ref: subject_ref.into(),
        summary: summary.into(),
    });
}

/// The top-level supervised-restart evidence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupervisedRestartEvidencePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Exact-build identifier.
    pub build_id: String,
    /// Doc ref quoted verbatim.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Restart lineage entries.
    pub lineage_entries: Vec<RestartLineageEntry>,
    /// Host-lane identity records.
    pub host_lane_identities: Vec<HostLaneIdentityRecord>,
    /// Supervised restart review decisions.
    pub review_decisions: Vec<SupervisedRestartReviewDecision>,
    /// No-rerun policy records.
    pub no_rerun_policies: Vec<NoRerunPolicyRecord>,
    /// Per-fault-domain restart summaries.
    pub domain_summaries: Vec<FaultDomainRestartSummary>,
    /// Count of lineage entries.
    pub lineage_entry_count: u32,
    /// Count of host lanes.
    pub host_lane_count: u32,
    /// Count of lanes requiring explicit review.
    pub review_required_lane_count: u32,
    /// Count of mutating lanes.
    pub mutating_lane_count: u32,
    /// Count of externally routed lanes.
    pub externally_routed_lane_count: u32,
    /// Count of quarantined lanes.
    pub quarantined_lane_count: u32,
    /// Whether the packet is metadata-only and export-safe.
    pub export_safe: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl SupervisedRestartEvidencePacket {
    /// Builds a packet from a fault-domain view packet and exact-build identity.
    pub fn from_fault_domain_view_packet(
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        build_id: impl Into<String>,
        fault_packet: &FaultDomainViewPacket,
    ) -> Self {
        let build_id = build_id.into();
        let mut lineage_entries = Vec::new();
        let mut host_lane_identities = Vec::new();
        let mut review_decisions = Vec::new();
        let mut no_rerun_policies = Vec::new();
        let mut domain_summaries = Vec::new();

        // Build per-domain accumulator state.
        let mut domain_restart_counts: std::collections::BTreeMap<RestartDomainClass, u32> =
            std::collections::BTreeMap::new();
        let mut domain_lane_counts: std::collections::BTreeMap<RestartDomainClass, u32> =
            std::collections::BTreeMap::new();
        let mut domain_quarantine_counts: std::collections::BTreeMap<RestartDomainClass, u32> =
            std::collections::BTreeMap::new();
        let mut domain_review_counts: std::collections::BTreeMap<RestartDomainClass, u32> =
            std::collections::BTreeMap::new();
        let mut domain_mutating_counts: std::collections::BTreeMap<RestartDomainClass, u32> =
            std::collections::BTreeMap::new();
        let mut domain_external_counts: std::collections::BTreeMap<RestartDomainClass, u32> =
            std::collections::BTreeMap::new();
        let mut domain_blocks_healthy: std::collections::BTreeMap<RestartDomainClass, bool> =
            std::collections::BTreeMap::new();

        for row in &fault_packet.rows {
            let domain = domain_from_fault_domain_token(&row.fault_domain_token);
            let can_mutate = domain.can_mutate();
            let externally_routed = domain.is_externally_routed();
            let quarantined = row.restart_budget_state_token == "quarantined";
            let review_required = row.reattach_decision_token.as_deref()
                == Some("reapproval_required")
                || row.restart_budget_state_token == "reattach_review_required";
            let blocks_healthy = row.health_token == "quarantined"
                || row.health_token == "crash_loop"
                || row.health_token == "disabled"
                || row.health_token == "disconnected";

            *domain_lane_counts.entry(domain).or_insert(0) += 1;
            *domain_quarantine_counts.entry(domain).or_insert(0) += if quarantined { 1 } else { 0 };
            *domain_review_counts.entry(domain).or_insert(0) += if review_required { 1 } else { 0 };
            *domain_mutating_counts.entry(domain).or_insert(0) += if can_mutate { 1 } else { 0 };
            *domain_external_counts.entry(domain).or_insert(0) +=
                if externally_routed { 1 } else { 0 };
            let prev = domain_blocks_healthy.entry(domain).or_insert(false);
            *prev = *prev || blocks_healthy;

            // Host lane identity.
            host_lane_identities.push(HostLaneIdentityRecord {
                record_kind: HOST_LANE_IDENTITY_RECORD_KIND.to_owned(),
                schema_version: SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_VERSION,
                host_lane_ref: row.host_lane_ref.clone(),
                host_family_label: row.host_family_label.clone(),
                fault_domain_id: row.fault_domain_id.clone(),
                fault_domain_token: row.fault_domain_token.clone(),
                boundary_badge_tokens: Vec::new(),
                can_mutate,
                externally_routed,
                health_token: row.health_token.clone(),
                restart_budget_ref: row.restart_budget_ref.clone(),
                affected_capability_tokens: row.affected_capability_tokens.clone(),
                preserved_checkpoint_refs: row.preserved_checkpoint_refs.clone(),
                partial_truth_result_refs: row.partial_truth_result_refs.clone(),
            });

            // Lineage entry from the row's current state.
            let trigger = if quarantined {
                RestartTriggerClass::QuarantineEntered
            } else if row.restart_strike_count > 0 {
                RestartTriggerClass::SupervisorRestart
            } else {
                RestartTriggerClass::ReattachAttempt
            };
            let budget_state = budget_state_from_token(&row.restart_budget_state_token);
            lineage_entries.push(RestartLineageEntry {
                record_kind: RESTART_LINEAGE_ENTRY_RECORD_KIND.to_owned(),
                schema_version: SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_VERSION,
                entry_id: format!(
                    "lineage:{}:{}",
                    row.host_lane_ref, fault_packet.generated_at
                ),
                domain,
                domain_token: domain.as_str().to_owned(),
                host_lane_ref: row.host_lane_ref.clone(),
                host_family_label: row.host_family_label.clone(),
                timestamp: fault_packet.generated_at.clone(),
                trigger,
                trigger_token: trigger.as_str().to_owned(),
                strike_count: row.restart_strike_count,
                restart_budget_in_window: row.restart_budget_in_window,
                budget_state,
                budget_state_token: budget_state.as_str().to_owned(),
                quarantined,
                quarantine_ref: row.crash_banner_ref.clone(),
                build_id: build_id.to_string(),
                summary: format!(
                    "{} {} restart strikes {}/{} budget={}",
                    row.host_family_label,
                    row.host_lane_ref,
                    row.restart_strike_count,
                    row.restart_budget_in_window,
                    budget_state.as_str(),
                ),
            });

            // Review decision.
            let decision = if let Some(decision_token) = &row.reattach_decision_token {
                decision_class_from_token(decision_token)
            } else if quarantined {
                SupervisedRestartDecisionClass::BlockedManualRepair
            } else if row.restart_budget_state_token == "reattach_review_required" {
                SupervisedRestartDecisionClass::ReviewRequired
            } else if can_mutate && row.restart_strike_count > 0 {
                SupervisedRestartDecisionClass::RerunRequired
            } else {
                SupervisedRestartDecisionClass::Current
            };
            review_decisions.push(SupervisedRestartReviewDecision {
                record_kind: SUPERVISED_RESTART_REVIEW_DECISION_RECORD_KIND.to_owned(),
                schema_version: SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_VERSION,
                decision_id: format!(
                    "decision:{}:{}",
                    row.host_lane_ref, fault_packet.generated_at
                ),
                host_lane_ref: row.host_lane_ref.clone(),
                decision,
                decision_token: decision.as_str().to_owned(),
                current_lane_accepted: decision.allows_current_claim(),
                explicit_review_required: decision.requires_explicit_review(),
                preserved_state_refs: row.preserved_checkpoint_refs.clone(),
                lost_state_refs: Vec::new(),
                summary: format!(
                    "{} decision={} review_required={}",
                    row.host_lane_ref,
                    decision.as_str(),
                    decision.requires_explicit_review()
                ),
            });

            // No-rerun policy.
            let policy = if quarantined || blocks_healthy {
                NoRerunPolicyClass::BlockedUntilRepair
            } else if can_mutate || externally_routed {
                NoRerunPolicyClass::ReapprovalRequired
            } else if row.restart_strike_count > 0 {
                NoRerunPolicyClass::ExplicitRerunRequired
            } else {
                NoRerunPolicyClass::SafeRehydrate
            };
            no_rerun_policies.push(NoRerunPolicyRecord {
                record_kind: NO_RERUN_POLICY_RECORD_KIND.to_owned(),
                schema_version: SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_VERSION,
                policy_id: format!("policy:{}:{}", row.host_lane_ref, fault_packet.generated_at),
                host_lane_ref: row.host_lane_ref.clone(),
                policy,
                policy_token: policy.as_str().to_owned(),
                forbids_silent_rerun: policy.forbids_silent_rerun(),
                reason: format!(
                    "domain={} mutate={} external={} strikes={}",
                    domain.as_str(),
                    can_mutate,
                    externally_routed,
                    row.restart_strike_count
                ),
            });
        }

        for domain in RestartDomainClass::ALL {
            let restart_entry_count = domain_restart_counts.entry(domain).or_insert(0);
            // Each lane gets one lineage entry in this builder, so count lanes.
            *restart_entry_count = domain_lane_counts.get(&domain).copied().unwrap_or(0);
            domain_summaries.push(FaultDomainRestartSummary {
                record_kind: FAULT_DOMAIN_RESTART_SUMMARY_RECORD_KIND.to_owned(),
                schema_version: SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_VERSION,
                summary_id: format!("summary:{}:{}", domain.as_str(), fault_packet.generated_at),
                domain,
                domain_token: domain.as_str().to_owned(),
                restart_entry_count: *restart_entry_count,
                lane_count: domain_lane_counts.get(&domain).copied().unwrap_or(0),
                quarantined_lane_count: domain_quarantine_counts.get(&domain).copied().unwrap_or(0),
                review_required_lane_count: domain_review_counts.get(&domain).copied().unwrap_or(0),
                mutating_lane_count: domain_mutating_counts.get(&domain).copied().unwrap_or(0),
                externally_routed_lane_count: domain_external_counts
                    .get(&domain)
                    .copied()
                    .unwrap_or(0),
                blocks_healthy_claim: domain_blocks_healthy.get(&domain).copied().unwrap_or(false),
            });
        }

        let lineage_entry_count = lineage_entries.len() as u32;
        let host_lane_count = host_lane_identities.len() as u32;
        let review_required_lane_count = review_decisions
            .iter()
            .filter(|d| d.explicit_review_required)
            .count() as u32;
        let mutating_lane_count =
            host_lane_identities.iter().filter(|l| l.can_mutate).count() as u32;
        let externally_routed_lane_count = host_lane_identities
            .iter()
            .filter(|l| l.externally_routed)
            .count() as u32;
        let quarantined_lane_count = no_rerun_policies
            .iter()
            .filter(|p| p.policy == NoRerunPolicyClass::BlockedUntilRepair)
            .count() as u32;

        Self {
            record_kind: SUPERVISED_RESTART_EVIDENCE_PACKET_RECORD_KIND.to_owned(),
            schema_version: SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            workspace_id: fault_packet.workspace_id.clone(),
            generated_at: generated_at.into(),
            build_id: build_id.into(),
            doc_ref: SUPERVISED_RESTART_EVIDENCE_PIPELINE_DOC_REF.to_owned(),
            schema_ref: SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
            lineage_entries,
            host_lane_identities,
            review_decisions,
            no_rerun_policies,
            domain_summaries,
            lineage_entry_count,
            host_lane_count,
            review_required_lane_count,
            mutating_lane_count,
            externally_routed_lane_count,
            quarantined_lane_count,
            export_safe: true,
            export_safe_summary:
                "Supervised-restart evidence packet is metadata-only and preserves host-lane restart lineage, review decisions, and no-rerun policy."
                    .to_owned(),
        }
    }

    /// Validates the packet invariants.
    pub fn validate(&self) -> Vec<SupervisedRestartViolation> {
        let mut violations = Vec::new();
        if self.record_kind != SUPERVISED_RESTART_EVIDENCE_PACKET_RECORD_KIND {
            push_violation(
                &mut violations,
                "record_kind",
                &self.packet_id,
                "record_kind must be supervised_restart_evidence_packet",
            );
        }
        if self.schema_version != SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "schema_version",
                &self.packet_id,
                "schema_version must be 1",
            );
        }
        if self.build_id.is_empty() {
            push_violation(
                &mut violations,
                "build_id",
                &self.packet_id,
                "build_id must be present for exact-build correlation",
            );
        }
        if self.lineage_entries.len() != self.host_lane_identities.len() {
            push_violation(
                &mut violations,
                "lineage_entries",
                &self.packet_id,
                "each host lane must have exactly one lineage entry in this seed builder",
            );
        }

        let mut lane_ids: BTreeSet<&str> = BTreeSet::new();
        for lane in &self.host_lane_identities {
            if !lane_ids.insert(&lane.host_lane_ref) {
                push_violation(
                    &mut violations,
                    "host_lane_identities.duplicate",
                    &lane.host_lane_ref,
                    "host lane refs must be unique",
                );
            }
            if lane.can_mutate || lane.externally_routed {
                let policy = self
                    .no_rerun_policies
                    .iter()
                    .find(|p| p.host_lane_ref == lane.host_lane_ref);
                match policy {
                    Some(p) if p.policy.forbids_silent_rerun() => {}
                    _ => {
                        push_violation(
                            &mut violations,
                            "no_rerun_policies",
                            &lane.host_lane_ref,
                            "mutating or externally routed lanes must carry a no-rerun policy that forbids silent rerun",
                        );
                    }
                }
            }
        }

        for decision in &self.review_decisions {
            if decision.explicit_review_required && decision.current_lane_accepted {
                push_violation(
                    &mut violations,
                    "review_decisions",
                    &decision.decision_id,
                    "lanes requiring explicit review cannot be accepted as current",
                );
            }
        }

        for summary in &self.domain_summaries {
            if summary.restart_entry_count == 0 && summary.lane_count > 0 {
                push_violation(
                    &mut violations,
                    "domain_summaries",
                    &summary.summary_id,
                    "domains with lanes must have at least one restart entry",
                );
            }
        }

        violations
    }

    /// Returns true when the packet is metadata-only.
    pub fn is_export_safe(&self) -> bool {
        self.export_safe
            && !self.build_id.is_empty()
            && !self.lineage_entries.is_empty()
            && !self.host_lane_identities.is_empty()
            && !self.no_rerun_policies.is_empty()
    }

    /// Renders deterministic plaintext suitable for support clipboard export.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Supervised-restart evidence packet\n");
        out.push_str(&format!("Packet: {}\n", self.packet_id));
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Build: {}\n", self.build_id));
        out.push_str(&format!("Generated: {}\n", self.generated_at));
        out.push_str(&format!("Lanes: {}\n", self.host_lane_count));
        out.push_str(&format!(
            "Mutating: {} | External: {} | Quarantined: {}\n",
            self.mutating_lane_count,
            self.externally_routed_lane_count,
            self.quarantined_lane_count
        ));
        out.push_str(&format!(
            "Review required: {}\n",
            self.review_required_lane_count
        ));
        for lane in &self.host_lane_identities {
            out.push_str(&format!("\nLane: {}\n", lane.host_lane_ref));
            out.push_str(&format!("  Family: {}\n", lane.host_family_label));
            out.push_str(&format!("  Domain: {}\n", lane.fault_domain_id));
            out.push_str(&format!(
                "  Mutating: {} | External: {}\n",
                lane.can_mutate, lane.externally_routed
            ));
            if let Some(policy) = self
                .no_rerun_policies
                .iter()
                .find(|p| p.host_lane_ref == lane.host_lane_ref)
            {
                out.push_str(&format!(
                    "  No-rerun policy: {} (forbids_silent_rerun={})\n",
                    policy.policy_token, policy.forbids_silent_rerun
                ));
            }
            if let Some(decision) = self
                .review_decisions
                .iter()
                .find(|d| d.host_lane_ref == lane.host_lane_ref)
            {
                out.push_str(&format!(
                    "  Review decision: {} (review_required={})\n",
                    decision.decision_token, decision.explicit_review_required
                ));
            }
            if !lane.partial_truth_result_refs.is_empty() {
                out.push_str(&format!(
                    "  Partial truth: {}\n",
                    lane.partial_truth_result_refs.join(", ")
                ));
            }
        }
        out.push_str("\nDomain summaries:\n");
        for summary in &self.domain_summaries {
            out.push_str(&format!(
                "  {}: lanes={} quarantined={} review_required={} blocks_healthy={}\n",
                summary.domain_token,
                summary.lane_count,
                summary.quarantined_lane_count,
                summary.review_required_lane_count,
                summary.blocks_healthy_claim
            ));
        }
        out
    }
}

fn domain_from_fault_domain_token(token: &str) -> RestartDomainClass {
    match token {
        "shell_interaction_core" => RestartDomainClass::Local,
        "remote_connector" => RestartDomainClass::Remote,
        "extension_or_tool_host" => RestartDomainClass::Extension,
        "session_execution_host" => RestartDomainClass::Debug,
        "notebook_kernel" => RestartDomainClass::Notebook,
        _ => {
            // Heuristic fallback based on lane family or token content.
            if token.contains("remote") {
                RestartDomainClass::Remote
            } else if token.contains("extension") || token.contains("tool") {
                RestartDomainClass::Extension
            } else if token.contains("notebook") {
                RestartDomainClass::Notebook
            } else if token.contains("session") || token.contains("debug") {
                RestartDomainClass::Debug
            } else {
                RestartDomainClass::Local
            }
        }
    }
}

fn budget_state_from_token(token: &str) -> RestartBudgetToken {
    match token {
        "within_budget" => RestartBudgetToken::WithinBudget,
        "budget_warning" => RestartBudgetToken::BudgetWarning,
        "budget_exhausted" => RestartBudgetToken::BudgetExhausted,
        "quarantined" => RestartBudgetToken::Quarantined,
        "no_automatic_restart" => RestartBudgetToken::NoAutomaticRestart,
        "reattach_review_required" => RestartBudgetToken::ReattachReviewRequired,
        "disabled" => RestartBudgetToken::Disabled,
        _ => RestartBudgetToken::NoAutomaticRestart,
    }
}

fn decision_class_from_token(token: &str) -> SupervisedRestartDecisionClass {
    match token {
        "current" => SupervisedRestartDecisionClass::Current,
        "auto_reattached_stale_refresh" => {
            SupervisedRestartDecisionClass::AutoReattachedStaleRefresh
        }
        "review_required" => SupervisedRestartDecisionClass::ReviewRequired,
        "reapproval_required" => SupervisedRestartDecisionClass::ReapprovalRequired,
        "rerun_required" => SupervisedRestartDecisionClass::RerunRequired,
        "blocked_manual_repair" => SupervisedRestartDecisionClass::BlockedManualRepair,
        _ => SupervisedRestartDecisionClass::ReviewRequired,
    }
}

/// Builds the canonical seeded supervised-restart evidence packet.
pub fn seeded_supervised_restart_evidence_packet() -> SupervisedRestartEvidencePacket {
    let fault_packet = crate::seeded_fault_domain_view_packet();
    SupervisedRestartEvidencePacket::from_fault_domain_view_packet(
        "supervised-restart:seed",
        "2026-05-18T12:10:00Z",
        "aureline-build:seed:m4",
        &fault_packet,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_has_all_five_domains() {
        let packet = seeded_supervised_restart_evidence_packet();
        let domain_tokens: BTreeSet<String> = packet
            .domain_summaries
            .iter()
            .map(|s| s.domain_token.clone())
            .collect();
        for expected in ["local", "remote", "extension", "debug", "notebook"] {
            assert!(
                domain_tokens.contains(expected),
                "missing domain {expected}"
            );
        }
    }

    #[test]
    fn mutating_lanes_carry_no_rerun_policy() {
        let packet = seeded_supervised_restart_evidence_packet();
        for lane in &packet.host_lane_identities {
            if lane.can_mutate || lane.externally_routed {
                let policy = packet
                    .no_rerun_policies
                    .iter()
                    .find(|p| p.host_lane_ref == lane.host_lane_ref)
                    .expect("mutating lane must have policy");
                assert!(
                    policy.forbids_silent_rerun,
                    "{} must forbid silent rerun",
                    lane.host_lane_ref
                );
            }
        }
    }

    #[test]
    fn non_mutating_lanes_may_rehydrate_safely() {
        let packet = seeded_supervised_restart_evidence_packet();
        for lane in &packet.host_lane_identities {
            if !lane.can_mutate && !lane.externally_routed {
                let policy = packet
                    .no_rerun_policies
                    .iter()
                    .find(|p| p.host_lane_ref == lane.host_lane_ref)
                    .expect("lane must have policy");
                assert!(
                    !policy.forbids_silent_rerun || lane.health_token != "healthy",
                    "{} healthy non-mutating lane should rehydrate safely",
                    lane.host_lane_ref
                );
            }
        }
    }

    #[test]
    fn packet_is_export_safe() {
        let packet = seeded_supervised_restart_evidence_packet();
        assert!(packet.is_export_safe());
        assert!(packet.validate().is_empty());
    }

    #[test]
    fn packet_round_trips_through_serde() {
        let packet = seeded_supervised_restart_evidence_packet();
        let json = serde_json::to_string(&packet).expect("serialize");
        let round: SupervisedRestartEvidencePacket =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round, packet);
    }

    #[test]
    fn plaintext_quotes_closed_tokens() {
        let packet = seeded_supervised_restart_evidence_packet();
        let text = packet.render_plaintext();
        for token in [
            "local",
            "remote",
            "extension",
            "debug",
            "notebook",
            "Supervised-restart evidence packet",
        ] {
            assert!(text.contains(token), "plaintext must quote '{token}'");
        }
        assert!(!text.contains("/Users/"));
    }

    #[test]
    fn review_required_decisions_block_current_claim() {
        let packet = seeded_supervised_restart_evidence_packet();
        for decision in &packet.review_decisions {
            if decision.explicit_review_required {
                assert!(
                    !decision.current_lane_accepted,
                    "{} must not accept current lane",
                    decision.decision_id
                );
            }
        }
    }

    #[test]
    fn domain_summary_counts_match_lane_state() {
        let packet = seeded_supervised_restart_evidence_packet();
        for summary in &packet.domain_summaries {
            assert_eq!(
                summary.restart_entry_count, summary.lane_count,
                "{}: restart entries must match lane count in seed builder",
                summary.domain_token
            );
        }
    }
}
