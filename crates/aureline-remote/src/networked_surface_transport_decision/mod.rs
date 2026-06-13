//! Resolve every new network-capable surface action through one shared
//! transport object model before any side effects leave the current boundary.
//!
//! The sibling [`crate::networked_surface_transport_matrix`] module *freezes*
//! the per-surface transport-policy, endpoint-class, route-choice, and
//! trust-material vocabulary. This module turns that frozen catalog into a
//! **runtime decision layer**: for every network-capable action a surface
//! takes, the shared transport-governance layer emits one inspectable
//! [`TransportDecision`] that resolves a [`TransportPolicySnapshot`] against an
//! [`EndpointDescriptor`] and records the typed [`TransportOutcomeClass`]
//! before the request crosses the boundary.
//!
//! The decision layer answers, per action and in one inspectable record:
//!
//! - **which endpoint** was contacted ([`EndpointDescriptor`]) — by origin
//!   scope and endpoint class plus an opaque handle, never a raw URL,
//! - **which policy** governed the action ([`TransportPolicySnapshot`]) — the
//!   resolved egress class, route choice, proxy-resolution tier (PAC → manual
//!   → system precedence), trust material, mirror/offline posture, and policy
//!   epoch ref,
//! - **what happened** ([`TransportOutcomeClass`]) — allowed, denied, served
//!   from a signed mirror, served from cache, deferred for idempotent replay,
//!   or offline-unavailable, with a typed [`DenialReasonClass`] when refused,
//!   and
//! - **that the action did not bypass governance** (`no_bypass`) — no surface
//!   ships a private proxy stack, direct CA override, undeclared public
//!   fallback, or hidden direct-connect retry.
//!
//! These decisions aggregate into a stable proof packet
//! ([`TransportDecisionLogPage`]) consumed by product UI, CLI/headless output,
//! diagnostics, support exports, and admin/audit surfaces, so route choices and
//! endpoint descriptors are inspectable rather than reconstructed from raw URLs
//! or logs.
//!
//! The stable claim holds when **all** of the following conditions are verified
//! simultaneously for every covered surface decision:
//!
//! 1. All required surfaces have a decision.
//! 2. No raw private material is present on any record.
//! 3. Every decision resolved through the shared transport-governance layer
//!    (`no_bypass: true`).
//! 4. No decision permits a silent fall-through from a confined egress class to
//!    the public internet.
//! 5. Any offline-deferred decision queues only an explicitly idempotent
//!    action.
//! 6. Every decision preserves local-core continuity.
//! 7. Every denied decision carries a typed denial reason.
//! 8. Every decision carries a non-empty trust-proof ref.
//! 9. Every decision whose egress class requires a policy epoch carries a
//!    last-known-good policy epoch ref.
//! 10. Every decision's trust proof is fresh (or stale only within an accepted
//!     grace window).
//! 11. Every decision carries fully-typed endpoint, egress, route, proxy, and
//!     outcome classifications.
//!
//! Four conditions force [`DecisionQualificationClass::Withdrawn`] immediately
//! and cannot be overridden: raw private material exposed, a bypass of the
//! shared governance layer, a silent public fall-through, or a non-idempotent
//! action queued for offline replay. A missing required surface narrows to
//! [`DecisionQualificationClass::Preview`]; the remaining gaps narrow to
//! `Beta`, which lets release and support tooling automatically narrow stale or
//! under-qualified rows before publication.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language summary sentences, and
//! opaque refs only. Raw endpoint URLs, raw hostnames, raw ports, raw
//! credentials, raw bearer/session tokens, raw cookie jars, raw private
//! certificate bytes, raw SSH private material, and raw PAC bodies stay outside
//! the support boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/network/networked-surface-transport-decision.md`
//! - Artifact: `artifacts/network/networked-surface-transport-decision.md`
//! - Schema: `schemas/network/networked_surface_transport_decision.schema.json`
//! - Contract ref: [`TRANSPORT_DECISION_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::networked_surface_transport_matrix::{
    AuthPostureClass, DenialReasonClass, EgressClass, EndpointClass, MirrorOfflineBehaviorClass,
    OriginScopeClass, ProofFreshnessClass, RouteChoiceClass, SurfaceClass, TrustMaterialClass,
    REQUIRED_SURFACES,
};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const TRANSPORT_DECISION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const TRANSPORT_DECISION_SHARED_CONTRACT_REF: &str =
    "remote:networked_surface_transport_decision:v1";

/// Record-kind tag for [`TransportDecisionLogPage`] payloads.
pub const TRANSPORT_DECISION_PAGE_RECORD_KIND: &str =
    "remote_networked_surface_transport_decision_page_record";

/// Record-kind tag for [`EndpointDescriptor`] payloads.
pub const TRANSPORT_DECISION_ENDPOINT_RECORD_KIND: &str =
    "remote_networked_surface_transport_decision_endpoint_descriptor_record";

/// Record-kind tag for [`TransportPolicySnapshot`] payloads.
pub const TRANSPORT_DECISION_POLICY_SNAPSHOT_RECORD_KIND: &str =
    "remote_networked_surface_transport_decision_policy_snapshot_record";

/// Record-kind tag for [`TransportDecision`] payloads.
pub const TRANSPORT_DECISION_RECORD_KIND: &str =
    "remote_networked_surface_transport_decision_record";

/// Record-kind tag for [`TransportDecisionRow`] payloads.
pub const TRANSPORT_DECISION_ROW_RECORD_KIND: &str =
    "remote_networked_surface_transport_decision_row_record";

/// Record-kind tag for [`TransportDecisionDefect`] payloads.
pub const TRANSPORT_DECISION_DEFECT_RECORD_KIND: &str =
    "remote_networked_surface_transport_decision_defect_record";

/// Record-kind tag for [`TransportDecisionSummary`] payloads.
pub const TRANSPORT_DECISION_SUMMARY_RECORD_KIND: &str =
    "remote_networked_surface_transport_decision_summary_record";

/// Record-kind tag for [`TransportDecisionSupportExport`] payloads.
pub const TRANSPORT_DECISION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_networked_surface_transport_decision_support_export_record";

/// Repo-relative path of the stable doc for this decision log.
pub const TRANSPORT_DECISION_DOC_REF: &str = "docs/network/networked-surface-transport-decision.md";

/// Repo-relative path of the artifact summary for this decision log.
pub const TRANSPORT_DECISION_ARTIFACT_REF: &str =
    "artifacts/network/networked-surface-transport-decision.md";

/// Repo-relative ref to the canonical evidence index this decision log binds
/// into for the closeout certification lane.
pub const TRANSPORT_DECISION_EVIDENCE_INDEX_REF: &str =
    "artifacts/release/m5/xt12-evidence-index.md";

// ---------------------------------------------------------------------------
// Proxy-resolution-source vocabulary
// ---------------------------------------------------------------------------

/// Which proxy-resolution tier selected the route for a decision.
///
/// Proxy resolution precedence is PAC → manual → system: a PAC-resolved route
/// wins over a manually-pinned proxy, which wins over the platform system
/// proxy. Recording the winning tier (rather than just the resulting route)
/// keeps the precedence inspectable so no surface can hide a private proxy
/// stack or a direct-connect retry outside this vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyResolutionSourceClass {
    /// No proxy participated; a direct connection was selected.
    DirectNoProxy,
    /// The route was selected by a PAC script (highest precedence).
    PacResolved,
    /// The route was selected by a manually-configured or policy-pinned proxy.
    ManualProxy,
    /// The route was selected by the platform system proxy (lowest precedence).
    SystemProxy,
    /// The route was directed to a declared signed mirror; no proxy and no
    /// public-internet egress participate.
    MirrorRoute,
    /// No route was resolved; the surface is offline.
    OfflineNoRoute,
}

