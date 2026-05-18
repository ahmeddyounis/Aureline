//! Native desktop entry, reopen, and interruption contract.
//!
//! This module composes shell-side native handoff review, notification
//! privacy, desktop continuity, and lifecycle recovery evidence into one
//! beta contract packet. It is intentionally pure: callers can render,
//! export, or validate the packet without registering handlers, opening
//! files, replaying callbacks, or mutating OS state.

use std::collections::BTreeSet;

use aureline_commands::invocation::now_rfc3339;
use serde::{Deserialize, Serialize};

use crate::deeplink::native_handoff::{
    seeded_native_boundary_handoff_packet, NativeBoundaryHandoffPacket,
    NativeBoundaryHandoffReviewRecord, NativeFileAffordanceClass, NativeFileHandoffReviewRecord,
    SourceSurfaceClass,
};
use crate::desktop_continuity_alpha::{
    seeded_desktop_continuity_alpha_packet, DesktopContinuityAlphaPacket,
};
use crate::notifications::beta::{
    seeded_notification_privacy_beta_page, validate_notification_privacy_beta_page,
    NotificationPrivacyBetaPage, NotificationPrivacyBetaRow, NotificationPrivacyBetaRowClass,
};

/// Stable record kind for [`DesktopEntryEvent`] payloads.
pub const DESKTOP_ENTRY_EVENT_RECORD_KIND: &str = "desktop_entry_event_record";

/// Stable record kind for [`DesktopInterruptionRecoveryRow`] payloads.
pub const DESKTOP_INTERRUPTION_RECOVERY_ROW_RECORD_KIND: &str = "desktop_interruption_recovery_row";

/// Stable record kind for [`PlatformDesktopDrillRow`] payloads.
pub const PLATFORM_DESKTOP_DRILL_ROW_RECORD_KIND: &str = "platform_desktop_drill_row";

/// Stable record kind for [`DesktopSupportMatrixRow`] payloads.
pub const DESKTOP_SUPPORT_MATRIX_ROW_RECORD_KIND: &str = "desktop_support_matrix_row";

/// Stable record kind for [`NativeDesktopContractPacket`] payloads.
pub const NATIVE_DESKTOP_CONTRACT_PACKET_RECORD_KIND: &str = "native_desktop_contract_packet";

/// Schema version for desktop-entry event payloads.
pub const DESKTOP_ENTRY_EVENT_SCHEMA_VERSION: u32 = 1;

/// Schema version for native desktop contract packets.
pub const NATIVE_DESKTOP_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// One OS-originated desktop entry or reopen event after product admission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopEntryEvent {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for the event payload.
    pub schema_version: u32,
    /// Stable event id.
    pub event_id: String,
    /// OS-facing source surface token.
    pub source_surface_token: String,
    /// Origin class token supplied by the handoff validator.
    pub origin_class_token: String,
    /// Requested action class token after parsing.
    pub requested_action_class_token: String,
    /// Product route class token selected for the request.
    pub route_class_token: String,
    /// Literal target label supplied by the OS or platform surface.
    pub literal_target_label: String,
    /// Export-safe literal target ref.
    pub literal_target_ref: String,
    /// Resulting mode token after admission or recovery routing.
    pub resulting_mode_token: String,
    /// Canonical target identity ref or placeholder identity ref.
    pub canonical_target_ref: String,
    /// Target kind token.
    pub target_kind_token: String,
    /// Target availability token.
    pub availability_class_token: String,
    /// Target freshness token.
    pub freshness_class_token: String,
    /// Owning channel ref for handler or summary-surface ownership.
    pub owning_channel_ref: String,
    /// Owning build ref for handler or summary-surface ownership.
    pub owner_build_ref: String,
    /// Handler ownership token.
    pub handler_ownership_token: String,
    /// Trust/profile context shown before risky execution.
    pub trust_profile_context_ref: String,
    /// Policy epoch ref used for admission.
    pub policy_epoch_ref: String,
    /// Recovery or review surface token.
    pub recovery_surface_token: String,
    /// Safe recovery actions rendered when exact open is unavailable.
    pub recovery_action_tokens: Vec<String>,
    /// Privacy payload token for notification and badge surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_payload_class_token: Option<String>,
    /// True when a lock-screen or summary payload is redacted.
    pub lock_screen_payload_redacted: bool,
    /// True when the OS payload is a bounded summary only.
    pub notification_summary_bounded: bool,
    /// True when badge/progress counts are traceable to durable truth.
    pub badge_or_progress_count_traceable: bool,
    /// True when the OS surface is forbidden from executing directly.
    pub direct_os_execution_forbidden: bool,
    /// True when the event lands on placeholder recovery.
    pub placeholder_recovery_required: bool,
    /// True when authority widening requires in-product review.
    pub authority_widening_review_required: bool,
    /// True when the event cannot silently replay mutating work.
    pub no_silent_mutating_replay: bool,
    /// True when raw private material is excluded from the event.
    pub raw_private_material_excluded: bool,
    /// Source evidence refs used by support and conformance exports.
    pub source_evidence_refs: Vec<String>,
}

impl DesktopEntryEvent {
    /// Returns true when the event can only land on an exact target or a truthful placeholder.
    pub fn exact_target_or_truthful_placeholder(&self) -> bool {
        !self.canonical_target_ref.trim().is_empty()
            && (self.availability_class_token == "exact_available"
                || self.placeholder_recovery_required
                || self.recovery_surface_token.contains("placeholder"))
    }

