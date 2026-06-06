//! Stable sandbox-profile, approval-lineage, and backend-fallback truth.
//!
//! This module owns the runtime packet that launch-capable surfaces use to
//! explain which sandbox profile was requested, which backend can enforce it,
//! which approval ticket authorized the capability envelope, and which narrower
//! fallback applies when enforcement is incomplete. It is metadata-only:
//! support exports and diagnostics carry opaque refs, hashes, and closed
//! vocabulary tokens rather than raw secrets, raw command lines, or ambient
//! authority.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record kind for [`SandboxProfilePacket`].
pub const SANDBOX_PROFILE_PACKET_RECORD_KIND: &str =
    "runtime_sandbox_profile_backend_truth_stable_packet";

/// Stable record kind for [`SandboxProfileSupportExport`].
pub const SANDBOX_PROFILE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "runtime_sandbox_profile_backend_truth_support_export";

/// Integer schema version for sandbox profile packets.
pub const SANDBOX_PROFILE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative schema path consumed by validators and support exports.
pub const SANDBOX_PROFILE_SCHEMA_REF: &str = "schemas/runtime/sandbox-profile.schema.json";

/// Repo-relative runtime doc path.
pub const SANDBOX_PROFILE_DOC_REF: &str = "docs/runtime/sandbox-profiles-and-fallbacks.md";

/// Repo-relative Help doc path.
pub const SANDBOX_PROFILE_HELP_DOC_REF: &str = "docs/help/runtime-sandbox-profiles.md";

/// Repo-relative backend crosswalk artifact path.
pub const SANDBOX_BACKEND_CROSSWALK_REF: &str = "artifacts/runtime/m4/sandbox-backend-crosswalk.md";

/// Repo-relative stable packet artifact path.
pub const SANDBOX_PROFILE_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/sandbox_profile_backend_truth_packet.json";

/// Stable sandbox profile IDs for launch-capable runtime families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxProfileId {
    /// Read-only analyzers, parsers, and import validators.
    ReadOnlyAnalyzerV1,
    /// Build, test, lint, formatter, and repo task execution.
    RepoTaskV1,
    /// Local or remote interactive PTY sessions.
    InteractiveTerminalV1,
    /// Debug launch, debug attach, profiler attach, and privileged inspection.
    DebugAttachV1,
    /// Notebook kernels, REPL kernels, and data-science runtimes.
    NotebookKernelV1,
    /// Template hooks, scaffold validators, and bootstrap tasks.
    BootstrapScaffoldV1,
    /// AI and recipe tools that mutate files, execute tools, or externalize data.
    AiToolMutatorV1,
    /// DB, warehouse, infrastructure, and cloud connectors.
    DataConnectorV1,
}

impl SandboxProfileId {
    /// Every stable profile ID claimed by this packet.
    pub const ALL: [Self; 8] = [
        Self::ReadOnlyAnalyzerV1,
        Self::RepoTaskV1,
        Self::InteractiveTerminalV1,
        Self::DebugAttachV1,
        Self::NotebookKernelV1,
        Self::BootstrapScaffoldV1,
        Self::AiToolMutatorV1,
        Self::DataConnectorV1,
    ];

    /// Stable token used in schemas, artifacts, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyAnalyzerV1 => "read_only_analyzer_v1",
            Self::RepoTaskV1 => "repo_task_v1",
            Self::InteractiveTerminalV1 => "interactive_terminal_v1",
            Self::DebugAttachV1 => "debug_attach_v1",
            Self::NotebookKernelV1 => "notebook_kernel_v1",
            Self::BootstrapScaffoldV1 => "bootstrap_scaffold_v1",
            Self::AiToolMutatorV1 => "ai_tool_mutator_v1",
            Self::DataConnectorV1 => "data_connector_v1",
        }
    }
}

/// Filesystem posture declared by a sandbox profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilesystemPosture {
    /// Workspace/workset read access only.
    WorkspaceReadOnly,
    /// Workspace read plus declared output sinks.
    WorkspaceReadDeclaredOutputs,
    /// User-selected cwd/runtime paths under a PTY boundary.
    UserSelectedRuntimePaths,
    /// Target-scoped source, symbol, and map stores.
    TargetScopedStores,
    /// Notebook, workspace, data, and export-sink paths declared per session.
    NotebookDataAndExportSinks,
    /// Side directory or side worktree until previewed apply.
    SideWorktreeBeforeApply,
    /// Reviewed target/worktree plus approved export sinks.
    ReviewedTargetAndExportSinks,
    /// Connector cache plus approved export path only.
    ConnectorCacheAndExportPath,
}

/// Network posture declared by a sandbox profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkPosture {
    /// No network by default.
    NoneByDefault,
    /// Only the declared policy-aware network class.
    DeclaredNetworkClassOnly,
    /// Local or remote PTY boundary with policy-aware egress.
    PtyBoundaryPolicyAware,
    /// Target-dependent network no broader than the debug target contract.
    TargetContractOnly,
    /// Declared endpoint classes per kernel/session.
    DeclaredEndpointClassOnly,
    /// Bootstrap, package mirror, or template network only.
    BootstrapMirrorOnly,
    /// Approved tool and endpoint set only.
    ApprovedToolEndpointSet,
    /// Declared endpoint or tunnel only.
    DeclaredEndpointOrTunnelOnly,
}

