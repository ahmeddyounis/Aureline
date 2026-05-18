//! Scoped companion-surface and desktop-handoff beta contract.
//!
//! This module is the shell-owned projection for browser and mobile companion
//! rows. It deliberately treats companion clients as scoped surfaces that
//! reuse desktop command, policy, target-identity, evidence, step-up, and
//! notification vocabulary. The validator rejects rows that imply desktop
//! parity, hide stale or offline state, let a companion own protected
//! approvals, or bypass desktop-owned handoff for native-depth work.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Schema version exported with every companion-scope beta record.
pub const COMPANION_SCOPE_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by shell rows, fixtures, docs, and exports.
pub const COMPANION_SCOPE_BETA_SHARED_CONTRACT_REF: &str =
    "shell:companion_scope_and_handoff_beta:v1";

/// Stable record kind for [`CompanionScopeBetaPage`] payloads.
pub const COMPANION_SCOPE_BETA_PAGE_RECORD_KIND: &str = "shell_companion_scope_beta_page_record";

/// Stable record kind for [`CompanionScopeBetaRow`] payloads.
pub const COMPANION_SCOPE_BETA_ROW_RECORD_KIND: &str = "shell_companion_scope_beta_row_record";

/// Stable record kind for [`CompanionScopeBetaSupportRow`] payloads.
pub const COMPANION_SCOPE_BETA_SUPPORT_ROW_RECORD_KIND: &str =
    "shell_companion_scope_beta_support_row_record";

/// Stable record kind for [`CompanionScopeBetaSupportExport`] payloads.
pub const COMPANION_SCOPE_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_companion_scope_beta_support_export_record";

/// Claimed companion workflow row classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionWorkflowClass {
    /// Review workspace triage, comments, and handoff to native approval.
    ReviewTriage,
    /// Documentation and help browsing with source and freshness truth.
    DocsAndHelp,
    /// Bounded remote text edits that reuse desktop command policy.
    LightRemoteEdit,
    /// Collaboration or incident session join/follow.
    RemoteSessionJoin,
    /// CI or pipeline status inspection.
    CiStatus,
    /// Incident awareness and lightweight acknowledgment.
    IncidentAwareness,
}

impl CompanionWorkflowClass {
    /// Stable token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewTriage => "review_triage",
            Self::DocsAndHelp => "docs_and_help",
            Self::LightRemoteEdit => "light_remote_edit",
            Self::RemoteSessionJoin => "remote_session_join",
            Self::CiStatus => "ci_status",
            Self::IncidentAwareness => "incident_awareness",
        }
    }
}

/// Companion client surface family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionSurfaceClass {
    /// Browser web companion.
    BrowserWebCompanion,
    /// Browser extension companion.
    BrowserExtensionCompanion,
    /// Mobile native companion.
    MobileNativeCompanion,
    /// Mobile web progressive-web-app companion.
    MobileWebPwaCompanion,
}

/// User-facing client scope label for a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionClientScopeLabel {
    /// Read-only review or inspection.
    ReviewOnly,
    /// Comment or acknowledge through canonical event identity.
    CommentCapable,
    /// Bounded light edit with explicit save and trust limits.
    LightEdit,
    /// Follow or observe a remote session.
    FollowCapable,
    /// Status-only inspection.
    StatusOnly,
    /// Light incident tooling only.
    LightIncidentTooling,
    /// Desktop is required for the named action.
    DesktopRequired,
}

impl CompanionClientScopeLabel {
    /// Stable token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewOnly => "review_only",
            Self::CommentCapable => "comment_capable",
            Self::LightEdit => "light_edit",
            Self::FollowCapable => "follow_capable",
            Self::StatusOnly => "status_only",
            Self::LightIncidentTooling => "light_incident_tooling",
            Self::DesktopRequired => "desktop_required",
        }
    }
}

/// Freshness and snapshot posture shown on companion rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionFreshnessClass {
    /// Live authoritative data within the declared grace window.
    LiveAuthoritativeFreshWithinGrace,
    /// Warm data within the grace window, marked as warm.
    WarmSnapshotWithinGrace,
    /// Stale data beyond the grace window, marked as stale.
    StaleSnapshotBeyondGrace,
    /// Offline snapshot with no refresh path at render time.
    OfflineSnapshotNoRefreshPath,
    /// Imported evidence-only snapshot without live target authority.
    ImportedSnapshotNoLiveTarget,
}

impl CompanionFreshnessClass {
    /// Stable token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveAuthoritativeFreshWithinGrace => "live_authoritative_fresh_within_grace",
            Self::WarmSnapshotWithinGrace => "warm_snapshot_within_grace",
            Self::StaleSnapshotBeyondGrace => "stale_snapshot_beyond_grace",
            Self::OfflineSnapshotNoRefreshPath => "offline_snapshot_no_refresh_path",
            Self::ImportedSnapshotNoLiveTarget => "imported_snapshot_no_live_target",
        }
    }

    /// True when the row may render a live/current claim.
    pub const fn permits_live_claim(self) -> bool {
        matches!(self, Self::LiveAuthoritativeFreshWithinGrace)
    }
}

/// Companion authority posture for the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionAuthorityClass {
    /// Inspect-only; no companion-side state change.
    InspectOnlyNoMutation,
    /// Comment or acknowledgment only through reused canonical event ids.
    CommentOrAckViaCanonicalEvent,
    /// Bounded light edit admitted through desktop command policy.
    LightEditViaDesktopCommandPolicy,
    /// Scoped follow/control request admitted by a desktop-owned grant.
    ScopedFollowGrantRequest,
    /// Companion may request approval but cannot own or grant it.
    ApprovalRequestOnlyDesktopOwned,
    /// Companion blocks the action and requires desktop handoff.
    DesktopHandoffRequired,
}

impl CompanionAuthorityClass {
    /// Stable token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnlyNoMutation => "inspect_only_no_mutation",
            Self::CommentOrAckViaCanonicalEvent => "comment_or_ack_via_canonical_event",
            Self::LightEditViaDesktopCommandPolicy => "light_edit_via_desktop_command_policy",
            Self::ScopedFollowGrantRequest => "scoped_follow_grant_request",
            Self::ApprovalRequestOnlyDesktopOwned => "approval_request_only_desktop_owned",
            Self::DesktopHandoffRequired => "desktop_handoff_required",
        }
    }
}

/// Desktop handoff reasons that remain outside companion authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesktopHandoffRequirementClass {
    /// Broad mutation must run in the native desktop review surface.
    BroadMutation,
    /// Protected approval must stay desktop-owned.
    ProtectedApproval,
    /// Sensitive admin flow must stay desktop-owned.
    SensitiveAdminFlow,
    /// High-risk publish or deploy path must stay desktop-owned.
    HighRiskPublish,
    /// Unmanaged secret entry is not allowed on the companion.
    UnmanagedSecretEntry,
    /// Deep local project editing is not a companion capability.
    DeepLocalProjectEditing,
    /// Terminal, debugger, or native runtime depth is desktop-only.
    NativeDepthWorkflow,
}

impl DesktopHandoffRequirementClass {
    /// Stable token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BroadMutation => "broad_mutation",
            Self::ProtectedApproval => "protected_approval",
            Self::SensitiveAdminFlow => "sensitive_admin_flow",
            Self::HighRiskPublish => "high_risk_publish",
            Self::UnmanagedSecretEntry => "unmanaged_secret_entry",
            Self::DeepLocalProjectEditing => "deep_local_project_editing",
            Self::NativeDepthWorkflow => "native_depth_workflow",
        }
    }
}

/// Desktop step-up or approval semantics reused by a companion row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepUpReuseClass {
    /// Inspect-only rows do not need step-up.
    NoStepUpInspectOnly,
    /// The row reuses a desktop approval-ticket envelope.
    DesktopApprovalTicketReused,
    /// The row reuses a desktop step-up challenge.
    DesktopStepUpChallengeReused,
    /// The row requires a managed-admin desktop approval.
    ManagedAdminApprovalRequiredOnDesktop,
    /// Step-up is intentionally blocked from the companion.
    StepUpBlockedCompanionMustHandoff,
}

impl StepUpReuseClass {
    /// Stable token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoStepUpInspectOnly => "no_step_up_inspect_only",
            Self::DesktopApprovalTicketReused => "desktop_approval_ticket_reused",
            Self::DesktopStepUpChallengeReused => "desktop_step_up_challenge_reused",
            Self::ManagedAdminApprovalRequiredOnDesktop => {
                "managed_admin_approval_required_on_desktop"
            }
            Self::StepUpBlockedCompanionMustHandoff => "step_up_blocked_companion_must_handoff",
        }
    }
}

