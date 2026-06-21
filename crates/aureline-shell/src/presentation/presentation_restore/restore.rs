//! Restore checkpoints, layout fidelity, and honest crash / interrupted-resume
//! recovery for presentation and walkthrough sessions.
//!
//! Entering presentation mode checkpoints the prior layout and selection context
//! first (see [`RestoreCheckpoint`]); every terminal transition — clean exit,
//! cancel, crash recovery, and *interrupted session resume* — replays that
//! checkpoint so the user lands back where they were rather than in an
//! improvised shell. This module is the governed restore packet the spec calls
//! for: it makes that restore an **inspectable, support-safe truth record**
//! whose fidelity is classified with the same vocabulary durable shell contexts
//! use, and whose degradation is *always surfaced* rather than hidden behind a
//! generic success message.
//!
//! The contracts this row exists to hold:
//!
//! - **Layout fidelity is visible.** A [`PresentationRestoreReport`] carries a
//!   [`PresentationRestoreClass`] (exact / compatible / layout-only /
//!   evidence-only / no-restore) that mirrors the durable-shell restore
//!   vocabulary one-to-one, so reviewers and support read presentation restore
//!   fidelity exactly as they read window-session restore fidelity.
//! - **Degradation is honest.** When a waypoint's target is gone — a missing
//!   dependency, a revoked sharing grant, an unavailable remote target, or an
//!   expired authority — the waypoint degrades to an honest
//!   [`WaypointAvailability::Placeholder`] or
//!   [`WaypointAvailability::Disconnected`] state with a named
//!   [`RestoreDegradeTrigger`]. It is never silently re-run or re-acquired, and
//!   the problem is never folded into a generic "restored" banner.
//! - **No hidden re-run or re-authority.** Restore replays *layout and
//!   attention only*. Every state carries `replayed_mutating_action = false` and
//!   `reacquired_authority = false`, and the report's aggregate guardrails prove
//!   the same at the session level, so an expired privileged flow stays expired.
//!
//! The fidelity vocabulary maps onto the canonical durable-shell restore classes
//! via [`PresentationRestoreClass::to_durable_restore_class`] and
//! [`RestoreDegradeTrigger::to_durable_downgrade_trigger`], so the presentation
//! lane stays a thin, parity-checked layer over the existing restore model in
//! [`aureline_recovery::session_restore`] rather than a second vocabulary.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use aureline_recovery::session_restore::records::{DowngradeTriggerClass, RestoreClass};

use crate::presentation_mode::{
    BoundaryLabel, PresentationSession, WalkthroughSurfaceKind,
    PRESENTATION_MODE_BETA_SCHEMA_VERSION, PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
};

/// Stable record kind for [`PresentationRestoreReport`] payloads.
pub const PRESENTATION_RESTORE_REPORT_RECORD_KIND: &str = "presentation_restore_report_record";

/// Stable record kind for [`WaypointRestoreState`] payloads.
pub const PRESENTATION_WAYPOINT_RESTORE_RECORD_KIND: &str = "presentation_waypoint_restore_record";

/// Stable record kind for [`PresentationRestoreSupportExport`] payloads.
pub const PRESENTATION_RESTORE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "presentation_restore_support_export_record";

/// Stable record kind for [`PresentationRestoreSupportExportRow`] payloads.
pub const PRESENTATION_RESTORE_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "presentation_restore_support_export_row_record";

/// The human-readable restore / recovery contract this module implements.
pub const PRESENTATION_RESTORE_AND_RECOVERY_DOC_REF: &str =
    "docs/help/presentation-restore-and-recovery.md";

/// The restore / crash coverage matrix this module's corpus backs.
pub const PRESENTATION_RESTORE_AND_CRASH_MATRIX_REF: &str =
    "artifacts/presentation/restore-and-crash-matrix.md";

/// Directory holding the checked-in no-rerun restore fixtures.
pub const PRESENTATION_RESTORE_FIXTURE_DIR: &str = "fixtures/presentation/restore-no-rerun";

/// What triggered a presentation restore.
///
/// Every trigger replays the same checkpoint, so the restored layout is
/// identical regardless of how the session ended. `InterruptedResume` covers a
/// session that was rehydrated after the shell was interrupted mid-presentation
/// (distinct from a clean crash-recovery boot).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationRestoreTrigger {
    /// The user exited presentation mode cleanly.
    Exit,
    /// The user cancelled while entering or mid-session.
    Cancel,
    /// Crash recovery rehydrated the session and restored the prior layout.
    CrashRecovery,
    /// An interrupted session was resumed and the prior layout was restored.
    InterruptedResume,
}

impl PresentationRestoreTrigger {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exit => "exit",
            Self::Cancel => "cancel",
            Self::CrashRecovery => "crash_recovery",
            Self::InterruptedResume => "interrupted_resume",
        }
    }

    /// The lifecycle state a session lands in after this trigger.
    pub const fn restored_lifecycle(self) -> PresentationRestoreLifecycle {
        match self {
            Self::Exit => PresentationRestoreLifecycle::ExitedRestored,
            Self::Cancel => PresentationRestoreLifecycle::CancelledRestored,
            Self::CrashRecovery => PresentationRestoreLifecycle::CrashRecoveredRestored,
            Self::InterruptedResume => PresentationRestoreLifecycle::ResumedRestored,
        }
    }
}

