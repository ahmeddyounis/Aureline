//! Native-desktop system-entry, handler-ownership, reopen, and
//! OS-notification matrix for the M5 desktop surfaces.
//!
//! Aureline's local-first, native-desktop promise reaches more M5 surfaces and
//! handoff objects, but the system-entry and OS-affordance contract is easy to
//! leave implicit: system open/save/reveal flows, file associations, protocol
//! handlers, browser auth callbacks, recent-item and dock/taskbar/jump-list
//! reopen paths, OS notifications, badge/progress signals, removable-volume or
//! network-share disappearance, and credential-store lock states each have
//! behavior that a single "OS integration supported" claim hides. This module
//! makes that contract explicit and inspectable: one machine-readable matrix
//! names every claimed system-entry, handler, reopen, notification, and
//! external-path surface and binds it to the controls every native-desktop
//! entry must honor.
//!
//! Track invariant the matrix enforces: OS-level entry and reopen never bypass
//! trust, profile, tenant, or policy evaluation; channel/build ownership is
//! inspectable so no handler can be silently taken over; notification, badge,
//! and progress signals derive from durable objects rather than transient
//! polls; and missing roots, locked stores, or topology drift preserve user
//! context through truthful placeholders and recovery actions.
//!
//! Each registered surface declares a binding for each of the canonical
//! native-desktop controls:
//!
//! - `trust_policy_evaluation`
//! - `channel_build_ownership`
//! - `wrong_target_recovery`
//! - `unavailable_path_recovery`
//! - `policy_block_recovery`
//! - `signal_durability`
//! - `notification_privacy`
//!
//! and every surface is one of the required system-entry/reopen surface kinds:
//!
//! - `system_open`
//! - `file_association`
//! - `protocol_handler`
//! - `auth_callback`
//! - `recent_item`
//! - `dock_taskbar_jumplist`
//! - `os_notification`
//! - `badge_progress`
//! - `removable_path`
//! - `store_lock_state`
//!
//! The resulting [`NativeDesktopMatrixReport`] is the canonical truth object
//! for the native-desktop entry/reopen/notification lane. It is consumed by:
//!
//! - the live shell platform inspector and Help/About rail (so the in-product
//!   matrix quotes the same per-control findings the CLI prints);
//! - the headless inspector (`aureline_shell_m5_native_desktop`), which is the
//!   only mint-from-truth path for the JSON fixtures checked in under
//!   `fixtures/platform/m5_os_entry_and_reopen/`;
//! - the support-export wrapper that lets a reviewer pivot from a support case
//!   to the surface that flagged a wrong-target, hidden-takeover, or
//!   privacy-unsafe result;
//! - the markdown matrix under
//!   `artifacts/platform/m5-native-desktop-matrix.md` and the shiproom review
//!   packet (rendered from the same seed); and
//! - install/update UI, release notes, and partner evaluations, which ingest
//!   the matrix directly instead of maintaining parallel installer notes, UI
//!   copy, and support tribal knowledge.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every required surface kind is present, and every registered surface
//!    declares a binding for each of the seven canonical controls.
//! 2. Every surface carries a channel/build owner, a trust checkpoint, a
//!    canonical reopen anchor, a non-empty continuity note, at least one
//!    claimed platform, a non-empty degraded-state vocabulary, a downgrade
//!    rule, and a flag asserting it rides the governed native-desktop harness;
//!    a missing field, or a surface that drives its own entry path off the
//!    harness, is a blocker.
//! 3. A satisfied control carries the evidence it requires — an evidence pack
//!    for every control, a recovery-path ref on the three recovery controls,
//!    and a durable-object ref on `signal_durability`. A failed control is a
//!    blocker, and each failure stays a distinct class: a bypassed trust
//!    evaluation, a hidden handler takeover, a wrong-target reopen with no
//!    recovery, a silent loss on an unavailable path, an unsafe policy block, a
//!    transient-poll signal, and a privacy-unsafe notification are never
//!    collapsed into one generic finding.
//! 4. Stale evidence on a marketed surface is a blocker, so release tooling can
//!    narrow the surface instead of shipping it as implicitly stable.
//! 5. The report cross-links the install-topology, embedded-boundary,
//!    activity-center, and auth-recovery packets so channel or handler
//!    ownership cannot drift independently.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/platform/m5_os_entry_and_reopen/` are
//! bit-for-bit equal to the seeded report returned by
//! [`seeded_native_desktop_matrix`].

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every native-desktop matrix record.
pub const NATIVE_DESKTOP_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every native-desktop surface.
pub const NATIVE_DESKTOP_SHARED_CONTRACT_REF: &str = "shell:m5_native_desktop:v1";

/// Stable record kind for [`NativeDesktopMatrixReport`] payloads.
pub const NATIVE_DESKTOP_REPORT_RECORD_KIND: &str = "shell_m5_native_desktop_matrix_report_record";

/// Stable record kind for [`NativeDesktopEntryRow`] payloads.
pub const NATIVE_DESKTOP_ROW_RECORD_KIND: &str = "shell_m5_native_desktop_entry_record";

/// Stable record kind for [`NativeDesktopSupportExport`] payloads.
pub const NATIVE_DESKTOP_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_native_desktop_support_export_record";

/// Stable report id quoted across surfaces.
pub const NATIVE_DESKTOP_REPORT_ID: &str = "shell:m5_native_desktop:matrix:v1";

/// Stable support-export id quoted in the published wrapper.
pub const NATIVE_DESKTOP_SUPPORT_EXPORT_ID: &str = "support-export:m5-native-desktop:001";

/// Source schema ref for the canonical native-desktop matrix contract.
pub const NATIVE_DESKTOP_SOURCE_SCHEMA_REF: &str =
    "schemas/platform/m5-native-desktop-matrix.schema.json";

/// Path of the published markdown matrix artifact.
pub const NATIVE_DESKTOP_PUBLISHED_REPORT_REF: &str =
    "artifacts/platform/m5-native-desktop-matrix.md";

/// Path of the published companion doc.
pub const NATIVE_DESKTOP_PUBLISHED_DOC_REF: &str =
    "docs/m5/native-desktop-integration-and-reopen.md";

/// Path of the published shiproom review packet.
pub const NATIVE_DESKTOP_REVIEW_PACKET_REF: &str =
    "artifacts/shiproom/m5-native-desktop-review-packet/native_desktop_review_packet.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-16T00:00:00Z";

/// One system-entry or reopen surface kind the matrix governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeDesktopSurfaceKind {
    /// System open / save / reveal flows.
    SystemOpen,
    /// File-type associations.
    FileAssociation,
    /// Protocol / deep-link scheme handlers.
    ProtocolHandler,
    /// Browser auth callbacks returning to the app.
    AuthCallback,
    /// Recent-item lists.
    RecentItem,
    /// Dock, taskbar, and jump-list reopen entries.
    DockTaskbarJumplist,
    /// OS notifications.
    OsNotification,
    /// Badge and progress indicators.
    BadgeProgress,
    /// Removable-volume or network-share paths that can disappear.
    RemovablePath,
    /// Credential-store lock states.
    StoreLockState,
}

impl NativeDesktopSurfaceKind {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemOpen => "system_open",
            Self::FileAssociation => "file_association",
            Self::ProtocolHandler => "protocol_handler",
            Self::AuthCallback => "auth_callback",
            Self::RecentItem => "recent_item",
            Self::DockTaskbarJumplist => "dock_taskbar_jumplist",
            Self::OsNotification => "os_notification",
            Self::BadgeProgress => "badge_progress",
            Self::RemovablePath => "removable_path",
            Self::StoreLockState => "store_lock_state",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::SystemOpen => "System open / save / reveal",
            Self::FileAssociation => "File association",
            Self::ProtocolHandler => "Protocol handler",
            Self::AuthCallback => "Auth callback",
            Self::RecentItem => "Recent item",
            Self::DockTaskbarJumplist => "Dock / taskbar / jump-list",
            Self::OsNotification => "OS notification",
            Self::BadgeProgress => "Badge / progress",
            Self::RemovablePath => "Removable / network path",
            Self::StoreLockState => "Credential-store lock state",
        }
    }

    /// `true` for the notification-class surfaces that emit a signal which must
    /// derive from a durable object and stay privacy-safe.
    pub const fn is_signal_surface(self) -> bool {
        matches!(self, Self::OsNotification | Self::BadgeProgress)
    }

    /// Returns the ten required surface kinds in canonical order.
    pub const fn required_kinds() -> [Self; 10] {
        [
            Self::SystemOpen,
            Self::FileAssociation,
            Self::ProtocolHandler,
            Self::AuthCallback,
            Self::RecentItem,
            Self::DockTaskbarJumplist,
            Self::OsNotification,
            Self::BadgeProgress,
            Self::RemovablePath,
            Self::StoreLockState,
        ]
    }
}

