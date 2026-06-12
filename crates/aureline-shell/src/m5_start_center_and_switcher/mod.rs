//! Start Center / workspace-switcher parity packet for the new M5 surfaces.
//!
//! The stable v1 shell established a single canonical recent-work object model:
//! Start Center, `Open Recent`, and the in-workspace switcher all project the
//! same [`RecentWorkEntryRecord`] rows so a missing local folder, a relocated
//! workspace file, a disconnected SSH or managed target, or a partially
//! restorable import packet never renders as an ordinary reachable local open.
//!
//! This module carries that contract forward into the new M5 entry surfaces —
//! notebook, request/data, profiler, framework, companion, and sync-adjacent
//! paths — by proving **parity** between the two real projections rather than
//! inventing a per-feature launcher. It seeds one canonical recent-work entry
//! per M5 entry-target class (local folder, workspace file, multi-root
//! workspace, SSH target, container/devcontainer, managed workspace, import
//! packet, and bundle-backed entry) plus the failure exemplars the lane must
//! disclose (missing root, relocated workspace, stale target, remote host
//! unreachable, and partial restore), then projects each entry through both the
//! live [`crate::start_center`] recent-work projection and the live
//! [`crate::workspace_switcher`] projection and records, per row:
//!
//! - the canonical **target kind** and its compact label, so two distinct
//!   kinds are never collapsed into a generic "recent project" row;
//! - the **trust state**, asserted identical across both surfaces and equal to
//!   the canonical entry, so neither surface can silently widen trust on a
//!   probable, relocated, or unreachable target;
//! - the **restore availability** / restore class, asserted identical across
//!   both surfaces, so a layout-only or evidence-only (partial) restore reads
//!   the same whether cold-starting or switching from an active session;
//! - the **root / missing-path / missing-host state** plus the keyboard-complete
//!   pin/remove and reconnect/locate recovery actions both surfaces expose; and
//! - an **export-safe diagnostic** for every missing-root, relocated-workspace,
//!   stale-target, remote-host-unreachable, and partial-restore row, redacted to
//!   the target-kind label so support and help/export surfaces can cite the
//!   state without a raw path, host, or provider body.
//!
//! The records are inspectable, serde-serializable truth packets that carry no
//! credential bodies or raw provider payloads. They are consumed by the live
//! shell, the headless inspector (`aureline_shell_m5_start_center_and_switcher`),
//! the support-export wrapper, the docs page under
//! `docs/help/m5-start-center-and-switcher.md`, and the published audit under
//! `artifacts/ux/m5/start-center-and-switcher-audit.md`. The seeded projection is
//! deterministic so the checked-in fixtures under
//! `fixtures/aureline-shell/m5-start-center-and-switcher/` are bit-for-bit equal
//! to the output of [`seeded_m5_start_center_and_switcher_packet`].

use serde::{Deserialize, Serialize};

use aureline_workspace::{
    PortabilityClass, RecentWorkEntryRecord, RecentWorkEntryRecordKind, RecentWorkFailureState,
    RecentWorkRegistry, RecentWorkRegistryRecordKind, RecentWorkTargetState, RestoreAvailability,
    SafeRecoveryAction, TargetKind, TrustState,
};

use crate::start_center::{
    build_searchable_recent_work_rows, StartCenterRecentWorkPrivacyMode, StartCenterRecentWorkRow,
};
use crate::workspace_switcher::{build_switcher_rows, WorkspaceSwitcherRow};

/// Schema version exported with every record.
pub const M5_START_CENTER_AND_SWITCHER_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by UI, CLI, docs, and support export.
pub const M5_START_CENTER_AND_SWITCHER_SHARED_CONTRACT_REF: &str =
    "shell:m5_start_center_and_switcher:v1";

/// Stable record kind for [`M5StartCenterSwitcherPacket`] payloads.
pub const M5_START_CENTER_AND_SWITCHER_PACKET_RECORD_KIND: &str =
    "shell_m5_start_center_and_switcher_packet_record";

/// Stable record kind for [`M5StartCenterSwitcherRow`] payloads.
pub const M5_START_CENTER_AND_SWITCHER_ROW_RECORD_KIND: &str =
    "shell_m5_start_center_and_switcher_row_record";

/// Stable record kind for [`M5EntryDiagnostic`] payloads.
pub const M5_START_CENTER_AND_SWITCHER_DIAGNOSTIC_RECORD_KIND: &str =
    "shell_m5_start_center_and_switcher_diagnostic_record";

/// Stable record kind for [`M5StartCenterSwitcherSupportExport`] payloads.
pub const M5_START_CENTER_AND_SWITCHER_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_start_center_and_switcher_support_export_record";

/// Stable packet id used to pivot across surfaces.
pub const M5_START_CENTER_AND_SWITCHER_PACKET_ID: &str =
    "shell:m5_start_center_and_switcher:v1:default";

/// Published audit artifact that mirrors the packet for release evidence.
pub const M5_START_CENTER_AND_SWITCHER_PUBLISHED_AUDIT_REF: &str =
    "artifacts/ux/m5/start-center-and-switcher-audit.md";

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-06-11T00:00:00Z";

/// One M5 entry-target class the start-center and switcher surfaces must keep
/// distinct instead of collapsing into a generic recent-project row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EntrySurfaceClass {
    /// A local folder, file, or repository root opened in place.
    LocalFolder,
    /// A saved single-root workspace file.
    WorkspaceFile,
    /// A multi-root workspace (workset) manifest.
    MultiRootWorkspace,
    /// An SSH or remote-repository-backed target.
    SshTarget,
    /// A container or dev-container workspace.
    ContainerOrDevcontainer,
    /// A managed cloud workspace.
    ManagedWorkspace,
    /// An imported state package, handoff packet, or imported config root.
    ImportPacket,
    /// A template, prebuild, or bundle-backed entry row.
    BundleBackedEntry,
}

