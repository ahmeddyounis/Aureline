//! Crash-loop recovery screens and issue-report / crash-intake flows that keep exact-build identity,
//! symbolication fidelity, restore provenance, and redaction posture honest from the first screen.
//!
//! Where the crash store owns *what a crash envelope contains* and the recovery ladder owns *which
//! repair is narrow enough to try*, this packet governs *how a blocked user is offered bounded recovery
//! and how the crash is taken in for support without ever overclaiming what is known*. It is a registry
//! of crash-recovery screens, one per crash-loop scenario worth recovering from, each carrying the
//! visible crash-envelope id, the copyable exact-build id and its identity fidelity, the symbolication
//! fidelity, the restore-provenance class, the install / advisory state, the redaction / export posture,
//! a set of **distinct, named recovery actions** (Restore, Open without restore, Safe mode, Disable
//! recently changed extension, Disable recently changed profile, Open logs, Report issue — never a
//! single generic "try again"), and the typed intake-packet modes (local save, team share, formal
//! support handoff) it offers. It reuses the crash and restore truth by reference: every screen carries
//! a `crash_envelope_ref` and a `source_of_truth_ref` projecting from the existing crash-envelope,
//! symbolication, restore-provenance, and quarantine objects rather than re-deriving any crash of its
//! own.
//!
//! The readiness analogue here is a fail-closed **recovery / intake gate**. The guardrail the source
//! set treats as core supportability UX is that a screen must never imply an exact build or a resolved
//! symbolication when only approximate or unresolved data exists, must never collapse the recovery
//! choices into one generic affordance, and must never present a clean "ready to send" intake that hides
//! a downgraded restore, an active advisory, or content that cannot leave the machine — and must never
//! make the local-save path look secondary to a team-share or formal-support send. Each screen therefore
//! publishes a [`RecoveryPresentation`] that is the weakest of three ceilings: a **fidelity** ceiling
//! (an exact build and resolved symbols present transparently; an approximate / unresolved build or a
//! stale / partial / unresolved symbolication narrows it), a **disposition** ceiling (a downgraded
//! restore or an active install advisory / extension quarantine narrows it), and a **sendability**
//! ceiling (an intake whose content cannot safely leave the machine for the selected send mode caps it
//! at send-blocked). A screen can never claim a cleaner presentation than its inputs support, and two
//! stricter rules still hold: every recovery action stays distinct, bounded, and labeled with whether it
//! reruns or discards state (no action ever discards user-owned state), and local-save stays at least as
//! prominent as every send mode — both enforced as hard invariants rather than soft downgrades.
//!
//! Every screen always carries its one-step `explain_entrypoint_ref` — the inspectable "Why this crash,
//! on which build?" answer — and its `cli_object_ref`, the CLI / headless equivalent, so the same
//! recovery and intake answer is reachable from the active crash-recovery screen, the Support Center,
//! the CLI / headless recovery path, the issue-report packet, and the support export. Every required
//! consumer surface binds to this one registry via a [`RecoveryConsumerBinding`] that must ingest it,
//! preserve its recovery / intake vocabulary and object ids, keep local-save first-class, and narrow
//! with it, so the desktop screen, Support Center, CLI / headless, issue-report packet, and support
//! export share one grammar.
//!
//! The packet is checked in at `artifacts/support/m5/m5-crash-intake-and-recovery.json` and embedded
//! here. It is metadata-only: every field is a typed state, a count, a visible id, or an opaque ref, and
//! it carries no credential bodies, raw provider payloads, raw stack dumps, or secret-bearing payloads.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported crash-intake-and-recovery schema version.
pub const M5_CRASH_INTAKE_RECOVERY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_CRASH_INTAKE_RECOVERY_RECORD_KIND: &str = "m5_crash_intake_and_recovery";

/// Repo-relative path to the checked-in packet.
pub const M5_CRASH_INTAKE_RECOVERY_PATH: &str =
    "artifacts/support/m5/m5-crash-intake-and-recovery.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_CRASH_INTAKE_RECOVERY_SCHEMA_REF: &str = "schemas/support/m5-crash-intake.schema.json";

/// Repo-relative path to the companion document.
pub const M5_CRASH_INTAKE_RECOVERY_DOC_REF: &str =
    "docs/help/support/m5-crash-intake-and-recovery.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_CRASH_INTAKE_RECOVERY_ARTIFACT_DOC_REF: &str =
    "artifacts/support/m5/m5-crash-intake-and-recovery.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_CRASH_INTAKE_RECOVERY_FIXTURE_DIR: &str =
    "fixtures/support/m5/m5-crash-intake-and-recovery";

/// Repo-relative path to the shiproom review packet that renders this registry.
pub const M5_CRASH_INTAKE_RECOVERY_REVIEW_PACKET_REF: &str =
    "artifacts/shiproom/m5-crash-intake-and-recovery-review-packet/crash_intake_and_recovery_review_packet.md";

/// Embedded checked-in packet JSON.
pub const M5_CRASH_INTAKE_RECOVERY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/support/m5/m5-crash-intake-and-recovery.json"
));

/// A distinct, named recovery action. There is no generic "try again" or "reset" class: a crash loop
/// routes to bounded, individually labeled actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionClass {
    /// Re-run the prior session's restore replay.
    Restore,
    /// Open the workspace with restore replay deferred.
    OpenWithoutRestore,
    /// Restart in the bounded safe-mode profile.
    SafeMode,
    /// Reversibly disable a recently changed extension suspected of the crash loop.
    DisableRecentlyChangedExtension,
    /// Reversibly disable a recently changed profile or layout suspected of the crash loop.
    DisableRecentlyChangedProfile,
    /// Open the local logs read-only.
    OpenLogs,
    /// Open the issue-report / crash-intake flow.
    ReportIssue,
}

impl RecoveryActionClass {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Restore,
        Self::OpenWithoutRestore,
        Self::SafeMode,
        Self::DisableRecentlyChangedExtension,
        Self::DisableRecentlyChangedProfile,
        Self::OpenLogs,
        Self::ReportIssue,
    ];

    /// The actions every recovery screen must always offer, regardless of the suspect change set.
    pub const CORE: [Self; 5] = [
        Self::Restore,
        Self::OpenWithoutRestore,
        Self::SafeMode,
        Self::OpenLogs,
        Self::ReportIssue,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Restore => "restore",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::SafeMode => "safe_mode",
            Self::DisableRecentlyChangedExtension => "disable_recently_changed_extension",
            Self::DisableRecentlyChangedProfile => "disable_recently_changed_profile",
            Self::OpenLogs => "open_logs",
            Self::ReportIssue => "report_issue",
        }
    }

    /// Stable command id bound to this action (command-backed and keyboard reachable).
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::Restore => "command.recovery.restore",
            Self::OpenWithoutRestore => "command.recovery.open_without_restore",
            Self::SafeMode => "command.recovery.enter_safe_mode",
            Self::DisableRecentlyChangedExtension => "command.recovery.disable_recent_extension",
            Self::DisableRecentlyChangedProfile => "command.recovery.disable_recent_profile",
            Self::OpenLogs => "command.recovery.open_logs",
            Self::ReportIssue => "command.recovery.report_issue",
        }
    }

    /// The rerun / discard effect this action carries.
    pub const fn effect(self) -> RecoveryActionEffect {
        match self {
            Self::Restore => RecoveryActionEffect::RerunsRestore,
            Self::OpenWithoutRestore => RecoveryActionEffect::OpensWithoutReplay,
            Self::SafeMode => RecoveryActionEffect::RestartsInSafeProfile,
            Self::DisableRecentlyChangedExtension | Self::DisableRecentlyChangedProfile => {
                RecoveryActionEffect::DisablesSuspectReversibly
            }
            Self::OpenLogs => RecoveryActionEffect::InspectsReadOnly,
            Self::ReportIssue => RecoveryActionEffect::OpensIntakeNoSend,
        }
    }

    /// The bounded blast radius this action applies.
    pub const fn blast_radius(self) -> BlastRadiusClass {
        match self {
            Self::Restore => BlastRadiusClass::FullSessionReplay,
            Self::OpenWithoutRestore => BlastRadiusClass::WorkspaceNoReplay,
            Self::SafeMode => BlastRadiusClass::ReducedProfile,
            Self::DisableRecentlyChangedExtension | Self::DisableRecentlyChangedProfile => {
                BlastRadiusClass::SingleSuspectToggle
            }
            Self::OpenLogs => BlastRadiusClass::ReadOnlyInspect,
            Self::ReportIssue => BlastRadiusClass::MetadataHandoff,
        }
    }

    /// Whether this action re-enters or relaunches the user's session and therefore must honor
    /// no-silent-rerun semantics.
    pub const fn is_session_reentry(self) -> bool {
        matches!(
            self,
            Self::Restore | Self::OpenWithoutRestore | Self::SafeMode
        )
    }

    /// Whether this action reversibly disables a recently changed suspect.
    pub const fn is_disable(self) -> bool {
        matches!(
            self,
            Self::DisableRecentlyChangedExtension | Self::DisableRecentlyChangedProfile
        )
    }

    /// Whether this action is one of the always-offered core actions.
    pub const fn is_core(self) -> bool {
        matches!(
            self,
            Self::Restore
                | Self::OpenWithoutRestore
                | Self::SafeMode
                | Self::OpenLogs
                | Self::ReportIssue
        )
    }

    /// Whether the action requires explicit user confirmation before it runs.
    ///
    /// Session re-entry and suspect-disable actions are mutating and never applied silently; inspect and
    /// report actions do not need confirmation.
    pub const fn requires_explicit_confirmation(self) -> bool {
        self.is_session_reentry() || self.is_disable()
    }
}

/// The precise rerun / discard semantics of a recovery action, so a user can tell which action reruns
/// or discards state without guessing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionEffect {
    /// Re-runs the prior session's restore replay.
    RerunsRestore,
    /// Opens the workspace with restore replay deferred; nothing is re-run.
    OpensWithoutReplay,
    /// Restarts in the bounded safe-mode profile with replay and extensions held back.
    RestartsInSafeProfile,
    /// Reversibly disables a suspect change; nothing is re-run.
    DisablesSuspectReversibly,
    /// Inspects logs read-only; the session is not changed.
    InspectsReadOnly,
    /// Opens the intake flow; nothing leaves the machine until the user confirms.
    OpensIntakeNoSend,
}