/// Secret posture declared by a sandbox profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretPosture {
    /// No secret projection is available.
    None,
    /// Brokered handle or delegated projection only.
    BrokeredHandleProjectionOnly,
    /// User-selected projection set shown in the inspector.
    SelectedProjectionVisible,
    /// Target-needed projection only.
    TargetNeededProjectionOnly,
    /// Session-scoped projection that is never sticky across kernels by default.
    SessionScopedProjectionOnly,
    /// Explicit projection only, never ambient inheritance.
    ExplicitProjectionOnly,
    /// Named handle or projection set from the approval ticket.
    TicketNamedProjectionSet,
    /// Connection-scoped handle or delegated token.
    ConnectionScopedHandle,
}

/// Child-process posture declared by a sandbox profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChildProcessPosture {
    /// Child-process launch is denied.
    Denied,
    /// Child-process launch is allowed only through derived envelopes.
    DerivedEnvelopeOnly,
    /// User-driven PTY children are allowed inside the PTY boundary.
    UserDrivenPtyBoundary,
    /// Privileged inspection is allowed only on claimed platform backends.
    PrivilegedClaimedPlatformOnly,
    /// Kernel-declared children are allowed only when the kernel contract declares them.
    KernelContractDeclaredOnly,
    /// Background daemons are forbidden; children stay inside the derived envelope.
    NoDaemonsDerivedEnvelopeOnly,
    /// Nested tool launch requires a derived envelope.
    NestedToolDerivedEnvelopeOnly,
    /// No general child spawn unless the adapter contract declares one.
    AdapterContractDeclaredOnly,
}

/// Trust requirement declared by a sandbox profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustRequirement {
    /// Profile is allowed in untrusted or trusted workspaces.
    UntrustedOrTrusted,
    /// Trusted workspace or explicit user override.
    TrustedWorkspaceOrExplicitOverride,
    /// Trusted workspace for repo-owned shell mutation.
    TrustedWorkspaceForRepoShellMutation,
    /// Trusted workspace plus explicit approval.
    TrustedWorkspaceAndExplicitApproval,
    /// Trusted notebook/workspace plus explicit trust class.
    TrustedNotebookWorkspaceAndTrustClass,
    /// Trusted template/bundle plus preview/apply approval.
    TrustedTemplateAndPreviewApplyApproval,
    /// Explicit reviewed plan plus approval ticket.
    ReviewedPlanAndApprovalTicket,
    /// Trusted connector plus endpoint-specific approval where required.
    TrustedConnectorAndEndpointApproval,
}

/// Platform/backend family that may enforce a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendPlatformClass {
    /// macOS desktop package/backend.
    MacosDesktop,
    /// Windows desktop package/backend.
    WindowsDesktop,
    /// Linux desktop package/backend.
    LinuxDesktop,
    /// Remote helper or managed execution broker.
    RemoteManaged,
    /// Browser companion surface.
    BrowserCompanion,
}

impl BackendPlatformClass {
    /// Every backend class that must publish fallback truth.
    pub const ALL: [Self; 5] = [
        Self::MacosDesktop,
        Self::WindowsDesktop,
        Self::LinuxDesktop,
        Self::RemoteManaged,
        Self::BrowserCompanion,
    ];

    /// Stable token used in schemas, artifacts, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MacosDesktop => "macos_desktop",
            Self::WindowsDesktop => "windows_desktop",
            Self::LinuxDesktop => "linux_desktop",
            Self::RemoteManaged => "remote_managed",
            Self::BrowserCompanion => "browser_companion",
        }
    }
}

/// Enforcement backend class published for one profile/platform pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendEnforcementClass {
    /// Native backend meets the published threshold for this profile.
    NativeFull,
    /// Native backend is partial and must narrow or block.
    NativePartial,
    /// Remote broker enforces the profile off-device.
    RemoteBroker,
    /// Managed broker enforces the profile in a managed environment.
    ManagedBroker,
    /// Browser companion has no local execution backend.
    BrowserNoLocalExecution,
    /// No published backend can enforce this profile.
    Unsupported,
}

/// Runtime posture after matching a requested profile to a backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementPosture {
    /// Requested profile is enforced as declared.
    Enforced,
    /// A stricter profile is enforced instead.
    StricterDowngrade,
    /// Capability is not supported on this backend.
    Unsupported,
    /// Launch fails closed rather than widening authority.
    FailClosed,
}

impl EnforcementPosture {
    /// True when the row does not widen into ambient user execution.
    pub const fn is_safe_for_stable_claim(self) -> bool {
        matches!(self, Self::Enforced | Self::StricterDowngrade)
    }
}

/// Action class bound by an approval ticket and capability envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalActionClass {
    /// Read-only analysis.
    ReadOnly,
    /// Local write or repair.
    LocalWrite,
    /// External side effect or provider mutation.
    ExternalSideEffect,
    /// Privileged debug or inspection attach.
    PrivilegedDebug,
    /// Secret use or credential projection.
    SecretUse,
    /// Trust, policy, or admin change.
    TrustChange,
    /// Remote execution.
    RemoteExecution,
}

impl ApprovalActionClass {
    /// True when remembered approvals must mint fresh short-lived tickets.
    pub const fn forbids_indefinite_remembered_approval(self) -> bool {
        !matches!(self, Self::ReadOnly)
    }
}

