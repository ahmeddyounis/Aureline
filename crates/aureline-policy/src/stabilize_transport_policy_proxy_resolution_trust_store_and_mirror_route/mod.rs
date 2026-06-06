//! Stable transport-policy, proxy-resolution, trust-store, network-event, and
//! mirror-route inspector contract.
//!
//! This module is the shared product truth for enterprise transport routing
//! across update, marketplace, docs, AI, provider, remote, and bootstrap lanes.
//! It keeps route source precedence, policy decisions, proxy chains,
//! trust-store layers, mirror/offline state, handshake outcomes, and
//! control-plane/data-plane impairment in typed records that are safe for
//! diagnostics and support export.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version carried on transport inspector records.
pub const TRANSPORT_POLICY_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref for transport policy inspector records.
pub const TRANSPORT_POLICY_INSPECTOR_SHARED_CONTRACT_REF: &str =
    "policy:transport_policy_network_event_mirror_route:v1";

/// Record-kind tag for [`TransportPolicyInspectorPage`].
pub const TRANSPORT_POLICY_INSPECTOR_PAGE_RECORD_KIND: &str =
    "policy_transport_policy_inspector_page_record";

/// Record-kind tag for [`TransportPolicyRecord`].
pub const TRANSPORT_POLICY_RECORD_KIND: &str = "policy_transport_policy_record";

/// Record-kind tag for [`NetworkEventRecord`].
pub const NETWORK_EVENT_RECORD_KIND: &str = "policy_network_event_record";

/// Record-kind tag for [`TransportPolicyInspectorDefect`].
pub const TRANSPORT_POLICY_INSPECTOR_DEFECT_RECORD_KIND: &str =
    "policy_transport_policy_inspector_defect_record";

/// Record-kind tag for [`TransportPolicyInspectorSupportExport`].
pub const TRANSPORT_POLICY_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_transport_policy_inspector_support_export_record";

/// Repo-relative doc path for this stable lane.
pub const TRANSPORT_POLICY_INSPECTOR_DOC_REF: &str =
    "docs/enterprise/m4/stabilize-transport-policy-proxy-resolution-trust-store-and-mirror-route.md";

/// Repo-relative artifact path for this stable lane.
pub const TRANSPORT_POLICY_INSPECTOR_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/stabilize-transport-policy-proxy-resolution-trust-store-and-mirror-route.md";

/// Stable proof-index row that references this packet.
pub const TRANSPORT_POLICY_INSPECTOR_STABLE_PROOF_INDEX_REF: &str =
    "artifacts/release/stable_proof_index.json#proof:transport_policy_inspector_truth";

/// Endpoint family whose egress path is governed by the shared transport policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointClass {
    /// Software update and release metadata.
    Update,
    /// Extension marketplace, registry, and catalog metadata.
    Marketplace,
    /// Documentation packs, docs search indexes, and help content.
    Docs,
    /// AI provider gateway, model metadata, and AI support calls.
    Ai,
    /// Connected code host, CI, issue, release, and partner providers.
    Provider,
    /// SSH, remote-agent, tunnel, and managed workspace routes.
    Remote,
    /// Bootstrap acquisition, first-run profile, and install bootstrap flows.
    Bootstrap,
}

impl EndpointClass {
    /// Required endpoint coverage set in canonical order.
    pub const ALL: [Self; 7] = [
        Self::Update,
        Self::Marketplace,
        Self::Docs,
        Self::Ai,
        Self::Provider,
        Self::Remote,
        Self::Bootstrap,
    ];

    /// Stable token for this endpoint class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Update => "update",
            Self::Marketplace => "marketplace",
            Self::Docs => "docs",
            Self::Ai => "ai",
            Self::Provider => "provider",
            Self::Remote => "remote",
            Self::Bootstrap => "bootstrap",
        }
    }
}

/// Source selected during proxy and route resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteSourceClass {
    /// Policy-pinned route from a signed managed policy.
    PolicyPinned,
    /// Manual route configured by an admin or user setting.
    Manual,
    /// Platform system proxy route.
    System,
    /// PAC route selected after PAC evaluation.
    Pac,
    /// Declared signed mirror route.
    MirrorOnly,
    /// Offline route with no live egress.
    Offline,
}

impl RouteSourceClass {
    /// Canonical route-source precedence, highest priority first.
    pub const PRECEDENCE: [Self; 6] = [
        Self::PolicyPinned,
        Self::Manual,
        Self::System,
        Self::Pac,
        Self::MirrorOnly,
        Self::Offline,
    ];

    /// Stable token for this route source.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyPinned => "policy_pinned",
            Self::Manual => "manual",
            Self::System => "system",
            Self::Pac => "pac",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
        }
    }

    /// Canonical precedence rank; lower is more authoritative.
    pub const fn precedence_rank(self) -> u8 {
        match self {
            Self::PolicyPinned => 1,
            Self::Manual => 2,
            Self::System => 3,
            Self::Pac => 4,
            Self::MirrorOnly => 5,
            Self::Offline => 6,
        }
    }
}

/// Effective egress decision for a governed endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EgressDecisionClass {
    /// Request is allowed through the selected route.
    Allow,
    /// Request is allowed through a declared mirror route.
    AllowMirror,
    /// Request is deferred offline and can be retried safely later.
    OfflineDeferred,
    /// Request is blocked by policy or egress class.
    DenyPolicy,
    /// Request is blocked by profile or endpoint contract mismatch.
    DenyContractMismatch,
    /// Request is blocked by trust validation failure.
    DenyTrust,
    /// Request is blocked by transport failure after policy allowed it.
    DenyTransport,
    /// Mirror metadata or revocation state is stale.
    StaleMirror,
}

