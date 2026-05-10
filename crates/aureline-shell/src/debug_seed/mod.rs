//! Debug-prep seed surface: thin projection over the shared
//! execution-context object.
//!
//! The debug-prep seed is the protected-row entry point a user opens when
//! they need to ask "what target, debug-adapter runtime, and prerequisites
//! would a debug session attach to here?" without committing to full
//! debugger orchestration. It is a thin consumer of the canonical
//! [`aureline_runtime::ExecutionContext`] object and reuses the shared
//! [`crate::badges::target_origin`] vocabulary; the surface owns no target,
//! toolchain, or trust truth of its own.
//!
//! ## Why a thin projection
//!
//! Debug-prep launches need the same `target → runtime → prerequisites`
//! answer the terminal pane and task seed already render. Forking a
//! debug-only context resolver would let the lanes drift their target
//! taxonomy or trust vocabulary the moment one side upgrades. This module
//! projects from the shared record and limits its own truth to the
//! seed-scope action vocabulary plus a derived prerequisite list.
//!
//! ## Seed scope
//!
//! Live actions in this seed are limited to inspection, support-export
//! copy, and a hand-off back to the terminal pane that opens an interactive
//! shell on the same execution context. Attaching to a running process,
//! launching a debugged target, configuring breakpoint maps, and managing
//! debug profiles are reserved; those rows are surfaced verbatim with a
//! typed [`DebugPrepSeedActionAvailability::ReservedForLaterMilestone`]
//! label so the user can see the lane exists without overstating depth.
//!
//! ## Failure-drill posture
//!
//! When the resolved context carries a degraded field (pending trust,
//! activator blocked by policy or trust, target unreachable, capsule drift,
//! and so on) the surface lists each one as a typed
//! [`DebugPrepSeedPrerequisite`] row and lights `honesty_marker_present`.
//! The fixtures under `fixtures/runtime/task_debug_seed_cases/*.json`
//! exercise the protected walk on a trusted local seed and the failure
//! drill where pending trust surfaces an honesty row instead of a stale
//! "ready" label.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    DegradedFieldReason, DegradedFieldRecord, ExecutionContext, ReachabilityState, TargetClass,
    ToolchainClass, TrustState,
};

use crate::badges::target_origin::{
    BadgeEntryPoint, HostBoundaryCue, OriginBadgeClass, TargetBadgeClass, TargetOriginBadge,
};

/// Stable record-kind tag carried in serialized debug-prep payloads.
pub const DEBUG_PREP_SEED_SURFACE_RECORD_KIND: &str = "debug_prep_seed_surface_record";

/// Schema version for the [`DebugPrepSeedSurface`] payload shape.
pub const DEBUG_PREP_SEED_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Seed notice rendered on the surface so the user can see the lane's scope
/// without inferring it from the action labels alone.
pub const DEBUG_PREP_SEED_SCOPE_NOTICE: &str =
    "Debug-prep seed surface: live actions cover inspection, support-export copy, \
     and hand-off to a terminal session on this context. Attach, launch, breakpoint \
     management, and debug-profile orchestration are reserved for a later milestone.";

/// Frozen seed-action vocabulary for the debug-prep surface.
///
/// The classes split into two groups: the live ones every reviewer can
/// exercise on a protected row, and the reserved ones that publish the lane
/// without claiming depth the seed does not own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugPrepSeedActionClass {
    /// Open the shared execution-context inspector. Live in the seed.
    OpenExecutionContextInspector,
    /// Copy the surface payload (badge + summary) to the clipboard so a
    /// support packet can quote it. Live in the seed.
    CopyContextForSupportExport,
    /// Open a terminal session against the same execution context. Live in
    /// the seed; the terminal pane already owns the hand-off contract.
    OpenInvokingTerminal,
    /// Attach the debug adapter to a running process. Reserved.
    AttachToRunningProcess,
    /// Launch a debugged target. Reserved.
    LaunchDebuggedTarget,
    /// Configure breakpoint and watch maps. Reserved.
    ConfigureBreakpointMap,
    /// Manage debug-profile presets. Reserved.
    ManageDebugProfile,
}

