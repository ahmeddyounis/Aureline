//! Beta durable-attention conformance projection.
//!
//! This module joins the existing activity-center and notification
//! privacy projections into one release-review corpus for durable job
//! rows, badge classes, quiet-hours suppression, exact reopen, and
//! support-export lineage. It does not mint a second task system: every
//! seeded case points at either the beta activity-center rows, the beta
//! notification privacy rows, or the frozen durable-job fixtures.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::activity_center::alpha::ActivityPartition;
use crate::activity_center::beta::{
    seeded_activity_center_beta_page, ActivityCenterBetaRow, AuthoritativeReopenClass,
};
use crate::notifications::actions::BadgeClass;
use crate::notifications::beta::{
    seeded_notification_privacy_beta_page, NotificationPrivacyBetaRow,
    NotificationPrivacyBetaRowClass,
};
use crate::notifications::envelope::{
    DedupeKeyScheme, FanoutSurfaceClass, QuietHoursMode, SourceSubsystem, SuppressionReason,
};

/// Schema version exported by durable-attention beta records.
pub const DURABLE_ATTENTION_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every durable-attention beta record.
pub const DURABLE_ATTENTION_BETA_SHARED_CONTRACT_REF: &str = "shell:durable_attention_beta:v1";

/// Stable record kind for [`DurableAttentionBetaPacket`] payloads.
pub const DURABLE_ATTENTION_BETA_PACKET_RECORD_KIND: &str =
    "shell_durable_attention_beta_packet_record";

/// Stable record kind for [`DurableJobRowStateMachineEntry`] payloads.
pub const DURABLE_JOB_ROW_STATE_MACHINE_RECORD_KIND: &str =
    "durable_job_row_state_machine_entry_record";

/// Stable record kind for [`DurableAttentionConformanceCase`] payloads.
pub const DURABLE_ATTENTION_CONFORMANCE_CASE_RECORD_KIND: &str =
    "durable_attention_conformance_case_record";

/// Stable record kind for [`BadgeClassAuditRow`] payloads.
pub const DURABLE_ATTENTION_BADGE_AUDIT_RECORD_KIND: &str =
    "durable_attention_badge_class_audit_row_record";

/// Stable record kind for [`QuietHoursSuppressionAuditRow`] payloads.
pub const DURABLE_ATTENTION_QUIET_HOURS_AUDIT_RECORD_KIND: &str =
    "durable_attention_quiet_hours_audit_row_record";

/// Stable record kind for [`ExactReopenProof`] payloads.
pub const DURABLE_ATTENTION_EXACT_REOPEN_PROOF_RECORD_KIND: &str =
    "durable_attention_exact_reopen_proof_record";

/// Stable record kind for [`SupportExportLineageRow`] payloads.
pub const DURABLE_ATTENTION_SUPPORT_LINEAGE_RECORD_KIND: &str =
    "durable_attention_support_export_lineage_row_record";

/// Durable job-row states exercised by the conformance state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableJobRowStateClass {
    /// Work is actively executing or preparing with a visible phase.
    Running,
    /// Work is accepted but waiting on a queue, resource, or boundary.
    QueuedWaiting,
    /// Work is blocked on user, trust, provider, policy, or admin approval.
    NeedsApproval,
    /// Work completed and remains reviewable.
    Completed,
    /// Work failed, partially completed, or needs follow-up.
    Failed,
    /// Work was cancelled by a user, subsystem, policy, or admin actor.
    Cancelled,
    /// Work is no longer active but remains available as history,
    /// evidence, suppression audit, or placeholder state.
    HistoryOnly,
}

impl DurableJobRowStateClass {
    /// Stable token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::QueuedWaiting => "queued_waiting",
            Self::NeedsApproval => "needs_approval",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::HistoryOnly => "history_only",
        }
    }
}

/// Beta job families covered by the durable-attention corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableAttentionJobFamily {
    /// Background indexing or search-readiness work.
    Indexing,
    /// Session restore, recovery, or restore-provenance work.
    Restore,
    /// Install, update, download, package, or bundle work.
    InstallUpdateDownload,
    /// Remote attach, reconnect, mirror, or transport-continuity work.
    RemoteReconnect,
    /// Task execution work.
    TaskRun,
    /// Test execution work.
    TestRun,
    /// Debug-session execution and attach work.
    DebugSession,
    /// AI review, AI apply, or AI approval work.
    AiReview,
    /// Git and hosted-review work.
    GitReview,
    /// Companion, browser, remote-agent, or managed-admin handoff work.
    CompanionHandoff,
    /// Admin or policy suppression work that still preserves durable
    /// local lineage.
    AdminPolicy,
}

impl DurableAttentionJobFamily {
    /// Stable token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Indexing => "indexing",
            Self::Restore => "restore",
            Self::InstallUpdateDownload => "install_update_download",
            Self::RemoteReconnect => "remote_reconnect",
            Self::TaskRun => "task_run",
            Self::TestRun => "test_run",
            Self::DebugSession => "debug_session",
            Self::AiReview => "ai_review",
            Self::GitReview => "git_review",
            Self::CompanionHandoff => "companion_handoff",
            Self::AdminPolicy => "admin_policy",
        }
    }
}

/// Source that proves a conformance case was not invented ad hoc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableAttentionCoverageSource {
    /// Case is backed by the seeded beta activity-center projection.
    ActivityCenterBetaRow,
    /// Case is backed by the seeded beta notification privacy projection.
    NotificationPrivacyBetaRow,
    /// Case is backed by a frozen durable-job or durable-attention fixture.
    ContractFixture,
}

/// Class of badge source asserted by an audited row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeSourceClass {
    /// Count derives from one durable row or envelope state transition.
    DerivedFromEnvelopeState,
    /// Count derives from the authoritative target object.
    DerivedFromCanonicalObject,
    /// Count derives from a grouped burst that collapses repeated events.
    AggregatedGroupedBurst,
    /// Row does not contribute to any active badge count.
    NotABadgeSource,
}

/// Quiet-hours or suppression decision applied to a conformance case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuietHoursDecisionClass {
    /// No quiet-hours, focus, admin, or companion hold applied.
    NotSuppressed,
    /// User quiet hours or an equivalent reduced-attention mode held fanout.
    HeldQuietHours,
    /// Admin policy narrowed or suppressed fanout while keeping audit.
    AdminSuppressed,
    /// Critical or blocking trust posture bypassed the hold.
    CriticalBypass,
    /// Cross-client fanout collapsed under the canonical event id.
    CrossClientDeduped,
}

/// Exact reopen class promised by a conformance case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExactReopenClass {
    /// Activation opens the exact durable object or activity row.
    ExactDurableObject,
    /// Activation opens a typed placeholder explaining the missing target.
    TruthfulPlaceholder,
    /// Activation opens an explained revalidation or denial path.
    DeniedRequiresRevalidation,
}

impl From<AuthoritativeReopenClass> for ExactReopenClass {
    fn from(value: AuthoritativeReopenClass) -> Self {
        match value {
            AuthoritativeReopenClass::ExactDurableObject => Self::ExactDurableObject,
            AuthoritativeReopenClass::TruthfulPlaceholder => Self::TruthfulPlaceholder,
            AuthoritativeReopenClass::DeniedAndExplained => Self::DeniedRequiresRevalidation,
        }
    }
}

/// One durable job-row state-machine entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableJobRowStateMachineEntry {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable state class.
    pub state_class: DurableJobRowStateClass,
    /// Activity-center partitions this state may appear in.
    pub valid_activity_partitions: Vec<ActivityPartition>,
    /// Phase or state fields required for this state.
    pub required_phase_fields: Vec<String>,
    /// Visible row cues required for this state.
    pub required_visible_cues: Vec<String>,
    /// Command-backed actions required or explainable for this state.
    pub required_actions: Vec<String>,
    /// True when exact reopen or a truthful placeholder is required.
    pub exact_reopen_or_placeholder_required: bool,
    /// True when support export must reconstruct the row lineage.
    pub support_export_lineage_required: bool,
    /// Always false for durable work; included so fixtures can prove the
    /// state machine rejects toast-only treatment mechanically.
    pub transient_only_allowed: bool,
}

