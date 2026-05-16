//! Extension host isolation, restart-budget, resource-limit, and
//! quarantine supervision contract.
//!
//! This module promotes the runtime v1 beta admission contract into a
//! supervision record that binds:
//!
//! - the per-axis runtime-budget evidence (discovery, cold / warm
//!   activation, idle polling, memory, egress, crash loop), each
//!   projected into the closed `pressure_class` vocabulary from
//!   `artifacts/extensions/runtime_budget_rows.yaml`;
//! - the restart-budget snapshot (typed posture, attempts used,
//!   attempts remaining, crash-loop window counters and trip
//!   thresholds);
//! - the supervisor's response class (none / throttle / disable until
//!   next session / disable until user explicit reenable / quarantine)
//!   resolved from those axes against the closed trigger-rule
//!   vocabulary in `artifacts/extensions/quarantine_rules.yaml`;
//! - the visibility posture and discovery-ranking posture so a runtime
//!   quarantine cannot silently sit on the runtime-status pill while
//!   the install review sheet keeps treating the row as healthy; and
//! - the maintainer-coverage posture so a single owner cannot quietly
//!   land an egress disable or a runtime quarantine on a quorum-bearing
//!   decision.
//!
//! One [`ExtensionHostSupervisionRecord`] is the inspectable answer to
//! "is this extension host running under budget, throttled, disabled,
//! awaiting reenable, quarantined, or recovering?". The first consumer
//! is a metadata-safe [`ExtensionHostSupervisionSupportExportRecord`]
//! that the support export, partner packet template, install review
//! chrome, and CLI / headless lanes read instead of inventing a local
//! "extension stopped working" string.
//!
//! The cross-tool boundary schema is
//! [`/schemas/extensions/host_isolation.schema.json`](../../../../schemas/extensions/host_isolation.schema.json);
//! the reviewer-facing landing page is
//! [`/docs/extensions/m3/host_isolation_beta.md`](../../../../docs/extensions/m3/host_isolation_beta.md);
//! the checked-in fixtures live under
//! [`/fixtures/extensions/m3/isolation_and_quarantine/`](../../../../fixtures/extensions/m3/isolation_and_quarantine/).

use serde::{Deserialize, Serialize};

use crate::manifest_baseline::RedactionClass;
use crate::runtime::{
    DegradedStateClass, HostPlacementClass, HostSupervisionClass, RestartPostureClass,
    RuntimeAdmissionDecisionClass, RuntimeLifecycleStateClass, RuntimeV1BetaContractRecord,
};

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized
/// [`ExtensionHostSupervisionRecord`] payloads.
pub const EXTENSION_HOST_SUPERVISION_RECORD_KIND: &str = "extension_host_supervision_record";

/// Record-kind tag carried on serialized
/// [`ExtensionHostSupervisionSupportExportRecord`] payloads.
pub const EXTENSION_HOST_SUPERVISION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "extension_host_supervision_support_export_record";

/// Schema version of the supervision payloads.
///
/// Bumped on breaking payload changes. Additive enum members or
/// optional fields are additive-minor and require consumers to keep
/// unknown-field preservation at their boundary.
pub const EXTENSION_HOST_SUPERVISION_SCHEMA_VERSION: u32 = 1;

/// Closed runtime-budget axis vocabulary mirrored from
/// `artifacts/extensions/runtime_budget_rows.yaml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupervisionAxisClass {
    Discovery,
    ColdActivation,
    WarmActivation,
    IdlePolling,
    Memory,
    Egress,
    CrashLoop,
}

/// Closed pressure-class vocabulary mirrored from
/// `artifacts/extensions/runtime_budget_rows.yaml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetPressureClass {
    Nominal,
    SoftBreach,
    HardBreach,
    CrashLoopWindowBreach,
    NotApplicable,
}

/// Closed response-class vocabulary mirrored from
/// `artifacts/extensions/quarantine_rules.yaml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupervisionResponseClass {
    NoneNominal,
    ThrottleBackgroundWork,
    DisableUntilNextSession,
    DisableUntilUserExplicitReenable,
    Quarantine,
}

