//! Preview-origin descriptor.
//!
//! A `PreviewOriginDescriptor` is the cross-surface answer to "which runtime
//! or renderer produced this view." It is intentionally distinct from the
//! preview-target descriptor (the *what*) and from the browser-runtime
//! session-origin descriptor (the *which-browser-session*); it names the
//! *where it came from* — the local dev server, the remote / container
//! runtime, the managed preview service, the embedded renderer, or the
//! imported / static evidence origin.

use serde::{Deserialize, Serialize};

use super::{PreviewLaneClass, PreviewOriginFinding};

/// Stable record-kind tag.
pub const PREVIEW_ORIGIN_DESCRIPTOR_RECORD_KIND: &str = "preview_origin_descriptor_record";

/// Schema version mirrored by
/// `/schemas/preview/preview_target_descriptor.schema.json#preview_origin_descriptor_record`.
pub const PREVIEW_ORIGIN_DESCRIPTOR_SCHEMA_VERSION: u32 = 1;

/// Closed preview-origin vocabulary. Names the runtime / renderer that
/// produced the view a beta preview row is currently displaying.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewOriginClass {
    /// Local dev server bound to a workspace-local process (e.g. a
    /// framework dev server, a `cargo run`-style local binary, or a local
    /// notebook kernel renderer running on the same machine).
    LocalDevServer,
    /// Remote or container runtime — a remote workspace, a devcontainer,
    /// or any preview process not running on the local machine but still
    /// reachable via a workspace-bound transport.
    RemoteOrContainerRuntime,
    /// Managed preview service — a governed preview runtime managed by an
    /// approval ticket and a managed-workspace lifecycle (not a generic
    /// "remote runtime"). Always disclosed because mutation actions cross
    /// a governed boundary.
    ManagedPreviewService,
    /// Embedded renderer — a renderer hosted *inside* Aureline's process
    /// or as an extension-host webview, with no separate runtime. Examples
    /// include the Markdown renderer, the design-token overlay renderer,
    /// the docs renderer, and extension-provided embedded previews.
    EmbeddedRenderer,
    /// Imported or static evidence — a captured / pinned / imported
    /// preview surface that has no live runtime at all. The "origin" is
    /// the evidence record itself; mutation is never admissible.
    ImportedOrStaticEvidence,
}

impl PreviewOriginClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDevServer => "local_dev_server",
            Self::RemoteOrContainerRuntime => "remote_or_container_runtime",
            Self::ManagedPreviewService => "managed_preview_service",
            Self::EmbeddedRenderer => "embedded_renderer",
            Self::ImportedOrStaticEvidence => "imported_or_static_evidence",
        }
    }

    /// True when this origin represents a runtime that can in principle
    /// accept hot reload, fast refresh, or reconnect events. Static
    /// imported evidence and embedded renderers without a kernel cannot.
    pub const fn admits_live_runtime_events(self) -> bool {
        matches!(
            self,
            Self::LocalDevServer | Self::RemoteOrContainerRuntime | Self::ManagedPreviewService
        )
    }

    /// True when actions against this origin can mutate remote / governed
    /// state. Mutation review surfaces use this to decide whether a
    /// "local-only safety" claim is admissible.
    pub const fn touches_remote_or_governed(self) -> bool {
        matches!(
            self,
            Self::RemoteOrContainerRuntime | Self::ManagedPreviewService
        )
    }
}

/// Closed lifecycle-phase vocabulary the descriptor uses to describe what
/// the origin is doing right now. Distinct from the hot-reload state — a
/// runtime can be `running` yet have a `failed` reload, or `cold` yet have
/// a stale `applied` event in its history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewOriginLifecyclePhase {
    NotStarted,
    Warming,
    Running,
    Reconnecting,
    Restarting,
    Suspended,
    Stopped,
    Unreachable,
    NotApplicableStaticEvidence,
}

impl PreviewOriginLifecyclePhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Warming => "warming",
            Self::Running => "running",
            Self::Reconnecting => "reconnecting",
            Self::Restarting => "restarting",
            Self::Suspended => "suspended",
            Self::Stopped => "stopped",
            Self::Unreachable => "unreachable",
            Self::NotApplicableStaticEvidence => "not_applicable_static_evidence",
        }
    }

    /// True when the lifecycle is in a "live runtime" posture (the origin
    /// is currently producing fresh frames or is reachable for inspection).
    pub const fn is_live_runtime_posture(self) -> bool {
        matches!(self, Self::Running | Self::Reconnecting)
    }
}

/// Closed sharing / exposure vocabulary. Mirrors the route-governance
/// exposure classes from `M03-202` so the preview-origin descriptor can be
/// honest about whether the runtime is observable beyond the local
/// workspace without minting a new exposure vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewOriginSharingPosture {
    LocalOnly,
    SameDeviceOrLan,
    AuthenticatedOrgRoute,
    SignedPreviewLink,
    PublicRoute,
    NotApplicableNoNetworkSurface,
}

impl PreviewOriginSharingPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::SameDeviceOrLan => "same_device_or_lan",
            Self::AuthenticatedOrgRoute => "authenticated_org_route",
            Self::SignedPreviewLink => "signed_preview_link",
            Self::PublicRoute => "public_route",
            Self::NotApplicableNoNetworkSurface => "not_applicable_no_network_surface",
        }
    }

    /// True when the route is observable beyond the local workspace.
    pub const fn implies_remote_audience(self) -> bool {
        matches!(
            self,
            Self::AuthenticatedOrgRoute | Self::SignedPreviewLink | Self::PublicRoute
        )
    }
}