/// Surface that owns an approval decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalOwnerSurfaceClass {
    /// No approval applies.
    NoApprovalApplies,
    /// Native desktop review or approval surface owns the decision.
    NativeDesktopReviewSurface,
    /// Provider surface owns a separately governed provider handoff.
    ProviderHandoffSurface,
    /// Companion can request but not grant the approval.
    CompanionRequestOnly,
}

impl ApprovalOwnerSurfaceClass {
    /// Stable token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoApprovalApplies => "no_approval_applies",
            Self::NativeDesktopReviewSurface => "native_desktop_review_surface",
            Self::ProviderHandoffSurface => "provider_handoff_surface",
            Self::CompanionRequestOnly => "companion_request_only",
        }
    }
}

/// Device-policy posture for notification fanout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DevicePolicyClass {
    /// Companion fanout inherits the desktop notification policy.
    InheritsDesktopNotificationPolicy,
    /// Managed device policy is applied before companion fanout.
    InheritsManagedDevicePolicy,
    /// Push is denied; local inspection or support export remains.
    PushDeniedLocalOnly,
}

impl DevicePolicyClass {
    /// Stable token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InheritsDesktopNotificationPolicy => "inherits_desktop_notification_policy",
            Self::InheritsManagedDevicePolicy => "inherits_managed_device_policy",
            Self::PushDeniedLocalOnly => "push_denied_local_only",
        }
    }
}

/// Quiet-hours posture for companion fanout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionQuietHoursClass {
    /// Companion fanout inherits in-app, OS, and companion quiet-hours state.
    InheritsUnifiedQuietHoursPolicy,
    /// Companion fanout is held until quiet hours release.
    HeldByQuietHours,
    /// Critical safety can bypass a hold while preserving an audit receipt.
    CriticalSafetyBypassAudited,
}

impl CompanionQuietHoursClass {
    /// Stable token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InheritsUnifiedQuietHoursPolicy => "inherits_unified_quiet_hours_policy",
            Self::HeldByQuietHours => "held_by_quiet_hours",
            Self::CriticalSafetyBypassAudited => "critical_safety_bypass_audited",
        }
    }
}

/// Validation failure class for companion-scope rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionScopeDefectKind {
    /// Record kind, schema version, or shared contract ref drifted.
    RecordIdentityDrift,
    /// A row token no longer matches its typed enum.
    RowTokenDrift,
    /// The seed no longer covers every required workflow class.
    MissingRequiredWorkflow,
    /// A row implies browser or mobile desktop parity.
    DesktopParityClaimed,
    /// A row allows unmanaged secret entry on the companion.
    UnmanagedSecretEntryAllowed,
    /// A row allows deep local project editing on the companion.
    DeepLocalProjectEditingAllowed,
    /// A row hides target identity.
    TargetIdentityHidden,
    /// A stale or offline row hides its stale/offline label.
    StaleOfflineLabelMissing,
    /// A row hides both read-only and desktop-handoff state.
    ReadOnlyOrHandoffLabelMissing,
    /// A required desktop handoff is absent or lacks an exact reopen target.
    MandatoryDesktopHandoffMissing,
    /// A companion row owns or grants a protected approval.
    CompanionOwnsProtectedApproval,
    /// A row needs authority but fails to reuse desktop step-up semantics.
    StepUpSemanticsNotReused,
    /// Companion fanout does not inherit device policy.
    DevicePolicyNotInherited,
    /// Companion fanout does not inherit quiet-hours policy.
    QuietHoursNotInherited,
    /// Companion fanout can bypass approval or notification guardrails.
    NotificationBypassAllowed,
    /// Support-export row drifted from its source row.
    SupportExportDrift,
}

impl CompanionScopeDefectKind {
    /// Stable token recorded in diagnostics and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordIdentityDrift => "record_identity_drift",
            Self::RowTokenDrift => "row_token_drift",
            Self::MissingRequiredWorkflow => "missing_required_workflow",
            Self::DesktopParityClaimed => "desktop_parity_claimed",
            Self::UnmanagedSecretEntryAllowed => "unmanaged_secret_entry_allowed",
            Self::DeepLocalProjectEditingAllowed => "deep_local_project_editing_allowed",
            Self::TargetIdentityHidden => "target_identity_hidden",
            Self::StaleOfflineLabelMissing => "stale_offline_label_missing",
            Self::ReadOnlyOrHandoffLabelMissing => "read_only_or_handoff_label_missing",
            Self::MandatoryDesktopHandoffMissing => "mandatory_desktop_handoff_missing",
            Self::CompanionOwnsProtectedApproval => "companion_owns_protected_approval",
            Self::StepUpSemanticsNotReused => "step_up_semantics_not_reused",
            Self::DevicePolicyNotInherited => "device_policy_not_inherited",
            Self::QuietHoursNotInherited => "quiet_hours_not_inherited",
            Self::NotificationBypassAllowed => "notification_bypass_allowed",
            Self::SupportExportDrift => "support_export_drift",
        }
    }
}

/// Target identity shown on a companion row and carried into handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionTargetIdentityBlock {
    /// Canonical object kind, such as review workspace, docs page, or run.
    pub target_kind: String,
    /// Opaque canonical object ref safe for support export.
    pub target_object_ref: String,
    /// Opaque execution context or session ref.
    pub execution_context_ref: String,
    /// Human-readable target identity label safe for review surfaces.
    pub target_identity_label: String,
    /// True when the target identity label is visible in UI.
    pub target_identity_label_visible: bool,
    /// Canonical event id reused by notifications and support export.
    pub canonical_event_id_ref: String,
}

/// Freshness and snapshot details for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionFreshnessBlock {
    /// Typed freshness class.
    pub freshness_class: CompanionFreshnessClass,
    /// Stable token matching [`Self::freshness_class`].
    pub freshness_token: String,
    /// Snapshot timestamp shown to users and support readers.
    pub snapshot_as_of: String,
    /// True when the UI explicitly labels stale or offline state.
    pub stale_or_offline_label_visible: bool,
    /// True when a live/current label is permitted for this row.
    pub live_claim_permitted: bool,
}

/// Visible labels that travel through shell, fanout, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionVisibleLabelBlock {
    /// True when the client-scope chip is visible.
    pub client_scope_label_visible: bool,
    /// True when target identity is visible.
    pub target_identity_label_visible: bool,
    /// True when freshness, stale, warm, or offline posture is visible.
    pub freshness_label_visible: bool,
    /// True when a read-only label is visible.
    pub read_only_label_visible: bool,
    /// True when a desktop-handoff label is visible.
    pub desktop_handoff_label_visible: bool,
    /// Stable label tokens safe for screenshots, docs, and exports.
    pub label_tokens: Vec<String>,
}

/// Step-up and approval envelope reused by a companion row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionStepUpBlock {
    /// Typed step-up reuse class.
    pub step_up_reuse_class: StepUpReuseClass,
    /// Stable token matching [`Self::step_up_reuse_class`].
    pub step_up_reuse_token: String,
    /// True when the requested action needs step-up.
    pub step_up_required: bool,
    /// Opaque desktop approval-ticket ref when one exists.
    pub desktop_approval_ticket_ref: Option<String>,
    /// Opaque native desktop review-surface ref when one exists.
    pub desktop_review_surface_ref: Option<String>,
    /// Surface that owns the approval decision.
    pub approval_owner_surface: ApprovalOwnerSurfaceClass,
    /// Stable token matching [`Self::approval_owner_surface`].
    pub approval_owner_surface_token: String,
}

/// Authority posture and forbidden widening flags for a companion row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionAuthorityBlock {
    /// Typed authority class.
    pub authority_class: CompanionAuthorityClass,
    /// Stable token matching [`Self::authority_class`].
    pub authority_token: String,
    /// True when the companion can request an approval envelope.
    pub companion_can_request_approval: bool,
    /// True when the companion can grant or own an approval.
    pub companion_can_grant_approval: bool,
    /// True when broad mutation is allowed directly on companion.
    pub broad_mutation_allowed_on_companion: bool,
    /// True when protected approval is allowed directly on companion.
    pub protected_approval_allowed_on_companion: bool,
    /// True when sensitive admin flows are allowed directly on companion.
    pub sensitive_admin_allowed_on_companion: bool,
    /// True when high-risk publish flows are allowed directly on companion.
    pub high_risk_publish_allowed_on_companion: bool,
    /// Reused desktop step-up and approval posture.
    pub step_up: CompanionStepUpBlock,
}

