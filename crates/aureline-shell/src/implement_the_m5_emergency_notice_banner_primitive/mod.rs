//! One reusable M5 emergency-notice banner primitive: reason class, affected
//! capability or component, blast radius, local-work continuity note, deadline /
//! urgency, and primary / recovery actions rendered with the same model whenever a
//! kill switch, trust-root rotation, channel freeze, forced-disable action, or signed
//! emergency bundle changes what is safe to do next.
//!
//! Aureline's frozen advisory-component matrix
//! ([`crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix`])
//! names the emergency notice as a governed component family and freezes the
//! controlled severity classes, action states, required actions, dismissal states,
//! continuity claims, export fields, and accessibility routes an emergency component
//! may use. This module *implements* that emergency-notice contract as one reusable
//! banner primitive so a kill switch, a trust-root rotation, a channel freeze, a
//! forced-disable action, or a signed emergency bundle reads the same everywhere it
//! surfaces — instead of collapsing into a generic red banner that implies broader
//! loss of local work than the evidence supports, or into one generic close button
//! that ignores the event class.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_emergency_banner`] — that takes one emergency affecting
//!    one reason-class lane (its copy-safe notice id, severity, affected capability,
//!    blast radius, local-work state, deadline, recovery path, signer / source state,
//!    action state, primary and recovery actions, local-continuity claim, and
//!    dismissal policy) and produces one [`M5ResolvedEmergencyBanner`] that derives the
//!    local-continuity posture from the local-work state, never implies data loss or
//!    unsafe local work unless the event actually proves it, derives the dismissal
//!    state and the allowed acknowledge / snooze / dismiss actions from the event's
//!    dismissal policy, keeps the banner visible, projects the same emergency truth
//!    into every claimed channel, and emits a copy-safe, export-safe summary. The
//!    resolver never hides the reason, scope, continuity, or next action behind a
//!    detail drawer and never drops the copy-safe notice id.
//! 2. A parity matrix — [`M5EmergencyBannerPrimitivePacket`] — that binds one row per
//!    claimed reason-class lane to the shared banner anatomy, the same severity
//!    vocabulary, the same channels, the same dismissal rules, the same export fields,
//!    and the same accessibility routes, so update, extension-host, native
//!    notification, and support surfaces render the same emergency banner from one
//!    shared model.
//!
//! The severity classes ([`M5AdvisorySeverityClass`]), action states
//! ([`M5AdvisoryActionState`]), required actions ([`M5AdvisoryRequiredAction`]),
//! dismissal states ([`M5AdvisoryDismissalState`]), continuity claims
//! ([`M5AdvisoryContinuityClaim`]), export fields ([`M5AdvisoryExportField`]),
//! accessibility routes ([`M5AdvisoryAccessibilityRoute`]), qualification classes
//! ([`M5AdvisoryQualificationClass`]), and downgrade triggers
//! ([`M5AdvisoryDowngradeTrigger`]) are reused verbatim from the frozen advisory
//! matrix; the shell topology — zones, responsive classes, window classes, and
//! consumer surfaces — is reused from the frozen shell-zone matrix. This module mints
//! new vocabulary only for what the frozen matrix left implicit about the banner
//! itself: its reason-class lanes, its banner anatomy, its channels, its focus
//! behaviors, and its dismissal actions. The local-work state, the derived continuity
//! posture, and the dismissal policy are resolver-side vocabularies, kept out of the
//! frozen set. No M5 surface invents a second emergency grammar or a parallel severity
//! vocabulary.
//!
//! Raw reporter identities, raw exploit payloads, raw signatures, raw hostnames, raw
//! paths, private registry URLs, credentials, and raw evidence bodies stay outside the
//! support boundary; opaque, export-safe reprs are the only material carried.
//!
//! The boundary schema is
//! [`schemas/security/m5-emergency-notice-banner.schema.json`](../../../../schemas/security/m5-emergency-notice-banner.schema.json)
//! and the contract doc is
//! [`docs/security/m5_emergency_notice_banner_primitive_contract.md`](../../../../docs/security/m5_emergency_notice_banner_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/security/m5-emergency-notice-banner-primitive/`](../../../../fixtures/security/m5-emergency-notice-banner-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_emergency_notice_banner_primitive_forced_disable_beta_narrowed,
    seeded_m5_emergency_notice_banner_primitive_packet,
    seeded_m5_emergency_notice_banner_primitive_signed_emergency_bundle_preview_narrowed,
    M5_EMERGENCY_BANNER_PRIMITIVE_PACKET_ID,
};

// The severity classes, action states, required actions, dismissal states, continuity
// claims, export fields, accessibility routes, qualification classes, and downgrade
// triggers are frozen once, in the advisory-component matrix. This primitive reuses
// them verbatim so it never invents a parallel severity vocabulary or a second
// emergency grammar.
pub use crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix::{
    M5AdvisoryAccessibilityRoute, M5AdvisoryActionState, M5AdvisoryContinuityClaim,
    M5AdvisoryDismissalState, M5AdvisoryDowngradeTrigger, M5AdvisoryExportField,
    M5AdvisoryQualificationClass, M5AdvisoryRequiredAction, M5AdvisorySeverityClass,
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

/// Stable record-kind tag carried by [`M5EmergencyBannerPrimitivePacket`].
pub const M5_EMERGENCY_BANNER_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_emergency_notice_banner_reason_class_affected_capability_continuity_deadline_and_dismissal_parity_primitive";

/// Schema version for M5 emergency-notice-banner-primitive records.
pub const M5_EMERGENCY_BANNER_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the emergency-notice-banner-primitive boundary schema.
pub const M5_EMERGENCY_BANNER_SCHEMA_REF: &str =
    "schemas/security/m5-emergency-notice-banner.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EMERGENCY_BANNER_DOC_REF: &str =
    "docs/security/m5_emergency_notice_banner_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds against.
pub const M5_EMERGENCY_BANNER_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen advisory-component matrix this primitive narrows
/// from.
pub const M5_EMERGENCY_BANNER_COMPONENT_MATRIX_REF: &str =
    "schemas/security/m5-advisory-component-matrix.schema.json";

/// Repo-relative path of the frozen emergency-action object model this primitive
/// aligns its reason, notice, and continuity vocabulary to.
pub const M5_EMERGENCY_BANNER_EMERGENCY_ACTION_REF: &str =
    "docs/security/emergency_action_model.md";

/// Repo-relative path of the frozen emergency-disable-bundle contract this primitive
/// aligns its kill-switch, channel-freeze, and forced-disable scope vocabulary to.
pub const M5_EMERGENCY_BANNER_DISABLE_BUNDLE_REF: &str =
    "schemas/security/emergency_disable_bundle.schema.json";

/// Repo-relative path of the frozen local-continuity card this primitive aligns its
/// local-work / continuity-posture vocabulary to.
pub const M5_EMERGENCY_BANNER_LOCAL_CONTINUITY_REF: &str =
    "schemas/security/local_continuity_card.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_EMERGENCY_BANNER_FIXTURE_DIR: &str =
    "fixtures/security/m5-emergency-notice-banner-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EMERGENCY_BANNER_ARTIFACT_REF: &str =
    "artifacts/release/m5-emergency-notice-banner-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_EMERGENCY_BANNER_CSV_REF: &str =
    "artifacts/release/m5-emergency-notice-banner-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_EMERGENCY_BANNER_REPORT_REF: &str =
    "artifacts/security/m5-emergency-notice-banner-primitive.md";

/// The export fields every emergency banner's support / admin summary must carry so a
/// support bundle reconstructs the emergency without a screenshot and never silently
/// drops a truth-bearing column.
pub const MANDATORY_EXPORT_FIELDS: [M5AdvisoryExportField; 6] = [
    M5AdvisoryExportField::AdvisoryId,
    M5AdvisoryExportField::Severity,
    M5AdvisoryExportField::ActionState,
    M5AdvisoryExportField::AffectedSurface,
    M5AdvisoryExportField::MitigationState,
    M5AdvisoryExportField::ContinuityNote,
];

/// One claimed reason-class lane an emergency banner can render. These are the events
/// the goal names — anywhere a kill switch, trust-root rotation, channel freeze,
/// forced-disable action, or signed emergency bundle changes what is safe to do next.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmergencyReasonClass {
    /// A capability kill switch fired.
    CapabilityKillSwitch,
    /// The trust root rotated.
    TrustRootRotation,
    /// A distribution / update channel was frozen.
    ChannelFreeze,
    /// A capability or component was forcibly disabled.
    ForcedDisable,
    /// A signed emergency bundle changed what is safe to do next.
    SignedEmergencyBundle,
}

