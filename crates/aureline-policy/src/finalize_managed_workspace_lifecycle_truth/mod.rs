//! Finalize managed-workspace lifecycle states, suspend/resume/rebuild/share
//! truth, and local or direct-remote fallback on any claimed managed row.
//!
//! This module validates the conditions that make managed-workspace lifecycle
//! state, persistence class, share mode, and fallback explicit enough that no
//! managed lane survives on implication alone:
//!
//! 1. **Descriptor completeness** — every claimed managed workspace carries a
//!    [`ManagedWorkspaceDescriptor`] with workspace ID, org/tenant, region,
//!    persistence class, template version, quota/billing owner, secret model, and
//!    expiry policy.
//! 2. **Lifecycle state explicit** — provisioning and runtime states are drawn
//!    from the closed [`ManagedProvisioningStateClass`] vocabulary; no surface
//!    may collapse `queued`, `allocating`, `booting`, `attaching`, `sync_warming`,
//!    `ready`, `reconnecting`, `suspended`, `rebuilding`, `deleting`, or `failed`
//!    into one generic loading banner.
//! 3. **Suspend/resume checkpoint honesty** — every suspend or resume carries a
//!    [`ManagedSuspendResumeCheckpoint`] that names retained state classes,
//!    version drift, pinned routes/endpoints, and whether attach is to the same
//!    live environment or a resumed snapshot.
//! 4. **Destructive-operation preview** — rebuild, recreate, reset, delete, and
//!    extend-TTL flows carry a [`ManagedRebuildPlan`] that previews preserved
//!    state, reprovisioned state, route revalidation needs, and policy locks.
//! 5. **Fallback path declared** — every profile that claims local or
//!    direct-remote fallback carries a [`ManagedFallbackPathClass`] declaration;
//!    control-plane outage drills keep the fallback visible.
//! 6. **Share/handoff token explicit** — share or handoff tokens carry a
//!    [`ManagedShareHandoffToken`] with join mode (`same_live`,
//!    `resume_snapshot`, `fresh_reprovision`) and authority scope.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language consequence labels, and
//! opaque refs only.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/policy/m4/finalize-managed-workspace-lifecycle-truth.md`
//! - Artifact: `artifacts/policy/m4/finalize-managed-workspace-lifecycle-truth.md`
//! - Contract ref: [`FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SHARED_CONTRACT_REF`]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SHARED_CONTRACT_REF: &str =
    "policy:finalize_managed_workspace_lifecycle_truth:v1";

/// Record-kind tag for [`FinalizeManagedWorkspaceLifecycleTruthPage`] payloads.
pub const FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_PAGE_RECORD_KIND: &str =
    "policy_finalize_managed_workspace_lifecycle_truth_page_record";

/// Record-kind tag for [`FinalizeManagedWorkspaceLifecycleTruthRow`] payloads.
pub const FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_ROW_RECORD_KIND: &str =
    "policy_finalize_managed_workspace_lifecycle_truth_row_record";

/// Record-kind tag for [`FinalizeManagedWorkspaceLifecycleTruthDefect`] payloads.
pub const FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_DEFECT_RECORD_KIND: &str =
    "policy_finalize_managed_workspace_lifecycle_truth_defect_record";

/// Record-kind tag for [`FinalizeManagedWorkspaceLifecycleTruthSupportExport`] payloads.
pub const FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_finalize_managed_workspace_lifecycle_truth_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_DOC_REF: &str =
    "docs/policy/m4/finalize-managed-workspace-lifecycle-truth.md";

/// Repo-relative path of the artifact summary for this lane.
pub const FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_ARTIFACT_REF: &str =
    "artifacts/policy/m4/finalize-managed-workspace-lifecycle-truth.md";

// ---------------------------------------------------------------------------
// Descriptor and vocabulary types
// ---------------------------------------------------------------------------

/// Closed provisioning and runtime state vocabulary.
///
/// Every managed-workspace surface must render one of these states verbatim;
/// no surface may collapse them into a generic loading banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedProvisioningStateClass {
    /// Request accepted, waiting for capacity or policy check.
    Queued,
    /// Environment resources being reserved.
    Allocating,
    /// Image/VM/container starting.
    Booting,
    /// Transport/agent channel being established.
    Attaching,
    /// File/index/session warmup in progress.
    SyncWarming,
    /// Claimed capabilities available.
    Ready,
    /// Reconnect or reauth in progress.
    Reconnecting,
    /// State preserved for later resume.
    Suspended,
    /// Rebuild in progress.
    Rebuilding,
    /// Deleting in progress.
    Deleting,
    /// Provisioning or runtime failed.
    Failed,
}

impl ManagedProvisioningStateClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Allocating => "allocating",
            Self::Booting => "booting",
            Self::Attaching => "attaching",
            Self::SyncWarming => "sync_warming",
            Self::Ready => "ready",
            Self::Reconnecting => "reconnecting",
            Self::Suspended => "suspended",
            Self::Rebuilding => "rebuilding",
            Self::Deleting => "deleting",
            Self::Failed => "failed",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Queued => "Queued",
            Self::Allocating => "Allocating",
            Self::Booting => "Booting",
            Self::Attaching => "Attaching",
            Self::SyncWarming => "Sync warming",
            Self::Ready => "Ready",
            Self::Reconnecting => "Reconnecting",
            Self::Suspended => "Suspended",
            Self::Rebuilding => "Rebuilding",
            Self::Deleting => "Deleting",
            Self::Failed => "Failed",
        }
    }

    /// True when the workspace is in a terminal or non-interactive state.
    pub const fn is_non_interactive(self) -> bool {
        matches!(
            self,
            Self::Queued | Self::Allocating | Self::Booting | Self::Deleting | Self::Failed
        )
    }

    /// True when remote mutation is admitted in this state.
    pub const fn admits_remote_mutation(self) -> bool {
        matches!(self, Self::Ready)
    }
}

