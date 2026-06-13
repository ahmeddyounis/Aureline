//! Expose effective transport posture and explain recent network decisions in
//! one shared vocabulary across product, CLI/headless, diagnostics, and support
//! exports.
//!
//! The sibling [`crate::networked_surface_transport_matrix`] module *freezes*
//! the per-surface transport vocabulary, and
//! [`crate::networked_surface_transport_decision`] turns that catalog into a
//! runtime decision layer that emits one inspectable
//! [`TransportDecision`](crate::networked_surface_transport_decision::TransportDecision)
//! per network-capable action. This module is the **explainability layer** that
//! sits on top of those decisions: it projects the decision stream into three
//! product-grade, export-safe views so users and admins get transport
//! explainability instead of a generic could-not-connect dialog or a private
//! support script.
//!
//! The three views are:
//!
//! - **Current transport-posture inspector** ([`TransportPostureInspector`]) —
//!   the effective proxy mode, trust source, and mirror/offline state for a
//!   surface right now, derived from its most recent decision.
//! - **Recent network-event ledger** ([`NetworkEventLedger`]) — the recent
//!   allow/deny history, filterable by endpoint class, origin scope, and
//!   allow/deny outcome without exposing raw secrets or payloads.
//! - **Per-action explain sheet** ([`ActionExplainSheet`]) — for one action,
//!   the route, trust, outcome, and (when refused) the typed denial explanation,
//!   rendered through a single stable field catalog so product, CLI/headless,
//!   and support output quote identical decision codes and field names.
//!
//! These views aggregate into one stable proof packet
//! ([`TransportExplainabilityPage`]) consumed by product UI, CLI/headless
//! output, diagnostics, support exports, and admin/audit surfaces.
//!
//! The stable claim holds when **all** of the following conditions are verified
//! simultaneously for every covered surface:
//!
//! 1. Every required surface has a posture inspector and an explain sheet.
//! 2. No raw private material is present on any record.
//! 3. Every projected decision resolved through the shared transport-governance
//!    layer (`no_bypass: true`).
//! 4. No decision permits a silent fall-through from a confined egress class to
//!    the public internet.
//! 5. Any offline-deferred decision queues only an explicitly idempotent action.
//! 6. Every decision preserves local-core continuity.
//! 7. Every denied event carries a typed denial explanation.
//! 8. Every posture inspector carries a non-empty trust-proof ref.
//! 9. Every decision's trust proof is fresh (or stale only within an accepted
//!    grace window).
//! 10. Every explain sheet renders through the shared field catalog so product,
//!     CLI/headless, and support views stay at parity.
//!
//! Four conditions force [`ExplainQualificationClass::Withdrawn`] immediately and
//! cannot be overridden: raw private material exposed, a bypass of the shared
//! governance layer, a silent public fall-through, or a non-idempotent action
//! queued for offline replay. A missing required surface narrows to
//! [`ExplainQualificationClass::Preview`]; the remaining gaps narrow to `Beta`,
//! which lets release and support tooling automatically narrow stale or
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
//! - Doc: `docs/network/networked-surface-transport-explainability.md`
//! - Artifact: `artifacts/network/networked-surface-transport-explainability.md`
//! - Schema:
//!   `schemas/network/networked_surface_transport_explainability.schema.json`
//! - Contract ref: [`TRANSPORT_EXPLAINABILITY_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::networked_surface_transport_decision::{
    seeded_transport_decision_snapshot, ProxyResolutionSourceClass, TransportDecision,
    TransportDecisionSnapshot, TransportOutcomeClass,
};
use crate::networked_surface_transport_matrix::{
    EgressClass, EndpointClass, MirrorOfflineBehaviorClass, OriginScopeClass, ProofFreshnessClass,
    RouteChoiceClass, SurfaceClass, TrustMaterialClass, REQUIRED_SURFACES,
};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const TRANSPORT_EXPLAINABILITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const TRANSPORT_EXPLAINABILITY_SHARED_CONTRACT_REF: &str =
    "remote:networked_surface_transport_explainability:v1";

/// Record-kind tag for [`TransportExplainabilityPage`] payloads.
pub const TRANSPORT_EXPLAINABILITY_PAGE_RECORD_KIND: &str =
    "remote_networked_surface_transport_explainability_page_record";

/// Record-kind tag for [`TransportPostureInspector`] payloads.
pub const TRANSPORT_EXPLAINABILITY_POSTURE_RECORD_KIND: &str =
    "remote_networked_surface_transport_explainability_posture_inspector_record";

/// Record-kind tag for [`NetworkEventLedgerEntry`] payloads.
pub const TRANSPORT_EXPLAINABILITY_EVENT_RECORD_KIND: &str =
    "remote_networked_surface_transport_explainability_event_record";

/// Record-kind tag for [`NetworkEventLedger`] payloads.
pub const TRANSPORT_EXPLAINABILITY_LEDGER_RECORD_KIND: &str =
    "remote_networked_surface_transport_explainability_ledger_record";

/// Record-kind tag for [`ActionExplainSheet`] payloads.
pub const TRANSPORT_EXPLAINABILITY_EXPLAIN_SHEET_RECORD_KIND: &str =
    "remote_networked_surface_transport_explainability_explain_sheet_record";

/// Record-kind tag for [`TransportExplainabilityRow`] payloads.
pub const TRANSPORT_EXPLAINABILITY_ROW_RECORD_KIND: &str =
    "remote_networked_surface_transport_explainability_row_record";

/// Record-kind tag for [`TransportExplainabilityDefect`] payloads.
pub const TRANSPORT_EXPLAINABILITY_DEFECT_RECORD_KIND: &str =
    "remote_networked_surface_transport_explainability_defect_record";

/// Record-kind tag for [`TransportExplainabilitySummary`] payloads.
pub const TRANSPORT_EXPLAINABILITY_SUMMARY_RECORD_KIND: &str =
    "remote_networked_surface_transport_explainability_summary_record";