impl RecoveryActionEffect {
    /// Every effect, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RerunsRestore,
        Self::OpensWithoutReplay,
        Self::RestartsInSafeProfile,
        Self::DisablesSuspectReversibly,
        Self::InspectsReadOnly,
        Self::OpensIntakeNoSend,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RerunsRestore => "reruns_restore",
            Self::OpensWithoutReplay => "opens_without_replay",
            Self::RestartsInSafeProfile => "restarts_in_safe_profile",
            Self::DisablesSuspectReversibly => "disables_suspect_reversibly",
            Self::InspectsReadOnly => "inspects_read_only",
            Self::OpensIntakeNoSend => "opens_intake_no_send",
        }
    }

    /// Whether the action re-runs the prior session's restore replay.
    ///
    /// Only [`Self::RerunsRestore`] replays the failed session; safe mode and open-without-restore
    /// explicitly defer replay so a blocked user is never silently re-run into the same crash.
    pub const fn reruns_session(self) -> bool {
        matches!(self, Self::RerunsRestore)
    }

    /// Whether the action discards user-owned state. No recovery action ever does.
    pub const fn discards_state(self) -> bool {
        false
    }
}

/// The bounded blast radius of a recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlastRadiusClass {
    /// Re-runs the full prior session restore.
    FullSessionReplay,
    /// Opens the workspace with replay deferred.
    WorkspaceNoReplay,
    /// Restarts in a reduced safe-mode profile.
    ReducedProfile,
    /// Toggles a single recently changed suspect.
    SingleSuspectToggle,
    /// Opens read-only logs.
    ReadOnlyInspect,
    /// Hands off a metadata-only intake packet.
    MetadataHandoff,
}

impl BlastRadiusClass {
    /// Every blast-radius class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullSessionReplay,
        Self::WorkspaceNoReplay,
        Self::ReducedProfile,
        Self::SingleSuspectToggle,
        Self::ReadOnlyInspect,
        Self::MetadataHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSessionReplay => "full_session_replay",
            Self::WorkspaceNoReplay => "workspace_no_replay",
            Self::ReducedProfile => "reduced_profile",
            Self::SingleSuspectToggle => "single_suspect_toggle",
            Self::ReadOnlyInspect => "read_only_inspect",
            Self::MetadataHandoff => "metadata_handoff",
        }
    }
}

/// How precisely the running build is identified, after capture.
///
/// The out-of-scope guardrail is that a screen must never imply an exact build when only approximate or
/// unresolved data exists; this class is what the fidelity ceiling reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildIdentityFidelity {
    /// The exact build identity is resolved and verifiable.
    ExactBuild,
    /// Only an approximate build identity is available.
    ApproximateBuild,
    /// The build identity could not be resolved.
    UnresolvedBuild,
}

impl BuildIdentityFidelity {
    /// Every build-identity fidelity, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ExactBuild,
        Self::ApproximateBuild,
        Self::UnresolvedBuild,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuild => "exact_build",
            Self::ApproximateBuild => "approximate_build",
            Self::UnresolvedBuild => "unresolved_build",
        }
    }

    /// Whether the running build is identified exactly.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::ExactBuild)
    }

    /// Highest presentation this fidelity permits.
    pub const fn presentation_ceiling(self) -> RecoveryPresentation {
        match self {
            Self::ExactBuild => RecoveryPresentation::ExactReady,
            Self::ApproximateBuild | Self::UnresolvedBuild => RecoveryPresentation::Narrowed,
        }
    }
}

/// How fully the crash's frames are symbolicated, after capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolicationFidelity {
    /// Frames are fully resolved against the matching symbol map.
    Resolved,
    /// Some frames are resolved; others are not.
    PartiallyResolved,
    /// The symbol map does not match this build; frames may be misattributed.
    StaleSymbolMap,
    /// No frames could be resolved.
    Unresolved,
}

impl SymbolicationFidelity {
    /// Every symbolication fidelity, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Resolved,
        Self::PartiallyResolved,
        Self::StaleSymbolMap,
        Self::Unresolved,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::PartiallyResolved => "partially_resolved",
            Self::StaleSymbolMap => "stale_symbol_map",
            Self::Unresolved => "unresolved",
        }
    }

    /// Whether the frames are fully resolved.
    pub const fn is_resolved(self) -> bool {
        matches!(self, Self::Resolved)
    }

    /// Highest presentation this fidelity permits.
    pub const fn presentation_ceiling(self) -> RecoveryPresentation {
        match self {
            Self::Resolved => RecoveryPresentation::ExactReady,
            Self::PartiallyResolved | Self::StaleSymbolMap | Self::Unresolved => {
                RecoveryPresentation::Narrowed
            }
        }
    }
}

/// The provenance class of the restore the failed launch attempted, aligned with the restore hydrator
/// and restore-preview surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreProvenanceClass {
    /// The prior session would be restored exactly.
    ExactRestore,
    /// The prior session would be restored with compatible substitutions.
    CompatibleRestore,
    /// Only the window / pane layout would be restored.
    LayoutOnly,
    /// Only evidence (drafts, history) would be surfaced; no live restore.
    EvidenceOnly,
    /// The restore was downgraded from a stronger class because the recorded state could not be honored.
    RestoreDowngraded,
    /// No restore was attempted for this launch.
    NoRestoreAttempted,
}

impl RestoreProvenanceClass {
    /// Every restore-provenance class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactRestore,
        Self::CompatibleRestore,
        Self::LayoutOnly,
        Self::EvidenceOnly,
        Self::RestoreDowngraded,
        Self::NoRestoreAttempted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::EvidenceOnly => "evidence_only",
            Self::RestoreDowngraded => "restore_downgraded",
            Self::NoRestoreAttempted => "no_restore_attempted",
        }
    }

    /// Whether the restore was downgraded from a stronger recorded class.
    pub const fn is_downgraded(self) -> bool {
        matches!(self, Self::RestoreDowngraded)
    }
}

/// The install / advisory state that applies to the running build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallAdvisoryState {
    /// No advisory or quarantine applies.
    Clean,
    /// An advisory applies to this build.
    AdvisoryActive,
    /// A quarantine applies to a suspect extension on this build.
    ExtensionQuarantineActive,
    /// A newer build is available.
    UpdateAvailable,
}

impl InstallAdvisoryState {
    /// Every install / advisory state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Clean,
        Self::AdvisoryActive,
        Self::ExtensionQuarantineActive,
        Self::UpdateAvailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::AdvisoryActive => "advisory_active",
            Self::ExtensionQuarantineActive => "extension_quarantine_active",
            Self::UpdateAvailable => "update_available",
        }
    }

    /// Whether an active advisory or quarantine narrows the screen.
    pub const fn narrows(self) -> bool {
        matches!(self, Self::AdvisoryActive | Self::ExtensionQuarantineActive)
    }

    /// Whether a quarantine applies to a suspect extension.
    pub const fn is_quarantine(self) -> bool {
        matches!(self, Self::ExtensionQuarantineActive)
    }
}

/// How the crash-intake content is redacted for export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionExportPosture {
    /// Metadata only; export-safe.
    MetadataSafeDefault,
    /// A redacted summary; export-safe.
    RedactedSummary,
    /// Retained on the machine only; not export-safe.
    LocalOnlyRetained,
    /// Content cannot be made safe; not export-safe.
    BlockedUnsafeContent,
}

impl RedactionExportPosture {
    /// Every redaction / export posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MetadataSafeDefault,
        Self::RedactedSummary,
        Self::LocalOnlyRetained,
        Self::BlockedUnsafeContent,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::RedactedSummary => "redacted_summary",
            Self::LocalOnlyRetained => "local_only_retained",
            Self::BlockedUnsafeContent => "blocked_unsafe_content",
        }
    }

    /// Whether content handled this way may be exported off the machine.
    pub const fn is_export_safe_off_machine(self) -> bool {
        matches!(self, Self::MetadataSafeDefault | Self::RedactedSummary)
    }
}

/// A typed intake-packet mode. Local save, team share, and formal-support handoff are all offered from
/// the same crash / intake surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntakeMode {
    /// A local-only save; the packet never leaves the machine.
    LocalSave,
    /// A share with the user's team.
    TeamShare,
    /// A formal support / vendor handoff.
    FormalSupportHandoff,
}

impl IntakeMode {
    /// Every intake mode, in declaration order.
    pub const ALL: [Self; 3] = [Self::LocalSave, Self::TeamShare, Self::FormalSupportHandoff];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSave => "local_save",
            Self::TeamShare => "team_share",
            Self::FormalSupportHandoff => "formal_support_handoff",
        }
    }

    /// Whether the packet stays on the machine.
    pub const fn is_local_save(self) -> bool {
        matches!(self, Self::LocalSave)
    }

    /// Whether selecting this mode causes the packet to leave the machine.
    pub const fn leaves_machine(self) -> bool {
        !self.is_local_save()
    }
}

/// How prominent an intake path is in the screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathProminence {
    /// The path is presented as the primary, default affordance.
    Primary,
    /// The path is presented co-equal to the other paths.
    CoEqual,
    /// The path is presented as a secondary affordance.
    Secondary,
}

impl PathProminence {
    /// Every prominence level, most prominent first.
    pub const ALL: [Self; 3] = [Self::Primary, Self::CoEqual, Self::Secondary];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::CoEqual => "co_equal",
            Self::Secondary => "secondary",
        }
    }

    /// Prominence rank; higher is more prominent.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Primary => 2,
            Self::CoEqual => 1,
            Self::Secondary => 0,
        }
    }
}

/// The kind of recently changed suspect a disable action targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecentChangeKind {
    /// A recently changed extension.
    Extension,
    /// A recently changed profile or layout.
    Profile,
}

impl RecentChangeKind {
    /// Every recent-change kind, in declaration order.
    pub const ALL: [Self; 2] = [Self::Extension, Self::Profile];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Extension => "extension",
            Self::Profile => "profile",
        }
    }

    /// The disable-action class that targets this kind of change.
    pub const fn disable_action_class(self) -> RecoveryActionClass {
        match self {
            Self::Extension => RecoveryActionClass::DisableRecentlyChangedExtension,
            Self::Profile => RecoveryActionClass::DisableRecentlyChangedProfile,
        }
    }
}