impl DebugPrepSeedActionClass {
    /// Stable string token recorded on the action row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenExecutionContextInspector => "open_execution_context_inspector",
            Self::CopyContextForSupportExport => "copy_context_for_support_export",
            Self::OpenInvokingTerminal => "open_invoking_terminal",
            Self::AttachToRunningProcess => "attach_to_running_process",
            Self::LaunchDebuggedTarget => "launch_debugged_target",
            Self::ConfigureBreakpointMap => "configure_breakpoint_map",
            Self::ManageDebugProfile => "manage_debug_profile",
        }
    }

    /// Human-readable label for the action.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenExecutionContextInspector => "Open execution-context inspector",
            Self::CopyContextForSupportExport => "Copy context for support export",
            Self::OpenInvokingTerminal => "Open terminal on this context",
            Self::AttachToRunningProcess => "Attach to running process",
            Self::LaunchDebuggedTarget => "Launch debugged target",
            Self::ConfigureBreakpointMap => "Configure breakpoint map",
            Self::ManageDebugProfile => "Manage debug profile",
        }
    }

    const fn default_availability(self) -> DebugPrepSeedActionAvailability {
        match self {
            Self::OpenExecutionContextInspector
            | Self::CopyContextForSupportExport
            | Self::OpenInvokingTerminal => DebugPrepSeedActionAvailability::Live,
            Self::AttachToRunningProcess
            | Self::LaunchDebuggedTarget
            | Self::ConfigureBreakpointMap
            | Self::ManageDebugProfile => {
                DebugPrepSeedActionAvailability::ReservedForLaterMilestone
            }
        }
    }
}

/// Availability class rendered on every action row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugPrepSeedActionAvailability {
    /// Live within the seed.
    Live,
    /// Reserved for a later milestone; the surface labels the row so the
    /// user can see the lane exists but cannot run it yet.
    ReservedForLaterMilestone,
    /// Trust posture on the resolved context is unresolved; live work is
    /// withheld until the workspace trust prompt is settled.
    BlockedByPendingTrust,
    /// Target reachability or activator gate is blocked by policy.
    BlockedByPolicy,
    /// Resolver flagged a degraded field that prevents safe launch.
    BlockedByDegradedContext,
}

impl DebugPrepSeedActionAvailability {
    /// Stable string token recorded on the action row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::ReservedForLaterMilestone => "reserved_for_later_milestone",
            Self::BlockedByPendingTrust => "blocked_by_pending_trust",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::BlockedByDegradedContext => "blocked_by_degraded_context",
        }
    }

    /// Human-readable label for the chip.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Live => "Live",
            Self::ReservedForLaterMilestone => "Reserved for a later milestone",
            Self::BlockedByPendingTrust => "Blocked: trust pending",
            Self::BlockedByPolicy => "Blocked: policy gate",
            Self::BlockedByDegradedContext => "Blocked: degraded context",
        }
    }
}

/// One action row on the seed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugPrepSeedAction {
    pub action_class: DebugPrepSeedActionClass,
    pub action_class_token: String,
    pub label: String,
    pub availability: DebugPrepSeedActionAvailability,
    pub availability_token: String,
    pub availability_label: String,
}

impl DebugPrepSeedAction {
    fn build(
        action_class: DebugPrepSeedActionClass,
        availability: DebugPrepSeedActionAvailability,
    ) -> Self {
        Self {
            action_class,
            action_class_token: action_class.as_str().to_owned(),
            label: action_class.label().to_owned(),
            availability,
            availability_token: availability.as_str().to_owned(),
            availability_label: availability.label().to_owned(),
        }
    }
}

/// Why a prerequisite blocks live launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugPrepSeedPrerequisiteReasonClass {
    /// Workspace trust posture is unresolved.
    PendingTrust,
    /// Resolver recorded a degraded field on the context.
    DegradedContextField,
    /// Target reachability is policy-blocked.
    PolicyBlocked,
}

impl DebugPrepSeedPrerequisiteReasonClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingTrust => "pending_trust",
            Self::DegradedContextField => "degraded_context_field",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// One blocked-or-missing prerequisite row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugPrepSeedPrerequisite {
    pub reason_class: DebugPrepSeedPrerequisiteReasonClass,
    pub reason_class_token: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_reason: Option<DegradedFieldReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_reason_token: Option<String>,
}

impl DebugPrepSeedPrerequisite {
    fn pending_trust() -> Self {
        Self {
            reason_class: DebugPrepSeedPrerequisiteReasonClass::PendingTrust,
            reason_class_token: DebugPrepSeedPrerequisiteReasonClass::PendingTrust
                .as_str()
                .to_owned(),
            label: "Workspace trust prompt has not been settled".to_owned(),
            field_path: Some("policy_and_trust.trust_state".to_owned()),
            degraded_reason: Some(DegradedFieldReason::TrustStateUnresolved),
            degraded_reason_token: Some(
                DegradedFieldReason::TrustStateUnresolved
                    .as_str()
                    .to_owned(),
            ),
        }
    }