/// Desktop handoff posture for a companion row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionDesktopHandoffBlock {
    /// True when the action must complete on desktop.
    pub mandatory_desktop_handoff: bool,
    /// Reason classes that require desktop ownership.
    pub reason_classes: Vec<DesktopHandoffRequirementClass>,
    /// Opaque browser or desktop handoff packet ref.
    pub desktop_handoff_ref: Option<String>,
    /// Exact reopen target safe for shell routing and support export.
    pub exact_reopen_target_ref: String,
    /// True when handoff preserves the target object identity exactly.
    pub handoff_preserves_target_identity: bool,
    /// Reviewable handoff label.
    pub handoff_label: String,
}

/// Notification and device-policy fanout posture for a companion row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionNotificationFanoutBlock {
    /// Notification surface ref safe for support export.
    pub notification_surface_ref: String,
    /// Typed device-policy class.
    pub device_policy_class: DevicePolicyClass,
    /// Stable token matching [`Self::device_policy_class`].
    pub device_policy_token: String,
    /// True when fanout inherits device policy.
    pub device_policy_inherited: bool,
    /// Typed quiet-hours class.
    pub quiet_hours_class: CompanionQuietHoursClass,
    /// Stable token matching [`Self::quiet_hours_class`].
    pub quiet_hours_token: String,
    /// True when fanout inherits quiet-hours policy.
    pub quiet_hours_inherited: bool,
    /// Dedupe key shared with desktop notification lineage.
    pub fanout_dedupe_key_ref: String,
    /// True when companion push is allowed after policy checks.
    pub companion_push_allowed: bool,
    /// True when fanout is coalesced or held instead of spamming.
    pub hidden_attention_spam_prevented: bool,
    /// True when companion notifications cannot complete shortcut mutations.
    pub shortcut_bypass_forbidden: bool,
    /// Exact reopen target attached to the fanout row.
    pub exact_reopen_target_ref: String,
}

/// Companion-origin provenance kept for support and release reconstruction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionActionProvenanceBlock {
    /// Surface that showed the companion object.
    pub origin_surface_ref: String,
    /// Shell, review, docs, or incident surface that displayed the object.
    pub displayed_object_surface: String,
    /// Surface that owns approval, if any.
    pub approval_owner_surface: ApprovalOwnerSurfaceClass,
    /// Desktop destination used for handoff.
    pub desktop_handoff_destination_ref: Option<String>,
    /// Support export row ref.
    pub support_export_ref: String,
    /// Reused canonical event id.
    pub canonical_event_id_ref: String,
    /// Audit event id safe for chronology reconstruction.
    pub audit_event_id_ref: String,
}

/// Explicit non-promises every companion row carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionNonPromiseBlock {
    /// True would imply desktop parity; validator rejects it.
    pub desktop_parity_claimed: bool,
    /// True would allow raw or unmanaged secret entry; validator rejects it.
    pub unmanaged_secret_entry_allowed: bool,
    /// True would allow deep local project editing; validator rejects it.
    pub deep_local_project_editing_allowed: bool,
    /// True would imply terminal or debugger parity; validator rejects it.
    pub terminal_or_debugger_parity_claimed: bool,
    /// True would imply full admin-console replacement; validator rejects it.
    pub admin_console_replacement_claimed: bool,
    /// Reviewable non-promise tokens safe for docs and support.
    pub non_promise_tokens: Vec<String>,
}

/// One claimed companion-scope beta row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionScopeBetaRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed across artifacts.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Stable case id.
    pub case_id: String,
    /// Claimed workflow class.
    pub workflow: CompanionWorkflowClass,
    /// Stable token matching [`Self::workflow`].
    pub workflow_token: String,
    /// Companion surface class.
    pub surface_class: CompanionSurfaceClass,
    /// User-facing client-scope label.
    pub client_scope_label: CompanionClientScopeLabel,
    /// Stable token matching [`Self::client_scope_label`].
    pub client_scope_token: String,
    /// Target identity block.
    pub target_identity: CompanionTargetIdentityBlock,
    /// Freshness posture block.
    pub freshness: CompanionFreshnessBlock,
    /// Visible labels block.
    pub labels: CompanionVisibleLabelBlock,
    /// Authority posture block.
    pub authority: CompanionAuthorityBlock,
    /// Desktop handoff posture block.
    pub desktop_handoff: CompanionDesktopHandoffBlock,
    /// Notification fanout and quiet-hours posture block.
    pub notification_fanout: CompanionNotificationFanoutBlock,
    /// Provenance block for support and release reconstruction.
    pub provenance: CompanionActionProvenanceBlock,
    /// Explicit non-promises.
    pub explicit_non_promises: CompanionNonPromiseBlock,
    /// True when raw paths, URLs, secrets, payloads, and actor identities are excluded.
    pub raw_private_material_excluded: bool,
    /// Reviewable summary safe for docs and support.
    pub reviewer_note: String,
}

/// Support-export projection for one companion-scope row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionScopeBetaSupportRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed across artifacts.
    pub shared_contract_ref: String,
    /// Source row id.
    pub row_id: String,
    /// Source case id.
    pub case_id: String,
    /// Workflow token.
    pub workflow_token: String,
    /// Client-scope token.
    pub client_scope_token: String,
    /// Target object ref.
    pub target_object_ref: String,
    /// Freshness token.
    pub freshness_token: String,
    /// True when read-only state is visible.
    pub read_only_label_visible: bool,
    /// True when desktop handoff state is visible.
    pub desktop_handoff_label_visible: bool,
    /// True when desktop handoff is mandatory.
    pub mandatory_desktop_handoff: bool,
    /// Handoff reason tokens.
    pub handoff_reason_tokens: Vec<String>,
    /// Approval-owner token.
    pub approval_owner_surface_token: String,
    /// Surface that showed the object.
    pub origin_surface_ref: String,
    /// Surface that displayed the object.
    pub displayed_object_surface: String,
    /// Destination for desktop handoff.
    pub desktop_handoff_destination_ref: Option<String>,
    /// True when device policy was inherited.
    pub device_policy_inherited: bool,
    /// True when quiet-hours policy was inherited.
    pub quiet_hours_inherited: bool,
    /// Reused canonical event id.
    pub canonical_event_id_ref: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl CompanionScopeBetaSupportRow {
    /// Build a support row from a companion-scope row.
    pub fn from_row(row: &CompanionScopeBetaRow) -> Self {
        Self {
            record_kind: COMPANION_SCOPE_BETA_SUPPORT_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPANION_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: COMPANION_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: row.row_id.clone(),
            case_id: row.case_id.clone(),
            workflow_token: row.workflow.as_str().to_owned(),
            client_scope_token: row.client_scope_label.as_str().to_owned(),
            target_object_ref: row.target_identity.target_object_ref.clone(),
            freshness_token: row.freshness.freshness_class.as_str().to_owned(),
            read_only_label_visible: row.labels.read_only_label_visible,
            desktop_handoff_label_visible: row.labels.desktop_handoff_label_visible,
            mandatory_desktop_handoff: row.desktop_handoff.mandatory_desktop_handoff,
            handoff_reason_tokens: row
                .desktop_handoff
                .reason_classes
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect(),
            approval_owner_surface_token: row
                .authority
                .step_up
                .approval_owner_surface
                .as_str()
                .to_owned(),
            origin_surface_ref: row.provenance.origin_surface_ref.clone(),
            displayed_object_surface: row.provenance.displayed_object_surface.clone(),
            desktop_handoff_destination_ref: row.provenance.desktop_handoff_destination_ref.clone(),
            device_policy_inherited: row.notification_fanout.device_policy_inherited,
            quiet_hours_inherited: row.notification_fanout.quiet_hours_inherited,
            canonical_event_id_ref: row.provenance.canonical_event_id_ref.clone(),
            raw_private_material_excluded: row.raw_private_material_excluded,
        }
    }
}

/// Aggregate summary for the companion-scope beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionScopeBetaSummary {
    /// Number of companion-scope rows.
    pub row_count: usize,
    /// Workflow tokens present on the page.
    pub workflows_present: Vec<String>,
    /// Number of stale or offline rows.
    pub stale_or_offline_row_count: usize,
    /// Number of rows with mandatory desktop handoff.
    pub mandatory_desktop_handoff_row_count: usize,
    /// Number of rows whose approval owner is native desktop.
    pub desktop_owned_approval_row_count: usize,
    /// Number of rows that inherit quiet-hours policy.
    pub quiet_hours_integrated_row_count: usize,
    /// Number of validation defects.
    pub defect_count: usize,
    /// True when raw private material is excluded from every row.
    pub raw_private_material_excluded: bool,
}

