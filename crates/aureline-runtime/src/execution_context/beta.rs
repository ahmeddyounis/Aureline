//! Beta finalize layer for the execution-context resolver.
//!
//! This module pins the closed set of beta execution-context lanes (local,
//! remote, container, and request-workspace), the [`TargetClass`] tokens each
//! lane is allowed to mint, and the consumer surfaces that resolve through
//! the lane. The lane manifest plus the ticket-drift evaluator land the third
//! acceptance bullet of the beta promote: any stored ticket or preview whose
//! binding disagrees with the freshly resolved context is invalidated with a
//! typed reason set instead of silently dispatching against the wrong target.
//!
//! The canonical type definitions still live in [`super`]; this module pins
//! them at the beta finalize boundary so terminal, task, test, debug, AI, and
//! request-workspace consumers read the same lane manifest, the same drift
//! evaluator, and the same support export shape without forking their own
//! parsers.
//!
//! The machine-readable boundary lives at
//! [`/schemas/execution/execution_context.schema.json`](../../../../schemas/execution/execution_context.schema.json)
//! and the reviewer-facing companion doc at
//! [`/docs/runtime/m3/execution_context_beta.md`](../../../../docs/runtime/m3/execution_context_beta.md).

use serde::{Deserialize, Serialize};

use super::{
    ActorClass, CapsuleDriftState, ExecutionContext, ExecutionContextRequest, ScopeClass,
    SurfaceClass, TargetClass, ToolchainClass, TrustState, EXECUTION_CONTEXT_SCHEMA_VERSION,
};

/// Stable record-kind tag for the beta lane coverage manifest.
pub const EXECUTION_CONTEXT_BETA_COVERAGE_MANIFEST_RECORD_KIND: &str =
    "execution_context_beta_coverage_manifest_record";

/// Stable record-kind tag for the beta support-export packet.
pub const EXECUTION_CONTEXT_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "execution_context_beta_support_export_record";

/// Stable record-kind tag for the ticket-drift evaluation record.
pub const EXECUTION_CONTEXT_TICKET_DRIFT_RECORD_KIND: &str =
    "execution_context_ticket_drift_record";

/// Schema-version tag the beta layer republishes so downstream consumers can
/// compare against the boundary schema without reading the seed crate first.
pub const EXECUTION_CONTEXT_BETA_SCHEMA_VERSION: u32 = EXECUTION_CONTEXT_SCHEMA_VERSION;

/// Beta execution-context lane declared by the canonical resolver model.
///
/// Adding or removing a lane is a vocabulary change that MUST update the
/// canonical schema, the reviewer doc, and the lane-coverage fixture together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionContextBetaLane {
    /// Local-host lane covering the host desktop and local notebook kernels.
    LocalHost,
    /// Remote-attach lane covering SSH-attached hosts and remote notebook
    /// kernels.
    RemoteAttach,
    /// Container lane covering ad-hoc local containers and devcontainers.
    Container,
    /// Request-workspace lane covering managed / remote-workspace VMs,
    /// prebuild runtimes, and the AI sandbox.
    RequestWorkspace,
}

impl ExecutionContextBetaLane {
    /// All beta execution-context lanes declared by the canonical model.
    pub const ALL: [Self; 4] = [
        Self::LocalHost,
        Self::RemoteAttach,
        Self::Container,
        Self::RequestWorkspace,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHost => "local_host",
            Self::RemoteAttach => "remote_attach",
            Self::Container => "container",
            Self::RequestWorkspace => "request_workspace",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalHost => "Local host",
            Self::RemoteAttach => "Remote attach",
            Self::Container => "Container",
            Self::RequestWorkspace => "Request workspace",
        }
    }

    /// [`TargetClass`] tokens this lane is allowed to mint.
    pub fn target_classes(self) -> Vec<TargetClass> {
        match self {
            Self::LocalHost => vec![TargetClass::LocalHost, TargetClass::NotebookKernelLocal],
            Self::RemoteAttach => {
                vec![TargetClass::SshRemote, TargetClass::NotebookKernelRemote]
            }
            Self::Container => vec![TargetClass::ContainerLocal, TargetClass::Devcontainer],
            Self::RequestWorkspace => vec![
                TargetClass::RemoteWorkspaceVm,
                TargetClass::ManagedWorkspace,
                TargetClass::PrebuildRuntime,
                TargetClass::AiSandbox,
            ],
        }
    }

    /// Launch-capable [`SurfaceClass`] tokens that resolve through this lane.
    ///
    /// The list is intentionally closed: surfaces outside the set MUST
    /// visibly declare why they cannot route through the shared contract
    /// instead of inventing local overrides.
    pub fn claimed_surfaces(self) -> Vec<SurfaceClass> {
        vec![
            SurfaceClass::Terminal,
            SurfaceClass::Task,
            SurfaceClass::Test,
            SurfaceClass::Debug,
            SurfaceClass::AiToolCall,
        ]
    }

    /// True when consumer chrome rendering this lane MUST surface the
    /// local-vs-managed boundary cue.
    pub const fn requires_boundary_cue(self) -> bool {
        !matches!(self, Self::LocalHost)
    }
}

