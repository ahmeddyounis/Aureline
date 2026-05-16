//! Beta launch and attach profile model.
//!
//! This module owns the runtime store for user-authored launch and attach
//! profiles in the beta run / debug surfaces. One [`LaunchProfile`] captures
//! how a task, test, or debug lane will reach a target: the canonical target
//! binding, the adapter binding, the environment-capsule binding, the
//! configured arguments, and the declared side-effect classes. Profiles are
//! never mutated in place — every change produces a new
//! [`LaunchProfileRevision`] whose parent points at the prior revision, so
//! edits are durable, diffable, and reversible. The store can mint a
//! [`LaunchProfilePreview`] against a freshly resolved
//! [`crate::execution_context::ExecutionContext`] so the shell, status
//! surfaces, and support export agree on the selected profile and the exact
//! resolved execution context before any dispatch.
//!
//! The reviewer-facing landing page is
//! [`/docs/runtime/m3/run_debug_profiles_beta.md`](../../../../docs/runtime/m3/run_debug_profiles_beta.md).
//! The cross-tool boundary schema is
//! [`/schemas/runtime/launch_profile.schema.json`](../../../../schemas/runtime/launch_profile.schema.json).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::execution_context::{ExecutionContext, ReachabilityState, SurfaceClass};

/// Integer schema version for launch / attach profile records.
pub const LAUNCH_PROFILE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for a stored profile snapshot.
pub const LAUNCH_PROFILE_RECORD_KIND: &str = "launch_profile_record";

/// Stable record-kind tag for one immutable revision of a profile.
pub const LAUNCH_PROFILE_REVISION_RECORD_KIND: &str = "launch_profile_revision_record";

/// Stable record-kind tag for one edit applied to a profile.
pub const LAUNCH_PROFILE_EDIT_RECORD_KIND: &str = "launch_profile_edit_record";

/// Stable record-kind tag for a preview of a profile against the current
/// execution context.
pub const LAUNCH_PROFILE_PREVIEW_RECORD_KIND: &str = "launch_profile_preview_record";

/// Stable record-kind tag for the support-export projection of profile state.
pub const LAUNCH_PROFILE_SUPPORT_EXPORT_RECORD_KIND: &str = "launch_profile_support_export_record";

/// Launch versus attach posture for a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchProfileMode {
    /// Adapter or task runner launches the target.
    Launch,
    /// Adapter attaches to an already-running target.
    Attach,
}

impl LaunchProfileMode {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Launch => "launch",
            Self::Attach => "attach",
        }
    }
}

/// Lane the profile feeds. Closed vocabulary so support exports and shell
/// rows never invent free-form labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchProfileKind {
    /// Task lane (build, package script, generic command).
    Task,
    /// Test lane.
    Test,
    /// Debug lane (DAP-backed launch or attach).
    Debug,
}

impl LaunchProfileKind {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Task => "task",
            Self::Test => "test",
            Self::Debug => "debug",
        }
    }

    /// Runtime surface expected for this lane.
    pub const fn expected_surface(self) -> SurfaceClass {
        match self {
            Self::Task => SurfaceClass::Task,
            Self::Test => SurfaceClass::Test,
            Self::Debug => SurfaceClass::Debug,
        }
    }
}

/// Side effects a profile may produce when dispatched. The vocabulary is
/// closed so previews disclose exactly what the user is about to authorize.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchProfileSideEffectClass {
    /// Profile spawns or relaunches the target process.
    TargetProcessSpawn,
    /// Profile attaches to an already-running target process.
    TargetProcessAttach,
    /// Profile writes to the workspace filesystem.
    WorkspaceFilesystemWrite,
    /// Profile opens outbound network connections.
    OutboundNetwork,
    /// Profile binds an inbound network listener (debug or test server).
    InboundNetworkListener,
    /// Profile mutates process environment variables beyond the capsule.
    ProcessEnvMutation,
    /// Profile hands work off to a managed or remote host.
    RemoteHostHandoff,
}

impl LaunchProfileSideEffectClass {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetProcessSpawn => "target_process_spawn",
            Self::TargetProcessAttach => "target_process_attach",
            Self::WorkspaceFilesystemWrite => "workspace_filesystem_write",
            Self::OutboundNetwork => "outbound_network",
            Self::InboundNetworkListener => "inbound_network_listener",
            Self::ProcessEnvMutation => "process_env_mutation",
            Self::RemoteHostHandoff => "remote_host_handoff",
        }
    }
}

/// Class of an edit applied to a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchProfileEditClass {
    /// Profile was created.
    Created,
    /// Profile display name changed.
    RenamedDisplayName,
    /// Profile launch / attach mode changed.
    ModeChanged,
    /// Profile target binding changed.
    TargetBindingChanged,
    /// Profile adapter binding changed.
    AdapterBindingChanged,
    /// Profile environment binding changed.
    EnvironmentBindingChanged,
    /// Profile arguments (program / args / cwd / attach pid) changed.
    ArgumentsChanged,
    /// Declared side-effect set changed.
    SideEffectsChanged,
    /// Profile rolled back to an earlier revision.
    RolledBack,
}

impl LaunchProfileEditClass {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::RenamedDisplayName => "renamed_display_name",
            Self::ModeChanged => "mode_changed",
            Self::TargetBindingChanged => "target_binding_changed",
            Self::AdapterBindingChanged => "adapter_binding_changed",
            Self::EnvironmentBindingChanged => "environment_binding_changed",
            Self::ArgumentsChanged => "arguments_changed",
            Self::SideEffectsChanged => "side_effects_changed",
            Self::RolledBack => "rolled_back",
        }
    }
}