    fn policy_blocked_target() -> Self {
        Self {
            reason_class: DebugPrepSeedPrerequisiteReasonClass::PolicyBlocked,
            reason_class_token: DebugPrepSeedPrerequisiteReasonClass::PolicyBlocked
                .as_str()
                .to_owned(),
            label: "Target reachability is blocked by policy".to_owned(),
            field_path: Some("target_identity.reachability_state".to_owned()),
            degraded_reason: None,
            degraded_reason_token: None,
        }
    }

    fn from_degraded_field(record: &DegradedFieldRecord) -> Self {
        let reason_class = match record.reason {
            DegradedFieldReason::ActivatorBlockedByTrust
            | DegradedFieldReason::ActivatorBlockedByPolicy => {
                DebugPrepSeedPrerequisiteReasonClass::PolicyBlocked
            }
            DegradedFieldReason::TrustStateUnresolved => {
                DebugPrepSeedPrerequisiteReasonClass::PendingTrust
            }
            _ => DebugPrepSeedPrerequisiteReasonClass::DegradedContextField,
        };
        Self {
            reason_class,
            reason_class_token: reason_class.as_str().to_owned(),
            label: degraded_field_prerequisite_label(record).to_owned(),
            field_path: Some(record.field_path.clone()),
            degraded_reason: Some(record.reason),
            degraded_reason_token: Some(record.reason.as_str().to_owned()),
        }
    }
}

/// Debug-prep seed surface record.
///
/// Every value comes from the resolved
/// [`aureline_runtime::ExecutionContext`] (and the projected
/// [`crate::badges::target_origin::TargetOriginBadge`]); the surface owns
/// no target / toolchain / trust truth of its own.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugPrepSeedSurface {
    pub record_kind: String,
    pub schema_version: u32,
    pub entry_point: BadgeEntryPoint,
    pub workspace_id: String,
    pub execution_context_ref: String,
    pub badge: TargetOriginBadge,
    pub target_class: TargetBadgeClass,
    pub target_class_token: String,
    pub target_label: String,
    pub canonical_target_id: String,
    pub origin_class: OriginBadgeClass,
    pub origin_class_token: String,
    pub origin_label: String,
    pub boundary_cue: HostBoundaryCue,
    pub boundary_cue_token: String,
    pub boundary_cue_visible: bool,
    pub toolchain_class: ToolchainClass,
    pub toolchain_class_token: String,
    pub toolchain_class_label: String,
    pub toolchain_id: String,
    pub resolved_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    pub trust_state: TrustState,
    pub trust_state_token: String,
    pub seed_scope_notice: String,
    pub actions: Vec<DebugPrepSeedAction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocked_prerequisites: Vec<DebugPrepSeedPrerequisite>,
    pub honesty_marker_present: bool,
}

impl DebugPrepSeedSurface {
    /// Project a debug-prep seed surface from a resolved
    /// [`ExecutionContext`].
    pub fn project(context: &ExecutionContext) -> Self {
        let badge = TargetOriginBadge::project(BadgeEntryPoint::DebugPrepSeed, context);
        let prerequisites = collect_prerequisites(context);
        let trust_pending = matches!(
            context.policy_and_trust.trust_state,
            TrustState::PendingEvaluation
        );
        let policy_blocked = is_policy_blocked(context);
        let degraded = !context.degraded_fields.is_empty();

        let actions = build_actions(trust_pending, policy_blocked, degraded);

        let honesty_marker_present = badge.honesty_marker_present || !prerequisites.is_empty();

        Self {
            record_kind: DEBUG_PREP_SEED_SURFACE_RECORD_KIND.to_owned(),
            schema_version: DEBUG_PREP_SEED_SURFACE_SCHEMA_VERSION,
            entry_point: BadgeEntryPoint::DebugPrepSeed,
            workspace_id: context.invocation_subject.workspace_id.clone(),
            execution_context_ref: context.execution_context_id.clone(),
            target_class: badge.target_class,
            target_class_token: badge.target_class_token.clone(),
            target_label: badge.target_label.clone(),
            canonical_target_id: badge.canonical_target_id.clone(),
            origin_class: badge.origin_class,
            origin_class_token: badge.origin_class_token.clone(),
            origin_label: badge.origin_label.clone(),
            boundary_cue: badge.boundary_cue,
            boundary_cue_token: badge.boundary_cue_token.clone(),
            boundary_cue_visible: badge.boundary_cue_visible,
            toolchain_class: context.toolchain_identity.toolchain_class,
            toolchain_class_token: context
                .toolchain_identity
                .toolchain_class
                .as_str()
                .to_owned(),
            toolchain_class_label: toolchain_class_label(
                context.toolchain_identity.toolchain_class,
            )
            .to_owned(),
            toolchain_id: context.toolchain_identity.toolchain_id.clone(),
            resolved_version: context.toolchain_identity.resolved_version.clone(),
            working_directory: context.target_identity.working_directory.clone(),
            trust_state: context.policy_and_trust.trust_state,
            trust_state_token: trust_token(context.policy_and_trust.trust_state).to_owned(),
            seed_scope_notice: DEBUG_PREP_SEED_SCOPE_NOTICE.to_owned(),
            badge,
            actions,
            blocked_prerequisites: prerequisites,
            honesty_marker_present,
        }
    }