impl M5EntrySurfaceClass {
    /// Returns the stable schema token for this surface class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFolder => "local_folder",
            Self::WorkspaceFile => "workspace_file",
            Self::MultiRootWorkspace => "multi_root_workspace",
            Self::SshTarget => "ssh_target",
            Self::ContainerOrDevcontainer => "container_or_devcontainer",
            Self::ManagedWorkspace => "managed_workspace",
            Self::ImportPacket => "import_packet",
            Self::BundleBackedEntry => "bundle_backed_entry",
        }
    }

    /// Returns the reviewer-facing label for this surface class.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::LocalFolder => "Local folder",
            Self::WorkspaceFile => "Workspace file",
            Self::MultiRootWorkspace => "Multi-root workspace",
            Self::SshTarget => "SSH target",
            Self::ContainerOrDevcontainer => "Container / dev container",
            Self::ManagedWorkspace => "Managed workspace",
            Self::ImportPacket => "Import packet",
            Self::BundleBackedEntry => "Bundle-backed entry",
        }
    }

    /// Returns every required M5 entry-target class in canonical order.
    pub const fn required_classes() -> [Self; 8] {
        [
            Self::LocalFolder,
            Self::WorkspaceFile,
            Self::MultiRootWorkspace,
            Self::SshTarget,
            Self::ContainerOrDevcontainer,
            Self::ManagedWorkspace,
            Self::ImportPacket,
            Self::BundleBackedEntry,
        ]
    }

    /// Classifies a canonical [`TargetKind`] into its M5 entry surface class.
    ///
    /// Returns [`None`] for target kinds that are not part of the M5 entry
    /// surface (deep links and bare recovery checkpoints) so a caller never
    /// accidentally folds them into a generic row.
    pub const fn from_target_kind(target_kind: TargetKind) -> Option<Self> {
        match target_kind {
            TargetKind::LocalFile | TargetKind::LocalFolder | TargetKind::LocalRepoRoot => {
                Some(Self::LocalFolder)
            }
            TargetKind::WorkspaceManifest => Some(Self::WorkspaceFile),
            TargetKind::WorksetManifest => Some(Self::MultiRootWorkspace),
            TargetKind::RemoteRepository | TargetKind::SshWorkspace => Some(Self::SshTarget),
            TargetKind::ContainerWorkspace | TargetKind::DevcontainerWorkspace => {
                Some(Self::ContainerOrDevcontainer)
            }
            TargetKind::ManagedCloudWorkspace => Some(Self::ManagedWorkspace),
            TargetKind::PortableStatePackage
            | TargetKind::HandoffPacket
            | TargetKind::CompetitorConfigRoot => Some(Self::ImportPacket),
            TargetKind::TemplateOrPrebuildSnapshot => Some(Self::BundleBackedEntry),
            TargetKind::ReviewOrWorkItemDeepLink | TargetKind::RecoveryCheckpoint => None,
        }
    }
}

/// Root-resolution posture a row advertises before activation.
///
/// This keeps a healthy root, a missing local path, a relocated root, a stale
/// snapshot, and an unreachable remote host distinct so an empty or partial
/// open never silently reads as a healthy one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RootState {
    /// The root or target resolves and can be opened normally.
    RootResolved,
    /// The captured local path or mount is missing.
    MissingRoot,
    /// The root appears to have moved away from the stored identity.
    RelocatedRoot,
    /// Only cached/stale metadata is available for the target.
    StaleRoot,
    /// A remote, SSH, container, or managed host needs reconnect or reauth.
    RemoteHostUnreachable,
    /// Policy, quarantine, or an external lock blocks normal activation.
    BlockedRoot,
    /// The row cannot prove a stronger root state.
    UnknownRoot,
}

impl M5RootState {
    /// Returns the stable schema token for this root state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RootResolved => "root_resolved",
            Self::MissingRoot => "missing_root",
            Self::RelocatedRoot => "relocated_root",
            Self::StaleRoot => "stale_root",
            Self::RemoteHostUnreachable => "remote_host_unreachable",
            Self::BlockedRoot => "blocked_root",
            Self::UnknownRoot => "unknown_root",
        }
    }

    /// Returns `true` when the root resolves and needs no recovery path.
    pub const fn is_resolved(self) -> bool {
        matches!(self, Self::RootResolved)
    }

    fn from_failure(
        failure_state: RecentWorkFailureState,
        target_state: RecentWorkTargetState,
    ) -> Self {
        match failure_state {
            RecentWorkFailureState::Ready => Self::RootResolved,
            RecentWorkFailureState::MissingPath => Self::MissingRoot,
            RecentWorkFailureState::MovedRoot => Self::RelocatedRoot,
            RecentWorkFailureState::ReconnectRequired => Self::RemoteHostUnreachable,
            RecentWorkFailureState::InspectOnly => {
                if matches!(target_state, RecentWorkTargetState::StaleMetadata) {
                    Self::StaleRoot
                } else {
                    Self::RootResolved
                }
            }
            RecentWorkFailureState::Blocked => Self::BlockedRoot,
            RecentWorkFailureState::Unknown => Self::UnknownRoot,
        }
    }
}

/// Export-safe diagnostic class for an unavailable or partially restorable row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticClass {
    /// A local path or mount the row depends on is missing.
    MissingRoot,
    /// The workspace root has been relocated away from its stored identity.
    RelocatedWorkspace,
    /// Only stale cached metadata is available for the target.
    StaleTarget,
    /// A remote, SSH, container, or managed host is unreachable.
    RemoteHostUnreachable,
    /// The target is reachable but only a partial (layout/evidence) restore is
    /// available.
    PartialRestore,
}

impl M5DiagnosticClass {
    /// Returns the stable schema token for this diagnostic class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingRoot => "missing_root",
            Self::RelocatedWorkspace => "relocated_workspace",
            Self::StaleTarget => "stale_target",
            Self::RemoteHostUnreachable => "remote_host_unreachable",
            Self::PartialRestore => "partial_restore",
        }
    }

    /// Returns the reviewer-facing label for this diagnostic class.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::MissingRoot => "Missing root",
            Self::RelocatedWorkspace => "Relocated workspace",
            Self::StaleTarget => "Stale target",
            Self::RemoteHostUnreachable => "Remote host unreachable",
            Self::PartialRestore => "Partial restore",
        }
    }

    /// Returns every diagnostic class the lane must be able to disclose.
    pub const fn required_classes() -> [Self; 5] {
        [
            Self::MissingRoot,
            Self::RelocatedWorkspace,
            Self::StaleTarget,
            Self::RemoteHostUnreachable,
            Self::PartialRestore,
        ]
    }

    fn classify(
        root_state: M5RootState,
        restore_availability: RestoreAvailability,
    ) -> Option<Self> {
        match root_state {
            M5RootState::MissingRoot => Some(Self::MissingRoot),
            M5RootState::RelocatedRoot => Some(Self::RelocatedWorkspace),
            M5RootState::StaleRoot => Some(Self::StaleTarget),
            M5RootState::RemoteHostUnreachable => Some(Self::RemoteHostUnreachable),
            M5RootState::RootResolved => {
                if matches!(
                    restore_availability,
                    RestoreAvailability::LayoutOnly | RestoreAvailability::EvidenceOnly
                ) {
                    Some(Self::PartialRestore)
                } else {
                    None
                }
            }
            M5RootState::BlockedRoot | M5RootState::UnknownRoot => None,
        }
    }
}

/// Cross-surface parity flags computed from the two live projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceParity {
    /// Both surfaces and the canonical entry agree on the target kind.
    pub target_kind_parity: bool,
    /// Both surfaces and the canonical entry agree on the trust state.
    pub trust_parity: bool,
    /// Both surfaces and the canonical entry agree on the restore class.
    pub restore_parity: bool,
    /// Both surfaces agree on the unavailable-target failure state.
    pub failure_parity: bool,
    /// Both surfaces expose the same setup-later / recovery action set.
    pub recovery_action_parity: bool,
}