/// Closed visibility-posture vocabulary mirrored from
/// `artifacts/extensions/quarantine_rules.yaml`.
///
/// A response other than `none_nominal` MUST surface on at least one
/// user-facing surface among install_review, permission_inspector, or
/// the runtime_status_pill. A throttle or disable that surfaces nowhere
/// is non-conforming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibilityPostureClass {
    /// Reserved sentinel for `none_nominal` responses with no surface.
    NotVisibleNominalRow,
    RuntimeStatusPillOnly,
    InstallReviewAndPermissionInspector,
    InstallReviewAndRuntimeStatusPill,
    PermissionInspectorAndRuntimeStatusPill,
    InstallReviewAndPermissionInspectorAndRuntimeStatusPill,
}

/// Closed discovery-ranking posture vocabulary mirrored from
/// `artifacts/extensions/quarantine_rules.yaml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryRankingPostureClass {
    UnchangedStillUserInstallable,
    DemotedFairRankedWithVisibleWarning,
    SuppressInstalledInManyWorkspacesSignal,
    RemovedFromRanking,
}

/// Closed maintainer-coverage class.
///
/// A `disable_until_user_explicit_reenable` or `quarantine` response
/// MUST cite `RequiredQuorumRecorded`. A `RequiredQuorumMissing` row
/// is admitted only as a denial drill paired with
/// [`SupervisionDecisionClass::RefuseInconsistentInput`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaintainerCoverageClass {
    NotRequired,
    RequiredQuorumRecorded,
    RequiredQuorumMissing,
}

/// Closed recovery-precondition vocabulary mirrored from
/// `artifacts/extensions/quarantine_rules.yaml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryPreconditionClass {
    NoneNotRecovering,
    ResourceGovernorReturnedToNominal,
    EfficiencyStateReturnedToNominalOrRecovery,
    AdminPolicyClearedQuarantine,
    PublisherContinuityRowCleared,
    UserExplicitReenable,
    NextSessionColdStart,
    CrashLoopWindowClearedWithoutBreach,
}

/// Closed visible-projection-on-recovery vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryVisibleProjectionClass {
    NotRecovering,
    Warming,
    Partial,
    Ready,
}

/// Closed supervision-decision class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupervisionDecisionClass {
    /// Every axis is nominal; the host continues under budget.
    ContinueAdmitted,
    /// A soft-breach axis sustained pressure; background work is throttled.
    ThrottleBackgroundWork,
    /// A hard-breach axis tripped a per-session disable.
    DisableUntilNextSession,
    /// An egress or admin-bearing disable that only an explicit user
    /// or admin reenable may clear.
    DisableUntilUserExplicitReenable,
    /// Crash-loop or runtime-budget quarantine is active; activation
    /// is held until the quarantine clears.
    QuarantinePendingReview,
    /// Publisher-level block is in force; the host holds without a
    /// quarantine because the cause is upstream.
    HoldPublisherBlocked,
    /// A recovery precondition is pending or in progress.
    RecoveryInProgress,
    /// A typed inconsistency between axis pressure, response, lifecycle,
    /// maintainer coverage, or visibility posture; the supervision row
    /// is rejected and the runtime falls back to its previous response.
    RefuseInconsistentInput,
}

/// Closed supervision-reason class paired with
/// [`SupervisionDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupervisionReasonClass {
    NominalUnderBudget,
    SustainedSoftBreachThrottlesBackground,
    IdlePollingHardBreachDisablesUntilNextSession,
    MemoryHardCapBreachDisablesUntilNextSession,
    EgressHardBreachRequiresUserReenable,
    CrashLoopWindowBreachTripsQuarantine,
    RuntimeBudgetQuarantineActive,
    PublisherBlockActive,
    RecoveryInProgressReturningToNominal,
    RefusedRuntimeContractNotAdmitted,
    RefusedMaintainerCoverageMissingOnQuorumDecision,
    RefusedAxisPressureInconsistentWithResponse,
    RefusedQuarantineWithoutTriggerRule,
    RefusedResponseVisibilityMissingFromUserSurfaces,
    RefusedQuarantineWithoutDiscoveryRemoval,
}

/// One axis-pressure entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AxisBudgetEntry {
    pub axis_class: SupervisionAxisClass,
    pub pressure_class: BudgetPressureClass,
}

