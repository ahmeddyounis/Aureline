//! Restore provenance records and fidelity projections for shell recovery.
//!
//! The module is the shell-side implementation of the shared restore
//! provenance contract. It records the artifact that was restored, the
//! resulting fidelity class, missing-dependency placeholder cards, schema
//! migration notes, preserved compare/export handles, and no-rerun downgrade
//! labels for terminal, task, debug, notebook, preview, and remote-backed
//! surfaces.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_recovery::session_restore::proposal::{
    RestoreProposal, RestoreProposalPanePlan, RestoreProposalPlanKind,
};
use aureline_recovery::session_restore::records::{
    RestoreClass as RecoveryRestoreClass, SurfaceClass as RecoverySurfaceClass,
    SurfaceRole as RecoverySurfaceRole,
};

/// Schema version for shell-emitted restore provenance records.
pub const RESTORE_PROVENANCE_SCHEMA_VERSION: u32 = 1;

/// Record kind for the shared restore provenance and placeholder payload.
pub const RESTORE_PROVENANCE_RECORD_KIND: &str = "state_restore_provenance_and_placeholder_record";

/// Human-facing surface that consumes the same provenance record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreTruthSurface {
    /// Startup recovery card shown before or during session restore.
    StartupRecovery,
    /// Restore summary shown after the restore decision is applied.
    RestoreSummary,
    /// Diagnostics or Project Doctor view.
    Diagnostics,
    /// Support export or support preview row.
    SupportExport,
}

impl RestoreTruthSurface {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartupRecovery => "startup_recovery",
            Self::RestoreSummary => "restore_summary",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
        }
    }
}

/// Source event family that produced a restore provenance record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreSourceEventClass {
    /// Restore came from an automatic checkpoint.
    AutoCheckpoint,
    /// Restore came from an explicit user export.
    ManualExport,
    /// Restore came from a backup artifact.
    Backup,
    /// Restore came from a sync snapshot.
    Sync,
    /// Restore came from an import flow.
    Import,
}

impl RestoreSourceEventClass {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutoCheckpoint => "auto_checkpoint",
            Self::ManualExport => "manual_export",
            Self::Backup => "backup",
            Self::Sync => "sync",
            Self::Import => "import",
        }
    }
}

/// Artifact family consumed by a restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreArtifactFamily {
    /// Workspace portable-state package.
    PortableStatePackage,
    /// Portable profile artifact.
    PortableProfile,
    /// Profile sync snapshot.
    ProfileSyncSnapshot,
    /// Layout or window-topology snapshot.
    LayoutSnapshot,
    /// Session-restore manifest.
    SessionRestoreManifest,
    /// Workspace manifest bundle.
    WorkspaceManifestBundle,
    /// Support recovery bundle.
    SupportRecoveryBundle,
    /// Import bridge bundle.
    ImportBridgeBundle,
}

impl RestoreArtifactFamily {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PortableStatePackage => "portable_state_package",
            Self::PortableProfile => "portable_profile",
            Self::ProfileSyncSnapshot => "profile_sync_snapshot",
            Self::LayoutSnapshot => "layout_snapshot",
            Self::SessionRestoreManifest => "session_restore_manifest",
            Self::WorkspaceManifestBundle => "workspace_manifest_bundle",
            Self::SupportRecoveryBundle => "support_recovery_bundle",
            Self::ImportBridgeBundle => "import_bridge_bundle",
        }
    }
}

/// Concrete source class for the artifact consumed by a restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreSourceClass {
    /// Aureline workspace portable-state package.
    AurelinePortableStatePackage,
    /// Aureline portable profile.
    AurelinePortableProfile,
    /// Aureline profile sync snapshot.
    AurelineProfileSyncSnapshot,
    /// Aureline layout snapshot.
    AurelineLayoutSnapshot,
    /// Aureline session-restore manifest.
    AurelineSessionRestoreManifest,
    /// Aureline workspace manifest bundle.
    AurelineWorkspaceManifestBundle,
    /// Support recovery bundle.
    SupportRecoveryBundle,
    /// Imported profile from a different IDE.
    ImportedCompetitorProfile,
    /// Imported layout bundle from a different IDE.
    ImportedCompetitorLayoutBundle,
    /// Mixed state export containing several source families.
    MixedStateExportBundle,
}

impl RestoreSourceClass {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AurelinePortableStatePackage => "aureline_portable_state_package",
            Self::AurelinePortableProfile => "aureline_portable_profile",
            Self::AurelineProfileSyncSnapshot => "aureline_profile_sync_snapshot",
            Self::AurelineLayoutSnapshot => "aureline_layout_snapshot",
            Self::AurelineSessionRestoreManifest => "aureline_session_restore_manifest",
            Self::AurelineWorkspaceManifestBundle => "aureline_workspace_manifest_bundle",
            Self::SupportRecoveryBundle => "support_recovery_bundle",
            Self::ImportedCompetitorProfile => "imported_competitor_profile",
            Self::ImportedCompetitorLayoutBundle => "imported_competitor_layout_bundle",
            Self::MixedStateExportBundle => "mixed_state_export_bundle",
        }
    }
}

/// Redaction posture for the provenance record itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreRedactionClass {
    /// No redaction needed.
    None,
    /// UI strings only.
    UiStringOnly,
    /// Values are redacted while preserving shape.
    RedactValuePreserveShape,
    /// Values are reduced to class labels.
    RedactToClassLabel,
    /// Record is excluded from export.
    ExcludeFromExport,
}

impl RestoreRedactionClass {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::UiStringOnly => "ui_string_only",
            Self::RedactValuePreserveShape => "redact_value_preserve_shape",
            Self::RedactToClassLabel => "redact_to_class_label",
            Self::ExcludeFromExport => "exclude_from_export",
        }
    }
}

/// Closed restore fidelity class shared by startup, diagnostics, and support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreFidelityClass {
    /// Exact restore with no translation, placeholder, or review downgrade.
    Exact,
    /// Compatible restore through a declared translation or adjustment.
    Compatible,
    /// Layout and context restored without live surface reattachment.
    LayoutOnly,
    /// User-authored drafts were recovered for compare/review.
    RecoveredDrafts,
    /// Only evidence, transcripts, snapshots, and provenance survived.
    EvidenceOnly,
}

