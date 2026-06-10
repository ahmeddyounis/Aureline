//! Remote preview route lifecycle, expiry, target identity, and preview/runtime trust disclosure.
//!
//! This module implements the canonical M5 truth packet for the in-product
//! remote preview surface: the lane that publishes a remote preview route for a
//! reviewed change without ever surfacing a route that is unattributable, that
//! never expires, that hides which target it points at, or that conceals the
//! trust posture of the runtime serving it. It binds four pillars into one
//! export-safe record:
//!
//! - **Route lifecycle** — each [`RemotePreviewRouteRow`] carries the route's
//!   durable review anchor, its lifecycle phase, and a list of bound
//!   [`RouteLifecycleEventRow`] transitions, so the route's progression from
//!   provisioning to live to expired/revoked is explicit and reviewable rather
//!   than an opaque toggle.
//! - **Expiry** — each route carries a [`RouteExpiryDisclosure`] with a typed
//!   expiry state, a positive TTL, an honest expiry label, and an auto-revoke
//!   flag, so every live route stays time-bounded; an unbounded route is blocked
//!   and flagged rather than served.
//! - **Target identity** — each route names what it points at (the target
//!   identity label, the run it was built from, and the commit/branch identity),
//!   so a remote preview can never hide which change a reviewer is looking at.
//! - **Preview / runtime trust disclosure** — each route carries a
//!   [`PreviewRuntimeTrustDisclosure`] with the runtime trust class, the network
//!   egress class, and the host identity, so the preview can never present
//!   untrusted remote content or unrestricted egress without disclosing it.
//!
//! The packet references upstream preview-route, browser-runtime-session,
//! pipeline-run-row, and trust-class contracts by id rather than embedding their
//! content. Raw preview URLs, raw host names, raw run / log bodies, raw provider
//! payloads, raw absolute paths, raw author email addresses, credentials, and
//! live provider responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/review/add-remote-preview-route-lifecycle-expiry-target-identity-and-preview-runtime-trust-disclosure.schema.json`](../../../../schemas/review/add-remote-preview-route-lifecycle-expiry-target-identity-and-preview-runtime-trust-disclosure.schema.json).
//! The contract doc is
//! [`docs/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure.md`](../../../../docs/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/`](../../../../fixtures/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`RemotePreviewRoutePacket`].
pub const REMOTE_PREVIEW_ROUTE_RECORD_KIND: &str =
    "add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure";

/// Schema version for remote preview route records.
pub const REMOTE_PREVIEW_ROUTE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const REMOTE_PREVIEW_ROUTE_SCHEMA_REF: &str =
    "schemas/review/add-remote-preview-route-lifecycle-expiry-target-identity-and-preview-runtime-trust-disclosure.schema.json";

/// Repo-relative path of the remote preview route contract doc.
pub const REMOTE_PREVIEW_ROUTE_DOC_REF: &str =
    "docs/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure.md";

/// Repo-relative path of the frozen preview-route contract this lane builds on.
pub const REMOTE_PREVIEW_ROUTE_PREVIEW_ROUTE_CONTRACT_REF: &str =
    "schemas/runtime/preview_route.schema.json";

/// Repo-relative path of the browser-runtime-session contract reused for trust disclosure.
pub const REMOTE_PREVIEW_ROUTE_BROWSER_RUNTIME_CONTRACT_REF: &str =
    "schemas/runtime/browser_runtime_session.schema.json";

/// Repo-relative path of the pipeline-run-row contract the target identity binds to.
pub const REMOTE_PREVIEW_ROUTE_PIPELINE_RUN_CONTRACT_REF: &str =
    "schemas/ci/pipeline_run_row.schema.json";

/// Repo-relative path of the trust-class vocabulary this lane reuses.
pub const REMOTE_PREVIEW_ROUTE_TRUST_CLASS_CONTRACT_REF: &str =
    "schemas/security/trust_class.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const REMOTE_PREVIEW_ROUTE_FIXTURE_DIR: &str =
    "fixtures/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure";

/// Repo-relative path of the checked support-export artifact.
pub const REMOTE_PREVIEW_ROUTE_ARTIFACT_REF: &str =
    "artifacts/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const REMOTE_PREVIEW_ROUTE_SUMMARY_REF: &str =
    "artifacts/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure.md";