impl EgressDecisionClass {
    /// Stable token for this decision.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::AllowMirror => "allow_mirror",
            Self::OfflineDeferred => "offline_deferred",
            Self::DenyPolicy => "deny_policy",
            Self::DenyContractMismatch => "deny_contract_mismatch",
            Self::DenyTrust => "deny_trust",
            Self::DenyTransport => "deny_transport",
            Self::StaleMirror => "stale_mirror",
        }
    }

    /// True when this decision selected mirror or offline continuity.
    pub const fn uses_mirror_or_offline(self) -> bool {
        matches!(
            self,
            Self::AllowMirror | Self::OfflineDeferred | Self::StaleMirror
        )
    }
}

/// Result of route handshake after policy and proxy resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandshakeOutcomeClass {
    /// Handshake succeeded.
    Succeeded,
    /// Request was blocked by policy before handshake.
    PolicyBlocked,
    /// Endpoint contract did not match the declared deployment profile.
    ContractMismatch,
    /// CA, pin, SSH host proof, or client certificate validation failed.
    TrustFailed,
    /// DNS, proxy, TCP, TLS transport, or tunnel setup failed.
    TransportFailed,
    /// Handshake did not run because the route used cached or offline content.
    NotAttemptedOffline,
}

impl HandshakeOutcomeClass {
    /// Stable token for this handshake outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::PolicyBlocked => "policy_blocked",
            Self::ContractMismatch => "contract_mismatch",
            Self::TrustFailed => "trust_failed",
            Self::TransportFailed => "transport_failed",
            Self::NotAttemptedOffline => "not_attempted_offline",
        }
    }
}

/// Trust-store layer represented in the inspector snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportTrustLayerClass {
    /// OS platform root store.
    OsRoots,
    /// Organization custom CA bundle.
    CustomCaBundle,
    /// Pinned SSH host proof.
    PinnedSshHostProof,
    /// Client-certificate binding.
    ClientCertificate,
    /// Imported mirror trust root.
    MirrorTrustRoot,
}

impl TransportTrustLayerClass {
    /// Required trust-store layer set in canonical order.
    pub const ALL: [Self; 5] = [
        Self::OsRoots,
        Self::CustomCaBundle,
        Self::PinnedSshHostProof,
        Self::ClientCertificate,
        Self::MirrorTrustRoot,
    ];

    /// Stable token for this trust layer.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsRoots => "os_roots",
            Self::CustomCaBundle => "custom_ca_bundle",
            Self::PinnedSshHostProof => "pinned_ssh_host_proof",
            Self::ClientCertificate => "client_certificate",
            Self::MirrorTrustRoot => "mirror_trust_root",
        }
    }
}

/// Control-plane or data-plane status for transport inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaneStatusClass {
    /// Plane is reachable and not degraded.
    Reachable,
    /// Plane is unavailable while local-core work can continue.
    UnreachableLocalContinuity,
    /// Plane is impaired but can serve cached or mirrored content.
    DegradedCachedOrMirrored,
    /// Plane was blocked by policy.
    PolicyBlocked,
}

impl PlaneStatusClass {
    /// Stable token for this plane status.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::UnreachableLocalContinuity => "unreachable_local_continuity",
            Self::DegradedCachedOrMirrored => "degraded_cached_or_mirrored",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// One step in the effective proxy resolution chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyResolutionStep {
    /// Route source evaluated by this step.
    pub source: RouteSourceClass,
    /// Stable token for [`Self::source`].
    pub source_token: String,
    /// Canonical precedence rank; lower wins.
    pub precedence_rank: u8,
    /// Whether this step was selected as the effective route.
    pub selected: bool,
    /// Export-safe status for this step.
    pub resolution_status_token: String,
    /// Export-safe explanation of this step.
    pub explanation_label: String,
}

/// Snapshot of one trust-store layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustLayerSnapshot {
    /// Trust-store layer represented by this row.
    pub layer: TransportTrustLayerClass,
    /// Stable token for [`Self::layer`].
    pub layer_token: String,
    /// Opaque layer ref; never raw certificate or key material.
    pub layer_ref: String,
    /// Export-safe health token.
    pub health_token: String,
    /// Opaque change-attribution ref for the latest relevant event.
    pub change_event_ref: String,
    /// Narrow repair or revalidation action.
    pub repair_action_token: String,
    /// Opaque refs to endpoint classes affected by this layer.
    pub affected_endpoint_refs: Vec<String>,
    /// True when raw trust material is excluded.
    pub raw_trust_material_excluded: bool,
}

/// Mirror or offline route state bound to a policy record or event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorRouteState {
    /// Opaque mirror or offline route ref.
    pub mirror_route_ref: String,
    /// Route state token.
    pub state_token: String,
    /// Freshness token for mirror metadata and revocation state.
    pub freshness_token: String,
    /// Signer-continuity token for mirrored material.
    pub signer_continuity_token: String,
    /// Whether this route is eligible for offline use.
    pub offline_eligible: bool,
    /// Export-safe route label.
    pub route_label: String,
}

