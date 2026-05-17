//! Beta Start Center and workspace-switcher projection.
//!
//! The projection in this module is the shell-level truth packet for
//! first-run, no-workspace, recent-work, restore, and in-workspace switching
//! rows. It deliberately reads the same recent-work registry and recovery
//! taxonomy as the live shell so Start Center rows, switcher rows, and support
//! exports agree on target kind, trust state, restore availability, missing
//! target posture, and safe recovery actions.

use std::collections::{BTreeMap, BTreeSet};

use aureline_commands::enablement::PreflightDecisionClass;
use aureline_commands::registry::seeded_registry;
use aureline_workspace::{
    PortabilityClass, RecentWorkEntryRecord, RecentWorkEntryRecordKind, RecentWorkFailureState,
    RecentWorkListSection, RecentWorkRegistry, RecentWorkRegistryRecordKind, RecentWorkTargetState,
    RecoveryCheckpointRef, RestoreAvailability, SafeRecoveryAction, TargetKind, TrustState,
};
use serde::{Deserialize, Serialize};

use crate::restore::placeholders::WorkspaceSwitchRecoveryAction;
use crate::workspace_switcher::build_switcher_rows;

use super::{
    build_action_rows, build_recent_work_rows, build_searchable_recent_work_rows,
    StartCenterPrimaryActionId, StartCenterRecentWorkPrivacyMode, StartCenterRecentWorkRow,
    StartCenterRuntimeInputs,
};

/// Schema version exported with every Start Center beta record.
pub const START_CENTER_SWITCHER_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by Start Center, switcher, and support export.
pub const START_CENTER_SWITCHER_BETA_SHARED_CONTRACT_REF: &str =
    "shell:start_center_workspace_switcher_beta:v1";

/// Stable record kind for [`StartCenterSwitcherBetaPage`] payloads.
pub const START_CENTER_SWITCHER_BETA_PAGE_RECORD_KIND: &str =
    "shell_start_center_workspace_switcher_beta_page_record";

/// Stable record kind for [`StartCenterBetaPrimaryActionRecord`] payloads.
pub const START_CENTER_BETA_PRIMARY_ACTION_RECORD_KIND: &str =
    "shell_start_center_beta_primary_action_record";

/// Stable record kind for [`StartCenterSwitcherBetaWorkRow`] payloads.
pub const START_CENTER_SWITCHER_BETA_WORK_ROW_RECORD_KIND: &str =
    "shell_start_center_workspace_switcher_beta_work_row_record";

/// Stable record kind for [`StartCenterSwitcherBetaSupportRow`] payloads.
pub const START_CENTER_SWITCHER_BETA_SUPPORT_ROW_RECORD_KIND: &str =
    "shell_start_center_workspace_switcher_beta_support_row_record";

/// Stable record kind for [`StartCenterSwitcherBetaSupportExport`] payloads.
pub const START_CENTER_SWITCHER_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_start_center_workspace_switcher_beta_support_export_record";

/// Stable record kind for [`StartCenterSwitcherBetaPrivacyModeRecord`] payloads.
pub const START_CENTER_SWITCHER_BETA_PRIVACY_RECORD_KIND: &str =
    "shell_start_center_workspace_switcher_beta_privacy_mode_record";

/// Stable record kind for [`StartCenterSwitcherBetaOpenWindowRecord`] payloads.
pub const START_CENTER_SWITCHER_BETA_OPEN_WINDOW_RECORD_KIND: &str =
    "shell_start_center_workspace_switcher_beta_open_window_record";

const WORKSPACE_OBJECT_MODEL_REF: &str = "docs/workspace/entry_restore_object_model.md";
const ERROR_HANDLING_PATH_REF: &str = "docs/ux/recent_work_and_restore_card_contract.md";
const START_CENTER_CONTRACT_REF: &str = "docs/ux/start_center_contract.md";

/// Surface family represented by one work row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StartCenterSwitcherSurfaceClass {
    /// First-run or no-workspace Start Center row.
    StartCenter,
    /// In-workspace switcher row.
    WorkspaceSwitcher,
}

impl StartCenterSwitcherSurfaceClass {
    /// Returns the stable token used by fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenter => "start_center",
            Self::WorkspaceSwitcher => "workspace_switcher",
        }
    }
}

/// Action role used by compact row action records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StartCenterSwitcherActionRole {
    /// Activates the row or primary Start Center command.
    Primary,
    /// Non-destructive row management.
    Secondary,
    /// Repairs or revalidates an unavailable target.
    Recovery,
    /// Removes only recent-work metadata after confirmation.
    DestructiveMetadataOnly,
}

impl StartCenterSwitcherActionRole {
    /// Returns the stable role token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Recovery => "recovery",
            Self::DestructiveMetadataOnly => "destructive_metadata_only",
        }
    }
}

/// Compact recovery-action record shared by rows and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartCenterSwitcherRecoveryActionRecord {
    /// Stable action id from the recent-work recovery vocabulary.
    pub action_id: String,
    /// Compact label rendered in rows.
    pub action_label: String,
    /// Role used for placement and confirmation.
    pub action_role: StartCenterSwitcherActionRole,
    /// Whether the action is available for the current row.
    pub enabled: bool,
    /// Whether the user must confirm before the action mutates metadata.
    pub requires_confirmation: bool,
    /// Whether the action is guaranteed to affect recent-work metadata only.
    pub metadata_only_cleanup: bool,
}

impl StartCenterSwitcherRecoveryActionRecord {
    fn from_action(action: SafeRecoveryAction) -> Self {
        let action_role = action_role(action);
        Self {
            action_id: action.as_str().to_string(),
            action_label: action.surface_label().to_string(),
            action_role,
            enabled: true,
            requires_confirmation: action == SafeRecoveryAction::RemoveFromRecents,
            metadata_only_cleanup: action == SafeRecoveryAction::RemoveFromRecents,
        }
    }
}

/// Activation contract repeated on every Start Center and switcher row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartCenterSwitcherActivationContract {
    /// Upstream workspace object model the row resolves through.
    pub workspace_object_model_ref: String,
    /// Shared row-level recovery and unavailable-target contract.
    pub error_handling_path_ref: String,
    /// Whether activation emits or reuses a project-entry action record.
    pub project_entry_action_record_required: bool,
    /// Whether Start Center and switcher rows must agree for this target.
    pub same_model_across_start_center_and_switcher: bool,
    /// Whether switch failure preserves a cancel or previous-workspace route.
    pub switch_failure_preserves_previous_workspace: bool,
}

impl StartCenterSwitcherActivationContract {
    fn shared() -> Self {
        Self {
            workspace_object_model_ref: WORKSPACE_OBJECT_MODEL_REF.to_string(),
            error_handling_path_ref: ERROR_HANDLING_PATH_REF.to_string(),
            project_entry_action_record_required: true,
            same_model_across_start_center_and_switcher: true,
            switch_failure_preserves_previous_workspace: true,
        }
    }
}