impl ProxyResolutionSourceClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectNoProxy => "direct_no_proxy",
            Self::PacResolved => "pac_resolved",
            Self::ManualProxy => "manual_proxy",
            Self::SystemProxy => "system_proxy",
            Self::MirrorRoute => "mirror_route",
            Self::OfflineNoRoute => "offline_no_route",
        }
    }

    /// Returns the route choice this resolution tier is consistent with.
    pub const fn consistent_route_choice(self) -> RouteChoiceClass {
        match self {
            Self::DirectNoProxy => RouteChoiceClass::Direct,
            Self::PacResolved => RouteChoiceClass::PacResolved,
            Self::ManualProxy => RouteChoiceClass::ManualProxy,
            Self::SystemProxy => RouteChoiceClass::SystemProxy,
            Self::MirrorRoute => RouteChoiceClass::MirrorFirst,
            Self::OfflineNoRoute => RouteChoiceClass::Offline,
        }
    }
}

// ---------------------------------------------------------------------------
// Transport-outcome vocabulary
// ---------------------------------------------------------------------------

/// What the shared transport-governance layer decided for one action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportOutcomeClass {
    /// Egress was permitted and the request proceeded over the chosen route.
    Allowed,
    /// The request was refused; a typed [`DenialReasonClass`] is recorded.
    Denied,
    /// Served from the declared signed mirror; no public egress was attempted.
    ServedFromMirror,
    /// Served from previously-cached content; no live egress was attempted.
    ServedFromCache,
    /// The action was queued for offline-deferred replay (idempotent only).
    OfflineDeferred,
    /// No route was available; the surface is offline and local-core continues.
    OfflineUnavailable,
}

impl TransportOutcomeClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Denied => "denied",
            Self::ServedFromMirror => "served_from_mirror",
            Self::ServedFromCache => "served_from_cache",
            Self::OfflineDeferred => "offline_deferred",
            Self::OfflineUnavailable => "offline_unavailable",
        }
    }

    /// Returns `true` when this outcome must carry a typed denial reason.
    pub const fn requires_denial_reason(self) -> bool {
        matches!(self, Self::Denied)
    }

    /// Returns `true` when this outcome defers the action for later replay.
    pub const fn is_offline_deferred(self) -> bool {
        matches!(self, Self::OfflineDeferred)
    }

    /// Returns `true` when this outcome let live traffic reach the network.
    pub const fn reached_network(self) -> bool {
        matches!(self, Self::Allowed)
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall decision log and for
/// individual decision rows.
///
/// The tier is derived, not asserted: it is computed by comparing the audit
/// defect list against the stability conditions. A caller may never bump a row
/// to `Stable` without a clean audit and complete surface coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionQualificationClass {
    /// All stability conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required surface has no decision; the coverage gap prevents a beta
    /// claim for the missing surface.
    Preview,
    /// A hard guardrail was violated; the packet is withdrawn immediately and
    /// cannot be overridden.
    Withdrawn,
}

impl DecisionQualificationClass {
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

/// Typed reason a packet or decision row was narrowed below
/// [`DecisionQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionNarrowReasonClass {
    /// No narrowing — the decision qualifies stable.
    NotNarrowed,
    /// A record carries `raw_private_material_excluded: false`; withdraws the
    /// packet immediately.
    RawPrivateMaterialExposed,
    /// A decision resolved outside the shared transport-governance layer
    /// (`no_bypass: false`); withdraws the packet immediately.
    BypassedSharedGovernance,
    /// A decision permits a silent fall-through to the public internet from a
    /// confined egress class; withdraws the packet immediately.
    SilentPublicFallbackResolved,
    /// A decision queued a non-idempotent action for offline replay; withdraws
    /// the packet immediately.
    NonIdempotentReplayQueued,
    /// A required surface has no decision; narrows to preview.
    RequiredSurfaceMissing,
    /// A denied decision carries no typed denial reason.
    DenialReasonMissing,
    /// A decision does not preserve local-core continuity.
    LocalCoreContinuityNotPreserved,
    /// A decision does not carry a trust-proof ref.
    TrustProofMissing,
    /// A decision whose egress class requires a policy epoch is missing the
    /// last-known-good policy epoch ref.
    PolicyEpochRefMissing,
    /// A decision is missing one of its endpoint/egress/route/proxy/outcome
    /// classifications.
    TransportClassificationIncomplete,
    /// A decision's trust proof has expired beyond its freshness window.
    ProofStaleBeyondWindow,
}

impl DecisionNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::BypassedSharedGovernance => "bypassed_shared_governance",
            Self::SilentPublicFallbackResolved => "silent_public_fallback_resolved",
            Self::NonIdempotentReplayQueued => "non_idempotent_replay_queued",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::DenialReasonMissing => "denial_reason_missing",
            Self::LocalCoreContinuityNotPreserved => "local_core_continuity_not_preserved",
            Self::TrustProofMissing => "trust_proof_missing",
            Self::PolicyEpochRefMissing => "policy_epoch_ref_missing",
            Self::TransportClassificationIncomplete => "transport_classification_incomplete",
            Self::ProofStaleBeyondWindow => "proof_stale_beyond_window",
        }
    }

    /// Returns `true` when this reason is a hard guardrail that withdraws the
    /// packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::RawPrivateMaterialExposed
                | Self::BypassedSharedGovernance
                | Self::SilentPublicFallbackResolved
                | Self::NonIdempotentReplayQueued
        )
    }

    /// Returns `true` when this reason narrows to preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(self, Self::RequiredSurfaceMissing)
    }
}

// ---------------------------------------------------------------------------
// Endpoint descriptor
// ---------------------------------------------------------------------------

/// Inspectable descriptor of the endpoint a decision contacts.
///
/// The descriptor names the endpoint by origin scope and endpoint class plus an
/// opaque handle, so product, CLI, and support surfaces can identify the
/// endpoint without reconstructing it from a raw URL. No raw endpoint URL, raw
/// hostname, raw IP, or raw port ever appears on this record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Surface this endpoint belongs to.
    pub surface: SurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Origin ownership scope for this endpoint.
    pub origin_scope: OriginScopeClass,
    /// Stable token for [`Self::origin_scope`].
    pub origin_scope_token: String,
    /// Endpoint class this descriptor identifies.
    pub endpoint_class: EndpointClass,
    /// Stable token for [`Self::endpoint_class`].
    pub endpoint_class_token: String,
    /// Opaque handle identifying the endpoint. Never a raw URL/host/IP/port.
    pub endpoint_handle: String,
    /// Human-readable endpoint label safe for UI and exports.
    pub endpoint_label: String,
    /// `true` when no raw endpoint URL/host/IP/port is present on this record.
    pub raw_private_material_excluded: bool,
}