    /// Render a deterministic plaintext block for the copy-context action and
    /// support exports.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Debug-prep seed surface\n");
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!(
            "Execution context: {}\n",
            self.execution_context_ref
        ));
        out.push_str(&format!(
            "Target: {} ({})\n",
            self.target_label, self.target_class_token
        ));
        out.push_str(&format!(
            "Origin: {} ({})\n",
            self.origin_label, self.origin_class_token
        ));
        out.push_str(&format!(
            "Boundary cue: {} (visible: {})\n",
            self.boundary_cue_token, self.boundary_cue_visible
        ));
        out.push_str(&format!(
            "Runtime: {} ({})\n",
            self.toolchain_class_label, self.toolchain_class_token
        ));
        if let Some(cwd) = &self.working_directory {
            out.push_str(&format!("Working directory: {cwd}\n"));
        } else {
            out.push_str("Working directory: (not settled by resolver)\n");
        }
        out.push_str(&format!("Trust: {}\n", self.trust_state_token));
        out.push_str(&format!("Notice: {}\n", self.seed_scope_notice));
        out.push_str("\nActions:\n");
        for action in &self.actions {
            out.push_str(&format!(
                "  - {}: {} [{}]\n",
                action.action_class_token, action.label, action.availability_token,
            ));
        }
        if !self.blocked_prerequisites.is_empty() {
            out.push_str("\nBlocked / missing prerequisites:\n");
            for prereq in &self.blocked_prerequisites {
                out.push_str(&format!(
                    "  - [{}] {}",
                    prereq.reason_class_token, prereq.label
                ));
                if let Some(field) = &prereq.field_path {
                    out.push_str(&format!(" (field: {field})"));
                }
                if let Some(reason) = &prereq.degraded_reason_token {
                    out.push_str(&format!(" [{reason}]"));
                }
                out.push('\n');
            }
        }
        out
    }
}

fn build_actions(
    trust_pending: bool,
    policy_blocked: bool,
    degraded: bool,
) -> Vec<DebugPrepSeedAction> {
    let action_classes = [
        DebugPrepSeedActionClass::OpenExecutionContextInspector,
        DebugPrepSeedActionClass::CopyContextForSupportExport,
        DebugPrepSeedActionClass::OpenInvokingTerminal,
        DebugPrepSeedActionClass::AttachToRunningProcess,
        DebugPrepSeedActionClass::LaunchDebuggedTarget,
        DebugPrepSeedActionClass::ConfigureBreakpointMap,
        DebugPrepSeedActionClass::ManageDebugProfile,
    ];
    action_classes
        .into_iter()
        .map(|class| {
            let availability = adjust_availability(class, trust_pending, policy_blocked, degraded);
            DebugPrepSeedAction::build(class, availability)
        })
        .collect()
}

fn adjust_availability(
    class: DebugPrepSeedActionClass,
    trust_pending: bool,
    policy_blocked: bool,
    degraded: bool,
) -> DebugPrepSeedActionAvailability {
    let default = class.default_availability();
    if matches!(
        default,
        DebugPrepSeedActionAvailability::ReservedForLaterMilestone
    ) {
        return default;
    }
    if matches!(
        class,
        DebugPrepSeedActionClass::OpenExecutionContextInspector
            | DebugPrepSeedActionClass::CopyContextForSupportExport
    ) {
        return DebugPrepSeedActionAvailability::Live;
    }
    if trust_pending {
        return DebugPrepSeedActionAvailability::BlockedByPendingTrust;
    }
    if policy_blocked {
        return DebugPrepSeedActionAvailability::BlockedByPolicy;
    }
    if degraded {
        return DebugPrepSeedActionAvailability::BlockedByDegradedContext;
    }
    DebugPrepSeedActionAvailability::Live
}