/// The overall crash-intake disposition of a screen — the headline reason it is or is not a clean,
/// exact, send-safe screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashIntakeStatus {
    /// Exact build, resolved symbols, a clean install state, a non-downgraded restore, and a send-safe
    /// intake: the screen presents full-fidelity crash identity and every offered intake mode.
    ExactReady,
    /// An approximate / unresolved build, a stale / partial symbolication, or a downgraded restore
    /// narrows the screen; the labels are shown as approximate rather than overclaimed.
    FidelityNarrowed,
    /// An active install advisory or extension quarantine narrows the screen.
    AdvisoryNarrowed,
    /// The selected intake mode would carry content that cannot safely leave the machine; the send is
    /// blocked.
    SendBlocked,
}

impl CrashIntakeStatus {
    /// Every crash-intake status, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExactReady,
        Self::FidelityNarrowed,
        Self::AdvisoryNarrowed,
        Self::SendBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactReady => "exact_ready",
            Self::FidelityNarrowed => "fidelity_narrowed",
            Self::AdvisoryNarrowed => "advisory_narrowed",
            Self::SendBlocked => "send_blocked",
        }
    }

    /// Highest presentation this status permits.
    pub const fn presentation_ceiling(self) -> RecoveryPresentation {
        match self {
            Self::ExactReady => RecoveryPresentation::ExactReady,
            Self::FidelityNarrowed | Self::AdvisoryNarrowed => RecoveryPresentation::Narrowed,
            Self::SendBlocked => RecoveryPresentation::SendBlocked,
        }
    }

    /// Whether the status itself needs the user to act.
    pub const fn requires_attention(self) -> bool {
        !matches!(self, Self::ExactReady)
    }

    /// Whether this status names blockers the user must reconcile before sending.
    pub const fn requires_blockers(self) -> bool {
        matches!(self, Self::SendBlocked)
    }
}

/// The presentation the recovery / intake gate publishes for a screen, highest to lowest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryPresentation {
    /// Exact build, resolved symbols, and a send-safe intake; crash identity and every intake mode are
    /// shown in full.
    ExactReady,
    /// The screen is shown but narrowed: an approximate / unresolved build, a stale / partial
    /// symbolication, a downgraded restore, or an active advisory needs attention. Recovery actions stay
    /// available and local-save stays first-class; fidelity labels are shown as approximate.
    Narrowed,
    /// A send mode is selected but the content cannot safely leave the machine; the screen warns and
    /// blocks the send before any packet leaves.
    SendBlocked,
}

impl RecoveryPresentation {
    /// Every presentation, highest to lowest.
    pub const ALL: [Self; 3] = [Self::ExactReady, Self::Narrowed, Self::SendBlocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactReady => "exact_ready",
            Self::Narrowed => "narrowed",
            Self::SendBlocked => "send_blocked",
        }
    }

    /// Rank for the fail-closed gate; higher is more revealing.
    pub const fn rank(self) -> u8 {
        match self {
            Self::ExactReady => 2,
            Self::Narrowed => 1,
            Self::SendBlocked => 0,
        }
    }

    /// Whether the gate narrowed or blocked the screen below a fully exact, send-safe screen.
    pub const fn requires_attention(self) -> bool {
        !matches!(self, Self::ExactReady)
    }

    /// Whether the screen must warn and block before a packet leaves the machine.
    pub const fn warns_before_send(self) -> bool {
        matches!(self, Self::SendBlocked)
    }
}

/// The weaker (lower-rank) of two presentations.
fn weaker(a: RecoveryPresentation, b: RecoveryPresentation) -> RecoveryPresentation {
    if b.rank() < a.rank() {
        b
    } else {
        a
    }
}

/// A headline reason the recovery / intake gate narrows or blocks a screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashIntakeDowngradeReason {
    /// Only an approximate build identity is available.
    ApproximateBuildIdentity,
    /// The build identity could not be resolved.
    UnresolvedBuildIdentity,
    /// Symbolication is stale, partial, or unresolved.
    StaleOrPartialSymbolication,
    /// The restore was downgraded from a stronger recorded class.
    RestoreProvenanceDowngraded,
    /// An advisory applies to this build.
    InstallAdvisoryActive,
    /// A quarantine applies to a suspect extension on this build.
    ExtensionQuarantineActive,
    /// The selected intake mode would carry content that cannot safely leave the machine.
    IntakeSendBlockedUnsafeContent,
}

impl CrashIntakeDowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ApproximateBuildIdentity,
        Self::UnresolvedBuildIdentity,
        Self::StaleOrPartialSymbolication,
        Self::RestoreProvenanceDowngraded,
        Self::InstallAdvisoryActive,
        Self::ExtensionQuarantineActive,
        Self::IntakeSendBlockedUnsafeContent,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApproximateBuildIdentity => "approximate_build_identity",
            Self::UnresolvedBuildIdentity => "unresolved_build_identity",
            Self::StaleOrPartialSymbolication => "stale_or_partial_symbolication",
            Self::RestoreProvenanceDowngraded => "restore_provenance_downgraded",
            Self::InstallAdvisoryActive => "install_advisory_active",
            Self::ExtensionQuarantineActive => "extension_quarantine_active",
            Self::IntakeSendBlockedUnsafeContent => "intake_send_blocked_unsafe_content",
        }
    }
}

/// A downstream surface that must ingest this registry and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryConsumerSurface {
    /// The active desktop crash-recovery screen.
    CrashRecoveryScreen,
    /// The Support Center's crash / recovery views.
    SupportCenter,
    /// The CLI / headless recovery path.
    CliHeadless,
    /// The issue-report / crash-intake packet.
    IssueReportPacket,
    /// The support export of the crash recovery.
    SupportExport,
}

impl RecoveryConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::CrashRecoveryScreen,
        Self::SupportCenter,
        Self::CliHeadless,
        Self::IssueReportPacket,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashRecoveryScreen => "crash_recovery_screen",
            Self::SupportCenter => "support_center",
            Self::CliHeadless => "cli_headless",
            Self::IssueReportPacket => "issue_report_packet",
            Self::SupportExport => "support_export",
        }
    }
}

/// One distinct, named recovery action offered by a screen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryAction {
    /// Stable action id.
    pub action_id: String,
    /// The action class.
    pub action_class: RecoveryActionClass,
    /// Command id bound to the action; must equal the class's command id.
    pub command_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing description.
    pub description: String,
    /// The rerun / discard effect; must equal the class's effect.
    pub effect: RecoveryActionEffect,
    /// The bounded blast radius; must equal the class's blast radius.
    pub blast_radius: BlastRadiusClass,
    /// Whether the action re-runs the prior session; must equal the effect's value.
    pub reruns_session: bool,
    /// Whether the action discards user-owned state; must be false.
    pub discards_state: bool,
    /// Whether the action requires explicit confirmation; must equal the class's value.
    pub requires_explicit_confirmation: bool,
    /// Whether the action enforces no-silent-rerun; must equal the class's session-reentry value.
    pub no_silent_rerun: bool,
    /// The recent-change this action targets, for a disable action.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub targets_change_ref: Option<String>,
    /// Plain-language disposition: what this action reruns, preserves, or defers.
    pub disposition_summary: String,
}

impl RecoveryAction {
    /// Whether the action carries its non-empty id, title, description, command id, and disposition.
    pub fn is_well_formed(&self) -> bool {
        !self.action_id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.description.trim().is_empty()
            && !self.command_id.trim().is_empty()
            && !self.disposition_summary.trim().is_empty()
    }

    /// Whether the recorded command id, effect, blast radius, rerun / discard flags, confirmation, and
    /// no-silent-rerun flag agree with the action class.
    pub fn is_class_consistent(&self) -> bool {
        self.command_id == self.action_class.command_id()
            && self.effect == self.action_class.effect()
            && self.blast_radius == self.action_class.blast_radius()
            && self.reruns_session == self.effect.reruns_session()
            && self.discards_state == self.effect.discards_state()
            && self.requires_explicit_confirmation
                == self.action_class.requires_explicit_confirmation()
            && self.no_silent_rerun == self.action_class.is_session_reentry()
    }
}

/// One typed intake-packet mode offered by a screen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntakePacketMode {
    /// The intake mode.
    pub mode: IntakeMode,
    /// How prominent this path is in the screen.
    pub prominence: PathProminence,
    /// Whether this path is offered for the current screen.
    pub enabled: bool,
    /// Whether this path is the one selected for this screen.
    pub selected: bool,
    /// Whether selecting this path causes the packet to leave the machine; must match the mode.
    pub leaves_machine: bool,
    /// The redaction posture applied to this mode.
    pub redaction_posture: RedactionExportPosture,
    /// Human-readable label (e.g. "Save crash report locally").
    pub label: String,
    /// Ref to the mode's destination wiring.
    pub destination_ref: String,
}

impl IntakePacketMode {
    /// Whether this is a local-save path.
    pub fn is_local_save(&self) -> bool {
        self.mode.is_local_save()
    }

    /// Whether the row's `leaves_machine` flag matches its mode.
    pub fn leaves_machine_consistent(&self) -> bool {
        self.leaves_machine == self.mode.leaves_machine()
    }

    /// Whether the mode carries its non-empty label and ref.
    pub fn is_well_formed(&self) -> bool {
        !self.label.trim().is_empty() && !self.destination_ref.trim().is_empty()
    }
}

/// A recently changed suspect surfaced as a candidate behind the crash loop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryRecentChange {
    /// Stable change id (referenced by the matching disable action).
    pub change_id: String,
    /// The kind of change.
    pub change_kind: RecentChangeKind,
    /// Opaque subject ref (extension id, profile id) safe for export.
    pub subject_ref: String,
    /// Reviewer-facing label that excludes raw paths and private content.
    pub display_label: String,
    /// Whether disabling the change is reversible.
    pub reversible: bool,
}

impl RecoveryRecentChange {
    /// Whether the change carries its non-empty id, subject ref, and label.
    pub fn is_well_formed(&self) -> bool {
        !self.change_id.trim().is_empty()
            && !self.subject_ref.trim().is_empty()
            && !self.display_label.trim().is_empty()
    }
}