impl EndpointDescriptor {
    /// Construct an endpoint descriptor, filling in token fields from the typed
    /// enum values.
    pub fn new(
        surface: SurfaceClass,
        origin_scope: OriginScopeClass,
        endpoint_class: EndpointClass,
        endpoint_handle: impl Into<String>,
        endpoint_label: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: TRANSPORT_DECISION_ENDPOINT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_DECISION_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_DECISION_SHARED_CONTRACT_REF.to_owned(),
            surface,
            surface_token: surface.as_str().to_owned(),
            origin_scope,
            origin_scope_token: origin_scope.as_str().to_owned(),
            endpoint_class,
            endpoint_class_token: endpoint_class.as_str().to_owned(),
            endpoint_handle: endpoint_handle.into(),
            endpoint_label: endpoint_label.into(),
            raw_private_material_excluded: true,
        }
    }

    /// Returns `true` when every classification token is present.
    pub fn is_fully_classified(&self) -> bool {
        !self.surface_token.is_empty()
            && !self.origin_scope_token.is_empty()
            && !self.endpoint_class_token.is_empty()
            && !self.endpoint_handle.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Transport policy snapshot
// ---------------------------------------------------------------------------

/// The resolved transport policy that governed one decision.
///
/// The snapshot captures the policy the shared transport-governance layer
/// resolved at decision time: the egress class enforced, the route choice and
/// the proxy-resolution tier that selected it (PAC → manual → system), the
/// trust material anchoring host proof, the mirror/offline posture, and the
/// last-known-good policy epoch. Only closed-vocabulary tokens and opaque refs
/// cross the boundary; no raw PAC body, raw proxy host, raw CA bundle, or raw
/// certificate material appears.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportPolicySnapshot {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque id for this snapshot.
    pub snapshot_id: String,
    /// UTC instant the policy snapshot was captured.
    pub captured_at: String,
    /// Surface this snapshot governs.
    pub surface: SurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Egress class enforced for this decision.
    pub egress_class: EgressClass,
    /// Stable token for [`Self::egress_class`].
    pub egress_class_token: String,
    /// Route choice resolved for this decision.
    pub route_choice: RouteChoiceClass,
    /// Stable token for [`Self::route_choice`].
    pub route_choice_token: String,
    /// Proxy-resolution tier that selected the route.
    pub proxy_resolution_source: ProxyResolutionSourceClass,
    /// Stable token for [`Self::proxy_resolution_source`].
    pub proxy_resolution_source_token: String,
    /// Trust input anchoring host proof for this decision.
    pub trust_material: TrustMaterialClass,
    /// Stable token for [`Self::trust_material`].
    pub trust_material_token: String,
    /// Opaque ref to the trust proof evidence. Required; an empty ref narrows
    /// the decision to beta.
    pub trust_proof_ref: String,
    /// Freshness of the trust proof.
    pub trust_proof_freshness: ProofFreshnessClass,
    /// Stable token for [`Self::trust_proof_freshness`].
    pub trust_proof_freshness_token: String,
    /// Mirror/offline behavior when the primary route is unavailable.
    pub mirror_offline_behavior: MirrorOfflineBehaviorClass,
    /// Stable token for [`Self::mirror_offline_behavior`].
    pub mirror_offline_behavior_token: String,
    /// Opaque ref to the last-known-good policy epoch governing this decision.
    /// Present for egress classes that require it; `None` otherwise.
    pub policy_epoch_ref: Option<String>,
    /// `true` when no silent fall-through to the public internet is permitted
    /// from a confined egress class. Must be `true` for the stable claim.
    pub no_silent_public_fallback: bool,
    /// `true` when local-core editing continues regardless of this surface's
    /// availability.
    pub local_core_continuity_preserved: bool,
    /// `true` when no raw PAC body, raw proxy host, raw CA bundle, or raw
    /// certificate material is present on this record.
    pub raw_private_material_excluded: bool,
}

impl TransportPolicySnapshot {
    /// Construct a policy snapshot, filling in token fields from the typed enum
    /// values.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        snapshot_id: impl Into<String>,
        captured_at: impl Into<String>,
        surface: SurfaceClass,
        egress_class: EgressClass,
        route_choice: RouteChoiceClass,
        proxy_resolution_source: ProxyResolutionSourceClass,
        trust_material: TrustMaterialClass,
        trust_proof_ref: impl Into<String>,
        trust_proof_freshness: ProofFreshnessClass,
        mirror_offline_behavior: MirrorOfflineBehaviorClass,
        policy_epoch_ref: Option<impl Into<String>>,
        no_silent_public_fallback: bool,
        local_core_continuity_preserved: bool,
    ) -> Self {
        Self {
            record_kind: TRANSPORT_DECISION_POLICY_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_DECISION_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_DECISION_SHARED_CONTRACT_REF.to_owned(),
            snapshot_id: snapshot_id.into(),
            captured_at: captured_at.into(),
            surface,
            surface_token: surface.as_str().to_owned(),
            egress_class,
            egress_class_token: egress_class.as_str().to_owned(),
            route_choice,
            route_choice_token: route_choice.as_str().to_owned(),
            proxy_resolution_source,
            proxy_resolution_source_token: proxy_resolution_source.as_str().to_owned(),
            trust_material,
            trust_material_token: trust_material.as_str().to_owned(),
            trust_proof_ref: trust_proof_ref.into(),
            trust_proof_freshness,
            trust_proof_freshness_token: trust_proof_freshness.as_str().to_owned(),
            mirror_offline_behavior,
            mirror_offline_behavior_token: mirror_offline_behavior.as_str().to_owned(),
            policy_epoch_ref: policy_epoch_ref.map(Into::into),
            no_silent_public_fallback,
            local_core_continuity_preserved,
            raw_private_material_excluded: true,
        }
    }

    /// Returns `true` when the route choice agrees with the proxy-resolution
    /// tier and every classification token is present.
    pub fn is_route_consistent(&self) -> bool {
        self.proxy_resolution_source.consistent_route_choice() == self.route_choice
            && !self.egress_class_token.is_empty()
            && !self.route_choice_token.is_empty()
            && !self.proxy_resolution_source_token.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Transport decision (per-action)
// ---------------------------------------------------------------------------

/// One inspectable transport decision emitted before a network-capable action
/// leaves the current boundary.
///
/// A decision binds an [`EndpointDescriptor`] and a [`TransportPolicySnapshot`]
/// to a typed [`TransportOutcomeClass`], the handle-only auth posture that was
/// presented, the typed denial reason when refused, and the guardrail flags
/// that prove the action resolved through the shared transport-governance layer
/// (`no_bypass`) and queued only idempotent actions for replay.
///
/// No raw endpoint URLs, raw credentials, raw bearer/session tokens, raw cookie
/// jars, raw private certificate bytes, raw SSH private material, or raw PAC
/// bodies may appear on this record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportDecision {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque id for this decision.
    pub decision_id: String,
    /// UTC instant the decision was made.
    pub decided_at: String,
    /// Surface this decision belongs to.
    pub surface: SurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Endpoint the decision contacts.
    pub endpoint: EndpointDescriptor,
    /// Resolved policy that governed the decision.
    pub policy: TransportPolicySnapshot,
    /// Handle-only auth posture presented to the endpoint.
    pub auth_posture: AuthPostureClass,
    /// Stable token for [`Self::auth_posture`].
    pub auth_posture_token: String,
    /// Typed outcome of the decision.
    pub outcome: TransportOutcomeClass,
    /// Stable token for [`Self::outcome`].
    pub outcome_token: String,
    /// Typed denial reason when the outcome is a denial; `None` otherwise.
    pub denial_reason: Option<DenialReasonClass>,
    /// Stable token for [`Self::denial_reason`]; empty when `None`.
    pub denial_reason_token: String,
    /// `true` when the action this decision governs is idempotent.
    pub action_is_idempotent: bool,
    /// `true` when the decision resolved through the shared
    /// transport-governance layer and did not ship a private proxy stack,
    /// direct CA override, undeclared public fallback, or direct-connect retry.
    pub no_bypass: bool,
    /// Plain-language summary safe for UI, support export, and diagnostics.
    pub summary: String,
    /// `true` when no raw private material is present on this record.
    pub raw_private_material_excluded: bool,
}