/// Canonical preview-origin descriptor. Every beta preview row that
/// participates in the hot-reload, source-jump, or mutation-review flows
/// references one descriptor; surfaces quote the descriptor verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewOriginDescriptor {
    pub record_kind: String,
    pub preview_origin_descriptor_schema_version: u32,
    pub preview_origin_descriptor_id: String,
    /// ISO 8601 UTC monotonic capture timestamp.
    pub observed_at: String,

    pub preview_lane_class: PreviewLaneClass,
    pub origin_class: PreviewOriginClass,
    pub lifecycle_phase: PreviewOriginLifecyclePhase,
    pub sharing_posture: PreviewOriginSharingPosture,

    /// Opaque runtime handle (process / kernel / container / managed
    /// reservation ref). Raw URLs / hostnames / serial numbers never appear
    /// here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_handle_ref: Option<String>,
    /// Opaque ref to the route / exposure record (`M03-202`) when the
    /// runtime is reachable beyond local-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exposure_record_ref: Option<String>,
    /// Opaque ref to the underlying preview_snapshot_record. Optional —
    /// imported / static evidence origins may not have a live snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_snapshot_record_ref: Option<String>,
    /// Opaque ref to the managed-workspace approval ticket — required when
    /// `origin_class = managed_preview_service`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_workspace_approval_ref: Option<String>,
    /// Reviewer-facing redacted label for the runtime. Never contains raw
    /// URLs, raw IPs, raw account handles, raw container IDs.
    pub redacted_runtime_label: String,
    /// One-sentence summary the chrome renders below the origin badge.
    pub summary: String,
}

impl PreviewOriginDescriptor {
    /// Run the cross-record honesty rules over the descriptor. Empty vec
    /// means clean; populated vec is what the chrome / audit must publish.
    pub fn validate(&self) -> Vec<PreviewOriginFinding> {
        let mut findings = Vec::new();
        let subject = self.preview_origin_descriptor_id.as_str();

        if self.record_kind != PREVIEW_ORIGIN_DESCRIPTOR_RECORD_KIND {
            findings.push(PreviewOriginFinding::new(
                "preview_origin_descriptor.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    PREVIEW_ORIGIN_DESCRIPTOR_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.preview_origin_descriptor_schema_version != PREVIEW_ORIGIN_DESCRIPTOR_SCHEMA_VERSION
        {
            findings.push(PreviewOriginFinding::new(
                "preview_origin_descriptor.schema_version",
                subject,
                format!(
                    "schema_version must be {}, found {}",
                    PREVIEW_ORIGIN_DESCRIPTOR_SCHEMA_VERSION,
                    self.preview_origin_descriptor_schema_version
                ),
            ));
        }

        if matches!(self.origin_class, PreviewOriginClass::ManagedPreviewService)
            && self.managed_workspace_approval_ref.is_none()
        {
            findings.push(PreviewOriginFinding::new(
                "preview_origin_descriptor.managed_requires_approval",
                subject,
                "managed_preview_service requires a non-null managed_workspace_approval_ref",
            ));
        }

        if matches!(
            self.origin_class,
            PreviewOriginClass::ImportedOrStaticEvidence
        ) && self.lifecycle_phase != PreviewOriginLifecyclePhase::NotApplicableStaticEvidence
        {
            findings.push(PreviewOriginFinding::new(
                "preview_origin_descriptor.static_evidence_lifecycle",
                subject,
                "imported_or_static_evidence requires lifecycle_phase = not_applicable_static_evidence",
            ));
        }
        if !matches!(
            self.origin_class,
            PreviewOriginClass::ImportedOrStaticEvidence
        ) && self.lifecycle_phase == PreviewOriginLifecyclePhase::NotApplicableStaticEvidence
        {
            findings.push(PreviewOriginFinding::new(
                "preview_origin_descriptor.lifecycle_static_only",
                subject,
                "lifecycle_phase = not_applicable_static_evidence is reserved for imported_or_static_evidence origins",
            ));
        }

        if !self.origin_class.admits_live_runtime_events()
            && self.lifecycle_phase.is_live_runtime_posture()
        {
            findings.push(PreviewOriginFinding::new(
                "preview_origin_descriptor.live_phase_without_runtime",
                subject,
                "non-runtime origin must not declare a running / reconnecting lifecycle phase",
            ));
        }

        if self.sharing_posture.implies_remote_audience() && self.exposure_record_ref.is_none() {
            findings.push(PreviewOriginFinding::new(
                "preview_origin_descriptor.remote_audience_requires_exposure_record",
                subject,
                "remote-audience sharing posture requires a non-null exposure_record_ref",
            ));
        }
        if matches!(self.sharing_posture, PreviewOriginSharingPosture::LocalOnly)
            && self.origin_class.touches_remote_or_governed()
        {
            findings.push(PreviewOriginFinding::new(
                "preview_origin_descriptor.local_only_safety_overclaim",
                subject,
                "remote / managed origin must not advertise local_only sharing posture",
            ));
        }

        findings
    }

    /// True when a `live-preview`-style claim is honest on this descriptor
    /// right now (runtime origin AND a live lifecycle).
    pub fn implies_live_runtime(&self) -> bool {
        self.origin_class.admits_live_runtime_events()
            && self.lifecycle_phase.is_live_runtime_posture()
    }

    /// Render a deterministic plaintext summary safe to embed in support
    /// exports.
    pub fn render_plaintext(&self) -> String {
        format!(
            "preview_origin {id} lane={lane} origin={origin} phase={phase} sharing={sharing}: {summary}",
            id = self.preview_origin_descriptor_id,
            lane = self.preview_lane_class.as_str(),
            origin = self.origin_class.as_str(),
            phase = self.lifecycle_phase.as_str(),
            sharing = self.sharing_posture.as_str(),
            summary = self.summary,
        )
    }
}