/// Lifecycle phase a remote preview route is presently in.
///
/// `unknown_phase_provider_owned` must never be flattened into a known phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteLifecyclePhase {
    /// The route has been requested but not yet provisioned.
    Requested,
    /// The route is being provisioned.
    Provisioning,
    /// The route is live and serving the preview.
    Live,
    /// The route is live but within its expiry warning window.
    ExpiringSoon,
    /// The route has expired and no longer serves the preview.
    Expired,
    /// The route was revoked before its expiry.
    Revoked,
    /// The route failed to provision or serve.
    Failed,
    /// Provider returned a phase the contract does not recognise yet.
    UnknownPhaseProviderOwned,
}

impl RouteLifecyclePhase {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Provisioning => "provisioning",
            Self::Live => "live",
            Self::ExpiringSoon => "expiring_soon",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::Failed => "failed",
            Self::UnknownPhaseProviderOwned => "unknown_phase_provider_owned",
        }
    }

    /// Whether this phase needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::Expired | Self::Revoked | Self::Failed | Self::UnknownPhaseProviderOwned
        )
    }
}

/// Typed expiry state of a remote preview route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteExpiryState {
    /// The route has a future expiry and is within its active window.
    ActiveTimeBounded,
    /// The route is within its expiry warning window.
    ExpiringSoon,
    /// The route has passed its expiry.
    Expired,
    /// The route was revoked before its expiry.
    RevokedBeforeExpiry,
    /// The route has no expiry set; this is not allowed and must be blocked.
    NoExpiryUnbounded,
}

impl RouteExpiryState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveTimeBounded => "active_time_bounded",
            Self::ExpiringSoon => "expiring_soon",
            Self::Expired => "expired",
            Self::RevokedBeforeExpiry => "revoked_before_expiry",
            Self::NoExpiryUnbounded => "no_expiry_unbounded",
        }
    }

    /// Whether the route carries an expiry bound (every state except `no_expiry_unbounded`).
    pub const fn is_time_bounded(self) -> bool {
        !matches!(self, Self::NoExpiryUnbounded)
    }
}

/// Host / provider identity class serving a remote preview route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteHostClass {
    /// An Aureline-managed preview host.
    AurelineManagedHost,
    /// A provider-hosted preview environment.
    ProviderHosted,
    /// A self-hosted tunnel back to the developer's machine.
    SelfHostedTunnel,
    /// A local loopback preview not exposed beyond the device.
    LocalLoopback,
    /// Provider returned a host class the contract does not recognise yet.
    UnknownHostProviderOwned,
}

impl RouteHostClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AurelineManagedHost => "aureline_managed_host",
            Self::ProviderHosted => "provider_hosted",
            Self::SelfHostedTunnel => "self_hosted_tunnel",
            Self::LocalLoopback => "local_loopback",
            Self::UnknownHostProviderOwned => "unknown_host_provider_owned",
        }
    }

    /// Whether this host class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(self, Self::UnknownHostProviderOwned)
    }
}

/// Runtime trust class of the preview the route serves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewRuntimeTrustClass {
    /// Fully sandboxed and network-isolated runtime.
    SandboxedIsolated,
    /// Sandboxed runtime with limited, named network access.
    SandboxedNetworkLimited,
    /// Runtime trusted at the workspace's trust level.
    RuntimeTrustedWorkspace,
    /// Runtime serving untrusted remote content.
    UntrustedRemoteContent,
    /// Provider returned a trust class the contract does not recognise yet.
    UnknownTrustProviderOwned,
}

impl PreviewRuntimeTrustClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SandboxedIsolated => "sandboxed_isolated",
            Self::SandboxedNetworkLimited => "sandboxed_network_limited",
            Self::RuntimeTrustedWorkspace => "runtime_trusted_workspace",
            Self::UntrustedRemoteContent => "untrusted_remote_content",
            Self::UnknownTrustProviderOwned => "unknown_trust_provider_owned",
        }
    }

    /// Whether this trust class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::UntrustedRemoteContent | Self::UnknownTrustProviderOwned
        )
    }
}

/// Network egress posture disclosed for a remote preview runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkEgressClass {
    /// The runtime makes no outbound network calls.
    NoEgress,
    /// The runtime may reach only a set of named targets.
    EgressToNamedTargets,
    /// The runtime may reach any outbound target.
    UnrestrictedEgress,
    /// Provider returned an egress posture the contract does not recognise yet.
    UnknownEgressProviderOwned,
}