/// Restart-budget snapshot.
///
/// `attempts_used` MUST agree with the runtime contract's
/// `restart_attempt_count`. The crash-loop window counters mirror the
/// `crash_loop_window` block on `runtime_budget_rows.yaml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartBudgetSnapshot {
    pub restart_posture_class: RestartPostureClass,
    pub attempts_used: u32,
    pub attempts_remaining: u32,
    pub crash_loop_window_distinct_failures: u32,
    pub crash_loop_trip_disable_threshold: u32,
    pub crash_loop_trip_quarantine_threshold: u32,
}

/// Inputs supplied by the extension host to build a supervision record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionHostSupervisionInput {
    pub supervision_id: String,
    pub runtime_contract: RuntimeV1BetaContractRecord,
    pub axis_budget_entries: Vec<AxisBudgetEntry>,
    pub restart_budget: RestartBudgetSnapshot,
    pub visibility_posture_class: VisibilityPostureClass,
    pub discovery_ranking_posture_class: DiscoveryRankingPostureClass,
    pub maintainer_coverage_class: MaintainerCoverageClass,
    pub recovery_precondition_class: RecoveryPreconditionClass,
    pub recovery_visible_projection_class: RecoveryVisibleProjectionClass,
    pub trigger_rule_ref: Option<String>,
    pub paired_audit_event_refs: Vec<String>,
    pub repair_affordance_label: String,
    pub decided_at: String,
}

/// One supervision record per evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionHostSupervisionRecord {
    pub record_kind: String,
    pub extension_host_supervision_schema_version: u32,
    pub supervision_id: String,
    pub contract_ref: String,
    pub extension_identity_ref: String,
    pub extension_version: String,
    pub host_placement_class: HostPlacementClass,
    pub host_supervision_class: HostSupervisionClass,
    pub lifecycle_state_class: RuntimeLifecycleStateClass,
    pub restart_posture_class: RestartPostureClass,
    pub restart_attempt_count: u32,
    pub degraded_state_class: DegradedStateClass,
    pub axis_budget_entries: Vec<AxisBudgetEntry>,
    pub restart_budget: RestartBudgetSnapshot,
    pub response_class: SupervisionResponseClass,
    pub visibility_posture_class: VisibilityPostureClass,
    pub discovery_ranking_posture_class: DiscoveryRankingPostureClass,
    pub maintainer_coverage_class: MaintainerCoverageClass,
    pub recovery_precondition_class: RecoveryPreconditionClass,
    pub recovery_visible_projection_class: RecoveryVisibleProjectionClass,
    pub trigger_rule_ref: Option<String>,
    pub paired_audit_event_refs: Vec<String>,
    pub repair_affordance_label: String,
    pub supervision_decision_class: SupervisionDecisionClass,
    pub supervision_reason_class: SupervisionReasonClass,
    pub supervision_summary: String,
    pub blocks_activation: bool,
    pub decided_at: String,
    pub redaction_class: RedactionClass,
}

/// First consumer projection: a metadata-safe support / partner export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionHostSupervisionSupportExportRecord {
    pub record_kind: String,
    pub extension_host_supervision_schema_version: u32,
    pub export_id: String,
    pub supervision_ref: String,
    pub contract_ref: String,
    pub extension_identity_ref: String,
    pub extension_version: String,
    pub host_placement_class: HostPlacementClass,
    pub host_supervision_class: HostSupervisionClass,
    pub lifecycle_state_class: RuntimeLifecycleStateClass,
    pub restart_posture_class: RestartPostureClass,
    pub restart_attempt_count: u32,
    pub restart_attempts_remaining: u32,
    pub crash_loop_window_distinct_failures: u32,
    pub degraded_state_class: DegradedStateClass,
    pub worst_axis_class: Option<SupervisionAxisClass>,
    pub worst_axis_pressure_class: BudgetPressureClass,
    pub response_class: SupervisionResponseClass,
    pub visibility_posture_class: VisibilityPostureClass,
    pub discovery_ranking_posture_class: DiscoveryRankingPostureClass,
    pub maintainer_coverage_class: MaintainerCoverageClass,
    pub recovery_precondition_class: RecoveryPreconditionClass,
    pub recovery_visible_projection_class: RecoveryVisibleProjectionClass,
    pub trigger_rule_ref: Option<String>,
    pub repair_affordance_label: String,
    pub supervision_decision_class: SupervisionDecisionClass,
    pub supervision_reason_class: SupervisionReasonClass,
    pub blocks_activation: bool,
    pub export_safe_summary: String,
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by supervision validators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionHostSupervisionFinding {
    pub check_id: &'static str,
    pub message: String,
}

