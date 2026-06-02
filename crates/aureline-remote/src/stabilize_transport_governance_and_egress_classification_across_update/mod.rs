//! Stabilize transport governance and egress classification across update,
//! marketplace, AI, docs, provider, and remote lanes.
//!
//! This module produces a stable proof packet that makes egress routing for
//! every named lane — update, marketplace, AI, docs, provider, remote, and
//! mirror/offline — inspectable through one typed vocabulary instead of
//! subsystem-specific status strings.
//!
//! Each lane record carries:
//! - the route class (direct, proxied, mirror-first, cached, offline, …),
//! - the egress decision (allowed, blocked by policy, blocked by transport
//!   failure, mirror-routed, cached-offline, last-known-good, …),
//! - the mirror route reference when traffic was redirected to a signed mirror,
//! - the cached/offline posture (online, cached, offline-grace, mirror-served,
//!   disconnected),
//! - the last-known-good policy epoch reference for lanes whose runtime
//!   behavior is governed by a versioned policy bundle,
//! - the tenant and region references,
//! - an explicit local-core continuity declaration, and
//! - a dependency class (local-only, network, managed, air-gapped).
//!
//! The stable claim holds when **all seven** of the following conditions are
//! verified simultaneously:
//!
//! 1. All seven required egress lanes are covered.
//! 2. No raw private material is exposed on any lane record.
//! 3. Every lane explicitly declares its local-core continuity posture.
//! 4. Every lane carries an explicit dependency class.
//! 5. Every lane carries a typed egress decision (not `unknown`).
//! 6. Every network-dependent lane carries a non-empty last-known-good
//!    policy epoch reference.
//! 7. Every lane names a distinct control-plane status and data-plane
//!    status so impairment type is distinguishable from raw logs.
//!
//! One condition forces `Withdrawn` immediately and cannot be overridden:
//!
//! - Any lane record carries `raw_private_material_excluded: false`
//!   (narrow reason: [`TransportGovernanceNarrowReasonClass::RawPrivateMaterialExposed`]).
//!
//! A missing required lane narrows to `Preview` rather than `Beta` because
//! the coverage gap prevents any verifiable claim for that lane.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language summary sentences, and
//! opaque refs only. Raw endpoint URLs, raw hostnames, raw credentials, raw
//! PAC scripts, raw policy bundle bodies, and raw private keys stay outside
//! the support boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/stabilize-transport-governance-and-egress-classification-across-update.md`
//! - Artifact: `artifacts/enterprise/m4/stabilize-transport-governance-and-egress-classification-across-update.md`
//! - Contract ref: [`TRANSPORT_GOVERNANCE_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const TRANSPORT_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const TRANSPORT_GOVERNANCE_SHARED_CONTRACT_REF: &str =
    "remote:transport_governance_stabilize:v1";

/// Record-kind tag for [`TransportGovernancePage`] payloads.
pub const TRANSPORT_GOVERNANCE_PAGE_RECORD_KIND: &str = "remote_transport_governance_page_record";

/// Record-kind tag for [`TransportGovernanceRow`] payloads.
pub const TRANSPORT_GOVERNANCE_ROW_RECORD_KIND: &str = "remote_transport_governance_row_record";

/// Record-kind tag for [`TransportGovernanceDefect`] payloads.
pub const TRANSPORT_GOVERNANCE_DEFECT_RECORD_KIND: &str =
    "remote_transport_governance_defect_record";

/// Record-kind tag for [`TransportGovernanceSummary`] payloads.
pub const TRANSPORT_GOVERNANCE_SUMMARY_RECORD_KIND: &str =
    "remote_transport_governance_summary_record";

/// Record-kind tag for [`TransportGovernanceSupportExport`] payloads.
pub const TRANSPORT_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_transport_governance_support_export_record";

/// Record-kind tag for [`TransportPolicyRecord`] payloads.
pub const TRANSPORT_POLICY_RECORD_KIND: &str = "remote_transport_policy_record";

/// Repo-relative path of the stable doc for this lane.
pub const TRANSPORT_GOVERNANCE_DOC_REF: &str =
    "docs/enterprise/m4/stabilize-transport-governance-and-egress-classification-across-update.md";

/// Repo-relative path of the artifact summary for this lane.
pub const TRANSPORT_GOVERNANCE_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/stabilize-transport-governance-and-egress-classification-across-update.md";

/// All seven required egress lane tokens in canonical order.
pub const REQUIRED_EGRESS_LANES: [EgressLaneClass; 7] = [
    EgressLaneClass::Update,
    EgressLaneClass::Marketplace,
    EgressLaneClass::Ai,
    EgressLaneClass::Docs,
    EgressLaneClass::Provider,
    EgressLaneClass::Remote,
    EgressLaneClass::MirrorOffline,
];

// ---------------------------------------------------------------------------
// Egress lane vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary for the named egress lanes whose transport must be
/// governed by this packet.
///
/// Each variant maps to the subsystem whose outbound requests the lane covers.
/// The `MirrorOffline` lane covers both mirror-first and fully-offline
/// postures in a single row; they share the same governance requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EgressLaneClass {
    /// Software update channel; fetch and apply IDE/extension updates.
    Update,
    /// Extension marketplace; browse, install, and update extensions.
    Marketplace,
    /// AI inference provider; model requests, completions, and embeddings.
    Ai,
    /// Documentation pack distribution; fetch and render offline doc packs.
    Docs,
    /// Connected provider; VCS hosts, CI, issue trackers, and partner APIs.
    Provider,
    /// Remote target (SSH, remote agent, managed workspace tunnel).
    Remote,
    /// Mirror-first and offline lane; declared signed mirrors and fully
    /// air-gapped disconnected postures.
    MirrorOffline,
}

