//! Frozen M5 workspace-trust-banner, trust-fact-grid, trust-elevation-sheet,
//! restricted-capability-row, root-trust-strip, repair-transaction-preview-card,
//! rollback-class-strip, and repair-result-receipt-row component matrix.
//!
//! This module locks Aureline's reusable workspace-trust and guided-repair UI components into one
//! export-safe packet. Every workspace-trust or guided-repair surface M5 claims that still ships
//! its own trust or repair chrome — the workspace-trust banner, the trust-fact grid, the
//! trust-elevation sheet, the restricted-capability row, the root-trust strip, the
//! repair-transaction preview card, the rollback-class strip, and the repair-result receipt row —
//! is named once here and constrained by the same grant-source, trust-scope, narrowed-capability,
//! per-root-trust, reversal-class, checkpoint, repair-outcome, and preview vocabulary regardless of
//! the surface family that renders it.
//!
//! The matrix does not re-architect trust evaluation, entitlement issuance, or Project Doctor's
//! repair backend — it is the shared trust-and-repair-honesty component contract layered on top of
//! them. The controlled vocabularies are frozen in one self-describing
//! [`M5WorkspaceTrustRepairVocabularySet`] rather than minted per surface. The single controlled
//! trust/repair-disposition vocabulary consumers bind to — trusted, restricted, mixed-root,
//! policy-blocked, reduced-mode, preview-ready, checkpoint-missing, exact-reversal, compensate,
//! regenerate, manual-follow-up, and audit-only — keeps a restricted or mixed-root workspace from
//! ever reading as blanket trust, keeps a repair preview from hiding checkpoint absence or reversal
//! limits, and keeps exact / compensate / regenerate / manual / audit-only outcomes from collapsing
//! into a single generic success. Raw secret values and private endpoints stay outside the export
//! boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_workspace_trust_repair_component_matrix,
    seeded_m5_workspace_trust_repair_component_matrix_repair_transaction_preview_card_preview_narrowed,
    seeded_m5_workspace_trust_repair_component_matrix_trust_elevation_sheet_beta_narrowed,
    M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5WorkspaceTrustRepairComponentMatrixPacket`].
pub const M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix";

/// Schema version for M5 workspace-trust-repair component-matrix records.
pub const M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined workspace-trust-repair component-matrix schema.
pub const M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-workspace-trust-repair-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF: &str =
    "docs/trust/m5_workspace_trust_repair_components_contract.md";

/// Repo-relative path of the workspace-trust-banner canonical component schema.
pub const M5_WORKSPACE_TRUST_BANNER_SCHEMA_REF: &str =
    "schemas/ui/m5-workspace-trust-banner.schema.json";

/// Repo-relative path of the trust-fact-grid canonical component schema.
pub const M5_TRUST_FACT_GRID_SCHEMA_REF: &str = "schemas/ui/m5-trust-fact-grid.schema.json";

/// Repo-relative path of the trust-elevation-sheet canonical component schema.
pub const M5_TRUST_ELEVATION_SHEET_SCHEMA_REF: &str =
    "schemas/ui/m5-trust-elevation-sheet.schema.json";

/// Repo-relative path of the restricted-capability-row canonical component schema.
pub const M5_RESTRICTED_CAPABILITY_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-restricted-capability-row.schema.json";

/// Repo-relative path of the root-trust-strip canonical component schema.
pub const M5_ROOT_TRUST_STRIP_SCHEMA_REF: &str = "schemas/ui/m5-root-trust-strip.schema.json";

/// Repo-relative path of the repair-transaction-preview-card canonical component schema.
pub const M5_REPAIR_TRANSACTION_PREVIEW_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-repair-transaction-preview-card.schema.json";

/// Repo-relative path of the rollback-class-strip canonical component schema.
pub const M5_ROLLBACK_CLASS_STRIP_SCHEMA_REF: &str =
    "schemas/ui/m5-rollback-class-strip.schema.json";

/// Repo-relative path of the repair-result-receipt-row canonical component schema.
pub const M5_REPAIR_RESULT_RECEIPT_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-repair-result-receipt-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_WORKSPACE_TRUST_REPAIR_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-workspace-trust-repair-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_WORKSPACE_TRUST_REPAIR_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-workspace-trust-repair-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_WORKSPACE_TRUST_REPAIR_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-workspace-trust-repair-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_WORKSPACE_TRUST_REPAIR_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-workspace-trust-repair-component-matrix.md";

/// One of the eight governed workspace-trust / guided-repair component families this matrix
/// freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRepairComponentFamily {
    /// A workspace-trust banner naming whether the workspace is trusted, restricted, or mixed-root
    /// and who granted the trust.
    WorkspaceTrustBanner,
    /// A trust-fact grid naming grant source, trust scope, narrowed capability, and per-root trust
    /// in one place.
    TrustFactGrid,
    /// A trust-elevation sheet naming what a trust elevation grants and its source and scope.
    TrustElevationSheet,
    /// A restricted-capability row naming exactly which capability is narrowed and why.
    RestrictedCapabilityRow,
    /// A root-trust strip naming per-root trust so mixed-root trust never reads as uniform.
    RootTrustStrip,
    /// A repair-transaction preview card naming repair targets, checkpoint availability, and
    /// reversal class before anything is applied.
    RepairTransactionPreviewCard,
    /// A rollback-class strip naming exact / compensate / regenerate / manual / audit-only reversal
    /// class and checkpoint availability.
    RollbackClassStrip,
    /// A repair-result receipt row naming the applied outcome and any manual follow-up.
    RepairResultReceiptRow,
}

impl M5WorkspaceTrustRepairComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::WorkspaceTrustBanner,
        Self::TrustFactGrid,
        Self::TrustElevationSheet,
        Self::RestrictedCapabilityRow,
        Self::RootTrustStrip,
        Self::RepairTransactionPreviewCard,
        Self::RollbackClassStrip,
        Self::RepairResultReceiptRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceTrustBanner => "workspace_trust_banner",
            Self::TrustFactGrid => "trust_fact_grid",
            Self::TrustElevationSheet => "trust_elevation_sheet",
            Self::RestrictedCapabilityRow => "restricted_capability_row",
            Self::RootTrustStrip => "root_trust_strip",
            Self::RepairTransactionPreviewCard => "repair_transaction_preview_card",
            Self::RollbackClassStrip => "rollback_class_strip",
            Self::RepairResultReceiptRow => "repair_result_receipt_row",
        }
    }

    /// The canonical per-component schema ref a downstream row points at instead of restating this
    /// component's trust / repair truth by hand.
    pub const fn canonical_component_schema_ref(self) -> &'static str {
        match self {
            Self::WorkspaceTrustBanner => M5_WORKSPACE_TRUST_BANNER_SCHEMA_REF,
            Self::TrustFactGrid => M5_TRUST_FACT_GRID_SCHEMA_REF,
            Self::TrustElevationSheet => M5_TRUST_ELEVATION_SHEET_SCHEMA_REF,
            Self::RestrictedCapabilityRow => M5_RESTRICTED_CAPABILITY_ROW_SCHEMA_REF,
            Self::RootTrustStrip => M5_ROOT_TRUST_STRIP_SCHEMA_REF,
            Self::RepairTransactionPreviewCard => M5_REPAIR_TRANSACTION_PREVIEW_CARD_SCHEMA_REF,
            Self::RollbackClassStrip => M5_ROLLBACK_CLASS_STRIP_SCHEMA_REF,
            Self::RepairResultReceiptRow => M5_REPAIR_RESULT_RECEIPT_ROW_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled grant-source class.
    pub const fn declares_grant_source(self) -> bool {
        matches!(
            self,
            Self::WorkspaceTrustBanner
                | Self::TrustFactGrid
                | Self::TrustElevationSheet
                | Self::RootTrustStrip
        )
    }

    /// `true` when this family must name a controlled trust-scope state.
    pub const fn declares_trust_scope(self) -> bool {
        matches!(
            self,
            Self::WorkspaceTrustBanner
                | Self::TrustFactGrid
                | Self::TrustElevationSheet
                | Self::RestrictedCapabilityRow
        )
    }

    /// `true` when this family must name a controlled narrowed-capability state.
    pub const fn declares_capability_narrow(self) -> bool {
        matches!(
            self,
            Self::WorkspaceTrustBanner | Self::TrustFactGrid | Self::RestrictedCapabilityRow
        )
    }

    /// `true` when this family must name a controlled per-root trust state.
    pub const fn declares_root_trust(self) -> bool {
        matches!(self, Self::TrustFactGrid | Self::RootTrustStrip)
    }

    /// `true` when this family must name a controlled reversal class.
    pub const fn declares_reversal_class(self) -> bool {
        matches!(
            self,
            Self::RepairTransactionPreviewCard
                | Self::RollbackClassStrip
                | Self::RepairResultReceiptRow
        )
    }

    /// `true` when this family must name a controlled checkpoint state.
    pub const fn declares_checkpoint(self) -> bool {
        matches!(
            self,
            Self::RepairTransactionPreviewCard | Self::RollbackClassStrip
        )
    }

    /// `true` when this family must name a controlled repair-outcome class.
    pub const fn declares_repair_outcome(self) -> bool {
        matches!(self, Self::RepairResultReceiptRow)
    }

    /// `true` when this family must name a controlled preview state.
    pub const fn declares_preview_state(self) -> bool {
        matches!(self, Self::RepairTransactionPreviewCard)
    }
}

/// The single controlled trust / repair-disposition vocabulary every workspace-trust or
/// guided-repair consumer binds to. These are the exact acceptance-criteria tokens that keep a
/// restricted or mixed-root workspace from reading as blanket trust and keep exact / compensate /
/// regenerate / manual / audit-only outcomes from collapsing into a single generic success. No
/// trust or repair surface invents a parallel word for any of these dispositions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRepairDisposition {
    /// The workspace or root is trusted.
    Trusted,
    /// The workspace or root is in restricted mode.
    Restricted,
    /// Some roots are trusted and some are not; trust is mixed.
    MixedRoot,
    /// Trust or a capability is blocked by policy.
    PolicyBlocked,
    /// The surface is operating in a reduced mode with narrowed capability.
    ReducedMode,
    /// A repair transaction is previewed and ready to review.
    PreviewReady,
    /// No checkpoint is available for a repair, labelled as checkpoint-missing.
    CheckpointMissing,
    /// The repair can be reversed exactly.
    ExactReversal,
    /// The repair can only be reversed by a compensating action.
    Compensate,
    /// The repair can only be reversed by regenerating the affected state.
    Regenerate,
    /// The repair requires manual follow-up to complete or reverse.
    ManualFollowUp,
    /// The change is audit-only and cannot be reversed in-product.
    AuditOnly,
}

impl M5WorkspaceTrustRepairDisposition {
    /// Every disposition token, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Trusted,
        Self::Restricted,
        Self::MixedRoot,
        Self::PolicyBlocked,
        Self::ReducedMode,
        Self::PreviewReady,
        Self::CheckpointMissing,
        Self::ExactReversal,
        Self::Compensate,
        Self::Regenerate,
        Self::ManualFollowUp,
        Self::AuditOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Restricted => "restricted",
            Self::MixedRoot => "mixed_root",
            Self::PolicyBlocked => "policy_blocked",
            Self::ReducedMode => "reduced_mode",
            Self::PreviewReady => "preview_ready",
            Self::CheckpointMissing => "checkpoint_missing",
            Self::ExactReversal => "exact_reversal",
            Self::Compensate => "compensate",
            Self::Regenerate => "regenerate",
            Self::ManualFollowUp => "manual_follow_up",
            Self::AuditOnly => "audit_only",
        }
    }

    /// Whether this disposition is the one clean fully-trusted state.
    pub const fn is_full_trust(self) -> bool {
        matches!(self, Self::Trusted)
    }
}

/// Controlled grant-source class — who granted the workspace trust and under which policy epoch, so
/// trust lineage is never left implicit and a policy-managed grant never reads as a user's own
/// explicit choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustGrantSourceClass {
    /// The user explicitly trusted this workspace or root.
    UserExplicit,
    /// Trust was inherited from a trusted parent folder.
    InheritedParent,
    /// Trust was granted by an org / managed policy.
    PolicyManaged,
    /// Trust came from a checked-in workspace configuration.
    WorkspaceConfig,
    /// A first-party local default with no external grant.
    FirstPartyDefault,
    /// The grant source cannot currently be resolved.
    GrantSourceUnknown,
}

impl M5TrustGrantSourceClass {
    /// Every grant-source class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UserExplicit,
        Self::InheritedParent,
        Self::PolicyManaged,
        Self::WorkspaceConfig,
        Self::FirstPartyDefault,
        Self::GrantSourceUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserExplicit => "user_explicit",
            Self::InheritedParent => "inherited_parent",
            Self::PolicyManaged => "policy_managed",
            Self::WorkspaceConfig => "workspace_config",
            Self::FirstPartyDefault => "first_party_default",
            Self::GrantSourceUnknown => "grant_source_unknown",
        }
    }
}

/// Controlled trust-scope state — the trusted object and root scope, so a restricted, mixed-root, or
/// policy-blocked workspace is never presented as blanket trust across roots or routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustScopeState {
    /// The whole workspace is trusted.
    TrustedWorkspace,
    /// A specific root is trusted, not the whole workspace.
    TrustedRoot,
    /// The workspace is in restricted mode.
    RestrictedWorkspace,
    /// Some roots are trusted and some are not.
    MixedRoot,
    /// Trust is blocked by policy.
    PolicyBlocked,
    /// The trust scope cannot currently be resolved.
    ScopeUnknown,
}

impl M5TrustScopeState {
    /// Every trust-scope state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TrustedWorkspace,
        Self::TrustedRoot,
        Self::RestrictedWorkspace,
        Self::MixedRoot,
        Self::PolicyBlocked,
        Self::ScopeUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedWorkspace => "trusted_workspace",
            Self::TrustedRoot => "trusted_root",
            Self::RestrictedWorkspace => "restricted_workspace",
            Self::MixedRoot => "mixed_root",
            Self::PolicyBlocked => "policy_blocked",
            Self::ScopeUnknown => "scope_unknown",
        }
    }
}

/// Controlled narrowed-capability state — exactly which capability a restricted or reduced mode
/// removes, so a narrowed capability is always named rather than left as a vague "some features are
/// off".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityNarrowState {
    /// Full capability; nothing is narrowed.
    FullCapability,
    /// The surface is in reduced mode.
    ReducedMode,
    /// A specific task is blocked.
    TaskBlocked,
    /// Code execution is blocked.
    ExecutionBlocked,
    /// Extension activation is blocked.
    ExtensionBlocked,
    /// The narrowed-capability state cannot currently be resolved.
    CapabilityUnknown,
}

impl M5CapabilityNarrowState {
    /// Every narrowed-capability state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullCapability,
        Self::ReducedMode,
        Self::TaskBlocked,
        Self::ExecutionBlocked,
        Self::ExtensionBlocked,
        Self::CapabilityUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullCapability => "full_capability",
            Self::ReducedMode => "reduced_mode",
            Self::TaskBlocked => "task_blocked",
            Self::ExecutionBlocked => "execution_blocked",
            Self::ExtensionBlocked => "extension_blocked",
            Self::CapabilityUnknown => "capability_unknown",
        }
    }
}

/// Controlled per-root trust state — the trust of a single root within a multi-root workspace, so
/// mixed-root trust never collapses into one uniform trust badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RootTrustState {
    /// This root is trusted.
    RootTrusted,
    /// This root is restricted.
    RootRestricted,
    /// This root inherits trust from its parent.
    RootInherited,
    /// This root is blocked by policy.
    RootPolicyBlocked,
    /// This root has mixed trust among its children.
    RootMixedChildren,
    /// This root's trust cannot currently be resolved.
    RootUnknown,
}

impl M5RootTrustState {
    /// Every per-root trust state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RootTrusted,
        Self::RootRestricted,
        Self::RootInherited,
        Self::RootPolicyBlocked,
        Self::RootMixedChildren,
        Self::RootUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RootTrusted => "root_trusted",
            Self::RootRestricted => "root_restricted",
            Self::RootInherited => "root_inherited",
            Self::RootPolicyBlocked => "root_policy_blocked",
            Self::RootMixedChildren => "root_mixed_children",
            Self::RootUnknown => "root_unknown",
        }
    }
}

/// Controlled reversal class — how a repair transaction can be undone, so exact, compensating,
/// regenerating, manual, and audit-only reversals are never collapsed into one generic "undo".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairReversalClass {
    /// The repair can be reversed exactly to the prior state.
    ExactReversal,
    /// The repair can only be reversed by a compensating action.
    CompensatingReversal,
    /// The repair can only be reversed by regenerating the affected state.
    RegenerateReversal,
    /// The repair requires manual follow-up to reverse.
    ManualFollowUp,
    /// The change is audit-only and cannot be reversed in-product.
    AuditOnly,
    /// The reversal class cannot currently be resolved.
    ReversalUnknown,
}

impl M5RepairReversalClass {
    /// Every reversal class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactReversal,
        Self::CompensatingReversal,
        Self::RegenerateReversal,
        Self::ManualFollowUp,
        Self::AuditOnly,
        Self::ReversalUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactReversal => "exact_reversal",
            Self::CompensatingReversal => "compensating_reversal",
            Self::RegenerateReversal => "regenerate_reversal",
            Self::ManualFollowUp => "manual_follow_up",
            Self::AuditOnly => "audit_only",
            Self::ReversalUnknown => "reversal_unknown",
        }
    }
}

/// Controlled checkpoint state — whether a repair has a restore checkpoint, so checkpoint absence is
/// never hidden behind an otherwise reassuring preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairCheckpointState {
    /// A full checkpoint is available before the repair.
    CheckpointAvailable,
    /// Only a partial checkpoint is available.
    CheckpointPartial,
    /// No checkpoint is available, labelled as checkpoint-missing.
    CheckpointMissing,
    /// A checkpoint existed but has expired.
    CheckpointExpired,
    /// The checkpoint is held externally, outside product control.
    CheckpointExternal,
    /// The checkpoint state cannot currently be resolved.
    CheckpointUnknown,
}

impl M5RepairCheckpointState {
    /// Every checkpoint state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CheckpointAvailable,
        Self::CheckpointPartial,
        Self::CheckpointMissing,
        Self::CheckpointExpired,
        Self::CheckpointExternal,
        Self::CheckpointUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheckpointAvailable => "checkpoint_available",
            Self::CheckpointPartial => "checkpoint_partial",
            Self::CheckpointMissing => "checkpoint_missing",
            Self::CheckpointExpired => "checkpoint_expired",
            Self::CheckpointExternal => "checkpoint_external",
            Self::CheckpointUnknown => "checkpoint_unknown",
        }
    }
}

/// Controlled repair-outcome class — the applied result of a repair transaction, so partial success
/// is never shown as complete and each outcome keeps its own honest word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairOutcomeClass {
    /// The repair applied and can be reversed exactly.
    RepairAppliedExact,
    /// The repair applied via a compensating action.
    RepairCompensated,
    /// The repair applied by regenerating the affected state.
    RepairRegenerated,
    /// The repair partially succeeded and some targets remain.
    RepairPartialSuccess,
    /// The repair requires manual follow-up to finish.
    RepairManualRequired,
    /// The repair failed and nothing was changed.
    RepairFailed,
}

impl M5RepairOutcomeClass {
    /// Every repair-outcome class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RepairAppliedExact,
        Self::RepairCompensated,
        Self::RepairRegenerated,
        Self::RepairPartialSuccess,
        Self::RepairManualRequired,
        Self::RepairFailed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepairAppliedExact => "repair_applied_exact",
            Self::RepairCompensated => "repair_compensated",
            Self::RepairRegenerated => "repair_regenerated",
            Self::RepairPartialSuccess => "repair_partial_success",
            Self::RepairManualRequired => "repair_manual_required",
            Self::RepairFailed => "repair_failed",
        }
    }
}

/// Controlled preview state — whether a repair-transaction preview is complete and ready to review,
/// so an incomplete or blocked preview never reads as fully previewed and ready to apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairPreviewState {
    /// The preview is complete and ready to review.
    PreviewReady,
    /// The preview is incomplete; not every target is resolved.
    PreviewIncomplete,
    /// The preview is blocked by policy or a missing dependency.
    PreviewBlocked,
    /// A repair candidate is identified with a stable id.
    CandidateIdentified,
    /// A repair candidate is ambiguous and needs disambiguation.
    CandidateAmbiguous,
    /// The preview state cannot currently be resolved.
    PreviewUnknown,
}

impl M5RepairPreviewState {
    /// Every preview state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PreviewReady,
        Self::PreviewIncomplete,
        Self::PreviewBlocked,
        Self::CandidateIdentified,
        Self::CandidateAmbiguous,
        Self::PreviewUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewReady => "preview_ready",
            Self::PreviewIncomplete => "preview_incomplete",
            Self::PreviewBlocked => "preview_blocked",
            Self::CandidateIdentified => "candidate_identified",
            Self::CandidateAmbiguous => "candidate_ambiguous",
            Self::PreviewUnknown => "preview_unknown",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a workspace-trust-repair component. No
/// component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRepairSurfaceFamily {
    /// The workspace shell.
    WorkspaceShell,
    /// The settings / trust pane.
    SettingsTrust,
    /// Project Doctor.
    ProjectDoctor,
    /// Safe mode.
    SafeMode,
    /// The guided-repair flow.
    GuidedRepair,
    /// The support export.
    SupportExport,
}

impl M5WorkspaceTrustRepairSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WorkspaceShell,
        Self::SettingsTrust,
        Self::ProjectDoctor,
        Self::SafeMode,
        Self::GuidedRepair,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceShell => "workspace_shell",
            Self::SettingsTrust => "settings_trust",
            Self::ProjectDoctor => "project_doctor",
            Self::SafeMode => "safe_mode",
            Self::GuidedRepair => "guided_repair",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's grant-source,
/// trust-scope, reversal-class, or checkpoint truth never silently narrows or widens between
/// deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRepairDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5WorkspaceTrustRepairDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRepairConsumerSurface {
    /// The workspace-trust UI.
    WorkspaceTrustUi,
    /// The settings UI.
    SettingsUi,
    /// The Project Doctor UI.
    DoctorUi,
    /// The safe-mode UI.
    SafeModeUi,
    /// The extensions UI.
    ExtensionsUi,
    /// The remote / workspace UI.
    RemoteUi,
    /// The AI context surface.
    AiContextUi,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5WorkspaceTrustRepairConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::WorkspaceTrustUi,
        Self::SettingsUi,
        Self::DoctorUi,
        Self::SafeModeUi,
        Self::ExtensionsUi,
        Self::RemoteUi,
        Self::AiContextUi,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceTrustUi => "workspace_trust_ui",
            Self::SettingsUi => "settings_ui",
            Self::DoctorUi => "doctor_ui",
            Self::SafeModeUi => "safe_mode_ui",
            Self::ExtensionsUi => "extensions_ui",
            Self::RemoteUi => "remote_ui",
            Self::AiContextUi => "ai_context_ui",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no trust or repair truth is
/// hover-only, pointer-only, menu-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRepairAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet, never menu-only.
    SupportExportable,
}

impl M5WorkspaceTrustRepairAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Reason a workspace-trust-repair component has degraded below its qualified state. Required on
/// every row so a stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRepairDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The grant source cannot be verified.
    GrantSourceUnavailable,
    /// The policy epoch behind a grant is unknown.
    PolicyEpochUnknown,
    /// The checkpoint signal is unavailable.
    CheckpointSignalUnavailable,
    /// The reversal class could not be verified.
    ReversalClassUnverified,
    /// An upstream trust lane narrowed.
    UpstreamTrustNarrowed,
}

impl M5WorkspaceTrustRepairDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::GrantSourceUnavailable,
        Self::PolicyEpochUnknown,
        Self::CheckpointSignalUnavailable,
        Self::ReversalClassUnverified,
        Self::UpstreamTrustNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::GrantSourceUnavailable => "grant_source_unavailable",
            Self::PolicyEpochUnknown => "policy_epoch_unknown",
            Self::CheckpointSignalUnavailable => "checkpoint_signal_unavailable",
            Self::ReversalClassUnverified => "reversal_class_unverified",
            Self::UpstreamTrustNarrowed => "upstream_trust_narrowed",
        }
    }
}

/// Mandatory label a claimed workspace-trust-repair component must be able to show. The first three
/// are hard requirements on every component; the remaining three close the acceptance-criteria
/// ambiguity about grant lineage, reversal / checkpoint, and narrowed capability plus root scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRepairRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state / disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The grant source and trust scope behind the component.
    GrantSourceAndScope,
    /// The reversal class and checkpoint availability behind the component.
    ReversalAndCheckpoint,
    /// The narrowed capability and per-root scope behind the component.
    CapabilityAndRootScope,
}

impl M5WorkspaceTrustRepairRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::GrantSourceAndScope,
        Self::ReversalAndCheckpoint,
        Self::CapabilityAndRootScope,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::GrantSourceAndScope => "grant_source_and_scope",
            Self::ReversalAndCheckpoint => "reversal_and_checkpoint",
            Self::CapabilityAndRootScope => "capability_and_root_scope",
        }
    }
}

/// Qualification class for an M5 workspace-trust-repair component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRepairQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5WorkspaceTrustRepairQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a workspace-trust-repair component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRepairDowngradeTrigger {
    /// A component left its grant source unstated.
    GrantSourceUnstated,
    /// A component left its policy epoch unstated.
    PolicyEpochUnstated,
    /// A component collapsed root scope into blanket trust.
    RootScopeCollapsedIntoBlanketTrust,
    /// A component left its narrowed capability unstated.
    NarrowedCapabilityUnstated,
    /// A component hid the absence of a checkpoint.
    CheckpointAbsenceHidden,
    /// A component hid the limits of a reversal.
    ReversalLimitHidden,
    /// A component collapsed distinct reversal outcomes into a generic success.
    ReversalClassCollapsedIntoGenericSuccess,
    /// A component left its repair-target ids unstated.
    RepairTargetIdsUnstated,
    /// A component showed a partial success as complete.
    PartialSuccessShownAsComplete,
    /// A component showed mixed-root trust as uniform trust.
    MixedRootShownAsUniformTrust,
    /// Generic chrome wording concealed trust or repair truth.
    GenericChromeWordingUsed,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5WorkspaceTrustRepairDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::GrantSourceUnstated,
        Self::PolicyEpochUnstated,
        Self::RootScopeCollapsedIntoBlanketTrust,
        Self::NarrowedCapabilityUnstated,
        Self::CheckpointAbsenceHidden,
        Self::ReversalLimitHidden,
        Self::ReversalClassCollapsedIntoGenericSuccess,
        Self::RepairTargetIdsUnstated,
        Self::PartialSuccessShownAsComplete,
        Self::MixedRootShownAsUniformTrust,
        Self::GenericChromeWordingUsed,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GrantSourceUnstated => "grant_source_unstated",
            Self::PolicyEpochUnstated => "policy_epoch_unstated",
            Self::RootScopeCollapsedIntoBlanketTrust => "root_scope_collapsed_into_blanket_trust",
            Self::NarrowedCapabilityUnstated => "narrowed_capability_unstated",
            Self::CheckpointAbsenceHidden => "checkpoint_absence_hidden",
            Self::ReversalLimitHidden => "reversal_limit_hidden",
            Self::ReversalClassCollapsedIntoGenericSuccess => {
                "reversal_class_collapsed_into_generic_success"
            }
            Self::RepairTargetIdsUnstated => "repair_target_ids_unstated",
            Self::PartialSuccessShownAsComplete => "partial_success_shown_as_complete",
            Self::MixedRootShownAsUniformTrust => "mixed_root_shown_as_uniform_trust",
            Self::GenericChromeWordingUsed => "generic_chrome_wording_used",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed workspace-trust-repair component family bound to the surface-
/// specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceTrustRepairComponentRow {
    /// Governed component family.
    pub component_family: M5WorkspaceTrustRepairComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5WorkspaceTrustRepairQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this component.
    pub surface_families: Vec<M5WorkspaceTrustRepairSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5WorkspaceTrustRepairDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5WorkspaceTrustRepairRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5WorkspaceTrustRepairRequiredLabel>,
    /// Trust / repair dispositions this component can carry (the frozen AC vocabulary; required on
    /// every component).
    pub dispositions: Vec<M5WorkspaceTrustRepairDisposition>,
    /// Grant-source classes this component names (grant-bearing families only).
    pub grant_source_classes: Vec<M5TrustGrantSourceClass>,
    /// Trust-scope states this component names (scope-bearing families only).
    pub trust_scope_states: Vec<M5TrustScopeState>,
    /// Narrowed-capability states this component names (capability-bearing families only).
    pub capability_narrow_states: Vec<M5CapabilityNarrowState>,
    /// Per-root trust states this component names (root-bearing families only).
    pub root_trust_states: Vec<M5RootTrustState>,
    /// Reversal classes this component names (repair families only).
    pub reversal_classes: Vec<M5RepairReversalClass>,
    /// Checkpoint states this component names (checkpoint-bearing families only).
    pub checkpoint_states: Vec<M5RepairCheckpointState>,
    /// Repair-outcome classes this component names (outcome-bearing families only).
    pub repair_outcomes: Vec<M5RepairOutcomeClass>,
    /// Preview states this component names (preview-bearing families only).
    pub preview_states: Vec<M5RepairPreviewState>,
    /// Degraded reasons this component can name (required on every component).
    pub degraded_reasons: Vec<M5WorkspaceTrustRepairDegradedReason>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5WorkspaceTrustRepairAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5WorkspaceTrustRepairConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5WorkspaceTrustRepairDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component (must include its own canonical component
    /// schema so downstream rows have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never implies blanket trust across roots, profiles, or routes.
    /// MUST be `false`.
    pub implies_blanket_trust_across_roots_or_routes: bool,
    /// Hard invariant: this component never hides checkpoint absence or reversal limits. MUST be
    /// `false`.
    pub hides_checkpoint_absence_or_reversal_limits: bool,
    /// Hard invariant: this component never collapses exact / compensate / regenerate / manual /
    /// audit-only outcomes into generic success copy. MUST be `false`.
    pub collapses_reversal_outcomes_into_generic_success: bool,
    /// Hard invariant: this component never presents a partial success as a complete success. MUST
    /// be `false`.
    pub presents_partial_success_as_complete: bool,
}

impl M5WorkspaceTrustRepairComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5WorkspaceTrustRepairRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5WorkspaceTrustRepairRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.implies_blanket_trust_across_roots_or_routes
            && !self.hides_checkpoint_absence_or_reversal_limits
            && !self.collapses_reversal_outcomes_into_generic_success
            && !self.presents_partial_success_as_complete
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceTrustRepairVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Trust / repair-disposition tokens.
    pub dispositions: Vec<String>,
    /// Grant-source tokens.
    pub grant_source_classes: Vec<String>,
    /// Trust-scope tokens.
    pub trust_scope_states: Vec<String>,
    /// Narrowed-capability tokens.
    pub capability_narrow_states: Vec<String>,
    /// Per-root trust tokens.
    pub root_trust_states: Vec<String>,
    /// Reversal-class tokens.
    pub reversal_classes: Vec<String>,
    /// Checkpoint-state tokens.
    pub checkpoint_states: Vec<String>,
    /// Repair-outcome tokens.
    pub repair_outcomes: Vec<String>,
    /// Preview-state tokens.
    pub preview_states: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5WorkspaceTrustRepairVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5WorkspaceTrustRepairComponentFamily::ALL, |v| v.as_str()),
            dispositions: tokens(&M5WorkspaceTrustRepairDisposition::ALL, |v| v.as_str()),
            grant_source_classes: tokens(&M5TrustGrantSourceClass::ALL, |v| v.as_str()),
            trust_scope_states: tokens(&M5TrustScopeState::ALL, |v| v.as_str()),
            capability_narrow_states: tokens(&M5CapabilityNarrowState::ALL, |v| v.as_str()),
            root_trust_states: tokens(&M5RootTrustState::ALL, |v| v.as_str()),
            reversal_classes: tokens(&M5RepairReversalClass::ALL, |v| v.as_str()),
            checkpoint_states: tokens(&M5RepairCheckpointState::ALL, |v| v.as_str()),
            repair_outcomes: tokens(&M5RepairOutcomeClass::ALL, |v| v.as_str()),
            preview_states: tokens(&M5RepairPreviewState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5WorkspaceTrustRepairSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5WorkspaceTrustRepairDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5WorkspaceTrustRepairConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5WorkspaceTrustRepairAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5WorkspaceTrustRepairDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5WorkspaceTrustRepairRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5WorkspaceTrustRepairDowngradeTrigger::ALL, |v| {
                v.as_str()
            }),
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
pub struct M5WorkspaceTrustRepairGovernanceReview {
    /// The workspace-trust banner shows its grant source and trust scope.
    pub workspace_trust_banner_shows_grant_source_and_scope: bool,
    /// The trust-fact grid shows grant source, scope, capability, and root together.
    pub trust_fact_grid_shows_grant_scope_capability_root_together: bool,
    /// The trust-elevation sheet shows the grant source and scope change.
    pub trust_elevation_sheet_shows_grant_source_and_scope_change: bool,
    /// The restricted-capability row shows exactly which capability is narrowed.
    pub restricted_capability_row_shows_narrowed_capability: bool,
    /// The root-trust strip shows per-root trust.
    pub root_trust_strip_shows_per_root_trust: bool,
    /// The repair-transaction preview card shows targets, checkpoint, and reversal.
    pub repair_transaction_preview_card_shows_targets_checkpoint_reversal: bool,
    /// The rollback-class strip shows the reversal class and checkpoint availability.
    pub rollback_class_strip_shows_reversal_class_and_checkpoint: bool,
    /// The repair-result receipt row shows the outcome and any manual follow-up.
    pub repair_result_receipt_row_shows_outcome_and_followup: bool,
    /// No trust surface implies blanket approval across roots, profiles, or routes.
    pub no_trust_surface_implies_blanket_approval: bool,
    /// Grant source and policy epoch are always explicit.
    pub grant_source_and_policy_epoch_always_explicit: bool,
    /// Checkpoint absence is never hidden.
    pub checkpoint_absence_never_hidden: bool,
    /// Reversal outcomes are never collapsed into a generic success.
    pub reversal_outcomes_never_collapsed_into_generic_success: bool,
    /// A partial success is never shown as complete.
    pub partial_success_never_shown_as_complete: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel trust / repair vocabulary.
    pub later_rows_cannot_invent_parallel_trust_repair_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceTrustRepairConsumerProjection {
    /// Trust surfaces consume the shared grant-source vocabulary.
    pub trust_surfaces_consume_grant_source_vocabulary: bool,
    /// Settings and Doctor surfaces consume the shared narrowed-capability vocabulary.
    pub settings_and_doctor_consume_capability_narrow_vocabulary: bool,
    /// Safe mode consumes the shared per-root trust vocabulary.
    pub safe_mode_consumes_root_trust_vocabulary: bool,
    /// Repair surfaces consume the shared reversal-class vocabulary.
    pub repair_surfaces_consume_reversal_class_vocabulary: bool,
    /// Guided repair consumes the shared checkpoint vocabulary.
    pub guided_repair_consumes_checkpoint_vocabulary: bool,
    /// Support / export reads a single canonical trust / repair source.
    pub support_export_reads_single_trust_repair_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceTrustRepairProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the workspace-trust-repair component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceTrustRepairReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting trust / repair component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5WorkspaceTrustRepairComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WorkspaceTrustRepairComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5WorkspaceTrustRepairComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WorkspaceTrustRepairVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WorkspaceTrustRepairGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WorkspaceTrustRepairConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WorkspaceTrustRepairProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WorkspaceTrustRepairReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 workspace-trust-repair component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceTrustRepairComponentMatrixPacket {
    /// Record kind; must equal [`M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5WorkspaceTrustRepairComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WorkspaceTrustRepairVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WorkspaceTrustRepairGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WorkspaceTrustRepairConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WorkspaceTrustRepairProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WorkspaceTrustRepairReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5WorkspaceTrustRepairComponentMatrixPacket {
    /// Builds an M5 workspace-trust-repair component matrix packet from stable-lane input.
    pub fn new(input: M5WorkspaceTrustRepairComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 workspace-trust-repair component matrix invariants.
    pub fn validate(&self) -> Vec<M5WorkspaceTrustRepairComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 workspace-trust-repair component matrix serializes"),
        ) {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 workspace-trust-repair component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.component_family.canonical_component_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Workspace-Trust-Banner, Trust-Fact-Grid, Trust-Elevation-Sheet, Restricted-Capability-Row, Root-Trust-Strip, Repair-Transaction-Preview-Card, Rollback-Class-Strip, and Repair-Result-Receipt-Row Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Trust / repair dispositions: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Reversal classes: {}\n",
            self.vocabulary_set.reversal_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.component_family.canonical_component_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 workspace-trust-repair matrix export.
#[derive(Debug)]
pub enum M5WorkspaceTrustRepairComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5WorkspaceTrustRepairComponentMatrixViolation>),
}