/// One primary action shown above account, marketplace, and marketing content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartCenterBetaPrimaryActionRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable action token.
    pub action_id: String,
    /// Verb-first label.
    pub label: String,
    /// Short helper summary.
    pub summary: String,
    /// Canonical command id.
    pub command_id: String,
    /// Command-preflight decision class.
    pub preflight_decision_class: String,
    /// Whether the action is keyboard reachable.
    pub keyboard_reachable: bool,
    /// Whether sign-in is required before the action can render.
    pub sign_in_required_before_render: bool,
    /// Whether network readiness is required before the action can render.
    pub network_required_before_render: bool,
    /// Start Center placement zone.
    pub start_center_zone: String,
}

/// One privacy-mode projection for Start Center recent-work metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartCenterSwitcherBetaPrivacyModeRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Privacy mode token.
    pub privacy_mode: String,
    /// Whether recent-work rows are hidden.
    pub metadata_hidden: bool,
    /// Number of rows shown under this mode.
    pub row_count: usize,
    /// Number of rows with path or host redaction.
    pub privacy_redaction_applied_count: usize,
    /// Whether opening a local folder remains available.
    pub local_open_still_available: bool,
    /// Whether opening a workspace file remains available.
    pub workspace_open_still_available: bool,
    /// Whether restoring local state remains available.
    pub restore_local_state_still_available: bool,
    /// Whether clearing recent-work metadata remains available.
    pub clear_recent_work_available: bool,
}

/// One open window row included in the switcher page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartCenterSwitcherBetaOpenWindowRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable window id.
    pub window_id: String,
    /// Recent-work entry hosted by the window.
    pub recent_work_entry_ref: String,
    /// Human-readable window label.
    pub window_label: String,
    /// Whether the switcher can focus the existing window.
    pub focus_existing_window_available: bool,
    /// Whether the prior workspace can be reopened after failed switch.
    pub reopen_previous_workspace_available: bool,
}

/// One Start Center or switcher work row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartCenterSwitcherBetaWorkRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source surface for this projection.
    pub surface_class: StartCenterSwitcherSurfaceClass,
    /// Stable row id unique to the surface.
    pub row_id: String,
    /// Upstream recent-work entry id.
    pub recent_work_entry_ref: String,
    /// Pinned or recent section.
    pub list_section: RecentWorkListSection,
    /// Project, workspace, or target label.
    pub primary_label: String,
    /// Redaction-aware path, host, provider, or target subtitle.
    pub location_or_target_subtitle: Option<String>,
    /// Canonical target kind.
    pub target_kind: TargetKind,
    /// Compact target-kind label.
    pub target_kind_label: String,
    /// Raw target state.
    pub target_state: RecentWorkTargetState,
    /// Shared failure-state taxonomy.
    pub failure_state: RecentWorkFailureState,
    /// Workspace trust posture.
    pub trust_state: TrustState,
    /// Restore availability before activation.
    pub restore_availability: RestoreAvailability,
    /// Last-opened timestamp.
    pub last_opened_at: String,
    /// Whether the row is pinned.
    pub pinned: bool,
    /// Workspace-switcher classes, empty on Start Center rows.
    pub switcher_entry_classes: Vec<String>,
    /// Row actions in rendered order.
    pub row_actions: Vec<StartCenterSwitcherRecoveryActionRecord>,
    /// Switch failure actions, present on switcher rows.
    pub switch_failure_actions: Vec<String>,
    /// Write-safety badge derived from the failure state.
    pub write_safety_badge: String,
    /// Placeholder summary for unavailable targets.
    pub placeholder_summary: Option<String>,
    /// Whether privacy redaction changed the subtitle.
    pub privacy_redaction_applied: bool,
    /// Shared activation contract.
    pub activation_contract: StartCenterSwitcherActivationContract,
}

/// Support-export row that mirrors Start Center and switcher truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartCenterSwitcherBetaSupportRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Upstream recent-work entry id.
    pub recent_work_entry_ref: String,
    /// Matching Start Center row id.
    pub start_center_row_id: String,
    /// Matching switcher row id.
    pub workspace_switcher_row_id: String,
    /// Canonical target kind.
    pub target_kind: TargetKind,
    /// Raw target state.
    pub target_state: RecentWorkTargetState,
    /// Shared failure state.
    pub failure_state: RecentWorkFailureState,
    /// Workspace trust posture.
    pub trust_state: TrustState,
    /// Restore availability.
    pub restore_availability: RestoreAvailability,
    /// Action ids rendered by Start Center.
    pub start_center_action_ids: Vec<String>,
    /// Action ids rendered by the workspace switcher.
    pub workspace_switcher_action_ids: Vec<String>,
    /// Switch failure route tokens.
    pub switch_failure_actions: Vec<String>,
    /// Shared activation contract.
    pub activation_contract: StartCenterSwitcherActivationContract,
    /// Whether cleanup actions are scoped to recent-work metadata only.
    pub metadata_only_cleanup: bool,
}

/// Summary block for the beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartCenterSwitcherBetaSummary {
    /// Number of primary actions.
    pub primary_action_count: usize,
    /// Number of Start Center recent-work rows.
    pub start_center_row_count: usize,
    /// Number of workspace-switcher rows.
    pub workspace_switcher_row_count: usize,
    /// Number of pinned rows.
    pub pinned_row_count: usize,
    /// Number of non-pinned recent rows.
    pub recent_row_count: usize,
    /// Counts by failure-state token.
    pub rows_by_failure_state: BTreeMap<String, usize>,
    /// Counts by target-kind token.
    pub rows_by_target_kind: BTreeMap<String, usize>,
    /// Whether all switcher rows include cancel and reopen-prior routes.
    pub switch_failure_return_paths_preserved: bool,
}

impl StartCenterSwitcherBetaSummary {
    fn from_rows(
        primary_actions: &[StartCenterBetaPrimaryActionRecord],
        start_center_rows: &[StartCenterSwitcherBetaWorkRow],
        workspace_switcher_rows: &[StartCenterSwitcherBetaWorkRow],
    ) -> Self {
        let mut rows_by_failure_state = BTreeMap::new();
        let mut rows_by_target_kind = BTreeMap::new();
        let mut pinned_row_count = 0;
        let mut recent_row_count = 0;

        for row in start_center_rows {
            *rows_by_failure_state
                .entry(row.failure_state.as_str().to_string())
                .or_insert(0) += 1;
            *rows_by_target_kind
                .entry(row.target_kind.as_str().to_string())
                .or_insert(0) += 1;
            match row.list_section {
                RecentWorkListSection::Pinned => pinned_row_count += 1,
                RecentWorkListSection::Recent => recent_row_count += 1,
            }
        }

        let switch_failure_return_paths_preserved = workspace_switcher_rows.iter().all(|row| {
            row.switch_failure_actions
                .iter()
                .any(|action| action == WorkspaceSwitchRecoveryAction::CancelSwitch.as_str())
                && row.switch_failure_actions.iter().any(|action| {
                    action == WorkspaceSwitchRecoveryAction::ReopenPreviousWorkspace.as_str()
                })
        });

        Self {
            primary_action_count: primary_actions.len(),
            start_center_row_count: start_center_rows.len(),
            workspace_switcher_row_count: workspace_switcher_rows.len(),
            pinned_row_count,
            recent_row_count,
            rows_by_failure_state,
            rows_by_target_kind,
            switch_failure_return_paths_preserved,
        }
    }
}

