//! Published-port/tunnel revocation, writable-mount and lifecycle-script
//! disclosure, and browser/companion handoff truth for container/devcontainer
//! flows.
//!
//! This module owns one canonical, checked-in packet —
//! [`ProjectDoctorContainerHandoffTruth`] — that keeps three otherwise-easy-to-
//! flatten facts first-class on every container-route share: *which route is
//! published and how it is revoked*, *which environment mutation the reopen
//! actually carries*, and *who the browser or companion handoff actually points
//! at*. Each [`ContainerHandoffScenario`] seeds one container-route situation in
//! one workflow surface (remote preview, incident workflow, or companion follow)
//! and pins, in one record:
//!
//! - a [`PublishedRoute`] naming the [`RouteKind`] (published port or tunnel),
//!   the target/service identity and port, the [`AudienceScope`], the
//!   [`PolicyPosture`], a time bound ([`RouteTimeBound`]) that is never unbounded,
//!   and a revocation block ([`RouteRevocation`]) that always carries a non-empty
//!   revocation path — so a tunnel or published port can never behave like a
//!   durable silent share,
//! - an [`EnvironmentMutationDisclosure`] listing the writable mounts and
//!   lifecycle/install scripts the reopen carries and the flows the disclosure
//!   survives in ([`DisclosureFlow`]: reopen, attach, rebuild, export, support
//!   bundle), so writable mounts and lifecycle/install scripts are never reduced
//!   to undocumented side effects, and
//! - a [`HandoffPacket`] attributing the browser or companion handoff to its
//!   owner/origin, engine, target, route, and target service, plus its
//!   live-versus-snapshot freshness ([`HandoffLiveness`]) and revocation
//!   visibility — so a handoff is never a durable opaque share, and any write
//!   channel is bounded and approval-gated ([`HandoffMutationScope`]).
//!
//! The central guarantee is a **non-inheriting handoff gate**: every scenario's
//! published [`ContainerHandoffScenario::published_handoff_posture`] and
//! [`ContainerHandoffScenario::published_handoff_reason`] are validated against
//! the posture recomputed from the scenario's own policy posture, route
//! revocation state, audience scope, write scope, and disclosed environment
//! mutation ([`ContainerHandoffScenario::recompute_gate`]). A policy-blocked
//! route routes to a non-dead-end offer-alternative outcome; a revoked or expired
//! route collapses the handoff to a snapshot-only share rather than a live opaque
//! link; a public audience, a bounded write channel, a policy restriction, or any
//! disclosed environment mutation forces explicit disclosure rather than a silent
//! live share.
//!
//! The packet is checked in at
//! `artifacts/doctor/m5/project-doctor-container-handoff-truth.json` and embedded
//! here via `include_str!`, so this typed consumer and any CI gate agree on every
//! row without a cargo build in CI. The model is metadata-only: every field is a
//! typed state or an opaque ref. It carries no credential bodies, raw provider
//! payloads, or mount/port/tunnel secrets.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Supported container-handoff-truth packet schema version.
pub const PROJECT_DOCTOR_CONTAINER_HANDOFF_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const PROJECT_DOCTOR_CONTAINER_HANDOFF_RECORD_KIND: &str =
    "project_doctor_container_handoff_truth";

/// Repo-relative path to the checked-in packet.
pub const PROJECT_DOCTOR_CONTAINER_HANDOFF_PATH: &str =
    "artifacts/doctor/m5/project-doctor-container-handoff-truth.json";

/// Repo-relative path to the handoff schema.
pub const PROJECT_DOCTOR_CONTAINER_HANDOFF_SCHEMA_REF: &str =
    "schemas/doctor/project-doctor-container-handoff-truth.schema.json";

/// Repo-relative path to the companion document.
pub const PROJECT_DOCTOR_CONTAINER_HANDOFF_DOC_REF: &str =
    "docs/doctor/m5/project-doctor-container-handoff-truth.md";

/// Stable finding-code prefix every initiating finding must use.
pub const DOCTOR_FINDING_PREFIX: &str = "doctor.finding.";

/// Stable repair-id prefix every preserved repair id must use.
pub const DOCTOR_REPAIR_PREFIX: &str = "repair.";

/// Required redaction class for every support linkage.
pub const METADATA_SAFE_REDACTION_CLASS: &str = "metadata_safe_default";

/// Canonical, locale-invariant machine-meaning keys every scenario must carry,
/// so localized prose can never silently change what a surface means.
pub const REQUIRED_MACHINE_MEANING_KEYS: [&str; 5] = [
    "scenario_id",
    "surface",
    "engine_class",
    "route_kind",
    "handoff_posture",
];

/// Embedded checked-in packet JSON.
pub const PROJECT_DOCTOR_CONTAINER_HANDOFF_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/doctor/m5/project-doctor-container-handoff-truth.json"
));

/// Generic, non-actionable explanation tokens that may never stand in for a
/// specific handoff explanation.
const GENERIC_EXPLANATION_TOKENS: [&str; 7] = [
    "unavailable",
    "error",
    "failed",
    "failure",
    "unknown",
    "shared",
    "n_a",
];

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// The workflow surface a container-route handoff is surfaced in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffSurface {
    /// Remote preview route, port, and tunnel workflow.
    RemotePreview,
    /// Incident packet and recovery workflow.
    IncidentWorkflow,
    /// Companion-safe session follow workflow.
    CompanionFollow,
}

impl HandoffSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::RemotePreview,
        Self::IncidentWorkflow,
        Self::CompanionFollow,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemotePreview => "remote_preview",
            Self::IncidentWorkflow => "incident_workflow",
            Self::CompanionFollow => "companion_follow",
        }
    }

    /// The stable finding-code prefix every initiating finding on this surface
    /// must start with.
    pub fn finding_code_prefix(self) -> String {
        format!("{DOCTOR_FINDING_PREFIX}{}.", self.as_str())
    }
}

impl std::fmt::Display for HandoffSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The active container engine class behind a route.
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

