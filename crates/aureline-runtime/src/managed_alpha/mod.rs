//! Managed-workspace lifecycle truth for one preview/runtime alpha lane.
//!
//! This module owns a bounded [`ManagedWorkspaceAlphaRecord`] and the first
//! consumer projection, [`ManagedWorkspaceRuntimeInspection`]. The lane is
//! intentionally narrow: it models suspend, resume, reattach, and reconnect
//! truth for a single helper-backed preview/runtime row. It does not model a
//! general hosted workspace platform.

use serde::{Deserialize, Serialize};

use crate::execution_context::TargetClass;

/// Schema version of the managed-workspace alpha lane records.
pub const MANAGED_WORKSPACE_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for a managed-workspace lifecycle truth record.
pub const MANAGED_WORKSPACE_ALPHA_RECORD_KIND: &str = "managed_workspace_alpha_record";

/// Stable record-kind tag for runtime/preview inspection projections.
pub const MANAGED_WORKSPACE_RUNTIME_INSPECTION_RECORD_KIND: &str =
    "managed_workspace_runtime_inspection_record";

/// Stable record-kind tag for support/export projections.
pub const MANAGED_WORKSPACE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "managed_workspace_support_export_record";

/// Stable id for the only lane this alpha contract claims.
pub const MANAGED_WORKSPACE_ALPHA_LANE_ID: &str = "managed_workspace_preview_runtime_alpha";

/// Bounded lane scope for managed-workspace lifecycle truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedWorkspaceLaneScope {
    /// A single helper-backed preview/runtime inspection row.
    PreviewRuntimeInspection,
}

impl ManagedWorkspaceLaneScope {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewRuntimeInspection => "preview_runtime_inspection",
        }
    }
}

/// Runtime placement class surfaced by preview/runtime inspectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedRuntimePlacementClass {
    /// Work runs on the local desktop with no helper boundary.
    Local,
    /// Work runs behind a helper or remote agent boundary.
    HelperBacked,
    /// Work runs in a managed workspace instance.
    ManagedWorkspace,
}

impl ManagedRuntimePlacementClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::HelperBacked => "helper_backed",
            Self::ManagedWorkspace => "managed_workspace",
        }
    }

    /// True when the runtime is not the local desktop.
    pub const fn crosses_helper_boundary(self) -> bool {
        matches!(self, Self::HelperBacked | Self::ManagedWorkspace)
    }
}

/// Managed-workspace lifecycle state used by the alpha lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedWorkspaceLifecycleState {
    /// The workspace is available for normal work.
    Ready,
    /// Compute is paused and no live runtime traffic is allowed.
    Suspended,
    /// Resume has started but authority or runtime readiness is not complete.
    Resuming,
    /// Resume completed enough to inspect the runtime.
    Resumed,
    /// A reattach is in progress after a reconnect, wake, or restored session.
    Reattaching,
    /// Reattach completed against an accepted target witness.
    Reattached,
    /// Reconnect or reauthorization must happen before launch or mutation.
    ReconnectRequired,
    /// The last known runtime state is stale or target identity is not current.
    Stale,
    /// Only inspection is allowed; mutation and rerun are blocked.
    InspectOnly,
    /// The current environment cannot safely resume and needs rebuild review.
    RebuildRequired,
    /// The lane has no live reopen path in the current session.
    Closed,
}

impl ManagedWorkspaceLifecycleState {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Suspended => "suspended",
            Self::Resuming => "resuming",
            Self::Resumed => "resumed",
            Self::Reattaching => "reattaching",
            Self::Reattached => "reattached",
            Self::ReconnectRequired => "reconnect_required",
            Self::Stale => "stale",
            Self::InspectOnly => "inspect_only",
            Self::RebuildRequired => "rebuild_required",
            Self::Closed => "closed",
        }
    }
}

/// Reachability class for the managed runtime boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedReachabilityClass {
    /// The runtime is reachable and current.
    Reachable,
    /// The runtime is reachable only after session refresh or reauth.
    ReachablePendingReauth,
    /// Compute is suspended and no traffic should be forwarded.
    SuspendedNoTraffic,
    /// A reconnect path must be completed before launch or mutation.
    ReconnectRequired,
    /// The runtime cannot currently be reached.
    Unreachable,
    /// Policy blocks the runtime.
    PolicyBlocked,
}