/// State of a profile preview against the current execution context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchProfilePreviewState {
    /// Preview is consistent with the resolved context; dispatch may proceed.
    ReadyToDispatch,
    /// Preview discloses one or more drifts that the user must review first.
    DriftRequiresReview,
    /// The resolved current target is not reachable.
    TargetUnreachable,
    /// Profile is missing data required to dispatch.
    UnavailableInvalidConfig,
}

impl LaunchProfilePreviewState {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToDispatch => "ready_to_dispatch",
            Self::DriftRequiresReview => "drift_requires_review",
            Self::TargetUnreachable => "target_unreachable",
            Self::UnavailableInvalidConfig => "unavailable_invalid_config",
        }
    }

    /// Returns true when shell / status surfaces MUST disclose the preview
    /// to the user before dispatch.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::ReadyToDispatch)
    }
}

/// Reason a preview is marked invalid. Closed vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchProfileInvalidReason {
    /// Profile carries no canonical target id.
    MissingTargetBinding,
    /// Profile in `Debug` lane carries no adapter binding.
    MissingAdapterBinding,
    /// Attach-mode profile is missing the inferior process id.
    AttachMissingProcessId,
    /// Launch-mode profile is missing a program to launch.
    LaunchMissingProgram,
}

impl LaunchProfileInvalidReason {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingTargetBinding => "missing_target_binding",
            Self::MissingAdapterBinding => "missing_adapter_binding",
            Self::AttachMissingProcessId => "attach_missing_process_id",
            Self::LaunchMissingProgram => "launch_missing_program",
        }
    }
}

/// Error returned by the store when a request cannot be served.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaunchProfileStoreError {
    /// No profile is registered under the given id.
    ProfileNotFound { profile_id: String },
    /// A profile with the same id is already registered.
    DuplicateProfileId { profile_id: String },
    /// The named revision does not belong to the profile.
    RevisionNotFound {
        profile_id: String,
        revision_id: String,
    },
    /// The named profile has no current revision (deleted but retained).
    ProfileHasNoRevisions { profile_id: String },
}

impl std::fmt::Display for LaunchProfileStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProfileNotFound { profile_id } => {
                write!(f, "launch profile not found: {profile_id}")
            }
            Self::DuplicateProfileId { profile_id } => {
                write!(f, "launch profile already exists: {profile_id}")
            }
            Self::RevisionNotFound {
                profile_id,
                revision_id,
            } => write!(
                f,
                "revision {revision_id} not found on launch profile {profile_id}"
            ),
            Self::ProfileHasNoRevisions { profile_id } => {
                write!(f, "launch profile has no revisions: {profile_id}")
            }
        }
    }
}

impl std::error::Error for LaunchProfileStoreError {}

/// Stable reference to an execution-context target binding for a profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchProfileTargetBinding {
    /// Canonical target id from the shared execution context.
    pub canonical_target_id: String,
    /// Target-class token (mirrors the shared `TargetClass` vocabulary).
    pub target_class_token: String,
    /// Plain-language target label suitable for support export and previews.
    pub target_label: String,
    /// Resolved working directory recorded with the binding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    /// Workspace scope class token expected when this binding is dispatched.
    pub scope_class_token: String,
}

/// Adapter binding for debug-class profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchProfileAdapterBinding {
    /// Stable adapter id (e.g. `adapter:node:dap`).
    pub adapter_id: String,
    /// Plain-language adapter label.
    pub adapter_label: String,
    /// Transport-class token mirrored from
    /// [`crate::debug::DebugAdapterTransportClass`].
    pub transport_class_token: String,
    /// Requested DAP protocol version.
    pub requested_dap_protocol_version: String,
    /// Capability tokens the profile requires from the adapter.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_capability_tokens: Vec<String>,
}

/// Environment-capsule binding for a profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchProfileEnvironmentBinding {
    /// Capsule id this profile expects.
    pub capsule_id: String,
    /// Capsule content hash recorded when the binding was stored.
    pub capsule_hash: String,
    /// Sorted keys of declared environment overlays. Only key names are
    /// stored so support exports never leak overlay values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub declared_overlay_keys: Vec<String>,
}

/// Argument vector recorded with the profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchProfileArguments {
    /// Program path or task name to launch. Required for launch-mode profiles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
    /// Argument vector.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// Working directory override for the dispatch. When omitted, the
    /// resolved [`crate::execution_context::TargetIdentity::working_directory`]
    /// wins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    /// Inferior process id. Required for attach-mode profiles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attach_process_id: Option<u32>,
}

/// One stored profile snapshot. Profiles are versioned: every edit produces
/// a new [`LaunchProfileRevision`] that points at the prior revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchProfile {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable profile id (opaque, scoped to the workspace).
    pub profile_id: String,
    /// Workspace that owns the profile.
    pub workspace_id: String,
    /// Display name shown in the shell.
    pub display_name: String,
    /// Profile mode.
    pub mode: LaunchProfileMode,
    /// Stable mode token.
    pub mode_token: String,
    /// Lane the profile feeds.
    pub kind: LaunchProfileKind,
    /// Stable kind token.
    pub kind_token: String,
    /// Target binding.
    pub target: LaunchProfileTargetBinding,
    /// Adapter binding (debug lane and any other lane that declares one).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter: Option<LaunchProfileAdapterBinding>,
    /// Environment binding.
    pub environment: LaunchProfileEnvironmentBinding,
    /// Argument vector.
    pub arguments: LaunchProfileArguments,
    /// Declared side effects, in stable sorted order.
    pub declared_side_effects: Vec<LaunchProfileSideEffectClass>,
    /// Identity of the revision this snapshot belongs to.
    pub revision_id: String,
    /// Identity of the parent revision, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_revision_id: Option<String>,
    /// Wall-clock timestamp of the last edit applied to this snapshot.
    pub last_edited_at: String,
}