/// Record-kind tag for [`TransportExplainabilitySupportExport`] payloads.
pub const TRANSPORT_EXPLAINABILITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_networked_surface_transport_explainability_support_export_record";

/// Repo-relative path of the stable doc for this explainability layer.
pub const TRANSPORT_EXPLAINABILITY_DOC_REF: &str =
    "docs/network/networked-surface-transport-explainability.md";

/// Repo-relative path of the artifact summary for this explainability layer.
pub const TRANSPORT_EXPLAINABILITY_ARTIFACT_REF: &str =
    "artifacts/network/networked-surface-transport-explainability.md";

/// Repo-relative ref to the canonical evidence index this layer binds into for
/// the closeout certification lane.
pub const TRANSPORT_EXPLAINABILITY_EVIDENCE_INDEX_REF: &str =
    "artifacts/release/m5/xt12-evidence-index.md";

/// Stable, ordered catalog of the field names a per-action explain sheet
/// renders.
///
/// Product surfaces, CLI/headless output, and support exports MUST all render an
/// explain sheet through this exact ordered field set, so the decision codes and
/// field names a user reads in the UI are identical to the ones CLI output and
/// support packets quote. [`ActionExplainSheet::explain_fields`] is the single
/// renderer; [`ActionExplainSheet::fields_at_parity`] verifies the rendered
/// field names match this catalog.
pub const EXPLAIN_FIELD_NAMES: [&str; 11] = [
    "surface",
    "origin_scope",
    "endpoint_class",
    "egress_class",
    "route_choice",
    "proxy_resolution_source",
    "auth_posture",
    "trust_material",
    "mirror_offline_behavior",
    "outcome",
    "denial_reason",
];

// ---------------------------------------------------------------------------
// Event disposition vocabulary
// ---------------------------------------------------------------------------

/// Coarse allow/deny disposition for a recent network event.
///
/// The ledger groups the finer-grained
/// [`TransportOutcomeClass`](crate::networked_surface_transport_decision::TransportOutcomeClass)
/// into the small set users and admins filter by: an action either reached the
/// network, was refused, was satisfied without live egress, was deferred for
/// idempotent replay, or found no route at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventDispositionClass {
    /// Egress was permitted and the request reached the network.
    Allowed,
    /// The request was refused; a typed denial reason is recorded.
    Denied,
    /// Satisfied from a signed mirror or cache; no live egress was attempted.
    ServedWithoutEgress,
    /// Deferred for offline replay (idempotent actions only).
    Deferred,
    /// No route was available; the surface is offline and local-core continues.
    Unavailable,
}

impl EventDispositionClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Denied => "denied",
            Self::ServedWithoutEgress => "served_without_egress",
            Self::Deferred => "deferred",
            Self::Unavailable => "unavailable",
        }
    }

    /// Derive the coarse disposition from a fine-grained transport outcome.
    pub const fn from_outcome(outcome: TransportOutcomeClass) -> Self {
        match outcome {
            TransportOutcomeClass::Allowed => Self::Allowed,
            TransportOutcomeClass::Denied => Self::Denied,
            TransportOutcomeClass::ServedFromMirror | TransportOutcomeClass::ServedFromCache => {
                Self::ServedWithoutEgress
            }
            TransportOutcomeClass::OfflineDeferred => Self::Deferred,
            TransportOutcomeClass::OfflineUnavailable => Self::Unavailable,
        }
    }

    /// Returns `true` when this disposition let live traffic reach the network.
    pub const fn is_allow(self) -> bool {
        matches!(self, Self::Allowed)
    }

    /// Returns `true` when this disposition refused the request.
    pub const fn is_deny(self) -> bool {
        matches!(self, Self::Denied)
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall explainability page and for
/// individual surface rows.
///
/// The tier is derived, not asserted: it is computed by comparing the audit
/// defect list against the stability conditions. A caller may never bump a row
/// to `Stable` without a clean audit and complete surface coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainQualificationClass {
    /// All stability conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required surface has no posture/explain coverage; the gap prevents a
    /// beta claim for the missing surface.
    Preview,
    /// A hard guardrail was violated; the packet is withdrawn immediately and
    /// cannot be overridden.
    Withdrawn,
}

impl ExplainQualificationClass {
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

/// Typed reason a packet or row was narrowed below
/// [`ExplainQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainNarrowReasonClass {
    /// No narrowing — the row qualifies stable.
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
    /// A required surface has no posture/explain coverage; narrows to preview.
    RequiredSurfaceMissing,
    /// A denied event carries no typed denial explanation.
    DenialExplanationMissing,
    /// A decision does not preserve local-core continuity.
    LocalCoreContinuityNotPreserved,
    /// A posture inspector carries no trust-proof ref.
    TrustProofMissing,
    /// A decision's trust proof has expired beyond its freshness window.
    ProofStaleBeyondWindow,
    /// An explain sheet does not render through the shared field catalog, so
    /// product/CLI/support views would diverge.
    ExplainFieldParityBroken,
}

impl ExplainNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::BypassedSharedGovernance => "bypassed_shared_governance",
            Self::SilentPublicFallbackResolved => "silent_public_fallback_resolved",
            Self::NonIdempotentReplayQueued => "non_idempotent_replay_queued",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::DenialExplanationMissing => "denial_explanation_missing",
            Self::LocalCoreContinuityNotPreserved => "local_core_continuity_not_preserved",
            Self::TrustProofMissing => "trust_proof_missing",
            Self::ProofStaleBeyondWindow => "proof_stale_beyond_window",
            Self::ExplainFieldParityBroken => "explain_field_parity_broken",
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
// Transport posture inspector
// ---------------------------------------------------------------------------

