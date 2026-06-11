//! Container engine summaries, open-in-container preflights, rebuild/log/port
//! review, and route/time-bound truth for M5 remote-preview and incident flows.
//!
//! This module owns one canonical, checked-in packet —
//! [`ProjectDoctorContainerBoundaryTruth`] — that makes the container and
//! devcontainer boundary legible *before* a user reopens, rebuilds, or attaches.
//! Each [`ContainerWorkspaceScenario`] seeds one container-dependent situation in
//! one M5 workflow surface (remote preview or incident workflow) and pins, in one
//! record:
//!
//! - an [`EngineSummary`] naming the active [`EngineClass`], its
//!   [`EngineReachability`], its [`SupportClass`]/certification note, and the
//!   diagnostics actions available when the engine is unreachable or
//!   policy-blocked, so a window is never treated as merely "in a container"
//!   without naming engine class, support class, and side effects,
//! - the [`WorkspaceMode`] and [`BoundaryLabel`] plus the
//!   [`ContainerWorkspaceScenario::target_ref`] target identity being rebuilt or
//!   attached, so local/remote/managed boundary labels survive into preview and
//!   incident surfaces,
//! - a [`RebuildReview`] preflight sheet disclosing the definition source,
//!   rebuild-versus-reuse decision, trust-gated lifecycle hooks, extension
//!   installs, published ports, writable mounts, affected services/images, and a
//!   non-empty stay-local alternative, so hooks/mounts/ports are reviewed before
//!   commitment and trust-gated hooks never run silently, and
//! - a [`LogTruth`] block threading live/buffered/snapshot availability, an
//!   export-safe time range, and the redaction posture into the flow instead of
//!   leaving logs as context-free streams.
//!
//! The central guarantee is a **non-inheriting preflight gate**: every scenario's
//! published [`ContainerWorkspaceScenario::published_preflight_decision`] and
//! [`ContainerWorkspaceScenario::published_preflight_reason`] are validated
//! against the decision recomputed from the scenario's own reachability, support
//! class, and disclosed side effects ([`ContainerWorkspaceScenario::recompute_gate`]).
//! An unreachable or policy-blocked engine routes to a non-dead-end
//! offer-alternative outcome rather than a stranding modal; an unsupported engine,
//! a trust-gated hook, or any rebuild/port/mount side effect forces explicit
//! disclosure rather than a silent proceed.
//!
//! The packet is checked in at
//! `artifacts/doctor/m5/project-doctor-container-boundary-truth.json` and embedded
//! here via `include_str!`, so this typed consumer and any CI gate agree on every
//! row without a cargo build in CI. The model is metadata-only: every field is a
//! typed state or an opaque ref. It carries no credential bodies, raw provider
//! payloads, or mount/port/tunnel secrets.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Supported container-boundary-truth packet schema version.
pub const PROJECT_DOCTOR_CONTAINER_BOUNDARY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const PROJECT_DOCTOR_CONTAINER_BOUNDARY_RECORD_KIND: &str =
    "project_doctor_container_boundary_truth";

/// Repo-relative path to the checked-in packet.
pub const PROJECT_DOCTOR_CONTAINER_BOUNDARY_PATH: &str =
    "artifacts/doctor/m5/project-doctor-container-boundary-truth.json";

/// Repo-relative path to the boundary schema.
pub const PROJECT_DOCTOR_CONTAINER_BOUNDARY_SCHEMA_REF: &str =
    "schemas/doctor/project-doctor-container-boundary-truth.schema.json";

/// Repo-relative path to the companion document.
pub const PROJECT_DOCTOR_CONTAINER_BOUNDARY_DOC_REF: &str =
    "docs/doctor/m5/project-doctor-container-boundary-truth.md";

/// Stable finding-code prefix every initiating finding must use.
pub const DOCTOR_FINDING_PREFIX: &str = "doctor.finding.";

/// Stable repair-id prefix every preserved repair id must use.
pub const DOCTOR_REPAIR_PREFIX: &str = "repair.";

/// Required redaction class for every support linkage and log-truth posture.
pub const METADATA_SAFE_REDACTION_CLASS: &str = "metadata_safe_default";

/// Canonical, locale-invariant machine-meaning keys every scenario must carry,
/// so localized prose can never silently change what a surface means.
pub const REQUIRED_MACHINE_MEANING_KEYS: [&str; 5] = [
    "scenario_id",
    "surface",
    "engine_class",
    "workspace_mode",
    "preflight_decision",
];

/// Embedded checked-in packet JSON.
pub const PROJECT_DOCTOR_CONTAINER_BOUNDARY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/doctor/m5/project-doctor-container-boundary-truth.json"
));

/// Generic, non-actionable explanation tokens that may never stand in for a
/// specific boundary explanation.
const GENERIC_EXPLANATION_TOKENS: [&str; 7] = [
    "unavailable",
    "error",
    "failed",
    "failure",
    "unknown",
    "in_a_container",
    "n_a",
];

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// The M5 workflow surface a container-dependent scenario is surfaced in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowSurface {
    /// Remote preview route, port, and tunnel workflow.
    RemotePreview,
    /// Incident packet and recovery workflow.
    IncidentWorkflow,
}

impl WorkflowSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 2] = [Self::RemotePreview, Self::IncidentWorkflow];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemotePreview => "remote_preview",
            Self::IncidentWorkflow => "incident_workflow",
        }
    }

    /// The stable finding-code prefix every initiating finding on this surface
    /// must start with.
    pub fn finding_code_prefix(self) -> String {
        format!("{DOCTOR_FINDING_PREFIX}{}.", self.as_str())
    }
}