/// Closed persistence-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedPersistenceClass {
    /// Ephemeral; no state survives stop.
    Ephemeral,
    /// Files and editor state persist across suspend/resume.
    FilesAndEditorState,
    /// Full workspace snapshot including processes and terminals.
    FullWorkspaceSnapshot,
    /// Customer-managed persistent volume.
    CustomerManagedVolume,
    /// Policy-mandated retention with explicit expiry.
    PolicyRetentionWithExpiry,
}

impl ManagedPersistenceClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ephemeral => "ephemeral",
            Self::FilesAndEditorState => "files_and_editor_state",
            Self::FullWorkspaceSnapshot => "full_workspace_snapshot",
            Self::CustomerManagedVolume => "customer_managed_volume",
            Self::PolicyRetentionWithExpiry => "policy_retention_with_expiry",
        }
    }
}

/// Closed secret-model vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedSecretModelClass {
    /// Secrets are injected at provision time and not rotated.
    StaticInjection,
    /// Secrets are delegated to a secret broker with rotation.
    BrokerDelegatedWithRotation,
    /// Secrets are customer-managed and never touch the control plane.
    CustomerManagedNeverLeavesTenant,
    /// Secrets are ephemeral and recreated on every resume.
    EphemeralPerSession,
}

impl ManagedSecretModelClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaticInjection => "static_injection",
            Self::BrokerDelegatedWithRotation => "broker_delegated_with_rotation",
            Self::CustomerManagedNeverLeavesTenant => "customer_managed_never_leaves_tenant",
            Self::EphemeralPerSession => "ephemeral_per_session",
        }
    }
}

/// Closed join-mode vocabulary for share/handoff tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedJoinModeClass {
    /// Recipient joins the exact same live environment.
    SameLive,
    /// Recipient gets a resumed snapshot; not the same live instance.
    ResumeSnapshot,
    /// Recipient triggers a fresh reprovision; no live-state continuity.
    FreshReprovision,
}

impl ManagedJoinModeClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameLive => "same_live",
            Self::ResumeSnapshot => "resume_snapshot",
            Self::FreshReprovision => "fresh_reprovision",
        }
    }

    /// True when the recipient receives live-state continuity.
    pub const fn implies_live_continuity(self) -> bool {
        matches!(self, Self::SameLive)
    }
}

/// Closed fallback-path vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedFallbackPathClass {
    /// No fallback is declared.
    NotDeclared,
    /// Local folder/workspace remains available.
    LocalWorkspace,
    /// Direct SSH remote target remains available.
    DirectSsh,
    /// Direct container/devcontainer target remains available.
    DirectContainer,
    /// Local and direct-remote both remain available.
    LocalAndDirectRemote,
}

impl ManagedFallbackPathClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotDeclared => "not_declared",
            Self::LocalWorkspace => "local_workspace",
            Self::DirectSsh => "direct_ssh",
            Self::DirectContainer => "direct_container",
            Self::LocalAndDirectRemote => "local_and_direct_remote",
        }
    }

    /// True when a fallback path is declared.
    pub const fn has_fallback(self) -> bool {
        !matches!(self, Self::NotDeclared)
    }
}

/// Closed destructive-operation vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedDestructiveOperationClass {
    /// Rebuild from template/snapshot.
    Rebuild,
    /// Recreate as a new instance.
    Recreate,
    /// Reset to initial template state.
    Reset,
    /// Delete the workspace.
    Delete,
    /// Extend time-to-live.
    ExtendTtl,
}

impl ManagedDestructiveOperationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rebuild => "rebuild",
            Self::Recreate => "recreate",
            Self::Reset => "reset",
            Self::Delete => "delete",
            Self::ExtendTtl => "extend_ttl",
        }
    }
}

/// Managed-workspace descriptor carried on every claimed managed row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceDescriptor {
    /// Opaque workspace ID.
    pub workspace_id: String,
    /// Human-readable workspace name.
    pub workspace_name: String,
    /// Org or tenant identifier.
    pub org_or_tenant: String,
    /// Region token.
    pub region: String,
    /// Persistence class.
    pub persistence_class: ManagedPersistenceClass,
    /// Stable persistence token.
    pub persistence_class_token: String,
    /// Template or snapshot version.
    pub template_version: String,
    /// Quota/billing owner identifier.
    pub quota_billing_owner: String,
    /// Secret model.
    pub secret_model: ManagedSecretModelClass,
    /// Stable secret-model token.
    pub secret_model_token: String,
    /// Expiry policy summary.
    pub expiry_policy: String,
    /// Current control-plane versus data-plane state token.
    pub plane_state_token: String,
}