impl NetworkEgressClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoEgress => "no_egress",
            Self::EgressToNamedTargets => "egress_to_named_targets",
            Self::UnrestrictedEgress => "unrestricted_egress",
            Self::UnknownEgressProviderOwned => "unknown_egress_provider_owned",
        }
    }

    /// Whether this egress posture needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::UnrestrictedEgress | Self::UnknownEgressProviderOwned
        )
    }
}

/// Provider-mode mutation mode a route was published under.
///
/// Publishing a remote preview route always reaches upstream host state, so the
/// local-only `local_draft` mode is intentionally absent. Each mode cites the
/// grant it depends on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteMutationMode {
    /// Publishes the route now; must cite an approval ticket.
    PublishNow,
    /// Hands off to the provider in the browser; must cite a browser-handoff packet.
    OpenInProvider,
    /// Queues the route for a later drain; must cite a publish-later queue item.
    DeferredPublish,
}

impl RouteMutationMode {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublishNow => "publish_now",
            Self::OpenInProvider => "open_in_provider",
            Self::DeferredPublish => "deferred_publish",
        }
    }

    /// Whether this mode must cite an approval ticket ref.
    pub const fn requires_approval_ref(self) -> bool {
        matches!(self, Self::PublishNow)
    }

    /// Whether this mode must cite a browser-handoff packet ref.
    pub const fn requires_browser_handoff_ref(self) -> bool {
        matches!(self, Self::OpenInProvider)
    }

    /// Whether this mode must cite a publish-later queue item ref.
    pub const fn requires_deferred_queue_ref(self) -> bool {
        matches!(self, Self::DeferredPublish)
    }
}

/// Why a remote preview route is blocked, if it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteBlockedClass {
    /// The route is admissible.
    NotBlocked,
    /// No auth grant to publish the route.
    BlockedNoAuthForRoute,
    /// Policy forbids remote preview for this workspace.
    BlockedPolicyForbidsRemotePreview,
    /// The route has expired.
    BlockedRouteExpired,
    /// The route was revoked.
    BlockedRouteRevoked,
    /// The route has no expiry bound and may not be served.
    BlockedNoExpiryNotTimeBounded,
    /// The runtime trust is untrusted or unknown and a review is required first.
    BlockedUntrustedRuntimeReviewRequired,
    /// The host identity is undisclosed and a review is required first.
    BlockedHostIdentityUndisclosed,
    /// The surface is offline or disconnected.
    BlockedOfflineOrDisconnected,
    /// Provider returned a block reason the contract does not recognise yet.
    BlockedUnknownReasonProviderOwned,
}

impl RouteBlockedClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotBlocked => "not_blocked",
            Self::BlockedNoAuthForRoute => "blocked_no_auth_for_route",
            Self::BlockedPolicyForbidsRemotePreview => "blocked_policy_forbids_remote_preview",
            Self::BlockedRouteExpired => "blocked_route_expired",
            Self::BlockedRouteRevoked => "blocked_route_revoked",
            Self::BlockedNoExpiryNotTimeBounded => "blocked_no_expiry_not_time_bounded",
            Self::BlockedUntrustedRuntimeReviewRequired => {
                "blocked_untrusted_runtime_review_required"
            }
            Self::BlockedHostIdentityUndisclosed => "blocked_host_identity_undisclosed",
            Self::BlockedOfflineOrDisconnected => "blocked_offline_or_disconnected",
            Self::BlockedUnknownReasonProviderOwned => "blocked_unknown_reason_provider_owned",
        }
    }

    /// Whether the route is blocked.
    pub const fn is_blocked(self) -> bool {
        !matches!(self, Self::NotBlocked)
    }

    /// Whether this block class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        self.is_blocked()
    }
}

/// Lifecycle transition event kind bound to a route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteLifecycleEventKind {
    /// The route was provisioned.
    Provisioned,
    /// The route went live.
    WentLive,
    /// The route's expiry was extended.
    ExtendedExpiry,
    /// The route entered its expiry warning window.
    ExpiryWarning,
    /// The route expired.
    Expired,
    /// The route was revoked.
    Revoked,
    /// The route failed.
    Failed,
    /// Provider returned an event the contract does not recognise yet.
    UnknownEventProviderOwned,
}