/// One conformance case in the beta durable-attention corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableAttentionConformanceCase {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across fixture files.
    pub case_id: String,
    /// Reviewer-facing case label.
    pub case_label: String,
    /// Covered job family.
    pub job_family: DurableAttentionJobFamily,
    /// Projection source that backs the case.
    pub coverage_source: DurableAttentionCoverageSource,
    /// Stable ref to the row or fixture backing this case.
    pub source_projection_ref: String,
    /// Durable row state class.
    pub state_class: DurableJobRowStateClass,
    /// Activity-center partition.
    pub activity_partition: ActivityPartition,
    /// Source subsystem.
    pub source_subsystem: SourceSubsystem,
    /// Actor or subsystem label rendered by the row.
    pub actor_or_subsystem_label: String,
    /// Execution origin class.
    pub execution_origin_class: String,
    /// Scope or target label.
    pub scope_label: String,
    /// Current phase label.
    pub phase_label: String,
    /// Authoritative age label.
    pub age_label: String,
    /// Stable durable-job id ref.
    pub durable_job_id_ref: String,
    /// Stable canonical event id.
    pub canonical_event_id: String,
    /// Authoritative object target ref.
    pub canonical_object_target_ref: String,
    /// Badge class emitted by this case, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_class: Option<BadgeClass>,
    /// Badge source class.
    pub badge_source_class: BadgeSourceClass,
    /// Dedupe key scheme.
    pub dedupe_key_scheme: DedupeKeyScheme,
    /// Quiet-hours or suppression decision.
    pub quiet_hours_decision: QuietHoursDecisionClass,
    /// Reopen class promised by this row.
    pub reopen_class: ExactReopenClass,
    /// Exact target identity ref when the row reopens an exact object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_target_identity_ref: Option<String>,
    /// Placeholder reason when the row reopens a truthful placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_reason_label: Option<String>,
    /// Denial or revalidation reason when reopen is denied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason_label: Option<String>,
    /// Ref to the badge audit row.
    pub badge_audit_id_ref: String,
    /// Ref to the quiet-hours audit row.
    pub quiet_hours_audit_id_ref: String,
    /// Ref to the exact reopen proof row.
    pub reopen_proof_id_ref: String,
    /// Ref to the support-export lineage row.
    pub support_export_lineage_id_ref: String,
    /// True when the case names the authoritative durable object.
    pub has_authoritative_durable_object: bool,
    /// True when the case is represented by a durable row, attention
    /// item, history row, or audit object instead of transient chrome.
    pub represented_by_durable_row: bool,
    /// True when toast-only or spinner-only treatment is explicitly denied.
    pub transient_only_denied: bool,
    /// Count of repeated failures from the same root cause.
    pub repeated_failure_root_cause_count: u32,
    /// True when repeated failures collapse onto one durable item.
    pub repeated_failures_coalesced: bool,
    /// True when OS, lock-screen, or companion payloads are privacy safe.
    pub privacy_safe_external_payload: bool,
    /// True when external shortcuts cannot bypass preview, approval, or
    /// trust logic.
    pub no_shortcut_bypass: bool,
    /// True when support export can reconstruct durable-attention lineage.
    pub support_export_can_reconstruct_lineage: bool,
    /// True when no raw private material crosses the corpus boundary.
    pub raw_private_material_excluded: bool,
    /// Fixture and projection refs that support this case.
    pub fixture_refs: Vec<String>,
    /// Reviewer-facing narrative.
    pub narrative: String,
}

/// Badge-class audit row for one conformance case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeClassAuditRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable audit id.
    pub audit_id: String,
    /// Case id this audit row covers.
    pub case_id: String,
    /// Badge class emitted by the case, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_class: Option<BadgeClass>,
    /// Source class for the badge count.
    pub badge_source_class: BadgeSourceClass,
    /// Dedupe key scheme used for counting.
    pub dedupe_key_scheme: DedupeKeyScheme,
    /// Active count delta contributed by the case.
    pub active_count_delta: u32,
    /// Held or suppressed count delta contributed by the case.
    pub held_or_suppressed_count_delta: u32,
    /// True when mixed-class counting is explicitly rejected.
    pub mixed_class_count_denied: bool,
    /// Stable ref the badge count is derived from.
    pub count_source_ref: String,
    /// True when expanding the badge returns to the authoritative object.
    pub maps_to_authoritative_object: bool,
    /// Support-export row carrying this count.
    pub support_export_ref: String,
}

/// Quiet-hours and suppression audit row for one conformance case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuietHoursSuppressionAuditRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable audit id.
    pub audit_id: String,
    /// Case id this audit row covers.
    pub case_id: String,
    /// Quiet-hours decision class.
    pub decision_class: QuietHoursDecisionClass,
    /// Active quiet-hours or suppression modes.
    pub active_modes: Vec<QuietHoursMode>,
    /// Suppression reasons emitted by the routing layer.
    pub suppression_reasons: Vec<SuppressionReason>,
    /// Surfaces that would have rendered without suppression.
    pub intended_surfaces: Vec<FanoutSurfaceClass>,
    /// Durable surfaces preserved after suppression or dedupe.
    pub preserved_durable_surfaces: Vec<FanoutSurfaceClass>,
    /// True when durable history is preserved.
    pub durable_history_preserved: bool,
    /// True when suppression audit trail exists.
    pub suppression_audit_trail_present: bool,
    /// Stable audit trail ref.
    pub audit_trail_ref: String,
    /// Release rule for held delivery.
    pub release_rule_label: String,
    /// True when a critical safety event bypassed a hold.
    pub critical_safety_bypassed_hold: bool,
    /// True when admin policy narrowed without widening authority.
    pub admin_policy_narrowed_without_widening: bool,
    /// Support-export row carrying suppression facts.
    pub support_export_ref: String,
}

/// Exact-target reopen proof for one conformance case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactReopenProof {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable proof id.
    pub proof_id: String,
    /// Case id this proof covers.
    pub case_id: String,
    /// Reopen class.
    pub reopen_class: ExactReopenClass,
    /// Surfaces whose activation must resolve through this proof.
    pub activation_surfaces: Vec<FanoutSurfaceClass>,
    /// Stable canonical event id.
    pub canonical_event_id: String,
    /// Authoritative object target ref.
    pub canonical_object_target_ref: String,
    /// Exact target identity ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_target_identity_ref: Option<String>,
    /// Placeholder reason, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_reason_label: Option<String>,
    /// Denial or revalidation reason, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason_label: Option<String>,
    /// Command id used for open-details activation.
    pub open_details_command_id: String,
    /// True when generic home fallback is explicitly denied.
    pub generic_home_fallback_denied: bool,
    /// True when activation preserves preview, approval, and trust logic.
    pub preserves_preview_approval_trust_logic: bool,
}

/// Support-export lineage row for one conformance case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportLineageRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable lineage row id.
    pub lineage_id: String,
    /// Case id this lineage row covers.
    pub case_id: String,
    /// Stable canonical event id.
    pub canonical_event_id: String,
    /// Stable durable-job id ref.
    pub durable_job_id_ref: String,
    /// Authoritative object target ref.
    pub canonical_object_target_ref: String,
    /// Source subsystem.
    pub source_subsystem: SourceSubsystem,
    /// Target scope label.
    pub target_scope_label: String,
    /// Durable row state class.
    pub state_class: DurableJobRowStateClass,
    /// Badge audit id ref.
    pub badge_audit_id_ref: String,
    /// Quiet-hours audit id ref.
    pub quiet_hours_audit_id_ref: String,
    /// Reopen proof id ref.
    pub reopen_proof_id_ref: String,
    /// Fanout receipt refs included in the export.
    pub fanout_receipt_refs: Vec<String>,
    /// Audit event refs included in the export.
    pub audit_event_refs: Vec<String>,
    /// Export field refs included in the export.
    pub export_field_refs: Vec<String>,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

/// Aggregate summary for the durable-attention packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableAttentionBetaSummary {
    /// Number of conformance cases.
    pub case_count: usize,
    /// Number of state-machine entries.
    pub state_machine_entry_count: usize,
    /// Number of exact reopen proofs.
    pub exact_reopen_proof_count: usize,
    /// Number of badge audit rows.
    pub badge_audit_row_count: usize,
    /// Number of quiet-hours audit rows.
    pub quiet_hours_audit_row_count: usize,
    /// Number of support lineage rows.
    pub support_lineage_row_count: usize,
    /// Job families present in the corpus.
    pub job_families_present: Vec<DurableAttentionJobFamily>,
    /// State classes present in the corpus.
    pub state_classes_present: Vec<DurableJobRowStateClass>,
    /// Number of cases with repeated failures coalesced.
    pub coalesced_repeated_failure_case_count: usize,
    /// Number of held or suppressed cases.
    pub held_or_suppressed_case_count: usize,
    /// Number of cases proving exact durable-object reopen.
    pub exact_durable_object_reopen_count: usize,
    /// Number of cases proving truthful placeholder reopen.
    pub truthful_placeholder_reopen_count: usize,
    /// Number of cases proving denied or revalidated reopen.
    pub denied_reopen_count: usize,
}

impl DurableAttentionBetaSummary {
    fn from_parts(
        state_machine: &[DurableJobRowStateMachineEntry],
        cases: &[DurableAttentionConformanceCase],
        badge_audit: &[BadgeClassAuditRow],
        quiet_hours_audit: &[QuietHoursSuppressionAuditRow],
        reopen_proofs: &[ExactReopenProof],
        support_export_lineage: &[SupportExportLineageRow],
    ) -> Self {
        let mut families = BTreeSet::new();
        let mut states = BTreeSet::new();
        let mut coalesced = 0usize;
        let mut held_or_suppressed = 0usize;
        let mut exact = 0usize;
        let mut placeholder = 0usize;
        let mut denied = 0usize;
        for case in cases {
            families.insert(case.job_family);
            states.insert(case.state_class);
            if case.repeated_failures_coalesced {
                coalesced += 1;
            }
            if matches!(
                case.quiet_hours_decision,
                QuietHoursDecisionClass::HeldQuietHours
                    | QuietHoursDecisionClass::AdminSuppressed
                    | QuietHoursDecisionClass::CrossClientDeduped
            ) {
                held_or_suppressed += 1;
            }
            match case.reopen_class {
                ExactReopenClass::ExactDurableObject => exact += 1,
                ExactReopenClass::TruthfulPlaceholder => placeholder += 1,
                ExactReopenClass::DeniedRequiresRevalidation => denied += 1,
            }
        }
        Self {
            case_count: cases.len(),
            state_machine_entry_count: state_machine.len(),
            exact_reopen_proof_count: reopen_proofs.len(),
            badge_audit_row_count: badge_audit.len(),
            quiet_hours_audit_row_count: quiet_hours_audit.len(),
            support_lineage_row_count: support_export_lineage.len(),
            job_families_present: families.into_iter().collect(),
            state_classes_present: states.into_iter().collect(),
            coalesced_repeated_failure_case_count: coalesced,
            held_or_suppressed_case_count: held_or_suppressed,
            exact_durable_object_reopen_count: exact,
            truthful_placeholder_reopen_count: placeholder,
            denied_reopen_count: denied,
        }
    }
}

