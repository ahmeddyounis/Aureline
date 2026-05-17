//! Support Center beta shell surface.
//!
//! Bounded shell-side projection that gathers the recovery-ladder
//! entries, the current claim/degraded-state truth, and the export
//! routes into one keyboard-reachable surface. The module never owns
//! the live recovery decisions, the live scorecards, or the live
//! support bundles: each row is a typed reference that the user can
//! launch (`launch_action_rows`), open as evidence (`claim_truth_rows`,
//! `degraded_truth_rows`), or follow into a local-first export
//! (`export_route_rows`). Truth keeps living in
//! [`aureline_support`]; the Support Center surface is the place a
//! blocked user lands so they don't have to search docs.
//!
//! ## Honesty invariants enforced here
//!
//! - The surface is reachable by keyboard. Every action row pins at
//!   least one [`SupportCenterKeyboardReachClass`] and a stable
//!   `cmd:*` id.
//! - The recovery-ladder entry set is provable on the row set:
//!   exactly one row each for safe-mode, doctor, bisect, repair
//!   preview, and an export action must be present.
//! - Local-only recovery lanes never require account creation or a
//!   hosted service. The surface refuses any row whose
//!   `service_dependency_class` is `requires_*` or whose
//!   `account_requirement_class` is anything but
//!   `no_account_required` on a local-only row.
//! - Every launch action preserves `user_authored_files`. No row may
//!   be projected that would delete user-owned state.
//! - Every export route names a local-first path; an
//!   `upload_required_for_first_action: true` row is refused.
//! - The support-packet projection is metadata-safe: raw private
//!   material and ambient authority are excluded.
//!
//! This module is bounded to the shell-side projection and its
//! validator. Wiring the rows into a live panel, the command palette,
//! or the headless renderer is reserved for a later milestone; the
//! projection here is what those surfaces will consume.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one launchable action row.
pub const SUPPORT_CENTER_LAUNCH_ACTION_ROW_RECORD_KIND: &str =
    "support_center_launch_action_row_record";

/// Stable record-kind tag for one claim-truth row.
pub const SUPPORT_CENTER_CLAIM_TRUTH_ROW_RECORD_KIND: &str =
    "support_center_claim_truth_row_record";

/// Stable record-kind tag for one degraded-truth row.
pub const SUPPORT_CENTER_DEGRADED_TRUTH_ROW_RECORD_KIND: &str =
    "support_center_degraded_truth_row_record";

/// Stable record-kind tag for one export-route row.
pub const SUPPORT_CENTER_EXPORT_ROUTE_ROW_RECORD_KIND: &str =
    "support_center_export_route_row_record";

/// Stable record-kind tag for the surface record.
pub const SUPPORT_CENTER_BETA_SURFACE_RECORD_KIND: &str = "support_center_beta_surface_record";

/// Stable record-kind tag for the metadata-safe support packet.
pub const SUPPORT_CENTER_BETA_SUPPORT_PACKET_RECORD_KIND: &str =
    "support_center_beta_support_packet_record";

/// Frozen schema version for the Support Center beta projection.
pub const SUPPORT_CENTER_BETA_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema this module mirrors.
pub const SUPPORT_CENTER_BETA_SCHEMA_REF: &str = "schemas/support/support_center_beta.schema.json";

/// Repo-relative path of the reviewer doc.
pub const SUPPORT_CENTER_BETA_DOC_REF: &str = "docs/ux/m3/support_center_beta.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const SUPPORT_CENTER_BETA_FIXTURES_DIR: &str = "fixtures/ux/m3/support_center";

/// Stable command id the chrome routes when the user opens the
/// Support Center surface itself.
pub const COMMAND_ID_OPEN_SUPPORT_CENTER: &str = "cmd:support_center.open";

/// Closed beta-lane vocabulary. Reuses the seven M3 beta lanes from
/// `aureline-support` plus the three Support Center cards
/// (`recovery_ladder`, `support_bundle`, `crash_triage`) that launch
/// from the same surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportCenterBetaLaneClass {
    SafeMode,
    ExtensionBisect,
    RepairTransactionPreview,
    DoctorProbePacks,
    ProjectDoctorFindingContract,
    RecordsGovernance,
    RuntimeReplayPackets,
    RecoveryLadder,
    SupportBundle,
    CrashTriage,
}

impl SupportCenterBetaLaneClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeMode => "safe_mode",
            Self::ExtensionBisect => "extension_bisect",
            Self::RepairTransactionPreview => "repair_transaction_preview",
            Self::DoctorProbePacks => "doctor_probe_packs",
            Self::ProjectDoctorFindingContract => "project_doctor_finding_contract",
            Self::RecordsGovernance => "records_governance",
            Self::RuntimeReplayPackets => "runtime_replay_packets",
            Self::RecoveryLadder => "recovery_ladder",
            Self::SupportBundle => "support_bundle",
            Self::CrashTriage => "crash_triage",
        }
    }
}