/// Lookup the beta lane that owns a given [`TargetClass`].
pub fn lane_for_target_class(target_class: TargetClass) -> ExecutionContextBetaLane {
    for lane in ExecutionContextBetaLane::ALL {
        if lane.target_classes().contains(&target_class) {
            return lane;
        }
    }
    // The lane vocabulary covers every TargetClass; the unreachable branch
    // exists so future widenings without a lane update are caught by the
    // unit tests in this module.
    ExecutionContextBetaLane::LocalHost
}

/// Lookup the beta lane that owns the freshly resolved [`ExecutionContext`].
pub fn lane_for_context(context: &ExecutionContext) -> ExecutionContextBetaLane {
    lane_for_target_class(context.target_identity.target_class)
}

/// One row of beta lane coverage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContextBetaLaneCoverageRow {
    /// Beta lane.
    pub lane: ExecutionContextBetaLane,
    /// Stable lane token.
    pub lane_token: String,
    /// Short reviewer-facing label.
    pub lane_label: String,
    /// [`TargetClass`] tokens this lane covers.
    pub target_classes: Vec<TargetClass>,
    /// Stable target-class tokens.
    pub target_class_tokens: Vec<String>,
    /// Launch-capable surfaces this lane claims.
    pub claimed_surfaces: Vec<SurfaceClass>,
    /// Stable surface tokens.
    pub claimed_surface_tokens: Vec<String>,
    /// Whether consumer chrome MUST render the boundary cue for this lane.
    pub requires_boundary_cue: bool,
}

impl ExecutionContextBetaLaneCoverageRow {
    /// Builds the canonical coverage row for one lane.
    pub fn canonical(lane: ExecutionContextBetaLane) -> Self {
        let target_classes = lane.target_classes();
        let target_class_tokens = target_classes
            .iter()
            .map(|class| class.as_str().to_owned())
            .collect();
        let claimed_surfaces = lane.claimed_surfaces();
        let claimed_surface_tokens = claimed_surfaces
            .iter()
            .map(|surface| surface.as_str().to_owned())
            .collect();
        Self {
            lane,
            lane_token: lane.as_str().to_owned(),
            lane_label: lane.label().to_owned(),
            target_classes,
            target_class_tokens,
            claimed_surfaces,
            claimed_surface_tokens,
            requires_boundary_cue: lane.requires_boundary_cue(),
        }
    }
}

/// Coverage manifest pinning the canonical beta execution-context lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContextBetaCoverageManifest {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the canonical execution-context record.
    pub execution_context_schema_version: u32,
    /// Manifest id.
    pub manifest_id: String,
    /// Manifest timestamp.
    pub generated_at: String,
    /// Canonical lane coverage rows.
    pub lanes: Vec<ExecutionContextBetaLaneCoverageRow>,
}

impl ExecutionContextBetaCoverageManifest {
    /// Builds the canonical beta coverage manifest.
    pub fn canonical(manifest_id: impl Into<String>, generated_at: impl Into<String>) -> Self {
        Self {
            record_kind: EXECUTION_CONTEXT_BETA_COVERAGE_MANIFEST_RECORD_KIND.to_owned(),
            execution_context_schema_version: EXECUTION_CONTEXT_BETA_SCHEMA_VERSION,
            manifest_id: manifest_id.into(),
            generated_at: generated_at.into(),
            lanes: ExecutionContextBetaLane::ALL
                .into_iter()
                .map(ExecutionContextBetaLaneCoverageRow::canonical)
                .collect(),
        }
    }

    /// Returns the canonical row for one lane, if present.
    pub fn row_for_lane(
        &self,
        lane: ExecutionContextBetaLane,
    ) -> Option<&ExecutionContextBetaLaneCoverageRow> {
        self.lanes.iter().find(|row| row.lane == lane)
    }