/// Top-level durable-attention beta packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableAttentionBetaPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Generated-at timestamp.
    pub generated_at: String,
    /// Aggregate packet summary.
    pub summary: DurableAttentionBetaSummary,
    /// Durable job-row state machine.
    pub state_machine: Vec<DurableJobRowStateMachineEntry>,
    /// Conformance corpus cases.
    pub cases: Vec<DurableAttentionConformanceCase>,
    /// Badge-class audit rows.
    pub badge_audit: Vec<BadgeClassAuditRow>,
    /// Quiet-hours suppression audit rows.
    pub quiet_hours_audit: Vec<QuietHoursSuppressionAuditRow>,
    /// Exact-target reopen proofs.
    pub exact_reopen_proofs: Vec<ExactReopenProof>,
    /// Support-export lineage rows.
    pub support_export_lineage: Vec<SupportExportLineageRow>,
}

/// Validation error raised when the durable-attention packet fails a
/// conformance invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DurableAttentionBetaValidationError {
    /// A required state-machine class was missing.
    StateMachineCoverageMissing {
        /// Missing state class.
        missing_state_class: String,
    },
    /// A state-machine entry admitted transient-only treatment.
    StateMachineAdmitsTransientOnly {
        /// State class that admitted transient-only treatment.
        state_class: String,
    },
    /// A required job family was missing from the corpus.
    JobFamilyCoverageMissing {
        /// Missing job family.
        missing_job_family: String,
    },
    /// A case referenced a state class that is absent or incompatible.
    CaseStateMachineMismatch {
        /// Case id.
        case_id: String,
        /// Reason label.
        reason: String,
    },
    /// A long-running or reviewable case admitted toast-only treatment.
    DurableObjectMissing {
        /// Case id.
        case_id: String,
        /// Reason label.
        reason: String,
    },
    /// A repeated-failure case failed to coalesce under a safe dedupe scheme.
    RepeatedFailureDedupeMissing {
        /// Case id.
        case_id: String,
    },
    /// Badge audit was missing or drifted from the case.
    BadgeAuditDrift {
        /// Case id.
        case_id: String,
        /// Reason label.
        reason: String,
    },
    /// Quiet-hours audit was missing or admitted silent suppression.
    QuietHoursAuditDrift {
        /// Case id.
        case_id: String,
        /// Reason label.
        reason: String,
    },
    /// Exact reopen proof was missing or failed its class requirements.
    ExactReopenProofInvalid {
        /// Case id.
        case_id: String,
        /// Reason label.
        reason: String,
    },
    /// Support-export lineage was missing or incomplete.
    SupportLineageIncomplete {
        /// Case id.
        case_id: String,
        /// Reason label.
        reason: String,
    },
}

impl std::fmt::Display for DurableAttentionBetaValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StateMachineCoverageMissing {
                missing_state_class,
            } => write!(
                f,
                "durable job-row state machine missing state class {missing_state_class}"
            ),
            Self::StateMachineAdmitsTransientOnly { state_class } => write!(
                f,
                "durable job-row state class {state_class} admits transient-only treatment"
            ),
            Self::JobFamilyCoverageMissing { missing_job_family } => write!(
                f,
                "durable-attention corpus missing job family {missing_job_family}"
            ),
            Self::CaseStateMachineMismatch { case_id, reason } => write!(
                f,
                "case {case_id} does not match the state machine: {reason}"
            ),
            Self::DurableObjectMissing { case_id, reason } => {
                write!(f, "case {case_id} lacks durable object truth: {reason}")
            }
            Self::RepeatedFailureDedupeMissing { case_id } => write!(
                f,
                "case {case_id} repeats one root cause but does not coalesce under grouped or subsystem-phase dedupe"
            ),
            Self::BadgeAuditDrift { case_id, reason } => {
                write!(f, "case {case_id} badge audit drifted: {reason}")
            }
            Self::QuietHoursAuditDrift { case_id, reason } => {
                write!(f, "case {case_id} quiet-hours audit drifted: {reason}")
            }
            Self::ExactReopenProofInvalid { case_id, reason } => {
                write!(f, "case {case_id} exact reopen proof invalid: {reason}")
            }
            Self::SupportLineageIncomplete { case_id, reason } => {
                write!(f, "case {case_id} support lineage incomplete: {reason}")
            }
        }
    }
}

impl std::error::Error for DurableAttentionBetaValidationError {}