/// Closed set of launch actions reachable from the Support Center
/// beta surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportCenterLaunchActionClass {
    EnterSafeMode,
    OpenProjectDoctor,
    StartExtensionBisect,
    OpenRepairPreview,
    OpenRecoveryLadder,
    OpenCrashTriage,
    PreviewSupportBundle,
    ExportSupportBundle,
    ExportObjectHandoffPacket,
    ExportEscalationPacketDraft,
}

impl SupportCenterLaunchActionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnterSafeMode => "enter_safe_mode",
            Self::OpenProjectDoctor => "open_project_doctor",
            Self::StartExtensionBisect => "start_extension_bisect",
            Self::OpenRepairPreview => "open_repair_preview",
            Self::OpenRecoveryLadder => "open_recovery_ladder",
            Self::OpenCrashTriage => "open_crash_triage",
            Self::PreviewSupportBundle => "preview_support_bundle",
            Self::ExportSupportBundle => "export_support_bundle",
            Self::ExportObjectHandoffPacket => "export_object_handoff_packet",
            Self::ExportEscalationPacketDraft => "export_escalation_packet_draft",
        }
    }

    /// True when the action is an export route the surface MUST also
    /// list on the `export_route_rows` side.
    pub const fn is_export_action(self) -> bool {
        matches!(
            self,
            Self::PreviewSupportBundle
                | Self::ExportSupportBundle
                | Self::ExportObjectHandoffPacket
                | Self::ExportEscalationPacketDraft
        )
    }
}

/// Closed keyboard-reach vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportCenterKeyboardReachClass {
    PaletteCommand,
    DefaultKeyboardShortcut,
    PanelFocusChord,
    StartCenterEntry,
}

impl SupportCenterKeyboardReachClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PaletteCommand => "palette_command",
            Self::DefaultKeyboardShortcut => "default_keyboard_shortcut",
            Self::PanelFocusChord => "panel_focus_chord",
            Self::StartCenterEntry => "start_center_entry",
        }
    }
}

/// Closed account-requirement vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportCenterAccountRequirementClass {
    NoAccountRequired,
    ManagedAdminAccountRequiredForApply,
    ManagedAdminAccountRequiredForHandoff,
}

impl SupportCenterAccountRequirementClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAccountRequired => "no_account_required",
            Self::ManagedAdminAccountRequiredForApply => "managed_admin_account_required_for_apply",
            Self::ManagedAdminAccountRequiredForHandoff => {
                "managed_admin_account_required_for_handoff"
            }
        }
    }
}

/// Closed service-dependency vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportCenterServiceDependencyClass {
    LocalOnly,
    OptionalMirrorCache,
    OptionalManagedPolicySync,
    RequiresManagedControlPlane,
    RequiresHostedIntake,
}

impl SupportCenterServiceDependencyClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::OptionalMirrorCache => "optional_mirror_cache",
            Self::OptionalManagedPolicySync => "optional_managed_policy_sync",
            Self::RequiresManagedControlPlane => "requires_managed_control_plane",
            Self::RequiresHostedIntake => "requires_hosted_intake",
        }
    }

    /// True when the dependency requires a hosted service to be
    /// reachable for the action to run.
    pub const fn requires_hosted_service(self) -> bool {
        matches!(
            self,
            Self::RequiresManagedControlPlane | Self::RequiresHostedIntake
        )
    }
}

/// Closed claim-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportCenterClaimStateClass {
    GreenClaimProven,
    YellowAgingEvidence,
    RedBlocksClaim,
    NotClaimedInThisMilestone,
    DegradedLocalOnly,
}

impl SupportCenterClaimStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GreenClaimProven => "green_claim_proven",
            Self::YellowAgingEvidence => "yellow_aging_evidence",
            Self::RedBlocksClaim => "red_blocks_claim",
            Self::NotClaimedInThisMilestone => "not_claimed_in_this_milestone",
            Self::DegradedLocalOnly => "degraded_local_only",
        }
    }
}

/// Closed degraded-truth vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportCenterDegradedTruthClass {
    NoneDegraded,
    SafeModeActive,
    ExtensionQuarantined,
    CacheOrIndexRepairPending,
    RestrictedFallbackActive,
    SessionRestoreSkipped,
    CrashLoopContainmentActive,
}

impl SupportCenterDegradedTruthClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDegraded => "none_degraded",
            Self::SafeModeActive => "safe_mode_active",
            Self::ExtensionQuarantined => "extension_quarantined",
            Self::CacheOrIndexRepairPending => "cache_or_index_repair_pending",
            Self::RestrictedFallbackActive => "restricted_fallback_active",
            Self::SessionRestoreSkipped => "session_restore_skipped",
            Self::CrashLoopContainmentActive => "crash_loop_containment_active",
        }
    }
}