    /// Returns true when OS summary surfaces keep privacy and reopen safety.
    pub fn summary_surface_safe(&self) -> bool {
        match self.source_surface_token.as_str() {
            "os_notification_click" | "os_badge_activation" => {
                self.raw_private_material_excluded
                    && self.lock_screen_payload_redacted
                    && self.notification_summary_bounded
                    && self.direct_os_execution_forbidden
                    && self.exact_target_or_truthful_placeholder()
            }
            "dock_taskbar_recent" | "dock_taskbar_jump_action" => {
                self.direct_os_execution_forbidden
                    && self.exact_target_or_truthful_placeholder()
                    && self.no_silent_mutating_replay
            }
            _ => true,
        }
    }
}

/// One recovery row for interruption or unavailable-target scenarios.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopInterruptionRecoveryRow {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Interruption class token.
    pub interruption_class_token: String,
    /// Affected object or target identity ref.
    pub affected_target_ref: String,
    /// Target availability token.
    pub availability_class_token: String,
    /// Continuity state tokens visible to the user and support exports.
    pub continuity_state_tokens: Vec<String>,
    /// Resulting mode token after recovery routing.
    pub resulting_mode_token: String,
    /// Safe recovery actions for the row.
    pub recovery_action_tokens: Vec<String>,
    /// True when a placeholder is rendered instead of pretending live state.
    pub placeholder_required: bool,
    /// True when local work remains available where possible.
    pub local_work_preserved: bool,
    /// True when privileged or mutating work is paused for review.
    pub privileged_or_mutating_work_paused: bool,
    /// True when no mutating work or stale authority can replay silently.
    pub no_silent_replay_or_authority_reuse: bool,
    /// Source evidence refs backing the row.
    pub source_evidence_refs: Vec<String>,
}

/// One per-platform drill row backing a desktop support claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformDesktopDrillRow {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Claimed platform profile id.
    pub platform_profile_id: String,
    /// Drill class token.
    pub drill_class_token: String,
    /// Source fixture or artifact ref.
    pub source_evidence_ref: String,
    /// Disclosure tokens the drill requires.
    pub required_disclosure_tokens: Vec<String>,
    /// True when the drill blocks last-writer-wins ownership.
    pub channel_precedence_explicit: bool,
    /// True when handler spoofing fails closed.
    pub handler_spoof_resistant: bool,
    /// True when recent or summary activation opens the exact object or placeholder.
    pub exact_reopen_or_placeholder: bool,
    /// True when lock-screen payloads are redacted or suppressed.
    pub lock_screen_redacted: bool,
    /// True when wake and resume state remains truthful.
    pub wake_resume_truthful: bool,
    /// True when privileged or mutating work cannot replay silently.
    pub no_silent_mutating_replay: bool,
    /// True when the drill has current proof.
    pub current_proof: bool,
}

/// One row in the beta desktop support matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopSupportMatrixRow {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Surface token covered by the row.
    pub surface_token: String,
    /// Claimed platform profile ids covered by the row.
    pub platform_profile_ids: Vec<String>,
    /// Current proof refs backing the row.
    pub proof_refs: Vec<String>,
    /// Support status token.
    pub support_status_token: String,
    /// True when proof is current for the row.
    pub current_proof: bool,
    /// True when activation lands on an exact target or truthful placeholder.
    pub exact_target_or_truthful_placeholder: bool,
    /// True when privacy posture is safe for external surfaces.
    pub privacy_safe: bool,
    /// True when hidden mutating replay is blocked.
    pub no_silent_replay: bool,
}

/// Aggregate beta packet for native desktop integration truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDesktopContractPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Running build identity ref.
    pub build_identity_ref: String,
    /// Export-safe generation timestamp.
    pub generated_at: String,
    /// Source packets and reports this packet composes.
    pub source_packet_refs: Vec<String>,
    /// Typed OS-entry and reopen events.
    pub desktop_entry_events: Vec<DesktopEntryEvent>,
    /// Recovery rows for interruption and unavailable-target scenarios.
    pub recovery_rows: Vec<DesktopInterruptionRecoveryRow>,
    /// Per-platform drill rows.
    pub platform_drills: Vec<PlatformDesktopDrillRow>,
    /// Beta desktop support matrix rows.
    pub support_matrix_rows: Vec<DesktopSupportMatrixRow>,
}