impl ManagedReachabilityClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::ReachablePendingReauth => "reachable_pending_reauth",
            Self::SuspendedNoTraffic => "suspended_no_traffic",
            Self::ReconnectRequired => "reconnect_required",
            Self::Unreachable => "unreachable",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Freshness class for the target witness backing the runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedTargetFreshnessClass {
    /// The target witness is live and authoritative.
    AuthoritativeLive,
    /// The target witness is cached and may support inspection only.
    WarmCached,
    /// The target witness is stale and must not admit mutation.
    StaleTargetIdentity,
    /// The target witness cannot currently be verified.
    UnverifiableTarget,
}

impl ManagedTargetFreshnessClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::StaleTargetIdentity => "stale_target_identity",
            Self::UnverifiableTarget => "unverifiable_target",
        }
    }

    /// True when a surface must show a stale-state marker.
    pub const fn is_stale_for_live_runtime(self) -> bool {
        !matches!(self, Self::AuthoritativeLive)
    }
}

/// Reapproval requirement before execution can resume.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedReapprovalRequirementClass {
    /// No reapproval is required.
    NotRequired,
    /// Session ticket refresh is required before launch or mutation.
    SessionTicketRefreshRequired,
    /// Target witness review is required before launch or mutation.
    TargetWitnessReviewRequired,
    /// Policy reapproval is required before launch or mutation.
    PolicyReapprovalRequired,
    /// Rebuild review is required before this runtime can be used.
    RebuildReviewRequired,
}

impl ManagedReapprovalRequirementClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::SessionTicketRefreshRequired => "session_ticket_refresh_required",
            Self::TargetWitnessReviewRequired => "target_witness_review_required",
            Self::PolicyReapprovalRequired => "policy_reapproval_required",
            Self::RebuildReviewRequired => "rebuild_review_required",
        }
    }

    /// True when launch or mutation must wait for review.
    pub const fn requires_review(self) -> bool {
        !matches!(self, Self::NotRequired)
    }
}

/// Rerun posture for this managed runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedRerunPostureClass {
    /// Rerun can use the current context without review.
    CurrentContextSafe,
    /// Exact-prior rerun is possible only after a target/context review.
    ExactPriorReviewRequired,
    /// Reconnect must complete before rerun can be prepared.
    ReconnectBeforeRerun,
    /// Rerun is blocked; inspection and export remain available.
    BlockedInspectOnly,
}

impl ManagedRerunPostureClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentContextSafe => "current_context_safe",
            Self::ExactPriorReviewRequired => "exact_prior_review_required",
            Self::ReconnectBeforeRerun => "reconnect_before_rerun",
            Self::BlockedInspectOnly => "blocked_inspect_only",
        }
    }
}

/// Transition reason for a managed-workspace lifecycle row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedWorkspaceTransitionReason {
    /// The user or lifecycle policy suspended the workspace.
    UserRequestedSuspend,
    /// The user requested resume from a suspended workspace.
    UserRequestedResume,
    /// The user or restore flow requested reattach to an existing runtime.
    UserRequestedReattach,
    /// The reconnect window expired or transport must be reestablished.
    ReconnectWindowElapsed,
    /// The target witness became stale or unverifiable.
    TargetWitnessStale,
    /// The row is local-only and used only for comparison inspection.
    LocalRuntimeInspection,
}

impl ManagedWorkspaceTransitionReason {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserRequestedSuspend => "user_requested_suspend",
            Self::UserRequestedResume => "user_requested_resume",
            Self::UserRequestedReattach => "user_requested_reattach",
            Self::ReconnectWindowElapsed => "reconnect_window_elapsed",
            Self::TargetWitnessStale => "target_witness_stale",
            Self::LocalRuntimeInspection => "local_runtime_inspection",
        }
    }
}

/// Runtime inspection surface that consumes the alpha truth record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedWorkspaceInspectionSurface {
    /// Preview chrome or browser-runtime workspace.
    Preview,
    /// Runtime inspector detail card.
    RuntimeInspector,
    /// Support or headless export projection.
    SupportExport,
}

impl ManagedWorkspaceInspectionSurface {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::RuntimeInspector => "runtime_inspector",
            Self::SupportExport => "support_export",
        }
    }
}