impl LaunchProfile {
    /// True when the profile carries all data required to dispatch.
    pub fn invalid_reason(&self) -> Option<LaunchProfileInvalidReason> {
        if self.target.canonical_target_id.is_empty() {
            return Some(LaunchProfileInvalidReason::MissingTargetBinding);
        }
        if matches!(self.kind, LaunchProfileKind::Debug) && self.adapter.is_none() {
            return Some(LaunchProfileInvalidReason::MissingAdapterBinding);
        }
        match self.mode {
            LaunchProfileMode::Launch => {
                if self
                    .arguments
                    .program
                    .as_deref()
                    .map(str::is_empty)
                    .unwrap_or(true)
                {
                    return Some(LaunchProfileInvalidReason::LaunchMissingProgram);
                }
            }
            LaunchProfileMode::Attach => {
                if self.arguments.attach_process_id.is_none() {
                    return Some(LaunchProfileInvalidReason::AttachMissingProcessId);
                }
            }
        }
        None
    }

    fn sorted_side_effects(
        side_effects: impl IntoIterator<Item = LaunchProfileSideEffectClass>,
    ) -> Vec<LaunchProfileSideEffectClass> {
        let mut out: Vec<LaunchProfileSideEffectClass> = side_effects.into_iter().collect();
        out.sort();
        out.dedup();
        out
    }
}

/// Description of a single profile edit. Stored on every revision so the
/// support export can replay the edit history without forking truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchProfileEdit {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Profile id this edit applies to.
    pub profile_id: String,
    /// Class of the edit.
    pub edit_class: LaunchProfileEditClass,
    /// Stable edit-class token.
    pub edit_class_token: String,
    /// Export-safe summary line.
    pub summary: String,
    /// Wall-clock timestamp of the edit.
    pub observed_at: String,
    /// Revision id created by this edit.
    pub created_revision_id: String,
    /// Revision id this edit moved away from, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_revision_id: Option<String>,
    /// When the edit class is [`LaunchProfileEditClass::RolledBack`], the
    /// revision id the rollback targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_target_revision_id: Option<String>,
}

/// Immutable snapshot of a profile at one revision. Stored verbatim so the
/// store can replay any prior state during rollback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchProfileRevision {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Profile id this revision belongs to.
    pub profile_id: String,
    /// Stable revision id (opaque, scoped to the profile).
    pub revision_id: String,
    /// Revision id of the parent revision when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_revision_id: Option<String>,
    /// Wall-clock timestamp this revision was created.
    pub created_at: String,
    /// Edit that produced this revision.
    pub edit: LaunchProfileEdit,
    /// Profile snapshot at this revision.
    pub snapshot: LaunchProfile,
}

/// One disclosure row produced by a preview.
///
/// Used for target, environment, adapter, and side-effect disclosure, plus
/// the drift rows recorded when the profile's stored binding does not match
/// the resolved current context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchProfileDisclosureRow {
    /// Field path the row describes (dotted notation).
    pub field_path: String,
    /// Plain-language label.
    pub label: String,
    /// Value stored on the profile, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_value_token: Option<String>,
    /// Value resolved from the current execution context, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_value_token: Option<String>,
    /// True when the row records a drift the user must review.
    pub drift_detected: bool,
    /// True when the changed field can alter where work executes.
    pub affects_target_boundary: bool,
}

/// Preview of a profile against the current execution context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchProfilePreview {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Profile id this preview is for.
    pub profile_id: String,
    /// Profile workspace id.
    pub workspace_id: String,
    /// Display name shown in the preview header.
    pub display_name: String,
    /// Profile mode.
    pub mode_token: String,
    /// Lane the profile feeds.
    pub kind_token: String,
    /// Current revision id at preview time.
    pub current_revision_id: String,
    /// Resolved execution-context id for the preview.
    pub execution_context_ref: String,
    /// Wall-clock timestamp the preview was minted.
    pub observed_at: String,
    /// Preview state.
    pub state: LaunchProfilePreviewState,
    /// Stable state token.
    pub state_token: String,
    /// Target disclosure rows.
    pub target_disclosure: Vec<LaunchProfileDisclosureRow>,
    /// Environment disclosure rows.
    pub environment_disclosure: Vec<LaunchProfileDisclosureRow>,
    /// Adapter disclosure rows.
    pub adapter_disclosure: Vec<LaunchProfileDisclosureRow>,
    /// Side-effect disclosure tokens (sorted).
    pub side_effect_disclosure_tokens: Vec<String>,
    /// Drift rows recorded between the stored binding and the resolved
    /// current context. A non-empty list forces the preview to disclose
    /// `drift_requires_review` before dispatch.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drift_rows: Vec<LaunchProfileDisclosureRow>,
    /// True when one or more drifts affect target identity or boundary.
    pub target_or_boundary_changed: bool,
    /// True when the resolved current target is reachable.
    pub current_target_reachable: bool,
    /// True when dispatch must wait for a visible user review.
    pub requires_review_before_dispatch: bool,
    /// True when shell / status surfaces MUST disclose the preview to the
    /// user. Derived from the preview state and the drift rows.
    pub honesty_marker_present: bool,
    /// Invalid reason, when the profile cannot dispatch from its current
    /// snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invalid_reason: Option<LaunchProfileInvalidReason>,
    /// Stable invalid-reason token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invalid_reason_token: Option<String>,
    /// Export-safe summary headline.
    pub summary_headline: String,
}

