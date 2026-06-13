//! Resolve every new network-capable surface action through one shared
//! proxy-resolution model before any request leaves the current boundary.
//!
//! The sibling [`crate::networked_surface_transport_decision`] module emits one
//! transport decision per action and records *which* proxy-resolution tier
//! selected the route. This module makes the **proxy-resolution step itself** a
//! first-class governed object: for every covered surface it freezes the
//! ordered candidate chain that resolution walked, the winning tier, and a
//! typed `deny_proxy_resolution` state when no tier may be honored — so the
//! precedence is inspectable and no helper, client, or extension can ship a
//! private proxy stack, a direct CA override, or a hidden direct-connect
//! fallback outside this vocabulary.
//!
//! Proxy-resolution precedence is fixed and ordered, highest first:
//!
//! 1. **PAC script** ([`ProxyResolutionTierClass::PacScript`]) — a
//!    proxy-auto-config script selects the route.
//! 2. **Manual / policy-pinned proxy** ([`ProxyResolutionTierClass::ManualPinned`]).
//! 3. **Environment proxy** ([`ProxyResolutionTierClass::EnvironmentProxy`]) —
//!    a declared, governed `HTTP(S)_PROXY`-style environment value.
//! 4. **System proxy** ([`ProxyResolutionTierClass::SystemProxy`]) — the
//!    platform/OS proxy.
//! 5. **Direct, no proxy** ([`ProxyResolutionTierClass::DirectNoProxy`]) — only
//!    when direct egress is the *declared* choice, never a silent fallback.
//!
//! Two tiers stand outside the precedence ladder: a signed-mirror route carries
//! no proxy at all ([`ProxyResolutionTierClass::MirrorPinned`]), and an offline
//! surface resolves no route ([`ProxyResolutionTierClass::OfflineNoRoute`]).
//!
//! Each [`ProxyResolutionRecord`] answers, per surface and in one inspectable
//! record:
//!
//! - **which tiers were considered** ([`ProxyCandidate`]) — in precedence order,
//!   each flagged available/selected and whether it is a forbidden private
//!   stack, by opaque handle only (never a raw proxy host or PAC body),
//! - **which tier won** ([`ProxyResolutionRecord::selected_tier`]) and whether
//!   that respects precedence ([`ProxyResolutionRecord::precedence_respected`]),
//! - **what resolution decided** ([`ProxyResolutionOutcomeClass`]) — resolved,
//!   mirror-pinned, degraded-awaiting-policy, denied, or offline — with a typed
//!   [`ProxyResolutionDenialClass`] when refused, and
//! - **that resolution did not bypass governance** — no private proxy stack
//!   ([`ProxyResolutionRecord::no_private_proxy_stack`]), no direct CA override
//!   ([`ProxyResolutionRecord::no_direct_ca_override`]), and no silent
//!   direct-to-public fallback
//!   ([`ProxyResolutionRecord::no_silent_direct_fallback`]).
//!
//! These records aggregate into a stable proof packet
//! ([`ProxyResolutionGovernancePage`]) consumed by product UI, CLI/headless
//! output, diagnostics, support exports, and admin/audit surfaces. A contradictory
//! or unresolvable proxy state produces a labeled **degraded** or **denied**
//! record rather than a hidden public-direct fallback.
//!
//! The stable claim holds when **all** of the following conditions are verified
//! simultaneously for every covered surface record:
//!
//! 1. All required surfaces have a proxy-resolution record.
//! 2. No raw private material (raw proxy host, raw PAC body, raw CA bytes) is
//!    present on any record.
//! 3. No record ships a private proxy stack (`no_private_proxy_stack`).
//! 4. No record ships a direct CA override (`no_direct_ca_override`).
//! 5. No record permits a silent direct-to-public fallback
//!    (`no_silent_direct_fallback`).
//! 6. Every record preserves local-core continuity.
//! 7. Every denied record carries a typed `deny_proxy_resolution` reason.
//! 8. Every record's selected tier respects the precedence ladder.
//! 9. Every record whose egress class requires a policy epoch carries a
//!    last-known-good policy epoch ref.
//! 10. Every record carries a fully-typed candidate chain, outcome, and selected
//!     tier.
//!
//! Four conditions force [`ProxyQualificationClass::Withdrawn`] immediately and
//! cannot be overridden: raw private material exposed, a shipped private proxy
//! stack, a shipped direct CA override, or a silent direct-to-public fallback. A
//! missing required surface narrows to [`ProxyQualificationClass::Preview`]; the
//! remaining gaps narrow to `Beta`, which lets release and support tooling
//! automatically narrow stale or under-qualified rows before publication.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language summary sentences, and
//! opaque handles/refs only. Raw proxy hosts, raw PAC bodies, raw CA bundles,
//! raw certificate bytes, raw credentials, and raw bearer/session tokens stay
//! outside the support boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/network/networked-surface-proxy-resolution.md`
//! - Artifact: `artifacts/network/networked-surface-proxy-resolution.md`
//! - Schema: `schemas/network/networked_surface_proxy_resolution.schema.json`
//! - Contract ref: [`PROXY_RESOLUTION_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

use crate::networked_surface_transport_matrix::{
    EgressClass, MirrorOfflineBehaviorClass, OriginScopeClass, SurfaceClass, REQUIRED_SURFACES,
};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const PROXY_RESOLUTION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const PROXY_RESOLUTION_SHARED_CONTRACT_REF: &str =
    "remote:networked_surface_proxy_resolution:v1";

/// Record-kind tag for [`ProxyResolutionGovernancePage`] payloads.
pub const PROXY_RESOLUTION_PAGE_RECORD_KIND: &str =
    "remote_networked_surface_proxy_resolution_page_record";

/// Record-kind tag for [`ProxyCandidate`] payloads.
pub const PROXY_RESOLUTION_CANDIDATE_RECORD_KIND: &str =
    "remote_networked_surface_proxy_resolution_candidate_record";

/// Record-kind tag for [`ProxyResolutionRecord`] payloads.
pub const PROXY_RESOLUTION_RECORD_KIND: &str = "remote_networked_surface_proxy_resolution_record";

/// Record-kind tag for [`ProxyResolutionRow`] payloads.
pub const PROXY_RESOLUTION_ROW_RECORD_KIND: &str =
    "remote_networked_surface_proxy_resolution_row_record";

/// Record-kind tag for [`ProxyResolutionDefect`] payloads.
pub const PROXY_RESOLUTION_DEFECT_RECORD_KIND: &str =
    "remote_networked_surface_proxy_resolution_defect_record";

/// Record-kind tag for [`ProxyResolutionSummary`] payloads.
pub const PROXY_RESOLUTION_SUMMARY_RECORD_KIND: &str =
    "remote_networked_surface_proxy_resolution_summary_record";

/// Record-kind tag for [`ProxyResolutionSupportExport`] payloads.
pub const PROXY_RESOLUTION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_networked_surface_proxy_resolution_support_export_record";

/// Repo-relative path of the stable doc for this proxy-resolution lane.
pub const PROXY_RESOLUTION_DOC_REF: &str = "docs/network/networked-surface-proxy-resolution.md";

/// Repo-relative path of the artifact summary for this proxy-resolution lane.
pub const PROXY_RESOLUTION_ARTIFACT_REF: &str =
    "artifacts/network/networked-surface-proxy-resolution.md";