/// Closed export-route destination vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportCenterExportRouteDestinationClass {
    SupportBundlePreviewLocalFirst,
    DoctorFindingRecordExport,
    RecoveryLadderSupportPacket,
    ObjectHandoffPacketDraft,
    EscalationPacketDraft,
    NoExportDestination,
}

impl SupportCenterExportRouteDestinationClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportBundlePreviewLocalFirst => "support_bundle_preview_local_first",
            Self::DoctorFindingRecordExport => "doctor_finding_record_export",
            Self::RecoveryLadderSupportPacket => "recovery_ladder_support_packet",
            Self::ObjectHandoffPacketDraft => "object_handoff_packet_draft",
            Self::EscalationPacketDraft => "escalation_packet_draft",
            Self::NoExportDestination => "no_export_destination",
        }
    }
}

/// Closed deployment-context vocabulary; re-exported from
/// `schemas/support/support_center_capability_card.schema.json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportCenterDeploymentContextClass {
    LocalOnly,
    Managed,
    SelfHosted,
    Mirrored,
    Offline,
}

impl SupportCenterDeploymentContextClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Managed => "managed",
            Self::SelfHosted => "self_hosted",
            Self::Mirrored => "mirrored",
            Self::Offline => "offline",
        }
    }
}

/// Closed preserved-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportCenterPreservedStateClass {
    UserAuthoredFiles,
    OpenBufferSelection,
    DurableWorkspaceIndexes,
    WorkspaceTrustStore,
    CredentialStore,
    SessionRestoreStore,
    SupportExportStore,
}

impl SupportCenterPreservedStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAuthoredFiles => "user_authored_files",
            Self::OpenBufferSelection => "open_buffer_selection",
            Self::DurableWorkspaceIndexes => "durable_workspace_indexes",
            Self::WorkspaceTrustStore => "workspace_trust_store",
            Self::CredentialStore => "credential_store",
            Self::SessionRestoreStore => "session_restore_store",
            Self::SupportExportStore => "support_export_store",
        }
    }
}

/// One launchable action row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCenterLaunchActionRow {
    pub schema_version: u32,
    pub record_kind: String,
    pub row_id: String,
    pub beta_lane_class: SupportCenterBetaLaneClass,
    pub launch_action_class: SupportCenterLaunchActionClass,
    pub command_id: String,
    pub keyboard_reach_classes: Vec<SupportCenterKeyboardReachClass>,
    pub account_requirement_class: SupportCenterAccountRequirementClass,
    pub service_dependency_class: SupportCenterServiceDependencyClass,
    pub preserved_state_classes: Vec<SupportCenterPreservedStateClass>,
    pub label: String,
    pub summary: String,
}

/// One claim-truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCenterClaimTruthRow {
    pub schema_version: u32,
    pub record_kind: String,
    pub row_id: String,
    pub beta_lane_class: SupportCenterBetaLaneClass,
    pub claim_state_class: SupportCenterClaimStateClass,
    pub scorecard_target: String,
    pub evidence_packet_ref: String,
    pub summary: String,
}

/// One degraded-truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCenterDegradedTruthRow {
    pub schema_version: u32,
    pub record_kind: String,
    pub row_id: String,
    pub degraded_truth_class: SupportCenterDegradedTruthClass,
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_command_id: Option<String>,
    pub summary: String,
}

/// One export-route row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCenterExportRouteRow {
    pub schema_version: u32,
    pub record_kind: String,
    pub row_id: String,
    pub launch_action_class: SupportCenterLaunchActionClass,
    pub destination_class: SupportCenterExportRouteDestinationClass,
    pub destination_schema_ref: String,
    pub local_first_path_named: bool,
    pub upload_required_for_first_action: bool,
    pub summary: String,
}

/// Governance bindings pinned on every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCenterGovernanceBindings {
    pub support_center_concept_ref: String,
    pub support_center_ia_ref: String,
    pub support_center_routes_ref: String,
    pub m3_scenario_corpus_ref: String,
    pub recovery_ladder_doc_ref: String,
    pub safe_mode_doc_ref: String,
    pub extension_bisect_doc_ref: String,
    pub repair_transaction_doc_ref: String,
    pub project_doctor_doc_ref: String,
    pub support_bundle_contract_ref: String,
}

/// Support Center beta surface record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCenterBetaSurface {
    pub schema_version: u32,
    pub record_kind: String,
    pub surface_id: String,
    pub title: String,
    pub summary: String,
    pub deployment_context_class: SupportCenterDeploymentContextClass,
    pub primary_command_id: String,
    pub primary_keyboard_reach_class: SupportCenterKeyboardReachClass,
    pub launch_action_rows: Vec<SupportCenterLaunchActionRow>,
    pub claim_truth_rows: Vec<SupportCenterClaimTruthRow>,
    pub degraded_truth_rows: Vec<SupportCenterDegradedTruthRow>,
    pub export_route_rows: Vec<SupportCenterExportRouteRow>,
    pub no_account_required_for_local_only: bool,
    pub no_hidden_service_required_for_local_only: bool,
    pub ia_capability_card_refs: Vec<String>,
    pub ia_route_refs: Vec<String>,
    pub governance_bindings: SupportCenterGovernanceBindings,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub emitted_at: String,
}

