//! One reusable M5 notification / activity-center handoff routing primitive: an
//! advisory or revocation event routed into a durable activity-center row, a
//! privacy-safe native OS notification where policy allows, a Help/About summary, and a
//! support-bundle export field with the same model, so a security-affecting event never
//! collapses to a badge-only, toast-only, or website-only state and always reopens onto
//! the authoritative affected-install or disclosure surface.
//!
//! Aureline's frozen advisory-component matrix
//! ([`crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix`])
//! names the native-notification handoff as a governed component family and freezes the
//! controlled severity classes, action states, required actions, continuity claims,
//! delivery profiles, mirror-freshness states, notification behaviors, export fields, and
//! accessibility routes an advisory component may use. This module *implements* that
//! notification-handoff contract as one reusable routing primitive so an event delivered
//! while the app is focused, while it is backgrounded, during quiet hours, under a
//! do-not-disturb rule, deferred behind an offline / mirror lag, or restricted by a
//! managed policy reads the same everywhere it surfaces — instead of degrading to a bare
//! badge, a transient toast that disappears, or a link to an external page that hides the
//! current advisory state.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_notification_handoff`] — that takes one advisory or
//!    revocation event on one notification-delivery lane (its copy-safe advisory id,
//!    severity, event kind, affected scope, current status, authoritative reopen surface,
//!    reopen target, signer / source state, delivery profile, mirror freshness, action
//!    state, primary action, and local-continuity claim) and produces one
//!    [`M5ResolvedNotificationHandoff`] that derives the delivery posture from the
//!    delivery lane and severity (so a suppressed OS notification still lands durably in
//!    the activity center instead of collapsing to a badge, and an emergency severity
//!    bypasses quiet hours), keeps the reopen target pointed at the authoritative
//!    affected-install or disclosure surface (never a dead-end link), derives the
//!    privacy-safe native-notification behaviors, keeps the event durable and visible,
//!    projects the same advisory truth into every claimed channel, and emits a copy-safe,
//!    export-safe summary. The resolver never lets an event collapse to a badge-only,
//!    toast-only, or website-only state and never splits the notification and activity-row
//!    vocabulary.
//! 2. A parity matrix — [`M5NotificationHandoffPacket`] — that binds one row per claimed
//!    notification-delivery lane to the shared handoff anatomy, the same severity
//!    vocabulary, the same channels, the same notification behaviors, the same event
//!    kinds, the same export fields, and the same accessibility routes, so the activity
//!    center, native notifications, Help/About, and support-bundle surfaces render the
//!    same advisory event from one shared model.
//!
//! The severity classes ([`M5AdvisorySeverityClass`]), action states
//! ([`M5AdvisoryActionState`]), required actions ([`M5AdvisoryRequiredAction`]),
//! continuity claims ([`M5AdvisoryContinuityClaim`]), delivery profiles
//! ([`M5AdvisoryDeliveryProfile`]), mirror-freshness states
//! ([`M5AdvisoryFreshnessState`]), notification behaviors
//! ([`M5AdvisoryNotificationBehavior`]), export fields ([`M5AdvisoryExportField`]),
//! accessibility routes ([`M5AdvisoryAccessibilityRoute`]), qualification classes
//! ([`M5AdvisoryQualificationClass`]), and downgrade triggers
//! ([`M5AdvisoryDowngradeTrigger`]) are reused verbatim from the frozen advisory matrix;
//! the shell topology — zones, responsive classes, window classes, and consumer surfaces —
//! is reused from the frozen shell-zone matrix. This module mints new vocabulary only for
//! what the frozen matrix left implicit about the handoff itself: its notification-delivery
//! lanes, its handoff anatomy, its channels, its focus behaviors, its event kinds, and the
//! derived delivery posture and reopen surface. No M5 surface invents a second
//! notification grammar.
//!
//! Raw hostnames, raw absolute paths, raw exploit payloads, raw signatures, private
//! registry URLs, credentials, and raw notification bodies stay outside the support
//! boundary; opaque, export-safe reprs are the only material carried, and the advisory id
//! is a copy-safe identifier, never a link.
//!
//! The boundary schema is
//! [`schemas/security/m5-notification-activity-handoff.schema.json`](../../../../schemas/security/m5-notification-activity-handoff.schema.json)
//! and the contract doc is
//! [`docs/security/m5_notification_activity_handoff_primitive_contract.md`](../../../../docs/security/m5_notification_activity_handoff_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/security/m5-notification-activity-handoff-primitive/`](../../../../fixtures/security/m5-notification-activity-handoff-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_notification_activity_handoff_primitive_offline_deferred_preview_narrowed,
    seeded_m5_notification_activity_handoff_primitive_packet,
    seeded_m5_notification_activity_handoff_primitive_quiet_hours_beta_narrowed,
    M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_PACKET_ID,
};

// The severity classes, action states, required actions, continuity claims, delivery
// profiles, mirror-freshness states, notification behaviors, export fields, accessibility
// routes, qualification classes, and downgrade triggers are frozen once, in the
// advisory-component matrix. This primitive reuses them verbatim so it never invents a
// parallel severity vocabulary or a second notification grammar.
pub use crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix::{
    M5AdvisoryAccessibilityRoute, M5AdvisoryActionState, M5AdvisoryContinuityClaim,
    M5AdvisoryDeliveryProfile, M5AdvisoryDowngradeTrigger, M5AdvisoryExportField,
    M5AdvisoryFreshnessState, M5AdvisoryNotificationBehavior, M5AdvisoryQualificationClass,
    M5AdvisoryRequiredAction, M5AdvisorySeverityClass,
};

// The canonical shell topology — zones, responsive classes, window classes, and
// consumer surfaces — is frozen once, in the shell-zone matrix.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5NotificationHandoffPacket`].
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_notification_and_activity_center_handoff_durable_reopenable_routing_and_help_support_parity_primitive";

/// Schema version for M5 notification-activity-handoff-primitive records.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the notification-activity-handoff-primitive boundary schema.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_SCHEMA_REF: &str =
    "schemas/security/m5-notification-activity-handoff.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_DOC_REF: &str =
    "docs/security/m5_notification_activity_handoff_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds against.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_SHELL_ZONE_REF: &str =
    "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen advisory-component matrix this primitive narrows
/// from.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_COMPONENT_MATRIX_REF: &str =
    "schemas/security/m5-advisory-component-matrix.schema.json";

/// Repo-relative path of the frozen advisory-identity record this primitive aligns its
/// copy-safe advisory-id vocabulary to.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_IDENTITY_REF: &str =
    "schemas/security/advisory_identity.schema.json";

/// Repo-relative path of the frozen OS-notification / quiet-hours contract this primitive
/// aligns its delivery-posture and privacy-safe-payload behavior to.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_OS_NOTIFICATION_DOC_REF: &str =
    "docs/ux/os_notification_and_quiet_hours_contract.md";

/// Repo-relative path of the frozen attention-routing schema this primitive aligns its
/// durable activity-center routing and reopen continuity to.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_ATTENTION_ROUTING_REF: &str =
    "schemas/activity/m5-attention-routing.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_FIXTURE_DIR: &str =
    "fixtures/security/m5-notification-activity-handoff-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_ARTIFACT_REF: &str =
    "artifacts/release/m5-notification-activity-handoff-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_CSV_REF: &str =
    "artifacts/release/m5-notification-activity-handoff-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_REPORT_REF: &str =
    "artifacts/security/m5-notification-activity-handoff-primitive.md";

/// The export fields every notification handoff's support summary must carry so a support
/// bundle and Help/About reconstruct the advisory event without a screenshot and never
/// silently drop the affected scope or the continuity note.
pub const MANDATORY_EXPORT_FIELDS: [M5AdvisoryExportField; 6] = [
    M5AdvisoryExportField::AdvisoryId,
    M5AdvisoryExportField::Severity,
    M5AdvisoryExportField::ActionState,
    M5AdvisoryExportField::AffectedSurface,
    M5AdvisoryExportField::MitigationState,
    M5AdvisoryExportField::ContinuityNote,
];