/// Repo-relative ref to the canonical evidence index this lane binds into for
/// the closeout certification lane.
pub const PROXY_RESOLUTION_EVIDENCE_INDEX_REF: &str = "artifacts/release/m5/xt12-evidence-index.md";

// ---------------------------------------------------------------------------
// Proxy-resolution tier vocabulary
// ---------------------------------------------------------------------------

/// One tier in the ordered proxy-resolution precedence ladder.
///
/// The ladder is fixed: a PAC-resolved route wins over a manually-pinned proxy,
/// which wins over a declared environment proxy, which wins over the platform
/// system proxy, which wins over a declared direct connection. The mirror and
/// offline tiers sit outside the ladder. Recording the ordered candidate chain
/// (rather than just the winning route) keeps the precedence inspectable so no
/// surface can hide a private proxy stack or a direct-connect retry outside this
/// vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyResolutionTierClass {
    /// A proxy-auto-config (PAC) script selected the route (highest precedence).
    PacScript,
    /// A manually-configured or policy-pinned proxy selected the route.
    ManualPinned,
    /// A declared, governed environment proxy value selected the route.
    EnvironmentProxy,
    /// The platform/OS system proxy selected the route (lowest proxy tier).
    SystemProxy,
    /// A declared direct connection with no proxy (never a silent fallback).
    DirectNoProxy,
    /// A signed-mirror route; no proxy and no public-internet egress participate.
    MirrorPinned,
    /// No route was resolved; the surface is offline.
    OfflineNoRoute,
}

impl ProxyResolutionTierClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacScript => "pac_script",
            Self::ManualPinned => "manual_pinned",
            Self::EnvironmentProxy => "environment_proxy",
            Self::SystemProxy => "system_proxy",
            Self::DirectNoProxy => "direct_no_proxy",
            Self::MirrorPinned => "mirror_pinned",
            Self::OfflineNoRoute => "offline_no_route",
        }
    }

    /// Human-readable tier label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PacScript => "PAC script",
            Self::ManualPinned => "Manual / policy-pinned proxy",
            Self::EnvironmentProxy => "Environment proxy",
            Self::SystemProxy => "System proxy",
            Self::DirectNoProxy => "Direct (no proxy)",
            Self::MirrorPinned => "Signed-mirror route",
            Self::OfflineNoRoute => "Offline (no route)",
        }
    }

    /// Precedence rank within the ladder; **lower wins**.
    ///
    /// The four in-ladder tiers rank `0..=4`. The mirror and offline tiers are
    /// ranked outside the ladder so they never silently outrank or are outranked
    /// by a proxy tier; they are only ever the sole candidate for their record.
    pub const fn precedence_rank(self) -> u8 {
        match self {
            Self::PacScript => 0,
            Self::ManualPinned => 1,
            Self::EnvironmentProxy => 2,
            Self::SystemProxy => 3,
            Self::DirectNoProxy => 4,
            Self::MirrorPinned => 10,
            Self::OfflineNoRoute => 20,
        }
    }

    /// Returns `true` when this tier traverses a proxy.
    pub const fn is_proxied(self) -> bool {
        matches!(
            self,
            Self::PacScript | Self::ManualPinned | Self::EnvironmentProxy | Self::SystemProxy
        )
    }

    /// Returns `true` when this tier sits inside the precedence ladder (i.e. it
    /// participates in highest-available-wins selection).
    pub const fn is_in_ladder(self) -> bool {
        matches!(
            self,
            Self::PacScript
                | Self::ManualPinned
                | Self::EnvironmentProxy
                | Self::SystemProxy
                | Self::DirectNoProxy
        )
    }
}

// ---------------------------------------------------------------------------
// Proxy-resolution outcome vocabulary
// ---------------------------------------------------------------------------

/// What the shared proxy-resolution layer decided for one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyResolutionOutcomeClass {
    /// A proxy tier (or a declared direct connection) was selected and egress
    /// proceeds under it.
    Resolved,
    /// The surface is mirror-pinned; no proxy participates.
    MirrorPinnedNoProxy,
    /// Proxy state is unresolved pending a policy refresh; the surface is
    /// labeled **degraded** and holds rather than silently direct-connecting.
    DegradedAwaitingPolicy,
    /// Resolution was refused; a typed [`ProxyResolutionDenialClass`] is
    /// recorded and no fallback route is taken.
    DeniedProxyResolution,
    /// No route was available; the surface is offline and local-core continues.
    OfflineNoRoute,
}

impl ProxyResolutionOutcomeClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::MirrorPinnedNoProxy => "mirror_pinned_no_proxy",
            Self::DegradedAwaitingPolicy => "degraded_awaiting_policy",
            Self::DeniedProxyResolution => "denied_proxy_resolution",
            Self::OfflineNoRoute => "offline_no_route",
        }
    }

    /// Returns `true` when this outcome must carry a typed denial reason.
    pub const fn requires_denial_reason(self) -> bool {
        matches!(self, Self::DeniedProxyResolution)
    }

    /// Returns `true` when this outcome selected a usable proxy/direct tier.
    pub const fn selected_a_tier(self) -> bool {
        matches!(self, Self::Resolved | Self::MirrorPinnedNoProxy)
    }
}

// ---------------------------------------------------------------------------
// deny_proxy_resolution vocabulary
// ---------------------------------------------------------------------------

/// Closed `deny_proxy_resolution` vocabulary: the typed reasons proxy resolution
/// may be refused or degraded.
///
/// A contradictory or unresolvable proxy state is surfaced as one of these typed
/// reasons rather than a hidden public-direct fallback. Every M5 client, helper,
/// and extension quotes the same token rather than parsing a free-form error
/// string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyResolutionDenialClass {
    /// Multiple proxy sources disagree and policy forbids picking one silently.
    ContradictoryProxyState,
    /// A component presented its own private/undeclared proxy stack.
    PrivateProxyStackDetected,
    /// A component attempted a direct certificate-authority override.
    DirectCaOverrideDetected,
    /// A tier would silently direct-connect to the public internet.
    UndeclaredPublicFallback,
    /// The PAC script could not be fetched or evaluated.
    PacUnreachable,
    /// The selected proxy endpoint is unreachable.
    ProxyUnreachable,
    /// No tier produced a route and direct fallback is forbidden by policy.
    NoResolvableRoute,
    /// A mirror-only / deny-all profile forbids any proxy participation.
    MirrorOnlyNoProxyPermitted,
    /// Proxy policy has not been delivered yet for this surface.
    PolicyEpochUnavailable,
}

impl ProxyResolutionDenialClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContradictoryProxyState => "contradictory_proxy_state",
            Self::PrivateProxyStackDetected => "private_proxy_stack_detected",
            Self::DirectCaOverrideDetected => "direct_ca_override_detected",
            Self::UndeclaredPublicFallback => "undeclared_public_fallback",
            Self::PacUnreachable => "pac_unreachable",
            Self::ProxyUnreachable => "proxy_unreachable",
            Self::NoResolvableRoute => "no_resolvable_route",
            Self::MirrorOnlyNoProxyPermitted => "mirror_only_no_proxy_permitted",
            Self::PolicyEpochUnavailable => "policy_epoch_unavailable",
        }
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and for individual
/// proxy-resolution rows.
///
/// The tier is derived, not asserted: it is computed by comparing the audit
/// defect list against the stability conditions. A caller may never bump a row
/// to `Stable` without a clean audit and complete surface coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyQualificationClass {
    /// All stability conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required surface has no record; the coverage gap prevents a beta claim.
    Preview,
    /// A hard guardrail was violated; the packet is withdrawn immediately and
    /// cannot be overridden.
    Withdrawn,
}