impl std::fmt::Display for WorkflowSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The active container engine class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineClass {
    /// Docker engine.
    Docker,
    /// Podman engine.
    Podman,
    /// The devcontainers CLI orchestrating an engine.
    DevcontainersCli,
    /// A managed cloud workspace engine.
    ManagedCloud,
}

impl EngineClass {
    /// Every engine class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Docker,
        Self::Podman,
        Self::DevcontainersCli,
        Self::ManagedCloud,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Docker => "docker",
            Self::Podman => "podman",
            Self::DevcontainersCli => "devcontainers_cli",
            Self::ManagedCloud => "managed_cloud",
        }
    }
}

impl std::fmt::Display for EngineClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Whether the engine can be reached, or why it cannot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineReachability {
    /// The engine answered and is usable.
    Reachable,
    /// The engine did not answer.
    Unreachable,
    /// Policy blocks using the engine in this context.
    PolicyBlocked,
}

impl EngineReachability {
    /// Every reachability state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Reachable, Self::Unreachable, Self::PolicyBlocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::Unreachable => "unreachable",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The support/certification class of the active engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Certified for production use.
    Certified,
    /// Supported but not certified.
    Supported,
    /// Experimental; usable with disclosure.
    Experimental,
    /// Unsupported in this build; usable only with explicit disclosure.
    Unsupported,
}

impl SupportClass {
    /// Every support class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Certified,
        Self::Supported,
        Self::Experimental,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Supported => "supported",
            Self::Experimental => "experimental",
            Self::Unsupported => "unsupported",
        }
    }
}

/// How the current window relates to the container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceMode {
    /// Attached to an already-built container.
    AttachedContainer,
    /// A devcontainer built from a definition.
    Devcontainer,
    /// A remote, managed workspace.
    RemoteManaged,
}

impl WorkspaceMode {
    /// Every mode, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::AttachedContainer,
        Self::Devcontainer,
        Self::RemoteManaged,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AttachedContainer => "attached_container",
            Self::Devcontainer => "devcontainer",
            Self::RemoteManaged => "remote_managed",
        }
    }
}

impl std::fmt::Display for WorkspaceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The local/remote/managed boundary label that must survive into preview and
/// incident surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryLabel {
    /// Runs on the local host.
    Local,
    /// Runs on a remote host the user controls.
    Remote,
    /// Runs in a provider-managed environment.
    Managed,
}

impl BoundaryLabel {
    /// Every boundary label, in declaration order.
    pub const ALL: [Self; 3] = [Self::Local, Self::Remote, Self::Managed];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Managed => "managed",
        }
    }
}

/// Where the container/devcontainer definition comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefinitionSource {
    /// A `devcontainer.json` definition.
    DevcontainerJson,
    /// A `Dockerfile`.
    Dockerfile,
    /// A Compose file.
    ComposeFile,
    /// A bare image reference.
    ImageReference,
    /// A managed-workspace template.
    ManagedTemplate,
}

impl DefinitionSource {
    /// Every definition source, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DevcontainerJson,
        Self::Dockerfile,
        Self::ComposeFile,
        Self::ImageReference,
        Self::ManagedTemplate,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DevcontainerJson => "devcontainer_json",
            Self::Dockerfile => "dockerfile",
            Self::ComposeFile => "compose_file",
            Self::ImageReference => "image_reference",
            Self::ManagedTemplate => "managed_template",
        }
    }
}

/// Whether the preflight rebuilds the container or reuses an existing one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RebuildDecision {
    /// Rebuild the container from its definition.
    Rebuild,
    /// Reuse the existing container.
    ReuseExisting,
}

impl RebuildDecision {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rebuild => "rebuild",
            Self::ReuseExisting => "reuse_existing",
        }
    }
}

/// The lifecycle stage a hook runs at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleHookKind {
    /// Runs on the host before the container is created.
    Initialize,
    /// Runs once when the container is created.
    OnCreate,
    /// Runs when workspace content updates.
    UpdateContent,
    /// Runs after creation completes.
    PostCreate,
    /// Runs each time the container starts.
    PostStart,
    /// Runs each time a client attaches.
    PostAttach,
}

impl LifecycleHookKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::OnCreate => "on_create",
            Self::UpdateContent => "update_content",
            Self::PostCreate => "post_create",
            Self::PostStart => "post_start",
            Self::PostAttach => "post_attach",
        }
    }
}

/// The exposure level of a published port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortVisibility {
    /// Bound to localhost only.
    LocalOnly,
    /// Visible on the local network.
    LanVisible,
    /// Exposed through a public tunnel.
    PublicTunnel,
}

impl PortVisibility {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::LanVisible => "lan_visible",
            Self::PublicTunnel => "public_tunnel",
        }
    }
}

/// The kind of a mount binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MountKind {
    /// The workspace folder bound into the container.
    WorkspaceBind,
    /// A named volume.
    NamedVolume,
    /// An ephemeral tmpfs scratch mount.
    TmpfsScratch,
    /// A host path bound into the container.
    HostPathBind,
}

impl MountKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceBind => "workspace_bind",
            Self::NamedVolume => "named_volume",
            Self::TmpfsScratch => "tmpfs_scratch",
            Self::HostPathBind => "host_path_bind",
        }
    }
}