/// Validates a durable-attention beta packet.
pub fn validate_durable_attention_beta_packet(
    packet: &DurableAttentionBetaPacket,
) -> Result<(), Vec<DurableAttentionBetaValidationError>> {
    let mut errors = Vec::new();

    let state_machine_by_class: BTreeMap<DurableJobRowStateClass, &DurableJobRowStateMachineEntry> =
        packet
            .state_machine
            .iter()
            .map(|entry| (entry.state_class, entry))
            .collect();
    for required in required_state_classes() {
        match state_machine_by_class.get(&required) {
            Some(entry) => {
                if entry.transient_only_allowed {
                    errors.push(
                        DurableAttentionBetaValidationError::StateMachineAdmitsTransientOnly {
                            state_class: required.as_str().to_owned(),
                        },
                    );
                }
                if !entry.exact_reopen_or_placeholder_required
                    || !entry.support_export_lineage_required
                {
                    errors.push(
                        DurableAttentionBetaValidationError::CaseStateMachineMismatch {
                            case_id: format!("state-machine:{}", required.as_str()),
                            reason: "state machine must require exact reopen and support lineage"
                                .to_owned(),
                        },
                    );
                }
            }
            None => errors.push(
                DurableAttentionBetaValidationError::StateMachineCoverageMissing {
                    missing_state_class: required.as_str().to_owned(),
                },
            ),
        }
    }

    for required in required_job_families() {
        if !packet
            .summary
            .job_families_present
            .iter()
            .any(|family| *family == required)
        {
            errors.push(
                DurableAttentionBetaValidationError::JobFamilyCoverageMissing {
                    missing_job_family: required.as_str().to_owned(),
                },
            );
        }
    }

    let badge_by_case: BTreeMap<&str, &BadgeClassAuditRow> = packet
        .badge_audit
        .iter()
        .map(|row| (row.case_id.as_str(), row))
        .collect();
    let quiet_by_case: BTreeMap<&str, &QuietHoursSuppressionAuditRow> = packet
        .quiet_hours_audit
        .iter()
        .map(|row| (row.case_id.as_str(), row))
        .collect();
    let proof_by_case: BTreeMap<&str, &ExactReopenProof> = packet
        .exact_reopen_proofs
        .iter()
        .map(|row| (row.case_id.as_str(), row))
        .collect();
    let lineage_by_case: BTreeMap<&str, &SupportExportLineageRow> = packet
        .support_export_lineage
        .iter()
        .map(|row| (row.case_id.as_str(), row))
        .collect();

    for case in &packet.cases {
        match state_machine_by_class.get(&case.state_class) {
            Some(entry) => {
                if !entry
                    .valid_activity_partitions
                    .contains(&case.activity_partition)
                {
                    errors.push(
                        DurableAttentionBetaValidationError::CaseStateMachineMismatch {
                            case_id: case.case_id.clone(),
                            reason: format!(
                                "partition {} is not valid for state {}",
                                case.activity_partition.as_str(),
                                case.state_class.as_str()
                            ),
                        },
                    );
                }
            }
            None => errors.push(
                DurableAttentionBetaValidationError::CaseStateMachineMismatch {
                    case_id: case.case_id.clone(),
                    reason: "state class is absent from state machine".to_owned(),
                },
            ),
        }

        if !case.has_authoritative_durable_object
            || !case.represented_by_durable_row
            || !case.transient_only_denied
        {
            errors.push(DurableAttentionBetaValidationError::DurableObjectMissing {
                case_id: case.case_id.clone(),
                reason: "case must name a durable object, durable row, and transient-only denial"
                    .to_owned(),
            });
        }
        if !case.support_export_can_reconstruct_lineage || !case.raw_private_material_excluded {
            errors.push(
                DurableAttentionBetaValidationError::SupportLineageIncomplete {
                    case_id: case.case_id.clone(),
                    reason: "support export must reconstruct lineage without raw private material"
                        .to_owned(),
                },
            );
        }
        if !case.privacy_safe_external_payload || !case.no_shortcut_bypass {
            errors.push(DurableAttentionBetaValidationError::DurableObjectMissing {
                case_id: case.case_id.clone(),
                reason: "external payloads must be privacy safe and shortcut-safe".to_owned(),
            });
        }
        if case.repeated_failure_root_cause_count > 1
            && (!case.repeated_failures_coalesced
                || !matches!(
                    case.dedupe_key_scheme,
                    DedupeKeyScheme::GroupedBurstId | DedupeKeyScheme::SubsystemPlusObjectPlusPhase
                ))
        {
            errors.push(
                DurableAttentionBetaValidationError::RepeatedFailureDedupeMissing {
                    case_id: case.case_id.clone(),
                },
            );
        }

        match badge_by_case.get(case.case_id.as_str()) {
            Some(row) => {
                if row.badge_class != case.badge_class {
                    errors.push(DurableAttentionBetaValidationError::BadgeAuditDrift {
                        case_id: case.case_id.clone(),
                        reason: "badge class differs from case".to_owned(),
                    });
                }
                if row.badge_source_class != case.badge_source_class {
                    errors.push(DurableAttentionBetaValidationError::BadgeAuditDrift {
                        case_id: case.case_id.clone(),
                        reason: "badge source class differs from case".to_owned(),
                    });
                }
                if row.dedupe_key_scheme != case.dedupe_key_scheme {
                    errors.push(DurableAttentionBetaValidationError::BadgeAuditDrift {
                        case_id: case.case_id.clone(),
                        reason: "dedupe key scheme differs from case".to_owned(),
                    });
                }
                if !row.mixed_class_count_denied || !row.maps_to_authoritative_object {
                    errors.push(DurableAttentionBetaValidationError::BadgeAuditDrift {
                        case_id: case.case_id.clone(),
                        reason: "badge audit must deny mixed-class counts and map to object"
                            .to_owned(),
                    });
                }
                if matches!(row.badge_source_class, BadgeSourceClass::NotABadgeSource)
                    && (row.active_count_delta > 0 || row.held_or_suppressed_count_delta > 0)
                {
                    errors.push(DurableAttentionBetaValidationError::BadgeAuditDrift {
                        case_id: case.case_id.clone(),
                        reason: "not_a_badge_source must not contribute counts".to_owned(),
                    });
                }
            }
            None => errors.push(DurableAttentionBetaValidationError::BadgeAuditDrift {
                case_id: case.case_id.clone(),
                reason: "missing badge audit row".to_owned(),
            }),
        }

        match quiet_by_case.get(case.case_id.as_str()) {
            Some(row) => {
                if row.decision_class != case.quiet_hours_decision {
                    errors.push(DurableAttentionBetaValidationError::QuietHoursAuditDrift {
                        case_id: case.case_id.clone(),
                        reason: "decision class differs from case".to_owned(),
                    });
                }
                if !row.durable_history_preserved || !row.suppression_audit_trail_present {
                    errors.push(DurableAttentionBetaValidationError::QuietHoursAuditDrift {
                        case_id: case.case_id.clone(),
                        reason: "suppression must preserve durable history and audit trail"
                            .to_owned(),
                    });
                }
                if matches!(
                    row.decision_class,
                    QuietHoursDecisionClass::HeldQuietHours
                        | QuietHoursDecisionClass::AdminSuppressed
                ) && row.preserved_durable_surfaces.is_empty()
                {
                    errors.push(DurableAttentionBetaValidationError::QuietHoursAuditDrift {
                        case_id: case.case_id.clone(),
                        reason: "held or suppressed rows must preserve a durable surface"
                            .to_owned(),
                    });
                }
            }
            None => errors.push(DurableAttentionBetaValidationError::QuietHoursAuditDrift {
                case_id: case.case_id.clone(),
                reason: "missing quiet-hours audit row".to_owned(),
            }),
        }

        match proof_by_case.get(case.case_id.as_str()) {
            Some(proof) => {
                if proof.reopen_class != case.reopen_class {
                    errors.push(
                        DurableAttentionBetaValidationError::ExactReopenProofInvalid {
                            case_id: case.case_id.clone(),
                            reason: "reopen class differs from case".to_owned(),
                        },
                    );
                }
                if proof.canonical_event_id != case.canonical_event_id
                    || proof.canonical_object_target_ref != case.canonical_object_target_ref
                    || !proof.generic_home_fallback_denied
                    || !proof.preserves_preview_approval_trust_logic
                {
                    errors.push(
                        DurableAttentionBetaValidationError::ExactReopenProofInvalid {
                            case_id: case.case_id.clone(),
                            reason: "proof must preserve canonical identity, deny generic home, and keep review logic"
                                .to_owned(),
                        },
                    );
                }
                match proof.reopen_class {
                    ExactReopenClass::ExactDurableObject => {
                        if proof
                            .exact_target_identity_ref
                            .as_deref()
                            .map_or(true, str::is_empty)
                        {
                            errors.push(
                                DurableAttentionBetaValidationError::ExactReopenProofInvalid {
                                    case_id: case.case_id.clone(),
                                    reason: "exact reopen requires exact target identity"
                                        .to_owned(),
                                },
                            );
                        }
                    }
                    ExactReopenClass::TruthfulPlaceholder => {
                        if proof
                            .placeholder_reason_label
                            .as_deref()
                            .map_or(true, str::is_empty)
                        {
                            errors.push(
                                DurableAttentionBetaValidationError::ExactReopenProofInvalid {
                                    case_id: case.case_id.clone(),
                                    reason: "placeholder reopen requires a reason label".to_owned(),
                                },
                            );
                        }
                    }
                    ExactReopenClass::DeniedRequiresRevalidation => {
                        if proof
                            .denial_reason_label
                            .as_deref()
                            .map_or(true, str::is_empty)
                        {
                            errors.push(
                                DurableAttentionBetaValidationError::ExactReopenProofInvalid {
                                    case_id: case.case_id.clone(),
                                    reason: "denied reopen requires a reason label".to_owned(),
                                },
                            );
                        }
                    }
                }
            }
            None => errors.push(
                DurableAttentionBetaValidationError::ExactReopenProofInvalid {
                    case_id: case.case_id.clone(),
                    reason: "missing exact reopen proof".to_owned(),
                },
            ),
        }

        match lineage_by_case.get(case.case_id.as_str()) {
            Some(lineage) => {
                if lineage.canonical_event_id != case.canonical_event_id
                    || lineage.durable_job_id_ref != case.durable_job_id_ref
                    || lineage.canonical_object_target_ref != case.canonical_object_target_ref
                    || lineage.fanout_receipt_refs.is_empty()
                    || lineage.audit_event_refs.is_empty()
                    || lineage.export_field_refs.is_empty()
                    || !lineage.raw_private_material_excluded
                {
                    errors.push(DurableAttentionBetaValidationError::SupportLineageIncomplete {
                        case_id: case.case_id.clone(),
                        reason: "lineage row must carry identity, fanout, audit, export fields, and redaction"
                            .to_owned(),
                    });
                }
            }
            None => errors.push(
                DurableAttentionBetaValidationError::SupportLineageIncomplete {
                    case_id: case.case_id.clone(),
                    reason: "missing support lineage row".to_owned(),
                },
            ),
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Builds the seeded durable-attention beta packet.
pub fn seeded_durable_attention_beta_packet() -> DurableAttentionBetaPacket {
    let activity_page = seeded_activity_center_beta_page();
    let notification_page = seeded_notification_privacy_beta_page();
    let state_machine = seeded_state_machine();
    let cases = seeded_cases(&activity_page.rows, &notification_page.rows);
    let badge_audit = cases.iter().map(make_badge_audit).collect::<Vec<_>>();
    let quiet_hours_audit = cases.iter().map(make_quiet_hours_audit).collect::<Vec<_>>();
    let exact_reopen_proofs = cases.iter().map(make_reopen_proof).collect::<Vec<_>>();
    let support_export_lineage = cases.iter().map(make_support_lineage).collect::<Vec<_>>();
    let summary = DurableAttentionBetaSummary::from_parts(
        &state_machine,
        &cases,
        &badge_audit,
        &quiet_hours_audit,
        &exact_reopen_proofs,
        &support_export_lineage,
    );
    DurableAttentionBetaPacket {
        record_kind: DURABLE_ATTENTION_BETA_PACKET_RECORD_KIND.to_owned(),
        schema_version: DURABLE_ATTENTION_BETA_SCHEMA_VERSION,
        shared_contract_ref: DURABLE_ATTENTION_BETA_SHARED_CONTRACT_REF.to_owned(),
        packet_id: "shell:durable-attention:beta:packet:default".to_owned(),
        generated_at: "2026-05-17T00:00:00Z".to_owned(),
        summary,
        state_machine,
        cases,
        badge_audit,
        quiet_hours_audit,
        exact_reopen_proofs,
        support_export_lineage,
    }
}

fn required_state_classes() -> [DurableJobRowStateClass; 7] {
    [
        DurableJobRowStateClass::Running,
        DurableJobRowStateClass::QueuedWaiting,
        DurableJobRowStateClass::NeedsApproval,
        DurableJobRowStateClass::Completed,
        DurableJobRowStateClass::Failed,
        DurableJobRowStateClass::Cancelled,
        DurableJobRowStateClass::HistoryOnly,
    ]
}

fn required_job_families() -> [DurableAttentionJobFamily; 11] {
    [
        DurableAttentionJobFamily::Indexing,
        DurableAttentionJobFamily::Restore,
        DurableAttentionJobFamily::InstallUpdateDownload,
        DurableAttentionJobFamily::RemoteReconnect,
        DurableAttentionJobFamily::TaskRun,
        DurableAttentionJobFamily::TestRun,
        DurableAttentionJobFamily::DebugSession,
        DurableAttentionJobFamily::AiReview,
        DurableAttentionJobFamily::GitReview,
        DurableAttentionJobFamily::CompanionHandoff,
        DurableAttentionJobFamily::AdminPolicy,
    ]
}

fn seeded_state_machine() -> Vec<DurableJobRowStateMachineEntry> {
    use ActivityPartition as Partition;
    use DurableJobRowStateClass as State;
    vec![
        state_machine_entry(
            State::Running,
            &[Partition::CurrentWork, Partition::SuppressedHeld],
            &["phase_label", "age_label", "actor_or_subsystem_label"],
            &["state_label", "phase", "target", "open_details"],
            &["open_details", "cancel_or_explain"],
        ),
        state_machine_entry(
            State::QueuedWaiting,
            &[Partition::CurrentWork],
            &["queue_reason_label", "expected_boundary_class", "age_label"],
            &["state_label", "queue_reason", "target", "open_details"],
            &["open_details", "cancel_or_explain"],
        ),
        state_machine_entry(
            State::NeedsApproval,
            &[Partition::NeedsAttention],
            &["approval_source_label", "waiting_for_label", "age_label"],
            &["state_label", "approval_owner", "target", "evidence_link"],
            &["open_details", "review_approval", "cancel_or_explain"],
        ),
        state_machine_entry(
            State::Completed,
            &[Partition::Completed],
            &["completion_summary_label", "finished_at", "age_label"],
            &["state_label", "finish_time", "target", "open_details"],
            &["open_details", "open_history"],
        ),
        state_machine_entry(
            State::Failed,
            &[Partition::NeedsAttention],
            &[
                "failure_or_partial_summary_label",
                "finished_at_or_blocker",
                "age_label",
            ],
            &["state_label", "failure_reason", "target", "evidence_link"],
            &["open_details", "retry_or_explain", "acknowledge"],
        ),
        state_machine_entry(
            State::Cancelled,
            &[Partition::Completed],
            &["cancellation_actor_label", "finished_at", "age_label"],
            &[
                "state_label",
                "cancellation_reason",
                "target",
                "history_link",
            ],
            &["open_details", "open_history", "retry_or_explain"],
        ),
        state_machine_entry(
            State::HistoryOnly,
            &[Partition::Completed, Partition::SuppressedHeld],
            &["history_reason_label", "age_label", "audit_event_refs"],
            &[
                "state_label",
                "history_reason",
                "target_or_placeholder",
                "audit_link",
            ],
            &["open_details", "open_history", "export_support"],
        ),
    ]
}

fn state_machine_entry(
    state_class: DurableJobRowStateClass,
    partitions: &[ActivityPartition],
    required_phase_fields: &[&str],
    required_visible_cues: &[&str],
    required_actions: &[&str],
) -> DurableJobRowStateMachineEntry {
    DurableJobRowStateMachineEntry {
        record_kind: DURABLE_JOB_ROW_STATE_MACHINE_RECORD_KIND.to_owned(),
        schema_version: DURABLE_ATTENTION_BETA_SCHEMA_VERSION,
        shared_contract_ref: DURABLE_ATTENTION_BETA_SHARED_CONTRACT_REF.to_owned(),
        state_class,
        valid_activity_partitions: partitions.to_vec(),
        required_phase_fields: strings(required_phase_fields),
        required_visible_cues: strings(required_visible_cues),
        required_actions: strings(required_actions),
        exact_reopen_or_placeholder_required: true,
        support_export_lineage_required: true,
        transient_only_allowed: false,
    }
}

fn seeded_cases(
    activity_rows: &[ActivityCenterBetaRow],
    notification_rows: &[NotificationPrivacyBetaRow],
) -> Vec<DurableAttentionConformanceCase> {
    let indexing = activity_row(activity_rows, "indexing:hot-set");
    let restore_completed = activity_row(activity_rows, "restore:last-session");
    let install = activity_row(activity_rows, "install:package-update");
    let task = activity_row(activity_rows, "task:dev-server");
    let test = activity_row(activity_rows, "test:pytest-suite");
    let git = activity_row(activity_rows, "git:publish-feature-branch");
    let restore_placeholder = activity_row(activity_rows, "restore:archived-session");
    let coalesced = notification_row(
        notification_rows,
        NotificationPrivacyBetaRowClass::CoalescedRepeatedFailure,
    );
    let companion = notification_row(
        notification_rows,
        NotificationPrivacyBetaRowClass::CompanionCrossClientFanout,
    );
    let admin = notification_row(
        notification_rows,
        NotificationPrivacyBetaRowClass::AdminPolicySuppressed,
    );

    vec![
        from_activity_row(
            "case:indexing-hot-set",
            "Indexing active workspace hot set",
            DurableAttentionJobFamily::Indexing,
            DurableJobRowStateClass::Running,
            indexing,
            Some(BadgeClass::DurableRunningCount),
            BadgeSourceClass::DerivedFromEnvelopeState,
            DedupeKeyScheme::CanonicalEventId,
            QuietHoursDecisionClass::NotSuppressed,
            "Indexer",
            "local_background_worker",
            "Active workspace",
            "Scanning hot set",
            "Running 2 min",
            1,
            false,
            "fixtures/ux/m3/activity_center/rows.json",
            "Indexing is represented by the activity-center beta row; status and toast mirrors cannot replace it.",
        ),
        from_activity_row(
            "case:restore-completed",
            "Restore completion remains reviewable",
            DurableAttentionJobFamily::Restore,
            DurableJobRowStateClass::Completed,
            restore_completed,
            Some(BadgeClass::CompletionUnread),
            BadgeSourceClass::DerivedFromEnvelopeState,
            DedupeKeyScheme::CanonicalEventId,
            QuietHoursDecisionClass::NotSuppressed,
            "Shell",
            "local_restore",
            "Last session workspace",
            "Restore completed",
            "Finished 1 min ago",
            1,
            false,
            "fixtures/ux/m3/activity_center/rows.json",
            "Restore completion remains a durable row after the success toast expires.",
        ),
        from_activity_row(
            "case:install-update-partial",
            "Package update partial result needs review",
            DurableAttentionJobFamily::InstallUpdateDownload,
            DurableJobRowStateClass::Failed,
            install,
            Some(BadgeClass::NeedsReview),
            BadgeSourceClass::DerivedFromCanonicalObject,
            DedupeKeyScheme::CanonicalObjectTargetPlusEventClass,
            QuietHoursDecisionClass::NotSuppressed,
            "Install/update",
            "local_with_network",
            "Workspace extensions",
            "Review failed items",
            "Needs review 3 min",
            1,
            false,
            "fixtures/ux/m3/activity_center/rows.json",
            "The partial package update contributes to needs-review by canonical object, not by stacked failure toasts.",
        ),
        contract_case(CaseSpec {
            case_id: "case:download-cancelled",
            case_label: "Cancelled download keeps history and restart path",
            job_family: DurableAttentionJobFamily::InstallUpdateDownload,
            coverage_source: DurableAttentionCoverageSource::ContractFixture,
            source_projection_ref: "fixtures/ux/durable_job_cases/cancelled_download.json",
            state_class: DurableJobRowStateClass::Cancelled,
            activity_partition: ActivityPartition::Completed,
            source_subsystem: SourceSubsystem::InstallUpdateAttach,
            actor_or_subsystem_label: "Local operator",
            execution_origin_class: "user_initiated",
            scope_label: "Artifact bundle",
            phase_label: "Cancelled",
            age_label: "Cancelled 3 sec ago",
            durable_job_id_ref: "ux:durable-job:download-upload:artifact-bundle",
            canonical_event_id: "ux:event:download-upload:artifact-bundle",
            canonical_object_target_ref: "obj:download-upload:artifact-bundle",
            badge_class: None,
            badge_source_class: BadgeSourceClass::NotABadgeSource,
            dedupe_key_scheme: DedupeKeyScheme::CanonicalEventId,
            quiet_hours_decision: QuietHoursDecisionClass::NotSuppressed,
            reopen_class: ExactReopenClass::ExactDurableObject,
            exact_target_identity_ref: Some("ux:durable-job:download-upload:artifact-bundle"),
            placeholder_reason_label: None,
            denial_reason_label: None,
            repeated_failure_root_cause_count: 1,
            repeated_failures_coalesced: false,
            fixture_refs: vec!["fixtures/ux/durable_job_cases/cancelled_download.json"],
            narrative: "Cancelled download rows stay in completed/history lanes with restart and evidence paths; no transient spinner is authoritative.",
        }),
        contract_case(CaseSpec {
            case_id: "case:remote-reconnect-held",
            case_label: "Remote reconnect held by quiet hours preserves row",
            job_family: DurableAttentionJobFamily::RemoteReconnect,
            coverage_source: DurableAttentionCoverageSource::ContractFixture,
            source_projection_ref: "fixtures/ux/durable_attention_cases/transport_reconnect.yaml",
            state_class: DurableJobRowStateClass::Running,
            activity_partition: ActivityPartition::SuppressedHeld,
            source_subsystem: SourceSubsystem::SyncMirror,
            actor_or_subsystem_label: "Sync transport",
            execution_origin_class: "remote_reconnect",
            scope_label: "Remote mirror",
            phase_label: "Reconnecting",
            age_label: "Held 8 min",
            durable_job_id_ref: "ux:durable-job:transport:remote-mirror",
            canonical_event_id: "ux:event:transport:remote-mirror:reconnecting",
            canonical_object_target_ref: "obj:transport-session:remote-mirror",
            badge_class: Some(BadgeClass::HeldOrSuppressedCount),
            badge_source_class: BadgeSourceClass::AggregatedGroupedBurst,
            dedupe_key_scheme: DedupeKeyScheme::SubsystemPlusObjectPlusPhase,
            quiet_hours_decision: QuietHoursDecisionClass::HeldQuietHours,
            reopen_class: ExactReopenClass::ExactDurableObject,
            exact_target_identity_ref: Some("ux:reopen:transport:remote-mirror:attention"),
            placeholder_reason_label: None,
            denial_reason_label: None,
            repeated_failure_root_cause_count: 3,
            repeated_failures_coalesced: true,
            fixture_refs: vec!["fixtures/ux/durable_attention_cases/transport_reconnect.yaml"],
            narrative: "Remote reconnect repeats collapse into one held durable row whose quiet-hours audit records the held fanout.",
        }),
        from_activity_row(
            "case:task-queued",
            "Task queued with visible boundary reason",
            DurableAttentionJobFamily::TaskRun,
            DurableJobRowStateClass::QueuedWaiting,
            task,
            Some(BadgeClass::DurableRunningCount),
            BadgeSourceClass::DerivedFromEnvelopeState,
            DedupeKeyScheme::CanonicalEventId,
            QuietHoursDecisionClass::NotSuppressed,
            "Task runner",
            "local_execution_profile",
            "tasks.json dev-server",
            "Waiting for execution profile",
            "Queued 30 sec",
            1,
            false,
            "fixtures/ux/m3/activity_center/rows.json",
            "Queued task rows name the expected execution boundary and stay inspectable without a toast.",
        ),
        from_activity_row(
            "case:test-failed-coalesced",
            "Repeated test failure coalesces into one failed row",
            DurableAttentionJobFamily::TestRun,
            DurableJobRowStateClass::Failed,
            test,
            Some(BadgeClass::FailedRuns),
            BadgeSourceClass::AggregatedGroupedBurst,
            coalesced.dedupe_key_scheme,
            QuietHoursDecisionClass::NotSuppressed,
            "Test runner",
            "local_test_runner",
            "pytest tests",
            "Failed",
            "Failed 1 min ago",
            coalesced.occurrence_count.max(2),
            true,
            "fixtures/ux/m3/notification_privacy/rows.json",
            "Repeated test failures share one root cause and increment the durable occurrence count rather than stacking toasts.",
        ),
        contract_case(CaseSpec {
            case_id: "case:debug-session-running",
            case_label: "Debug session survives focus loss",
            job_family: DurableAttentionJobFamily::DebugSession,
            coverage_source: DurableAttentionCoverageSource::ContractFixture,
            source_projection_ref: "fixtures/ux/durable_attention_cases/debug_session.yaml",
            state_class: DurableJobRowStateClass::Running,
            activity_partition: ActivityPartition::CurrentWork,
            source_subsystem: SourceSubsystem::DebugSession,
            actor_or_subsystem_label: "Debugger",
            execution_origin_class: "local_debug_adapter",
            scope_label: "Debug session",
            phase_label: "Paused at breakpoint",
            age_label: "Running 6 min",
            durable_job_id_ref: "ux:durable-job:debug-session:api-worker",
            canonical_event_id: "ux:event:debug-session:api-worker:running",
            canonical_object_target_ref: "obj:debug-session:api-worker",
            badge_class: Some(BadgeClass::DurableRunningCount),
            badge_source_class: BadgeSourceClass::DerivedFromEnvelopeState,
            dedupe_key_scheme: DedupeKeyScheme::CanonicalEventId,
            quiet_hours_decision: QuietHoursDecisionClass::NotSuppressed,
            reopen_class: ExactReopenClass::ExactDurableObject,
            exact_target_identity_ref: Some("ux:reopen:debug-session:api-worker:durable"),
            placeholder_reason_label: None,
            denial_reason_label: None,
            repeated_failure_root_cause_count: 1,
            repeated_failures_coalesced: false,
            fixture_refs: vec!["fixtures/ux/durable_attention_cases/debug_session.yaml"],
            narrative: "Debug sessions are durable job rows with evidence-only reopen rather than hidden adapter state.",
        }),
        contract_case(CaseSpec {
            case_id: "case:ai-review-awaiting-approval",
            case_label: "AI review waits on trust approval without shortcut bypass",
            job_family: DurableAttentionJobFamily::AiReview,
            coverage_source: DurableAttentionCoverageSource::ContractFixture,
            source_projection_ref: "fixtures/ux/durable_job_cases/waiting_input_review_approval.json",
            state_class: DurableJobRowStateClass::NeedsApproval,
            activity_partition: ActivityPartition::NeedsAttention,
            source_subsystem: SourceSubsystem::AiApply,
            actor_or_subsystem_label: "AI apply",
            execution_origin_class: "user_initiated_review",
            scope_label: "Workspace refactor",
            phase_label: "Awaiting trust review",
            age_label: "Waiting 4 min",
            durable_job_id_ref: "ux:durable-job:ai-apply:refactor-pack",
            canonical_event_id: "ux:event:ai-apply:refactor-pack",
            canonical_object_target_ref: "obj:review:ai-apply:refactor-pack",
            badge_class: Some(BadgeClass::NeedsReview),
            badge_source_class: BadgeSourceClass::DerivedFromCanonicalObject,
            dedupe_key_scheme: DedupeKeyScheme::CanonicalObjectTargetPlusEventClass,
            quiet_hours_decision: QuietHoursDecisionClass::NotSuppressed,
            reopen_class: ExactReopenClass::ExactDurableObject,
            exact_target_identity_ref: Some("ux:durable-job:ai-apply:refactor-pack"),
            placeholder_reason_label: None,
            denial_reason_label: None,
            repeated_failure_root_cause_count: 1,
            repeated_failures_coalesced: false,
            fixture_refs: vec![
                "fixtures/ux/durable_job_cases/waiting_input_review_approval.json",
                "fixtures/ux/durable_attention_cases/ai_approval_pending.yaml",
            ],
            narrative: "AI review rows require in-product approval; OS or companion actions can reopen but cannot approve directly.",
        }),
        from_activity_row(
            "case:git-publish-denied",
            "Git publish denial reopens explained revalidation",
            DurableAttentionJobFamily::GitReview,
            DurableJobRowStateClass::Failed,
            git,
            Some(BadgeClass::NeedsReview),
            BadgeSourceClass::DerivedFromCanonicalObject,
            DedupeKeyScheme::CanonicalObjectTargetPlusEventClass,
            QuietHoursDecisionClass::CriticalBypass,
            "Provider policy",
            "provider_publish",
            "feature branch upstream",
            "Denied by managed policy",
            "Denied 45 sec ago",
            1,
            false,
            "fixtures/ux/m3/activity_center/rows.json",
            "Provider publish denial opens an explained revalidation path and cannot be retried from external chrome.",
        ),
        from_activity_row(
            "case:restore-placeholder-history",
            "Archived restore reopens a truthful placeholder",
            DurableAttentionJobFamily::Restore,
            DurableJobRowStateClass::HistoryOnly,
            restore_placeholder,
            None,
            BadgeSourceClass::NotABadgeSource,
            DedupeKeyScheme::CanonicalEventId,
            QuietHoursDecisionClass::NotSuppressed,
            "Shell",
            "local_restore_history",
            "Archived session",
            "History only",
            "Archived 9 days ago",
            1,
            false,
            "fixtures/ux/m3/activity_center/rows.json",
            "Archived restore history opens a placeholder that explains retention instead of a generic home surface.",
        ),
        from_notification_row(
            "case:companion-cross-client",
            "Companion fanout dedupes under canonical event",
            DurableAttentionJobFamily::CompanionHandoff,
            DurableJobRowStateClass::NeedsApproval,
            companion,
            Some(BadgeClass::SessionRequests),
            BadgeSourceClass::AggregatedGroupedBurst,
            QuietHoursDecisionClass::CrossClientDeduped,
            "Companion handoff",
            "cross_client_fanout",
            "Companion surface",
            "Awaiting in-product open",
            "Delivered 20 sec ago",
            "fixtures/ux/m3/notification_privacy/rows.json",
            "Companion and desktop rows share one canonical event id; dismissing desktop attention collapses companion fanout.",
        ),
        from_notification_row(
            "case:admin-policy-suppressed",
            "Admin suppression preserves audit-only durable lineage",
            DurableAttentionJobFamily::AdminPolicy,
            DurableJobRowStateClass::HistoryOnly,
            admin,
            Some(BadgeClass::HeldOrSuppressedCount),
            BadgeSourceClass::DerivedFromEnvelopeState,
            QuietHoursDecisionClass::AdminSuppressed,
            "Admin policy",
            "managed_policy",
            "Managed workspace",
            "Suppressed by policy",
            "Suppressed 5 min",
            "fixtures/ux/m3/notification_privacy/rows.json",
            "Admin suppression narrows fanout while preserving durable audit and support-export lineage.",
        ),
    ]
}

struct CaseSpec<'a> {
    case_id: &'a str,
    case_label: &'a str,
    job_family: DurableAttentionJobFamily,
    coverage_source: DurableAttentionCoverageSource,
    source_projection_ref: &'a str,
    state_class: DurableJobRowStateClass,
    activity_partition: ActivityPartition,
    source_subsystem: SourceSubsystem,
    actor_or_subsystem_label: &'a str,
    execution_origin_class: &'a str,
    scope_label: &'a str,
    phase_label: &'a str,
    age_label: &'a str,
    durable_job_id_ref: &'a str,
    canonical_event_id: &'a str,
    canonical_object_target_ref: &'a str,
    badge_class: Option<BadgeClass>,
    badge_source_class: BadgeSourceClass,
    dedupe_key_scheme: DedupeKeyScheme,
    quiet_hours_decision: QuietHoursDecisionClass,
    reopen_class: ExactReopenClass,
    exact_target_identity_ref: Option<&'a str>,
    placeholder_reason_label: Option<&'a str>,
    denial_reason_label: Option<&'a str>,
    repeated_failure_root_cause_count: u32,
    repeated_failures_coalesced: bool,
    fixture_refs: Vec<&'a str>,
    narrative: &'a str,
}

fn contract_case(spec: CaseSpec<'_>) -> DurableAttentionConformanceCase {
    DurableAttentionConformanceCase {
        record_kind: DURABLE_ATTENTION_CONFORMANCE_CASE_RECORD_KIND.to_owned(),
        schema_version: DURABLE_ATTENTION_BETA_SCHEMA_VERSION,
        shared_contract_ref: DURABLE_ATTENTION_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id: spec.case_id.to_owned(),
        case_label: spec.case_label.to_owned(),
        job_family: spec.job_family,
        coverage_source: spec.coverage_source,
        source_projection_ref: spec.source_projection_ref.to_owned(),
        state_class: spec.state_class,
        activity_partition: spec.activity_partition,
        source_subsystem: spec.source_subsystem,
        actor_or_subsystem_label: spec.actor_or_subsystem_label.to_owned(),
        execution_origin_class: spec.execution_origin_class.to_owned(),
        scope_label: spec.scope_label.to_owned(),
        phase_label: spec.phase_label.to_owned(),
        age_label: spec.age_label.to_owned(),
        durable_job_id_ref: spec.durable_job_id_ref.to_owned(),
        canonical_event_id: spec.canonical_event_id.to_owned(),
        canonical_object_target_ref: spec.canonical_object_target_ref.to_owned(),
        badge_class: spec.badge_class,
        badge_source_class: spec.badge_source_class,
        dedupe_key_scheme: spec.dedupe_key_scheme,
        quiet_hours_decision: spec.quiet_hours_decision,
        reopen_class: spec.reopen_class,
        exact_target_identity_ref: spec.exact_target_identity_ref.map(ToOwned::to_owned),
        placeholder_reason_label: spec.placeholder_reason_label.map(ToOwned::to_owned),
        denial_reason_label: spec.denial_reason_label.map(ToOwned::to_owned),
        badge_audit_id_ref: id_ref("badge-audit", spec.case_id),
        quiet_hours_audit_id_ref: id_ref("quiet-hours-audit", spec.case_id),
        reopen_proof_id_ref: id_ref("reopen-proof", spec.case_id),
        support_export_lineage_id_ref: id_ref("support-lineage", spec.case_id),
        has_authoritative_durable_object: true,
        represented_by_durable_row: true,
        transient_only_denied: true,
        repeated_failure_root_cause_count: spec.repeated_failure_root_cause_count,
        repeated_failures_coalesced: spec.repeated_failures_coalesced,
        privacy_safe_external_payload: true,
        no_shortcut_bypass: true,
        support_export_can_reconstruct_lineage: true,
        raw_private_material_excluded: true,
        fixture_refs: spec
            .fixture_refs
            .into_iter()
            .map(ToOwned::to_owned)
            .collect(),
        narrative: spec.narrative.to_owned(),
    }
}

fn from_activity_row(
    case_id: &str,
    case_label: &str,
    job_family: DurableAttentionJobFamily,
    state_class: DurableJobRowStateClass,
    row: &ActivityCenterBetaRow,
    badge_class: Option<BadgeClass>,
    badge_source_class: BadgeSourceClass,
    dedupe_key_scheme: DedupeKeyScheme,
    quiet_hours_decision: QuietHoursDecisionClass,
    actor_or_subsystem_label: &str,
    execution_origin_class: &str,
    scope_label: &str,
    phase_label: &str,
    age_label: &str,
    repeated_failure_root_cause_count: u32,
    repeated_failures_coalesced: bool,
    fixture_ref: &str,
    narrative: &str,
) -> DurableAttentionConformanceCase {
    let reopen_class = ExactReopenClass::from(row.reopen_class);
    let canonical_object_target_ref = row
        .exact_target_identity_ref
        .clone()
        .unwrap_or_else(|| format!("obj:{}", row.durable_job_id));
    contract_case(CaseSpec {
        case_id,
        case_label,
        job_family,
        coverage_source: DurableAttentionCoverageSource::ActivityCenterBetaRow,
        source_projection_ref: &format!("{fixture_ref}#{}", row.row_id),
        state_class,
        activity_partition: match state_class {
            DurableJobRowStateClass::Running | DurableJobRowStateClass::QueuedWaiting => {
                row.activity_partition
            }
            DurableJobRowStateClass::HistoryOnly => row.activity_partition,
            _ => row.activity_partition,
        },
        source_subsystem: row.source_subsystem,
        actor_or_subsystem_label,
        execution_origin_class,
        scope_label,
        phase_label,
        age_label,
        durable_job_id_ref: &row.durable_job_id,
        canonical_event_id: &row.canonical_event_id,
        canonical_object_target_ref: &canonical_object_target_ref,
        badge_class,
        badge_source_class,
        dedupe_key_scheme,
        quiet_hours_decision,
        reopen_class,
        exact_target_identity_ref: row.exact_target_identity_ref.as_deref(),
        placeholder_reason_label: row.placeholder_reason_label.as_deref(),
        denial_reason_label: row.denial_reason_label.as_deref(),
        repeated_failure_root_cause_count,
        repeated_failures_coalesced,
        fixture_refs: vec![fixture_ref],
        narrative,
    })
}

fn from_notification_row(
    case_id: &str,
    case_label: &str,
    job_family: DurableAttentionJobFamily,
    state_class: DurableJobRowStateClass,
    row: &NotificationPrivacyBetaRow,
    badge_class: Option<BadgeClass>,
    badge_source_class: BadgeSourceClass,
    quiet_hours_decision: QuietHoursDecisionClass,
    actor_or_subsystem_label: &str,
    execution_origin_class: &str,
    scope_label: &str,
    phase_label: &str,
    age_label: &str,
    fixture_ref: &str,
    narrative: &str,
) -> DurableAttentionConformanceCase {
    let exact_ref = row.reopen_target.exact_target_identity_ref.as_deref();
    let reopen_class = if row
        .reopen_target
        .revalidation_required_reason_label
        .as_ref()
        .is_some_and(|label| !label.is_empty())
    {
        ExactReopenClass::DeniedRequiresRevalidation
    } else if row
        .reopen_target
        .placeholder_announcement_label
        .as_ref()
        .is_some_and(|label| !label.is_empty())
    {
        ExactReopenClass::TruthfulPlaceholder
    } else {
        ExactReopenClass::ExactDurableObject
    };
    let durable_job_id_ref = exact_ref
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| format!("ux:durable-job:notification:{}", row.row_id));
    contract_case(CaseSpec {
        case_id,
        case_label,
        job_family,
        coverage_source: DurableAttentionCoverageSource::NotificationPrivacyBetaRow,
        source_projection_ref: &format!("{fixture_ref}#{}", row.row_id),
        state_class,
        activity_partition: match quiet_hours_decision {
            QuietHoursDecisionClass::AdminSuppressed => ActivityPartition::SuppressedHeld,
            QuietHoursDecisionClass::CrossClientDeduped => ActivityPartition::NeedsAttention,
            _ => ActivityPartition::NeedsAttention,
        },
        source_subsystem: row.source_subsystem,
        actor_or_subsystem_label,
        execution_origin_class,
        scope_label,
        phase_label,
        age_label,
        durable_job_id_ref: &durable_job_id_ref,
        canonical_event_id: &row.canonical_event_id,
        canonical_object_target_ref: &row.reopen_target.reopen_target_ref,
        badge_class,
        badge_source_class,
        dedupe_key_scheme: row.dedupe_key_scheme,
        quiet_hours_decision,
        reopen_class,
        exact_target_identity_ref: exact_ref,
        placeholder_reason_label: row.reopen_target.placeholder_announcement_label.as_deref(),
        denial_reason_label: row
            .reopen_target
            .revalidation_required_reason_label
            .as_deref(),
        repeated_failure_root_cause_count: row.occurrence_count.max(1),
        repeated_failures_coalesced: row.is_dedupe_repeat
            || matches!(
                row.dedupe_key_scheme,
                DedupeKeyScheme::CrossClientCanonicalEventId
            ),
        fixture_refs: vec![fixture_ref],
        narrative,
    })
}

fn make_badge_audit(case: &DurableAttentionConformanceCase) -> BadgeClassAuditRow {
    let active_count_delta = if matches!(case.badge_source_class, BadgeSourceClass::NotABadgeSource)
    {
        0
    } else if matches!(case.badge_class, Some(BadgeClass::HeldOrSuppressedCount)) {
        0
    } else {
        1
    };
    let held_or_suppressed_count_delta =
        if matches!(case.badge_class, Some(BadgeClass::HeldOrSuppressedCount)) {
            1
        } else {
            0
        };
    BadgeClassAuditRow {
        record_kind: DURABLE_ATTENTION_BADGE_AUDIT_RECORD_KIND.to_owned(),
        schema_version: DURABLE_ATTENTION_BETA_SCHEMA_VERSION,
        shared_contract_ref: DURABLE_ATTENTION_BETA_SHARED_CONTRACT_REF.to_owned(),
        audit_id: case.badge_audit_id_ref.clone(),
        case_id: case.case_id.clone(),
        badge_class: case.badge_class,
        badge_source_class: case.badge_source_class,
        dedupe_key_scheme: case.dedupe_key_scheme,
        active_count_delta,
        held_or_suppressed_count_delta,
        mixed_class_count_denied: true,
        count_source_ref: case.canonical_object_target_ref.clone(),
        maps_to_authoritative_object: true,
        support_export_ref: case.support_export_lineage_id_ref.clone(),
    }
}

fn make_quiet_hours_audit(case: &DurableAttentionConformanceCase) -> QuietHoursSuppressionAuditRow {
    let (active_modes, suppression_reasons, intended_surfaces, release_rule_label) =
        match case.quiet_hours_decision {
            QuietHoursDecisionClass::HeldQuietHours => (
                vec![QuietHoursMode::ModeQuietHoursUser],
                vec![SuppressionReason::QuietHoursUserPolicy],
                vec![
                    FanoutSurfaceClass::Toast,
                    FanoutSurfaceClass::OsNotification,
                    FanoutSurfaceClass::LockScreenSummary,
                    FanoutSurfaceClass::CompanionPush,
                ],
                "Release as grouped digest when quiet hours end.",
            ),
            QuietHoursDecisionClass::AdminSuppressed => (
                vec![QuietHoursMode::ModeAdminSuppression],
                vec![SuppressionReason::AdminPolicySuppression],
                vec![
                    FanoutSurfaceClass::OsNotification,
                    FanoutSurfaceClass::LockScreenSummary,
                    FanoutSurfaceClass::CompanionPush,
                ],
                "No external release; audit trail and in-product durable row remain.",
            ),
            QuietHoursDecisionClass::CriticalBypass => (
                vec![QuietHoursMode::ModeDoNotDisturbUser],
                Vec::new(),
                vec![
                    FanoutSurfaceClass::Toast,
                    FanoutSurfaceClass::OsNotification,
                ],
                "Bypass hold because active trust or policy scope cannot wait.",
            ),
            QuietHoursDecisionClass::CrossClientDeduped => (
                vec![QuietHoursMode::ModeNone],
                vec![SuppressionReason::DedupeSameCanonicalEvent],
                vec![
                    FanoutSurfaceClass::OsNotification,
                    FanoutSurfaceClass::CompanionPush,
                ],
                "Collapse companion and desktop fanout under the canonical event id.",
            ),
            QuietHoursDecisionClass::NotSuppressed => (
                vec![QuietHoursMode::ModeNone],
                Vec::new(),
                vec![FanoutSurfaceClass::DurableJobRow],
                "No hold applied.",
            ),
        };
    QuietHoursSuppressionAuditRow {
        record_kind: DURABLE_ATTENTION_QUIET_HOURS_AUDIT_RECORD_KIND.to_owned(),
        schema_version: DURABLE_ATTENTION_BETA_SCHEMA_VERSION,
        shared_contract_ref: DURABLE_ATTENTION_BETA_SHARED_CONTRACT_REF.to_owned(),
        audit_id: case.quiet_hours_audit_id_ref.clone(),
        case_id: case.case_id.clone(),
        decision_class: case.quiet_hours_decision,
        active_modes,
        suppression_reasons,
        intended_surfaces,
        preserved_durable_surfaces: vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::ActivityCenterDigestCard,
            FanoutSurfaceClass::StatusItem,
        ],
        durable_history_preserved: true,
        suppression_audit_trail_present: true,
        audit_trail_ref: format!("audit:durable-attention:{}", case.case_id),
        release_rule_label: release_rule_label.to_owned(),
        critical_safety_bypassed_hold: matches!(
            case.quiet_hours_decision,
            QuietHoursDecisionClass::CriticalBypass
        ),
        admin_policy_narrowed_without_widening: true,
        support_export_ref: case.support_export_lineage_id_ref.clone(),
    }
}