impl ProxyQualificationClass {
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
/// [`ProxyQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyNarrowReasonClass {
    /// No narrowing — the record qualifies stable.
    NotNarrowed,
    /// A record carries `raw_private_material_excluded: false`; withdraws the
    /// packet immediately.
    RawPrivateMaterialExposed,
    /// A record ships a private/undeclared proxy stack; withdraws the packet
    /// immediately.
    PrivateProxyStackShipped,
    /// A record ships a direct CA override; withdraws the packet immediately.
    DirectCaOverrideShipped,
    /// A record permits a silent direct-to-public fallback; withdraws the packet
    /// immediately.
    SilentDirectFallbackResolved,
    /// A required surface has no record; narrows to preview.
    RequiredSurfaceMissing,
    /// A denied record carries no typed `deny_proxy_resolution` reason.
    DenyReasonMissing,
    /// A record's selected tier does not respect the precedence ladder.
    PrecedenceNotRespected,
    /// A record does not preserve local-core continuity.
    LocalCoreContinuityNotPreserved,
    /// A record whose egress class requires a policy epoch is missing the
    /// last-known-good policy epoch ref.
    PolicyEpochRefMissing,
    /// A record is missing part of its candidate chain, outcome, or tier
    /// classification.
    ProxyChainClassificationIncomplete,
}

impl ProxyNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::PrivateProxyStackShipped => "private_proxy_stack_shipped",
            Self::DirectCaOverrideShipped => "direct_ca_override_shipped",
            Self::SilentDirectFallbackResolved => "silent_direct_fallback_resolved",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::DenyReasonMissing => "deny_reason_missing",
            Self::PrecedenceNotRespected => "precedence_not_respected",
            Self::LocalCoreContinuityNotPreserved => "local_core_continuity_not_preserved",
            Self::PolicyEpochRefMissing => "policy_epoch_ref_missing",
            Self::ProxyChainClassificationIncomplete => "proxy_chain_classification_incomplete",
        }
    }

    /// Returns `true` when this reason is a hard guardrail that withdraws the
    /// packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::RawPrivateMaterialExposed
                | Self::PrivateProxyStackShipped
                | Self::DirectCaOverrideShipped
                | Self::SilentDirectFallbackResolved
        )
    }

    /// Returns `true` when this reason narrows to preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(self, Self::RequiredSurfaceMissing)
    }
}

// ---------------------------------------------------------------------------
// Proxy candidate (one considered tier)
// ---------------------------------------------------------------------------

/// One tier considered during proxy resolution for a surface.
///
/// The candidate names the tier by closed token plus an opaque handle, so
/// product, CLI, and support surfaces can identify the candidate without
/// reconstructing it from a raw proxy host or PAC body. No raw proxy host, raw
/// PAC body, raw CA bytes, or raw port ever appears on this record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyCandidate {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Tier this candidate represents.
    pub tier: ProxyResolutionTierClass,
    /// Stable token for [`Self::tier`].
    pub tier_token: String,
    /// Precedence rank (lower wins) for [`Self::tier`].
    pub precedence_rank: u8,
    /// `true` when this tier's configuration source was present and offered a
    /// route for this surface.
    pub available: bool,
    /// `true` when this tier won and selected the route.
    pub selected: bool,
    /// `true` when this candidate is a forbidden private/undeclared proxy stack.
    /// Must be `false` on every clean record.
    pub is_private_stack: bool,
    /// Opaque handle identifying the candidate's config source. Never a raw
    /// proxy host, raw PAC body, or raw port.
    pub candidate_handle: String,
    /// Export-safe note describing why the tier was or was not selected.
    pub note: String,
}

impl ProxyCandidate {
    /// Construct a governed (non-private-stack) candidate.
    pub fn new(
        tier: ProxyResolutionTierClass,
        available: bool,
        selected: bool,
        candidate_handle: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: PROXY_RESOLUTION_CANDIDATE_RECORD_KIND.to_owned(),
            schema_version: PROXY_RESOLUTION_SCHEMA_VERSION,
            shared_contract_ref: PROXY_RESOLUTION_SHARED_CONTRACT_REF.to_owned(),
            tier,
            tier_token: tier.as_str().to_owned(),
            precedence_rank: tier.precedence_rank(),
            available,
            selected,
            is_private_stack: false,
            candidate_handle: candidate_handle.into(),
            note: note.into(),
        }
    }

    /// Construct a candidate that represents a forbidden private proxy stack,
    /// used by the guardrail drills.
    pub fn private_stack(
        tier: ProxyResolutionTierClass,
        candidate_handle: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let mut candidate = Self::new(tier, true, false, candidate_handle, note);
        candidate.is_private_stack = true;
        candidate
    }
}

// ---------------------------------------------------------------------------
// Proxy-resolution record (per surface)
// ---------------------------------------------------------------------------

/// One inspectable proxy-resolution record emitted before a network-capable
/// surface action leaves the current boundary.
///
/// The record freezes the ordered candidate chain resolution walked, the winning
/// tier, the typed outcome, the typed `deny_proxy_resolution` reason when
/// refused, and the guardrail flags that prove resolution did not ship a private
/// proxy stack, a direct CA override, or a silent direct-to-public fallback.
///
/// No raw proxy hosts, raw PAC bodies, raw CA bundles, raw ports, raw
/// credentials, or raw bearer/session tokens may appear on this record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyResolutionRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque id for this record.
    pub record_id: String,
    /// UTC instant resolution was performed.
    pub resolved_at: String,
    /// Surface this record belongs to.
    pub surface: SurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Origin ownership scope for the endpoint being reached.
    pub origin_scope: OriginScopeClass,
    /// Stable token for [`Self::origin_scope`].
    pub origin_scope_token: String,
    /// Egress class enforced for this surface.
    pub egress_class: EgressClass,
    /// Stable token for [`Self::egress_class`].
    pub egress_class_token: String,
    /// Ordered candidate chain (highest precedence first).
    pub candidates: Vec<ProxyCandidate>,
    /// Tier that won, when one did.
    pub selected_tier: Option<ProxyResolutionTierClass>,
    /// Stable token for [`Self::selected_tier`]; empty when `None`.
    pub selected_tier_token: String,
    /// Typed outcome of resolution.
    pub outcome: ProxyResolutionOutcomeClass,
    /// Stable token for [`Self::outcome`].
    pub outcome_token: String,
    /// Typed `deny_proxy_resolution` reason when refused/degraded; `None`
    /// otherwise.
    pub denial_reason: Option<ProxyResolutionDenialClass>,
    /// Stable token for [`Self::denial_reason`]; empty when `None`.
    pub denial_reason_token: String,
    /// Mirror/offline behavior when the primary route is unavailable.
    pub mirror_offline_behavior: MirrorOfflineBehaviorClass,
    /// Stable token for [`Self::mirror_offline_behavior`].
    pub mirror_offline_behavior_token: String,
    /// Opaque ref to the last-known-good policy epoch governing this surface.
    /// Present for egress classes that require it; `None` otherwise.
    pub policy_epoch_ref: Option<String>,
    /// `true` when no candidate is a private/undeclared proxy stack and
    /// resolution ran through the shared governance layer.
    pub no_private_proxy_stack: bool,
    /// `true` when resolution did not override the certificate authority.
    pub no_direct_ca_override: bool,
    /// `true` when resolution does not silently fall through to a public direct
    /// connection.
    pub no_silent_direct_fallback: bool,
    /// `true` when local-core editing continues regardless of this surface's
    /// availability.
    pub local_core_continuity_preserved: bool,
    /// Plain-language summary safe for UI, support export, and diagnostics.
    pub summary: String,
    /// `true` when no raw private material is present on this record.
    pub raw_private_material_excluded: bool,
}