/// Where a session sits after a restore. Each value is the terminal lifecycle
/// for one [`PresentationRestoreTrigger`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationRestoreLifecycle {
    /// The user exited cleanly; the prior layout was restored.
    ExitedRestored,
    /// The user cancelled; the prior layout was restored.
    CancelledRestored,
    /// Crash recovery restored the prior layout from the checkpoint.
    CrashRecoveredRestored,
    /// An interrupted session was resumed and the prior layout restored.
    ResumedRestored,
}

impl PresentationRestoreLifecycle {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExitedRestored => "exited_restored",
            Self::CancelledRestored => "cancelled_restored",
            Self::CrashRecoveredRestored => "crash_recovered_restored",
            Self::ResumedRestored => "resumed_restored",
        }
    }
}

/// Restore-fidelity class for a presentation session.
///
/// This vocabulary mirrors the durable-shell restore classes in
/// [`aureline_recovery::session_restore::records::RestoreClass`] so presentation
/// restore fidelity reads identically to window-session restore fidelity. The
/// mapping is proven by [`Self::to_durable_restore_class`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationRestoreClass {
    /// The layout and every waypoint came back exactly as checkpointed.
    ExactRestore,
    /// The layout came back through a compatible translation; every waypoint is
    /// still live.
    CompatibleRestore,
    /// The layout came back, but one or more waypoint targets degraded to an
    /// honest placeholder / disconnected state.
    LayoutOnly,
    /// The live walkthrough could not be rehydrated; only the prior layout and an
    /// evidence record of the session came back.
    EvidenceOnly,
    /// Nothing could be restored (no checkpoint existed); the user keeps their
    /// current layout and is told the resume could not proceed.
    NoRestore,
}

impl PresentationRestoreClass {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::EvidenceOnly => "evidence_only",
            Self::NoRestore => "no_restore",
        }
    }

    /// The canonical durable-shell restore class this presentation class maps to.
    ///
    /// Proves the presentation lane reuses the existing restore vocabulary rather
    /// than minting a parallel one.
    pub const fn to_durable_restore_class(self) -> RestoreClass {
        match self {
            Self::ExactRestore => RestoreClass::ExactRestore,
            Self::CompatibleRestore => RestoreClass::CompatibleRestore,
            Self::LayoutOnly => RestoreClass::LayoutOnly,
            Self::EvidenceOnly => RestoreClass::EvidenceOnly,
            Self::NoRestore => RestoreClass::NoRestore,
        }
    }

    /// True when this class represents a degraded restore whose cause must be
    /// surfaced rather than hidden behind a generic success message.
    pub const fn is_degraded(self) -> bool {
        matches!(
            self,
            Self::LayoutOnly | Self::EvidenceOnly | Self::NoRestore
        )
    }
}

/// Why a presentation restore narrowed its fidelity.
///
/// The waypoint-scoped triggers ([`Self::MissingDependency`],
/// [`Self::RevokedSharingGrant`], [`Self::UnavailableRemoteTarget`],
/// [`Self::ExpiredAuthority`]) name the spec's honest-degradation cases; the
/// session-scoped triggers ([`Self::LiveSessionUnavailable`],
/// [`Self::CheckpointUnavailable`]) explain an evidence-only or no-restore
/// outcome. Each maps onto a durable-shell
/// [`DowngradeTriggerClass`](aureline_recovery::session_restore::records::DowngradeTriggerClass)
/// via [`Self::to_durable_downgrade_trigger`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreDegradeTrigger {
    /// A surface / extension dependency the waypoint needs is no longer present.
    MissingDependency,
    /// A sharing grant that authorized the waypoint's target was revoked.
    RevokedSharingGrant,
    /// A remote target the waypoint anchored to is unreachable.
    UnavailableRemoteTarget,
    /// A privileged grant or approval the waypoint relied on has expired.
    ExpiredAuthority,
    /// The live walkthrough session itself could not be rehydrated.
    LiveSessionUnavailable,
    /// No checkpoint was captured before the interruption, so nothing can be
    /// restored.
    CheckpointUnavailable,
}