impl LaunchProfilePreview {
    /// Renders a deterministic plaintext block for the support-export
    /// clipboard action.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!("Launch profile preview: {}\n", self.display_name);
        out.push_str(&format!("  Profile: {}\n", self.profile_id));
        out.push_str(&format!("  Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("  Mode: {}\n", self.mode_token));
        out.push_str(&format!("  Kind: {}\n", self.kind_token));
        out.push_str(&format!("  Revision: {}\n", self.current_revision_id));
        out.push_str(&format!("  Context: {}\n", self.execution_context_ref));
        out.push_str(&format!("  State: {}\n", self.state_token));
        out.push_str(&format!(
            "  Review required: {}\n",
            self.requires_review_before_dispatch
        ));
        out.push_str(&format!(
            "  Target reachable: {}\n",
            self.current_target_reachable
        ));
        if !self.side_effect_disclosure_tokens.is_empty() {
            out.push_str(&format!(
                "  Side effects: {}\n",
                self.side_effect_disclosure_tokens.join(",")
            ));
        }
        if !self.drift_rows.is_empty() {
            out.push_str("  Drift:\n");
            for row in &self.drift_rows {
                out.push_str(&format!(
                    "    - {}: {} -> {}\n",
                    row.field_path,
                    row.profile_value_token.as_deref().unwrap_or("(none)"),
                    row.current_value_token.as_deref().unwrap_or("(none)"),
                ));
            }
        }
        if let Some(token) = &self.invalid_reason_token {
            out.push_str(&format!("  Invalid reason: {token}\n"));
        }
        out
    }
}

/// Support-export row carrying one profile's edit and preview lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchProfileSupportRow {
    /// Profile id.
    pub profile_id: String,
    /// Display name shown in the export.
    pub display_name: String,
    /// Current revision id.
    pub current_revision_id: String,
    /// Mode token at the current revision.
    pub mode_token: String,
    /// Kind token at the current revision.
    pub kind_token: String,
    /// Canonical target id at the current revision.
    pub canonical_target_id: String,
    /// Adapter id at the current revision, when one is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_id: Option<String>,
    /// Environment capsule id at the current revision.
    pub capsule_id: String,
    /// Number of revisions retained.
    pub revision_count: usize,
    /// Edit lineage in stable order.
    pub edit_lineage: Vec<LaunchProfileEdit>,
    /// Preview rendered with the latest available execution context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_preview: Option<LaunchProfilePreview>,
}

/// Support-export packet covering every retained profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchProfileSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Workspace covered by this export.
    pub workspace_id: String,
    /// Wall-clock timestamp the export was generated.
    pub generated_at: String,
    /// Profile rows in stable id order.
    pub profile_rows: Vec<LaunchProfileSupportRow>,
    /// True when one or more profile previews require disclosure.
    pub honesty_marker_present: bool,
    /// Export-safe summary headline.
    pub summary_headline: String,
}

impl LaunchProfileSupportExport {
    /// Renders a deterministic plaintext block for the support-export
    /// clipboard action.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!("Launch-profile support export: {}\n", self.export_id);
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Generated at: {}\n", self.generated_at));
        out.push_str(&format!("Profiles: {}\n", self.profile_rows.len()));
        for row in &self.profile_rows {
            out.push_str(&format!(
                "\nProfile: {} ({})\n",
                row.display_name, row.profile_id
            ));
            out.push_str(&format!(
                "  Mode: {} | Kind: {} | Revision: {}\n",
                row.mode_token, row.kind_token, row.current_revision_id
            ));
            out.push_str(&format!("  Target: {}\n", row.canonical_target_id));
            if let Some(adapter) = &row.adapter_id {
                out.push_str(&format!("  Adapter: {adapter}\n"));
            }
            out.push_str(&format!("  Capsule: {}\n", row.capsule_id));
            out.push_str(&format!("  Revisions retained: {}\n", row.revision_count));
            out.push_str("  Edit lineage:\n");
            for edit in &row.edit_lineage {
                out.push_str(&format!(
                    "    - {} @ {}: {}\n",
                    edit.edit_class_token, edit.observed_at, edit.summary
                ));
            }
            if let Some(preview) = &row.latest_preview {
                out.push_str(&format!("  Latest preview: {}\n", preview.state_token));
            }
        }
        out
    }
}

/// Request used by [`LaunchProfileStore::create_profile`] to seed a profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchProfileCreateRequest {
    /// Profile id (caller-chosen, stable).
    pub profile_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Display name.
    pub display_name: String,
    /// Mode.
    pub mode: LaunchProfileMode,
    /// Lane.
    pub kind: LaunchProfileKind,
    /// Target binding.
    pub target: LaunchProfileTargetBinding,
    /// Adapter binding (required for [`LaunchProfileKind::Debug`]).
    pub adapter: Option<LaunchProfileAdapterBinding>,
    /// Environment binding.
    pub environment: LaunchProfileEnvironmentBinding,
    /// Argument vector.
    pub arguments: LaunchProfileArguments,
    /// Declared side effects.
    pub declared_side_effects: Vec<LaunchProfileSideEffectClass>,
    /// Wall-clock timestamp.
    pub observed_at: String,
}