impl ExtensionHostSupervisionFinding {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

/// Evaluate a supervision record.
///
/// The evaluator is deterministic. Decision precedence:
///
/// 1. If the runtime contract is not in an admitted, admitted-narrowed,
///    or quarantined posture, refuse with
///    `refused_runtime_contract_not_admitted`.
/// 2. If the contract's lifecycle is `publisher_blocked`, hold with
///    `publisher_block_active`.
/// 3. If any axis is at `crash_loop_window_breach`, quarantine with
///    `crash_loop_window_breach_trips_quarantine` (after maintainer-
///    coverage, visibility, and discovery-ranking guardrails).
/// 4. If `memory` is at `hard_breach`, disable until next session with
///    `memory_hard_cap_breach_disables_until_next_session`.
/// 5. If `idle_polling` is at `hard_breach`, disable until next session.
/// 6. If `egress` is at `hard_breach`, require an explicit user / admin
///    reenable (maintainer-coverage required).
/// 7. If any sustained axis is at `soft_breach`, throttle background work.
/// 8. If the contract is quarantined under any non-crash-loop budget
///    trip, quarantine with `runtime_budget_quarantine_active`.
/// 9. If a recovery precondition is pending, return `recovery_in_progress`.
/// 10. Otherwise return `continue_admitted` / `nominal_under_budget`.
pub fn evaluate_extension_host_supervision(
    input: ExtensionHostSupervisionInput,
) -> ExtensionHostSupervisionRecord {
    let contract = &input.runtime_contract;
    let response_class = response_for_axes(&input.axis_budget_entries);
    let (supervision_decision_class, supervision_reason_class, supervision_summary) =
        decide_supervision(&input, response_class);
    let blocks_activation = matches!(
        supervision_decision_class,
        SupervisionDecisionClass::DisableUntilNextSession
            | SupervisionDecisionClass::DisableUntilUserExplicitReenable
            | SupervisionDecisionClass::QuarantinePendingReview
            | SupervisionDecisionClass::HoldPublisherBlocked
            | SupervisionDecisionClass::RefuseInconsistentInput
    );

    ExtensionHostSupervisionRecord {
        record_kind: EXTENSION_HOST_SUPERVISION_RECORD_KIND.to_string(),
        extension_host_supervision_schema_version: EXTENSION_HOST_SUPERVISION_SCHEMA_VERSION,
        supervision_id: input.supervision_id,
        contract_ref: contract.contract_id.clone(),
        extension_identity_ref: contract.extension_identity_ref.clone(),
        extension_version: contract.extension_version.clone(),
        host_placement_class: contract.host_placement_class,
        host_supervision_class: contract.host_supervision_class,
        lifecycle_state_class: contract.lifecycle_state_class,
        restart_posture_class: contract.restart_posture_class,
        restart_attempt_count: contract.restart_attempt_count,
        degraded_state_class: contract.degraded_state_class,
        axis_budget_entries: input.axis_budget_entries,
        restart_budget: input.restart_budget,
        response_class,
        visibility_posture_class: input.visibility_posture_class,
        discovery_ranking_posture_class: input.discovery_ranking_posture_class,
        maintainer_coverage_class: input.maintainer_coverage_class,
        recovery_precondition_class: input.recovery_precondition_class,
        recovery_visible_projection_class: input.recovery_visible_projection_class,
        trigger_rule_ref: input.trigger_rule_ref,
        paired_audit_event_refs: input.paired_audit_event_refs,
        repair_affordance_label: input.repair_affordance_label,
        supervision_decision_class,
        supervision_reason_class,
        supervision_summary,
        blocks_activation,
        decided_at: input.decided_at,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Project a supervision record into the first consumer surface.
pub fn project_extension_host_supervision_support_export(
    record: &ExtensionHostSupervisionRecord,
) -> ExtensionHostSupervisionSupportExportRecord {
    let (worst_axis_class, worst_axis_pressure_class) = worst_axis(&record.axis_budget_entries);
    let export_safe_summary = format!(
        "{} Worst axis: {} at {:?}. Response={:?}. Restart {:?} used={} remaining={} crash_loop_failures={}/{}.",
        record.supervision_summary,
        worst_axis_class
            .map(|axis| format!("{axis:?}"))
            .unwrap_or_else(|| "none".to_string()),
        worst_axis_pressure_class,
        record.response_class,
        record.restart_budget.restart_posture_class,
        record.restart_budget.attempts_used,
        record.restart_budget.attempts_remaining,
        record.restart_budget.crash_loop_window_distinct_failures,
        record.restart_budget.crash_loop_trip_quarantine_threshold,
    );

    ExtensionHostSupervisionSupportExportRecord {
        record_kind: EXTENSION_HOST_SUPERVISION_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        extension_host_supervision_schema_version: EXTENSION_HOST_SUPERVISION_SCHEMA_VERSION,
        export_id: format!(
            "extension_host_supervision_support_export:{}",
            record.supervision_id
        ),
        supervision_ref: record.supervision_id.clone(),
        contract_ref: record.contract_ref.clone(),
        extension_identity_ref: record.extension_identity_ref.clone(),
        extension_version: record.extension_version.clone(),
        host_placement_class: record.host_placement_class,
        host_supervision_class: record.host_supervision_class,
        lifecycle_state_class: record.lifecycle_state_class,
        restart_posture_class: record.restart_posture_class,
        restart_attempt_count: record.restart_attempt_count,
        restart_attempts_remaining: record.restart_budget.attempts_remaining,
        crash_loop_window_distinct_failures: record
            .restart_budget
            .crash_loop_window_distinct_failures,
        degraded_state_class: record.degraded_state_class,
        worst_axis_class,
        worst_axis_pressure_class,
        response_class: record.response_class,
        visibility_posture_class: record.visibility_posture_class,
        discovery_ranking_posture_class: record.discovery_ranking_posture_class,
        maintainer_coverage_class: record.maintainer_coverage_class,
        recovery_precondition_class: record.recovery_precondition_class,
        recovery_visible_projection_class: record.recovery_visible_projection_class,
        trigger_rule_ref: record.trigger_rule_ref.clone(),
        repair_affordance_label: record.repair_affordance_label.clone(),
        supervision_decision_class: record.supervision_decision_class,
        supervision_reason_class: record.supervision_reason_class,
        blocks_activation: record.blocks_activation,
        export_safe_summary,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate structural invariants for a supervision record.
pub fn validate_extension_host_supervision(
    record: &ExtensionHostSupervisionRecord,
) -> Vec<ExtensionHostSupervisionFinding> {
    let mut findings = Vec::new();

    if record.record_kind != EXTENSION_HOST_SUPERVISION_RECORD_KIND {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.record_kind_wrong",
            format!(
                "record_kind must be '{EXTENSION_HOST_SUPERVISION_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.extension_host_supervision_schema_version != EXTENSION_HOST_SUPERVISION_SCHEMA_VERSION
    {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.schema_version_wrong",
            format!(
                "extension_host_supervision_schema_version must be {EXTENSION_HOST_SUPERVISION_SCHEMA_VERSION}; got {}",
                record.extension_host_supervision_schema_version
            ),
        ));
    }
    if !record.supervision_id.starts_with("supervision:") {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.id_unprefixed",
            "supervision_id must start with 'supervision:'",
        ));
    }
    if !record.contract_ref.starts_with("runtime_v1_beta:") {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.contract_ref_unprefixed",
            "contract_ref must start with 'runtime_v1_beta:'",
        ));
    }
    if let Some(trigger) = record.trigger_rule_ref.as_deref() {
        if !trigger.starts_with("quarantine_rule:") {
            findings.push(ExtensionHostSupervisionFinding::new(
                "extension_host_supervision.trigger_rule_ref_unprefixed",
                "trigger_rule_ref must start with 'quarantine_rule:'",
            ));
        }
    }
    if record.restart_budget.attempts_used != record.restart_attempt_count {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.restart_attempts_disagree",
            "restart_budget.attempts_used must equal restart_attempt_count",
        ));
    }
    if record.restart_budget.restart_posture_class != record.restart_posture_class {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.restart_posture_disagree",
            "restart_budget.restart_posture_class must equal restart_posture_class",
        ));
    }
    if record.restart_budget.crash_loop_trip_disable_threshold
        > record.restart_budget.crash_loop_trip_quarantine_threshold
    {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.crash_loop_thresholds_invalid",
            "crash_loop_trip_disable_threshold must be <= crash_loop_trip_quarantine_threshold",
        ));
    }
    if matches!(
        record.response_class,
        SupervisionResponseClass::Quarantine
            | SupervisionResponseClass::DisableUntilUserExplicitReenable
    ) && !matches!(
        record.maintainer_coverage_class,
        MaintainerCoverageClass::RequiredQuorumRecorded
    ) {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.maintainer_coverage_required",
            "quarantine and disable-until-user-explicit-reenable require RequiredQuorumRecorded",
        ));
    }
    if matches!(record.response_class, SupervisionResponseClass::Quarantine)
        && record.discovery_ranking_posture_class
            != DiscoveryRankingPostureClass::RemovedFromRanking
    {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.quarantine_requires_discovery_removal",
            "quarantine response requires discovery_ranking_posture_class=removed_from_ranking",
        ));
    }
    if record.response_class != SupervisionResponseClass::NoneNominal
        && record.visibility_posture_class == VisibilityPostureClass::NotVisibleNominalRow
    {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.response_requires_visibility",
            "a non-nominal response requires a visibility_posture_class other than not_visible_nominal_row",
        ));
    }
    if matches!(
        record.response_class,
        SupervisionResponseClass::Quarantine
            | SupervisionResponseClass::DisableUntilUserExplicitReenable
            | SupervisionResponseClass::DisableUntilNextSession
    ) && !visibility_includes_install_review_or_permission_inspector(
        record.visibility_posture_class,
    ) {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.disable_quarantine_visibility_required",
            "disable / quarantine responses must surface on install_review or permission_inspector",
        ));
    }
    if matches!(
        record.supervision_decision_class,
        SupervisionDecisionClass::RecoveryInProgress
    ) && record.recovery_visible_projection_class
        == RecoveryVisibleProjectionClass::NotRecovering
    {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.recovery_visibility_inconsistent",
            "a recovery_in_progress decision requires a non-not_recovering visible projection",
        ));
    }
    if record.response_class == SupervisionResponseClass::Quarantine
        && record.trigger_rule_ref.is_none()
    {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.quarantine_requires_trigger_rule",
            "quarantine response must cite a trigger_rule_ref",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.redaction_class_must_be_metadata_safe",
            "supervision records must emit RedactionClass::MetadataSafeDefault",
        ));
    }
    if record.repair_affordance_label.trim().is_empty() {
        findings.push(ExtensionHostSupervisionFinding::new(
            "extension_host_supervision.repair_affordance_label_required",
            "supervision records must cite a non-empty repair_affordance_label",
        ));
    }

    findings
}