impl RestoreDegradeTrigger {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingDependency => "missing_dependency",
            Self::RevokedSharingGrant => "revoked_sharing_grant",
            Self::UnavailableRemoteTarget => "unavailable_remote_target",
            Self::ExpiredAuthority => "expired_authority",
            Self::LiveSessionUnavailable => "live_session_unavailable",
            Self::CheckpointUnavailable => "checkpoint_unavailable",
        }
    }

    /// The canonical durable-shell downgrade trigger this maps to.
    pub const fn to_durable_downgrade_trigger(self) -> DowngradeTriggerClass {
        match self {
            Self::MissingDependency => DowngradeTriggerClass::MissingExtensionDependency,
            Self::RevokedSharingGrant => DowngradeTriggerClass::MissingRemoteAuthority,
            Self::UnavailableRemoteTarget => DowngradeTriggerClass::MissingRemoteSession,
            Self::ExpiredAuthority => DowngradeTriggerClass::PolicyNarrowing,
            Self::LiveSessionUnavailable => DowngradeTriggerClass::MissingRemoteSession,
            Self::CheckpointUnavailable => DowngradeTriggerClass::ManualRepairRequired,
        }
    }

    /// True when this trigger degrades a single waypoint (as opposed to the whole
    /// session).
    pub const fn is_waypoint_scoped(self) -> bool {
        matches!(
            self,
            Self::MissingDependency
                | Self::RevokedSharingGrant
                | Self::UnavailableRemoteTarget
                | Self::ExpiredAuthority
        )
    }

    /// The honest availability a waypoint degrades to under this trigger, or
    /// `None` for a session-scoped trigger.
    pub const fn degraded_availability(self) -> Option<WaypointAvailability> {
        match self {
            Self::MissingDependency => Some(WaypointAvailability::Placeholder),
            Self::RevokedSharingGrant | Self::UnavailableRemoteTarget | Self::ExpiredAuthority => {
                Some(WaypointAvailability::Disconnected)
            }
            Self::LiveSessionUnavailable | Self::CheckpointUnavailable => None,
        }
    }
}

/// The honest availability of a waypoint's target after restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaypointAvailability {
    /// The target is available; the prior anchor was restored read-only.
    Restored,
    /// A missing dependency degraded the target to an honest placeholder card.
    Placeholder,
    /// A revoked grant, unavailable remote, or expired authority left the target
    /// disconnected.
    Disconnected,
}

impl WaypointAvailability {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Restored => "restored",
            Self::Placeholder => "placeholder",
            Self::Disconnected => "disconnected",
        }
    }

    /// True when this availability is an honest degrade that must carry a trigger
    /// and a placeholder label.
    pub const fn is_degraded(self) -> bool {
        matches!(self, Self::Placeholder | Self::Disconnected)
    }
}

/// The restored state of one waypoint after a presentation restore.
///
/// A restored waypoint replays the prior *anchor and attention only*. Both
/// guardrail flags are fixed safe: restore never replays a mutating action and
/// never re-acquires authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaypointRestoreState {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// The waypoint this state restores.
    pub waypoint_id: String,
    /// The existing surface kind the waypoint targets.
    pub surface_kind: WalkthroughSurfaceKind,
    /// The local / remote / shared boundary the target lives on.
    pub boundary_label: BoundaryLabel,
    /// Stable id of the target object the waypoint anchored to.
    pub target_object_ref: String,
    /// The honest availability of the target after restore.
    pub availability: WaypointAvailability,
    /// The trigger that degraded this waypoint, present iff degraded.
    pub degrade_trigger: Option<RestoreDegradeTrigger>,
    /// A human-facing placeholder / disconnected label, present iff degraded. The
    /// support export carries only a boolean, never this body.
    pub placeholder_label: Option<String>,
    /// Always `false`: restoring a waypoint never replays a mutating action.
    pub replayed_mutating_action: bool,
    /// Always `false`: restoring a waypoint never re-acquires authority.
    pub reacquired_authority: bool,
}