/// The local/remote/managed boundary label that must survive into handoff
/// surfaces.
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

impl std::fmt::Display for BoundaryLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Whether a route is a published host port or a tunnel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteKind {
    /// A port published on a host.
    PublishedPort,
    /// A tunnel exposing a service beyond the host.
    Tunnel,
}

impl RouteKind {
    /// Every route kind, in declaration order.
    pub const ALL: [Self; 2] = [Self::PublishedPort, Self::Tunnel];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublishedPort => "published_port",
            Self::Tunnel => "tunnel",
        }
    }
}

impl std::fmt::Display for RouteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Who can reach a published route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudienceScope {
    /// Bound to localhost only.
    LocalOnly,
    /// Visible on the local network.
    Lan,
    /// Reachable by authenticated members of the working team.
    AuthenticatedTeam,
    /// Reachable across the org.
    Org,
    /// Reachable publicly.
    Public,
}

impl AudienceScope {
    /// Every audience scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOnly,
        Self::Lan,
        Self::AuthenticatedTeam,
        Self::Org,
        Self::Public,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Lan => "lan",
            Self::AuthenticatedTeam => "authenticated_team",
            Self::Org => "org",
            Self::Public => "public",
        }
    }
}

/// The policy posture governing a route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyPosture {
    /// Policy allows the route as published.
    PolicyAllowed,
    /// Policy allows the route only with a narrowed audience and disclosure.
    PolicyRestricted,
    /// Policy blocks the route in this context.
    PolicyBlocked,
}

impl PolicyPosture {
    /// Every policy posture, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::PolicyAllowed,
        Self::PolicyRestricted,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyAllowed => "policy_allowed",
            Self::PolicyRestricted => "policy_restricted",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// How a route's lifetime is bounded. No variant represents an unbounded share.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeBoundClass {
    /// Lives only as long as the current session.
    SessionBound,
    /// Lives for a fixed rolling window.
    TimeBoxed,
    /// Lives until a fixed deadline.
    Deadline,
}

impl TimeBoundClass {
    /// Every time-bound class, in declaration order.
    pub const ALL: [Self; 3] = [Self::SessionBound, Self::TimeBoxed, Self::Deadline];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionBound => "session_bound",
            Self::TimeBoxed => "time_boxed",
            Self::Deadline => "deadline",
        }
    }
}

/// The revocation state of a route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevocationState {
    /// The route is currently live.
    Active,
    /// The route's time bound elapsed.
    Expired,
    /// The route was explicitly revoked.
    Revoked,
}

impl RevocationState {
    /// Every revocation state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Active, Self::Expired, Self::Revoked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
        }
    }

    /// True when the route is no longer live (expired or revoked).
    pub const fn is_dead(self) -> bool {
        matches!(self, Self::Expired | Self::Revoked)
    }
}

/// The kind of a mount binding disclosed by an environment-mutation disclosure.
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

/// The kind of a disclosed lifecycle or install script.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptKind {
    /// Runs on the host before the container is created.
    Initialize,
    /// Runs once when the container is created.
    OnCreate,
    /// Runs after creation completes.
    PostCreate,
    /// Runs each time the container starts.
    PostStart,
    /// Runs each time a client attaches.
    PostAttach,
    /// A dependency/install script invoked by the definition.
    InstallScript,
}

impl ScriptKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::OnCreate => "on_create",
            Self::PostCreate => "post_create",
            Self::PostStart => "post_start",
            Self::PostAttach => "post_attach",
            Self::InstallScript => "install_script",
        }
    }
}

/// A flow the environment-mutation disclosure must survive into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureFlow {
    /// Reopening the workspace in the container.
    Reopen,
    /// Attaching to an existing container.
    Attach,
    /// Rebuilding the container.
    Rebuild,
    /// Exporting the workspace/handoff.
    Export,
    /// A support-bundle capture.
    SupportBundle,
}

impl DisclosureFlow {
    /// Every flow the disclosure must survive into, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Reopen,
        Self::Attach,
        Self::Rebuild,
        Self::Export,
        Self::SupportBundle,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reopen => "reopen",
            Self::Attach => "attach",
            Self::Rebuild => "rebuild",
            Self::Export => "export",
            Self::SupportBundle => "support_bundle",
        }
    }
}

/// The channel a handoff is rendered on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffChannel {
    /// A browser handoff.
    Browser,
    /// A companion (mobile/desktop follow) handoff.
    Companion,
}

impl HandoffChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 2] = [Self::Browser, Self::Companion];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Browser => "browser",
            Self::Companion => "companion",
        }
    }
}

/// Whether a handoff is a live channel or a captured snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffLiveness {
    /// A live channel attached to a live route.
    Live,
    /// A captured point-in-time snapshot, not a live channel.
    Snapshot,
}

impl HandoffLiveness {
    /// Every liveness, in declaration order.
    pub const ALL: [Self; 2] = [Self::Live, Self::Snapshot];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Snapshot => "snapshot",
        }
    }
}

/// The mutation scope of a handoff channel. No variant represents an
/// unrestricted mutate channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffMutationScope {
    /// The handoff is read-only.
    ReadOnly,
    /// The handoff carries a bounded write channel that must be approval-gated.
    BoundedWrite,
}

impl HandoffMutationScope {
    /// Every mutation scope, in declaration order.
    pub const ALL: [Self; 2] = [Self::ReadOnly, Self::BoundedWrite];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::BoundedWrite => "bounded_write",
        }
    }
}

/// The handoff gate posture for a scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffPosture {
    /// A live, read-only, in-audience share; full clarity, minimal disclosure.
    ShareLive,
    /// Proceed only after the disclosed scope/expiry/mutation is reviewed.
    ShareWithDisclosure,
    /// The route is dead; only a snapshot handoff remains, never a live share.
    ShareSnapshotOnly,
    /// Do not share; offer the stay-local (or other) alternative.
    BlockedOfferAlternative,
}