impl M5EmergencyReasonClass {
    /// Every reason-class lane, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CapabilityKillSwitch,
        Self::TrustRootRotation,
        Self::ChannelFreeze,
        Self::ForcedDisable,
        Self::SignedEmergencyBundle,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapabilityKillSwitch => "capability_kill_switch",
            Self::TrustRootRotation => "trust_root_rotation",
            Self::ChannelFreeze => "channel_freeze",
            Self::ForcedDisable => "forced_disable",
            Self::SignedEmergencyBundle => "signed_emergency_bundle",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CapabilityKillSwitch => "Capability Kill Switch",
            Self::TrustRootRotation => "Trust-Root Rotation",
            Self::ChannelFreeze => "Channel Freeze",
            Self::ForcedDisable => "Forced Disable",
            Self::SignedEmergencyBundle => "Signed Emergency Bundle",
        }
    }
}

/// One anatomy part the shared emergency banner surfaces. Every part is mandatory: the
/// whole point of the primitive is that the reason, affected capability, blast radius,
/// local-work continuity, deadline, and the next / recovery actions are visible inline
/// without opening a secondary detail drawer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmergencyBannerAnatomyPart {
    /// The reason class that raised the emergency.
    ReasonClass,
    /// The affected capability or component.
    AffectedCapability,
    /// The explicit blast radius / affected scope.
    BlastRadius,
    /// The local-work continuity note.
    LocalContinuityNote,
    /// The deadline / urgency.
    DeadlineUrgency,
    /// The primary action the user or admin can take.
    PrimaryAction,
    /// The recovery action that restores safe operation.
    RecoveryAction,
}

impl M5EmergencyBannerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReasonClass,
        Self::AffectedCapability,
        Self::BlastRadius,
        Self::LocalContinuityNote,
        Self::DeadlineUrgency,
        Self::PrimaryAction,
        Self::RecoveryAction,
    ];

    /// The anatomy parts every emergency banner must render inline. All parts are
    /// mandatory — no emergency truth may hide behind a detail drawer.
    pub const MANDATORY: [Self; 7] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReasonClass => "reason_class",
            Self::AffectedCapability => "affected_capability",
            Self::BlastRadius => "blast_radius",
            Self::LocalContinuityNote => "local_continuity_note",
            Self::DeadlineUrgency => "deadline_urgency",
            Self::PrimaryAction => "primary_action",
            Self::RecoveryAction => "recovery_action",
        }
    }
}

/// One channel that renders the shared emergency banner. Every banner projects the same
/// reason, continuity posture, primary action, and dismissal state into all four so
/// update, extension-host, native-notification, and support surfaces describe the same
/// emergency truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmergencyBannerChannel {
    /// The update center.
    UpdateCenter,
    /// The extension / marketplace host surface.
    ExtensionHost,
    /// The operating-system native notification.
    NativeNotification,
    /// A support bundle export.
    SupportBundle,
}

impl M5EmergencyBannerChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UpdateCenter,
        Self::ExtensionHost,
        Self::NativeNotification,
        Self::SupportBundle,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpdateCenter => "update_center",
            Self::ExtensionHost => "extension_host",
            Self::NativeNotification => "native_notification",
            Self::SupportBundle => "support_bundle",
        }
    }
}

/// A focus / navigation behavior the emergency banner supports so the reason,
/// continuity, the primary and recovery actions, and the dismissal control stay
/// keyboard-reachable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmergencyBannerFocusBehavior {
    /// The banner is reachable and operable by keyboard focus.
    BannerKeyboardFocusable,
    /// The primary action is keyboard-reachable.
    PrimaryActionReachable,
    /// The recovery action is keyboard-reachable.
    RecoveryActionReachable,
    /// The dismissal control(s) permitted by the policy are keyboard-reachable.
    DismissalControlReachable,
    /// The local-continuity posture is announced to a screen reader, never color-only.
    ContinuityAnnouncedToScreenReader,
    /// A stable deep-link anchor jumps to the full emergency detail.
    DeepLinkToEmergencyDetail,
}

impl M5EmergencyBannerFocusBehavior {
    /// Every focus behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BannerKeyboardFocusable,
        Self::PrimaryActionReachable,
        Self::RecoveryActionReachable,
        Self::DismissalControlReachable,
        Self::ContinuityAnnouncedToScreenReader,
        Self::DeepLinkToEmergencyDetail,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BannerKeyboardFocusable => "banner_keyboard_focusable",
            Self::PrimaryActionReachable => "primary_action_reachable",
            Self::RecoveryActionReachable => "recovery_action_reachable",
            Self::DismissalControlReachable => "dismissal_control_reachable",
            Self::ContinuityAnnouncedToScreenReader => "continuity_announced_to_screen_reader",
            Self::DeepLinkToEmergencyDetail => "deep_link_to_emergency_detail",
        }
    }
}

/// One dismissal action an emergency banner can offer. Which actions are offered is
/// derived from the event's dismissal policy so user agency matches the event class
/// instead of one generic close button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmergencyDismissalAction {
    /// Acknowledge that the emergency was seen (acknowledgement is not remediation).
    Acknowledge,
    /// Snooze the banner until a scheduled review.
    Snooze,
    /// Dismiss the banner.
    Dismiss,
}

impl M5EmergencyDismissalAction {
    /// Every dismissal action, in declaration order.
    pub const ALL: [Self; 3] = [Self::Acknowledge, Self::Snooze, Self::Dismiss];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Acknowledge => "acknowledge",
            Self::Snooze => "snooze",
            Self::Dismiss => "dismiss",
        }
    }
}