impl WaypointRestoreState {
    /// A cleanly restored waypoint: target available, anchor replayed read-only.
    pub fn restored(
        waypoint_id: impl Into<String>,
        surface_kind: WalkthroughSurfaceKind,
        boundary_label: BoundaryLabel,
        target_object_ref: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: PRESENTATION_WAYPOINT_RESTORE_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            waypoint_id: waypoint_id.into(),
            surface_kind,
            boundary_label,
            target_object_ref: target_object_ref.into(),
            availability: WaypointAvailability::Restored,
            degrade_trigger: None,
            placeholder_label: None,
            replayed_mutating_action: false,
            reacquired_authority: false,
        }
    }

    /// A degraded waypoint: the target is gone, so it shows an honest
    /// placeholder / disconnected card carrying the cause. The availability is
    /// derived from the trigger so it cannot disagree with it.
    pub fn degraded(
        waypoint_id: impl Into<String>,
        surface_kind: WalkthroughSurfaceKind,
        boundary_label: BoundaryLabel,
        target_object_ref: impl Into<String>,
        trigger: RestoreDegradeTrigger,
        placeholder_label: impl Into<String>,
    ) -> Self {
        let availability = trigger
            .degraded_availability()
            .unwrap_or(WaypointAvailability::Disconnected);
        Self {
            record_kind: PRESENTATION_WAYPOINT_RESTORE_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            waypoint_id: waypoint_id.into(),
            surface_kind,
            boundary_label,
            target_object_ref: target_object_ref.into(),
            availability,
            degrade_trigger: Some(trigger),
            placeholder_label: Some(placeholder_label.into()),
            replayed_mutating_action: false,
            reacquired_authority: false,
        }
    }

    /// True when the state's availability, trigger, label, and guardrails line up.
    pub fn is_consistent(&self) -> bool {
        self.consistency_violation().is_none()
    }

    /// The first consistency violation for this waypoint state, if any.
    fn consistency_violation(&self) -> Option<PresentationRestoreViolation> {
        let waypoint_id = self.waypoint_id.clone();
        // Restore replays attention only, never mutation or authority.
        if self.replayed_mutating_action {
            return Some(PresentationRestoreViolation::ReplayedMutatingAction { waypoint_id });
        }
        if self.reacquired_authority {
            return Some(PresentationRestoreViolation::ReacquiredAuthority { waypoint_id });
        }
        match self.availability {
            WaypointAvailability::Restored => {
                if self.degrade_trigger.is_some() || self.placeholder_label.is_some() {
                    return Some(PresentationRestoreViolation::WaypointInconsistent {
                        waypoint_id,
                    });
                }
            }
            WaypointAvailability::Placeholder | WaypointAvailability::Disconnected => {
                let Some(trigger) = self.degrade_trigger else {
                    return Some(PresentationRestoreViolation::DegradeHiddenBehindSuccess {
                        waypoint_id,
                    });
                };
                // A waypoint degrade must come from a waypoint-scoped trigger and
                // its availability must match what that trigger degrades to.
                if !trigger.is_waypoint_scoped()
                    || trigger.degraded_availability() != Some(self.availability)
                {
                    return Some(PresentationRestoreViolation::WaypointInconsistent {
                        waypoint_id,
                    });
                }
                if self.placeholder_label.is_none() {
                    return Some(PresentationRestoreViolation::DegradeHiddenBehindSuccess {
                        waypoint_id,
                    });
                }
            }
        }
        None
    }
}

/// A presentation-session restore report: the canonical, support-safe truth of
/// what a restore did, how faithful it was, and what it honestly could not bring
/// back.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationRestoreReport {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// The session this report restores.
    pub session_id: String,
    /// What triggered the restore.
    pub trigger: PresentationRestoreTrigger,
    /// The lifecycle state the session landed in.
    pub resulting_lifecycle: PresentationRestoreLifecycle,
    /// The restore-fidelity class.
    pub restore_class: PresentationRestoreClass,
    /// The checkpoint this restore replayed (empty for [`PresentationRestoreClass::NoRestore`]).
    pub checkpoint_id: String,
    /// Restored window-topology ref (empty only for no-restore).
    pub restored_layout_ref: String,
    /// Restored focus-chain / selection ref (empty only for no-restore).
    pub restored_focus_ref: String,
    /// Restored panel-visibility ref (empty only for no-restore).
    pub restored_panel_visibility_ref: String,
    /// Restored accessibility-posture ref (empty only for no-restore).
    pub restored_accessibility_posture_ref: String,
    /// One state per restored waypoint. Empty for evidence-only / no-restore.
    pub waypoint_restores: Vec<WaypointRestoreState>,
    /// The distinct degrade triggers this restore surfaced, sorted. Non-empty
    /// for every degraded class, so a problem is never hidden.
    pub degrade_triggers: Vec<RestoreDegradeTrigger>,
    /// A session-scoped degrade cause for an evidence-only / no-restore outcome.
    pub session_degrade: Option<RestoreDegradeTrigger>,
    /// Whether the layout came back through a compatible translation.
    pub compatible_translation_applied: bool,
    /// Whether the live walkthrough session was rehydrated.
    pub live_session_rehydrated: bool,
    // ---- guardrail / honesty flags (derived; re-checked by validate) ----
    /// True only when the restore class is [`PresentationRestoreClass::ExactRestore`].
    pub matches_checkpoint: bool,
    /// Always `false`: restore never replays a mutating action anywhere.
    pub replayed_any_mutating_action: bool,
    /// Always `false`: restore never re-acquires authority anywhere.
    pub reacquired_any_authority: bool,
    /// Always `false`: the user is never stranded in an improvised layout.
    pub left_in_improvised_shell: bool,
    /// Always `false`: a degrade is never hidden behind a generic success.
    pub hides_degrade_behind_generic_success: bool,
}

impl PresentationRestoreReport {
    /// Count of waypoints that came back cleanly.
    pub fn restored_waypoint_count(&self) -> u32 {
        self.waypoint_restores
            .iter()
            .filter(|w| w.availability == WaypointAvailability::Restored)
            .count() as u32
    }

    /// Count of waypoints degraded to a placeholder.
    pub fn placeholder_waypoint_count(&self) -> u32 {
        self.waypoint_restores
            .iter()
            .filter(|w| w.availability == WaypointAvailability::Placeholder)
            .count() as u32
    }

    /// Count of waypoints degraded to a disconnected state.
    pub fn disconnected_waypoint_count(&self) -> u32 {
        self.waypoint_restores
            .iter()
            .filter(|w| w.availability == WaypointAvailability::Disconnected)
            .count() as u32
    }

