//! One reusable M5 repair-action-card / repair-preview-row primitive: the repair class,
//! the target or scope, the changed-versus-unchanged state classes, the
//! local-or-remote-or-managed target boundary, the trust / policy requirement, and the
//! reversal class, projected the same way across every claimed M5 recovery surface.
//!
//! Aureline's frozen runtime-boundary component matrix
//! ([`crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix`])
//! names the repair action card as one governed component family and freezes its
//! controlled vocabulary — the repair blast radius, the reversibility class, and the host
//! boundary. This module *implements* that contract, plus the reusable repair-preview row
//! it needs, as one primitive so a user can always review what a Doctor or support fix
//! will change, what it will leave untouched, where it runs, and whether reversal is
//! exact, compensating, regenerate, or manual — before any mutation executes.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_repair_action`] — that takes one repair action's class,
//!    opaque target / scope, blast radius, host boundary, reversibility, trust
//!    requirement, changed-versus-unchanged state classes, and the four preview truths
//!    (`preview only`, `approval required`, `rerunnable`, and `factory reset out of
//!    band`), and produces one [`M5ResolvedRepairAction`] carrying the derived
//!    local-or-remote-or-managed target boundary, whether reversal is exact, whether the
//!    action requires approval, an honest non-generic action-label class (so a remote,
//!    policy-gated, or non-exact repair never reads like a generic `Fix now`), and the
//!    available preview / approval / apply / rollback / factory-reset / cancel actions.
//!    The resolver never understates the blast radius, never overstates reversibility,
//!    never masks the target boundary, and never hides which state classes change versus
//!    stay untouched.
//! 2. A parity matrix — [`M5RepairActionCardPrimitivePacket`] — that binds one row per
//!    claimed M5 recovery surface (the Project Doctor panel, the Doctor repair card, the
//!    guided repair wizard, the support-bundle repair row, the environment repair prompt,
//!    the toolchain repair card, the remote-host repair card, the repair preview sheet,
//!    and the activity-center repair entry) to the shared repair-action-card anatomy,
//!    repair-preview-row anatomy, the same repair classes, blast radii, target
//!    boundaries, reversibility classes, trust requirements, change classes, action-label
//!    classes, actions, export fields, and non-visual accessibility routes, so what a
//!    repair changes / leaves untouched / where it runs / how reversible it is stays
//!    identical on every surface and the support / export packet reconstructs the same
//!    repair explanation outside the live UI.
//!
//! The repair blast radius ([`M5RepairBlastRadius`]), reversibility class
//! ([`M5ReversibilityClass`]), host boundary ([`M5HostBoundaryClass`]), non-visual
//! accessibility routes ([`M5RuntimeBoundaryAccessibilityRoute`]), qualification classes
//! ([`M5RuntimeBoundaryQualificationClass`]), and downgrade triggers
//! ([`M5RuntimeBoundaryDowngradeTrigger`]) are reused verbatim from the frozen
//! runtime-boundary matrix; the shell topology — zones, responsive classes, window
//! classes, and consumer surfaces — is reused from the frozen shell-zone matrix. This
//! module mints new vocabulary only for what the frozen matrix left implicit about the
//! repair action card and the repair preview row themselves: their recovery surfaces,
//! their anatomy parts, their repair classes, their derived target boundaries, their
//! change classes, their trust requirements, their derived action-label classes, their
//! actions, and their export fields. No M5 surface invents a second repair grammar.
//!
//! Raw command lines, file paths, host names, tokens, and user text bodies stay outside
//! the support boundary; every repair title and target / scope is carried only as an
//! opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-repair-action-card.schema.json`](../../../../schemas/ui/m5-repair-action-card.schema.json)
//! and the contract doc is
//! [`docs/components/m5_repair_action_card_primitive_contract.md`](../../../../docs/components/m5_repair_action_card_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-repair-action-card-primitive/`](../../../../fixtures/ui/m5-repair-action-card-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_repair_action_card_primitive_packet,
    seeded_m5_repair_action_card_primitive_remote_host_repair_card_beta_narrowed,
    seeded_m5_repair_action_card_primitive_repair_preview_sheet_preview_narrowed,
    M5_REPAIR_ACTION_CARD_PRIMITIVE_PACKET_ID,
};

// The repair blast radius, reversibility class, host boundary, accessibility routes,
// qualification classes, and downgrade triggers are frozen once, in the runtime-boundary
// component matrix. This primitive reuses them verbatim so it never invents a parallel
// repair vocabulary.
pub use crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix::{
    M5HostBoundaryClass, M5RepairBlastRadius, M5ReversibilityClass,
    M5RuntimeBoundaryAccessibilityRoute, M5RuntimeBoundaryDowngradeTrigger,
    M5RuntimeBoundaryQualificationClass,
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

/// Stable record-kind tag carried by [`M5RepairActionCardPrimitivePacket`].
pub const M5_REPAIR_ACTION_CARD_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_repair_action_card_and_repair_preview_row_impact_scope_target_boundary_and_reversal_class_primitive";

/// Schema version for M5 repair-action-card primitive records.
pub const M5_REPAIR_ACTION_CARD_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the repair-action-card boundary schema (the packet schema).
pub const M5_REPAIR_ACTION_CARD_SCHEMA_REF: &str = "schemas/ui/m5-repair-action-card.schema.json";

/// Repo-relative path of the companion repair-preview-row component schema.
pub const M5_REPAIR_PREVIEW_ROW_SCHEMA_REF: &str = "schemas/ui/m5-repair-preview-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_REPAIR_ACTION_CARD_DOC_REF: &str =
    "docs/components/m5_repair_action_card_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds against.
pub const M5_REPAIR_ACTION_CARD_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen runtime-boundary component matrix this primitive
/// narrows from.
pub const M5_REPAIR_ACTION_CARD_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-runtime-boundary-components.schema.json";

/// Repo-relative path of the repair-transaction contract this primitive projects blast
/// radius and reversibility truth from.
pub const M5_REPAIR_ACTION_CARD_TRANSACTION_REF: &str =
    "schemas/support/repair_transaction.schema.json";

/// Repo-relative path of the repair-preview contract this primitive projects the preview
/// truths from.
pub const M5_REPAIR_ACTION_CARD_PREVIEW_CONTRACT_REF: &str =
    "schemas/support/repair_preview.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_REPAIR_ACTION_CARD_FIXTURE_DIR: &str = "fixtures/ui/m5-repair-action-card-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REPAIR_ACTION_CARD_ARTIFACT_REF: &str =
    "artifacts/release/m5-repair-action-card-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_REPAIR_ACTION_CARD_CSV_REF: &str =
    "artifacts/release/m5-repair-action-card-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_REPAIR_ACTION_CARD_REPORT_REF: &str =
    "artifacts/components/m5-repair-action-card-primitive.md";

/// One claimed M5 recovery surface that renders the shared repair action card and repair
/// preview row. These are the surfaces where a user reviews what a repair will change,
/// what it leaves untouched, where it runs, and how reversible it is — the Project Doctor
/// panel, the Doctor repair card, the guided repair wizard, the support-bundle repair
/// row, the environment repair prompt, the toolchain repair card, the remote-host repair
/// card, the repair preview sheet, and the activity-center repair entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairConsumerSurface {
    /// The Project Doctor panel.
    ProjectDoctorPanel,
    /// A single Doctor repair card.
    DoctorRepairCard,
    /// The guided repair wizard.
    GuidedRepairWizard,
    /// The support-bundle repair row.
    SupportBundleRepairRow,
    /// The environment repair prompt.
    EnvironmentRepairPrompt,
    /// The toolchain repair card.
    ToolchainRepairCard,
    /// The remote-host repair card.
    RemoteHostRepairCard,
    /// The repair preview sheet.
    RepairPreviewSheet,
    /// The activity-center repair entry.
    ActivityCenterRepair,
}