    /// True when every [`TargetClass`] variant is represented by at least one
    /// lane row. The runtime tests exercise this assertion so a future
    /// widening of [`TargetClass`] without a lane update fails the build.
    pub fn covers_every_target_class(&self) -> bool {
        let claimed: Vec<TargetClass> = self
            .lanes
            .iter()
            .flat_map(|row| row.target_classes.iter().copied())
            .collect();
        for class in EVERY_TARGET_CLASS {
            if !claimed.contains(&class) {
                return false;
            }
        }
        true
    }
}

/// Fields the ticket-drift evaluator compares between a stored binding and
/// the freshly resolved [`ExecutionContext`]. The vocabulary is closed; a
/// new field is additive-minor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TicketDriftField {
    /// `target_identity.target_class` changed.
    TargetClass,
    /// `target_identity.canonical_target_id` changed.
    CanonicalTargetId,
    /// `target_identity.working_directory` changed.
    WorkingDirectory,
    /// `toolchain_identity.toolchain_class` changed.
    ToolchainClass,
    /// `environment_capsule_ref.capsule_hash` changed.
    CapsuleHash,
    /// `environment_capsule_ref.drift_state` regressed away from `in_sync`.
    CapsuleDriftState,
    /// `workset_scope_class` changed.
    ScopeClass,
    /// `policy_and_trust.policy_epoch` advanced past the stored value.
    PolicyEpochAdvanced,
    /// `policy_and_trust.trust_state` regressed (trusted → restricted /
    /// pending or restricted → pending).
    TrustStateRegressed,
}

impl TicketDriftField {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetClass => "target_class",
            Self::CanonicalTargetId => "canonical_target_id",
            Self::WorkingDirectory => "working_directory",
            Self::ToolchainClass => "toolchain_class",
            Self::CapsuleHash => "capsule_hash",
            Self::CapsuleDriftState => "capsule_drift_state",
            Self::ScopeClass => "scope_class",
            Self::PolicyEpochAdvanced => "policy_epoch_advanced",
            Self::TrustStateRegressed => "trust_state_regressed",
        }
    }
}

/// Stored binding the ticket-drift evaluator compares against the freshly
/// resolved context.
///
/// Callers persist this projection alongside any approval ticket, run preview,
/// or rerun snapshot. It quotes the minimum fields required to decide whether
/// dispatching against a newer context would silently change the target,
/// toolchain, scope, capsule, trust, or policy posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketDriftBinding {
    /// Stable id of the originally resolved context.
    pub execution_context_id: String,
    /// Lane the context was minted on.
    pub lane: ExecutionContextBetaLane,
    /// Target class at resolve time.
    pub target_class: TargetClass,
    /// Canonical target id at resolve time.
    pub canonical_target_id: String,
    /// Working directory at resolve time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    /// Toolchain class at resolve time.
    pub toolchain_class: ToolchainClass,
    /// Capsule hash at resolve time.
    pub capsule_hash: String,
    /// Capsule drift state at resolve time.
    pub capsule_drift_state: CapsuleDriftState,
    /// Workset scope class at resolve time.
    pub workset_scope_class: ScopeClass,
    /// Policy epoch at resolve time.
    pub policy_epoch: u64,
    /// Trust state at resolve time.
    pub trust_state: TrustState,
}

impl TicketDriftBinding {
    /// Projects the stored binding from a freshly resolved [`ExecutionContext`].
    pub fn from_context(context: &ExecutionContext) -> Self {
        Self {
            execution_context_id: context.execution_context_id.clone(),
            lane: lane_for_context(context),
            target_class: context.target_identity.target_class,
            canonical_target_id: context.target_identity.canonical_target_id.clone(),
            working_directory: context.target_identity.working_directory.clone(),
            toolchain_class: context.toolchain_identity.toolchain_class,
            capsule_hash: context.environment_capsule_ref.capsule_hash.clone(),
            capsule_drift_state: context.environment_capsule_ref.drift_state,
            workset_scope_class: context.workset_scope_class,
            policy_epoch: context.policy_and_trust.policy_epoch,
            trust_state: context.policy_and_trust.trust_state,
        }
    }
}

/// One drift row recorded by the ticket-drift evaluator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketDriftRow {
    /// Field that drifted.
    pub field: TicketDriftField,
    /// Stable field token.
    pub field_token: String,
    /// Value token recorded on the stored binding.
    pub stored_value_token: String,
    /// Value token recorded on the freshly resolved context.
    pub fresh_value_token: String,
}