/// Effective transport policy record for one endpoint class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportPolicyRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub policy_record_id: String,
    /// Endpoint class governed by this record.
    pub endpoint_class: EndpointClass,
    /// Stable token for [`Self::endpoint_class`].
    pub endpoint_class_token: String,
    /// Selected route source.
    pub effective_route_source: RouteSourceClass,
    /// Stable token for [`Self::effective_route_source`].
    pub effective_route_source_token: String,
    /// Effective egress decision.
    pub egress_decision: EgressDecisionClass,
    /// Stable token for [`Self::egress_decision`].
    pub egress_decision_token: String,
    /// Opaque policy epoch ref.
    pub policy_epoch_ref: String,
    /// Ordered proxy and route-resolution chain.
    pub proxy_resolution_chain: Vec<ProxyResolutionStep>,
    /// Mirror/offline route state for this endpoint.
    pub mirror_route_state: MirrorRouteState,
    /// Trust-store layer snapshots in effect for this endpoint.
    pub trust_store_snapshot: Vec<TrustLayerSnapshot>,
    /// Control-plane status.
    pub control_plane_status: PlaneStatusClass,
    /// Stable token for [`Self::control_plane_status`].
    pub control_plane_status_token: String,
    /// Data-plane status.
    pub data_plane_status: PlaneStatusClass,
    /// Stable token for [`Self::data_plane_status`].
    pub data_plane_status_token: String,
    /// Cached or offline posture token.
    pub cached_offline_posture_token: String,
    /// True when local-core continuity is explicit.
    pub local_core_continuity_explicit: bool,
    /// Export-safe explanation of why the route won.
    pub explanation_label: String,
    /// True when raw credentials, tokens, PAC bodies, and private material are excluded.
    pub raw_secret_material_excluded: bool,
}

/// Event emitted for a network decision or failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEventRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable event id.
    pub event_id: String,
    /// Endpoint class for the request.
    pub endpoint_class: EndpointClass,
    /// Stable token for [`Self::endpoint_class`].
    pub endpoint_class_token: String,
    /// Opaque target ref; no raw hostname, URL, or IP address.
    pub target_ref: String,
    /// Opaque resolved route ref.
    pub resolved_route_ref: String,
    /// Decision made for this event.
    pub egress_decision: EgressDecisionClass,
    /// Stable token for [`Self::egress_decision`].
    pub egress_decision_token: String,
    /// Handshake outcome.
    pub handshake_outcome: HandshakeOutcomeClass,
    /// Stable token for [`Self::handshake_outcome`].
    pub handshake_outcome_token: String,
    /// Selected route source token.
    pub resolved_route_source_token: String,
    /// Proxy chain used for the event.
    pub proxy_resolution_chain: Vec<ProxyResolutionStep>,
    /// Trust layers consulted by this event.
    pub trust_layer_refs: Vec<String>,
    /// Mirror or offline state visible to the event.
    pub mirror_route_state: MirrorRouteState,
    /// Cached/offline posture token.
    pub cached_offline_posture_token: String,
    /// Control-plane status.
    pub control_plane_status: PlaneStatusClass,
    /// Stable token for [`Self::control_plane_status`].
    pub control_plane_status_token: String,
    /// Data-plane status.
    pub data_plane_status: PlaneStatusClass,
    /// Stable token for [`Self::data_plane_status`].
    pub data_plane_status_token: String,
    /// Narrow recovery action hint.
    pub recovery_action_hint_token: String,
    /// Export-safe event explanation.
    pub explanation_label: String,
    /// True when raw credentials, payloads, URLs, and private material are excluded.
    pub raw_secret_material_excluded: bool,
}

/// Qualification tier derived by the transport policy inspector audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportPolicyInspectorQualificationClass {
    /// All required conditions hold.
    Stable,
    /// A non-withdrawal condition is incomplete.
    Beta,
    /// Required endpoint, route-source, event, or trust-layer coverage is missing.
    Preview,
    /// A hard redaction guardrail was violated.
    Withdrawn,
}

impl TransportPolicyInspectorQualificationClass {
    /// Stable token for this qualification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }
}

/// Narrow reason emitted by the transport policy inspector audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportPolicyInspectorNarrowReasonClass {
    /// No narrowing occurred.
    NotNarrowed,
    /// Required endpoint coverage is missing.
    MissingEndpointCoverage,
    /// Canonical route-source precedence is missing or out of order.
    RouteSourcePrecedenceIncomplete,
    /// Network events do not cover every claimed endpoint.
    NetworkEventCoverageMissing,
    /// Required trust-store layer coverage is missing.
    TrustLayerCoverageMissing,
    /// A proxy-resolution chain is missing or does not select exactly one route.
    ProxyResolutionChainInvalid,
    /// A policy or event has no mirror/offline state.
    MirrorRouteStateMissing,
    /// A policy epoch ref is missing.
    PolicyEpochMissing,
    /// Control-plane/data-plane state is collapsed or missing.
    PlaneStateCollapsed,
    /// A failure lacks a typed recovery action.
    RecoveryActionMissing,
    /// Local-core continuity was not explicit.
    LocalCoreContinuityMissing,
    /// Raw secret or trust material was exposed.
    RawSecretMaterialExposed,
}

impl TransportPolicyInspectorNarrowReasonClass {
    /// Stable token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::MissingEndpointCoverage => "missing_endpoint_coverage",
            Self::RouteSourcePrecedenceIncomplete => "route_source_precedence_incomplete",
            Self::NetworkEventCoverageMissing => "network_event_coverage_missing",
            Self::TrustLayerCoverageMissing => "trust_layer_coverage_missing",
            Self::ProxyResolutionChainInvalid => "proxy_resolution_chain_invalid",
            Self::MirrorRouteStateMissing => "mirror_route_state_missing",
            Self::PolicyEpochMissing => "policy_epoch_missing",
            Self::PlaneStateCollapsed => "plane_state_collapsed",
            Self::RecoveryActionMissing => "recovery_action_missing",
            Self::LocalCoreContinuityMissing => "local_core_continuity_missing",
            Self::RawSecretMaterialExposed => "raw_secret_material_exposed",
        }
    }

    /// True when the reason withdraws the packet immediately.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::RawSecretMaterialExposed)
    }

    const fn qualification(self) -> TransportPolicyInspectorQualificationClass {
        match self {
            Self::NotNarrowed => TransportPolicyInspectorQualificationClass::Stable,
            Self::MissingEndpointCoverage
            | Self::RouteSourcePrecedenceIncomplete
            | Self::NetworkEventCoverageMissing
            | Self::TrustLayerCoverageMissing => {
                TransportPolicyInspectorQualificationClass::Preview
            }
            Self::RawSecretMaterialExposed => TransportPolicyInspectorQualificationClass::Withdrawn,
            _ => TransportPolicyInspectorQualificationClass::Beta,
        }
    }
}