impl TransportDecision {
    /// Construct a transport decision, filling in token fields from the typed
    /// enum values.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        decision_id: impl Into<String>,
        decided_at: impl Into<String>,
        endpoint: EndpointDescriptor,
        policy: TransportPolicySnapshot,
        auth_posture: AuthPostureClass,
        outcome: TransportOutcomeClass,
        denial_reason: Option<DenialReasonClass>,
        action_is_idempotent: bool,
        no_bypass: bool,
        summary: impl Into<String>,
    ) -> Self {
        let surface = endpoint.surface;
        let denial_reason_token = denial_reason
            .map(|d| d.as_str().to_owned())
            .unwrap_or_default();
        Self {
            record_kind: TRANSPORT_DECISION_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_DECISION_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_DECISION_SHARED_CONTRACT_REF.to_owned(),
            decision_id: decision_id.into(),
            decided_at: decided_at.into(),
            surface,
            surface_token: surface.as_str().to_owned(),
            endpoint,
            policy,
            auth_posture,
            auth_posture_token: auth_posture.as_str().to_owned(),
            outcome,
            outcome_token: outcome.as_str().to_owned(),
            denial_reason,
            denial_reason_token,
            action_is_idempotent,
            no_bypass,
            summary: summary.into(),
            raw_private_material_excluded: true,
        }
    }

    /// Returns `true` when no record on this decision exposes raw material.
    pub fn raw_material_excluded(&self) -> bool {
        self.raw_private_material_excluded
            && self.endpoint.raw_private_material_excluded
            && self.policy.raw_private_material_excluded
    }
}

// ---------------------------------------------------------------------------
// Decision snapshot (aggregate of all decisions)
// ---------------------------------------------------------------------------

/// Aggregate of all transport decisions for the covered surfaces.
///
/// The snapshot carries one [`TransportDecision`] per network-capable surface.
/// A snapshot missing any required surface causes the decision log page to
/// narrow to `Preview`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportDecisionSnapshot {
    /// All decision records in the snapshot.
    pub decisions: Vec<TransportDecision>,
}

impl TransportDecisionSnapshot {
    /// Returns the decision for the given surface, if present.
    pub fn decision_for_surface(&self, surface: SurfaceClass) -> Option<&TransportDecision> {
        self.decisions.iter().find(|d| d.surface == surface)
    }