fn make_reopen_proof(case: &DurableAttentionConformanceCase) -> ExactReopenProof {
    let mut activation_surfaces = vec![
        FanoutSurfaceClass::DurableJobRow,
        FanoutSurfaceClass::StatusItem,
        FanoutSurfaceClass::ActivityCenterDigestCard,
    ];
    if !matches!(
        case.quiet_hours_decision,
        QuietHoursDecisionClass::AdminSuppressed
    ) {
        activation_surfaces.push(FanoutSurfaceClass::OsNotification);
    }
    if matches!(
        case.quiet_hours_decision,
        QuietHoursDecisionClass::CrossClientDeduped | QuietHoursDecisionClass::HeldQuietHours
    ) {
        activation_surfaces.push(FanoutSurfaceClass::CompanionPush);
    }
    ExactReopenProof {
        record_kind: DURABLE_ATTENTION_EXACT_REOPEN_PROOF_RECORD_KIND.to_owned(),
        schema_version: DURABLE_ATTENTION_BETA_SCHEMA_VERSION,
        shared_contract_ref: DURABLE_ATTENTION_BETA_SHARED_CONTRACT_REF.to_owned(),
        proof_id: case.reopen_proof_id_ref.clone(),
        case_id: case.case_id.clone(),
        reopen_class: case.reopen_class,
        activation_surfaces,
        canonical_event_id: case.canonical_event_id.clone(),
        canonical_object_target_ref: case.canonical_object_target_ref.clone(),
        exact_target_identity_ref: case.exact_target_identity_ref.clone(),
        placeholder_reason_label: case.placeholder_reason_label.clone(),
        denial_reason_label: case.denial_reason_label.clone(),
        open_details_command_id: "cmd:activity.open_job_details".to_owned(),
        generic_home_fallback_denied: true,
        preserves_preview_approval_trust_logic: true,
    }
}