impl NativeDesktopContractPacket {
    /// Validates the beta desktop contract packet.
    pub fn validate(&self) -> Result<(), NativeDesktopContractValidationError> {
        if self.record_kind != NATIVE_DESKTOP_CONTRACT_PACKET_RECORD_KIND {
            return Err(NativeDesktopContractValidationError::InvalidRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        if self.schema_version != NATIVE_DESKTOP_CONTRACT_SCHEMA_VERSION {
            return Err(NativeDesktopContractValidationError::InvalidSchemaVersion {
                actual: self.schema_version,
            });
        }

        let event_surfaces = self
            .desktop_entry_events
            .iter()
            .map(|event| event.source_surface_token.as_str())
            .collect::<BTreeSet<_>>();
        for required in [
            "system_open",
            "file_association",
            "default_browser_callback",
            "protocol_handler",
            "dock_taskbar_recent",
            "dock_taskbar_jump_action",
            "os_notification_click",
            "os_badge_activation",
            "reveal_in_system_shell",
        ] {
            if !event_surfaces.contains(required) {
                return Err(NativeDesktopContractValidationError::MissingEntrySurface {
                    surface: required.to_owned(),
                });
            }
        }

        for event in &self.desktop_entry_events {
            if event.literal_target_label.trim().is_empty()
                || event.canonical_target_ref.trim().is_empty()
                || event.origin_class_token.trim().is_empty()
                || event.requested_action_class_token.trim().is_empty()
                || event.owning_channel_ref.trim().is_empty()
                || event.owner_build_ref.trim().is_empty()
                || event.trust_profile_context_ref.trim().is_empty()
                || event.policy_epoch_ref.trim().is_empty()
                || event.recovery_action_tokens.is_empty()
            {
                return Err(NativeDesktopContractValidationError::IncompleteEntryEvent {
                    event_id: event.event_id.clone(),
                });
            }
            if !event.exact_target_or_truthful_placeholder() {
                return Err(NativeDesktopContractValidationError::GenericReopen {
                    event_id: event.event_id.clone(),
                });
            }
            if !event.summary_surface_safe() {
                return Err(NativeDesktopContractValidationError::UnsafeSummarySurface {
                    event_id: event.event_id.clone(),
                });
            }
            if !event.no_silent_mutating_replay {
                return Err(NativeDesktopContractValidationError::SilentReplayAllowed {
                    ref_id: event.event_id.clone(),
                });
            }
        }

        let recovery_classes = self
            .recovery_rows
            .iter()
            .map(|row| row.interruption_class_token.as_str())
            .collect::<BTreeSet<_>>();
        for required in [
            "removable_volume_loss",
            "removable_volume_return",
            "network_share_unavailable",
            "missing_root",
            "credential_store_locked",
            "display_topology_drift",
            "wake_resume",
            "sleep_expired_callback",
            "network_transition",
        ] {
            if !recovery_classes.contains(required) {
                return Err(NativeDesktopContractValidationError::MissingRecoveryClass {
                    class_token: required.to_owned(),
                });
            }
        }
        for row in &self.recovery_rows {
            if row.recovery_action_tokens.is_empty() || !row.no_silent_replay_or_authority_reuse {
                return Err(NativeDesktopContractValidationError::SilentReplayAllowed {
                    ref_id: row.row_id.clone(),
                });
            }
        }

        let drill_profiles = self
            .platform_drills
            .iter()
            .map(|row| row.platform_profile_id.as_str())
            .collect::<BTreeSet<_>>();
        for required in [
            "macos_15_plus_universal",
            "windows_11_23h2_plus_x86_64",
            "linux_ubuntu_24_04_gnome_wayland_x86_64",
        ] {
            if !drill_profiles.contains(required) {
                return Err(NativeDesktopContractValidationError::MissingPlatformDrill {
                    platform_profile_id: required.to_owned(),
                });
            }
        }
        let drill_classes = self
            .platform_drills
            .iter()
            .map(|row| row.drill_class_token.as_str())
            .collect::<BTreeSet<_>>();
        for required in [
            "channel_precedence",
            "handler_spoof_resistance",
            "recent_reopen_fidelity",
            "lock_screen_redaction",
            "wake_resume_truth",
        ] {
            if !drill_classes.contains(required) {
                return Err(NativeDesktopContractValidationError::MissingPlatformDrill {
                    platform_profile_id: required.to_owned(),
                });
            }
        }
        if self.platform_drills.iter().any(|row| {
            !row.current_proof
                || !row.no_silent_mutating_replay
                || (!row.channel_precedence_explicit
                    && !row.handler_spoof_resistant
                    && !row.exact_reopen_or_placeholder
                    && !row.lock_screen_redacted
                    && !row.wake_resume_truthful)
        }) {
            return Err(NativeDesktopContractValidationError::DrillProofMissing);
        }

        let support_surfaces = self
            .support_matrix_rows
            .iter()
            .map(|row| row.surface_token.as_str())
            .collect::<BTreeSet<_>>();
        for required in [
            "system_open",
            "auth_callbacks",
            "file_associations",
            "recent_items",
            "privacy_safe_native_notifications",
        ] {
            if !support_surfaces.contains(required) {
                return Err(
                    NativeDesktopContractValidationError::MissingSupportSurface {
                        surface: required.to_owned(),
                    },
                );
            }
        }
        for row in &self.support_matrix_rows {
            if !row.current_proof
                || !row.exact_target_or_truthful_placeholder
                || !row.privacy_safe
                || !row.no_silent_replay
                || row.proof_refs.is_empty()
            {
                return Err(NativeDesktopContractValidationError::SupportProofMissing {
                    surface: row.surface_token.clone(),
                });
            }
        }

        Ok(())
    }
}

/// Validation error for the native desktop contract packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeDesktopContractValidationError {
    /// The packet carried an unexpected record kind.
    InvalidRecordKind {
        /// Actual record kind in the packet.
        actual: String,
    },
    /// The packet carried an unexpected schema version.
    InvalidSchemaVersion {
        /// Actual schema version in the packet.
        actual: u32,
    },
    /// A required entry surface is missing.
    MissingEntrySurface {
        /// Missing surface token.
        surface: String,
    },
    /// A required entry event omitted required truth.
    IncompleteEntryEvent {
        /// Event id that failed validation.
        event_id: String,
    },
    /// An entry event can reopen a generic or untruthful target.
    GenericReopen {
        /// Event id that failed validation.
        event_id: String,
    },
    /// A summary surface violates privacy or reopen safety.
    UnsafeSummarySurface {
        /// Event id that failed validation.
        event_id: String,
    },
    /// A required recovery class is missing.
    MissingRecoveryClass {
        /// Missing recovery class token.
        class_token: String,
    },
    /// A path permits hidden mutating replay or stale authority reuse.
    SilentReplayAllowed {
        /// Row or event id that failed validation.
        ref_id: String,
    },
    /// A required platform drill is missing.
    MissingPlatformDrill {
        /// Platform profile id or drill class token that is missing.
        platform_profile_id: String,
    },
    /// One or more platform drills do not carry current proof.
    DrillProofMissing,
    /// A required support matrix surface is missing.
    MissingSupportSurface {
        /// Missing support surface token.
        surface: String,
    },
    /// A support matrix row lacks current proof.
    SupportProofMissing {
        /// Surface token that failed validation.
        surface: String,
    },
}

impl std::fmt::Display for NativeDesktopContractValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRecordKind { actual } => {
                write!(
                    f,
                    "native desktop contract has invalid record kind {actual}"
                )
            }
            Self::InvalidSchemaVersion { actual } => write!(
                f,
                "native desktop contract has invalid schema version {actual}"
            ),
            Self::MissingEntrySurface { surface } => {
                write!(f, "native desktop contract is missing {surface}")
            }
            Self::IncompleteEntryEvent { event_id } => {
                write!(f, "desktop entry event {event_id} omitted required truth")
            }
            Self::GenericReopen { event_id } => {
                write!(
                    f,
                    "desktop entry event {event_id} can reopen a generic target"
                )
            }
            Self::UnsafeSummarySurface { event_id } => {
                write!(f, "desktop summary event {event_id} is unsafe")
            }
            Self::MissingRecoveryClass { class_token } => {
                write!(
                    f,
                    "native desktop contract is missing recovery class {class_token}"
                )
            }
            Self::SilentReplayAllowed { ref_id } => {
                write!(f, "native desktop row {ref_id} allows hidden replay")
            }
            Self::MissingPlatformDrill {
                platform_profile_id,
            } => write!(
                f,
                "native desktop contract is missing platform drill {platform_profile_id}"
            ),
            Self::DrillProofMissing => write!(
                f,
                "native desktop contract has a platform drill without current proof"
            ),
            Self::MissingSupportSurface { surface } => {
                write!(f, "native desktop support matrix is missing {surface}")
            }
            Self::SupportProofMissing { surface } => {
                write!(f, "native desktop support matrix row {surface} lacks proof")
            }
        }
    }
}