/// The local-work state at the time the emergency fired, before resolution. This is a
/// resolver-side vocabulary and is not part of the frozen advisory-matrix set. The key
/// property is that only [`Self::DataLossConfirmed`] proves data loss — every other
/// state keeps local editing, review, and export safe and must not imply otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmergencyLocalWorkState {
    /// Editing, review, and export all continue safely.
    EditingReviewExportSafe,
    /// Local work continues in a degraded but safe mode.
    DegradedButSafe,
    /// The affected capability is suspended; other local work stays safe.
    AffectedCapabilitySuspended,
    /// Some actions are blocked until acknowledged; local files stay safe.
    BlockedPendingAcknowledgement,
    /// A specific data-loss event is confirmed — the only state that proves loss.
    DataLossConfirmed,
    /// Continuity has not yet been determined.
    ContinuityNotYetDetermined,
}

impl M5EmergencyLocalWorkState {
    /// Every local-work state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EditingReviewExportSafe,
        Self::DegradedButSafe,
        Self::AffectedCapabilitySuspended,
        Self::BlockedPendingAcknowledgement,
        Self::DataLossConfirmed,
        Self::ContinuityNotYetDetermined,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditingReviewExportSafe => "editing_review_export_safe",
            Self::DegradedButSafe => "degraded_but_safe",
            Self::AffectedCapabilitySuspended => "affected_capability_suspended",
            Self::BlockedPendingAcknowledgement => "blocked_pending_acknowledgement",
            Self::DataLossConfirmed => "data_loss_confirmed",
            Self::ContinuityNotYetDetermined => "continuity_not_yet_determined",
        }
    }

    /// The normalized continuity posture this local-work state resolves to.
    pub const fn continuity_posture(self) -> M5EmergencyContinuityPosture {
        match self {
            Self::EditingReviewExportSafe => M5EmergencyContinuityPosture::LocalWorkContinuesSafely,
            Self::DegradedButSafe => M5EmergencyContinuityPosture::LocalWorkContinuesDegraded,
            Self::AffectedCapabilitySuspended => {
                M5EmergencyContinuityPosture::AffectedCapabilitySuspendedLocalSafe
            }
            Self::BlockedPendingAcknowledgement => {
                M5EmergencyContinuityPosture::BlockedPendingAcknowledgement
            }
            Self::DataLossConfirmed => M5EmergencyContinuityPosture::DataLossProven,
            Self::ContinuityNotYetDetermined => {
                M5EmergencyContinuityPosture::ContinuityAssessmentPending
            }
        }
    }

    /// `true` only when the event actually proves data loss. Every other state must not
    /// let the banner imply data loss.
    pub const fn implies_data_loss(self) -> bool {
        matches!(self, Self::DataLossConfirmed)
    }

    /// `true` when local editing, review, and export can still continue safely. Blocked
    /// and suspended states are still local-safe; only confirmed data loss or an
    /// undetermined assessment are not asserted safe.
    pub const fn local_work_safe(self) -> bool {
        matches!(
            self,
            Self::EditingReviewExportSafe
                | Self::DegradedButSafe
                | Self::AffectedCapabilitySuspended
                | Self::BlockedPendingAcknowledgement
        )
    }
}

/// The normalized local-continuity posture an emergency banner shows. This is a
/// resolver-side vocabulary and is not part of the frozen advisory-matrix set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmergencyContinuityPosture {
    /// Local work continues safely.
    LocalWorkContinuesSafely,
    /// Local work continues in a degraded but safe mode.
    LocalWorkContinuesDegraded,
    /// The affected capability is suspended; other local work stays safe.
    AffectedCapabilitySuspendedLocalSafe,
    /// Some actions are blocked pending acknowledgement; local files stay safe.
    BlockedPendingAcknowledgement,
    /// Data loss is proven by the event.
    DataLossProven,
    /// Continuity assessment is pending.
    ContinuityAssessmentPending,
}

impl M5EmergencyContinuityPosture {
    /// Every continuity posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalWorkContinuesSafely,
        Self::LocalWorkContinuesDegraded,
        Self::AffectedCapabilitySuspendedLocalSafe,
        Self::BlockedPendingAcknowledgement,
        Self::DataLossProven,
        Self::ContinuityAssessmentPending,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkContinuesSafely => "local_work_continues_safely",
            Self::LocalWorkContinuesDegraded => "local_work_continues_degraded",
            Self::AffectedCapabilitySuspendedLocalSafe => {
                "affected_capability_suspended_local_safe"
            }
            Self::BlockedPendingAcknowledgement => "blocked_pending_acknowledgement",
            Self::DataLossProven => "data_loss_proven",
            Self::ContinuityAssessmentPending => "continuity_assessment_pending",
        }
    }
}

/// The dismissal policy an emergency carries, before resolution. This is a
/// resolver-side vocabulary and is not part of the frozen advisory-matrix set. It maps
/// to the frozen [`M5AdvisoryDismissalState`] default and to the allowed acknowledge /
/// snooze / dismiss actions so user agency matches the event class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmergencyDismissalPolicy {
    /// Blocked until remediated: acknowledge only, no snooze, no dismiss.
    NotDismissableBlocked,
    /// Acknowledgement is required before the banner can be cleared.
    AcknowledgeRequired,
    /// The banner may be acknowledged or snoozed until a scheduled review.
    AcknowledgeOrSnooze,
    /// The banner may be acknowledged, snoozed, or dismissed.
    FullyDismissible,
    /// Informational: the banner may be dismissed (nothing to acknowledge).
    InformationalDismissible,
}

impl M5EmergencyDismissalPolicy {
    /// Every dismissal policy, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotDismissableBlocked,
        Self::AcknowledgeRequired,
        Self::AcknowledgeOrSnooze,
        Self::FullyDismissible,
        Self::InformationalDismissible,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotDismissableBlocked => "not_dismissable_blocked",
            Self::AcknowledgeRequired => "acknowledge_required",
            Self::AcknowledgeOrSnooze => "acknowledge_or_snooze",
            Self::FullyDismissible => "fully_dismissible",
            Self::InformationalDismissible => "informational_dismissible",
        }
    }

    /// The default dismissal state (frozen vocabulary) this policy resolves to.
    pub const fn default_state(self) -> M5AdvisoryDismissalState {
        match self {
            Self::NotDismissableBlocked => M5AdvisoryDismissalState::BlockedUntilRemediated,
            Self::AcknowledgeRequired => M5AdvisoryDismissalState::Unacknowledged,
            Self::AcknowledgeOrSnooze => M5AdvisoryDismissalState::Unacknowledged,
            Self::FullyDismissible => M5AdvisoryDismissalState::Unacknowledged,
            Self::InformationalDismissible => M5AdvisoryDismissalState::NotAcknowledgeable,
        }
    }

    /// The dismissal actions this policy permits, in a stable order.
    pub fn allowed_actions(self) -> Vec<M5EmergencyDismissalAction> {
        use M5EmergencyDismissalAction as D;
        match self {
            Self::NotDismissableBlocked => vec![D::Acknowledge],
            Self::AcknowledgeRequired => vec![D::Acknowledge],
            Self::AcknowledgeOrSnooze => vec![D::Acknowledge, D::Snooze],
            Self::FullyDismissible => vec![D::Acknowledge, D::Snooze, D::Dismiss],
            Self::InformationalDismissible => vec![D::Dismiss],
        }
    }

    /// `true` when this policy permits an outright dismiss.
    pub fn permits_dismiss(self) -> bool {
        self.allowed_actions()
            .contains(&M5EmergencyDismissalAction::Dismiss)
    }
}