impl M5SurfaceParity {
    /// Returns `true` when every parity dimension holds.
    pub const fn is_complete(&self) -> bool {
        self.target_kind_parity
            && self.trust_parity
            && self.restore_parity
            && self.failure_parity
            && self.recovery_action_parity
    }
}

/// One M5 entry row projected through both the Start Center and the in-workspace
/// switcher, with target-kind, trust, restore, and missing-root truth recorded
/// for parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StartCenterSwitcherRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable upstream recent-work entry id shared by both surfaces.
    pub recent_work_id: String,
    /// Project, workspace, or target label.
    pub presentation_label: String,
    /// Redaction-aware path, host, provider, or target subtitle.
    pub location_subtitle: Option<String>,
    /// M5 entry-target class kept distinct from a generic recent row.
    pub surface_class: M5EntrySurfaceClass,
    /// Canonical target kind from the workspace entry model.
    pub target_kind: TargetKind,
    /// Compact target-kind label shared by Start Center and switcher rows.
    pub target_kind_label: String,
    /// Canonical trust posture advertised by the entry.
    pub canonical_trust_state: TrustState,
    /// Trust posture the Start Center row displayed.
    pub start_center_trust_state: TrustState,
    /// Trust posture the switcher row displayed.
    pub switcher_trust_state: TrustState,
    /// Canonical restore availability advertised by the entry.
    pub canonical_restore_availability: RestoreAvailability,
    /// Restore availability the Start Center row displayed.
    pub start_center_restore_availability: RestoreAvailability,
    /// Restore availability the switcher row displayed.
    pub switcher_restore_availability: RestoreAvailability,
    /// Raw target state captured for this entry.
    pub target_state: RecentWorkTargetState,
    /// Shared unavailable-target classification.
    pub failure_state: RecentWorkFailureState,
    /// Root-resolution posture derived from the failure state.
    pub root_state: M5RootState,
    /// Whether the row appears in the Start Center recent-work projection.
    pub present_in_start_center: bool,
    /// Whether the row appears in the in-workspace switcher projection.
    pub present_in_switcher: bool,
    /// Whether the row is pinned.
    pub pinned: bool,
    /// Whether both surfaces expose a keyboard-complete action set.
    pub keyboard_complete: bool,
    /// Whether pin/unpin is available on the row.
    pub pin_action_available: bool,
    /// Whether remove-from-recents is available on the row.
    pub remove_action_available: bool,
    /// Whether a reconnect or locate recovery path is available on the row.
    pub reconnect_or_locate_available: bool,
    /// Recovery actions the Start Center row exposes.
    pub start_center_safe_actions: Vec<SafeRecoveryAction>,
    /// Recovery actions the switcher row exposes.
    pub switcher_safe_actions: Vec<SafeRecoveryAction>,
    /// Cross-surface parity flags.
    pub parity: M5SurfaceParity,
    /// Diagnostic class when the row is unavailable or only partially
    /// restorable, otherwise [`None`].
    pub diagnostic_class: Option<M5DiagnosticClass>,
}

impl M5StartCenterSwitcherRow {
    /// Returns `true` when the row holds full cross-surface parity.
    pub const fn parity_complete(&self) -> bool {
        self.parity.is_complete()
    }

    /// Returns `true` when neither surface widened trust beyond the canonical
    /// entry.
    pub fn trust_not_widened(&self) -> bool {
        self.start_center_trust_state == self.canonical_trust_state
            && self.switcher_trust_state == self.canonical_trust_state
    }

    /// Returns `true` when the row's root resolves cleanly.
    pub const fn root_resolved(&self) -> bool {
        self.root_state.is_resolved()
    }

    fn from_projections(
        start_center: &StartCenterRecentWorkRow,
        switcher: &WorkspaceSwitcherRow,
        canonical: &RecentWorkEntryRecord,
        surface_class: M5EntrySurfaceClass,
    ) -> Self {
        let failure_state = start_center.failure_state;
        let root_state = M5RootState::from_failure(failure_state, canonical.target_state);
        let diagnostic_class =
            M5DiagnosticClass::classify(root_state, canonical.restore_availability);

        let target_kind_parity = start_center.target_kind == canonical.target_kind
            && switcher.target_kind == canonical.target_kind;
        let trust_parity = start_center.trust_state == canonical.trust_state
            && switcher.trust_state == canonical.trust_state;
        let restore_parity = start_center.restore_availability == canonical.restore_availability
            && switcher.restore_availability == canonical.restore_availability;
        let failure_parity = start_center.failure_state == switcher.failure_state;
        let recovery_action_parity =
            start_center.safe_recovery_actions == switcher.safe_recovery_actions;

        let pin_action_available = action_present(&start_center.safe_recovery_actions, |action| {
            matches!(action, SafeRecoveryAction::Pin | SafeRecoveryAction::Unpin)
        });
        let remove_action_available = start_center
            .safe_recovery_actions
            .contains(&SafeRecoveryAction::RemoveFromRecents);
        let reconnect_or_locate_available =
            action_present(&start_center.safe_recovery_actions, |action| {
                matches!(
                    action,
                    SafeRecoveryAction::LocateMissingTarget
                        | SafeRecoveryAction::Reconnect
                        | SafeRecoveryAction::Reauth
                        | SafeRecoveryAction::OpenReadOnlyCachedView
                        | SafeRecoveryAction::RetryLater
                )
            });
        let keyboard_complete = !start_center.safe_recovery_actions.is_empty()
            && !switcher.safe_recovery_actions.is_empty();

        Self {
            record_kind: M5_START_CENTER_AND_SWITCHER_ROW_RECORD_KIND.to_owned(),
            schema_version: M5_START_CENTER_AND_SWITCHER_SCHEMA_VERSION,
            shared_contract_ref: M5_START_CENTER_AND_SWITCHER_SHARED_CONTRACT_REF.to_owned(),
            recent_work_id: canonical.recent_work_id.clone(),
            presentation_label: canonical.presentation_label.clone(),
            location_subtitle: canonical.presentation_subtitle.clone(),
            surface_class,
            target_kind: canonical.target_kind,
            target_kind_label: canonical.target_kind.surface_label().to_owned(),
            canonical_trust_state: canonical.trust_state,
            start_center_trust_state: start_center.trust_state,
            switcher_trust_state: switcher.trust_state,
            canonical_restore_availability: canonical.restore_availability,
            start_center_restore_availability: start_center.restore_availability,
            switcher_restore_availability: switcher.restore_availability,
            target_state: canonical.target_state,
            failure_state,
            root_state,
            present_in_start_center: true,
            present_in_switcher: true,
            pinned: canonical.pinned,
            keyboard_complete,
            pin_action_available,
            remove_action_available,
            reconnect_or_locate_available,
            start_center_safe_actions: start_center.safe_recovery_actions.clone(),
            switcher_safe_actions: switcher.safe_recovery_actions.clone(),
            parity: M5SurfaceParity {
                target_kind_parity,
                trust_parity,
                restore_parity,
                failure_parity,
                recovery_action_parity,
            },
            diagnostic_class,
        }
    }
}