/// A desktop platform the matrix scopes a surface to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeDesktopPlatform {
    /// macOS desktop platform.
    Macos,
    /// Windows desktop platform.
    Windows,
    /// Linux desktop platform.
    Linux,
}

impl NativeDesktopPlatform {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }

    /// Returns the three platforms in canonical order.
    pub const fn all() -> [Self; 3] {
        [Self::Macos, Self::Windows, Self::Linux]
    }
}

/// How the channel/build owns the OS-level registration for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeDesktopOwnershipKind {
    /// Each channel owns its own registration; side-by-side installs do not
    /// collide.
    ChannelScopedOwner,
    /// A shared default is arbitrated by explicit user or admin choice.
    SharedDefaultArbitrated,
    /// A managed fleet deployment owns the registration centrally.
    ManagedFleetOwned,
    /// A portable build does not register an OS-level handler.
    PortableNonRegistering,
}

impl NativeDesktopOwnershipKind {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelScopedOwner => "channel_scoped_owner",
            Self::SharedDefaultArbitrated => "shared_default_arbitrated",
            Self::ManagedFleetOwned => "managed_fleet_owned",
            Self::PortableNonRegistering => "portable_non_registering",
        }
    }
}

/// One of the seven canonical native-desktop controls every surface binds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeDesktopControl {
    /// OS-level entry routes through trust / profile / tenant / policy
    /// evaluation rather than bypassing it.
    TrustPolicyEvaluation,
    /// The channel/build that owns the OS registration is inspectable; no
    /// handler can be silently taken over.
    ChannelBuildOwnership,
    /// A reopen hits the exact target, and a wrong target offers a recovery
    /// path rather than dead-ending.
    WrongTargetRecovery,
    /// A missing root or unavailable path preserves context through a truthful
    /// placeholder and a recovery action.
    UnavailablePathRecovery,
    /// A policy-blocked entry degrades truthfully with a recovery action.
    PolicyBlockRecovery,
    /// A notification / badge / progress signal derives from a durable object
    /// rather than a transient poll.
    SignalDurability,
    /// Notification, badge, and progress content is privacy-safe and carries no
    /// credential body or secret on shared surfaces.
    NotificationPrivacy,
}

impl NativeDesktopControl {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustPolicyEvaluation => "trust_policy_evaluation",
            Self::ChannelBuildOwnership => "channel_build_ownership",
            Self::WrongTargetRecovery => "wrong_target_recovery",
            Self::UnavailablePathRecovery => "unavailable_path_recovery",
            Self::PolicyBlockRecovery => "policy_block_recovery",
            Self::SignalDurability => "signal_durability",
            Self::NotificationPrivacy => "notification_privacy",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::TrustPolicyEvaluation => "Trust / policy evaluation",
            Self::ChannelBuildOwnership => "Channel / build ownership",
            Self::WrongTargetRecovery => "Wrong-target recovery",
            Self::UnavailablePathRecovery => "Unavailable-path recovery",
            Self::PolicyBlockRecovery => "Policy-block recovery",
            Self::SignalDurability => "Signal durability",
            Self::NotificationPrivacy => "Notification privacy",
        }
    }

    /// Returns the seven canonical controls in canonical order.
    pub const fn required_controls() -> [Self; 7] {
        [
            Self::TrustPolicyEvaluation,
            Self::ChannelBuildOwnership,
            Self::WrongTargetRecovery,
            Self::UnavailablePathRecovery,
            Self::PolicyBlockRecovery,
            Self::SignalDurability,
            Self::NotificationPrivacy,
        ]
    }

    /// The distinct failure class this control fails into.
    pub const fn canonical_failure_mode(self) -> NativeDesktopFailureMode {
        match self {
            Self::TrustPolicyEvaluation => NativeDesktopFailureMode::TrustEvaluationBypassed,
            Self::ChannelBuildOwnership => NativeDesktopFailureMode::HiddenHandlerTakeover,
            Self::WrongTargetRecovery => NativeDesktopFailureMode::WrongTargetNoRecovery,
            Self::UnavailablePathRecovery => NativeDesktopFailureMode::UnavailablePathSilentLoss,
            Self::PolicyBlockRecovery => NativeDesktopFailureMode::PolicyBlockUnsafe,
            Self::SignalDurability => NativeDesktopFailureMode::TransientPollSignal,
            Self::NotificationPrivacy => NativeDesktopFailureMode::PrivacyUnsafeNotification,
        }
    }

    /// `true` for the three recovery controls that must name a recovery path
    /// when satisfied.
    pub const fn requires_recovery_path(self) -> bool {
        matches!(
            self,
            Self::WrongTargetRecovery | Self::UnavailablePathRecovery | Self::PolicyBlockRecovery
        )
    }

    /// `true` for the control whose satisfied binding must name the durable
    /// object the signal derives from.
    pub const fn requires_durable_object(self) -> bool {
        matches!(self, Self::SignalDurability)
    }
}

/// A distinct native-desktop failure class.
///
/// Each class names a materially different way a native-desktop surface can
/// break. They are never collapsed: a wrong-target reopen, a hidden handler
/// takeover, and a privacy-unsafe notification are separate findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeDesktopFailureMode {
    /// OS-level entry bypassed trust / profile / tenant / policy evaluation.
    TrustEvaluationBypassed,
    /// A handler was silently taken over with no inspectable channel/build
    /// owner.
    HiddenHandlerTakeover,
    /// A reopen hit the wrong target and offered no recovery.
    WrongTargetNoRecovery,
    /// An unavailable path silently lost user context.
    UnavailablePathSilentLoss,
    /// A policy-blocked entry behaved unsafely instead of degrading truthfully.
    PolicyBlockUnsafe,
    /// A signal derived from a transient poll instead of a durable object.
    TransientPollSignal,
    /// A notification, badge, or progress signal leaked private content.
    PrivacyUnsafeNotification,
}

impl NativeDesktopFailureMode {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustEvaluationBypassed => "trust_evaluation_bypassed",
            Self::HiddenHandlerTakeover => "hidden_handler_takeover",
            Self::WrongTargetNoRecovery => "wrong_target_no_recovery",
            Self::UnavailablePathSilentLoss => "unavailable_path_silent_loss",
            Self::PolicyBlockUnsafe => "policy_block_unsafe",
            Self::TransientPollSignal => "transient_poll_signal",
            Self::PrivacyUnsafeNotification => "privacy_unsafe_notification",
        }
    }
}

/// Status a surface reports for one canonical control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeDesktopControlStatus {
    /// The control is satisfied with captured evidence.
    Satisfied,
    /// The control does not apply to this surface; a reason MUST be set.
    NotApplicable,
    /// The surface narrows this control; a reason MUST be set.
    ExplicitlyNarrowed,
    /// The control is failed. Always a blocker.
    Failed,
}

impl NativeDesktopControlStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::NotApplicable => "not_applicable",
            Self::ExplicitlyNarrowed => "explicitly_narrowed",
            Self::Failed => "failed",
        }
    }

    /// `true` for statuses that require a `narrowing_reason`.
    pub const fn requires_narrowing_reason(self) -> bool {
        matches!(self, Self::NotApplicable | Self::ExplicitlyNarrowed)
    }

    /// `true` for the status that projects captured evidence.
    pub const fn projects_evidence(self) -> bool {
        matches!(self, Self::Satisfied)
    }
}

/// Freshness of the captured native-desktop evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeDesktopEvidenceFreshness {
    /// The evidence is current.
    Fresh,
    /// The evidence is stale. A blocker on a marketed surface.
    Stale,
}

impl NativeDesktopEvidenceFreshness {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

/// Cross-links to the canonical upstream packets the matrix depends on so
/// channel or handler ownership cannot drift independently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopCrossLinks {
    /// Install-topology / portability governance packet.
    pub install_topology_ref: String,
    /// Embedded-boundary audit packet.
    pub embedded_boundary_ref: String,
    /// Activity-center / durable-attention packet.
    pub activity_center_ref: String,
    /// Auth-and-recovery packet.
    pub auth_recovery_ref: String,
    /// Channel-ownership audit ledger.
    pub channel_ownership_ref: String,
    /// Protocol-handler ownership matrix.
    pub protocol_handler_ownership_ref: String,
    /// File-association ownership matrix.
    pub file_association_ownership_ref: String,
}