/// In-memory launch / attach profile store.
#[derive(Debug, Clone, Default)]
pub struct LaunchProfileStore {
    workspace_id: String,
    profiles: BTreeMap<String, ProfileSlot>,
    next_revision_seq: u64,
}

#[derive(Debug, Clone)]
struct ProfileSlot {
    current: LaunchProfile,
    revisions: Vec<LaunchProfileRevision>,
}

impl LaunchProfileStore {
    /// Creates an empty store scoped to one workspace.
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            profiles: BTreeMap::new(),
            next_revision_seq: 1,
        }
    }

    /// Workspace this store covers.
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    /// Returns the current snapshot for one profile.
    pub fn profile(&self, profile_id: &str) -> Result<&LaunchProfile, LaunchProfileStoreError> {
        self.profiles
            .get(profile_id)
            .map(|slot| &slot.current)
            .ok_or_else(|| LaunchProfileStoreError::ProfileNotFound {
                profile_id: profile_id.to_owned(),
            })
    }

    /// Returns the retained revision history for one profile in chronological
    /// order.
    pub fn revisions(
        &self,
        profile_id: &str,
    ) -> Result<&[LaunchProfileRevision], LaunchProfileStoreError> {
        self.profiles
            .get(profile_id)
            .map(|slot| slot.revisions.as_slice())
            .ok_or_else(|| LaunchProfileStoreError::ProfileNotFound {
                profile_id: profile_id.to_owned(),
            })
    }

    /// Returns every profile snapshot in stable id order.
    pub fn iter_profiles(&self) -> impl Iterator<Item = &LaunchProfile> {
        self.profiles.values().map(|slot| &slot.current)
    }

    /// Mints a fresh revision id derived from a stable monotonic counter.
    fn next_revision_id(&mut self, profile_id: &str) -> String {
        let id = format!("rev:{profile_id}:{:04}", self.next_revision_seq);
        self.next_revision_seq += 1;
        id
    }

    /// Creates a profile and emits its first revision.
    pub fn create_profile(
        &mut self,
        request: LaunchProfileCreateRequest,
    ) -> Result<LaunchProfileRevision, LaunchProfileStoreError> {
        if self.profiles.contains_key(&request.profile_id) {
            return Err(LaunchProfileStoreError::DuplicateProfileId {
                profile_id: request.profile_id,
            });
        }
        let revision_id = self.next_revision_id(&request.profile_id);
        let sorted_effects = LaunchProfile::sorted_side_effects(request.declared_side_effects);
        let snapshot = LaunchProfile {
            record_kind: LAUNCH_PROFILE_RECORD_KIND.to_owned(),
            schema_version: LAUNCH_PROFILE_SCHEMA_VERSION,
            profile_id: request.profile_id.clone(),
            workspace_id: request.workspace_id,
            display_name: request.display_name,
            mode: request.mode,
            mode_token: request.mode.as_str().to_owned(),
            kind: request.kind,
            kind_token: request.kind.as_str().to_owned(),
            target: request.target,
            adapter: request.adapter,
            environment: request.environment,
            arguments: request.arguments,
            declared_side_effects: sorted_effects,
            revision_id: revision_id.clone(),
            parent_revision_id: None,
            last_edited_at: request.observed_at.clone(),
        };
        let edit = LaunchProfileEdit {
            record_kind: LAUNCH_PROFILE_EDIT_RECORD_KIND.to_owned(),
            schema_version: LAUNCH_PROFILE_SCHEMA_VERSION,
            profile_id: request.profile_id.clone(),
            edit_class: LaunchProfileEditClass::Created,
            edit_class_token: LaunchProfileEditClass::Created.as_str().to_owned(),
            summary: format!("Created profile `{}`", snapshot.display_name),
            observed_at: request.observed_at.clone(),
            created_revision_id: revision_id.clone(),
            previous_revision_id: None,
            rollback_target_revision_id: None,
        };
        let revision = LaunchProfileRevision {
            record_kind: LAUNCH_PROFILE_REVISION_RECORD_KIND.to_owned(),
            schema_version: LAUNCH_PROFILE_SCHEMA_VERSION,
            profile_id: request.profile_id.clone(),
            revision_id,
            parent_revision_id: None,
            created_at: request.observed_at,
            edit,
            snapshot: snapshot.clone(),
        };
        let slot = ProfileSlot {
            current: snapshot,
            revisions: vec![revision.clone()],
        };
        self.profiles.insert(request.profile_id, slot);
        Ok(revision)
    }

    /// Applies a mutation to a profile and returns the new revision. The
    /// mutation is described by `edit_class` so the resulting revision can
    /// disclose what changed without diffing the snapshots from scratch.
    pub fn apply_edit<F>(
        &mut self,
        profile_id: &str,
        edit_class: LaunchProfileEditClass,
        observed_at: impl Into<String>,
        mut mutate: F,
    ) -> Result<LaunchProfileRevision, LaunchProfileStoreError>
    where
        F: FnMut(&mut LaunchProfileMutable),
    {
        if !self.profiles.contains_key(profile_id) {
            return Err(LaunchProfileStoreError::ProfileNotFound {
                profile_id: profile_id.to_owned(),
            });
        }
        let observed_at = observed_at.into();
        let revision_id = self.next_revision_id(profile_id);
        let slot = self.profiles.get_mut(profile_id).expect("profile present");
        let previous_revision_id = slot.current.revision_id.clone();
        let mut snapshot = slot.current.clone();
        let mut mutable = LaunchProfileMutable {
            display_name: &mut snapshot.display_name,
            mode: &mut snapshot.mode,
            target: &mut snapshot.target,
            adapter: &mut snapshot.adapter,
            environment: &mut snapshot.environment,
            arguments: &mut snapshot.arguments,
            declared_side_effects: &mut snapshot.declared_side_effects,
        };
        mutate(&mut mutable);
        snapshot.mode_token = snapshot.mode.as_str().to_owned();
        snapshot.declared_side_effects =
            LaunchProfile::sorted_side_effects(snapshot.declared_side_effects.iter().copied());
        snapshot.revision_id = revision_id.clone();
        snapshot.parent_revision_id = Some(previous_revision_id.clone());
        snapshot.last_edited_at = observed_at.clone();
        let edit = LaunchProfileEdit {
            record_kind: LAUNCH_PROFILE_EDIT_RECORD_KIND.to_owned(),
            schema_version: LAUNCH_PROFILE_SCHEMA_VERSION,
            profile_id: profile_id.to_owned(),
            edit_class,
            edit_class_token: edit_class.as_str().to_owned(),
            summary: summarize_edit(edit_class, &snapshot),
            observed_at: observed_at.clone(),
            created_revision_id: revision_id.clone(),
            previous_revision_id: Some(previous_revision_id.clone()),
            rollback_target_revision_id: None,
        };
        let revision = LaunchProfileRevision {
            record_kind: LAUNCH_PROFILE_REVISION_RECORD_KIND.to_owned(),
            schema_version: LAUNCH_PROFILE_SCHEMA_VERSION,
            profile_id: profile_id.to_owned(),
            revision_id,
            parent_revision_id: Some(previous_revision_id),
            created_at: observed_at,
            edit,
            snapshot: snapshot.clone(),
        };
        slot.current = snapshot;
        slot.revisions.push(revision.clone());
        Ok(revision)
    }

    /// Rolls a profile back to a prior revision. A new revision is created
    /// so the lineage records the rollback rather than truncating history.
    pub fn rollback_to(
        &mut self,
        profile_id: &str,
        target_revision_id: &str,
        observed_at: impl Into<String>,
    ) -> Result<LaunchProfileRevision, LaunchProfileStoreError> {
        let slot = self.profiles.get(profile_id).ok_or_else(|| {
            LaunchProfileStoreError::ProfileNotFound {
                profile_id: profile_id.to_owned(),
            }
        })?;
        let target_snapshot = slot
            .revisions
            .iter()
            .find(|rev| rev.revision_id == target_revision_id)
            .map(|rev| rev.snapshot.clone())
            .ok_or_else(|| LaunchProfileStoreError::RevisionNotFound {
                profile_id: profile_id.to_owned(),
                revision_id: target_revision_id.to_owned(),
            })?;
        let observed_at = observed_at.into();
        let new_revision_id = self.next_revision_id(profile_id);
        let slot = self.profiles.get_mut(profile_id).expect("profile present");
        let previous_revision_id = slot.current.revision_id.clone();
        let mut snapshot = target_snapshot;
        snapshot.revision_id = new_revision_id.clone();
        snapshot.parent_revision_id = Some(previous_revision_id.clone());
        snapshot.last_edited_at = observed_at.clone();
        let edit = LaunchProfileEdit {
            record_kind: LAUNCH_PROFILE_EDIT_RECORD_KIND.to_owned(),
            schema_version: LAUNCH_PROFILE_SCHEMA_VERSION,
            profile_id: profile_id.to_owned(),
            edit_class: LaunchProfileEditClass::RolledBack,
            edit_class_token: LaunchProfileEditClass::RolledBack.as_str().to_owned(),
            summary: format!(
                "Rolled back `{}` to revision {target_revision_id}",
                snapshot.display_name
            ),
            observed_at: observed_at.clone(),
            created_revision_id: new_revision_id.clone(),
            previous_revision_id: Some(previous_revision_id.clone()),
            rollback_target_revision_id: Some(target_revision_id.to_owned()),
        };
        let revision = LaunchProfileRevision {
            record_kind: LAUNCH_PROFILE_REVISION_RECORD_KIND.to_owned(),
            schema_version: LAUNCH_PROFILE_SCHEMA_VERSION,
            profile_id: profile_id.to_owned(),
            revision_id: new_revision_id,
            parent_revision_id: Some(previous_revision_id),
            created_at: observed_at,
            edit,
            snapshot: snapshot.clone(),
        };
        slot.current = snapshot;
        slot.revisions.push(revision.clone());
        Ok(revision)
    }

    /// Returns a preview of the profile against the current execution
    /// context. Surfaces, support exports, and runtime dispatch consume the
    /// same preview record so they cannot disagree on what the user is
    /// about to authorize.
    pub fn preview(
        &self,
        profile_id: &str,
        current_context: &ExecutionContext,
        observed_at: impl Into<String>,
    ) -> Result<LaunchProfilePreview, LaunchProfileStoreError> {
        let slot = self.profiles.get(profile_id).ok_or_else(|| {
            LaunchProfileStoreError::ProfileNotFound {
                profile_id: profile_id.to_owned(),
            }
        })?;
        Ok(project_preview(&slot.current, current_context, observed_at))
    }

    /// Builds a deterministic support-export packet covering every profile.
    /// The `context_for` lookup supplies the most-recently resolved
    /// execution context for each profile id; profiles without a current
    /// context are exported without a preview.
    pub fn support_export<F>(
        &self,
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        mut context_for: F,
    ) -> LaunchProfileSupportExport
    where
        F: FnMut(&str) -> Option<ExecutionContext>,
    {
        let export_id = export_id.into();
        let generated_at = generated_at.into();
        let mut rows = Vec::with_capacity(self.profiles.len());
        let mut honesty_marker_present = false;
        for (profile_id, slot) in &self.profiles {
            let preview = context_for(profile_id)
                .as_ref()
                .map(|ctx| project_preview(&slot.current, ctx, generated_at.clone()));
            if let Some(p) = &preview {
                if p.honesty_marker_present {
                    honesty_marker_present = true;
                }
            }
            let row = LaunchProfileSupportRow {
                profile_id: profile_id.clone(),
                display_name: slot.current.display_name.clone(),
                current_revision_id: slot.current.revision_id.clone(),
                mode_token: slot.current.mode_token.clone(),
                kind_token: slot.current.kind_token.clone(),
                canonical_target_id: slot.current.target.canonical_target_id.clone(),
                adapter_id: slot.current.adapter.as_ref().map(|a| a.adapter_id.clone()),
                capsule_id: slot.current.environment.capsule_id.clone(),
                revision_count: slot.revisions.len(),
                edit_lineage: slot.revisions.iter().map(|rev| rev.edit.clone()).collect(),
                latest_preview: preview,
            };
            rows.push(row);
        }
        let summary_headline = format!(
            "{} profiles ({} require disclosure)",
            rows.len(),
            rows.iter()
                .filter(|row| row
                    .latest_preview
                    .as_ref()
                    .map(|p| p.honesty_marker_present)
                    .unwrap_or(false))
                .count()
        );
        LaunchProfileSupportExport {
            record_kind: LAUNCH_PROFILE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: LAUNCH_PROFILE_SCHEMA_VERSION,
            export_id,
            workspace_id: self.workspace_id.clone(),
            generated_at,
            profile_rows: rows,
            honesty_marker_present,
            summary_headline,
        }
    }
}