fn collect_prerequisites(context: &ExecutionContext) -> Vec<DebugPrepSeedPrerequisite> {
    let mut rows = Vec::new();
    let mut saw_trust_unresolved = false;
    for record in &context.degraded_fields {
        if matches!(record.reason, DegradedFieldReason::TrustStateUnresolved) {
            saw_trust_unresolved = true;
        }
        rows.push(DebugPrepSeedPrerequisite::from_degraded_field(record));
    }
    if !saw_trust_unresolved
        && matches!(
            context.policy_and_trust.trust_state,
            TrustState::PendingEvaluation
        )
    {
        rows.insert(0, DebugPrepSeedPrerequisite::pending_trust());
    }
    if matches!(
        context.target_identity.reachability_state,
        ReachabilityState::PolicyBlocked
    ) && !rows.iter().any(|row| {
        matches!(
            row.degraded_reason,
            Some(DegradedFieldReason::ActivatorBlockedByPolicy)
        )
    }) {
        rows.push(DebugPrepSeedPrerequisite::policy_blocked_target());
    }
    rows
}

fn is_policy_blocked(context: &ExecutionContext) -> bool {
    if matches!(
        context.target_identity.reachability_state,
        ReachabilityState::PolicyBlocked
    ) {
        return true;
    }
    context.degraded_fields.iter().any(|record| {
        matches!(
            record.reason,
            DegradedFieldReason::ActivatorBlockedByPolicy
                | DegradedFieldReason::ActivatorBlockedByTrust
        )
    })
}

const fn trust_token(state: TrustState) -> &'static str {
    match state {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
        TrustState::PendingEvaluation => "pending_evaluation",
    }
}

const fn toolchain_class_label(class: ToolchainClass) -> &'static str {
    match class {
        ToolchainClass::Interpreter => "Interpreter",
        ToolchainClass::CompilerToolchain => "Compiler toolchain",
        ToolchainClass::PackageManagerRunner => "Package-manager runner",
        ToolchainClass::ContainerisedRuntime => "Containerised runtime",
        ToolchainClass::NotebookKernelRuntime => "Notebook kernel runtime",
        ToolchainClass::LanguageServerProcess => "Language-server process",
        ToolchainClass::DebugAdapterRuntime => "Debug-adapter runtime",
        ToolchainClass::TestRunnerRuntime => "Test-runner runtime",
        ToolchainClass::BuildDriverRuntime => "Build-driver runtime",
        ToolchainClass::AiToolRuntime => "AI tool runtime",
        ToolchainClass::LoginShell => "Login shell",
    }
}

const fn degraded_field_prerequisite_label(record: &DegradedFieldRecord) -> &'static str {
    match record.reason {
        DegradedFieldReason::ToolchainFallback => {
            "Toolchain resolved to a less-preferred lane; review before launch"
        }
        DegradedFieldReason::ActivatorBlockedByTrust => "Activator gate is blocked by trust policy",
        DegradedFieldReason::ActivatorBlockedByPolicy => "Activator gate is blocked by org policy",
        DegradedFieldReason::ActivatorUnsupportedOnTarget => {
            "Activator is unsupported on this target"
        }
        DegradedFieldReason::CapsuleUnresolved => "Environment capsule did not resolve",
        DegradedFieldReason::CapsuleDriftDetected => "Environment capsule drifted from inputs",
        DegradedFieldReason::TargetUnreachable => "Target is unreachable",
        DegradedFieldReason::PolicyEpochStale => "Policy epoch is stale; refresh before launching",
        DegradedFieldReason::TrustStateUnresolved => "Workspace trust prompt has not been settled",
        DegradedFieldReason::WorksetMemberUnavailable => "A workset member is unavailable",
        DegradedFieldReason::ProvenanceGap => "Provenance is incomplete for this lane",
        DegradedFieldReason::ConfidenceLow => "Resolver confidence is low",
        DegradedFieldReason::RemoteAgentScopeMismatch => "Remote-agent scope mismatch",
    }
}

#[allow(dead_code)]
const fn target_is_remote_or_managed(class: TargetClass) -> bool {
    !matches!(class, TargetClass::LocalHost)
}

#[cfg(test)]
mod tests;