/// The availability of logs for the workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogAvailability {
    /// A live, streaming log feed.
    Live,
    /// A buffered, recent window of log lines.
    Buffered,
    /// A captured point-in-time snapshot.
    Snapshot,
    /// No logs are available (e.g. the engine is down).
    Unavailable,
}

impl LogAvailability {
    /// Every availability, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Live,
        Self::Buffered,
        Self::Snapshot,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Buffered => "buffered",
            Self::Snapshot => "snapshot",
            Self::Unavailable => "unavailable",
        }
    }

    /// True when a usable log channel exists (anything but [`Self::Unavailable`]).
    pub const fn is_available(self) -> bool {
        !matches!(self, Self::Unavailable)
    }
}

/// The preflight gate decision for a scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightDecision {
    /// Proceed; no disclosure beyond the summary is required.
    ProceedFull,
    /// Proceed only after the disclosed side effects are reviewed.
    ProceedWithDisclosure,
    /// Do not proceed; offer the stay-local (or other) alternative.
    BlockedOfferAlternative,
}

impl PreflightDecision {
    /// Every decision, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ProceedFull,
        Self::ProceedWithDisclosure,
        Self::BlockedOfferAlternative,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProceedFull => "proceed_full",
            Self::ProceedWithDisclosure => "proceed_with_disclosure",
            Self::BlockedOfferAlternative => "blocked_offer_alternative",
        }
    }
}

/// Why the preflight gate narrowed or blocked a scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightReason {
    /// No narrowing applied; the scenario proceeds at full strength.
    None,
    /// The engine did not answer.
    EngineUnreachable,
    /// Policy blocks the engine in this context.
    PolicyBlocked,
    /// The engine is unsupported in this build.
    UnsupportedEngine,
    /// One or more lifecycle hooks are trust-gated and must be reviewed.
    TrustGatedHooks,
    /// Rebuild/port/mount side effects must be reviewed.
    SideEffectsRequireReview,
}

impl PreflightReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::None,
        Self::EngineUnreachable,
        Self::PolicyBlocked,
        Self::UnsupportedEngine,
        Self::TrustGatedHooks,
        Self::SideEffectsRequireReview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::EngineUnreachable => "engine_unreachable",
            Self::PolicyBlocked => "policy_blocked",
            Self::UnsupportedEngine => "unsupported_engine",
            Self::TrustGatedHooks => "trust_gated_hooks",
            Self::SideEffectsRequireReview => "side_effects_require_review",
        }
    }
}

/// A surface that must render the same scenario identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParitySurface {
    /// Desktop preflight/rebuild review sheet.
    DesktopSheet,
    /// CLI/headless inspect row.
    CliInspect,
    /// Headless machine-readable JSON.
    HeadlessJson,
    /// Browser handoff view.
    BrowserHandoff,
    /// Support-bundle export.
    SupportExport,
    /// Incident-packet view.
    IncidentPacket,
}

impl ParitySurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopSheet,
        Self::CliInspect,
        Self::HeadlessJson,
        Self::BrowserHandoff,
        Self::SupportExport,
        Self::IncidentPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopSheet => "desktop_sheet",
            Self::CliInspect => "cli_inspect",
            Self::HeadlessJson => "headless_json",
            Self::BrowserHandoff => "browser_handoff",
            Self::SupportExport => "support_export",
            Self::IncidentPacket => "incident_packet",
        }
    }
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// The active-engine summary row rendered wherever a surface depends on a
/// container or devcontainer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineSummary {
    /// Active engine class.
    pub engine_class: EngineClass,
    /// Opaque engine version ref.
    pub engine_version_ref: String,
    /// Whether the engine is reachable, or why it is not.
    pub reachability: EngineReachability,
    /// Support/certification class of the engine.
    pub support_class: SupportClass,
    /// Human-readable support/certification note.
    pub certification_note: String,
    /// Diagnostics actions offered for the engine (opaque action ids).
    pub diagnostics_actions: Vec<String>,
}

impl EngineSummary {
    /// True when the engine is reachable.
    pub fn is_reachable(&self) -> bool {
        self.reachability == EngineReachability::Reachable
    }
}

/// One trust-aware lifecycle hook disclosed by a rebuild review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleHook {
    /// The lifecycle stage the hook runs at.
    pub kind: LifecycleHookKind,
    /// Opaque ref to the command the hook runs.
    pub command_ref: String,
    /// True when the hook is trust-gated and must be reviewed before it runs.
    pub trust_gated: bool,
}

/// One published port mapping disclosed by a rebuild review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortMapping {
    /// Port inside the container.
    pub container_port: u32,
    /// Port published on the host.
    pub host_port: u32,
    /// Exposure level of the published port.
    pub visibility: PortVisibility,
}

/// One mount binding disclosed by a rebuild review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MountBinding {
    /// The kind of mount.
    pub kind: MountKind,
    /// Opaque scope ref for the mount source.
    pub scope_ref: String,
    /// True when the mount is writable.
    pub writable: bool,
}