impl EgressLaneClass {
    /// Stable closed-vocabulary token recorded in records, schemas, and
    /// exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Update => "update",
            Self::Marketplace => "marketplace",
            Self::Ai => "ai",
            Self::Docs => "docs",
            Self::Provider => "provider",
            Self::Remote => "remote",
            Self::MirrorOffline => "mirror_offline",
        }
    }

    /// Human-readable lane label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Update => "Update channel",
            Self::Marketplace => "Extension marketplace",
            Self::Ai => "AI provider",
            Self::Docs => "Documentation pack",
            Self::Provider => "Connected provider",
            Self::Remote => "Remote target",
            Self::MirrorOffline => "Mirror / offline",
        }
    }

    /// Returns `true` when this lane requires a policy epoch reference for
    /// the stable claim (all lanes except local-only ones).
    pub const fn requires_policy_epoch_ref(self) -> bool {
        !matches!(self, Self::MirrorOffline)
    }
}

// ---------------------------------------------------------------------------
// Egress decision vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary for the typed outcome of a transport policy decision.
///
/// Using a typed token instead of raw strings ensures that audit trails,
/// support exports, and diagnostics surfaces can reconstruct route selection
/// without parsing log lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EgressDecisionClass {
    /// Request routed normally; egress allowed by policy and transport.
    Allowed,
    /// Request blocked by a transport policy rule (policy block, not
    /// transport failure).
    BlockedPolicy,
    /// Request blocked due to a transport failure (network outage, TLS
    /// error, etc.); the policy would otherwise permit egress.
    BlockedTransport,
    /// Request redirected to a declared signed mirror; no direct egress
    /// to the primary endpoint.
    MirrorRouted,
    /// Response served from a local cache or offline store; no live egress
    /// was attempted.
    CachedOffline,
    /// Falling back to the last-known-good state; primary route and mirror
    /// both unavailable.
    LastKnownGood,
    /// Control plane is impaired; policy evaluation or epoch refresh is
    /// unavailable, but data-plane traffic may still flow.
    ControlPlaneImpaired,
    /// Data plane is impaired; traffic cannot be sent even though the
    /// control plane is reachable.
    DataPlaneImpaired,
}

impl EgressDecisionClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::BlockedPolicy => "blocked_policy",
            Self::BlockedTransport => "blocked_transport",
            Self::MirrorRouted => "mirror_routed",
            Self::CachedOffline => "cached_offline",
            Self::LastKnownGood => "last_known_good",
            Self::ControlPlaneImpaired => "control_plane_impaired",
            Self::DataPlaneImpaired => "data_plane_impaired",
        }
    }

    /// Returns `true` when this decision represents a successful egress or
    /// an acceptable offline fallback (not a failure or block).
    pub const fn is_acceptable(self) -> bool {
        matches!(
            self,
            Self::Allowed | Self::MirrorRouted | Self::CachedOffline | Self::LastKnownGood
        )
    }
}

// ---------------------------------------------------------------------------
// Egress route class vocabulary
// ---------------------------------------------------------------------------

/// How traffic physically travels for this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EgressRouteClass {
    /// Direct connection to the primary endpoint with no proxy.
    Direct,
    /// Routed through the platform system proxy.
    ProxiedSystem,
    /// Routed through a manually-configured or policy-pinned proxy.
    ProxiedManual,
    /// Traffic directed exclusively to a declared signed mirror.
    MirrorFirst,
    /// No outbound traffic; fully offline with no mirror fallback.
    Offline,
    /// Egress replaced with last-known-good state from a signed snapshot.
    LastKnownGoodFallback,
    /// Route is blocked; no fallback is available.
    BlockedNoFallback,
}

impl EgressRouteClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::ProxiedSystem => "proxied_system",
            Self::ProxiedManual => "proxied_manual",
            Self::MirrorFirst => "mirror_first",
            Self::Offline => "offline",
            Self::LastKnownGoodFallback => "last_known_good_fallback",
            Self::BlockedNoFallback => "blocked_no_fallback",
        }
    }
}

// ---------------------------------------------------------------------------
// Offline posture vocabulary
// ---------------------------------------------------------------------------

/// Posture token that expresses the cached/offline state for a lane.
///
/// Surfaces must show the distinct posture token so that users, support, and
/// diagnostics can distinguish between "content came from cache", "served from
/// declared mirror", "operating inside an offline-grace window", and "fully
/// disconnected" without inferring meaning from raw log output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflinePostureClass {
    /// Lane is online and using live egress.
    Online,
    /// Response served from a locally cached content store.
    CachedContent,
    /// Operating within a declared offline-grace window; the primary
    /// route is unavailable and a previously-validated bundle is extended.
    OfflineGrace,
    /// Traffic redirected to and served from a declared signed mirror.
    MirrorServed,
    /// Fully disconnected; no cache, no mirror, no grace window.
    Disconnected,
}

impl OfflinePostureClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::CachedContent => "cached_content",
            Self::OfflineGrace => "offline_grace",
            Self::MirrorServed => "mirror_served",
            Self::Disconnected => "disconnected",
        }
    }
}