/// Label vocabulary rendered by preview/runtime inspectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedRuntimeInspectionLabel {
    /// The inspected runtime is local.
    Local,
    /// The inspected runtime crosses a helper or remote-agent boundary.
    HelperBacked,
    /// The inspected runtime was resumed from a suspended state.
    Resumed,
    /// The inspected runtime was reattached after restore or reconnect.
    Reattached,
    /// The inspected runtime is stale relative to live execution authority.
    Stale,
    /// The inspected runtime permits inspection but not mutation or rerun.
    InspectOnly,
    /// Reconnect is required before launch, mutation, or rerun.
    ReconnectRequired,
}

impl ManagedRuntimeInspectionLabel {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::HelperBacked => "helper_backed",
            Self::Resumed => "resumed",
            Self::Reattached => "reattached",
            Self::Stale => "stale",
            Self::InspectOnly => "inspect_only",
            Self::ReconnectRequired => "reconnect_required",
        }
    }
}

/// Persisted and lost state classes after suspend/resume/reattach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedWorkspaceStateClass {
    /// Files and workspace data.
    Files,
    /// Running processes.
    Processes,
    /// Credential/session material.
    Credentials,
    /// Forwarded routes and ports.
    ForwardedPorts,
    /// Notebook kernels.
    NotebookKernels,
    /// Terminal processes and live PTYs.
    Terminals,
}

impl ManagedWorkspaceStateClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Files => "files",
            Self::Processes => "processes",
            Self::Credentials => "credentials",
            Self::ForwardedPorts => "forwarded_ports",
            Self::NotebookKernels => "notebook_kernels",
            Self::Terminals => "terminals",
        }
    }
}

/// Continuity truth after suspend, resume, or reattach.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceContinuity {
    /// State classes preserved across the lifecycle transition.
    #[serde(default)]
    pub preserved_state_classes: Vec<ManagedWorkspaceStateClass>,
    /// State classes lost or requiring reattach after the transition.
    #[serde(default)]
    pub lost_state_classes: Vec<ManagedWorkspaceStateClass>,
    /// Export-safe refs that prove the preserved state.
    #[serde(default)]
    pub preserved_snapshot_refs: Vec<String>,
    /// Export-safe refs that prove cancellation or teardown.
    #[serde(default)]
    pub cancelled_action_refs: Vec<String>,
    /// User-visible continuity summary.
    pub summary: String,
}

impl ManagedWorkspaceContinuity {
    /// Returns stable tokens for state classes that were preserved.
    pub fn preserved_tokens(&self) -> Vec<String> {
        self.preserved_state_classes
            .iter()
            .map(|state| state.as_str().to_owned())
            .collect()
    }

    /// Returns stable tokens for state classes that were lost.
    pub fn lost_tokens(&self) -> Vec<String> {
        self.lost_state_classes
            .iter()
            .map(|state| state.as_str().to_owned())
            .collect()
    }
}

/// Helper and target-boundary truth for a managed runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceBoundary {
    /// True when local-vs-helper boundary chrome must be shown.
    pub helper_boundary_visible: bool,
    /// Runtime reachability state.
    pub reachability_class: ManagedReachabilityClass,
    /// Opaque helper session ref when a helper-backed target is involved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helper_session_ref: Option<String>,
    /// Opaque capability-envelope ref from helper negotiation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helper_capability_envelope_ref: Option<String>,
    /// Opaque remote-attach session ref when attach truth exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_attach_session_ref: Option<String>,
    /// Opaque managed control-plane ref when a control plane is involved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_control_plane_ref: Option<String>,
}

/// One transition row for suspend, resume, reattach, or reconnect-required truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceTransition {
    /// Opaque transition id.
    pub transition_id: String,
    /// Prior lifecycle state when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_state: Option<ManagedWorkspaceLifecycleState>,
    /// New lifecycle state.
    pub to_state: ManagedWorkspaceLifecycleState,
    /// Reason for the transition.
    pub reason: ManagedWorkspaceTransitionReason,
    /// Timestamp supplied by the caller.
    pub observed_at: String,
    /// Export-safe evidence refs for this transition.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