impl HandoffPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ShareLive,
        Self::ShareWithDisclosure,
        Self::ShareSnapshotOnly,
        Self::BlockedOfferAlternative,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShareLive => "share_live",
            Self::ShareWithDisclosure => "share_with_disclosure",
            Self::ShareSnapshotOnly => "share_snapshot_only",
            Self::BlockedOfferAlternative => "blocked_offer_alternative",
        }
    }
}

/// Why the handoff gate narrowed or blocked a scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffReason {
    /// No narrowing applied; the handoff shares at full strength.
    None,
    /// The route audience is public and must be disclosed.
    AudiencePublic,
    /// Policy restricts the route to a narrowed audience.
    PolicyRestricted,
    /// The handoff carries a bounded write channel that needs approval.
    BoundedWriteRequiresApproval,
    /// The reopen carries disclosed environment mutation.
    EnvironmentMutationDisclosed,
    /// The route's time bound elapsed.
    RouteExpired,
    /// The route was explicitly revoked.
    RouteRevoked,
    /// Policy blocks the route in this context.
    PolicyBlocked,
}

impl HandoffReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::None,
        Self::AudiencePublic,
        Self::PolicyRestricted,
        Self::BoundedWriteRequiresApproval,
        Self::EnvironmentMutationDisclosed,
        Self::RouteExpired,
        Self::RouteRevoked,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AudiencePublic => "audience_public",
            Self::PolicyRestricted => "policy_restricted",
            Self::BoundedWriteRequiresApproval => "bounded_write_requires_approval",
            Self::EnvironmentMutationDisclosed => "environment_mutation_disclosed",
            Self::RouteExpired => "route_expired",
            Self::RouteRevoked => "route_revoked",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// A surface that must render the same scenario identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParitySurface {
    /// Desktop route/handoff review sheet.
    DesktopSheet,
    /// CLI/headless inspect row.
    CliInspect,
    /// Headless machine-readable JSON.
    HeadlessJson,
    /// Browser handoff view.
    BrowserHandoff,
    /// Companion handoff view.
    CompanionHandoff,
    /// Support-bundle export.
    SupportExport,
    /// Incident-packet view.
    IncidentPacket,
}

impl ParitySurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DesktopSheet,
        Self::CliInspect,
        Self::HeadlessJson,
        Self::BrowserHandoff,
        Self::CompanionHandoff,
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
            Self::CompanionHandoff => "companion_handoff",
            Self::SupportExport => "support_export",
            Self::IncidentPacket => "incident_packet",
        }
    }
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// The time bound on a route. A route is never published unbounded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteTimeBound {
    /// How the lifetime is bounded.
    pub class: TimeBoundClass,
    /// Opaque ref to the concrete expiry; never empty.
    pub expires_at_ref: String,
    /// Human-readable note on the maximum lifetime.
    pub max_lifetime_note: String,
}

impl RouteTimeBound {
    /// True when the route carries a concrete expiry ref.
    pub fn is_time_bound(&self) -> bool {
        !self.expires_at_ref.trim().is_empty()
    }
}

/// The revocation block on a route. The revocation path is always first-class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteRevocation {
    /// The current revocation state.
    pub state: RevocationState,
    /// Opaque ref to the action that revokes the route; never empty.
    pub revocation_action_ref: String,
    /// Opaque ref to the revocation/expiry evidence; present once dead.
    pub revoked_evidence_ref: String,
}

impl RouteRevocation {
    /// True when the route exposes a non-empty revocation path.
    pub fn is_revocable(&self) -> bool {
        !self.revocation_action_ref.trim().is_empty()
    }
}

/// A published port or tunnel route, kept inspectable and revocable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishedRoute {
    /// Whether the route is a published port or a tunnel.
    pub route_kind: RouteKind,
    /// Opaque route id; never empty.
    pub route_id: String,
    /// Opaque identity of the engine/container target behind the route.
    pub target_ref: String,
    /// Opaque identity of the target service exposed by the route.
    pub target_service_ref: String,
    /// Port inside the container/service.
    pub container_port: u32,
    /// Port published on the host (or local tunnel bind).
    pub host_port: u32,
    /// Who can reach the route.
    pub audience_scope: AudienceScope,
    /// The policy posture governing the route.
    pub policy_posture: PolicyPosture,
    /// Human-readable policy note.
    pub policy_note: String,
    /// The route's time bound.
    pub time_bound: RouteTimeBound,
    /// The route's revocation block.
    pub revocation: RouteRevocation,
}

/// One mount binding disclosed by an environment-mutation disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MountBinding {
    /// The kind of mount.
    pub kind: MountKind,
    /// Opaque scope ref for the mount source.
    pub scope_ref: String,
    /// True when the mount is writable.
    pub writable: bool,
}

/// One lifecycle or install script disclosed by an environment-mutation
/// disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleScript {
    /// The kind of script.
    pub kind: ScriptKind,
    /// Opaque ref to the command the script runs.
    pub command_ref: String,
    /// True when the script is trust-gated and must be reviewed before it runs.
    pub trust_gated: bool,
    /// True when the script writes outside the container (host/workspace).
    pub writes_outside_container: bool,
}

/// The writable-mount and lifecycle/install-script disclosure carried into every
/// reopen, attach, rebuild, export, and support flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentMutationDisclosure {
    /// Disclosed mounts (read-only mounts may also be listed).
    pub writable_mounts: Vec<MountBinding>,
    /// Disclosed lifecycle/install scripts.
    pub lifecycle_scripts: Vec<LifecycleScript>,
    /// The flows this disclosure survives into.
    pub disclosure_persists_in: Vec<DisclosureFlow>,
    /// Human-readable summary of the disclosed environment mutation.
    pub disclosure_note: String,
}

impl EnvironmentMutationDisclosure {
    /// True when any disclosed mount is actually writable.
    pub fn has_writable_mount(&self) -> bool {
        self.writable_mounts.iter().any(|m| m.writable)
    }