/// Page-level beta projection for Start Center and workspace switching.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartCenterSwitcherBetaPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Reviewer-facing page label.
    pub page_label: String,
    /// Normative contract read by this projection.
    pub contract_ref: String,
    /// Query used for the default recent-work projection.
    pub recent_work_query: String,
    /// Primary actions above account and marketing content.
    pub primary_actions: Vec<StartCenterBetaPrimaryActionRecord>,
    /// Privacy-mode projections.
    pub privacy_modes: Vec<StartCenterSwitcherBetaPrivacyModeRecord>,
    /// Open-window rows projected into the switcher.
    pub open_windows: Vec<StartCenterSwitcherBetaOpenWindowRecord>,
    /// Start Center recent-work rows.
    pub start_center_rows: Vec<StartCenterSwitcherBetaWorkRow>,
    /// Workspace-switcher rows.
    pub workspace_switcher_rows: Vec<StartCenterSwitcherBetaWorkRow>,
    /// Support-export parity rows.
    pub support_rows: Vec<StartCenterSwitcherBetaSupportRow>,
    /// Page summary.
    pub summary: StartCenterSwitcherBetaSummary,
    /// Whether the Start Center is usable before sign-in.
    pub useful_before_sign_in: bool,
    /// Whether the Start Center can render before network readiness.
    pub useful_before_network_ready: bool,
    /// Whether keyboard selection and search are available.
    pub keyboard_first: bool,
    /// Whether account or marketing content is allowed above work actions.
    pub account_or_marketing_content_above_primary_actions: bool,
}

/// Support-export wrapper for the beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartCenterSwitcherBetaSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Embedded page projection.
    pub page: StartCenterSwitcherBetaPage,
    /// Per-row support projection.
    pub rows: Vec<StartCenterSwitcherBetaSupportRow>,
    /// Recent-work ids in page order.
    pub recent_work_ids: Vec<String>,
    /// Raw secret material is excluded by construction.
    pub raw_secret_material_excluded: bool,
}

impl StartCenterSwitcherBetaSupportExport {
    /// Builds a support export from a page projection.
    pub fn from_page(
        export_id: &str,
        exported_at: &str,
        page: StartCenterSwitcherBetaPage,
    ) -> Self {
        let rows = page.support_rows.clone();
        let recent_work_ids = page
            .start_center_rows
            .iter()
            .map(|row| row.recent_work_entry_ref.clone())
            .collect();
        Self {
            record_kind: START_CENTER_SWITCHER_BETA_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: START_CENTER_SWITCHER_BETA_SCHEMA_VERSION,
            shared_contract_ref: START_CENTER_SWITCHER_BETA_SHARED_CONTRACT_REF.to_string(),
            export_id: export_id.to_string(),
            exported_at: exported_at.to_string(),
            page,
            rows,
            recent_work_ids,
            raw_secret_material_excluded: true,
        }
    }
}

/// Returns deterministic recent-work seed data for the beta projection.
pub fn seeded_start_center_beta_recent_work_registry() -> RecentWorkRegistry {
    RecentWorkRegistry {
        record_kind: RecentWorkRegistryRecordKind::RecentWorkRegistryRecord,
        recent_work_registry_schema_version: 1,
        updated_at: "mono:2026-05-17T12:00:00.0000".to_string(),
        entries: vec![
            recent_entry(RecentEntrySeed {
                recent_work_id: "recent:start-center:docs-local-folder",
                presentation_label: "Aureline docs",
                presentation_subtitle: Some("~/Code/aureline-docs"),
                target_kind: TargetKind::LocalFolder,
                target_state: RecentWorkTargetState::Reachable,
                portability_class: PortabilityClass::LocalOnly,
                trust_state: TrustState::Trusted,
                restore_availability: RestoreAvailability::Exact,
                safe_recovery_actions: vec![
                    SafeRecoveryAction::Open,
                    SafeRecoveryAction::OpenInNewWindow,
                    SafeRecoveryAction::RevealInExplorer,
                    SafeRecoveryAction::RemoveFromRecents,
                ],
                pinned: true,
                last_opened_at: "mono:2026-05-17T10:45:00.0000",
                filesystem_identity_ref: Some("fs:docs-local-folder"),
                remote_target_descriptor_ref: None,
                artifact_descriptor_ref: None,
                recovery_checkpoint_refs: Some(vec![checkpoint(
                    "restore_from_session_checkpoint",
                    "ckpt:docs:last-good",
                )]),
            }),
            recent_entry(RecentEntrySeed {
                recent_work_id: "recent:start-center:platform-workspace-moved",
                presentation_label: "Platform workspace",
                presentation_subtitle: Some("~/Work/platform.code-workspace (identity mismatch)"),
                target_kind: TargetKind::WorkspaceManifest,
                target_state: RecentWorkTargetState::MovedTargetDetected,
                portability_class: PortabilityClass::LocalOnly,
                trust_state: TrustState::Trusted,
                restore_availability: RestoreAvailability::LayoutOnly,
                safe_recovery_actions: vec![
                    SafeRecoveryAction::OpenReadOnlyCachedView,
                    SafeRecoveryAction::LocateMissingTarget,
                    SafeRecoveryAction::OpenWithoutRestore,
                    SafeRecoveryAction::RemoveFromRecents,
                ],
                pinned: true,
                last_opened_at: "mono:2026-05-16T18:20:00.0000",
                filesystem_identity_ref: Some("fs:platform-workspace-moved"),
                remote_target_descriptor_ref: None,
                artifact_descriptor_ref: None,
                recovery_checkpoint_refs: Some(vec![checkpoint(
                    "restore_from_session_checkpoint",
                    "ckpt:platform:last-good",
                )]),
            }),
            recent_entry(RecentEntrySeed {
                recent_work_id: "recent:start-center:payments-missing",
                presentation_label: "Payments service",
                presentation_subtitle: Some("~/Code/payments"),
                target_kind: TargetKind::LocalRepoRoot,
                target_state: RecentWorkTargetState::MissingTarget,
                portability_class: PortabilityClass::LocalOnly,
                trust_state: TrustState::Trusted,
                restore_availability: RestoreAvailability::Compatible,
                safe_recovery_actions: vec![
                    SafeRecoveryAction::LocateMissingTarget,
                    SafeRecoveryAction::OpenWithoutRestore,
                    SafeRecoveryAction::RemoveFromRecents,
                ],
                pinned: false,
                last_opened_at: "mono:2026-05-15T09:10:00.0000",
                filesystem_identity_ref: Some("fs:payments-missing"),
                remote_target_descriptor_ref: None,
                artifact_descriptor_ref: None,
                recovery_checkpoint_refs: Some(vec![checkpoint(
                    "restore_from_session_checkpoint",
                    "ckpt:payments:last-good",
                )]),
            }),
            recent_entry(RecentEntrySeed {
                recent_work_id: "recent:start-center:infra-ssh-unavailable",
                presentation_label: "Infra provisioning",
                presentation_subtitle: Some("SSH workspace - host unavailable"),
                target_kind: TargetKind::SshWorkspace,
                target_state: RecentWorkTargetState::RemoteUnreachable,
                portability_class: PortabilityClass::ProviderLinked,
                trust_state: TrustState::PendingEvaluation,
                restore_availability: RestoreAvailability::EvidenceOnly,
                safe_recovery_actions: vec![
                    SafeRecoveryAction::Reconnect,
                    SafeRecoveryAction::OpenReadOnlyCachedView,
                    SafeRecoveryAction::RetryLater,
                    SafeRecoveryAction::OpenWithoutRestore,
                    SafeRecoveryAction::RemoveFromRecents,
                ],
                pinned: false,
                last_opened_at: "mono:2026-05-14T14:05:00.0000",
                filesystem_identity_ref: None,
                remote_target_descriptor_ref: Some("remote:ssh:infra-provisioning"),
                artifact_descriptor_ref: None,
                recovery_checkpoint_refs: Some(vec![checkpoint(
                    "restore_from_session_checkpoint",
                    "ckpt:infra:evidence",
                )]),
            }),
            recent_entry(RecentEntrySeed {
                recent_work_id: "recent:start-center:web-devcontainer-unavailable",
                presentation_label: "Web client dev container",
                presentation_subtitle: Some("Dev container - engine offline"),
                target_kind: TargetKind::DevcontainerWorkspace,
                target_state: RecentWorkTargetState::RemoteUnreachable,
                portability_class: PortabilityClass::ProviderLinked,
                trust_state: TrustState::Restricted,
                restore_availability: RestoreAvailability::Compatible,
                safe_recovery_actions: vec![
                    SafeRecoveryAction::Reconnect,
                    SafeRecoveryAction::RetryLater,
                    SafeRecoveryAction::OpenWithoutRestore,
                    SafeRecoveryAction::RemoveFromRecents,
                ],
                pinned: false,
                last_opened_at: "mono:2026-05-13T17:30:00.0000",
                filesystem_identity_ref: None,
                remote_target_descriptor_ref: Some("remote:devcontainer:web-client"),
                artifact_descriptor_ref: None,
                recovery_checkpoint_refs: Some(vec![checkpoint(
                    "restore_from_session_checkpoint",
                    "ckpt:web-client:compatible",
                )]),
            }),
            recent_entry(RecentEntrySeed {
                recent_work_id: "recent:start-center:managed-data-expired",
                presentation_label: "Managed data workspace",
                presentation_subtitle: Some("Managed cloud workspace - reauthorization required"),
                target_kind: TargetKind::ManagedCloudWorkspace,
                target_state: RecentWorkTargetState::AuthorityExpired,
                portability_class: PortabilityClass::ProviderLinked,
                trust_state: TrustState::PendingEvaluation,
                restore_availability: RestoreAvailability::LayoutOnly,
                safe_recovery_actions: vec![
                    SafeRecoveryAction::Reauth,
                    SafeRecoveryAction::RetryLater,
                    SafeRecoveryAction::OpenWithoutRestore,
                    SafeRecoveryAction::RemoveFromRecents,
                ],
                pinned: false,
                last_opened_at: "mono:2026-05-12T08:25:00.0000",
                filesystem_identity_ref: None,
                remote_target_descriptor_ref: Some("remote:managed-cloud:data-workspace"),
                artifact_descriptor_ref: None,
                recovery_checkpoint_refs: Some(vec![checkpoint(
                    "restore_from_session_checkpoint",
                    "ckpt:managed-data:layout",
                )]),
            }),
        ],
    }
}