/// Page record for the scoped companion and handoff beta contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionScopeBetaPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed across artifacts.
    pub shared_contract_ref: String,
    /// Companion-scope rows.
    pub rows: Vec<CompanionScopeBetaRow>,
    /// Support-export projection rows.
    pub support_rows: Vec<CompanionScopeBetaSupportRow>,
    /// Typed validation defects.
    pub defects: Vec<CompanionScopeDefect>,
    /// Aggregate page summary.
    pub summary: CompanionScopeBetaSummary,
}

impl CompanionScopeBetaPage {
    /// True when all required workflow classes are present.
    pub fn covers_required_workflows(&self) -> bool {
        let present: BTreeSet<_> = self.rows.iter().map(|row| row.workflow).collect();
        required_workflows()
            .into_iter()
            .all(|workflow| present.contains(&workflow))
    }
}

/// Support export wrapper for companion-origin action reconstruction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionScopeBetaSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed across artifacts.
    pub shared_contract_ref: String,
    /// Export id safe for support bundles.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Embedded page.
    pub page: CompanionScopeBetaPage,
    /// Per-row support projections.
    pub rows: Vec<CompanionScopeBetaSupportRow>,
    /// Stable case ids in page order.
    pub case_ids: Vec<String>,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Defect kinds present in the embedded page.
    pub defect_kinds_present: Vec<CompanionScopeDefectKind>,
    /// Defect counts by stable token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl CompanionScopeBetaSupportExport {
    /// Build a support export from a page.
    pub fn from_page(export_id: &str, exported_at: &str, page: CompanionScopeBetaPage) -> Self {
        let rows = page.support_rows.clone();
        let case_ids = page.rows.iter().map(|row| row.case_id.clone()).collect();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            *counts
                .entry(defect.defect_kind.as_str().to_owned())
                .or_insert(0) += 1;
        }
        let defect_kinds_present = page
            .defects
            .iter()
            .map(|defect| defect.defect_kind)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        Self {
            record_kind: COMPANION_SCOPE_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: COMPANION_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: COMPANION_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            page,
            rows,
            case_ids,
            raw_private_material_excluded: true,
            defect_kinds_present,
            defect_counts_by_kind: counts,
        }
    }
}

/// Typed validator defect for companion-scope rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionScopeDefect {
    /// Defect class.
    pub defect_kind: CompanionScopeDefectKind,
    /// Stable token matching [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Source row id, or `page` for page-level defects.
    pub row_id: String,
    /// Field or invariant that failed.
    pub field: String,
    /// Reviewable diagnostic note.
    pub note: String,
}

impl CompanionScopeDefect {
    fn new(
        defect_kind: CompanionScopeDefectKind,
        row_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            row_id: row_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Return the required companion workflow classes.
pub fn required_workflows() -> Vec<CompanionWorkflowClass> {
    vec![
        CompanionWorkflowClass::ReviewTriage,
        CompanionWorkflowClass::DocsAndHelp,
        CompanionWorkflowClass::LightRemoteEdit,
        CompanionWorkflowClass::RemoteSessionJoin,
        CompanionWorkflowClass::CiStatus,
        CompanionWorkflowClass::IncidentAwareness,
    ]
}

/// Build the seeded companion-scope beta page.
pub fn seeded_companion_scope_beta_page() -> CompanionScopeBetaPage {
    let rows = seeded_rows();
    let support_rows = rows
        .iter()
        .map(CompanionScopeBetaSupportRow::from_row)
        .collect::<Vec<_>>();
    let defects = audit_companion_scope_beta_rows(&rows, &support_rows);
    let summary = build_summary(&rows, &defects);
    CompanionScopeBetaPage {
        record_kind: COMPANION_SCOPE_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: COMPANION_SCOPE_BETA_SCHEMA_VERSION,
        shared_contract_ref: COMPANION_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
        rows,
        support_rows,
        defects,
        summary,
    }
}

/// Validate a companion-scope beta page.
pub fn validate_companion_scope_beta_page(
    page: &CompanionScopeBetaPage,
) -> Result<(), Vec<CompanionScopeDefect>> {
    let mut defects = Vec::new();
    if page.record_kind != COMPANION_SCOPE_BETA_PAGE_RECORD_KIND
        || page.schema_version != COMPANION_SCOPE_BETA_SCHEMA_VERSION
        || page.shared_contract_ref != COMPANION_SCOPE_BETA_SHARED_CONTRACT_REF
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::RecordIdentityDrift,
            "page",
            "record_identity",
            "page record kind, schema version, and shared contract ref must match constants",
        ));
    }
    defects.extend(audit_companion_scope_beta_rows(
        &page.rows,
        &page.support_rows,
    ));
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Validate a companion-scope beta support export.
pub fn validate_companion_scope_beta_support_export(
    export: &CompanionScopeBetaSupportExport,
) -> Result<(), Vec<CompanionScopeDefect>> {
    let mut defects = Vec::new();
    if export.record_kind != COMPANION_SCOPE_BETA_SUPPORT_EXPORT_RECORD_KIND
        || export.schema_version != COMPANION_SCOPE_BETA_SCHEMA_VERSION
        || export.shared_contract_ref != COMPANION_SCOPE_BETA_SHARED_CONTRACT_REF
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::RecordIdentityDrift,
            "support_export",
            "record_identity",
            "support export record kind, schema version, and shared contract ref must match constants",
        ));
    }
    if let Err(page_defects) = validate_companion_scope_beta_page(&export.page) {
        defects.extend(page_defects);
    }
    if export.rows != export.page.support_rows {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::SupportExportDrift,
            "support_export",
            "rows",
            "support export rows must equal the embedded page support rows",
        ));
    }
    if export.case_ids
        != export
            .page
            .rows
            .iter()
            .map(|row| row.case_id.clone())
            .collect::<Vec<_>>()
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::SupportExportDrift,
            "support_export",
            "case_ids",
            "support export case ids must match page row order",
        ));
    }
    let mut expected_counts: BTreeMap<String, usize> = BTreeMap::new();
    for defect in &export.page.defects {
        *expected_counts
            .entry(defect.defect_kind.as_str().to_owned())
            .or_insert(0) += 1;
    }
    let expected_kinds = export
        .page
        .defects
        .iter()
        .map(|defect| defect.defect_kind)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if export.defect_counts_by_kind != expected_counts
        || export.defect_kinds_present != expected_kinds
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::SupportExportDrift,
            "support_export",
            "defect_summary",
            "support export defect summary must match the embedded page defects",
        ));
    }
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Audit row and support-row invariants for companion scope and handoff.
pub fn audit_companion_scope_beta_rows(
    rows: &[CompanionScopeBetaRow],
    support_rows: &[CompanionScopeBetaSupportRow],
) -> Vec<CompanionScopeDefect> {
    let mut defects = Vec::new();
    let mut workflows_present = BTreeSet::new();

    for row in rows {
        workflows_present.insert(row.workflow);
        audit_row_identity(row, &mut defects);
        audit_row_tokens(row, &mut defects);
        audit_row_scope_truth(row, &mut defects);
        audit_row_authority(row, &mut defects);
        audit_row_fanout(row, &mut defects);
        audit_row_support_projection(row, support_rows, &mut defects);
    }

    for workflow in required_workflows() {
        if !workflows_present.contains(&workflow) {
            defects.push(CompanionScopeDefect::new(
                CompanionScopeDefectKind::MissingRequiredWorkflow,
                "page",
                "rows.workflow",
                format!("missing required workflow {}", workflow.as_str()),
            ));
        }
    }

    defects
}

fn audit_row_identity(row: &CompanionScopeBetaRow, defects: &mut Vec<CompanionScopeDefect>) {
    if row.record_kind != COMPANION_SCOPE_BETA_ROW_RECORD_KIND
        || row.schema_version != COMPANION_SCOPE_BETA_SCHEMA_VERSION
        || row.shared_contract_ref != COMPANION_SCOPE_BETA_SHARED_CONTRACT_REF
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::RecordIdentityDrift,
            &row.row_id,
            "record_identity",
            "row record kind, schema version, and shared contract ref must match constants",
        ));
    }
    if !row.raw_private_material_excluded {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::SupportExportDrift,
            &row.row_id,
            "raw_private_material_excluded",
            "companion rows must exclude raw paths, URLs, payloads, secrets, and actor identities",
        ));
    }
}