impl ManagedWorkspaceDescriptor {
    /// Builds a descriptor from typed fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        org_or_tenant: impl Into<String>,
        region: impl Into<String>,
        persistence_class: ManagedPersistenceClass,
        template_version: impl Into<String>,
        quota_billing_owner: impl Into<String>,
        secret_model: ManagedSecretModelClass,
        expiry_policy: impl Into<String>,
        plane_state_token: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            workspace_name: workspace_name.into(),
            org_or_tenant: org_or_tenant.into(),
            region: region.into(),
            persistence_class,
            persistence_class_token: persistence_class.as_str().to_owned(),
            template_version: template_version.into(),
            quota_billing_owner: quota_billing_owner.into(),
            secret_model,
            secret_model_token: secret_model.as_str().to_owned(),
            expiry_policy: expiry_policy.into(),
            plane_state_token: plane_state_token.into(),
        }
    }
}

/// Suspend/resume checkpoint stating what persists, what drifts, and which
/// routes/endpoints are pinned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedSuspendResumeCheckpoint {
    /// State classes that persist across suspend/resume.
    #[serde(default)]
    pub retained_state_classes: Vec<String>,
    /// State classes that drift or are invalidated.
    #[serde(default)]
    pub drift_state_classes: Vec<String>,
    /// Pinned routes/endpoints that survive resume.
    #[serde(default)]
    pub pinned_routes: Vec<String>,
    /// True when attach is to the same live environment.
    pub same_live_environment: bool,
    /// True when attach is to a resumed snapshot.
    pub resumed_snapshot: bool,
    /// Local journals or unsaved edits remaining outside the managed boundary.
    #[serde(default)]
    pub local_journals_outside_boundary: Vec<String>,
    /// Review-safe checkpoint summary.
    pub summary: String,
}

/// Rebuild/recreate/reset/delete/extend-TTL plan previewing what persists and
/// what is reprovisioned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedRebuildPlan {
    /// The destructive operation being previewed.
    pub operation: ManagedDestructiveOperationClass,
    /// Stable operation token.
    pub operation_token: String,
    /// State classes that persist.
    #[serde(default)]
    pub preserved_state_classes: Vec<String>,
    /// State classes that are reprovisioned.
    #[serde(default)]
    pub reprovisioned_state_classes: Vec<String>,
    /// Data that must be exported before the operation proceeds.
    #[serde(default)]
    pub must_export_first: Vec<String>,
    /// Policy locks that apply.
    #[serde(default)]
    pub policy_locks: Vec<String>,
    /// Billing implications summary.
    pub billing_implications: String,
    /// Review-safe plan summary.
    pub summary: String,
}

impl ManagedRebuildPlan {
    /// Builds a rebuild plan for the requested operation.
    pub fn new(
        operation: ManagedDestructiveOperationClass,
        preserved_state_classes: Vec<String>,
        reprovisioned_state_classes: Vec<String>,
        must_export_first: Vec<String>,
        policy_locks: Vec<String>,
        billing_implications: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            operation,
            operation_token: operation.as_str().to_owned(),
            preserved_state_classes,
            reprovisioned_state_classes,
            must_export_first,
            policy_locks,
            billing_implications: billing_implications.into(),
            summary: summary.into(),
        }
    }
}

/// Share/handoff token with explicit join mode and authority scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedShareHandoffToken {
    /// Target workspace identity.
    pub target_workspace_id: String,
    /// Audience description.
    pub audience: String,
    /// Expiry timestamp.
    pub expiry: String,
    /// Client-scope note.
    pub client_scope_note: String,
    /// Explicit join mode.
    pub join_mode: ManagedJoinModeClass,
    /// Stable join-mode token.
    pub join_mode_token: String,
    /// Authority scope summary.
    pub authority_scope: String,
}

impl ManagedShareHandoffToken {
    /// Builds a share/handoff token.
    pub fn new(
        target_workspace_id: impl Into<String>,
        audience: impl Into<String>,
        expiry: impl Into<String>,
        client_scope_note: impl Into<String>,
        join_mode: ManagedJoinModeClass,
        authority_scope: impl Into<String>,
    ) -> Self {
        Self {
            target_workspace_id: target_workspace_id.into(),
            audience: audience.into(),
            expiry: expiry.into(),
            client_scope_note: client_scope_note.into(),
            join_mode,
            join_mode_token: join_mode.as_str().to_owned(),
            authority_scope: authority_scope.into(),
        }
    }
}

/// Provisioning event recording operation state, step, retryability, and
/// fallback route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedProvisioningEvent {
    /// Opaque operation ID.
    pub operation_id: String,
    /// Current provisioning state.
    pub state: ManagedProvisioningStateClass,
    /// Stable state token.
    pub state_token: String,
    /// Current step description.
    pub step: String,
    /// True when the operation is retryable.
    pub retryable: bool,
    /// Logs/diagnostics refs.
    #[serde(default)]
    pub logs_refs: Vec<String>,
    /// Destructive-vs-nondestructive note.
    pub destructive_note: String,
    /// Fallback route when available.
    pub fallback_route: ManagedFallbackPathClass,
    /// Stable fallback-route token.
    pub fallback_route_token: String,
}

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// One managed-workspace row in the lifecycle truth input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceLifecycleInputRow {
    /// Opaque row identifier.
    pub row_id: String,
    /// Managed-workspace descriptor.
    pub descriptor: ManagedWorkspaceDescriptor,
    /// Current provisioning state.
    pub current_state: ManagedProvisioningStateClass,
    /// True when the lifecycle state is explicit (not collapsed into generic
    /// loading).
    pub state_explicit: bool,
    /// Suspend/resume checkpoint when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend_resume_checkpoint: Option<ManagedSuspendResumeCheckpoint>,
    /// Rebuild plans when applicable.
    #[serde(default)]
    pub rebuild_plans: Vec<ManagedRebuildPlan>,
    /// Share/handoff tokens when applicable.
    #[serde(default)]
    pub share_handoff_tokens: Vec<ManagedShareHandoffToken>,
    /// Declared fallback path.
    pub fallback_path: ManagedFallbackPathClass,
    /// True when a local or direct-remote fallback is actually qualified.
    pub fallback_qualified: bool,
    /// True when the row has been through a control-plane outage drill.
    pub outage_drill_passed: bool,
    /// True when destructive-operation previews are present.
    pub destructive_previews_present: bool,
}