/// Canonical alpha record for one managed-workspace preview/runtime lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceAlphaRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable lane id for this bounded prototype.
    pub lane_id: String,
    /// Bounded lane scope.
    pub lane_scope: ManagedWorkspaceLaneScope,
    /// Timestamp supplied by the caller.
    pub captured_at: String,
    /// Timestamp for the latest lifecycle update.
    pub updated_at: String,
    /// Opaque workspace ref.
    pub managed_workspace_ref: String,
    /// Opaque runtime ref shown by runtime inspection surfaces.
    pub runtime_ref: String,
    /// Opaque preview ref shown by preview surfaces.
    pub preview_ref: String,
    /// Execution-context ref backing this runtime.
    pub execution_context_ref: String,
    /// Target class resolved by the shared execution-context model.
    pub target_class: TargetClass,
    /// Canonical target id from the shared execution-context model.
    pub canonical_target_id: String,
    /// Target witness ref that must remain stable across resume/reattach.
    pub target_identity_witness_ref: String,
    /// Prior target witness ref when a stale or reattach comparison exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_target_identity_witness_ref: Option<String>,
    /// Effective lifecycle state.
    pub lifecycle_state: ManagedWorkspaceLifecycleState,
    /// Runtime placement shown by inspection surfaces.
    pub placement_class: ManagedRuntimePlacementClass,
    /// Target freshness class.
    pub target_freshness_class: ManagedTargetFreshnessClass,
    /// Reapproval requirement before mutation or launch.
    pub reapproval_requirement_class: ManagedReapprovalRequirementClass,
    /// Rerun posture for this runtime.
    pub rerun_posture_class: ManagedRerunPostureClass,
    /// State preserved or lost across the lifecycle transition.
    pub continuity: ManagedWorkspaceContinuity,
    /// Helper and target-boundary truth.
    pub boundary: ManagedWorkspaceBoundary,
    /// Lifecycle transition that produced the state.
    pub transition: ManagedWorkspaceTransition,
    /// Repair actions exposed by the consuming surface.
    #[serde(default)]
    pub repair_action_refs: Vec<String>,
    /// Export-safe evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Review-safe summary.
    pub summary: String,
}

impl ManagedWorkspaceAlphaRecord {
    /// Returns a runtime/preview inspection projection for this truth record.
    pub fn runtime_inspection(
        &self,
        surface: ManagedWorkspaceInspectionSurface,
        inspected_at: impl Into<String>,
    ) -> ManagedWorkspaceRuntimeInspection {
        let mutation_allowed = self.mutation_allowed();
        let reconnect_required = self.reconnect_required();
        let requires_reapproval = self.reapproval_requirement_class.requires_review();
        let inspect_only = self.inspect_only();
        let labels = self.runtime_labels(inspect_only, reconnect_required);
        let label_tokens = labels
            .iter()
            .map(|label| label.as_str().to_owned())
            .collect::<Vec<_>>();
        let stale_state_tokens = self.stale_state_tokens();
        let blocked_action_tokens = self.blocked_action_tokens(mutation_allowed);
        let summary_line = format!(
            "runtime={} state={} placement={} labels={} mutation_allowed={} rerun_posture={}",
            self.runtime_ref,
            self.lifecycle_state.as_str(),
            self.placement_class.as_str(),
            if label_tokens.is_empty() {
                "none".to_owned()
            } else {
                label_tokens.join("|")
            },
            mutation_allowed,
            self.rerun_posture_class.as_str()
        );

        ManagedWorkspaceRuntimeInspection {
            record_kind: MANAGED_WORKSPACE_RUNTIME_INSPECTION_RECORD_KIND.to_owned(),
            schema_version: MANAGED_WORKSPACE_ALPHA_SCHEMA_VERSION,
            inspection_id: format!(
                "managed_workspace_runtime_inspection:{}:{}",
                self.runtime_ref,
                surface.as_str()
            ),
            inspected_at: inspected_at.into(),
            source_record_ref: self.record_ref(),
            lane_id: self.lane_id.clone(),
            lane_scope: self.lane_scope,
            surface,
            surface_token: surface.as_str().to_owned(),
            runtime_ref: self.runtime_ref.clone(),
            preview_ref: self.preview_ref.clone(),
            execution_context_ref: self.execution_context_ref.clone(),
            target_class: self.target_class,
            target_class_token: self.target_class.as_str().to_owned(),
            canonical_target_id: self.canonical_target_id.clone(),
            target_identity_witness_ref: self.target_identity_witness_ref.clone(),
            prior_target_identity_witness_ref: self.prior_target_identity_witness_ref.clone(),
            lifecycle_state: self.lifecycle_state,
            lifecycle_state_token: self.lifecycle_state.as_str().to_owned(),
            placement_class: self.placement_class,
            placement_class_token: self.placement_class.as_str().to_owned(),
            target_freshness_class: self.target_freshness_class,
            target_freshness_token: self.target_freshness_class.as_str().to_owned(),
            reachability_class: self.boundary.reachability_class,
            reachability_token: self.boundary.reachability_class.as_str().to_owned(),
            reapproval_requirement_class: self.reapproval_requirement_class,
            reapproval_requirement_token: self.reapproval_requirement_class.as_str().to_owned(),
            rerun_posture_class: self.rerun_posture_class,
            rerun_posture_token: self.rerun_posture_class.as_str().to_owned(),
            labels,
            label_tokens,
            helper_boundary_visible: self.boundary.helper_boundary_visible,
            mutation_allowed,
            inspect_only,
            reconnect_required,
            requires_reapproval,
            stale_state_tokens,
            blocked_action_tokens,
            preserved_state_tokens: self.continuity.preserved_tokens(),
            lost_state_tokens: self.continuity.lost_tokens(),
            repair_action_refs: self.repair_action_refs.clone(),
            evidence_refs: self.combined_evidence_refs(),
            summary_line,
        }
    }

