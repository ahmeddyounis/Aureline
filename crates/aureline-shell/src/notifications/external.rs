//! Privacy-safe external notification payload projection.
//!
//! OS notifications, lock-screen summaries, and companion pushes are summary
//! surfaces. They can mirror routed notification truth, but they cannot expose
//! raw private material or complete privileged mutations. This module projects
//! a [`RoutedNotification`] into the bounded payload those surfaces may render.

use serde::{Deserialize, Serialize};

use super::envelope::{
    FanoutReceiptState, FanoutSurfaceClass, PrivacyPayloadClass, ReopenTarget, SeverityClass,
    SourceSubsystem, StableAction, StaleOrUndeliveredReason, StaleOrUndeliveredReasonClass,
};
use super::router::{RoutedNotification, SurfaceRoute};

/// Schema version for [`ExternalNotificationPayload`].
pub const EXTERNAL_NOTIFICATION_PAYLOAD_SCHEMA_VERSION: u32 = 1;
/// Stable record kind for external notification payloads.
pub const EXTERNAL_NOTIFICATION_PAYLOAD_RECORD_KIND: &str = "external_notification_payload_record";

/// Shortcut action classes that summary-only surfaces must never complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForbiddenShortcutActionClass {
    /// Publishing, applying, or otherwise committing destructive work.
    DestructivePublishOrApply,
    /// Revealing a secret or credential.
    SecretOrCredentialReveal,
    /// Performing an irreversible high-blast-radius action.
    IrreversibleHighBlast,
    /// Skipping a required review sheet.
    BypassReviewSheet,
    /// Skipping a required approval workflow.
    BypassApprovalWorkflow,
    /// Mutating another workspace from a summary surface.
    CrossWorkspaceMutation,
    /// Mutating directly from a lock-screen action.
    DirectMutationFromLockScreen,
    /// Mutating directly from a companion push.
    DirectMutationFromCompanionPush,
    /// Mutating directly from a dock or taskbar shortcut.
    DirectMutationFromDockOrTaskbar,
    /// Mutating directly from a system-tray action.
    DirectMutationFromSystemTray,
    /// Overriding policy from an OS shortcut.
    PolicyOverrideFromOsShortcut,
    /// Changing trust state from an OS shortcut.
    TrustStateChangeFromOsShortcut,
    /// Changing provider grants from an OS shortcut.
    ProviderGrantChangeFromOsShortcut,
}

impl ForbiddenShortcutActionClass {
    /// Stable token recorded in payloads and audit artifacts.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DestructivePublishOrApply => "destructive_publish_or_apply",
            Self::SecretOrCredentialReveal => "secret_or_credential_reveal",
            Self::IrreversibleHighBlast => "irreversible_high_blast",
            Self::BypassReviewSheet => "bypass_review_sheet",
            Self::BypassApprovalWorkflow => "bypass_approval_workflow",
            Self::CrossWorkspaceMutation => "cross_workspace_mutation",
            Self::DirectMutationFromLockScreen => "direct_mutation_from_lock_screen",
            Self::DirectMutationFromCompanionPush => "direct_mutation_from_companion_push",
            Self::DirectMutationFromDockOrTaskbar => "direct_mutation_from_dock_or_taskbar",
            Self::DirectMutationFromSystemTray => "direct_mutation_from_system_tray",
            Self::PolicyOverrideFromOsShortcut => "policy_override_from_os_shortcut",
            Self::TrustStateChangeFromOsShortcut => "trust_state_change_from_os_shortcut",
            Self::ProviderGrantChangeFromOsShortcut => "provider_grant_change_from_os_shortcut",
        }
    }
}