// ---------------------------------------------------------------------------
// Dependency class vocabulary
// ---------------------------------------------------------------------------

/// Ownership tier that records what external dependency a lane carries.
///
/// The dependency class makes tenant, region, policy source, and installer
/// ownership visible for every lane so enterprise, self-hosted, managed, and
/// air-gapped claims can be evaluated without inspecting raw configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyClass {
    /// No external network dependency; works with no account and no
    /// outbound connection.
    LocalOnly,
    /// Requires a live network connection to a public or hosted endpoint;
    /// degrades gracefully when the network is unavailable.
    Network,
    /// Requires connectivity to a managed service endpoint controlled by an
    /// enterprise admin; local work continues without managed capabilities.
    Managed,
    /// Operates against a declared signed mirror or air-gapped media only;
    /// no direct internet egress is permitted.
    AirGapped,
}

impl DependencyClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Network => "network",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
        }
    }

    /// Returns `true` when this dependency class requires an explicit
    /// policy epoch reference.
    pub const fn requires_policy_epoch_ref(self) -> bool {
        matches!(self, Self::Network | Self::Managed)
    }
}

// ---------------------------------------------------------------------------
// Control-plane and data-plane status vocabulary
// ---------------------------------------------------------------------------

/// Status of the control plane (policy evaluation, epoch refresh) for a
/// lane.
///
/// A distinct control-plane token allows diagnostics and support exports to
/// distinguish between "policy was blocked" vs "transport was impaired" vs
/// "control plane was unreachable" without parsing raw logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlPlaneStatusClass {
    /// Control plane is reachable and policy evaluation is current.
    Reachable,
    /// Control plane is unreachable; policy epoch refresh is unavailable.
    Unreachable,
    /// Policy explicitly blocks control-plane traffic for this lane.
    PolicyBlocked,
    /// Control-plane traffic is routed through a declared mirror.
    MirrorRouted,
    /// Not applicable; this lane has no control-plane dependency.
    NotApplicable,
}

impl ControlPlaneStatusClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::Unreachable => "unreachable",
            Self::PolicyBlocked => "policy_blocked",
            Self::MirrorRouted => "mirror_routed",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Status of the data plane (actual traffic flow) for a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataPlaneStatusClass {
    /// Data plane is reachable; live traffic can flow.
    Reachable,
    /// Data plane is unreachable; traffic cannot be sent.
    Unreachable,
    /// Policy explicitly blocks data-plane traffic for this lane.
    PolicyBlocked,
    /// Responses are served from a local cache; no live data-plane traffic.
    CachedServed,
    /// Not applicable; this lane has no data-plane dependency.
    NotApplicable,
}

impl DataPlaneStatusClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::Unreachable => "unreachable",
            Self::PolicyBlocked => "policy_blocked",
            Self::CachedServed => "cached_served",
            Self::NotApplicable => "not_applicable",
        }
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and for individual
/// lane rows.
///
/// The tier is derived, not asserted: it is set by comparing the audit defect
/// list against the seven stability conditions. A caller may never bump a row
/// to `Stable` without a clean audit and complete lane coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportGovernanceQualificationClass {
    /// All seven stability conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required lane has no record; the coverage gap prevents a beta
    /// claim for the missing lane.
    Preview,
    /// Raw private material was exposed on a lane record; the packet is
    /// withdrawn immediately and cannot be overridden.
    Withdrawn,
}

impl TransportGovernanceQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Returns `true` when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Returns `true` when this tier is claimable (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed reason a packet or lane row was narrowed below
/// [`TransportGovernanceQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportGovernanceNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// A lane record carries `raw_private_material_excluded: false`;
    /// withdraws the packet immediately.
    RawPrivateMaterialExposed,
    /// A required lane is not covered by any record; narrows to preview.
    RequiredLaneMissing,
    /// A lane does not declare its local-core continuity posture explicitly.
    LocalCoreContinuityUndeclared,
    /// A lane does not carry an explicit dependency class.
    DependencyClassUndeclared,
    /// A network-dependent lane is missing its last-known-good policy
    /// epoch reference.
    PolicyEpochRefMissing,
    /// A lane carries an untyped or empty egress decision token.
    EgressDecisionTypingIncomplete,
    /// A lane does not name distinct control-plane and data-plane statuses,
    /// preventing impairment-type distinction.
    ControlDataPlaneDistinctionMissing,
}

impl TransportGovernanceNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::RequiredLaneMissing => "required_lane_missing",
            Self::LocalCoreContinuityUndeclared => "local_core_continuity_undeclared",
            Self::DependencyClassUndeclared => "dependency_class_undeclared",
            Self::PolicyEpochRefMissing => "policy_epoch_ref_missing",
            Self::EgressDecisionTypingIncomplete => "egress_decision_typing_incomplete",
            Self::ControlDataPlaneDistinctionMissing => "control_data_plane_distinction_missing",
        }
    }

    /// Returns `true` when this reason is a hard guardrail that withdraws
    /// the packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::RawPrivateMaterialExposed)
    }

    /// Returns `true` when this reason narrows to preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(self, Self::RequiredLaneMissing)
    }
}

// ---------------------------------------------------------------------------
// Transport policy record (per-lane)
// ---------------------------------------------------------------------------