    /// Returns validation issues that would make this row overclaim runtime truth.
    pub fn validation_issues(&self) -> Vec<ManagedWorkspaceAlphaViolation> {
        let mut issues = Vec::new();
        if self.record_kind != MANAGED_WORKSPACE_ALPHA_RECORD_KIND {
            issues.push(ManagedWorkspaceAlphaViolation::new(
                "unexpected_record_kind",
                "record_kind",
                "managed workspace record kind must stay canonical",
            ));
        }
        if self.schema_version != MANAGED_WORKSPACE_ALPHA_SCHEMA_VERSION {
            issues.push(ManagedWorkspaceAlphaViolation::new(
                "unexpected_schema_version",
                "schema_version",
                "managed workspace schema version must match this crate",
            ));
        }
        if self.lane_id != MANAGED_WORKSPACE_ALPHA_LANE_ID {
            issues.push(ManagedWorkspaceAlphaViolation::new(
                "unbounded_lane_id",
                "lane_id",
                "managed workspace alpha lane must stay on the bounded lane id",
            ));
        }
        if self.lane_scope != ManagedWorkspaceLaneScope::PreviewRuntimeInspection {
            issues.push(ManagedWorkspaceAlphaViolation::new(
                "unbounded_lane_scope",
                "lane_scope",
                "managed workspace alpha lane must stay scoped to preview/runtime inspection",
            ));
        }
        if self.lifecycle_state != self.transition.to_state {
            issues.push(ManagedWorkspaceAlphaViolation::new(
                "transition_state_mismatch",
                "transition.to_state",
                "transition target must match the effective lifecycle state",
            ));
        }
        if self.placement_class == ManagedRuntimePlacementClass::Local
            && self.target_class != TargetClass::LocalHost
        {
            issues.push(ManagedWorkspaceAlphaViolation::new(
                "local_runtime_target_mismatch",
                "target_class",
                "local inspection rows must point at the local host target class",
            ));
        }
        if self.placement_class.crosses_helper_boundary() && !self.boundary.helper_boundary_visible
        {
            issues.push(ManagedWorkspaceAlphaViolation::new(
                "missing_helper_boundary",
                "boundary.helper_boundary_visible",
                "helper-backed runtime rows must keep the boundary cue visible",
            ));
        }
        if self.reconnect_required() && self.mutation_allowed() {
            issues.push(ManagedWorkspaceAlphaViolation::new(
                "reconnect_allows_mutation",
                "boundary.reachability_class",
                "reconnect-required rows must not permit mutation",
            ));
        }
        if self.reapproval_requirement_class.requires_review() && self.mutation_allowed() {
            issues.push(ManagedWorkspaceAlphaViolation::new(
                "reapproval_allows_mutation",
                "reapproval_requirement_class",
                "rows requiring reapproval must not permit mutation",
            ));
        }
        if self.target_freshness_class.is_stale_for_live_runtime()
            && !self
                .runtime_inspection(ManagedWorkspaceInspectionSurface::RuntimeInspector, "")
                .labels
                .contains(&ManagedRuntimeInspectionLabel::Stale)
        {
            issues.push(ManagedWorkspaceAlphaViolation::new(
                "stale_target_without_label",
                "target_freshness_class",
                "stale target freshness must produce a stale inspection label",
            ));
        }
        if matches!(
            self.lifecycle_state,
            ManagedWorkspaceLifecycleState::Suspended
                | ManagedWorkspaceLifecycleState::Resumed
                | ManagedWorkspaceLifecycleState::Reattached
        ) && self.continuity.preserved_state_classes.is_empty()
            && self.continuity.lost_state_classes.is_empty()
        {
            issues.push(ManagedWorkspaceAlphaViolation::new(
                "missing_continuity_truth",
                "continuity",
                "suspend/resume/reattach rows must declare preserved or lost state classes",
            ));
        }
        issues
    }