impl std::error::Error for NativeDesktopContractValidationError {}

/// Builds the seeded beta native desktop contract packet.
pub fn seeded_native_desktop_contract_packet(
    build_identity_ref: impl Into<String>,
) -> NativeDesktopContractPacket {
    seeded_native_desktop_contract_packet_with_time(build_identity_ref, now_rfc3339())
}

/// Builds the seeded beta native desktop contract packet with a stable timestamp.
pub fn seeded_native_desktop_contract_packet_with_time(
    build_identity_ref: impl Into<String>,
    generated_at: impl Into<String>,
) -> NativeDesktopContractPacket {
    let build_identity_ref = build_identity_ref.into();
    let generated_at = generated_at.into();
    let native_packet = seeded_native_boundary_handoff_packet(build_identity_ref.clone());
    let continuity_packet = seeded_desktop_continuity_alpha_packet(build_identity_ref.clone());
    let notification_page = seeded_notification_privacy_beta_page();
    validate_notification_privacy_beta_page(&notification_page)
        .expect("seeded notification beta page must validate");

    let mut events = desktop_entry_events(&native_packet, &notification_page, &build_identity_ref);
    events.sort_by(|left, right| left.event_id.cmp(&right.event_id));

    NativeDesktopContractPacket {
        record_kind: NATIVE_DESKTOP_CONTRACT_PACKET_RECORD_KIND.to_owned(),
        schema_version: NATIVE_DESKTOP_CONTRACT_SCHEMA_VERSION,
        packet_id: "native-desktop-contract:beta:default".to_owned(),
        build_identity_ref: build_identity_ref.clone(),
        generated_at,
        source_packet_refs: vec![
            native_packet.packet_id.clone(),
            continuity_packet.packet_id.clone(),
            notification_page.page_id.clone(),
            "install:ownership_audit:v1".to_owned(),
            "artifacts/platform/desktop_summary_surface_matrix.yaml".to_owned(),
            "artifacts/platform/native_lifecycle_drill_packet.md".to_owned(),
        ],
        desktop_entry_events: events,
        recovery_rows: recovery_rows(&continuity_packet),
        platform_drills: platform_drill_rows(),
        support_matrix_rows: support_matrix_rows(),
    }
}

fn desktop_entry_events(
    native_packet: &NativeBoundaryHandoffPacket,
    notification_page: &NotificationPrivacyBetaPage,
    build_identity_ref: &str,
) -> Vec<DesktopEntryEvent> {
    let mut events = [
        SourceSurfaceClass::SystemOpen,
        SourceSurfaceClass::FileAssociation,
        SourceSurfaceClass::DefaultBrowserCallback,
        SourceSurfaceClass::DockTaskbarRecent,
        SourceSurfaceClass::DockTaskbarJumpAction,
    ]
    .into_iter()
    .filter_map(|source| {
        native_packet
            .handoff_for_source(source)
            .map(|review| event_from_handoff(source, review, build_identity_ref))
    })
    .collect::<Vec<_>>();

    if let Some(review) = native_packet.handoff_reviews.iter().find(|review| {
        review.source_surface_token == SourceSurfaceClass::ProtocolHandler.as_str()
            && review.requested_action_token == "privileged_authority_widening"
    }) {
        events.push(event_from_handoff(
            SourceSurfaceClass::ProtocolHandler,
            review,
            build_identity_ref,
        ));
    }

    if let Some(review) =
        native_packet.native_file_review_for(NativeFileAffordanceClass::RevealInSystemShell)
    {
        events.push(event_from_native_file_review(review, build_identity_ref));
    }

    let delivered = notification_page
        .rows
        .iter()
        .find(|row| row.row_class == NotificationPrivacyBetaRowClass::DeliveredSummarySafe)
        .expect("seeded page has delivered notification row");
    events.push(event_from_notification_row(
        "os_notification_click",
        "open_existing_context",
        delivered,
        build_identity_ref,
    ));

    let lock_screen = notification_page
        .rows
        .iter()
        .find(|row| row.row_class == NotificationPrivacyBetaRowClass::LockScreenSafeGenericPayload)
        .expect("seeded page has lock-screen-safe notification row");
    events.push(event_from_notification_row(
        "os_badge_activation",
        "inspect_only",
        lock_screen,
        build_identity_ref,
    ));

    events
}