impl NativeDesktopCrossLinks {
    /// Returns the cross-link fields as `(label, ref)` pairs in canonical
    /// order.
    pub fn as_pairs(&self) -> [(&'static str, &str); 7] {
        [
            ("install_topology_ref", &self.install_topology_ref),
            ("embedded_boundary_ref", &self.embedded_boundary_ref),
            ("activity_center_ref", &self.activity_center_ref),
            ("auth_recovery_ref", &self.auth_recovery_ref),
            ("channel_ownership_ref", &self.channel_ownership_ref),
            (
                "protocol_handler_ownership_ref",
                &self.protocol_handler_ownership_ref,
            ),
            (
                "file_association_ownership_ref",
                &self.file_association_ownership_ref,
            ),
        ]
    }

    /// The canonical cross-link set every matrix carries.
    pub fn canonical() -> Self {
        Self {
            install_topology_ref: "artifacts/install/m5/m5-install-and-portability-governance.md"
                .to_owned(),
            embedded_boundary_ref:
                "artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md"
                    .to_owned(),
            activity_center_ref:
                "artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md".to_owned(),
            auth_recovery_ref: "artifacts/auth/m5_auth_and_recovery.md".to_owned(),
            channel_ownership_ref: "artifacts/release/channel_ownership_audit.yaml".to_owned(),
            protocol_handler_ownership_ref:
                "artifacts/platform/protocol_handler_ownership_matrix.yaml".to_owned(),
            file_association_ownership_ref:
                "artifacts/platform/file_association_ownership_matrix.yaml".to_owned(),
        }
    }
}

/// Canonical descriptor for one native-desktop entry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopDescriptor {
    /// Stable entry id (e.g. `entry:protocol.aureline_scheme`).
    pub entry_id: String,
    /// Surface kind the entry belongs to.
    pub surface_kind: NativeDesktopSurfaceKind,
    /// Descriptor revision the matrix was produced against.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Channel/build owner of the OS-level registration. MUST be non-empty.
    pub channel_build_owner_ref: String,
    /// How the channel/build owns the registration.
    pub ownership_kind: NativeDesktopOwnershipKind,
    /// Trust / profile / tenant / policy checkpoint the entry routes through.
    /// MUST be non-empty.
    pub trust_checkpoint_ref: String,
    /// Durable object the entry reopens. MUST be non-empty.
    pub reopen_anchor_ref: String,
    /// Continuity note retained on the descriptor. MUST be non-empty.
    pub continuity_note: String,
    /// Exact degraded-state vocabulary user-visible surfaces MUST use when this
    /// entry is degraded. MUST be non-empty.
    pub degraded_state_vocabulary: Vec<String>,
    /// Claimed platforms. MUST be non-empty.
    pub claimed_platforms: Vec<NativeDesktopPlatform>,
    /// Freshness of the captured evidence.
    pub evidence_freshness: NativeDesktopEvidenceFreshness,
    /// Timestamp the evidence was captured.
    pub evidence_captured_at: String,
    /// Rule user-visible surfaces follow when evidence goes stale. MUST be
    /// non-empty.
    pub downgrade_rule_ref: String,
    /// `true` when the surface is marketed and must pass the matrix or narrow.
    pub marketed: bool,
    /// `true` once the surface rides the governed native-desktop harness and
    /// does not drive its own entry path. MUST be `true`.
    pub registered_on_native_desktop_harness: bool,
}

/// Per-control binding a surface reports for one canonical control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopControlBinding {
    /// Control this binding covers.
    pub control: NativeDesktopControl,
    /// Status the surface reports.
    pub status: NativeDesktopControlStatus,
    /// Failure mode (`Some` only when `status` is `failed`).
    pub failure_mode: Option<NativeDesktopFailureMode>,
    /// Captured evidence-pack ref (required when satisfied).
    pub evidence_pack_ref: Option<String>,
    /// Recovery-path ref (required for the recovery controls when satisfied).
    pub recovery_path_ref: Option<String>,
    /// Durable-object ref (required for `signal_durability` when satisfied).
    pub durable_object_ref: Option<String>,
    /// Narrowing reason set when `status` requires one.
    pub narrowing_reason: Option<String>,
    /// Reviewer-facing free-form note retained on the binding.
    pub note: Option<String>,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum NativeDesktopBlockingFinding {
    /// OS-level entry bypassed trust / policy evaluation.
    TrustEvaluationBypassed {
        /// Surface that exposes the gap.
        entry_id: String,
        /// Control that exposes the gap.
        control: NativeDesktopControl,
    },
    /// A handler was silently taken over.
    HiddenHandlerTakeover {
        entry_id: String,
        control: NativeDesktopControl,
    },
    /// A reopen hit the wrong target with no recovery.
    WrongTargetNoRecovery {
        entry_id: String,
        control: NativeDesktopControl,
    },
    /// An unavailable path silently lost user context.
    UnavailablePathSilentLoss {
        entry_id: String,
        control: NativeDesktopControl,
    },
    /// A policy-blocked entry behaved unsafely.
    PolicyBlockUnsafe {
        entry_id: String,
        control: NativeDesktopControl,
    },
    /// A signal derived from a transient poll.
    TransientPollSignal {
        entry_id: String,
        control: NativeDesktopControl,
    },
    /// A signal leaked private content.
    PrivacyUnsafeNotification {
        entry_id: String,
        control: NativeDesktopControl,
    },
    /// A binding's declared failure mode disagrees with its control's canonical
    /// failure mode, or a failure mode is set without a failed status.
    FailureModeDrift {
        entry_id: String,
        control: NativeDesktopControl,
        /// Declared failure mode, when present.
        declared_failure_mode: Option<NativeDesktopFailureMode>,
    },
    /// A satisfied control is missing its captured evidence pack.
    MissingEvidencePack {
        entry_id: String,
        control: NativeDesktopControl,
    },
    /// A satisfied recovery control is missing its recovery-path ref.
    MissingRecoveryPath {
        entry_id: String,
        control: NativeDesktopControl,
    },
    /// A satisfied `signal_durability` control is missing its durable-object
    /// ref.
    MissingDurableObject {
        entry_id: String,
        control: NativeDesktopControl,
    },
    /// A narrowed control is missing the `narrowing_reason`.
    MissingNarrowingReason {
        entry_id: String,
        control: NativeDesktopControl,
        status: NativeDesktopControlStatus,
    },
    /// The surface is missing a required control binding.
    MissingRequiredControl {
        entry_id: String,
        control: NativeDesktopControl,
    },
    /// A marketed surface carries stale evidence.
    StaleEvidenceOnMarketedSurface { entry_id: String },
    /// The descriptor carries no channel/build owner.
    MissingChannelBuildOwner { entry_id: String },
    /// The descriptor carries no trust checkpoint.
    MissingTrustCheckpoint { entry_id: String },
    /// The descriptor carries no reopen anchor.
    MissingReopenAnchor { entry_id: String },
    /// The descriptor carries no continuity note.
    MissingContinuityNote { entry_id: String },
    /// The descriptor carries no degraded-state vocabulary.
    MissingDegradedStateVocabulary { entry_id: String },
    /// The descriptor claims no platform.
    MissingClaimedPlatforms { entry_id: String },
    /// The descriptor carries no downgrade rule.
    MissingDowngradeRule { entry_id: String },
    /// The surface drives its own entry path off the governed harness.
    SurfaceNotOnHarness { entry_id: String },
}

impl NativeDesktopBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::TrustEvaluationBypassed { .. } => "trust_evaluation_bypassed",
            Self::HiddenHandlerTakeover { .. } => "hidden_handler_takeover",
            Self::WrongTargetNoRecovery { .. } => "wrong_target_no_recovery",
            Self::UnavailablePathSilentLoss { .. } => "unavailable_path_silent_loss",
            Self::PolicyBlockUnsafe { .. } => "policy_block_unsafe",
            Self::TransientPollSignal { .. } => "transient_poll_signal",
            Self::PrivacyUnsafeNotification { .. } => "privacy_unsafe_notification",
            Self::FailureModeDrift { .. } => "failure_mode_drift",
            Self::MissingEvidencePack { .. } => "missing_evidence_pack",
            Self::MissingRecoveryPath { .. } => "missing_recovery_path",
            Self::MissingDurableObject { .. } => "missing_durable_object",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingRequiredControl { .. } => "missing_required_control",
            Self::StaleEvidenceOnMarketedSurface { .. } => "stale_evidence_on_marketed_surface",
            Self::MissingChannelBuildOwner { .. } => "missing_channel_build_owner",
            Self::MissingTrustCheckpoint { .. } => "missing_trust_checkpoint",
            Self::MissingReopenAnchor { .. } => "missing_reopen_anchor",
            Self::MissingContinuityNote { .. } => "missing_continuity_note",
            Self::MissingDegradedStateVocabulary { .. } => "missing_degraded_state_vocabulary",
            Self::MissingClaimedPlatforms { .. } => "missing_claimed_platforms",
            Self::MissingDowngradeRule { .. } => "missing_downgrade_rule",
            Self::SurfaceNotOnHarness { .. } => "surface_not_on_harness",
        }
    }

    /// Returns the entry id this finding is attached to.
    pub fn entry_id(&self) -> &str {
        match self {
            Self::TrustEvaluationBypassed { entry_id, .. }
            | Self::HiddenHandlerTakeover { entry_id, .. }
            | Self::WrongTargetNoRecovery { entry_id, .. }
            | Self::UnavailablePathSilentLoss { entry_id, .. }
            | Self::PolicyBlockUnsafe { entry_id, .. }
            | Self::TransientPollSignal { entry_id, .. }
            | Self::PrivacyUnsafeNotification { entry_id, .. }
            | Self::FailureModeDrift { entry_id, .. }
            | Self::MissingEvidencePack { entry_id, .. }
            | Self::MissingRecoveryPath { entry_id, .. }
            | Self::MissingDurableObject { entry_id, .. }
            | Self::MissingNarrowingReason { entry_id, .. }
            | Self::MissingRequiredControl { entry_id, .. }
            | Self::StaleEvidenceOnMarketedSurface { entry_id }
            | Self::MissingChannelBuildOwner { entry_id }
            | Self::MissingTrustCheckpoint { entry_id }
            | Self::MissingReopenAnchor { entry_id }
            | Self::MissingContinuityNote { entry_id }
            | Self::MissingDegradedStateVocabulary { entry_id }
            | Self::MissingClaimedPlatforms { entry_id }
            | Self::MissingDowngradeRule { entry_id }
            | Self::SurfaceNotOnHarness { entry_id } => entry_id,
        }
    }

    /// Returns the control this finding is attached to, when control-scoped.
    pub fn control(&self) -> Option<NativeDesktopControl> {
        match self {
            Self::TrustEvaluationBypassed { control, .. }
            | Self::HiddenHandlerTakeover { control, .. }
            | Self::WrongTargetNoRecovery { control, .. }
            | Self::UnavailablePathSilentLoss { control, .. }
            | Self::PolicyBlockUnsafe { control, .. }
            | Self::TransientPollSignal { control, .. }
            | Self::PrivacyUnsafeNotification { control, .. }
            | Self::FailureModeDrift { control, .. }
            | Self::MissingEvidencePack { control, .. }
            | Self::MissingRecoveryPath { control, .. }
            | Self::MissingDurableObject { control, .. }
            | Self::MissingNarrowingReason { control, .. }
            | Self::MissingRequiredControl { control, .. } => Some(*control),
            _ => None,
        }
    }
}