/// The open-in-container preflight / rebuild review sheet.
///
/// Discloses everything a reopen or rebuild will do before it is committed: the
/// definition source, whether the container is rebuilt or reused, the lifecycle
/// hooks (and which are trust-gated), extension installs, published ports,
/// writable mounts, affected services/images, and a non-empty stay-local
/// alternative so no flow is a dead end.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RebuildReview {
    /// Where the definition comes from.
    pub definition_source: DefinitionSource,
    /// Opaque ref to the definition source.
    pub definition_source_ref: String,
    /// Whether the container is rebuilt or reused.
    pub rebuild_decision: RebuildDecision,
    /// Disclosed lifecycle hooks.
    pub lifecycle_hooks: Vec<LifecycleHook>,
    /// Opaque ids of extensions installed into the container.
    pub extension_installs: Vec<String>,
    /// Published ports.
    pub published_ports: Vec<PortMapping>,
    /// Writable mounts (read-only mounts may also be listed).
    pub writable_mounts: Vec<MountBinding>,
    /// Opaque ids of affected services.
    pub affected_services: Vec<String>,
    /// Opaque ids of affected images.
    pub affected_images: Vec<String>,
    /// The stay-local (or otherwise non-container) alternative offered; never
    /// empty, so no flow is a dead end.
    pub stay_local_alternative: String,
}

impl RebuildReview {
    /// True when any disclosed lifecycle hook is trust-gated.
    pub fn has_trust_gated_hooks(&self) -> bool {
        self.lifecycle_hooks.iter().any(|h| h.trust_gated)
    }

    /// True when any mount in [`Self::writable_mounts`] is actually writable.
    pub fn has_writable_mount(&self) -> bool {
        self.writable_mounts.iter().any(|m| m.writable)
    }

    /// True when the review carries side effects beyond a read-only reuse: a
    /// rebuild, a published port, a writable mount, or an extension install.
    /// Trust-gated hooks are tracked separately by [`Self::has_trust_gated_hooks`].
    pub fn has_side_effects(&self) -> bool {
        self.rebuild_decision == RebuildDecision::Rebuild
            || !self.published_ports.is_empty()
            || self.has_writable_mount()
            || !self.extension_installs.is_empty()
    }
}

/// Live/buffered/snapshot log truth plus export-safe time range and redaction
/// posture threaded into the workflow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogTruth {
    /// The availability of logs.
    pub availability: LogAvailability,
    /// Opaque ref to the export-safe time range; empty only when unavailable.
    pub export_time_range_ref: String,
    /// Redaction posture; must be the metadata-safe default.
    pub redaction_posture: String,
    /// Human-readable note describing the live/buffered/snapshot truth.
    pub truth_note: String,
}

impl LogTruth {
    /// True when the log posture is metadata-safe.
    pub fn is_metadata_safe(&self) -> bool {
        self.redaction_posture == METADATA_SAFE_REDACTION_CLASS
    }

    /// True when an available log channel also carries an export-safe time range.
    pub fn export_range_present_when_available(&self) -> bool {
        !self.availability.is_available() || !self.export_time_range_ref.trim().is_empty()
    }
}

/// The support-bundle and escalation-packet linkage preserving Doctor identity
/// for a scenario without overcapturing raw content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportBundleLinkage {
    /// Opaque ref to the support-bundle manifest.
    pub bundle_manifest_ref: String,
    /// Opaque ref to the escalation packet.
    pub escalation_packet_ref: String,
    /// Finding ids preserved in the packet (each `doctor.finding.`-prefixed).
    pub preserved_finding_ids: Vec<String>,
    /// Repair ids preserved in the packet (each `repair.`-prefixed).
    pub preserved_repair_ids: Vec<String>,
    /// Opaque scope refs preserved (mount/port/tunnel/target identities).
    pub preserved_scope_refs: Vec<String>,
    /// Opaque refs to durable evidence carried for reconstruction.
    pub durable_evidence_refs: Vec<String>,
    /// Redaction posture; must be the metadata-safe default.
    pub redaction_class: String,
    /// True when raw private material is excluded from the packet.
    pub raw_private_material_excluded: bool,
    /// True when content overcapture is excluded (identity-only, no bodies).
    pub overcapture_excluded: bool,
}

impl SupportBundleLinkage {
    /// True when the linkage preserves the stable identity (manifest, escalation
    /// packet, findings, scope) needed to reconstruct the scenario.
    pub fn preserves_identity(&self) -> bool {
        !self.bundle_manifest_ref.trim().is_empty()
            && !self.escalation_packet_ref.trim().is_empty()
            && !self.preserved_finding_ids.is_empty()
            && !self.preserved_scope_refs.is_empty()
    }

    /// True when the linkage is metadata-safe.
    pub fn is_metadata_safe(&self) -> bool {
        self.redaction_class == METADATA_SAFE_REDACTION_CLASS
            && self.raw_private_material_excluded
            && self.overcapture_excluded
    }
}

/// One seeded container-dependent scenario in one M5 workflow surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerWorkspaceScenario {
    /// Unique scenario id.
    pub scenario_id: String,
    /// The M5 workflow surface this scenario is surfaced in.
    pub surface: WorkflowSurface,
    /// Reviewer-facing one-line summary.
    pub title: String,
    /// The active-engine summary row.
    pub engine_summary: EngineSummary,
    /// How the window relates to the container.
    pub workspace_mode: WorkspaceMode,
    /// The local/remote/managed boundary label.
    pub boundary_label: BoundaryLabel,
    /// Opaque identity of the container/devcontainer/image being rebuilt or
    /// attached; never empty, so no window is treated as merely "in a container".
    pub target_ref: String,
    /// The open-in-container preflight / rebuild review sheet.
    pub rebuild_review: RebuildReview,
    /// Live/buffered/snapshot log truth.
    pub log_truth: LogTruth,
    /// Initiating findings (surface-scoped, `doctor.finding.<surface>.`-prefixed).
    pub initiating_findings: Vec<String>,
    /// Support-bundle/escalation linkage.
    pub support_linkage: SupportBundleLinkage,
    /// Surfaces that render this scenario's identity.
    pub parity_surfaces: Vec<ParitySurface>,
    /// Locale-invariant machine-meaning keys.
    pub machine_meaning_keys: Vec<String>,
    /// Specific boundary explanation.
    pub explanation: String,
    /// The published preflight decision (validated against the recomputed gate).
    pub published_preflight_decision: PreflightDecision,
    /// The published preflight reason (validated against the recomputed gate).
    pub published_preflight_reason: PreflightReason,
    /// Reviewer notes.
    pub notes: String,
}