    /// Returns a stable support/export ref for this record.
    pub fn record_ref(&self) -> String {
        format!(
            "{}:{}",
            MANAGED_WORKSPACE_ALPHA_RECORD_KIND, self.managed_workspace_ref
        )
    }

    fn mutation_allowed(&self) -> bool {
        matches!(
            self.lifecycle_state,
            ManagedWorkspaceLifecycleState::Ready
                | ManagedWorkspaceLifecycleState::Resumed
                | ManagedWorkspaceLifecycleState::Reattached
        ) && self.boundary.reachability_class == ManagedReachabilityClass::Reachable
            && self.target_freshness_class == ManagedTargetFreshnessClass::AuthoritativeLive
            && self.reapproval_requirement_class == ManagedReapprovalRequirementClass::NotRequired
            && self.rerun_posture_class == ManagedRerunPostureClass::CurrentContextSafe
    }

    fn inspect_only(&self) -> bool {
        !self.mutation_allowed()
            || matches!(
                self.lifecycle_state,
                ManagedWorkspaceLifecycleState::Suspended
                    | ManagedWorkspaceLifecycleState::ReconnectRequired
                    | ManagedWorkspaceLifecycleState::Stale
                    | ManagedWorkspaceLifecycleState::InspectOnly
                    | ManagedWorkspaceLifecycleState::RebuildRequired
                    | ManagedWorkspaceLifecycleState::Closed
            )
    }

    fn reconnect_required(&self) -> bool {
        self.lifecycle_state == ManagedWorkspaceLifecycleState::ReconnectRequired
            || self.boundary.reachability_class == ManagedReachabilityClass::ReconnectRequired
            || self.rerun_posture_class == ManagedRerunPostureClass::ReconnectBeforeRerun
    }

    fn runtime_labels(
        &self,
        inspect_only: bool,
        reconnect_required: bool,
    ) -> Vec<ManagedRuntimeInspectionLabel> {
        let mut labels = Vec::new();
        if self.placement_class == ManagedRuntimePlacementClass::Local {
            push_label(&mut labels, ManagedRuntimeInspectionLabel::Local);
        } else {
            push_label(&mut labels, ManagedRuntimeInspectionLabel::HelperBacked);
        }
        if matches!(
            self.lifecycle_state,
            ManagedWorkspaceLifecycleState::Resumed | ManagedWorkspaceLifecycleState::Reattached
        ) {
            push_label(&mut labels, ManagedRuntimeInspectionLabel::Resumed);
        }
        if self.lifecycle_state == ManagedWorkspaceLifecycleState::Reattached {
            push_label(&mut labels, ManagedRuntimeInspectionLabel::Reattached);
        }
        if self.target_freshness_class.is_stale_for_live_runtime()
            || matches!(
                self.lifecycle_state,
                ManagedWorkspaceLifecycleState::Suspended
                    | ManagedWorkspaceLifecycleState::Stale
                    | ManagedWorkspaceLifecycleState::ReconnectRequired
                    | ManagedWorkspaceLifecycleState::InspectOnly
            )
        {
            push_label(&mut labels, ManagedRuntimeInspectionLabel::Stale);
        }
        if inspect_only {
            push_label(&mut labels, ManagedRuntimeInspectionLabel::InspectOnly);
        }
        if reconnect_required {
            push_label(
                &mut labels,
                ManagedRuntimeInspectionLabel::ReconnectRequired,
            );
        }
        labels
    }