/// Metadata-safe support packet projection of the surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCenterBetaSupportPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub surface_ref: String,
    pub launch_action_row_refs: Vec<String>,
    pub claim_truth_row_refs: Vec<String>,
    pub degraded_truth_row_refs: Vec<String>,
    pub export_route_row_refs: Vec<String>,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub emitted_at: String,
}

/// One evaluator violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCenterBetaViolation {
    pub check_id: String,
    pub target_ref: String,
    pub message: String,
}

/// Loads a Support Center beta surface from YAML text.
pub fn load_surface_from_yaml(yaml: &str) -> Result<SupportCenterBetaSurface, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads a Support Center beta surface from JSON text.
pub fn load_surface_from_json(json: &str) -> Result<SupportCenterBetaSurface, serde_json::Error> {
    serde_json::from_str(json)
}

/// Required launch-action classes the surface MUST cover.
pub const REQUIRED_LAUNCH_ACTION_CLASSES: [SupportCenterLaunchActionClass; 4] = [
    SupportCenterLaunchActionClass::EnterSafeMode,
    SupportCenterLaunchActionClass::OpenProjectDoctor,
    SupportCenterLaunchActionClass::StartExtensionBisect,
    SupportCenterLaunchActionClass::OpenRepairPreview,
];

/// Evaluator that validates a Support Center beta surface against the
/// acceptance contract.
pub struct SupportCenterBetaEvaluator;

impl SupportCenterBetaEvaluator {
    /// Validates the surface; returns one violation per failed check.
    pub fn validate(surface: &SupportCenterBetaSurface) -> Vec<SupportCenterBetaViolation> {
        let mut violations = Vec::new();

        if surface.schema_version != SUPPORT_CENTER_BETA_SCHEMA_VERSION {
            push(
                &mut violations,
                "surface.schema_version",
                &surface.surface_id,
                format!(
                    "schema_version must be {}, got {}",
                    SUPPORT_CENTER_BETA_SCHEMA_VERSION, surface.schema_version
                ),
            );
        }

        if surface.record_kind != SUPPORT_CENTER_BETA_SURFACE_RECORD_KIND {
            push(
                &mut violations,
                "surface.record_kind",
                &surface.surface_id,
                format!(
                    "record_kind must be {}",
                    SUPPORT_CENTER_BETA_SURFACE_RECORD_KIND
                ),
            );
        }

        if !surface.primary_command_id.starts_with("cmd:") {
            push(
                &mut violations,
                "surface.primary_command_id",
                &surface.surface_id,
                "primary_command_id must start with 'cmd:'".to_string(),
            );
        }

        if !surface.no_account_required_for_local_only {
            push(
                &mut violations,
                "surface.no_account_required_for_local_only",
                &surface.surface_id,
                "surface MUST pin no_account_required_for_local_only=true".to_string(),
            );
        }

        if !surface.no_hidden_service_required_for_local_only {
            push(
                &mut violations,
                "surface.no_hidden_service_required_for_local_only",
                &surface.surface_id,
                "surface MUST pin no_hidden_service_required_for_local_only=true".to_string(),
            );
        }

        Self::validate_required_actions(surface, &mut violations);
        Self::validate_launch_action_rows(surface, &mut violations);
        Self::validate_claim_truth_rows(surface, &mut violations);
        Self::validate_degraded_truth_rows(surface, &mut violations);
        Self::validate_export_route_rows(surface, &mut violations);

        violations
    }

    fn validate_required_actions(
        surface: &SupportCenterBetaSurface,
        violations: &mut Vec<SupportCenterBetaViolation>,
    ) {
        let mut seen: BTreeSet<SupportCenterLaunchActionClass> = BTreeSet::new();
        let mut export_action_present = false;
        for row in &surface.launch_action_rows {
            seen.insert(row.launch_action_class);
            if row.launch_action_class.is_export_action() {
                export_action_present = true;
            }
        }
        for required in REQUIRED_LAUNCH_ACTION_CLASSES {
            if !seen.contains(&required) {
                push(
                    violations,
                    "surface.required_launch_action_missing",
                    &surface.surface_id,
                    format!(
                        "launch action {} MUST be present on the surface",
                        required.as_str()
                    ),
                );
            }
        }
        if !export_action_present {
            push(
                violations,
                "surface.required_export_action_missing",
                &surface.surface_id,
                "surface MUST list at least one export or preview action row".to_string(),
            );
        }
    }