/// Mutable view supplied to [`LaunchProfileStore::apply_edit`]. Callers MAY
/// mutate any of the exposed fields; the store sorts side effects and stamps
/// the resulting revision automatically.
#[derive(Debug)]
pub struct LaunchProfileMutable<'a> {
    pub display_name: &'a mut String,
    pub mode: &'a mut LaunchProfileMode,
    pub target: &'a mut LaunchProfileTargetBinding,
    pub adapter: &'a mut Option<LaunchProfileAdapterBinding>,
    pub environment: &'a mut LaunchProfileEnvironmentBinding,
    pub arguments: &'a mut LaunchProfileArguments,
    pub declared_side_effects: &'a mut Vec<LaunchProfileSideEffectClass>,
}

fn summarize_edit(edit_class: LaunchProfileEditClass, snapshot: &LaunchProfile) -> String {
    match edit_class {
        LaunchProfileEditClass::Created => {
            format!("Created profile `{}`", snapshot.display_name)
        }
        LaunchProfileEditClass::RenamedDisplayName => {
            format!("Renamed profile to `{}`", snapshot.display_name)
        }
        LaunchProfileEditClass::ModeChanged => {
            format!("Mode is now {}", snapshot.mode_token)
        }
        LaunchProfileEditClass::TargetBindingChanged => format!(
            "Target now points to {}",
            snapshot.target.canonical_target_id
        ),
        LaunchProfileEditClass::AdapterBindingChanged => match &snapshot.adapter {
            Some(adapter) => format!("Adapter is now {}", adapter.adapter_id),
            None => "Adapter binding cleared".to_owned(),
        },
        LaunchProfileEditClass::EnvironmentBindingChanged => {
            format!(
                "Environment capsule is now {}",
                snapshot.environment.capsule_id
            )
        }
        LaunchProfileEditClass::ArgumentsChanged => "Arguments updated".to_owned(),
        LaunchProfileEditClass::SideEffectsChanged => format!(
            "Side effects: {}",
            if snapshot.declared_side_effects.is_empty() {
                "(none declared)".to_owned()
            } else {
                snapshot
                    .declared_side_effects
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            }
        ),
        LaunchProfileEditClass::RolledBack => {
            format!("Rolled back to revision {}", snapshot.revision_id)
        }
    }
}