    /// Returns the set of surface tokens covered by this snapshot.
    pub fn covered_surface_tokens(&self) -> BTreeSet<&str> {
        self.decisions
            .iter()
            .map(|d| d.surface_token.as_str())
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Decision row (per-decision stability row)
// ---------------------------------------------------------------------------

/// Stability qualification for one decision in the decision log page.
///
/// Each row is derived from a single [`TransportDecision`] in the snapshot. The
/// qualification is computed from the decision against the stability
/// conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportDecisionRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Decision id for this row.
    pub decision_id: String,
    /// Surface token for this row.
    pub surface_token: String,
    /// Origin scope token from the endpoint descriptor.
    pub origin_scope_token: String,
    /// Endpoint class token from the endpoint descriptor.
    pub endpoint_class_token: String,
    /// Egress class token from the policy snapshot.
    pub egress_class_token: String,
    /// Route choice token from the policy snapshot.
    pub route_choice_token: String,
    /// Proxy resolution source token from the policy snapshot.
    pub proxy_resolution_source_token: String,
    /// Auth posture token from the decision.
    pub auth_posture_token: String,
    /// Trust material token from the policy snapshot.
    pub trust_material_token: String,
    /// Mirror/offline behavior token from the policy snapshot.
    pub mirror_offline_behavior_token: String,
    /// Outcome token from the decision.
    pub outcome_token: String,
    /// Denial reason token from the decision; empty when not denied.
    pub denial_reason_token: String,
    /// `true` when the decision resolved through the shared governance layer.
    pub no_bypass: bool,
    /// `true` when no silent public fall-through is permitted.
    pub no_silent_public_fallback: bool,
    /// `true` when the deferred action is idempotent (always `true` when not
    /// offline-deferred).
    pub replay_idempotent_only: bool,
    /// `true` when local-core continuity is preserved.
    pub local_core_continuity_preserved: bool,
    /// `true` when a policy epoch ref is present.
    pub policy_epoch_present: bool,
    /// Trust proof freshness token from the policy snapshot.
    pub proof_freshness_token: String,
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

/// Aggregate banner emitted with the decision log page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TransportDecisionSummary {
    /// Total row count (one row per decision in the snapshot).
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Surface tokens covered by the snapshot.
    pub surfaces_covered: Vec<String>,
    /// Number of decisions that resolved through the shared governance layer.
    pub no_bypass_count: usize,
    /// Number of decisions that preserve local-core continuity.
    pub local_core_continuity_preserved_count: usize,
    /// Number of decisions with a present policy epoch ref.
    pub policy_epoch_present_count: usize,
    /// Number of decisions with a fresh (or grace-window) trust proof.
    pub usable_proof_count: usize,
    /// Decision counts by outcome token.
    pub outcome_counts: BTreeMap<String, usize>,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl TransportDecisionSummary {
    fn from_rows(
        rows: &[TransportDecisionRow],
        snapshot: &TransportDecisionSnapshot,
        defects: &[TransportDecisionDefect],
    ) -> Self {
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
        // The overall tier is derived from the full defect list, not just the
        // per-row qualifications, so a missing required surface (which has no
        // row) still narrows the page to preview.
        let has_withdrawal = defects
            .iter()
            .any(|d| d.narrow_reason.is_withdrawal_reason());
        let has_preview = defects.iter().any(|d| d.narrow_reason.is_preview_reason());
        let overall = if has_withdrawal || withdrawn > 0 {
            DecisionQualificationClass::Withdrawn
        } else if has_preview || preview > 0 {
            DecisionQualificationClass::Preview
        } else if !defects.is_empty() || beta > 0 {
            DecisionQualificationClass::Beta
        } else {
            DecisionQualificationClass::Stable
        };
        let surfaces_covered: Vec<String> = snapshot
            .decisions
            .iter()
            .map(|d| d.surface_token.clone())
            .collect();
        let no_bypass_count = snapshot.decisions.iter().filter(|d| d.no_bypass).count();
        let local_core_continuity_preserved_count = snapshot
            .decisions
            .iter()
            .filter(|d| d.policy.local_core_continuity_preserved)
            .count();
        let policy_epoch_present_count = snapshot
            .decisions
            .iter()
            .filter(|d| d.policy.policy_epoch_ref.is_some())
            .count();
        let usable_proof_count = snapshot
            .decisions
            .iter()
            .filter(|d| d.policy.trust_proof_freshness.is_usable())
            .count();
        let mut outcome_counts: BTreeMap<String, usize> = BTreeMap::new();
        for decision in &snapshot.decisions {
            *outcome_counts
                .entry(decision.outcome_token.clone())
                .or_insert(0) += 1;
        }
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            surfaces_covered,
            no_bypass_count,
            local_core_continuity_preserved_count,
            policy_epoch_present_count,
            usable_proof_count,
            outcome_counts,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the decision log page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportDecisionDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: DecisionNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (surface token or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl TransportDecisionDefect {
    fn new(
        narrow_reason: DecisionNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: TRANSPORT_DECISION_DEFECT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_DECISION_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_DECISION_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "remote:defect:networked-surface-transport-decision:{}:{}",
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
// Decision log page (proof packet)
// ---------------------------------------------------------------------------

/// Stable decision-log proof packet for the network-capable surfaces.
///
/// The packet is the single inspectable record that proves every claimed M5
/// network-capable surface resolves its actions through the shared transport
/// object model. Dashboards, docs, Help/About surfaces, CLI/headless output,
/// support exports, release tooling, and diagnostics should ingest this packet
/// rather than reconstructing route choices from raw URLs or logs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportDecisionLogPage {
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
    /// Repo-relative ref to the canonical evidence index this log binds into.
    pub evidence_index_ref: String,
    /// Aggregate summary derived from all rows.
    pub summary: TransportDecisionSummary,
    /// Per-decision stability rows.
    pub rows: Vec<TransportDecisionRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<TransportDecisionDefect>,
    /// The decision snapshot embedded as evidence.
    pub decision_snapshot: TransportDecisionSnapshot,
}

impl TransportDecisionLogPage {
    /// Build the decision log page from a decision snapshot.
    ///
    /// Rows are derived per decision, and the qualification for each is
    /// computed from the combined audit of the whole snapshot.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        decision_snapshot: TransportDecisionSnapshot,
    ) -> Self {
        let defects = audit_snapshot(&decision_snapshot);
        let rows = derive_decision_rows(&decision_snapshot, &defects);
        let summary = TransportDecisionSummary::from_rows(&rows, &decision_snapshot, &defects);
        Self {
            record_kind: TRANSPORT_DECISION_PAGE_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_DECISION_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_DECISION_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            evidence_index_ref: TRANSPORT_DECISION_EVIDENCE_INDEX_REF.to_owned(),
            summary,
            rows,
            defects,
            decision_snapshot,
        }
    }

    /// Returns `true` when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token == DecisionQualificationClass::Stable.as_str()
    }

    /// Returns `true` when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// Returns `true` when all required surfaces have a decision.
    pub fn covers_all_required_surfaces(&self) -> bool {
        let covered = self.decision_snapshot.covered_surface_tokens();
        REQUIRED_SURFACES
            .iter()
            .all(|surface| covered.contains(surface.as_str()))
    }

    /// Returns `true` when every decision resolved through the shared
    /// transport-governance layer.
    pub fn no_decision_bypasses_governance(&self) -> bool {
        self.decision_snapshot.decisions.iter().all(|d| d.no_bypass)
    }

    /// Returns `true` when no decision permits a silent fall-through to the
    /// public internet.
    pub fn no_decision_allows_silent_public_fallback(&self) -> bool {
        self.decision_snapshot
            .decisions
            .iter()
            .all(|d| d.policy.no_silent_public_fallback)
    }

    /// Returns `true` when every offline-deferred decision queues only an
    /// idempotent action.
    pub fn replay_queues_are_idempotent_only(&self) -> bool {
        self.decision_snapshot
            .decisions
            .iter()
            .all(|d| !d.outcome.is_offline_deferred() || d.action_is_idempotent)
    }

    /// Returns `true` when every egress class that requires a policy epoch ref
    /// carries one.
    pub fn egress_classes_have_policy_epoch_refs(&self) -> bool {
        self.decision_snapshot.decisions.iter().all(|d| {
            if d.policy.egress_class.requires_policy_epoch_ref() {
                d.policy.policy_epoch_ref.is_some()
            } else {
                true
            }
        })
    }

    /// Returns `true` when every denied decision carries a typed denial reason.
    pub fn denied_decisions_carry_reasons(&self) -> bool {
        self.decision_snapshot
            .decisions
            .iter()
            .all(|d| !d.outcome.requires_denial_reason() || d.denial_reason.is_some())
    }

    /// Returns `true` when every decision carries a non-empty trust-proof ref.
    pub fn all_decisions_declare_trust_proof(&self) -> bool {
        self.decision_snapshot
            .decisions
            .iter()
            .all(|d| !d.policy.trust_proof_ref.is_empty())
    }

    /// Returns `true` when every decision's trust proof is usable (fresh or
    /// stale only within an accepted grace window).
    pub fn all_decision_proofs_usable(&self) -> bool {
        self.decision_snapshot
            .decisions
            .iter()
            .all(|d| d.policy.trust_proof_freshness.is_usable())
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that carries the decision log page plus a
/// metadata-safe defect roll-up.
///
/// No raw endpoint URLs, raw hostnames, raw credentials, raw cookies, or raw
/// private key material may appear in this export. Only closed-vocabulary
/// tokens, opaque refs, counts, and plain-language summary sentences cross the
/// boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportDecisionSupportExport {
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
    /// The decision log page embedded as evidence.
    pub page: TransportDecisionLogPage,
    /// Narrow-reason class values present in the page's defect list.
    pub narrow_reasons_present: Vec<DecisionNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// `true` when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl TransportDecisionSupportExport {
    /// Wrap a decision log page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: TransportDecisionLogPage,
    ) -> Self {
        let mut reasons: Vec<DecisionNarrowReasonClass> = Vec::new();
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
            record_kind: TRANSPORT_DECISION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_DECISION_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_DECISION_SHARED_CONTRACT_REF.to_owned(),
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

/// Re-run the decision audit over the snapshot embedded in a page.
pub fn audit_transport_decision_page(
    page: &TransportDecisionLogPage,
) -> Vec<TransportDecisionDefect> {
    audit_snapshot(&page.decision_snapshot)
}

/// Validate a decision log page; returns `Ok` when the audit is clean.
pub fn validate_transport_decision_page(
    page: &TransportDecisionLogPage,
) -> Result<(), Vec<TransportDecisionDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal audit logic
// ---------------------------------------------------------------------------

fn audit_snapshot(snapshot: &TransportDecisionSnapshot) -> Vec<TransportDecisionDefect> {
    let mut defects: Vec<TransportDecisionDefect> = Vec::new();

    // Hard guardrails first — any one of these withdraws the packet and makes
    // no further check meaningful.
    for decision in &snapshot.decisions {
        if !decision.raw_material_excluded() {
            defects.push(TransportDecisionDefect::new(
                DecisionNarrowReasonClass::RawPrivateMaterialExposed,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' exposes raw private material; packet is withdrawn",
                    decision.decision_id, decision.surface_token
                ),
            ));
            return defects;
        }
        if !decision.no_bypass {
            defects.push(TransportDecisionDefect::new(
                DecisionNarrowReasonClass::BypassedSharedGovernance,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' resolved outside the shared transport-governance layer; packet is withdrawn",
                    decision.decision_id, decision.surface_token
                ),
            ));
            return defects;
        }
        if !decision.policy.no_silent_public_fallback {
            defects.push(TransportDecisionDefect::new(
                DecisionNarrowReasonClass::SilentPublicFallbackResolved,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' permits a silent fall-through to the public internet; packet is withdrawn",
                    decision.decision_id, decision.surface_token
                ),
            ));
            return defects;
        }
        if decision.outcome.is_offline_deferred() && !decision.action_is_idempotent {
            defects.push(TransportDecisionDefect::new(
                DecisionNarrowReasonClass::NonIdempotentReplayQueued,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' queues a non-idempotent action for offline replay; packet is withdrawn",
                    decision.decision_id, decision.surface_token
                ),
            ));
            return defects;
        }
    }

    let covered: BTreeSet<&str> = snapshot
        .decisions
        .iter()
        .map(|d| d.surface_token.as_str())
        .collect();

    // Coverage check: all required surfaces must have a decision.
    for required_surface in &REQUIRED_SURFACES {
        if !covered.contains(required_surface.as_str()) {
            defects.push(TransportDecisionDefect::new(
                DecisionNarrowReasonClass::RequiredSurfaceMissing,
                required_surface.as_str(),
                format!(
                    "required surface '{}' has no transport decision; packet is narrowed to preview",
                    required_surface.as_str()
                ),
            ));
        }
    }

    // Per-decision checks.
    for decision in &snapshot.decisions {
        if decision.outcome.requires_denial_reason() && decision.denial_reason.is_none() {
            defects.push(TransportDecisionDefect::new(
                DecisionNarrowReasonClass::DenialReasonMissing,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' is denied but carries no typed denial reason",
                    decision.decision_id, decision.surface_token
                ),
            ));
        }

        if !decision.policy.local_core_continuity_preserved {
            defects.push(TransportDecisionDefect::new(
                DecisionNarrowReasonClass::LocalCoreContinuityNotPreserved,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' does not preserve local-core continuity; local work may be blocked",
                    decision.decision_id, decision.surface_token
                ),
            ));
        }

        if decision.policy.trust_proof_ref.is_empty() {
            defects.push(TransportDecisionDefect::new(
                DecisionNarrowReasonClass::TrustProofMissing,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' carries no trust-proof ref; host proof is unverifiable",
                    decision.decision_id, decision.surface_token
                ),
            ));
        }