fn make_support_lineage(case: &DurableAttentionConformanceCase) -> SupportExportLineageRow {
    SupportExportLineageRow {
        record_kind: DURABLE_ATTENTION_SUPPORT_LINEAGE_RECORD_KIND.to_owned(),
        schema_version: DURABLE_ATTENTION_BETA_SCHEMA_VERSION,
        shared_contract_ref: DURABLE_ATTENTION_BETA_SHARED_CONTRACT_REF.to_owned(),
        lineage_id: case.support_export_lineage_id_ref.clone(),
        case_id: case.case_id.clone(),
        canonical_event_id: case.canonical_event_id.clone(),
        durable_job_id_ref: case.durable_job_id_ref.clone(),
        canonical_object_target_ref: case.canonical_object_target_ref.clone(),
        source_subsystem: case.source_subsystem,
        target_scope_label: case.scope_label.clone(),
        state_class: case.state_class,
        badge_audit_id_ref: case.badge_audit_id_ref.clone(),
        quiet_hours_audit_id_ref: case.quiet_hours_audit_id_ref.clone(),
        reopen_proof_id_ref: case.reopen_proof_id_ref.clone(),
        fanout_receipt_refs: vec![
            format!("receipt:{}:durable_job_row", case.case_id),
            format!("receipt:{}:status_item", case.case_id),
        ],
        audit_event_refs: vec![
            format!("audit:{}:created", case.case_id),
            format!("audit:{}:phase_or_delivery", case.case_id),
        ],
        export_field_refs: vec![
            "export.durable_attention.identity".to_owned(),
            "export.durable_attention.state".to_owned(),
            "export.durable_attention.badge".to_owned(),
            "export.durable_attention.quiet_hours".to_owned(),
            "export.durable_attention.reopen".to_owned(),
        ],
        raw_private_material_excluded: true,
    }
}