fn action_present(
    actions: &[SafeRecoveryAction],
    predicate: impl Fn(SafeRecoveryAction) -> bool,
) -> bool {
    actions.iter().copied().any(predicate)
}

/// Export-safe diagnostic for one unavailable or partially restorable row.
///
/// The diagnostic redacts the row to its target-kind label so support, help,
/// and release-evidence surfaces can cite the state without a raw path, host,
/// or provider body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EntryDiagnostic {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the diagnostic.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable diagnostic id.
    pub diagnostic_id: String,
    /// Recent-work entry id the diagnostic refers to.
    pub recent_work_id: String,
    /// Diagnostic class.
    pub diagnostic_class: M5DiagnosticClass,
    /// M5 entry-target class.
    pub surface_class: M5EntrySurfaceClass,
    /// Canonical target kind.
    pub target_kind: TargetKind,
    /// Redacted location: the target-kind label only, never a raw path or host.
    pub redacted_location: String,
    /// Raw target state captured for the entry.
    pub target_state: RecentWorkTargetState,
    /// Shared unavailable-target classification.
    pub failure_state: RecentWorkFailureState,
    /// Root-resolution posture.
    pub root_state: M5RootState,
    /// Trust posture (never widened by the diagnostic).
    pub trust_state: TrustState,
    /// Restore availability.
    pub restore_availability: RestoreAvailability,
    /// Recovery actions both surfaces expose for the row.
    pub recovery_actions: Vec<SafeRecoveryAction>,
    /// Always `true`: the diagnostic carries no raw path, host, or credential
    /// body.
    pub export_safe: bool,
}

impl M5EntryDiagnostic {
    fn from_row(row: &M5StartCenterSwitcherRow, diagnostic_class: M5DiagnosticClass) -> Self {
        Self {
            record_kind: M5_START_CENTER_AND_SWITCHER_DIAGNOSTIC_RECORD_KIND.to_owned(),
            schema_version: M5_START_CENTER_AND_SWITCHER_SCHEMA_VERSION,
            shared_contract_ref: M5_START_CENTER_AND_SWITCHER_SHARED_CONTRACT_REF.to_owned(),
            diagnostic_id: format!(
                "diagnostic:m5_start_center_and_switcher:{}:{}",
                diagnostic_class.as_str(),
                row.recent_work_id
            ),
            recent_work_id: row.recent_work_id.clone(),
            diagnostic_class,
            surface_class: row.surface_class,
            target_kind: row.target_kind,
            redacted_location: row.target_kind_label.clone(),
            target_state: row.target_state,
            failure_state: row.failure_state,
            root_state: row.root_state,
            trust_state: row.canonical_trust_state,
            restore_availability: row.canonical_restore_availability,
            recovery_actions: row.start_center_safe_actions.clone(),
            export_safe: true,
        }
    }
}

/// Surface-class coverage summary across the packet's rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceClassCoverageSummary {
    /// Surface classes covered by the packet, in canonical order.
    pub covered_classes: Vec<M5EntrySurfaceClass>,
    /// Total number of required surface classes.
    pub total_required_classes: usize,
    /// Diagnostic classes covered by the packet, in canonical order.
    pub covered_diagnostic_classes: Vec<M5DiagnosticClass>,
}

impl SurfaceClassCoverageSummary {
    /// Builds the coverage summary from a packet's parity rows.
    pub fn from_rows(rows: &[M5StartCenterSwitcherRow]) -> Self {
        let mut covered_classes = Vec::new();
        for class in M5EntrySurfaceClass::required_classes() {
            if rows.iter().any(|row| row.surface_class == class) {
                covered_classes.push(class);
            }
        }
        let mut covered_diagnostic_classes = Vec::new();
        for class in M5DiagnosticClass::required_classes() {
            if rows.iter().any(|row| row.diagnostic_class == Some(class)) {
                covered_diagnostic_classes.push(class);
            }
        }
        Self {
            covered_classes,
            total_required_classes: M5EntrySurfaceClass::required_classes().len(),
            covered_diagnostic_classes,
        }
    }

    /// Returns `true` when every required surface class is covered.
    pub fn covers_every_class(&self) -> bool {
        self.covered_classes.len() == self.total_required_classes
    }
}