/// One crash-recovery screen: the bounded recovery actions and the exact-build-aware crash intake for a
/// single crash-loop scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrashRecoveryScreen {
    /// Stable screen id.
    pub screen_id: String,
    /// Human-readable label for the screen.
    pub title: String,
    /// Visible crash-envelope id (kept visible in-product and in exports).
    pub crash_envelope_id: String,
    /// Visible, copyable exact-build id (kept visible in-product and in exports).
    pub exact_build_id: String,
    /// Whether the build id is copyable; must be true.
    pub build_id_copyable: bool,
    /// How precisely the running build is identified.
    pub build_identity_fidelity: BuildIdentityFidelity,
    /// How fully the crash's frames are symbolicated.
    pub symbolication_fidelity: SymbolicationFidelity,
    /// Ref to the symbolication report.
    pub symbolication_report_ref: String,
    /// The provenance class of the restore the failed launch attempted.
    pub restore_provenance: RestoreProvenanceClass,
    /// The install / advisory state for the running build.
    pub install_advisory_state: InstallAdvisoryState,
    /// The redaction / export posture; must equal the selected intake mode's posture.
    pub redaction_export_posture: RedactionExportPosture,
    /// Reviewer-facing summary of what triggered the screen.
    pub trigger_summary: String,
    /// Restart strikes observed in the active window.
    pub strike_count: u32,
    /// Automatic restart budget for the window.
    pub strike_budget: u32,
    /// Whether the invisible restart loop was suppressed by routing here; must be true.
    pub silent_restart_suppressed: bool,
    /// The intake mode selected for this screen; must equal the selected mode row.
    pub selected_intake_mode: IntakeMode,
    /// Overall crash-intake disposition; must equal the recomputed status.
    pub intake_status: CrashIntakeStatus,
    /// Presentation actually published after the gate; must equal the recomputed decision.
    pub presentation: RecoveryPresentation,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<CrashIntakeDowngradeReason>,
    /// Attestation that the screen claims an exact build; must equal the build fidelity's exact flag.
    pub claims_exact_build: bool,
    /// Attestation that the screen claims resolved symbolication; must equal the symbolication's flag.
    pub claims_resolved_symbolication: bool,
    /// Attestation that the local-save path is at least as prominent as every send mode; must equal the
    /// recomputed parity.
    pub local_save_first_class: bool,
    /// True when the screen warns and blocks before a packet leaves the machine; required iff
    /// send-blocked.
    pub blocked_before_send: bool,
    /// Whether any destructive (factory-reset / delete-state) action is offered; must be false.
    pub destructive_action_offered: bool,
    /// Attestation that no raw secret bodies, raw dumps, or raw payloads are carried; always true.
    pub raw_material_excluded: bool,
    /// The bounded, named recovery actions; at least the five core actions are required.
    #[serde(default)]
    pub recovery_actions: Vec<RecoveryAction>,
    /// The typed intake-packet modes; at least one enabled local-save mode is required.
    #[serde(default)]
    pub intake_modes: Vec<IntakePacketMode>,
    /// Recent changes surfaced as candidate suspects.
    #[serde(default)]
    pub recent_changes: Vec<RecoveryRecentChange>,
    /// Caveats attached to a narrowed or blocked screen.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// The blockers the user must reconcile before sending.
    #[serde(default)]
    pub blockers: Vec<String>,
    /// Ref to the crash envelope this screen projects.
    pub crash_envelope_ref: String,
    /// Ref to the crash / restore truth object this screen projects.
    pub source_of_truth_ref: String,
    /// One-step "Why this crash, on which build?" entrypoint; always present.
    pub explain_entrypoint_ref: String,
    /// The equivalent CLI / headless object id; always present.
    pub cli_object_ref: String,
    /// Ref to the conformance suite backing the screen.
    pub conformance_ref: String,
    /// Ref to the screen's supporting evidence.
    pub evidence_ref: String,
    /// Ref to the machine-readable intake receipt.
    pub intake_receipt_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl CrashRecoveryScreen {
    /// The recovery action with the given class, if present.
    pub fn action(&self, class: RecoveryActionClass) -> Option<&RecoveryAction> {
        self.recovery_actions
            .iter()
            .find(|a| a.action_class == class)
    }

    /// The selected intake mode row, if exactly one is declared.
    pub fn selected_intake(&self) -> Option<&IntakePacketMode> {
        let mut selected = self.intake_modes.iter().filter(|m| m.selected);
        let first = selected.next()?;
        if selected.next().is_some() {
            None
        } else {
            Some(first)
        }
    }

    /// The enabled local-save intake modes.
    pub fn local_save_modes(&self) -> impl Iterator<Item = &IntakePacketMode> {
        self.intake_modes
            .iter()
            .filter(|m| m.is_local_save() && m.enabled)
    }

    /// The enabled send (leaves-machine) intake modes.
    pub fn send_modes(&self) -> impl Iterator<Item = &IntakePacketMode> {
        self.intake_modes
            .iter()
            .filter(|m| m.leaves_machine && m.enabled)
    }

    /// The highest send-mode prominence rank, or `0` when no send mode is enabled.
    pub fn max_send_prominence_rank(&self) -> u8 {
        self.send_modes()
            .map(|m| m.prominence.rank())
            .max()
            .unwrap_or(0)
    }

    /// Whether an enabled local-save mode exists and is at least as prominent as every send mode.
    pub fn local_save_is_first_class(&self) -> bool {
        match self.local_save_modes().map(|m| m.prominence.rank()).max() {
            Some(local_rank) => local_rank >= self.max_send_prominence_rank(),
            None => false,
        }
    }

    /// Whether the selected send mode would carry content that cannot safely leave the machine.
    pub fn send_unsafe(&self) -> bool {
        match self.selected_intake() {
            Some(mode) if mode.leaves_machine => {
                !mode.redaction_posture.is_export_safe_off_machine()
            }
            _ => false,
        }
    }

    /// Whether the build identity is exact.
    pub fn build_is_exact(&self) -> bool {
        self.build_identity_fidelity.is_exact()
    }

    /// Whether the symbolication is resolved.
    pub fn symbolication_is_resolved(&self) -> bool {
        self.symbolication_fidelity.is_resolved()
    }

    /// Whether any fidelity axis (build, symbolication, restore provenance) is downgraded.
    pub fn has_fidelity_downgrade(&self) -> bool {
        !self.build_is_exact()
            || !self.symbolication_is_resolved()
            || self.restore_provenance.is_downgraded()
    }

    /// The crash-intake status recomputed from the screen's observed states.
    ///
    /// An unsafe send dominates a fidelity downgrade, which dominates an active advisory; a clean screen
    /// is exact-ready.
    pub fn computed_status(&self) -> CrashIntakeStatus {
        if self.send_unsafe() {
            CrashIntakeStatus::SendBlocked
        } else if self.has_fidelity_downgrade() {
            CrashIntakeStatus::FidelityNarrowed
        } else if self.install_advisory_state.narrows() {
            CrashIntakeStatus::AdvisoryNarrowed
        } else {
            CrashIntakeStatus::ExactReady
        }
    }

    /// Highest presentation the build and symbolication fidelity permit.
    pub fn fidelity_ceiling(&self) -> RecoveryPresentation {
        weaker(
            self.build_identity_fidelity.presentation_ceiling(),
            self.symbolication_fidelity.presentation_ceiling(),
        )
    }

    /// Highest presentation the restore provenance and install state permit.
    pub fn disposition_ceiling(&self) -> RecoveryPresentation {
        if self.restore_provenance.is_downgraded() || self.install_advisory_state.narrows() {
            RecoveryPresentation::Narrowed
        } else {
            RecoveryPresentation::ExactReady
        }
    }

    /// Highest presentation the intake sendability permits.
    pub fn sendability_ceiling(&self) -> RecoveryPresentation {
        if self.send_unsafe() {
            RecoveryPresentation::SendBlocked
        } else {
            RecoveryPresentation::ExactReady
        }
    }

    /// The presentation the gate permits this screen to publish.
    ///
    /// Lowers the clean baseline to the weakest of the fidelity, disposition, and sendability ceilings,
    /// so an approximate build, a stale symbolication, a downgraded restore, an active advisory, or
    /// unsafe content can never present a fuller claim than the inputs support.
    pub fn effective_presentation(&self) -> RecoveryPresentation {
        weaker(
            self.fidelity_ceiling(),
            weaker(self.disposition_ceiling(), self.sendability_ceiling()),
        )
    }

    /// The headline downgrade reasons recomputed from the screen's observed states.
    pub fn computed_downgrade_reasons(&self) -> Vec<CrashIntakeDowngradeReason> {
        CrashIntakeDowngradeReason::ALL
            .into_iter()
            .filter(|reason| match reason {
                CrashIntakeDowngradeReason::ApproximateBuildIdentity => {
                    self.build_identity_fidelity == BuildIdentityFidelity::ApproximateBuild
                }
                CrashIntakeDowngradeReason::UnresolvedBuildIdentity => {
                    self.build_identity_fidelity == BuildIdentityFidelity::UnresolvedBuild
                }
                CrashIntakeDowngradeReason::StaleOrPartialSymbolication => {
                    !self.symbolication_is_resolved()
                }
                CrashIntakeDowngradeReason::RestoreProvenanceDowngraded => {
                    self.restore_provenance.is_downgraded()
                }
                CrashIntakeDowngradeReason::InstallAdvisoryActive => {
                    self.install_advisory_state == InstallAdvisoryState::AdvisoryActive
                }
                CrashIntakeDowngradeReason::ExtensionQuarantineActive => {
                    self.install_advisory_state == InstallAdvisoryState::ExtensionQuarantineActive
                }
                CrashIntakeDowngradeReason::IntakeSendBlockedUnsafeContent => self.send_unsafe(),
            })
            .collect()
    }

    /// Whether the screen presents a fully exact, send-safe screen.
    pub fn is_exact_ready(&self) -> bool {
        self.effective_presentation() == RecoveryPresentation::ExactReady
    }

    /// Whether the screen carries its own non-empty one-step explain and CLI-equivalent refs.
    pub fn has_one_step_explainability(&self) -> bool {
        !self.explain_entrypoint_ref.trim().is_empty() && !self.cli_object_ref.trim().is_empty()
    }

    /// The recovery actions present, as a set of classes.
    fn action_classes(&self) -> BTreeSet<RecoveryActionClass> {
        self.recovery_actions
            .iter()
            .map(|a| a.action_class)
            .collect()
    }

    /// Whether the recorded status, presentation, reasons, parity, claims, and blocked flag agree with
    /// the gate.
    pub fn gate_consistent(&self) -> bool {
        let effective = self.effective_presentation();
        self.intake_status == self.computed_status()
            && self.presentation == effective
            && self.downgrade_reasons == self.computed_downgrade_reasons()
            && self.local_save_first_class == self.local_save_is_first_class()
            && self.blocked_before_send == effective.warns_before_send()
            && self.claims_exact_build == self.build_is_exact()
            && self.claims_resolved_symbolication == self.symbolication_is_resolved()
    }
}