/// Typed defect emitted by [`audit_transport_policy_inspector_page`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportPolicyInspectorDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason.
    pub narrow_reason: TransportPolicyInspectorNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id.
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl TransportPolicyInspectorDefect {
    fn new(
        narrow_reason: TransportPolicyInspectorNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: TRANSPORT_POLICY_INSPECTOR_DEFECT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_POLICY_INSPECTOR_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_POLICY_INSPECTOR_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:transport-policy-inspector:{}:{}",
                narrow_reason.as_str(),
                source
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source,
            note: note.into(),
        }
    }
}

/// Aggregate summary for a transport policy inspector page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TransportPolicyInspectorSummary {
    /// Total transport policy record count.
    pub policy_record_count: usize,
    /// Total network event count.
    pub network_event_count: usize,
    /// Endpoint tokens covered by policy records.
    pub endpoint_classes_covered: Vec<String>,
    /// Endpoint tokens covered by network events.
    pub event_endpoint_classes_covered: Vec<String>,
    /// Canonical route-source precedence tokens.
    pub route_source_precedence_tokens: Vec<String>,
    /// Trust-store layer tokens present in the inspector.
    pub trust_layer_tokens: Vec<String>,
    /// Egress decision tokens present across records and events.
    pub egress_decision_tokens: Vec<String>,
    /// Number of mirror or offline route states.
    pub mirror_route_state_count: usize,
    /// Number of records/events with explicit local-core continuity.
    pub local_core_continuity_explicit_count: usize,
    /// Stable proof-index ref that carries this proof packet.
    pub stable_proof_index_ref: String,
    /// Whether raw private material is excluded from the whole page.
    pub raw_secret_material_excluded: bool,
    /// Derived overall qualification token.
    pub overall_qualification_token: String,
}

impl TransportPolicyInspectorSummary {
    fn from_page(
        policies: &[TransportPolicyRecord],
        events: &[NetworkEventRecord],
        trust_layers: &[TrustLayerSnapshot],
        route_source_precedence: &[RouteSourceClass],
        defects: &[TransportPolicyInspectorDefect],
    ) -> Self {
        let mut endpoint_classes = BTreeSet::new();
        let mut event_endpoint_classes = BTreeSet::new();
        let mut trust_layer_tokens = BTreeSet::new();
        let mut decisions = BTreeSet::new();
        let mut local_core = 0usize;
        let mut mirror_count = 0usize;
        let mut raw_secret_material_excluded = true;

        for policy in policies {
            endpoint_classes.insert(policy.endpoint_class_token.clone());
            decisions.insert(policy.egress_decision_token.clone());
            if policy.local_core_continuity_explicit {
                local_core += 1;
            }
            if !policy.mirror_route_state.mirror_route_ref.is_empty() {
                mirror_count += 1;
            }
            raw_secret_material_excluded &= policy.raw_secret_material_excluded
                && policy
                    .trust_store_snapshot
                    .iter()
                    .all(|layer| layer.raw_trust_material_excluded);
        }
        for event in events {
            event_endpoint_classes.insert(event.endpoint_class_token.clone());
            decisions.insert(event.egress_decision_token.clone());
            if !event.mirror_route_state.mirror_route_ref.is_empty() {
                mirror_count += 1;
            }
            raw_secret_material_excluded &= event.raw_secret_material_excluded;
        }
        for layer in trust_layers {
            trust_layer_tokens.insert(layer.layer_token.clone());
            raw_secret_material_excluded &= layer.raw_trust_material_excluded;
        }

        let highest = defects
            .iter()
            .map(|defect| defect.narrow_reason.qualification())
            .max()
            .unwrap_or(TransportPolicyInspectorQualificationClass::Stable);

        Self {
            policy_record_count: policies.len(),
            network_event_count: events.len(),
            endpoint_classes_covered: endpoint_classes.into_iter().collect(),
            event_endpoint_classes_covered: event_endpoint_classes.into_iter().collect(),
            route_source_precedence_tokens: route_source_precedence
                .iter()
                .map(|source| source.as_str().to_owned())
                .collect(),
            trust_layer_tokens: trust_layer_tokens.into_iter().collect(),
            egress_decision_tokens: decisions.into_iter().collect(),
            mirror_route_state_count: mirror_count,
            local_core_continuity_explicit_count: local_core,
            stable_proof_index_ref: TRANSPORT_POLICY_INSPECTOR_STABLE_PROOF_INDEX_REF.to_owned(),
            raw_secret_material_excluded,
            overall_qualification_token: highest.as_str().to_owned(),
        }
    }
}

/// Stable inspector snapshot combining policies, events, trust layers, and mirror routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportPolicyInspectorPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable label.
    pub page_label: String,
    /// UTC generation time.
    pub generated_at: String,
    /// Canonical route-source precedence.
    pub route_source_precedence: Vec<RouteSourceClass>,
    /// Effective transport policies by endpoint class.
    pub policies: Vec<TransportPolicyRecord>,
    /// Bounded recent network-event ledger.
    pub network_events: Vec<NetworkEventRecord>,
    /// Trust-store layer snapshots.
    pub trust_store_layers: Vec<TrustLayerSnapshot>,
    /// Aggregate summary.
    pub summary: TransportPolicyInspectorSummary,
    /// Typed defects emitted by the audit.
    pub defects: Vec<TransportPolicyInspectorDefect>,
}