    /// The distinct degrade triggers actually present across the report.
    fn actual_degrade_triggers(&self) -> Vec<RestoreDegradeTrigger> {
        let mut set: BTreeSet<RestoreDegradeTrigger> = self
            .waypoint_restores
            .iter()
            .filter_map(|w| w.degrade_trigger)
            .collect();
        if let Some(session_degrade) = self.session_degrade {
            set.insert(session_degrade);
        }
        set.into_iter().collect()
    }

    /// Re-derive the restore class from the report's signals so a hand-edited
    /// record cannot quietly claim a fidelity it did not deliver.
    fn expected_restore_class(&self) -> PresentationRestoreClass {
        let checkpoint_present = !self.restored_layout_ref.is_empty();
        let any_degraded = self
            .waypoint_restores
            .iter()
            .any(|w| w.availability.is_degraded());
        derive_restore_class(
            checkpoint_present,
            self.live_session_rehydrated,
            self.compatible_translation_applied,
            any_degraded,
        )
    }

    /// Re-derive every invariant this report claims and return all violations.
    pub fn validate(&self) -> Vec<PresentationRestoreViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PRESENTATION_RESTORE_REPORT_RECORD_KIND
            || self.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
            || self.shared_contract_ref != PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF
        {
            violations.push(PresentationRestoreViolation::MalformedRecord);
        }

        if self.resulting_lifecycle != self.trigger.restored_lifecycle() {
            violations.push(PresentationRestoreViolation::LifecycleMismatch);
        }

        for waypoint in &self.waypoint_restores {
            if waypoint.record_kind != PRESENTATION_WAYPOINT_RESTORE_RECORD_KIND
                || waypoint.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
                || waypoint.shared_contract_ref != PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF
            {
                violations.push(PresentationRestoreViolation::MalformedRecord);
            }
            if let Some(violation) = waypoint.consistency_violation() {
                violations.push(violation);
            }
        }

        let expected_class = self.expected_restore_class();
        if self.restore_class != expected_class {
            violations.push(PresentationRestoreViolation::RestoreClassMismatch {
                expected: expected_class,
                found: self.restore_class,
            });
        }

        if self.matches_checkpoint != (self.restore_class == PresentationRestoreClass::ExactRestore)
        {
            violations.push(PresentationRestoreViolation::MatchesCheckpointMismatch);
        }

        // Aggregate guardrails are fixed safe.
        if self.replayed_any_mutating_action
            || self
                .waypoint_restores
                .iter()
                .any(|w| w.replayed_mutating_action)
        {
            violations.push(PresentationRestoreViolation::ReplayedMutatingAction {
                waypoint_id: String::new(),
            });
        }
        if self.reacquired_any_authority
            || self
                .waypoint_restores
                .iter()
                .any(|w| w.reacquired_authority)
        {
            violations.push(PresentationRestoreViolation::ReacquiredAuthority {
                waypoint_id: String::new(),
            });
        }

        // Layout fidelity: every class but no-restore brings the layout back, so
        // the user is never stranded in an improvised shell.
        let refs_present = !self.restored_layout_ref.is_empty()
            && !self.restored_focus_ref.is_empty()
            && !self.restored_panel_visibility_ref.is_empty()
            && !self.restored_accessibility_posture_ref.is_empty();
        match self.restore_class {
            PresentationRestoreClass::NoRestore => {
                if refs_present || !self.restored_layout_ref.is_empty() {
                    violations.push(PresentationRestoreViolation::ImprovisedShell);
                }
            }
            _ => {
                if !refs_present || self.left_in_improvised_shell {
                    violations.push(PresentationRestoreViolation::ImprovisedShell);
                }
            }
        }

        // Honesty: a degraded class must surface its cause, never fold it into a
        // generic success. Evidence-only / no-restore carry a session-scoped
        // cause; layout-only carries per-waypoint causes.
        let actual_triggers = self.actual_degrade_triggers();
        if self.degrade_triggers != actual_triggers {
            violations.push(PresentationRestoreViolation::DegradeTriggerSetMismatch);
        }
        match self.restore_class {
            PresentationRestoreClass::EvidenceOnly | PresentationRestoreClass::NoRestore => {
                if !self.waypoint_restores.is_empty() {
                    violations.push(PresentationRestoreViolation::WaypointInconsistent {
                        waypoint_id: String::new(),
                    });
                }
                match self.session_degrade {
                    Some(trigger) if !trigger.is_waypoint_scoped() => {}
                    _ => {
                        violations.push(PresentationRestoreViolation::DegradeHiddenBehindSuccess {
                            waypoint_id: String::new(),
                        })
                    }
                }
            }
            PresentationRestoreClass::LayoutOnly => {
                if self.session_degrade.is_some() {
                    violations.push(PresentationRestoreViolation::DegradeTriggerSetMismatch);
                }
                if actual_triggers.is_empty() {
                    violations.push(PresentationRestoreViolation::DegradeHiddenBehindSuccess {
                        waypoint_id: String::new(),
                    });
                }
            }
            PresentationRestoreClass::ExactRestore
            | PresentationRestoreClass::CompatibleRestore => {
                if self.session_degrade.is_some() || !actual_triggers.is_empty() {
                    violations.push(PresentationRestoreViolation::DegradeTriggerSetMismatch);
                }
            }
        }
        if self.hides_degrade_behind_generic_success {
            violations.push(PresentationRestoreViolation::DegradeHiddenBehindSuccess {
                waypoint_id: String::new(),
            });
        }