/// Builds the seeded beta page.
pub fn seeded_start_center_switcher_beta_page() -> StartCenterSwitcherBetaPage {
    let registry = seeded_start_center_beta_recent_work_registry();
    let command_registry = seeded_registry();
    let runtime = StartCenterRuntimeInputs {
        client_scope: "desktop_product",
        workspace_trust_state: "trusted",
        execution_context_available: true,
        provider_linked: None,
        credential_available: None,
        policy_disabled: false,
        policy_blocked_in_context: false,
        labs_enabled: false,
    };
    let primary_actions = project_primary_actions(&command_registry, runtime);
    let recent_projection =
        build_searchable_recent_work_rows(&registry, StartCenterRecentWorkPrivacyMode::Default, "");
    let start_center_rows: Vec<_> = recent_projection
        .rows
        .iter()
        .map(StartCenterSwitcherBetaWorkRow::from_start_center_row)
        .collect();
    let workspace_switcher_rows: Vec<_> = build_switcher_rows(&registry, "")
        .iter()
        .map(StartCenterSwitcherBetaWorkRow::from_switcher_row)
        .collect();
    let privacy_modes = project_privacy_modes(&registry);
    let open_windows = vec![StartCenterSwitcherBetaOpenWindowRecord {
        record_kind: START_CENTER_SWITCHER_BETA_OPEN_WINDOW_RECORD_KIND.to_string(),
        schema_version: START_CENTER_SWITCHER_BETA_SCHEMA_VERSION,
        shared_contract_ref: START_CENTER_SWITCHER_BETA_SHARED_CONTRACT_REF.to_string(),
        window_id: "window:current:docs-local-folder".to_string(),
        recent_work_entry_ref: "recent:start-center:docs-local-folder".to_string(),
        window_label: "Aureline docs".to_string(),
        focus_existing_window_available: true,
        reopen_previous_workspace_available: true,
    }];
    let support_rows = build_support_rows(&start_center_rows, &workspace_switcher_rows);
    let summary = StartCenterSwitcherBetaSummary::from_rows(
        &primary_actions,
        &start_center_rows,
        &workspace_switcher_rows,
    );

    StartCenterSwitcherBetaPage {
        record_kind: START_CENTER_SWITCHER_BETA_PAGE_RECORD_KIND.to_string(),
        schema_version: START_CENTER_SWITCHER_BETA_SCHEMA_VERSION,
        shared_contract_ref: START_CENTER_SWITCHER_BETA_SHARED_CONTRACT_REF.to_string(),
        page_label: "Start Center and workspace switcher beta".to_string(),
        contract_ref: START_CENTER_CONTRACT_REF.to_string(),
        recent_work_query: recent_projection.query,
        primary_actions,
        privacy_modes,
        open_windows,
        start_center_rows,
        workspace_switcher_rows,
        support_rows,
        summary,
        useful_before_sign_in: true,
        useful_before_network_ready: true,
        keyboard_first: true,
        account_or_marketing_content_above_primary_actions: false,
    }
}