impl TransportPolicyInspectorPage {
    /// Builds a page and derives defects plus summary.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        route_source_precedence: Vec<RouteSourceClass>,
        policies: Vec<TransportPolicyRecord>,
        network_events: Vec<NetworkEventRecord>,
        trust_store_layers: Vec<TrustLayerSnapshot>,
    ) -> Self {
        let defects = audit_transport_policy_parts(
            &route_source_precedence,
            &policies,
            &network_events,
            &trust_store_layers,
        );
        let summary = TransportPolicyInspectorSummary::from_page(
            &policies,
            &network_events,
            &trust_store_layers,
            &route_source_precedence,
            &defects,
        );
        Self {
            record_kind: TRANSPORT_POLICY_INSPECTOR_PAGE_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_POLICY_INSPECTOR_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_POLICY_INSPECTOR_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            route_source_precedence,
            policies,
            network_events,
            trust_store_layers,
            summary,
            defects,
        }
    }

    /// True when the page qualifies stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == TransportPolicyInspectorQualificationClass::Stable.as_str()
    }

    /// True when policy records cover all required endpoint classes.
    pub fn covers_all_endpoint_classes(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .policies
            .iter()
            .map(|policy| policy.endpoint_class_token.as_str())
            .collect();
        EndpointClass::ALL
            .iter()
            .all(|endpoint| covered.contains(endpoint.as_str()))
    }

    /// True when all trust-store layers are present in the inspector.
    pub fn covers_all_trust_layers(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .trust_store_layers
            .iter()
            .map(|layer| layer.layer_token.as_str())
            .collect();
        TransportTrustLayerClass::ALL
            .iter()
            .all(|layer| covered.contains(layer.as_str()))
    }

    /// True when all default-export records exclude secret and private material.
    pub fn excludes_raw_secret_material(&self) -> bool {
        self.summary.raw_secret_material_excluded
    }
}

/// Support-export envelope for a transport policy inspector page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportPolicyInspectorSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC generation time.
    pub generated_at: String,
    /// Inspector page embedded as export evidence.
    pub page: TransportPolicyInspectorPage,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw secrets and private trust material are excluded.
    pub raw_secret_material_excluded: bool,
}

impl TransportPolicyInspectorSupportExport {
    /// Wraps a page in a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: TransportPolicyInspectorPage,
    ) -> Self {
        let mut counts = BTreeMap::new();
        for defect in &page.defects {
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        let raw_secret_material_excluded = page.excludes_raw_secret_material();
        Self {
            record_kind: TRANSPORT_POLICY_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_POLICY_INSPECTOR_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_POLICY_INSPECTOR_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            defect_counts_by_narrow_reason: counts,
            raw_secret_material_excluded,
        }
    }
}

/// Audits an inspector page and returns typed defects.
pub fn audit_transport_policy_inspector_page(
    page: &TransportPolicyInspectorPage,
) -> Vec<TransportPolicyInspectorDefect> {
    audit_transport_policy_parts(
        &page.route_source_precedence,
        &page.policies,
        &page.network_events,
        &page.trust_store_layers,
    )
}

/// Validates an inspector page, returning all typed defects on failure.
pub fn validate_transport_policy_inspector_page(
    page: &TransportPolicyInspectorPage,
) -> Result<(), Vec<TransportPolicyInspectorDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

/// Builds the seeded stable transport policy inspector page.
pub fn seeded_transport_policy_inspector_page() -> TransportPolicyInspectorPage {
    TransportPolicyInspectorPage::new(
        "policy:transport-policy-inspector:seeded:0001",
        "Transport policy, proxy resolution, trust-store layering, network-event, and mirror-route inspector truth",
        "2026-06-01T00:00:00Z",
        RouteSourceClass::PRECEDENCE.to_vec(),
        seeded_policies(),
        seeded_network_events(),
        seeded_trust_layers(),
    )
}