impl ContainerWorkspaceScenario {
    /// Recomputes the non-inheriting preflight gate decision for this scenario
    /// from its own engine reachability, support class, and disclosed side
    /// effects.
    ///
    /// Precedence (most to least narrowing): a policy-blocked or unreachable
    /// engine routes to a non-dead-end offer-alternative outcome; an unsupported
    /// engine, then a trust-gated hook, then any rebuild/port/mount side effect
    /// each force explicit disclosure; otherwise the reopen proceeds at full
    /// strength. No decision is inherited from another scenario.
    pub fn recompute_gate(&self) -> (PreflightDecision, PreflightReason) {
        match self.engine_summary.reachability {
            EngineReachability::PolicyBlocked => {
                return (
                    PreflightDecision::BlockedOfferAlternative,
                    PreflightReason::PolicyBlocked,
                );
            }
            EngineReachability::Unreachable => {
                return (
                    PreflightDecision::BlockedOfferAlternative,
                    PreflightReason::EngineUnreachable,
                );
            }
            EngineReachability::Reachable => {}
        }
        if self.engine_summary.support_class == SupportClass::Unsupported {
            return (
                PreflightDecision::ProceedWithDisclosure,
                PreflightReason::UnsupportedEngine,
            );
        }
        if self.rebuild_review.has_trust_gated_hooks() {
            return (
                PreflightDecision::ProceedWithDisclosure,
                PreflightReason::TrustGatedHooks,
            );
        }
        if self.rebuild_review.has_side_effects() {
            return (
                PreflightDecision::ProceedWithDisclosure,
                PreflightReason::SideEffectsRequireReview,
            );
        }
        (PreflightDecision::ProceedFull, PreflightReason::None)
    }

    /// True when the boundary label is consistent with the workspace mode: a
    /// managed mode must carry a managed/remote label, and any other mode must
    /// carry a local/remote label.
    pub fn boundary_is_consistent(&self) -> bool {
        match self.workspace_mode {
            WorkspaceMode::RemoteManaged => {
                matches!(
                    self.boundary_label,
                    BoundaryLabel::Managed | BoundaryLabel::Remote
                )
            }
            _ => matches!(
                self.boundary_label,
                BoundaryLabel::Local | BoundaryLabel::Remote
            ),
        }
    }

    /// True when the scenario renders on every parity surface.
    pub fn is_cross_surface_stable(&self) -> bool {
        ParitySurface::ALL
            .iter()
            .all(|surface| self.parity_surfaces.contains(surface))
    }
}

/// Roll-up summary over all scenarios.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorContainerBoundarySummary {
    /// Total scenarios.
    pub scenario_count: usize,
    /// Distinct workflow surfaces covered.
    pub surfaces_covered: usize,
    /// Scenarios that proceed at full strength.
    pub proceed_full_count: usize,
    /// Scenarios that proceed only with disclosure.
    pub proceed_with_disclosure_count: usize,
    /// Scenarios blocked with an offered alternative.
    pub blocked_count: usize,
    /// Scenarios whose preflight rebuilds the container.
    pub rebuild_count: usize,
    /// Scenarios disclosing at least one trust-gated hook.
    pub trust_gated_hook_count: usize,
    /// Scenarios whose engine is unreachable or policy-blocked.
    pub engine_unavailable_count: usize,
    /// Scenarios with no available log channel.
    pub log_unavailable_count: usize,
}

/// The canonical container-boundary-truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorContainerBoundaryTruth {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Packet id.
    pub packet_id: String,
    /// Publication status.
    pub status: String,
    /// Overview doc page.
    pub overview_page: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Capture date.
    pub as_of: String,
    /// Enumerated workflow-surface vocabulary.
    pub workflow_surfaces: Vec<String>,
    /// Enumerated engine-class vocabulary.
    pub engine_classes: Vec<String>,
    /// Enumerated engine-reachability vocabulary.
    pub engine_reachabilities: Vec<String>,
    /// Enumerated support-class vocabulary.
    pub support_classes: Vec<String>,
    /// Enumerated workspace-mode vocabulary.
    pub workspace_modes: Vec<String>,
    /// Enumerated boundary-label vocabulary.
    pub boundary_labels: Vec<String>,
    /// Enumerated log-availability vocabulary.
    pub log_availabilities: Vec<String>,
    /// Enumerated preflight-decision vocabulary.
    pub preflight_decisions: Vec<String>,
    /// Enumerated preflight-reason vocabulary.
    pub preflight_reasons: Vec<String>,
    /// Enumerated parity-surface vocabulary.
    pub parity_surfaces: Vec<String>,
    /// The seeded container-dependent scenarios.
    pub scenarios: Vec<ContainerWorkspaceScenario>,
    /// Roll-up summary.
    pub summary: ProjectDoctorContainerBoundarySummary,
}