        if decision.policy.egress_class.requires_policy_epoch_ref()
            && decision.policy.policy_epoch_ref.is_none()
        {
            defects.push(TransportDecisionDefect::new(
                DecisionNarrowReasonClass::PolicyEpochRefMissing,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' ({}) has no policy_epoch_ref; policy epoch must be traceable",
                    decision.decision_id, decision.surface_token, decision.policy.egress_class_token
                ),
            ));
        }

        if !decision.endpoint.is_fully_classified()
            || !decision.policy.is_route_consistent()
            || decision.outcome_token.is_empty()
            || decision.auth_posture_token.is_empty()
        {
            defects.push(TransportDecisionDefect::new(
                DecisionNarrowReasonClass::TransportClassificationIncomplete,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' is missing or inconsistent in its endpoint/egress/route/proxy/outcome classification",
                    decision.decision_id, decision.surface_token
                ),
            ));
        }

        if !decision.policy.trust_proof_freshness.is_usable() {
            defects.push(TransportDecisionDefect::new(
                DecisionNarrowReasonClass::ProofStaleBeyondWindow,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' trust proof is {}; stable claim is narrowed to beta",
                    decision.decision_id,
                    decision.surface_token,
                    decision.policy.trust_proof_freshness_token
                ),
            ));
        }
    }

    defects
}