impl RestoreFidelityClass {
    /// Maps the recovery crate restore class into the closed five-class vocabulary.
    pub const fn from_recovery_class(class: RecoveryRestoreClass) -> Self {
        match class {
            RecoveryRestoreClass::ExactRestore => Self::Exact,
            RecoveryRestoreClass::CompatibleRestore => Self::Compatible,
            RecoveryRestoreClass::LayoutOnly => Self::LayoutOnly,
            RecoveryRestoreClass::RecoveredDrafts => Self::RecoveredDrafts,
            RecoveryRestoreClass::EvidenceOnly | RecoveryRestoreClass::NoRestore => {
                Self::EvidenceOnly
            }
        }
    }

    /// Returns the restore-level value paired with this fidelity class.
    pub const fn restore_level(self) -> RestoreLevel {
        match self {
            Self::Exact => RestoreLevel::ExactRestore,
            Self::Compatible => RestoreLevel::CompatibleRestore,
            Self::LayoutOnly => RestoreLevel::LayoutOnly,
            Self::RecoveredDrafts => RestoreLevel::RecoveredDrafts,
            Self::EvidenceOnly => RestoreLevel::EvidenceOnly,
        }
    }

    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compatible => "compatible",
            Self::LayoutOnly => "layout_only",
            Self::RecoveredDrafts => "recovered_drafts",
            Self::EvidenceOnly => "evidence_only",
        }
    }

    /// Returns the controlled user-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Exact => "Exact restore",
            Self::Compatible => "Compatible restore",
            Self::LayoutOnly => "Layout only",
            Self::RecoveredDrafts => "Recovered drafts",
            Self::EvidenceOnly => "Evidence only",
        }
    }
}

/// Restore level paired one-to-one with [`RestoreFidelityClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreLevel {
    /// Exact restore level.
    ExactRestore,
    /// Compatible restore level.
    CompatibleRestore,
    /// Layout-only restore level.
    LayoutOnly,
    /// Recovered-drafts restore level.
    RecoveredDrafts,
    /// Evidence-only restore level.
    EvidenceOnly,
}

impl RestoreLevel {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::RecoveredDrafts => "recovered_drafts",
            Self::EvidenceOnly => "evidence_only",
        }
    }
}

/// Missing dependency class that requires a placeholder card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingDependencyClass {
    /// Extension or feature pack is absent.
    AbsentExtension,
    /// Remote target, kernel, host, or endpoint is absent.
    AbsentRemoteTarget,
    /// Permission, grant, or delegated approval was revoked.
    RevokedPermission,
    /// External service dependency is stale or unavailable.
    StaleServiceDependency,
    /// Workspace authority checkpoint is missing.
    MissingWorkspaceAuthority,
    /// Schema equivalence map is missing or refused.
    MissingSchemaEquivalenceMap,
}

impl MissingDependencyClass {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AbsentExtension => "absent_extension",
            Self::AbsentRemoteTarget => "absent_remote_target",
            Self::RevokedPermission => "revoked_permission",
            Self::StaleServiceDependency => "stale_service_dependency",
            Self::MissingWorkspaceAuthority => "missing_workspace_authority",
            Self::MissingSchemaEquivalenceMap => "missing_schema_equivalence_map",
        }
    }
}

/// Original pane role preserved by a missing-dependency placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservedPaneRole {
    /// Editor pane.
    Editor,
    /// Diff pane.
    Diff,
    /// Terminal pane.
    Terminal,
    /// Debugger pane.
    Debugger,
    /// Notebook pane.
    Notebook,
    /// Search pane.
    Search,
    /// Problems pane.
    Problems,
    /// Source-control pane.
    Scm,
    /// Documentation pane.
    Docs,
    /// Preview pane.
    Preview,
    /// AI panel.
    AiPanel,
    /// Explorer pane.
    Explorer,
    /// Test pane.
    Test,
    /// Custom extension pane.
    CustomExtension,
}

impl PreservedPaneRole {
    /// Converts the recovery crate pane role when it can be represented in provenance.
    pub const fn from_recovery_role(role: RecoverySurfaceRole) -> Option<Self> {
        match role {
            RecoverySurfaceRole::Editor => Some(Self::Editor),
            RecoverySurfaceRole::Diff => Some(Self::Diff),
            RecoverySurfaceRole::Terminal => Some(Self::Terminal),
            RecoverySurfaceRole::Debugger => Some(Self::Debugger),
            RecoverySurfaceRole::Notebook => Some(Self::Notebook),
            RecoverySurfaceRole::Search => Some(Self::Search),
            RecoverySurfaceRole::Problems => Some(Self::Problems),
            RecoverySurfaceRole::Scm => Some(Self::Scm),
            RecoverySurfaceRole::Docs => Some(Self::Docs),
            RecoverySurfaceRole::Preview => Some(Self::Preview),
            RecoverySurfaceRole::AiPanel => Some(Self::AiPanel),
            RecoverySurfaceRole::Explorer => Some(Self::Explorer),
            RecoverySurfaceRole::Test => Some(Self::Test),
            RecoverySurfaceRole::CustomExtension => Some(Self::CustomExtension),
            RecoverySurfaceRole::Placeholder => None,
        }
    }

    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Diff => "diff",
            Self::Terminal => "terminal",
            Self::Debugger => "debugger",
            Self::Notebook => "notebook",
            Self::Search => "search",
            Self::Problems => "problems",
            Self::Scm => "scm",
            Self::Docs => "docs",
            Self::Preview => "preview",
            Self::AiPanel => "ai_panel",
            Self::Explorer => "explorer",
            Self::Test => "test",
            Self::CustomExtension => "custom_extension",
        }
    }
}

/// Original surface class preserved by a missing-dependency placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservedSurfaceClass {
    /// Text editor surface.
    TextEditor,
    /// Diff editor surface.
    DiffEditor,
    /// Terminal view surface.
    TerminalView,
    /// Debug view surface.
    DebugView,
    /// Notebook view surface.
    NotebookView,
    /// Search results surface.
    SearchResults,
    /// Problems panel surface.
    ProblemsPanel,
    /// Source-control panel surface.
    ScmPanel,
    /// Documentation browser surface.
    DocsBrowser,
    /// Preview canvas surface.
    PreviewCanvas,
    /// AI panel surface.
    AiPanel,
    /// Explorer tree surface.
    ExplorerTree,
    /// Test results surface.
    TestResults,
    /// Extension view surface.
    ExtensionView,
}