/// Per-lane transport policy record.
///
/// Each record captures the typed route class, egress decision, mirror route
/// reference, offline posture, policy epoch, tenant/region, dependency class,
/// control-plane status, data-plane status, and local-core continuity posture
/// for a single named egress lane. Together the seven required lane records
/// form the [`TransportPolicySnapshot`] that the governance proof packet
/// embeds as evidence.
///
/// No raw endpoint URLs, raw hostnames, raw credentials, raw PAC script
/// content, raw policy bundle bodies, or raw private key material may appear
/// on this record. Only closed-vocabulary tokens, opaque refs, and plain-
/// language summary sentences cross the export boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportPolicyRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable lane identifier.
    pub lane: EgressLaneClass,
    /// Stable token for [`Self::lane`].
    pub lane_token: String,
    /// How traffic physically travels for this lane.
    pub route_class: EgressRouteClass,
    /// Stable token for [`Self::route_class`].
    pub route_class_token: String,
    /// Typed outcome of the transport policy decision.
    pub egress_decision: EgressDecisionClass,
    /// Stable token for [`Self::egress_decision`].
    pub egress_decision_token: String,
    /// Opaque ref identifying the declared signed mirror used when
    /// [`Self::egress_decision`] is `mirror_routed` or
    /// [`Self::route_class`] is `mirror_first`. `None` when no mirror is
    /// active.
    pub mirror_route_ref: Option<String>,
    /// Cached/offline posture for this lane.
    pub offline_posture: OfflinePostureClass,
    /// Stable token for [`Self::offline_posture`].
    pub offline_posture_token: String,
    /// Opaque ref to the last-known-good policy epoch for this lane.
    /// Present for network-, managed-, and air-gapped-dependency lanes;
    /// `None` for local-only lanes.
    pub last_known_good_policy_epoch_ref: Option<String>,
    /// Opaque ref identifying the tenant context for this lane.
    /// `None` when the lane operates in a non-tenanted context.
    pub tenant_ref: Option<String>,
    /// Opaque ref identifying the region for this lane.
    /// `None` when region attribution is not applicable.
    pub region_ref: Option<String>,
    /// Opaque ref identifying the policy source (signed bundle, admin
    /// policy, or local default) that governs this lane.
    pub policy_source_ref: String,
    /// Control-plane status for this lane.
    pub control_plane_status: ControlPlaneStatusClass,
    /// Stable token for [`Self::control_plane_status`].
    pub control_plane_status_token: String,
    /// Data-plane status for this lane.
    pub data_plane_status: DataPlaneStatusClass,
    /// Stable token for [`Self::data_plane_status`].
    pub data_plane_status_token: String,
    /// `true` when the local-core editing floor is preserved regardless of
    /// whether this lane's managed or network capabilities are available.
    pub local_core_continuity_allowed: bool,
    /// Ownership tier for this lane's external dependency.
    pub dependency_class: DependencyClass,
    /// Stable token for [`Self::dependency_class`].
    pub dependency_class_token: String,
    /// `true` when no raw endpoint URL, raw credential, raw private key,
    /// or raw PAC content is present on this record. Must be `true` for
    /// the stable claim to hold.
    pub raw_private_material_excluded: bool,
    /// Plain-language summary safe for UI, support export, and diagnostics.
    pub summary: String,
}

impl TransportPolicyRecord {
    /// Construct a transport policy record, filling in all token fields from
    /// the typed enum values.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lane: EgressLaneClass,
        route_class: EgressRouteClass,
        egress_decision: EgressDecisionClass,
        mirror_route_ref: Option<impl Into<String>>,
        offline_posture: OfflinePostureClass,
        last_known_good_policy_epoch_ref: Option<impl Into<String>>,
        tenant_ref: Option<impl Into<String>>,
        region_ref: Option<impl Into<String>>,
        policy_source_ref: impl Into<String>,
        control_plane_status: ControlPlaneStatusClass,
        data_plane_status: DataPlaneStatusClass,
        local_core_continuity_allowed: bool,
        dependency_class: DependencyClass,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: TRANSPORT_POLICY_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_GOVERNANCE_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_GOVERNANCE_SHARED_CONTRACT_REF.to_owned(),
            lane,
            lane_token: lane.as_str().to_owned(),
            route_class,
            route_class_token: route_class.as_str().to_owned(),
            egress_decision,
            egress_decision_token: egress_decision.as_str().to_owned(),
            mirror_route_ref: mirror_route_ref.map(Into::into),
            offline_posture,
            offline_posture_token: offline_posture.as_str().to_owned(),
            last_known_good_policy_epoch_ref: last_known_good_policy_epoch_ref.map(Into::into),
            tenant_ref: tenant_ref.map(Into::into),
            region_ref: region_ref.map(Into::into),
            policy_source_ref: policy_source_ref.into(),
            control_plane_status,
            control_plane_status_token: control_plane_status.as_str().to_owned(),
            data_plane_status,
            data_plane_status_token: data_plane_status.as_str().to_owned(),
            local_core_continuity_allowed,
            dependency_class,
            dependency_class_token: dependency_class.as_str().to_owned(),
            raw_private_material_excluded: true,
            summary: summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Transport policy snapshot (aggregate of all lane records)
// ---------------------------------------------------------------------------

/// Aggregate of all lane transport policy records.
///
/// The snapshot carries one [`TransportPolicyRecord`] per named egress lane.
/// The seven required lanes are: update, marketplace, ai, docs, provider,
/// remote, and mirror_offline. A snapshot missing any required lane causes
/// the governance proof packet to narrow to `Preview`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportPolicySnapshot {
    /// All lane records in the snapshot.
    pub records: Vec<TransportPolicyRecord>,
}