/// `true` when the severity is emergency-grade — critical or operational emergency — and
/// therefore bypasses quiet hours / do-not-disturb rather than being held for later.
pub const fn severity_is_emergency_grade(severity: M5AdvisorySeverityClass) -> bool {
    matches!(
        severity,
        M5AdvisorySeverityClass::Critical | M5AdvisorySeverityClass::OperationalEmergency
    )
}

/// One claimed notification-delivery lane a handoff can route through. These are the
/// delivery contexts the goal names — the event stays durable and reopenable whether it is
/// delivered live, deferred, or suppressed by a quiet-hours / do-not-disturb / managed
/// policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationDeliveryLane {
    /// The app is in the foreground and focused.
    ForegroundFocused,
    /// The app is running but unfocused / backgrounded.
    BackgroundUnfocused,
    /// Quiet hours are active for non-emergency severities.
    QuietHoursActive,
    /// A do-not-disturb rule is enforced.
    DoNotDisturbEnforced,
    /// Delivery is deferred behind an offline / mirror lag.
    OfflineOrMirrorDeferred,
    /// A managed policy restricts OS-level notifications.
    ManagedPolicyRestricted,
}

impl M5NotificationDeliveryLane {
    /// Every notification-delivery lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ForegroundFocused,
        Self::BackgroundUnfocused,
        Self::QuietHoursActive,
        Self::DoNotDisturbEnforced,
        Self::OfflineOrMirrorDeferred,
        Self::ManagedPolicyRestricted,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ForegroundFocused => "foreground_focused",
            Self::BackgroundUnfocused => "background_unfocused",
            Self::QuietHoursActive => "quiet_hours_active",
            Self::DoNotDisturbEnforced => "do_not_disturb_enforced",
            Self::OfflineOrMirrorDeferred => "offline_or_mirror_deferred",
            Self::ManagedPolicyRestricted => "managed_policy_restricted",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ForegroundFocused => "Foreground Focused",
            Self::BackgroundUnfocused => "Background Unfocused",
            Self::QuietHoursActive => "Quiet Hours Active",
            Self::DoNotDisturbEnforced => "Do-Not-Disturb Enforced",
            Self::OfflineOrMirrorDeferred => "Offline / Mirror Deferred",
            Self::ManagedPolicyRestricted => "Managed Policy Restricted",
        }
    }

    /// `true` when this lane suppresses OS-level notifications for non-emergency
    /// severities — exactly the lanes that must keep the event durable in the activity
    /// center instead of collapsing to a badge.
    pub const fn suppresses_os_notification(self) -> bool {
        matches!(
            self,
            Self::QuietHoursActive | Self::DoNotDisturbEnforced | Self::ManagedPolicyRestricted
        )
    }

    /// The delivery posture this lane resolves to for a given severity. A suppressed lane
    /// keeps a non-emergency event durable in the activity center (never badge-only) and
    /// still delivers an emergency-grade event by bypassing the suppression; an offline /
    /// mirror-deferred lane defers then lands durably.
    pub const fn delivery_posture(
        self,
        severity: M5AdvisorySeverityClass,
    ) -> M5NotificationDeliveryPosture {
        match self {
            Self::ForegroundFocused | Self::BackgroundUnfocused => {
                M5NotificationDeliveryPosture::NativeNotificationPlusActivityRow
            }
            Self::QuietHoursActive | Self::DoNotDisturbEnforced | Self::ManagedPolicyRestricted => {
                if severity_is_emergency_grade(severity) {
                    M5NotificationDeliveryPosture::EmergencyBypassDelivered
                } else {
                    M5NotificationDeliveryPosture::ActivityCenterDurableOnly
                }
            }
            Self::OfflineOrMirrorDeferred => M5NotificationDeliveryPosture::DeferredThenDurable,
        }
    }
}

/// One anatomy part the shared notification / activity handoff surfaces. Every part is
/// mandatory: the whole point of the primitive is that the event identity, severity,
/// affected scope, current status, delivery state, reopen target, and primary action are
/// carried by every route without hiding behind a secondary detail drawer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationHandoffAnatomyPart {
    /// The advisory id and event kind.
    EventIdentity,
    /// The severity.
    Severity,
    /// The affected scope / surface.
    AffectedScope,
    /// The current status / mitigation state.
    CurrentStatus,
    /// The delivery state (posture) of the handoff.
    DeliveryState,
    /// The reopen target (the authoritative surface).
    ReopenTarget,
    /// The primary action.
    PrimaryAction,
}

impl M5NotificationHandoffAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::EventIdentity,
        Self::Severity,
        Self::AffectedScope,
        Self::CurrentStatus,
        Self::DeliveryState,
        Self::ReopenTarget,
        Self::PrimaryAction,
    ];

    /// The anatomy parts every handoff must render inline. All parts are mandatory — no
    /// advisory truth may hide behind a detail drawer.
    pub const MANDATORY: [Self; 7] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EventIdentity => "event_identity",
            Self::Severity => "severity",
            Self::AffectedScope => "affected_scope",
            Self::CurrentStatus => "current_status",
            Self::DeliveryState => "delivery_state",
            Self::ReopenTarget => "reopen_target",
            Self::PrimaryAction => "primary_action",
        }
    }
}

/// One channel that renders the shared notification / activity handoff. Every handoff
/// projects the same advisory id, severity, affected scope, delivery posture, and reopen
/// surface into all four so the activity center, native notifications, Help/About, and
/// support-bundle surfaces describe the same event and share one advisory vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationChannel {
    /// The durable in-product activity center / history.
    ActivityCenter,
    /// A privacy-safe native OS notification.
    NativeNotification,
    /// The Help / About surface.
    HelpAbout,
    /// A support-bundle export.
    SupportBundle,
}

impl M5NotificationChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ActivityCenter,
        Self::NativeNotification,
        Self::HelpAbout,
        Self::SupportBundle,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivityCenter => "activity_center",
            Self::NativeNotification => "native_notification",
            Self::HelpAbout => "help_about",
            Self::SupportBundle => "support_bundle",
        }
    }
}

/// A focus / navigation behavior the handoff supports so the delivery state, the affected
/// scope, the reopen action, and the advisory id stay keyboard-reachable and never
/// hover-only or color-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationFocusBehavior {
    /// The activity row is reachable and operable by keyboard focus.
    RowKeyboardFocusable,
    /// The native notification is keyboard-dismissible and its dismissal syncs in-app.
    NotificationKeyboardDismissible,
    /// The reopen action is keyboard-reachable.
    ReopenActionReachable,
    /// The delivery state is announced to a screen reader, never color-only.
    DeliveryStateAnnouncedToScreenReader,
    /// The affected scope is announced to a screen reader.
    AffectedScopeAnnouncedToScreenReader,
    /// A stable deep-link anchor jumps to the authoritative surface.
    DeepLinkToAuthoritativeSurface,
}

impl M5NotificationFocusBehavior {
    /// Every focus behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RowKeyboardFocusable,
        Self::NotificationKeyboardDismissible,
        Self::ReopenActionReachable,
        Self::DeliveryStateAnnouncedToScreenReader,
        Self::AffectedScopeAnnouncedToScreenReader,
        Self::DeepLinkToAuthoritativeSurface,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RowKeyboardFocusable => "row_keyboard_focusable",
            Self::NotificationKeyboardDismissible => "notification_keyboard_dismissible",
            Self::ReopenActionReachable => "reopen_action_reachable",
            Self::DeliveryStateAnnouncedToScreenReader => {
                "delivery_state_announced_to_screen_reader"
            }
            Self::AffectedScopeAnnouncedToScreenReader => {
                "affected_scope_announced_to_screen_reader"
            }
            Self::DeepLinkToAuthoritativeSurface => "deep_link_to_authoritative_surface",
        }
    }
}