impl PreservedSurfaceClass {
    /// Converts the recovery crate surface class when it can be represented in provenance.
    pub const fn from_recovery_class(class: RecoverySurfaceClass) -> Option<Self> {
        match class {
            RecoverySurfaceClass::TextEditor => Some(Self::TextEditor),
            RecoverySurfaceClass::DiffEditor => Some(Self::DiffEditor),
            RecoverySurfaceClass::TerminalView => Some(Self::TerminalView),
            RecoverySurfaceClass::DebugView => Some(Self::DebugView),
            RecoverySurfaceClass::NotebookView => Some(Self::NotebookView),
            RecoverySurfaceClass::SearchResults => Some(Self::SearchResults),
            RecoverySurfaceClass::ProblemsPanel => Some(Self::ProblemsPanel),
            RecoverySurfaceClass::ScmPanel => Some(Self::ScmPanel),
            RecoverySurfaceClass::DocsBrowser => Some(Self::DocsBrowser),
            RecoverySurfaceClass::PreviewCanvas => Some(Self::PreviewCanvas),
            RecoverySurfaceClass::AiPanel => Some(Self::AiPanel),
            RecoverySurfaceClass::ExplorerTree => Some(Self::ExplorerTree),
            RecoverySurfaceClass::TestResults => Some(Self::TestResults),
            RecoverySurfaceClass::ExtensionView => Some(Self::ExtensionView),
            RecoverySurfaceClass::PlaceholderCard => None,
        }
    }

    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextEditor => "text_editor",
            Self::DiffEditor => "diff_editor",
            Self::TerminalView => "terminal_view",
            Self::DebugView => "debug_view",
            Self::NotebookView => "notebook_view",
            Self::SearchResults => "search_results",
            Self::ProblemsPanel => "problems_panel",
            Self::ScmPanel => "scm_panel",
            Self::DocsBrowser => "docs_browser",
            Self::PreviewCanvas => "preview_canvas",
            Self::AiPanel => "ai_panel",
            Self::ExplorerTree => "explorer_tree",
            Self::TestResults => "test_results",
            Self::ExtensionView => "extension_view",
        }
    }
}

/// Closed recovery action vocabulary for restore placeholders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreRecoveryAction {
    /// Try to hydrate again.
    RetryHydrate,
    /// Locate the missing extension.
    LocateExtension,
    /// Install the missing extension.
    InstallExtension,
    /// Open without the missing dependency.
    OpenWithout,
    /// Reconnect the remote target.
    ReconnectRemote,
    /// Reauthenticate or unlock credentials.
    Reauthenticate,
    /// Open in restricted mode.
    OpenRestricted,
    /// Rebind an existing session after review.
    RebindExistingSession,
    /// Rerun explicitly.
    RerunExplicitly,
    /// Compare with a preserved artifact.
    CompareWithPreservedArtifact,
    /// Open repair instructions.
    OpenRepairInstructions,
    /// Escalate to manual repair.
    EscalateToManualRepair,
    /// Export retained evidence.
    ExportEvidence,
    /// Remove the pane deliberately.
    RemovePane,
}

impl RestoreRecoveryAction {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetryHydrate => "retry_hydrate",
            Self::LocateExtension => "locate_extension",
            Self::InstallExtension => "install_extension",
            Self::OpenWithout => "open_without",
            Self::ReconnectRemote => "reconnect_remote",
            Self::Reauthenticate => "reauthenticate",
            Self::OpenRestricted => "open_restricted",
            Self::RebindExistingSession => "rebind_existing_session",
            Self::RerunExplicitly => "rerun_explicitly",
            Self::CompareWithPreservedArtifact => "compare_with_preserved_artifact",
            Self::OpenRepairInstructions => "open_repair_instructions",
            Self::EscalateToManualRepair => "escalate_to_manual_repair",
            Self::ExportEvidence => "export_evidence",
            Self::RemovePane => "remove_pane",
        }
    }
}

/// Schema migration outcome attached to the provenance record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaMigrationNoteClass {
    /// Source and target schemas matched.
    NoMigrationRequired,
    /// Translation succeeded through a declared equivalence map.
    SchemaTranslationApplied,
    /// Translation changed meaning and preserved a prior artifact.
    SchemaMeaningChanged,
    /// Restore is blocked pending human review.
    BlockedPendingReview,
    /// Producer refused to emit a downgraded schema.
    ProducerSchemaDowngradeRefused,
}

impl SchemaMigrationNoteClass {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMigrationRequired => "no_migration_required",
            Self::SchemaTranslationApplied => "schema_translation_applied",
            Self::SchemaMeaningChanged => "schema_meaning_changed",
            Self::BlockedPendingReview => "blocked_pending_review",
            Self::ProducerSchemaDowngradeRefused => "producer_schema_downgrade_refused",
        }
    }
}

/// Intentional exclusion class kept separate from missing dependencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentionalExclusionClass {
    /// Live PTY, kernel, debug, remote, or session authority.
    NonPortableLiveAuthority,
    /// Secret or credential material.
    SecretOrCredentialMaterial,
    /// Delegated approval or ticket.
    DelegatedApprovalOrTicket,
    /// Machine-unique handle.
    MachineUniqueHandle,
    /// Raw provider payload.
    RawProviderPayload,
    /// Raw path or URL.
    RawPathOrUrl,
    /// Raw command line.
    RawCommandLine,
    /// Raw log or trace.
    RawLogOrTrace,
    /// Raw source content.
    RawSourceContent,
    /// User declined to include the class.
    UserDeclined,
    /// Policy excluded the class.
    PolicyExcluded,
}

impl IntentionalExclusionClass {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonPortableLiveAuthority => "non_portable_live_authority",
            Self::SecretOrCredentialMaterial => "secret_or_credential_material",
            Self::DelegatedApprovalOrTicket => "delegated_approval_or_ticket",
            Self::MachineUniqueHandle => "machine_unique_handle",
            Self::RawProviderPayload => "raw_provider_payload",
            Self::RawPathOrUrl => "raw_path_or_url",
            Self::RawCommandLine => "raw_command_line",
            Self::RawLogOrTrace => "raw_log_or_trace",
            Self::RawSourceContent => "raw_source_content",
            Self::UserDeclined => "user_declined",
            Self::PolicyExcluded => "policy_excluded",
        }
    }
}