/// One binding wiring a downstream surface to this registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer_surface: RecoveryConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Packet id this surface ingests.
    pub packet_id_ref: String,
    /// True when the surface ingests this registry rather than a parallel list.
    pub ingests_registry: bool,
    /// True when the surface preserves the recovery / intake vocabulary verbatim.
    pub preserves_recovery_vocabulary: bool,
    /// True when the surface preserves the screen and CLI object ids rather than reminting them.
    pub preserves_object_ids: bool,
    /// True when the surface preserves the exact-build and crash-envelope ids by reference.
    pub preserves_exact_build_lineage: bool,
    /// True when the surface keeps the local-save path at least as prominent as every send mode.
    pub local_save_first_class: bool,
    /// True when the surface narrows automatically as screens are narrowed or blocked.
    pub narrows_on_downgrade: bool,
    /// True when raw secret, dump, or payload material is excluded from the binding.
    pub raw_material_excluded: bool,
}

impl RecoveryConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && self.ingests_registry
            && self.preserves_recovery_vocabulary
            && self.preserves_object_ids
            && self.preserves_exact_build_lineage
            && self.local_save_first_class
            && self.narrows_on_downgrade
            && self.raw_material_excluded
            && !self.binding_ref.trim().is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5CrashIntakeAndRecoverySummary {
    /// Total crash-recovery screens.
    pub total_screens: usize,
    /// Screens that present a fully exact, send-safe screen.
    pub exact_ready_screens: usize,
    /// Screens the gate narrowed.
    pub narrowed_screens: usize,
    /// Screens the gate blocked from sending.
    pub send_blocked_screens: usize,
    /// Screens carrying a build / symbolication / restore fidelity downgrade.
    pub screens_with_fidelity_downgrade: usize,
    /// Screens carrying an active install advisory or extension quarantine.
    pub screens_with_active_advisory: usize,
    /// Screens that surface at least one recently changed suspect.
    pub screens_with_recent_change_suspects: usize,
    /// Screens that keep the local-save path first-class; equals total when the gate passes.
    pub local_save_first_class_screens: usize,
    /// Total recovery actions across all screens.
    pub total_recovery_actions: usize,
    /// Total intake modes across all screens.
    pub total_intake_modes: usize,
}

/// A redaction-safe export row projected from a crash-recovery screen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CrashIntakeAndRecoveryExportRow {
    /// Screen id.
    pub screen_id: String,
    /// Visible crash-envelope id.
    pub crash_envelope_id: String,
    /// Visible, copyable exact-build id.
    pub exact_build_id: String,
    /// Build-identity-fidelity token.
    pub build_identity_fidelity: String,
    /// Symbolication-fidelity token.
    pub symbolication_fidelity: String,
    /// Restore-provenance token.
    pub restore_provenance: String,
    /// Install / advisory-state token.
    pub install_advisory_state: String,
    /// Redaction / export-posture token.
    pub redaction_export_posture: String,
    /// Selected-intake-mode token.
    pub selected_intake_mode: String,
    /// Crash-intake-status token.
    pub intake_status: String,
    /// Published-presentation token.
    pub presentation: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Whether the screen claims an exact build.
    pub claims_exact_build: bool,
    /// Whether the screen claims resolved symbolication.
    pub claims_resolved_symbolication: bool,
    /// Whether the local-save path stays first-class.
    pub local_save_first_class: bool,
    /// Whether the screen warns and blocks before sending.
    pub blocked_before_send: bool,
    /// Recovery-action tokens, in order.
    pub recovery_actions: Vec<String>,
    /// Intake-mode tokens, in order.
    pub intake_modes: Vec<String>,
    /// One-step explain entrypoint ref.
    pub explain_entrypoint_ref: String,
    /// CLI / headless equivalent object id.
    pub cli_object_ref: String,
    /// Crash-envelope ref.
    pub crash_envelope_ref: String,
    /// Source-of-truth ref.
    pub source_of_truth_ref: String,
    /// Intake-receipt ref.
    pub intake_receipt_ref: String,
    /// Whether the screen presents as exact-ready.
    pub exact_ready: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the registry — the canonical crash-recovery index downstream
/// surfaces render instead of restating each crash scenario by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CrashIntakeAndRecoveryExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5CrashIntakeAndRecoveryExportRow>,
    /// Whether every screen's published presentation and decision agree with the gate.
    pub all_screens_gate_consistent: bool,
    /// Whether every screen keeps the local-save path first-class.
    pub all_local_save_first_class: bool,
    /// Screens that present as exact-ready.
    pub exact_ready_count: usize,
    /// Screens the gate narrowed.
    pub narrowed_count: usize,
    /// Screens the gate blocked from sending.
    pub send_blocked_count: usize,
}

/// The typed crash-intake-and-recovery registry packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5CrashIntakeAndRecovery {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed recovery-action vocabulary.
    pub recovery_action_classes: Vec<RecoveryActionClass>,
    /// Closed recovery-action-effect vocabulary.
    pub recovery_action_effects: Vec<RecoveryActionEffect>,
    /// Closed blast-radius vocabulary.
    pub blast_radius_classes: Vec<BlastRadiusClass>,
    /// Closed build-identity-fidelity vocabulary.
    pub build_identity_fidelities: Vec<BuildIdentityFidelity>,
    /// Closed symbolication-fidelity vocabulary.
    pub symbolication_fidelities: Vec<SymbolicationFidelity>,
    /// Closed restore-provenance vocabulary.
    pub restore_provenance_classes: Vec<RestoreProvenanceClass>,
    /// Closed install / advisory-state vocabulary.
    pub install_advisory_states: Vec<InstallAdvisoryState>,
    /// Closed redaction / export-posture vocabulary.
    pub redaction_export_postures: Vec<RedactionExportPosture>,
    /// Closed intake-mode vocabulary.
    pub intake_modes: Vec<IntakeMode>,
    /// Closed path-prominence vocabulary.
    pub path_prominences: Vec<PathProminence>,
    /// Closed recent-change-kind vocabulary.
    pub recent_change_kinds: Vec<RecentChangeKind>,
    /// Closed crash-intake-status vocabulary.
    pub intake_statuses: Vec<CrashIntakeStatus>,
    /// Closed presentation vocabulary.
    pub presentations: Vec<RecoveryPresentation>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<CrashIntakeDowngradeReason>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<RecoveryConsumerSurface>,
    /// Crash-recovery screens, one per crash-loop scenario worth recovering from.
    #[serde(default)]
    pub screens: Vec<CrashRecoveryScreen>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<RecoveryConsumerBinding>,
    /// Summary counts.
    pub summary: M5CrashIntakeAndRecoverySummary,
}

impl M5CrashIntakeAndRecovery {
    /// Returns the screen with the given id.
    pub fn screen(&self, screen_id: &str) -> Option<&CrashRecoveryScreen> {
        self.screens.iter().find(|s| s.screen_id == screen_id)
    }

    /// Screens that present as exact-ready.
    pub fn exact_ready_screens(&self) -> impl Iterator<Item = &CrashRecoveryScreen> {
        self.screens
            .iter()
            .filter(|s| s.effective_presentation() == RecoveryPresentation::ExactReady)
    }

    /// Screens the gate narrowed.
    pub fn narrowed_screens(&self) -> impl Iterator<Item = &CrashRecoveryScreen> {
        self.screens
            .iter()
            .filter(|s| s.effective_presentation() == RecoveryPresentation::Narrowed)
    }

    /// Screens the gate blocked from sending.
    pub fn send_blocked_screens(&self) -> impl Iterator<Item = &CrashRecoveryScreen> {
        self.screens
            .iter()
            .filter(|s| s.effective_presentation() == RecoveryPresentation::SendBlocked)
    }