fn response_for_axes(axes: &[AxisBudgetEntry]) -> SupervisionResponseClass {
    if axes
        .iter()
        .any(|entry| entry.pressure_class == BudgetPressureClass::CrashLoopWindowBreach)
    {
        return SupervisionResponseClass::Quarantine;
    }
    if axes.iter().any(|entry| {
        entry.axis_class == SupervisionAxisClass::Memory
            && entry.pressure_class == BudgetPressureClass::HardBreach
    }) {
        return SupervisionResponseClass::DisableUntilNextSession;
    }
    if axes.iter().any(|entry| {
        entry.axis_class == SupervisionAxisClass::IdlePolling
            && entry.pressure_class == BudgetPressureClass::HardBreach
    }) {
        return SupervisionResponseClass::DisableUntilNextSession;
    }
    if axes.iter().any(|entry| {
        entry.axis_class == SupervisionAxisClass::Egress
            && entry.pressure_class == BudgetPressureClass::HardBreach
    }) {
        return SupervisionResponseClass::DisableUntilUserExplicitReenable;
    }
    if axes
        .iter()
        .any(|entry| entry.pressure_class == BudgetPressureClass::SoftBreach)
    {
        return SupervisionResponseClass::ThrottleBackgroundWork;
    }
    SupervisionResponseClass::NoneNominal
}