impl M5RepairConsumerSurface {
    /// Every claimed recovery surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProjectDoctorPanel,
        Self::DoctorRepairCard,
        Self::GuidedRepairWizard,
        Self::SupportBundleRepairRow,
        Self::EnvironmentRepairPrompt,
        Self::ToolchainRepairCard,
        Self::RemoteHostRepairCard,
        Self::RepairPreviewSheet,
        Self::ActivityCenterRepair,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDoctorPanel => "project_doctor_panel",
            Self::DoctorRepairCard => "doctor_repair_card",
            Self::GuidedRepairWizard => "guided_repair_wizard",
            Self::SupportBundleRepairRow => "support_bundle_repair_row",
            Self::EnvironmentRepairPrompt => "environment_repair_prompt",
            Self::ToolchainRepairCard => "toolchain_repair_card",
            Self::RemoteHostRepairCard => "remote_host_repair_card",
            Self::RepairPreviewSheet => "repair_preview_sheet",
            Self::ActivityCenterRepair => "activity_center_repair",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProjectDoctorPanel => "Project Doctor Panel",
            Self::DoctorRepairCard => "Doctor Repair Card",
            Self::GuidedRepairWizard => "Guided Repair Wizard",
            Self::SupportBundleRepairRow => "Support-Bundle Repair Row",
            Self::EnvironmentRepairPrompt => "Environment Repair Prompt",
            Self::ToolchainRepairCard => "Toolchain Repair Card",
            Self::RemoteHostRepairCard => "Remote-Host Repair Card",
            Self::RepairPreviewSheet => "Repair Preview Sheet",
            Self::ActivityCenterRepair => "Activity-Center Repair",
        }
    }
}

/// The class of repair a card offers, so a card never presents a destructive reinstall
/// and a read-only cache clear with the same generic affordance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairClass {
    /// Reinstall the toolchain.
    ReinstallToolchain,
    /// Repair the environment configuration.
    RepairEnvironmentConfig,
    /// Rebuild the workspace index.
    RebuildIndex,
    /// Clear cache artifacts.
    ClearCache,
    /// Repair file permissions.
    RepairPermissions,
    /// Regenerate a lockfile.
    RegenerateLockfile,
    /// Reconnect a remote target.
    ReconnectRemoteTarget,
    /// Factory-reset a component.
    FactoryResetComponent,
}

impl M5RepairClass {
    /// Every repair class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ReinstallToolchain,
        Self::RepairEnvironmentConfig,
        Self::RebuildIndex,
        Self::ClearCache,
        Self::RepairPermissions,
        Self::RegenerateLockfile,
        Self::ReconnectRemoteTarget,
        Self::FactoryResetComponent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReinstallToolchain => "reinstall_toolchain",
            Self::RepairEnvironmentConfig => "repair_environment_config",
            Self::RebuildIndex => "rebuild_index",
            Self::ClearCache => "clear_cache",
            Self::RepairPermissions => "repair_permissions",
            Self::RegenerateLockfile => "regenerate_lockfile",
            Self::ReconnectRemoteTarget => "reconnect_remote_target",
            Self::FactoryResetComponent => "factory_reset_component",
        }
    }
}

/// The local-or-remote-or-managed target boundary a repair runs against, derived from the
/// host boundary so a card never masks a repair reaching a remote or managed target as a
/// local one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairTargetBoundary {
    /// The repair runs on the local machine.
    LocalTarget,
    /// The repair runs on a remote host or VM.
    RemoteTarget,
    /// The repair runs on a managed / container / sandbox host.
    ManagedTarget,
}

impl M5RepairTargetBoundary {
    /// Every target boundary, in declaration order.
    pub const ALL: [Self; 3] = [Self::LocalTarget, Self::RemoteTarget, Self::ManagedTarget];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalTarget => "local_target",
            Self::RemoteTarget => "remote_target",
            Self::ManagedTarget => "managed_target",
        }
    }

    /// The target boundary a given host boundary resolves to.
    pub const fn from_host_boundary(host: M5HostBoundaryClass) -> Self {
        match host {
            M5HostBoundaryClass::LocalHost => Self::LocalTarget,
            M5HostBoundaryClass::RemoteSshHost | M5HostBoundaryClass::VirtualMachineHost => {
                Self::RemoteTarget
            }
            M5HostBoundaryClass::ContainerHost
            | M5HostBoundaryClass::ManagedWorkspaceHost
            | M5HostBoundaryClass::WasmSandboxHost => Self::ManagedTarget,
        }
    }

    /// True when the repair runs on the local machine.
    pub const fn is_local(self) -> bool {
        matches!(self, Self::LocalTarget)
    }
}

/// One class of state a repair either changes or leaves untouched, so a preview always
/// identifies both the changed-versus-unchanged classes and a user can judge risk
/// correctly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairChangeClass {
    /// Installed toolchain binaries.
    ToolchainBinaries,
    /// Workspace configuration.
    WorkspaceConfig,
    /// Cache artifacts.
    CacheArtifacts,
    /// Index data.
    IndexData,
    /// File permissions.
    FilePermissions,
    /// Lockfile state.
    LockfileState,
    /// Remote session state.
    RemoteSessionState,
    /// User source files.
    UserSourceFiles,
}