/// Privacy-safe payload emitted to an OS, lock-screen, or companion surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalNotificationPayload {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable payload id.
    pub external_payload_id: String,
    /// Source notification envelope id.
    pub notification_envelope_id: String,
    /// Canonical event id shared with durable in-product state.
    pub canonical_event_id: String,
    /// Surface this payload targets.
    pub delivery_surface_class: FanoutSurfaceClass,
    /// Outcome from the routed fanout receipt.
    pub receipt_state: FanoutReceiptState,
    /// Stale or failed delivery reason when not delivered.
    pub stale_or_undelivered_reason: StaleOrUndeliveredReason,
    /// Source subsystem.
    pub source_subsystem: SourceSubsystem,
    /// Severity class.
    pub severity_class: SeverityClass,
    /// Payload privacy class.
    pub privacy_payload_class: PrivacyPayloadClass,
    /// Summary-safe copy. This never contains raw bodies, paths, URLs,
    /// prompt text, secret material, or customer-owned identifiers.
    pub summary_label: String,
    /// At most one safe primary action. Mutating or privileged actions are
    /// omitted and must reopen in-product instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safe_primary_action: Option<StableAction>,
    /// Exact in-product reopen target.
    pub exact_reopen_target: ReopenTarget,
    /// True when activation is only an exact reopen, not shortcut replay.
    pub exact_reopen_only: bool,
    /// True when privileged detail must be inspected in-product.
    pub opens_in_product_for_privileged_detail: bool,
    /// True when raw private material was excluded from the payload.
    pub raw_private_material_excluded: bool,
    /// True when summary-surface shortcuts are barred from bypassing review.
    pub shortcut_bypass_prohibited: bool,
    /// Forbidden shortcut classes this payload refuses to complete.
    pub forbidden_shortcut_action_classes: Vec<ForbiddenShortcutActionClass>,
    /// True when a held, suppressed, stale, failed, or deduped external
    /// fanout remains visible as durable truth.
    pub fanout_not_delivered_visible_truth: bool,
    /// Timestamp from the routed notification.
    pub minted_at: String,
}

impl ExternalNotificationPayload {
    /// Projects a routed notification onto one external summary surface.
    pub fn project(
        routed: &RoutedNotification,
        surface: FanoutSurfaceClass,
    ) -> Option<ExternalNotificationPayload> {
        if !matches!(
            surface,
            FanoutSurfaceClass::OsNotification
                | FanoutSurfaceClass::LockScreenSummary
                | FanoutSurfaceClass::CompanionPush
        ) {
            return None;
        }
        let route = routed
            .surface_routes
            .iter()
            .find(|route| route.fanout_surface_class == surface)?;
        Some(Self::from_routed_and_route(routed, route))
    }

    fn from_routed_and_route(routed: &RoutedNotification, route: &SurfaceRoute) -> Self {
        Self {
            record_kind: EXTERNAL_NOTIFICATION_PAYLOAD_RECORD_KIND.to_owned(),
            schema_version: EXTERNAL_NOTIFICATION_PAYLOAD_SCHEMA_VERSION,
            external_payload_id: format!(
                "external-payload:{}:{}",
                routed.notification_envelope_id,
                route.fanout_surface_class.as_str()
            ),
            notification_envelope_id: routed.notification_envelope_id.clone(),
            canonical_event_id: routed.canonical_event_id.clone(),
            delivery_surface_class: route.fanout_surface_class,
            receipt_state: route.receipt_state,
            stale_or_undelivered_reason: route.stale_or_undelivered_reason.clone(),
            source_subsystem: routed.source_subsystem,
            severity_class: routed.severity_class,
            privacy_payload_class: routed.privacy_payload_class,
            summary_label: external_summary_label(routed),
            safe_primary_action: routed
                .actions
                .iter()
                .find(|action| action_is_safe_external_primary(action, &routed.reopen_target))
                .cloned(),
            exact_reopen_target: routed.reopen_target.clone(),
            exact_reopen_only: true,
            opens_in_product_for_privileged_detail: true,
            raw_private_material_excluded: true,
            shortcut_bypass_prohibited: true,
            forbidden_shortcut_action_classes: forbidden_shortcut_action_classes(),
            fanout_not_delivered_visible_truth: route_not_delivered_but_visible(route),
            minted_at: routed.minted_at.clone(),
        }
    }
}

fn route_not_delivered_but_visible(route: &SurfaceRoute) -> bool {
    !matches!(
        route.receipt_state,
        FanoutReceiptState::Delivered | FanoutReceiptState::ReleasedFromHold
    ) || !matches!(
        route.stale_or_undelivered_reason.reason_class,
        StaleOrUndeliveredReasonClass::None
    )
}