        violations
    }
}

/// A consistency / honesty violation found by validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresentationRestoreViolation {
    /// A record carried the wrong kind, schema version, or contract ref.
    MalformedRecord,
    /// The resulting lifecycle did not match the trigger.
    LifecycleMismatch,
    /// A waypoint state's availability, trigger, or label disagree.
    WaypointInconsistent {
        /// The offending waypoint (empty for a report-level inconsistency).
        waypoint_id: String,
    },
    /// The claimed restore class does not match the report's signals.
    RestoreClassMismatch {
        /// The class the signals imply.
        expected: PresentationRestoreClass,
        /// The class the report claimed.
        found: PresentationRestoreClass,
    },
    /// `matches_checkpoint` disagreed with the restore class.
    MatchesCheckpointMismatch,
    /// A mutating action was marked replayed during restore.
    ReplayedMutatingAction {
        /// The offending waypoint (empty for the report aggregate).
        waypoint_id: String,
    },
    /// Authority was marked re-acquired during restore.
    ReacquiredAuthority {
        /// The offending waypoint (empty for the report aggregate).
        waypoint_id: String,
    },
    /// The user would be left in an improvised layout.
    ImprovisedShell,
    /// A degrade was hidden behind a generic success instead of being surfaced.
    DegradeHiddenBehindSuccess {
        /// The offending waypoint (empty for a report-level hide).
        waypoint_id: String,
    },
    /// The surfaced degrade-trigger set did not match the actual causes.
    DegradeTriggerSetMismatch,
}

/// Re-derive the restore class from a session's restore signals.
fn derive_restore_class(
    checkpoint_present: bool,
    live_session_rehydrated: bool,
    compatible_translation_applied: bool,
    any_waypoint_degraded: bool,
) -> PresentationRestoreClass {
    if !checkpoint_present {
        PresentationRestoreClass::NoRestore
    } else if !live_session_rehydrated {
        PresentationRestoreClass::EvidenceOnly
    } else if any_waypoint_degraded {
        PresentationRestoreClass::LayoutOnly
    } else if compatible_translation_applied {
        PresentationRestoreClass::CompatibleRestore
    } else {
        PresentationRestoreClass::ExactRestore
    }
}

/// A per-waypoint degrade decision fed into [`project_restore_report`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaypointDegrade {
    /// The waypoint that degraded.
    pub waypoint_id: String,
    /// Why it degraded.
    pub trigger: RestoreDegradeTrigger,
    /// The honest placeholder / disconnected label to show.
    pub placeholder_label: String,
}

impl WaypointDegrade {
    /// Build a degrade decision.
    pub fn new(
        waypoint_id: impl Into<String>,
        trigger: RestoreDegradeTrigger,
        placeholder_label: impl Into<String>,
    ) -> Self {
        Self {
            waypoint_id: waypoint_id.into(),
            trigger,
            placeholder_label: placeholder_label.into(),
        }
    }
}

/// Inputs that drive a live-rehydrated restore projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreProjectionInputs {
    /// What triggered the restore.
    pub trigger: PresentationRestoreTrigger,
    /// Whether the layout came back through a compatible translation.
    pub compatible_translation_applied: bool,
    /// Per-waypoint degrade decisions, keyed by waypoint id. Waypoints not listed
    /// here come back cleanly.
    pub waypoint_degrades: Vec<WaypointDegrade>,
}

impl RestoreProjectionInputs {
    /// A clean restore: every waypoint comes back and no translation is needed.
    pub fn exact(trigger: PresentationRestoreTrigger) -> Self {
        Self {
            trigger,
            compatible_translation_applied: false,
            waypoint_degrades: Vec::new(),
        }
    }

    /// A restore that needed a compatible layout translation.
    pub fn compatible(trigger: PresentationRestoreTrigger) -> Self {
        Self {
            trigger,
            compatible_translation_applied: true,
            waypoint_degrades: Vec::new(),
        }
    }

    /// A restore with one or more degraded waypoints.
    pub fn with_degrades(
        trigger: PresentationRestoreTrigger,
        waypoint_degrades: Vec<WaypointDegrade>,
    ) -> Self {
        Self {
            trigger,
            compatible_translation_applied: false,
            waypoint_degrades,
        }
    }
}