/// Reason a previous artifact was preserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservationReason {
    /// Schema meaning changed.
    SchemaMeaningChanged,
    /// Artifact was preserved for downgraded compare.
    DowngradedForCompare,
    /// Artifact was preserved for support export.
    SupportExport,
    /// Artifact was preserved for manual repair escalation.
    ManualRepairEscalation,
    /// Artifact was retained for rollback.
    RollbackRetained,
}

impl PreservationReason {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaMeaningChanged => "schema_meaning_changed",
            Self::DowngradedForCompare => "downgraded_for_compare",
            Self::SupportExport => "support_export",
            Self::ManualRepairEscalation => "manual_repair_escalation",
            Self::RollbackRetained => "rollback_retained",
        }
    }
}

/// Label for a surface that reopens without live runtime reattachment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreWithoutRerunLabel {
    /// Transcript or snapshot restored, command not rerun.
    TranscriptOnly,
    /// Reconnect is required before live authority resumes.
    ReconnectRequired,
    /// Prior session ended and was not reattached.
    SessionEnded,
    /// Explicit rerun is required before live output resumes.
    RerunRequired,
    /// Credential unlock is required before resume.
    CredentialUnlockNeeded,
    /// Layout was adjusted after display topology changed.
    LayoutAdjustedForDisplayChange,
    /// Resume needs review after wake, reconnect, or target loss.
    ResumeReviewNeeded,
}

impl RestoreWithoutRerunLabel {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TranscriptOnly => "transcript_only",
            Self::ReconnectRequired => "reconnect_required",
            Self::SessionEnded => "session_ended",
            Self::RerunRequired => "rerun_required",
            Self::CredentialUnlockNeeded => "credential_unlock_needed",
            Self::LayoutAdjustedForDisplayChange => "layout_adjusted_for_display_change",
            Self::ResumeReviewNeeded => "resume_review_needed",
        }
    }

    /// Returns the controlled display label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::TranscriptOnly => "transcript restored; command not rerun",
            Self::ReconnectRequired => "reconnect required",
            Self::SessionEnded => "session ended; command not rerun",
            Self::RerunRequired => "rerun required",
            Self::CredentialUnlockNeeded => "credential unlock needed",
            Self::LayoutAdjustedForDisplayChange => "layout adjusted for display change",
            Self::ResumeReviewNeeded => "resume review needed",
        }
    }
}

/// Producer channel for the provenance record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProducerChannel {
    /// Experimental channel.
    Experimental,
    /// Beta channel.
    Beta,
    /// Stable channel.
    Stable,
    /// Long-term-support channel.
    Lts,
}

/// Producer platform class for the provenance record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProducerPlatformClass {
    /// macOS producer.
    Macos,
    /// Windows producer.
    Windows,
    /// Linux producer.
    Linux,
    /// Container producer.
    Container,
    /// Remote-agent producer.
    RemoteAgent,
    /// Managed-cloud producer.
    ManagedCloud,
    /// Other producer class.
    Other,
}

/// Producer build stamp carried on restore provenance records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProducerBuildStamp {
    /// Producer name.
    pub producer_name: String,
    /// Producer version.
    pub producer_version: String,
    /// Producer channel.
    pub producer_channel: Option<ProducerChannel>,
    /// Producer platform class.
    pub producer_platform_class: Option<ProducerPlatformClass>,
    /// Opaque producer instance handle.
    pub producer_instance_handle: Option<String>,
}

/// Source artifact consumed by the restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProvenanceSource {
    /// Artifact family.
    pub artifact_family: RestoreArtifactFamily,
    /// Concrete source class.
    pub source_class: RestoreSourceClass,
    /// Opaque artifact ref.
    pub source_artifact_ref: String,
}

/// One missing-dependency placeholder card preserving a pane slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissingDependencyPlaceholderCard {
    /// Stable placeholder card id.
    pub placeholder_card_id: String,
    /// Missing dependency class.
    pub missing_dependency_class: MissingDependencyClass,
    /// Opaque missing dependency ref.
    pub missing_dependency_ref: Option<String>,
    /// Original pane id preserved by the placeholder.
    pub preserved_pane_id: String,
    /// Original pane role.
    pub preserved_pane_role: PreservedPaneRole,
    /// Original surface class.
    pub preserved_surface_class: PreservedSurfaceClass,
    /// Best-effort title hint.
    pub preserved_title_hint: Option<String>,
    /// Whether retained evidence exists behind the placeholder.
    pub evidence_retained: bool,
    /// Opaque evidence ref.
    pub evidence_ref: Option<String>,
    /// Closed recovery actions.
    pub recovery_actions: Vec<RestoreRecoveryAction>,
    /// Last-known redaction-aware provenance label.
    pub last_known_provenance_label: Option<String>,
    /// Redaction-aware note.
    pub note: String,
}

impl MissingDependencyPlaceholderCard {
    /// Builds an absent-remote placeholder for tests and restore summaries.
    pub fn absent_remote(
        placeholder_card_id: impl Into<String>,
        preserved_pane_id: impl Into<String>,
        preserved_pane_role: PreservedPaneRole,
        preserved_surface_class: PreservedSurfaceClass,
        title: impl Into<String>,
        evidence_ref: impl Into<String>,
    ) -> Self {
        Self {
            placeholder_card_id: placeholder_card_id.into(),
            missing_dependency_class: MissingDependencyClass::AbsentRemoteTarget,
            missing_dependency_ref: None,
            preserved_pane_id: preserved_pane_id.into(),
            preserved_pane_role,
            preserved_surface_class,
            preserved_title_hint: Some(title.into()),
            evidence_retained: true,
            evidence_ref: Some(evidence_ref.into()),
            recovery_actions: vec![
                RestoreRecoveryAction::ReconnectRemote,
                RestoreRecoveryAction::Reauthenticate,
                RestoreRecoveryAction::ExportEvidence,
                RestoreRecoveryAction::RemovePane,
            ],
            last_known_provenance_label: Some("remote target".to_string()),
            note: "Remote target was unavailable; evidence retained behind a placeholder."
                .to_string(),
        }
    }
}