fn audit_transport_policy_parts(
    route_source_precedence: &[RouteSourceClass],
    policies: &[TransportPolicyRecord],
    events: &[NetworkEventRecord],
    trust_layers: &[TrustLayerSnapshot],
) -> Vec<TransportPolicyInspectorDefect> {
    let mut defects = Vec::new();

    if policies
        .iter()
        .any(|policy| !policy.raw_secret_material_excluded)
        || events
            .iter()
            .any(|event| !event.raw_secret_material_excluded)
        || trust_layers
            .iter()
            .any(|layer| !layer.raw_trust_material_excluded)
        || policies.iter().any(|policy| {
            policy
                .trust_store_snapshot
                .iter()
                .any(|layer| !layer.raw_trust_material_excluded)
        })
    {
        defects.push(TransportPolicyInspectorDefect::new(
            TransportPolicyInspectorNarrowReasonClass::RawSecretMaterialExposed,
            "page",
            "Raw credential, token, PAC, certificate, or private-key material crossed the inspector boundary.",
        ));
        return defects;
    }

    let policy_endpoints: BTreeSet<&str> = policies
        .iter()
        .map(|policy| policy.endpoint_class_token.as_str())
        .collect();
    for endpoint in EndpointClass::ALL {
        if !policy_endpoints.contains(endpoint.as_str()) {
            defects.push(TransportPolicyInspectorDefect::new(
                TransportPolicyInspectorNarrowReasonClass::MissingEndpointCoverage,
                endpoint.as_str(),
                "A required endpoint class has no transport-policy record.",
            ));
        }
    }

    if route_source_precedence != RouteSourceClass::PRECEDENCE {
        defects.push(TransportPolicyInspectorDefect::new(
            TransportPolicyInspectorNarrowReasonClass::RouteSourcePrecedenceIncomplete,
            "route_source_precedence",
            "Route source precedence must be policy_pinned, manual, system, pac, mirror_only, offline.",
        ));
    }

    let event_endpoints: BTreeSet<&str> = events
        .iter()
        .map(|event| event.endpoint_class_token.as_str())
        .collect();
    for endpoint in EndpointClass::ALL {
        if !event_endpoints.contains(endpoint.as_str()) {
            defects.push(TransportPolicyInspectorDefect::new(
                TransportPolicyInspectorNarrowReasonClass::NetworkEventCoverageMissing,
                endpoint.as_str(),
                "A required endpoint class has no network-event record.",
            ));
        }
    }

    let trust_layer_tokens: BTreeSet<&str> = trust_layers
        .iter()
        .map(|layer| layer.layer_token.as_str())
        .collect();
    for layer in TransportTrustLayerClass::ALL {
        if !trust_layer_tokens.contains(layer.as_str()) {
            defects.push(TransportPolicyInspectorDefect::new(
                TransportPolicyInspectorNarrowReasonClass::TrustLayerCoverageMissing,
                layer.as_str(),
                "A required trust-store layer is absent from the inspector snapshot.",
            ));
        }
    }

    for policy in policies {
        if policy.policy_epoch_ref.trim().is_empty() {
            defects.push(TransportPolicyInspectorDefect::new(
                TransportPolicyInspectorNarrowReasonClass::PolicyEpochMissing,
                &policy.policy_record_id,
                "Transport policy record is missing an opaque policy epoch ref.",
            ));
        }
        if !valid_proxy_chain(
            &policy.proxy_resolution_chain,
            policy.effective_route_source,
        ) {
            defects.push(TransportPolicyInspectorDefect::new(
                TransportPolicyInspectorNarrowReasonClass::ProxyResolutionChainInvalid,
                &policy.policy_record_id,
                "Proxy resolution chain must include canonical sources and select exactly the effective route.",
            ));
        }
        if policy.mirror_route_state.mirror_route_ref.trim().is_empty()
            || (policy.egress_decision.uses_mirror_or_offline()
                && policy.mirror_route_state.state_token == "not_applicable")
        {
            defects.push(TransportPolicyInspectorDefect::new(
                TransportPolicyInspectorNarrowReasonClass::MirrorRouteStateMissing,
                &policy.policy_record_id,
                "Policy record is missing mirror/offline route state.",
            ));
        }
        if policy.control_plane_status_token.trim().is_empty()
            || policy.data_plane_status_token.trim().is_empty()
        {
            defects.push(TransportPolicyInspectorDefect::new(
                TransportPolicyInspectorNarrowReasonClass::PlaneStateCollapsed,
                &policy.policy_record_id,
                "Control-plane and data-plane impairment must remain separate typed states.",
            ));
        }
        if !policy.local_core_continuity_explicit {
            defects.push(TransportPolicyInspectorDefect::new(
                TransportPolicyInspectorNarrowReasonClass::LocalCoreContinuityMissing,
                &policy.policy_record_id,
                "Local-core continuity must be explicit for every endpoint class.",
            ));
        }
    }

    for event in events {
        let selected_source = event
            .proxy_resolution_chain
            .iter()
            .find(|step| step.selected)
            .map(|step| step.source_token.as_str());
        if selected_source != Some(event.resolved_route_source_token.as_str()) {
            defects.push(TransportPolicyInspectorDefect::new(
                TransportPolicyInspectorNarrowReasonClass::ProxyResolutionChainInvalid,
                &event.event_id,
                "Event proxy chain must select exactly the route source recorded on the event.",
            ));
        }
        if event.mirror_route_state.mirror_route_ref.trim().is_empty() {
            defects.push(TransportPolicyInspectorDefect::new(
                TransportPolicyInspectorNarrowReasonClass::MirrorRouteStateMissing,
                &event.event_id,
                "Network event is missing mirror/offline route state.",
            ));
        }
        if event.control_plane_status_token.trim().is_empty()
            || event.data_plane_status_token.trim().is_empty()
        {
            defects.push(TransportPolicyInspectorDefect::new(
                TransportPolicyInspectorNarrowReasonClass::PlaneStateCollapsed,
                &event.event_id,
                "Network event collapsed control-plane and data-plane impairment.",
            ));
        }
        if event.egress_decision != EgressDecisionClass::Allow
            && event.recovery_action_hint_token.trim().is_empty()
        {
            defects.push(TransportPolicyInspectorDefect::new(
                TransportPolicyInspectorNarrowReasonClass::RecoveryActionMissing,
                &event.event_id,
                "Non-allow network event is missing a narrow recovery action hint.",
            ));
        }
    }

    defects
}

fn valid_proxy_chain(chain: &[ProxyResolutionStep], selected: RouteSourceClass) -> bool {
    if chain.is_empty() {
        return false;
    }
    let selected_count = chain.iter().filter(|step| step.selected).count();
    let selected_matches = chain
        .iter()
        .any(|step| step.selected && step.source == selected);
    let sources: BTreeSet<RouteSourceClass> = chain.iter().map(|step| step.source).collect();
    selected_count == 1
        && selected_matches
        && RouteSourceClass::PRECEDENCE
            .iter()
            .all(|source| sources.contains(source))
}

fn proxy_chain(selected: RouteSourceClass) -> Vec<ProxyResolutionStep> {
    RouteSourceClass::PRECEDENCE
        .iter()
        .map(|source| ProxyResolutionStep {
            source: *source,
            source_token: source.as_str().to_owned(),
            precedence_rank: source.precedence_rank(),
            selected: *source == selected,
            resolution_status_token: if *source == selected {
                "selected"
            } else {
                "shadowed"
            }
            .to_owned(),
            explanation_label: if *source == selected {
                format!("{} selected by effective transport policy", source.as_str())
            } else {
                format!("{} evaluated and not selected", source.as_str())
            },
        })
        .collect()
}