/// The kind of advisory / revocation event a handoff routes. The activity center and the
/// native notification carry the same event kind so a security-affecting event reads the
/// same across every route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationEventKind {
    /// A newly published advisory.
    AdvisoryPublished,
    /// An updated advisory.
    AdvisoryUpdated,
    /// A revocation / kill event.
    AdvisoryRevoked,
    /// An emergency notice.
    EmergencyNotice,
    /// A mitigation became available.
    MitigationAvailable,
    /// The advisory reached resolved.
    AdvisoryResolved,
}

impl M5NotificationEventKind {
    /// Every event kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AdvisoryPublished,
        Self::AdvisoryUpdated,
        Self::AdvisoryRevoked,
        Self::EmergencyNotice,
        Self::MitigationAvailable,
        Self::AdvisoryResolved,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvisoryPublished => "advisory_published",
            Self::AdvisoryUpdated => "advisory_updated",
            Self::AdvisoryRevoked => "advisory_revoked",
            Self::EmergencyNotice => "emergency_notice",
            Self::MitigationAvailable => "mitigation_available",
            Self::AdvisoryResolved => "advisory_resolved",
        }
    }
}

/// The normalized delivery posture a handoff resolves to. This is a resolver-side
/// vocabulary derived from the delivery lane and severity so a suppressed OS notification
/// still lands durably in the activity center instead of collapsing to a badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationDeliveryPosture {
    /// A native OS notification plus a durable activity row.
    NativeNotificationPlusActivityRow,
    /// The OS notification is suppressed by policy but the event lands durably in the
    /// activity center — never badge-only.
    ActivityCenterDurableOnly,
    /// An emergency-grade event bypasses quiet hours / do-not-disturb and is delivered.
    EmergencyBypassDelivered,
    /// Delivery is deferred behind an offline / mirror lag, then lands durably.
    DeferredThenDurable,
}

impl M5NotificationDeliveryPosture {
    /// Every delivery posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NativeNotificationPlusActivityRow,
        Self::ActivityCenterDurableOnly,
        Self::EmergencyBypassDelivered,
        Self::DeferredThenDurable,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeNotificationPlusActivityRow => "native_notification_plus_activity_row",
            Self::ActivityCenterDurableOnly => "activity_center_durable_only",
            Self::EmergencyBypassDelivered => "emergency_bypass_delivered",
            Self::DeferredThenDurable => "deferred_then_durable",
        }
    }

    /// `true` when this posture delivers a live native OS notification (as opposed to a
    /// suppressed or deferred posture that lands durably in the activity center only).
    pub const fn delivers_native_os_notification(self) -> bool {
        matches!(
            self,
            Self::NativeNotificationPlusActivityRow | Self::EmergencyBypassDelivered
        )
    }
}

/// The authoritative surface a handoff reopens onto. Every notification and activity row
/// lands on one of these in-product surfaces — never a dead-end badge or external page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationReopenSurface {
    /// The affected-install assessment panel.
    AffectedInstallPanel,
    /// The disclosure / history block.
    DisclosureBlock,
    /// The advisory card.
    AdvisoryCard,
    /// The emergency-notice banner.
    EmergencyNotice,
}

impl M5NotificationReopenSurface {
    /// Every reopen surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AffectedInstallPanel,
        Self::DisclosureBlock,
        Self::AdvisoryCard,
        Self::EmergencyNotice,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AffectedInstallPanel => "affected_install_panel",
            Self::DisclosureBlock => "disclosure_block",
            Self::AdvisoryCard => "advisory_card",
            Self::EmergencyNotice => "emergency_notice",
        }
    }
}

/// The full input to the notification-handoff resolver for one advisory / revocation event
/// on one notification-delivery lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationHandoffResolutionInput {
    /// The notification-delivery lane this handoff routes through.
    pub delivery_lane: M5NotificationDeliveryLane,
    /// The copy-safe Aureline advisory id.
    pub advisory_id: String,
    /// The advisory's severity.
    pub severity: M5AdvisorySeverityClass,
    /// The advisory / revocation event kind.
    pub event_kind: M5NotificationEventKind,
    /// Opaque, export-safe representation of the affected scope / surface.
    pub affected_scope_repr: String,
    /// Opaque, export-safe representation of the current status / mitigation state.
    pub current_status_repr: String,
    /// The authoritative surface this handoff reopens onto.
    pub authoritative_surface: M5NotificationReopenSurface,
    /// Opaque, export-safe representation of the reopen target (deep-link anchor).
    pub reopen_target_repr: String,
    /// Opaque, export-safe representation of the signer / source state.
    pub signer_source_state_repr: String,
    /// The delivery profile of this event.
    pub delivery_profile: M5AdvisoryDeliveryProfile,
    /// The mirror / distribution freshness of this event.
    pub mirror_freshness: M5AdvisoryFreshnessState,
    /// The action state this event carries.
    pub action_state: M5AdvisoryActionState,
    /// The primary required action.
    pub primary_action: M5AdvisoryRequiredAction,
    /// The local-continuity claim this event makes.
    pub continuity_claim: M5AdvisoryContinuityClaim,
}

impl M5NotificationHandoffResolutionInput {
    /// True when any representation carries forbidden material.
    fn carries_forbidden_material(&self) -> bool {
        repr_is_forbidden(&self.advisory_id)
            || repr_is_forbidden(&self.affected_scope_repr)
            || repr_is_forbidden(&self.current_status_repr)
            || repr_is_forbidden(&self.reopen_target_repr)
            || repr_is_forbidden(&self.signer_source_state_repr)
    }

    /// The privacy-safe native-notification behaviors this event carries. The OS payload
    /// always carries a compact summary with no sensitive body, click-through to the
    /// in-product advisory, and in-app dismissal sync; a non-emergency event respects
    /// quiet hours while an emergency-grade event bypasses them.
    fn notification_behaviors(&self) -> Vec<M5AdvisoryNotificationBehavior> {
        let mut behaviors = vec![
            M5AdvisoryNotificationBehavior::OsNotificationSummary,
            M5AdvisoryNotificationBehavior::ClickThroughToAdvisory,
            M5AdvisoryNotificationBehavior::NoSensitiveBodyInPayload,
            M5AdvisoryNotificationBehavior::DismissalSyncsToInApp,
        ];
        if severity_is_emergency_grade(self.severity) {
            behaviors.push(M5AdvisoryNotificationBehavior::EmergencyBypassesQuietHours);
        } else {
            behaviors.push(M5AdvisoryNotificationBehavior::RespectsQuietHours);
        }
        behaviors.sort();
        behaviors
    }
}

/// One channel projection of a resolved notification handoff. Every projection carries the
/// same core truth — advisory id, severity, affected scope, delivery posture, and reopen
/// surface — so the channels stay in parity; only the channel-scoped headline framing
/// differs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedNotificationChannelProjection {
    /// The channel this projection renders on.
    pub channel: M5NotificationChannel,
    /// The channel-scoped headline (built from the shared advisory truth).
    pub headline: String,
    /// The copy-safe advisory id (identical across channels).
    pub advisory_id: String,
    /// The severity (identical across channels).
    pub severity: M5AdvisorySeverityClass,
    /// The affected scope (identical across channels).
    pub affected_scope_repr: String,
    /// The delivery posture (identical across channels).
    pub delivery_posture: M5NotificationDeliveryPosture,
    /// The reopen surface (identical across channels).
    pub reopen_surface: M5NotificationReopenSurface,
}