impl M5RepairChangeClass {
    /// Every change class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ToolchainBinaries,
        Self::WorkspaceConfig,
        Self::CacheArtifacts,
        Self::IndexData,
        Self::FilePermissions,
        Self::LockfileState,
        Self::RemoteSessionState,
        Self::UserSourceFiles,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ToolchainBinaries => "toolchain_binaries",
            Self::WorkspaceConfig => "workspace_config",
            Self::CacheArtifacts => "cache_artifacts",
            Self::IndexData => "index_data",
            Self::FilePermissions => "file_permissions",
            Self::LockfileState => "lockfile_state",
            Self::RemoteSessionState => "remote_session_state",
            Self::UserSourceFiles => "user_source_files",
        }
    }
}

/// The trust / policy requirement a repair carries, so a policy-gated or admin-managed
/// fix never reads like an ordinary local one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairTrustRequirement {
    /// No elevation required.
    NoElevation,
    /// A local confirmation is required.
    LocalConfirmation,
    /// Local administrator / elevation is required.
    AdminElevation,
    /// Policy approval is required before the repair runs.
    PolicyApprovalRequired,
    /// The repair is managed by an administrator and cannot be self-applied.
    ManagedByAdministrator,
}

impl M5RepairTrustRequirement {
    /// Every trust requirement, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoElevation,
        Self::LocalConfirmation,
        Self::AdminElevation,
        Self::PolicyApprovalRequired,
        Self::ManagedByAdministrator,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoElevation => "no_elevation",
            Self::LocalConfirmation => "local_confirmation",
            Self::AdminElevation => "admin_elevation",
            Self::PolicyApprovalRequired => "policy_approval_required",
            Self::ManagedByAdministrator => "managed_by_administrator",
        }
    }

    /// True when this requirement means the repair needs off-device / policy approval
    /// before it can run.
    pub const fn requires_approval(self) -> bool {
        matches!(
            self,
            Self::PolicyApprovalRequired | Self::ManagedByAdministrator
        )
    }
}

/// The derived, honest class of the primary action label a repair card shows, so a
/// remote, policy-gated, non-exact, preview-only, or factory-reset repair never reads
/// like a generic `Fix now`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairActionLabelClass {
    /// A local, exact-reversible fix with no approval gate: an ordinary apply.
    ApplyLocalReversible,
    /// A preview-only row that will not execute a mutation.
    PreviewOnly,
    /// A policy-gated repair that must request approval first.
    RequestPolicyApproval,
    /// A repair that runs off-device (remote or managed) and must be reviewed there.
    ReviewOffDeviceRepair,
    /// A repair whose reversal is not exact.
    ApplyNonExactRepair,
    /// A factory reset that is performed out of band.
    OpenFactoryResetOutOfBand,
}

impl M5RepairActionLabelClass {
    /// Every action-label class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ApplyLocalReversible,
        Self::PreviewOnly,
        Self::RequestPolicyApproval,
        Self::ReviewOffDeviceRepair,
        Self::ApplyNonExactRepair,
        Self::OpenFactoryResetOutOfBand,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApplyLocalReversible => "apply_local_reversible",
            Self::PreviewOnly => "preview_only",
            Self::RequestPolicyApproval => "request_policy_approval",
            Self::ReviewOffDeviceRepair => "review_off_device_repair",
            Self::ApplyNonExactRepair => "apply_non_exact_repair",
            Self::OpenFactoryResetOutOfBand => "open_factory_reset_out_of_band",
        }
    }

    /// Review-safe label text for the primary action.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ApplyLocalReversible => "Apply fix",
            Self::PreviewOnly => "Preview repair",
            Self::RequestPolicyApproval => "Request approval",
            Self::ReviewOffDeviceRepair => "Review off-device repair",
            Self::ApplyNonExactRepair => "Apply non-exact repair",
            Self::OpenFactoryResetOutOfBand => "Open factory reset",
        }
    }

    /// True when this label is explicit about a remote, policy-gated, non-exact,
    /// preview-only, or factory-reset repair — i.e. anything other than the ordinary
    /// local apply. When this is true, the card never reads like a generic `Fix now`.
    pub const fn is_explicit(self) -> bool {
        !matches!(self, Self::ApplyLocalReversible)
    }
}

/// One anatomy part the shared repair action card surfaces. The parts in
/// [`M5RepairActionCardPart::MANDATORY`] are required on every card so a user can read
/// the repair class, its blast radius, its target boundary, its reversal class, and an
/// honest action label without inferring any of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairActionCardPart {
    /// The repair class label.
    RepairClassLabel,
    /// The target / scope label.
    TargetScopeLabel,
    /// The blast-radius badge.
    BlastRadiusBadge,
    /// The target-boundary badge.
    TargetBoundaryBadge,
    /// The trust / policy requirement badge.
    TrustRequirementBadge,
    /// The reversal-class badge.
    ReversalClassBadge,
    /// The honest primary action label.
    ActionLabel,
}

impl M5RepairActionCardPart {
    /// Every card part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::RepairClassLabel,
        Self::TargetScopeLabel,
        Self::BlastRadiusBadge,
        Self::TargetBoundaryBadge,
        Self::TrustRequirementBadge,
        Self::ReversalClassBadge,
        Self::ActionLabel,
    ];

    /// The card parts every repair action card must render.
    pub const MANDATORY: [Self; 5] = [
        Self::RepairClassLabel,
        Self::BlastRadiusBadge,
        Self::TargetBoundaryBadge,
        Self::ReversalClassBadge,
        Self::ActionLabel,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepairClassLabel => "repair_class_label",
            Self::TargetScopeLabel => "target_scope_label",
            Self::BlastRadiusBadge => "blast_radius_badge",
            Self::TargetBoundaryBadge => "target_boundary_badge",
            Self::TrustRequirementBadge => "trust_requirement_badge",
            Self::ReversalClassBadge => "reversal_class_badge",
            Self::ActionLabel => "action_label",
        }
    }
}

/// One anatomy part the shared repair preview row surfaces. Every part is mandatory so a
/// preview never drops one of the four pre-execution truths or either change-class list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairPreviewRowPart {
    /// The `preview only` flag.
    PreviewOnlyFlag,
    /// The `approval required` flag.
    ApprovalRequiredFlag,
    /// The `rerunnable or not` flag.
    RerunnableFlag,
    /// The `factory reset out of band` flag.
    FactoryResetOutOfBandFlag,
    /// The changed-state-class list.
    ChangedClassList,
    /// The unchanged-state-class list.
    UnchangedClassList,
}

impl M5RepairPreviewRowPart {
    /// Every preview-row part, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PreviewOnlyFlag,
        Self::ApprovalRequiredFlag,
        Self::RerunnableFlag,
        Self::FactoryResetOutOfBandFlag,
        Self::ChangedClassList,
        Self::UnchangedClassList,
    ];

    /// The preview-row parts every repair preview row must render (all of them).
    pub const MANDATORY: [Self; 6] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewOnlyFlag => "preview_only_flag",
            Self::ApprovalRequiredFlag => "approval_required_flag",
            Self::RerunnableFlag => "rerunnable_flag",
            Self::FactoryResetOutOfBandFlag => "factory_reset_out_of_band_flag",
            Self::ChangedClassList => "changed_class_list",
            Self::UnchangedClassList => "unchanged_class_list",
        }
    }
}