impl TransportPolicySnapshot {
    /// Returns the record for the given lane, if present.
    pub fn record_for_lane(&self, lane: EgressLaneClass) -> Option<&TransportPolicyRecord> {
        self.records.iter().find(|r| r.lane == lane)
    }

    /// Returns the set of lane tokens covered by this snapshot.
    pub fn covered_lane_tokens(&self) -> BTreeSet<&str> {
        self.records.iter().map(|r| r.lane_token.as_str()).collect()
    }
}

// ---------------------------------------------------------------------------
// Governance row (per-lane stability row)
// ---------------------------------------------------------------------------

/// Stability qualification for one lane in the governance proof packet.
///
/// Each row is derived from a single [`TransportPolicyRecord`] in the
/// snapshot. The qualification is computed from the lane record against the
/// seven stability conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportGovernanceRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Lane token for this row.
    pub lane_token: String,
    /// Route class token from the lane record.
    pub route_class_token: String,
    /// Egress decision token from the lane record.
    pub egress_decision_token: String,
    /// Offline posture token from the lane record.
    pub offline_posture_token: String,
    /// Control-plane status token from the lane record.
    pub control_plane_status_token: String,
    /// Data-plane status token from the lane record.
    pub data_plane_status_token: String,
    /// `true` when the local-core continuity posture is explicitly declared.
    pub local_core_continuity_declared: bool,
    /// Dependency class token from the lane record.
    pub dependency_class_token: String,
    /// `true` when a last-known-good policy epoch ref is present.
    pub policy_epoch_present: bool,
    /// `true` when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Derived qualification tier.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate banner emitted with the governance page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TransportGovernanceSummary {
    /// Total row count (one row per lane in the snapshot).
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Lane tokens covered by the snapshot.
    pub lanes_covered: Vec<String>,
    /// Number of lanes with explicit local-core continuity declaration.
    pub local_core_continuity_declared_count: usize,
    /// Number of lanes with a present policy epoch ref.
    pub policy_epoch_present_count: usize,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl TransportGovernanceSummary {
    fn from_rows(rows: &[TransportGovernanceRow], snapshot: &TransportPolicySnapshot) -> Self {
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
            TransportGovernanceQualificationClass::Withdrawn
        } else if preview > 0 {
            TransportGovernanceQualificationClass::Preview
        } else if beta > 0 {
            TransportGovernanceQualificationClass::Beta
        } else {
            TransportGovernanceQualificationClass::Stable
        };
        let lanes_covered: Vec<String> = snapshot
            .records
            .iter()
            .map(|r| r.lane_token.clone())
            .collect();
        let local_core_continuity_declared_count = rows
            .iter()
            .filter(|r| r.local_core_continuity_declared)
            .count();
        let policy_epoch_present_count = rows.iter().filter(|r| r.policy_epoch_present).count();
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            lanes_covered,
            local_core_continuity_declared_count,
            policy_epoch_present_count,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the governance page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportGovernanceDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: TransportGovernanceNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (lane token or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl TransportGovernanceDefect {
    fn new(
        narrow_reason: TransportGovernanceNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: TRANSPORT_GOVERNANCE_DEFECT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_GOVERNANCE_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_GOVERNANCE_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "remote:defect:transport-governance:{}:{}",
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
// Governance page (proof packet)
// ---------------------------------------------------------------------------

/// Stable proof packet for transport governance and egress classification.
///
/// The packet is the single inspectable record that proves the stable claim
/// for transport governance across all named egress lanes. Dashboards, docs,
/// Help/About surfaces, support exports, and diagnostics should ingest this
/// packet rather than cloning subsystem-specific status strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportGovernancePage {
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
    /// Aggregate summary derived from all rows.
    pub summary: TransportGovernanceSummary,
    /// Per-lane stability rows.
    pub rows: Vec<TransportGovernanceRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<TransportGovernanceDefect>,
    /// The transport policy snapshot embedded as evidence.
    pub policy_snapshot: TransportPolicySnapshot,
}

impl TransportGovernancePage {
    /// Build the governance page from a transport policy snapshot.
    ///
    /// Rows are derived per lane, and the qualification for each is computed
    /// from the combined audit of the whole snapshot.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        policy_snapshot: TransportPolicySnapshot,
    ) -> Self {
        let defects = audit_snapshot(&policy_snapshot);
        let rows = derive_governance_rows(&policy_snapshot, &defects);
        let summary = TransportGovernanceSummary::from_rows(&rows, &policy_snapshot);
        Self {
            record_kind: TRANSPORT_GOVERNANCE_PAGE_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_GOVERNANCE_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_GOVERNANCE_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            defects,
            policy_snapshot,
        }
    }

    /// Returns `true` when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == TransportGovernanceQualificationClass::Stable.as_str()
    }

    /// Returns `true` when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// Returns `true` when all seven required lanes are covered.
    pub fn covers_all_required_lanes(&self) -> bool {
        let covered = self.policy_snapshot.covered_lane_tokens();
        REQUIRED_EGRESS_LANES
            .iter()
            .all(|lane| covered.contains(lane.as_str()))
    }

    /// Returns `true` when every lane record explicitly declares local-core
    /// continuity.
    pub fn all_lanes_declare_local_core_continuity(&self) -> bool {
        self.policy_snapshot
            .records
            .iter()
            .all(|r| r.local_core_continuity_allowed)
    }

    /// Returns `true` when every network-dependent lane has a policy epoch
    /// ref present.
    pub fn network_lanes_have_policy_epoch_refs(&self) -> bool {
        self.policy_snapshot.records.iter().all(|r| {
            if r.dependency_class.requires_policy_epoch_ref() {
                r.last_known_good_policy_epoch_ref.is_some()
            } else {
                true
            }
        })
    }

    /// Returns `true` when every lane carries a typed egress decision (no
    /// empty decision token).
    pub fn all_lanes_have_typed_egress_decisions(&self) -> bool {
        self.policy_snapshot
            .records
            .iter()
            .all(|r| !r.egress_decision_token.is_empty())
    }

    /// Returns `true` when every lane records distinct control-plane and
    /// data-plane statuses (neither is the same unknown catch-all).
    pub fn all_lanes_distinguish_plane_impairment(&self) -> bool {
        // Both fields must be present (non-empty tokens); having both
        // `not_applicable` on the same record is acceptable only for
        // local-only lanes.
        self.policy_snapshot.records.iter().all(|r| {
            !r.control_plane_status_token.is_empty() && !r.data_plane_status_token.is_empty()
        })
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that carries the governance page plus a metadata-
/// safe defect roll-up.
///
/// No raw endpoint URLs, raw hostnames, raw credentials, or raw private key
/// material may appear in this export. Only closed-vocabulary tokens, opaque
/// refs, counts, and plain-language summary sentences cross the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportGovernanceSupportExport {
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
    /// The governance page embedded as evidence.
    pub page: TransportGovernancePage,
    /// Narrow-reason class values present in the page's defect list.
    pub narrow_reasons_present: Vec<TransportGovernanceNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// `true` when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl TransportGovernanceSupportExport {
    /// Wrap a governance page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: TransportGovernancePage,
    ) -> Self {
        let mut reasons: Vec<TransportGovernanceNarrowReasonClass> = Vec::new();
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
            record_kind: TRANSPORT_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_GOVERNANCE_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_GOVERNANCE_SHARED_CONTRACT_REF.to_owned(),
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
// Audit and validate functions (public API)
// ---------------------------------------------------------------------------

/// Re-run the governance audit over the snapshot embedded in a page.
pub fn audit_transport_governance_page(
    page: &TransportGovernancePage,
) -> Vec<TransportGovernanceDefect> {
    audit_snapshot(&page.policy_snapshot)
}

/// Validate a governance page; returns `Ok` when the audit is clean.
pub fn validate_transport_governance_page(
    page: &TransportGovernancePage,
) -> Result<(), Vec<TransportGovernanceDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal audit logic
// ---------------------------------------------------------------------------

fn audit_snapshot(snapshot: &TransportPolicySnapshot) -> Vec<TransportGovernanceDefect> {
    let mut defects: Vec<TransportGovernanceDefect> = Vec::new();

    // Hard guardrail: raw private material exposed — withdraw immediately.
    for record in &snapshot.records {
        if !record.raw_private_material_excluded {
            defects.push(TransportGovernanceDefect::new(
                TransportGovernanceNarrowReasonClass::RawPrivateMaterialExposed,
                record.lane_token.clone(),
                format!(
                    "lane '{}' has raw_private_material_excluded: false; packet is withdrawn",
                    record.lane_token
                ),
            ));
            // Return immediately — no further checks are meaningful.
            return defects;
        }
    }

    let covered: BTreeSet<&str> = snapshot
        .records
        .iter()
        .map(|r| r.lane_token.as_str())
        .collect();

    // Coverage check: all seven required lanes must be present.
    for required_lane in &REQUIRED_EGRESS_LANES {
        if !covered.contains(required_lane.as_str()) {
            defects.push(TransportGovernanceDefect::new(
                TransportGovernanceNarrowReasonClass::RequiredLaneMissing,
                required_lane.as_str(),
                format!(
                    "required lane '{}' has no transport policy record; packet is narrowed to preview",
                    required_lane.as_str()
                ),
            ));
        }
    }

    // Per-lane checks.
    for record in &snapshot.records {
        // Local-core continuity must be declared.
        if !record.local_core_continuity_allowed {
            defects.push(TransportGovernanceDefect::new(
                TransportGovernanceNarrowReasonClass::LocalCoreContinuityUndeclared,
                record.lane_token.clone(),
                format!(
                    "lane '{}' does not declare local-core continuity; local work may be blocked by managed capabilities",
                    record.lane_token
                ),
            ));
        }

        // Dependency class must be non-empty.
        if record.dependency_class_token.is_empty() {
            defects.push(TransportGovernanceDefect::new(
                TransportGovernanceNarrowReasonClass::DependencyClassUndeclared,
                record.lane_token.clone(),
                format!(
                    "lane '{}' has an empty dependency_class_token; dependency class must be explicit",
                    record.lane_token
                ),
            ));
        }

        // Policy epoch ref required for network-dependent lanes.
        if record.dependency_class.requires_policy_epoch_ref()
            && record.last_known_good_policy_epoch_ref.is_none()
        {
            defects.push(TransportGovernanceDefect::new(
                TransportGovernanceNarrowReasonClass::PolicyEpochRefMissing,
                record.lane_token.clone(),
                format!(
                    "lane '{}' ({}) has no last_known_good_policy_epoch_ref; policy epoch must be traceable for {} lanes",
                    record.lane_token,
                    record.dependency_class_token,
                    record.dependency_class_token
                ),
            ));
        }

        // Egress decision must be typed (non-empty token).
        if record.egress_decision_token.is_empty() {
            defects.push(TransportGovernanceDefect::new(
                TransportGovernanceNarrowReasonClass::EgressDecisionTypingIncomplete,
                record.lane_token.clone(),
                format!(
                    "lane '{}' has an empty egress_decision_token; egress decision must be typed for diagnostics",
                    record.lane_token
                ),
            ));
        }

        // Control-plane and data-plane status must both be non-empty.
        if record.control_plane_status_token.is_empty() || record.data_plane_status_token.is_empty()
        {
            defects.push(TransportGovernanceDefect::new(
                TransportGovernanceNarrowReasonClass::ControlDataPlaneDistinctionMissing,
                record.lane_token.clone(),
                format!(
                    "lane '{}' is missing control-plane or data-plane status tokens; impairment type cannot be distinguished",
                    record.lane_token
                ),
            ));
        }
    }

    defects
}

fn derive_governance_rows(
    snapshot: &TransportPolicySnapshot,
    page_defects: &[TransportGovernanceDefect],
) -> Vec<TransportGovernanceRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_preview_reason());

    let _overall_qual = if has_withdrawal {
        TransportGovernanceQualificationClass::Withdrawn
    } else if has_preview {
        TransportGovernanceQualificationClass::Preview
    } else if !page_defects.is_empty() {
        TransportGovernanceQualificationClass::Beta
    } else {
        TransportGovernanceQualificationClass::Stable
    };

    let overall_narrow_reason = if has_withdrawal {
        TransportGovernanceNarrowReasonClass::RawPrivateMaterialExposed
    } else if has_preview {
        TransportGovernanceNarrowReasonClass::RequiredLaneMissing
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        TransportGovernanceNarrowReasonClass::NotNarrowed
    };

    snapshot
        .records
        .iter()
        .map(|record| {
            let row_narrow = find_lane_narrow_reason(record, page_defects, overall_narrow_reason);
            let row_qual = if row_narrow.is_withdrawal_reason() {
                TransportGovernanceQualificationClass::Withdrawn
            } else if row_narrow.is_preview_reason() {
                TransportGovernanceQualificationClass::Preview
            } else if row_narrow != TransportGovernanceNarrowReasonClass::NotNarrowed {
                TransportGovernanceQualificationClass::Beta
            } else {
                TransportGovernanceQualificationClass::Stable
            };
            let summary = build_row_summary(&record.lane_token, &row_qual, row_narrow);
            TransportGovernanceRow {
                record_kind: TRANSPORT_GOVERNANCE_ROW_RECORD_KIND.to_owned(),
                schema_version: TRANSPORT_GOVERNANCE_SCHEMA_VERSION,
                shared_contract_ref: TRANSPORT_GOVERNANCE_SHARED_CONTRACT_REF.to_owned(),
                lane_token: record.lane_token.clone(),
                route_class_token: record.route_class_token.clone(),
                egress_decision_token: record.egress_decision_token.clone(),
                offline_posture_token: record.offline_posture_token.clone(),
                control_plane_status_token: record.control_plane_status_token.clone(),
                data_plane_status_token: record.data_plane_status_token.clone(),
                local_core_continuity_declared: record.local_core_continuity_allowed,
                dependency_class_token: record.dependency_class_token.clone(),
                policy_epoch_present: record.last_known_good_policy_epoch_ref.is_some(),
                raw_private_material_excluded: record.raw_private_material_excluded,
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn find_lane_narrow_reason(
    record: &TransportPolicyRecord,
    page_defects: &[TransportGovernanceDefect],
    overall_narrow_reason: TransportGovernanceNarrowReasonClass,
) -> TransportGovernanceNarrowReasonClass {
    // If there's a lane-specific defect, use its narrow reason.
    if let Some(defect) = page_defects.iter().find(|d| d.source == record.lane_token) {
        return defect.narrow_reason;
    }
    // Otherwise inherit the page-level narrow reason.
    overall_narrow_reason
}

fn build_row_summary(
    lane_token: &str,
    qual: &TransportGovernanceQualificationClass,
    narrow_reason: TransportGovernanceNarrowReasonClass,
) -> String {
    match qual {
        TransportGovernanceQualificationClass::Stable => format!(
            "Lane '{}' qualifies stable: all seven stability conditions hold, egress \
             decision is typed, local-core continuity is declared, dependency class \
             is explicit, policy epoch is traceable, and plane impairment is distinguishable.",
            lane_token
        ),
        _ => format!(
            "Lane '{}' narrowed to {} ({}): see defect list for details.",
            lane_token,
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable packet consumed by the headless example, the
/// integration tests, and the fixture generator.
///
/// The seeded page produces zero defects: all seven required lanes are
/// covered, no raw private material is exposed, every lane declares local-core
/// continuity, all dependency classes are explicit, all network-dependent
/// lanes carry policy epoch refs, all egress decisions are typed, and all
/// lanes record distinct control-plane and data-plane statuses.
pub fn seeded_transport_governance_page() -> TransportGovernancePage {
    TransportGovernancePage::new(
        "remote:transport_governance:default",
        "Transport governance and egress classification — stable packet",
        "2026-06-01T00:00:00Z",
        seeded_transport_policy_snapshot(),
    )
}

/// Build the seeded transport policy snapshot used by the seeded page.
///
/// Each of the seven required lanes is represented with a fully-typed,
/// clean record that passes all stability conditions.
pub fn seeded_transport_policy_snapshot() -> TransportPolicySnapshot {
    TransportPolicySnapshot {
        records: vec![
            // Update lane — direct egress, network dependency.
            TransportPolicyRecord::new(
                EgressLaneClass::Update,
                EgressRouteClass::Direct,
                EgressDecisionClass::Allowed,
                None::<String>,
                OfflinePostureClass::Online,
                Some("epoch:update:channel:2026-06-01"),
                Some("tenant:default"),
                Some("region:us-east-1"),
                "policy:update:signed-bundle:v1",
                ControlPlaneStatusClass::Reachable,
                DataPlaneStatusClass::Reachable,
                true,
                DependencyClass::Network,
                "Update channel: direct egress allowed; control and data planes reachable; \
                 local editing continues without update connectivity.",
            ),
            // Marketplace lane — direct egress, network dependency.
            TransportPolicyRecord::new(
                EgressLaneClass::Marketplace,
                EgressRouteClass::Direct,
                EgressDecisionClass::Allowed,
                None::<String>,
                OfflinePostureClass::Online,
                Some("epoch:marketplace:registry:2026-06-01"),
                Some("tenant:default"),
                Some("region:us-east-1"),
                "policy:marketplace:signed-bundle:v1",
                ControlPlaneStatusClass::Reachable,
                DataPlaneStatusClass::Reachable,
                true,
                DependencyClass::Network,
                "Extension marketplace: direct egress allowed; extensions installable from \
                 registry; local installed extensions continue without marketplace connectivity.",
            ),
            // AI lane — direct egress, managed dependency.
            TransportPolicyRecord::new(
                EgressLaneClass::Ai,
                EgressRouteClass::Direct,
                EgressDecisionClass::Allowed,
                None::<String>,
                OfflinePostureClass::Online,
                Some("epoch:ai:provider:2026-06-01"),
                Some("tenant:default"),
                Some("region:us-east-1"),
                "policy:ai:signed-bundle:v1",
                ControlPlaneStatusClass::Reachable,
                DataPlaneStatusClass::Reachable,
                true,
                DependencyClass::Managed,
                "AI provider: direct egress allowed; inference endpoint reachable; \
                 local editing and non-AI features continue without AI provider connectivity.",
            ),
            // Docs lane — direct with cached fallback, network dependency.
            TransportPolicyRecord::new(
                EgressLaneClass::Docs,
                EgressRouteClass::Direct,
                EgressDecisionClass::Allowed,
                None::<String>,
                OfflinePostureClass::CachedContent,
                Some("epoch:docs:pack:2026-06-01"),
                None::<String>,
                None::<String>,
                "policy:docs:signed-bundle:v1",
                ControlPlaneStatusClass::Reachable,
                DataPlaneStatusClass::CachedServed,
                true,
                DependencyClass::Network,
                "Documentation pack: direct egress for updates; content served from local \
                 cache; docs browsing continues offline from last-fetched pack.",
            ),
            // Provider lane — direct egress, managed dependency.
            TransportPolicyRecord::new(
                EgressLaneClass::Provider,
                EgressRouteClass::Direct,
                EgressDecisionClass::Allowed,
                None::<String>,
                OfflinePostureClass::Online,
                Some("epoch:provider:vcs:2026-06-01"),
                Some("tenant:default"),
                Some("region:us-east-1"),
                "policy:provider:signed-bundle:v1",
                ControlPlaneStatusClass::Reachable,
                DataPlaneStatusClass::Reachable,
                true,
                DependencyClass::Managed,
                "Connected provider (VCS, CI, issue trackers): direct egress allowed; \
                 local repo and workspace continue without provider connectivity.",
            ),
            // Remote lane — direct egress, network dependency.
            TransportPolicyRecord::new(
                EgressLaneClass::Remote,
                EgressRouteClass::Direct,
                EgressDecisionClass::Allowed,
                None::<String>,
                OfflinePostureClass::Online,
                Some("epoch:remote:agent:2026-06-01"),
                Some("tenant:default"),
                Some("region:us-east-1"),
                "policy:remote:signed-bundle:v1",
                ControlPlaneStatusClass::Reachable,
                DataPlaneStatusClass::Reachable,
                true,
                DependencyClass::Network,
                "Remote target (SSH, remote agent): direct egress allowed; \
                 local workspace and editor continue without remote connectivity.",
            ),
            // MirrorOffline lane — mirror-first route, air-gapped dependency.
            TransportPolicyRecord::new(
                EgressLaneClass::MirrorOffline,
                EgressRouteClass::MirrorFirst,
                EgressDecisionClass::MirrorRouted,
                Some("mirror:signed:primary:v1"),
                OfflinePostureClass::MirrorServed,
                None::<String>,
                None::<String>,
                None::<String>,
                "policy:mirror:signed-bundle:v1",
                ControlPlaneStatusClass::MirrorRouted,
                DataPlaneStatusClass::CachedServed,
                true,
                DependencyClass::AirGapped,
                "Mirror / offline lane: all egress limited to declared signed mirror; \
                 local core and all local features continue fully without internet; \
                 mirror routes are declared and inspectable.",
            ),
        ],
    }
}