    fn validate_launch_action_rows(
        surface: &SupportCenterBetaSurface,
        violations: &mut Vec<SupportCenterBetaViolation>,
    ) {
        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        for row in &surface.launch_action_rows {
            if row.record_kind != SUPPORT_CENTER_LAUNCH_ACTION_ROW_RECORD_KIND {
                push(
                    violations,
                    "launch_action.record_kind",
                    &row.row_id,
                    format!(
                        "record_kind must be {}",
                        SUPPORT_CENTER_LAUNCH_ACTION_ROW_RECORD_KIND
                    ),
                );
            }
            if !seen_ids.insert(row.row_id.as_str()) {
                push(
                    violations,
                    "launch_action.row_id_duplicate",
                    &row.row_id,
                    "duplicate launch-action row_id".to_string(),
                );
            }
            if !row.command_id.starts_with("cmd:") {
                push(
                    violations,
                    "launch_action.command_id",
                    &row.row_id,
                    "command_id must start with 'cmd:'".to_string(),
                );
            }
            if row.keyboard_reach_classes.is_empty() {
                push(
                    violations,
                    "launch_action.keyboard_reach_empty",
                    &row.row_id,
                    "launch action MUST declare at least one keyboard reach class".to_string(),
                );
            }
            if !row
                .preserved_state_classes
                .iter()
                .any(|c| *c == SupportCenterPreservedStateClass::UserAuthoredFiles)
            {
                push(
                    violations,
                    "launch_action.preserved_state.user_authored_files_missing",
                    &row.row_id,
                    "launch action MUST preserve user_authored_files".to_string(),
                );
            }
            if surface.deployment_context_class == SupportCenterDeploymentContextClass::LocalOnly
                && row.account_requirement_class
                    != SupportCenterAccountRequirementClass::NoAccountRequired
            {
                push(
                    violations,
                    "launch_action.local_only_account_requirement",
                    &row.row_id,
                    "local-only surface MUST NOT require an account on a launch action"
                        .to_string(),
                );
            }
            if surface.deployment_context_class == SupportCenterDeploymentContextClass::LocalOnly
                && row.service_dependency_class.requires_hosted_service()
            {
                push(
                    violations,
                    "launch_action.local_only_service_dependency",
                    &row.row_id,
                    "local-only surface MUST NOT require a hosted service on a launch action"
                        .to_string(),
                );
            }
            // On any deployment context, a launch action for a
            // local-only recovery lane (safe mode, doctor, bisect,
            // repair preview, recovery ladder) must not require a
            // hosted service or an account.
            if Self::is_local_only_recovery_lane(row.beta_lane_class) {
                if row.service_dependency_class.requires_hosted_service() {
                    push(
                        violations,
                        "launch_action.local_only_lane_hosted_service",
                        &row.row_id,
                        format!(
                            "launch action for local-only recovery lane {} MUST NOT require a hosted service",
                            row.beta_lane_class.as_str()
                        ),
                    );
                }
                if row.account_requirement_class
                    != SupportCenterAccountRequirementClass::NoAccountRequired
                {
                    push(
                        violations,
                        "launch_action.local_only_lane_account",
                        &row.row_id,
                        format!(
                            "launch action for local-only recovery lane {} MUST NOT require an account",
                            row.beta_lane_class.as_str()
                        ),
                    );
                }
            }
        }
    }

    fn validate_claim_truth_rows(
        surface: &SupportCenterBetaSurface,
        violations: &mut Vec<SupportCenterBetaViolation>,
    ) {
        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        for row in &surface.claim_truth_rows {
            if row.record_kind != SUPPORT_CENTER_CLAIM_TRUTH_ROW_RECORD_KIND {
                push(
                    violations,
                    "claim_truth.record_kind",
                    &row.row_id,
                    format!(
                        "record_kind must be {}",
                        SUPPORT_CENTER_CLAIM_TRUTH_ROW_RECORD_KIND
                    ),
                );
            }
            if !seen_ids.insert(row.row_id.as_str()) {
                push(
                    violations,
                    "claim_truth.row_id_duplicate",
                    &row.row_id,
                    "duplicate claim-truth row_id".to_string(),
                );
            }
            if row.scorecard_target.is_empty() {
                push(
                    violations,
                    "claim_truth.scorecard_target",
                    &row.row_id,
                    "scorecard_target must be a non-empty stable id".to_string(),
                );
            }
            if row.evidence_packet_ref.is_empty() {
                push(
                    violations,
                    "claim_truth.evidence_packet_ref",
                    &row.row_id,
                    "evidence_packet_ref must be a non-empty repo-relative ref".to_string(),
                );
            }
        }
    }