/// Schema migration note with compare and rollback handles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaMigrationNote {
    /// Migration note class.
    pub note_class: SchemaMigrationNoteClass,
    /// Source schema version.
    pub source_schema_version: String,
    /// Target schema version.
    pub target_schema_version: String,
    /// Equivalence map ref.
    pub equivalence_map_ref: Option<String>,
    /// Rollback checkpoint ref.
    pub rollback_checkpoint_ref: Option<String>,
    /// Preserved prior artifact ref.
    pub preserved_prior_artifact_ref: Option<String>,
    /// Redaction-aware note.
    pub note: String,
}

impl SchemaMigrationNote {
    /// Builds a no-migration-required note.
    pub fn no_migration_required(
        source_schema_version: impl Into<String>,
        target_schema_version: impl Into<String>,
    ) -> Self {
        Self {
            note_class: SchemaMigrationNoteClass::NoMigrationRequired,
            source_schema_version: source_schema_version.into(),
            target_schema_version: target_schema_version.into(),
            equivalence_map_ref: None,
            rollback_checkpoint_ref: None,
            preserved_prior_artifact_ref: None,
            note: "Source and target schema versions matched.".to_string(),
        }
    }
}

/// Intentional exclusion row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentionalExclusionRow {
    /// Stable exclusion id.
    pub exclusion_id: String,
    /// Exclusion class.
    pub exclusion_class: IntentionalExclusionClass,
    /// Redaction-aware scope note.
    pub scope_note: String,
}

/// Prior artifact preserved for compare, export, rollback, or support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservedPriorArtifactRecord {
    /// Preserved artifact ref.
    pub artifact_ref: String,
    /// Artifact family.
    pub artifact_family: RestoreArtifactFamily,
    /// Preservation reason.
    pub preservation_reason: PreservationReason,
    /// Redaction class.
    pub redaction_class: RestoreRedactionClass,
    /// Compare ref.
    pub compare_ref: Option<String>,
    /// Export ref.
    pub export_ref: Option<String>,
    /// Rollback note.
    pub rollback_note: String,
    /// Redaction-aware note.
    pub note: String,
}

/// One surface downgrade that preserves context without rerunning live work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreWithoutRerunDowngrade {
    /// Stable downgrade id.
    pub downgrade_id: String,
    /// Pane id the downgrade applies to.
    pub pane_id: String,
    /// Original pane role.
    pub surface_role: PreservedPaneRole,
    /// Original surface class.
    pub surface_class: PreservedSurfaceClass,
    /// Controlled no-rerun label.
    pub label: RestoreWithoutRerunLabel,
    /// Whether the runtime actually survived.
    pub runtime_survived: bool,
    /// Whether command rerun is forbidden until explicit user action.
    pub command_rerun_forbidden: bool,
    /// Whether authority reacquire is forbidden until explicit user action.
    pub authority_reacquire_forbidden: bool,
    /// Opaque evidence ref retained for compare/export.
    pub evidence_ref: Option<String>,
    /// Redaction-aware note.
    pub note: String,
}

impl RestoreWithoutRerunDowngrade {
    /// Builds a no-rerun downgrade row from a blocked restore pane plan.
    pub fn from_pane_plan(plan: &RestoreProposalPanePlan) -> Option<Self> {
        if plan.plan_kind != RestoreProposalPlanKind::BlockedSideEffectful {
            return None;
        }
        let role = PreservedPaneRole::from_recovery_role(plan.surface_role)?;
        let class = PreservedSurfaceClass::from_recovery_class(plan.surface_class)?;
        let label = no_rerun_label_for_role(role);
        Some(Self {
            downgrade_id: format!("restore-without-rerun:{}", stable_token(&plan.pane_id)),
            pane_id: plan.pane_id.clone(),
            surface_role: role,
            surface_class: class,
            label,
            runtime_survived: false,
            command_rerun_forbidden: true,
            authority_reacquire_forbidden: true,
            evidence_ref: Some(format!("restore-evidence:{}", stable_token(&plan.pane_id))),
            note: label.display_label().to_string(),
        })
    }
}

/// Input required to materialize a provenance record from a restore proposal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreProvenanceInput {
    /// Stable provenance id.
    pub restore_provenance_id: String,
    /// Source event class.
    pub source_event_class: RestoreSourceEventClass,
    /// Source artifact summary.
    pub source: RestoreProvenanceSource,
    /// Record creation time.
    pub created_at: String,
    /// Producer build.
    pub producer_build: RestoreProducerBuildStamp,
    /// Source schema version.
    pub source_schema_version: String,
    /// Target schema version for migration notes.
    pub target_schema_version: String,
    /// Redaction class.
    pub redaction_class: RestoreRedactionClass,
    /// Top-level rollback checkpoint ref.
    pub rollback_checkpoint_ref: Option<String>,
    /// Top-level equivalence map ref.
    pub equivalence_map_ref: Option<String>,
    /// Top-level compare ref.
    pub compare_ref: Option<String>,
    /// Top-level export ref.
    pub export_ref: Option<String>,
    /// Placeholder cards supplied by restore, migration, or support flows.
    pub missing_dependency_placeholder_cards: Vec<MissingDependencyPlaceholderCard>,
    /// Schema migration note, if not the default no-migration note.
    pub schema_migration_note: Option<SchemaMigrationNote>,
    /// Preserved prior artifacts.
    pub preserved_prior_artifacts: Vec<PreservedPriorArtifactRecord>,
    /// Intentional exclusions.
    pub intentional_exclusions: Vec<IntentionalExclusionRow>,
    /// Extra no-rerun downgrades not derivable from the proposal.
    pub restore_without_rerun_downgrades: Vec<RestoreWithoutRerunDowngrade>,
    /// Record emission time.
    pub emitted_at: String,
    /// Redaction-aware note.
    pub notes: Option<String>,
}