/// The full input to the emergency-banner resolver for one emergency on one
/// reason-class lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmergencyBannerResolutionInput {
    /// The reason-class lane this emergency renders on.
    pub reason_class: M5EmergencyReasonClass,
    /// The copy-safe emergency-notice id (never a raw reporter identity or URL).
    pub notice_id: String,
    /// The emergency's severity.
    pub severity: M5AdvisorySeverityClass,
    /// Opaque, export-safe representation of the affected capability or component.
    pub affected_capability_repr: String,
    /// Opaque, export-safe representation of the blast radius / affected scope.
    pub blast_radius_repr: String,
    /// The local-work state at the time the emergency fired.
    pub local_work_state: M5EmergencyLocalWorkState,
    /// Opaque, export-safe representation of the deadline / urgency.
    pub deadline_repr: String,
    /// Opaque, export-safe representation of the recovery path.
    pub recovery_repr: String,
    /// Opaque, export-safe representation of the signer / source continuity state.
    pub signer_source_state_repr: String,
    /// The action state this emergency carries.
    pub action_state: M5AdvisoryActionState,
    /// The primary next action this emergency offers.
    pub primary_action: M5AdvisoryRequiredAction,
    /// The recovery action that restores safe operation.
    pub recovery_action: M5AdvisoryRequiredAction,
    /// The local-continuity claim this emergency makes.
    pub continuity_claim: M5AdvisoryContinuityClaim,
    /// The dismissal policy that governs this emergency's agency.
    pub dismissal_policy: M5EmergencyDismissalPolicy,
}

impl M5EmergencyBannerResolutionInput {
    /// True when any representation carries forbidden material.
    fn carries_forbidden_material(&self) -> bool {
        repr_is_forbidden(&self.notice_id)
            || repr_is_forbidden(&self.affected_capability_repr)
            || repr_is_forbidden(&self.blast_radius_repr)
            || repr_is_forbidden(&self.deadline_repr)
            || repr_is_forbidden(&self.recovery_repr)
            || repr_is_forbidden(&self.signer_source_state_repr)
    }
}

/// One channel projection of a resolved emergency banner. Every projection carries the
/// same core truth — severity, continuity posture, primary action, and dismissal
/// state — so the channels stay in parity; only the channel-scoped headline framing
/// differs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEmergencyChannelProjection {
    /// The channel this projection renders on.
    pub channel: M5EmergencyBannerChannel,
    /// The channel-scoped headline (built from the shared emergency truth).
    pub headline: String,
    /// The emergency severity (identical across channels).
    pub severity: M5AdvisorySeverityClass,
    /// The local-continuity posture (identical across channels).
    pub continuity_posture: M5EmergencyContinuityPosture,
    /// The primary next action (identical across channels).
    pub primary_action: M5AdvisoryRequiredAction,
    /// The dismissal state (identical across channels).
    pub dismissal_state: M5AdvisoryDismissalState,
}

/// One export column of the copy-safe emergency-banner summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmergencyExportColumn {
    /// The export field.
    pub field: M5AdvisoryExportField,
    /// The export-safe value.
    pub value: String,
}

/// The copy-safe, export-safe summary of a resolved emergency banner, for support and
/// admin flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmergencyBannerExportSummary {
    /// The copy-safe emergency-notice id.
    pub notice_id: String,
    /// The mandatory export columns, in [`MANDATORY_EXPORT_FIELDS`] order.
    pub columns: Vec<M5EmergencyExportColumn>,
}

/// The resolved emergency banner for one emergency on one reason-class lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEmergencyBanner {
    /// The reason-class lane this emergency renders on.
    pub reason_class: M5EmergencyReasonClass,
    /// The copy-safe emergency-notice id.
    pub notice_id: String,
    /// The emergency's severity.
    pub severity: M5AdvisorySeverityClass,
    /// The opaque affected-capability representation.
    pub affected_capability_repr: String,
    /// The opaque blast-radius representation.
    pub blast_radius_repr: String,
    /// The local-work state at the time the emergency fired.
    pub local_work_state: M5EmergencyLocalWorkState,
    /// The normalized local-continuity posture.
    pub continuity_posture: M5EmergencyContinuityPosture,
    /// The opaque deadline / urgency representation.
    pub deadline_repr: String,
    /// The opaque recovery-path representation.
    pub recovery_repr: String,
    /// The opaque signer / source continuity representation.
    pub signer_source_state_repr: String,
    /// The action state this emergency carries.
    pub action_state: M5AdvisoryActionState,
    /// The primary next action this emergency offers.
    pub primary_action: M5AdvisoryRequiredAction,
    /// The recovery action that restores safe operation.
    pub recovery_action: M5AdvisoryRequiredAction,
    /// The local-continuity claim this emergency makes.
    pub continuity_claim: M5AdvisoryContinuityClaim,
    /// The dismissal policy that governs this emergency's agency.
    pub dismissal_policy: M5EmergencyDismissalPolicy,
    /// The derived default dismissal state.
    pub dismissal_state: M5AdvisoryDismissalState,
    /// The derived allowed dismissal actions.
    pub allowed_dismissal_actions: Vec<M5EmergencyDismissalAction>,
    /// True when local editing, review, and export can still continue safely.
    pub local_work_safe: bool,
    /// True only when the event actually proves data loss.
    pub implies_data_loss: bool,
    /// True — the primitive always keeps the emergency banner visible.
    pub remains_visible: bool,
    /// The same emergency truth projected into every channel.
    pub channel_projections: Vec<M5ResolvedEmergencyChannelProjection>,
    /// The copy-safe, export-safe summary.
    pub export_summary: M5EmergencyBannerExportSummary,
}

/// Errors returned by [`resolve_emergency_banner`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5EmergencyBannerResolutionError {
    /// The notice id was empty.
    EmptyNoticeId,
    /// The affected-capability representation was empty.
    EmptyAffectedCapability,
    /// The blast-radius representation was empty.
    EmptyBlastRadius,
    /// The deadline representation was empty.
    EmptyDeadline,
    /// The recovery-path representation was empty.
    EmptyRecovery,
    /// The signer / source-state representation was empty.
    EmptySignerSourceState,
    /// A representation carried forbidden material.
    ForbiddenMaterial,
}