impl ProxyResolutionRecord {
    /// Construct a proxy-resolution record, filling in token fields from the
    /// typed enum values and the candidate chain.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        record_id: impl Into<String>,
        resolved_at: impl Into<String>,
        surface: SurfaceClass,
        origin_scope: OriginScopeClass,
        egress_class: EgressClass,
        candidates: Vec<ProxyCandidate>,
        outcome: ProxyResolutionOutcomeClass,
        denial_reason: Option<ProxyResolutionDenialClass>,
        mirror_offline_behavior: MirrorOfflineBehaviorClass,
        policy_epoch_ref: Option<impl Into<String>>,
        local_core_continuity_preserved: bool,
        summary: impl Into<String>,
    ) -> Self {
        let selected = candidates.iter().find(|c| c.selected).map(|c| c.tier);
        let selected_tier_token = selected.map(|t| t.as_str().to_owned()).unwrap_or_default();
        let denial_reason_token = denial_reason
            .map(|d| d.as_str().to_owned())
            .unwrap_or_default();
        let no_private_proxy_stack = !candidates.iter().any(|c| c.is_private_stack);
        Self {
            record_kind: PROXY_RESOLUTION_RECORD_KIND.to_owned(),
            schema_version: PROXY_RESOLUTION_SCHEMA_VERSION,
            shared_contract_ref: PROXY_RESOLUTION_SHARED_CONTRACT_REF.to_owned(),
            record_id: record_id.into(),
            resolved_at: resolved_at.into(),
            surface,
            surface_token: surface.as_str().to_owned(),
            origin_scope,
            origin_scope_token: origin_scope.as_str().to_owned(),
            egress_class,
            egress_class_token: egress_class.as_str().to_owned(),
            candidates,
            selected_tier: selected,
            selected_tier_token,
            outcome,
            outcome_token: outcome.as_str().to_owned(),
            denial_reason,
            denial_reason_token,
            mirror_offline_behavior,
            mirror_offline_behavior_token: mirror_offline_behavior.as_str().to_owned(),
            policy_epoch_ref: policy_epoch_ref.map(Into::into),
            no_private_proxy_stack,
            no_direct_ca_override: true,
            no_silent_direct_fallback: true,
            local_core_continuity_preserved,
            summary: summary.into(),
            raw_private_material_excluded: true,
        }
    }

    /// Returns `true` when the selected tier is the highest-precedence available
    /// candidate (or there is no in-ladder selection to check).
    ///
    /// Resolution must pick the lowest-`precedence_rank` candidate among those
    /// that are `available`. A mirror-pinned or offline record has a single
    /// out-of-ladder candidate and is trivially precedence-consistent.
    pub fn precedence_respected(&self) -> bool {
        let Some(selected) = self.selected_tier else {
            // No tier selected: only valid for denied/degraded/offline outcomes.
            return !self.outcome.selected_a_tier();
        };
        // The selected candidate must actually be marked selected and available.
        let selected_candidate = self
            .candidates
            .iter()
            .find(|c| c.tier == selected && c.selected);
        let Some(selected_candidate) = selected_candidate else {
            return false;
        };
        if !selected_candidate.available {
            return false;
        }
        if !selected.is_in_ladder() {
            // Mirror-pinned: must be the only candidate.
            return self.candidates.len() == 1;
        }
        // No available in-ladder candidate may outrank the winner.
        let best_available_rank = self
            .candidates
            .iter()
            .filter(|c| c.available && c.tier.is_in_ladder())
            .map(|c| c.precedence_rank)
            .min();
        best_available_rank == Some(selected_candidate.precedence_rank)
    }

    /// Returns `true` when every classification token and the candidate chain
    /// are present and internally consistent.
    pub fn is_fully_classified(&self) -> bool {
        if self.surface_token.is_empty()
            || self.origin_scope_token.is_empty()
            || self.egress_class_token.is_empty()
            || self.outcome_token.is_empty()
            || self.candidates.is_empty()
        {
            return false;
        }
        // Exactly one candidate may be selected, and only when the outcome
        // selected a tier.
        let selected_count = self.candidates.iter().filter(|c| c.selected).count();
        if self.outcome.selected_a_tier() {
            if selected_count != 1 || self.selected_tier.is_none() {
                return false;
            }
        } else if selected_count != 0 {
            return false;
        }
        self.candidates
            .iter()
            .all(|c| !c.candidate_handle.is_empty())
    }

    /// Returns `true` when no record on this resolution exposes raw material.
    pub fn raw_material_excluded(&self) -> bool {
        self.raw_private_material_excluded
    }

    /// Returns `true` when resolution honored every no-bypass guardrail.
    pub fn no_bypass(&self) -> bool {
        self.no_private_proxy_stack && self.no_direct_ca_override && self.no_silent_direct_fallback
    }
}

// ---------------------------------------------------------------------------
// Resolution snapshot (aggregate of all records)
// ---------------------------------------------------------------------------

/// Aggregate of all proxy-resolution records for the covered surfaces.
///
/// The snapshot carries one [`ProxyResolutionRecord`] per network-capable
/// surface. A snapshot missing any required surface causes the page to narrow to
/// `Preview`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyResolutionSnapshot {
    /// All proxy-resolution records in the snapshot.
    pub records: Vec<ProxyResolutionRecord>,
}

impl ProxyResolutionSnapshot {
    /// Returns the record for the given surface, if present.
    pub fn record_for_surface(&self, surface: SurfaceClass) -> Option<&ProxyResolutionRecord> {
        self.records.iter().find(|r| r.surface == surface)
    }