/// Project a [`PresentationRestoreReport`] for a live-rehydrated session.
///
/// Walks the session's waypoints, restoring each cleanly unless `inputs` names a
/// degrade for it, then derives the restore class and fills the guardrails to
/// their safe values. The layout / focus / panel / accessibility refs come
/// straight from the session's checkpoint, so the prior environment is always
/// what comes back.
pub fn project_restore_report(
    session: &PresentationSession,
    inputs: &RestoreProjectionInputs,
) -> PresentationRestoreReport {
    let checkpoint = &session.restore_checkpoint;
    let waypoint_restores: Vec<WaypointRestoreState> = session
        .waypoints
        .iter()
        .map(|waypoint| {
            match inputs
                .waypoint_degrades
                .iter()
                .find(|d| d.waypoint_id == waypoint.waypoint_id)
            {
                Some(degrade) => WaypointRestoreState::degraded(
                    waypoint.waypoint_id.clone(),
                    waypoint.surface_kind,
                    waypoint.boundary_label,
                    waypoint.target_object_ref.clone(),
                    degrade.trigger,
                    degrade.placeholder_label.clone(),
                ),
                None => WaypointRestoreState::restored(
                    waypoint.waypoint_id.clone(),
                    waypoint.surface_kind,
                    waypoint.boundary_label,
                    waypoint.target_object_ref.clone(),
                ),
            }
        })
        .collect();

    let any_degraded = waypoint_restores
        .iter()
        .any(|w| w.availability.is_degraded());
    let restore_class = derive_restore_class(
        true,
        true,
        inputs.compatible_translation_applied,
        any_degraded,
    );
    let degrade_triggers: Vec<RestoreDegradeTrigger> = waypoint_restores
        .iter()
        .filter_map(|w| w.degrade_trigger)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    PresentationRestoreReport {
        record_kind: PRESENTATION_RESTORE_REPORT_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        session_id: session.session_id.clone(),
        trigger: inputs.trigger,
        resulting_lifecycle: inputs.trigger.restored_lifecycle(),
        restore_class,
        checkpoint_id: checkpoint.checkpoint_id.clone(),
        restored_layout_ref: checkpoint.prior_layout_ref.clone(),
        restored_focus_ref: checkpoint.prior_focus_ref.clone(),
        restored_panel_visibility_ref: checkpoint.prior_panel_visibility_ref.clone(),
        restored_accessibility_posture_ref: checkpoint.accessibility_posture_ref.clone(),
        waypoint_restores,
        degrade_triggers,
        session_degrade: None,
        compatible_translation_applied: inputs.compatible_translation_applied,
        live_session_rehydrated: true,
        matches_checkpoint: restore_class == PresentationRestoreClass::ExactRestore,
        replayed_any_mutating_action: false,
        reacquired_any_authority: false,
        left_in_improvised_shell: false,
        hides_degrade_behind_generic_success: false,
    }
}

/// Project an evidence-only restore: the prior layout came back, but the live
/// walkthrough could not be rehydrated, so only an evidence record remains.
///
/// `session_cause` must be a session-scoped trigger (it is normalized to
/// [`RestoreDegradeTrigger::LiveSessionUnavailable`] if a waypoint-scoped trigger
/// is passed). No waypoint is re-run and no authority is re-acquired.
pub fn project_evidence_only_report(
    session: &PresentationSession,
    trigger: PresentationRestoreTrigger,
    session_cause: RestoreDegradeTrigger,
) -> PresentationRestoreReport {
    let checkpoint = &session.restore_checkpoint;
    let session_degrade = if session_cause.is_waypoint_scoped() {
        RestoreDegradeTrigger::LiveSessionUnavailable
    } else {
        session_cause
    };
    PresentationRestoreReport {
        record_kind: PRESENTATION_RESTORE_REPORT_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        session_id: session.session_id.clone(),
        trigger,
        resulting_lifecycle: trigger.restored_lifecycle(),
        restore_class: PresentationRestoreClass::EvidenceOnly,
        checkpoint_id: checkpoint.checkpoint_id.clone(),
        restored_layout_ref: checkpoint.prior_layout_ref.clone(),
        restored_focus_ref: checkpoint.prior_focus_ref.clone(),
        restored_panel_visibility_ref: checkpoint.prior_panel_visibility_ref.clone(),
        restored_accessibility_posture_ref: checkpoint.accessibility_posture_ref.clone(),
        waypoint_restores: Vec::new(),
        degrade_triggers: vec![session_degrade],
        session_degrade: Some(session_degrade),
        compatible_translation_applied: false,
        live_session_rehydrated: false,
        matches_checkpoint: false,
        replayed_any_mutating_action: false,
        reacquired_any_authority: false,
        left_in_improvised_shell: false,
        hides_degrade_behind_generic_success: false,
    }
}