    fn validate_degraded_truth_rows(
        surface: &SupportCenterBetaSurface,
        violations: &mut Vec<SupportCenterBetaViolation>,
    ) {
        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        for row in &surface.degraded_truth_rows {
            if row.record_kind != SUPPORT_CENTER_DEGRADED_TRUTH_ROW_RECORD_KIND {
                push(
                    violations,
                    "degraded_truth.record_kind",
                    &row.row_id,
                    format!(
                        "record_kind must be {}",
                        SUPPORT_CENTER_DEGRADED_TRUTH_ROW_RECORD_KIND
                    ),
                );
            }
            if !seen_ids.insert(row.row_id.as_str()) {
                push(
                    violations,
                    "degraded_truth.row_id_duplicate",
                    &row.row_id,
                    "duplicate degraded-truth row_id".to_string(),
                );
            }
            match row.degraded_truth_class {
                SupportCenterDegradedTruthClass::NoneDegraded => {
                    if row.active {
                        push(
                            violations,
                            "degraded_truth.none_active_mismatch",
                            &row.row_id,
                            "none_degraded row MUST have active=false".to_string(),
                        );
                    }
                    if row.exit_command_id.is_some() {
                        push(
                            violations,
                            "degraded_truth.none_exit_command_present",
                            &row.row_id,
                            "none_degraded row MUST NOT carry an exit_command_id".to_string(),
                        );
                    }
                }
                _ => {
                    if row.active {
                        match &row.exit_command_id {
                            Some(id) if id.starts_with("cmd:") => {}
                            _ => push(
                                violations,
                                "degraded_truth.exit_command_missing",
                                &row.row_id,
                                "active degraded-truth row MUST name an exit_command_id starting with 'cmd:'"
                                    .to_string(),
                            ),
                        }
                    }
                }
            }
        }
    }

    fn validate_export_route_rows(
        surface: &SupportCenterBetaSurface,
        violations: &mut Vec<SupportCenterBetaViolation>,
    ) {
        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        for row in &surface.export_route_rows {
            if row.record_kind != SUPPORT_CENTER_EXPORT_ROUTE_ROW_RECORD_KIND {
                push(
                    violations,
                    "export_route.record_kind",
                    &row.row_id,
                    format!(
                        "record_kind must be {}",
                        SUPPORT_CENTER_EXPORT_ROUTE_ROW_RECORD_KIND
                    ),
                );
            }
            if !seen_ids.insert(row.row_id.as_str()) {
                push(
                    violations,
                    "export_route.row_id_duplicate",
                    &row.row_id,
                    "duplicate export-route row_id".to_string(),
                );
            }
            if !row.local_first_path_named {
                push(
                    violations,
                    "export_route.local_first_path_named",
                    &row.row_id,
                    "export route MUST pin local_first_path_named=true".to_string(),
                );
            }
            if row.upload_required_for_first_action {
                push(
                    violations,
                    "export_route.upload_required_for_first_action",
                    &row.row_id,
                    "export route MUST pin upload_required_for_first_action=false".to_string(),
                );
            }
            if row.destination_class
                == SupportCenterExportRouteDestinationClass::NoExportDestination
                && row.destination_schema_ref != "no_destination_schema"
            {
                push(
                    violations,
                    "export_route.no_destination_schema_token",
                    &row.row_id,
                    "no_export_destination rows MUST carry the literal token 'no_destination_schema'"
                        .to_string(),
                );
            }
        }
    }

    /// Returns true when the lane is one of the local-only recovery
    /// lanes that MUST remain accountless and service-free.
    pub const fn is_local_only_recovery_lane(lane: SupportCenterBetaLaneClass) -> bool {
        matches!(
            lane,
            SupportCenterBetaLaneClass::SafeMode
                | SupportCenterBetaLaneClass::ExtensionBisect
                | SupportCenterBetaLaneClass::RepairTransactionPreview
                | SupportCenterBetaLaneClass::DoctorProbePacks
                | SupportCenterBetaLaneClass::ProjectDoctorFindingContract
                | SupportCenterBetaLaneClass::RecoveryLadder
                | SupportCenterBetaLaneClass::CrashTriage
        )
    }
}

impl SupportCenterBetaSurface {
    /// Builds a metadata-safe support packet projection of the
    /// surface. The packet quotes only stable row ids; raw private
    /// material and ambient authority are excluded.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        emitted_at: impl Into<String>,
    ) -> SupportCenterBetaSupportPacket {
        SupportCenterBetaSupportPacket {
            schema_version: SUPPORT_CENTER_BETA_SCHEMA_VERSION,
            record_kind: SUPPORT_CENTER_BETA_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            packet_id: packet_id.into(),
            surface_ref: self.surface_id.clone(),
            launch_action_row_refs: self
                .launch_action_rows
                .iter()
                .map(|r| r.row_id.clone())
                .collect(),
            claim_truth_row_refs: self
                .claim_truth_rows
                .iter()
                .map(|r| r.row_id.clone())
                .collect(),
            degraded_truth_row_refs: self
                .degraded_truth_rows
                .iter()
                .map(|r| r.row_id.clone())
                .collect(),
            export_route_row_refs: self
                .export_route_rows
                .iter()
                .map(|r| r.row_id.clone())
                .collect(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            notes: None,
            emitted_at: emitted_at.into(),
        }
    }
}