/// One action a repair card can offer, so previewing, approving, applying, rolling back,
/// factory-resetting, and cancelling are never conflated behind a single generic button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairAction {
    /// Preview the repair's blast radius and reversibility before it runs.
    PreviewRepair,
    /// Request approval for a policy-gated / managed repair.
    RequestApproval,
    /// Apply the repair.
    ApplyRepair,
    /// Roll back an applied repair.
    RollbackRepair,
    /// Open the out-of-band factory reset.
    OpenFactoryReset,
    /// Cancel the repair.
    CancelRepair,
}

impl M5RepairAction {
    /// Every repair action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PreviewRepair,
        Self::RequestApproval,
        Self::ApplyRepair,
        Self::RollbackRepair,
        Self::OpenFactoryReset,
        Self::CancelRepair,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewRepair => "preview_repair",
            Self::RequestApproval => "request_approval",
            Self::ApplyRepair => "apply_repair",
            Self::RollbackRepair => "rollback_repair",
            Self::OpenFactoryReset => "open_factory_reset",
            Self::CancelRepair => "cancel_repair",
        }
    }
}

/// A field the support / export packet carries so the same repair can be explained
/// outside the live UI. The fields in [`M5RepairExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairExportField {
    /// The opaque repair identity / title.
    RepairIdentity,
    /// The repair class.
    RepairClass,
    /// The opaque target / scope.
    TargetScope,
    /// The blast radius.
    BlastRadius,
    /// The target boundary.
    TargetBoundary,
    /// The reversal / reversibility class.
    ReversalClass,
    /// The trust / policy requirement.
    TrustRequirement,
    /// The changed state classes.
    ChangedClasses,
    /// The unchanged state classes.
    UnchangedClasses,
    /// The four preview truths.
    PreviewFlags,
    /// The available actions.
    AvailableActions,
}

impl M5RepairExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::RepairIdentity,
        Self::RepairClass,
        Self::TargetScope,
        Self::BlastRadius,
        Self::TargetBoundary,
        Self::ReversalClass,
        Self::TrustRequirement,
        Self::ChangedClasses,
        Self::UnchangedClasses,
        Self::PreviewFlags,
        Self::AvailableActions,
    ];

    /// The export fields every repair export must carry.
    pub const MANDATORY: [Self; 7] = [
        Self::RepairIdentity,
        Self::RepairClass,
        Self::BlastRadius,
        Self::TargetBoundary,
        Self::ReversalClass,
        Self::ChangedClasses,
        Self::UnchangedClasses,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepairIdentity => "repair_identity",
            Self::RepairClass => "repair_class",
            Self::TargetScope => "target_scope",
            Self::BlastRadius => "blast_radius",
            Self::TargetBoundary => "target_boundary",
            Self::ReversalClass => "reversal_class",
            Self::TrustRequirement => "trust_requirement",
            Self::ChangedClasses => "changed_classes",
            Self::UnchangedClasses => "unchanged_classes",
            Self::PreviewFlags => "preview_flags",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// True when the blast radius involves writes — anything other than a no-writes preview.
const fn blast_radius_is_write(blast_radius: M5RepairBlastRadius) -> bool {
    !matches!(blast_radius, M5RepairBlastRadius::NoWritesPreview)
}

/// True when the reversibility class is an exact rollback (a checkpoint), so a card never
/// overstates a backup, partial, irreversible, or manual reversal as exact.
const fn reversibility_is_exact(reversibility: M5ReversibilityClass) -> bool {
    matches!(
        reversibility,
        M5ReversibilityClass::FullyReversibleCheckpoint
    )
}

/// True when the reversibility class still offers some automatic reversal path (a
/// rollback action applies).
const fn reversibility_is_reversible(reversibility: M5ReversibilityClass) -> bool {
    matches!(
        reversibility,
        M5ReversibilityClass::FullyReversibleCheckpoint
            | M5ReversibilityClass::ReversibleWithBackup
            | M5ReversibilityClass::PartiallyReversible
    )
}

/// The full input to the repair-action resolver for one recovery surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairActionResolutionInput {
    /// The opaque, export-safe repair title.
    pub repair_title: String,
    /// The repair class.
    pub repair_class: M5RepairClass,
    /// The opaque, export-safe target / scope representation.
    pub target_scope_repr: String,
    /// The repair blast radius.
    pub blast_radius: M5RepairBlastRadius,
    /// The host boundary the repair runs against.
    pub host_boundary: M5HostBoundaryClass,
    /// The reversibility class.
    pub reversibility: M5ReversibilityClass,
    /// The trust / policy requirement.
    pub trust_requirement: M5RepairTrustRequirement,
    /// The state classes this repair changes.
    pub changed_classes: Vec<M5RepairChangeClass>,
    /// The state classes this repair leaves untouched.
    pub unchanged_classes: Vec<M5RepairChangeClass>,
    /// Whether this row is preview-only (will not execute a mutation).
    pub preview_only: bool,
    /// Whether this row's fix explicitly requires approval before executing.
    pub approval_required: bool,
    /// Whether this repair is rerunnable.
    pub rerunnable: bool,
    /// Whether the reset is performed out of band as a factory reset.
    pub factory_reset_out_of_band: bool,
}

/// The resolved repair action for one recovery surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRepairAction {
    /// The opaque repair title.
    pub repair_title: String,
    /// The repair class.
    pub repair_class: M5RepairClass,
    /// The opaque target / scope.
    pub target_scope_repr: String,
    /// The repair blast radius.
    pub blast_radius: M5RepairBlastRadius,
    /// The host boundary the repair runs against.
    pub host_boundary: M5HostBoundaryClass,
    /// The derived local-or-remote-or-managed target boundary.
    pub target_boundary: M5RepairTargetBoundary,
    /// The reversibility class.
    pub reversibility: M5ReversibilityClass,
    /// True when the reversal is an exact checkpoint rollback.
    pub reversal_is_exact: bool,
    /// The trust / policy requirement.
    pub trust_requirement: M5RepairTrustRequirement,
    /// True when the repair needs off-device / policy approval (explicit flag or trust
    /// requirement).
    pub requires_approval: bool,
    /// The state classes this repair changes.
    pub changed_classes: Vec<M5RepairChangeClass>,
    /// The state classes this repair leaves untouched.
    pub unchanged_classes: Vec<M5RepairChangeClass>,
    /// Whether this row is preview-only.
    pub preview_only: bool,
    /// Whether the fix explicitly requires approval before executing.
    pub approval_required: bool,
    /// Whether this repair is rerunnable.
    pub rerunnable: bool,
    /// Whether the reset is performed out of band as a factory reset.
    pub factory_reset_out_of_band: bool,
    /// The derived honest primary action-label class.
    pub action_label_class: M5RepairActionLabelClass,
    /// The actions this surface exposes.
    pub available_actions: Vec<M5RepairAction>,
    /// True when the blast radius and reversibility can be reviewed before any mutation
    /// runs (a preview action is always available). Always `true`.
    pub blast_radius_reviewable: bool,
    /// True when both a changed-class list and an unchanged-class list are disclosed.
    pub changed_and_unchanged_disclosed: bool,
}