fn mirror_state(ref_suffix: &str, state_token: &str, offline_eligible: bool) -> MirrorRouteState {
    MirrorRouteState {
        mirror_route_ref: format!("mirror-route:{ref_suffix}"),
        state_token: state_token.to_owned(),
        freshness_token: "current".to_owned(),
        signer_continuity_token: "verified".to_owned(),
        offline_eligible,
        route_label: "Declared signed mirror/offline route state is inspectable.".to_owned(),
    }
}

fn trust_layer(
    layer: TransportTrustLayerClass,
    health_token: &str,
    repair_action_token: &str,
    affected: &[&str],
) -> TrustLayerSnapshot {
    TrustLayerSnapshot {
        layer,
        layer_token: layer.as_str().to_owned(),
        layer_ref: format!("trust-layer:{}", layer.as_str()),
        health_token: health_token.to_owned(),
        change_event_ref: format!("trust-change:{}", layer.as_str()),
        repair_action_token: repair_action_token.to_owned(),
        affected_endpoint_refs: affected.iter().map(|item| (*item).to_owned()).collect(),
        raw_trust_material_excluded: true,
    }
}

fn seeded_trust_layers() -> Vec<TrustLayerSnapshot> {
    vec![
        trust_layer(
            TransportTrustLayerClass::OsRoots,
            "active",
            "none_required",
            &["update", "marketplace", "docs"],
        ),
        trust_layer(
            TransportTrustLayerClass::CustomCaBundle,
            "active",
            "revalidate_custom_ca_bundle",
            &["ai", "provider", "remote"],
        ),
        trust_layer(
            TransportTrustLayerClass::PinnedSshHostProof,
            "active",
            "reenroll_ssh_host_proof",
            &["remote", "provider"],
        ),
        trust_layer(
            TransportTrustLayerClass::ClientCertificate,
            "active",
            "renew_client_certificate",
            &["ai", "remote", "bootstrap"],
        ),
        trust_layer(
            TransportTrustLayerClass::MirrorTrustRoot,
            "active",
            "refresh_mirror_trust_root",
            &["update", "marketplace", "docs", "bootstrap"],
        ),
    ]
}

fn policy(
    endpoint: EndpointClass,
    route: RouteSourceClass,
    decision: EgressDecisionClass,
    mirror: MirrorRouteState,
    control: PlaneStatusClass,
    data: PlaneStatusClass,
    cached: &str,
    explanation: &str,
) -> TransportPolicyRecord {
    TransportPolicyRecord {
        record_kind: TRANSPORT_POLICY_RECORD_KIND.to_owned(),
        schema_version: TRANSPORT_POLICY_INSPECTOR_SCHEMA_VERSION,
        shared_contract_ref: TRANSPORT_POLICY_INSPECTOR_SHARED_CONTRACT_REF.to_owned(),
        policy_record_id: format!("transport-policy:{}", endpoint.as_str()),
        endpoint_class: endpoint,
        endpoint_class_token: endpoint.as_str().to_owned(),
        effective_route_source: route,
        effective_route_source_token: route.as_str().to_owned(),
        egress_decision: decision,
        egress_decision_token: decision.as_str().to_owned(),
        policy_epoch_ref: "policy-epoch:enterprise-transport:2026-06-01".to_owned(),
        proxy_resolution_chain: proxy_chain(route),
        mirror_route_state: mirror,
        trust_store_snapshot: seeded_trust_layers(),
        control_plane_status: control,
        control_plane_status_token: control.as_str().to_owned(),
        data_plane_status: data,
        data_plane_status_token: data.as_str().to_owned(),
        cached_offline_posture_token: cached.to_owned(),
        local_core_continuity_explicit: true,
        explanation_label: explanation.to_owned(),
        raw_secret_material_excluded: true,
    }
}

fn seeded_policies() -> Vec<TransportPolicyRecord> {
    vec![
        policy(
            EndpointClass::Update,
            RouteSourceClass::PolicyPinned,
            EgressDecisionClass::Allow,
            mirror_state("update", "available_not_selected", true),
            PlaneStatusClass::Reachable,
            PlaneStatusClass::Reachable,
            "online",
            "Update traffic uses policy-pinned egress with mirror fallback visible.",
        ),
        policy(
            EndpointClass::Marketplace,
            RouteSourceClass::Manual,
            EgressDecisionClass::DenyPolicy,
            mirror_state("marketplace", "not_selected_policy_block", true),
            PlaneStatusClass::PolicyBlocked,
            PlaneStatusClass::Reachable,
            "online",
            "Marketplace route is blocked by egress policy without hiding transport health.",
        ),
        policy(
            EndpointClass::Docs,
            RouteSourceClass::MirrorOnly,
            EgressDecisionClass::AllowMirror,
            mirror_state("docs", "selected", true),
            PlaneStatusClass::DegradedCachedOrMirrored,
            PlaneStatusClass::DegradedCachedOrMirrored,
            "mirror_served",
            "Docs route is served from a declared signed mirror.",
        ),
        policy(
            EndpointClass::Ai,
            RouteSourceClass::System,
            EgressDecisionClass::DenyTrust,
            mirror_state("ai", "not_applicable", false),
            PlaneStatusClass::Reachable,
            PlaneStatusClass::UnreachableLocalContinuity,
            "online",
            "AI data plane is blocked by trust validation while local work continues.",
        ),
        policy(
            EndpointClass::Provider,
            RouteSourceClass::Pac,
            EgressDecisionClass::DenyTransport,
            mirror_state("provider", "not_applicable", false),
            PlaneStatusClass::Reachable,
            PlaneStatusClass::UnreachableLocalContinuity,
            "online",
            "Provider route resolved through PAC but transport failed after policy allowed it.",
        ),
        policy(
            EndpointClass::Remote,
            RouteSourceClass::System,
            EgressDecisionClass::DenyContractMismatch,
            mirror_state("remote", "not_applicable", false),
            PlaneStatusClass::Reachable,
            PlaneStatusClass::PolicyBlocked,
            "online",
            "Remote attach contract mismatch is distinct from generic connectivity failure.",
        ),
        policy(
            EndpointClass::Bootstrap,
            RouteSourceClass::Offline,
            EgressDecisionClass::OfflineDeferred,
            mirror_state("bootstrap", "offline_deferred", true),
            PlaneStatusClass::UnreachableLocalContinuity,
            PlaneStatusClass::DegradedCachedOrMirrored,
            "offline_grace",
            "Bootstrap uses signed local/offline media and defers live egress.",
        ),
    ]
}