/// Current effective transport posture for one surface.
///
/// The inspector answers, at a glance and in one inspectable record, the
/// posture questions a user or admin needs before trusting a networked surface:
/// the effective proxy mode (PAC → manual → system precedence), the trust source
/// anchoring host proof, the mirror/offline state, and whether local-core work
/// continues regardless. It is derived from the surface's most recent
/// [`TransportDecision`]; no raw endpoint URL, raw proxy host, raw CA bundle, or
/// raw certificate material appears.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportPostureInspector {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Surface this posture describes.
    pub surface: SurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Human-readable surface label safe for UI and exports.
    pub surface_label: String,
    /// Origin ownership scope of the endpoint the surface contacts.
    pub origin_scope: OriginScopeClass,
    /// Stable token for [`Self::origin_scope`].
    pub origin_scope_token: String,
    /// Endpoint class the surface contacts.
    pub endpoint_class: EndpointClass,
    /// Stable token for [`Self::endpoint_class`].
    pub endpoint_class_token: String,
    /// Effective egress class enforced for the surface.
    pub egress_class: EgressClass,
    /// Stable token for [`Self::egress_class`].
    pub egress_class_token: String,
    /// Effective route choice resolved for the surface.
    pub route_choice: RouteChoiceClass,
    /// Stable token for [`Self::route_choice`].
    pub route_choice_token: String,
    /// Effective proxy mode (PAC → manual → system precedence tier).
    pub effective_proxy_mode: ProxyResolutionSourceClass,
    /// Stable token for [`Self::effective_proxy_mode`].
    pub effective_proxy_mode_token: String,
    /// Trust source anchoring host proof for the surface.
    pub trust_source: TrustMaterialClass,
    /// Stable token for [`Self::trust_source`].
    pub trust_source_token: String,
    /// Opaque ref to the trust proof evidence; required for the stable claim.
    pub trust_proof_ref: String,
    /// Freshness of the trust proof.
    pub trust_proof_freshness: ProofFreshnessClass,
    /// Stable token for [`Self::trust_proof_freshness`].
    pub trust_proof_freshness_token: String,
    /// Mirror/offline state when the primary route is unavailable.
    pub mirror_offline_state: MirrorOfflineBehaviorClass,
    /// Stable token for [`Self::mirror_offline_state`].
    pub mirror_offline_state_token: String,
    /// `true` when local-core editing continues regardless of this surface's
    /// availability.
    pub local_core_continuity_preserved: bool,
    /// `true` when no silent fall-through to the public internet is permitted
    /// from a confined egress class.
    pub no_silent_public_fallback: bool,
    /// `true` when the posture resolved through the shared transport-governance
    /// layer and did not ship a private proxy/trust stack.
    pub no_bypass: bool,
    /// Plain-language posture summary safe for UI, support export, and
    /// diagnostics.
    pub summary: String,
    /// `true` when no raw private material is present on this record.
    pub raw_private_material_excluded: bool,
}

impl TransportPostureInspector {
    /// Project a transport decision into its current posture inspector.
    pub fn from_decision(decision: &TransportDecision) -> Self {
        let policy = &decision.policy;
        let endpoint = &decision.endpoint;
        let summary = format!(
            "Surface '{}' is reachable over the {} route ({} proxy tier) to a {} endpoint with \
             {} egress; host proof is anchored by {} ({}); when the route is unavailable it \
             {}; local editing continues regardless.",
            decision.surface_token,
            policy.route_choice_token,
            policy.proxy_resolution_source_token,
            endpoint.endpoint_class_token,
            policy.egress_class_token,
            policy.trust_material_token,
            policy.trust_proof_freshness_token,
            policy.mirror_offline_behavior_token,
        );
        Self {
            record_kind: TRANSPORT_EXPLAINABILITY_POSTURE_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_EXPLAINABILITY_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_EXPLAINABILITY_SHARED_CONTRACT_REF.to_owned(),
            surface: decision.surface,
            surface_token: decision.surface_token.clone(),
            surface_label: decision.surface.label().to_owned(),
            origin_scope: endpoint.origin_scope,
            origin_scope_token: endpoint.origin_scope_token.clone(),
            endpoint_class: endpoint.endpoint_class,
            endpoint_class_token: endpoint.endpoint_class_token.clone(),
            egress_class: policy.egress_class,
            egress_class_token: policy.egress_class_token.clone(),
            route_choice: policy.route_choice,
            route_choice_token: policy.route_choice_token.clone(),
            effective_proxy_mode: policy.proxy_resolution_source,
            effective_proxy_mode_token: policy.proxy_resolution_source_token.clone(),
            trust_source: policy.trust_material,
            trust_source_token: policy.trust_material_token.clone(),
            trust_proof_ref: policy.trust_proof_ref.clone(),
            trust_proof_freshness: policy.trust_proof_freshness,
            trust_proof_freshness_token: policy.trust_proof_freshness_token.clone(),
            mirror_offline_state: policy.mirror_offline_behavior,
            mirror_offline_state_token: policy.mirror_offline_behavior_token.clone(),
            local_core_continuity_preserved: policy.local_core_continuity_preserved,
            no_silent_public_fallback: policy.no_silent_public_fallback,
            no_bypass: decision.no_bypass,
            summary,
            raw_private_material_excluded: decision.raw_material_excluded(),
        }
    }