/// Outcome of [`evaluate_ticket_drift`].
///
/// `Fresh` means no drift fields changed and the stored ticket / preview is
/// safe to dispatch. `Invalidated` carries the typed drift rows so consumers
/// can surface "this ticket is stale because target_class changed from local
/// to ssh_remote" without re-deriving the comparison locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum TicketDriftOutcome {
    /// Stored ticket / preview matches the freshly resolved context.
    Fresh,
    /// Stored ticket / preview drifted. Dispatch MUST be re-authorised before
    /// proceeding.
    Invalidated {
        /// Drift rows recorded by the evaluator.
        drift_rows: Vec<TicketDriftRow>,
    },
}

impl TicketDriftOutcome {
    /// True when the outcome carries at least one drift row.
    pub fn is_invalidated(&self) -> bool {
        matches!(self, Self::Invalidated { .. })
    }

    /// Drift rows when the outcome is `Invalidated`; empty otherwise.
    pub fn drift_rows(&self) -> &[TicketDriftRow] {
        match self {
            Self::Fresh => &[],
            Self::Invalidated { drift_rows } => drift_rows,
        }
    }
}

/// Full evaluation record for a stored ticket against a freshly resolved
/// context. The record is export-safe: it carries no raw env values, no raw
/// command lines, and no secrets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketDriftEvaluation {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema-version tag.
    pub execution_context_schema_version: u32,
    /// Stored ticket binding.
    pub stored_binding: TicketDriftBinding,
    /// Stable id of the freshly resolved context.
    pub fresh_execution_context_id: String,
    /// Lane the fresh context resolves onto.
    pub fresh_lane: ExecutionContextBetaLane,
    /// Outcome.
    pub outcome: TicketDriftOutcome,
}

/// Evaluates whether a stored ticket / preview binding is still valid against
/// a freshly resolved [`ExecutionContext`].
pub fn evaluate_ticket_drift(
    stored: &TicketDriftBinding,
    fresh: &ExecutionContext,
) -> TicketDriftEvaluation {
    let mut drift_rows: Vec<TicketDriftRow> = Vec::new();

    if stored.target_class != fresh.target_identity.target_class {
        drift_rows.push(TicketDriftRow {
            field: TicketDriftField::TargetClass,
            field_token: TicketDriftField::TargetClass.as_str().to_owned(),
            stored_value_token: stored.target_class.as_str().to_owned(),
            fresh_value_token: fresh.target_identity.target_class.as_str().to_owned(),
        });
    }
    if stored.canonical_target_id != fresh.target_identity.canonical_target_id {
        drift_rows.push(TicketDriftRow {
            field: TicketDriftField::CanonicalTargetId,
            field_token: TicketDriftField::CanonicalTargetId.as_str().to_owned(),
            stored_value_token: stored.canonical_target_id.clone(),
            fresh_value_token: fresh.target_identity.canonical_target_id.clone(),
        });
    }
    if stored.working_directory != fresh.target_identity.working_directory {
        drift_rows.push(TicketDriftRow {
            field: TicketDriftField::WorkingDirectory,
            field_token: TicketDriftField::WorkingDirectory.as_str().to_owned(),
            stored_value_token: stored
                .working_directory
                .clone()
                .unwrap_or_else(|| "<unset>".to_owned()),
            fresh_value_token: fresh
                .target_identity
                .working_directory
                .clone()
                .unwrap_or_else(|| "<unset>".to_owned()),
        });
    }
    if stored.toolchain_class != fresh.toolchain_identity.toolchain_class {
        drift_rows.push(TicketDriftRow {
            field: TicketDriftField::ToolchainClass,
            field_token: TicketDriftField::ToolchainClass.as_str().to_owned(),
            stored_value_token: stored.toolchain_class.as_str().to_owned(),
            fresh_value_token: fresh.toolchain_identity.toolchain_class.as_str().to_owned(),
        });
    }
    if stored.capsule_hash != fresh.environment_capsule_ref.capsule_hash {
        drift_rows.push(TicketDriftRow {
            field: TicketDriftField::CapsuleHash,
            field_token: TicketDriftField::CapsuleHash.as_str().to_owned(),
            stored_value_token: stored.capsule_hash.clone(),
            fresh_value_token: fresh.environment_capsule_ref.capsule_hash.clone(),
        });
    }
    if stored.capsule_drift_state == CapsuleDriftState::InSync
        && fresh.environment_capsule_ref.drift_state != CapsuleDriftState::InSync
    {
        drift_rows.push(TicketDriftRow {
            field: TicketDriftField::CapsuleDriftState,
            field_token: TicketDriftField::CapsuleDriftState.as_str().to_owned(),
            stored_value_token: stored.capsule_drift_state.as_str().to_owned(),
            fresh_value_token: fresh
                .environment_capsule_ref
                .drift_state
                .as_str()
                .to_owned(),
        });
    }
    if stored.workset_scope_class != fresh.workset_scope_class {
        drift_rows.push(TicketDriftRow {
            field: TicketDriftField::ScopeClass,
            field_token: TicketDriftField::ScopeClass.as_str().to_owned(),
            stored_value_token: stored.workset_scope_class.as_str().to_owned(),
            fresh_value_token: fresh.workset_scope_class.as_str().to_owned(),
        });
    }
    if fresh.policy_and_trust.policy_epoch > stored.policy_epoch {
        drift_rows.push(TicketDriftRow {
            field: TicketDriftField::PolicyEpochAdvanced,
            field_token: TicketDriftField::PolicyEpochAdvanced.as_str().to_owned(),
            stored_value_token: stored.policy_epoch.to_string(),
            fresh_value_token: fresh.policy_and_trust.policy_epoch.to_string(),
        });
    }
    if trust_state_regressed(stored.trust_state, fresh.policy_and_trust.trust_state) {
        drift_rows.push(TicketDriftRow {
            field: TicketDriftField::TrustStateRegressed,
            field_token: TicketDriftField::TrustStateRegressed.as_str().to_owned(),
            stored_value_token: trust_state_token(stored.trust_state).to_owned(),
            fresh_value_token: trust_state_token(fresh.policy_and_trust.trust_state).to_owned(),
        });
    }

    let fresh_lane = lane_for_context(fresh);
    let outcome = if drift_rows.is_empty() {
        TicketDriftOutcome::Fresh
    } else {
        TicketDriftOutcome::Invalidated { drift_rows }
    };
    TicketDriftEvaluation {
        record_kind: EXECUTION_CONTEXT_TICKET_DRIFT_RECORD_KIND.to_owned(),
        execution_context_schema_version: EXECUTION_CONTEXT_BETA_SCHEMA_VERSION,
        stored_binding: stored.clone(),
        fresh_execution_context_id: fresh.execution_context_id.clone(),
        fresh_lane,
        outcome,
    }
}