fn activity_row<'a>(
    rows: &'a [ActivityCenterBetaRow],
    row_suffix: &str,
) -> &'a ActivityCenterBetaRow {
    rows.iter()
        .find(|row| row.row_id.ends_with(row_suffix))
        .unwrap_or_else(|| panic!("missing seeded activity-center row with suffix {row_suffix}"))
}

fn notification_row(
    rows: &[NotificationPrivacyBetaRow],
    class: NotificationPrivacyBetaRowClass,
) -> &NotificationPrivacyBetaRow {
    rows.iter()
        .find(|row| row.row_class == class)
        .unwrap_or_else(|| panic!("missing seeded notification privacy row {}", class.as_str()))
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn id_ref(prefix: &str, case_id: &str) -> String {
    format!("{prefix}:{}", case_id.replace(':', "-"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates() {
        let packet = seeded_durable_attention_beta_packet();
        validate_durable_attention_beta_packet(&packet).expect("seeded packet validates");
    }

    #[test]
    fn repeated_failure_case_uses_coalescing_scheme() {
        let packet = seeded_durable_attention_beta_packet();
        let case = packet
            .cases
            .iter()
            .find(|case| case.case_id == "case:test-failed-coalesced")
            .expect("test coalescing case exists");
        assert!(case.repeated_failures_coalesced);
        assert!(matches!(
            case.dedupe_key_scheme,
            DedupeKeyScheme::GroupedBurstId | DedupeKeyScheme::SubsystemPlusObjectPlusPhase
        ));
    }
}