/// One per-surface native-desktop matrix row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopEntryRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the surface.
    pub descriptor: NativeDesktopDescriptor,
    /// Per-control bindings, in canonical control order.
    pub bindings: Vec<NativeDesktopControlBinding>,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<NativeDesktopBlockingFinding>,
    /// `true` when the surface is marketed.
    pub marketed: bool,
}

/// One `(class, count)` blocking-finding tally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopFindingCount {
    /// Finding class token.
    pub class: String,
    /// Number of findings in this class.
    pub count: usize,
}

/// Per-class blocking-finding summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopFindingSummary {
    /// Total blocking findings across the matrix.
    pub total_blocking_findings: usize,
    /// Per-class tallies, sorted by class token.
    pub by_class: Vec<NativeDesktopFindingCount>,
}

/// Per-control coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopControlCoverage {
    /// Control this summary covers.
    pub control: NativeDesktopControl,
    /// Number of surfaces that satisfy the control.
    pub satisfied: usize,
    /// Number of surfaces that mark the control not applicable.
    pub not_applicable: usize,
    /// Number of surfaces that explicitly narrow the control.
    pub explicitly_narrowed: usize,
    /// Number of surfaces that fail the control.
    pub failed: usize,
}

/// Per-surface-kind presence summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopKindCoverage {
    /// Surface kind this summary covers.
    pub surface_kind: NativeDesktopSurfaceKind,
    /// Number of registered surfaces of this kind.
    pub entry_count: usize,
}

/// A single reopen-anchor index entry so platform QA, docs, and release
/// surfaces can reopen each entry by its anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopReopenAnchorEntry {
    /// Surface kind the anchor belongs to.
    pub surface_kind: NativeDesktopSurfaceKind,
    /// Entry id the anchor reopens.
    pub entry_id: String,
    /// Canonical reopen anchor ref.
    pub reopen_anchor_ref: String,
}

/// One marketed surface release tooling should narrow because its evidence is
/// stale or a control failed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopNarrowableEntry {
    /// Entry id that must narrow.
    pub entry_id: String,
    /// Control that must narrow, when control-scoped.
    pub control: Option<NativeDesktopControl>,
    /// Stable reason the surface is narrowable.
    pub reason: String,
}

/// Native-desktop system-entry, handler-ownership, reopen, and notification
/// matrix report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopMatrixReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Required surface kinds, in canonical order.
    pub required_surface_kinds: Vec<NativeDesktopSurfaceKind>,
    /// Required controls, in canonical order.
    pub required_controls: Vec<NativeDesktopControl>,
    /// Union of claimed platforms across all surfaces, sorted.
    pub claimed_platforms: Vec<NativeDesktopPlatform>,
    /// Cross-links to upstream packets.
    pub cross_links: NativeDesktopCrossLinks,
    /// Per-surface rows, sorted by `descriptor.entry_id`.
    pub entries: Vec<NativeDesktopEntryRow>,
    /// Per-control coverage summary, in canonical control order.
    pub control_coverage: Vec<NativeDesktopControlCoverage>,
    /// Per-surface-kind presence summary, in canonical kind order.
    pub surface_kind_coverage: Vec<NativeDesktopKindCoverage>,
    /// Per-class blocking-finding summary.
    pub findings_summary: NativeDesktopFindingSummary,
    /// Canonical reopen-anchor index, sorted by entry id.
    pub reopen_anchor_index: Vec<NativeDesktopReopenAnchorEntry>,
    /// Number of registered surfaces present.
    pub registered_entry_count: usize,
    /// Number of surfaces marketed.
    pub marketed_entry_count: usize,
    /// Total control bindings checked.
    pub controls_checked: usize,
    /// Marketed surfaces release tooling should narrow.
    pub narrowable_marketed_entries: Vec<NativeDesktopNarrowableEntry>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this matrix is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Shiproom review-packet publication ref.
    pub review_packet_ref: String,
    /// Docs/help refs the matrix can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the matrix can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the matrix was generated.
    pub generated_at: String,
}

impl NativeDesktopMatrixReport {
    /// Returns `true` when every required control is satisfied by at least one
    /// surface.
    pub fn every_control_satisfied(&self) -> bool {
        NativeDesktopControl::required_controls()
            .into_iter()
            .all(|control| {
                self.entries.iter().any(|entry| {
                    entry.bindings.iter().any(|binding| {
                        binding.control == control
                            && binding.status == NativeDesktopControlStatus::Satisfied
                    })
                })
            })
    }