/// Beta support-export packet projecting the lane coverage manifest, the per
/// lane resolved-context sample ids, and any ticket-drift evaluations the
/// support flow attached. Raw env, raw command lines, and raw secrets are out
/// of scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContextBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema-version tag shared with the canonical record.
    pub execution_context_schema_version: u32,
    /// Lane coverage manifest at export time.
    pub coverage_manifest: ExecutionContextBetaCoverageManifest,
    /// Per-lane execution-context id samples the support flow attached.
    pub lane_samples: Vec<ExecutionContextBetaLaneSample>,
    /// Ticket-drift evaluations carried alongside the export.
    pub ticket_drift_evaluations: Vec<TicketDriftEvaluation>,
}

impl ExecutionContextBetaSupportExport {
    /// Builds the support-export packet for a snapshot of resolved contexts
    /// and ticket-drift evaluations.
    pub fn new(
        manifest_id: impl Into<String>,
        generated_at: impl Into<String>,
        lane_samples: Vec<ExecutionContextBetaLaneSample>,
        ticket_drift_evaluations: Vec<TicketDriftEvaluation>,
    ) -> Self {
        Self {
            record_kind: EXECUTION_CONTEXT_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            execution_context_schema_version: EXECUTION_CONTEXT_BETA_SCHEMA_VERSION,
            coverage_manifest: ExecutionContextBetaCoverageManifest::canonical(
                manifest_id,
                generated_at,
            ),
            lane_samples,
            ticket_drift_evaluations,
        }
    }
}

/// One lane sample row carried by the beta support-export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContextBetaLaneSample {
    /// Lane the sample resolved onto.
    pub lane: ExecutionContextBetaLane,
    /// Stable lane token.
    pub lane_token: String,
    /// Surface the sample was minted for.
    pub surface: SurfaceClass,
    /// Resolved execution-context id.
    pub execution_context_id: String,
    /// Resolved canonical target id.
    pub canonical_target_id: String,
    /// Resolved target class.
    pub target_class: TargetClass,
    /// Visible boundary-cue posture at resolve time.
    pub boundary_cue_visible: bool,
    /// True when the context resolves with at least one degraded-field row.
    pub has_degraded_field: bool,
}