impl RestoreProvenanceInput {
    /// Builds input with required identity fields and no optional rows.
    pub fn new(
        restore_provenance_id: impl Into<String>,
        source_event_class: RestoreSourceEventClass,
        source: RestoreProvenanceSource,
        created_at: impl Into<String>,
        producer_build: RestoreProducerBuildStamp,
        source_schema_version: impl Into<String>,
        target_schema_version: impl Into<String>,
        redaction_class: RestoreRedactionClass,
        emitted_at: impl Into<String>,
    ) -> Self {
        Self {
            restore_provenance_id: restore_provenance_id.into(),
            source_event_class,
            source,
            created_at: created_at.into(),
            producer_build,
            source_schema_version: source_schema_version.into(),
            target_schema_version: target_schema_version.into(),
            redaction_class,
            rollback_checkpoint_ref: None,
            equivalence_map_ref: None,
            compare_ref: None,
            export_ref: None,
            missing_dependency_placeholder_cards: Vec::new(),
            schema_migration_note: None,
            preserved_prior_artifacts: Vec::new(),
            intentional_exclusions: Vec::new(),
            restore_without_rerun_downgrades: Vec::new(),
            emitted_at: emitted_at.into(),
            notes: None,
        }
    }
}

/// Restore provenance and placeholder record shared by restore and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProvenanceRecord {
    /// Optional schema hint carried by fixtures.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Optional fixture metadata.
    #[serde(
        rename = "__fixture__",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture: Option<serde_json::Value>,
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub restore_provenance_schema_version: u32,
    /// Stable provenance id.
    pub restore_provenance_id: String,
    /// Source event class such as auto checkpoint, manual export, backup, sync, or import.
    pub source_event_class: RestoreSourceEventClass,
    /// Source artifact consumed by the restore.
    pub source: RestoreProvenanceSource,
    /// Record creation time.
    pub created_at: String,
    /// Producer build.
    pub producer_build: RestoreProducerBuildStamp,
    /// Source schema version.
    pub source_schema_version: String,
    /// Redaction class.
    pub redaction_class: RestoreRedactionClass,
    /// Resulting fidelity.
    pub resulting_fidelity: RestoreFidelityClass,
    /// Restore level paired with fidelity.
    pub restore_level: RestoreLevel,
    /// Aggregate missing dependency classes.
    pub missing_dependency_classes: Vec<MissingDependencyClass>,
    /// Missing dependency placeholder cards.
    pub missing_dependency_placeholder_cards: Vec<MissingDependencyPlaceholderCard>,
    /// Schema migration note.
    pub schema_migration_note: SchemaMigrationNote,
    /// Preserved prior artifacts.
    pub preserved_prior_artifacts: Vec<PreservedPriorArtifactRecord>,
    /// Intentional exclusions.
    pub intentional_exclusions: Vec<IntentionalExclusionRow>,
    /// Top-level rollback checkpoint ref.
    pub rollback_checkpoint_ref: Option<String>,
    /// Top-level equivalence map ref.
    pub equivalence_map_ref: Option<String>,
    /// Top-level compare ref.
    pub compare_ref: Option<String>,
    /// Top-level export ref.
    pub export_ref: Option<String>,
    /// No-rerun downgrade rows for session-scoped live surfaces.
    #[serde(default)]
    pub restore_without_rerun_downgrades: Vec<RestoreWithoutRerunDowngrade>,
    /// Record emission time.
    pub emitted_at: String,
    /// Redaction-aware note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl RestoreProvenanceRecord {
    /// Builds a provenance record from a pre-rehydration restore proposal.
    pub fn from_proposal(
        proposal: &RestoreProposal,
        input: RestoreProvenanceInput,
    ) -> Result<Self, RestoreProvenanceValidationError> {
        let fidelity = RestoreFidelityClass::from_recovery_class(proposal.restore_class);
        let mut no_rerun_rows = proposal
            .pane_plans
            .iter()
            .filter_map(RestoreWithoutRerunDowngrade::from_pane_plan)
            .collect::<Vec<_>>();
        no_rerun_rows.extend(input.restore_without_rerun_downgrades);

        let mut record = Self {
            schema: None,
            fixture: None,
            record_kind: RESTORE_PROVENANCE_RECORD_KIND.to_string(),
            restore_provenance_schema_version: RESTORE_PROVENANCE_SCHEMA_VERSION,
            restore_provenance_id: input.restore_provenance_id,
            source_event_class: input.source_event_class,
            source: input.source,
            created_at: input.created_at,
            producer_build: input.producer_build,
            source_schema_version: input.source_schema_version.clone(),
            redaction_class: input.redaction_class,
            resulting_fidelity: fidelity,
            restore_level: fidelity.restore_level(),
            missing_dependency_classes: Vec::new(),
            missing_dependency_placeholder_cards: input.missing_dependency_placeholder_cards,
            schema_migration_note: input.schema_migration_note.unwrap_or_else(|| {
                SchemaMigrationNote::no_migration_required(
                    input.source_schema_version,
                    input.target_schema_version,
                )
            }),
            preserved_prior_artifacts: input.preserved_prior_artifacts,
            intentional_exclusions: input.intentional_exclusions,
            rollback_checkpoint_ref: input.rollback_checkpoint_ref,
            equivalence_map_ref: input.equivalence_map_ref,
            compare_ref: input.compare_ref,
            export_ref: input.export_ref,
            restore_without_rerun_downgrades: no_rerun_rows,
            emitted_at: input.emitted_at,
            notes: input.notes,
        };
        record.missing_dependency_classes =
            missing_classes_from_cards(&record.missing_dependency_placeholder_cards);
        record.validate()?;
        Ok(record)
    }

    /// Validates fidelity, placeholder, migration, and no-rerun invariants.
    pub fn validate(&self) -> Result<(), RestoreProvenanceValidationError> {
        if self.record_kind != RESTORE_PROVENANCE_RECORD_KIND {
            return Err(RestoreProvenanceValidationError::WrongRecordKind);
        }
        if self.restore_provenance_schema_version != RESTORE_PROVENANCE_SCHEMA_VERSION {
            return Err(RestoreProvenanceValidationError::WrongSchemaVersion {
                expected: RESTORE_PROVENANCE_SCHEMA_VERSION,
                actual: self.restore_provenance_schema_version,
            });
        }
        if self.restore_level != self.resulting_fidelity.restore_level() {
            return Err(RestoreProvenanceValidationError::RestoreLevelMismatch {
                fidelity: self.resulting_fidelity,
                restore_level: self.restore_level,
            });
        }

        let card_classes = missing_classes_from_cards(&self.missing_dependency_placeholder_cards);
        if card_classes != self.missing_dependency_classes {
            return Err(RestoreProvenanceValidationError::MissingDependencyClassMismatch);
        }
        for card in &self.missing_dependency_placeholder_cards {
            if card.recovery_actions.is_empty() {
                return Err(RestoreProvenanceValidationError::PlaceholderMissingAction {
                    placeholder_card_id: card.placeholder_card_id.clone(),
                });
            }
        }

        match self.resulting_fidelity {
            RestoreFidelityClass::Exact => {
                if !self.missing_dependency_classes.is_empty()
                    || !self.missing_dependency_placeholder_cards.is_empty()
                    || !self.restore_without_rerun_downgrades.is_empty()
                    || self.rollback_checkpoint_ref.is_some()
                    || self.equivalence_map_ref.is_some()
                {
                    return Err(RestoreProvenanceValidationError::ExactRestoreHasDowngrade);
                }
            }
            RestoreFidelityClass::Compatible => {
                require_ref("rollback_checkpoint_ref", &self.rollback_checkpoint_ref)?;
                require_ref("equivalence_map_ref", &self.equivalence_map_ref)?;
                require_ref("compare_ref", &self.compare_ref)?;
                require_ref("export_ref", &self.export_ref)?;
            }
            RestoreFidelityClass::LayoutOnly => {
                require_ref("compare_ref", &self.compare_ref)?;
                require_ref("export_ref", &self.export_ref)?;
            }
            RestoreFidelityClass::RecoveredDrafts => {
                require_ref("rollback_checkpoint_ref", &self.rollback_checkpoint_ref)?;
                require_ref("compare_ref", &self.compare_ref)?;
                require_ref("export_ref", &self.export_ref)?;
                if self.preserved_prior_artifacts.is_empty() {
                    return Err(
                        RestoreProvenanceValidationError::MissingPreservedPriorArtifacts {
                            fidelity: self.resulting_fidelity,
                        },
                    );
                }
            }
            RestoreFidelityClass::EvidenceOnly => {
                require_ref("rollback_checkpoint_ref", &self.rollback_checkpoint_ref)?;
                require_ref("compare_ref", &self.compare_ref)?;
                require_ref("export_ref", &self.export_ref)?;
            }
        }

        match self.schema_migration_note.note_class {
            SchemaMigrationNoteClass::SchemaTranslationApplied => {
                require_ref(
                    "schema_migration_note.equivalence_map_ref",
                    &self.schema_migration_note.equivalence_map_ref,
                )?;
                require_ref(
                    "schema_migration_note.rollback_checkpoint_ref",
                    &self.schema_migration_note.rollback_checkpoint_ref,
                )?;
            }
            SchemaMigrationNoteClass::SchemaMeaningChanged => {
                require_ref(
                    "schema_migration_note.equivalence_map_ref",
                    &self.schema_migration_note.equivalence_map_ref,
                )?;
                require_ref(
                    "schema_migration_note.rollback_checkpoint_ref",
                    &self.schema_migration_note.rollback_checkpoint_ref,
                )?;
                require_ref(
                    "schema_migration_note.preserved_prior_artifact_ref",
                    &self.schema_migration_note.preserved_prior_artifact_ref,
                )?;
                if self.preserved_prior_artifacts.is_empty() {
                    return Err(
                        RestoreProvenanceValidationError::MissingPreservedPriorArtifacts {
                            fidelity: self.resulting_fidelity,
                        },
                    );
                }
            }
            SchemaMigrationNoteClass::NoMigrationRequired
            | SchemaMigrationNoteClass::BlockedPendingReview
            | SchemaMigrationNoteClass::ProducerSchemaDowngradeRefused => {}
        }

        for row in &self.restore_without_rerun_downgrades {
            if !row.runtime_survived
                && (!row.command_rerun_forbidden || !row.authority_reacquire_forbidden)
            {
                return Err(
                    RestoreProvenanceValidationError::NoRerunDowngradeClaimsLive {
                        downgrade_id: row.downgrade_id.clone(),
                    },
                );
            }
        }
        Ok(())
    }

    /// Builds the common surface projections for startup, summary, diagnostics, and support export.
    pub fn surface_projections(&self) -> Vec<RestoreProvenanceSurfaceProjection> {
        [
            RestoreTruthSurface::StartupRecovery,
            RestoreTruthSurface::RestoreSummary,
            RestoreTruthSurface::Diagnostics,
            RestoreTruthSurface::SupportExport,
        ]
        .iter()
        .map(|surface| self.surface_projection(*surface))
        .collect()
    }

    /// Builds one surface projection without changing the underlying record.
    pub fn surface_projection(
        &self,
        surface: RestoreTruthSurface,
    ) -> RestoreProvenanceSurfaceProjection {
        let downgrade_labels = self
            .restore_without_rerun_downgrades
            .iter()
            .map(|row| row.label)
            .collect::<Vec<_>>();
        RestoreProvenanceSurfaceProjection {
            record_kind: "restore_provenance_surface_projection".to_string(),
            surface,
            restore_provenance_id: self.restore_provenance_id.clone(),
            source_event_class: self.source_event_class,
            fidelity: self.resulting_fidelity,
            restore_level: self.restore_level,
            missing_dependency_classes: self.missing_dependency_classes.clone(),
            missing_dependency_count: self.missing_dependency_placeholder_cards.len(),
            restore_without_rerun_labels: downgrade_labels,
            compare_ref: self.compare_ref.clone(),
            export_ref: self.export_ref.clone(),
            summary_line: self.summary_line(),
        }
    }

    /// Renders a concise status line suitable for restore summaries and support rows.
    pub fn summary_line(&self) -> String {
        let missing = self
            .missing_dependency_classes
            .iter()
            .map(|class| class.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let no_rerun = self
            .restore_without_rerun_downgrades
            .iter()
            .map(|row| row.label.display_label())
            .collect::<Vec<_>>()
            .join("; ");
        let missing_label = if missing.is_empty() {
            "none"
        } else {
            missing.as_str()
        };
        let no_rerun_label = if no_rerun.is_empty() {
            "none"
        } else {
            no_rerun.as_str()
        };
        format!(
            "restore_provenance={} source_event={} fidelity={} restore_level={} missing_dependencies={} no_rerun={}",
            self.restore_provenance_id,
            self.source_event_class.as_str(),
            self.resulting_fidelity.display_label(),
            self.restore_level.as_str(),
            missing_label,
            no_rerun_label,
        )
    }

    /// Returns a stable support-pack item id for this provenance record.
    pub fn support_pack_item_id(&self) -> String {
        format!(
            "support.item.restore_provenance.{}",
            stable_token(&self.restore_provenance_id)
        )
    }
}