/// Start Center / workspace-switcher parity packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StartCenterSwitcherPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// Parity rows in canonical seed order.
    pub rows: Vec<M5StartCenterSwitcherRow>,
    /// Export-safe diagnostics for unavailable / partial rows.
    pub diagnostics: Vec<M5EntryDiagnostic>,
    /// Surface-class coverage summary across the rows.
    pub surface_class_coverage: SurfaceClassCoverageSummary,
    /// True when every row appears in both the Start Center and switcher.
    pub all_rows_in_both_surfaces: bool,
    /// True when no two distinct target kinds collapse into one surface class.
    pub no_target_kind_collapsed: bool,
    /// True when no surface widened trust beyond the canonical entry.
    pub no_trust_widened: bool,
    /// True when every row holds full cross-surface parity.
    pub full_parity: bool,
    /// Release-evidence refs that ingest the packet.
    pub release_evidence_refs: Vec<String>,
    /// Readiness review refs that consume the packet.
    pub readiness_review_refs: Vec<String>,
    /// Published audit artifact that mirrors the packet.
    pub published_audit_ref: String,
    /// Docs/help refs the packet reopens from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the packet reopens from.
    pub support_export_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl M5StartCenterSwitcherPacket {
    /// Returns the row count for the packet.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns `true` when every required surface class is covered.
    pub fn covers_every_class(&self) -> bool {
        self.surface_class_coverage.covers_every_class()
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "packet: id={}, rows={}, classes={}/{}, diagnostics={}",
            self.packet_id,
            self.rows.len(),
            self.surface_class_coverage.covered_classes.len(),
            self.surface_class_coverage.total_required_classes,
            self.diagnostics.len(),
        ));
        lines.push(format!(
            "all_rows_in_both_surfaces={} no_target_kind_collapsed={} no_trust_widened={} full_parity={}",
            self.all_rows_in_both_surfaces,
            self.no_target_kind_collapsed,
            self.no_trust_widened,
            self.full_parity,
        ));
        for row in &self.rows {
            lines.push(format!(
                "  {} [{}] kind={} trust={} restore={} root={} parity={}{}",
                row.recent_work_id,
                row.surface_class.as_str(),
                row.target_kind.as_str(),
                row.canonical_trust_state.as_str(),
                row.canonical_restore_availability.as_str(),
                row.root_state.as_str(),
                row.parity_complete(),
                match row.diagnostic_class {
                    Some(class) => format!(" diagnostic={}", class.as_str()),
                    None => String::new(),
                },
            ));
        }
        lines
    }

    /// Renders the markdown audit artifact for the packet.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Start Center and workspace-switcher parity for M5 entry surfaces\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_start_center_and_switcher`](../../../crates/aureline-shell/src/m5_start_center_and_switcher/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- markdown > \\\n  artifacts/ux/m5/start-center-and-switcher-audit.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!("- Rows: {}\n", self.rows.len()));
        out.push_str(&format!(
            "- Surface classes covered: {}/{}\n",
            self.surface_class_coverage.covered_classes.len(),
            self.surface_class_coverage.total_required_classes
        ));
        out.push_str(&format!("- Diagnostics: {}\n", self.diagnostics.len()));
        out.push_str(&format!(
            "- All rows in both surfaces: {}\n",
            self.all_rows_in_both_surfaces
        ));
        out.push_str(&format!(
            "- No target kind collapsed: {}\n",
            self.no_target_kind_collapsed
        ));
        out.push_str(&format!("- No trust widened: {}\n", self.no_trust_widened));
        out.push_str(&format!("- Full parity: {}\n", self.full_parity));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Parity rows\n\n");
        out.push_str(
            "| Surface class | Target kind | Trust | Restore | Root state | In both | Parity | Diagnostic |\n",
        );
        out.push_str("|---|---|---|---|---|:---:|:---:|---|\n");
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | {} | {} | {} |\n",
                row.surface_class.display_label(),
                row.target_kind.as_str(),
                row.canonical_trust_state.as_str(),
                row.canonical_restore_availability.as_str(),
                row.root_state.as_str(),
                if row.present_in_start_center && row.present_in_switcher {
                    "yes"
                } else {
                    "NO"
                },
                if row.parity_complete() {
                    "full"
                } else {
                    "BROKEN"
                },
                match row.diagnostic_class {
                    Some(class) => format!("`{}`", class.as_str()),
                    None => "—".to_owned(),
                },
            ));
        }
        out.push('\n');

        out.push_str("## Export-safe diagnostics\n\n");
        out.push_str("| Diagnostic | Surface class | Redacted location | Trust | Restore | Recovery actions |\n");
        out.push_str("|---|---|---|---|---|---|\n");
        for diagnostic in &self.diagnostics {
            out.push_str(&format!(
                "| {} | {} | {} | `{}` | `{}` | {} |\n",
                diagnostic.diagnostic_class.display_label(),
                diagnostic.surface_class.display_label(),
                diagnostic.redacted_location,
                diagnostic.trust_state.as_str(),
                diagnostic.restore_availability.as_str(),
                diagnostic
                    .recovery_actions
                    .iter()
                    .map(|action| format!("`{}`", action.as_str()))
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }
        out.push('\n');

        out
    }
}

/// Support-export wrapper that quotes the packet plus every stable id reviewers
/// need to pivot across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StartCenterSwitcherSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the wrapper.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: M5StartCenterSwitcherPacket,
    /// Stable packet id, recent-work ids, and diagnostic ids in deterministic
    /// order.
    pub case_ids: Vec<String>,
}

impl M5StartCenterSwitcherSupportExport {
    /// Builds the support-export wrapper for a packet.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: M5StartCenterSwitcherPacket,
    ) -> Self {
        let mut case_ids = Vec::new();
        case_ids.push(packet.packet_id.clone());
        for row in &packet.rows {
            case_ids.push(row.recent_work_id.clone());
        }
        for diagnostic in &packet.diagnostics {
            case_ids.push(diagnostic.diagnostic_id.clone());
        }
        Self {
            record_kind: M5_START_CENTER_AND_SWITCHER_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_START_CENTER_AND_SWITCHER_SCHEMA_VERSION,
            shared_contract_ref: M5_START_CENTER_AND_SWITCHER_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            case_ids,
        }
    }
}

/// Validation error produced by [`validate_m5_start_center_and_switcher_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum M5StartCenterSwitcherValidationError {
    /// A required surface class has no row in the packet.
    MissingSurfaceClass {
        /// Surface class with the missing row.
        surface_class: String,
    },
    /// A required diagnostic class has no diagnostic in the packet.
    MissingDiagnosticClass {
        /// Diagnostic class with the missing diagnostic.
        diagnostic_class: String,
    },
    /// A row is missing from the Start Center projection.
    RowMissingFromStartCenter {
        /// Row that violated the invariant.
        recent_work_id: String,
    },
    /// A row is missing from the switcher projection.
    RowMissingFromSwitcher {
        /// Row that violated the invariant.
        recent_work_id: String,
    },
    /// A row's surface class does not match its canonical target kind, i.e. two
    /// distinct kinds were collapsed.
    SurfaceClassCollapsed {
        /// Row that violated the invariant.
        recent_work_id: String,
    },
    /// A row does not hold target-kind parity across the two surfaces.
    TargetKindParityBroken {
        /// Row that violated the invariant.
        recent_work_id: String,
    },
    /// A row does not hold trust parity across the two surfaces.
    TrustParityBroken {
        /// Row that violated the invariant.
        recent_work_id: String,
    },
    /// A surface widened trust beyond the canonical entry.
    TrustWidened {
        /// Row that violated the invariant.
        recent_work_id: String,
    },
    /// A row does not hold restore parity across the two surfaces.
    RestoreParityBroken {
        /// Row that violated the invariant.
        recent_work_id: String,
    },
    /// A row does not hold failure-state parity across the two surfaces.
    FailureParityBroken {
        /// Row that violated the invariant.
        recent_work_id: String,
    },
    /// A row does not expose a keyboard-complete action set.
    KeyboardIncomplete {
        /// Row that violated the invariant.
        recent_work_id: String,
    },
    /// An unresolved-root row does not expose a reconnect or locate path.
    MissingRootWithoutRecoveryPath {
        /// Row that violated the invariant.
        recent_work_id: String,
    },
    /// A row does not expose a pin/unpin action.
    PinActionMissing {
        /// Row that violated the invariant.
        recent_work_id: String,
    },
    /// A diagnostic leaked a raw location instead of the redacted target label.
    DiagnosticLeaksRawLocation {
        /// Diagnostic that violated the invariant.
        diagnostic_id: String,
    },
    /// The surface-class coverage summary does not match the rows.
    CoverageSummaryStale,
    /// The packet does not declare a release-evidence ref.
    ReleaseEvidenceMissing,
    /// The packet does not declare a readiness review ref.
    ReadinessReviewMissing,
}