fn event_from_handoff(
    source: SourceSurfaceClass,
    review: &NativeBoundaryHandoffReviewRecord,
    build_identity_ref: &str,
) -> DesktopEntryEvent {
    let literal = literal_target_for(source, review);
    let placeholder = review.placeholder_recovery_required;
    let high_risk = review.authority_delta_token != "none"
        || matches!(
            review.requested_action_token.as_str(),
            "create_or_add_context"
                | "join_presence"
                | "resume_session"
                | "mutating_command_request"
                | "privileged_authority_widening"
        );
    let mut recovery_actions = review
        .recovery_actions
        .iter()
        .map(|action| action.action_token.clone())
        .collect::<Vec<_>>();
    if recovery_actions.is_empty() {
        recovery_actions.push("open_bound_target".to_owned());
    }

    DesktopEntryEvent {
        record_kind: DESKTOP_ENTRY_EVENT_RECORD_KIND.to_owned(),
        schema_version: DESKTOP_ENTRY_EVENT_SCHEMA_VERSION,
        event_id: format!("desktop-entry-event:{}", review.source_surface_token),
        source_surface_token: review.source_surface_token.clone(),
        origin_class_token: review.origin_class_token.clone(),
        requested_action_class_token: review.requested_action_token.clone(),
        route_class_token: review.route_class_token.clone(),
        literal_target_label: literal.label,
        literal_target_ref: literal.ref_id,
        resulting_mode_token: if placeholder {
            "truthful_placeholder".to_owned()
        } else {
            review.route_class_token.clone()
        },
        canonical_target_ref: review.target.object_identity_ref.clone(),
        target_kind_token: review.target.target_kind_token.clone(),
        availability_class_token: review.target.availability_class_token.clone(),
        freshness_class_token: review.target.freshness_class_token.clone(),
        owning_channel_ref: review
            .owning_channel_ref
            .clone()
            .unwrap_or_else(|| "channel:stable".to_owned()),
        owner_build_ref: review
            .owner_build_ref
            .clone()
            .unwrap_or_else(|| build_identity_ref.to_owned()),
        handler_ownership_token: review.handler_ownership_token.clone(),
        trust_profile_context_ref: trust_profile_context(review),
        policy_epoch_ref: review.policy_epoch_ref.clone(),
        recovery_surface_token: review.review_surface_token.clone(),
        recovery_action_tokens: recovery_actions,
        privacy_payload_class_token: None,
        lock_screen_payload_redacted: true,
        notification_summary_bounded: true,
        badge_or_progress_count_traceable: false,
        direct_os_execution_forbidden: review.direct_os_execution_forbidden
            || source.is_summary_only(),
        placeholder_recovery_required: placeholder,
        authority_widening_review_required: high_risk,
        no_silent_mutating_replay: true,
        raw_private_material_excluded: true,
        source_evidence_refs: vec![review.review_id.clone()],
    }
}

fn event_from_native_file_review(
    review: &NativeFileHandoffReviewRecord,
    build_identity_ref: &str,
) -> DesktopEntryEvent {
    DesktopEntryEvent {
        record_kind: DESKTOP_ENTRY_EVENT_RECORD_KIND.to_owned(),
        schema_version: DESKTOP_ENTRY_EVENT_SCHEMA_VERSION,
        event_id: "desktop-entry-event:reveal_in_system_shell".to_owned(),
        source_surface_token: "reveal_in_system_shell".to_owned(),
        origin_class_token: "os_shell".to_owned(),
        requested_action_class_token: "reveal_only".to_owned(),
        route_class_token: "local_file_open".to_owned(),
        literal_target_label: review.literal_target_label.clone(),
        literal_target_ref: format!("literal:{}", review.review_id),
        resulting_mode_token: "reveal_only".to_owned(),
        canonical_target_ref: review.object_identity_ref.clone(),
        target_kind_token: review.target_kind_token.clone(),
        availability_class_token: review.availability_class_token.clone(),
        freshness_class_token: "authoritative_live".to_owned(),
        owning_channel_ref: "channel:stable".to_owned(),
        owner_build_ref: build_identity_ref.to_owned(),
        handler_ownership_token: "current_user_registered".to_owned(),
        trust_profile_context_ref: format!("{}:scope:local-file", review.trust_state_token),
        policy_epoch_ref: "pe:platform:reveal-in-shell:01".to_owned(),
        recovery_surface_token: review.review_surface_token.clone(),
        recovery_action_tokens: review
            .recovery_actions
            .iter()
            .map(|action| action.action_token.clone())
            .collect(),
        privacy_payload_class_token: None,
        lock_screen_payload_redacted: true,
        notification_summary_bounded: true,
        badge_or_progress_count_traceable: false,
        direct_os_execution_forbidden: true,
        placeholder_recovery_required: false,
        authority_widening_review_required: false,
        no_silent_mutating_replay: true,
        raw_private_material_excluded: true,
        source_evidence_refs: vec![review.review_id.clone()],
    }
}