impl RouteLifecycleEventKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provisioned => "provisioned",
            Self::WentLive => "went_live",
            Self::ExtendedExpiry => "extended_expiry",
            Self::ExpiryWarning => "expiry_warning",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::Failed => "failed",
            Self::UnknownEventProviderOwned => "unknown_event_provider_owned",
        }
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemotePreviewRouteDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A route was surfaced without attribution.
    RouteAttributionMissing,
    /// A route was surfaced without an expiry bound.
    RouteExpiryUnbounded,
    /// A route is expired or revoked.
    RouteExpiredOrRevoked,
    /// The runtime trust was not disclosed.
    RuntimeTrustUndisclosed,
    /// The host identity was not disclosed.
    HostIdentityUndisclosed,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl RemotePreviewRouteDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::RouteAttributionMissing,
        Self::RouteExpiryUnbounded,
        Self::RouteExpiredOrRevoked,
        Self::RuntimeTrustUndisclosed,
        Self::HostIdentityUndisclosed,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::RouteAttributionMissing => "route_attribution_missing",
            Self::RouteExpiryUnbounded => "route_expiry_unbounded",
            Self::RouteExpiredOrRevoked => "route_expired_or_revoked",
            Self::RuntimeTrustUndisclosed => "runtime_trust_undisclosed",
            Self::HostIdentityUndisclosed => "host_identity_undisclosed",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project this lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemotePreviewRouteConsumerSurface {
    /// Preview panel.
    PreviewPanel,
    /// Remote preview route card.
    RemotePreviewRouteCard,
    /// Route lifecycle sheet.
    RouteLifecycleSheet,
    /// Review workspace header.
    ReviewWorkspaceHeader,
    /// Command palette.
    CommandPalette,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl RemotePreviewRouteConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::PreviewPanel,
        Self::RemotePreviewRouteCard,
        Self::RouteLifecycleSheet,
        Self::ReviewWorkspaceHeader,
        Self::CommandPalette,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewPanel => "preview_panel",
            Self::RemotePreviewRouteCard => "remote_preview_route_card",
            Self::RouteLifecycleSheet => "route_lifecycle_sheet",
            Self::ReviewWorkspaceHeader => "review_workspace_header",
            Self::CommandPalette => "command_palette",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Expiry disclosure for a remote preview route.
///
/// A time-bounded route must carry a positive TTL, an honest (redaction-aware)
/// expiry label, and auto-revoke on expiry, so a live route can never outlive its
/// bound.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteExpiryDisclosure {
    /// Typed expiry state.
    pub expiry_state: RouteExpiryState,
    /// Time-to-live in seconds; positive for a time-bounded route.
    pub ttl_seconds: u64,
    /// Redaction-aware expiry label (e.g. "expires in 2 hours"); required when time-bounded.
    pub expires_at_label: String,
    /// Whether the route auto-revokes on expiry; required true when time-bounded.
    pub auto_revoke_on_expiry: bool,
}

impl RouteExpiryDisclosure {
    /// Whether the expiry disclosure is complete for a time-bounded route.
    pub fn is_time_bounded_complete(&self) -> bool {
        self.ttl_seconds > 0
            && !self.expires_at_label.trim().is_empty()
            && self.auto_revoke_on_expiry
    }
}

/// Host / provider identity disclosure for a remote preview route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteHostIdentity {
    /// Typed host class.
    pub host_class: RouteHostClass,
    /// Redaction-aware host label (no raw URL or host name).
    pub host_label: String,
    /// Whether the host origin is disclosed; required true.
    pub origin_disclosed: bool,
}

/// Preview / runtime trust disclosure for a remote preview route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewRuntimeTrustDisclosure {
    /// Runtime trust class.
    pub trust_class: PreviewRuntimeTrustClass,
    /// Network egress class.
    pub network_egress: NetworkEgressClass,
    /// Whether the runtime executes untrusted code.
    pub executes_untrusted_code: bool,
    /// Whether the runtime's write scope is disclosed; required true.
    pub runtime_writes_disclosed: bool,
    /// Human-readable trust disclosure label.
    pub trust_disclosure_label: String,
}

impl PreviewRuntimeTrustDisclosure {
    /// Whether this trust disclosure needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.trust_class.requires_attention_reason()
            || self.network_egress.requires_attention_reason()
            || self.executes_untrusted_code
    }
}