fn derive_decision_rows(
    snapshot: &TransportDecisionSnapshot,
    page_defects: &[TransportDecisionDefect],
) -> Vec<TransportDecisionRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_preview_reason());

    let overall_narrow_reason = if has_withdrawal {
        page_defects
            .iter()
            .find(|d| d.narrow_reason.is_withdrawal_reason())
            .map(|d| d.narrow_reason)
            .unwrap_or(DecisionNarrowReasonClass::RawPrivateMaterialExposed)
    } else if has_preview {
        DecisionNarrowReasonClass::RequiredSurfaceMissing
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        DecisionNarrowReasonClass::NotNarrowed
    };

    snapshot
        .decisions
        .iter()
        .map(|decision| {
            let row_narrow =
                find_decision_narrow_reason(decision, page_defects, overall_narrow_reason);
            let row_qual = qualification_for_reason(row_narrow);
            let summary = build_row_summary(&decision.surface_token, &row_qual, row_narrow);
            TransportDecisionRow {
                record_kind: TRANSPORT_DECISION_ROW_RECORD_KIND.to_owned(),
                schema_version: TRANSPORT_DECISION_SCHEMA_VERSION,
                shared_contract_ref: TRANSPORT_DECISION_SHARED_CONTRACT_REF.to_owned(),
                decision_id: decision.decision_id.clone(),
                surface_token: decision.surface_token.clone(),
                origin_scope_token: decision.endpoint.origin_scope_token.clone(),
                endpoint_class_token: decision.endpoint.endpoint_class_token.clone(),
                egress_class_token: decision.policy.egress_class_token.clone(),
                route_choice_token: decision.policy.route_choice_token.clone(),
                proxy_resolution_source_token: decision
                    .policy
                    .proxy_resolution_source_token
                    .clone(),
                auth_posture_token: decision.auth_posture_token.clone(),
                trust_material_token: decision.policy.trust_material_token.clone(),
                mirror_offline_behavior_token: decision
                    .policy
                    .mirror_offline_behavior_token
                    .clone(),
                outcome_token: decision.outcome_token.clone(),
                denial_reason_token: decision.denial_reason_token.clone(),
                no_bypass: decision.no_bypass,
                no_silent_public_fallback: decision.policy.no_silent_public_fallback,
                replay_idempotent_only: !decision.outcome.is_offline_deferred()
                    || decision.action_is_idempotent,
                local_core_continuity_preserved: decision.policy.local_core_continuity_preserved,
                policy_epoch_present: decision.policy.policy_epoch_ref.is_some(),
                proof_freshness_token: decision.policy.trust_proof_freshness_token.clone(),
                raw_private_material_excluded: decision.raw_material_excluded(),
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn qualification_for_reason(reason: DecisionNarrowReasonClass) -> DecisionQualificationClass {
    if reason.is_withdrawal_reason() {
        DecisionQualificationClass::Withdrawn
    } else if reason.is_preview_reason() {
        DecisionQualificationClass::Preview
    } else if reason != DecisionNarrowReasonClass::NotNarrowed {
        DecisionQualificationClass::Beta
    } else {
        DecisionQualificationClass::Stable
    }
}

fn find_decision_narrow_reason(
    decision: &TransportDecision,
    page_defects: &[TransportDecisionDefect],
    overall_narrow_reason: DecisionNarrowReasonClass,
) -> DecisionNarrowReasonClass {
    // A withdrawal reason taints the whole packet; every row is withdrawn.
    if overall_narrow_reason.is_withdrawal_reason() {
        return overall_narrow_reason;
    }
    // Otherwise a decision-specific defect governs the row.
    if let Some(defect) = page_defects
        .iter()
        .find(|d| d.source == decision.surface_token)
    {
        return defect.narrow_reason;
    }
    DecisionNarrowReasonClass::NotNarrowed
}

fn build_row_summary(
    surface_token: &str,
    qual: &DecisionQualificationClass,
    narrow_reason: DecisionNarrowReasonClass,
) -> String {
    match qual {
        DecisionQualificationClass::Stable => format!(
            "Surface '{}' decision qualifies stable: the endpoint descriptor, resolved egress \
             class, route choice, proxy-resolution tier, trust material, and outcome are all \
             typed; it resolved through the shared governance layer with no silent public \
             fall-through and a fresh trust proof.",
            surface_token
        ),
        _ => format!(
            "Surface '{}' decision narrowed to {} ({}): see defect list for details.",
            surface_token,
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable decision log page consumed by the headless example,
/// the integration tests, and the fixture generator.
///
/// The seeded page produces zero defects: all required surfaces have a
/// decision, no raw private material is present, every decision resolved
/// through the shared governance layer, no decision allows a silent public
/// fall-through, every offline-deferred decision is idempotent-only, every
/// decision preserves local-core continuity, every denied decision carries a
/// typed reason, carries a trust-proof ref, carries a policy epoch ref where
/// required, and has a fresh trust proof.
pub fn seeded_transport_decision_page() -> TransportDecisionLogPage {
    TransportDecisionLogPage::new(
        "remote:networked_surface_transport_decision:default",
        "Networked-surface transport-decision log — stable packet",
        "2026-06-01T00:00:00Z",
        seeded_transport_decision_snapshot(),
    )
}

/// Build the seeded transport-decision snapshot used by the seeded page.
///
/// Each required surface is represented with a fully-typed, clean decision that
/// passes all stability conditions and aligns with the frozen per-surface
/// values in [`crate::networked_surface_transport_matrix`].
pub fn seeded_transport_decision_snapshot() -> TransportDecisionSnapshot {
    let at = "2026-06-01T00:00:00Z";
    TransportDecisionSnapshot {
        decisions: vec![
            // AI inference gateway — managed endpoint, direct, bearer handle; allowed.
            TransportDecision::new(
                "remote:transport_decision:ai_gateway:0001",
                at,
                EndpointDescriptor::new(
                    SurfaceClass::AiGateway,
                    OriginScopeClass::ManagedTenant,
                    EndpointClass::InferenceGateway,
                    "endpoint:ai_gateway:managed:0001",
                    "Managed AI inference gateway",
                ),
                TransportPolicySnapshot::new(
                    "remote:transport_policy:ai_gateway:0001",
                    at,
                    SurfaceClass::AiGateway,
                    EgressClass::ManagedEndpoint,
                    RouteChoiceClass::Direct,
                    ProxyResolutionSourceClass::DirectNoProxy,
                    TrustMaterialClass::ManagedTrustBundle,
                    "trust:ai_gateway:proof:2026-06-01",
                    ProofFreshnessClass::Fresh,
                    MirrorOfflineBehaviorClass::LocalCoreOnly,
                    Some("epoch:ai_gateway:2026-06-01"),
                    true,
                    true,
                ),
                AuthPostureClass::BearerTokenHandle,
                TransportOutcomeClass::Allowed,
                None,
                true,
                true,
                "AI inference request resolved to the managed gateway over a direct route with a \
                 bearer-token handle and managed trust bundle; egress allowed; local editing \
                 continues without the gateway.",
            ),
            // Docs / browser fetcher — public internet via system proxy; allowed.
            TransportDecision::new(
                "remote:transport_decision:docs_browser_fetcher:0001",
                at,
                EndpointDescriptor::new(
                    SurfaceClass::DocsBrowserFetcher,
                    OriginScopeClass::ThirdParty,
                    EndpointClass::ContentOrigin,
                    "endpoint:docs_browser_fetcher:content:0001",
                    "Third-party documentation content origin",
                ),
                TransportPolicySnapshot::new(
                    "remote:transport_policy:docs_browser_fetcher:0001",
                    at,
                    SurfaceClass::DocsBrowserFetcher,
                    EgressClass::PublicInternet,
                    RouteChoiceClass::SystemProxy,
                    ProxyResolutionSourceClass::SystemProxy,
                    TrustMaterialClass::SystemTrustStore,
                    "trust:docs_browser_fetcher:proof:2026-06-01",
                    ProofFreshnessClass::Fresh,
                    MirrorOfflineBehaviorClass::CachedOffline,
                    Some("epoch:docs_browser_fetcher:2026-06-01"),
                    true,
                    true,
                ),
                AuthPostureClass::Anonymous,
                TransportOutcomeClass::Allowed,
                None,
                true,
                true,
                "Documentation fetch resolved to a content origin over the system proxy (lowest \
                 precedence tier), anonymous, system trust store; egress allowed; cached content \
                 stays available offline.",
            ),
            // Request / API client — public internet via manual proxy; allowed.
            TransportDecision::new(
                "remote:transport_decision:request_api_client:0001",
                at,
                EndpointDescriptor::new(
                    SurfaceClass::RequestApiClient,
                    OriginScopeClass::UserConfigured,
                    EndpointClass::RestApi,
                    "endpoint:request_api_client:rest:0001",
                    "User-configured REST API endpoint",
                ),
                TransportPolicySnapshot::new(
                    "remote:transport_policy:request_api_client:0001",
                    at,
                    SurfaceClass::RequestApiClient,
                    EgressClass::PublicInternet,
                    RouteChoiceClass::ManualProxy,
                    ProxyResolutionSourceClass::ManualProxy,
                    TrustMaterialClass::SystemTrustStore,
                    "trust:request_api_client:proof:2026-06-01",
                    ProofFreshnessClass::Fresh,
                    MirrorOfflineBehaviorClass::DenyAll,
                    Some("epoch:request_api_client:2026-06-01"),
                    true,
                    true,
                ),
                AuthPostureClass::ApiKeyHandle,
                TransportOutcomeClass::Allowed,
                None,
                true,
                true,
                "API request resolved to a REST endpoint over a manually-pinned proxy (preferred \
                 over the system proxy), API-key handle, system trust store; egress allowed; \
                 denies all when offline; local work continues.",
            ),
            // Database / cloud connector — public internet, direct, client cert; allowed.
            TransportDecision::new(
                "remote:transport_decision:database_cloud_connector:0001",
                at,
                EndpointDescriptor::new(
                    SurfaceClass::DatabaseCloudConnector,
                    OriginScopeClass::UserConfigured,
                    EndpointClass::DataStore,
                    "endpoint:database_cloud_connector:data_store:0001",
                    "User-configured cloud data store",
                ),
                TransportPolicySnapshot::new(
                    "remote:transport_policy:database_cloud_connector:0001",
                    at,
                    SurfaceClass::DatabaseCloudConnector,
                    EgressClass::PublicInternet,
                    RouteChoiceClass::Direct,
                    ProxyResolutionSourceClass::DirectNoProxy,
                    TrustMaterialClass::PinnedCaHandle,
                    "trust:database_cloud_connector:proof:2026-06-01",
                    ProofFreshnessClass::Fresh,
                    MirrorOfflineBehaviorClass::DenyAll,
                    Some("epoch:database_cloud_connector:2026-06-01"),
                    true,
                    true,
                ),
                AuthPostureClass::ClientCertificateHandle,
                TransportOutcomeClass::Allowed,
                None,
                true,
                true,
                "Data-store connection resolved over a direct route with a client-certificate \
                 handle anchored to a pinned CA; egress allowed; denies all when offline; local \
                 work continues.",
            ),
            // Registry read — mirror-only, mirror-first; served from mirror.
            TransportDecision::new(
                "remote:transport_decision:registry_read:0001",
                at,
                EndpointDescriptor::new(
                    SurfaceClass::RegistryRead,
                    OriginScopeClass::FirstParty,
                    EndpointClass::ArtifactRegistry,
                    "endpoint:registry_read:mirror:0001",
                    "Signed first-party artifact mirror",
                ),
                TransportPolicySnapshot::new(
                    "remote:transport_policy:registry_read:0001",
                    at,
                    SurfaceClass::RegistryRead,
                    EgressClass::MirrorOnly,
                    RouteChoiceClass::MirrorFirst,
                    ProxyResolutionSourceClass::MirrorRoute,
                    TrustMaterialClass::MirrorRootHandle,
                    "trust:registry_read:proof:2026-06-01",
                    ProofFreshnessClass::Fresh,
                    MirrorOfflineBehaviorClass::MirrorFirstThenDeny,
                    Some("epoch:registry_read:2026-06-01"),
                    true,
                    true,
                ),
                AuthPostureClass::Anonymous,
                TransportOutcomeClass::ServedFromMirror,
                None,
                true,
                true,
                "Registry read resolved to the signed mirror and was served from it; the route \
                 denies rather than falling through to the public internet; installed items \
                 continue without registry access.",
            ),
            // Companion handoff — loopback, direct, session cookie; allowed.
            TransportDecision::new(
                "remote:transport_decision:companion_handoff:0001",
                at,
                EndpointDescriptor::new(
                    SurfaceClass::CompanionHandoff,
                    OriginScopeClass::LoopbackLocal,
                    EndpointClass::PeerDevice,
                    "endpoint:companion_handoff:peer:0001",
                    "Loopback companion peer device",
                ),
                TransportPolicySnapshot::new(
                    "remote:transport_policy:companion_handoff:0001",
                    at,
                    SurfaceClass::CompanionHandoff,
                    EgressClass::LoopbackOnly,
                    RouteChoiceClass::Direct,
                    ProxyResolutionSourceClass::DirectNoProxy,
                    TrustMaterialClass::NoTlsLoopback,
                    "trust:companion_handoff:proof:2026-06-01",
                    ProofFreshnessClass::Fresh,
                    MirrorOfflineBehaviorClass::LocalCoreOnly,
                    None::<String>,
                    true,
                    true,
                ),
                AuthPostureClass::SessionCookieHandle,
                TransportOutcomeClass::Allowed,
                None,
                true,
                true,
                "Companion handoff resolved over a loopback-only direct route with a \
                 session-cookie handle on the on-device trust boundary; egress allowed; the \
                 desktop continues without the companion.",
            ),
            // Provider mutation — managed, direct, OAuth handle; allowed, not idempotent.
            TransportDecision::new(
                "remote:transport_decision:provider_mutation:0001",
                at,
                EndpointDescriptor::new(
                    SurfaceClass::ProviderMutation,
                    OriginScopeClass::ManagedTenant,
                    EndpointClass::VcsHost,
                    "endpoint:provider_mutation:vcs:0001",
                    "Managed version-control provider host",
                ),
                TransportPolicySnapshot::new(
                    "remote:transport_policy:provider_mutation:0001",
                    at,
                    SurfaceClass::ProviderMutation,
                    EgressClass::ManagedEndpoint,
                    RouteChoiceClass::Direct,
                    ProxyResolutionSourceClass::DirectNoProxy,
                    TrustMaterialClass::ManagedTrustBundle,
                    "trust:provider_mutation:proof:2026-06-01",
                    ProofFreshnessClass::Fresh,
                    MirrorOfflineBehaviorClass::DenyAll,
                    Some("epoch:provider_mutation:2026-06-01"),
                    true,
                    true,
                ),
                AuthPostureClass::OauthDelegatedHandle,
                TransportOutcomeClass::Allowed,
                None,
                // The mutation is not idempotent, but the outcome is `allowed`,
                // not `offline_deferred`, so it is never queued for replay.
                false,
                true,
                "Provider mutation resolved to the managed VCS host over a direct route with a \
                 delegated-OAuth handle; egress allowed inline; the non-idempotent mutation is \
                 never queued for offline replay; denies all when offline.",
            ),
            // Sync / offboarding — managed, direct, bearer; offline-deferred idempotent.
            TransportDecision::new(
                "remote:transport_decision:sync_offboarding:0001",
                at,
                EndpointDescriptor::new(
                    SurfaceClass::SyncOffboarding,
                    OriginScopeClass::ManagedTenant,
                    EndpointClass::SyncService,
                    "endpoint:sync_offboarding:sync:0001",
                    "Managed sync service",
                ),
                TransportPolicySnapshot::new(
                    "remote:transport_policy:sync_offboarding:0001",
                    at,
                    SurfaceClass::SyncOffboarding,
                    EgressClass::ManagedEndpoint,
                    RouteChoiceClass::Direct,
                    ProxyResolutionSourceClass::DirectNoProxy,
                    TrustMaterialClass::ManagedTrustBundle,
                    "trust:sync_offboarding:proof:2026-06-01",
                    ProofFreshnessClass::Fresh,
                    MirrorOfflineBehaviorClass::OfflineGrace,
                    Some("epoch:sync_offboarding:2026-06-01"),
                    true,
                    true,
                ),
                AuthPostureClass::BearerTokenHandle,
                TransportOutcomeClass::OfflineDeferred,
                None,
                // Only the idempotent sync action is permitted into the
                // offline-deferred replay queue.
                true,
                true,
                "Sync action resolved to the managed sync service but the route was offline; the \
                 idempotent action was deferred for replay within the offline-grace window; local \
                 data is retained.",
            ),
            // Remote preview route — managed, direct, bearer; allowed.
            TransportDecision::new(
                "remote:transport_decision:remote_preview_route:0001",
                at,
                EndpointDescriptor::new(
                    SurfaceClass::RemotePreviewRoute,
                    OriginScopeClass::FirstParty,
                    EndpointClass::PreviewOrigin,
                    "endpoint:remote_preview_route:preview:0001",
                    "First-party remote preview origin",
                ),
                TransportPolicySnapshot::new(
                    "remote:transport_policy:remote_preview_route:0001",
                    at,
                    SurfaceClass::RemotePreviewRoute,
                    EgressClass::ManagedEndpoint,
                    RouteChoiceClass::Direct,
                    ProxyResolutionSourceClass::DirectNoProxy,
                    TrustMaterialClass::ManagedTrustBundle,
                    "trust:remote_preview_route:proof:2026-06-01",
                    ProofFreshnessClass::Fresh,
                    MirrorOfflineBehaviorClass::DenyAll,
                    Some("epoch:remote_preview_route:2026-06-01"),
                    true,
                    true,
                ),
                AuthPostureClass::BearerTokenHandle,
                TransportOutcomeClass::Allowed,
                None,
                true,
                true,
                "Remote preview route resolved to the first-party preview origin over a direct \
                 route with a bearer-token handle and managed trust bundle; egress allowed; \
                 denies all when offline; the local workspace continues without the preview.",
            ),
        ],
    }
}