    /// Returns `true` when every required surface kind has at least one
    /// registered surface.
    pub fn every_kind_present(&self) -> bool {
        NativeDesktopSurfaceKind::required_kinds()
            .into_iter()
            .all(|kind| {
                self.entries
                    .iter()
                    .any(|entry| entry.descriptor.surface_kind == kind)
            })
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "matrix: surfaces={}, marketed={}, controls={}, blocking={}, clean={}",
            self.registered_entry_count,
            self.marketed_entry_count,
            self.controls_checked,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for coverage in &self.control_coverage {
            lines.push(format!(
                "{}: satisfied={}, not_applicable={}, narrowed={}, failed={}",
                coverage.control.as_str(),
                coverage.satisfied,
                coverage.not_applicable,
                coverage.explicitly_narrowed,
                coverage.failed,
            ));
        }
        for entry in &self.entries {
            for finding in &entry.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {} -- {}",
                    finding.class_token(),
                    finding.entry_id(),
                    finding
                        .control()
                        .map(NativeDesktopControl::as_str)
                        .unwrap_or("surface"),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_entries {
            lines.push(format!(
                "narrowable: {} -- {} -- {}",
                narrowable.entry_id,
                narrowable
                    .control
                    .map(NativeDesktopControl::as_str)
                    .unwrap_or("surface"),
                narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown matrix artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 native-desktop integration and reopen matrix\n\n");
        out.push_str(
            "Generated from the seeded matrix in\n\
             [`crate::m5_native_desktop`](../../crates/aureline-shell/src/m5_native_desktop/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- report-md > \\\n  artifacts/platform/m5-native-desktop-matrix.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Claimed platforms: {}\n",
            self.claimed_platforms
                .iter()
                .map(|platform| format!("`{}`", platform.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Registered surfaces: `{}`\n",
            self.registered_entry_count
        ));
        out.push_str(&format!(
            "- Marketed surfaces: `{}`\n",
            self.marketed_entry_count
        ));
        out.push_str(&format!(
            "- Controls checked: `{}`\n",
            self.controls_checked
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed surfaces: `{}`\n",
            self.narrowable_marketed_entries.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Cross-links\n\n");
        out.push_str("| Upstream packet | Ref |\n| --------------- | --- |\n");
        for (label, value) in self.cross_links.as_pairs() {
            out.push_str(&format!("| `{label}` | `{value}` |\n"));
        }
        out.push('\n');

        out.push_str("## Per-control coverage\n\n");
        out.push_str(
            "| Control | Satisfied | Not applicable | Narrowed | Failed |\n\
             | ------- | --------: | -------------: | -------: | -----: |\n",
        );
        for coverage in &self.control_coverage {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                coverage.control.display_label(),
                coverage.satisfied,
                coverage.not_applicable,
                coverage.explicitly_narrowed,
                coverage.failed,
            ));
        }
        out.push('\n');

        out.push_str("## Per-surface-kind coverage\n\n");
        out.push_str(
            "| Surface kind | Registered surfaces |\n| ------------ | ------------------: |\n",
        );
        for coverage in &self.surface_kind_coverage {
            out.push_str(&format!(
                "| {} | {} |\n",
                coverage.surface_kind.display_label(),
                coverage.entry_count,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        for tally in &self.findings_summary.by_class {
            out.push_str(&format!("| `{}` | {} |\n", tally.class, tally.count));
        }
        if self.findings_summary.by_class.is_empty() {
            out.push_str("| _(none)_ | 0 |\n");
        }
        out.push('\n');

        out.push_str("## Reopen anchor index\n\n");
        out.push_str(
            "| Surface kind | Entry | Reopen anchor |\n| ------------ | ----- | ------------- |\n",
        );
        for entry in &self.reopen_anchor_index {
            out.push_str(&format!(
                "| {} | `{}` | `{}` |\n",
                entry.surface_kind.display_label(),
                entry.entry_id,
                entry.reopen_anchor_ref,
            ));
        }
        out.push('\n');

        out.push_str("## Per-surface rows\n\n");
        for entry in &self.entries {
            out.push_str(&format!(
                "### `{}` ({})\n\n",
                entry.descriptor.entry_id,
                entry.descriptor.surface_kind.as_str(),
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                entry.descriptor.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Channel/build owner: `{}` (`{}`)\n",
                entry.descriptor.channel_build_owner_ref,
                entry.descriptor.ownership_kind.as_str(),
            ));
            out.push_str(&format!(
                "- Trust checkpoint: `{}`\n",
                entry.descriptor.trust_checkpoint_ref
            ));
            out.push_str(&format!(
                "- Reopen anchor: `{}`\n",
                entry.descriptor.reopen_anchor_ref
            ));
            out.push_str(&format!(
                "- Claimed platforms: {}\n",
                entry
                    .descriptor
                    .claimed_platforms
                    .iter()
                    .map(|platform| format!("`{}`", platform.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "- Evidence freshness: `{}` (captured `{}`)\n",
                entry.descriptor.evidence_freshness.as_str(),
                entry.descriptor.evidence_captured_at,
            ));
            out.push_str(&format!(
                "- Downgrade rule: `{}`\n",
                entry.descriptor.downgrade_rule_ref
            ));
            out.push_str(&format!(
                "- Marketed: `{}`\n",
                if entry.marketed { "yes" } else { "no" }
            ));
            out.push_str(&format!(
                "- Continuity note: {}\n",
                entry.descriptor.continuity_note
            ));
            out.push_str("- Degraded-state vocabulary:\n");
            for phrase in &entry.descriptor.degraded_state_vocabulary {
                out.push_str(&format!("  - {phrase}\n"));
            }
            out.push('\n');

            out.push_str(
                "| Control | Status | Failure | Recovery path | Durable object | Narrowing reason |\n\
                 | ------- | ------ | ------- | ------------- | -------------- | ---------------- |\n",
            );
            for binding in &entry.bindings {
                let failure = binding
                    .failure_mode
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let recovery = binding.recovery_path_ref.as_deref().unwrap_or("-");
                let durable = binding.durable_object_ref.as_deref().unwrap_or("-");
                let narrowing = binding.narrowing_reason.as_deref().unwrap_or("-");
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` | `{}` | {} |\n",
                    binding.control.display_label(),
                    binding.status.as_str(),
                    failure,
                    recovery,
                    durable,
                    narrowing,
                ));
            }
            out.push('\n');

            if entry.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &entry.blocking_findings {
                    out.push_str(&format!(
                        "- `{}` on `{}`\n",
                        finding.class_token(),
                        finding
                            .control()
                            .map(NativeDesktopControl::as_str)
                            .unwrap_or("surface"),
                    ));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_native_desktop_fixtures\n");
        out.push_str("python3 tools/ci/m5/native_desktop_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the native-desktop matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Matrix report quoted in full.
    pub report: NativeDesktopMatrixReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl NativeDesktopSupportExport {
    /// Builds the support-export wrapper for a matrix report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: NativeDesktopMatrixReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for entry in &report.entries {
            case_ids.push(entry.descriptor.entry_id.clone());
            case_ids.push(entry.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: NATIVE_DESKTOP_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: NATIVE_DESKTOP_SCHEMA_VERSION,
            shared_contract_ref: NATIVE_DESKTOP_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Computes the per-surface blocking findings from a descriptor and its
/// control bindings.
fn compute_surface_findings(
    descriptor: &NativeDesktopDescriptor,
    bindings: &[NativeDesktopControlBinding],
) -> Vec<NativeDesktopBlockingFinding> {
    let mut findings = Vec::new();
    let entry_id = descriptor.entry_id.clone();

    // Descriptor-level (surface-scoped) findings.
    if descriptor.channel_build_owner_ref.trim().is_empty() {
        findings.push(NativeDesktopBlockingFinding::MissingChannelBuildOwner {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.trust_checkpoint_ref.trim().is_empty() {
        findings.push(NativeDesktopBlockingFinding::MissingTrustCheckpoint {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.reopen_anchor_ref.trim().is_empty() {
        findings.push(NativeDesktopBlockingFinding::MissingReopenAnchor {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.continuity_note.trim().is_empty() {
        findings.push(NativeDesktopBlockingFinding::MissingContinuityNote {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor
        .degraded_state_vocabulary
        .iter()
        .all(|phrase| phrase.trim().is_empty())
    {
        findings.push(
            NativeDesktopBlockingFinding::MissingDegradedStateVocabulary {
                entry_id: entry_id.clone(),
            },
        );
    }
    if descriptor.claimed_platforms.is_empty() {
        findings.push(NativeDesktopBlockingFinding::MissingClaimedPlatforms {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.downgrade_rule_ref.trim().is_empty() {
        findings.push(NativeDesktopBlockingFinding::MissingDowngradeRule {
            entry_id: entry_id.clone(),
        });
    }
    if !descriptor.registered_on_native_desktop_harness {
        findings.push(NativeDesktopBlockingFinding::SurfaceNotOnHarness {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.marketed && descriptor.evidence_freshness == NativeDesktopEvidenceFreshness::Stale
    {
        findings.push(
            NativeDesktopBlockingFinding::StaleEvidenceOnMarketedSurface {
                entry_id: entry_id.clone(),
            },
        );
    }

    // Every required control must be bound.
    let present: Vec<NativeDesktopControl> =
        bindings.iter().map(|binding| binding.control).collect();
    for control in NativeDesktopControl::required_controls() {
        if !present.contains(&control) {
            findings.push(NativeDesktopBlockingFinding::MissingRequiredControl {
                entry_id: entry_id.clone(),
                control,
            });
        }
    }

    for binding in bindings {
        compute_binding_findings(&entry_id, binding, &mut findings);
    }

    findings
}

/// Computes the blocking findings for one control binding.
fn compute_binding_findings(
    entry_id: &str,
    binding: &NativeDesktopControlBinding,
    findings: &mut Vec<NativeDesktopBlockingFinding>,
) {
    let control = binding.control;

    match binding.status {
        NativeDesktopControlStatus::Failed => {
            // The control's canonical failure class is always emitted, so the
            // distinct failure modes never collapse into one finding.
            findings.push(failure_finding(control, entry_id));
            // A declared failure mode that disagrees with the canonical mode is
            // drift.
            if binding.failure_mode != Some(control.canonical_failure_mode()) {
                findings.push(NativeDesktopBlockingFinding::FailureModeDrift {
                    entry_id: entry_id.to_owned(),
                    control,
                    declared_failure_mode: binding.failure_mode,
                });
            }
        }
        NativeDesktopControlStatus::Satisfied => {
            if binding.failure_mode.is_some() {
                findings.push(NativeDesktopBlockingFinding::FailureModeDrift {
                    entry_id: entry_id.to_owned(),
                    control,
                    declared_failure_mode: binding.failure_mode,
                });
            }
            if binding.evidence_pack_ref.is_none() {
                findings.push(NativeDesktopBlockingFinding::MissingEvidencePack {
                    entry_id: entry_id.to_owned(),
                    control,
                });
            }
            if control.requires_recovery_path() && binding.recovery_path_ref.is_none() {
                findings.push(NativeDesktopBlockingFinding::MissingRecoveryPath {
                    entry_id: entry_id.to_owned(),
                    control,
                });
            }
            if control.requires_durable_object() && binding.durable_object_ref.is_none() {
                findings.push(NativeDesktopBlockingFinding::MissingDurableObject {
                    entry_id: entry_id.to_owned(),
                    control,
                });
            }
        }
        status if status.requires_narrowing_reason() => {
            if binding.failure_mode.is_some() {
                findings.push(NativeDesktopBlockingFinding::FailureModeDrift {
                    entry_id: entry_id.to_owned(),
                    control,
                    declared_failure_mode: binding.failure_mode,
                });
            }
            let reason_ok = binding
                .narrowing_reason
                .as_deref()
                .map(str::trim)
                .map(str::is_empty)
                == Some(false);
            if !reason_ok {
                findings.push(NativeDesktopBlockingFinding::MissingNarrowingReason {
                    entry_id: entry_id.to_owned(),
                    control,
                    status,
                });
            }
        }
        _ => {}
    }
}

/// Maps a failed control to its distinct blocking finding.
fn failure_finding(control: NativeDesktopControl, entry_id: &str) -> NativeDesktopBlockingFinding {
    let entry_id = entry_id.to_owned();
    match control.canonical_failure_mode() {
        NativeDesktopFailureMode::TrustEvaluationBypassed => {
            NativeDesktopBlockingFinding::TrustEvaluationBypassed { entry_id, control }
        }
        NativeDesktopFailureMode::HiddenHandlerTakeover => {
            NativeDesktopBlockingFinding::HiddenHandlerTakeover { entry_id, control }
        }
        NativeDesktopFailureMode::WrongTargetNoRecovery => {
            NativeDesktopBlockingFinding::WrongTargetNoRecovery { entry_id, control }
        }
        NativeDesktopFailureMode::UnavailablePathSilentLoss => {
            NativeDesktopBlockingFinding::UnavailablePathSilentLoss { entry_id, control }
        }
        NativeDesktopFailureMode::PolicyBlockUnsafe => {
            NativeDesktopBlockingFinding::PolicyBlockUnsafe { entry_id, control }
        }
        NativeDesktopFailureMode::TransientPollSignal => {
            NativeDesktopBlockingFinding::TransientPollSignal { entry_id, control }
        }
        NativeDesktopFailureMode::PrivacyUnsafeNotification => {
            NativeDesktopBlockingFinding::PrivacyUnsafeNotification { entry_id, control }
        }
    }
}

/// Computes the per-control and per-class summaries from finished surfaces.
fn summarize_report(
    entries: &[NativeDesktopEntryRow],
) -> (
    Vec<NativeDesktopControlCoverage>,
    Vec<NativeDesktopKindCoverage>,
    NativeDesktopFindingSummary,
) {
    let mut control_coverage: Vec<NativeDesktopControlCoverage> =
        NativeDesktopControl::required_controls()
            .into_iter()
            .map(|control| NativeDesktopControlCoverage {
                control,
                satisfied: 0,
                not_applicable: 0,
                explicitly_narrowed: 0,
                failed: 0,
            })
            .collect();

    let mut kind_coverage: Vec<NativeDesktopKindCoverage> =
        NativeDesktopSurfaceKind::required_kinds()
            .into_iter()
            .map(|surface_kind| NativeDesktopKindCoverage {
                surface_kind,
                entry_count: 0,
            })
            .collect();

    let mut class_counts: Vec<NativeDesktopFindingCount> = Vec::new();
    let mut total = 0usize;

    for entry in entries {
        if let Some(kind_row) = kind_coverage
            .iter_mut()
            .find(|row| row.surface_kind == entry.descriptor.surface_kind)
        {
            kind_row.entry_count += 1;
        }
        for binding in &entry.bindings {
            if let Some(coverage) = control_coverage
                .iter_mut()
                .find(|row| row.control == binding.control)
            {
                match binding.status {
                    NativeDesktopControlStatus::Satisfied => coverage.satisfied += 1,
                    NativeDesktopControlStatus::NotApplicable => coverage.not_applicable += 1,
                    NativeDesktopControlStatus::ExplicitlyNarrowed => {
                        coverage.explicitly_narrowed += 1
                    }
                    NativeDesktopControlStatus::Failed => coverage.failed += 1,
                }
            }
        }
        for finding in &entry.blocking_findings {
            total += 1;
            let class = finding.class_token();
            if let Some(tally) = class_counts.iter_mut().find(|tally| tally.class == class) {
                tally.count += 1;
            } else {
                class_counts.push(NativeDesktopFindingCount {
                    class: class.to_owned(),
                    count: 1,
                });
            }
        }
    }

    class_counts.sort_by(|left, right| left.class.cmp(&right.class));
    (
        control_coverage,
        kind_coverage,
        NativeDesktopFindingSummary {
            total_blocking_findings: total,
            by_class: class_counts,
        },
    )
}

/// Computes the marketed surfaces release tooling should narrow because their
/// evidence is stale or a control failed.
fn compute_narrowable_entries(
    entries: &[NativeDesktopEntryRow],
) -> Vec<NativeDesktopNarrowableEntry> {
    let mut narrowable = Vec::new();
    for entry in entries {
        if !entry.marketed {
            continue;
        }
        for finding in &entry.blocking_findings {
            narrowable.push(NativeDesktopNarrowableEntry {
                entry_id: entry.descriptor.entry_id.clone(),
                control: finding.control(),
                reason: format!("blocking_finding:{}", finding.class_token()),
            });
        }
    }
    narrowable
}

/// Builds a [`NativeDesktopEntryRow`] from a descriptor and its bindings,
/// computing the per-surface blocking findings.
pub fn build_native_desktop_row(
    descriptor: NativeDesktopDescriptor,
    bindings: Vec<NativeDesktopControlBinding>,
) -> NativeDesktopEntryRow {
    let marketed = descriptor.marketed;
    let blocking_findings = compute_surface_findings(&descriptor, &bindings);

    NativeDesktopEntryRow {
        record_kind: NATIVE_DESKTOP_ROW_RECORD_KIND.to_owned(),
        schema_version: NATIVE_DESKTOP_SCHEMA_VERSION,
        shared_contract_ref: NATIVE_DESKTOP_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        bindings,
        blocking_findings,
        marketed,
    }
}

/// Builds a full [`NativeDesktopMatrixReport`] from per-surface rows.
pub fn build_native_desktop_matrix(
    entries: Vec<NativeDesktopEntryRow>,
) -> NativeDesktopMatrixReport {
    let mut entries = entries;
    entries.sort_by(|left, right| left.descriptor.entry_id.cmp(&right.descriptor.entry_id));

    let registered_entry_count = entries.len();
    let marketed_entry_count = entries.iter().filter(|entry| entry.marketed).count();
    let controls_checked = entries
        .iter()
        .map(|entry| entry.bindings.len())
        .sum::<usize>();

    let (control_coverage, surface_kind_coverage, findings_summary) = summarize_report(&entries);
    let narrowable_marketed_entries = compute_narrowable_entries(&entries);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut platform_set: Vec<NativeDesktopPlatform> = Vec::new();
    for entry in &entries {
        for platform in &entry.descriptor.claimed_platforms {
            if !platform_set.contains(platform) {
                platform_set.push(*platform);
            }
        }
    }
    platform_set.sort();

    let mut reopen_anchor_index: Vec<NativeDesktopReopenAnchorEntry> = entries
        .iter()
        .map(|entry| NativeDesktopReopenAnchorEntry {
            surface_kind: entry.descriptor.surface_kind,
            entry_id: entry.descriptor.entry_id.clone(),
            reopen_anchor_ref: entry.descriptor.reopen_anchor_ref.clone(),
        })
        .collect();
    reopen_anchor_index.sort_by(|left, right| left.entry_id.cmp(&right.entry_id));

    NativeDesktopMatrixReport {
        record_kind: NATIVE_DESKTOP_REPORT_RECORD_KIND.to_owned(),
        schema_version: NATIVE_DESKTOP_SCHEMA_VERSION,
        shared_contract_ref: NATIVE_DESKTOP_SHARED_CONTRACT_REF.to_owned(),
        report_id: NATIVE_DESKTOP_REPORT_ID.to_owned(),
        source_schema_ref: NATIVE_DESKTOP_SOURCE_SCHEMA_REF.to_owned(),
        required_surface_kinds: NativeDesktopSurfaceKind::required_kinds().to_vec(),
        required_controls: NativeDesktopControl::required_controls().to_vec(),
        claimed_platforms: platform_set,
        cross_links: NativeDesktopCrossLinks::canonical(),
        entries,
        control_coverage,
        surface_kind_coverage,
        findings_summary,
        reopen_anchor_index,
        registered_entry_count,
        marketed_entry_count,
        controls_checked,
        narrowable_marketed_entries,
        report_clean,
        published_report_ref: NATIVE_DESKTOP_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: NATIVE_DESKTOP_PUBLISHED_DOC_REF.to_owned(),
        review_packet_ref: NATIVE_DESKTOP_REVIEW_PACKET_REF.to_owned(),
        docs_help_refs: vec![
            NATIVE_DESKTOP_PUBLISHED_DOC_REF.to_owned(),
            "docs/help/native_desktop_integration.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-native-desktop".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_native_desktop_matrix`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum NativeDesktopValidationError {
    /// The matrix has no registered surfaces.
    NoRegisteredEntries,
    /// A required surface kind has no registered surface.
    RequiredSurfaceKindMissing { surface_kind: String },
    /// A required control has no satisfied surface.
    RequiredControlNotSatisfied { control: String },
    /// A surface is missing a required control from its binding set.
    MissingRequiredControl { entry_id: String, control: String },
    /// A blocking finding remains on the surface.
    BlockingFindingPresent {
        entry_id: String,
        control: String,
        class: String,
    },
    /// A cross-link ref is empty.
    CrossLinkMissing { field: String },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// The shiproom review-packet ref is empty.
    ReviewPacketRefMissing,
    /// A surface's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { entry_id: String },
}

/// Validates a matrix report against the native-desktop acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_native_desktop_matrix(
    report: &NativeDesktopMatrixReport,
) -> Result<(), Vec<NativeDesktopValidationError>> {
    let mut errors = Vec::new();

    if report.entries.is_empty() {
        errors.push(NativeDesktopValidationError::NoRegisteredEntries);
    }

    for kind in NativeDesktopSurfaceKind::required_kinds() {
        let present = report
            .entries
            .iter()
            .any(|entry| entry.descriptor.surface_kind == kind);
        if !present {
            errors.push(NativeDesktopValidationError::RequiredSurfaceKindMissing {
                surface_kind: kind.as_str().to_owned(),
            });
        }
    }

    for control in NativeDesktopControl::required_controls() {
        let any_satisfied = report.entries.iter().any(|entry| {
            entry.bindings.iter().any(|binding| {
                binding.control == control
                    && binding.status == NativeDesktopControlStatus::Satisfied
            })
        });
        if !any_satisfied {
            errors.push(NativeDesktopValidationError::RequiredControlNotSatisfied {
                control: control.as_str().to_owned(),
            });
        }
    }

    for entry in &report.entries {
        for control in NativeDesktopControl::required_controls() {
            if !entry
                .bindings
                .iter()
                .any(|binding| binding.control == control)
            {
                errors.push(NativeDesktopValidationError::MissingRequiredControl {
                    entry_id: entry.descriptor.entry_id.clone(),
                    control: control.as_str().to_owned(),
                });
            }
        }
        if entry.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(NativeDesktopValidationError::MissingDescriptorRevisionRef {
                entry_id: entry.descriptor.entry_id.clone(),
            });
        }
        for finding in &entry.blocking_findings {
            errors.push(NativeDesktopValidationError::BlockingFindingPresent {
                entry_id: finding.entry_id().to_owned(),
                control: finding
                    .control()
                    .map(|control| control.as_str().to_owned())
                    .unwrap_or_else(|| "surface".to_owned()),
                class: finding.class_token().to_owned(),
            });
        }
    }

    for (field, value) in report.cross_links.as_pairs() {
        if value.trim().is_empty() {
            errors.push(NativeDesktopValidationError::CrossLinkMissing {
                field: field.to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(NativeDesktopValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(NativeDesktopValidationError::PublishedDocRefMissing);
    }
    if report.review_packet_ref.trim().is_empty() {
        errors.push(NativeDesktopValidationError::ReviewPacketRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_native_desktop_matrix`].
struct SurfaceSeed {
    entry_id: &'static str,
    surface_kind: NativeDesktopSurfaceKind,
    descriptor_revision_ref: &'static str,
    primary_label_ref: &'static str,
    channel_build_owner_ref: &'static str,
    ownership_kind: NativeDesktopOwnershipKind,
    trust_checkpoint_ref: &'static str,
    reopen_anchor_ref: &'static str,
    continuity_note: &'static str,
    degraded_state_vocabulary: &'static [&'static str],
    /// Controls this surface narrows (everything else is satisfied), with the
    /// honest reason.
    narrowed_controls: &'static [(NativeDesktopControl, &'static str)],
}

/// Builds the control bindings for a seed: every required control is satisfied
/// unless the seed narrows it with a documented reason.
fn build_bindings_from_seed(seed: &SurfaceSeed) -> Vec<NativeDesktopControlBinding> {
    NativeDesktopControl::required_controls()
        .into_iter()
        .map(|control| {
            if let Some((_, reason)) = seed
                .narrowed_controls
                .iter()
                .find(|(narrowed, _)| *narrowed == control)
            {
                NativeDesktopControlBinding {
                    control,
                    status: NativeDesktopControlStatus::NotApplicable,
                    failure_mode: None,
                    evidence_pack_ref: None,
                    recovery_path_ref: None,
                    durable_object_ref: None,
                    narrowing_reason: Some((*reason).to_owned()),
                    note: None,
                }
            } else {
                NativeDesktopControlBinding {
                    control,
                    status: NativeDesktopControlStatus::Satisfied,
                    failure_mode: None,
                    evidence_pack_ref: Some(format!(
                        "drill:{}:{}",
                        seed.entry_id,
                        control.as_str()
                    )),
                    recovery_path_ref: control
                        .requires_recovery_path()
                        .then(|| format!("recovery:{}:{}", seed.entry_id, control.as_str())),
                    durable_object_ref: control
                        .requires_durable_object()
                        .then(|| seed.reopen_anchor_ref.to_owned()),
                    narrowing_reason: None,
                    note: None,
                }
            }
        })
        .collect()
}

fn build_surface_from_seed(seed: &SurfaceSeed) -> NativeDesktopEntryRow {
    let descriptor = NativeDesktopDescriptor {
        entry_id: seed.entry_id.to_owned(),
        surface_kind: seed.surface_kind,
        descriptor_revision_ref: seed.descriptor_revision_ref.to_owned(),
        primary_label_ref: seed.primary_label_ref.to_owned(),
        channel_build_owner_ref: seed.channel_build_owner_ref.to_owned(),
        ownership_kind: seed.ownership_kind,
        trust_checkpoint_ref: seed.trust_checkpoint_ref.to_owned(),
        reopen_anchor_ref: seed.reopen_anchor_ref.to_owned(),
        continuity_note: seed.continuity_note.to_owned(),
        degraded_state_vocabulary: seed
            .degraded_state_vocabulary
            .iter()
            .map(|phrase| (*phrase).to_owned())
            .collect(),
        claimed_platforms: NativeDesktopPlatform::all().to_vec(),
        evidence_freshness: NativeDesktopEvidenceFreshness::Fresh,
        evidence_captured_at: GENERATED_AT.to_owned(),
        downgrade_rule_ref: "downgrade:native_desktop:narrow_on_stale_evidence".to_owned(),
        marketed: true,
        registered_on_native_desktop_harness: true,
    };
    let bindings = build_bindings_from_seed(seed);
    build_native_desktop_row(descriptor, bindings)
}

// The non-signal entry/reopen surfaces narrow the two signal controls; the
// notification-class surfaces satisfy all seven.
const NOT_A_SIGNAL: &[(NativeDesktopControl, &str)] = &[
    (
        NativeDesktopControl::SignalDurability,
        "surface_emits_no_os_signal_so_signal_durability_is_not_applicable",
    ),
    (
        NativeDesktopControl::NotificationPrivacy,
        "surface_emits_no_os_signal_so_notification_privacy_is_not_applicable",
    ),
];

const SURFACE_SEEDS: &[SurfaceSeed] = &[
    // System open / save / reveal.
    SurfaceSeed {
        entry_id: "entry:system_open.workspace_target",
        surface_kind: NativeDesktopSurfaceKind::SystemOpen,
        descriptor_revision_ref: "entry-rev:system_open.workspace_target:2026.06.01-01",
        primary_label_ref: "label:system_open.workspace_target:primary",
        channel_build_owner_ref: "channel-owner:system_open.active_install",
        ownership_kind: NativeDesktopOwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: "trust:system_open.profile_tenant_policy",
        reopen_anchor_ref: "reopen:anchor:system_open:workspace_target",
        continuity_note: "A system open routes through the active profile and tenant policy before it reveals a target, and a missing target reopens to a truthful placeholder rather than a blank window.",
        degraded_state_vocabulary: &[
            "Open in this profile",
            "Choose a different target",
            "This target is no longer available",
        ],
        narrowed_controls: NOT_A_SIGNAL,
    },
    // File association.
    SurfaceSeed {
        entry_id: "entry:file_association.notebook_doc",
        surface_kind: NativeDesktopSurfaceKind::FileAssociation,
        descriptor_revision_ref: "entry-rev:file_association.notebook_doc:2026.06.01-01",
        primary_label_ref: "label:file_association.notebook_doc:primary",
        channel_build_owner_ref: "channel-owner:file_association.shared_default",
        ownership_kind: NativeDesktopOwnershipKind::SharedDefaultArbitrated,
        trust_checkpoint_ref: "trust:file_association.profile_tenant_policy",
        reopen_anchor_ref: "reopen:anchor:file_association:notebook_doc",
        continuity_note: "An opened document carries the file path through profile and policy evaluation, and a side-by-side channel cannot claim the shared default without explicit arbitration.",
        degraded_state_vocabulary: &[
            "Open with Aureline",
            "This file type is registered to another channel",
            "Reopen the original file",
        ],
        narrowed_controls: NOT_A_SIGNAL,
    },
    // Protocol handler.
    SurfaceSeed {
        entry_id: "entry:protocol_handler.aureline_scheme",
        surface_kind: NativeDesktopSurfaceKind::ProtocolHandler,
        descriptor_revision_ref: "entry-rev:protocol_handler.aureline_scheme:2026.06.01-01",
        primary_label_ref: "label:protocol_handler.aureline_scheme:primary",
        channel_build_owner_ref: "channel-owner:protocol_handler.shared_default",
        ownership_kind: NativeDesktopOwnershipKind::SharedDefaultArbitrated,
        trust_checkpoint_ref: "trust:protocol_handler.profile_tenant_policy",
        reopen_anchor_ref: "reopen:anchor:protocol_handler:aureline_scheme",
        continuity_note: "A deep link resolves the exact target in the signed-in profile or fails closed with a recovery action; the scheme owner is inspectable so no install can silently take it over.",
        degraded_state_vocabulary: &[
            "Reopen this link in your signed-in profile",
            "This link points to a target you cannot access",
            "This link has expired",
        ],
        narrowed_controls: NOT_A_SIGNAL,
    },
    // Auth callback.
    SurfaceSeed {
        entry_id: "entry:auth_callback.browser_return",
        surface_kind: NativeDesktopSurfaceKind::AuthCallback,
        descriptor_revision_ref: "entry-rev:auth_callback.browser_return:2026.06.01-01",
        primary_label_ref: "label:auth_callback.browser_return:primary",
        channel_build_owner_ref: "channel-owner:auth_callback.active_install",
        ownership_kind: NativeDesktopOwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: "trust:auth_callback.profile_tenant_policy",
        reopen_anchor_ref: "reopen:anchor:auth_callback:browser_return",
        continuity_note: "A browser auth callback returns to the exact pending sign-in in the originating profile, and an expired or policy-blocked callback recovers truthfully instead of dead-ending.",
        degraded_state_vocabulary: &[
            "Return to Aureline to finish signing in",
            "This sign-in link has expired",
            "Sign-in was blocked by policy",
        ],
        narrowed_controls: NOT_A_SIGNAL,
    },
    // Recent item.
    SurfaceSeed {
        entry_id: "entry:recent_item.workspace_list",
        surface_kind: NativeDesktopSurfaceKind::RecentItem,
        descriptor_revision_ref: "entry-rev:recent_item.workspace_list:2026.06.01-01",
        primary_label_ref: "label:recent_item.workspace_list:primary",
        channel_build_owner_ref: "channel-owner:recent_item.active_install",
        ownership_kind: NativeDesktopOwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: "trust:recent_item.profile_tenant_policy",
        reopen_anchor_ref: "reopen:anchor:recent_item:workspace_list",
        continuity_note: "A recent-item reopen re-evaluates profile and policy and lands on the exact target, and a moved or removed item shows a truthful placeholder with a recovery action.",
        degraded_state_vocabulary: &[
            "This item moved or was removed",
            "Reopen in the original workspace",
            "Sign in to reopen this item",
        ],
        narrowed_controls: NOT_A_SIGNAL,
    },
    // Dock / taskbar / jump-list.
    SurfaceSeed {
        entry_id: "entry:dock_taskbar_jumplist.reopen",
        surface_kind: NativeDesktopSurfaceKind::DockTaskbarJumplist,
        descriptor_revision_ref: "entry-rev:dock_taskbar_jumplist.reopen:2026.06.01-01",
        primary_label_ref: "label:dock_taskbar_jumplist.reopen:primary",
        channel_build_owner_ref: "channel-owner:dock_taskbar_jumplist.active_install",
        ownership_kind: NativeDesktopOwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: "trust:dock_taskbar_jumplist.profile_tenant_policy",
        reopen_anchor_ref: "reopen:anchor:dock_taskbar_jumplist:reopen",
        continuity_note: "A dock, taskbar, or jump-list reopen routes through policy and lands on the durable target, and a pinned entry whose target is gone or blocked recovers truthfully.",
        degraded_state_vocabulary: &[
            "This pinned item is no longer available",
            "Reopen in this profile",
            "Removed by policy",
        ],
        narrowed_controls: NOT_A_SIGNAL,
    },
    // OS notification (signal surface).
    SurfaceSeed {
        entry_id: "entry:os_notification.run_complete",
        surface_kind: NativeDesktopSurfaceKind::OsNotification,
        descriptor_revision_ref: "entry-rev:os_notification.run_complete:2026.06.01-01",
        primary_label_ref: "label:os_notification.run_complete:primary",
        channel_build_owner_ref: "channel-owner:os_notification.active_install",
        ownership_kind: NativeDesktopOwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: "trust:os_notification.profile_tenant_policy",
        reopen_anchor_ref: "reopen:anchor:os_notification:run_complete",
        continuity_note: "A completion notification derives from a durable activity object, reopens the exact run in the signed-in profile, stays privacy-safe on the lock screen, and respects quiet hours and policy.",
        degraded_state_vocabulary: &[
            "Reopen the item this alert is about",
            "This item is no longer available",
            "Muted by quiet hours",
        ],
        narrowed_controls: &[],
    },
    // Badge / progress (signal surface).
    SurfaceSeed {
        entry_id: "entry:badge_progress.dock_badge",
        surface_kind: NativeDesktopSurfaceKind::BadgeProgress,
        descriptor_revision_ref: "entry-rev:badge_progress.dock_badge:2026.06.01-01",
        primary_label_ref: "label:badge_progress.dock_badge:primary",
        channel_build_owner_ref: "channel-owner:badge_progress.active_install",
        ownership_kind: NativeDesktopOwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: "trust:badge_progress.profile_tenant_policy",
        reopen_anchor_ref: "reopen:anchor:badge_progress:dock_badge",
        continuity_note: "A dock badge and progress indicator derive from durable counts for the active profile only, hide private detail on shared surfaces, and pause under policy rather than polling.",
        degraded_state_vocabulary: &[
            "Counts reflect this profile only",
            "Hidden on the lock screen",
            "Paused by policy",
        ],
        narrowed_controls: &[],
    },
    // Removable / network path.
    SurfaceSeed {
        entry_id: "entry:removable_path.network_share",
        surface_kind: NativeDesktopSurfaceKind::RemovablePath,
        descriptor_revision_ref: "entry-rev:removable_path.network_share:2026.06.01-01",
        primary_label_ref: "label:removable_path.network_share:primary",
        channel_build_owner_ref: "channel-owner:removable_path.active_install",
        ownership_kind: NativeDesktopOwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: "trust:removable_path.profile_tenant_policy",
        reopen_anchor_ref: "reopen:anchor:removable_path:network_share",
        continuity_note: "A removable volume or network share that disappears keeps the user on the last saved copy with a truthful placeholder and a reconnect action, never a silent data loss.",
        degraded_state_vocabulary: &[
            "Reconnect the drive to continue",
            "This volume is no longer mounted",
            "Working from the last saved copy",
        ],
        narrowed_controls: NOT_A_SIGNAL,
    },
    // Credential-store lock state.
    SurfaceSeed {
        entry_id: "entry:store_lock_state.credential_store",
        surface_kind: NativeDesktopSurfaceKind::StoreLockState,
        descriptor_revision_ref: "entry-rev:store_lock_state.credential_store:2026.06.01-01",
        primary_label_ref: "label:store_lock_state.credential_store:primary",
        channel_build_owner_ref: "channel-owner:store_lock_state.active_install",
        ownership_kind: NativeDesktopOwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: "trust:store_lock_state.profile_tenant_policy",
        reopen_anchor_ref: "reopen:anchor:store_lock_state:credential_store",
        continuity_note: "A locked credential store preserves the pending action with a truthful unlock prompt and a recovery action, and never silently signs the user out or proceeds without trust evaluation.",
        degraded_state_vocabulary: &[
            "Unlock the credential store to continue",
            "The credential store is locked",
            "Signed out until you unlock",
        ],
        narrowed_controls: NOT_A_SIGNAL,
    },
];

/// Seeded matrix builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/platform/m5_os_entry_and_reopen/`.
pub fn seeded_native_desktop_matrix() -> NativeDesktopMatrixReport {
    let entries = SURFACE_SEEDS.iter().map(build_surface_from_seed).collect();
    build_native_desktop_matrix(entries)
}