fn event_from_notification_row(
    source_surface_token: &str,
    requested_action_token: &str,
    row: &NotificationPrivacyBetaRow,
    build_identity_ref: &str,
) -> DesktopEntryEvent {
    let exact_target = row
        .reopen_target
        .exact_target_identity_ref
        .clone()
        .unwrap_or_else(|| row.reopen_target.reopen_target_ref.clone());
    let placeholder = row.reopen_target.exact_target_identity_ref.is_none();
    DesktopEntryEvent {
        record_kind: DESKTOP_ENTRY_EVENT_RECORD_KIND.to_owned(),
        schema_version: DESKTOP_ENTRY_EVENT_SCHEMA_VERSION,
        event_id: format!("desktop-entry-event:{source_surface_token}:{}", row.case_id),
        source_surface_token: source_surface_token.to_owned(),
        origin_class_token: "os_shell".to_owned(),
        requested_action_class_token: requested_action_token.to_owned(),
        route_class_token: "notification_reopen".to_owned(),
        literal_target_label: row.notification_envelope_id.clone(),
        literal_target_ref: row.reopen_target.reopen_target_ref.clone(),
        resulting_mode_token: if placeholder {
            "truthful_placeholder".to_owned()
        } else {
            "durable_truth_reopen".to_owned()
        },
        canonical_target_ref: exact_target,
        target_kind_token: enum_token(&row.reopen_target.reopen_target_kind),
        availability_class_token: if placeholder {
            "unknown".to_owned()
        } else {
            "exact_available".to_owned()
        },
        freshness_class_token: if placeholder {
            "unverified".to_owned()
        } else {
            "authoritative_live".to_owned()
        },
        owning_channel_ref: "channel:stable".to_owned(),
        owner_build_ref: build_identity_ref.to_owned(),
        handler_ownership_token: "current_user_registered".to_owned(),
        trust_profile_context_ref: "trusted:scope:notification-lineage".to_owned(),
        policy_epoch_ref: "pe:notification-privacy-beta:01".to_owned(),
        recovery_surface_token: if placeholder {
            "placeholder_recovery_card".to_owned()
        } else {
            "exact_reopen".to_owned()
        },
        recovery_action_tokens: if placeholder {
            vec![
                "open_activity_center".to_owned(),
                "open_placeholder".to_owned(),
                "export_notification_metadata".to_owned(),
            ]
        } else {
            vec![
                "open_in_product_detail".to_owned(),
                "open_activity_center".to_owned(),
                "export_notification_metadata".to_owned(),
            ]
        },
        privacy_payload_class_token: Some(enum_token(&row.privacy_payload_class)),
        lock_screen_payload_redacted: row.raw_private_material_excluded,
        notification_summary_bounded: true,
        badge_or_progress_count_traceable: true,
        direct_os_execution_forbidden: true,
        placeholder_recovery_required: placeholder,
        authority_widening_review_required: false,
        no_silent_mutating_replay: true,
        raw_private_material_excluded: row.raw_private_material_excluded,
        source_evidence_refs: vec![row.row_id.clone(), row.notification_envelope_id.clone()],
    }
}

fn recovery_rows(packet: &DesktopContinuityAlphaPacket) -> Vec<DesktopInterruptionRecoveryRow> {
    let mut rows = Vec::new();

    for lifecycle in &packet.lifecycle_rows {
        let class = match lifecycle.lifecycle_event_token.as_str() {
            "wake_from_sleep" => "wake_resume",
            other => other,
        };
        rows.push(DesktopInterruptionRecoveryRow {
            record_kind: DESKTOP_INTERRUPTION_RECOVERY_ROW_RECORD_KIND.to_owned(),
            schema_version: NATIVE_DESKTOP_CONTRACT_SCHEMA_VERSION,
            row_id: format!("desktop-recovery:{class}"),
            interruption_class_token: class.to_owned(),
            affected_target_ref: lifecycle.object_identity_ref.clone(),
            availability_class_token: if lifecycle
                .continuity_state_tokens
                .iter()
                .any(|token| token == "root_unavailable")
            {
                "missing_or_unmounted".to_owned()
            } else if lifecycle
                .continuity_state_tokens
                .iter()
                .any(|token| token == "local_fallback")
            {
                "remote_unreachable".to_owned()
            } else {
                "stale_available".to_owned()
            },
            continuity_state_tokens: lifecycle.continuity_state_tokens.clone(),
            resulting_mode_token: lifecycle.resulting_fidelity_token.clone(),
            recovery_action_tokens: lifecycle.recovery_action_tokens.clone(),
            placeholder_required: true,
            local_work_preserved: lifecycle.local_work_continues,
            privileged_or_mutating_work_paused: lifecycle.privileged_or_mutating_work_paused,
            no_silent_replay_or_authority_reuse: lifecycle.no_silent_rerun_or_authority_reuse,
            source_evidence_refs: vec![lifecycle.source_fixture_ref.clone()],
        });
    }

    for topology in &packet.topology_rows {
        rows.push(DesktopInterruptionRecoveryRow {
            record_kind: DESKTOP_INTERRUPTION_RECOVERY_ROW_RECORD_KIND.to_owned(),
            schema_version: NATIVE_DESKTOP_CONTRACT_SCHEMA_VERSION,
            row_id: format!("desktop-recovery:{}", topology.event_class_token),
            interruption_class_token: "display_topology_drift".to_owned(),
            affected_target_ref: topology.display_context_token.clone(),
            availability_class_token: "stale_available".to_owned(),
            continuity_state_tokens: topology.continuity_state_tokens.clone(),
            resulting_mode_token: topology.resulting_fidelity_token.clone(),
            recovery_action_tokens: topology.recovery_action_tokens.clone(),
            placeholder_required: false,
            local_work_preserved: true,
            privileged_or_mutating_work_paused: true,
            no_silent_replay_or_authority_reuse: true,
            source_evidence_refs: vec![topology.source_fixture_ref.clone()],
        });
    }

    if let Some(row) = packet
        .credential_store_rows
        .iter()
        .find(|row| row.continuity_state_token == "paused_credential_store_locked")
    {
        rows.push(DesktopInterruptionRecoveryRow {
            record_kind: DESKTOP_INTERRUPTION_RECOVERY_ROW_RECORD_KIND.to_owned(),
            schema_version: NATIVE_DESKTOP_CONTRACT_SCHEMA_VERSION,
            row_id: "desktop-recovery:credential-store-locked".to_owned(),
            interruption_class_token: "credential_store_locked".to_owned(),
            affected_target_ref: row.secret_broker_row_ref.clone(),
            availability_class_token: "auth_required".to_owned(),
            continuity_state_tokens: vec![
                row.continuity_state_token.clone(),
                row.unlock_state_token.clone(),
                row.local_continuation_token.clone(),
            ],
            resulting_mode_token: "local_only_fallback".to_owned(),
            recovery_action_tokens: row.recovery_action_tokens.clone(),
            placeholder_required: false,
            local_work_preserved: row.local_work_continues,
            privileged_or_mutating_work_paused: row.credentialed_actions_paused,
            no_silent_replay_or_authority_reuse: row.stale_authority_reuse_forbidden,
            source_evidence_refs: vec![row.source_support_export_ref.clone()],
        });
    }

    rows.push(static_recovery_row(
        "network_share_unavailable",
        "obj:network-share:server-share:app-ts:01",
        "remote_unreachable",
        &["reconnecting", "cached_context_available"],
        &[
            "locate_target",
            "open_read_only_cached_view",
            "close_placeholder",
        ],
        "fixtures/platform/network_share_alias_cases/disconnected_network_share_placeholder_locate_or_cached.yaml",
    ));
    rows.push(static_recovery_row(
        "missing_root",
        "obj:workspace-manifest:payments:01",
        "missing_or_unmounted",
        &["root_unavailable", "stale", "cached_context_available"],
        &["locate_target", "open_cached_context", "remove_root"],
        "fixtures/platform/exact_target_reopen_cases/workspace_open_missing_target_denied.yaml",
    ));
    rows.push(static_recovery_row(
        "removable_volume_return",
        "obj:workspace-root:client-drive:payments:01",
        "stale_available",
        &["local_fallback", "resume_review_needed", "stale"],
        &[
            "review_returned_root",
            "open_cached_context",
            "locate_target",
            "continue_local",
        ],
        "fixtures/platform/native_lifecycle_cases/removable_volume_return_review_required.yaml",
    ));
    rows.push(static_recovery_row(
        "sleep_expired_callback",
        "obj:auth-session:tenant-payments:01",
        "expired",
        &["reopen_required", "expired_session"],
        &["reauthenticate", "continue_local", "export_restore_details"],
        "fixtures/platform/native_lifecycle_cases/expired_callback_after_sleep_reopen_required.yaml",
    ));

    rows
}