    /// Returns `true` when every classification token is present.
    pub fn is_fully_classified(&self) -> bool {
        !self.surface_token.is_empty()
            && !self.origin_scope_token.is_empty()
            && !self.endpoint_class_token.is_empty()
            && !self.egress_class_token.is_empty()
            && !self.route_choice_token.is_empty()
            && !self.effective_proxy_mode_token.is_empty()
            && !self.trust_source_token.is_empty()
            && !self.mirror_offline_state_token.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Network-event ledger
// ---------------------------------------------------------------------------

/// One recent network event in the ledger.
///
/// Each entry projects a single [`TransportDecision`] into the small set of
/// fields users and admins filter recent history by — endpoint class, origin
/// scope, and allow/deny disposition — plus the typed denial reason when
/// refused. No raw endpoint URL, raw credential, or raw payload appears.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEventLedgerEntry {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable event id (the originating decision id).
    pub event_id: String,
    /// UTC instant the event occurred (the decision's `decided_at`).
    pub occurred_at: String,
    /// Surface this event belongs to.
    pub surface_token: String,
    /// Origin scope of the contacted endpoint.
    pub origin_scope_token: String,
    /// Endpoint class of the contacted endpoint.
    pub endpoint_class_token: String,
    /// Effective egress class for the event.
    pub egress_class_token: String,
    /// Effective route choice for the event.
    pub route_choice_token: String,
    /// Handle-only auth posture presented.
    pub auth_posture_token: String,
    /// Fine-grained transport outcome token.
    pub outcome_token: String,
    /// Coarse allow/deny disposition for filtering.
    pub disposition: EventDispositionClass,
    /// Stable token for [`Self::disposition`].
    pub disposition_token: String,
    /// Typed denial reason token when denied; empty otherwise.
    pub denial_reason_token: String,
    /// Plain-language one-line event summary.
    pub summary: String,
    /// `true` when no raw private material is present on this record.
    pub raw_private_material_excluded: bool,
}

impl NetworkEventLedgerEntry {
    /// Project a transport decision into a ledger event.
    pub fn from_decision(decision: &TransportDecision) -> Self {
        let disposition = EventDispositionClass::from_outcome(decision.outcome);
        Self {
            record_kind: TRANSPORT_EXPLAINABILITY_EVENT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_EXPLAINABILITY_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_EXPLAINABILITY_SHARED_CONTRACT_REF.to_owned(),
            event_id: decision.decision_id.clone(),
            occurred_at: decision.decided_at.clone(),
            surface_token: decision.surface_token.clone(),
            origin_scope_token: decision.endpoint.origin_scope_token.clone(),
            endpoint_class_token: decision.endpoint.endpoint_class_token.clone(),
            egress_class_token: decision.policy.egress_class_token.clone(),
            route_choice_token: decision.policy.route_choice_token.clone(),
            auth_posture_token: decision.auth_posture_token.clone(),
            outcome_token: decision.outcome_token.clone(),
            disposition,
            disposition_token: disposition.as_str().to_owned(),
            denial_reason_token: decision.denial_reason_token.clone(),
            summary: decision.summary.clone(),
            raw_private_material_excluded: decision.raw_material_excluded(),
        }
    }
}

/// Recent network-event ledger across the covered surfaces.
///
/// The ledger holds one entry per recent decision and exposes the filters
/// support, CLI, and product surfaces share: by endpoint class, by origin scope,
/// and by allow/deny disposition. Filtering never reveals raw secrets or
/// payloads because the entries only carry closed-vocabulary tokens and opaque
/// refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEventLedger {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// All recent events, newest decisions last in seed order.
    pub entries: Vec<NetworkEventLedgerEntry>,
}

impl NetworkEventLedger {
    /// Build a ledger from a decision snapshot.
    pub fn from_snapshot(snapshot: &TransportDecisionSnapshot) -> Self {
        Self {
            record_kind: TRANSPORT_EXPLAINABILITY_LEDGER_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_EXPLAINABILITY_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_EXPLAINABILITY_SHARED_CONTRACT_REF.to_owned(),
            entries: snapshot
                .decisions
                .iter()
                .map(NetworkEventLedgerEntry::from_decision)
                .collect(),
        }
    }

    /// Returns the events whose endpoint class matches `endpoint_class`.
    pub fn filter_by_endpoint_class(
        &self,
        endpoint_class: EndpointClass,
    ) -> Vec<&NetworkEventLedgerEntry> {
        let token = endpoint_class.as_str();
        self.entries
            .iter()
            .filter(|e| e.endpoint_class_token == token)
            .collect()
    }

    /// Returns the events whose origin scope matches `origin_scope`.
    pub fn filter_by_origin_scope(
        &self,
        origin_scope: OriginScopeClass,
    ) -> Vec<&NetworkEventLedgerEntry> {
        let token = origin_scope.as_str();
        self.entries
            .iter()
            .filter(|e| e.origin_scope_token == token)
            .collect()
    }

    /// Returns the events whose coarse disposition matches `disposition`.
    pub fn filter_by_disposition(
        &self,
        disposition: EventDispositionClass,
    ) -> Vec<&NetworkEventLedgerEntry> {
        self.entries
            .iter()
            .filter(|e| e.disposition == disposition)
            .collect()
    }

    /// Returns only the allowed events.
    pub fn allowed_events(&self) -> Vec<&NetworkEventLedgerEntry> {
        self.entries
            .iter()
            .filter(|e| e.disposition.is_allow())
            .collect()
    }

    /// Returns only the denied events.
    pub fn denied_events(&self) -> Vec<&NetworkEventLedgerEntry> {
        self.entries
            .iter()
            .filter(|e| e.disposition.is_deny())
            .collect()
    }

    /// Returns the count of events per disposition token.
    pub fn disposition_counts(&self) -> BTreeMap<String, usize> {
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for entry in &self.entries {
            *counts.entry(entry.disposition_token.clone()).or_insert(0) += 1;
        }
        counts
    }
}

// ---------------------------------------------------------------------------
// Per-action explain sheet
// ---------------------------------------------------------------------------