fn event(
    endpoint: EndpointClass,
    route: RouteSourceClass,
    decision: EgressDecisionClass,
    handshake: HandshakeOutcomeClass,
    control: PlaneStatusClass,
    data: PlaneStatusClass,
    recovery: &str,
    explanation: &str,
) -> NetworkEventRecord {
    NetworkEventRecord {
        record_kind: NETWORK_EVENT_RECORD_KIND.to_owned(),
        schema_version: TRANSPORT_POLICY_INSPECTOR_SCHEMA_VERSION,
        shared_contract_ref: TRANSPORT_POLICY_INSPECTOR_SHARED_CONTRACT_REF.to_owned(),
        event_id: format!("network-event:{}", endpoint.as_str()),
        endpoint_class: endpoint,
        endpoint_class_token: endpoint.as_str().to_owned(),
        target_ref: format!("endpoint-class:{}", endpoint.as_str()),
        resolved_route_ref: format!("resolved-route:{}:{}", endpoint.as_str(), route.as_str()),
        egress_decision: decision,
        egress_decision_token: decision.as_str().to_owned(),
        handshake_outcome: handshake,
        handshake_outcome_token: handshake.as_str().to_owned(),
        resolved_route_source_token: route.as_str().to_owned(),
        proxy_resolution_chain: proxy_chain(route),
        trust_layer_refs: TransportTrustLayerClass::ALL
            .iter()
            .map(|layer| format!("trust-layer:{}", layer.as_str()))
            .collect(),
        mirror_route_state: mirror_state(
            endpoint.as_str(),
            if decision.uses_mirror_or_offline() {
                "selected"
            } else {
                "available_or_not_applicable"
            },
            decision.uses_mirror_or_offline(),
        ),
        cached_offline_posture_token: if decision.uses_mirror_or_offline() {
            "offline_or_mirror_visible"
        } else {
            "online"
        }
        .to_owned(),
        control_plane_status: control,
        control_plane_status_token: control.as_str().to_owned(),
        data_plane_status: data,
        data_plane_status_token: data.as_str().to_owned(),
        recovery_action_hint_token: recovery.to_owned(),
        explanation_label: explanation.to_owned(),
        raw_secret_material_excluded: true,
    }
}

fn seeded_network_events() -> Vec<NetworkEventRecord> {
    vec![
        event(
            EndpointClass::Update,
            RouteSourceClass::PolicyPinned,
            EgressDecisionClass::Allow,
            HandshakeOutcomeClass::Succeeded,
            PlaneStatusClass::Reachable,
            PlaneStatusClass::Reachable,
            "none_required",
            "Update handshake succeeded through policy-pinned route.",
        ),
        event(
            EndpointClass::Marketplace,
            RouteSourceClass::Manual,
            EgressDecisionClass::DenyPolicy,
            HandshakeOutcomeClass::PolicyBlocked,
            PlaneStatusClass::PolicyBlocked,
            PlaneStatusClass::Reachable,
            "inspect_policy_source",
            "Marketplace request was blocked by policy before transport.",
        ),
        event(
            EndpointClass::Docs,
            RouteSourceClass::MirrorOnly,
            EgressDecisionClass::AllowMirror,
            HandshakeOutcomeClass::NotAttemptedOffline,
            PlaneStatusClass::DegradedCachedOrMirrored,
            PlaneStatusClass::DegradedCachedOrMirrored,
            "refresh_mirror_when_online",
            "Docs request was served from signed mirror content.",
        ),
        event(
            EndpointClass::Ai,
            RouteSourceClass::System,
            EgressDecisionClass::DenyTrust,
            HandshakeOutcomeClass::TrustFailed,
            PlaneStatusClass::Reachable,
            PlaneStatusClass::UnreachableLocalContinuity,
            "revalidate_custom_ca_bundle",
            "AI provider handshake failed trust validation.",
        ),
        event(
            EndpointClass::Provider,
            RouteSourceClass::Pac,
            EgressDecisionClass::DenyTransport,
            HandshakeOutcomeClass::TransportFailed,
            PlaneStatusClass::Reachable,
            PlaneStatusClass::UnreachableLocalContinuity,
            "inspect_pac_or_proxy_transport",
            "Provider route resolved but transport failed.",
        ),
        event(
            EndpointClass::Remote,
            RouteSourceClass::System,
            EgressDecisionClass::DenyContractMismatch,
            HandshakeOutcomeClass::ContractMismatch,
            PlaneStatusClass::Reachable,
            PlaneStatusClass::PolicyBlocked,
            "select_supported_remote_profile",
            "Remote attach endpoint contract mismatch is explicit.",
        ),
        event(
            EndpointClass::Bootstrap,
            RouteSourceClass::Offline,
            EgressDecisionClass::OfflineDeferred,
            HandshakeOutcomeClass::NotAttemptedOffline,
            PlaneStatusClass::UnreachableLocalContinuity,
            PlaneStatusClass::DegradedCachedOrMirrored,
            "retry_after_signed_media_revalidation",
            "Bootstrap live egress is deferred while local signed media remains inspectable.",
        ),
    ]
}