/// Validates a packet against the M5 start-center / switcher parity invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_m5_start_center_and_switcher_packet(
    packet: &M5StartCenterSwitcherPacket,
) -> Result<(), Vec<M5StartCenterSwitcherValidationError>> {
    let mut errors = Vec::new();

    let coverage = SurfaceClassCoverageSummary::from_rows(&packet.rows);
    if coverage != packet.surface_class_coverage {
        errors.push(M5StartCenterSwitcherValidationError::CoverageSummaryStale);
    }

    for class in M5EntrySurfaceClass::required_classes() {
        if !packet.rows.iter().any(|row| row.surface_class == class) {
            errors.push(M5StartCenterSwitcherValidationError::MissingSurfaceClass {
                surface_class: class.as_str().to_owned(),
            });
        }
    }

    for class in M5DiagnosticClass::required_classes() {
        if !packet
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.diagnostic_class == class)
        {
            errors.push(
                M5StartCenterSwitcherValidationError::MissingDiagnosticClass {
                    diagnostic_class: class.as_str().to_owned(),
                },
            );
        }
    }

    for row in &packet.rows {
        if !row.present_in_start_center {
            errors.push(
                M5StartCenterSwitcherValidationError::RowMissingFromStartCenter {
                    recent_work_id: row.recent_work_id.clone(),
                },
            );
        }
        if !row.present_in_switcher {
            errors.push(
                M5StartCenterSwitcherValidationError::RowMissingFromSwitcher {
                    recent_work_id: row.recent_work_id.clone(),
                },
            );
        }
        if M5EntrySurfaceClass::from_target_kind(row.target_kind) != Some(row.surface_class) {
            errors.push(
                M5StartCenterSwitcherValidationError::SurfaceClassCollapsed {
                    recent_work_id: row.recent_work_id.clone(),
                },
            );
        }
        if !row.parity.target_kind_parity {
            errors.push(
                M5StartCenterSwitcherValidationError::TargetKindParityBroken {
                    recent_work_id: row.recent_work_id.clone(),
                },
            );
        }
        if !row.parity.trust_parity {
            errors.push(M5StartCenterSwitcherValidationError::TrustParityBroken {
                recent_work_id: row.recent_work_id.clone(),
            });
        }
        if !row.trust_not_widened() {
            errors.push(M5StartCenterSwitcherValidationError::TrustWidened {
                recent_work_id: row.recent_work_id.clone(),
            });
        }
        if !row.parity.restore_parity {
            errors.push(M5StartCenterSwitcherValidationError::RestoreParityBroken {
                recent_work_id: row.recent_work_id.clone(),
            });
        }
        if !row.parity.failure_parity {
            errors.push(M5StartCenterSwitcherValidationError::FailureParityBroken {
                recent_work_id: row.recent_work_id.clone(),
            });
        }
        if !row.keyboard_complete {
            errors.push(M5StartCenterSwitcherValidationError::KeyboardIncomplete {
                recent_work_id: row.recent_work_id.clone(),
            });
        }
        if !row.pin_action_available {
            errors.push(M5StartCenterSwitcherValidationError::PinActionMissing {
                recent_work_id: row.recent_work_id.clone(),
            });
        }
        if !row.root_state.is_resolved() && !row.reconnect_or_locate_available {
            errors.push(
                M5StartCenterSwitcherValidationError::MissingRootWithoutRecoveryPath {
                    recent_work_id: row.recent_work_id.clone(),
                },
            );
        }
    }

    for diagnostic in &packet.diagnostics {
        let leaks_raw = !diagnostic.export_safe
            || diagnostic.redacted_location != diagnostic.target_kind.surface_label();
        if leaks_raw {
            errors.push(
                M5StartCenterSwitcherValidationError::DiagnosticLeaksRawLocation {
                    diagnostic_id: diagnostic.diagnostic_id.clone(),
                },
            );
        }
    }

    if packet.release_evidence_refs.is_empty() {
        errors.push(M5StartCenterSwitcherValidationError::ReleaseEvidenceMissing);
    }
    if packet.readiness_review_refs.is_empty() {
        errors.push(M5StartCenterSwitcherValidationError::ReadinessReviewMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Builds the seeded recent-work registry that backs the parity packet.
///
/// One entry covers each required M5 entry-target class, plus failure exemplars
/// for the missing-root, relocated-workspace, stale-target, remote-host, and
/// partial-restore diagnostics. The registry is the single source both the
/// Start Center and the switcher project from.
pub fn seeded_m5_recent_work_registry() -> RecentWorkRegistry {
    RecentWorkRegistry {
        record_kind: RecentWorkRegistryRecordKind::RecentWorkRegistryRecord,
        recent_work_registry_schema_version: 1,
        updated_at: "mono:0000:00:00:00.0000".to_owned(),
        entries: entry_seeds().iter().map(build_entry).collect(),
    }
}

struct EntrySeed {
    recent_work_id: &'static str,
    presentation_label: &'static str,
    presentation_subtitle: &'static str,
    target_kind: TargetKind,
    target_state: RecentWorkTargetState,
    portability_class: PortabilityClass,
    trust_state: TrustState,
    restore_availability: RestoreAvailability,
    safe_recovery_actions: &'static [SafeRecoveryAction],
    pinned: bool,
    last_opened_at: &'static str,
    filesystem_identity_ref: Option<&'static str>,
    remote_target_descriptor_ref: Option<&'static str>,
    artifact_descriptor_ref: Option<&'static str>,
}

fn build_entry(seed: &EntrySeed) -> RecentWorkEntryRecord {
    RecentWorkEntryRecord {
        record_kind: RecentWorkEntryRecordKind::RecentWorkEntryRecord,
        entry_and_restore_schema_version: 1,
        recent_work_id: seed.recent_work_id.to_owned(),
        presentation_label: seed.presentation_label.to_owned(),
        presentation_subtitle: Some(seed.presentation_subtitle.to_owned()),
        target_kind: seed.target_kind,
        target_state: seed.target_state,
        portability_class: seed.portability_class,
        trust_state: seed.trust_state,
        restore_availability: seed.restore_availability,
        safe_recovery_actions: seed.safe_recovery_actions.to_vec(),
        pinned: seed.pinned,
        last_opened_at: seed.last_opened_at.to_owned(),
        filesystem_identity_ref: seed.filesystem_identity_ref.map(str::to_owned),
        remote_target_descriptor_ref: seed.remote_target_descriptor_ref.map(str::to_owned),
        artifact_descriptor_ref: seed.artifact_descriptor_ref.map(str::to_owned),
        recovery_checkpoint_refs: None,
    }
}

fn entry_seeds() -> Vec<EntrySeed> {
    vec![
        // One healthy/representative row per required surface class.
        EntrySeed {
            recent_work_id: "recent:m5.local_folder",
            presentation_label: "checkout",
            presentation_subtitle: "~/Code/checkout",
            target_kind: TargetKind::LocalFolder,
            target_state: RecentWorkTargetState::Reachable,
            portability_class: PortabilityClass::LocalOnly,
            trust_state: TrustState::Trusted,
            restore_availability: RestoreAvailability::Compatible,
            safe_recovery_actions: &[SafeRecoveryAction::Open],
            pinned: true,
            last_opened_at: "mono:0000:00:00:00.0010",
            filesystem_identity_ref: Some("fs:checkout"),
            remote_target_descriptor_ref: None,
            artifact_descriptor_ref: None,
        },
        EntrySeed {
            recent_work_id: "recent:m5.workspace_file",
            presentation_label: "platform",
            presentation_subtitle: "platform.aureline-workspace",
            target_kind: TargetKind::WorkspaceManifest,
            target_state: RecentWorkTargetState::Reachable,
            portability_class: PortabilityClass::LocalOnly,
            trust_state: TrustState::Trusted,
            restore_availability: RestoreAvailability::Exact,
            safe_recovery_actions: &[SafeRecoveryAction::Open],
            pinned: false,
            last_opened_at: "mono:0000:00:00:00.0009",
            filesystem_identity_ref: Some("fs:platform_workspace"),
            remote_target_descriptor_ref: None,
            artifact_descriptor_ref: None,
        },
        EntrySeed {
            recent_work_id: "recent:m5.multi_root_workspace",
            presentation_label: "monorepo bundle",
            presentation_subtitle: "monorepo.aureline-workset",
            target_kind: TargetKind::WorksetManifest,
            target_state: RecentWorkTargetState::Reachable,
            portability_class: PortabilityClass::LocalOnly,
            trust_state: TrustState::Trusted,
            restore_availability: RestoreAvailability::Compatible,
            safe_recovery_actions: &[SafeRecoveryAction::Open],
            pinned: false,
            last_opened_at: "mono:0000:00:00:00.0008",
            filesystem_identity_ref: Some("fs:monorepo_workset"),
            remote_target_descriptor_ref: None,
            artifact_descriptor_ref: None,
        },
        EntrySeed {
            recent_work_id: "recent:m5.ssh_target",
            presentation_label: "edge-build",
            presentation_subtitle: "SSH workspace",
            target_kind: TargetKind::SshWorkspace,
            target_state: RecentWorkTargetState::Reachable,
            portability_class: PortabilityClass::ProviderLinked,
            trust_state: TrustState::Trusted,
            restore_availability: RestoreAvailability::Compatible,
            safe_recovery_actions: &[SafeRecoveryAction::Open],
            pinned: false,
            last_opened_at: "mono:0000:00:00:00.0007",
            filesystem_identity_ref: None,
            remote_target_descriptor_ref: Some("remote:edge_build"),
            artifact_descriptor_ref: None,
        },
        EntrySeed {
            recent_work_id: "recent:m5.container",
            presentation_label: "api sandbox",
            presentation_subtitle: "Dev container",
            target_kind: TargetKind::DevcontainerWorkspace,
            target_state: RecentWorkTargetState::Reachable,
            portability_class: PortabilityClass::ProviderLinked,
            trust_state: TrustState::Trusted,
            restore_availability: RestoreAvailability::Compatible,
            safe_recovery_actions: &[SafeRecoveryAction::Open],
            pinned: false,
            last_opened_at: "mono:0000:00:00:00.0006",
            filesystem_identity_ref: None,
            remote_target_descriptor_ref: Some("remote:api_sandbox"),
            artifact_descriptor_ref: None,
        },
        EntrySeed {
            recent_work_id: "recent:m5.managed_workspace",
            presentation_label: "research cloud",
            presentation_subtitle: "Cloud workspace",
            target_kind: TargetKind::ManagedCloudWorkspace,
            target_state: RecentWorkTargetState::Reachable,
            portability_class: PortabilityClass::ProviderLinked,
            trust_state: TrustState::PendingEvaluation,
            restore_availability: RestoreAvailability::Compatible,
            safe_recovery_actions: &[SafeRecoveryAction::Open],
            pinned: false,
            last_opened_at: "mono:0000:00:00:00.0005",
            filesystem_identity_ref: None,
            remote_target_descriptor_ref: Some("remote:research_cloud"),
            artifact_descriptor_ref: None,
        },
        EntrySeed {
            recent_work_id: "recent:m5.import_packet",
            presentation_label: "imported settings",
            presentation_subtitle: "Portable state",
            target_kind: TargetKind::PortableStatePackage,
            target_state: RecentWorkTargetState::Reachable,
            portability_class: PortabilityClass::Imported,
            trust_state: TrustState::PendingEvaluation,
            restore_availability: RestoreAvailability::None,
            safe_recovery_actions: &[SafeRecoveryAction::Open],
            pinned: false,
            last_opened_at: "mono:0000:00:00:00.0004",
            filesystem_identity_ref: None,
            remote_target_descriptor_ref: None,
            artifact_descriptor_ref: Some("artifact:imported_settings"),
        },
        EntrySeed {
            recent_work_id: "recent:m5.bundle_backed",
            presentation_label: "typescript web app",
            presentation_subtitle: "Template",
            target_kind: TargetKind::TemplateOrPrebuildSnapshot,
            target_state: RecentWorkTargetState::Reachable,
            portability_class: PortabilityClass::Synced,
            trust_state: TrustState::Trusted,
            restore_availability: RestoreAvailability::None,
            safe_recovery_actions: &[SafeRecoveryAction::Open],
            pinned: false,
            last_opened_at: "mono:0000:00:00:00.0003",
            filesystem_identity_ref: None,
            remote_target_descriptor_ref: None,
            artifact_descriptor_ref: Some("artifact:ts_web_app_template"),
        },
        // Failure exemplars that drive the export-safe diagnostics.
        EntrySeed {
            recent_work_id: "recent:m5.missing_root",
            presentation_label: "payments",
            presentation_subtitle: "Local repository",
            target_kind: TargetKind::LocalRepoRoot,
            target_state: RecentWorkTargetState::MissingTarget,
            portability_class: PortabilityClass::LocalOnly,
            trust_state: TrustState::Trusted,
            restore_availability: RestoreAvailability::LayoutOnly,
            safe_recovery_actions: &[SafeRecoveryAction::LocateMissingTarget],
            pinned: true,
            last_opened_at: "mono:0000:00:00:00.0002",
            filesystem_identity_ref: Some("fs:payments"),
            remote_target_descriptor_ref: None,
            artifact_descriptor_ref: None,
        },
        EntrySeed {
            recent_work_id: "recent:m5.relocated_workspace",
            presentation_label: "design system",
            presentation_subtitle: "Workspace",
            target_kind: TargetKind::WorkspaceManifest,
            target_state: RecentWorkTargetState::MovedTargetDetected,
            portability_class: PortabilityClass::LocalOnly,
            trust_state: TrustState::Trusted,
            restore_availability: RestoreAvailability::LayoutOnly,
            safe_recovery_actions: &[SafeRecoveryAction::LocateMissingTarget],
            pinned: false,
            last_opened_at: "mono:0000:00:00:00.0001",
            filesystem_identity_ref: Some("fs:design_system"),
            remote_target_descriptor_ref: None,
            artifact_descriptor_ref: None,
        },
        EntrySeed {
            recent_work_id: "recent:m5.stale_target",
            presentation_label: "vendored docs",
            presentation_subtitle: "Handoff packet",
            target_kind: TargetKind::HandoffPacket,
            target_state: RecentWorkTargetState::StaleMetadata,
            portability_class: PortabilityClass::Stale,
            trust_state: TrustState::PendingEvaluation,
            restore_availability: RestoreAvailability::EvidenceOnly,
            safe_recovery_actions: &[SafeRecoveryAction::OpenReadOnlyCachedView],
            pinned: false,
            last_opened_at: "mono:0000:00:00:00.0000",
            filesystem_identity_ref: None,
            remote_target_descriptor_ref: None,
            artifact_descriptor_ref: Some("artifact:vendored_docs"),
        },
        EntrySeed {
            recent_work_id: "recent:m5.remote_host_unreachable",
            presentation_label: "staging cluster",
            presentation_subtitle: "Remote repository",
            target_kind: TargetKind::RemoteRepository,
            target_state: RecentWorkTargetState::RemoteUnreachable,
            portability_class: PortabilityClass::ProviderLinked,
            trust_state: TrustState::PendingEvaluation,
            restore_availability: RestoreAvailability::EvidenceOnly,
            safe_recovery_actions: &[SafeRecoveryAction::Reconnect],
            pinned: false,
            last_opened_at: "mono:0000:00:00:00.0000",
            filesystem_identity_ref: None,
            remote_target_descriptor_ref: Some("remote:staging_cluster"),
            artifact_descriptor_ref: None,
        },
        EntrySeed {
            recent_work_id: "recent:m5.partial_restore",
            presentation_label: "training run",
            presentation_subtitle: "Cloud workspace",
            target_kind: TargetKind::ManagedCloudWorkspace,
            target_state: RecentWorkTargetState::Reachable,
            portability_class: PortabilityClass::ProviderLinked,
            trust_state: TrustState::PendingEvaluation,
            restore_availability: RestoreAvailability::LayoutOnly,
            safe_recovery_actions: &[SafeRecoveryAction::Open],
            pinned: false,
            last_opened_at: "mono:0000:00:00:00.0000",
            filesystem_identity_ref: None,
            remote_target_descriptor_ref: Some("remote:training_run"),
            artifact_descriptor_ref: None,
        },
    ]
}

/// Builds the seeded Start Center / switcher parity packet.
///
/// # Panics
/// Panics if a seeded entry has no canonical M5 surface class or is dropped by
/// one of the two live projections; both conditions are programming errors in
/// the seed data and are exercised by the unit tests.
pub fn seeded_m5_start_center_and_switcher_packet() -> M5StartCenterSwitcherPacket {
    let registry = seeded_m5_recent_work_registry();
    let start_center =
        build_searchable_recent_work_rows(&registry, StartCenterRecentWorkPrivacyMode::Default, "");
    let switcher = build_switcher_rows(&registry, "");

    let mut rows = Vec::with_capacity(registry.entries.len());
    for entry in &registry.entries {
        let surface_class = M5EntrySurfaceClass::from_target_kind(entry.target_kind)
            .unwrap_or_else(|| {
                panic!(
                    "seed entry {} has no M5 surface class",
                    entry.recent_work_id
                )
            });
        let start_center_row = start_center
            .rows
            .iter()
            .find(|row| row.recent_work_id == entry.recent_work_id)
            .unwrap_or_else(|| {
                panic!(
                    "seed entry {} missing from Start Center projection",
                    entry.recent_work_id
                )
            });
        let switcher_row = switcher
            .iter()
            .find(|row| row.recent_work_id == entry.recent_work_id)
            .unwrap_or_else(|| {
                panic!(
                    "seed entry {} missing from switcher projection",
                    entry.recent_work_id
                )
            });
        rows.push(M5StartCenterSwitcherRow::from_projections(
            start_center_row,
            switcher_row,
            entry,
            surface_class,
        ));
    }

    let mut diagnostics = Vec::new();
    for row in &rows {
        if let Some(diagnostic_class) = row.diagnostic_class {
            diagnostics.push(M5EntryDiagnostic::from_row(row, diagnostic_class));
        }
    }

    let surface_class_coverage = SurfaceClassCoverageSummary::from_rows(&rows);
    let all_rows_in_both_surfaces = rows
        .iter()
        .all(|row| row.present_in_start_center && row.present_in_switcher);
    let no_target_kind_collapsed = rows.iter().all(|row| {
        M5EntrySurfaceClass::from_target_kind(row.target_kind) == Some(row.surface_class)
    });
    let no_trust_widened = rows.iter().all(M5StartCenterSwitcherRow::trust_not_widened);
    let full_parity = rows.iter().all(M5StartCenterSwitcherRow::parity_complete);

    M5StartCenterSwitcherPacket {
        record_kind: M5_START_CENTER_AND_SWITCHER_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_START_CENTER_AND_SWITCHER_SCHEMA_VERSION,
        shared_contract_ref: M5_START_CENTER_AND_SWITCHER_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_START_CENTER_AND_SWITCHER_PACKET_ID.to_owned(),
        headline:
            "Start Center and in-workspace switcher project the same M5 entry-target classes, trust, restore, and missing-root truth."
                .to_owned(),
        rows,
        diagnostics,
        surface_class_coverage,
        all_rows_in_both_surfaces,
        no_target_kind_collapsed,
        no_trust_widened,
        full_parity,
        release_evidence_refs: vec![
            "release-evidence:m5:start_center_and_switcher_parity".to_owned(),
            "artifacts/ux/m5/start-center-and-switcher-audit.md".to_owned(),
        ],
        readiness_review_refs: vec![
            "readiness-review:m5:project_entry_parity".to_owned(),
            "readiness-review:m5:resume_continuity".to_owned(),
        ],
        published_audit_ref: M5_START_CENTER_AND_SWITCHER_PUBLISHED_AUDIT_REF.to_owned(),
        docs_help_refs: vec![
            "docs/help/m5-start-center-and-switcher.md".to_owned(),
            "docs/workspaces/recent_work.md".to_owned(),
        ],
        support_export_refs: vec![
            "support:export.include_m5_start_center_and_switcher_packet".to_owned(),
        ],
        generated_at: GENERATED_AT.to_owned(),
    }
}

#[cfg(test)]
mod tests;