    fn stale_state_tokens(&self) -> Vec<String> {
        let mut tokens = Vec::new();
        if self.target_freshness_class.is_stale_for_live_runtime() {
            tokens.push(self.target_freshness_class.as_str().to_owned());
        }
        if matches!(
            self.lifecycle_state,
            ManagedWorkspaceLifecycleState::Suspended
                | ManagedWorkspaceLifecycleState::Stale
                | ManagedWorkspaceLifecycleState::ReconnectRequired
                | ManagedWorkspaceLifecycleState::InspectOnly
        ) {
            tokens.push(self.lifecycle_state.as_str().to_owned());
        }
        tokens
    }

    fn blocked_action_tokens(&self, mutation_allowed: bool) -> Vec<String> {
        if mutation_allowed {
            return Vec::new();
        }
        let mut tokens = Vec::new();
        match self.rerun_posture_class {
            ManagedRerunPostureClass::CurrentContextSafe => {}
            ManagedRerunPostureClass::ExactPriorReviewRequired => {
                tokens.push("rerun_requires_target_review".to_owned())
            }
            ManagedRerunPostureClass::ReconnectBeforeRerun => {
                tokens.push("rerun_requires_reconnect".to_owned())
            }
            ManagedRerunPostureClass::BlockedInspectOnly => {
                tokens.push("rerun_blocked_inspect_only".to_owned())
            }
        }
        if self.reapproval_requirement_class.requires_review() {
            tokens.push(format!(
                "reapproval:{}",
                self.reapproval_requirement_class.as_str()
            ));
        }
        if self.boundary.reachability_class != ManagedReachabilityClass::Reachable {
            tokens.push(format!(
                "reachability:{}",
                self.boundary.reachability_class.as_str()
            ));
        }
        tokens
    }

    fn combined_evidence_refs(&self) -> Vec<String> {
        let mut refs = self.evidence_refs.clone();
        for evidence_ref in &self.transition.evidence_refs {
            if !refs.contains(evidence_ref) {
                refs.push(evidence_ref.clone());
            }
        }
        refs
    }
}

/// Runtime/preview inspection projection consumed by UI, CLI, or support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceRuntimeInspection {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Opaque inspection id.
    pub inspection_id: String,
    /// Timestamp supplied by the caller.
    pub inspected_at: String,
    /// Source managed-workspace alpha record ref.
    pub source_record_ref: String,
    /// Stable lane id.
    pub lane_id: String,
    /// Bounded lane scope.
    pub lane_scope: ManagedWorkspaceLaneScope,
    /// Consuming inspection surface.
    pub surface: ManagedWorkspaceInspectionSurface,
    /// Stable consuming-surface token.
    pub surface_token: String,
    /// Opaque runtime ref.
    pub runtime_ref: String,
    /// Opaque preview ref.
    pub preview_ref: String,
    /// Execution-context ref backing this runtime.
    pub execution_context_ref: String,
    /// Shared execution-context target class.
    pub target_class: TargetClass,
    /// Stable target-class token.
    pub target_class_token: String,
    /// Canonical target id from the shared execution-context model.
    pub canonical_target_id: String,
    /// Current target witness ref.
    pub target_identity_witness_ref: String,
    /// Prior target witness ref when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_target_identity_witness_ref: Option<String>,
    /// Effective lifecycle state.
    pub lifecycle_state: ManagedWorkspaceLifecycleState,
    /// Stable lifecycle-state token.
    pub lifecycle_state_token: String,
    /// Runtime placement class.
    pub placement_class: ManagedRuntimePlacementClass,
    /// Stable placement token.
    pub placement_class_token: String,
    /// Target freshness class.
    pub target_freshness_class: ManagedTargetFreshnessClass,
    /// Stable target freshness token.
    pub target_freshness_token: String,
    /// Runtime reachability class.
    pub reachability_class: ManagedReachabilityClass,
    /// Stable reachability token.
    pub reachability_token: String,
    /// Reapproval requirement class.
    pub reapproval_requirement_class: ManagedReapprovalRequirementClass,
    /// Stable reapproval token.
    pub reapproval_requirement_token: String,
    /// Rerun posture class.
    pub rerun_posture_class: ManagedRerunPostureClass,
    /// Stable rerun posture token.
    pub rerun_posture_token: String,
    /// Typed labels rendered by preview/runtime surfaces.
    pub labels: Vec<ManagedRuntimeInspectionLabel>,
    /// Stable label tokens.
    pub label_tokens: Vec<String>,
    /// True when local-vs-helper boundary chrome must be shown.
    pub helper_boundary_visible: bool,
    /// True when launch or mutation is currently allowed.
    pub mutation_allowed: bool,
    /// True when inspection is allowed but mutation or rerun is blocked.
    pub inspect_only: bool,
    /// True when reconnect is required before launch, mutation, or rerun.
    pub reconnect_required: bool,
    /// True when reapproval is required before launch or mutation.
    pub requires_reapproval: bool,
    /// Stable stale-state tokens.
    #[serde(default)]
    pub stale_state_tokens: Vec<String>,
    /// Stable blocked-action tokens.
    #[serde(default)]
    pub blocked_action_tokens: Vec<String>,
    /// State classes preserved across the transition.
    #[serde(default)]
    pub preserved_state_tokens: Vec<String>,
    /// State classes lost or requiring reattach.
    #[serde(default)]
    pub lost_state_tokens: Vec<String>,
    /// Repair actions exposed by the consuming surface.
    #[serde(default)]
    pub repair_action_refs: Vec<String>,
    /// Export-safe evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Compact support/export line.
    pub summary_line: String,
}