/// Full auditable input for the managed-workspace lifecycle truth page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceLifecycleTruthInput {
    /// Claimed managed-workspace rows.
    pub workspace_rows: Vec<ManagedWorkspaceLifecycleInputRow>,
    /// True when all surfaces resolve the same closed-vocabulary token for the
    /// same provisioning state.
    pub vocabulary_consistent_across_surfaces: bool,
    /// True when every claimed managed row has a descriptor.
    pub all_rows_have_descriptors: bool,
    /// True when every resume or rebuild flow explains what persists versus what
    /// is reprovisioned.
    pub persistence_truth_explicit: bool,
    /// True when share/handoff tokens name join mode and authority scope.
    pub share_tokens_explicit: bool,
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and for individual rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeManagedWorkspaceLifecycleQualificationClass {
    /// All conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required row is missing or incomplete; coverage gap prevents a beta claim.
    Preview,
    /// A hard guardrail was violated; the packet is withdrawn.
    Withdrawn,
}

impl FinalizeManagedWorkspaceLifecycleQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// True when this tier is claimable (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed reason a packet or row was narrowed below stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeManagedWorkspaceLifecycleNarrowReasonClass {
    /// No narrowing.
    NotNarrowed,
    /// A managed-workspace row lacks a descriptor.
    DescriptorMissing,
    /// Lifecycle state is collapsed into a generic loading banner.
    LifecycleStateCollapsed,
    /// Suspend/resume checkpoint does not state what persists or drifts.
    SuspendResumeCheckpointIncomplete,
    /// Destructive operation does not preview preserved vs reprovisioned state.
    DestructivePreviewMissing,
    /// Fallback path is claimed but not qualified.
    FallbackClaimedButUnqualified,
    /// Share/handoff token lacks explicit join mode or authority scope.
    ShareTokenImplicit,
    /// Control-plane outage drill not passed for a row claiming fallback.
    OutageDrillNotPassed,
    /// Persistence class or billing owner is not visible in degraded UI.
    PersistenceTruthHiddenInDegradedUi,
}

impl FinalizeManagedWorkspaceLifecycleNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::DescriptorMissing => "descriptor_missing",
            Self::LifecycleStateCollapsed => "lifecycle_state_collapsed",
            Self::SuspendResumeCheckpointIncomplete => "suspend_resume_checkpoint_incomplete",
            Self::DestructivePreviewMissing => "destructive_preview_missing",
            Self::FallbackClaimedButUnqualified => "fallback_claimed_but_unqualified",
            Self::ShareTokenImplicit => "share_token_implicit",
            Self::OutageDrillNotPassed => "outage_drill_not_passed",
            Self::PersistenceTruthHiddenInDegradedUi => "persistence_truth_hidden_in_degraded_ui",
        }
    }

    /// True when this reason is a hard guardrail that withdraws the packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::DescriptorMissing)
    }
}

// ---------------------------------------------------------------------------
// Row, summary, defect types
// ---------------------------------------------------------------------------

/// Stability qualification for one managed-workspace row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeManagedWorkspaceLifecycleTruthRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Row identifier.
    pub row_id: String,
    /// Current state token.
    pub state_token: String,
    /// Persistence class token.
    pub persistence_class_token: String,
    /// Fallback path token.
    pub fallback_path_token: String,
    /// True when fallback is qualified.
    pub fallback_qualified: bool,
    /// True when destructive previews are present.
    pub destructive_previews_present: bool,
    /// True when suspend/resume checkpoint is present and complete.
    pub suspend_resume_checkpoint_complete: bool,
    /// True when share tokens are explicit.
    pub share_tokens_explicit: bool,
    /// Derived qualification tier.
    pub qualification_token: String,
    /// Why the row was narrowed.
    pub narrow_reason_token: String,
    /// Plain-language summary.
    pub plain_language_summary: String,
}

/// Aggregate banner emitted with the truth page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FinalizeManagedWorkspaceLifecycleTruthSummary {
    /// Total workspace row count.
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Number of defects in this packet.
    pub defect_count: usize,
    /// Overall qualification token.
    pub overall_qualification_token: String,
}