impl ProjectDoctorContainerBoundaryTruth {
    /// Returns all scenarios in the given surface.
    pub fn scenarios_in_surface(
        &self,
        surface: WorkflowSurface,
    ) -> impl Iterator<Item = &ContainerWorkspaceScenario> {
        self.scenarios.iter().filter(move |s| s.surface == surface)
    }

    /// Recomputes the roll-up summary from the scenarios.
    pub fn computed_summary(&self) -> ProjectDoctorContainerBoundarySummary {
        let surfaces: BTreeSet<WorkflowSurface> =
            self.scenarios.iter().map(|s| s.surface).collect();
        ProjectDoctorContainerBoundarySummary {
            scenario_count: self.scenarios.len(),
            surfaces_covered: surfaces.len(),
            proceed_full_count: self
                .scenarios
                .iter()
                .filter(|s| s.published_preflight_decision == PreflightDecision::ProceedFull)
                .count(),
            proceed_with_disclosure_count: self
                .scenarios
                .iter()
                .filter(|s| {
                    s.published_preflight_decision == PreflightDecision::ProceedWithDisclosure
                })
                .count(),
            blocked_count: self
                .scenarios
                .iter()
                .filter(|s| {
                    s.published_preflight_decision == PreflightDecision::BlockedOfferAlternative
                })
                .count(),
            rebuild_count: self
                .scenarios
                .iter()
                .filter(|s| s.rebuild_review.rebuild_decision == RebuildDecision::Rebuild)
                .count(),
            trust_gated_hook_count: self
                .scenarios
                .iter()
                .filter(|s| s.rebuild_review.has_trust_gated_hooks())
                .count(),
            engine_unavailable_count: self
                .scenarios
                .iter()
                .filter(|s| !s.engine_summary.is_reachable())
                .count(),
            log_unavailable_count: self
                .scenarios
                .iter()
                .filter(|s| !s.log_truth.availability.is_available())
                .count(),
        }
    }

    /// Builds the metadata-safe support-export projection.
    pub fn export_projection(&self) -> ProjectDoctorContainerBoundaryExportProjection {
        ProjectDoctorContainerBoundaryExportProjection {
            packet_id: self.packet_id.clone(),
            schema_ref: self.schema_ref.clone(),
            rows: self
                .scenarios
                .iter()
                .map(|s| ProjectDoctorContainerBoundaryExportRow {
                    scenario_id: s.scenario_id.clone(),
                    surface: s.surface,
                    engine_class: s.engine_summary.engine_class,
                    reachability: s.engine_summary.reachability,
                    support_class: s.engine_summary.support_class,
                    workspace_mode: s.workspace_mode,
                    boundary_label: s.boundary_label,
                    preflight_decision: s.published_preflight_decision,
                    preflight_reason: s.published_preflight_reason,
                    log_availability: s.log_truth.availability,
                    bundle_manifest_ref: s.support_linkage.bundle_manifest_ref.clone(),
                    escalation_packet_ref: s.support_linkage.escalation_packet_ref.clone(),
                })
                .collect(),
            proceed_full_count: self
                .scenarios
                .iter()
                .filter(|s| s.published_preflight_decision == PreflightDecision::ProceedFull)
                .count(),
            raw_private_material_excluded: true,
        }
    }