    /// Returns the set of surface tokens covered by this snapshot.
    pub fn covered_surface_tokens(&self) -> BTreeSet<&str> {
        self.records
            .iter()
            .map(|r| r.surface_token.as_str())
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Resolution row (per-record stability row)
// ---------------------------------------------------------------------------

/// Stability qualification for one proxy-resolution record in the page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyResolutionRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Record id for this row.
    pub record_id: String,
    /// Surface token for this row.
    pub surface_token: String,
    /// Origin scope token from the record.
    pub origin_scope_token: String,
    /// Egress class token from the record.
    pub egress_class_token: String,
    /// Ordered candidate tier tokens (highest precedence first).
    pub candidate_tier_tokens: Vec<String>,
    /// Selected tier token; empty when no tier was selected.
    pub selected_tier_token: String,
    /// Outcome token from the record.
    pub outcome_token: String,
    /// Denial reason token from the record; empty when not denied.
    pub denial_reason_token: String,
    /// Mirror/offline behavior token from the record.
    pub mirror_offline_behavior_token: String,
    /// `true` when no candidate is a private proxy stack.
    pub no_private_proxy_stack: bool,
    /// `true` when no direct CA override is present.
    pub no_direct_ca_override: bool,
    /// `true` when no silent direct-to-public fallback is permitted.
    pub no_silent_direct_fallback: bool,
    /// `true` when the selected tier respects the precedence ladder.
    pub precedence_respected: bool,
    /// `true` when local-core continuity is preserved.
    pub local_core_continuity_preserved: bool,
    /// `true` when a policy epoch ref is present.
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

/// Aggregate banner emitted with the proxy-resolution page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ProxyResolutionSummary {
    /// Total row count (one row per record in the snapshot).
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
    /// Number of records with no private proxy stack.
    pub no_private_proxy_stack_count: usize,
    /// Number of records with no direct CA override.
    pub no_direct_ca_override_count: usize,
    /// Number of records with no silent direct-to-public fallback.
    pub no_silent_direct_fallback_count: usize,
    /// Number of records whose selected tier respects precedence.
    pub precedence_respected_count: usize,
    /// Number of records that preserve local-core continuity.
    pub local_core_continuity_preserved_count: usize,
    /// Record counts by selected-tier token.
    pub selected_tier_counts: BTreeMap<String, usize>,
    /// Record counts by outcome token.
    pub outcome_counts: BTreeMap<String, usize>,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl ProxyResolutionSummary {
    fn from_rows(
        rows: &[ProxyResolutionRow],
        snapshot: &ProxyResolutionSnapshot,
        defects: &[ProxyResolutionDefect],
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
            ProxyQualificationClass::Withdrawn
        } else if has_preview || preview > 0 {
            ProxyQualificationClass::Preview
        } else if !defects.is_empty() || beta > 0 {
            ProxyQualificationClass::Beta
        } else {
            ProxyQualificationClass::Stable
        };
        let surfaces_covered: Vec<String> = snapshot
            .records
            .iter()
            .map(|r| r.surface_token.clone())
            .collect();
        let no_private_proxy_stack_count = snapshot
            .records
            .iter()
            .filter(|r| r.no_private_proxy_stack)
            .count();
        let no_direct_ca_override_count = snapshot
            .records
            .iter()
            .filter(|r| r.no_direct_ca_override)
            .count();
        let no_silent_direct_fallback_count = snapshot
            .records
            .iter()
            .filter(|r| r.no_silent_direct_fallback)
            .count();
        let precedence_respected_count = snapshot
            .records
            .iter()
            .filter(|r| r.precedence_respected())
            .count();
        let local_core_continuity_preserved_count = snapshot
            .records
            .iter()
            .filter(|r| r.local_core_continuity_preserved)
            .count();
        let mut selected_tier_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut outcome_counts: BTreeMap<String, usize> = BTreeMap::new();
        for record in &snapshot.records {
            if !record.selected_tier_token.is_empty() {
                *selected_tier_counts
                    .entry(record.selected_tier_token.clone())
                    .or_insert(0) += 1;
            }
            *outcome_counts
                .entry(record.outcome_token.clone())
                .or_insert(0) += 1;
        }
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            surfaces_covered,
            no_private_proxy_stack_count,
            no_direct_ca_override_count,
            no_silent_direct_fallback_count,
            precedence_respected_count,
            local_core_continuity_preserved_count,
            selected_tier_counts,
            outcome_counts,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the proxy-resolution page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyResolutionDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: ProxyNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (surface token or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl ProxyResolutionDefect {
    fn new(
        narrow_reason: ProxyNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: PROXY_RESOLUTION_DEFECT_RECORD_KIND.to_owned(),
            schema_version: PROXY_RESOLUTION_SCHEMA_VERSION,
            shared_contract_ref: PROXY_RESOLUTION_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "remote:defect:networked-surface-proxy-resolution:{}:{}",
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
// Proxy-resolution governance page (proof packet)
// ---------------------------------------------------------------------------

/// Stable proxy-resolution proof packet for the network-capable surfaces.
///
/// The packet is the single inspectable record that proves every claimed M5
/// network-capable surface resolves its proxy through one shared model with
/// explicit precedence, a typed `deny_proxy_resolution` vocabulary, and no
/// private proxy stack or silent direct-connect fallback. Dashboards, docs,
/// Help/About surfaces, CLI/headless output, support exports, release tooling,
/// and diagnostics should ingest this packet rather than reconstructing
/// precedence from raw proxy config.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyResolutionGovernancePage {
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
    /// Repo-relative ref to the canonical evidence index this packet binds into.
    pub evidence_index_ref: String,
    /// Aggregate summary derived from all rows.
    pub summary: ProxyResolutionSummary,
    /// Per-record stability rows.
    pub rows: Vec<ProxyResolutionRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<ProxyResolutionDefect>,
    /// The resolution snapshot embedded as evidence.
    pub resolution_snapshot: ProxyResolutionSnapshot,
}

impl ProxyResolutionGovernancePage {
    /// Build the proxy-resolution page from a resolution snapshot.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        resolution_snapshot: ProxyResolutionSnapshot,
    ) -> Self {
        let defects = audit_snapshot(&resolution_snapshot);
        let rows = derive_rows(&resolution_snapshot, &defects);
        let summary = ProxyResolutionSummary::from_rows(&rows, &resolution_snapshot, &defects);
        Self {
            record_kind: PROXY_RESOLUTION_PAGE_RECORD_KIND.to_owned(),
            schema_version: PROXY_RESOLUTION_SCHEMA_VERSION,
            shared_contract_ref: PROXY_RESOLUTION_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            evidence_index_ref: PROXY_RESOLUTION_EVIDENCE_INDEX_REF.to_owned(),
            summary,
            rows,
            defects,
            resolution_snapshot,
        }
    }

    /// Returns `true` when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token == ProxyQualificationClass::Stable.as_str()
    }

    /// Returns `true` when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// Returns `true` when all required surfaces have a record.
    pub fn covers_all_required_surfaces(&self) -> bool {
        let covered = self.resolution_snapshot.covered_surface_tokens();
        REQUIRED_SURFACES
            .iter()
            .all(|surface| covered.contains(surface.as_str()))
    }

    /// Returns `true` when no record ships a private proxy stack.
    pub fn no_record_ships_private_proxy_stack(&self) -> bool {
        self.resolution_snapshot
            .records
            .iter()
            .all(|r| r.no_private_proxy_stack)
    }

    /// Returns `true` when no record ships a direct CA override.
    pub fn no_record_ships_direct_ca_override(&self) -> bool {
        self.resolution_snapshot
            .records
            .iter()
            .all(|r| r.no_direct_ca_override)
    }

    /// Returns `true` when no record permits a silent direct-to-public fallback.
    pub fn no_record_allows_silent_direct_fallback(&self) -> bool {
        self.resolution_snapshot
            .records
            .iter()
            .all(|r| r.no_silent_direct_fallback)
    }

    /// Returns `true` when every record's selected tier respects precedence.
    pub fn all_records_respect_precedence(&self) -> bool {
        self.resolution_snapshot
            .records
            .iter()
            .all(|r| r.precedence_respected())
    }

    /// Returns `true` when every denied record carries a typed denial reason.
    pub fn denied_records_carry_reasons(&self) -> bool {
        self.resolution_snapshot
            .records
            .iter()
            .all(|r| !r.outcome.requires_denial_reason() || r.denial_reason.is_some())
    }

    /// Returns `true` when every egress class that requires a policy epoch ref
    /// carries one.
    pub fn egress_classes_have_policy_epoch_refs(&self) -> bool {
        self.resolution_snapshot.records.iter().all(|r| {
            if r.egress_class.requires_policy_epoch_ref() {
                r.policy_epoch_ref.is_some()
            } else {
                true
            }
        })
    }

    /// Render a stable CLI/headless view of the packet so terminal, diagnostics,
    /// and support surfaces quote identical tier tokens and outcome codes.
    pub fn render_cli_view(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "PROXY RESOLUTION — {}", self.page_label);
        let _ = writeln!(
            out,
            "overall: {}  rows: {}  stable: {}  beta: {}  preview: {}  withdrawn: {}",
            self.summary.overall_qualification_token,
            self.summary.row_count,
            self.summary.stable_row_count,
            self.summary.beta_row_count,
            self.summary.preview_row_count,
            self.summary.withdrawn_row_count,
        );
        let _ = writeln!(
            out,
            "guardrails: no_private_proxy_stack={} no_direct_ca_override={} no_silent_direct_fallback={} precedence_respected={}",
            self.summary.no_private_proxy_stack_count,
            self.summary.no_direct_ca_override_count,
            self.summary.no_silent_direct_fallback_count,
            self.summary.precedence_respected_count,
        );
        for row in &self.rows {
            let selected = if row.selected_tier_token.is_empty() {
                "(none)"
            } else {
                row.selected_tier_token.as_str()
            };
            let denial = if row.denial_reason_token.is_empty() {
                "-"
            } else {
                row.denial_reason_token.as_str()
            };
            let _ = writeln!(
                out,
                "  {:<26} egress={:<16} chain=[{}] selected={:<18} outcome={:<24} deny={:<28} {}",
                row.surface_token,
                row.egress_class_token,
                row.candidate_tier_tokens.join(">"),
                selected,
                row.outcome_token,
                denial,
                row.qualification_token,
            );
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that carries the proxy-resolution page plus a
/// metadata-safe defect roll-up.
///
/// No raw proxy hosts, raw PAC bodies, raw credentials, raw cookies, or raw
/// private key material may appear in this export. Only closed-vocabulary
/// tokens, opaque refs, counts, and plain-language summary sentences cross the
/// boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyResolutionSupportExport {
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
    /// The proxy-resolution page embedded as evidence.
    pub page: ProxyResolutionGovernancePage,
    /// Narrow-reason class values present in the page's defect list.
    pub narrow_reasons_present: Vec<ProxyNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// `true` when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl ProxyResolutionSupportExport {
    /// Wrap a proxy-resolution page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: ProxyResolutionGovernancePage,
    ) -> Self {
        let mut reasons: Vec<ProxyNarrowReasonClass> = Vec::new();
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
            record_kind: PROXY_RESOLUTION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PROXY_RESOLUTION_SCHEMA_VERSION,
            shared_contract_ref: PROXY_RESOLUTION_SHARED_CONTRACT_REF.to_owned(),
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

/// Re-run the proxy-resolution audit over the snapshot embedded in a page.
pub fn audit_proxy_resolution_page(
    page: &ProxyResolutionGovernancePage,
) -> Vec<ProxyResolutionDefect> {
    audit_snapshot(&page.resolution_snapshot)
}

/// Validate a proxy-resolution page; returns `Ok` when the audit is clean.
pub fn validate_proxy_resolution_page(
    page: &ProxyResolutionGovernancePage,
) -> Result<(), Vec<ProxyResolutionDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal audit logic
// ---------------------------------------------------------------------------

fn audit_snapshot(snapshot: &ProxyResolutionSnapshot) -> Vec<ProxyResolutionDefect> {
    let mut defects: Vec<ProxyResolutionDefect> = Vec::new();

    // Hard guardrails first — any one of these withdraws the packet and makes no
    // further check meaningful.
    for record in &snapshot.records {
        if !record.raw_material_excluded() {
            defects.push(ProxyResolutionDefect::new(
                ProxyNarrowReasonClass::RawPrivateMaterialExposed,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' exposes raw private material; packet is withdrawn",
                    record.record_id, record.surface_token
                ),
            ));
            return defects;
        }
        if !record.no_private_proxy_stack {
            defects.push(ProxyResolutionDefect::new(
                ProxyNarrowReasonClass::PrivateProxyStackShipped,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' ships a private/undeclared proxy stack; packet is withdrawn",
                    record.record_id, record.surface_token
                ),
            ));
            return defects;
        }
        if !record.no_direct_ca_override {
            defects.push(ProxyResolutionDefect::new(
                ProxyNarrowReasonClass::DirectCaOverrideShipped,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' ships a direct CA override; packet is withdrawn",
                    record.record_id, record.surface_token
                ),
            ));
            return defects;
        }
        if !record.no_silent_direct_fallback {
            defects.push(ProxyResolutionDefect::new(
                ProxyNarrowReasonClass::SilentDirectFallbackResolved,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' permits a silent direct-to-public fallback; packet is withdrawn",
                    record.record_id, record.surface_token
                ),
            ));
            return defects;
        }
    }

    let covered: BTreeSet<&str> = snapshot
        .records
        .iter()
        .map(|r| r.surface_token.as_str())
        .collect();

    // Coverage check: all required surfaces must have a record.
    for required_surface in &REQUIRED_SURFACES {
        if !covered.contains(required_surface.as_str()) {
            defects.push(ProxyResolutionDefect::new(
                ProxyNarrowReasonClass::RequiredSurfaceMissing,
                required_surface.as_str(),
                format!(
                    "required surface '{}' has no proxy-resolution record; packet is narrowed to preview",
                    required_surface.as_str()
                ),
            ));
        }
    }

    // Per-record checks.
    for record in &snapshot.records {
        if record.outcome.requires_denial_reason() && record.denial_reason.is_none() {
            defects.push(ProxyResolutionDefect::new(
                ProxyNarrowReasonClass::DenyReasonMissing,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' is denied but carries no typed deny_proxy_resolution reason",
                    record.record_id, record.surface_token
                ),
            ));
        }

        if !record.is_fully_classified() {
            defects.push(ProxyResolutionDefect::new(
                ProxyNarrowReasonClass::ProxyChainClassificationIncomplete,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' is missing or inconsistent in its candidate-chain/outcome/tier classification",
                    record.record_id, record.surface_token
                ),
            ));
        } else if !record.precedence_respected() {
            // Only meaningful when the chain is well-formed.
            defects.push(ProxyResolutionDefect::new(
                ProxyNarrowReasonClass::PrecedenceNotRespected,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' selected '{}' but a higher-precedence available tier was not honored",
                    record.record_id, record.surface_token, record.selected_tier_token
                ),
            ));
        }

        if !record.local_core_continuity_preserved {
            defects.push(ProxyResolutionDefect::new(
                ProxyNarrowReasonClass::LocalCoreContinuityNotPreserved,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' does not preserve local-core continuity; local work may be blocked",
                    record.record_id, record.surface_token
                ),
            ));
        }

        if record.egress_class.requires_policy_epoch_ref() && record.policy_epoch_ref.is_none() {
            defects.push(ProxyResolutionDefect::new(
                ProxyNarrowReasonClass::PolicyEpochRefMissing,
                record.surface_token.clone(),
                format!(
                    "record '{}' on surface '{}' ({}) has no policy_epoch_ref; the governing proxy policy must be traceable",
                    record.record_id, record.surface_token, record.egress_class_token
                ),
            ));
        }
    }

    defects
}