fn push(
    violations: &mut Vec<SupportCenterBetaViolation>,
    check_id: &str,
    target_ref: &str,
    message: String,
) {
    violations.push(SupportCenterBetaViolation {
        check_id: check_id.to_owned(),
        target_ref: target_ref.to_owned(),
        message,
    });
}

/// Error returned when a corpus cannot be loaded.
#[derive(Debug)]
pub enum SupportCenterCorpusError {
    /// A YAML parse failure on a specific fixture file.
    Parse {
        fixture_ref: String,
        source: serde_yaml::Error,
    },
}

impl fmt::Display for SupportCenterCorpusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse {
                fixture_ref,
                source,
            } => write!(f, "failed to parse {}: {}", fixture_ref, source),
        }
    }
}

impl Error for SupportCenterCorpusError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse { source, .. } => Some(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline_row(
        row_id: &str,
        lane: SupportCenterBetaLaneClass,
        action: SupportCenterLaunchActionClass,
    ) -> SupportCenterLaunchActionRow {
        SupportCenterLaunchActionRow {
            schema_version: SUPPORT_CENTER_BETA_SCHEMA_VERSION,
            record_kind: SUPPORT_CENTER_LAUNCH_ACTION_ROW_RECORD_KIND.into(),
            row_id: row_id.into(),
            beta_lane_class: lane,
            launch_action_class: action,
            command_id: format!("cmd:test.{}", action.as_str()),
            keyboard_reach_classes: vec![SupportCenterKeyboardReachClass::PaletteCommand],
            account_requirement_class: SupportCenterAccountRequirementClass::NoAccountRequired,
            service_dependency_class: SupportCenterServiceDependencyClass::LocalOnly,
            preserved_state_classes: vec![SupportCenterPreservedStateClass::UserAuthoredFiles],
            label: action.as_str().into(),
            summary: format!("Launch {}.", action.as_str()),
        }
    }

    fn baseline_surface() -> SupportCenterBetaSurface {
        SupportCenterBetaSurface {
            schema_version: SUPPORT_CENTER_BETA_SCHEMA_VERSION,
            record_kind: SUPPORT_CENTER_BETA_SURFACE_RECORD_KIND.into(),
            surface_id: "support_center_beta:surface:test.baseline".into(),
            title: "Support Center".into(),
            summary: "Baseline test surface.".into(),
            deployment_context_class: SupportCenterDeploymentContextClass::LocalOnly,
            primary_command_id: COMMAND_ID_OPEN_SUPPORT_CENTER.into(),
            primary_keyboard_reach_class: SupportCenterKeyboardReachClass::PaletteCommand,
            launch_action_rows: vec![
                baseline_row(
                    "support_center_beta:action:test.safe_mode",
                    SupportCenterBetaLaneClass::SafeMode,
                    SupportCenterLaunchActionClass::EnterSafeMode,
                ),
                baseline_row(
                    "support_center_beta:action:test.doctor",
                    SupportCenterBetaLaneClass::DoctorProbePacks,
                    SupportCenterLaunchActionClass::OpenProjectDoctor,
                ),
                baseline_row(
                    "support_center_beta:action:test.bisect",
                    SupportCenterBetaLaneClass::ExtensionBisect,
                    SupportCenterLaunchActionClass::StartExtensionBisect,
                ),
                baseline_row(
                    "support_center_beta:action:test.repair",
                    SupportCenterBetaLaneClass::RepairTransactionPreview,
                    SupportCenterLaunchActionClass::OpenRepairPreview,
                ),
                baseline_row(
                    "support_center_beta:action:test.bundle",
                    SupportCenterBetaLaneClass::SupportBundle,
                    SupportCenterLaunchActionClass::PreviewSupportBundle,
                ),
            ],
            claim_truth_rows: vec![SupportCenterClaimTruthRow {
                schema_version: SUPPORT_CENTER_BETA_SCHEMA_VERSION,
                record_kind: SUPPORT_CENTER_CLAIM_TRUTH_ROW_RECORD_KIND.into(),
                row_id: "support_center_beta:claim:test.safe_mode".into(),
                beta_lane_class: SupportCenterBetaLaneClass::SafeMode,
                claim_state_class: SupportCenterClaimStateClass::GreenClaimProven,
                scorecard_target: "m3.beta_lane.safe_mode".into(),
                evidence_packet_ref: "artifacts/support/m3/drill_harness_report.md".into(),
                summary: "Test claim row.".into(),
            }],
            degraded_truth_rows: vec![SupportCenterDegradedTruthRow {
                schema_version: SUPPORT_CENTER_BETA_SCHEMA_VERSION,
                record_kind: SUPPORT_CENTER_DEGRADED_TRUTH_ROW_RECORD_KIND.into(),
                row_id: "support_center_beta:degraded:test.none".into(),
                degraded_truth_class: SupportCenterDegradedTruthClass::NoneDegraded,
                active: false,
                exit_command_id: None,
                summary: "No degraded state active.".into(),
            }],
            export_route_rows: vec![SupportCenterExportRouteRow {
                schema_version: SUPPORT_CENTER_BETA_SCHEMA_VERSION,
                record_kind: SUPPORT_CENTER_EXPORT_ROUTE_ROW_RECORD_KIND.into(),
                row_id: "support_center_beta:export_route:test.preview".into(),
                launch_action_class: SupportCenterLaunchActionClass::PreviewSupportBundle,
                destination_class:
                    SupportCenterExportRouteDestinationClass::SupportBundlePreviewLocalFirst,
                destination_schema_ref: "schemas/support/support_bundle_manifest.schema.json"
                    .into(),
                local_first_path_named: true,
                upload_required_for_first_action: false,
                summary: "Local-first bundle preview.".into(),
            }],
            no_account_required_for_local_only: true,
            no_hidden_service_required_for_local_only: true,
            ia_capability_card_refs: vec!["support_center_card:project_doctor".into()],
            ia_route_refs: vec!["support_center_route:error_surface.project_doctor".into()],
            governance_bindings: SupportCenterGovernanceBindings {
                support_center_concept_ref: "docs/support/support_center_concept.md".into(),
                support_center_ia_ref:
                    "docs/support/support_center_information_architecture.md".into(),
                support_center_routes_ref: "artifacts/support/support_center_routes.yaml".into(),
                m3_scenario_corpus_ref: "docs/support/m3/support_scenario_corpus.md".into(),
                recovery_ladder_doc_ref: "docs/support/recovery_ladder_alpha.md".into(),
                safe_mode_doc_ref: "docs/support/m3/safe_mode_beta.md".into(),
                extension_bisect_doc_ref: "docs/support/m3/extension_bisect_beta.md".into(),
                repair_transaction_doc_ref: "docs/support/m3/repair_transaction_beta.md".into(),
                project_doctor_doc_ref: "docs/support/m3/project_doctor_beta.md".into(),
                support_bundle_contract_ref: "docs/support/support_bundle_contract.md".into(),
            },
            notes: None,
            emitted_at: "2026-05-16T00:00:00Z".into(),
        }
    }

    #[test]
    fn baseline_surface_passes_validation() {
        let surface = baseline_surface();
        let violations = SupportCenterBetaEvaluator::validate(&surface);
        assert!(violations.is_empty(), "{:?}", violations);
    }

    #[test]
    fn missing_required_action_is_refused() {
        let mut surface = baseline_surface();
        // Drop the safe-mode launch row.
        surface.launch_action_rows.retain(|r| {
            r.launch_action_class != SupportCenterLaunchActionClass::EnterSafeMode
        });
        let violations = SupportCenterBetaEvaluator::validate(&surface);
        assert!(violations
            .iter()
            .any(|v| v.check_id == "surface.required_launch_action_missing"));
    }

    #[test]
    fn upload_first_export_route_is_refused() {
        let mut surface = baseline_surface();
        surface.export_route_rows[0].upload_required_for_first_action = true;
        let violations = SupportCenterBetaEvaluator::validate(&surface);
        assert!(violations
            .iter()
            .any(|v| v.check_id == "export_route.upload_required_for_first_action"));
    }

    #[test]
    fn local_only_recovery_lane_with_hosted_service_is_refused() {
        let mut surface = baseline_surface();
        // Force safe-mode row to declare a hosted-service requirement.
        if let Some(row) = surface
            .launch_action_rows
            .iter_mut()
            .find(|r| r.launch_action_class == SupportCenterLaunchActionClass::EnterSafeMode)
        {
            row.service_dependency_class =
                SupportCenterServiceDependencyClass::RequiresHostedIntake;
        }
        let violations = SupportCenterBetaEvaluator::validate(&surface);
        assert!(violations
            .iter()
            .any(|v| v.check_id == "launch_action.local_only_lane_hosted_service"));
    }

    #[test]
    fn missing_user_authored_files_preservation_is_refused() {
        let mut surface = baseline_surface();
        if let Some(row) = surface
            .launch_action_rows
            .iter_mut()
            .find(|r| r.launch_action_class == SupportCenterLaunchActionClass::OpenProjectDoctor)
        {
            row.preserved_state_classes.clear();
            row.preserved_state_classes
                .push(SupportCenterPreservedStateClass::DurableWorkspaceIndexes);
        }
        let violations = SupportCenterBetaEvaluator::validate(&surface);
        assert!(violations
            .iter()
            .any(|v| v.check_id == "launch_action.preserved_state.user_authored_files_missing"));
    }

    #[test]
    fn support_packet_is_metadata_safe() {
        let surface = baseline_surface();
        let packet = surface.support_packet("packet:test", "2026-05-16T00:00:00Z");
        assert!(packet.raw_private_material_excluded);
        assert!(packet.ambient_authority_excluded);
        assert_eq!(packet.surface_ref, surface.surface_id);
        assert_eq!(packet.launch_action_row_refs.len(), 5);
    }
}