fn external_summary_label(routed: &RoutedNotification) -> String {
    match routed.privacy_payload_class {
        PrivacyPayloadClass::LockScreenSafeScoped => routed.summary_label.clone(),
        PrivacyPayloadClass::LockScreenSafeGeneric => {
            format!(
                "{} {} notification",
                severity_label(routed.severity_class),
                subsystem_label(routed.source_subsystem)
            )
        }
        PrivacyPayloadClass::RedactedMetadataOnly => {
            format!("{} notification", subsystem_label(routed.source_subsystem))
        }
        PrivacyPayloadClass::InProductOnly | PrivacyPayloadClass::PolicyForbiddenOnLockScreen => {
            "Open Aureline to view details".to_owned()
        }
    }
}

fn severity_label(severity: SeverityClass) -> &'static str {
    match severity {
        SeverityClass::Info => "Info",
        SeverityClass::Success => "Success",
        SeverityClass::Warning => "Warning",
        SeverityClass::Degraded => "Degraded",
        SeverityClass::Error => "Error",
        SeverityClass::Blocking => "Blocking",
        SeverityClass::Critical => "Critical",
    }
}

fn subsystem_label(source: SourceSubsystem) -> &'static str {
    match source {
        SourceSubsystem::Editor => "editor",
        SourceSubsystem::Terminal => "terminal",
        SourceSubsystem::ReviewAndDiff => "review",
        SourceSubsystem::PaletteAndSearch => "search",
        SourceSubsystem::InstallUpdateAttach => "install",
        SourceSubsystem::AiApply => "AI apply",
        SourceSubsystem::Collaboration => "collaboration",
        SourceSubsystem::ProviderBearing => "provider",
        SourceSubsystem::DocsHelpServiceHealth => "docs",
        SourceSubsystem::SupportExport => "support",
        SourceSubsystem::BuildSystem => "build",
        SourceSubsystem::TestRunner => "test",
        SourceSubsystem::DebugSession => "debug",
        SourceSubsystem::TaskRunner => "task",
        SourceSubsystem::Indexer => "indexer",
        SourceSubsystem::VfsSave => "save",
        SourceSubsystem::SyncMirror => "sync",
        SourceSubsystem::NotebookKernel => "notebook",
        SourceSubsystem::RemoteAgent => "remote",
        SourceSubsystem::ExtensionHost => "extension",
        SourceSubsystem::WorkspaceTrust => "trust",
        SourceSubsystem::PolicyResolver => "policy",
        SourceSubsystem::AdminPolicy => "admin",
        SourceSubsystem::SecretBroker => "security",
        SourceSubsystem::RuntimePowerManager => "runtime",
        SourceSubsystem::Shell => "shell",
    }
}

fn action_is_safe_external_primary(action: &StableAction, reopen_target: &ReopenTarget) -> bool {
    if action.is_destructive {
        return false;
    }
    let matches_exact_target = reopen_target
        .exact_target_identity_ref
        .as_deref()
        .map(|exact| exact == action.target_identity_ref)
        .unwrap_or(false);
    matches_exact_target && command_is_open_only(&action.command_id)
}

fn command_is_open_only(command_id: &str) -> bool {
    command_id.contains(".open")
        || command_id.contains("_open")
        || command_id.contains(".focus")
        || command_id.ends_with(".show")
}

fn forbidden_shortcut_action_classes() -> Vec<ForbiddenShortcutActionClass> {
    use ForbiddenShortcutActionClass as Class;
    vec![
        Class::DestructivePublishOrApply,
        Class::SecretOrCredentialReveal,
        Class::IrreversibleHighBlast,
        Class::BypassReviewSheet,
        Class::BypassApprovalWorkflow,
        Class::CrossWorkspaceMutation,
        Class::DirectMutationFromLockScreen,
        Class::DirectMutationFromCompanionPush,
        Class::DirectMutationFromDockOrTaskbar,
        Class::DirectMutationFromSystemTray,
        Class::PolicyOverrideFromOsShortcut,
        Class::TrustStateChangeFromOsShortcut,
        Class::ProviderGrantChangeFromOsShortcut,
    ]
}