/// One export column of the copy-safe notification summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationExportColumn {
    /// The export field.
    pub field: M5AdvisoryExportField,
    /// The export-safe value.
    pub value: String,
}

/// The copy-safe, export-safe summary of a resolved notification handoff, for support and
/// Help/About flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationExportSummary {
    /// The copy-safe advisory id.
    pub advisory_id: String,
    /// The mandatory export columns, in [`MANDATORY_EXPORT_FIELDS`] order.
    pub columns: Vec<M5NotificationExportColumn>,
}

/// The resolved notification / activity handoff for one advisory event on one delivery
/// lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedNotificationHandoff {
    /// The notification-delivery lane this handoff routes through.
    pub delivery_lane: M5NotificationDeliveryLane,
    /// The copy-safe Aureline advisory id.
    pub advisory_id: String,
    /// The advisory's severity.
    pub severity: M5AdvisorySeverityClass,
    /// The advisory / revocation event kind.
    pub event_kind: M5NotificationEventKind,
    /// The opaque affected-scope representation.
    pub affected_scope_repr: String,
    /// The opaque current-status representation.
    pub current_status_repr: String,
    /// The derived delivery posture.
    pub delivery_posture: M5NotificationDeliveryPosture,
    /// The authoritative surface this handoff reopens onto.
    pub reopen_surface: M5NotificationReopenSurface,
    /// The opaque reopen-target representation.
    pub reopen_target_repr: String,
    /// The opaque signer / source-state representation.
    pub signer_source_state_repr: String,
    /// The delivery profile of this event.
    pub delivery_profile: M5AdvisoryDeliveryProfile,
    /// The mirror / distribution freshness of this event.
    pub mirror_freshness: M5AdvisoryFreshnessState,
    /// The action state this event carries.
    pub action_state: M5AdvisoryActionState,
    /// The primary required action.
    pub primary_action: M5AdvisoryRequiredAction,
    /// The local-continuity claim this event makes.
    pub continuity_claim: M5AdvisoryContinuityClaim,
    /// The privacy-safe native-notification behaviors attached to the handoff.
    pub notification_behaviors: Vec<M5AdvisoryNotificationBehavior>,
    /// True — the event lands durably in the activity center.
    pub remains_durable: bool,
    /// False — the event never collapses to a bare OS badge.
    pub collapses_to_badge_only: bool,
    /// False — the event never collapses to a transient toast that disappears.
    pub collapses_to_toast_only: bool,
    /// False — the event never collapses to an external website link only.
    pub collapses_to_website_only: bool,
    /// True — this posture delivers a live native OS notification.
    pub delivers_native_os_notification: bool,
    /// True — the handoff reopens onto the authoritative in-product surface.
    pub reopens_to_authoritative_surface: bool,
    /// False — the handoff is never a dead-end that lands nowhere.
    pub is_dead_end: bool,
    /// True — the native notification and the activity row share one advisory vocabulary.
    pub shares_advisory_vocabulary: bool,
    /// True — the native-notification payload carries no sensitive body.
    pub payload_is_privacy_safe: bool,
    /// True — the primitive always keeps the handoff visible.
    pub remains_visible: bool,
    /// The same advisory truth projected into every channel.
    pub channel_projections: Vec<M5ResolvedNotificationChannelProjection>,
    /// The copy-safe, export-safe summary.
    pub export_summary: M5NotificationExportSummary,
}

/// Errors returned by [`resolve_notification_handoff`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5NotificationHandoffResolutionError {
    /// The advisory id was empty.
    EmptyAdvisoryId,
    /// The affected-scope representation was empty.
    EmptyAffectedScope,
    /// The current-status representation was empty.
    EmptyCurrentStatus,
    /// The reopen-target representation was empty.
    EmptyReopenTarget,
    /// The signer / source-state representation was empty.
    EmptySignerSourceState,
    /// A representation carried forbidden material.
    ForbiddenMaterial,
}