/// Project a no-restore report: no checkpoint was captured before the
/// interruption, so nothing is restored and the user keeps their current layout.
///
/// This is the honest answer the spec's out-of-scope rule demands — the resume
/// could not proceed, and that is surfaced rather than hidden behind a generic
/// "restored" message.
pub fn project_no_restore_report(
    session_id: impl Into<String>,
    trigger: PresentationRestoreTrigger,
) -> PresentationRestoreReport {
    PresentationRestoreReport {
        record_kind: PRESENTATION_RESTORE_REPORT_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        session_id: session_id.into(),
        trigger,
        resulting_lifecycle: trigger.restored_lifecycle(),
        restore_class: PresentationRestoreClass::NoRestore,
        checkpoint_id: String::new(),
        restored_layout_ref: String::new(),
        restored_focus_ref: String::new(),
        restored_panel_visibility_ref: String::new(),
        restored_accessibility_posture_ref: String::new(),
        waypoint_restores: Vec::new(),
        degrade_triggers: vec![RestoreDegradeTrigger::CheckpointUnavailable],
        session_degrade: Some(RestoreDegradeTrigger::CheckpointUnavailable),
        compatible_translation_applied: false,
        live_session_rehydrated: false,
        matches_checkpoint: false,
        replayed_any_mutating_action: false,
        reacquired_any_authority: false,
        left_in_improvised_shell: false,
        hides_degrade_behind_generic_success: false,
    }
}

/// One support-safe row per restore report. Carries enums, counts, and booleans —
/// never checkpoint refs or placeholder bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationRestoreSupportExportRow {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Session id.
    pub session_id: String,
    /// What triggered the restore.
    pub trigger: PresentationRestoreTrigger,
    /// The lifecycle state landed in.
    pub resulting_lifecycle: PresentationRestoreLifecycle,
    /// The restore-fidelity class.
    pub restore_class: PresentationRestoreClass,
    /// The canonical durable-shell class this maps to, for cross-surface joins.
    pub durable_restore_class: RestoreClass,
    /// Waypoints that came back cleanly.
    pub restored_waypoint_count: u32,
    /// Waypoints degraded to a placeholder.
    pub placeholder_waypoint_count: u32,
    /// Waypoints degraded to a disconnected state.
    pub disconnected_waypoint_count: u32,
    /// The distinct degrade triggers surfaced, sorted.
    pub degrade_triggers: Vec<RestoreDegradeTrigger>,
    /// A session-scoped degrade cause, when present.
    pub session_degrade: Option<RestoreDegradeTrigger>,
    /// Whether the layout came back through a compatible translation.
    pub compatible_translation_applied: bool,
    /// Whether the live walkthrough was rehydrated.
    pub live_session_rehydrated: bool,
    /// Whether the restore matched the checkpoint exactly.
    pub matches_checkpoint: bool,
    /// Whether any mutating action was replayed (always `false`).
    pub replayed_any_mutating_action: bool,
    /// Whether any authority was re-acquired (always `false`).
    pub reacquired_any_authority: bool,
    /// Whether the user was left in an improvised shell (always `false`).
    pub left_in_improvised_shell: bool,
    /// Whether a degrade was hidden behind a generic success (always `false`).
    pub hides_degrade_behind_generic_success: bool,
}

/// Support-export wrapper over a set of restore reports. Privacy-safe by
/// construction: no checkpoint refs or placeholder bodies are carried.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationRestoreSupportExport {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Export id.
    pub export_id: String,
    /// Mint timestamp.
    pub generated_at: String,
    /// Support-safe rows.
    pub rows: Vec<PresentationRestoreSupportExportRow>,
    /// Always `true`: checkpoint refs and placeholder bodies are excluded.
    pub raw_private_material_excluded: bool,
}

impl PresentationRestoreSupportExport {
    /// Project a set of restore reports into a support-safe export.
    pub fn from_reports<'a>(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        reports: impl IntoIterator<Item = &'a PresentationRestoreReport>,
    ) -> Self {
        let rows = reports
            .into_iter()
            .map(|report| PresentationRestoreSupportExportRow {
                record_kind: PRESENTATION_RESTORE_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
                schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
                shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
                session_id: report.session_id.clone(),
                trigger: report.trigger,
                resulting_lifecycle: report.resulting_lifecycle,
                restore_class: report.restore_class,
                durable_restore_class: report.restore_class.to_durable_restore_class(),
                restored_waypoint_count: report.restored_waypoint_count(),
                placeholder_waypoint_count: report.placeholder_waypoint_count(),
                disconnected_waypoint_count: report.disconnected_waypoint_count(),
                degrade_triggers: report.degrade_triggers.clone(),
                session_degrade: report.session_degrade,
                compatible_translation_applied: report.compatible_translation_applied,
                live_session_rehydrated: report.live_session_rehydrated,
                matches_checkpoint: report.matches_checkpoint,
                replayed_any_mutating_action: report.replayed_any_mutating_action,
                reacquired_any_authority: report.reacquired_any_authority,
                left_in_improvised_shell: report.left_in_improvised_shell,
                hides_degrade_behind_generic_success: report.hides_degrade_behind_generic_success,
            })
            .collect();
        Self {
            record_kind: PRESENTATION_RESTORE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            rows,
            raw_private_material_excluded: true,
        }
    }
}