/// Per-action transport explain sheet rendered through one stable field
/// catalog.
///
/// The sheet is the canonical answer to "why did this action route and resolve
/// the way it did?". It binds the action's route choice, trust source, outcome,
/// and (when refused) the typed denial reason, and renders them through the
/// shared [`EXPLAIN_FIELD_NAMES`] catalog so the product UI, CLI/headless
/// output, and support exports all quote identical decision codes and field
/// names. No raw endpoint URL, raw credential, or raw payload appears.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionExplainSheet {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable explain-sheet id (the originating decision id).
    pub action_id: String,
    /// Surface this action belongs to.
    pub surface_token: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Coarse allow/deny disposition for the action.
    pub disposition_token: String,
    /// Stable, ordered (field_name, token_value) pairs rendered through the
    /// shared catalog. The single source product/CLI/support all read.
    pub fields: Vec<ExplainField>,
    /// Plain-language route explanation safe for every surface.
    pub route_explanation: String,
    /// Plain-language denial explanation when refused; `None` otherwise.
    pub denial_explanation: Option<String>,
    /// `true` when no raw private material is present on this record.
    pub raw_private_material_excluded: bool,
}

/// One (field name, token value) pair in an explain sheet's rendered field set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplainField {
    /// Canonical field name from [`EXPLAIN_FIELD_NAMES`].
    pub name: String,
    /// Closed-vocabulary token value for the field; empty when not applicable.
    pub value: String,
}

impl ActionExplainSheet {
    /// Project a transport decision into its per-action explain sheet.
    pub fn from_decision(decision: &TransportDecision) -> Self {
        let disposition = EventDispositionClass::from_outcome(decision.outcome);
        let fields = render_explain_fields(decision);
        let route_explanation = format!(
            "Action '{}' on surface '{}' resolved through the shared transport-governance layer: \
             {} egress over the {} route ({} proxy tier) to a {} endpoint, presenting a {} \
             credential handle, with host proof anchored by {}.",
            decision.decision_id,
            decision.surface_token,
            decision.policy.egress_class_token,
            decision.policy.route_choice_token,
            decision.policy.proxy_resolution_source_token,
            decision.endpoint.endpoint_class_token,
            decision.auth_posture_token,
            decision.policy.trust_material_token,
        );
        let denial_explanation = decision.denial_reason.map(|reason| {
            format!(
                "The action was denied with reason '{}'; the surface keeps local-core continuity \
                 and surfaces this typed reason rather than a generic could-not-connect error.",
                reason.as_str()
            )
        });
        Self {
            record_kind: TRANSPORT_EXPLAINABILITY_EXPLAIN_SHEET_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_EXPLAINABILITY_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_EXPLAINABILITY_SHARED_CONTRACT_REF.to_owned(),
            action_id: decision.decision_id.clone(),
            surface_token: decision.surface_token.clone(),
            surface_label: decision.surface.label().to_owned(),
            disposition_token: disposition.as_str().to_owned(),
            fields,
            route_explanation,
            denial_explanation,
            raw_private_material_excluded: decision.raw_material_excluded(),
        }
    }

    /// Render the sheet's fields as ordered `(name, value)` pairs.
    ///
    /// This is the single renderer product, CLI/headless, and support views all
    /// call, so the field names and decision codes stay identical across
    /// surfaces.
    pub fn explain_fields(&self) -> Vec<(String, String)> {
        self.fields
            .iter()
            .map(|f| (f.name.clone(), f.value.clone()))
            .collect()
    }

    /// Render the sheet as CLI/headless `key=value` lines.
    pub fn render_cli_lines(&self) -> Vec<String> {
        self.explain_fields()
            .into_iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect()
    }

    /// Render the sheet as support-export `key: value` lines.
    pub fn render_support_lines(&self) -> Vec<String> {
        self.explain_fields()
            .into_iter()
            .map(|(name, value)| format!("{name}: {value}"))
            .collect()
    }

    /// Returns `true` when the sheet's field names match [`EXPLAIN_FIELD_NAMES`]
    /// in order, proving product/CLI/support parity by construction.
    pub fn fields_at_parity(&self) -> bool {
        self.fields.len() == EXPLAIN_FIELD_NAMES.len()
            && self
                .fields
                .iter()
                .zip(EXPLAIN_FIELD_NAMES.iter())
                .all(|(field, expected)| field.name == *expected)
    }
}