/// Revocation state exported for approval lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRevocationState {
    /// Ticket is currently valid.
    Active,
    /// Ticket expired.
    Expired,
    /// Ticket was explicitly revoked.
    Revoked,
    /// Ticket was invalidated by target, policy, version, authority, or profile drift.
    DriftInvalidated,
}

/// Revalidation trigger shown in approval history and expiry banners.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevalidationTrigger {
    /// Target identity changed.
    TargetDrift,
    /// Policy epoch changed.
    PolicyDrift,
    /// Runtime/backend or tool version changed.
    VersionDrift,
    /// Authority source or actor changed.
    AuthorityDrift,
    /// Sandbox profile or capability hash changed.
    SandboxOrCapabilityDrift,
    /// Ticket expired.
    Expiry,
    /// Ticket or remembered rule was revoked.
    Revocation,
}

/// Consumer surface that must show profile/backend/approval truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxConsumerSurface {
    /// Installer or package diagnostics surface.
    Installer,
    /// Runtime inspector.
    RuntimeInspector,
    /// Extension detail or permission surface.
    ExtensionDetail,
    /// Quarantine/restricted-runtime screen.
    QuarantineScreen,
    /// Command diagnostics and headless explain output.
    CommandDiagnostic,
    /// Support bundle/export surface.
    SupportBundle,
    /// Docs and help surface.
    DocsHelp,
}

impl SandboxConsumerSurface {
    /// Every consumer that must preserve packet truth.
    pub const REQUIRED: [Self; 7] = [
        Self::Installer,
        Self::RuntimeInspector,
        Self::ExtensionDetail,
        Self::QuarantineScreen,
        Self::CommandDiagnostic,
        Self::SupportBundle,
        Self::DocsHelp,
    ];

    /// Stable token used in projection refs and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Installer => "installer",
            Self::RuntimeInspector => "runtime_inspector",
            Self::ExtensionDetail => "extension_detail",
            Self::QuarantineScreen => "quarantine_screen",
            Self::CommandDiagnostic => "command_diagnostic",
            Self::SupportBundle => "support_bundle",
            Self::DocsHelp => "docs_help",
        }
    }
}

/// Validation severity for a sandbox packet finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding.
    Warning,
    /// Stable publication blocker.
    Blocker,
}

/// Validation finding kind for sandbox packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxFindingKind {
    /// Required identity field is empty.
    MissingIdentity,
    /// A required sandbox profile is absent.
    MissingProfile,
    /// A required backend platform is absent.
    MissingBackendPlatform,
    /// A claimed stable backend cannot safely enforce or narrow the profile.
    ClaimedStableBackendUnsafe,
    /// Unsupported or incomplete backend widens to ambient execution.
    AmbientFallbackAdmitted,
    /// Browser companion row allows hidden local execution.
    BrowserCompanionLocalExecutionAllowed,
    /// Approval lineage is missing a required binding field.
    ApprovalLineageIncomplete,
    /// Remembered approval bypasses fresh-ticket minting.
    RememberedApprovalBypassesFreshTicket,
    /// Drift or expiry revalidation trigger is missing.
    RevalidationTriggerMissing,
    /// Approval history row or expiry banner projection is missing.
    ApprovalHistoryProjectionMissing,
    /// Required consumer surface is absent.
    MissingConsumerProjection,
    /// Consumer projection dropped profile/backend/approval truth.
    ConsumerProjectionDrift,
    /// Raw secrets, raw command bodies, or ambient authority crossed the export boundary.
    RawOrAmbientMaterialPresent,
}

impl SandboxFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingIdentity => "missing_identity",
            Self::MissingProfile => "missing_profile",
            Self::MissingBackendPlatform => "missing_backend_platform",
            Self::ClaimedStableBackendUnsafe => "claimed_stable_backend_unsafe",
            Self::AmbientFallbackAdmitted => "ambient_fallback_admitted",
            Self::BrowserCompanionLocalExecutionAllowed => {
                "browser_companion_local_execution_allowed"
            }
            Self::ApprovalLineageIncomplete => "approval_lineage_incomplete",
            Self::RememberedApprovalBypassesFreshTicket => {
                "remembered_approval_bypasses_fresh_ticket"
            }
            Self::RevalidationTriggerMissing => "revalidation_trigger_missing",
            Self::ApprovalHistoryProjectionMissing => "approval_history_projection_missing",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::RawOrAmbientMaterialPresent => "raw_or_ambient_material_present",
        }
    }
}

/// Stable promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxPromotionState {
    /// Packet has no blocker findings.
    Stable,
    /// Packet has reviewable warnings but no blockers.
    NeedsReview,
    /// Packet blocks stable publication.
    BlocksStable,
}

/// One sandbox profile definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxProfile {
    /// Stable profile ID.
    pub profile_id: SandboxProfileId,
    /// Human-readable but controlled label.
    pub display_label: String,
    /// Filesystem posture.
    pub filesystem_posture: FilesystemPosture,
    /// Network posture.
    pub network_posture: NetworkPosture,
    /// Secret posture.
    pub secret_posture: SecretPosture,
    /// Child-process posture.
    pub child_process_posture: ChildProcessPosture,
    /// Trust requirement.
    pub trust_requirement: TrustRequirement,
}