/// Validates the beta page invariants.
pub fn validate_start_center_switcher_beta_page(
    page: &StartCenterSwitcherBetaPage,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if page.record_kind != START_CENTER_SWITCHER_BETA_PAGE_RECORD_KIND {
        errors.push("page.record_kind.invalid".to_string());
    }
    if page.schema_version != START_CENTER_SWITCHER_BETA_SCHEMA_VERSION {
        errors.push("page.schema_version.invalid".to_string());
    }
    if page.shared_contract_ref != START_CENTER_SWITCHER_BETA_SHARED_CONTRACT_REF {
        errors.push("page.shared_contract_ref.invalid".to_string());
    }
    if !page.useful_before_sign_in {
        errors.push("page.must_render_before_sign_in".to_string());
    }
    if !page.useful_before_network_ready {
        errors.push("page.must_render_before_network_ready".to_string());
    }
    if !page.keyboard_first {
        errors.push("page.keyboard_first.missing".to_string());
    }
    if page.account_or_marketing_content_above_primary_actions {
        errors.push("page.primary_actions.displaced_by_secondary_content".to_string());
    }

    validate_primary_actions(&page.primary_actions, &mut errors);
    validate_privacy_modes(&page.privacy_modes, &mut errors);
    validate_work_rows(page, &mut errors);
    validate_support_rows(page, &mut errors);

    if page.open_windows.is_empty() {
        errors.push("open_windows.empty".to_string());
    }
    for window in &page.open_windows {
        if !window.focus_existing_window_available {
            errors.push(format!("open_window.{}.focus_missing", window.window_id));
        }
        if !window.reopen_previous_workspace_available {
            errors.push(format!(
                "open_window.{}.reopen_prior_missing",
                window.window_id
            ));
        }
    }

    if page.summary.primary_action_count != page.primary_actions.len() {
        errors.push("summary.primary_action_count.drift".to_string());
    }
    if page.summary.start_center_row_count != page.start_center_rows.len() {
        errors.push("summary.start_center_row_count.drift".to_string());
    }
    if page.summary.workspace_switcher_row_count != page.workspace_switcher_rows.len() {
        errors.push("summary.workspace_switcher_row_count.drift".to_string());
    }
    if !page.summary.switch_failure_return_paths_preserved {
        errors.push("summary.switch_failure_return_paths_missing".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates the support-export wrapper against its embedded page.
pub fn validate_start_center_switcher_beta_support_export(
    export: &StartCenterSwitcherBetaSupportExport,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if export.record_kind != START_CENTER_SWITCHER_BETA_SUPPORT_EXPORT_RECORD_KIND {
        errors.push("support_export.record_kind.invalid".to_string());
    }
    if export.shared_contract_ref != START_CENTER_SWITCHER_BETA_SHARED_CONTRACT_REF {
        errors.push("support_export.shared_contract_ref.invalid".to_string());
    }
    if !export.raw_secret_material_excluded {
        errors.push("support_export.raw_secret_material_present".to_string());
    }
    if export.rows != export.page.support_rows {
        errors.push("support_export.rows.drift_from_page".to_string());
    }
    let expected_ids: Vec<String> = export
        .page
        .start_center_rows
        .iter()
        .map(|row| row.recent_work_entry_ref.clone())
        .collect();
    if export.recent_work_ids != expected_ids {
        errors.push("support_export.recent_work_ids.drift_from_page".to_string());
    }
    if let Err(page_errors) = validate_start_center_switcher_beta_page(&export.page) {
        errors.extend(page_errors.into_iter().map(|error| format!("page.{error}")));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

impl StartCenterSwitcherBetaWorkRow {
    fn from_start_center_row(row: &StartCenterRecentWorkRow) -> Self {
        Self {
            record_kind: START_CENTER_SWITCHER_BETA_WORK_ROW_RECORD_KIND.to_string(),
            schema_version: START_CENTER_SWITCHER_BETA_SCHEMA_VERSION,
            shared_contract_ref: START_CENTER_SWITCHER_BETA_SHARED_CONTRACT_REF.to_string(),
            surface_class: StartCenterSwitcherSurfaceClass::StartCenter,
            row_id: format!("start-center:{}", row.recent_work_id),
            recent_work_entry_ref: row.recent_work_id.clone(),
            list_section: row.list_section,
            primary_label: row.primary_label.clone(),
            location_or_target_subtitle: row.location_or_target_subtitle.clone(),
            target_kind: row.target_kind,
            target_kind_label: row.target_kind_label.to_string(),
            target_state: row.target_state,
            failure_state: row.failure_state,
            trust_state: row.trust_state,
            restore_availability: row.restore_availability,
            last_opened_at: row.last_opened_at.clone(),
            pinned: row.pinned,
            switcher_entry_classes: Vec::new(),
            row_actions: row
                .safe_recovery_actions
                .iter()
                .copied()
                .map(StartCenterSwitcherRecoveryActionRecord::from_action)
                .collect(),
            switch_failure_actions: Vec::new(),
            write_safety_badge: write_safety_badge(row.failure_state).to_string(),
            placeholder_summary: row
                .placeholder_card
                .as_ref()
                .map(|card| card.recovery_summary.clone()),
            privacy_redaction_applied: row.privacy_redaction_applied,
            activation_contract: StartCenterSwitcherActivationContract::shared(),
        }
    }

    fn from_switcher_row(row: &crate::workspace_switcher::WorkspaceSwitcherRow) -> Self {
        Self {
            record_kind: START_CENTER_SWITCHER_BETA_WORK_ROW_RECORD_KIND.to_string(),
            schema_version: START_CENTER_SWITCHER_BETA_SCHEMA_VERSION,
            shared_contract_ref: START_CENTER_SWITCHER_BETA_SHARED_CONTRACT_REF.to_string(),
            surface_class: StartCenterSwitcherSurfaceClass::WorkspaceSwitcher,
            row_id: format!("workspace-switcher:{}", row.recent_work_id),
            recent_work_entry_ref: row.recent_work_id.clone(),
            list_section: if row.pinned {
                RecentWorkListSection::Pinned
            } else {
                RecentWorkListSection::Recent
            },
            primary_label: row.primary_label.clone(),
            location_or_target_subtitle: row.location_or_target_subtitle.clone(),
            target_kind: row.target_kind,
            target_kind_label: row.target_kind_label.to_string(),
            target_state: row.target_state,
            failure_state: row.failure_state,
            trust_state: row.trust_state,
            restore_availability: row.restore_availability,
            last_opened_at: row.last_opened_at.clone(),
            pinned: row.pinned,
            switcher_entry_classes: row
                .entry_classes
                .iter()
                .map(|class| class.as_str().to_string())
                .collect(),
            row_actions: row
                .safe_recovery_actions
                .iter()
                .copied()
                .map(StartCenterSwitcherRecoveryActionRecord::from_action)
                .collect(),
            switch_failure_actions: row
                .switch_failure_actions
                .iter()
                .map(|action| action.as_str().to_string())
                .collect(),
            write_safety_badge: write_safety_badge(row.failure_state).to_string(),
            placeholder_summary: row
                .placeholder_card
                .as_ref()
                .map(|card| card.recovery_summary.clone()),
            privacy_redaction_applied: false,
            activation_contract: StartCenterSwitcherActivationContract::shared(),
        }
    }
}

fn project_primary_actions(
    command_registry: &aureline_commands::CommandRegistry,
    runtime: StartCenterRuntimeInputs<'_>,
) -> Vec<StartCenterBetaPrimaryActionRecord> {
    build_action_rows(command_registry, runtime)
        .into_iter()
        .map(|row| StartCenterBetaPrimaryActionRecord {
            record_kind: START_CENTER_BETA_PRIMARY_ACTION_RECORD_KIND.to_string(),
            schema_version: START_CENTER_SWITCHER_BETA_SCHEMA_VERSION,
            shared_contract_ref: START_CENTER_SWITCHER_BETA_SHARED_CONTRACT_REF.to_string(),
            action_id: row.action_id.token().to_string(),
            label: row.title.to_string(),
            summary: row.summary.to_string(),
            command_id: row.command_id.to_string(),
            preflight_decision_class: row
                .preflight
                .as_ref()
                .map(|decision| preflight_class_as_str(decision.decision_class))
                .unwrap_or("missing_command")
                .to_string(),
            keyboard_reachable: true,
            sign_in_required_before_render: false,
            network_required_before_render: false,
            start_center_zone: "primary_work_resume".to_string(),
        })
        .collect()
}

fn project_privacy_modes(
    registry: &RecentWorkRegistry,
) -> Vec<StartCenterSwitcherBetaPrivacyModeRecord> {
    [
        StartCenterRecentWorkPrivacyMode::Default,
        StartCenterRecentWorkPrivacyMode::HidePaths,
        StartCenterRecentWorkPrivacyMode::HideRecentWork,
        StartCenterRecentWorkPrivacyMode::HideAllExceptOpenAndClone,
    ]
    .into_iter()
    .map(|privacy_mode| {
        let projection = build_recent_work_rows(registry, privacy_mode);
        StartCenterSwitcherBetaPrivacyModeRecord {
            record_kind: START_CENTER_SWITCHER_BETA_PRIVACY_RECORD_KIND.to_string(),
            schema_version: START_CENTER_SWITCHER_BETA_SCHEMA_VERSION,
            shared_contract_ref: START_CENTER_SWITCHER_BETA_SHARED_CONTRACT_REF.to_string(),
            privacy_mode: privacy_mode.as_str().to_string(),
            metadata_hidden: projection.metadata_hidden,
            row_count: projection.rows.len(),
            privacy_redaction_applied_count: projection
                .rows
                .iter()
                .filter(|row| row.privacy_redaction_applied)
                .count(),
            local_open_still_available: projection.local_open_still_available,
            workspace_open_still_available: projection.workspace_open_still_available,
            restore_local_state_still_available: projection.restore_local_state_still_available,
            clear_recent_work_available: projection.clear_recent_work_available,
        }
    })
    .collect()
}

fn build_support_rows(
    start_center_rows: &[StartCenterSwitcherBetaWorkRow],
    workspace_switcher_rows: &[StartCenterSwitcherBetaWorkRow],
) -> Vec<StartCenterSwitcherBetaSupportRow> {
    start_center_rows
        .iter()
        .filter_map(|start_row| {
            let switcher_row = workspace_switcher_rows
                .iter()
                .find(|row| row.recent_work_entry_ref == start_row.recent_work_entry_ref)?;
            Some(StartCenterSwitcherBetaSupportRow {
                record_kind: START_CENTER_SWITCHER_BETA_SUPPORT_ROW_RECORD_KIND.to_string(),
                schema_version: START_CENTER_SWITCHER_BETA_SCHEMA_VERSION,
                shared_contract_ref: START_CENTER_SWITCHER_BETA_SHARED_CONTRACT_REF.to_string(),
                recent_work_entry_ref: start_row.recent_work_entry_ref.clone(),
                start_center_row_id: start_row.row_id.clone(),
                workspace_switcher_row_id: switcher_row.row_id.clone(),
                target_kind: start_row.target_kind,
                target_state: start_row.target_state,
                failure_state: start_row.failure_state,
                trust_state: start_row.trust_state,
                restore_availability: start_row.restore_availability,
                start_center_action_ids: action_ids(&start_row.row_actions),
                workspace_switcher_action_ids: action_ids(&switcher_row.row_actions),
                switch_failure_actions: switcher_row.switch_failure_actions.clone(),
                activation_contract: StartCenterSwitcherActivationContract::shared(),
                metadata_only_cleanup: start_row
                    .row_actions
                    .iter()
                    .chain(switcher_row.row_actions.iter())
                    .filter(|action| {
                        action.action_id == SafeRecoveryAction::RemoveFromRecents.as_str()
                    })
                    .all(|action| action.metadata_only_cleanup),
            })
        })
        .collect()
}

fn validate_primary_actions(
    actions: &[StartCenterBetaPrimaryActionRecord],
    errors: &mut Vec<String>,
) {
    let expected = StartCenterPrimaryActionId::ordered();
    if actions.len() != expected.len() {
        errors.push("primary_actions.count.invalid".to_string());
        return;
    }
    for (idx, expected_action) in expected.iter().enumerate() {
        let row = &actions[idx];
        if row.action_id != expected_action.token() {
            errors.push(format!("primary_actions.{idx}.order.invalid"));
        }
        if row.label != expected_action.label() {
            errors.push(format!("primary_actions.{}.label.invalid", row.action_id));
        }
        if row.command_id != expected_action.command_id() {
            errors.push(format!("primary_actions.{}.command.invalid", row.action_id));
        }
        if row.start_center_zone != "primary_work_resume" {
            errors.push(format!("primary_actions.{}.zone.invalid", row.action_id));
        }
        if !row.keyboard_reachable {
            errors.push(format!(
                "primary_actions.{}.keyboard_missing",
                row.action_id
            ));
        }
        if row.sign_in_required_before_render {
            errors.push(format!("primary_actions.{}.sign_in_gated", row.action_id));
        }
        if row.network_required_before_render {
            errors.push(format!("primary_actions.{}.network_gated", row.action_id));
        }
    }
}

fn validate_privacy_modes(
    privacy_modes: &[StartCenterSwitcherBetaPrivacyModeRecord],
    errors: &mut Vec<String>,
) {
    let by_mode: BTreeMap<_, _> = privacy_modes
        .iter()
        .map(|mode| (mode.privacy_mode.as_str(), mode))
        .collect();
    for required in [
        "default",
        "hide_paths",
        "hide_recent_work",
        "hide_all_except_open_and_clone",
    ] {
        if !by_mode.contains_key(required) {
            errors.push(format!("privacy_mode.{required}.missing"));
        }
    }
    if let Some(default_mode) = by_mode.get("default") {
        if default_mode.metadata_hidden || default_mode.row_count == 0 {
            errors.push("privacy_mode.default.rows_missing".to_string());
        }
    }
    if let Some(hide_paths) = by_mode.get("hide_paths") {
        if hide_paths.privacy_redaction_applied_count == 0 {
            errors.push("privacy_mode.hide_paths.redaction_missing".to_string());
        }
        if !hide_paths.local_open_still_available || !hide_paths.workspace_open_still_available {
            errors.push("privacy_mode.hide_paths.entry_paths_missing".to_string());
        }
    }
    for hidden_mode in ["hide_recent_work", "hide_all_except_open_and_clone"] {
        if let Some(mode) = by_mode.get(hidden_mode) {
            if !mode.metadata_hidden || mode.row_count != 0 {
                errors.push(format!("privacy_mode.{hidden_mode}.metadata_not_hidden"));
            }
            if !mode.local_open_still_available || !mode.workspace_open_still_available {
                errors.push(format!("privacy_mode.{hidden_mode}.entry_paths_missing"));
            }
        }
    }
}

fn validate_work_rows(page: &StartCenterSwitcherBetaPage, errors: &mut Vec<String>) {
    if page.start_center_rows.is_empty() {
        errors.push("start_center_rows.empty".to_string());
    }
    if page.workspace_switcher_rows.is_empty() {
        errors.push("workspace_switcher_rows.empty".to_string());
    }
    if page.start_center_rows.len() != page.workspace_switcher_rows.len() {
        errors.push("work_rows.surface_count_drift".to_string());
    }

    let mut seen_failures = BTreeSet::new();
    for start_row in &page.start_center_rows {
        seen_failures.insert(start_row.failure_state.as_str().to_string());
        if start_row.primary_label.trim().is_empty() {
            errors.push(format!("row.{}.label_empty", start_row.row_id));
        }
        if start_row.last_opened_at.trim().is_empty() {
            errors.push(format!("row.{}.last_opened_missing", start_row.row_id));
        }
        if start_row.target_kind_label.trim().is_empty() {
            errors.push(format!(
                "row.{}.target_kind_label_missing",
                start_row.row_id
            ));
        }
        validate_recovery_actions(start_row, errors);

        let Some(switcher_row) = page
            .workspace_switcher_rows
            .iter()
            .find(|row| row.recent_work_entry_ref == start_row.recent_work_entry_ref)
        else {
            errors.push(format!("row.{}.switcher_pair_missing", start_row.row_id));
            continue;
        };
        if start_row.target_kind != switcher_row.target_kind {
            errors.push(format!("row.{}.target_kind_drift", start_row.row_id));
        }
        if start_row.target_state != switcher_row.target_state {
            errors.push(format!("row.{}.target_state_drift", start_row.row_id));
        }
        if start_row.failure_state != switcher_row.failure_state {
            errors.push(format!("row.{}.failure_state_drift", start_row.row_id));
        }
        if start_row.trust_state != switcher_row.trust_state {
            errors.push(format!("row.{}.trust_state_drift", start_row.row_id));
        }
        if start_row.restore_availability != switcher_row.restore_availability {
            errors.push(format!(
                "row.{}.restore_availability_drift",
                start_row.row_id
            ));
        }
        if !switcher_row
            .switch_failure_actions
            .iter()
            .any(|action| action == WorkspaceSwitchRecoveryAction::CancelSwitch.as_str())
        {
            errors.push(format!("row.{}.cancel_switch_missing", switcher_row.row_id));
        }
        if !switcher_row
            .switch_failure_actions
            .iter()
            .any(|action| action == WorkspaceSwitchRecoveryAction::ReopenPreviousWorkspace.as_str())
        {
            errors.push(format!(
                "row.{}.reopen_previous_missing",
                switcher_row.row_id
            ));
        }
    }

    for required in ["missing_path", "moved_root", "reconnect_required"] {
        if !seen_failures.contains(required) {
            errors.push(format!("failure_coverage.{required}.missing"));
        }
    }
}

fn validate_recovery_actions(row: &StartCenterSwitcherBetaWorkRow, errors: &mut Vec<String>) {
    let action_ids = action_ids(&row.row_actions);
    match row.failure_state {
        RecentWorkFailureState::Ready => {
            if !contains_any(&action_ids, &["open", "open_in_new_window"]) {
                errors.push(format!("row.{}.open_action_missing", row.row_id));
            }
        }
        RecentWorkFailureState::MissingPath | RecentWorkFailureState::MovedRoot => {
            for required in [
                SafeRecoveryAction::LocateMissingTarget.as_str(),
                SafeRecoveryAction::OpenWithoutRestore.as_str(),
                SafeRecoveryAction::RemoveFromRecents.as_str(),
            ] {
                if !action_ids.iter().any(|action| action == required) {
                    errors.push(format!("row.{}.action.{required}.missing", row.row_id));
                }
            }
            if row.placeholder_summary.is_none() {
                errors.push(format!("row.{}.placeholder_summary_missing", row.row_id));
            }
        }
        RecentWorkFailureState::ReconnectRequired => {
            if !contains_any(
                &action_ids,
                &[
                    SafeRecoveryAction::Reconnect.as_str(),
                    SafeRecoveryAction::Reauth.as_str(),
                ],
            ) {
                errors.push(format!("row.{}.reconnect_or_reauth_missing", row.row_id));
            }
            for required in [
                SafeRecoveryAction::OpenWithoutRestore.as_str(),
                SafeRecoveryAction::RemoveFromRecents.as_str(),
            ] {
                if !action_ids.iter().any(|action| action == required) {
                    errors.push(format!("row.{}.action.{required}.missing", row.row_id));
                }
            }
        }
        RecentWorkFailureState::InspectOnly => {
            if !action_ids
                .iter()
                .any(|action| action == SafeRecoveryAction::OpenReadOnlyCachedView.as_str())
            {
                errors.push(format!("row.{}.cached_view_missing", row.row_id));
            }
        }
        RecentWorkFailureState::Blocked | RecentWorkFailureState::Unknown => {
            if !contains_any(
                &action_ids,
                &[
                    SafeRecoveryAction::OpenRestricted.as_str(),
                    SafeRecoveryAction::RetryLater.as_str(),
                ],
            ) {
                errors.push(format!("row.{}.restricted_route_missing", row.row_id));
            }
        }
    }
}

fn validate_support_rows(page: &StartCenterSwitcherBetaPage, errors: &mut Vec<String>) {
    if page.support_rows.len() != page.start_center_rows.len() {
        errors.push("support_rows.count_drift".to_string());
    }
    for support_row in &page.support_rows {
        let Some(start_row) = page
            .start_center_rows
            .iter()
            .find(|row| row.row_id == support_row.start_center_row_id)
        else {
            errors.push(format!(
                "support_row.{}.start_center_missing",
                support_row.recent_work_entry_ref
            ));
            continue;
        };
        let Some(switcher_row) = page
            .workspace_switcher_rows
            .iter()
            .find(|row| row.row_id == support_row.workspace_switcher_row_id)
        else {
            errors.push(format!(
                "support_row.{}.switcher_missing",
                support_row.recent_work_entry_ref
            ));
            continue;
        };
        if support_row.target_kind != start_row.target_kind
            || support_row.target_kind != switcher_row.target_kind
        {
            errors.push(format!(
                "support_row.{}.target_kind_drift",
                support_row.recent_work_entry_ref
            ));
        }
        if support_row.failure_state != start_row.failure_state
            || support_row.failure_state != switcher_row.failure_state
        {
            errors.push(format!(
                "support_row.{}.failure_state_drift",
                support_row.recent_work_entry_ref
            ));
        }
        if support_row.start_center_action_ids != action_ids(&start_row.row_actions) {
            errors.push(format!(
                "support_row.{}.start_actions_drift",
                support_row.recent_work_entry_ref
            ));
        }
        if support_row.workspace_switcher_action_ids != action_ids(&switcher_row.row_actions) {
            errors.push(format!(
                "support_row.{}.switcher_actions_drift",
                support_row.recent_work_entry_ref
            ));
        }
        if !support_row.metadata_only_cleanup {
            errors.push(format!(
                "support_row.{}.cleanup_scope_invalid",
                support_row.recent_work_entry_ref
            ));
        }
    }
}

fn action_ids(actions: &[StartCenterSwitcherRecoveryActionRecord]) -> Vec<String> {
    actions
        .iter()
        .map(|action| action.action_id.clone())
        .collect()
}

fn contains_any(actions: &[String], required: &[&str]) -> bool {
    required
        .iter()
        .any(|required| actions.iter().any(|action| action == required))
}

fn action_role(action: SafeRecoveryAction) -> StartCenterSwitcherActionRole {
    match action {
        SafeRecoveryAction::Open
        | SafeRecoveryAction::OpenInNewWindow
        | SafeRecoveryAction::OpenRestricted
        | SafeRecoveryAction::OpenReadOnlyCachedView
        | SafeRecoveryAction::OpenWithoutRestore => StartCenterSwitcherActionRole::Primary,
        SafeRecoveryAction::LocateMissingTarget
        | SafeRecoveryAction::Reconnect
        | SafeRecoveryAction::Reauth
        | SafeRecoveryAction::RetryLater
        | SafeRecoveryAction::CompareBeforeRestore => StartCenterSwitcherActionRole::Recovery,
        SafeRecoveryAction::RemoveFromRecents => {
            StartCenterSwitcherActionRole::DestructiveMetadataOnly
        }
        SafeRecoveryAction::Unpin
        | SafeRecoveryAction::Pin
        | SafeRecoveryAction::RevealInExplorer => StartCenterSwitcherActionRole::Secondary,
    }
}

fn write_safety_badge(failure_state: RecentWorkFailureState) -> &'static str {
    match failure_state {
        RecentWorkFailureState::Ready => "writes_allowed",
        RecentWorkFailureState::MissingPath | RecentWorkFailureState::MovedRoot => {
            "writes_blocked_target_unavailable"
        }
        RecentWorkFailureState::ReconnectRequired => "writes_unsafe_stale_or_disconnected",
        RecentWorkFailureState::InspectOnly => "writes_blocked_cached_view_only",
        RecentWorkFailureState::Blocked => "writes_blocked_policy",
        RecentWorkFailureState::Unknown => "writes_require_revalidation",
    }
}

fn preflight_class_as_str(class: PreflightDecisionClass) -> &'static str {
    match class {
        PreflightDecisionClass::Allowed => "allowed",
        PreflightDecisionClass::BlockedByPolicy => "blocked_by_policy",
        PreflightDecisionClass::DisabledWithReason => "disabled_with_reason",
        PreflightDecisionClass::PreviewRequired => "preview_required",
        PreflightDecisionClass::ApprovalRequired => "approval_required",
    }
}

struct RecentEntrySeed {
    recent_work_id: &'static str,
    presentation_label: &'static str,
    presentation_subtitle: Option<&'static str>,
    target_kind: TargetKind,
    target_state: RecentWorkTargetState,
    portability_class: PortabilityClass,
    trust_state: TrustState,
    restore_availability: RestoreAvailability,
    safe_recovery_actions: Vec<SafeRecoveryAction>,
    pinned: bool,
    last_opened_at: &'static str,
    filesystem_identity_ref: Option<&'static str>,
    remote_target_descriptor_ref: Option<&'static str>,
    artifact_descriptor_ref: Option<&'static str>,
    recovery_checkpoint_refs: Option<Vec<RecoveryCheckpointRef>>,
}

fn recent_entry(seed: RecentEntrySeed) -> RecentWorkEntryRecord {
    RecentWorkEntryRecord {
        record_kind: RecentWorkEntryRecordKind::RecentWorkEntryRecord,
        entry_and_restore_schema_version: 1,
        recent_work_id: seed.recent_work_id.to_string(),
        presentation_label: seed.presentation_label.to_string(),
        presentation_subtitle: seed.presentation_subtitle.map(str::to_string),
        target_kind: seed.target_kind,
        target_state: seed.target_state,
        portability_class: seed.portability_class,
        trust_state: seed.trust_state,
        restore_availability: seed.restore_availability,
        safe_recovery_actions: seed.safe_recovery_actions,
        pinned: seed.pinned,
        last_opened_at: seed.last_opened_at.to_string(),
        filesystem_identity_ref: seed.filesystem_identity_ref.map(str::to_string),
        remote_target_descriptor_ref: seed.remote_target_descriptor_ref.map(str::to_string),
        artifact_descriptor_ref: seed.artifact_descriptor_ref.map(str::to_string),
        recovery_checkpoint_refs: seed.recovery_checkpoint_refs,
    }
}

fn checkpoint(recovery_class: &str, checkpoint_ref: &str) -> RecoveryCheckpointRef {
    RecoveryCheckpointRef {
        recovery_class: recovery_class.to_string(),
        checkpoint_ref: checkpoint_ref.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_switcher_and_support_parity() {
        let page = seeded_start_center_switcher_beta_page();
        validate_start_center_switcher_beta_page(&page).expect("page must validate");
        assert_eq!(page.primary_actions.len(), 5);
        assert_eq!(
            page.start_center_rows.len(),
            page.workspace_switcher_rows.len()
        );
        assert!(page.summary.pinned_row_count > 0);
        assert!(page.summary.recent_row_count > 0);
        assert!(page
            .start_center_rows
            .iter()
            .any(|row| row.failure_state == RecentWorkFailureState::MissingPath));
        assert!(page
            .start_center_rows
            .iter()
            .any(|row| row.failure_state == RecentWorkFailureState::MovedRoot));
        assert!(page
            .start_center_rows
            .iter()
            .any(|row| row.failure_state == RecentWorkFailureState::ReconnectRequired));
    }

    #[test]
    fn support_export_validates_against_embedded_page() {
        let page = seeded_start_center_switcher_beta_page();
        let export = StartCenterSwitcherBetaSupportExport::from_page(
            "support-export:start-center-switcher-beta:001",
            "2026-05-17T00:00:00Z",
            page,
        );
        validate_start_center_switcher_beta_support_export(&export)
            .expect("support export must validate");
        assert!(export.raw_secret_material_excluded);
    }

    #[test]
    fn search_projection_keeps_pinned_and_recent_sections() {
        let registry = seeded_start_center_beta_recent_work_registry();
        let projection = build_searchable_recent_work_rows(
            &registry,
            StartCenterRecentWorkPrivacyMode::Default,
            "ssh",
        );
        assert_eq!(projection.pinned_rows.len(), 0);
        assert_eq!(projection.recent_rows.len(), 1);
        assert_eq!(
            projection.recent_rows[0].target_kind,
            TargetKind::SshWorkspace
        );
    }

    #[test]
    fn validator_rejects_missing_switch_failure_return_path() {
        let mut page = seeded_start_center_switcher_beta_page();
        page.workspace_switcher_rows[0]
            .switch_failure_actions
            .clear();
        page.summary = StartCenterSwitcherBetaSummary::from_rows(
            &page.primary_actions,
            &page.start_center_rows,
            &page.workspace_switcher_rows,
        );
        let errors = validate_start_center_switcher_beta_page(&page)
            .expect_err("page must reject missing return path");
        assert!(errors
            .iter()
            .any(|error| error.contains("reopen_previous_missing")));
    }
}