    /// True when any disclosed script is trust-gated.
    pub fn has_trust_gated_script(&self) -> bool {
        self.lifecycle_scripts.iter().any(|s| s.trust_gated)
    }

    /// True when any disclosed script writes outside the container.
    pub fn has_external_write(&self) -> bool {
        self.lifecycle_scripts
            .iter()
            .any(|s| s.writes_outside_container)
    }

    /// True when the disclosure carries environment mutation that must be
    /// reviewed before the reopen proceeds.
    pub fn requires_disclosure(&self) -> bool {
        self.has_writable_mount() || self.has_trust_gated_script() || self.has_external_write()
    }

    /// True when the disclosure survives into every required flow.
    pub fn survives_required_flows(&self) -> bool {
        DisclosureFlow::ALL
            .iter()
            .all(|flow| self.disclosure_persists_in.contains(flow))
    }
}

/// A browser or companion handoff attributed to its owner, engine, target, and
/// route rather than flattened into an opaque URL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffPacket {
    /// The channel the handoff renders on.
    pub channel: HandoffChannel,
    /// Opaque handoff id; never empty.
    pub handoff_id: String,
    /// Opaque owner identity; never empty.
    pub owner_ref: String,
    /// Opaque origin (host/workspace) identity; never empty.
    pub origin_ref: String,
    /// The engine class behind the handoff.
    pub engine_class: EngineClass,
    /// Opaque target identity; never empty and equal to the route target.
    pub target_ref: String,
    /// Opaque route id; never empty and equal to the route id.
    pub route_id: String,
    /// Opaque target service identity; equal to the route target service.
    pub target_service_ref: String,
    /// Whether the handoff is a live channel or a snapshot.
    pub liveness: HandoffLiveness,
    /// Opaque ref to the capture time; present for a snapshot.
    pub captured_at_ref: String,
    /// True when the handoff renders the route's revocation state.
    pub revocation_visible: bool,
    /// The mutation scope of the handoff channel.
    pub mutation_scope: HandoffMutationScope,
    /// True when a write channel is approval-gated.
    pub approval_gated: bool,
}

impl HandoffPacket {
    /// True when the handoff preserves the attribution that keeps it from being a
    /// durable opaque share (owner, origin, target, route, service).
    pub fn preserves_attribution(&self) -> bool {
        !self.handoff_id.trim().is_empty()
            && !self.owner_ref.trim().is_empty()
            && !self.origin_ref.trim().is_empty()
            && !self.target_ref.trim().is_empty()
            && !self.route_id.trim().is_empty()
            && !self.target_service_ref.trim().is_empty()
    }

    /// True when any write channel is approval-gated (no unrestricted mutate).
    pub fn write_channel_is_gated(&self) -> bool {
        self.mutation_scope == HandoffMutationScope::ReadOnly || self.approval_gated
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
    /// Opaque scope refs preserved (route/port/tunnel/mount/target identities).
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
    /// True when the linkage preserves the stable identity needed to reconstruct
    /// the scenario.
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

/// One seeded container-route handoff scenario in one workflow surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerHandoffScenario {
    /// Unique scenario id.
    pub scenario_id: String,
    /// The workflow surface this scenario is surfaced in.
    pub surface: HandoffSurface,
    /// Reviewer-facing one-line summary.
    pub title: String,
    /// The active engine class.
    pub engine_class: EngineClass,
    /// The local/remote/managed boundary label.
    pub boundary_label: BoundaryLabel,
    /// The published port or tunnel route.
    pub route: PublishedRoute,
    /// The writable-mount and lifecycle/install-script disclosure.
    pub environment_disclosure: EnvironmentMutationDisclosure,
    /// The browser or companion handoff.
    pub handoff: HandoffPacket,
    /// The stay-local (or otherwise non-share) alternative offered; never empty.
    pub stay_local_alternative: String,
    /// Initiating findings (surface-scoped, `doctor.finding.<surface>.`-prefixed).
    pub initiating_findings: Vec<String>,
    /// Support-bundle/escalation linkage.
    pub support_linkage: SupportBundleLinkage,
    /// Surfaces that render this scenario's identity.
    pub parity_surfaces: Vec<ParitySurface>,
    /// Locale-invariant machine-meaning keys.
    pub machine_meaning_keys: Vec<String>,
    /// Specific handoff explanation.
    pub explanation: String,
    /// The published handoff posture (validated against the recomputed gate).
    pub published_handoff_posture: HandoffPosture,
    /// The published handoff reason (validated against the recomputed gate).
    pub published_handoff_reason: HandoffReason,
    /// Reviewer notes.
    pub notes: String,
}

impl ContainerHandoffScenario {
    /// Recomputes the non-inheriting handoff gate posture for this scenario from
    /// its own policy posture, route revocation state, audience scope, write
    /// scope, and disclosed environment mutation.
    ///
    /// Precedence (most to least narrowing): a policy-blocked route routes to a
    /// non-dead-end offer-alternative outcome; a revoked then expired route
    /// collapses to a snapshot-only share; a bounded write channel, then a public
    /// audience, then a policy restriction, then any disclosed environment
    /// mutation each force explicit disclosure; otherwise the share proceeds live
    /// at full strength. No posture is inherited from another scenario.
    pub fn recompute_gate(&self) -> (HandoffPosture, HandoffReason) {
        if self.route.policy_posture == PolicyPosture::PolicyBlocked {
            return (
                HandoffPosture::BlockedOfferAlternative,
                HandoffReason::PolicyBlocked,
            );
        }
        match self.route.revocation.state {
            RevocationState::Revoked => {
                return (
                    HandoffPosture::ShareSnapshotOnly,
                    HandoffReason::RouteRevoked,
                );
            }
            RevocationState::Expired => {
                return (
                    HandoffPosture::ShareSnapshotOnly,
                    HandoffReason::RouteExpired,
                );
            }
            RevocationState::Active => {}
        }
        if self.handoff.mutation_scope == HandoffMutationScope::BoundedWrite {
            return (
                HandoffPosture::ShareWithDisclosure,
                HandoffReason::BoundedWriteRequiresApproval,
            );
        }
        if self.route.audience_scope == AudienceScope::Public {
            return (
                HandoffPosture::ShareWithDisclosure,
                HandoffReason::AudiencePublic,
            );
        }
        if self.route.policy_posture == PolicyPosture::PolicyRestricted {
            return (
                HandoffPosture::ShareWithDisclosure,
                HandoffReason::PolicyRestricted,
            );
        }
        if self.environment_disclosure.requires_disclosure() {
            return (
                HandoffPosture::ShareWithDisclosure,
                HandoffReason::EnvironmentMutationDisclosed,
            );
        }
        (HandoffPosture::ShareLive, HandoffReason::None)
    }