impl M5NotificationHandoffResolutionError {
    /// Stable token for tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyAdvisoryId => "empty_advisory_id",
            Self::EmptyAffectedScope => "empty_affected_scope",
            Self::EmptyCurrentStatus => "empty_current_status",
            Self::EmptyReopenTarget => "empty_reopen_target",
            Self::EmptySignerSourceState => "empty_signer_source_state",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5NotificationHandoffResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "notification-handoff resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5NotificationHandoffResolutionError {}

/// Resolves one advisory / revocation event into one notification / activity handoff.
///
/// The resolver derives the delivery posture from the delivery lane and severity (so a
/// suppressed OS notification still lands durably in the activity center instead of
/// collapsing to a badge, and an emergency severity bypasses quiet hours), keeps the
/// reopen target pointed at the authoritative affected-install or disclosure surface
/// (never a dead-end link), derives the privacy-safe native-notification behaviors, keeps
/// the event durable and visible, projects the same advisory truth into every channel, and
/// emits a copy-safe, export-safe summary. It never lets an event collapse to a badge-only,
/// toast-only, or website-only state and never splits the notification and activity-row
/// vocabulary.
pub fn resolve_notification_handoff(
    input: &M5NotificationHandoffResolutionInput,
) -> Result<M5ResolvedNotificationHandoff, M5NotificationHandoffResolutionError> {
    if input.advisory_id.trim().is_empty() {
        return Err(M5NotificationHandoffResolutionError::EmptyAdvisoryId);
    }
    if input.affected_scope_repr.trim().is_empty() {
        return Err(M5NotificationHandoffResolutionError::EmptyAffectedScope);
    }
    if input.current_status_repr.trim().is_empty() {
        return Err(M5NotificationHandoffResolutionError::EmptyCurrentStatus);
    }
    if input.reopen_target_repr.trim().is_empty() {
        return Err(M5NotificationHandoffResolutionError::EmptyReopenTarget);
    }
    if input.signer_source_state_repr.trim().is_empty() {
        return Err(M5NotificationHandoffResolutionError::EmptySignerSourceState);
    }
    if input.carries_forbidden_material() {
        return Err(M5NotificationHandoffResolutionError::ForbiddenMaterial);
    }

    let delivery_posture = input.delivery_lane.delivery_posture(input.severity);
    let notification_behaviors = input.notification_behaviors();

    let channel_projections = M5NotificationChannel::ALL
        .iter()
        .map(|channel| M5ResolvedNotificationChannelProjection {
            channel: *channel,
            headline: render_channel_headline(*channel, input, delivery_posture),
            advisory_id: input.advisory_id.clone(),
            severity: input.severity,
            affected_scope_repr: input.affected_scope_repr.clone(),
            delivery_posture,
            reopen_surface: input.authoritative_surface,
        })
        .collect();

    let export_summary = build_export_summary(input);

    Ok(M5ResolvedNotificationHandoff {
        delivery_lane: input.delivery_lane,
        advisory_id: input.advisory_id.clone(),
        severity: input.severity,
        event_kind: input.event_kind,
        affected_scope_repr: input.affected_scope_repr.clone(),
        current_status_repr: input.current_status_repr.clone(),
        delivery_posture,
        reopen_surface: input.authoritative_surface,
        reopen_target_repr: input.reopen_target_repr.clone(),
        signer_source_state_repr: input.signer_source_state_repr.clone(),
        delivery_profile: input.delivery_profile,
        mirror_freshness: input.mirror_freshness,
        action_state: input.action_state,
        primary_action: input.primary_action,
        continuity_claim: input.continuity_claim,
        notification_behaviors,
        // The event always lands durably; it is never a transient badge or toast.
        remains_durable: true,
        collapses_to_badge_only: false,
        collapses_to_toast_only: false,
        collapses_to_website_only: false,
        delivers_native_os_notification: delivery_posture.delivers_native_os_notification(),
        // The handoff always reopens onto the authoritative in-product surface.
        reopens_to_authoritative_surface: true,
        is_dead_end: false,
        // The native notification and the activity row read from one advisory vocabulary.
        shares_advisory_vocabulary: true,
        // The OS payload carries no sensitive body.
        payload_is_privacy_safe: true,
        // The primitive structurally keeps the handoff visible.
        remains_visible: true,
        channel_projections,
        export_summary,
    })
}

/// Renders one channel-scoped headline from the shared advisory truth. Every channel
/// carries the same advisory id, severity, affected scope, delivery posture, and reopen
/// surface; only the channel prefix differs.
fn render_channel_headline(
    channel: M5NotificationChannel,
    input: &M5NotificationHandoffResolutionInput,
    delivery_posture: M5NotificationDeliveryPosture,
) -> String {
    format!(
        "[{}] {} · {} · {} · delivery: {} · reopen: {}",
        channel.as_str(),
        input.advisory_id,
        input.event_kind.as_str(),
        input.severity.as_str(),
        delivery_posture.as_str(),
        input.authoritative_surface.as_str(),
    )
}

/// Builds the copy-safe, export-safe summary from the shared advisory truth.
fn build_export_summary(
    input: &M5NotificationHandoffResolutionInput,
) -> M5NotificationExportSummary {
    let columns = MANDATORY_EXPORT_FIELDS
        .iter()
        .map(|field| M5NotificationExportColumn {
            field: *field,
            value: export_value(*field, input),
        })
        .collect();
    M5NotificationExportSummary {
        advisory_id: input.advisory_id.clone(),
        columns,
    }
}

/// Resolves the export-safe value for one export field.
fn export_value(
    field: M5AdvisoryExportField,
    input: &M5NotificationHandoffResolutionInput,
) -> String {
    match field {
        M5AdvisoryExportField::AdvisoryId => input.advisory_id.clone(),
        M5AdvisoryExportField::Severity => input.severity.as_str().to_owned(),
        M5AdvisoryExportField::ActionState => input.action_state.as_str().to_owned(),
        M5AdvisoryExportField::AffectedSurface => input.affected_scope_repr.clone(),
        M5AdvisoryExportField::MitigationState => input.current_status_repr.clone(),
        M5AdvisoryExportField::ContinuityNote => input.continuity_claim.as_str().to_owned(),
        // Only the mandatory-export fields are projected into the summary; any other
        // field resolves to its stable token so the mapping stays total.
        other => other.as_str().to_owned(),
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs advisory truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationHandoffResolutionCase {
    /// The resolver input.
    pub input: M5NotificationHandoffResolutionInput,
    /// The resolved handoff. Must equal `resolve_notification_handoff(&input)`.
    pub resolved: M5ResolvedNotificationHandoff,
}

impl M5NotificationHandoffResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5NotificationHandoffResolutionInput) -> Self {
        let resolved = resolve_notification_handoff(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_notification_handoff(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one notification-delivery lane bound to the shared
/// handoff anatomy, severity vocabulary, channels, notification behaviors, event kinds,
/// export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationDeliveryRow {
    /// Notification-delivery lane.
    pub delivery_lane: M5NotificationDeliveryLane,
    /// Qualification class earned by this lane.
    pub qualification: M5AdvisoryQualificationClass,
    /// Owner role accountable for keeping this lane governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this row attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this row must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this row keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Anatomy parts this row renders inline (must include the mandatory parts).
    pub anatomy_parts: Vec<M5NotificationHandoffAnatomyPart>,
    /// Severity classes this row can show.
    pub severity_classes: Vec<M5AdvisorySeverityClass>,
    /// Channels this row projects into (must include every channel — parity).
    pub channels: Vec<M5NotificationChannel>,
    /// Action states this row projects.
    pub action_states: Vec<M5AdvisoryActionState>,
    /// Required actions this row can reference.
    pub required_actions: Vec<M5AdvisoryRequiredAction>,
    /// Local-continuity claims this row makes.
    pub continuity_claims: Vec<M5AdvisoryContinuityClaim>,
    /// Delivery profiles this row can carry.
    pub delivery_profiles: Vec<M5AdvisoryDeliveryProfile>,
    /// Mirror-freshness states this row can carry.
    pub freshness_states: Vec<M5AdvisoryFreshnessState>,
    /// Native-notification behaviors this row carries (must include every behavior).
    pub notification_behaviors: Vec<M5AdvisoryNotificationBehavior>,
    /// Event kinds this row can route (must include every event kind).
    pub event_kinds: Vec<M5NotificationEventKind>,
    /// Focus behaviors this row supports.
    pub focus_behaviors: Vec<M5NotificationFocusBehavior>,
    /// Export fields this row carries (must include the mandatory truth fields).
    pub export_fields: Vec<M5AdvisoryExportField>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5AdvisoryAccessibilityRoute>,
    /// Shell subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5AdvisoryDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this lane.
    pub example_handoffs: Vec<M5NotificationHandoffResolutionCase>,
    /// Hard invariant: this row never collapses an event to a badge-only, toast-only, or
    /// website-only state. MUST be `false`.
    pub collapses_to_badge_toast_or_website_only: bool,
    /// Hard invariant: this row never hides handoff truth behind a detail drawer. MUST be
    /// `false`.
    pub hides_field_behind_detail_drawer: bool,
    /// Hard invariant: this row never drops an event out of the durable activity history.
    /// MUST be `false`.
    pub drops_event_from_durable_history: bool,
    /// Hard invariant: this row never splits the native-notification and activity-row
    /// vocabulary. MUST be `false`.
    pub splits_notification_and_activity_vocabulary: bool,
    /// Hard invariant: this row never drops the copy-safe id or export summary. MUST be
    /// `false`.
    pub drops_copy_safe_id_or_export: bool,
}

impl M5NotificationDeliveryRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5NotificationHandoffAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5NotificationHandoffAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every channel (all four projected in parity).
    fn declares_all_channels(&self) -> bool {
        let present: BTreeSet<M5NotificationChannel> = self.channels.iter().copied().collect();
        M5NotificationChannel::ALL
            .iter()
            .all(|channel| present.contains(channel))
    }

    /// True when the row declares every native-notification behavior.
    fn declares_all_notification_behaviors(&self) -> bool {
        let present: BTreeSet<M5AdvisoryNotificationBehavior> =
            self.notification_behaviors.iter().copied().collect();
        M5AdvisoryNotificationBehavior::ALL
            .iter()
            .all(|behavior| present.contains(behavior))
    }

    /// True when the row declares every event kind.
    fn declares_all_event_kinds(&self) -> bool {
        let present: BTreeSet<M5NotificationEventKind> = self.event_kinds.iter().copied().collect();
        M5NotificationEventKind::ALL
            .iter()
            .all(|kind| present.contains(kind))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5AdvisoryExportField> = self.export_fields.iter().copied().collect();
        MANDATORY_EXPORT_FIELDS
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_to_badge_toast_or_website_only
            && !self.hides_field_behind_detail_drawer
            && !self.drops_event_from_durable_history
            && !self.splits_notification_and_activity_vocabulary
            && !self.drops_copy_safe_id_or_export
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationHandoffVocabularySet {
    /// Notification-delivery-lane tokens.
    pub delivery_lanes: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Severity-class tokens (reused from the frozen matrix).
    pub severity_classes: Vec<String>,
    /// Action-state tokens (reused from the frozen matrix).
    pub action_states: Vec<String>,
    /// Required-action tokens (reused from the frozen matrix).
    pub required_actions: Vec<String>,
    /// Continuity-claim tokens (reused from the frozen matrix).
    pub continuity_claims: Vec<String>,
    /// Delivery-profile tokens (reused from the frozen matrix).
    pub delivery_profiles: Vec<String>,
    /// Mirror-freshness-state tokens (reused from the frozen matrix).
    pub freshness_states: Vec<String>,
    /// Native-notification-behavior tokens (reused from the frozen matrix).
    pub notification_behaviors: Vec<String>,
    /// Event-kind tokens.
    pub event_kinds: Vec<String>,
    /// Delivery-posture tokens.
    pub delivery_postures: Vec<String>,
    /// Reopen-surface tokens.
    pub reopen_surfaces: Vec<String>,
    /// Channel tokens.
    pub channels: Vec<String>,
    /// Focus-behavior tokens.
    pub focus_behaviors: Vec<String>,
    /// Export-field tokens (reused from the frozen matrix).
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5NotificationHandoffVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            delivery_lanes: tokens(&M5NotificationDeliveryLane::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5NotificationHandoffAnatomyPart::ALL, |v| v.as_str()),
            severity_classes: tokens(&M5AdvisorySeverityClass::ALL, |v| v.as_str()),
            action_states: tokens(&M5AdvisoryActionState::ALL, |v| v.as_str()),
            required_actions: tokens(&M5AdvisoryRequiredAction::ALL, |v| v.as_str()),
            continuity_claims: tokens(&M5AdvisoryContinuityClaim::ALL, |v| v.as_str()),
            delivery_profiles: tokens(&M5AdvisoryDeliveryProfile::ALL, |v| v.as_str()),
            freshness_states: tokens(&M5AdvisoryFreshnessState::ALL, |v| v.as_str()),
            notification_behaviors: tokens(&M5AdvisoryNotificationBehavior::ALL, |v| v.as_str()),
            event_kinds: tokens(&M5NotificationEventKind::ALL, |v| v.as_str()),
            delivery_postures: tokens(&M5NotificationDeliveryPosture::ALL, |v| v.as_str()),
            reopen_surfaces: tokens(&M5NotificationReopenSurface::ALL, |v| v.as_str()),
            channels: tokens(&M5NotificationChannel::ALL, |v| v.as_str()),
            focus_behaviors: tokens(&M5NotificationFocusBehavior::ALL, |v| v.as_str()),
            export_fields: tokens(&M5AdvisoryExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5AdvisoryAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5NotificationHandoffGovernanceReview {
    /// One notification / activity handoff model is reused across every delivery lane.
    pub one_handoff_model_across_delivery_lanes: bool,
    /// The event identity, severity, and affected scope are visible without a detail
    /// drawer.
    pub event_identity_severity_scope_visible_without_drawer: bool,
    /// The native notification and the activity row share one advisory vocabulary.
    pub native_notification_and_activity_row_share_vocabulary: bool,
    /// An event never collapses to a badge-only, toast-only, or website-only state.
    pub never_collapses_to_badge_toast_or_website_only: bool,
    /// A suppressed OS notification still lands durably in the activity center.
    pub suppressed_os_notification_still_lands_durably: bool,
    /// An emergency-grade severity bypasses quiet hours / do-not-disturb.
    pub emergency_severity_bypasses_quiet_hours: bool,
    /// The reopen action lands on the authoritative affected-install / disclosure surface.
    pub reopen_lands_on_authoritative_surface: bool,
    /// The handoff never reopens onto a dead-end.
    pub no_dead_end_reopen: bool,
    /// The native-notification payload carries no sensitive body.
    pub privacy_safe_no_sensitive_body_in_payload: bool,
    /// The export summary reconstructs advisory truth for support and Help/About.
    pub export_summary_reconstructs_advisory_truth: bool,
    /// Every row is bound to a canonical shell zone.
    pub every_row_bound_to_shell_zone: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 lanes cannot invent parallel notification vocabulary.
    pub later_lanes_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationHandoffConsumerProjection {
    /// The activity center renders the shared handoff.
    pub activity_center_renders_shared_handoff: bool,
    /// A native notification renders the shared handoff.
    pub native_notification_renders_shared_handoff: bool,
    /// Help/About renders the shared handoff.
    pub help_about_renders_shared_handoff: bool,
    /// The support bundle renders the shared handoff.
    pub support_bundle_renders_shared_handoff: bool,
    /// The resolver reads a single canonical notification vocabulary.
    pub resolver_reads_single_notification_vocabulary: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationHandoffProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the notification-handoff primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationHandoffReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting notification audit.
    pub notification_audit_ref: String,
    /// True when support / export parity is required for every lane.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every lane.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5NotificationHandoffPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5NotificationHandoffPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Notification-delivery rows.
    pub delivery_rows: Vec<M5NotificationDeliveryRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5NotificationHandoffVocabularySet,
    /// Governance-review block.
    pub governance_review: M5NotificationHandoffGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5NotificationHandoffConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5NotificationHandoffProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5NotificationHandoffReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 notification-activity-handoff-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationHandoffPacket {
    /// Record kind; must equal [`M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Notification-delivery rows.
    pub delivery_rows: Vec<M5NotificationDeliveryRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5NotificationHandoffVocabularySet,
    /// Governance-review block.
    pub governance_review: M5NotificationHandoffGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5NotificationHandoffConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5NotificationHandoffProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5NotificationHandoffReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5NotificationHandoffPacket {
    /// Builds an M5 notification-activity-handoff-primitive packet from stable-lane input.
    pub fn new(input: M5NotificationHandoffPacketInput) -> Self {
        Self {
            record_kind: M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            delivery_rows: input.delivery_rows,
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

    /// Validates the M5 notification-activity-handoff-primitive invariants.
    pub fn validate(&self) -> Vec<M5NotificationHandoffViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_RECORD_KIND {
            violations.push(M5NotificationHandoffViolation::WrongRecordKind);
        }
        if self.schema_version != M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5NotificationHandoffViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5NotificationHandoffViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_delivery_rows(self, &mut violations);
        validate_channel_parity(self, &mut violations);
        validate_durable_routing(self, &mut violations);
        validate_reopen_continuity(self, &mut violations);
        validate_event_kind_coverage(self, &mut violations);
        validate_severity_coverage(self, &mut violations);
        validate_delivery_posture_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 notification handoff packet serializes"),
        ) {
            violations.push(M5NotificationHandoffViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 notification handoff packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per notification-delivery lane.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "delivery_lane,qualification,owner,shell_zone_slot,severity_classes,channels,anatomy_parts,event_kinds,notification_behaviors,export_fields,accessibility_routes,example_count\n",
        );
        for row in &self.delivery_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.delivery_lane.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.severity_classes, |v| v.as_str()),
                join_tokens(&row.channels, |v| v.as_str()),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.event_kinds, |v| v.as_str()),
                join_tokens(&row.notification_behaviors, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                join_tokens(&row.accessibility_routes, |v| v.as_str()),
                row.example_handoffs.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .delivery_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Notification / Activity-Center Handoff Routing Primitive: Durable, Reopenable Advisory Routing and Help/Support Parity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Notification-delivery lanes: {} ({} stable)\n",
            self.delivery_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Anatomy parts: {}\n",
            self.vocabulary_set.anatomy_parts.join(", ")
        ));
        out.push_str(&format!(
            "- Severity classes: {}\n",
            self.vocabulary_set.severity_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Channels: {}\n",
            self.vocabulary_set.channels.join(", ")
        ));
        out.push_str(&format!(
            "- Event kinds: {}\n",
            self.vocabulary_set.event_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Delivery postures: {}\n",
            self.vocabulary_set.delivery_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Export fields: {}\n",
            self.vocabulary_set.export_fields.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Notification-delivery lanes\n\n");
        for row in &self.delivery_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.delivery_lane.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Worked handoffs: {}\n",
                row.example_handoffs.len()
            ));
            for case in &row.example_handoffs {
                out.push_str(&format!(
                    "    - `{}` — {} ({}), delivery `{}`, reopen `{}`\n",
                    case.resolved.advisory_id,
                    case.resolved.severity.as_str(),
                    case.resolved.event_kind.as_str(),
                    case.resolved.delivery_posture.as_str(),
                    case.resolved.reopen_surface.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 notification-activity-handoff-primitive
/// export.
#[derive(Debug)]
pub enum M5NotificationHandoffArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5NotificationHandoffViolation>),
}

impl fmt::Display for M5NotificationHandoffArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 notification handoff export parse failed: {error}"
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
                    "m5 notification handoff export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5NotificationHandoffArtifactError {}

/// Validation failures emitted by [`M5NotificationHandoffPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5NotificationHandoffViolation {
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
    /// A required notification-delivery lane is missing from the matrix.
    RequiredDeliveryLaneMissing,
    /// A notification-delivery row is incomplete.
    DeliveryRowIncomplete,
    /// A notification-delivery row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A notification-delivery row declares no severity classes.
    SeverityClassMissing,
    /// A notification-delivery row does not declare every channel (channel parity broken).
    ChannelParityMismatch,
    /// A notification-delivery row declares no action states.
    ActionStateMissing,
    /// A notification-delivery row declares no required actions.
    RequiredActionMissing,
    /// A notification-delivery row declares no continuity claims.
    ContinuityClaimMissing,
    /// A notification-delivery row declares no delivery profiles.
    DeliveryProfileMissing,
    /// A notification-delivery row declares no mirror-freshness states.
    FreshnessStateMissing,
    /// A notification-delivery row does not declare every notification behavior.
    NotificationBehaviorMissing,
    /// A notification-delivery row does not declare every event kind.
    EventKindMissing,
    /// A notification-delivery row declares no focus behaviors.
    FocusBehaviorMissing,
    /// A notification-delivery row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A notification-delivery row declares no accessibility routes (or misses keyboard
    /// focus).
    AccessibilityRouteMissing,
    /// A notification-delivery row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A notification-delivery row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A notification-delivery row declares no worked resolution cases.
    ExampleHandoffMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleHandoffDrift,
    /// A lane claiming Stable is missing required proof packet refs.
    StableLaneMissingProof,
    /// The worked resolutions do not prove the shared channel parity and copy-safe export
    /// behavior across every channel.
    ChannelParityUnproven,
    /// The worked resolutions do not prove an event stays durable without collapsing to a
    /// badge-only, toast-only, or website-only state.
    DurableRoutingUnproven,
    /// The worked resolutions do not prove a handoff reopens onto the authoritative
    /// affected-install / disclosure surface without a dead-end.
    ReopenContinuityUnproven,
    /// No worked resolution across the matrix exercises every event kind.
    EventKindCoverageUnproven,
    /// No worked resolution across the matrix exercises every severity class.
    SeverityCoverageUnproven,
    /// No worked resolution across the matrix exercises every delivery posture.
    DeliveryPostureCoverageUnproven,
    /// A notification-delivery row violates a hard invariant.
    DeliveryInvariantViolated,
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

impl M5NotificationHandoffViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredDeliveryLaneMissing => "required_delivery_lane_missing",
            Self::DeliveryRowIncomplete => "delivery_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::SeverityClassMissing => "severity_class_missing",
            Self::ChannelParityMismatch => "channel_parity_mismatch",
            Self::ActionStateMissing => "action_state_missing",
            Self::RequiredActionMissing => "required_action_missing",
            Self::ContinuityClaimMissing => "continuity_claim_missing",
            Self::DeliveryProfileMissing => "delivery_profile_missing",
            Self::FreshnessStateMissing => "freshness_state_missing",
            Self::NotificationBehaviorMissing => "notification_behavior_missing",
            Self::EventKindMissing => "event_kind_missing",
            Self::FocusBehaviorMissing => "focus_behavior_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleHandoffMissing => "example_handoff_missing",
            Self::ExampleHandoffDrift => "example_handoff_drift",
            Self::StableLaneMissingProof => "stable_lane_missing_proof",
            Self::ChannelParityUnproven => "channel_parity_unproven",
            Self::DurableRoutingUnproven => "durable_routing_unproven",
            Self::ReopenContinuityUnproven => "reopen_continuity_unproven",
            Self::EventKindCoverageUnproven => "event_kind_coverage_unproven",
            Self::SeverityCoverageUnproven => "severity_coverage_unproven",
            Self::DeliveryPostureCoverageUnproven => "delivery_posture_coverage_unproven",
            Self::DeliveryInvariantViolated => "delivery_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 notification-activity-handoff-primitive
/// export.
pub fn current_stable_m5_notification_activity_handoff_primitive_export(
) -> Result<M5NotificationHandoffPacket, M5NotificationHandoffArtifactError> {
    let packet: M5NotificationHandoffPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-notification-activity-handoff-proof/support_export.json"
    )))
    .map_err(M5NotificationHandoffArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5NotificationHandoffArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_NOTIFICATION_ACTIVITY_HANDOFF_SCHEMA_REF,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_DOC_REF,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_SHELL_ZONE_REF,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_COMPONENT_MATRIX_REF,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_IDENTITY_REF,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_OS_NOTIFICATION_DOC_REF,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_ATTENTION_ROUTING_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5NotificationHandoffViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5NotificationHandoffViolation::VocabularySetDrift);
    }
}

fn validate_delivery_rows(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    let present: BTreeSet<M5NotificationDeliveryLane> = packet
        .delivery_rows
        .iter()
        .map(|row| row.delivery_lane)
        .collect();
    for required in M5NotificationDeliveryLane::ALL {
        if !present.contains(&required) {
            violations.push(M5NotificationHandoffViolation::RequiredDeliveryLaneMissing);
            return;
        }
    }

    for row in &packet.delivery_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
        {
            violations.push(M5NotificationHandoffViolation::DeliveryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5NotificationHandoffViolation::MandatoryAnatomyMissing);
        }
        if row.severity_classes.is_empty() {
            violations.push(M5NotificationHandoffViolation::SeverityClassMissing);
        }
        if !row.declares_all_channels() {
            violations.push(M5NotificationHandoffViolation::ChannelParityMismatch);
        }
        if row.action_states.is_empty() {
            violations.push(M5NotificationHandoffViolation::ActionStateMissing);
        }
        if row.required_actions.is_empty() {
            violations.push(M5NotificationHandoffViolation::RequiredActionMissing);
        }
        if row.continuity_claims.is_empty() {
            violations.push(M5NotificationHandoffViolation::ContinuityClaimMissing);
        }
        if row.delivery_profiles.is_empty() {
            violations.push(M5NotificationHandoffViolation::DeliveryProfileMissing);
        }
        if row.freshness_states.is_empty() {
            violations.push(M5NotificationHandoffViolation::FreshnessStateMissing);
        }
        if !row.declares_all_notification_behaviors() {
            violations.push(M5NotificationHandoffViolation::NotificationBehaviorMissing);
        }
        if !row.declares_all_event_kinds() {
            violations.push(M5NotificationHandoffViolation::EventKindMissing);
        }
        if row.focus_behaviors.is_empty() {
            violations.push(M5NotificationHandoffViolation::FocusBehaviorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5NotificationHandoffViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5AdvisoryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5NotificationHandoffViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5NotificationHandoffViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5NotificationHandoffViolation::DowngradeTriggersMissing);
        }
        if row.example_handoffs.is_empty() {
            violations.push(M5NotificationHandoffViolation::ExampleHandoffMissing);
        }
        if row
            .example_handoffs
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5NotificationHandoffViolation::ExampleHandoffDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5NotificationHandoffViolation::StableLaneMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5NotificationHandoffViolation::DeliveryInvariantViolated);
        }
    }
}

/// Every worked resolution must project all four channels with identical core truth, and
/// at least one worked resolution must carry a full copy-safe export whose Help/About and
/// support-bundle channels share the same advisory id and severity as the native
/// notification and the activity row — the acceptance-criterion proof (AC2 / AC3) that
/// native notifications, activity rows, Help/About, and support share one advisory
/// vocabulary and one export.
fn validate_channel_parity(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    let cases: Vec<&M5ResolvedNotificationHandoff> = packet
        .delivery_rows
        .iter()
        .flat_map(|row| row.example_handoffs.iter())
        .map(|case| &case.resolved)
        .collect();
    if cases.is_empty() {
        violations.push(M5NotificationHandoffViolation::ChannelParityUnproven);
        return;
    }
    let all_channels_projected = cases.iter().all(|handoff| {
        let present: BTreeSet<M5NotificationChannel> = handoff
            .channel_projections
            .iter()
            .map(|projection| projection.channel)
            .collect();
        M5NotificationChannel::ALL
            .iter()
            .all(|channel| present.contains(channel))
            && handoff.shares_advisory_vocabulary
            && handoff.channel_projections.iter().all(|projection| {
                projection.advisory_id == handoff.advisory_id
                    && projection.severity == handoff.severity
                    && projection.affected_scope_repr == handoff.affected_scope_repr
                    && projection.delivery_posture == handoff.delivery_posture
                    && projection.reopen_surface == handoff.reopen_surface
            })
    });
    let export_proven = cases.iter().any(|handoff| {
        handoff.export_summary.columns.len() >= MANDATORY_EXPORT_FIELDS.len()
            && handoff
                .export_summary
                .columns
                .iter()
                .all(|column| !column.value.trim().is_empty())
            && !repr_is_forbidden(&handoff.advisory_id)
            && handoff.export_summary.advisory_id == handoff.advisory_id
    });
    if !all_channels_projected || !export_proven {
        violations.push(M5NotificationHandoffViolation::ChannelParityUnproven);
    }
}

/// Every worked resolution must stay durable without collapsing to a badge-only,
/// toast-only, or website-only state, and at least one worked resolution must exercise a
/// suppressed OS-notification lane that still lands durably in the activity center — the
/// acceptance-criterion proof (AC1) that an event never collapses to a badge / toast /
/// website-only state.
fn validate_durable_routing(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    let cases: Vec<&M5ResolvedNotificationHandoff> = packet
        .delivery_rows
        .iter()
        .flat_map(|row| row.example_handoffs.iter())
        .map(|case| &case.resolved)
        .collect();
    if cases.is_empty() {
        violations.push(M5NotificationHandoffViolation::DurableRoutingUnproven);
        return;
    }
    let all_durable = cases.iter().all(|handoff| {
        handoff.remains_durable
            && !handoff.collapses_to_badge_only
            && !handoff.collapses_to_toast_only
            && !handoff.collapses_to_website_only
            && handoff.remains_visible
    });
    let suppressed_stays_durable = cases.iter().any(|handoff| {
        handoff.delivery_lane.suppresses_os_notification()
            && handoff.delivery_posture == M5NotificationDeliveryPosture::ActivityCenterDurableOnly
            && handoff.remains_durable
            && !handoff.collapses_to_badge_only
    });
    if !all_durable || !suppressed_stays_durable {
        violations.push(M5NotificationHandoffViolation::DurableRoutingUnproven);
    }
}

/// Every worked resolution must reopen onto its authoritative surface without a dead-end,
/// and the worked resolutions together must reopen onto both the affected-install panel and
/// the disclosure block — the acceptance-criterion proof (AC1) that a notification or
/// activity row lands on the authoritative affected-install or disclosure surface.
fn validate_reopen_continuity(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    let cases: Vec<&M5ResolvedNotificationHandoff> = packet
        .delivery_rows
        .iter()
        .flat_map(|row| row.example_handoffs.iter())
        .map(|case| &case.resolved)
        .collect();
    if cases.is_empty() {
        violations.push(M5NotificationHandoffViolation::ReopenContinuityUnproven);
        return;
    }
    let all_reopen = cases.iter().all(|handoff| {
        handoff.reopens_to_authoritative_surface
            && !handoff.is_dead_end
            && !handoff.reopen_target_repr.trim().is_empty()
    });
    let surfaces: BTreeSet<M5NotificationReopenSurface> =
        cases.iter().map(|handoff| handoff.reopen_surface).collect();
    let covers_authoritative_surfaces = surfaces
        .contains(&M5NotificationReopenSurface::AffectedInstallPanel)
        && surfaces.contains(&M5NotificationReopenSurface::DisclosureBlock);
    if !all_reopen || !covers_authoritative_surfaces {
        violations.push(M5NotificationHandoffViolation::ReopenContinuityUnproven);
    }
}

/// Every event kind must be exercised by some worked resolution so the handoff is proven to
/// route every advisory / revocation event kind.
fn validate_event_kind_coverage(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    let present: BTreeSet<M5NotificationEventKind> = packet
        .delivery_rows
        .iter()
        .flat_map(|row| row.example_handoffs.iter())
        .map(|case| case.resolved.event_kind)
        .collect();
    if !M5NotificationEventKind::ALL
        .iter()
        .all(|kind| present.contains(kind))
    {
        violations.push(M5NotificationHandoffViolation::EventKindCoverageUnproven);
    }
}

/// Every severity class must be exercised by some worked resolution so the handoff is
/// proven to route every severity.
fn validate_severity_coverage(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    let present: BTreeSet<M5AdvisorySeverityClass> = packet
        .delivery_rows
        .iter()
        .flat_map(|row| row.example_handoffs.iter())
        .map(|case| case.resolved.severity)
        .collect();
    if !M5AdvisorySeverityClass::ALL
        .iter()
        .all(|severity| present.contains(severity))
    {
        violations.push(M5NotificationHandoffViolation::SeverityCoverageUnproven);
    }
}

/// Every delivery posture must be exercised by some worked resolution so the handoff is
/// proven to route the live, suppressed-durable, emergency-bypass, and deferred-durable
/// postures.
fn validate_delivery_posture_coverage(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    let present: BTreeSet<M5NotificationDeliveryPosture> = packet
        .delivery_rows
        .iter()
        .flat_map(|row| row.example_handoffs.iter())
        .map(|case| case.resolved.delivery_posture)
        .collect();
    if !M5NotificationDeliveryPosture::ALL
        .iter()
        .all(|posture| present.contains(posture))
    {
        violations.push(M5NotificationHandoffViolation::DeliveryPostureCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_handoff_model_across_delivery_lanes,
        review.event_identity_severity_scope_visible_without_drawer,
        review.native_notification_and_activity_row_share_vocabulary,
        review.never_collapses_to_badge_toast_or_website_only,
        review.suppressed_os_notification_still_lands_durably,
        review.emergency_severity_bypasses_quiet_hours,
        review.reopen_lands_on_authoritative_surface,
        review.no_dead_end_reopen,
        review.privacy_safe_no_sensitive_body_in_payload,
        review.export_summary_reconstructs_advisory_truth,
        review.every_row_bound_to_shell_zone,
        review.every_row_declares_accessibility_route,
        review.later_lanes_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5NotificationHandoffViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.activity_center_renders_shared_handoff,
        projection.native_notification_renders_shared_handoff,
        projection.help_about_renders_shared_handoff,
        projection.support_bundle_renders_shared_handoff,
        projection.resolver_reads_single_notification_vocabulary,
    ] {
        if !ok {
            violations.push(M5NotificationHandoffViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5NotificationHandoffViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5NotificationHandoffPacket,
    violations: &mut Vec<M5NotificationHandoffViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.notification_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5NotificationHandoffViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
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
fn repr_is_forbidden(value: &str) -> bool {
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
        serde_json::Value::String(s) => repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