impl M5EmergencyBannerResolutionError {
    /// Stable token for tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyNoticeId => "empty_notice_id",
            Self::EmptyAffectedCapability => "empty_affected_capability",
            Self::EmptyBlastRadius => "empty_blast_radius",
            Self::EmptyDeadline => "empty_deadline",
            Self::EmptyRecovery => "empty_recovery",
            Self::EmptySignerSourceState => "empty_signer_source_state",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5EmergencyBannerResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "emergency-banner resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5EmergencyBannerResolutionError {}

/// Resolves one emergency into one emergency banner.
///
/// The resolver derives the local-continuity posture from the local-work state, never
/// implies data loss or unsafe local work unless the event actually proves it, derives
/// the dismissal state and the allowed acknowledge / snooze / dismiss actions from the
/// event's dismissal policy, keeps the banner visible, projects the same
/// severity / continuity / primary-action / dismissal-state truth into every channel,
/// and emits a copy-safe, export-safe summary. It never hides the reason, scope,
/// continuity, or next action behind a detail drawer and never drops the copy-safe
/// notice id.
pub fn resolve_emergency_banner(
    input: &M5EmergencyBannerResolutionInput,
) -> Result<M5ResolvedEmergencyBanner, M5EmergencyBannerResolutionError> {
    if input.notice_id.trim().is_empty() {
        return Err(M5EmergencyBannerResolutionError::EmptyNoticeId);
    }
    if input.affected_capability_repr.trim().is_empty() {
        return Err(M5EmergencyBannerResolutionError::EmptyAffectedCapability);
    }
    if input.blast_radius_repr.trim().is_empty() {
        return Err(M5EmergencyBannerResolutionError::EmptyBlastRadius);
    }
    if input.deadline_repr.trim().is_empty() {
        return Err(M5EmergencyBannerResolutionError::EmptyDeadline);
    }
    if input.recovery_repr.trim().is_empty() {
        return Err(M5EmergencyBannerResolutionError::EmptyRecovery);
    }
    if input.signer_source_state_repr.trim().is_empty() {
        return Err(M5EmergencyBannerResolutionError::EmptySignerSourceState);
    }
    if input.carries_forbidden_material() {
        return Err(M5EmergencyBannerResolutionError::ForbiddenMaterial);
    }

    let continuity_posture = input.local_work_state.continuity_posture();
    let local_work_safe = input.local_work_state.local_work_safe();
    let implies_data_loss = input.local_work_state.implies_data_loss();
    let dismissal_state = input.dismissal_policy.default_state();
    let allowed_dismissal_actions = input.dismissal_policy.allowed_actions();

    let channel_projections = M5EmergencyBannerChannel::ALL
        .iter()
        .map(|channel| M5ResolvedEmergencyChannelProjection {
            channel: *channel,
            headline: render_channel_headline(*channel, input, continuity_posture),
            severity: input.severity,
            continuity_posture,
            primary_action: input.primary_action,
            dismissal_state,
        })
        .collect();

    let export_summary = build_export_summary(input);

    Ok(M5ResolvedEmergencyBanner {
        reason_class: input.reason_class,
        notice_id: input.notice_id.clone(),
        severity: input.severity,
        affected_capability_repr: input.affected_capability_repr.clone(),
        blast_radius_repr: input.blast_radius_repr.clone(),
        local_work_state: input.local_work_state,
        continuity_posture,
        deadline_repr: input.deadline_repr.clone(),
        recovery_repr: input.recovery_repr.clone(),
        signer_source_state_repr: input.signer_source_state_repr.clone(),
        action_state: input.action_state,
        primary_action: input.primary_action,
        recovery_action: input.recovery_action,
        continuity_claim: input.continuity_claim,
        dismissal_policy: input.dismissal_policy,
        dismissal_state,
        allowed_dismissal_actions,
        local_work_safe,
        implies_data_loss,
        // The primitive structurally keeps the emergency banner visible: every
        // emergency always resolves to a full, visible banner.
        remains_visible: true,
        channel_projections,
        export_summary,
    })
}

/// Renders one channel-scoped headline from the shared emergency truth. Every channel
/// carries the same reason, severity, continuity posture, deadline, and next action;
/// only the channel prefix differs.
fn render_channel_headline(
    channel: M5EmergencyBannerChannel,
    input: &M5EmergencyBannerResolutionInput,
    continuity_posture: M5EmergencyContinuityPosture,
) -> String {
    format!(
        "[{}] {} · {} · {} · {} · deadline: {} · next: {}",
        channel.as_str(),
        input.notice_id,
        input.reason_class.as_str(),
        input.severity.as_str(),
        continuity_posture.as_str(),
        input.deadline_repr,
        input.primary_action.as_str(),
    )
}

/// Builds the copy-safe, export-safe summary from the shared emergency truth.
fn build_export_summary(
    input: &M5EmergencyBannerResolutionInput,
) -> M5EmergencyBannerExportSummary {
    let columns = MANDATORY_EXPORT_FIELDS
        .iter()
        .map(|field| M5EmergencyExportColumn {
            field: *field,
            value: export_value(*field, input),
        })
        .collect();
    M5EmergencyBannerExportSummary {
        notice_id: input.notice_id.clone(),
        columns,
    }
}

/// Resolves the export-safe value for one export field.
fn export_value(field: M5AdvisoryExportField, input: &M5EmergencyBannerResolutionInput) -> String {
    match field {
        M5AdvisoryExportField::AdvisoryId => input.notice_id.clone(),
        M5AdvisoryExportField::Severity => input.severity.as_str().to_owned(),
        M5AdvisoryExportField::ActionState => input.action_state.as_str().to_owned(),
        M5AdvisoryExportField::AffectedSurface => input.affected_capability_repr.clone(),
        M5AdvisoryExportField::MitigationState => input.recovery_repr.clone(),
        M5AdvisoryExportField::ContinuityNote => input.continuity_claim.as_str().to_owned(),
        // Only the mandatory-export fields are projected into the summary; any other
        // field resolves to its stable token so the mapping stays total.
        other => other.as_str().to_owned(),
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs emergency truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmergencyBannerResolutionCase {
    /// The resolver input.
    pub input: M5EmergencyBannerResolutionInput,
    /// The resolved emergency banner. Must equal `resolve_emergency_banner(&input)`.
    pub resolved: M5ResolvedEmergencyBanner,
}

impl M5EmergencyBannerResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5EmergencyBannerResolutionInput) -> Self {
        let resolved = resolve_emergency_banner(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_emergency_banner(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one reason-class lane bound to the shared banner
/// anatomy, severity vocabulary, channels, dismissal rules, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmergencyReasonRow {
    /// Reason-class lane.
    pub reason_class: M5EmergencyReasonClass,
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
    pub anatomy_parts: Vec<M5EmergencyBannerAnatomyPart>,
    /// Severity classes this row can show.
    pub severity_classes: Vec<M5AdvisorySeverityClass>,
    /// Channels this row projects into (must include every channel — parity).
    pub channels: Vec<M5EmergencyBannerChannel>,
    /// Action states this row projects.
    pub action_states: Vec<M5AdvisoryActionState>,
    /// Primary / recovery actions this row offers.
    pub required_actions: Vec<M5AdvisoryRequiredAction>,
    /// Local-continuity claims this row makes.
    pub continuity_claims: Vec<M5AdvisoryContinuityClaim>,
    /// Dismissal policies this row can carry.
    pub dismissal_policies: Vec<M5EmergencyDismissalPolicy>,
    /// Focus behaviors this row supports.
    pub focus_behaviors: Vec<M5EmergencyBannerFocusBehavior>,
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
    pub example_notices: Vec<M5EmergencyBannerResolutionCase>,
    /// Hard invariant: this row never hides emergency truth behind a detail drawer.
    /// MUST be `false`.
    pub hides_field_behind_detail_drawer: bool,
    /// Hard invariant: this row never implies data loss without proof. MUST be
    /// `false`.
    pub implies_data_loss_without_proof: bool,
    /// Hard invariant: this row never collapses dismissal into one generic close
    /// button. MUST be `false`.
    pub collapses_to_single_generic_dismiss: bool,
    /// Hard invariant: this row never drops the copy-safe id or export summary. MUST
    /// be `false`.
    pub drops_copy_safe_id_or_export: bool,
}

impl M5EmergencyReasonRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5EmergencyBannerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5EmergencyBannerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every channel (all four projected in parity).
    fn declares_all_channels(&self) -> bool {
        let present: BTreeSet<M5EmergencyBannerChannel> = self.channels.iter().copied().collect();
        M5EmergencyBannerChannel::ALL
            .iter()
            .all(|channel| present.contains(channel))
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
        !self.hides_field_behind_detail_drawer
            && !self.implies_data_loss_without_proof
            && !self.collapses_to_single_generic_dismiss
            && !self.drops_copy_safe_id_or_export
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmergencyBannerVocabularySet {
    /// Reason-class-lane tokens.
    pub reason_classes: Vec<String>,
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
    /// Dismissal-state tokens (reused from the frozen matrix).
    pub dismissal_states: Vec<String>,
    /// Dismissal-action tokens.
    pub dismissal_actions: Vec<String>,
    /// Channel tokens.
    pub channels: Vec<String>,
    /// Focus-behavior tokens.
    pub focus_behaviors: Vec<String>,
    /// Export-field tokens (reused from the frozen matrix).
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5EmergencyBannerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            reason_classes: tokens(&M5EmergencyReasonClass::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5EmergencyBannerAnatomyPart::ALL, |v| v.as_str()),
            severity_classes: tokens(&M5AdvisorySeverityClass::ALL, |v| v.as_str()),
            action_states: tokens(&M5AdvisoryActionState::ALL, |v| v.as_str()),
            required_actions: tokens(&M5AdvisoryRequiredAction::ALL, |v| v.as_str()),
            continuity_claims: tokens(&M5AdvisoryContinuityClaim::ALL, |v| v.as_str()),
            dismissal_states: tokens(&M5AdvisoryDismissalState::ALL, |v| v.as_str()),
            dismissal_actions: tokens(&M5EmergencyDismissalAction::ALL, |v| v.as_str()),
            channels: tokens(&M5EmergencyBannerChannel::ALL, |v| v.as_str()),
            focus_behaviors: tokens(&M5EmergencyBannerFocusBehavior::ALL, |v| v.as_str()),
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
pub struct M5EmergencyBannerGovernanceReview {
    /// One emergency banner model is reused across every reason class.
    pub one_banner_model_across_reason_classes: bool,
    /// Reason, scope, continuity, deadline, and the next action are visible without a
    /// secondary detail drawer.
    pub reason_scope_continuity_deadline_visible_without_drawer: bool,
    /// No emergency ever implies data loss unless the event proves it.
    pub never_implies_data_loss_without_proof: bool,
    /// Local-safe continuity messaging is preserved when work can still continue.
    pub local_safe_continuity_preserved: bool,
    /// Dismissal rules match the event class instead of one generic close button.
    pub dismissal_rules_match_event_class: bool,
    /// The copy-safe emergency-notice id is always preserved.
    pub copy_safe_notice_id_preserved: bool,
    /// The export summary reconstructs emergency truth for support / admin.
    pub export_summary_reconstructs_emergency_truth: bool,
    /// Every row is bound to a canonical shell zone.
    pub every_row_bound_to_shell_zone: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 lanes cannot invent parallel emergency-banner vocabulary.
    pub later_lanes_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmergencyBannerConsumerProjection {
    /// The update center renders the shared emergency banner.
    pub update_center_renders_shared_banner: bool,
    /// The extension host renders the shared emergency banner.
    pub extension_host_renders_shared_banner: bool,
    /// The native notification renders the shared emergency banner.
    pub native_notification_renders_shared_banner: bool,
    /// Support / export reads a single canonical emergency-banner source.
    pub support_export_reads_single_source: bool,
    /// The resolver reads a single canonical emergency vocabulary.
    pub resolver_reads_single_emergency_vocabulary: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmergencyBannerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the emergency-banner primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmergencyBannerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting emergency-banner audit.
    pub emergency_banner_audit_ref: String,
    /// True when support / export parity is required for every lane.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every lane.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5EmergencyBannerPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EmergencyBannerPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Reason-class rows.
    pub reason_rows: Vec<M5EmergencyReasonRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EmergencyBannerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EmergencyBannerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EmergencyBannerConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EmergencyBannerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EmergencyBannerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 emergency-notice-banner-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EmergencyBannerPrimitivePacket {
    /// Record kind; must equal [`M5_EMERGENCY_BANNER_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EMERGENCY_BANNER_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Reason-class rows.
    pub reason_rows: Vec<M5EmergencyReasonRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EmergencyBannerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EmergencyBannerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EmergencyBannerConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EmergencyBannerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EmergencyBannerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5EmergencyBannerPrimitivePacket {
    /// Builds an M5 emergency-notice-banner-primitive packet from stable-lane input.
    pub fn new(input: M5EmergencyBannerPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_EMERGENCY_BANNER_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_EMERGENCY_BANNER_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            reason_rows: input.reason_rows,
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

    /// Validates the M5 emergency-notice-banner-primitive invariants.
    pub fn validate(&self) -> Vec<M5EmergencyBannerPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EMERGENCY_BANNER_PRIMITIVE_RECORD_KIND {
            violations.push(M5EmergencyBannerPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EMERGENCY_BANNER_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5EmergencyBannerPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5EmergencyBannerPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_reason_rows(self, &mut violations);
        validate_channel_parity_covered(self, &mut violations);
        validate_local_safe_continuity_covered(self, &mut violations);
        validate_dismissal_rule_covered(self, &mut violations);
        validate_severity_coverage(self, &mut violations);
        validate_continuity_posture_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 emergency-banner primitive packet serializes"),
        ) {
            violations.push(M5EmergencyBannerPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 emergency-banner primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per reason-class lane.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "reason_class,qualification,owner,shell_zone_slot,severity_classes,channels,anatomy_parts,dismissal_policies,export_fields,accessibility_routes,example_count\n",
        );
        for row in &self.reason_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.reason_class.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.severity_classes, |v| v.as_str()),
                join_tokens(&row.channels, |v| v.as_str()),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.dismissal_policies, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                join_tokens(&row.accessibility_routes, |v| v.as_str()),
                row.example_notices.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .reason_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Emergency-Notice Banner Primitive: Reason Class, Affected Capability, Continuity, Deadline, and Dismissal Parity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Reason-class lanes: {} ({} stable)\n",
            self.reason_rows.len(),
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
            "- Dismissal actions: {}\n",
            self.vocabulary_set.dismissal_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Export fields: {}\n",
            self.vocabulary_set.export_fields.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Reason-class lanes\n\n");
        for row in &self.reason_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.reason_class.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Worked emergencies: {}\n",
                row.example_notices.len()
            ));
            for case in &row.example_notices {
                out.push_str(&format!(
                    "    - `{}` — {} ({}), dismissal `{}`{}\n",
                    case.resolved.notice_id,
                    case.resolved.severity.as_str(),
                    case.resolved.continuity_posture.as_str(),
                    case.resolved.dismissal_state.as_str(),
                    if case.resolved.implies_data_loss {
                        ", data loss proven by the event"
                    } else if case.resolved.local_work_safe {
                        ", local work stays safe"
                    } else {
                        ""
                    }
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 emergency-notice-banner-primitive
/// export.
#[derive(Debug)]
pub enum M5EmergencyBannerPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5EmergencyBannerPrimitiveViolation>),
}

impl fmt::Display for M5EmergencyBannerPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 emergency-banner primitive export parse failed: {error}"
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
                    "m5 emergency-banner primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5EmergencyBannerPrimitiveArtifactError {}

/// Validation failures emitted by [`M5EmergencyBannerPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5EmergencyBannerPrimitiveViolation {
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
    /// A required reason-class lane is missing from the matrix.
    RequiredReasonMissing,
    /// A reason-class row is incomplete.
    ReasonRowIncomplete,
    /// A reason-class row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A reason-class row declares no severity classes.
    SeverityClassMissing,
    /// A reason-class row does not declare every channel (channel parity broken).
    ChannelParityMismatch,
    /// A reason-class row declares no action states.
    ActionStateMissing,
    /// A reason-class row declares no required actions.
    RequiredActionMissing,
    /// A reason-class row declares no continuity claims.
    ContinuityClaimMissing,
    /// A reason-class row declares no dismissal policies.
    DismissalPolicyMissing,
    /// A reason-class row declares no focus behaviors.
    FocusBehaviorMissing,
    /// A reason-class row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A reason-class row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A reason-class row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A reason-class row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A reason-class row declares no worked resolution cases.
    ExampleNoticeMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleNoticeDrift,
    /// A lane claiming Stable is missing required proof packet refs.
    StableReasonMissingProof,
    /// No worked resolution across the matrix projects every channel in parity.
    ChannelParityUnproven,
    /// No worked resolution across the matrix keeps local work safe without implying
    /// data loss.
    LocalSafeContinuityUnproven,
    /// The worked resolutions do not prove explicit, event-matched dismissal rules.
    DismissalRuleUnproven,
    /// No worked resolution across the matrix exercises every severity class.
    SeverityCoverageUnproven,
    /// No worked resolution across the matrix exercises every continuity posture.
    ContinuityPostureCoverageUnproven,
    /// A reason-class row violates a hard invariant.
    ReasonInvariantViolated,
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

impl M5EmergencyBannerPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredReasonMissing => "required_reason_missing",
            Self::ReasonRowIncomplete => "reason_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::SeverityClassMissing => "severity_class_missing",
            Self::ChannelParityMismatch => "channel_parity_mismatch",
            Self::ActionStateMissing => "action_state_missing",
            Self::RequiredActionMissing => "required_action_missing",
            Self::ContinuityClaimMissing => "continuity_claim_missing",
            Self::DismissalPolicyMissing => "dismissal_policy_missing",
            Self::FocusBehaviorMissing => "focus_behavior_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleNoticeMissing => "example_notice_missing",
            Self::ExampleNoticeDrift => "example_notice_drift",
            Self::StableReasonMissingProof => "stable_reason_missing_proof",
            Self::ChannelParityUnproven => "channel_parity_unproven",
            Self::LocalSafeContinuityUnproven => "local_safe_continuity_unproven",
            Self::DismissalRuleUnproven => "dismissal_rule_unproven",
            Self::SeverityCoverageUnproven => "severity_coverage_unproven",
            Self::ContinuityPostureCoverageUnproven => "continuity_posture_coverage_unproven",
            Self::ReasonInvariantViolated => "reason_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 emergency-notice-banner-primitive
/// export.
pub fn current_stable_m5_emergency_notice_banner_primitive_export(
) -> Result<M5EmergencyBannerPrimitivePacket, M5EmergencyBannerPrimitiveArtifactError> {
    let packet: M5EmergencyBannerPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-emergency-notice-banner-proof/support_export.json"
    )))
    .map_err(M5EmergencyBannerPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5EmergencyBannerPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5EmergencyBannerPrimitivePacket,
    violations: &mut Vec<M5EmergencyBannerPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EMERGENCY_BANNER_SCHEMA_REF,
        M5_EMERGENCY_BANNER_DOC_REF,
        M5_EMERGENCY_BANNER_SHELL_ZONE_REF,
        M5_EMERGENCY_BANNER_COMPONENT_MATRIX_REF,
        M5_EMERGENCY_BANNER_EMERGENCY_ACTION_REF,
        M5_EMERGENCY_BANNER_DISABLE_BUNDLE_REF,
        M5_EMERGENCY_BANNER_LOCAL_CONTINUITY_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5EmergencyBannerPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5EmergencyBannerPrimitivePacket,
    violations: &mut Vec<M5EmergencyBannerPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5EmergencyBannerPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_reason_rows(
    packet: &M5EmergencyBannerPrimitivePacket,
    violations: &mut Vec<M5EmergencyBannerPrimitiveViolation>,
) {
    let present: BTreeSet<M5EmergencyReasonClass> = packet
        .reason_rows
        .iter()
        .map(|row| row.reason_class)
        .collect();
    for required in M5EmergencyReasonClass::ALL {
        if !present.contains(&required) {
            violations.push(M5EmergencyBannerPrimitiveViolation::RequiredReasonMissing);
            return;
        }
    }

    for row in &packet.reason_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
        {
            violations.push(M5EmergencyBannerPrimitiveViolation::ReasonRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5EmergencyBannerPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.severity_classes.is_empty() {
            violations.push(M5EmergencyBannerPrimitiveViolation::SeverityClassMissing);
        }
        if !row.declares_all_channels() {
            violations.push(M5EmergencyBannerPrimitiveViolation::ChannelParityMismatch);
        }
        if row.action_states.is_empty() {
            violations.push(M5EmergencyBannerPrimitiveViolation::ActionStateMissing);
        }
        if row.required_actions.is_empty() {
            violations.push(M5EmergencyBannerPrimitiveViolation::RequiredActionMissing);
        }
        if row.continuity_claims.is_empty() {
            violations.push(M5EmergencyBannerPrimitiveViolation::ContinuityClaimMissing);
        }
        if row.dismissal_policies.is_empty() {
            violations.push(M5EmergencyBannerPrimitiveViolation::DismissalPolicyMissing);
        }
        if row.focus_behaviors.is_empty() {
            violations.push(M5EmergencyBannerPrimitiveViolation::FocusBehaviorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5EmergencyBannerPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5AdvisoryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5EmergencyBannerPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5EmergencyBannerPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5EmergencyBannerPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_notices.is_empty() {
            violations.push(M5EmergencyBannerPrimitiveViolation::ExampleNoticeMissing);
        }
        if row
            .example_notices
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5EmergencyBannerPrimitiveViolation::ExampleNoticeDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5EmergencyBannerPrimitiveViolation::StableReasonMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5EmergencyBannerPrimitiveViolation::ReasonInvariantViolated);
        }
    }
}

/// Every channel must be projected in parity by some worked resolution — the
/// acceptance-criterion proof (AC1) that a kill switch, trust-root rotation, channel
/// freeze, and forced-disable all render one emergency banner model across update,
/// extension-host, native-notification, and support surfaces.
fn validate_channel_parity_covered(
    packet: &M5EmergencyBannerPrimitivePacket,
    violations: &mut Vec<M5EmergencyBannerPrimitiveViolation>,
) {
    let present: BTreeSet<M5EmergencyBannerChannel> = packet
        .reason_rows
        .iter()
        .flat_map(|row| row.example_notices.iter())
        .flat_map(|case| case.resolved.channel_projections.iter())
        .map(|projection| projection.channel)
        .collect();
    if !M5EmergencyBannerChannel::ALL
        .iter()
        .all(|channel| present.contains(channel))
    {
        violations.push(M5EmergencyBannerPrimitiveViolation::ChannelParityUnproven);
    }
}

/// At least one worked resolution must keep local work safe while NOT implying data
/// loss, with the full banner rendered inline and a complete export summary — the
/// acceptance-criterion proof (AC2) that an emergency notice no longer implies data
/// loss or unsafe local work unless the event actually proves it.
fn validate_local_safe_continuity_covered(
    packet: &M5EmergencyBannerPrimitivePacket,
    violations: &mut Vec<M5EmergencyBannerPrimitiveViolation>,
) {
    let proven = packet
        .reason_rows
        .iter()
        .flat_map(|row| row.example_notices.iter())
        .any(|case| {
            let banner = &case.resolved;
            banner.remains_visible
                && banner.local_work_safe
                && !banner.implies_data_loss
                && !banner.notice_id.trim().is_empty()
                && !banner.affected_capability_repr.trim().is_empty()
                && !banner.blast_radius_repr.trim().is_empty()
                && !banner.deadline_repr.trim().is_empty()
                && !banner.recovery_repr.trim().is_empty()
                && banner.export_summary.columns.len() >= MANDATORY_EXPORT_FIELDS.len()
                && banner
                    .export_summary
                    .columns
                    .iter()
                    .all(|column| !column.value.trim().is_empty())
        });
    if !proven {
        violations.push(M5EmergencyBannerPrimitiveViolation::LocalSafeContinuityUnproven);
    }
}

/// The worked resolutions must prove explicit, event-matched dismissal rules — the
/// acceptance-criterion proof (AC3) that dismissal behavior is explicit and consistent
/// and not one generic close button: the union of allowed dismissal actions must cover
/// acknowledge, snooze, and dismiss, and at least one worked emergency must forbid an
/// outright dismiss (a non-dismissable, must-acknowledge event).
fn validate_dismissal_rule_covered(
    packet: &M5EmergencyBannerPrimitivePacket,
    violations: &mut Vec<M5EmergencyBannerPrimitiveViolation>,
) {
    let cases: Vec<&M5ResolvedEmergencyBanner> = packet
        .reason_rows
        .iter()
        .flat_map(|row| row.example_notices.iter())
        .map(|case| &case.resolved)
        .collect();
    let actions: BTreeSet<M5EmergencyDismissalAction> = cases
        .iter()
        .flat_map(|banner| banner.allowed_dismissal_actions.iter().copied())
        .collect();
    let has_non_dismissable = cases.iter().any(|banner| {
        !banner
            .allowed_dismissal_actions
            .contains(&M5EmergencyDismissalAction::Dismiss)
    });
    let covers_all_actions = M5EmergencyDismissalAction::ALL
        .iter()
        .all(|action| actions.contains(action));
    if !covers_all_actions || !has_non_dismissable {
        violations.push(M5EmergencyBannerPrimitiveViolation::DismissalRuleUnproven);
    }
}

/// Every severity class must be exercised by some worked resolution so the banner is
/// proven to render every severity.
fn validate_severity_coverage(
    packet: &M5EmergencyBannerPrimitivePacket,
    violations: &mut Vec<M5EmergencyBannerPrimitiveViolation>,
) {
    let present: BTreeSet<M5AdvisorySeverityClass> = packet
        .reason_rows
        .iter()
        .flat_map(|row| row.example_notices.iter())
        .map(|case| case.resolved.severity)
        .collect();
    if !M5AdvisorySeverityClass::ALL
        .iter()
        .all(|severity| present.contains(severity))
    {
        violations.push(M5EmergencyBannerPrimitiveViolation::SeverityCoverageUnproven);
    }
}

/// Every continuity posture must be exercised by some worked resolution so the banner
/// is proven to render every local-continuity posture — including the sole posture
/// that proves data loss.
fn validate_continuity_posture_coverage(
    packet: &M5EmergencyBannerPrimitivePacket,
    violations: &mut Vec<M5EmergencyBannerPrimitiveViolation>,
) {
    let present: BTreeSet<M5EmergencyContinuityPosture> = packet
        .reason_rows
        .iter()
        .flat_map(|row| row.example_notices.iter())
        .map(|case| case.resolved.continuity_posture)
        .collect();
    if !M5EmergencyContinuityPosture::ALL
        .iter()
        .all(|posture| present.contains(posture))
    {
        violations.push(M5EmergencyBannerPrimitiveViolation::ContinuityPostureCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5EmergencyBannerPrimitivePacket,
    violations: &mut Vec<M5EmergencyBannerPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_banner_model_across_reason_classes,
        review.reason_scope_continuity_deadline_visible_without_drawer,
        review.never_implies_data_loss_without_proof,
        review.local_safe_continuity_preserved,
        review.dismissal_rules_match_event_class,
        review.copy_safe_notice_id_preserved,
        review.export_summary_reconstructs_emergency_truth,
        review.every_row_bound_to_shell_zone,
        review.every_row_declares_accessibility_route,
        review.later_lanes_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5EmergencyBannerPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5EmergencyBannerPrimitivePacket,
    violations: &mut Vec<M5EmergencyBannerPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.update_center_renders_shared_banner,
        projection.extension_host_renders_shared_banner,
        projection.native_notification_renders_shared_banner,
        projection.support_export_reads_single_source,
        projection.resolver_reads_single_emergency_vocabulary,
    ] {
        if !ok {
            violations.push(M5EmergencyBannerPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5EmergencyBannerPrimitivePacket,
    violations: &mut Vec<M5EmergencyBannerPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5EmergencyBannerPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5EmergencyBannerPrimitivePacket,
    violations: &mut Vec<M5EmergencyBannerPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.emergency_banner_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5EmergencyBannerPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces
/// a stray comma.
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