/// One remote preview route row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemotePreviewRouteRow {
    /// Stable route id.
    pub route_id: String,
    /// Durable review anchor id bound to this route.
    pub durable_anchor_id: String,
    /// Human-readable target identity (what the route points at).
    pub target_identity_label: String,
    /// Run id the preview target was built from.
    pub target_run_id: String,
    /// Human-readable commit / branch identity the route serves.
    pub target_commit_label: String,
    /// Lifecycle phase the route is presently in.
    pub lifecycle_phase: RouteLifecyclePhase,
    /// Expiry disclosure.
    pub expiry: RouteExpiryDisclosure,
    /// Host identity disclosure.
    pub host_identity: RouteHostIdentity,
    /// Preview / runtime trust disclosure.
    pub preview_trust: PreviewRuntimeTrustDisclosure,
    /// Provider-mode mutation mode the route was published under.
    pub mutation_mode: RouteMutationMode,
    /// Why the route is blocked, if it is.
    pub blocked_class: RouteBlockedClass,
    /// Human-readable actor attribution (under whose authority the route was published).
    pub actor_attribution_label: String,
    /// Opaque ref to the audit row that lands when the route is published or revoked.
    pub audit_row_ref: String,
    /// Attention reasons; required and non-empty when the route needs attention.
    pub attention_reasons: Vec<String>,
    /// Human-readable review summary.
    pub review_summary: String,
    /// Approval ticket ref; required when the mutation mode is `publish_now`.
    pub approval_ticket_ref: Option<String>,
    /// Browser-handoff packet ref; required when the mutation mode is `open_in_provider`.
    pub browser_handoff_ref: Option<String>,
    /// Publish-later queue item ref; required when the mutation mode is `deferred_publish`.
    pub deferred_queue_ref: Option<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl RemotePreviewRouteRow {
    /// Whether this route needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.lifecycle_phase.requires_attention_reason()
            || self.host_identity.host_class.requires_attention_reason()
            || self.preview_trust.requires_attention_reason()
            || self.blocked_class.requires_attention_reason()
    }
}