impl ManagedWorkspaceRuntimeInspection {
    /// True when this inspection carries the requested label.
    pub fn has_label(&self, label: ManagedRuntimeInspectionLabel) -> bool {
        self.labels.contains(&label)
    }
}

/// Support/export projection for managed-workspace alpha truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Export id supplied by the caller.
    pub export_id: String,
    /// Timestamp supplied by the caller.
    pub generated_at: String,
    /// Stable lane id.
    pub lane_id: String,
    /// Bounded lane scope.
    pub lane_scope: ManagedWorkspaceLaneScope,
    /// Inspection rows included in the export.
    pub inspections: Vec<ManagedWorkspaceRuntimeInspection>,
}

impl ManagedWorkspaceSupportExport {
    /// Builds a support/export projection from managed-workspace alpha records.
    pub fn from_records(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        records: &[ManagedWorkspaceAlphaRecord],
    ) -> Self {
        let generated_at = generated_at.into();
        Self {
            record_kind: MANAGED_WORKSPACE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MANAGED_WORKSPACE_ALPHA_SCHEMA_VERSION,
            export_id: export_id.into(),
            generated_at: generated_at.clone(),
            lane_id: MANAGED_WORKSPACE_ALPHA_LANE_ID.to_owned(),
            lane_scope: ManagedWorkspaceLaneScope::PreviewRuntimeInspection,
            inspections: records
                .iter()
                .map(|record| {
                    record.runtime_inspection(
                        ManagedWorkspaceInspectionSurface::SupportExport,
                        generated_at.clone(),
                    )
                })
                .collect(),
        }
    }

    /// Renders stable plaintext lines for support exports.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!("Managed workspace support export: {}\n", self.export_id);
        for inspection in &self.inspections {
            out.push_str(&inspection.summary_line);
            out.push('\n');
        }
        out
    }
}

/// Validation issue raised when a managed-workspace row overclaims truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceAlphaViolation {
    /// Stable violation code.
    pub code: String,
    /// Dotted field path responsible for the issue.
    pub field_path: String,
    /// Review-safe issue summary.
    pub summary: String,
}

impl ManagedWorkspaceAlphaViolation {
    /// Creates a validation issue.
    pub fn new(
        code: impl Into<String>,
        field_path: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            field_path: field_path.into(),
            summary: summary.into(),
        }
    }
}

fn push_label(
    labels: &mut Vec<ManagedRuntimeInspectionLabel>,
    label: ManagedRuntimeInspectionLabel,
) {
    if !labels.contains(&label) {
        labels.push(label);
    }
}