    /// Validates the packet and returns every violation found.
    pub fn validate(&self) -> Vec<ProjectDoctorContainerBoundaryViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PROJECT_DOCTOR_CONTAINER_BOUNDARY_SCHEMA_VERSION {
            push(
                &mut violations,
                "container_boundary.schema_version",
                &self.packet_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != PROJECT_DOCTOR_CONTAINER_BOUNDARY_RECORD_KIND {
            push(
                &mut violations,
                "container_boundary.record_kind",
                &self.packet_id,
                "record_kind must be project_doctor_container_boundary_truth",
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("schema_ref", &self.schema_ref),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                push(
                    &mut violations,
                    "container_boundary.empty_field",
                    &self.packet_id,
                    format!("{field} must be non-empty"),
                );
            }
        }
        if self.schema_ref != PROJECT_DOCTOR_CONTAINER_BOUNDARY_SCHEMA_REF {
            push(
                &mut violations,
                "container_boundary.schema_ref",
                &self.packet_id,
                format!("schema_ref must equal {PROJECT_DOCTOR_CONTAINER_BOUNDARY_SCHEMA_REF}"),
            );
        }
        if self.overview_page != PROJECT_DOCTOR_CONTAINER_BOUNDARY_DOC_REF {
            push(
                &mut violations,
                "container_boundary.overview_page",
                &self.packet_id,
                format!("overview_page must equal {PROJECT_DOCTOR_CONTAINER_BOUNDARY_DOC_REF}"),
            );
        }
        if self.scenarios.is_empty() {
            push(
                &mut violations,
                "container_boundary.no_scenarios",
                &self.packet_id,
                "packet must contain at least one scenario",
            );
        }

        let mut seen_ids = BTreeSet::new();
        for scenario in &self.scenarios {
            self.validate_scenario(scenario, &mut seen_ids, &mut violations);
        }

        if self.summary != self.computed_summary() {
            push(
                &mut violations,
                "container_boundary.summary_mismatch",
                &self.packet_id,
                "summary does not match the recomputed summary",
            );
        }

        violations
    }

    fn validate_scenario(
        &self,
        scenario: &ContainerWorkspaceScenario,
        seen_ids: &mut BTreeSet<String>,
        violations: &mut Vec<ProjectDoctorContainerBoundaryViolation>,
    ) {
        let sid = scenario.scenario_id.as_str();
        if scenario.scenario_id.trim().is_empty() {
            push(
                violations,
                "container_boundary.scenario_id_empty",
                &self.packet_id,
                "scenario_id must be non-empty",
            );
        }
        if !seen_ids.insert(scenario.scenario_id.clone()) {
            push(
                violations,
                "container_boundary.scenario_id_duplicate",
                sid,
                "scenario_id must be unique",
            );
        }

        // Guardrail: never treat a window as merely "in a container" without a
        // named target identity.
        if scenario.target_ref.trim().is_empty() {
            push(
                violations,
                "container_boundary.target_ref_missing",
                sid,
                "scenario must name the container/devcontainer/image target_ref",
            );
        }

        // Initiating findings: present and surface-scoped.
        if scenario.initiating_findings.is_empty() {
            push(
                violations,
                "container_boundary.findings_missing",
                sid,
                "scenario must declare at least one initiating finding",
            );
        }
        let surface_prefix = scenario.surface.finding_code_prefix();
        for finding in &scenario.initiating_findings {
            if !finding.starts_with(DOCTOR_FINDING_PREFIX) {
                push(
                    violations,
                    "container_boundary.finding_prefix",
                    sid,
                    format!("initiating finding {finding} must start with {DOCTOR_FINDING_PREFIX}"),
                );
            } else if !finding.starts_with(&surface_prefix) {
                push(
                    violations,
                    "container_boundary.finding_surface_mismatch",
                    sid,
                    format!(
                        "initiating finding {finding} must be surface-scoped under {surface_prefix}"
                    ),
                );
            }
        }

        // Engine summary: a non-reachable engine must still offer diagnostics so
        // users are not stranded.
        if !scenario.engine_summary.is_reachable()
            && scenario.engine_summary.diagnostics_actions.is_empty()
        {
            push(
                violations,
                "container_boundary.no_diagnostics_when_unreachable",
                sid,
                "an unreachable or policy-blocked engine must offer at least one diagnostics action",
            );
        }
        if scenario.engine_summary.certification_note.trim().is_empty() {
            push(
                violations,
                "container_boundary.certification_note_missing",
                sid,
                "engine summary must carry a support/certification note",
            );
        }

        // Boundary label must be consistent with workspace mode.
        if !scenario.boundary_is_consistent() {
            push(
                violations,
                "container_boundary.boundary_label_inconsistent",
                sid,
                "boundary_label must be consistent with workspace_mode (managed mode → managed/remote; otherwise local/remote)",
            );
        }

        // Rebuild review: a non-empty stay-local alternative is always required.
        if scenario
            .rebuild_review
            .stay_local_alternative
            .trim()
            .is_empty()
        {
            push(
                violations,
                "container_boundary.no_stay_local_alternative",
                sid,
                "rebuild review must offer a non-empty stay-local alternative (no dead-end flow)",
            );
        }
        validate_ports(scenario, violations);

        // Log truth: available logs must carry an export-safe time range, and the
        // posture must be metadata-safe.
        if !scenario.log_truth.export_range_present_when_available() {
            push(
                violations,
                "container_boundary.log_export_range_missing",
                sid,
                "an available log channel must carry an export-safe time range ref",
            );
        }
        if !scenario.log_truth.is_metadata_safe() {
            push(
                violations,
                "container_boundary.log_posture_not_metadata_safe",
                sid,
                "log truth redaction_posture must be metadata_safe_default",
            );
        }

        // Support-bundle linkage.
        let linkage = &scenario.support_linkage;
        if !linkage.preserves_identity() {
            push(
                violations,
                "container_boundary.identity_not_preserved",
                sid,
                "support linkage must preserve manifest, escalation packet, findings, and scope",
            );
        }
        if !linkage.is_metadata_safe() {
            push(
                violations,
                "container_boundary.linkage_not_metadata_safe",
                sid,
                "support linkage must be metadata-safe (redaction_class metadata_safe_default, raw + overcapture excluded)",
            );
        }
        for finding in &linkage.preserved_finding_ids {
            if !finding.starts_with(DOCTOR_FINDING_PREFIX) {
                push(
                    violations,
                    "container_boundary.preserved_finding_prefix",
                    sid,
                    format!("preserved finding {finding} must start with {DOCTOR_FINDING_PREFIX}"),
                );
            }
        }
        for repair in &linkage.preserved_repair_ids {
            if !repair.starts_with(DOCTOR_REPAIR_PREFIX) {
                push(
                    violations,
                    "container_boundary.preserved_repair_prefix",
                    sid,
                    format!("preserved repair id {repair} must start with {DOCTOR_REPAIR_PREFIX}"),
                );
            }
        }

        // Cross-surface stability.
        if !scenario.is_cross_surface_stable() {
            push(
                violations,
                "container_boundary.parity_surface_missing",
                sid,
                "scenario must render on every parity surface",
            );
        }

        // Machine-meaning keys.
        for required in REQUIRED_MACHINE_MEANING_KEYS {
            if !scenario.machine_meaning_keys.iter().any(|k| k == required) {
                push(
                    violations,
                    "container_boundary.machine_meaning_key_missing",
                    sid,
                    format!("scenario must carry machine-meaning key {required}"),
                );
            }
        }

        // Explanation must be specific.
        let explanation = scenario.explanation.trim().to_ascii_lowercase();
        if explanation.is_empty() {
            push(
                violations,
                "container_boundary.explanation_empty",
                sid,
                "explanation must be non-empty",
            );
        } else if GENERIC_EXPLANATION_TOKENS.contains(&explanation.as_str()) {
            push(
                violations,
                "container_boundary.explanation_generic",
                sid,
                "explanation must be specific, not a generic token",
            );
        }

        // Non-inheriting preflight gate: published == recomputed.
        let (decision, reason) = scenario.recompute_gate();
        if scenario.published_preflight_decision != decision {
            push(
                violations,
                "container_boundary.gate_decision_mismatch",
                sid,
                format!(
                    "published_preflight_decision {} does not match recomputed gate decision {}",
                    scenario.published_preflight_decision.as_str(),
                    decision.as_str()
                ),
            );
        }
        if scenario.published_preflight_reason != reason {
            push(
                violations,
                "container_boundary.gate_reason_mismatch",
                sid,
                format!(
                    "published_preflight_reason {} does not match recomputed reason {}",
                    scenario.published_preflight_reason.as_str(),
                    reason.as_str()
                ),
            );
        }

        // Guardrail: trust-gated hooks never run silently.
        if scenario.rebuild_review.has_trust_gated_hooks()
            && scenario.published_preflight_decision == PreflightDecision::ProceedFull
        {
            push(
                violations,
                "container_boundary.trust_gated_hook_runs_silently",
                sid,
                "a scenario with trust-gated hooks must not proceed at full strength without disclosure",
            );
        }

        // Guardrail: blocked scenarios must offer a non-dead-end alternative.
        if scenario.published_preflight_decision == PreflightDecision::BlockedOfferAlternative
            && scenario
                .rebuild_review
                .stay_local_alternative
                .trim()
                .is_empty()
        {
            push(
                violations,
                "container_boundary.blocked_without_alternative",
                sid,
                "a blocked scenario must offer a stay-local alternative",
            );
        }

        // Guardrail: a full-strength proceed must be reachable, supported, free of
        // trust-gated hooks, and free of side effects. (The gate equality above
        // already enforces this; this explicit check keeps the guardrail legible
        // and independently tested.)
        if scenario.published_preflight_decision == PreflightDecision::ProceedFull
            && (!scenario.engine_summary.is_reachable()
                || scenario.engine_summary.support_class == SupportClass::Unsupported
                || scenario.rebuild_review.has_trust_gated_hooks()
                || scenario.rebuild_review.has_side_effects())
        {
            push(
                violations,
                "container_boundary.full_proceed_unsupported",
                sid,
                "proceed_full requires a reachable, supported engine with no trust-gated hooks and no side effects",
            );
        }
    }
}

fn validate_ports(
    scenario: &ContainerWorkspaceScenario,
    violations: &mut Vec<ProjectDoctorContainerBoundaryViolation>,
) {
    let sid = scenario.scenario_id.as_str();
    for port in &scenario.rebuild_review.published_ports {
        if port.container_port == 0 || port.host_port == 0 {
            push(
                violations,
                "container_boundary.port_invalid",
                sid,
                "published port mappings must use non-zero container and host ports",
            );
        }
    }
}

/// One validation violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorContainerBoundaryViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref (packet id or scenario id).
    pub subject_ref: String,
    /// Human-readable message.
    pub message: String,
}