impl FinalizeManagedWorkspaceLifecycleTruthSummary {
    fn from_rows(
        rows: &[FinalizeManagedWorkspaceLifecycleTruthRow],
        defects: &[FinalizeManagedWorkspaceLifecycleTruthDefect],
    ) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
        }
        let overall = if withdrawn > 0 {
            FinalizeManagedWorkspaceLifecycleQualificationClass::Withdrawn
        } else if preview > 0 {
            FinalizeManagedWorkspaceLifecycleQualificationClass::Preview
        } else if beta > 0 {
            FinalizeManagedWorkspaceLifecycleQualificationClass::Beta
        } else {
            FinalizeManagedWorkspaceLifecycleQualificationClass::Stable
        };
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            defect_count: defects.len(),
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

/// Typed defect emitted by the truth page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeManagedWorkspaceLifecycleTruthDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: FinalizeManagedWorkspaceLifecycleNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (row id or `input`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl FinalizeManagedWorkspaceLifecycleTruthDefect {
    fn new(
        narrow_reason: FinalizeManagedWorkspaceLifecycleNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_DEFECT_RECORD_KIND.to_owned(),
            schema_version: FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SCHEMA_VERSION,
            shared_contract_ref: FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SHARED_CONTRACT_REF
                .to_owned(),
            defect_id: format!(
                "policy:defect:finalize-managed-workspace-lifecycle-truth:{}:{}",
                narrow_reason.as_str(),
                &source_str
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source: source_str,
            note: note.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Main page
// ---------------------------------------------------------------------------

/// Stable proof packet for managed-workspace lifecycle truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeManagedWorkspaceLifecycleTruthPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC instant when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary.
    pub summary: FinalizeManagedWorkspaceLifecycleTruthSummary,
    /// Per-workspace stability rows.
    pub rows: Vec<FinalizeManagedWorkspaceLifecycleTruthRow>,
    /// Typed validation defects.
    pub defects: Vec<FinalizeManagedWorkspaceLifecycleTruthDefect>,
    /// The audited input embedded as evidence.
    pub input: ManagedWorkspaceLifecycleTruthInput,
}

impl FinalizeManagedWorkspaceLifecycleTruthPage {
    /// Builds the truth page from an input.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: ManagedWorkspaceLifecycleTruthInput,
    ) -> Self {
        let defects = audit_managed_workspace_lifecycle_input(&input);
        let rows = derive_lifecycle_rows(&input, &defects);
        let summary = FinalizeManagedWorkspaceLifecycleTruthSummary::from_rows(&rows, &defects);
        Self {
            record_kind: FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_PAGE_RECORD_KIND.to_owned(),
            schema_version: FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SCHEMA_VERSION,
            shared_contract_ref: FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SHARED_CONTRACT_REF
                .to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            defects,
            input,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == FinalizeManagedWorkspaceLifecycleQualificationClass::Stable.as_str()
    }

    /// True when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// True when all rows carry descriptors.
    pub fn all_rows_have_descriptors(&self) -> bool {
        self.input.all_rows_have_descriptors
    }

    /// True when lifecycle states are explicit across all rows.
    pub fn lifecycle_states_are_explicit(&self) -> bool {
        self.input
            .workspace_rows
            .iter()
            .all(|row| row.state_explicit)
    }

    /// True when all rows claiming fallback have passed an outage drill.
    pub fn fallback_drills_passed(&self) -> bool {
        self.input
            .workspace_rows
            .iter()
            .all(|row| !row.fallback_path.has_fallback() || row.outage_drill_passed)
    }

    /// True when share tokens are explicit across all rows.
    pub fn share_tokens_are_explicit(&self) -> bool {
        self.input.share_tokens_explicit
    }

    /// True when persistence truth is explicit.
    pub fn persistence_truth_is_explicit(&self) -> bool {
        self.input.persistence_truth_explicit
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeManagedWorkspaceLifecycleTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC export timestamp.
    pub generated_at: String,
    /// The truth page embedded as evidence.
    pub page: FinalizeManagedWorkspaceLifecycleTruthPage,
    /// Narrow-reason tokens present.
    pub narrow_reasons_present: Vec<FinalizeManagedWorkspaceLifecycleNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl FinalizeManagedWorkspaceLifecycleTruthSupportExport {
    /// Wraps a truth page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: FinalizeManagedWorkspaceLifecycleTruthPage,
    ) -> Self {
        let mut reasons: Vec<FinalizeManagedWorkspaceLifecycleNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        Self {
            record_kind: FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SUPPORT_EXPORT_RECORD_KIND
                .to_owned(),
            schema_version: FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SCHEMA_VERSION,
            shared_contract_ref: FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SHARED_CONTRACT_REF
                .to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit and validate functions
// ---------------------------------------------------------------------------

/// Re-runs the managed-workspace lifecycle audit over the embedded input.
pub fn audit_finalize_managed_workspace_lifecycle_truth_page(
    page: &FinalizeManagedWorkspaceLifecycleTruthPage,
) -> Vec<FinalizeManagedWorkspaceLifecycleTruthDefect> {
    audit_managed_workspace_lifecycle_input(&page.input)
}

/// Validates a truth page; returns `Ok` when the audit is clean.
pub fn validate_finalize_managed_workspace_lifecycle_truth_page(
    page: &FinalizeManagedWorkspaceLifecycleTruthPage,
) -> Result<(), Vec<FinalizeManagedWorkspaceLifecycleTruthDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Seeded helpers
// ---------------------------------------------------------------------------

/// Returns a seeded input with two well-formed managed-workspace rows.
pub fn seeded_managed_workspace_lifecycle_input() -> ManagedWorkspaceLifecycleTruthInput {
    ManagedWorkspaceLifecycleTruthInput {
        workspace_rows: vec![
            ManagedWorkspaceLifecycleInputRow {
                row_id: "managed-workspace-lifecycle-row:alpha".to_owned(),
                descriptor: ManagedWorkspaceDescriptor::new(
                    "ws-alpha",
                    "Alpha Workspace",
                    "org-example",
                    "us-west-2",
                    ManagedPersistenceClass::FullWorkspaceSnapshot,
                    "template:v2.1.0",
                    "billing:team-pro",
                    ManagedSecretModelClass::BrokerDelegatedWithRotation,
                    "suspend_after_30min_ttl_7d",
                    "control_plane_reachable",
                ),
                current_state: ManagedProvisioningStateClass::Ready,
                state_explicit: true,
                suspend_resume_checkpoint: Some(ManagedSuspendResumeCheckpoint {
                    retained_state_classes: vec![
                        "files".to_owned(),
                        "editor_state".to_owned(),
                        "restore_journal".to_owned(),
                    ],
                    drift_state_classes: vec!["forwarded_ports".to_owned()],
                    pinned_routes: vec!["ssh:2222".to_owned()],
                    same_live_environment: true,
                    resumed_snapshot: false,
                    local_journals_outside_boundary: vec!["unsaved_buffer_1".to_owned()],
                    summary: "Suspend preserves files, editor state, and restore journal; forwarded ports drift; attach returns to same live environment.".to_owned(),
                }),
                rebuild_plans: vec![
                    ManagedRebuildPlan::new(
                        ManagedDestructiveOperationClass::Rebuild,
                        vec!["persistent_volume".to_owned()],
                        vec!["runtime_image".to_owned(), "extensions".to_owned()],
                        vec!["local_unsaved_edits".to_owned()],
                        vec!["policy:rebuild_requires_approval".to_owned()],
                        "Rebuild resets compute quota but does not affect billing tier.",
                        "Rebuild preserves persistent volume; runtime image and extensions are reprovisioned.",
                    ),
                ],
                share_handoff_tokens: vec![
                    ManagedShareHandoffToken::new(
                        "ws-alpha",
                        "team-member@example.com",
                        "2026-06-10T00:00:00Z",
                        "same-project-scope",
                        ManagedJoinModeClass::SameLive,
                        "workspace-editor-read-write",
                    ),
                ],
                fallback_path: ManagedFallbackPathClass::LocalAndDirectRemote,
                fallback_qualified: true,
                outage_drill_passed: true,
                destructive_previews_present: true,
            },
            ManagedWorkspaceLifecycleInputRow {
                row_id: "managed-workspace-lifecycle-row:beta".to_owned(),
                descriptor: ManagedWorkspaceDescriptor::new(
                    "ws-beta",
                    "Beta Workspace",
                    "org-example",
                    "eu-central-1",
                    ManagedPersistenceClass::FilesAndEditorState,
                    "template:v1.8.0",
                    "billing:individual",
                    ManagedSecretModelClass::CustomerManagedNeverLeavesTenant,
                    "suspend_after_60min_ttl_14d",
                    "control_plane_reachable",
                ),
                current_state: ManagedProvisioningStateClass::Suspended,
                state_explicit: true,
                suspend_resume_checkpoint: Some(ManagedSuspendResumeCheckpoint {
                    retained_state_classes: vec!["files".to_owned(), "editor_state".to_owned()],
                    drift_state_classes: vec!["processes".to_owned(), "terminals".to_owned()],
                    pinned_routes: vec![],
                    same_live_environment: false,
                    resumed_snapshot: true,
                    local_journals_outside_boundary: vec![],
                    summary: "Resume attaches to a resumed snapshot, not the same live environment.".to_owned(),
                }),
                rebuild_plans: vec![],
                share_handoff_tokens: vec![],
                fallback_path: ManagedFallbackPathClass::LocalWorkspace,
                fallback_qualified: true,
                outage_drill_passed: true,
                destructive_previews_present: false,
            },
        ],
        vocabulary_consistent_across_surfaces: true,
        all_rows_have_descriptors: true,
        persistence_truth_explicit: true,
        share_tokens_explicit: true,
    }
}

/// Returns a seeded truth page built from the seeded input.
pub fn seeded_finalize_managed_workspace_lifecycle_truth_page(
) -> FinalizeManagedWorkspaceLifecycleTruthPage {
    FinalizeManagedWorkspaceLifecycleTruthPage::new(
        "policy:finalize-managed-workspace-lifecycle-truth:seeded",
        "Seeded managed-workspace lifecycle truth page",
        "2026-06-03T00:00:00Z",
        seeded_managed_workspace_lifecycle_input(),
    )
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn audit_managed_workspace_lifecycle_input(
    input: &ManagedWorkspaceLifecycleTruthInput,
) -> Vec<FinalizeManagedWorkspaceLifecycleTruthDefect> {
    let mut defects: Vec<FinalizeManagedWorkspaceLifecycleTruthDefect> = Vec::new();

    // Hard guardrail: any row lacking a descriptor withdraws the packet.
    if !input.all_rows_have_descriptors {
        defects.push(FinalizeManagedWorkspaceLifecycleTruthDefect::new(
            FinalizeManagedWorkspaceLifecycleNarrowReasonClass::DescriptorMissing,
            "input",
            "at least one managed-workspace row lacks a descriptor; every claimed managed row must carry workspace ID, org/tenant, region, persistence class, template version, quota/billing owner, secret model, and expiry policy",
        ));
    }
    for row in &input.workspace_rows {
        if row.descriptor.workspace_id.is_empty() {
            defects.push(FinalizeManagedWorkspaceLifecycleTruthDefect::new(
                FinalizeManagedWorkspaceLifecycleNarrowReasonClass::DescriptorMissing,
                &row.row_id,
                "managed-workspace descriptor is incomplete; workspace_id is required",
            ));
        }
    }
    if defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason())
    {
        return defects;
    }

    // Condition 1: Vocabulary consistent across surfaces.
    if !input.vocabulary_consistent_across_surfaces {
        defects.push(FinalizeManagedWorkspaceLifecycleTruthDefect::new(
            FinalizeManagedWorkspaceLifecycleNarrowReasonClass::LifecycleStateCollapsed,
            "input",
            "provisioning-state vocabulary is not consistent across all registered consumer surfaces; no surface may collapse queued, allocating, booting, attaching, sync_warming, ready, reconnecting, suspended, rebuilding, deleting, or failed into one generic loading banner",
        ));
    }

    // Condition 2: Lifecycle state explicit per row.
    for row in &input.workspace_rows {
        if !row.state_explicit {
            defects.push(FinalizeManagedWorkspaceLifecycleTruthDefect::new(
                FinalizeManagedWorkspaceLifecycleNarrowReasonClass::LifecycleStateCollapsed,
                &row.row_id,
                "lifecycle state is collapsed into a generic loading banner; the surface must show the explicit provisioning state",
            ));
        }
    }

    // Condition 3: Suspend/resume checkpoint completeness.
    for row in &input.workspace_rows {
        if matches!(
            row.current_state,
            ManagedProvisioningStateClass::Suspended | ManagedProvisioningStateClass::Ready
        ) {
            let checkpoint_complete = row
                .suspend_resume_checkpoint
                .as_ref()
                .map(|cp| {
                    !cp.retained_state_classes.is_empty() || !cp.drift_state_classes.is_empty()
                })
                .unwrap_or(false);
            if !checkpoint_complete {
                defects.push(FinalizeManagedWorkspaceLifecycleTruthDefect::new(
                    FinalizeManagedWorkspaceLifecycleNarrowReasonClass::SuspendResumeCheckpointIncomplete,
                    &row.row_id,
                    "suspend/resume checkpoint does not state what persists or drifts; the checkpoint must name retained state classes, drift state classes, pinned routes, and whether attach is to the same live environment or a resumed snapshot",
                ));
            }
        }
    }

    // Condition 4: Destructive-operation previews.
    for row in &input.workspace_rows {
        if !row.rebuild_plans.is_empty() && !row.destructive_previews_present {
            defects.push(FinalizeManagedWorkspaceLifecycleTruthDefect::new(
                FinalizeManagedWorkspaceLifecycleNarrowReasonClass::DestructivePreviewMissing,
                &row.row_id,
                "destructive-operation preview is missing; rebuild, recreate, reset, delete, and extend-TTL flows must preview which data persists, which is reprovisioned, what must be exported first, and which policy locks apply",
            ));
        }
    }

    // Condition 5: Fallback path declared and qualified.
    for row in &input.workspace_rows {
        if row.fallback_path.has_fallback() && !row.fallback_qualified {
            defects.push(FinalizeManagedWorkspaceLifecycleTruthDefect::new(
                FinalizeManagedWorkspaceLifecycleNarrowReasonClass::FallbackClaimedButUnqualified,
                &row.row_id,
                "fallback path is claimed but not qualified; local or direct-remote fallback must be proven in a control-plane outage drill",
            ));
        }
        if row.fallback_path.has_fallback() && !row.outage_drill_passed {
            defects.push(FinalizeManagedWorkspaceLifecycleTruthDefect::new(
                FinalizeManagedWorkspaceLifecycleNarrowReasonClass::OutageDrillNotPassed,
                &row.row_id,
                "control-plane outage drill not passed for a row claiming fallback; the drill must preserve local or direct-remote fallback and keep data-plane truth visible in degraded UI",
            ));
        }
    }

    // Condition 6: Share/handoff token explicit.
    if !input.share_tokens_explicit {
        defects.push(FinalizeManagedWorkspaceLifecycleTruthDefect::new(
            FinalizeManagedWorkspaceLifecycleNarrowReasonClass::ShareTokenImplicit,
            "input",
            "share or handoff tokens do not carry explicit join mode or authority scope; tokens must name same_live, resume_snapshot, or fresh_reprovision and declare authority scope",
        ));
    }
    for row in &input.workspace_rows {
        for token in &row.share_handoff_tokens {
            if token.join_mode_token.is_empty() || token.authority_scope.is_empty() {
                defects.push(FinalizeManagedWorkspaceLifecycleTruthDefect::new(
                    FinalizeManagedWorkspaceLifecycleNarrowReasonClass::ShareTokenImplicit,
                    &row.row_id,
                    "share/handoff token lacks join mode or authority scope",
                ));
            }
        }
    }

    // Condition 7: Persistence truth explicit.
    if !input.persistence_truth_explicit {
        defects.push(FinalizeManagedWorkspaceLifecycleTruthDefect::new(
            FinalizeManagedWorkspaceLifecycleNarrowReasonClass::PersistenceTruthHiddenInDegradedUi,
            "input",
            "persistence class and billing owner are not visible in degraded UI; control-plane outage drills must keep persistence class, join mode, and data-plane truth visible",
        ));
    }

    defects
}

fn derive_lifecycle_rows(
    input: &ManagedWorkspaceLifecycleTruthInput,
    page_defects: &[FinalizeManagedWorkspaceLifecycleTruthDefect],
) -> Vec<FinalizeManagedWorkspaceLifecycleTruthRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());

    let overall_qual = if has_withdrawal {
        FinalizeManagedWorkspaceLifecycleQualificationClass::Withdrawn
    } else if !page_defects.is_empty() {
        FinalizeManagedWorkspaceLifecycleQualificationClass::Beta
    } else {
        FinalizeManagedWorkspaceLifecycleQualificationClass::Stable
    };

    let leading_narrow_reason = if has_withdrawal {
        FinalizeManagedWorkspaceLifecycleNarrowReasonClass::DescriptorMissing
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        FinalizeManagedWorkspaceLifecycleNarrowReasonClass::NotNarrowed
    };

    let mut row_defects: BTreeMap<String, FinalizeManagedWorkspaceLifecycleNarrowReasonClass> =
        BTreeMap::new();
    for defect in page_defects {
        let entry = row_defects
            .entry(defect.source.clone())
            .or_insert(FinalizeManagedWorkspaceLifecycleNarrowReasonClass::NotNarrowed);
        if defect.narrow_reason.is_withdrawal_reason()
            || *entry == FinalizeManagedWorkspaceLifecycleNarrowReasonClass::NotNarrowed
        {
            *entry = defect.narrow_reason;
        }
    }
    let global_defect = row_defects
        .get("input")
        .copied()
        .unwrap_or(FinalizeManagedWorkspaceLifecycleNarrowReasonClass::NotNarrowed);

    input
        .workspace_rows
        .iter()
        .map(|row| {
            let per_profile = row_defects
                .get(&row.row_id)
                .copied()
                .unwrap_or(FinalizeManagedWorkspaceLifecycleNarrowReasonClass::NotNarrowed);
            let row_narrow = if has_withdrawal
                || per_profile.is_withdrawal_reason()
                || global_defect.is_withdrawal_reason()
            {
                leading_narrow_reason
            } else if per_profile != FinalizeManagedWorkspaceLifecycleNarrowReasonClass::NotNarrowed
            {
                per_profile
            } else if global_defect
                != FinalizeManagedWorkspaceLifecycleNarrowReasonClass::NotNarrowed
            {
                global_defect
            } else {
                FinalizeManagedWorkspaceLifecycleNarrowReasonClass::NotNarrowed
            };

            let row_qual = if has_withdrawal {
                FinalizeManagedWorkspaceLifecycleQualificationClass::Withdrawn
            } else {
                overall_qual
            };

            let summary = build_row_summary(&row.row_id, &row_qual, row_narrow);
            let checkpoint_complete = row
                .suspend_resume_checkpoint
                .as_ref()
                .map(|cp| {
                    !cp.retained_state_classes.is_empty() || !cp.drift_state_classes.is_empty()
                })
                .unwrap_or(false);

            FinalizeManagedWorkspaceLifecycleTruthRow {
                record_kind: FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_ROW_RECORD_KIND.to_owned(),
                schema_version: FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SCHEMA_VERSION,
                shared_contract_ref: FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SHARED_CONTRACT_REF
                    .to_owned(),
                row_id: row.row_id.clone(),
                state_token: row.current_state.as_str().to_owned(),
                persistence_class_token: row.descriptor.persistence_class_token.clone(),
                fallback_path_token: row.fallback_path.as_str().to_owned(),
                fallback_qualified: row.fallback_qualified,
                destructive_previews_present: row.destructive_previews_present,
                suspend_resume_checkpoint_complete: checkpoint_complete,
                share_tokens_explicit: row
                    .share_handoff_tokens
                    .iter()
                    .all(|t| !t.join_mode_token.is_empty() && !t.authority_scope.is_empty()),
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn build_row_summary(
    row_id: &str,
    qual: &FinalizeManagedWorkspaceLifecycleQualificationClass,
    narrow_reason: FinalizeManagedWorkspaceLifecycleNarrowReasonClass,
) -> String {
    match qual {
        FinalizeManagedWorkspaceLifecycleQualificationClass::Stable => {
            format!(
                "row '{}' qualifies stable; all managed-workspace lifecycle conditions hold",
                row_id
            )
        }
        FinalizeManagedWorkspaceLifecycleQualificationClass::Beta => {
            format!(
                "row '{}' narrowed to beta because {}",
                row_id,
                narrow_reason.as_str()
            )
        }
        FinalizeManagedWorkspaceLifecycleQualificationClass::Preview => {
            format!(
                "row '{}' narrowed to preview because {}",
                row_id,
                narrow_reason.as_str()
            )
        }
        FinalizeManagedWorkspaceLifecycleQualificationClass::Withdrawn => {
            format!(
                "row '{}' withdrawn because {}",
                row_id,
                narrow_reason.as_str()
            )
        }
    }
}