/// One approval ticket to capability-envelope binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalEnvelopeBinding {
    /// Approval ticket ID.
    pub ticket_id: String,
    /// Actor or subject ref.
    pub actor_ref: String,
    /// Issuing surface ref.
    pub issuing_surface_ref: String,
    /// Action class.
    pub action_class: ApprovalActionClass,
    /// Workspace/workset/slice scope ref.
    pub workspace_scope_ref: String,
    /// Target identity ref.
    pub target_identity_ref: String,
    /// Sandbox profile ID authorized for launch.
    pub sandbox_profile_id: SandboxProfileId,
    /// Capability envelope ref.
    pub capability_envelope_ref: String,
    /// Capability hash ref.
    pub capability_hash_ref: String,
    /// Policy epoch ref.
    pub policy_epoch_ref: String,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Revoke or expiry state.
    pub revocation_state: ApprovalRevocationState,
    /// Audit lineage refs safe for support export.
    pub audit_lineage_refs: Vec<String>,
    /// Revalidation triggers that force reapproval.
    pub revalidation_triggers: Vec<RevalidationTrigger>,
    /// True when remembered approvals mint fresh tickets at use time.
    pub remembered_approval_mints_fresh_ticket: bool,
    /// True when approval-history row can show actor, scope, target, expiry, and drift reason.
    pub approval_history_row_visible: bool,
    /// True when expiry banner can show who authorized the action, scope, expiry, and revalidation cause.
    pub expiry_banner_visible: bool,
}

impl ApprovalEnvelopeBinding {
    fn has_required_bindings(&self) -> bool {
        !self.ticket_id.trim().is_empty()
            && !self.actor_ref.trim().is_empty()
            && !self.issuing_surface_ref.trim().is_empty()
            && !self.workspace_scope_ref.trim().is_empty()
            && !self.target_identity_ref.trim().is_empty()
            && !self.capability_envelope_ref.trim().is_empty()
            && !self.capability_hash_ref.trim().is_empty()
            && !self.policy_epoch_ref.trim().is_empty()
            && !self.expires_at.trim().is_empty()
            && !self.audit_lineage_refs.is_empty()
    }
}

/// One platform/profile backend classification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxBackendRow {
    /// Stable row ID.
    pub row_id: String,
    /// Platform/backend family.
    pub platform_class: BackendPlatformClass,
    /// Sandbox profile being classified.
    pub profile_id: SandboxProfileId,
    /// Published backend classification.
    pub backend_class: BackendEnforcementClass,
    /// Enforcement posture for this platform/profile pair.
    pub enforcement_posture: EnforcementPosture,
    /// True when this row is claimed stable by product surfaces.
    pub claimed_stable: bool,
    /// Optional narrower fallback profile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_profile_id: Option<SandboxProfileId>,
    /// True when unsupported/incomplete enforcement cannot fall back to ambient user execution.
    pub ambient_fallback_denied: bool,
    /// True when browser companion cannot launch local shell/kernel/device execution.
    pub hidden_local_execution_denied: bool,
    /// Disclosure ref shown when row is narrowed or unsupported.
    pub disclosure_ref: String,
}

/// Consumer projection proving a surface reads the packet without reminting truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxConsumerProjection {
    /// Consumer surface.
    pub consumer_surface: SandboxConsumerSurface,
    /// Projection ref.
    pub projection_ref: String,
    /// True when profile IDs are preserved.
    pub preserves_profile_id: bool,
    /// True when backend class is preserved.
    pub preserves_backend_class: bool,
    /// True when fallback class is preserved.
    pub preserves_fallback_truth: bool,
    /// True when approval lineage, expiry, and revalidation triggers are visible.
    pub preserves_approval_lineage: bool,
    /// True when raw secrets and command bodies are excluded.
    pub raw_material_excluded: bool,
    /// True when ambient authority is excluded.
    pub ambient_authority_excluded: bool,
}