    /// Whether a consumer binding preserves this registry for the given surface.
    pub fn has_binding_for(&self, surface: RecoveryConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every screen's recorded decision agrees with the gate.
    pub fn all_screens_gate_consistent(&self) -> bool {
        self.screens
            .iter()
            .all(CrashRecoveryScreen::gate_consistent)
    }

    /// Whether every screen keeps the local-save path first-class.
    pub fn all_local_save_first_class(&self) -> bool {
        self.screens
            .iter()
            .all(CrashRecoveryScreen::local_save_is_first_class)
    }

    /// Recomputes the summary block from the screens.
    pub fn computed_summary(&self) -> M5CrashIntakeAndRecoverySummary {
        let count_presentation = |decision: RecoveryPresentation| {
            self.screens
                .iter()
                .filter(|s| s.effective_presentation() == decision)
                .count()
        };
        let mut total_actions = 0usize;
        let mut total_modes = 0usize;
        for screen in &self.screens {
            total_actions += screen.recovery_actions.len();
            total_modes += screen.intake_modes.len();
        }
        M5CrashIntakeAndRecoverySummary {
            total_screens: self.screens.len(),
            exact_ready_screens: count_presentation(RecoveryPresentation::ExactReady),
            narrowed_screens: count_presentation(RecoveryPresentation::Narrowed),
            send_blocked_screens: count_presentation(RecoveryPresentation::SendBlocked),
            screens_with_fidelity_downgrade: self
                .screens
                .iter()
                .filter(|s| s.has_fidelity_downgrade())
                .count(),
            screens_with_active_advisory: self
                .screens
                .iter()
                .filter(|s| s.install_advisory_state.narrows())
                .count(),
            screens_with_recent_change_suspects: self
                .screens
                .iter()
                .filter(|s| !s.recent_changes.is_empty())
                .count(),
            local_save_first_class_screens: self
                .screens
                .iter()
                .filter(|s| s.local_save_is_first_class())
                .count(),
            total_recovery_actions: total_actions,
            total_intake_modes: total_modes,
        }
    }

    /// Produces the crash-recovery index downstream surfaces render instead of restating each crash
    /// scenario by hand.
    pub fn export_projection(&self) -> M5CrashIntakeAndRecoveryExportProjection {
        let rows = self
            .screens
            .iter()
            .map(|s| M5CrashIntakeAndRecoveryExportRow {
                screen_id: s.screen_id.clone(),
                crash_envelope_id: s.crash_envelope_id.clone(),
                exact_build_id: s.exact_build_id.clone(),
                build_identity_fidelity: s.build_identity_fidelity.as_str().to_owned(),
                symbolication_fidelity: s.symbolication_fidelity.as_str().to_owned(),
                restore_provenance: s.restore_provenance.as_str().to_owned(),
                install_advisory_state: s.install_advisory_state.as_str().to_owned(),
                redaction_export_posture: s.redaction_export_posture.as_str().to_owned(),
                selected_intake_mode: s.selected_intake_mode.as_str().to_owned(),
                intake_status: s.intake_status.as_str().to_owned(),
                presentation: s.presentation.as_str().to_owned(),
                downgrade_reasons: s
                    .downgrade_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                claims_exact_build: s.claims_exact_build,
                claims_resolved_symbolication: s.claims_resolved_symbolication,
                local_save_first_class: s.local_save_first_class,
                blocked_before_send: s.blocked_before_send,
                recovery_actions: s
                    .recovery_actions
                    .iter()
                    .map(|a| a.action_class.as_str().to_owned())
                    .collect(),
                intake_modes: s
                    .intake_modes
                    .iter()
                    .map(|m| m.mode.as_str().to_owned())
                    .collect(),
                explain_entrypoint_ref: s.explain_entrypoint_ref.clone(),
                cli_object_ref: s.cli_object_ref.clone(),
                crash_envelope_ref: s.crash_envelope_ref.clone(),
                source_of_truth_ref: s.source_of_truth_ref.clone(),
                intake_receipt_ref: s.intake_receipt_ref.clone(),
                exact_ready: s.is_exact_ready(),
                summary: format!(
                    "{}: crash {} on build {} ({}), restore {}, presentation {}",
                    s.screen_id,
                    s.crash_envelope_id,
                    s.exact_build_id,
                    s.build_identity_fidelity.as_str(),
                    s.restore_provenance.as_str(),
                    s.presentation.as_str()
                ),
            })
            .collect();
        M5CrashIntakeAndRecoveryExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_screens_gate_consistent: self.all_screens_gate_consistent(),
            all_local_save_first_class: self.all_local_save_first_class(),
            exact_ready_count: self.exact_ready_screens().count(),
            narrowed_count: self.narrowed_screens().count(),
            send_blocked_count: self.send_blocked_screens().count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact crash-recovery registry.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5CrashIntakeAndRecoverySupportExport {
        M5CrashIntakeAndRecoverySupportExport {
            record_kind: M5_CRASH_INTAKE_RECOVERY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_CRASH_INTAKE_RECOVERY_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_material_excluded: true,
            registry: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5CrashIntakeAndRecoveryViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_ids = BTreeSet::new();
        for screen in &self.screens {
            if !seen_ids.insert(screen.screen_id.clone()) {
                violations.push(M5CrashIntakeAndRecoveryViolation::DuplicateScreen {
                    screen_id: screen.screen_id.clone(),
                });
            }
            self.validate_screen(screen, &mut violations);
        }

        for surface in RecoveryConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(M5CrashIntakeAndRecoveryViolation::MissingConsumerBinding {
                    surface: surface.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5CrashIntakeAndRecoveryViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5CrashIntakeAndRecoveryViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5CrashIntakeAndRecoveryViolation>) {
        if self.schema_version != M5_CRASH_INTAKE_RECOVERY_SCHEMA_VERSION {
            violations.push(
                M5CrashIntakeAndRecoveryViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_CRASH_INTAKE_RECOVERY_RECORD_KIND {
            violations.push(M5CrashIntakeAndRecoveryViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5CrashIntakeAndRecoveryViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "recovery_action_classes",
                self.recovery_action_classes == RecoveryActionClass::ALL.to_vec(),
            ),
            (
                "recovery_action_effects",
                self.recovery_action_effects == RecoveryActionEffect::ALL.to_vec(),
            ),
            (
                "blast_radius_classes",
                self.blast_radius_classes == BlastRadiusClass::ALL.to_vec(),
            ),
            (
                "build_identity_fidelities",
                self.build_identity_fidelities == BuildIdentityFidelity::ALL.to_vec(),
            ),
            (
                "symbolication_fidelities",
                self.symbolication_fidelities == SymbolicationFidelity::ALL.to_vec(),
            ),
            (
                "restore_provenance_classes",
                self.restore_provenance_classes == RestoreProvenanceClass::ALL.to_vec(),
            ),
            (
                "install_advisory_states",
                self.install_advisory_states == InstallAdvisoryState::ALL.to_vec(),
            ),
            (
                "redaction_export_postures",
                self.redaction_export_postures == RedactionExportPosture::ALL.to_vec(),
            ),
            (
                "intake_modes",
                self.intake_modes == IntakeMode::ALL.to_vec(),
            ),
            (
                "path_prominences",
                self.path_prominences == PathProminence::ALL.to_vec(),
            ),
            (
                "recent_change_kinds",
                self.recent_change_kinds == RecentChangeKind::ALL.to_vec(),
            ),
            (
                "intake_statuses",
                self.intake_statuses == CrashIntakeStatus::ALL.to_vec(),
            ),
            (
                "presentations",
                self.presentations == RecoveryPresentation::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == CrashIntakeDowngradeReason::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == RecoveryConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations
                    .push(M5CrashIntakeAndRecoveryViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_screen(
        &self,
        screen: &CrashRecoveryScreen,
        violations: &mut Vec<M5CrashIntakeAndRecoveryViolation>,
    ) {
        for (field, value) in [
            ("screen_id", &screen.screen_id),
            ("title", &screen.title),
            ("crash_envelope_id", &screen.crash_envelope_id),
            ("exact_build_id", &screen.exact_build_id),
            ("symbolication_report_ref", &screen.symbolication_report_ref),
            ("trigger_summary", &screen.trigger_summary),
            ("crash_envelope_ref", &screen.crash_envelope_ref),
            ("source_of_truth_ref", &screen.source_of_truth_ref),
            ("explain_entrypoint_ref", &screen.explain_entrypoint_ref),
            ("cli_object_ref", &screen.cli_object_ref),
            ("conformance_ref", &screen.conformance_ref),
            ("evidence_ref", &screen.evidence_ref),
            ("intake_receipt_ref", &screen.intake_receipt_ref),
            ("note", &screen.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5CrashIntakeAndRecoveryViolation::EmptyField {
                    id: screen.screen_id.clone(),
                    field_name: field,
                });
            }
        }

        // The exact-build id is always visible and copyable, so a blocked user can quote it to support.
        if !screen.build_id_copyable {
            violations.push(M5CrashIntakeAndRecoveryViolation::BuildIdNotCopyable {
                screen_id: screen.screen_id.clone(),
            });
        }

        // The screen is only ever shown for a suppressed restart loop, never as an invisible retry.
        if !screen.silent_restart_suppressed {
            violations.push(
                M5CrashIntakeAndRecoveryViolation::SilentRestartNotSuppressed {
                    screen_id: screen.screen_id.clone(),
                },
            );
        }

        // No destructive factory-reset / delete-state action is ever offered.
        if screen.destructive_action_offered {
            violations.push(
                M5CrashIntakeAndRecoveryViolation::DestructiveActionOffered {
                    screen_id: screen.screen_id.clone(),
                },
            );
        }

        // No raw secret bodies, raw dumps, or raw payloads may be carried, ever.
        if !screen.raw_material_excluded {
            violations.push(M5CrashIntakeAndRecoveryViolation::RawMaterialNotExcluded {
                screen_id: screen.screen_id.clone(),
            });
        }

        // Every screen must carry its one-step "Why this crash, on which build?" entry and its
        // CLI / headless equivalent.
        if !screen.has_one_step_explainability() {
            violations.push(
                M5CrashIntakeAndRecoveryViolation::MissingOneStepExplainability {
                    screen_id: screen.screen_id.clone(),
                },
            );
        }

        self.validate_actions(screen, violations);
        self.validate_intake_modes(screen, violations);
        self.validate_recent_changes(screen, violations);
        self.validate_local_save_parity(screen, violations);
        self.validate_gate(screen, violations);
    }

    fn validate_actions(
        &self,
        screen: &CrashRecoveryScreen,
        violations: &mut Vec<M5CrashIntakeAndRecoveryViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for action in &screen.recovery_actions {
            if !seen.insert(action.action_class) {
                violations.push(M5CrashIntakeAndRecoveryViolation::DuplicateAction {
                    screen_id: screen.screen_id.clone(),
                    action_class: action.action_class.as_str(),
                });
            }
            if !action.is_well_formed() {
                violations.push(M5CrashIntakeAndRecoveryViolation::ActionIncomplete {
                    screen_id: screen.screen_id.clone(),
                    action_class: action.action_class.as_str(),
                });
            }
            if !action.is_class_consistent() {
                violations.push(M5CrashIntakeAndRecoveryViolation::ActionClassInconsistent {
                    screen_id: screen.screen_id.clone(),
                    action_class: action.action_class.as_str(),
                });
            }
            // A disable action must target a present recent change of the matching kind.
            if action.action_class.is_disable() {
                match &action.targets_change_ref {
                    Some(change_ref) => {
                        let matched = screen.recent_changes.iter().any(|c| {
                            c.change_id == *change_ref
                                && c.change_kind.disable_action_class() == action.action_class
                        });
                        if !matched {
                            violations.push(
                                M5CrashIntakeAndRecoveryViolation::DisableActionTargetMissing {
                                    screen_id: screen.screen_id.clone(),
                                    action_class: action.action_class.as_str(),
                                },
                            );
                        }
                    }
                    None => violations.push(
                        M5CrashIntakeAndRecoveryViolation::DisableActionTargetMissing {
                            screen_id: screen.screen_id.clone(),
                            action_class: action.action_class.as_str(),
                        },
                    ),
                }
            } else if action.targets_change_ref.is_some() {
                // A non-disable action must not pretend to target a suspect change.
                violations.push(M5CrashIntakeAndRecoveryViolation::ActionClassInconsistent {
                    screen_id: screen.screen_id.clone(),
                    action_class: action.action_class.as_str(),
                });
            }
        }

        // The five core actions are present on every screen, so the recovery choices are never collapsed
        // into one generic affordance.
        let present = screen.action_classes();
        for class in RecoveryActionClass::CORE {
            if !present.contains(&class) {
                violations.push(M5CrashIntakeAndRecoveryViolation::MissingCoreAction {
                    screen_id: screen.screen_id.clone(),
                    action_class: class.as_str(),
                });
            }
        }
    }

    fn validate_intake_modes(
        &self,
        screen: &CrashRecoveryScreen,
        violations: &mut Vec<M5CrashIntakeAndRecoveryViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for mode in &screen.intake_modes {
            if !seen.insert(mode.mode) {
                violations.push(M5CrashIntakeAndRecoveryViolation::DuplicateIntakeMode {
                    screen_id: screen.screen_id.clone(),
                    mode: mode.mode.as_str(),
                });
            }
            if !mode.is_well_formed() {
                violations.push(M5CrashIntakeAndRecoveryViolation::IntakeModeIncomplete {
                    screen_id: screen.screen_id.clone(),
                    mode: mode.mode.as_str(),
                });
            }
            if !mode.leaves_machine_consistent() {
                violations.push(
                    M5CrashIntakeAndRecoveryViolation::IntakeModeLeavesMachineMismatch {
                        screen_id: screen.screen_id.clone(),
                        mode: mode.mode.as_str(),
                    },
                );
            }
        }

        // Exactly one intake mode is selected, and the screen's selected mode and posture must match it.
        let selected: Vec<&IntakePacketMode> =
            screen.intake_modes.iter().filter(|m| m.selected).collect();
        match selected.as_slice() {
            [] => violations.push(M5CrashIntakeAndRecoveryViolation::NoSelectedIntakeMode {
                screen_id: screen.screen_id.clone(),
            }),
            [one] => {
                if one.mode != screen.selected_intake_mode {
                    violations.push(M5CrashIntakeAndRecoveryViolation::SelectedIntakeMismatch {
                        screen_id: screen.screen_id.clone(),
                    });
                }
                if one.redaction_posture != screen.redaction_export_posture {
                    violations.push(
                        M5CrashIntakeAndRecoveryViolation::SelectedIntakePostureMismatch {
                            screen_id: screen.screen_id.clone(),
                        },
                    );
                }
                if !one.enabled {
                    violations.push(M5CrashIntakeAndRecoveryViolation::SelectedIntakeDisabled {
                        screen_id: screen.screen_id.clone(),
                    });
                }
            }
            _ => violations.push(
                M5CrashIntakeAndRecoveryViolation::MultipleSelectedIntakeModes {
                    screen_id: screen.screen_id.clone(),
                },
            ),
        }
    }

    fn validate_recent_changes(
        &self,
        screen: &CrashRecoveryScreen,
        violations: &mut Vec<M5CrashIntakeAndRecoveryViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for change in &screen.recent_changes {
            if !seen.insert(change.change_id.clone()) {
                violations.push(M5CrashIntakeAndRecoveryViolation::DuplicateRecentChange {
                    screen_id: screen.screen_id.clone(),
                    change_id: change.change_id.clone(),
                });
            }
            if !change.is_well_formed() {
                violations.push(M5CrashIntakeAndRecoveryViolation::RecentChangeIncomplete {
                    screen_id: screen.screen_id.clone(),
                    change_id: change.change_id.clone(),
                });
            }
            // Every recent change must be matched by a disable action targeting it, so a suspect is never
            // surfaced without a bounded way to disable it.
            let has_disable = screen.recovery_actions.iter().any(|a| {
                a.action_class == change.change_kind.disable_action_class()
                    && a.targets_change_ref.as_deref() == Some(change.change_id.as_str())
            });
            if !has_disable {
                violations.push(
                    M5CrashIntakeAndRecoveryViolation::RecentChangeWithoutDisableAction {
                        screen_id: screen.screen_id.clone(),
                        change_id: change.change_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_local_save_parity(
        &self,
        screen: &CrashRecoveryScreen,
        violations: &mut Vec<M5CrashIntakeAndRecoveryViolation>,
    ) {
        // A local-save path is always offered and enabled, so saving the report locally is never hidden.
        if screen.local_save_modes().next().is_none() {
            violations.push(M5CrashIntakeAndRecoveryViolation::NoLocalSaveMode {
                screen_id: screen.screen_id.clone(),
            });
        }
        // The local-save path is at least as prominent as every send mode.
        if !screen.local_save_is_first_class() {
            violations.push(M5CrashIntakeAndRecoveryViolation::LocalSaveNotFirstClass {
                screen_id: screen.screen_id.clone(),
            });
        }
    }

    fn validate_gate(
        &self,
        screen: &CrashRecoveryScreen,
        violations: &mut Vec<M5CrashIntakeAndRecoveryViolation>,
    ) {
        // The recorded crash-intake status must equal the recomputed status.
        let computed_status = screen.computed_status();
        if screen.intake_status != computed_status {
            violations.push(M5CrashIntakeAndRecoveryViolation::IntakeStatusMismatch {
                screen_id: screen.screen_id.clone(),
                declared: screen.intake_status.as_str(),
                computed: computed_status.as_str(),
            });
        }

        // The published presentation must equal the gate's recomputed decision.
        let effective = screen.effective_presentation();
        if screen.presentation != effective {
            violations.push(M5CrashIntakeAndRecoveryViolation::OverstatedPresentation {
                screen_id: screen.screen_id.clone(),
                published: screen.presentation.as_str(),
                computed: effective.as_str(),
            });
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &screen.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(
                    M5CrashIntakeAndRecoveryViolation::DuplicateDowngradeReason {
                        screen_id: screen.screen_id.clone(),
                        reason: reason.as_str(),
                    },
                );
            }
        }
        if screen.downgrade_reasons != screen.computed_downgrade_reasons() {
            violations.push(
                M5CrashIntakeAndRecoveryViolation::DowngradeReasonsMismatch {
                    screen_id: screen.screen_id.clone(),
                },
            );
        }

        // The exact-build and resolved-symbolication claims must equal the observed fidelity, so the
        // screen never implies exact / resolved when only approximate / unresolved data exists.
        if screen.claims_exact_build != screen.build_is_exact() {
            violations.push(M5CrashIntakeAndRecoveryViolation::ExactBuildClaimMismatch {
                screen_id: screen.screen_id.clone(),
            });
        }
        if screen.claims_resolved_symbolication != screen.symbolication_is_resolved() {
            violations.push(
                M5CrashIntakeAndRecoveryViolation::ResolvedSymbolicationClaimMismatch {
                    screen_id: screen.screen_id.clone(),
                },
            );
        }

        // A screen that presents as exact-ready may never carry an approximate build, an unresolved
        // symbolication, a downgraded restore, an active advisory, or an unsafe send.
        if effective == RecoveryPresentation::ExactReady
            && (!screen.build_is_exact()
                || !screen.symbolication_is_resolved()
                || screen.restore_provenance.is_downgraded()
                || screen.install_advisory_state.narrows()
                || screen.send_unsafe()
                || !screen.claims_exact_build
                || !screen.claims_resolved_symbolication)
        {
            violations.push(M5CrashIntakeAndRecoveryViolation::OverclaimedFidelity {
                screen_id: screen.screen_id.clone(),
            });
        }

        // The local-save-first-class attestation must equal the recomputed parity.
        if screen.local_save_first_class != screen.local_save_is_first_class() {
            violations.push(
                M5CrashIntakeAndRecoveryViolation::LocalSaveAttestationMismatch {
                    screen_id: screen.screen_id.clone(),
                },
            );
        }

        // A send-blocked screen must warn before any packet leaves; a non-blocked one must not claim it.
        if screen.blocked_before_send != effective.warns_before_send() {
            violations.push(
                M5CrashIntakeAndRecoveryViolation::BlockedBeforeSendMismatch {
                    screen_id: screen.screen_id.clone(),
                },
            );
        }

        // A narrowed or blocked screen always carries a caveat naming why it is not cleanly exact-ready.
        if effective.requires_attention() && screen.caveats.is_empty() {
            violations.push(M5CrashIntakeAndRecoveryViolation::EmptyField {
                id: screen.screen_id.clone(),
                field_name: "caveats",
            });
        }

        // A send-blocked screen always names the blockers the user must reconcile.
        if computed_status.requires_blockers() && screen.blockers.is_empty() {
            violations.push(M5CrashIntakeAndRecoveryViolation::EmptyField {
                id: screen.screen_id.clone(),
                field_name: "blockers",
            });
        }
    }
}

/// A validation violation for the crash-intake-and-recovery registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5CrashIntakeAndRecoveryViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Screen or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A screen id appears more than once.
    DuplicateScreen {
        /// Duplicate screen id.
        screen_id: String,
    },
    /// A screen's build id is not marked copyable.
    BuildIdNotCopyable {
        /// Screen id.
        screen_id: String,
    },
    /// A screen does not suppress the invisible restart loop.
    SilentRestartNotSuppressed {
        /// Screen id.
        screen_id: String,
    },
    /// A screen offers a destructive factory-reset / delete-state action.
    DestructiveActionOffered {
        /// Screen id.
        screen_id: String,
    },
    /// A screen does not attest that raw secret / dump / payload material is excluded.
    RawMaterialNotExcluded {
        /// Screen id.
        screen_id: String,
    },
    /// A screen is missing its one-step explain entry or CLI-equivalent object id.
    MissingOneStepExplainability {
        /// Screen id.
        screen_id: String,
    },
    /// A screen lists a recovery action more than once.
    DuplicateAction {
        /// Screen id.
        screen_id: String,
        /// Action-class token.
        action_class: &'static str,
    },
    /// A recovery action is missing its id, title, description, command, or disposition.
    ActionIncomplete {
        /// Screen id.
        screen_id: String,
        /// Action-class token.
        action_class: &'static str,
    },
    /// A recovery action's command id, effect, blast radius, rerun / discard flags, confirmation, or
    /// no-silent-rerun flag disagrees with its class.
    ActionClassInconsistent {
        /// Screen id.
        screen_id: String,
        /// Action-class token.
        action_class: &'static str,
    },
    /// A disable action does not target a present recent change of the matching kind.
    DisableActionTargetMissing {
        /// Screen id.
        screen_id: String,
        /// Action-class token.
        action_class: &'static str,
    },
    /// A screen is missing one of the always-offered core actions.
    MissingCoreAction {
        /// Screen id.
        screen_id: String,
        /// Action-class token.
        action_class: &'static str,
    },
    /// A screen lists an intake mode more than once.
    DuplicateIntakeMode {
        /// Screen id.
        screen_id: String,
        /// Intake-mode token.
        mode: &'static str,
    },
    /// An intake mode is missing its label or ref.
    IntakeModeIncomplete {
        /// Screen id.
        screen_id: String,
        /// Intake-mode token.
        mode: &'static str,
    },
    /// An intake mode's `leaves_machine` flag disagrees with its mode.
    IntakeModeLeavesMachineMismatch {
        /// Screen id.
        screen_id: String,
        /// Intake-mode token.
        mode: &'static str,
    },
    /// A screen names no selected intake mode.
    NoSelectedIntakeMode {
        /// Screen id.
        screen_id: String,
    },
    /// A screen names more than one selected intake mode.
    MultipleSelectedIntakeModes {
        /// Screen id.
        screen_id: String,
    },
    /// The selected intake mode disagrees with the selected mode row.
    SelectedIntakeMismatch {
        /// Screen id.
        screen_id: String,
    },
    /// The screen-level redaction posture disagrees with the selected mode's posture.
    SelectedIntakePostureMismatch {
        /// Screen id.
        screen_id: String,
    },
    /// The selected intake mode is not enabled.
    SelectedIntakeDisabled {
        /// Screen id.
        screen_id: String,
    },
    /// A screen lists a recent change more than once.
    DuplicateRecentChange {
        /// Screen id.
        screen_id: String,
        /// Change id.
        change_id: String,
    },
    /// A recent change is missing its id, subject ref, or label.
    RecentChangeIncomplete {
        /// Screen id.
        screen_id: String,
        /// Change id.
        change_id: String,
    },
    /// A recent change is surfaced without a matching disable action.
    RecentChangeWithoutDisableAction {
        /// Screen id.
        screen_id: String,
        /// Change id.
        change_id: String,
    },
    /// A screen offers no enabled local-save path.
    NoLocalSaveMode {
        /// Screen id.
        screen_id: String,
    },
    /// A screen's local-save path is less prominent than a send mode.
    LocalSaveNotFirstClass {
        /// Screen id.
        screen_id: String,
    },
    /// The recorded crash-intake status disagrees with the recomputed status.
    IntakeStatusMismatch {
        /// Screen id.
        screen_id: String,
        /// Declared status token.
        declared: &'static str,
        /// Computed status token.
        computed: &'static str,
    },
    /// A screen publishes a presentation cleaner than the gate computes.
    OverstatedPresentation {
        /// Screen id.
        screen_id: String,
        /// Published presentation token.
        published: &'static str,
        /// Computed effective presentation token.
        computed: &'static str,
    },
    /// A screen lists a downgrade reason more than once.
    DuplicateDowngradeReason {
        /// Screen id.
        screen_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A screen's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Screen id.
        screen_id: String,
    },
    /// A screen's exact-build claim disagrees with its build fidelity.
    ExactBuildClaimMismatch {
        /// Screen id.
        screen_id: String,
    },
    /// A screen's resolved-symbolication claim disagrees with its symbolication fidelity.
    ResolvedSymbolicationClaimMismatch {
        /// Screen id.
        screen_id: String,
    },
    /// A screen presents as exact-ready while carrying an approximate / unresolved input.
    OverclaimedFidelity {
        /// Screen id.
        screen_id: String,
    },
    /// A screen's local-save-first-class attestation disagrees with the recomputed parity.
    LocalSaveAttestationMismatch {
        /// Screen id.
        screen_id: String,
    },
    /// A screen's blocked-before-send flag disagrees with the gate.
    BlockedBeforeSendMismatch {
        /// Screen id.
        screen_id: String,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer binding drops or remints registry truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the screens.
    SummaryMismatch,
}

impl fmt::Display for M5CrashIntakeAndRecoveryViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateScreen { screen_id } => write!(f, "duplicate screen id {screen_id}"),
            Self::BuildIdNotCopyable { screen_id } => {
                write!(f, "screen {screen_id} build id is not copyable")
            }
            Self::SilentRestartNotSuppressed { screen_id } => write!(
                f,
                "screen {screen_id} does not suppress the invisible restart loop"
            ),
            Self::DestructiveActionOffered { screen_id } => write!(
                f,
                "screen {screen_id} offers a destructive factory-reset / delete-state action"
            ),
            Self::RawMaterialNotExcluded { screen_id } => write!(
                f,
                "screen {screen_id} does not attest raw secret/dump/payload material is excluded"
            ),
            Self::MissingOneStepExplainability { screen_id } => write!(
                f,
                "screen {screen_id} is missing its one-step explain entry or CLI-equivalent object id"
            ),
            Self::DuplicateAction {
                screen_id,
                action_class,
            } => write!(f, "screen {screen_id} lists action {action_class} more than once"),
            Self::ActionIncomplete {
                screen_id,
                action_class,
            } => write!(
                f,
                "screen {screen_id} action {action_class} is missing its id, title, description, command, or disposition"
            ),
            Self::ActionClassInconsistent {
                screen_id,
                action_class,
            } => write!(
                f,
                "screen {screen_id} action {action_class} command/effect/blast-radius/flags disagree with its class"
            ),
            Self::DisableActionTargetMissing {
                screen_id,
                action_class,
            } => write!(
                f,
                "screen {screen_id} disable action {action_class} does not target a present recent change"
            ),
            Self::MissingCoreAction {
                screen_id,
                action_class,
            } => write!(f, "screen {screen_id} is missing core action {action_class}"),
            Self::DuplicateIntakeMode { screen_id, mode } => {
                write!(f, "screen {screen_id} lists intake mode {mode} more than once")
            }
            Self::IntakeModeIncomplete { screen_id, mode } => write!(
                f,
                "screen {screen_id} intake mode {mode} is missing its label or ref"
            ),
            Self::IntakeModeLeavesMachineMismatch { screen_id, mode } => write!(
                f,
                "screen {screen_id} intake mode {mode} leaves-machine flag disagrees with its mode"
            ),
            Self::NoSelectedIntakeMode { screen_id } => {
                write!(f, "screen {screen_id} names no selected intake mode")
            }
            Self::MultipleSelectedIntakeModes { screen_id } => {
                write!(f, "screen {screen_id} names more than one selected intake mode")
            }
            Self::SelectedIntakeMismatch { screen_id } => write!(
                f,
                "screen {screen_id} selected-intake-mode disagrees with the selected mode row"
            ),
            Self::SelectedIntakePostureMismatch { screen_id } => write!(
                f,
                "screen {screen_id} redaction posture disagrees with the selected mode's posture"
            ),
            Self::SelectedIntakeDisabled { screen_id } => {
                write!(f, "screen {screen_id} selected intake mode is not enabled")
            }
            Self::DuplicateRecentChange {
                screen_id,
                change_id,
            } => write!(f, "screen {screen_id} lists recent change {change_id} more than once"),
            Self::RecentChangeIncomplete {
                screen_id,
                change_id,
            } => write!(
                f,
                "screen {screen_id} recent change {change_id} is missing its id, subject ref, or label"
            ),
            Self::RecentChangeWithoutDisableAction {
                screen_id,
                change_id,
            } => write!(
                f,
                "screen {screen_id} recent change {change_id} has no matching disable action"
            ),
            Self::NoLocalSaveMode { screen_id } => {
                write!(f, "screen {screen_id} offers no enabled local-save path")
            }
            Self::LocalSaveNotFirstClass { screen_id } => write!(
                f,
                "screen {screen_id} local-save path is less prominent than a send mode"
            ),
            Self::IntakeStatusMismatch {
                screen_id,
                declared,
                computed,
            } => write!(
                f,
                "screen {screen_id} records intake status {declared} but the gate computes {computed}"
            ),
            Self::OverstatedPresentation {
                screen_id,
                published,
                computed,
            } => write!(
                f,
                "screen {screen_id} publishes presentation {published} but the gate computes {computed}"
            ),
            Self::DuplicateDowngradeReason { screen_id, reason } => {
                write!(f, "screen {screen_id} repeats downgrade reason {reason}")
            }
            Self::DowngradeReasonsMismatch { screen_id } => {
                write!(f, "screen {screen_id} downgrade reasons disagree with the gate")
            }
            Self::ExactBuildClaimMismatch { screen_id } => write!(
                f,
                "screen {screen_id} exact-build claim disagrees with its build fidelity"
            ),
            Self::ResolvedSymbolicationClaimMismatch { screen_id } => write!(
                f,
                "screen {screen_id} resolved-symbolication claim disagrees with its symbolication fidelity"
            ),
            Self::OverclaimedFidelity { screen_id } => write!(
                f,
                "screen {screen_id} presents as exact-ready while carrying an approximate / unresolved input"
            ),
            Self::LocalSaveAttestationMismatch { screen_id } => write!(
                f,
                "screen {screen_id} local-save-first-class attestation disagrees with the recomputed parity"
            ),
            Self::BlockedBeforeSendMismatch { screen_id } => write!(
                f,
                "screen {screen_id} blocked-before-send flag disagrees with the gate"
            ),
            Self::MissingConsumerBinding { surface } => {
                write!(f, "missing consumer binding for surface {surface}")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(f, "binding {binding_ref} does not preserve registry truth")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the screens"),
        }
    }
}

impl Error for M5CrashIntakeAndRecoveryViolation {}

/// Stable record-kind tag for [`M5CrashIntakeAndRecoverySupportExport`].
pub const M5_CRASH_INTAKE_RECOVERY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_crash_intake_and_recovery_support_export";

/// Support-export wrapper preserving the registry verbatim for support and evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CrashIntakeAndRecoverySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw secret, dump, or payload material is excluded.
    pub raw_material_excluded: bool,
    /// Exact registry preserved by the export.
    pub registry: M5CrashIntakeAndRecovery,
}

impl M5CrashIntakeAndRecoverySupportExport {
    /// Whether the export preserves the same packet id and a clean registry.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_CRASH_INTAKE_RECOVERY_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_CRASH_INTAKE_RECOVERY_SCHEMA_VERSION
            && self.packet_id_ref == self.registry.packet_id
            && self.raw_material_excluded
            && self.registry.validate().is_empty()
    }
}

/// Loads the embedded crash-intake-and-recovery registry packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5CrashIntakeAndRecovery`].
pub fn current_m5_crash_intake_and_recovery() -> Result<M5CrashIntakeAndRecovery, serde_json::Error>
{
    serde_json::from_str(M5_CRASH_INTAKE_RECOVERY_JSON)
}

#[cfg(test)]
mod tests;