fn audit_row_tokens(row: &CompanionScopeBetaRow, defects: &mut Vec<CompanionScopeDefect>) {
    let expected_pairs = [
        (
            "workflow_token",
            row.workflow.as_str(),
            row.workflow_token.as_str(),
        ),
        (
            "client_scope_token",
            row.client_scope_label.as_str(),
            row.client_scope_token.as_str(),
        ),
        (
            "freshness_token",
            row.freshness.freshness_class.as_str(),
            row.freshness.freshness_token.as_str(),
        ),
        (
            "authority_token",
            row.authority.authority_class.as_str(),
            row.authority.authority_token.as_str(),
        ),
        (
            "step_up_reuse_token",
            row.authority.step_up.step_up_reuse_class.as_str(),
            row.authority.step_up.step_up_reuse_token.as_str(),
        ),
        (
            "approval_owner_surface_token",
            row.authority.step_up.approval_owner_surface.as_str(),
            row.authority.step_up.approval_owner_surface_token.as_str(),
        ),
        (
            "device_policy_token",
            row.notification_fanout.device_policy_class.as_str(),
            row.notification_fanout.device_policy_token.as_str(),
        ),
        (
            "quiet_hours_token",
            row.notification_fanout.quiet_hours_class.as_str(),
            row.notification_fanout.quiet_hours_token.as_str(),
        ),
    ];

    for (field, expected, actual) in expected_pairs {
        if expected != actual {
            defects.push(CompanionScopeDefect::new(
                CompanionScopeDefectKind::RowTokenDrift,
                &row.row_id,
                field,
                format!("expected {expected}, got {actual}"),
            ));
        }
    }
}

fn audit_row_scope_truth(row: &CompanionScopeBetaRow, defects: &mut Vec<CompanionScopeDefect>) {
    let non_promises = &row.explicit_non_promises;
    if non_promises.desktop_parity_claimed
        || non_promises.terminal_or_debugger_parity_claimed
        || non_promises.admin_console_replacement_claimed
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::DesktopParityClaimed,
            &row.row_id,
            "explicit_non_promises",
            "companion rows must not imply desktop parity, terminal/debugger parity, or admin-console replacement",
        ));
    }
    if non_promises.unmanaged_secret_entry_allowed {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::UnmanagedSecretEntryAllowed,
            &row.row_id,
            "explicit_non_promises.unmanaged_secret_entry_allowed",
            "unmanaged secret entry is forbidden on companion surfaces",
        ));
    }
    if non_promises.deep_local_project_editing_allowed {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::DeepLocalProjectEditingAllowed,
            &row.row_id,
            "explicit_non_promises.deep_local_project_editing_allowed",
            "deep local project editing is not a companion capability",
        ));
    }
    if row.target_identity.target_object_ref.is_empty()
        || row.target_identity.execution_context_ref.is_empty()
        || !row.target_identity.target_identity_label_visible
        || !row.labels.target_identity_label_visible
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::TargetIdentityHidden,
            &row.row_id,
            "target_identity",
            "target identity and execution context must be visible on companion rows",
        ));
    }
    if !row.freshness.freshness_class.permits_live_claim()
        && (!row.freshness.stale_or_offline_label_visible
            || !row.labels.freshness_label_visible
            || row.freshness.live_claim_permitted)
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::StaleOfflineLabelMissing,
            &row.row_id,
            "freshness",
            "warm, stale, offline, or imported snapshots must render an explicit non-live label",
        ));
    }
    if !row.labels.client_scope_label_visible
        || (!row.labels.read_only_label_visible && !row.labels.desktop_handoff_label_visible)
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::ReadOnlyOrHandoffLabelMissing,
            &row.row_id,
            "labels",
            "client scope plus read-only or desktop-handoff state must be visible",
        ));
    }
}

fn audit_row_authority(row: &CompanionScopeBetaRow, defects: &mut Vec<CompanionScopeDefect>) {
    let authority = &row.authority;
    if authority.broad_mutation_allowed_on_companion
        || authority.protected_approval_allowed_on_companion
        || authority.sensitive_admin_allowed_on_companion
        || authority.high_risk_publish_allowed_on_companion
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::CompanionOwnsProtectedApproval,
            &row.row_id,
            "authority",
            "companion rows may not silently widen broad mutation, protected approval, sensitive admin, or high-risk publish authority",
        ));
    }

    let reasons = row
        .desktop_handoff
        .reason_classes
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let requires_native_desktop = reasons.iter().any(|reason| {
        matches!(
            reason,
            DesktopHandoffRequirementClass::BroadMutation
                | DesktopHandoffRequirementClass::ProtectedApproval
                | DesktopHandoffRequirementClass::SensitiveAdminFlow
                | DesktopHandoffRequirementClass::HighRiskPublish
                | DesktopHandoffRequirementClass::UnmanagedSecretEntry
                | DesktopHandoffRequirementClass::DeepLocalProjectEditing
                | DesktopHandoffRequirementClass::NativeDepthWorkflow
        )
    });

    if requires_native_desktop
        && (!row.desktop_handoff.mandatory_desktop_handoff
            || row
                .desktop_handoff
                .desktop_handoff_ref
                .as_deref()
                .unwrap_or_default()
                .is_empty()
            || row.desktop_handoff.exact_reopen_target_ref.is_empty()
            || !row.desktop_handoff.handoff_preserves_target_identity)
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::MandatoryDesktopHandoffMissing,
            &row.row_id,
            "desktop_handoff",
            "native-depth, secret-bearing, protected, admin, publish, and broad mutation flows require exact desktop handoff",
        ));
    }

    if requires_native_desktop
        && (authority.companion_can_grant_approval
            || authority.step_up.approval_owner_surface
                != ApprovalOwnerSurfaceClass::NativeDesktopReviewSurface)
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::CompanionOwnsProtectedApproval,
            &row.row_id,
            "authority.step_up.approval_owner_surface",
            "protected companion-origin actions must be owned by the native desktop review surface",
        ));
    }

    if authority.step_up.step_up_required {
        let reuses_desktop = matches!(
            authority.step_up.step_up_reuse_class,
            StepUpReuseClass::DesktopApprovalTicketReused
                | StepUpReuseClass::DesktopStepUpChallengeReused
                | StepUpReuseClass::ManagedAdminApprovalRequiredOnDesktop
                | StepUpReuseClass::StepUpBlockedCompanionMustHandoff
        );
        if !reuses_desktop
            || authority
                .step_up
                .desktop_review_surface_ref
                .as_deref()
                .unwrap_or_default()
                .is_empty()
        {
            defects.push(CompanionScopeDefect::new(
                CompanionScopeDefectKind::StepUpSemanticsNotReused,
                &row.row_id,
                "authority.step_up",
                "authority-bearing companion rows must reuse desktop step-up or approval semantics",
            ));
        }
    }
}

fn audit_row_fanout(row: &CompanionScopeBetaRow, defects: &mut Vec<CompanionScopeDefect>) {
    if !row.notification_fanout.device_policy_inherited {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::DevicePolicyNotInherited,
            &row.row_id,
            "notification_fanout.device_policy_inherited",
            "companion fanout must inherit device policy before delivery",
        ));
    }
    if !row.notification_fanout.quiet_hours_inherited {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::QuietHoursNotInherited,
            &row.row_id,
            "notification_fanout.quiet_hours_inherited",
            "companion fanout must inherit quiet-hours and suppression policy",
        ));
    }
    if row.notification_fanout.fanout_dedupe_key_ref.is_empty()
        || row.notification_fanout.exact_reopen_target_ref.is_empty()
        || !row.notification_fanout.hidden_attention_spam_prevented
        || !row.notification_fanout.shortcut_bypass_forbidden
    {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::NotificationBypassAllowed,
            &row.row_id,
            "notification_fanout",
            "companion fanout must dedupe, exact-reopen, obey quiet hours, and forbid shortcut mutation bypass",
        ));
    }
}

fn audit_row_support_projection(
    row: &CompanionScopeBetaRow,
    support_rows: &[CompanionScopeBetaSupportRow],
    defects: &mut Vec<CompanionScopeDefect>,
) {
    let Some(support_row) = support_rows
        .iter()
        .find(|support| support.row_id == row.row_id)
    else {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::SupportExportDrift,
            &row.row_id,
            "support_rows",
            "every companion row must have a support-export projection row",
        ));
        return;
    };
    let expected = CompanionScopeBetaSupportRow::from_row(row);
    if support_row != &expected {
        defects.push(CompanionScopeDefect::new(
            CompanionScopeDefectKind::SupportExportDrift,
            &row.row_id,
            "support_rows",
            "support-export projection must preserve scope, target, freshness, handoff, fanout, and provenance truth",
        ));
    }
}