fn render_explain_fields(decision: &TransportDecision) -> Vec<ExplainField> {
    let values: [&str; EXPLAIN_FIELD_NAMES.len()] = [
        decision.surface_token.as_str(),
        decision.endpoint.origin_scope_token.as_str(),
        decision.endpoint.endpoint_class_token.as_str(),
        decision.policy.egress_class_token.as_str(),
        decision.policy.route_choice_token.as_str(),
        decision.policy.proxy_resolution_source_token.as_str(),
        decision.auth_posture_token.as_str(),
        decision.policy.trust_material_token.as_str(),
        decision.policy.mirror_offline_behavior_token.as_str(),
        decision.outcome_token.as_str(),
        decision.denial_reason_token.as_str(),
    ];
    EXPLAIN_FIELD_NAMES
        .iter()
        .zip(values.iter())
        .map(|(name, value)| ExplainField {
            name: (*name).to_owned(),
            value: (*value).to_owned(),
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Explainability row (per-surface stability row)
// ---------------------------------------------------------------------------

/// Stability qualification for one surface in the explainability page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportExplainabilityRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Surface token for this row.
    pub surface_token: String,
    /// Action/decision id projected for this row.
    pub action_id: String,
    /// Origin scope token.
    pub origin_scope_token: String,
    /// Endpoint class token.
    pub endpoint_class_token: String,
    /// Effective egress class token.
    pub egress_class_token: String,
    /// Effective route choice token.
    pub route_choice_token: String,
    /// Effective proxy mode token.
    pub effective_proxy_mode_token: String,
    /// Trust source token.
    pub trust_source_token: String,
    /// Mirror/offline state token.
    pub mirror_offline_state_token: String,
    /// Coarse disposition token for the most recent event.
    pub disposition_token: String,
    /// Denial reason token; empty when not denied.
    pub denial_reason_token: String,
    /// `true` when the decision resolved through the shared governance layer.
    pub no_bypass: bool,
    /// `true` when no silent public fall-through is permitted.
    pub no_silent_public_fallback: bool,
    /// `true` when local-core continuity is preserved.
    pub local_core_continuity_preserved: bool,
    /// `true` when the explain sheet renders at field-catalog parity.
    pub explain_fields_at_parity: bool,
    /// Trust proof freshness token.
    pub proof_freshness_token: String,
    /// `true` when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Derived qualification tier token.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate banner emitted with the explainability page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TransportExplainabilitySummary {
    /// Total row count (one row per covered surface).
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Surface tokens covered by the page.
    pub surfaces_covered: Vec<String>,
    /// Number of posture inspectors that resolved through the shared layer.
    pub no_bypass_count: usize,
    /// Number of posture inspectors that preserve local-core continuity.
    pub local_core_continuity_preserved_count: usize,
    /// Number of posture inspectors with a fresh (or grace-window) trust proof.
    pub usable_proof_count: usize,
    /// Number of explain sheets at field-catalog parity.
    pub explain_fields_at_parity_count: usize,
    /// Ledger event counts by disposition token.
    pub disposition_counts: BTreeMap<String, usize>,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl TransportExplainabilitySummary {
    fn build(
        rows: &[TransportExplainabilityRow],
        inspectors: &[TransportPostureInspector],
        sheets: &[ActionExplainSheet],
        ledger: &NetworkEventLedger,
        defects: &[TransportExplainabilityDefect],
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
        let has_withdrawal = defects
            .iter()
            .any(|d| d.narrow_reason.is_withdrawal_reason());
        let has_preview = defects.iter().any(|d| d.narrow_reason.is_preview_reason());
        let overall = if has_withdrawal || withdrawn > 0 {
            ExplainQualificationClass::Withdrawn
        } else if has_preview || preview > 0 {
            ExplainQualificationClass::Preview
        } else if !defects.is_empty() || beta > 0 {
            ExplainQualificationClass::Beta
        } else {
            ExplainQualificationClass::Stable
        };
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            surfaces_covered: inspectors.iter().map(|i| i.surface_token.clone()).collect(),
            no_bypass_count: inspectors.iter().filter(|i| i.no_bypass).count(),
            local_core_continuity_preserved_count: inspectors
                .iter()
                .filter(|i| i.local_core_continuity_preserved)
                .count(),
            usable_proof_count: inspectors
                .iter()
                .filter(|i| i.trust_proof_freshness.is_usable())
                .count(),
            explain_fields_at_parity_count: sheets.iter().filter(|s| s.fields_at_parity()).count(),
            disposition_counts: ledger.disposition_counts(),
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the explainability page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportExplainabilityDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: ExplainNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (surface token or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl TransportExplainabilityDefect {
    fn new(
        narrow_reason: ExplainNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: TRANSPORT_EXPLAINABILITY_DEFECT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_EXPLAINABILITY_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_EXPLAINABILITY_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "remote:defect:networked-surface-transport-explainability:{}:{}",
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
// Explainability page (proof packet)
// ---------------------------------------------------------------------------

/// Stable transport-explainability proof packet for the network-capable
/// surfaces.
///
/// The packet is the single inspectable record that gives users and admins
/// product-grade transport explainability: it carries the current posture
/// inspectors, the recent network-event ledger, and the per-action explain
/// sheets, all projected from one decision snapshot so product UI, CLI/headless
/// output, diagnostics, support exports, and admin/audit surfaces quote the same
/// stable decision codes and field names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportExplainabilityPage {
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
    /// Repo-relative ref to the canonical evidence index this page binds into.
    pub evidence_index_ref: String,
    /// Aggregate summary derived from all rows.
    pub summary: TransportExplainabilitySummary,
    /// Per-surface stability rows.
    pub rows: Vec<TransportExplainabilityRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<TransportExplainabilityDefect>,
    /// Current transport-posture inspectors, one per covered surface.
    pub posture_inspectors: Vec<TransportPostureInspector>,
    /// Recent network-event ledger.
    pub event_ledger: NetworkEventLedger,
    /// Per-action explain sheets, one per covered surface.
    pub explain_sheets: Vec<ActionExplainSheet>,
    /// The decision snapshot embedded as evidence.
    pub decision_snapshot: TransportDecisionSnapshot,
}

impl TransportExplainabilityPage {
    /// Build the explainability page from a decision snapshot.
    ///
    /// Posture inspectors, the event ledger, and explain sheets are projected
    /// from the snapshot, and the qualification for each surface is computed from
    /// the combined audit of the whole snapshot.
    pub fn from_decision_snapshot(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        decision_snapshot: TransportDecisionSnapshot,
    ) -> Self {
        let posture_inspectors: Vec<TransportPostureInspector> = decision_snapshot
            .decisions
            .iter()
            .map(TransportPostureInspector::from_decision)
            .collect();
        let event_ledger = NetworkEventLedger::from_snapshot(&decision_snapshot);
        let explain_sheets: Vec<ActionExplainSheet> = decision_snapshot
            .decisions
            .iter()
            .map(ActionExplainSheet::from_decision)
            .collect();
        let defects = audit_snapshot(&decision_snapshot, &explain_sheets);
        let rows = derive_rows(&decision_snapshot, &explain_sheets, &defects);
        let summary = TransportExplainabilitySummary::build(
            &rows,
            &posture_inspectors,
            &explain_sheets,
            &event_ledger,
            &defects,
        );
        Self {
            record_kind: TRANSPORT_EXPLAINABILITY_PAGE_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_EXPLAINABILITY_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_EXPLAINABILITY_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            evidence_index_ref: TRANSPORT_EXPLAINABILITY_EVIDENCE_INDEX_REF.to_owned(),
            summary,
            rows,
            defects,
            posture_inspectors,
            event_ledger,
            explain_sheets,
            decision_snapshot,
        }
    }

    /// Returns `true` when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token == ExplainQualificationClass::Stable.as_str()
    }

    /// Returns `true` when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// Returns `true` when all required surfaces have a posture inspector.
    pub fn covers_all_required_surfaces(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .posture_inspectors
            .iter()
            .map(|i| i.surface_token.as_str())
            .collect();
        REQUIRED_SURFACES
            .iter()
            .all(|surface| covered.contains(surface.as_str()))
    }

    /// Returns `true` when every explain sheet renders at field-catalog parity.
    pub fn all_explain_sheets_at_parity(&self) -> bool {
        self.explain_sheets.iter().all(|s| s.fields_at_parity())
    }

    /// Returns `true` when every denied event carries a typed denial reason.
    pub fn denied_events_carry_reasons(&self) -> bool {
        self.event_ledger
            .entries
            .iter()
            .all(|e| !e.disposition.is_deny() || !e.denial_reason_token.is_empty())
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that carries the explainability page plus a
/// metadata-safe defect roll-up.
///
/// No raw endpoint URLs, raw hostnames, raw credentials, raw cookies, or raw
/// private key material may appear in this export. Only closed-vocabulary
/// tokens, opaque refs, counts, and plain-language summary sentences cross the
/// boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportExplainabilitySupportExport {
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
    /// The explainability page embedded as evidence.
    pub page: TransportExplainabilityPage,
    /// Narrow-reason class values present in the page's defect list.
    pub narrow_reasons_present: Vec<ExplainNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// `true` when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl TransportExplainabilitySupportExport {
    /// Wrap an explainability page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: TransportExplainabilityPage,
    ) -> Self {
        let mut reasons: Vec<ExplainNarrowReasonClass> = Vec::new();
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
            record_kind: TRANSPORT_EXPLAINABILITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_EXPLAINABILITY_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_EXPLAINABILITY_SHARED_CONTRACT_REF.to_owned(),
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

/// Re-run the explainability audit over the snapshot and sheets embedded in a
/// page.
pub fn audit_transport_explainability_page(
    page: &TransportExplainabilityPage,
) -> Vec<TransportExplainabilityDefect> {
    audit_snapshot(&page.decision_snapshot, &page.explain_sheets)
}

/// Validate an explainability page; returns `Ok` when the audit is clean.
pub fn validate_transport_explainability_page(
    page: &TransportExplainabilityPage,
) -> Result<(), Vec<TransportExplainabilityDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal audit logic
// ---------------------------------------------------------------------------

fn audit_snapshot(
    snapshot: &TransportDecisionSnapshot,
    sheets: &[ActionExplainSheet],
) -> Vec<TransportExplainabilityDefect> {
    let mut defects: Vec<TransportExplainabilityDefect> = Vec::new();

    // Hard guardrails first — any one of these withdraws the packet and makes
    // no further check meaningful.
    for decision in &snapshot.decisions {
        if !decision.raw_material_excluded() {
            defects.push(TransportExplainabilityDefect::new(
                ExplainNarrowReasonClass::RawPrivateMaterialExposed,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' exposes raw private material; packet is withdrawn",
                    decision.decision_id, decision.surface_token
                ),
            ));
            return defects;
        }
        if !decision.no_bypass {
            defects.push(TransportExplainabilityDefect::new(
                ExplainNarrowReasonClass::BypassedSharedGovernance,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' resolved outside the shared transport-governance layer; packet is withdrawn",
                    decision.decision_id, decision.surface_token
                ),
            ));
            return defects;
        }
        if !decision.policy.no_silent_public_fallback {
            defects.push(TransportExplainabilityDefect::new(
                ExplainNarrowReasonClass::SilentPublicFallbackResolved,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' permits a silent fall-through to the public internet; packet is withdrawn",
                    decision.decision_id, decision.surface_token
                ),
            ));
            return defects;
        }
        if decision.outcome.is_offline_deferred() && !decision.action_is_idempotent {
            defects.push(TransportExplainabilityDefect::new(
                ExplainNarrowReasonClass::NonIdempotentReplayQueued,
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

    // Coverage check: all required surfaces must have a decision (and therefore
    // a posture inspector and explain sheet).
    for required_surface in &REQUIRED_SURFACES {
        if !covered.contains(required_surface.as_str()) {
            defects.push(TransportExplainabilityDefect::new(
                ExplainNarrowReasonClass::RequiredSurfaceMissing,
                required_surface.as_str(),
                format!(
                    "required surface '{}' has no posture inspector or explain sheet; packet is narrowed to preview",
                    required_surface.as_str()
                ),
            ));
        }
    }

    // Per-surface checks.
    for decision in &snapshot.decisions {
        if decision.outcome.requires_denial_reason() && decision.denial_reason.is_none() {
            defects.push(TransportExplainabilityDefect::new(
                ExplainNarrowReasonClass::DenialExplanationMissing,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' is denied but its explain sheet carries no typed denial reason",
                    decision.decision_id, decision.surface_token
                ),
            ));
        }

        if !decision.policy.local_core_continuity_preserved {
            defects.push(TransportExplainabilityDefect::new(
                ExplainNarrowReasonClass::LocalCoreContinuityNotPreserved,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' does not preserve local-core continuity; local work may be blocked",
                    decision.decision_id, decision.surface_token
                ),
            ));
        }

        if decision.policy.trust_proof_ref.is_empty() {
            defects.push(TransportExplainabilityDefect::new(
                ExplainNarrowReasonClass::TrustProofMissing,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' carries no trust-proof ref; the posture inspector cannot show a trust source",
                    decision.decision_id, decision.surface_token
                ),
            ));
        }

        if !decision.policy.trust_proof_freshness.is_usable() {
            defects.push(TransportExplainabilityDefect::new(
                ExplainNarrowReasonClass::ProofStaleBeyondWindow,
                decision.surface_token.clone(),
                format!(
                    "decision '{}' on surface '{}' trust proof is {}; stable claim is narrowed to beta",
                    decision.decision_id,
                    decision.surface_token,
                    decision.policy.trust_proof_freshness_token
                ),
            ));
        }

        if let Some(sheet) = sheets.iter().find(|s| s.action_id == decision.decision_id) {
            if !sheet.fields_at_parity() {
                defects.push(TransportExplainabilityDefect::new(
                    ExplainNarrowReasonClass::ExplainFieldParityBroken,
                    decision.surface_token.clone(),
                    format!(
                        "explain sheet '{}' on surface '{}' does not render through the shared field catalog; product/CLI/support views would diverge",
                        decision.decision_id, decision.surface_token
                    ),
                ));
            }
        }
    }

    defects
}

fn derive_rows(
    snapshot: &TransportDecisionSnapshot,
    sheets: &[ActionExplainSheet],
    page_defects: &[TransportExplainabilityDefect],
) -> Vec<TransportExplainabilityRow> {
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
            .unwrap_or(ExplainNarrowReasonClass::RawPrivateMaterialExposed)
    } else if has_preview {
        ExplainNarrowReasonClass::RequiredSurfaceMissing
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        ExplainNarrowReasonClass::NotNarrowed
    };

    snapshot
        .decisions
        .iter()
        .map(|decision| {
            let row_narrow = find_row_narrow_reason(decision, page_defects, overall_narrow_reason);
            let row_qual = qualification_for_reason(row_narrow);
            let summary = build_row_summary(&decision.surface_token, &row_qual, row_narrow);
            let parity = sheets
                .iter()
                .find(|s| s.action_id == decision.decision_id)
                .map(|s| s.fields_at_parity())
                .unwrap_or(false);
            let disposition = EventDispositionClass::from_outcome(decision.outcome);
            TransportExplainabilityRow {
                record_kind: TRANSPORT_EXPLAINABILITY_ROW_RECORD_KIND.to_owned(),
                schema_version: TRANSPORT_EXPLAINABILITY_SCHEMA_VERSION,
                shared_contract_ref: TRANSPORT_EXPLAINABILITY_SHARED_CONTRACT_REF.to_owned(),
                surface_token: decision.surface_token.clone(),
                action_id: decision.decision_id.clone(),
                origin_scope_token: decision.endpoint.origin_scope_token.clone(),
                endpoint_class_token: decision.endpoint.endpoint_class_token.clone(),
                egress_class_token: decision.policy.egress_class_token.clone(),
                route_choice_token: decision.policy.route_choice_token.clone(),
                effective_proxy_mode_token: decision.policy.proxy_resolution_source_token.clone(),
                trust_source_token: decision.policy.trust_material_token.clone(),
                mirror_offline_state_token: decision.policy.mirror_offline_behavior_token.clone(),
                disposition_token: disposition.as_str().to_owned(),
                denial_reason_token: decision.denial_reason_token.clone(),
                no_bypass: decision.no_bypass,
                no_silent_public_fallback: decision.policy.no_silent_public_fallback,
                local_core_continuity_preserved: decision.policy.local_core_continuity_preserved,
                explain_fields_at_parity: parity,
                proof_freshness_token: decision.policy.trust_proof_freshness_token.clone(),
                raw_private_material_excluded: decision.raw_material_excluded(),
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn qualification_for_reason(reason: ExplainNarrowReasonClass) -> ExplainQualificationClass {
    if reason.is_withdrawal_reason() {
        ExplainQualificationClass::Withdrawn
    } else if reason.is_preview_reason() {
        ExplainQualificationClass::Preview
    } else if reason != ExplainNarrowReasonClass::NotNarrowed {
        ExplainQualificationClass::Beta
    } else {
        ExplainQualificationClass::Stable
    }
}

fn find_row_narrow_reason(
    decision: &TransportDecision,
    page_defects: &[TransportExplainabilityDefect],
    overall_narrow_reason: ExplainNarrowReasonClass,
) -> ExplainNarrowReasonClass {
    // A withdrawal reason taints the whole packet; every row is withdrawn.
    if overall_narrow_reason.is_withdrawal_reason() {
        return overall_narrow_reason;
    }
    // Otherwise a surface-specific defect governs the row.
    if let Some(defect) = page_defects
        .iter()
        .find(|d| d.source == decision.surface_token)
    {
        return defect.narrow_reason;
    }
    ExplainNarrowReasonClass::NotNarrowed
}

fn build_row_summary(
    surface_token: &str,
    qual: &ExplainQualificationClass,
    narrow_reason: ExplainNarrowReasonClass,
) -> String {
    match qual {
        ExplainQualificationClass::Stable => format!(
            "Surface '{}' explainability qualifies stable: the posture inspector shows the \
             effective proxy mode, trust source, and mirror/offline state; the recent event has a \
             typed disposition; and the explain sheet renders at field-catalog parity so product, \
             CLI, and support views match.",
            surface_token
        ),
        _ => format!(
            "Surface '{}' explainability narrowed to {} ({}): see defect list for details.",
            surface_token,
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable explainability page consumed by the headless
/// example, the integration tests, and the fixture generator.
///
/// The seeded page reuses the seeded decision snapshot from
/// [`crate::networked_surface_transport_decision`] as its canonical event
/// source, so the posture inspectors, ledger entries, and explain sheets stay in
/// lock-step with the decision layer and produce zero defects.
pub fn seeded_transport_explainability_page() -> TransportExplainabilityPage {
    TransportExplainabilityPage::from_decision_snapshot(
        "remote:networked_surface_transport_explainability:default",
        "Networked-surface transport-explainability — stable packet",
        "2026-06-01T00:00:00Z",
        seeded_transport_explainability_snapshot(),
    )
}

/// Build the seeded decision snapshot the explainability page projects from.
///
/// This is the same canonical snapshot the decision layer seeds, reused here so
/// the explainability views never diverge from the decisions they explain.
pub fn seeded_transport_explainability_snapshot() -> TransportDecisionSnapshot {
    seeded_transport_decision_snapshot()
}