fn static_recovery_row(
    class_token: &str,
    target_ref: &str,
    availability: &str,
    state_tokens: &[&str],
    action_tokens: &[&str],
    source_ref: &str,
) -> DesktopInterruptionRecoveryRow {
    DesktopInterruptionRecoveryRow {
        record_kind: DESKTOP_INTERRUPTION_RECOVERY_ROW_RECORD_KIND.to_owned(),
        schema_version: NATIVE_DESKTOP_CONTRACT_SCHEMA_VERSION,
        row_id: format!("desktop-recovery:{class_token}"),
        interruption_class_token: class_token.to_owned(),
        affected_target_ref: target_ref.to_owned(),
        availability_class_token: availability.to_owned(),
        continuity_state_tokens: state_tokens
            .iter()
            .map(|token| (*token).to_owned())
            .collect(),
        resulting_mode_token: "truthful_placeholder".to_owned(),
        recovery_action_tokens: action_tokens
            .iter()
            .map(|token| (*token).to_owned())
            .collect(),
        placeholder_required: true,
        local_work_preserved: true,
        privileged_or_mutating_work_paused: true,
        no_silent_replay_or_authority_reuse: true,
        source_evidence_refs: vec![source_ref.to_owned()],
    }
}

fn platform_drill_rows() -> Vec<PlatformDesktopDrillRow> {
    let profiles = [
        (
            "macos_15_plus_universal",
            "fixtures/platform/desktop_summary_surface_cases/macos_dock_summary_surfaces.yaml",
        ),
        (
            "windows_11_23h2_plus_x86_64",
            "fixtures/platform/desktop_summary_surface_cases/windows_taskbar_jump_list_summary_surfaces.yaml",
        ),
        (
            "linux_ubuntu_24_04_gnome_wayland_x86_64",
            "fixtures/platform/desktop_summary_surface_cases/linux_gnome_launcher_summary_surfaces_degraded.yaml",
        ),
    ];
    let drills: [(&str, &str, &[&str]); 5] = [
        (
            "channel_precedence",
            "artifacts/platform/recent_item_and_protocol_ownership_audit.md",
            &["owning_channel_ref", "owner_build_ref", "no_last_writer_wins"],
        ),
        (
            "handler_spoof_resistance",
            "fixtures/platform/deep_link_replay_cases/browser_return_replay_denied_origin_mismatch.yaml",
            &["origin", "handler_owner", "policy_epoch"],
        ),
        (
            "recent_reopen_fidelity",
            "fixtures/platform/exact_target_reopen_cases/workspace_open_missing_target_denied.yaml",
            &["target_identity", "freshness_class", "fallback"],
        ),
        (
            "lock_screen_redaction",
            "fixtures/ux/m3/notification_privacy/rows.json",
            &["privacy_payload_class", "redaction_class", "exact_reopen_target"],
        ),
        (
            "wake_resume_truth",
            "fixtures/platform/native_lifecycle_cases/wake_from_sleep_local_context_preserved.yaml",
            &["lifecycle_state", "revalidation", "no_silent_replay"],
        ),
    ];

    profiles
        .iter()
        .flat_map(|(profile, profile_ref)| {
            drills
                .iter()
                .map(move |(class, evidence_ref, disclosures)| {
                    let source_ref = if *class == "channel_precedence" {
                        (*evidence_ref).to_owned()
                    } else if *class == "lock_screen_redaction" || *class == "wake_resume_truth" {
                        (*evidence_ref).to_owned()
                    } else {
                        format!("{profile_ref}; {evidence_ref}")
                    };
                    PlatformDesktopDrillRow {
                        record_kind: PLATFORM_DESKTOP_DRILL_ROW_RECORD_KIND.to_owned(),
                        schema_version: NATIVE_DESKTOP_CONTRACT_SCHEMA_VERSION,
                        row_id: format!("platform-drill:{profile}:{class}"),
                        platform_profile_id: (*profile).to_owned(),
                        drill_class_token: (*class).to_owned(),
                        source_evidence_ref: source_ref,
                        required_disclosure_tokens: disclosures
                            .iter()
                            .map(|token| (*token).to_owned())
                            .collect(),
                        channel_precedence_explicit: *class == "channel_precedence",
                        handler_spoof_resistant: *class == "handler_spoof_resistance",
                        exact_reopen_or_placeholder: *class == "recent_reopen_fidelity",
                        lock_screen_redacted: *class == "lock_screen_redaction",
                        wake_resume_truthful: *class == "wake_resume_truth",
                        no_silent_mutating_replay: true,
                        current_proof: true,
                    }
                })
        })
        .collect()
}