impl SandboxConsumerProjection {
    fn preserves_truth(&self) -> bool {
        !self.projection_ref.trim().is_empty()
            && self.preserves_profile_id
            && self.preserves_backend_class
            && self.preserves_fallback_truth
            && self.preserves_approval_lineage
            && self.raw_material_excluded
            && self.ambient_authority_excluded
    }
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxValidationFinding {
    /// Finding kind.
    pub finding_kind: SandboxFindingKind,
    /// Finding severity.
    pub severity: SandboxFindingSeverity,
    /// Support-safe explanation.
    pub summary: String,
}

impl SandboxValidationFinding {
    fn blocker(finding_kind: SandboxFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: SandboxFindingSeverity::Blocker,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`SandboxProfilePacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxProfilePacketInput {
    /// Stable packet ID.
    pub packet_id: String,
    /// Workflow or surface ID that generated the packet.
    pub workflow_or_surface_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Profile definitions.
    pub profiles: Vec<SandboxProfile>,
    /// Backend crosswalk rows.
    pub backend_rows: Vec<SandboxBackendRow>,
    /// Approval-ticket to capability-envelope bindings.
    pub approval_bindings: Vec<ApprovalEnvelopeBinding>,
    /// Consumer projections.
    pub consumer_projections: Vec<SandboxConsumerProjection>,
    /// Source docs, schemas, and artifacts consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Stable sandbox profile and backend truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxProfilePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet ID.
    pub packet_id: String,
    /// Workflow or surface ID.
    pub workflow_or_surface_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Profile definitions.
    pub profiles: Vec<SandboxProfile>,
    /// Backend crosswalk rows.
    pub backend_rows: Vec<SandboxBackendRow>,
    /// Approval-ticket to capability-envelope bindings.
    pub approval_bindings: Vec<ApprovalEnvelopeBinding>,
    /// Consumer projections.
    pub consumer_projections: Vec<SandboxConsumerProjection>,
    /// Source docs, schemas, and artifacts consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: SandboxPromotionState,
    /// Derived validation findings.
    pub validation_findings: Vec<SandboxValidationFinding>,
}

impl SandboxProfilePacket {
    /// Materializes a packet and derives validation findings.
    pub fn materialize(input: SandboxProfilePacketInput) -> Self {
        let mut packet = Self {
            record_kind: SANDBOX_PROFILE_PACKET_RECORD_KIND.to_owned(),
            schema_version: SANDBOX_PROFILE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            profiles: input.profiles,
            backend_rows: input.backend_rows,
            approval_bindings: input.approval_bindings,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: SandboxPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.validate();
        packet.promotion_state = promotion_state_for(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates stable sandbox invariants.
    pub fn validate(&self) -> Vec<SandboxValidationFinding> {
        let mut findings = Vec::new();

        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(SandboxValidationFinding::blocker(
                SandboxFindingKind::MissingIdentity,
                "sandbox packet identity fields must not be empty",
            ));
        }

        let profile_ids: BTreeSet<SandboxProfileId> = self
            .profiles
            .iter()
            .map(|profile| profile.profile_id)
            .collect();
        for profile_id in SandboxProfileId::ALL {
            if !profile_ids.contains(&profile_id) {
                findings.push(SandboxValidationFinding::blocker(
                    SandboxFindingKind::MissingProfile,
                    format!("missing stable sandbox profile {}", profile_id.as_str()),
                ));
            }
        }

        let platforms: BTreeSet<BackendPlatformClass> = self
            .backend_rows
            .iter()
            .map(|row| row.platform_class)
            .collect();
        for platform in BackendPlatformClass::ALL {
            if !platforms.contains(&platform) {
                findings.push(SandboxValidationFinding::blocker(
                    SandboxFindingKind::MissingBackendPlatform,
                    format!("missing backend platform {}", platform.as_str()),
                ));
            }
        }

        for row in &self.backend_rows {
            if row.row_id.trim().is_empty() || row.disclosure_ref.trim().is_empty() {
                findings.push(SandboxValidationFinding::blocker(
                    SandboxFindingKind::MissingIdentity,
                    "backend row id and disclosure ref must not be empty",
                ));
            }
            if row.claimed_stable && !row.enforcement_posture.is_safe_for_stable_claim() {
                findings.push(SandboxValidationFinding::blocker(
                    SandboxFindingKind::ClaimedStableBackendUnsafe,
                    format!(
                        "claimed stable backend row {} cannot enforce or narrow safely",
                        row.row_id
                    ),
                ));
            }
            if !row.ambient_fallback_denied {
                findings.push(SandboxValidationFinding::blocker(
                    SandboxFindingKind::AmbientFallbackAdmitted,
                    format!("backend row {} admits ambient fallback", row.row_id),
                ));
            }
            if row.platform_class == BackendPlatformClass::BrowserCompanion
                && !row.hidden_local_execution_denied
            {
                findings.push(SandboxValidationFinding::blocker(
                    SandboxFindingKind::BrowserCompanionLocalExecutionAllowed,
                    format!(
                        "browser companion row {} allows hidden local execution",
                        row.row_id
                    ),
                ));
            }
        }

        for binding in &self.approval_bindings {
            if !binding.has_required_bindings() {
                findings.push(SandboxValidationFinding::blocker(
                    SandboxFindingKind::ApprovalLineageIncomplete,
                    format!("approval binding {} is incomplete", binding.ticket_id),
                ));
            }
            if binding
                .action_class
                .forbids_indefinite_remembered_approval()
                && !binding.remembered_approval_mints_fresh_ticket
            {
                findings.push(SandboxValidationFinding::blocker(
                    SandboxFindingKind::RememberedApprovalBypassesFreshTicket,
                    format!(
                        "approval binding {} bypasses fresh-ticket minting",
                        binding.ticket_id
                    ),
                ));
            }
            if binding
                .action_class
                .forbids_indefinite_remembered_approval()
                && binding.revalidation_triggers.is_empty()
            {
                findings.push(SandboxValidationFinding::blocker(
                    SandboxFindingKind::RevalidationTriggerMissing,
                    format!(
                        "approval binding {} lacks revalidation triggers",
                        binding.ticket_id
                    ),
                ));
            }
            if !binding.approval_history_row_visible || !binding.expiry_banner_visible {
                findings.push(SandboxValidationFinding::blocker(
                    SandboxFindingKind::ApprovalHistoryProjectionMissing,
                    format!(
                        "approval binding {} lacks approval-history or expiry-banner projection",
                        binding.ticket_id
                    ),
                ));
            }
        }

        let consumer_surfaces: BTreeSet<SandboxConsumerSurface> = self
            .consumer_projections
            .iter()
            .map(|projection| projection.consumer_surface)
            .collect();
        for surface in SandboxConsumerSurface::REQUIRED {
            if !consumer_surfaces.contains(&surface) {
                findings.push(SandboxValidationFinding::blocker(
                    SandboxFindingKind::MissingConsumerProjection,
                    "required sandbox consumer projection is missing",
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth() {
                findings.push(SandboxValidationFinding::blocker(
                    SandboxFindingKind::ConsumerProjectionDrift,
                    format!(
                        "consumer projection {} does not preserve sandbox truth",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if self.consumer_projections.iter().any(|projection| {
            !projection.raw_material_excluded || !projection.ambient_authority_excluded
        }) {
            findings.push(SandboxValidationFinding::blocker(
                SandboxFindingKind::RawOrAmbientMaterialPresent,
                "consumer projection admits raw or ambient material",
            ));
        }

        findings
    }

    /// Returns true when there are no blocker findings.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == SandboxFindingSeverity::Blocker)
    }

    /// Returns stable profile tokens carried by this packet.
    pub fn profile_tokens(&self) -> Vec<&'static str> {
        self.profiles
            .iter()
            .map(|profile| profile.profile_id.as_str())
            .collect()
    }

    /// Builds a support-export-safe projection.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> SandboxProfileSupportExport {
        SandboxProfileSupportExport {
            record_kind: SANDBOX_PROFILE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SANDBOX_PROFILE_SCHEMA_VERSION,
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            source_packet_id: self.packet_id.clone(),
            promotion_state: self.promotion_state,
            profile_ids: self
                .profile_tokens()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            backend_row_count: self.backend_rows.len(),
            approval_binding_count: self.approval_bindings.len(),
            finding_kinds: self
                .validation_findings
                .iter()
                .map(|finding| finding.finding_kind.as_str().to_owned())
                .collect(),
            raw_secret_material_excluded: true,
            raw_command_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }
}

/// Metadata-only support export for sandbox profile truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxProfileSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Export ID.
    pub export_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Source packet ID.
    pub source_packet_id: String,
    /// Promotion state.
    pub promotion_state: SandboxPromotionState,
    /// Stable profile IDs.
    pub profile_ids: Vec<String>,
    /// Backend row count.
    pub backend_row_count: usize,
    /// Approval binding count.
    pub approval_binding_count: usize,
    /// Validation finding kind tokens.
    pub finding_kinds: Vec<String>,
    /// True when raw secret material is excluded.
    pub raw_secret_material_excluded: bool,
    /// True when raw command material is excluded.
    pub raw_command_material_excluded: bool,
    /// True when ambient authority is excluded.
    pub ambient_authority_excluded: bool,
}

fn promotion_state_for(findings: &[SandboxValidationFinding]) -> SandboxPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == SandboxFindingSeverity::Blocker)
    {
        SandboxPromotionState::BlocksStable
    } else if findings.is_empty() {
        SandboxPromotionState::Stable
    } else {
        SandboxPromotionState::NeedsReview
    }
}

/// Returns the current stable sandbox profile packet.
pub fn current_stable_sandbox_profile_packet() -> SandboxProfilePacket {
    SandboxProfilePacket::materialize(current_stable_sandbox_profile_packet_input())
}

/// Returns the constructor input for the current stable sandbox packet.
pub fn current_stable_sandbox_profile_packet_input() -> SandboxProfilePacketInput {
    SandboxProfilePacketInput {
        packet_id: "packet:runtime:sandbox-profile-backend-truth:stable".to_owned(),
        workflow_or_surface_id: "workflow.runtime.sandbox_profile_backend_truth".to_owned(),
        generated_at: "2026-06-06T12:00:00Z".to_owned(),
        profiles: stable_profiles(),
        backend_rows: stable_backend_rows(),
        approval_bindings: stable_approval_bindings(),
        consumer_projections: stable_consumer_projections(),
        source_contract_refs: vec![
            SANDBOX_PROFILE_SCHEMA_REF.to_owned(),
            SANDBOX_PROFILE_DOC_REF.to_owned(),
            SANDBOX_PROFILE_HELP_DOC_REF.to_owned(),
            SANDBOX_BACKEND_CROSSWALK_REF.to_owned(),
        ],
    }
}

fn stable_profiles() -> Vec<SandboxProfile> {
    vec![
        SandboxProfile {
            profile_id: SandboxProfileId::ReadOnlyAnalyzerV1,
            display_label: "Read-only analyzer".to_owned(),
            filesystem_posture: FilesystemPosture::WorkspaceReadOnly,
            network_posture: NetworkPosture::NoneByDefault,
            secret_posture: SecretPosture::None,
            child_process_posture: ChildProcessPosture::Denied,
            trust_requirement: TrustRequirement::UntrustedOrTrusted,
        },
        SandboxProfile {
            profile_id: SandboxProfileId::RepoTaskV1,
            display_label: "Repo task".to_owned(),
            filesystem_posture: FilesystemPosture::WorkspaceReadDeclaredOutputs,
            network_posture: NetworkPosture::DeclaredNetworkClassOnly,
            secret_posture: SecretPosture::BrokeredHandleProjectionOnly,
            child_process_posture: ChildProcessPosture::DerivedEnvelopeOnly,
            trust_requirement: TrustRequirement::TrustedWorkspaceOrExplicitOverride,
        },
        SandboxProfile {
            profile_id: SandboxProfileId::InteractiveTerminalV1,
            display_label: "Interactive terminal".to_owned(),
            filesystem_posture: FilesystemPosture::UserSelectedRuntimePaths,
            network_posture: NetworkPosture::PtyBoundaryPolicyAware,
            secret_posture: SecretPosture::SelectedProjectionVisible,
            child_process_posture: ChildProcessPosture::UserDrivenPtyBoundary,
            trust_requirement: TrustRequirement::TrustedWorkspaceForRepoShellMutation,
        },
        SandboxProfile {
            profile_id: SandboxProfileId::DebugAttachV1,
            display_label: "Debug attach".to_owned(),
            filesystem_posture: FilesystemPosture::TargetScopedStores,
            network_posture: NetworkPosture::TargetContractOnly,
            secret_posture: SecretPosture::TargetNeededProjectionOnly,
            child_process_posture: ChildProcessPosture::PrivilegedClaimedPlatformOnly,
            trust_requirement: TrustRequirement::TrustedWorkspaceAndExplicitApproval,
        },
        SandboxProfile {
            profile_id: SandboxProfileId::NotebookKernelV1,
            display_label: "Notebook kernel".to_owned(),
            filesystem_posture: FilesystemPosture::NotebookDataAndExportSinks,
            network_posture: NetworkPosture::DeclaredEndpointClassOnly,
            secret_posture: SecretPosture::SessionScopedProjectionOnly,
            child_process_posture: ChildProcessPosture::KernelContractDeclaredOnly,
            trust_requirement: TrustRequirement::TrustedNotebookWorkspaceAndTrustClass,
        },
        SandboxProfile {
            profile_id: SandboxProfileId::BootstrapScaffoldV1,
            display_label: "Bootstrap scaffold".to_owned(),
            filesystem_posture: FilesystemPosture::SideWorktreeBeforeApply,
            network_posture: NetworkPosture::BootstrapMirrorOnly,
            secret_posture: SecretPosture::ExplicitProjectionOnly,
            child_process_posture: ChildProcessPosture::NoDaemonsDerivedEnvelopeOnly,
            trust_requirement: TrustRequirement::TrustedTemplateAndPreviewApplyApproval,
        },
        SandboxProfile {
            profile_id: SandboxProfileId::AiToolMutatorV1,
            display_label: "AI tool mutator".to_owned(),
            filesystem_posture: FilesystemPosture::ReviewedTargetAndExportSinks,
            network_posture: NetworkPosture::ApprovedToolEndpointSet,
            secret_posture: SecretPosture::TicketNamedProjectionSet,
            child_process_posture: ChildProcessPosture::NestedToolDerivedEnvelopeOnly,
            trust_requirement: TrustRequirement::ReviewedPlanAndApprovalTicket,
        },
        SandboxProfile {
            profile_id: SandboxProfileId::DataConnectorV1,
            display_label: "Data connector".to_owned(),
            filesystem_posture: FilesystemPosture::ConnectorCacheAndExportPath,
            network_posture: NetworkPosture::DeclaredEndpointOrTunnelOnly,
            secret_posture: SecretPosture::ConnectionScopedHandle,
            child_process_posture: ChildProcessPosture::AdapterContractDeclaredOnly,
            trust_requirement: TrustRequirement::TrustedConnectorAndEndpointApproval,
        },
    ]
}

fn stable_backend_rows() -> Vec<SandboxBackendRow> {
    let mut rows = Vec::new();
    for platform in [
        BackendPlatformClass::MacosDesktop,
        BackendPlatformClass::WindowsDesktop,
        BackendPlatformClass::LinuxDesktop,
    ] {
        for profile_id in SandboxProfileId::ALL {
            let (claimed_stable, enforcement_posture, fallback_profile_id) = match profile_id {
                SandboxProfileId::DebugAttachV1
                    if platform == BackendPlatformClass::LinuxDesktop =>
                {
                    (
                        false,
                        EnforcementPosture::Unsupported,
                        Some(SandboxProfileId::ReadOnlyAnalyzerV1),
                    )
                }
                _ => (true, EnforcementPosture::Enforced, None),
            };
            rows.push(SandboxBackendRow {
                row_id: format!("row:{}:{}", platform.as_str(), profile_id.as_str()),
                platform_class: platform,
                profile_id,
                backend_class: if claimed_stable {
                    BackendEnforcementClass::NativeFull
                } else {
                    BackendEnforcementClass::Unsupported
                },
                enforcement_posture,
                claimed_stable,
                fallback_profile_id,
                ambient_fallback_denied: true,
                hidden_local_execution_denied: true,
                disclosure_ref: SANDBOX_BACKEND_CROSSWALK_REF.to_owned(),
            });
        }
    }
    for profile_id in SandboxProfileId::ALL {
        rows.push(SandboxBackendRow {
            row_id: format!("row:remote_managed:{}", profile_id.as_str()),
            platform_class: BackendPlatformClass::RemoteManaged,
            profile_id,
            backend_class: BackendEnforcementClass::ManagedBroker,
            enforcement_posture: EnforcementPosture::Enforced,
            claimed_stable: true,
            fallback_profile_id: None,
            ambient_fallback_denied: true,
            hidden_local_execution_denied: true,
            disclosure_ref: SANDBOX_BACKEND_CROSSWALK_REF.to_owned(),
        });
    }
    for profile_id in SandboxProfileId::ALL {
        let read_only = profile_id == SandboxProfileId::ReadOnlyAnalyzerV1;
        rows.push(SandboxBackendRow {
            row_id: format!("row:browser_companion:{}", profile_id.as_str()),
            platform_class: BackendPlatformClass::BrowserCompanion,
            profile_id,
            backend_class: BackendEnforcementClass::BrowserNoLocalExecution,
            enforcement_posture: if read_only {
                EnforcementPosture::StricterDowngrade
            } else {
                EnforcementPosture::Unsupported
            },
            claimed_stable: false,
            fallback_profile_id: Some(SandboxProfileId::ReadOnlyAnalyzerV1),
            ambient_fallback_denied: true,
            hidden_local_execution_denied: true,
            disclosure_ref: SANDBOX_BACKEND_CROSSWALK_REF.to_owned(),
        });
    }
    rows
}

fn stable_approval_bindings() -> Vec<ApprovalEnvelopeBinding> {
    vec![
        approval_binding(
            "approval:repo-task:fmt:0001",
            ApprovalActionClass::LocalWrite,
            SandboxProfileId::RepoTaskV1,
            "capability-envelope:repo-task:fmt:v1",
            "capability-hash:repo-task:fmt:v1",
            vec![
                RevalidationTrigger::TargetDrift,
                RevalidationTrigger::PolicyDrift,
                RevalidationTrigger::SandboxOrCapabilityDrift,
                RevalidationTrigger::Expiry,
            ],
        ),
        approval_binding(
            "approval:debug-attach:local:0002",
            ApprovalActionClass::PrivilegedDebug,
            SandboxProfileId::DebugAttachV1,
            "capability-envelope:debug-attach:local:v1",
            "capability-hash:debug-attach:local:v1",
            vec![
                RevalidationTrigger::TargetDrift,
                RevalidationTrigger::PolicyDrift,
                RevalidationTrigger::VersionDrift,
                RevalidationTrigger::AuthorityDrift,
                RevalidationTrigger::Expiry,
            ],
        ),
        approval_binding(
            "approval:ai-tool:scoped-apply:0003",
            ApprovalActionClass::ExternalSideEffect,
            SandboxProfileId::AiToolMutatorV1,
            "capability-envelope:ai-tool:scoped-apply:v1",
            "capability-hash:ai-tool:scoped-apply:v1",
            vec![
                RevalidationTrigger::TargetDrift,
                RevalidationTrigger::PolicyDrift,
                RevalidationTrigger::VersionDrift,
                RevalidationTrigger::AuthorityDrift,
                RevalidationTrigger::SandboxOrCapabilityDrift,
                RevalidationTrigger::Revocation,
            ],
        ),
        approval_binding(
            "approval:data-connector:query:0004",
            ApprovalActionClass::SecretUse,
            SandboxProfileId::DataConnectorV1,
            "capability-envelope:data-connector:query:v1",
            "capability-hash:data-connector:query:v1",
            vec![
                RevalidationTrigger::TargetDrift,
                RevalidationTrigger::PolicyDrift,
                RevalidationTrigger::AuthorityDrift,
                RevalidationTrigger::SandboxOrCapabilityDrift,
                RevalidationTrigger::Expiry,
            ],
        ),
    ]
}

fn approval_binding(
    ticket_id: &str,
    action_class: ApprovalActionClass,
    sandbox_profile_id: SandboxProfileId,
    capability_envelope_ref: &str,
    capability_hash_ref: &str,
    revalidation_triggers: Vec<RevalidationTrigger>,
) -> ApprovalEnvelopeBinding {
    ApprovalEnvelopeBinding {
        ticket_id: ticket_id.to_owned(),
        actor_ref: "actor:user:local-owner".to_owned(),
        issuing_surface_ref: "surface:shell:approval".to_owned(),
        action_class,
        workspace_scope_ref: "workspace:current:workset:active".to_owned(),
        target_identity_ref: "target:current:opaque".to_owned(),
        sandbox_profile_id,
        capability_envelope_ref: capability_envelope_ref.to_owned(),
        capability_hash_ref: capability_hash_ref.to_owned(),
        policy_epoch_ref: "policy-epoch:stable:2026-06-06".to_owned(),
        expires_at: "2026-06-06T12:10:00Z".to_owned(),
        revocation_state: ApprovalRevocationState::Active,
        audit_lineage_refs: vec![
            format!("audit:{ticket_id}:issue"),
            format!("audit:{ticket_id}:use"),
        ],
        revalidation_triggers,
        remembered_approval_mints_fresh_ticket: true,
        approval_history_row_visible: true,
        expiry_banner_visible: true,
    }
}

fn stable_consumer_projections() -> Vec<SandboxConsumerProjection> {
    SandboxConsumerSurface::REQUIRED
        .into_iter()
        .map(|consumer_surface| SandboxConsumerProjection {
            consumer_surface,
            projection_ref: format!("projection:sandbox:{}", consumer_surface.as_str()),
            preserves_profile_id: true,
            preserves_backend_class: true,
            preserves_fallback_truth: true,
            preserves_approval_lineage: true,
            raw_material_excluded: true,
            ambient_authority_excluded: true,
        })
        .collect()
}