fn project_preview(
    profile: &LaunchProfile,
    current_context: &ExecutionContext,
    observed_at: impl Into<String>,
) -> LaunchProfilePreview {
    let observed_at = observed_at.into();
    let mut target_rows = Vec::new();
    push_row(
        &mut target_rows,
        "target_binding.canonical_target_id",
        "Canonical target id",
        Some(profile.target.canonical_target_id.clone()),
        Some(current_context.target_identity.canonical_target_id.clone()),
        true,
    );
    push_row(
        &mut target_rows,
        "target_binding.target_class",
        "Target class",
        Some(profile.target.target_class_token.clone()),
        Some(
            current_context
                .target_identity
                .target_class
                .as_str()
                .to_owned(),
        ),
        true,
    );
    push_row(
        &mut target_rows,
        "target_binding.working_directory",
        "Working directory",
        profile.target.working_directory.clone(),
        current_context.target_identity.working_directory.clone(),
        true,
    );
    push_row(
        &mut target_rows,
        "target_binding.scope_class",
        "Workset scope",
        Some(profile.target.scope_class_token.clone()),
        Some(current_context.workset_scope_class.as_str().to_owned()),
        true,
    );

    let mut env_rows = Vec::new();
    push_row(
        &mut env_rows,
        "environment_binding.capsule_id",
        "Capsule id",
        Some(profile.environment.capsule_id.clone()),
        Some(current_context.environment_capsule_ref.capsule_id.clone()),
        false,
    );
    push_row(
        &mut env_rows,
        "environment_binding.capsule_hash",
        "Capsule hash",
        Some(profile.environment.capsule_hash.clone()),
        Some(current_context.environment_capsule_ref.capsule_hash.clone()),
        false,
    );

    let mut adapter_rows = Vec::new();
    if let Some(adapter) = &profile.adapter {
        push_row(
            &mut adapter_rows,
            "adapter_binding.adapter_id",
            "Adapter id",
            Some(adapter.adapter_id.clone()),
            None,
            false,
        );
        push_row(
            &mut adapter_rows,
            "adapter_binding.transport_class",
            "Adapter transport",
            Some(adapter.transport_class_token.clone()),
            None,
            false,
        );
        push_row(
            &mut adapter_rows,
            "adapter_binding.requested_dap_protocol_version",
            "Requested DAP version",
            Some(adapter.requested_dap_protocol_version.clone()),
            None,
            false,
        );
    }

    let drift_rows: Vec<LaunchProfileDisclosureRow> = target_rows
        .iter()
        .chain(env_rows.iter())
        .filter(|row| row.drift_detected)
        .cloned()
        .collect();

    let invalid_reason = profile.invalid_reason();
    let target_or_boundary_changed = drift_rows.iter().any(|row| row.affects_target_boundary);
    let current_target_reachable = matches!(
        current_context.target_identity.reachability_state,
        ReachabilityState::Reachable
    );

    let state = if let Some(_reason) = invalid_reason {
        LaunchProfilePreviewState::UnavailableInvalidConfig
    } else if !current_target_reachable {
        LaunchProfilePreviewState::TargetUnreachable
    } else if !drift_rows.is_empty() {
        LaunchProfilePreviewState::DriftRequiresReview
    } else {
        LaunchProfilePreviewState::ReadyToDispatch
    };

    let requires_review_before_dispatch =
        !matches!(state, LaunchProfilePreviewState::ReadyToDispatch);
    let honesty_marker_present = state.requires_disclosure();
    let side_effect_disclosure_tokens: Vec<String> = profile
        .declared_side_effects
        .iter()
        .map(|s| s.as_str().to_owned())
        .collect();
    let summary_headline = match state {
        LaunchProfilePreviewState::ReadyToDispatch => format!(
            "`{}` ready to dispatch against {}",
            profile.display_name, current_context.target_identity.canonical_target_id
        ),
        LaunchProfilePreviewState::DriftRequiresReview => format!(
            "`{}` drift requires review ({} fields)",
            profile.display_name,
            drift_rows.len()
        ),
        LaunchProfilePreviewState::TargetUnreachable => format!(
            "`{}` cannot dispatch: target {} unreachable",
            profile.display_name, current_context.target_identity.canonical_target_id
        ),
        LaunchProfilePreviewState::UnavailableInvalidConfig => format!(
            "`{}` invalid: {}",
            profile.display_name,
            invalid_reason
                .map(|r| r.as_str())
                .unwrap_or("invalid_configuration")
        ),
    };

    LaunchProfilePreview {
        record_kind: LAUNCH_PROFILE_PREVIEW_RECORD_KIND.to_owned(),
        schema_version: LAUNCH_PROFILE_SCHEMA_VERSION,
        profile_id: profile.profile_id.clone(),
        workspace_id: profile.workspace_id.clone(),
        display_name: profile.display_name.clone(),
        mode_token: profile.mode_token.clone(),
        kind_token: profile.kind_token.clone(),
        current_revision_id: profile.revision_id.clone(),
        execution_context_ref: current_context.execution_context_id.clone(),
        observed_at,
        state,
        state_token: state.as_str().to_owned(),
        target_disclosure: target_rows,
        environment_disclosure: env_rows,
        adapter_disclosure: adapter_rows,
        side_effect_disclosure_tokens,
        drift_rows,
        target_or_boundary_changed,
        current_target_reachable,
        requires_review_before_dispatch,
        honesty_marker_present,
        invalid_reason,
        invalid_reason_token: invalid_reason.map(|r| r.as_str().to_owned()),
        summary_headline,
    }
}

fn push_row(
    rows: &mut Vec<LaunchProfileDisclosureRow>,
    field_path: &str,
    label: &str,
    profile_value: Option<String>,
    current_value: Option<String>,
    affects_target_boundary: bool,
) {
    let drift_detected = match (&profile_value, &current_value) {
        (Some(a), Some(b)) => a != b,
        (None, None) => false,
        _ => true,
    };
    rows.push(LaunchProfileDisclosureRow {
        field_path: field_path.to_owned(),
        label: label.to_owned(),
        profile_value_token: profile_value,
        current_value_token: current_value,
        drift_detected,
        affects_target_boundary,
    });
}

#[cfg(test)]
mod tests;