/// Cross-surface projection of a restore provenance record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProvenanceSurfaceProjection {
    /// Projection record discriminator.
    pub record_kind: String,
    /// Surface consuming this projection.
    pub surface: RestoreTruthSurface,
    /// Shared provenance id.
    pub restore_provenance_id: String,
    /// Source event class.
    pub source_event_class: RestoreSourceEventClass,
    /// Controlled fidelity class.
    pub fidelity: RestoreFidelityClass,
    /// Restore level.
    pub restore_level: RestoreLevel,
    /// Missing dependency classes.
    pub missing_dependency_classes: Vec<MissingDependencyClass>,
    /// Number of placeholder cards.
    pub missing_dependency_count: usize,
    /// No-rerun labels.
    pub restore_without_rerun_labels: Vec<RestoreWithoutRerunLabel>,
    /// Compare ref.
    pub compare_ref: Option<String>,
    /// Export ref.
    pub export_ref: Option<String>,
    /// Summary line.
    pub summary_line: String,
}

/// Validation error for restore provenance records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreProvenanceValidationError {
    /// Record kind does not match the shared schema.
    WrongRecordKind,
    /// Schema version is not supported.
    WrongSchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// Fidelity and restore level disagree.
    RestoreLevelMismatch {
        /// Fidelity class.
        fidelity: RestoreFidelityClass,
        /// Restore level.
        restore_level: RestoreLevel,
    },
    /// Exact restore carried downgrade, placeholder, rollback, or equivalence state.
    ExactRestoreHasDowngrade,
    /// Required top-level or migration handle is missing.
    MissingRequiredRef {
        /// Field name.
        field: &'static str,
    },
    /// Preserved-prior-artifact rows are missing for a fidelity that requires them.
    MissingPreservedPriorArtifacts {
        /// Fidelity class.
        fidelity: RestoreFidelityClass,
    },
    /// Aggregate missing dependency classes do not match placeholder cards.
    MissingDependencyClassMismatch,
    /// Placeholder card has no recovery actions.
    PlaceholderMissingAction {
        /// Placeholder card id.
        placeholder_card_id: String,
    },
    /// A no-rerun downgrade row claims live behavior without the required guards.
    NoRerunDowngradeClaimsLive {
        /// Downgrade id.
        downgrade_id: String,
    },
}