/// Errors returned by [`resolve_repair_action`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RepairActionResolutionError {
    /// The repair title was empty.
    EmptyRepairTitle,
    /// The target / scope was empty.
    EmptyTargetScope,
    /// A representation carried forbidden material.
    ForbiddenRepairMaterial,
    /// The same change class was listed twice within one list.
    DuplicateChangeClass,
    /// A change class appeared in both the changed and the unchanged list.
    OverlappingChangeClasses,
    /// A no-writes preview claimed changed state classes.
    PreviewBlastRadiusClaimsChanges,
}

impl M5RepairActionResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyRepairTitle => "empty_repair_title",
            Self::EmptyTargetScope => "empty_target_scope",
            Self::ForbiddenRepairMaterial => "forbidden_repair_material",
            Self::DuplicateChangeClass => "duplicate_change_class",
            Self::OverlappingChangeClasses => "overlapping_change_classes",
            Self::PreviewBlastRadiusClaimsChanges => "preview_blast_radius_claims_changes",
        }
    }
}

impl fmt::Display for M5RepairActionResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "repair-action resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RepairActionResolutionError {}

/// Resolves one recovery surface's repair action card and repair preview row from its
/// repair class, blast radius, host boundary, reversibility, trust requirement,
/// changed-versus-unchanged state classes, and the four preview truths.
///
/// The target boundary is derived from the host boundary so a repair reaching a remote or
/// managed target is never masked as local. The action-label class is derived so a
/// preview-only, factory-reset, policy-gated, off-device, or non-exact repair never reads
/// like a generic `Fix now`. A preview action is always available so the blast radius and
/// reversibility can be reviewed before any mutation runs, and a rollback action stays
/// attached while some automatic reversal path exists.
pub fn resolve_repair_action(
    input: &M5RepairActionResolutionInput,
) -> Result<M5ResolvedRepairAction, M5RepairActionResolutionError> {
    if input.repair_title.trim().is_empty() {
        return Err(M5RepairActionResolutionError::EmptyRepairTitle);
    }
    if input.target_scope_repr.trim().is_empty() {
        return Err(M5RepairActionResolutionError::EmptyTargetScope);
    }
    if value_repr_is_forbidden(&input.repair_title)
        || value_repr_is_forbidden(&input.target_scope_repr)
    {
        return Err(M5RepairActionResolutionError::ForbiddenRepairMaterial);
    }

    let mut changed_set: BTreeSet<M5RepairChangeClass> = BTreeSet::new();
    for class in &input.changed_classes {
        if !changed_set.insert(*class) {
            return Err(M5RepairActionResolutionError::DuplicateChangeClass);
        }
    }
    let mut unchanged_set: BTreeSet<M5RepairChangeClass> = BTreeSet::new();
    for class in &input.unchanged_classes {
        if !unchanged_set.insert(*class) {
            return Err(M5RepairActionResolutionError::DuplicateChangeClass);
        }
        if changed_set.contains(class) {
            return Err(M5RepairActionResolutionError::OverlappingChangeClasses);
        }
    }

    // A no-writes preview writes nothing, so it must not claim any changed classes.
    if !blast_radius_is_write(input.blast_radius) && !input.changed_classes.is_empty() {
        return Err(M5RepairActionResolutionError::PreviewBlastRadiusClaimsChanges);
    }

    let target_boundary = M5RepairTargetBoundary::from_host_boundary(input.host_boundary);
    let reversal_is_exact = reversibility_is_exact(input.reversibility);
    let requires_approval = input.approval_required || input.trust_requirement.requires_approval();

    let action_label_class = if input.factory_reset_out_of_band {
        M5RepairActionLabelClass::OpenFactoryResetOutOfBand
    } else if input.preview_only {
        M5RepairActionLabelClass::PreviewOnly
    } else if requires_approval {
        M5RepairActionLabelClass::RequestPolicyApproval
    } else if !target_boundary.is_local() {
        M5RepairActionLabelClass::ReviewOffDeviceRepair
    } else if !reversal_is_exact {
        M5RepairActionLabelClass::ApplyNonExactRepair
    } else {
        M5RepairActionLabelClass::ApplyLocalReversible
    };

    // Actions are pushed in `M5RepairAction::ALL` order so the resolved order is stable.
    let mut available_actions = vec![M5RepairAction::PreviewRepair];
    if requires_approval && !input.preview_only {
        available_actions.push(M5RepairAction::RequestApproval);
    }
    if !input.preview_only && !input.factory_reset_out_of_band && !requires_approval {
        available_actions.push(M5RepairAction::ApplyRepair);
    }
    if reversibility_is_reversible(input.reversibility) {
        available_actions.push(M5RepairAction::RollbackRepair);
    }
    if input.factory_reset_out_of_band {
        available_actions.push(M5RepairAction::OpenFactoryReset);
    }
    available_actions.push(M5RepairAction::CancelRepair);

    let changed_and_unchanged_disclosed =
        !input.changed_classes.is_empty() && !input.unchanged_classes.is_empty();

    Ok(M5ResolvedRepairAction {
        repair_title: input.repair_title.clone(),
        repair_class: input.repair_class,
        target_scope_repr: input.target_scope_repr.clone(),
        blast_radius: input.blast_radius,
        host_boundary: input.host_boundary,
        target_boundary,
        reversibility: input.reversibility,
        reversal_is_exact,
        trust_requirement: input.trust_requirement,
        requires_approval,
        changed_classes: input.changed_classes.clone(),
        unchanged_classes: input.unchanged_classes.clone(),
        preview_only: input.preview_only,
        approval_required: input.approval_required,
        rerunnable: input.rerunnable,
        factory_reset_out_of_band: input.factory_reset_out_of_band,
        action_label_class,
        available_actions,
        blast_radius_reviewable: true,
        changed_and_unchanged_disclosed,
    })
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs the repair explanation from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairActionResolutionCase {
    /// The resolver input.
    pub input: M5RepairActionResolutionInput,
    /// The resolved truth. Must equal `resolve_repair_action(&input)`.
    pub resolved: M5ResolvedRepairAction,
}

impl M5RepairActionResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5RepairActionResolutionInput) -> Self {
        let resolved = resolve_repair_action(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_repair_action(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one recovery surface bound to the shared repair
/// action card / preview row anatomy, repair classes, blast radii, target boundaries,
/// reversibility classes, trust requirements, change classes, action-label classes,
/// actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairConsumerRow {
    /// Recovery surface family.
    pub consumer_surface: M5RepairConsumerSurface,
    /// Qualification class earned by this surface.
    pub qualification: M5RuntimeBoundaryQualificationClass,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this card / row attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this component must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this component keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Repair-action-card parts this surface renders (must include the mandatory parts).
    pub card_parts: Vec<M5RepairActionCardPart>,
    /// Repair-preview-row parts this surface renders (must include the mandatory parts).
    pub preview_row_parts: Vec<M5RepairPreviewRowPart>,
    /// Repair classes this surface distinguishes.
    pub repair_classes: Vec<M5RepairClass>,
    /// Blast radii this surface distinguishes.
    pub blast_radii: Vec<M5RepairBlastRadius>,
    /// Target boundaries this surface distinguishes.
    pub target_boundaries: Vec<M5RepairTargetBoundary>,
    /// Reversibility classes this surface distinguishes.
    pub reversibility_classes: Vec<M5ReversibilityClass>,
    /// Trust requirements this surface distinguishes.
    pub trust_requirements: Vec<M5RepairTrustRequirement>,
    /// Change classes this surface distinguishes.
    pub change_classes: Vec<M5RepairChangeClass>,
    /// Action-label classes this surface distinguishes.
    pub action_label_classes: Vec<M5RepairActionLabelClass>,
    /// Repair actions this surface offers.
    pub repair_actions: Vec<M5RepairAction>,
    /// Export fields this surface carries (must include the mandatory fields).
    pub export_fields: Vec<M5RepairExportField>,
    /// Non-visual accessibility routes this surface offers.
    pub accessibility_routes: Vec<M5RuntimeBoundaryAccessibilityRoute>,
    /// Shell subsystems that consume this surface's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5RuntimeBoundaryDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface.
    pub example_resolutions: Vec<M5RepairActionResolutionCase>,
    /// Hard invariant: this surface never understates the blast radius. MUST be `false`.
    pub understates_blast_radius: bool,
    /// Hard invariant: this surface never overstates reversibility. MUST be `false`.
    pub overstates_reversibility: bool,
    /// Hard invariant: this surface never masks the target boundary. MUST be `false`.
    pub masks_target_boundary: bool,
    /// Hard invariant: this surface never hides which state classes change versus stay
    /// untouched. MUST be `false`.
    pub hides_changed_or_unchanged_classes: bool,
}

impl M5RepairConsumerRow {
    /// True when the row declares every mandatory card part.
    fn declares_mandatory_card_parts(&self) -> bool {
        let present: BTreeSet<M5RepairActionCardPart> = self.card_parts.iter().copied().collect();
        M5RepairActionCardPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory preview-row part.
    fn declares_mandatory_preview_row_parts(&self) -> bool {
        let present: BTreeSet<M5RepairPreviewRowPart> =
            self.preview_row_parts.iter().copied().collect();
        M5RepairPreviewRowPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5RepairExportField> = self.export_fields.iter().copied().collect();
        M5RepairExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.understates_blast_radius
            && !self.overstates_reversibility
            && !self.masks_target_boundary
            && !self.hides_changed_or_unchanged_classes
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairVocabularySet {
    /// Recovery-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Repair-class tokens.
    pub repair_classes: Vec<String>,
    /// Card-part tokens.
    pub card_parts: Vec<String>,
    /// Preview-row-part tokens.
    pub preview_row_parts: Vec<String>,
    /// Target-boundary tokens.
    pub target_boundaries: Vec<String>,
    /// Change-class tokens.
    pub change_classes: Vec<String>,
    /// Trust-requirement tokens.
    pub trust_requirements: Vec<String>,
    /// Action-label-class tokens.
    pub action_label_classes: Vec<String>,
    /// Repair-action tokens.
    pub repair_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Blast-radius tokens (reused from the frozen matrix).
    pub blast_radii: Vec<String>,
    /// Reversibility-class tokens (reused from the frozen matrix).
    pub reversibility_classes: Vec<String>,
    /// Host-boundary tokens (reused from the frozen matrix).
    pub host_boundaries: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5RepairVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5RepairConsumerSurface::ALL, |v| v.as_str()),
            repair_classes: tokens(&M5RepairClass::ALL, |v| v.as_str()),
            card_parts: tokens(&M5RepairActionCardPart::ALL, |v| v.as_str()),
            preview_row_parts: tokens(&M5RepairPreviewRowPart::ALL, |v| v.as_str()),
            target_boundaries: tokens(&M5RepairTargetBoundary::ALL, |v| v.as_str()),
            change_classes: tokens(&M5RepairChangeClass::ALL, |v| v.as_str()),
            trust_requirements: tokens(&M5RepairTrustRequirement::ALL, |v| v.as_str()),
            action_label_classes: tokens(&M5RepairActionLabelClass::ALL, |v| v.as_str()),
            repair_actions: tokens(&M5RepairAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RepairExportField::ALL, |v| v.as_str()),
            blast_radii: tokens(&M5RepairBlastRadius::ALL, |v| v.as_str()),
            reversibility_classes: tokens(&M5ReversibilityClass::ALL, |v| v.as_str()),
            host_boundaries: tokens(&M5HostBoundaryClass::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5RuntimeBoundaryAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5RepairGovernanceReview {
    /// One primitive carries repair class, scope, boundary, and reversal truth on every
    /// surface.
    pub one_primitive_carries_repair_truth: bool,
    /// Blast radius and reversibility are reviewable before any mutation runs.
    pub blast_radius_and_reversibility_reviewable_before_mutation: bool,
    /// Changed and unchanged state classes are both identified.
    pub changed_and_unchanged_classes_both_identified: bool,
    /// The blast radius is never understated.
    pub blast_radius_never_understated: bool,
    /// Reversibility is never overstated.
    pub reversibility_never_overstated: bool,
    /// The local-or-remote-or-managed target boundary is never masked.
    pub target_boundary_never_masked: bool,
    /// Remote, policy-gated, and non-exact repairs never read like generic buttons.
    pub non_generic_action_labels_for_gated_or_non_exact_repairs: bool,
    /// The four preview truths survive into the support / export packet.
    pub preview_and_reversal_vocabulary_in_support_export: bool,
    /// No surface invents a second repair grammar.
    pub no_surface_invents_second_repair_grammar: bool,
    /// Every row is bound to a canonical shell zone.
    pub every_row_bound_to_shell_zone: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel repair vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairConsumerProjection {
    /// Doctor, guided-repair, support, environment, toolchain, remote, preview, and
    /// activity surfaces all consume the shared primitive.
    pub recovery_surfaces_consume_shared_primitive: bool,
    /// The repair resolver reads a single canonical repair-transaction source.
    pub repair_resolver_reads_single_transaction_source: bool,
    /// The preview rows read a single canonical preview source.
    pub preview_rows_read_single_preview_source: bool,
    /// The target boundary reads a single canonical host-boundary source.
    pub target_boundary_reads_single_host_source: bool,
    /// Support / export reads a single canonical repair source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting repair audit.
    pub repair_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RepairActionCardPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RepairActionCardPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5RepairConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RepairVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RepairGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RepairConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RepairProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RepairReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 repair-action-card primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairActionCardPrimitivePacket {
    /// Record kind; must equal [`M5_REPAIR_ACTION_CARD_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REPAIR_ACTION_CARD_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5RepairConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RepairVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RepairGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RepairConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RepairProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RepairReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RepairActionCardPrimitivePacket {
    /// Builds an M5 repair-action-card primitive packet from stable-lane input.
    pub fn new(input: M5RepairActionCardPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_REPAIR_ACTION_CARD_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_REPAIR_ACTION_CARD_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
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

    /// Validates the M5 repair-action-card primitive invariants.
    pub fn validate(&self) -> Vec<M5RepairActionCardPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REPAIR_ACTION_CARD_PRIMITIVE_RECORD_KIND {
            violations.push(M5RepairActionCardPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REPAIR_ACTION_CARD_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5RepairActionCardPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RepairActionCardPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_blast_radius_review_covered(self, &mut violations);
        validate_non_generic_label_covered(self, &mut violations);
        validate_changed_and_unchanged_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 repair-action-card primitive packet serializes"),
        ) {
            violations.push(M5RepairActionCardPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 repair-action-card primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per recovery surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,shell_zone_slot,card_parts,preview_row_parts,repair_classes,blast_radii,target_boundaries,reversibility_classes,trust_requirements,change_classes,action_label_classes,repair_actions,export_fields,example_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.card_parts, |v| v.as_str()),
                join_tokens(&row.preview_row_parts, |v| v.as_str()),
                join_tokens(&row.repair_classes, |v| v.as_str()),
                join_tokens(&row.blast_radii, |v| v.as_str()),
                join_tokens(&row.target_boundaries, |v| v.as_str()),
                join_tokens(&row.reversibility_classes, |v| v.as_str()),
                join_tokens(&row.trust_requirements, |v| v.as_str()),
                join_tokens(&row.change_classes, |v| v.as_str()),
                join_tokens(&row.action_label_classes, |v| v.as_str()),
                join_tokens(&row.repair_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Repair Action Card and Repair Preview Row Primitive: Impact Scope, Target Boundary, and Reversal-Class Honesty\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Recovery surfaces: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Repair classes: {}\n",
            self.vocabulary_set.repair_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Blast radii: {}\n",
            self.vocabulary_set.blast_radii.join(", ")
        ));
        out.push_str(&format!(
            "- Target boundaries: {}\n",
            self.vocabulary_set.target_boundaries.join(", ")
        ));
        out.push_str(&format!(
            "- Reversibility classes: {}\n",
            self.vocabulary_set.reversibility_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Action-label classes: {}\n",
            self.vocabulary_set.action_label_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Recovery surfaces\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                out.push_str(&format!(
                    "    - `{}` → class `{}`, blast `{}`, boundary `{}`, reversal `{}`, label `{}`, changed {} / unchanged {}\n",
                    case.resolved.repair_title,
                    case.resolved.repair_class.as_str(),
                    case.resolved.blast_radius.as_str(),
                    case.resolved.target_boundary.as_str(),
                    case.resolved.reversibility.as_str(),
                    case.resolved.action_label_class.as_str(),
                    case.resolved.changed_classes.len(),
                    case.resolved.unchanged_classes.len(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 repair-action-card export.
#[derive(Debug)]
pub enum M5RepairActionCardPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RepairActionCardPrimitiveViolation>),
}

impl fmt::Display for M5RepairActionCardPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 repair-action-card primitive export parse failed: {error}"
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
                    "m5 repair-action-card primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RepairActionCardPrimitiveArtifactError {}

/// Validation failures emitted by [`M5RepairActionCardPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RepairActionCardPrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required recovery-surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory card parts.
    MandatoryCardPartMissing,
    /// A consumer row omits one of the mandatory preview-row parts.
    MandatoryPreviewRowPartMissing,
    /// A consumer row declares no repair classes.
    RepairClassMissing,
    /// A consumer row declares no blast radii.
    BlastRadiusMissing,
    /// A consumer row declares no target boundaries.
    TargetBoundaryMissing,
    /// A consumer row declares no reversibility classes.
    ReversibilityClassMissing,
    /// A consumer row declares no trust requirements.
    TrustRequirementMissing,
    /// A consumer row declares no change classes.
    ChangeClassMissing,
    /// A consumer row declares no action-label classes.
    ActionLabelClassMissing,
    /// A consumer row declares no repair actions.
    RepairActionMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A surface claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// No worked resolution proves the blast radius and reversibility reviewable before a
    /// mutation runs.
    BlastRadiusReviewUnproven,
    /// No worked resolution proves a non-generic action label for a gated / off-device /
    /// non-exact repair.
    NonGenericLabelUnproven,
    /// No worked resolution proves both a changed-class and an unchanged-class list.
    ChangedUnchangedDisclosureUnproven,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
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

impl M5RepairActionCardPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryCardPartMissing => "mandatory_card_part_missing",
            Self::MandatoryPreviewRowPartMissing => "mandatory_preview_row_part_missing",
            Self::RepairClassMissing => "repair_class_missing",
            Self::BlastRadiusMissing => "blast_radius_missing",
            Self::TargetBoundaryMissing => "target_boundary_missing",
            Self::ReversibilityClassMissing => "reversibility_class_missing",
            Self::TrustRequirementMissing => "trust_requirement_missing",
            Self::ChangeClassMissing => "change_class_missing",
            Self::ActionLabelClassMissing => "action_label_class_missing",
            Self::RepairActionMissing => "repair_action_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::BlastRadiusReviewUnproven => "blast_radius_review_unproven",
            Self::NonGenericLabelUnproven => "non_generic_label_unproven",
            Self::ChangedUnchangedDisclosureUnproven => "changed_unchanged_disclosure_unproven",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 repair-action-card export.
pub fn current_stable_m5_repair_action_card_primitive_export(
) -> Result<M5RepairActionCardPrimitivePacket, M5RepairActionCardPrimitiveArtifactError> {
    let packet: M5RepairActionCardPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-repair-action-card-proof/support_export.json"
    )))
    .map_err(M5RepairActionCardPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RepairActionCardPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5RepairActionCardPrimitivePacket,
    violations: &mut Vec<M5RepairActionCardPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_REPAIR_ACTION_CARD_SCHEMA_REF,
        M5_REPAIR_PREVIEW_ROW_SCHEMA_REF,
        M5_REPAIR_ACTION_CARD_DOC_REF,
        M5_REPAIR_ACTION_CARD_SHELL_ZONE_REF,
        M5_REPAIR_ACTION_CARD_COMPONENT_MATRIX_REF,
        M5_REPAIR_ACTION_CARD_TRANSACTION_REF,
        M5_REPAIR_ACTION_CARD_PREVIEW_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RepairActionCardPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5RepairActionCardPrimitivePacket,
    violations: &mut Vec<M5RepairActionCardPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5RepairActionCardPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5RepairActionCardPrimitivePacket,
    violations: &mut Vec<M5RepairActionCardPrimitiveViolation>,
) {
    let present: BTreeSet<M5RepairConsumerSurface> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5RepairConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5RepairActionCardPrimitiveViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.card_parts.is_empty()
            || row.preview_row_parts.is_empty()
        {
            violations.push(M5RepairActionCardPrimitiveViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_card_parts() {
            violations.push(M5RepairActionCardPrimitiveViolation::MandatoryCardPartMissing);
        }
        if !row.declares_mandatory_preview_row_parts() {
            violations.push(M5RepairActionCardPrimitiveViolation::MandatoryPreviewRowPartMissing);
        }
        if row.repair_classes.is_empty() {
            violations.push(M5RepairActionCardPrimitiveViolation::RepairClassMissing);
        }
        if row.blast_radii.is_empty() {
            violations.push(M5RepairActionCardPrimitiveViolation::BlastRadiusMissing);
        }
        if row.target_boundaries.is_empty() {
            violations.push(M5RepairActionCardPrimitiveViolation::TargetBoundaryMissing);
        }
        if row.reversibility_classes.is_empty() {
            violations.push(M5RepairActionCardPrimitiveViolation::ReversibilityClassMissing);
        }
        if row.trust_requirements.is_empty() {
            violations.push(M5RepairActionCardPrimitiveViolation::TrustRequirementMissing);
        }
        if row.change_classes.is_empty() {
            violations.push(M5RepairActionCardPrimitiveViolation::ChangeClassMissing);
        }
        if row.action_label_classes.is_empty() {
            violations.push(M5RepairActionCardPrimitiveViolation::ActionLabelClassMissing);
        }
        if row.repair_actions.is_empty() {
            violations.push(M5RepairActionCardPrimitiveViolation::RepairActionMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5RepairActionCardPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5RuntimeBoundaryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5RepairActionCardPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5RepairActionCardPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5RepairActionCardPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5RepairActionCardPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5RepairActionCardPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5RepairActionCardPrimitiveViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5RepairActionCardPrimitiveViolation::ConsumerInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must be a real mutation (a writing
/// blast radius) whose resolved projection carries a preview action and its changed
/// classes — the acceptance-criterion example that a user can review blast radius and
/// reversibility before any Doctor / support mutation runs.
fn validate_blast_radius_review_covered(
    packet: &M5RepairActionCardPrimitivePacket,
    violations: &mut Vec<M5RepairActionCardPrimitiveViolation>,
) {
    let proven = packet.consumer_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            blast_radius_is_write(case.resolved.blast_radius)
                && case.resolved.blast_radius_reviewable
                && case
                    .resolved
                    .available_actions
                    .contains(&M5RepairAction::PreviewRepair)
                && !case.resolved.changed_classes.is_empty()
        })
    });
    if !proven {
        violations.push(M5RepairActionCardPrimitiveViolation::BlastRadiusReviewUnproven);
    }
}

/// At least one worked resolution across the matrix must earn a non-generic action label
/// — the acceptance-criterion example that a remote, policy-gated, or non-exact repair
/// never reads like a generic `Fix now`.
fn validate_non_generic_label_covered(
    packet: &M5RepairActionCardPrimitivePacket,
    violations: &mut Vec<M5RepairActionCardPrimitiveViolation>,
) {
    let proven = packet.consumer_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.action_label_class.is_explicit())
    });
    if !proven {
        violations.push(M5RepairActionCardPrimitiveViolation::NonGenericLabelUnproven);
    }
}

/// At least one worked resolution across the matrix must disclose both a changed-class
/// list and an unchanged-class list — the acceptance-criterion example that preview
/// artifacts identify both classes so users can judge risk correctly.
fn validate_changed_and_unchanged_covered(
    packet: &M5RepairActionCardPrimitivePacket,
    violations: &mut Vec<M5RepairActionCardPrimitiveViolation>,
) {
    let proven = packet.consumer_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.changed_and_unchanged_disclosed)
    });
    if !proven {
        violations.push(M5RepairActionCardPrimitiveViolation::ChangedUnchangedDisclosureUnproven);
    }
}

fn validate_governance_review(
    packet: &M5RepairActionCardPrimitivePacket,
    violations: &mut Vec<M5RepairActionCardPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_repair_truth,
        review.blast_radius_and_reversibility_reviewable_before_mutation,
        review.changed_and_unchanged_classes_both_identified,
        review.blast_radius_never_understated,
        review.reversibility_never_overstated,
        review.target_boundary_never_masked,
        review.non_generic_action_labels_for_gated_or_non_exact_repairs,
        review.preview_and_reversal_vocabulary_in_support_export,
        review.no_surface_invents_second_repair_grammar,
        review.every_row_bound_to_shell_zone,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5RepairActionCardPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RepairActionCardPrimitivePacket,
    violations: &mut Vec<M5RepairActionCardPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.recovery_surfaces_consume_shared_primitive,
        projection.repair_resolver_reads_single_transaction_source,
        projection.preview_rows_read_single_preview_source,
        projection.target_boundary_reads_single_host_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5RepairActionCardPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RepairActionCardPrimitivePacket,
    violations: &mut Vec<M5RepairActionCardPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RepairActionCardPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5RepairActionCardPrimitivePacket,
    violations: &mut Vec<M5RepairActionCardPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.repair_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RepairActionCardPrimitiveViolation::ReleasePostureIncomplete);
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
fn value_repr_is_forbidden(value: &str) -> bool {
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
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