fn decide_supervision(
    input: &ExtensionHostSupervisionInput,
    response_class: SupervisionResponseClass,
) -> (SupervisionDecisionClass, SupervisionReasonClass, String) {
    let contract = &input.runtime_contract;

    if !matches!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Admitted
            | RuntimeAdmissionDecisionClass::AdmittedNarrowed
            | RuntimeAdmissionDecisionClass::Quarantined
    ) {
        return (
            SupervisionDecisionClass::RefuseInconsistentInput,
            SupervisionReasonClass::RefusedRuntimeContractNotAdmitted,
            "Refused: the runtime v1 beta contract is not in an admitted, admitted-narrowed, or quarantined posture; supervision cannot evaluate.".to_string(),
        );
    }

    if matches!(
        contract.lifecycle_state_class,
        RuntimeLifecycleStateClass::PublisherBlocked
    ) {
        return (
            SupervisionDecisionClass::HoldPublisherBlocked,
            SupervisionReasonClass::PublisherBlockActive,
            "Hold: publisher-level block is in force; supervision holds the host without quarantining.".to_string(),
        );
    }

    let contract_quarantined = contract.runtime_budget_quarantine_active
        || matches!(
            contract.lifecycle_state_class,
            RuntimeLifecycleStateClass::Quarantined
        );

    if contract_quarantined && response_class != SupervisionResponseClass::Quarantine {
        return (
            SupervisionDecisionClass::RefuseInconsistentInput,
            SupervisionReasonClass::RefusedAxisPressureInconsistentWithResponse,
            "Refused: runtime contract reports an active quarantine but no axis is at crash-loop window breach pressure.".to_string(),
        );
    }
    if !contract_quarantined && response_class == SupervisionResponseClass::Quarantine {
        return (
            SupervisionDecisionClass::RefuseInconsistentInput,
            SupervisionReasonClass::RefusedAxisPressureInconsistentWithResponse,
            "Refused: axis reports crash-loop window breach pressure but the runtime contract is not quarantined.".to_string(),
        );
    }

    match response_class {
        SupervisionResponseClass::Quarantine => {
            if input.trigger_rule_ref.is_none() {
                return (
                    SupervisionDecisionClass::RefuseInconsistentInput,
                    SupervisionReasonClass::RefusedQuarantineWithoutTriggerRule,
                    "Refused: quarantine response requires a typed trigger_rule_ref.".to_string(),
                );
            }
            if input.maintainer_coverage_class != MaintainerCoverageClass::RequiredQuorumRecorded {
                return (
                    SupervisionDecisionClass::RefuseInconsistentInput,
                    SupervisionReasonClass::RefusedMaintainerCoverageMissingOnQuorumDecision,
                    "Refused: quarantine decision requires maintainer coverage = required_quorum_recorded.".to_string(),
                );
            }
            if input.discovery_ranking_posture_class
                != DiscoveryRankingPostureClass::RemovedFromRanking
            {
                return (
                    SupervisionDecisionClass::RefuseInconsistentInput,
                    SupervisionReasonClass::RefusedQuarantineWithoutDiscoveryRemoval,
                    "Refused: quarantine decision requires discovery ranking removed_from_ranking.".to_string(),
                );
            }
            if !visibility_includes_install_review_or_permission_inspector(
                input.visibility_posture_class,
            ) {
                return (
                    SupervisionDecisionClass::RefuseInconsistentInput,
                    SupervisionReasonClass::RefusedResponseVisibilityMissingFromUserSurfaces,
                    "Refused: quarantine decision must surface on install_review or permission_inspector.".to_string(),
                );
            }
            let (reason, summary) = if input
                .axis_budget_entries
                .iter()
                .any(|entry| entry.pressure_class == BudgetPressureClass::CrashLoopWindowBreach)
            {
                (
                    SupervisionReasonClass::CrashLoopWindowBreachTripsQuarantine,
                    "Quarantine: a crash-loop window breach tripped the quarantine response; activation is held until the trigger rule clears."
                        .to_string(),
                )
            } else {
                (
                    SupervisionReasonClass::RuntimeBudgetQuarantineActive,
                    "Quarantine: runtime-budget evidence reports an active quarantine, disable, or trip; activation is held until the trigger rule clears."
                        .to_string(),
                )
            };
            (
                SupervisionDecisionClass::QuarantinePendingReview,
                reason,
                summary,
            )
        }
        SupervisionResponseClass::DisableUntilUserExplicitReenable => {
            if input.maintainer_coverage_class != MaintainerCoverageClass::RequiredQuorumRecorded {
                return (
                    SupervisionDecisionClass::RefuseInconsistentInput,
                    SupervisionReasonClass::RefusedMaintainerCoverageMissingOnQuorumDecision,
                    "Refused: disable-until-user-explicit-reenable requires maintainer coverage = required_quorum_recorded.".to_string(),
                );
            }
            if !visibility_includes_install_review_or_permission_inspector(
                input.visibility_posture_class,
            ) {
                return (
                    SupervisionDecisionClass::RefuseInconsistentInput,
                    SupervisionReasonClass::RefusedResponseVisibilityMissingFromUserSurfaces,
                    "Refused: disable-until-user-explicit-reenable must surface on install_review or permission_inspector.".to_string(),
                );
            }
            (
                SupervisionDecisionClass::DisableUntilUserExplicitReenable,
                SupervisionReasonClass::EgressHardBreachRequiresUserReenable,
                "Disable: egress hard-cap breach requires an explicit user or admin reenable before activation resumes.".to_string(),
            )
        }
        SupervisionResponseClass::DisableUntilNextSession => {
            if !visibility_includes_install_review_or_permission_inspector(
                input.visibility_posture_class,
            ) {
                return (
                    SupervisionDecisionClass::RefuseInconsistentInput,
                    SupervisionReasonClass::RefusedResponseVisibilityMissingFromUserSurfaces,
                    "Refused: disable-until-next-session must surface on install_review or permission_inspector.".to_string(),
                );
            }
            let reason = if input.axis_budget_entries.iter().any(|entry| {
                entry.axis_class == SupervisionAxisClass::Memory
                    && entry.pressure_class == BudgetPressureClass::HardBreach
            }) {
                SupervisionReasonClass::MemoryHardCapBreachDisablesUntilNextSession
            } else {
                SupervisionReasonClass::IdlePollingHardBreachDisablesUntilNextSession
            };
            (
                SupervisionDecisionClass::DisableUntilNextSession,
                reason,
                "Disable: hard-cap breach on a session-scoped axis disables the host until the next cold start.".to_string(),
            )
        }
        SupervisionResponseClass::ThrottleBackgroundWork => (
            SupervisionDecisionClass::ThrottleBackgroundWork,
            SupervisionReasonClass::SustainedSoftBreachThrottlesBackground,
            "Throttle: a sustained soft breach throttles background work while keeping the host running.".to_string(),
        ),
        SupervisionResponseClass::NoneNominal => {
            if input.recovery_precondition_class != RecoveryPreconditionClass::NoneNotRecovering {
                return (
                    SupervisionDecisionClass::RecoveryInProgress,
                    SupervisionReasonClass::RecoveryInProgressReturningToNominal,
                    "Recovery: a pending recovery precondition is returning the host to nominal.".to_string(),
                );
            }
            (
                SupervisionDecisionClass::ContinueAdmitted,
                SupervisionReasonClass::NominalUnderBudget,
                "Continue: every axis is nominal under the runtime-budget envelope.".to_string(),
            )
        }
    }
}