fn support_matrix_rows() -> Vec<DesktopSupportMatrixRow> {
    let profiles = vec![
        "macos_15_plus_universal".to_owned(),
        "windows_11_23h2_plus_x86_64".to_owned(),
        "linux_ubuntu_24_04_gnome_wayland_x86_64".to_owned(),
    ];
    [
        (
            "system_open",
            vec![
                "crates/aureline-shell/src/deeplink/native_handoff.rs",
                "fixtures/platform/exact_target_reopen_cases/local_file_open_admitted_exact.yaml",
            ],
        ),
        (
            "auth_callbacks",
            vec![
                "docs/auth/m3/system_browser_and_passkey_beta.md",
                "fixtures/platform/exact_target_reopen_cases/auth_callback_replay_denied_consumed.yaml",
            ],
        ),
        (
            "file_associations",
            vec![
                "artifacts/platform/file_association_ownership_matrix.yaml",
                "fixtures/platform/native_file_affordance_cases/local_file_native_open_dialog_binds_identity.yaml",
            ],
        ),
        (
            "recent_items",
            vec![
                "docs/platform/desktop_summary_surface_matrix.md",
                "fixtures/platform/exact_target_reopen_cases/workspace_open_missing_target_denied.yaml",
            ],
        ),
        (
            "privacy_safe_native_notifications",
            vec![
                "fixtures/ux/m3/notification_privacy/page.json",
                "artifacts/platform/lock_screen_privacy_rows.yaml",
            ],
        ),
        (
            "wake_resume_and_missing_roots",
            vec![
                "artifacts/platform/native_lifecycle_drill_packet.md",
                "fixtures/platform/native_lifecycle_cases/removable_volume_loss_root_unavailable.yaml",
            ],
        ),
    ]
    .into_iter()
    .map(|(surface, proof_refs)| DesktopSupportMatrixRow {
        record_kind: DESKTOP_SUPPORT_MATRIX_ROW_RECORD_KIND.to_owned(),
        schema_version: NATIVE_DESKTOP_CONTRACT_SCHEMA_VERSION,
        row_id: format!("desktop-support-matrix:{surface}"),
        surface_token: surface.to_owned(),
        platform_profile_ids: profiles.clone(),
        proof_refs: proof_refs.into_iter().map(str::to_owned).collect(),
        support_status_token: "beta_current_proof".to_owned(),
        current_proof: true,
        exact_target_or_truthful_placeholder: true,
        privacy_safe: true,
        no_silent_replay: true,
    })
    .collect()
}

#[derive(Debug)]
struct LiteralTarget {
    label: String,
    ref_id: String,
}

fn literal_target_for(
    source: SourceSurfaceClass,
    review: &NativeBoundaryHandoffReviewRecord,
) -> LiteralTarget {
    let (label, ref_id) = match source {
        SourceSurfaceClass::SystemOpen => (
            "/Users/dev/workspaces/alpha-local".to_owned(),
            "literal:system-open:workspace-alpha-local".to_owned(),
        ),
        SourceSurfaceClass::FileAssociation => (
            "/Users/dev/src/demo/src/main.ts".to_owned(),
            "literal:file-association:demo-src-main".to_owned(),
        ),
        SourceSurfaceClass::DefaultBrowserCallback => (
            "aureline://auth/callback?session=obj:auth-session:oidc:01".to_owned(),
            "literal:auth-callback:oidc-session".to_owned(),
        ),
        SourceSurfaceClass::ProtocolHandler => (
            "aureline://command/trust-review?target=obj:command:trust-elevation:alpha".to_owned(),
            "literal:protocol-handler:trust-review".to_owned(),
        ),
        SourceSurfaceClass::DockTaskbarRecent => (
            "dock/taskbar recent: payments workspace".to_owned(),
            "literal:dock-taskbar-recent:payments-workspace".to_owned(),
        ),
        SourceSurfaceClass::DockTaskbarJumpAction => (
            "dock/taskbar jump: reopen alpha workspace".to_owned(),
            "literal:dock-taskbar-jump:alpha-workspace".to_owned(),
        ),
        _ => (
            review.target.object_identity_ref.clone(),
            format!("literal:{}", review.target.object_identity_ref),
        ),
    };
    LiteralTarget { label, ref_id }
}

fn trust_profile_context(review: &NativeBoundaryHandoffReviewRecord) -> String {
    format!(
        "{}:{}",
        review.trust_state_token,
        review
            .tenant_or_workspace_scope_ref
            .as_deref()
            .unwrap_or("scope:unbound")
    )
}

fn enum_token<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_else(|| "unknown".to_owned())
}