fn build_summary(
    rows: &[CompanionScopeBetaRow],
    defects: &[CompanionScopeDefect],
) -> CompanionScopeBetaSummary {
    let workflows_present = rows
        .iter()
        .map(|row| row.workflow.as_str().to_owned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    CompanionScopeBetaSummary {
        row_count: rows.len(),
        workflows_present,
        stale_or_offline_row_count: rows
            .iter()
            .filter(|row| !row.freshness.freshness_class.permits_live_claim())
            .count(),
        mandatory_desktop_handoff_row_count: rows
            .iter()
            .filter(|row| row.desktop_handoff.mandatory_desktop_handoff)
            .count(),
        desktop_owned_approval_row_count: rows
            .iter()
            .filter(|row| {
                row.authority.step_up.approval_owner_surface
                    == ApprovalOwnerSurfaceClass::NativeDesktopReviewSurface
            })
            .count(),
        quiet_hours_integrated_row_count: rows
            .iter()
            .filter(|row| row.notification_fanout.quiet_hours_inherited)
            .count(),
        defect_count: defects.len(),
        raw_private_material_excluded: rows.iter().all(|row| row.raw_private_material_excluded),
    }
}

fn seeded_rows() -> Vec<CompanionScopeBetaRow> {
    vec![
        row(
            "companion-scope:row:review-triage",
            "companion-scope:case:review-triage",
            CompanionWorkflowClass::ReviewTriage,
            CompanionSurfaceClass::BrowserWebCompanion,
            CompanionClientScopeLabel::CommentCapable,
            target(
                "review_workspace",
                "review-workspace:aur:change-104",
                "execution-context:managed-review:aur:change-104",
                "Review workspace, change 104",
                "canonical-event:review-thread:change-104",
            ),
            freshness(
                CompanionFreshnessClass::LiveAuthoritativeFreshWithinGrace,
                "2026-05-18T17:00:00Z",
            ),
            labels(
                true,
                false,
                true,
                [
                    "browser_companion",
                    "comment_capable",
                    "target_identity",
                    "desktop_approval_required",
                ],
            ),
            authority(
                CompanionAuthorityClass::CommentOrAckViaCanonicalEvent,
                true,
                false,
                step_up(
                    StepUpReuseClass::DesktopApprovalTicketReused,
                    true,
                    Some("approval-ticket:review:change-104:protected-approve"),
                    Some("desktop-review-surface:change-104"),
                    ApprovalOwnerSurfaceClass::NativeDesktopReviewSurface,
                ),
            ),
            handoff(
                true,
                [
                    DesktopHandoffRequirementClass::ProtectedApproval,
                    DesktopHandoffRequirementClass::BroadMutation,
                ],
                Some("desktop-handoff:review:change-104:approve"),
                "reopen:desktop:review-workspace:change-104",
                "Open protected approval in desktop review",
            ),
            fanout(
                "notification-surface:companion:review:change-104",
                DevicePolicyClass::InheritsDesktopNotificationPolicy,
                CompanionQuietHoursClass::InheritsUnifiedQuietHoursPolicy,
                true,
                "fanout-dedupe:review-thread:change-104",
                "reopen:desktop:review-workspace:change-104",
            ),
            provenance(
                "browser-web-companion:review",
                "shell-review-workspace",
                ApprovalOwnerSurfaceClass::NativeDesktopReviewSurface,
                Some("desktop-review-surface:change-104"),
                "support-export:companion-scope:review-triage",
                "canonical-event:review-thread:change-104",
                "audit-event:companion-origin:review-triage",
            ),
            "Review comments stay companion-capable, while protected approval and broad mutation reopen on desktop.",
        ),
        row(
            "companion-scope:row:docs-help",
            "companion-scope:case:docs-help",
            CompanionWorkflowClass::DocsAndHelp,
            CompanionSurfaceClass::BrowserWebCompanion,
            CompanionClientScopeLabel::ReviewOnly,
            target(
                "docs_article",
                "docs-node:remote-helper-skew-beta",
                "execution-context:docs-pack:2026-05-18",
                "Docs article, remote helper skew",
                "canonical-event:docs-help:remote-helper-skew",
            ),
            freshness(
                CompanionFreshnessClass::WarmSnapshotWithinGrace,
                "2026-05-18T16:40:00Z",
            ),
            labels(
                true,
                true,
                false,
                [
                    "browser_companion",
                    "read_only",
                    "warm_snapshot",
                    "source_version_visible",
                ],
            ),
            authority(
                CompanionAuthorityClass::InspectOnlyNoMutation,
                false,
                false,
                step_up(
                    StepUpReuseClass::NoStepUpInspectOnly,
                    false,
                    None,
                    None,
                    ApprovalOwnerSurfaceClass::NoApprovalApplies,
                ),
            ),
            handoff(
                false,
                [],
                None,
                "reopen:desktop:docs:remote-helper-skew-beta",
                "Open related workspace context in desktop",
            ),
            fanout(
                "notification-surface:companion:docs:remote-helper-skew",
                DevicePolicyClass::InheritsDesktopNotificationPolicy,
                CompanionQuietHoursClass::InheritsUnifiedQuietHoursPolicy,
                false,
                "fanout-dedupe:docs:remote-helper-skew",
                "reopen:desktop:docs:remote-helper-skew-beta",
            ),
            provenance(
                "browser-web-companion:docs",
                "shell-docs-browser",
                ApprovalOwnerSurfaceClass::NoApprovalApplies,
                None,
                "support-export:companion-scope:docs-help",
                "canonical-event:docs-help:remote-helper-skew",
                "audit-event:companion-origin:docs-help",
            ),
            "Docs are read-only in browser companion and carry source-version plus warm snapshot labels.",
        ),
        row(
            "companion-scope:row:light-remote-edit",
            "companion-scope:case:light-remote-edit",
            CompanionWorkflowClass::LightRemoteEdit,
            CompanionSurfaceClass::BrowserWebCompanion,
            CompanionClientScopeLabel::LightEdit,
            target(
                "remote_workspace_file",
                "remote-file:managed-workspace:src/app.rs",
                "execution-context:managed-workspace:aur-dev",
                "Managed workspace file, app.rs",
                "canonical-event:light-edit:app-rs",
            ),
            freshness(
                CompanionFreshnessClass::LiveAuthoritativeFreshWithinGrace,
                "2026-05-18T17:02:00Z",
            ),
            labels(
                true,
                false,
                true,
                [
                    "browser_companion",
                    "light_edit",
                    "save_scope_visible",
                    "desktop_handoff_for_deep_edit",
                ],
            ),
            authority(
                CompanionAuthorityClass::LightEditViaDesktopCommandPolicy,
                true,
                false,
                step_up(
                    StepUpReuseClass::DesktopStepUpChallengeReused,
                    true,
                    Some("approval-ticket:remote-edit:app-rs:narrow-save"),
                    Some("desktop-review-surface:remote-edit:app-rs"),
                    ApprovalOwnerSurfaceClass::NativeDesktopReviewSurface,
                ),
            ),
            handoff(
                true,
                [
                    DesktopHandoffRequirementClass::DeepLocalProjectEditing,
                    DesktopHandoffRequirementClass::NativeDepthWorkflow,
                    DesktopHandoffRequirementClass::UnmanagedSecretEntry,
                ],
                Some("desktop-handoff:remote-edit:app-rs:native-depth"),
                "reopen:desktop:managed-workspace:aur-dev:src/app.rs",
                "Open file in desktop for native-depth editing",
            ),
            fanout(
                "notification-surface:companion:light-edit:app-rs",
                DevicePolicyClass::InheritsManagedDevicePolicy,
                CompanionQuietHoursClass::InheritsUnifiedQuietHoursPolicy,
                true,
                "fanout-dedupe:light-edit:app-rs",
                "reopen:desktop:managed-workspace:aur-dev:src/app.rs",
            ),
            provenance(
                "browser-web-companion:light-edit",
                "shell-remote-edit-scope-card",
                ApprovalOwnerSurfaceClass::NativeDesktopReviewSurface,
                Some("desktop-review-surface:remote-edit:app-rs"),
                "support-export:companion-scope:light-remote-edit",
                "canonical-event:light-edit:app-rs",
                "audit-event:companion-origin:light-remote-edit",
            ),
            "Light remote edit is bounded; deep local edits, secrets, terminal, and debugger depth require desktop.",
        ),
        row(
            "companion-scope:row:remote-session-join",
            "companion-scope:case:remote-session-join",
            CompanionWorkflowClass::RemoteSessionJoin,
            CompanionSurfaceClass::MobileNativeCompanion,
            CompanionClientScopeLabel::FollowCapable,
            target(
                "collaboration_session",
                "session:incident-room:database-latency",
                "execution-context:remote-session:database-latency",
                "Incident session, database latency",
                "canonical-event:session-follow:database-latency",
            ),
            freshness(
                CompanionFreshnessClass::LiveAuthoritativeFreshWithinGrace,
                "2026-05-18T17:03:00Z",
            ),
            labels(
                true,
                false,
                true,
                [
                    "mobile_companion",
                    "follow_capable",
                    "session_identity",
                    "desktop_handoff_for_control",
                ],
            ),
            authority(
                CompanionAuthorityClass::ScopedFollowGrantRequest,
                true,
                false,
                step_up(
                    StepUpReuseClass::DesktopApprovalTicketReused,
                    true,
                    Some("approval-ticket:session-follow:database-latency"),
                    Some("desktop-review-surface:session-control:database-latency"),
                    ApprovalOwnerSurfaceClass::NativeDesktopReviewSurface,
                ),
            ),
            handoff(
                true,
                [
                    DesktopHandoffRequirementClass::BroadMutation,
                    DesktopHandoffRequirementClass::NativeDepthWorkflow,
                ],
                Some("desktop-handoff:session:database-latency:control"),
                "reopen:desktop:session:database-latency",
                "Open session controls in desktop",
            ),
            fanout(
                "notification-surface:companion:session:database-latency",
                DevicePolicyClass::InheritsManagedDevicePolicy,
                CompanionQuietHoursClass::HeldByQuietHours,
                true,
                "fanout-dedupe:session:database-latency",
                "reopen:desktop:session:database-latency",
            ),
            provenance(
                "mobile-native-companion:session-follow",
                "shell-session-follow-tile",
                ApprovalOwnerSurfaceClass::NativeDesktopReviewSurface,
                Some("desktop-review-surface:session-control:database-latency"),
                "support-export:companion-scope:remote-session-join",
                "canonical-event:session-follow:database-latency",
                "audit-event:companion-origin:remote-session-join",
            ),
            "Session join can follow and request scoped control; broad control stays desktop-owned.",
        ),
        row(
            "companion-scope:row:ci-status",
            "companion-scope:case:ci-status",
            CompanionWorkflowClass::CiStatus,
            CompanionSurfaceClass::MobileNativeCompanion,
            CompanionClientScopeLabel::StatusOnly,
            target(
                "ci_run",
                "ci-run:provider:build-8821",
                "execution-context:provider-ci:build-8821",
                "CI run 8821",
                "canonical-event:ci-status:build-8821",
            ),
            freshness(
                CompanionFreshnessClass::StaleSnapshotBeyondGrace,
                "2026-05-18T15:52:00Z",
            ),
            labels(
                true,
                true,
                true,
                [
                    "mobile_companion",
                    "status_only",
                    "stale_snapshot",
                    "desktop_handoff_for_rerun_publish",
                ],
            ),
            authority(
                CompanionAuthorityClass::ApprovalRequestOnlyDesktopOwned,
                true,
                false,
                step_up(
                    StepUpReuseClass::DesktopApprovalTicketReused,
                    true,
                    Some("approval-ticket:ci-rerun:build-8821"),
                    Some("desktop-review-surface:ci-rerun:build-8821"),
                    ApprovalOwnerSurfaceClass::NativeDesktopReviewSurface,
                ),
            ),
            handoff(
                true,
                [
                    DesktopHandoffRequirementClass::HighRiskPublish,
                    DesktopHandoffRequirementClass::BroadMutation,
                ],
                Some("desktop-handoff:ci:build-8821:rerun"),
                "reopen:desktop:ci-run:build-8821",
                "Open CI rerun or publish review in desktop",
            ),
            fanout(
                "notification-surface:companion:ci:build-8821",
                DevicePolicyClass::InheritsDesktopNotificationPolicy,
                CompanionQuietHoursClass::InheritsUnifiedQuietHoursPolicy,
                true,
                "fanout-dedupe:ci-run:build-8821",
                "reopen:desktop:ci-run:build-8821",
            ),
            provenance(
                "mobile-native-companion:ci-status",
                "shell-ci-status-card",
                ApprovalOwnerSurfaceClass::NativeDesktopReviewSurface,
                Some("desktop-review-surface:ci-rerun:build-8821"),
                "support-export:companion-scope:ci-status",
                "canonical-event:ci-status:build-8821",
                "audit-event:companion-origin:ci-status",
            ),
            "CI status can inspect a stale snapshot; rerun and publish paths require desktop review.",
        ),
        row(
            "companion-scope:row:incident-awareness",
            "companion-scope:case:incident-awareness",
            CompanionWorkflowClass::IncidentAwareness,
            CompanionSurfaceClass::MobileNativeCompanion,
            CompanionClientScopeLabel::LightIncidentTooling,
            target(
                "incident_workspace",
                "incident:payments-latency:2026-05-18",
                "execution-context:incident:payments-latency",
                "Incident workspace, payments latency",
                "canonical-event:incident-awareness:payments-latency",
            ),
            freshness(
                CompanionFreshnessClass::OfflineSnapshotNoRefreshPath,
                "2026-05-18T14:44:00Z",
            ),
            labels(
                true,
                true,
                true,
                [
                    "mobile_companion",
                    "light_incident_tooling",
                    "offline_snapshot",
                    "desktop_handoff_for_admin_action",
                ],
            ),
            authority(
                CompanionAuthorityClass::CommentOrAckViaCanonicalEvent,
                true,
                false,
                step_up(
                    StepUpReuseClass::ManagedAdminApprovalRequiredOnDesktop,
                    true,
                    Some("approval-ticket:incident-admin:payments-latency"),
                    Some("desktop-review-surface:incident-admin:payments-latency"),
                    ApprovalOwnerSurfaceClass::NativeDesktopReviewSurface,
                ),
            ),
            handoff(
                true,
                [
                    DesktopHandoffRequirementClass::SensitiveAdminFlow,
                    DesktopHandoffRequirementClass::HighRiskPublish,
                    DesktopHandoffRequirementClass::UnmanagedSecretEntry,
                ],
                Some("desktop-handoff:incident:payments-latency:admin"),
                "reopen:desktop:incident:payments-latency",
                "Open incident admin action in desktop",
            ),
            fanout(
                "notification-surface:companion:incident:payments-latency",
                DevicePolicyClass::InheritsManagedDevicePolicy,
                CompanionQuietHoursClass::CriticalSafetyBypassAudited,
                true,
                "fanout-dedupe:incident:payments-latency",
                "reopen:desktop:incident:payments-latency",
            ),
            provenance(
                "mobile-native-companion:incident",
                "shell-incident-snapshot-card",
                ApprovalOwnerSurfaceClass::NativeDesktopReviewSurface,
                Some("desktop-review-surface:incident-admin:payments-latency"),
                "support-export:companion-scope:incident-awareness",
                "canonical-event:incident-awareness:payments-latency",
                "audit-event:companion-origin:incident-awareness",
            ),
            "Incident awareness can acknowledge an offline snapshot, while admin remediation opens desktop.",
        ),
    ]
}

fn row(
    row_id: &str,
    case_id: &str,
    workflow: CompanionWorkflowClass,
    surface_class: CompanionSurfaceClass,
    client_scope_label: CompanionClientScopeLabel,
    target_identity: CompanionTargetIdentityBlock,
    freshness: CompanionFreshnessBlock,
    labels: CompanionVisibleLabelBlock,
    authority: CompanionAuthorityBlock,
    desktop_handoff: CompanionDesktopHandoffBlock,
    notification_fanout: CompanionNotificationFanoutBlock,
    provenance: CompanionActionProvenanceBlock,
    reviewer_note: &str,
) -> CompanionScopeBetaRow {
    CompanionScopeBetaRow {
        record_kind: COMPANION_SCOPE_BETA_ROW_RECORD_KIND.to_owned(),
        schema_version: COMPANION_SCOPE_BETA_SCHEMA_VERSION,
        shared_contract_ref: COMPANION_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        case_id: case_id.to_owned(),
        workflow,
        workflow_token: workflow.as_str().to_owned(),
        surface_class,
        client_scope_label,
        client_scope_token: client_scope_label.as_str().to_owned(),
        target_identity,
        freshness,
        labels,
        authority,
        desktop_handoff,
        notification_fanout,
        provenance,
        explicit_non_promises: CompanionNonPromiseBlock {
            desktop_parity_claimed: false,
            unmanaged_secret_entry_allowed: false,
            deep_local_project_editing_allowed: false,
            terminal_or_debugger_parity_claimed: false,
            admin_console_replacement_claimed: false,
            non_promise_tokens: vec![
                "no_desktop_parity".to_owned(),
                "no_unmanaged_secret_entry".to_owned(),
                "no_deep_local_project_editing".to_owned(),
                "no_terminal_or_debugger_parity".to_owned(),
                "no_admin_console_replacement".to_owned(),
            ],
        },
        raw_private_material_excluded: true,
        reviewer_note: reviewer_note.to_owned(),
    }
}

fn target(
    target_kind: &str,
    target_object_ref: &str,
    execution_context_ref: &str,
    target_identity_label: &str,
    canonical_event_id_ref: &str,
) -> CompanionTargetIdentityBlock {
    CompanionTargetIdentityBlock {
        target_kind: target_kind.to_owned(),
        target_object_ref: target_object_ref.to_owned(),
        execution_context_ref: execution_context_ref.to_owned(),
        target_identity_label: target_identity_label.to_owned(),
        target_identity_label_visible: true,
        canonical_event_id_ref: canonical_event_id_ref.to_owned(),
    }
}

fn freshness(
    freshness_class: CompanionFreshnessClass,
    snapshot_as_of: &str,
) -> CompanionFreshnessBlock {
    CompanionFreshnessBlock {
        freshness_class,
        freshness_token: freshness_class.as_str().to_owned(),
        snapshot_as_of: snapshot_as_of.to_owned(),
        stale_or_offline_label_visible: !freshness_class.permits_live_claim(),
        live_claim_permitted: freshness_class.permits_live_claim(),
    }
}

fn labels<const N: usize>(
    freshness_label_visible: bool,
    read_only_label_visible: bool,
    desktop_handoff_label_visible: bool,
    label_tokens: [&str; N],
) -> CompanionVisibleLabelBlock {
    CompanionVisibleLabelBlock {
        client_scope_label_visible: true,
        target_identity_label_visible: true,
        freshness_label_visible,
        read_only_label_visible,
        desktop_handoff_label_visible,
        label_tokens: label_tokens
            .into_iter()
            .map(std::borrow::ToOwned::to_owned)
            .collect(),
    }
}

fn authority(
    authority_class: CompanionAuthorityClass,
    companion_can_request_approval: bool,
    companion_can_grant_approval: bool,
    step_up: CompanionStepUpBlock,
) -> CompanionAuthorityBlock {
    CompanionAuthorityBlock {
        authority_class,
        authority_token: authority_class.as_str().to_owned(),
        companion_can_request_approval,
        companion_can_grant_approval,
        broad_mutation_allowed_on_companion: false,
        protected_approval_allowed_on_companion: false,
        sensitive_admin_allowed_on_companion: false,
        high_risk_publish_allowed_on_companion: false,
        step_up,
    }
}

fn step_up(
    step_up_reuse_class: StepUpReuseClass,
    step_up_required: bool,
    desktop_approval_ticket_ref: Option<&str>,
    desktop_review_surface_ref: Option<&str>,
    approval_owner_surface: ApprovalOwnerSurfaceClass,
) -> CompanionStepUpBlock {
    CompanionStepUpBlock {
        step_up_reuse_class,
        step_up_reuse_token: step_up_reuse_class.as_str().to_owned(),
        step_up_required,
        desktop_approval_ticket_ref: desktop_approval_ticket_ref.map(str::to_owned),
        desktop_review_surface_ref: desktop_review_surface_ref.map(str::to_owned),
        approval_owner_surface,
        approval_owner_surface_token: approval_owner_surface.as_str().to_owned(),
    }
}

fn handoff<const N: usize>(
    mandatory_desktop_handoff: bool,
    reason_classes: [DesktopHandoffRequirementClass; N],
    desktop_handoff_ref: Option<&str>,
    exact_reopen_target_ref: &str,
    handoff_label: &str,
) -> CompanionDesktopHandoffBlock {
    CompanionDesktopHandoffBlock {
        mandatory_desktop_handoff,
        reason_classes: reason_classes.into_iter().collect(),
        desktop_handoff_ref: desktop_handoff_ref.map(str::to_owned),
        exact_reopen_target_ref: exact_reopen_target_ref.to_owned(),
        handoff_preserves_target_identity: true,
        handoff_label: handoff_label.to_owned(),
    }
}

fn fanout(
    notification_surface_ref: &str,
    device_policy_class: DevicePolicyClass,
    quiet_hours_class: CompanionQuietHoursClass,
    companion_push_allowed: bool,
    fanout_dedupe_key_ref: &str,
    exact_reopen_target_ref: &str,
) -> CompanionNotificationFanoutBlock {
    CompanionNotificationFanoutBlock {
        notification_surface_ref: notification_surface_ref.to_owned(),
        device_policy_class,
        device_policy_token: device_policy_class.as_str().to_owned(),
        device_policy_inherited: true,
        quiet_hours_class,
        quiet_hours_token: quiet_hours_class.as_str().to_owned(),
        quiet_hours_inherited: true,
        fanout_dedupe_key_ref: fanout_dedupe_key_ref.to_owned(),
        companion_push_allowed,
        hidden_attention_spam_prevented: true,
        shortcut_bypass_forbidden: true,
        exact_reopen_target_ref: exact_reopen_target_ref.to_owned(),
    }
}

fn provenance(
    origin_surface_ref: &str,
    displayed_object_surface: &str,
    approval_owner_surface: ApprovalOwnerSurfaceClass,
    desktop_handoff_destination_ref: Option<&str>,
    support_export_ref: &str,
    canonical_event_id_ref: &str,
    audit_event_id_ref: &str,
) -> CompanionActionProvenanceBlock {
    CompanionActionProvenanceBlock {
        origin_surface_ref: origin_surface_ref.to_owned(),
        displayed_object_surface: displayed_object_surface.to_owned(),
        approval_owner_surface,
        desktop_handoff_destination_ref: desktop_handoff_destination_ref.map(str::to_owned),
        support_export_ref: support_export_ref.to_owned(),
        canonical_event_id_ref: canonical_event_id_ref.to_owned(),
        audit_event_id_ref: audit_event_id_ref.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_covers_required_workflows() {
        let page = seeded_companion_scope_beta_page();
        validate_companion_scope_beta_page(&page).expect("seeded page validates");
        assert!(page.covers_required_workflows());
        assert_eq!(page.summary.row_count, 6);
        assert_eq!(page.summary.defect_count, 0);
        assert!(page.summary.stale_or_offline_row_count >= 2);
        assert_eq!(
            page.summary.quiet_hours_integrated_row_count,
            page.rows.len()
        );
    }

    #[test]
    fn support_export_validates_with_lineage() {
        let page = seeded_companion_scope_beta_page();
        let export = CompanionScopeBetaSupportExport::from_page(
            "support-export:companion-scope:seed",
            "2026-05-18T17:05:00Z",
            page,
        );
        validate_companion_scope_beta_support_export(&export).expect("support export validates");
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
        assert_eq!(export.rows.len(), export.case_ids.len());
    }

    #[test]
    fn validator_flags_stale_snapshot_without_label() {
        let mut page = seeded_companion_scope_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|row| row.workflow == CompanionWorkflowClass::CiStatus)
            .expect("seeded ci row exists");
        row.freshness.stale_or_offline_label_visible = false;
        row.labels.freshness_label_visible = false;
        page.support_rows = page
            .rows
            .iter()
            .map(CompanionScopeBetaSupportRow::from_row)
            .collect();
        let defects = audit_companion_scope_beta_rows(&page.rows, &page.support_rows);
        assert!(
            defects
                .iter()
                .any(|defect| defect.defect_kind
                    == CompanionScopeDefectKind::StaleOfflineLabelMissing)
        );
    }

    #[test]
    fn validator_flags_companion_owned_protected_approval() {
        let mut page = seeded_companion_scope_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|row| row.workflow == CompanionWorkflowClass::ReviewTriage)
            .expect("seeded review row exists");
        row.authority.companion_can_grant_approval = true;
        row.authority.step_up.approval_owner_surface =
            ApprovalOwnerSurfaceClass::CompanionRequestOnly;
        row.authority.step_up.approval_owner_surface_token =
            ApprovalOwnerSurfaceClass::CompanionRequestOnly
                .as_str()
                .to_owned();
        page.support_rows = page
            .rows
            .iter()
            .map(CompanionScopeBetaSupportRow::from_row)
            .collect();
        let defects = audit_companion_scope_beta_rows(&page.rows, &page.support_rows);
        assert!(defects.iter().any(|defect| {
            defect.defect_kind == CompanionScopeDefectKind::CompanionOwnsProtectedApproval
        }));
    }
}