impl fmt::Display for RestoreProvenanceValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind => write!(f, "wrong restore provenance record kind"),
            Self::WrongSchemaVersion { expected, actual } => {
                write!(f, "expected schema version {expected}, got {actual}")
            }
            Self::RestoreLevelMismatch {
                fidelity,
                restore_level,
            } => write!(
                f,
                "restore level {} does not match fidelity {}",
                restore_level.as_str(),
                fidelity.as_str()
            ),
            Self::ExactRestoreHasDowngrade => {
                write!(f, "exact restore cannot carry downgrade state")
            }
            Self::MissingRequiredRef { field } => write!(f, "missing required ref {field}"),
            Self::MissingPreservedPriorArtifacts { fidelity } => write!(
                f,
                "missing preserved prior artifacts for {}",
                fidelity.as_str()
            ),
            Self::MissingDependencyClassMismatch => write!(
                f,
                "missing dependency class inventory does not match placeholder cards"
            ),
            Self::PlaceholderMissingAction {
                placeholder_card_id,
            } => write!(
                f,
                "placeholder {placeholder_card_id} has no recovery actions"
            ),
            Self::NoRerunDowngradeClaimsLive { downgrade_id } => write!(
                f,
                "no-rerun downgrade {downgrade_id} lacks explicit rerun and authority guards"
            ),
        }
    }
}

impl std::error::Error for RestoreProvenanceValidationError {}

fn missing_classes_from_cards(
    cards: &[MissingDependencyPlaceholderCard],
) -> Vec<MissingDependencyClass> {
    cards
        .iter()
        .map(|card| card.missing_dependency_class)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn require_ref(
    field: &'static str,
    value: &Option<String>,
) -> Result<(), RestoreProvenanceValidationError> {
    if value.as_deref().unwrap_or("").trim().is_empty() {
        Err(RestoreProvenanceValidationError::MissingRequiredRef { field })
    } else {
        Ok(())
    }
}

fn stable_token(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}

fn no_rerun_label_for_role(role: PreservedPaneRole) -> RestoreWithoutRerunLabel {
    match role {
        PreservedPaneRole::Terminal => RestoreWithoutRerunLabel::TranscriptOnly,
        PreservedPaneRole::Debugger | PreservedPaneRole::Notebook => {
            RestoreWithoutRerunLabel::SessionEnded
        }
        PreservedPaneRole::Test
        | PreservedPaneRole::Preview
        | PreservedPaneRole::AiPanel
        | PreservedPaneRole::CustomExtension => RestoreWithoutRerunLabel::RerunRequired,
        PreservedPaneRole::Editor
        | PreservedPaneRole::Diff
        | PreservedPaneRole::Search
        | PreservedPaneRole::Problems
        | PreservedPaneRole::Scm
        | PreservedPaneRole::Docs
        | PreservedPaneRole::Explorer => RestoreWithoutRerunLabel::ResumeReviewNeeded,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_recovery::session_restore::records::{SurfaceClass, SurfaceRole};

    #[test]
    fn blocked_terminal_becomes_transcript_only_no_rerun_row() {
        let plan = RestoreProposalPanePlan {
            pane_id: "pane-terminal-0001".to_string(),
            surface_role: SurfaceRole::Terminal,
            surface_class: SurfaceClass::TerminalView,
            plan_kind: RestoreProposalPlanKind::BlockedSideEffectful,
            title_hint: Some("deploy".to_string()),
            restore_metadata: None,
            note: "side-effectful surface; never auto-rerun".to_string(),
        };

        let row = RestoreWithoutRerunDowngrade::from_pane_plan(&plan).expect("row");
        assert_eq!(row.label, RestoreWithoutRerunLabel::TranscriptOnly);
        assert!(!row.runtime_survived);
        assert!(row.command_rerun_forbidden);
        assert!(row.authority_reacquire_forbidden);
    }
}