impl ExecutionContextBetaLaneSample {
    /// Builds a sample row from a freshly resolved context.
    pub fn from_context(context: &ExecutionContext) -> Self {
        let lane = lane_for_context(context);
        Self {
            lane,
            lane_token: lane.as_str().to_owned(),
            surface: context.invocation_subject.surface,
            execution_context_id: context.execution_context_id.clone(),
            canonical_target_id: context.target_identity.canonical_target_id.clone(),
            target_class: context.target_identity.target_class,
            boundary_cue_visible: context.boundary_cue_visible(),
            has_degraded_field: context.has_degraded_field(),
        }
    }
}

impl<'a> ExecutionContextRequest<'a> {
    /// Convenience constructor for a container-lane task seed. The caller
    /// chooses between `container_local` and `devcontainer` via the
    /// `target_class` argument.
    pub fn container_task_seed(
        command_id: &'a str,
        target_class: TargetClass,
        trust_state: TrustState,
        observed_at: &'a str,
    ) -> Self {
        debug_assert!(
            matches!(
                target_class,
                TargetClass::ContainerLocal | TargetClass::Devcontainer
            ),
            "container_task_seed expects a container target class"
        );
        Self {
            command_id,
            surface: SurfaceClass::Task,
            actor_class: ActorClass::UserCommand,
            trust_state,
            observed_at,
            requested_target_class: Some(target_class),
            requested_working_directory: None,
            requested_toolchain_class: Some(ToolchainClass::ContainerisedRuntime),
            override_target_class: None,
            override_working_directory: None,
            override_toolchain_class: None,
        }
    }

    /// Convenience constructor for a remote-attach task seed (SSH or remote
    /// notebook kernel).
    pub fn remote_attach_task_seed(
        command_id: &'a str,
        target_class: TargetClass,
        trust_state: TrustState,
        observed_at: &'a str,
    ) -> Self {
        debug_assert!(
            matches!(
                target_class,
                TargetClass::SshRemote | TargetClass::NotebookKernelRemote
            ),
            "remote_attach_task_seed expects a remote-attach target class"
        );
        Self {
            command_id,
            surface: SurfaceClass::Task,
            actor_class: ActorClass::UserCommand,
            trust_state,
            observed_at,
            requested_target_class: Some(target_class),
            requested_working_directory: None,
            requested_toolchain_class: Some(ToolchainClass::BuildDriverRuntime),
            override_target_class: None,
            override_working_directory: None,
            override_toolchain_class: None,
        }
    }

    /// Convenience constructor for a request-workspace task seed (managed
    /// workspace, remote workspace VM, prebuild runtime, or AI sandbox).
    pub fn request_workspace_task_seed(
        command_id: &'a str,
        target_class: TargetClass,
        trust_state: TrustState,
        observed_at: &'a str,
    ) -> Self {
        debug_assert!(
            matches!(
                target_class,
                TargetClass::RemoteWorkspaceVm
                    | TargetClass::ManagedWorkspace
                    | TargetClass::PrebuildRuntime
                    | TargetClass::AiSandbox
            ),
            "request_workspace_task_seed expects a request-workspace target class"
        );
        Self {
            command_id,
            surface: SurfaceClass::Task,
            actor_class: ActorClass::UserCommand,
            trust_state,
            observed_at,
            requested_target_class: Some(target_class),
            requested_working_directory: None,
            requested_toolchain_class: Some(ToolchainClass::BuildDriverRuntime),
            override_target_class: None,
            override_working_directory: None,
            override_toolchain_class: None,
        }
    }
}

const EVERY_TARGET_CLASS: [TargetClass; 10] = [
    TargetClass::LocalHost,
    TargetClass::SshRemote,
    TargetClass::ContainerLocal,
    TargetClass::Devcontainer,
    TargetClass::RemoteWorkspaceVm,
    TargetClass::PrebuildRuntime,
    TargetClass::ManagedWorkspace,
    TargetClass::NotebookKernelLocal,
    TargetClass::NotebookKernelRemote,
    TargetClass::AiSandbox,
];

const fn trust_state_token(state: TrustState) -> &'static str {
    match state {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
        TrustState::PendingEvaluation => "pending_evaluation",
    }
}

fn trust_state_rank(state: TrustState) -> u8 {
    match state {
        TrustState::Trusted => 0,
        TrustState::Restricted => 1,
        TrustState::PendingEvaluation => 2,
    }
}