fn derive_rows(
    snapshot: &ProxyResolutionSnapshot,
    page_defects: &[ProxyResolutionDefect],
) -> Vec<ProxyResolutionRow> {
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
            .unwrap_or(ProxyNarrowReasonClass::RawPrivateMaterialExposed)
    } else if has_preview {
        ProxyNarrowReasonClass::RequiredSurfaceMissing
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        ProxyNarrowReasonClass::NotNarrowed
    };

    snapshot
        .records
        .iter()
        .map(|record| {
            let row_narrow = find_row_narrow_reason(record, page_defects, overall_narrow_reason);
            let row_qual = qualification_for_reason(row_narrow);
            let summary = build_row_summary(&record.surface_token, &row_qual, row_narrow);
            ProxyResolutionRow {
                record_kind: PROXY_RESOLUTION_ROW_RECORD_KIND.to_owned(),
                schema_version: PROXY_RESOLUTION_SCHEMA_VERSION,
                shared_contract_ref: PROXY_RESOLUTION_SHARED_CONTRACT_REF.to_owned(),
                record_id: record.record_id.clone(),
                surface_token: record.surface_token.clone(),
                origin_scope_token: record.origin_scope_token.clone(),
                egress_class_token: record.egress_class_token.clone(),
                candidate_tier_tokens: record
                    .candidates
                    .iter()
                    .map(|c| c.tier_token.clone())
                    .collect(),
                selected_tier_token: record.selected_tier_token.clone(),
                outcome_token: record.outcome_token.clone(),
                denial_reason_token: record.denial_reason_token.clone(),
                mirror_offline_behavior_token: record.mirror_offline_behavior_token.clone(),
                no_private_proxy_stack: record.no_private_proxy_stack,
                no_direct_ca_override: record.no_direct_ca_override,
                no_silent_direct_fallback: record.no_silent_direct_fallback,
                precedence_respected: record.precedence_respected(),
                local_core_continuity_preserved: record.local_core_continuity_preserved,
                policy_epoch_present: record.policy_epoch_ref.is_some(),
                raw_private_material_excluded: record.raw_material_excluded(),
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn qualification_for_reason(reason: ProxyNarrowReasonClass) -> ProxyQualificationClass {
    if reason.is_withdrawal_reason() {
        ProxyQualificationClass::Withdrawn
    } else if reason.is_preview_reason() {
        ProxyQualificationClass::Preview
    } else if reason != ProxyNarrowReasonClass::NotNarrowed {
        ProxyQualificationClass::Beta
    } else {
        ProxyQualificationClass::Stable
    }
}

fn find_row_narrow_reason(
    record: &ProxyResolutionRecord,
    page_defects: &[ProxyResolutionDefect],
    overall_narrow_reason: ProxyNarrowReasonClass,
) -> ProxyNarrowReasonClass {
    if overall_narrow_reason.is_withdrawal_reason() {
        return overall_narrow_reason;
    }
    if let Some(defect) = page_defects
        .iter()
        .find(|d| d.source == record.surface_token)
    {
        return defect.narrow_reason;
    }
    ProxyNarrowReasonClass::NotNarrowed
}

fn build_row_summary(
    surface_token: &str,
    qual: &ProxyQualificationClass,
    narrow_reason: ProxyNarrowReasonClass,
) -> String {
    match qual {
        ProxyQualificationClass::Stable => format!(
            "Surface '{}' proxy resolution qualifies stable: the candidate chain, selected tier, \
             and outcome are typed; precedence is respected; no private proxy stack, direct CA \
             override, or silent direct-to-public fallback is present; and local editing \
             continues regardless of the route.",
            surface_token
        ),
        _ => format!(
            "Surface '{}' proxy resolution narrowed to {} ({}): see defect list for details.",
            surface_token,
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable proxy-resolution page consumed by the headless
/// example, the integration tests, and the fixture generator.
///
/// The seeded page produces zero defects: all required surfaces have a record,
/// no raw private material is present, no record ships a private proxy stack,
/// direct CA override, or silent direct-to-public fallback, every record
/// respects precedence and preserves local-core continuity, every denied record
/// carries a typed reason, and every record carries a policy epoch ref where
/// required.
pub fn seeded_proxy_resolution_page() -> ProxyResolutionGovernancePage {
    ProxyResolutionGovernancePage::new(
        "remote:networked_surface_proxy_resolution:default",
        "Networked-surface proxy-resolution governance — stable packet",
        "2026-06-01T00:00:00Z",
        seeded_proxy_resolution_snapshot(),
    )
}

/// Build the seeded proxy-resolution snapshot used by the seeded page.
///
/// Each required surface is represented with a fully-typed, clean record that
/// passes all stability conditions. The records together exercise every
/// in-ladder tier (PAC, manual, environment, system, direct) plus the
/// mirror-pinned tier and a typed `deny_proxy_resolution` outcome.
pub fn seeded_proxy_resolution_snapshot() -> ProxyResolutionSnapshot {
    let at = "2026-06-01T00:00:00Z";
    ProxyResolutionSnapshot {
        records: vec![
            // AI gateway — manual/policy-pinned proxy wins over system; managed.
            ProxyResolutionRecord::new(
                "remote:proxy_resolution:ai_gateway:0001",
                at,
                SurfaceClass::AiGateway,
                OriginScopeClass::ManagedTenant,
                EgressClass::ManagedEndpoint,
                vec![
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::ManualPinned,
                        true,
                        true,
                        "proxy_source:ai_gateway:managed_pin:0001",
                        "Managed policy pins the gateway proxy; highest available tier.",
                    ),
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::SystemProxy,
                        true,
                        false,
                        "proxy_source:ai_gateway:system:0001",
                        "System proxy present but outranked by the policy-pinned proxy.",
                    ),
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::DirectNoProxy,
                        false,
                        false,
                        "proxy_source:ai_gateway:direct:0001",
                        "Direct egress is not a declared option for this managed surface.",
                    ),
                ],
                ProxyResolutionOutcomeClass::Resolved,
                None,
                MirrorOfflineBehaviorClass::LocalCoreOnly,
                Some("epoch:ai_gateway:2026-06-01"),
                true,
                "AI gateway proxy resolved to the managed policy-pinned proxy, which outranks the \
                 available system proxy; no private stack or direct fallback; local editing \
                 continues without the gateway.",
            ),
            // Docs / browser fetcher — system proxy wins (no PAC/manual/env present).
            ProxyResolutionRecord::new(
                "remote:proxy_resolution:docs_browser_fetcher:0001",
                at,
                SurfaceClass::DocsBrowserFetcher,
                OriginScopeClass::ThirdParty,
                EgressClass::PublicInternet,
                vec![
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::SystemProxy,
                        true,
                        true,
                        "proxy_source:docs_browser_fetcher:system:0001",
                        "OS system proxy is the only available tier and is honored.",
                    ),
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::DirectNoProxy,
                        true,
                        false,
                        "proxy_source:docs_browser_fetcher:direct:0001",
                        "Direct is a declared fallback but is outranked by the system proxy.",
                    ),
                ],
                ProxyResolutionOutcomeClass::Resolved,
                None,
                MirrorOfflineBehaviorClass::CachedOffline,
                Some("epoch:docs_browser_fetcher:2026-06-01"),
                true,
                "Documentation fetch proxy resolved to the OS system proxy (no PAC, manual, or \
                 environment proxy present); cached content stays available offline.",
            ),
            // Request / API client — declared environment proxy wins over system.
            ProxyResolutionRecord::new(
                "remote:proxy_resolution:request_api_client:0001",
                at,
                SurfaceClass::RequestApiClient,
                OriginScopeClass::UserConfigured,
                EgressClass::PublicInternet,
                vec![
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::EnvironmentProxy,
                        true,
                        true,
                        "proxy_source:request_api_client:env:0001",
                        "Declared governed HTTPS_PROXY environment value; highest available tier.",
                    ),
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::SystemProxy,
                        true,
                        false,
                        "proxy_source:request_api_client:system:0001",
                        "System proxy present but outranked by the environment proxy.",
                    ),
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::DirectNoProxy,
                        false,
                        false,
                        "proxy_source:request_api_client:direct:0001",
                        "Direct egress is not a declared fallback for this surface.",
                    ),
                ],
                ProxyResolutionOutcomeClass::Resolved,
                None,
                MirrorOfflineBehaviorClass::DenyAll,
                Some("epoch:request_api_client:2026-06-01"),
                true,
                "API client proxy resolved to the declared environment proxy, which outranks the \
                 available system proxy; denies all when offline; local work continues.",
            ),
            // Database / cloud connector — declared direct (no proxy available).
            ProxyResolutionRecord::new(
                "remote:proxy_resolution:database_cloud_connector:0001",
                at,
                SurfaceClass::DatabaseCloudConnector,
                OriginScopeClass::UserConfigured,
                EgressClass::PublicInternet,
                vec![ProxyCandidate::new(
                    ProxyResolutionTierClass::DirectNoProxy,
                    true,
                    true,
                    "proxy_source:database_cloud_connector:direct:0001",
                    "Direct connection is the declared route; no proxy tier is configured.",
                )],
                ProxyResolutionOutcomeClass::Resolved,
                None,
                MirrorOfflineBehaviorClass::DenyAll,
                Some("epoch:database_cloud_connector:2026-06-01"),
                true,
                "Data-store proxy resolution selected a declared direct connection; no proxy tier \
                 was configured and direct is the governed choice, not a silent fallback; denies \
                 all when offline.",
            ),
            // Registry read — mirror-pinned; no proxy participates.
            ProxyResolutionRecord::new(
                "remote:proxy_resolution:registry_read:0001",
                at,
                SurfaceClass::RegistryRead,
                OriginScopeClass::FirstParty,
                EgressClass::MirrorOnly,
                vec![ProxyCandidate::new(
                    ProxyResolutionTierClass::MirrorPinned,
                    true,
                    true,
                    "proxy_source:registry_read:mirror:0001",
                    "Mirror-only egress pins traffic to the signed mirror; no proxy participates.",
                )],
                ProxyResolutionOutcomeClass::MirrorPinnedNoProxy,
                None,
                MirrorOfflineBehaviorClass::MirrorFirstThenDeny,
                Some("epoch:registry_read:2026-06-01"),
                true,
                "Registry read is mirror-pinned: traffic is directed to the signed mirror and no \
                 proxy tier participates; the route denies rather than falling through to the \
                 public internet.",
            ),
            // Companion handoff — loopback, declared direct, no proxy.
            ProxyResolutionRecord::new(
                "remote:proxy_resolution:companion_handoff:0001",
                at,
                SurfaceClass::CompanionHandoff,
                OriginScopeClass::LoopbackLocal,
                EgressClass::LoopbackOnly,
                vec![ProxyCandidate::new(
                    ProxyResolutionTierClass::DirectNoProxy,
                    true,
                    true,
                    "proxy_source:companion_handoff:direct:0001",
                    "Loopback-only egress uses a declared direct connection; no proxy applies.",
                )],
                ProxyResolutionOutcomeClass::Resolved,
                None,
                MirrorOfflineBehaviorClass::LocalCoreOnly,
                None::<String>,
                true,
                "Companion handoff proxy resolution selected a declared direct loopback \
                 connection; no proxy tier participates on the on-device boundary; the desktop \
                 continues without the companion.",
            ),
            // Provider mutation — PAC script wins over manual and system.
            ProxyResolutionRecord::new(
                "remote:proxy_resolution:provider_mutation:0001",
                at,
                SurfaceClass::ProviderMutation,
                OriginScopeClass::ManagedTenant,
                EgressClass::ManagedEndpoint,
                vec![
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::PacScript,
                        true,
                        true,
                        "proxy_source:provider_mutation:pac:0001",
                        "PAC script resolves the provider route; highest precedence tier.",
                    ),
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::ManualPinned,
                        true,
                        false,
                        "proxy_source:provider_mutation:manual:0001",
                        "Manual proxy present but outranked by the PAC-resolved route.",
                    ),
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::SystemProxy,
                        true,
                        false,
                        "proxy_source:provider_mutation:system:0001",
                        "System proxy present but outranked by the PAC-resolved route.",
                    ),
                ],
                ProxyResolutionOutcomeClass::Resolved,
                None,
                MirrorOfflineBehaviorClass::DenyAll,
                Some("epoch:provider_mutation:2026-06-01"),
                true,
                "Provider mutation proxy resolved to the PAC-scripted route, which outranks the \
                 available manual and system proxies; denies all when offline; local work \
                 continues.",
            ),
            // Sync / offboarding — system proxy; degraded-then-deny offline.
            ProxyResolutionRecord::new(
                "remote:proxy_resolution:sync_offboarding:0001",
                at,
                SurfaceClass::SyncOffboarding,
                OriginScopeClass::ManagedTenant,
                EgressClass::ManagedEndpoint,
                vec![
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::SystemProxy,
                        true,
                        true,
                        "proxy_source:sync_offboarding:system:0001",
                        "System proxy is the highest available tier for sync traffic.",
                    ),
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::DirectNoProxy,
                        false,
                        false,
                        "proxy_source:sync_offboarding:direct:0001",
                        "Direct egress is not a declared fallback for managed sync.",
                    ),
                ],
                ProxyResolutionOutcomeClass::Resolved,
                None,
                MirrorOfflineBehaviorClass::OfflineGrace,
                Some("epoch:sync_offboarding:2026-06-01"),
                true,
                "Sync proxy resolved to the system proxy; when the route is offline the surface \
                 holds within its offline-grace window rather than direct-connecting; local data \
                 is retained.",
            ),
            // Remote preview route — contradictory proxy state -> typed denial.
            ProxyResolutionRecord::new(
                "remote:proxy_resolution:remote_preview_route:0001",
                at,
                SurfaceClass::RemotePreviewRoute,
                OriginScopeClass::FirstParty,
                EgressClass::ManagedEndpoint,
                vec![
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::ManualPinned,
                        true,
                        false,
                        "proxy_source:remote_preview_route:manual:0001",
                        "Manual proxy and environment proxy disagree; policy forbids a silent pick.",
                    ),
                    ProxyCandidate::new(
                        ProxyResolutionTierClass::EnvironmentProxy,
                        true,
                        false,
                        "proxy_source:remote_preview_route:env:0001",
                        "Environment proxy conflicts with the policy-pinned proxy.",
                    ),
                ],
                ProxyResolutionOutcomeClass::DeniedProxyResolution,
                Some(ProxyResolutionDenialClass::ContradictoryProxyState),
                MirrorOfflineBehaviorClass::DenyAll,
                Some("epoch:remote_preview_route:2026-06-01"),
                true,
                "Remote preview proxy resolution found a contradictory proxy state (manual and \
                 environment proxies disagree); resolution is denied with a typed \
                 deny_proxy_resolution reason rather than silently direct-connecting; the local \
                 workspace continues without the preview.",
            ),
        ],
    }
}