/// One row of the metadata-safe support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorContainerBoundaryExportRow {
    /// Scenario id.
    pub scenario_id: String,
    /// Workflow surface.
    pub surface: WorkflowSurface,
    /// Engine class.
    pub engine_class: EngineClass,
    /// Engine reachability.
    pub reachability: EngineReachability,
    /// Support class.
    pub support_class: SupportClass,
    /// Workspace mode.
    pub workspace_mode: WorkspaceMode,
    /// Boundary label.
    pub boundary_label: BoundaryLabel,
    /// Preflight decision.
    pub preflight_decision: PreflightDecision,
    /// Preflight reason.
    pub preflight_reason: PreflightReason,
    /// Log availability.
    pub log_availability: LogAvailability,
    /// Support-bundle manifest ref.
    pub bundle_manifest_ref: String,
    /// Escalation packet ref.
    pub escalation_packet_ref: String,
}

/// The metadata-safe support-export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorContainerBoundaryExportProjection {
    /// Packet id.
    pub packet_id: String,
    /// Schema ref.
    pub schema_ref: String,
    /// One row per scenario.
    pub rows: Vec<ProjectDoctorContainerBoundaryExportRow>,
    /// Count of full-strength proceed scenarios.
    pub proceed_full_count: usize,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

fn push(
    violations: &mut Vec<ProjectDoctorContainerBoundaryViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(ProjectDoctorContainerBoundaryViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

/// Parses the embedded checked-in packet.
///
/// Returns a JSON parse error when the checked-in packet no longer matches the
/// typed model.
pub fn current_project_doctor_container_boundary_truth(
) -> Result<ProjectDoctorContainerBoundaryTruth, serde_json::Error> {
    serde_json::from_str(PROJECT_DOCTOR_CONTAINER_BOUNDARY_JSON)
}

#[cfg(test)]
mod tests;