fn trust_state_regressed(stored: TrustState, fresh: TrustState) -> bool {
    trust_state_rank(fresh) > trust_state_rank(stored)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_context::{
        EnvironmentCapsuleRef, ExecutionContextResolver, ExecutionContextResolverConfig,
        IdentityMode,
    };

    fn baseline_config() -> ExecutionContextResolverConfig {
        ExecutionContextResolverConfig {
            workspace_id: "ws-beta".to_owned(),
            profile_id: Some("prof.beta".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 1,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some("/workspace".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "caps:ws-beta:seed".to_owned(),
                capsule_hash: "sha256:beta".to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "beta-0".to_owned(),
        }
    }

    #[test]
    fn lane_manifest_covers_every_target_class() {
        let manifest = ExecutionContextBetaCoverageManifest::canonical(
            "execution-context-beta:test",
            "2026-05-15T00:00:00Z",
        );
        assert!(manifest.covers_every_target_class());
        assert_eq!(manifest.lanes.len(), ExecutionContextBetaLane::ALL.len());
        for lane in ExecutionContextBetaLane::ALL {
            let row = manifest.row_for_lane(lane).expect("row");
            assert_eq!(row.lane, lane);
            assert!(!row.target_classes.is_empty(), "lane must have targets");
            assert_eq!(row.requires_boundary_cue, lane.requires_boundary_cue());
        }
    }

    #[test]
    fn lane_lookup_routes_each_target_class_to_its_lane() {
        for class in EVERY_TARGET_CLASS {
            let lane = lane_for_target_class(class);
            assert!(
                lane.target_classes().contains(&class),
                "{} must map to lane {}",
                class.as_str(),
                lane.as_str()
            );
        }
        assert_eq!(
            lane_for_target_class(TargetClass::LocalHost),
            ExecutionContextBetaLane::LocalHost
        );
        assert_eq!(
            lane_for_target_class(TargetClass::SshRemote),
            ExecutionContextBetaLane::RemoteAttach
        );
        assert_eq!(
            lane_for_target_class(TargetClass::Devcontainer),
            ExecutionContextBetaLane::Container
        );
        assert_eq!(
            lane_for_target_class(TargetClass::ManagedWorkspace),
            ExecutionContextBetaLane::RequestWorkspace
        );
    }

    #[test]
    fn ticket_drift_evaluator_marks_fresh_when_nothing_changed() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run.beta",
            TrustState::Trusted,
            "mono:0",
        ));
        let binding = TicketDriftBinding::from_context(&context);
        let evaluation = evaluate_ticket_drift(&binding, &context);
        assert_eq!(evaluation.outcome, TicketDriftOutcome::Fresh);
        assert!(evaluation.outcome.drift_rows().is_empty());
        assert_eq!(
            evaluation.fresh_execution_context_id,
            context.execution_context_id
        );
        assert_eq!(evaluation.fresh_lane, ExecutionContextBetaLane::LocalHost);
    }

    #[test]
    fn ticket_drift_evaluator_invalidates_when_target_class_changes() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let stored_context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run.beta",
            TrustState::Trusted,
            "mono:0",
        ));
        let stored_binding = TicketDriftBinding::from_context(&stored_context);

        let mut request =
            ExecutionContextRequest::task_seed("task.run.beta", TrustState::Trusted, "mono:1");
        request.override_target_class = Some(TargetClass::SshRemote);
        let fresh_context = resolver.resolve(request);

        let evaluation = evaluate_ticket_drift(&stored_binding, &fresh_context);
        assert!(evaluation.outcome.is_invalidated());
        let rows = evaluation.outcome.drift_rows();
        assert!(rows
            .iter()
            .any(|row| row.field == TicketDriftField::TargetClass));
        assert!(rows
            .iter()
            .any(|row| row.field == TicketDriftField::CanonicalTargetId));
        assert_eq!(
            evaluation.fresh_lane,
            ExecutionContextBetaLane::RemoteAttach
        );
    }

    #[test]
    fn ticket_drift_evaluator_invalidates_when_policy_epoch_advances() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let stored_context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run.beta",
            TrustState::Trusted,
            "mono:0",
        ));
        let stored_binding = TicketDriftBinding::from_context(&stored_context);

        let mut next_config = baseline_config();
        next_config.policy_epoch = 5;
        let mut next_resolver = ExecutionContextResolver::new(next_config);
        let fresh_context = next_resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run.beta",
            TrustState::Trusted,
            "mono:1",
        ));

        let evaluation = evaluate_ticket_drift(&stored_binding, &fresh_context);
        assert!(evaluation.outcome.is_invalidated());
        assert!(evaluation
            .outcome
            .drift_rows()
            .iter()
            .any(|row| row.field == TicketDriftField::PolicyEpochAdvanced));
    }

    #[test]
    fn ticket_drift_evaluator_invalidates_when_trust_regresses() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let stored_context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run.beta",
            TrustState::Trusted,
            "mono:0",
        ));
        let stored_binding = TicketDriftBinding::from_context(&stored_context);

        let fresh_context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run.beta",
            TrustState::Restricted,
            "mono:1",
        ));

        let evaluation = evaluate_ticket_drift(&stored_binding, &fresh_context);
        assert!(evaluation.outcome.is_invalidated());
        assert!(evaluation
            .outcome
            .drift_rows()
            .iter()
            .any(|row| row.field == TicketDriftField::TrustStateRegressed));
    }

    #[test]
    fn ticket_drift_evaluator_records_capsule_hash_change() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let stored_context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run.beta",
            TrustState::Trusted,
            "mono:0",
        ));
        let stored_binding = TicketDriftBinding::from_context(&stored_context);

        let mut next_config = baseline_config();
        next_config.environment_capsule_ref.capsule_hash = "sha256:beta-next".to_owned();
        let mut next_resolver = ExecutionContextResolver::new(next_config);
        let fresh_context = next_resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run.beta",
            TrustState::Trusted,
            "mono:1",
        ));

        let evaluation = evaluate_ticket_drift(&stored_binding, &fresh_context);
        assert!(evaluation.outcome.is_invalidated());
        assert!(evaluation
            .outcome
            .drift_rows()
            .iter()
            .any(|row| row.field == TicketDriftField::CapsuleHash));
    }

    #[test]
    fn beta_request_constructors_resolve_through_the_canonical_resolver() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());

        let container = resolver.resolve(ExecutionContextRequest::container_task_seed(
            "task.run.container",
            TargetClass::Devcontainer,
            TrustState::Trusted,
            "mono:1",
        ));
        assert_eq!(
            lane_for_context(&container),
            ExecutionContextBetaLane::Container
        );
        assert_eq!(
            container.toolchain_identity.toolchain_class,
            ToolchainClass::ContainerisedRuntime
        );
        assert!(container.boundary_cue_visible());

        let remote = resolver.resolve(ExecutionContextRequest::remote_attach_task_seed(
            "task.run.remote",
            TargetClass::SshRemote,
            TrustState::Trusted,
            "mono:2",
        ));
        assert_eq!(
            lane_for_context(&remote),
            ExecutionContextBetaLane::RemoteAttach
        );
        assert!(remote.boundary_cue_visible());

        let workspace = resolver.resolve(ExecutionContextRequest::request_workspace_task_seed(
            "task.run.managed",
            TargetClass::ManagedWorkspace,
            TrustState::Trusted,
            "mono:3",
        ));
        assert_eq!(
            lane_for_context(&workspace),
            ExecutionContextBetaLane::RequestWorkspace
        );
        assert!(workspace.boundary_cue_visible());
    }

    #[test]
    fn support_export_packet_round_trips_through_serde() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let local = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
            "terminal.open",
            TrustState::Trusted,
            "mono:0",
        ));
        let remote = resolver.resolve(ExecutionContextRequest::remote_attach_task_seed(
            "task.run.remote",
            TargetClass::SshRemote,
            TrustState::Trusted,
            "mono:1",
        ));
        let stored = TicketDriftBinding::from_context(&local);
        let evaluation = evaluate_ticket_drift(&stored, &remote);
        let packet = ExecutionContextBetaSupportExport::new(
            "execution-context-beta:support",
            "2026-05-15T00:00:00Z",
            vec![
                ExecutionContextBetaLaneSample::from_context(&local),
                ExecutionContextBetaLaneSample::from_context(&remote),
            ],
            vec![evaluation],
        );

        let json = serde_json::to_string(&packet).expect("serialize");
        let round: ExecutionContextBetaSupportExport =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round, packet);
        assert_eq!(
            round.record_kind,
            EXECUTION_CONTEXT_BETA_SUPPORT_EXPORT_RECORD_KIND
        );
        assert!(round.coverage_manifest.covers_every_target_class());
        assert!(round
            .ticket_drift_evaluations
            .iter()
            .any(|eval| eval.outcome.is_invalidated()));
    }
}