/// One lifecycle transition event bound to a route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteLifecycleEventRow {
    /// Route id this event belongs to.
    pub route_id: String,
    /// Stable event id.
    pub event_id: String,
    /// Human-readable, redaction-aware event label.
    pub event_label: String,
    /// Phase the route transitioned from.
    pub from_phase: RouteLifecyclePhase,
    /// Phase the route transitioned to.
    pub to_phase: RouteLifecyclePhase,
    /// Typed event kind.
    pub event_kind: RouteLifecycleEventKind,
    /// Human-readable disclosure label shown for the event.
    pub disclosure_label: String,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemotePreviewRouteTrustReview {
    /// Lifecycle phase is explicit, never implied.
    pub route_lifecycle_phase_explicit: bool,
    /// Every served route is time-bounded; an unbounded route is blocked.
    pub every_route_time_bounded: bool,
    /// Expiry auto-revoke is enforced for every time-bounded route.
    pub expiry_auto_revoke_enforced: bool,
    /// The target identity is explicit, never hidden.
    pub target_identity_explicit: bool,
    /// The host identity is disclosed, never hidden.
    pub host_identity_disclosed: bool,
    /// The preview / runtime trust is disclosed, never hidden.
    pub preview_runtime_trust_disclosed: bool,
    /// The network egress posture is disclosed.
    pub network_egress_disclosed: bool,
    /// Every mutating route is attributable to an actor.
    pub every_mutating_route_attributable: bool,
    /// An audit row is recorded for every route.
    pub audit_row_recorded_for_every_route: bool,
    /// The mutation mode cites the grant it depends on.
    pub mutation_mode_cites_required_grant: bool,
    /// No remote preview route creates hidden write scope.
    pub no_hidden_write_scope: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl RemotePreviewRouteTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.route_lifecycle_phase_explicit
            && self.every_route_time_bounded
            && self.expiry_auto_revoke_enforced
            && self.target_identity_explicit
            && self.host_identity_disclosed
            && self.preview_runtime_trust_disclosed
            && self.network_egress_disclosed
            && self.every_mutating_route_attributable
            && self.audit_row_recorded_for_every_route
            && self.mutation_mode_cites_required_grant
            && self.no_hidden_write_scope
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemotePreviewRouteConsumerProjection {
    /// Preview panel shows the lifecycle phase.
    pub preview_panel_shows_lifecycle_phase: bool,
    /// Route card shows the expiry.
    pub route_card_shows_expiry: bool,
    /// Route card shows the target identity.
    pub route_card_shows_target_identity: bool,
    /// Route card shows the host identity.
    pub route_card_shows_host_identity: bool,
    /// Route lifecycle sheet shows the trust disclosure.
    pub route_lifecycle_sheet_shows_trust_disclosure: bool,
    /// Review workspace header shows the actor attribution.
    pub review_workspace_header_shows_attribution: bool,
    /// Command palette shows the route state.
    pub command_palette_shows_route_state: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_truth: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_truth: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_truth: bool,
    /// Preview / Labs lanes are labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified: bool,
}

impl RemotePreviewRouteConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.preview_panel_shows_lifecycle_phase
            && self.route_card_shows_expiry
            && self.route_card_shows_target_identity
            && self.route_card_shows_host_identity
            && self.route_lifecycle_sheet_shows_trust_disclosure
            && self.review_workspace_header_shows_attribution
            && self.command_palette_shows_route_state
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.diagnostics_shows_truth
            && self.help_about_shows_truth
            && self.preview_labs_label_for_unqualified
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemotePreviewRouteProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`RemotePreviewRoutePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemotePreviewRoutePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Remote preview route rows.
    pub route_rows: Vec<RemotePreviewRouteRow>,
    /// Lifecycle event rows.
    pub lifecycle_event_rows: Vec<RouteLifecycleEventRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<RemotePreviewRouteDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<RemotePreviewRouteConsumerSurface>,
    /// Trust review block.
    pub trust_review: RemotePreviewRouteTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RemotePreviewRouteConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RemotePreviewRouteProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe remote preview route lifecycle, expiry, target identity, and trust disclosure packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemotePreviewRoutePacket {
    /// Record kind; must equal [`REMOTE_PREVIEW_ROUTE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`REMOTE_PREVIEW_ROUTE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Remote preview route rows.
    pub route_rows: Vec<RemotePreviewRouteRow>,
    /// Lifecycle event rows.
    pub lifecycle_event_rows: Vec<RouteLifecycleEventRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<RemotePreviewRouteDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<RemotePreviewRouteConsumerSurface>,
    /// Trust review block.
    pub trust_review: RemotePreviewRouteTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RemotePreviewRouteConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RemotePreviewRouteProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RemotePreviewRoutePacket {
    /// Builds a remote preview route packet from stable-lane input.
    pub fn new(input: RemotePreviewRoutePacketInput) -> Self {
        Self {
            record_kind: REMOTE_PREVIEW_ROUTE_RECORD_KIND.to_owned(),
            schema_version: REMOTE_PREVIEW_ROUTE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            route_rows: input.route_rows,
            lifecycle_event_rows: input.lifecycle_event_rows,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the remote preview route review invariants.
    pub fn validate(&self) -> Vec<RemotePreviewRouteViolation> {
        let mut violations = Vec::new();

        if self.record_kind != REMOTE_PREVIEW_ROUTE_RECORD_KIND {
            violations.push(RemotePreviewRouteViolation::WrongRecordKind);
        }
        if self.schema_version != REMOTE_PREVIEW_ROUTE_SCHEMA_VERSION {
            violations.push(RemotePreviewRouteViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RemotePreviewRouteViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(RemotePreviewRouteViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(RemotePreviewRouteViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_route_rows(self, &mut violations);
        validate_lifecycle_event_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(RemotePreviewRouteViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(RemotePreviewRouteViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(RemotePreviewRouteViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("remote preview route packet serializes"),
        ) {
            violations.push(RemotePreviewRouteViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("remote preview route packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let live_routes = self
            .route_rows
            .iter()
            .filter(|row| {
                matches!(
                    row.lifecycle_phase,
                    RouteLifecyclePhase::Live | RouteLifecyclePhase::ExpiringSoon
                )
            })
            .count();
        let blocked_routes = self
            .route_rows
            .iter()
            .filter(|row| row.blocked_class.is_blocked())
            .count();
        let time_bounded_routes = self
            .route_rows
            .iter()
            .filter(|row| row.expiry.expiry_state.is_time_bounded())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Remote Preview Route Lifecycle, Expiry, Target Identity, and Preview/Runtime Trust Disclosure\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Routes: {} ({} live, {} time-bounded, {} blocked)\n",
            self.route_rows.len(),
            live_routes,
            time_bounded_routes,
            blocked_routes
        ));
        out.push_str(&format!(
            "- Lifecycle events: {}\n",
            self.lifecycle_event_rows.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Routes\n\n");
        for row in &self.route_rows {
            out.push_str(&format!(
                "- **{}** ({}) → anchor `{}`: expiry `{}`, host `{}`, trust `{}`/`{}`, mode `{}`, blocked `{}`, authority `{}`\n",
                row.target_identity_label,
                row.lifecycle_phase.as_str(),
                row.durable_anchor_id,
                row.expiry.expiry_state.as_str(),
                row.host_identity.host_class.as_str(),
                row.preview_trust.trust_class.as_str(),
                row.preview_trust.network_egress.as_str(),
                row.mutation_mode.as_str(),
                row.blocked_class.as_str(),
                row.actor_attribution_label
            ));
        }

        out.push_str("\n## Lifecycle events\n\n");
        for row in &self.lifecycle_event_rows {
            out.push_str(&format!(
                "- `{}` on `{}`: `{}` → `{}` ({})\n",
                row.event_id,
                row.route_id,
                row.from_phase.as_str(),
                row.to_phase.as_str(),
                row.event_kind.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in remote preview route export.
#[derive(Debug)]
pub enum RemotePreviewRouteArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RemotePreviewRouteViolation>),
}

impl fmt::Display for RemotePreviewRouteArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "remote preview route export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "remote preview route export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RemotePreviewRouteArtifactError {}

/// Validation failures emitted by [`RemotePreviewRoutePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RemotePreviewRouteViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No route rows are present.
    RouteRowsMissing,
    /// A route row is incomplete.
    RouteRowIncomplete,
    /// A route is missing its target identity, run, or commit.
    TargetIdentityMissing,
    /// A route is not time-bounded and is not blocked for it.
    ExpiryNotTimeBounded,
    /// A time-bounded route's expiry disclosure is incomplete.
    ExpiryDisclosureIncomplete,
    /// A route's host identity is undisclosed.
    HostIdentityUndisclosed,
    /// A route's preview / runtime trust is undisclosed.
    RuntimeTrustUndisclosed,
    /// A mutating route is missing its actor attribution or audit row.
    AttributionMissing,
    /// A mutation mode is missing the grant ref it requires.
    MutationGrantRefMissing,
    /// A route needing attention is missing its attention reasons.
    AttentionReasonMissing,
    /// A route has no lifecycle event row.
    RouteMissingLifecycleEvent,
    /// A lifecycle event row references a route id with no route row.
    OrphanEventReference,
    /// No lifecycle event rows are present.
    LifecycleEventRowsMissing,
    /// A lifecycle event row is incomplete.
    LifecycleEventRowIncomplete,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl RemotePreviewRouteViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RouteRowsMissing => "route_rows_missing",
            Self::RouteRowIncomplete => "route_row_incomplete",
            Self::TargetIdentityMissing => "target_identity_missing",
            Self::ExpiryNotTimeBounded => "expiry_not_time_bounded",
            Self::ExpiryDisclosureIncomplete => "expiry_disclosure_incomplete",
            Self::HostIdentityUndisclosed => "host_identity_undisclosed",
            Self::RuntimeTrustUndisclosed => "runtime_trust_undisclosed",
            Self::AttributionMissing => "attribution_missing",
            Self::MutationGrantRefMissing => "mutation_grant_ref_missing",
            Self::AttentionReasonMissing => "attention_reason_missing",
            Self::RouteMissingLifecycleEvent => "route_missing_lifecycle_event",
            Self::OrphanEventReference => "orphan_event_reference",
            Self::LifecycleEventRowsMissing => "lifecycle_event_rows_missing",
            Self::LifecycleEventRowIncomplete => "lifecycle_event_row_incomplete",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable remote preview route export.
pub fn current_remote_preview_route_export(
) -> Result<RemotePreviewRoutePacket, RemotePreviewRouteArtifactError> {
    let packet: RemotePreviewRoutePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/support_export.json"
    )))
    .map_err(RemotePreviewRouteArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RemotePreviewRouteArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &RemotePreviewRoutePacket,
    violations: &mut Vec<RemotePreviewRouteViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        REMOTE_PREVIEW_ROUTE_SCHEMA_REF,
        REMOTE_PREVIEW_ROUTE_DOC_REF,
        REMOTE_PREVIEW_ROUTE_PREVIEW_ROUTE_CONTRACT_REF,
        REMOTE_PREVIEW_ROUTE_BROWSER_RUNTIME_CONTRACT_REF,
        REMOTE_PREVIEW_ROUTE_PIPELINE_RUN_CONTRACT_REF,
        REMOTE_PREVIEW_ROUTE_TRUST_CLASS_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(RemotePreviewRouteViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_route_rows(
    packet: &RemotePreviewRoutePacket,
    violations: &mut Vec<RemotePreviewRouteViolation>,
) {
    if packet.route_rows.is_empty() {
        violations.push(RemotePreviewRouteViolation::RouteRowsMissing);
        return;
    }

    let routes_with_events: BTreeSet<&str> = packet
        .lifecycle_event_rows
        .iter()
        .map(|row| row.route_id.as_str())
        .collect();

    for row in &packet.route_rows {
        if row.route_id.trim().is_empty()
            || row.durable_anchor_id.trim().is_empty()
            || row.review_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(RemotePreviewRouteViolation::RouteRowIncomplete);
        }
        if row.target_identity_label.trim().is_empty()
            || row.target_run_id.trim().is_empty()
            || row.target_commit_label.trim().is_empty()
        {
            violations.push(RemotePreviewRouteViolation::TargetIdentityMissing);
        }
        validate_expiry(row, violations);
        if row.host_identity.host_label.trim().is_empty() || !row.host_identity.origin_disclosed {
            violations.push(RemotePreviewRouteViolation::HostIdentityUndisclosed);
        }
        if row.preview_trust.trust_disclosure_label.trim().is_empty()
            || !row.preview_trust.runtime_writes_disclosed
        {
            violations.push(RemotePreviewRouteViolation::RuntimeTrustUndisclosed);
        }
        if row.actor_attribution_label.trim().is_empty() || row.audit_row_ref.trim().is_empty() {
            violations.push(RemotePreviewRouteViolation::AttributionMissing);
        }
        validate_mutation_grant(row, violations);
        if row.requires_attention_reason() && row.attention_reasons.is_empty() {
            violations.push(RemotePreviewRouteViolation::AttentionReasonMissing);
        }
        if !row.route_id.trim().is_empty() && !routes_with_events.contains(row.route_id.as_str()) {
            violations.push(RemotePreviewRouteViolation::RouteMissingLifecycleEvent);
        }
    }
}

fn validate_expiry(row: &RemotePreviewRouteRow, violations: &mut Vec<RemotePreviewRouteViolation>) {
    if row.expiry.expiry_state.is_time_bounded() {
        if !row.expiry.is_time_bounded_complete() {
            violations.push(RemotePreviewRouteViolation::ExpiryDisclosureIncomplete);
        }
    } else if !matches!(
        row.blocked_class,
        RouteBlockedClass::BlockedNoExpiryNotTimeBounded
    ) {
        violations.push(RemotePreviewRouteViolation::ExpiryNotTimeBounded);
    }
}

fn validate_mutation_grant(
    row: &RemotePreviewRouteRow,
    violations: &mut Vec<RemotePreviewRouteViolation>,
) {
    let approval_ok = !row.mutation_mode.requires_approval_ref()
        || row
            .approval_ticket_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
    let handoff_ok = !row.mutation_mode.requires_browser_handoff_ref()
        || row
            .browser_handoff_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
    let deferred_ok = !row.mutation_mode.requires_deferred_queue_ref()
        || row
            .deferred_queue_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
    if !(approval_ok && handoff_ok && deferred_ok) {
        violations.push(RemotePreviewRouteViolation::MutationGrantRefMissing);
    }
}

fn validate_lifecycle_event_rows(
    packet: &RemotePreviewRoutePacket,
    violations: &mut Vec<RemotePreviewRouteViolation>,
) {
    if packet.lifecycle_event_rows.is_empty() {
        violations.push(RemotePreviewRouteViolation::LifecycleEventRowsMissing);
        return;
    }

    let route_ids: BTreeSet<&str> = packet
        .route_rows
        .iter()
        .map(|row| row.route_id.as_str())
        .collect();

    for row in &packet.lifecycle_event_rows {
        if row.route_id.trim().is_empty()
            || row.event_id.trim().is_empty()
            || row.event_label.trim().is_empty()
            || row.disclosure_label.trim().is_empty()
        {
            violations.push(RemotePreviewRouteViolation::LifecycleEventRowIncomplete);
        }
        if !row.route_id.trim().is_empty() && !route_ids.contains(row.route_id.as_str()) {
            violations.push(RemotePreviewRouteViolation::OrphanEventReference);
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret ")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