impl fmt::Display for M5WorkspaceTrustRepairComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 workspace-trust-repair component matrix export parse failed: {error}"
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
                    "m5 workspace-trust-repair component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5WorkspaceTrustRepairComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5WorkspaceTrustRepairComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5WorkspaceTrustRepairComponentMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component row does not point at its own canonical component schema.
    ComponentSchemaRefMissing,
    /// A component declares no trust / repair dispositions.
    DispositionMissing,
    /// A grant-bearing component declares no grant-source classes.
    GrantSourceMissing,
    /// A scope-bearing component declares no trust-scope states.
    TrustScopeMissing,
    /// A capability-bearing component declares no narrowed-capability states.
    CapabilityNarrowMissing,
    /// A root-bearing component declares no per-root trust states.
    RootTrustMissing,
    /// A repair component declares no reversal classes.
    ReversalClassMissing,
    /// A checkpoint-bearing component declares no checkpoint states.
    CheckpointStateMissing,
    /// An outcome-bearing component declares no repair-outcome classes.
    RepairOutcomeMissing,
    /// A preview-bearing component declares no preview states.
    PreviewStateMissing,
    /// A component declares no degraded reasons.
    DegradedReasonMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (implies blanket trust across roots or routes, hides
    /// checkpoint absence or reversal limits, collapses reversal outcomes into generic success, or
    /// presents partial success as complete).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5WorkspaceTrustRepairComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::DispositionMissing => "disposition_missing",
            Self::GrantSourceMissing => "grant_source_missing",
            Self::TrustScopeMissing => "trust_scope_missing",
            Self::CapabilityNarrowMissing => "capability_narrow_missing",
            Self::RootTrustMissing => "root_trust_missing",
            Self::ReversalClassMissing => "reversal_class_missing",
            Self::CheckpointStateMissing => "checkpoint_state_missing",
            Self::RepairOutcomeMissing => "repair_outcome_missing",
            Self::PreviewStateMissing => "preview_state_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 workspace-trust-repair matrix export.
pub fn current_stable_m5_workspace_trust_repair_component_matrix_export() -> Result<
    M5WorkspaceTrustRepairComponentMatrixPacket,
    M5WorkspaceTrustRepairComponentMatrixArtifactError,
> {
    let packet: M5WorkspaceTrustRepairComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-workspace-trust-repair-proof/support_export.json"
        )))
        .map_err(M5WorkspaceTrustRepairComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5WorkspaceTrustRepairComponentMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5WorkspaceTrustRepairComponentMatrixPacket,
    violations: &mut Vec<M5WorkspaceTrustRepairComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
        M5_WORKSPACE_TRUST_BANNER_SCHEMA_REF,
        M5_TRUST_FACT_GRID_SCHEMA_REF,
        M5_TRUST_ELEVATION_SHEET_SCHEMA_REF,
        M5_RESTRICTED_CAPABILITY_ROW_SCHEMA_REF,
        M5_ROOT_TRUST_STRIP_SCHEMA_REF,
        M5_REPAIR_TRANSACTION_PREVIEW_CARD_SCHEMA_REF,
        M5_ROLLBACK_CLASS_STRIP_SCHEMA_REF,
        M5_REPAIR_RESULT_RECEIPT_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5WorkspaceTrustRepairComponentMatrixPacket,
    violations: &mut Vec<M5WorkspaceTrustRepairComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5WorkspaceTrustRepairComponentMatrixPacket,
    violations: &mut Vec<M5WorkspaceTrustRepairComponentMatrixViolation>,
) {
    let present: BTreeSet<M5WorkspaceTrustRepairComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5WorkspaceTrustRepairComponentFamily::ALL {
        if !present.contains(&required) {
            violations
                .push(M5WorkspaceTrustRepairComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_component_schema_ref())
        {
            violations
                .push(M5WorkspaceTrustRepairComponentMatrixViolation::ComponentSchemaRefMissing);
        }
        if row.dispositions.is_empty() {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::DispositionMissing);
        }
        if family.declares_grant_source() && row.grant_source_classes.is_empty() {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::GrantSourceMissing);
        }
        if family.declares_trust_scope() && row.trust_scope_states.is_empty() {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::TrustScopeMissing);
        }
        if family.declares_capability_narrow() && row.capability_narrow_states.is_empty() {
            violations
                .push(M5WorkspaceTrustRepairComponentMatrixViolation::CapabilityNarrowMissing);
        }
        if family.declares_root_trust() && row.root_trust_states.is_empty() {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::RootTrustMissing);
        }
        if family.declares_reversal_class() && row.reversal_classes.is_empty() {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::ReversalClassMissing);
        }
        if family.declares_checkpoint() && row.checkpoint_states.is_empty() {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::CheckpointStateMissing);
        }
        if family.declares_repair_outcome() && row.repair_outcomes.is_empty() {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::RepairOutcomeMissing);
        }
        if family.declares_preview_state() && row.preview_states.is_empty() {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::PreviewStateMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations
                .push(M5WorkspaceTrustRepairComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations
                .push(M5WorkspaceTrustRepairComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations
                .push(M5WorkspaceTrustRepairComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations
                .push(M5WorkspaceTrustRepairComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations
                .push(M5WorkspaceTrustRepairComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5WorkspaceTrustRepairComponentMatrixPacket,
    violations: &mut Vec<M5WorkspaceTrustRepairComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.workspace_trust_banner_shows_grant_source_and_scope,
        review.trust_fact_grid_shows_grant_scope_capability_root_together,
        review.trust_elevation_sheet_shows_grant_source_and_scope_change,
        review.restricted_capability_row_shows_narrowed_capability,
        review.root_trust_strip_shows_per_root_trust,
        review.repair_transaction_preview_card_shows_targets_checkpoint_reversal,
        review.rollback_class_strip_shows_reversal_class_and_checkpoint,
        review.repair_result_receipt_row_shows_outcome_and_followup,
        review.no_trust_surface_implies_blanket_approval,
        review.grant_source_and_policy_epoch_always_explicit,
        review.checkpoint_absence_never_hidden,
        review.reversal_outcomes_never_collapsed_into_generic_success,
        review.partial_success_never_shown_as_complete,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_trust_repair_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5WorkspaceTrustRepairComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5WorkspaceTrustRepairComponentMatrixPacket,
    violations: &mut Vec<M5WorkspaceTrustRepairComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.trust_surfaces_consume_grant_source_vocabulary,
        projection.settings_and_doctor_consume_capability_narrow_vocabulary,
        projection.safe_mode_consumes_root_trust_vocabulary,
        projection.repair_surfaces_consume_reversal_class_vocabulary,
        projection.guided_repair_consumes_checkpoint_vocabulary,
        projection.support_export_reads_single_trust_repair_source,
    ] {
        if !ok {
            violations
                .push(M5WorkspaceTrustRepairComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5WorkspaceTrustRepairComponentMatrixPacket,
    violations: &mut Vec<M5WorkspaceTrustRepairComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5WorkspaceTrustRepairComponentMatrixPacket,
    violations: &mut Vec<M5WorkspaceTrustRepairComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5WorkspaceTrustRepairComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled
/// vocabulary deliberately uses trust / repair words; what is rejected is a raw secret *value*
/// shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