    /// True when the boundary label is consistent with the engine class: a
    /// managed-cloud engine carries the managed boundary, and any other engine
    /// carries a local/remote boundary.
    pub fn boundary_is_consistent(&self) -> bool {
        match self.engine_class {
            EngineClass::ManagedCloud => self.boundary_label == BoundaryLabel::Managed,
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
pub struct ProjectDoctorContainerHandoffSummary {
    /// Total scenarios.
    pub scenario_count: usize,
    /// Distinct workflow surfaces covered.
    pub surfaces_covered: usize,
    /// Scenarios that share live at full strength.
    pub share_live_count: usize,
    /// Scenarios that share only with disclosure.
    pub share_with_disclosure_count: usize,
    /// Scenarios collapsed to a snapshot-only share.
    pub share_snapshot_only_count: usize,
    /// Scenarios blocked with an offered alternative.
    pub blocked_count: usize,
    /// Scenarios whose route is a tunnel.
    pub tunnel_count: usize,
    /// Scenarios whose route audience is public.
    pub public_audience_count: usize,
    /// Scenarios whose route is revoked or expired.
    pub revoked_or_expired_count: usize,
    /// Scenarios whose handoff carries a bounded write channel.
    pub bounded_write_count: usize,
}

/// The canonical container-handoff-truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorContainerHandoffTruth {
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
    /// Handoff schema ref.
    pub schema_ref: String,
    /// Capture date.
    pub as_of: String,
    /// Enumerated workflow-surface vocabulary.
    pub workflow_surfaces: Vec<String>,
    /// Enumerated engine-class vocabulary.
    pub engine_classes: Vec<String>,
    /// Enumerated boundary-label vocabulary.
    pub boundary_labels: Vec<String>,
    /// Enumerated route-kind vocabulary.
    pub route_kinds: Vec<String>,
    /// Enumerated audience-scope vocabulary.
    pub audience_scopes: Vec<String>,
    /// Enumerated policy-posture vocabulary.
    pub policy_postures: Vec<String>,
    /// Enumerated time-bound-class vocabulary.
    pub time_bound_classes: Vec<String>,
    /// Enumerated revocation-state vocabulary.
    pub revocation_states: Vec<String>,
    /// Enumerated handoff-channel vocabulary.
    pub handoff_channels: Vec<String>,
    /// Enumerated handoff-liveness vocabulary.
    pub handoff_livenesses: Vec<String>,
    /// Enumerated handoff-mutation-scope vocabulary.
    pub handoff_mutation_scopes: Vec<String>,
    /// Enumerated handoff-posture vocabulary.
    pub handoff_postures: Vec<String>,
    /// Enumerated handoff-reason vocabulary.
    pub handoff_reasons: Vec<String>,
    /// Enumerated disclosure-flow vocabulary.
    pub disclosure_flows: Vec<String>,
    /// Enumerated parity-surface vocabulary.
    pub parity_surfaces: Vec<String>,
    /// The seeded container-route handoff scenarios.
    pub scenarios: Vec<ContainerHandoffScenario>,
    /// Roll-up summary.
    pub summary: ProjectDoctorContainerHandoffSummary,
}

impl ProjectDoctorContainerHandoffTruth {
    /// Returns all scenarios in the given surface.
    pub fn scenarios_in_surface(
        &self,
        surface: HandoffSurface,
    ) -> impl Iterator<Item = &ContainerHandoffScenario> {
        self.scenarios.iter().filter(move |s| s.surface == surface)
    }

    /// Recomputes the roll-up summary from the scenarios.
    pub fn computed_summary(&self) -> ProjectDoctorContainerHandoffSummary {
        let surfaces: BTreeSet<HandoffSurface> = self.scenarios.iter().map(|s| s.surface).collect();
        ProjectDoctorContainerHandoffSummary {
            scenario_count: self.scenarios.len(),
            surfaces_covered: surfaces.len(),
            share_live_count: self
                .scenarios
                .iter()
                .filter(|s| s.published_handoff_posture == HandoffPosture::ShareLive)
                .count(),
            share_with_disclosure_count: self
                .scenarios
                .iter()
                .filter(|s| s.published_handoff_posture == HandoffPosture::ShareWithDisclosure)
                .count(),
            share_snapshot_only_count: self
                .scenarios
                .iter()
                .filter(|s| s.published_handoff_posture == HandoffPosture::ShareSnapshotOnly)
                .count(),
            blocked_count: self
                .scenarios
                .iter()
                .filter(|s| s.published_handoff_posture == HandoffPosture::BlockedOfferAlternative)
                .count(),
            tunnel_count: self
                .scenarios
                .iter()
                .filter(|s| s.route.route_kind == RouteKind::Tunnel)
                .count(),
            public_audience_count: self
                .scenarios
                .iter()
                .filter(|s| s.route.audience_scope == AudienceScope::Public)
                .count(),
            revoked_or_expired_count: self
                .scenarios
                .iter()
                .filter(|s| s.route.revocation.state.is_dead())
                .count(),
            bounded_write_count: self
                .scenarios
                .iter()
                .filter(|s| s.handoff.mutation_scope == HandoffMutationScope::BoundedWrite)
                .count(),
        }
    }

    /// Builds the metadata-safe support-export projection.
    pub fn export_projection(&self) -> ProjectDoctorContainerHandoffExportProjection {
        ProjectDoctorContainerHandoffExportProjection {
            packet_id: self.packet_id.clone(),
            schema_ref: self.schema_ref.clone(),
            rows: self
                .scenarios
                .iter()
                .map(|s| ProjectDoctorContainerHandoffExportRow {
                    scenario_id: s.scenario_id.clone(),
                    surface: s.surface,
                    engine_class: s.engine_class,
                    route_kind: s.route.route_kind,
                    route_id: s.route.route_id.clone(),
                    target_service_ref: s.route.target_service_ref.clone(),
                    audience_scope: s.route.audience_scope,
                    policy_posture: s.route.policy_posture,
                    revocation_state: s.route.revocation.state,
                    revocation_action_ref: s.route.revocation.revocation_action_ref.clone(),
                    expires_at_ref: s.route.time_bound.expires_at_ref.clone(),
                    handoff_channel: s.handoff.channel,
                    handoff_liveness: s.handoff.liveness,
                    handoff_posture: s.published_handoff_posture,
                    handoff_reason: s.published_handoff_reason,
                    bundle_manifest_ref: s.support_linkage.bundle_manifest_ref.clone(),
                    escalation_packet_ref: s.support_linkage.escalation_packet_ref.clone(),
                })
                .collect(),
            share_live_count: self
                .scenarios
                .iter()
                .filter(|s| s.published_handoff_posture == HandoffPosture::ShareLive)
                .count(),
            raw_private_material_excluded: true,
        }
    }

    /// Validates the packet and returns every violation found.
    pub fn validate(&self) -> Vec<ProjectDoctorContainerHandoffViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PROJECT_DOCTOR_CONTAINER_HANDOFF_SCHEMA_VERSION {
            push(
                &mut violations,
                "container_handoff.schema_version",
                &self.packet_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != PROJECT_DOCTOR_CONTAINER_HANDOFF_RECORD_KIND {
            push(
                &mut violations,
                "container_handoff.record_kind",
                &self.packet_id,
                "record_kind must be project_doctor_container_handoff_truth",
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
                    "container_handoff.empty_field",
                    &self.packet_id,
                    format!("{field} must be non-empty"),
                );
            }
        }
        if self.schema_ref != PROJECT_DOCTOR_CONTAINER_HANDOFF_SCHEMA_REF {
            push(
                &mut violations,
                "container_handoff.schema_ref",
                &self.packet_id,
                format!("schema_ref must equal {PROJECT_DOCTOR_CONTAINER_HANDOFF_SCHEMA_REF}"),
            );
        }
        if self.overview_page != PROJECT_DOCTOR_CONTAINER_HANDOFF_DOC_REF {
            push(
                &mut violations,
                "container_handoff.overview_page",
                &self.packet_id,
                format!("overview_page must equal {PROJECT_DOCTOR_CONTAINER_HANDOFF_DOC_REF}"),
            );
        }
        if self.scenarios.is_empty() {
            push(
                &mut violations,
                "container_handoff.no_scenarios",
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
                "container_handoff.summary_mismatch",
                &self.packet_id,
                "summary does not match the recomputed summary",
            );
        }

        violations
    }

    fn validate_scenario(
        &self,
        scenario: &ContainerHandoffScenario,
        seen_ids: &mut BTreeSet<String>,
        violations: &mut Vec<ProjectDoctorContainerHandoffViolation>,
    ) {
        let sid = scenario.scenario_id.as_str();
        if scenario.scenario_id.trim().is_empty() {
            push(
                violations,
                "container_handoff.scenario_id_empty",
                &self.packet_id,
                "scenario_id must be non-empty",
            );
        }
        if !seen_ids.insert(scenario.scenario_id.clone()) {
            push(
                violations,
                "container_handoff.scenario_id_duplicate",
                sid,
                "scenario_id must be unique",
            );
        }

        validate_findings(scenario, violations);
        validate_route(scenario, violations);
        validate_environment_disclosure(scenario, violations);
        validate_handoff(scenario, violations);
        validate_linkage(scenario, violations);
        validate_parity_and_meaning(scenario, violations);
        validate_gate(scenario, violations);

        // Boundary consistency.
        if !scenario.boundary_is_consistent() {
            push(
                violations,
                "container_handoff.boundary_label_inconsistent",
                sid,
                "boundary_label must be consistent with engine_class (managed_cloud → managed; otherwise local/remote)",
            );
        }

        // No dead-end flow: a stay-local alternative is always required.
        if scenario.stay_local_alternative.trim().is_empty() {
            push(
                violations,
                "container_handoff.no_stay_local_alternative",
                sid,
                "scenario must offer a non-empty stay-local alternative (no dead-end flow)",
            );
        }
    }
}

fn validate_findings(
    scenario: &ContainerHandoffScenario,
    violations: &mut Vec<ProjectDoctorContainerHandoffViolation>,
) {
    let sid = scenario.scenario_id.as_str();
    if scenario.initiating_findings.is_empty() {
        push(
            violations,
            "container_handoff.findings_missing",
            sid,
            "scenario must declare at least one initiating finding",
        );
    }
    let surface_prefix = scenario.surface.finding_code_prefix();
    for finding in &scenario.initiating_findings {
        if !finding.starts_with(DOCTOR_FINDING_PREFIX) {
            push(
                violations,
                "container_handoff.finding_prefix",
                sid,
                format!("initiating finding {finding} must start with {DOCTOR_FINDING_PREFIX}"),
            );
        } else if !finding.starts_with(&surface_prefix) {
            push(
                violations,
                "container_handoff.finding_surface_mismatch",
                sid,
                format!(
                    "initiating finding {finding} must be surface-scoped under {surface_prefix}"
                ),
            );
        }
    }
}

fn validate_route(
    scenario: &ContainerHandoffScenario,
    violations: &mut Vec<ProjectDoctorContainerHandoffViolation>,
) {
    let sid = scenario.scenario_id.as_str();
    let route = &scenario.route;

    if route.route_id.trim().is_empty() {
        push(
            violations,
            "container_handoff.route_id_missing",
            sid,
            "route must carry a non-empty route_id",
        );
    }
    if route.target_ref.trim().is_empty() || route.target_service_ref.trim().is_empty() {
        push(
            violations,
            "container_handoff.route_target_missing",
            sid,
            "route must name a non-empty target_ref and target_service_ref",
        );
    }
    if route.container_port == 0 || route.host_port == 0 {
        push(
            violations,
            "container_handoff.route_port_invalid",
            sid,
            "route must use non-zero container and host ports",
        );
    }

    // Guardrail: a route is never a durable silent share — it is always
    // time-bound and exposes a revocation path.
    if !route.time_bound.is_time_bound() {
        push(
            violations,
            "container_handoff.route_not_time_bound",
            sid,
            "route must carry a non-empty expires_at_ref (no unbounded share)",
        );
    }
    if !route.revocation.is_revocable() {
        push(
            violations,
            "container_handoff.route_not_revocable",
            sid,
            "route must expose a non-empty revocation_action_ref (revocation stays first-class)",
        );
    }
    // A dead route must carry revocation/expiry evidence.
    if route.revocation.state.is_dead() && route.revocation.revoked_evidence_ref.trim().is_empty() {
        push(
            violations,
            "container_handoff.revocation_evidence_missing",
            sid,
            "a revoked or expired route must carry a non-empty revoked_evidence_ref",
        );
    }
}

fn validate_environment_disclosure(
    scenario: &ContainerHandoffScenario,
    violations: &mut Vec<ProjectDoctorContainerHandoffViolation>,
) {
    let sid = scenario.scenario_id.as_str();
    let disclosure = &scenario.environment_disclosure;

    // Guardrail: writable-mount and lifecycle/install-script disclosure survives
    // into every reopen/attach/rebuild/export/support flow.
    if !disclosure.survives_required_flows() {
        push(
            violations,
            "container_handoff.disclosure_flow_missing",
            sid,
            "environment disclosure must survive into reopen, attach, rebuild, export, and support_bundle flows",
        );
    }
    if disclosure.disclosure_note.trim().is_empty() {
        push(
            violations,
            "container_handoff.disclosure_note_missing",
            sid,
            "environment disclosure must carry a non-empty disclosure note",
        );
    }
    for mount in &disclosure.writable_mounts {
        if mount.scope_ref.trim().is_empty() {
            push(
                violations,
                "container_handoff.mount_scope_missing",
                sid,
                "every disclosed mount must carry a non-empty scope_ref",
            );
        }
    }
    for script in &disclosure.lifecycle_scripts {
        if script.command_ref.trim().is_empty() {
            push(
                violations,
                "container_handoff.script_command_missing",
                sid,
                "every disclosed script must carry a non-empty command_ref",
            );
        }
    }
}

fn validate_handoff(
    scenario: &ContainerHandoffScenario,
    violations: &mut Vec<ProjectDoctorContainerHandoffViolation>,
) {
    let sid = scenario.scenario_id.as_str();
    let handoff = &scenario.handoff;
    let route = &scenario.route;

    // Guardrail: a handoff is never a durable opaque share.
    if !handoff.preserves_attribution() {
        push(
            violations,
            "container_handoff.handoff_attribution_incomplete",
            sid,
            "handoff must preserve owner, origin, target, route, and service identity (not a flattened URL)",
        );
    }
    if handoff.route_id != route.route_id
        || handoff.target_ref != route.target_ref
        || handoff.target_service_ref != route.target_service_ref
        || handoff.engine_class != scenario.engine_class
    {
        push(
            violations,
            "container_handoff.handoff_route_mismatch",
            sid,
            "handoff route/target/service/engine identity must match the route and scenario",
        );
    }

    // Guardrail: a dead route never backs a live opaque share.
    if route.revocation.state.is_dead() {
        if handoff.liveness != HandoffLiveness::Snapshot {
            push(
                violations,
                "container_handoff.dead_route_live_share",
                sid,
                "a revoked or expired route must back a snapshot handoff, never a live channel",
            );
        }
        if !handoff.revocation_visible {
            push(
                violations,
                "container_handoff.dead_route_revocation_hidden",
                sid,
                "a revoked or expired route must surface its revocation state in the handoff",
            );
        }
    }
    // A snapshot handoff must carry a capture-time ref.
    if handoff.liveness == HandoffLiveness::Snapshot && handoff.captured_at_ref.trim().is_empty() {
        push(
            violations,
            "container_handoff.snapshot_capture_missing",
            sid,
            "a snapshot handoff must carry a non-empty captured_at_ref",
        );
    }

    // Out of scope guardrail: no unrestricted mutate channel; a bounded write
    // channel must be approval-gated.
    if !handoff.write_channel_is_gated() {
        push(
            violations,
            "container_handoff.bounded_write_without_approval",
            sid,
            "a bounded write handoff must be approval-gated (no unrestricted mutate channel)",
        );
    }
}

fn validate_linkage(
    scenario: &ContainerHandoffScenario,
    violations: &mut Vec<ProjectDoctorContainerHandoffViolation>,
) {
    let sid = scenario.scenario_id.as_str();
    let linkage = &scenario.support_linkage;
    if !linkage.preserves_identity() {
        push(
            violations,
            "container_handoff.identity_not_preserved",
            sid,
            "support linkage must preserve manifest, escalation packet, findings, and scope",
        );
    }
    if !linkage.is_metadata_safe() {
        push(
            violations,
            "container_handoff.linkage_not_metadata_safe",
            sid,
            "support linkage must be metadata-safe (redaction_class metadata_safe_default, raw + overcapture excluded)",
        );
    }
    for finding in &linkage.preserved_finding_ids {
        if !finding.starts_with(DOCTOR_FINDING_PREFIX) {
            push(
                violations,
                "container_handoff.preserved_finding_prefix",
                sid,
                format!("preserved finding {finding} must start with {DOCTOR_FINDING_PREFIX}"),
            );
        }
    }
    for repair in &linkage.preserved_repair_ids {
        if !repair.starts_with(DOCTOR_REPAIR_PREFIX) {
            push(
                violations,
                "container_handoff.preserved_repair_prefix",
                sid,
                format!("preserved repair id {repair} must start with {DOCTOR_REPAIR_PREFIX}"),
            );
        }
    }
}

fn validate_parity_and_meaning(
    scenario: &ContainerHandoffScenario,
    violations: &mut Vec<ProjectDoctorContainerHandoffViolation>,
) {
    let sid = scenario.scenario_id.as_str();
    if !scenario.is_cross_surface_stable() {
        push(
            violations,
            "container_handoff.parity_surface_missing",
            sid,
            "scenario must render on every parity surface",
        );
    }
    for required in REQUIRED_MACHINE_MEANING_KEYS {
        if !scenario.machine_meaning_keys.iter().any(|k| k == required) {
            push(
                violations,
                "container_handoff.machine_meaning_key_missing",
                sid,
                format!("scenario must carry machine-meaning key {required}"),
            );
        }
    }
    let explanation = scenario.explanation.trim().to_ascii_lowercase();
    if explanation.is_empty() {
        push(
            violations,
            "container_handoff.explanation_empty",
            sid,
            "explanation must be non-empty",
        );
    } else if GENERIC_EXPLANATION_TOKENS.contains(&explanation.as_str()) {
        push(
            violations,
            "container_handoff.explanation_generic",
            sid,
            "explanation must be specific, not a generic token",
        );
    }
}

fn validate_gate(
    scenario: &ContainerHandoffScenario,
    violations: &mut Vec<ProjectDoctorContainerHandoffViolation>,
) {
    let sid = scenario.scenario_id.as_str();
    let (posture, reason) = scenario.recompute_gate();
    if scenario.published_handoff_posture != posture {
        push(
            violations,
            "container_handoff.gate_posture_mismatch",
            sid,
            format!(
                "published_handoff_posture {} does not match recomputed gate posture {}",
                scenario.published_handoff_posture.as_str(),
                posture.as_str()
            ),
        );
    }
    if scenario.published_handoff_reason != reason {
        push(
            violations,
            "container_handoff.gate_reason_mismatch",
            sid,
            format!(
                "published_handoff_reason {} does not match recomputed reason {}",
                scenario.published_handoff_reason.as_str(),
                reason.as_str()
            ),
        );
    }

    // Guardrail: a share_live posture must be a live, read-only, in-audience,
    // policy-clean share of an active route with no disclosed mutation. (The gate
    // equality above already enforces this; this explicit check keeps the
    // guardrail legible and independently tested.)
    if scenario.published_handoff_posture == HandoffPosture::ShareLive
        && (scenario.route.policy_posture != PolicyPosture::PolicyAllowed
            || scenario.route.revocation.state != RevocationState::Active
            || scenario.route.audience_scope == AudienceScope::Public
            || scenario.handoff.mutation_scope != HandoffMutationScope::ReadOnly
            || scenario.handoff.liveness != HandoffLiveness::Live
            || scenario.environment_disclosure.requires_disclosure())
    {
        push(
            violations,
            "container_handoff.share_live_not_clean",
            sid,
            "share_live requires an active, policy-allowed, non-public, read-only, live share with no disclosed environment mutation",
        );
    }
}

/// One validation violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorContainerHandoffViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref (packet id or scenario id).
    pub subject_ref: String,
    /// Human-readable message.
    pub message: String,
}

/// One row of the metadata-safe support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorContainerHandoffExportRow {
    /// Scenario id.
    pub scenario_id: String,
    /// Workflow surface.
    pub surface: HandoffSurface,
    /// Engine class.
    pub engine_class: EngineClass,
    /// Route kind.
    pub route_kind: RouteKind,
    /// Route id.
    pub route_id: String,
    /// Target service ref.
    pub target_service_ref: String,
    /// Audience scope.
    pub audience_scope: AudienceScope,
    /// Policy posture.
    pub policy_posture: PolicyPosture,
    /// Revocation state.
    pub revocation_state: RevocationState,
    /// Revocation action ref.
    pub revocation_action_ref: String,
    /// Expiry ref.
    pub expires_at_ref: String,
    /// Handoff channel.
    pub handoff_channel: HandoffChannel,
    /// Handoff liveness.
    pub handoff_liveness: HandoffLiveness,
    /// Handoff posture.
    pub handoff_posture: HandoffPosture,
    /// Handoff reason.
    pub handoff_reason: HandoffReason,
    /// Support-bundle manifest ref.
    pub bundle_manifest_ref: String,
    /// Escalation packet ref.
    pub escalation_packet_ref: String,
}

/// The metadata-safe support-export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorContainerHandoffExportProjection {
    /// Packet id.
    pub packet_id: String,
    /// Schema ref.
    pub schema_ref: String,
    /// One row per scenario.
    pub rows: Vec<ProjectDoctorContainerHandoffExportRow>,
    /// Count of full-strength live-share scenarios.
    pub share_live_count: usize,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

fn push(
    violations: &mut Vec<ProjectDoctorContainerHandoffViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(ProjectDoctorContainerHandoffViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

/// Parses the embedded checked-in packet.
///
/// Returns a JSON parse error when the checked-in packet no longer matches the
/// typed model.
pub fn current_project_doctor_container_handoff_truth(
) -> Result<ProjectDoctorContainerHandoffTruth, serde_json::Error> {
    serde_json::from_str(PROJECT_DOCTOR_CONTAINER_HANDOFF_JSON)
}

#[cfg(test)]
mod tests;