fn visibility_includes_install_review_or_permission_inspector(
    class: VisibilityPostureClass,
) -> bool {
    matches!(
        class,
        VisibilityPostureClass::InstallReviewAndPermissionInspector
            | VisibilityPostureClass::InstallReviewAndRuntimeStatusPill
            | VisibilityPostureClass::PermissionInspectorAndRuntimeStatusPill
            | VisibilityPostureClass::InstallReviewAndPermissionInspectorAndRuntimeStatusPill
    )
}

fn worst_axis(entries: &[AxisBudgetEntry]) -> (Option<SupervisionAxisClass>, BudgetPressureClass) {
    let mut worst_class: Option<SupervisionAxisClass> = None;
    let mut worst_pressure = BudgetPressureClass::Nominal;
    for entry in entries {
        if pressure_rank(entry.pressure_class) > pressure_rank(worst_pressure) {
            worst_pressure = entry.pressure_class;
            worst_class = Some(entry.axis_class);
        }
    }
    (worst_class, worst_pressure)
}

fn pressure_rank(class: BudgetPressureClass) -> u8 {
    match class {
        BudgetPressureClass::NotApplicable => 0,
        BudgetPressureClass::Nominal => 1,
        BudgetPressureClass::SoftBreach => 2,
        BudgetPressureClass::HardBreach => 3,
        BudgetPressureClass::CrashLoopWindowBreach => 4,
    }
}
