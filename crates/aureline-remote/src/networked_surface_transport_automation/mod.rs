//! Canonical transport-denial vocabulary, network-activity filters,
//! route/origin history joins, and redaction-safe automation packets for the
//! claimed M5 networked surfaces.
//!
//! The sibling [`crate::networked_surface_transport_decision`] module emits one
//! inspectable transport decision per network-capable *action*,
//! [`crate::networked_surface_transport_explainability`] projects that stream
//! into product-grade posture, ledger, and explain views, and
//! [`crate::networked_surface_mirror_offline_continuity`] freezes per-family
//! mirror/offline route handling. This module is the **audit and automation
//! truth** layer: it gives network activity history, support exports, and
//! headless automation one stable denial vocabulary and one redaction-safe
//! route/origin history so an M5 network failure is explained from a single
//! packet set rather than reconstructed from per-feature logs.
//!
//! The layer answers four questions the other layers leave open:
//!
//! 1. **What is the canonical denial vocabulary?** Every network failure across
//!    every claimed M5 surface resolves to exactly one
//!    [`TransportDenialClass`] token — `proxy_misconfigured`,
//!    `proxy_auth_required`, `ca_untrusted`, `ssh_host_key_unknown`,
//!    `egress_blocked_policy`, `mirror_unreachable`, `offline_mode`, or
//!    `origin_scope_ambiguous` — so the per-feature proxy, trust, and matrix
//!    denial classes all map into one reusable vocabulary.
//! 2. **How is activity filtered?** A [`NetworkActivityRecord`] carries the
//!    surface, origin scope, endpoint class, egress class, route choice,
//!    allow/deny disposition, and canonical denial code for one network-capable
//!    action, and [`ActivityFilter`] selects records by any of those dimensions
//!    so product, CLI, and support views filter identically.
//! 3. **How does route/origin history join?** [`RouteOriginJoinRow`] aggregates
//!    the activity history by `(route_choice, origin_scope)` so an operator can
//!    see, per route and origin, how many actions were allowed, denied, or
//!    deferred and which denial codes appeared — without scanning raw logs.
//! 4. **Is automation redaction-safe?** Every record, row, join, and export
//!    carries closed-vocabulary tokens, opaque refs, and counts only. No raw
//!    URLs, hostnames, ports, paths, query strings, cookies, headers, bearer or
//!    session tokens, private certificate bytes, or SSH private material ever
//!    cross the boundary.
//!
//! These records aggregate into one stable proof packet
//! ([`TransportAutomationPage`]) consumed by product UI, CLI/headless output,
//! diagnostics, support exports, and admin/audit surfaces.
//!
//! The stable claim holds when **all** of the following conditions are verified
//! simultaneously:
//!
//! 1. Every required M5 surface has at least one activity record.
//! 2. No raw private material is present on any record.
//! 3. Every record resolved through the shared transport-governance layer
//!    (`no_bypass: true`).
//! 4. Any deferred record queues only an explicitly idempotent action.
//! 5. Every denied record carries a non-`none` canonical denial code.
//! 6. Every allowed record carries the `none` canonical denial code.
//! 7. The page exposes the complete canonical denial vocabulary.
//!
//! Three conditions force [`AutomationQualificationClass::Withdrawn`]
//! immediately and cannot be overridden: raw private material exposed, a bypass
//! of the shared governance layer, or a non-idempotent action queued for
//! offline replay. A missing required surface narrows to
//! [`AutomationQualificationClass::Preview`]; the remaining gaps narrow the
//! affected row to `Beta`, which lets release and support tooling automatically
//! narrow stale or under-qualified rows before publication.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/network/networked-surface-transport-automation.md`
//! - Artifact:
//!   `artifacts/network/networked-surface-transport-automation.md`
//! - Schema:
//!   `schemas/network/networked_surface_transport_automation.schema.json`
//! - Contract ref: [`TRANSPORT_AUTOMATION_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::networked_surface_proxy_resolution::ProxyResolutionDenialClass;
use crate::networked_surface_transport_matrix::{
    DenialReasonClass, EgressClass, EndpointClass, OriginScopeClass, RouteChoiceClass,
    SurfaceClass, REQUIRED_SURFACES,
};
use crate::networked_surface_transport_trust::TrustDenialClass;

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const TRANSPORT_AUTOMATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const TRANSPORT_AUTOMATION_SHARED_CONTRACT_REF: &str =
    "remote:networked_surface_transport_automation:v1";

/// Record-kind tag for [`TransportAutomationPage`] payloads.
pub const TRANSPORT_AUTOMATION_PAGE_RECORD_KIND: &str =
    "remote_networked_surface_transport_automation_page_record";

/// Record-kind tag for [`NetworkActivityRecord`] payloads.
pub const TRANSPORT_AUTOMATION_ACTIVITY_RECORD_KIND: &str =
    "remote_networked_surface_transport_automation_activity_record";

/// Record-kind tag for [`TransportAutomationRow`] payloads.
pub const TRANSPORT_AUTOMATION_ROW_RECORD_KIND: &str =
    "remote_networked_surface_transport_automation_row_record";

/// Record-kind tag for [`RouteOriginJoinRow`] payloads.
pub const TRANSPORT_AUTOMATION_JOIN_RECORD_KIND: &str =
    "remote_networked_surface_transport_automation_route_origin_join_record";

/// Record-kind tag for [`TransportAutomationDefect`] payloads.
pub const TRANSPORT_AUTOMATION_DEFECT_RECORD_KIND: &str =
    "remote_networked_surface_transport_automation_defect_record";

/// Record-kind tag for [`TransportAutomationSummary`] payloads.
pub const TRANSPORT_AUTOMATION_SUMMARY_RECORD_KIND: &str =
    "remote_networked_surface_transport_automation_summary_record";

/// Record-kind tag for [`TransportAutomationSupportExport`] payloads.
pub const TRANSPORT_AUTOMATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_networked_surface_transport_automation_support_export_record";

/// Repo-relative path of the stable doc for this automation layer.
pub const TRANSPORT_AUTOMATION_DOC_REF: &str =
    "docs/network/networked-surface-transport-automation.md";

/// Repo-relative path of the artifact summary for this automation layer.
pub const TRANSPORT_AUTOMATION_ARTIFACT_REF: &str =
    "artifacts/network/networked-surface-transport-automation.md";

/// Repo-relative ref to the canonical evidence index this layer binds into for
/// the closeout certification lane.
pub const TRANSPORT_AUTOMATION_EVIDENCE_INDEX_REF: &str =
    "artifacts/release/m5/xt12-evidence-index.md";

/// Stable, ordered catalog of the field names an activity record renders.
///
/// Product surfaces, CLI/headless output, and support exports MUST all render
/// an activity record through this exact ordered field set, so the tokens a
/// user reads in the UI are identical to the ones CLI output and support
/// packets quote. [`NetworkActivityRecord::render_fields`] is the single
/// renderer; [`TransportAutomationPage::all_records_at_field_parity`] verifies
/// the rendered field names match this catalog.
pub const ACTIVITY_FIELD_NAMES: [&str; 8] = [
    "surface",
    "origin_scope",
    "endpoint_class",
    "egress_class",
    "route_choice",
    "disposition",
    "denial_code",
    "occurred_at",
];

// ---------------------------------------------------------------------------
// Canonical transport-denial vocabulary
// ---------------------------------------------------------------------------

/// Canonical, closed denial vocabulary that every claimed M5 networked surface
/// reuses.
///
/// This is the single audit/automation vocabulary the row exists to make
/// authoritative. Per-feature denial classes — the matrix
/// [`DenialReasonClass`], the proxy [`ProxyResolutionDenialClass`], and the
/// trust [`TrustDenialClass`] — all map into this vocabulary via
/// [`TransportDenialClass::from_matrix_denial`],
/// [`TransportDenialClass::from_proxy_denial`], and
/// [`TransportDenialClass::from_trust_denial`], so a network failure is
/// explained with one stable token rather than a per-feature error string.
///
/// [`TransportDenialClass::None`] is the sentinel for "no denial" (an allowed
/// action). The eight denial codes in [`REQUIRED_DENIAL_CODES`] are the codes a
/// claimed M5 row must be able to surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportDenialClass {
    /// No denial — the action was allowed.
    None,
    /// The resolved proxy configuration is invalid or self-contradictory.
    ProxyMisconfigured,
    /// The selected proxy demands authentication that was not satisfied.
    ProxyAuthRequired,
    /// The endpoint's certificate chain could not be trusted.
    CaUntrusted,
    /// The SSH host key is unknown, changed, or revoked.
    SshHostKeyUnknown,
    /// Transport policy forbids egress for this action.
    EgressBlockedPolicy,
    /// The declared signed mirror could not be reached or did not match.
    MirrorUnreachable,
    /// The surface is offline and no in-policy route is available.
    OfflineMode,
    /// The endpoint's origin ownership scope could not be resolved
    /// unambiguously.
    OriginScopeAmbiguous,
}

/// The eight canonical denial codes a claimed M5 row must be able to surface.
///
/// [`TransportDenialClass::None`] is intentionally excluded: it is the
/// "no denial" sentinel, not a denial code.
pub const REQUIRED_DENIAL_CODES: [TransportDenialClass; 8] = [
    TransportDenialClass::ProxyMisconfigured,
    TransportDenialClass::ProxyAuthRequired,
    TransportDenialClass::CaUntrusted,
    TransportDenialClass::SshHostKeyUnknown,
    TransportDenialClass::EgressBlockedPolicy,
    TransportDenialClass::MirrorUnreachable,
    TransportDenialClass::OfflineMode,
    TransportDenialClass::OriginScopeAmbiguous,
];

impl TransportDenialClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ProxyMisconfigured => "proxy_misconfigured",
            Self::ProxyAuthRequired => "proxy_auth_required",
            Self::CaUntrusted => "ca_untrusted",
            Self::SshHostKeyUnknown => "ssh_host_key_unknown",
            Self::EgressBlockedPolicy => "egress_blocked_policy",
            Self::MirrorUnreachable => "mirror_unreachable",
            Self::OfflineMode => "offline_mode",
            Self::OriginScopeAmbiguous => "origin_scope_ambiguous",
        }
    }

    /// Human-readable denial label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "No denial",
            Self::ProxyMisconfigured => "Proxy misconfigured",
            Self::ProxyAuthRequired => "Proxy authentication required",
            Self::CaUntrusted => "Certificate authority untrusted",
            Self::SshHostKeyUnknown => "SSH host key unknown",
            Self::EgressBlockedPolicy => "Egress blocked by policy",
            Self::MirrorUnreachable => "Mirror unreachable",
            Self::OfflineMode => "Offline mode",
            Self::OriginScopeAmbiguous => "Origin scope ambiguous",
        }
    }

    /// Returns `true` when this is the `none` sentinel (no denial).
    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` when this is an actual denial code (not the sentinel).
    pub const fn is_denial(self) -> bool {
        !self.is_none()
    }

    /// The canonical denial vocabulary catalog (the eight required codes).
    pub fn canonical_catalog() -> Vec<TransportDenialClass> {
        REQUIRED_DENIAL_CODES.to_vec()
    }

    /// Map a matrix [`DenialReasonClass`] into the canonical vocabulary.
    pub const fn from_matrix_denial(reason: DenialReasonClass) -> Self {
        match reason {
            DenialReasonClass::PolicyBlocked
            | DenialReasonClass::EgressClassForbidden
            | DenialReasonClass::NonIdempotentReplayRejected => Self::EgressBlockedPolicy,
            DenialReasonClass::TrustProofMissing | DenialReasonClass::TrustProofExpired => {
                Self::CaUntrusted
            }
            DenialReasonClass::AuthPostureRejected => Self::ProxyAuthRequired,
            DenialReasonClass::ProxyUnreachable => Self::ProxyMisconfigured,
            DenialReasonClass::MirrorRootMismatch => Self::MirrorUnreachable,
            DenialReasonClass::OfflineNoFallback => Self::OfflineMode,
        }
    }

    /// Map a proxy [`ProxyResolutionDenialClass`] into the canonical vocabulary.
    pub const fn from_proxy_denial(reason: ProxyResolutionDenialClass) -> Self {
        match reason {
            ProxyResolutionDenialClass::ContradictoryProxyState
            | ProxyResolutionDenialClass::PrivateProxyStackDetected
            | ProxyResolutionDenialClass::PacUnreachable
            | ProxyResolutionDenialClass::ProxyUnreachable => Self::ProxyMisconfigured,
            ProxyResolutionDenialClass::DirectCaOverrideDetected => Self::CaUntrusted,
            ProxyResolutionDenialClass::UndeclaredPublicFallback
            | ProxyResolutionDenialClass::NoResolvableRoute
            | ProxyResolutionDenialClass::MirrorOnlyNoProxyPermitted
            | ProxyResolutionDenialClass::PolicyEpochUnavailable => Self::EgressBlockedPolicy,
        }
    }

    /// Map a trust [`TrustDenialClass`] into the canonical vocabulary.
    pub const fn from_trust_denial(reason: TrustDenialClass) -> Self {
        match reason {
            TrustDenialClass::TrustStoreUnavailable
            | TrustDenialClass::CaBundleMissing
            | TrustDenialClass::CaBundleStale
            | TrustDenialClass::ManagedBundleUnverified
            | TrustDenialClass::ClientCertRequiredAbsent
            | TrustDenialClass::TrustRootExpired
            | TrustDenialClass::PinSetMismatch => Self::CaUntrusted,
            TrustDenialClass::HostProofMissing
            | TrustDenialClass::HostProofChanged
            | TrustDenialClass::HostProofRevoked => Self::SshHostKeyUnknown,
            TrustDenialClass::MirrorRootMismatch => Self::MirrorUnreachable,
        }
    }
}

// ---------------------------------------------------------------------------
// Activity disposition vocabulary
// ---------------------------------------------------------------------------

/// Allow/deny outcome recorded for a network-capable action.
///
/// This is the primary axis the network-activity ledger is filtered on: an
/// operator asks "show me every denied action" or "every deferred action"
/// across all surfaces with one stable token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityDispositionClass {
    /// The action was allowed and left the boundary.
    Allowed,
    /// The action was denied with a typed denial code.
    Denied,
    /// The action was queued for idempotent offline replay.
    Deferred,
}

impl ActivityDispositionClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Denied => "denied",
            Self::Deferred => "deferred",
        }
    }

    /// Returns `true` when the action was allowed.
    pub const fn is_allowed(self) -> bool {
        matches!(self, Self::Allowed)
    }

    /// Returns `true` when the action was denied.
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::Denied)
    }

    /// Returns `true` when the action was deferred for replay.
    pub const fn is_deferred(self) -> bool {
        matches!(self, Self::Deferred)
    }
}

// ---------------------------------------------------------------------------
// Activity filter dimension vocabulary
// ---------------------------------------------------------------------------

/// The dimensions a network-activity ledger may be filtered on.
///
/// Product, CLI, and support views all render the same filter chips from this
/// closed vocabulary, so a saved filter is portable across surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityFilterDimension {
    /// Filter by networked surface.
    Surface,
    /// Filter by origin ownership scope.
    OriginScope,
    /// Filter by endpoint class.
    EndpointClass,
    /// Filter by physical route choice.
    RouteChoice,
    /// Filter by allow/deny disposition.
    Disposition,
    /// Filter by canonical denial code.
    DenialCode,
}

impl ActivityFilterDimension {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::OriginScope => "origin_scope",
            Self::EndpointClass => "endpoint_class",
            Self::RouteChoice => "route_choice",
            Self::Disposition => "disposition",
            Self::DenialCode => "denial_code",
        }
    }
}

/// Every filter dimension, in render order.
pub const ACTIVITY_FILTER_DIMENSIONS: [ActivityFilterDimension; 6] = [
    ActivityFilterDimension::Surface,
    ActivityFilterDimension::OriginScope,
    ActivityFilterDimension::EndpointClass,
    ActivityFilterDimension::RouteChoice,
    ActivityFilterDimension::Disposition,
    ActivityFilterDimension::DenialCode,
];

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall automation page and for
/// individual activity rows.
///
/// The tier is derived, not asserted: it is computed by comparing the audit
/// defect list against the stability conditions. A caller may never bump a row
/// to `Stable` without a clean audit and complete surface coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationQualificationClass {
    /// All stability conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required surface has no activity record; the coverage gap prevents a
    /// beta claim for the missing surface.
    Preview,
    /// A hard guardrail was violated; the packet is withdrawn immediately and
    /// cannot be overridden.
    Withdrawn,
}

impl AutomationQualificationClass {
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
/// [`AutomationQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationNarrowReasonClass {
    /// No narrowing — the row qualifies stable.
    NotNarrowed,
    /// A record carries `raw_private_material_excluded: false`; withdraws the
    /// packet immediately.
    RawPrivateMaterialExposed,
    /// A record resolved outside the shared transport-governance layer
    /// (`no_bypass: false`); withdraws the packet immediately.
    BypassedSharedGovernance,
    /// A deferred record queued a non-idempotent action for offline replay;
    /// withdraws the packet immediately.
    NonIdempotentReplayQueued,
    /// A required surface has no activity record; narrows to preview.
    RequiredSurfaceMissing,
    /// A denied record carries no canonical denial code.
    DeniedWithoutCanonicalCode,
    /// A record's disposition and denial code disagree (allowed-with-code or
    /// denied-with-none).
    DispositionDenialCodeMismatch,
    /// The page does not expose the complete canonical denial vocabulary.
    DenialVocabularyIncomplete,
}

impl AutomationNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::BypassedSharedGovernance => "bypassed_shared_governance",
            Self::NonIdempotentReplayQueued => "non_idempotent_replay_queued",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::DeniedWithoutCanonicalCode => "denied_without_canonical_code",
            Self::DispositionDenialCodeMismatch => "disposition_denial_code_mismatch",
            Self::DenialVocabularyIncomplete => "denial_vocabulary_incomplete",
        }
    }

    /// Returns `true` when this reason is a hard guardrail that withdraws the
    /// packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::RawPrivateMaterialExposed
                | Self::BypassedSharedGovernance
                | Self::NonIdempotentReplayQueued
        )
    }

    /// Returns `true` when this reason narrows to preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(self, Self::RequiredSurfaceMissing)
    }
}

// ---------------------------------------------------------------------------
// Network activity record (per action)
// ---------------------------------------------------------------------------

/// Inspectable, redaction-safe audit record for one network-capable action.
///
/// The record binds a networked surface to the origin scope, endpoint class,
/// egress class, route choice, allow/deny disposition, and canonical denial
/// code resolved for one action. No raw endpoint URL, raw hostname, raw
/// credential, or raw payload appears; only closed-vocabulary tokens, opaque
/// refs, and a UTC timestamp cross the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkActivityRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque id for this record.
    pub record_id: String,
    /// Monotonic sequence index within the activity history (for ordering).
    pub sequence: u32,
    /// UTC instant the action was resolved.
    pub occurred_at: String,
    /// Networked surface this action belongs to.
    pub surface: SurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Origin ownership scope of the endpoint.
    pub origin_scope: OriginScopeClass,
    /// Stable token for [`Self::origin_scope`].
    pub origin_scope_token: String,
    /// Endpoint class contacted.
    pub endpoint_class: EndpointClass,
    /// Stable token for [`Self::endpoint_class`].
    pub endpoint_class_token: String,
    /// Egress class enforced.
    pub egress_class: EgressClass,
    /// Stable token for [`Self::egress_class`].
    pub egress_class_token: String,
    /// Physical route choice taken.
    pub route_choice: RouteChoiceClass,
    /// Stable token for [`Self::route_choice`].
    pub route_choice_token: String,
    /// Allow/deny disposition.
    pub disposition: ActivityDispositionClass,
    /// Stable token for [`Self::disposition`].
    pub disposition_token: String,
    /// Canonical denial code; [`TransportDenialClass::None`] when allowed.
    pub denial_code: TransportDenialClass,
    /// Stable token for [`Self::denial_code`].
    pub denial_code_token: String,
    /// `true` when the action this record governs is idempotent.
    pub action_is_idempotent: bool,
    /// `true` when the record resolved through the shared transport-governance
    /// layer and shipped no private proxy stack, direct CA override, undeclared
    /// public fallback, or hidden direct-connect retry.
    pub no_bypass: bool,
    /// Plain-language summary safe for UI, support export, and diagnostics.
    pub summary: String,
    /// `true` when no raw private material is present on this record.
    pub raw_private_material_excluded: bool,
}

impl NetworkActivityRecord {
    /// Construct an activity record, filling in token fields from the typed
    /// enum values.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        record_id: impl Into<String>,
        sequence: u32,
        occurred_at: impl Into<String>,
        surface: SurfaceClass,
        origin_scope: OriginScopeClass,
        endpoint_class: EndpointClass,
        egress_class: EgressClass,
        route_choice: RouteChoiceClass,
        disposition: ActivityDispositionClass,
        denial_code: TransportDenialClass,
        action_is_idempotent: bool,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: TRANSPORT_AUTOMATION_ACTIVITY_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_AUTOMATION_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_AUTOMATION_SHARED_CONTRACT_REF.to_owned(),
            record_id: record_id.into(),
            sequence,
            occurred_at: occurred_at.into(),
            surface,
            surface_token: surface.as_str().to_owned(),
            surface_label: surface.label().to_owned(),
            origin_scope,
            origin_scope_token: origin_scope.as_str().to_owned(),
            endpoint_class,
            endpoint_class_token: endpoint_class.as_str().to_owned(),
            egress_class,
            egress_class_token: egress_class.as_str().to_owned(),
            route_choice,
            route_choice_token: route_choice.as_str().to_owned(),
            disposition,
            disposition_token: disposition.as_str().to_owned(),
            denial_code,
            denial_code_token: denial_code.as_str().to_owned(),
            action_is_idempotent,
            no_bypass: true,
            summary: summary.into(),
            raw_private_material_excluded: true,
        }
    }

    /// Returns `true` when the disposition and denial code are consistent.
    ///
    /// An allowed action must carry [`TransportDenialClass::None`]; a denied
    /// action must carry a real denial code. A deferred action may carry a
    /// code (typically `offline_mode`) or `none`.
    pub fn disposition_code_consistent(&self) -> bool {
        match self.disposition {
            ActivityDispositionClass::Allowed => self.denial_code.is_none(),
            ActivityDispositionClass::Denied => self.denial_code.is_denial(),
            ActivityDispositionClass::Deferred => true,
        }
    }

    /// Returns `true` when a denied action carries a canonical denial code.
    pub fn denied_has_canonical_code(&self) -> bool {
        !self.disposition.is_denied() || self.denial_code.is_denial()
    }

    /// Render the record's fields as ordered `(name, value)` pairs through the
    /// shared [`ACTIVITY_FIELD_NAMES`] catalog.
    pub fn render_fields(&self) -> Vec<(String, String)> {
        let values: [&str; ACTIVITY_FIELD_NAMES.len()] = [
            self.surface_token.as_str(),
            self.origin_scope_token.as_str(),
            self.endpoint_class_token.as_str(),
            self.egress_class_token.as_str(),
            self.route_choice_token.as_str(),
            self.disposition_token.as_str(),
            self.denial_code_token.as_str(),
            self.occurred_at.as_str(),
        ];
        ACTIVITY_FIELD_NAMES
            .iter()
            .zip(values.iter())
            .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
            .collect()
    }

    /// Render the record as CLI/headless `key=value` lines.
    pub fn render_cli_lines(&self) -> Vec<String> {
        self.render_fields()
            .into_iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect()
    }

    /// Render the record as support-export `key: value` lines.
    pub fn render_support_lines(&self) -> Vec<String> {
        self.render_fields()
            .into_iter()
            .map(|(name, value)| format!("{name}: {value}"))
            .collect()
    }

    /// Returns `true` when the rendered field names match
    /// [`ACTIVITY_FIELD_NAMES`] in order.
    pub fn fields_at_parity(&self) -> bool {
        let rendered = self.render_fields();
        rendered.len() == ACTIVITY_FIELD_NAMES.len()
            && rendered
                .iter()
                .zip(ACTIVITY_FIELD_NAMES.iter())
                .all(|((name, _), expected)| name == *expected)
    }

    /// The stable token for one filter dimension on this record.
    pub fn dimension_token(&self, dimension: ActivityFilterDimension) -> &str {
        match dimension {
            ActivityFilterDimension::Surface => &self.surface_token,
            ActivityFilterDimension::OriginScope => &self.origin_scope_token,
            ActivityFilterDimension::EndpointClass => &self.endpoint_class_token,
            ActivityFilterDimension::RouteChoice => &self.route_choice_token,
            ActivityFilterDimension::Disposition => &self.disposition_token,
            ActivityFilterDimension::DenialCode => &self.denial_code_token,
        }
    }
}

// ---------------------------------------------------------------------------
// Activity filter
// ---------------------------------------------------------------------------

/// A redaction-safe network-activity filter.
///
/// Each field selects records whose corresponding token matches. A `None` field
/// is a wildcard. The filter carries only closed-vocabulary enum values, so a
/// saved filter is portable across product, CLI, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ActivityFilter {
    /// Restrict to one surface.
    pub surface: Option<SurfaceClass>,
    /// Restrict to one origin scope.
    pub origin_scope: Option<OriginScopeClass>,
    /// Restrict to one endpoint class.
    pub endpoint_class: Option<EndpointClass>,
    /// Restrict to one route choice.
    pub route_choice: Option<RouteChoiceClass>,
    /// Restrict to one disposition.
    pub disposition: Option<ActivityDispositionClass>,
    /// Restrict to one canonical denial code.
    pub denial_code: Option<TransportDenialClass>,
}

impl ActivityFilter {
    /// An empty filter that matches every record.
    pub fn all() -> Self {
        Self::default()
    }

    /// Restrict to a surface.
    pub fn with_surface(mut self, surface: SurfaceClass) -> Self {
        self.surface = Some(surface);
        self
    }

    /// Restrict to an origin scope.
    pub fn with_origin_scope(mut self, origin_scope: OriginScopeClass) -> Self {
        self.origin_scope = Some(origin_scope);
        self
    }

    /// Restrict to an endpoint class.
    pub fn with_endpoint_class(mut self, endpoint_class: EndpointClass) -> Self {
        self.endpoint_class = Some(endpoint_class);
        self
    }

    /// Restrict to a route choice.
    pub fn with_route_choice(mut self, route_choice: RouteChoiceClass) -> Self {
        self.route_choice = Some(route_choice);
        self
    }

    /// Restrict to a disposition.
    pub fn with_disposition(mut self, disposition: ActivityDispositionClass) -> Self {
        self.disposition = Some(disposition);
        self
    }

    /// Restrict to a canonical denial code.
    pub fn with_denial_code(mut self, denial_code: TransportDenialClass) -> Self {
        self.denial_code = Some(denial_code);
        self
    }

    /// Returns `true` when the record satisfies every set field.
    pub fn matches(&self, record: &NetworkActivityRecord) -> bool {
        self.surface.map_or(true, |s| s == record.surface)
            && self.origin_scope.map_or(true, |s| s == record.origin_scope)
            && self
                .endpoint_class
                .map_or(true, |e| e == record.endpoint_class)
            && self.route_choice.map_or(true, |r| r == record.route_choice)
            && self.disposition.map_or(true, |d| d == record.disposition)
            && self.denial_code.map_or(true, |c| c == record.denial_code)
    }

    /// Apply the filter to a slice of records, returning the matching subset.
    pub fn apply<'a>(
        &self,
        records: &'a [NetworkActivityRecord],
    ) -> Vec<&'a NetworkActivityRecord> {
        records.iter().filter(|r| self.matches(r)).collect()
    }
}

// ---------------------------------------------------------------------------
// Filter facet (available filter values)
// ---------------------------------------------------------------------------

/// The distinct token values present on one filter dimension across the
/// activity history.
///
/// Product, CLI, and support views render the same filter chips from this set,
/// so the filter affordances stay identical across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityFilterFacet {
    /// The filter dimension.
    pub dimension: ActivityFilterDimension,
    /// Stable token for [`Self::dimension`].
    pub dimension_token: String,
    /// Distinct, sorted token values present for this dimension.
    pub values: Vec<String>,
}

// ---------------------------------------------------------------------------
// Route/origin history join
// ---------------------------------------------------------------------------

/// An aggregate join of the activity history keyed by `(route_choice,
/// origin_scope)`.
///
/// This is the route/origin history join the row exists to make explicit: per
/// route and origin, how many actions were allowed, denied, or deferred, and
/// which canonical denial codes appeared — all without scanning raw logs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteOriginJoinRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable join id.
    pub join_id: String,
    /// Route choice token for this join key.
    pub route_choice_token: String,
    /// Origin scope token for this join key.
    pub origin_scope_token: String,
    /// Total activity records on this join key.
    pub total_count: usize,
    /// Allowed records on this join key.
    pub allowed_count: usize,
    /// Denied records on this join key.
    pub denied_count: usize,
    /// Deferred records on this join key.
    pub deferred_count: usize,
    /// Distinct surface tokens present on this join key.
    pub surfaces_present: Vec<String>,
    /// Distinct canonical denial-code tokens present on this join key
    /// (excluding `none`).
    pub denial_codes_present: Vec<String>,
}

// ---------------------------------------------------------------------------
// Activity snapshot (aggregate of all records)
// ---------------------------------------------------------------------------

/// Aggregate of all network-activity records for the covered surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportAutomationSnapshot {
    /// All activity records in the snapshot, in sequence order.
    pub records: Vec<NetworkActivityRecord>,
}

impl TransportAutomationSnapshot {
    /// Returns the set of surface tokens covered by this snapshot.
    pub fn covered_surface_tokens(&self) -> BTreeSet<&str> {
        self.records
            .iter()
            .map(|r| r.surface_token.as_str())
            .collect()
    }

    /// Returns the set of canonical denial-code tokens present (excluding
    /// `none`).
    pub fn denial_codes_present(&self) -> BTreeSet<&str> {
        self.records
            .iter()
            .filter(|r| r.denial_code.is_denial())
            .map(|r| r.denial_code_token.as_str())
            .collect()
    }

    /// Build the available filter facets across every dimension.
    pub fn filter_facets(&self) -> Vec<ActivityFilterFacet> {
        ACTIVITY_FILTER_DIMENSIONS
            .iter()
            .map(|dimension| {
                let values: BTreeSet<String> = self
                    .records
                    .iter()
                    .map(|r| r.dimension_token(*dimension).to_owned())
                    .collect();
                ActivityFilterFacet {
                    dimension: *dimension,
                    dimension_token: dimension.as_str().to_owned(),
                    values: values.into_iter().collect(),
                }
            })
            .collect()
    }

    /// Build the route/origin history joins from the activity records.
    pub fn route_origin_joins(&self) -> Vec<RouteOriginJoinRow> {
        // Group by (route_choice_token, origin_scope_token), in sorted order.
        let mut keys: BTreeSet<(&str, &str)> = BTreeSet::new();
        for record in &self.records {
            keys.insert((
                record.route_choice_token.as_str(),
                record.origin_scope_token.as_str(),
            ));
        }
        keys.into_iter()
            .map(|(route, origin)| {
                let matching: Vec<&NetworkActivityRecord> = self
                    .records
                    .iter()
                    .filter(|r| r.route_choice_token == route && r.origin_scope_token == origin)
                    .collect();
                let allowed = matching
                    .iter()
                    .filter(|r| r.disposition.is_allowed())
                    .count();
                let denied = matching
                    .iter()
                    .filter(|r| r.disposition.is_denied())
                    .count();
                let deferred = matching
                    .iter()
                    .filter(|r| r.disposition.is_deferred())
                    .count();
                let surfaces: BTreeSet<String> =
                    matching.iter().map(|r| r.surface_token.clone()).collect();
                let denial_codes: BTreeSet<String> = matching
                    .iter()
                    .filter(|r| r.denial_code.is_denial())
                    .map(|r| r.denial_code_token.clone())
                    .collect();
                RouteOriginJoinRow {
                    record_kind: TRANSPORT_AUTOMATION_JOIN_RECORD_KIND.to_owned(),
                    schema_version: TRANSPORT_AUTOMATION_SCHEMA_VERSION,
                    shared_contract_ref: TRANSPORT_AUTOMATION_SHARED_CONTRACT_REF.to_owned(),
                    join_id: format!("remote:transport_automation:join:{route}:{origin}"),
                    route_choice_token: route.to_owned(),
                    origin_scope_token: origin.to_owned(),
                    total_count: matching.len(),
                    allowed_count: allowed,
                    denied_count: denied,
                    deferred_count: deferred,
                    surfaces_present: surfaces.into_iter().collect(),
                    denial_codes_present: denial_codes.into_iter().collect(),
                }
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Automation row (per activity record)
// ---------------------------------------------------------------------------

/// Stability qualification for one activity record in the automation page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportAutomationRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Record id for this row (matches the activity record id).
    pub record_id: String,
    /// Surface token for this row.
    pub surface_token: String,
    /// Origin scope token.
    pub origin_scope_token: String,
    /// Endpoint class token.
    pub endpoint_class_token: String,
    /// Egress class token.
    pub egress_class_token: String,
    /// Route choice token.
    pub route_choice_token: String,
    /// Disposition token.
    pub disposition_token: String,
    /// Canonical denial-code token.
    pub denial_code_token: String,
    /// `true` when the record resolved through the shared governance layer.
    pub no_bypass: bool,
    /// `true` when a deferred action is idempotent (always `true` when not
    /// deferred).
    pub replay_idempotent_only: bool,
    /// `true` when the disposition and denial code are consistent.
    pub disposition_code_consistent: bool,
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

/// Aggregate banner emitted with the automation page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TransportAutomationSummary {
    /// Total row count (one row per activity record).
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Total activity records.
    pub activity_count: usize,
    /// Allowed activity records.
    pub allowed_count: usize,
    /// Denied activity records.
    pub denied_count: usize,
    /// Deferred activity records.
    pub deferred_count: usize,
    /// Surface tokens covered by the page.
    pub surfaces_covered: Vec<String>,
    /// Number of records that resolved through the shared governance layer.
    pub no_bypass_count: usize,
    /// Activity counts by disposition token.
    pub disposition_counts: BTreeMap<String, usize>,
    /// Activity counts by canonical denial-code token (denials only).
    pub denial_code_counts: BTreeMap<String, usize>,
    /// Canonical denial-code tokens the page exposes as reusable vocabulary.
    pub denial_vocabulary: Vec<String>,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl TransportAutomationSummary {
    fn build(
        rows: &[TransportAutomationRow],
        snapshot: &TransportAutomationSnapshot,
        defects: &[TransportAutomationDefect],
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
            AutomationQualificationClass::Withdrawn
        } else if has_preview || preview > 0 {
            AutomationQualificationClass::Preview
        } else if !defects.is_empty() || beta > 0 {
            AutomationQualificationClass::Beta
        } else {
            AutomationQualificationClass::Stable
        };
        let mut disposition_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut denial_code_counts: BTreeMap<String, usize> = BTreeMap::new();
        for record in &snapshot.records {
            *disposition_counts
                .entry(record.disposition_token.clone())
                .or_insert(0) += 1;
            if record.denial_code.is_denial() {
                *denial_code_counts
                    .entry(record.denial_code_token.clone())
                    .or_insert(0) += 1;
            }
        }
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            activity_count: snapshot.records.len(),
            allowed_count: snapshot
                .records
                .iter()
                .filter(|r| r.disposition.is_allowed())
                .count(),
            denied_count: snapshot
                .records
                .iter()
                .filter(|r| r.disposition.is_denied())
                .count(),
            deferred_count: snapshot
                .records
                .iter()
                .filter(|r| r.disposition.is_deferred())
                .count(),
            surfaces_covered: snapshot
                .covered_surface_tokens()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            no_bypass_count: snapshot.records.iter().filter(|r| r.no_bypass).count(),
            disposition_counts,
            denial_code_counts,
            denial_vocabulary: TransportDenialClass::canonical_catalog()
                .into_iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the automation page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportAutomationDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: AutomationNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (activity record id, surface token, or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl TransportAutomationDefect {
    fn new(
        narrow_reason: AutomationNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: TRANSPORT_AUTOMATION_DEFECT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_AUTOMATION_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_AUTOMATION_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "remote:defect:networked-surface-transport-automation:{}:{}",
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
// Automation page (proof packet)
// ---------------------------------------------------------------------------

/// Stable transport audit and automation proof packet for the claimed M5
/// networked surfaces.
///
/// The packet is the single inspectable record that proves M5 network activity,
/// support export, and headless automation share one canonical denial
/// vocabulary, one redaction-safe route/origin history, and one set of activity
/// filters. Dashboards, docs, Help/About surfaces, CLI/headless output, support
/// exports, release tooling, and diagnostics should ingest this packet rather
/// than reconstructing transport failures from per-feature logs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportAutomationPage {
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
    pub summary: TransportAutomationSummary,
    /// Per-record stability rows.
    pub rows: Vec<TransportAutomationRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<TransportAutomationDefect>,
    /// The activity snapshot embedded as evidence.
    pub activity_snapshot: TransportAutomationSnapshot,
    /// The route/origin history joins derived from the activity snapshot.
    pub route_origin_joins: Vec<RouteOriginJoinRow>,
    /// The available filter facets across every filter dimension.
    pub filter_facets: Vec<ActivityFilterFacet>,
}

impl TransportAutomationPage {
    /// Build the automation page from an activity snapshot.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        activity_snapshot: TransportAutomationSnapshot,
    ) -> Self {
        let defects = audit_snapshot(&activity_snapshot);
        let rows = derive_rows(&activity_snapshot, &defects);
        let summary = TransportAutomationSummary::build(&rows, &activity_snapshot, &defects);
        let route_origin_joins = activity_snapshot.route_origin_joins();
        let filter_facets = activity_snapshot.filter_facets();
        Self {
            record_kind: TRANSPORT_AUTOMATION_PAGE_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_AUTOMATION_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_AUTOMATION_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            evidence_index_ref: TRANSPORT_AUTOMATION_EVIDENCE_INDEX_REF.to_owned(),
            summary,
            rows,
            defects,
            activity_snapshot,
            route_origin_joins,
            filter_facets,
        }
    }

    /// Returns `true` when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token == AutomationQualificationClass::Stable.as_str()
    }

    /// Returns `true` when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// Returns `true` when all required surfaces have at least one record.
    pub fn covers_all_required_surfaces(&self) -> bool {
        let covered = self.activity_snapshot.covered_surface_tokens();
        REQUIRED_SURFACES
            .iter()
            .all(|surface| covered.contains(surface.as_str()))
    }

    /// Returns `true` when the activity history surfaces the complete canonical
    /// denial vocabulary (all eight required codes appear at least once).
    pub fn surfaces_complete_denial_vocabulary(&self) -> bool {
        let present = self.activity_snapshot.denial_codes_present();
        REQUIRED_DENIAL_CODES
            .iter()
            .all(|code| present.contains(code.as_str()))
    }

    /// Returns `true` when the page exposes every canonical denial code as
    /// reusable vocabulary.
    pub fn exposes_full_denial_vocabulary(&self) -> bool {
        REQUIRED_DENIAL_CODES.iter().all(|code| {
            self.summary
                .denial_vocabulary
                .iter()
                .any(|t| t == code.as_str())
        })
    }

    /// Returns `true` when every deferred record queues only an idempotent
    /// action.
    pub fn replay_queues_are_idempotent_only(&self) -> bool {
        self.activity_snapshot
            .records
            .iter()
            .all(|r| !r.disposition.is_deferred() || r.action_is_idempotent)
    }

    /// Returns `true` when every denied record carries a canonical denial code.
    pub fn denied_records_carry_codes(&self) -> bool {
        self.activity_snapshot
            .records
            .iter()
            .all(|r| r.denied_has_canonical_code())
    }

    /// Returns `true` when every record's disposition and denial code agree.
    pub fn all_dispositions_consistent(&self) -> bool {
        self.activity_snapshot
            .records
            .iter()
            .all(|r| r.disposition_code_consistent())
    }

    /// Returns `true` when every record renders at field-catalog parity.
    pub fn all_records_at_field_parity(&self) -> bool {
        self.activity_snapshot
            .records
            .iter()
            .all(|r| r.fields_at_parity())
    }

    /// Apply an [`ActivityFilter`] to the embedded activity history.
    pub fn filter(&self, filter: &ActivityFilter) -> Vec<&NetworkActivityRecord> {
        filter.apply(&self.activity_snapshot.records)
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that carries the automation page plus a metadata-safe
/// defect roll-up.
///
/// No raw URLs, raw hostnames, raw credentials, raw cookies, or raw private key
/// material may appear in this export. Only closed-vocabulary tokens, opaque
/// refs, counts, and plain-language summary sentences cross the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportAutomationSupportExport {
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
    /// The automation page embedded as evidence.
    pub page: TransportAutomationPage,
    /// Narrow-reason class values present in the page's defect list.
    pub narrow_reasons_present: Vec<AutomationNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// `true` when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl TransportAutomationSupportExport {
    /// Wrap an automation page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: TransportAutomationPage,
    ) -> Self {
        let mut reasons: Vec<AutomationNarrowReasonClass> = Vec::new();
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
            record_kind: TRANSPORT_AUTOMATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TRANSPORT_AUTOMATION_SCHEMA_VERSION,
            shared_contract_ref: TRANSPORT_AUTOMATION_SHARED_CONTRACT_REF.to_owned(),
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

/// Re-run the automation audit over the snapshot embedded in a page.
pub fn audit_transport_automation_page(
    page: &TransportAutomationPage,
) -> Vec<TransportAutomationDefect> {
    audit_snapshot(&page.activity_snapshot)
}

/// Validate an automation page; returns `Ok` when the audit is clean.
pub fn validate_transport_automation_page(
    page: &TransportAutomationPage,
) -> Result<(), Vec<TransportAutomationDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal audit logic
// ---------------------------------------------------------------------------

fn audit_snapshot(snapshot: &TransportAutomationSnapshot) -> Vec<TransportAutomationDefect> {
    let mut defects: Vec<TransportAutomationDefect> = Vec::new();

    // Hard guardrails first — any one of these withdraws the packet and makes
    // no further check meaningful.
    for record in &snapshot.records {
        if !record.raw_private_material_excluded {
            defects.push(TransportAutomationDefect::new(
                AutomationNarrowReasonClass::RawPrivateMaterialExposed,
                record.record_id.clone(),
                format!(
                    "activity record '{}' for surface '{}' exposes raw private material; packet is withdrawn",
                    record.record_id, record.surface_token
                ),
            ));
            return defects;
        }
        if !record.no_bypass {
            defects.push(TransportAutomationDefect::new(
                AutomationNarrowReasonClass::BypassedSharedGovernance,
                record.record_id.clone(),
                format!(
                    "activity record '{}' for surface '{}' resolved outside the shared transport-governance layer; packet is withdrawn",
                    record.record_id, record.surface_token
                ),
            ));
            return defects;
        }
        if record.disposition.is_deferred() && !record.action_is_idempotent {
            defects.push(TransportAutomationDefect::new(
                AutomationNarrowReasonClass::NonIdempotentReplayQueued,
                record.record_id.clone(),
                format!(
                    "activity record '{}' for surface '{}' queues a non-idempotent action for offline replay; packet is withdrawn",
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
            defects.push(TransportAutomationDefect::new(
                AutomationNarrowReasonClass::RequiredSurfaceMissing,
                required_surface.as_str(),
                format!(
                    "required surface '{}' has no activity record; packet is narrowed to preview",
                    required_surface.as_str()
                ),
            ));
        }
    }

    // Denial-vocabulary completeness: every required canonical code must be
    // surfaced at least once across the activity history.
    let present_codes = snapshot.denial_codes_present();
    let missing_codes: Vec<&str> = REQUIRED_DENIAL_CODES
        .iter()
        .map(|c| c.as_str())
        .filter(|c| !present_codes.contains(c))
        .collect();
    if !missing_codes.is_empty() {
        defects.push(TransportAutomationDefect::new(
            AutomationNarrowReasonClass::DenialVocabularyIncomplete,
            "page",
            format!(
                "the activity history does not surface every canonical denial code; missing: {}",
                missing_codes.join(", ")
            ),
        ));
    }

    // Per-record checks.
    for record in &snapshot.records {
        if record.disposition.is_denied() && record.denial_code.is_none() {
            defects.push(TransportAutomationDefect::new(
                AutomationNarrowReasonClass::DeniedWithoutCanonicalCode,
                record.record_id.clone(),
                format!(
                    "activity record '{}' for surface '{}' is denied but carries no canonical denial code",
                    record.record_id, record.surface_token
                ),
            ));
        } else if !record.disposition_code_consistent() {
            defects.push(TransportAutomationDefect::new(
                AutomationNarrowReasonClass::DispositionDenialCodeMismatch,
                record.record_id.clone(),
                format!(
                    "activity record '{}' for surface '{}' has disposition '{}' but denial code '{}'",
                    record.record_id,
                    record.surface_token,
                    record.disposition_token,
                    record.denial_code_token
                ),
            ));
        }
    }

    defects
}

fn derive_rows(
    snapshot: &TransportAutomationSnapshot,
    page_defects: &[TransportAutomationDefect],
) -> Vec<TransportAutomationRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());

    let overall_narrow_reason = if has_withdrawal {
        page_defects
            .iter()
            .find(|d| d.narrow_reason.is_withdrawal_reason())
            .map(|d| d.narrow_reason)
            .unwrap_or(AutomationNarrowReasonClass::RawPrivateMaterialExposed)
    } else {
        AutomationNarrowReasonClass::NotNarrowed
    };

    snapshot
        .records
        .iter()
        .map(|record| {
            let row_narrow = find_row_narrow_reason(record, page_defects, overall_narrow_reason);
            let row_qual = qualification_for_reason(row_narrow);
            let summary = build_row_summary(&record.surface_token, &row_qual, row_narrow);
            TransportAutomationRow {
                record_kind: TRANSPORT_AUTOMATION_ROW_RECORD_KIND.to_owned(),
                schema_version: TRANSPORT_AUTOMATION_SCHEMA_VERSION,
                shared_contract_ref: TRANSPORT_AUTOMATION_SHARED_CONTRACT_REF.to_owned(),
                record_id: record.record_id.clone(),
                surface_token: record.surface_token.clone(),
                origin_scope_token: record.origin_scope_token.clone(),
                endpoint_class_token: record.endpoint_class_token.clone(),
                egress_class_token: record.egress_class_token.clone(),
                route_choice_token: record.route_choice_token.clone(),
                disposition_token: record.disposition_token.clone(),
                denial_code_token: record.denial_code_token.clone(),
                no_bypass: record.no_bypass,
                replay_idempotent_only: !record.disposition.is_deferred()
                    || record.action_is_idempotent,
                disposition_code_consistent: record.disposition_code_consistent(),
                raw_private_material_excluded: record.raw_private_material_excluded,
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn qualification_for_reason(reason: AutomationNarrowReasonClass) -> AutomationQualificationClass {
    if reason.is_withdrawal_reason() {
        AutomationQualificationClass::Withdrawn
    } else if reason.is_preview_reason() {
        AutomationQualificationClass::Preview
    } else if reason != AutomationNarrowReasonClass::NotNarrowed {
        AutomationQualificationClass::Beta
    } else {
        AutomationQualificationClass::Stable
    }
}

fn find_row_narrow_reason(
    record: &NetworkActivityRecord,
    page_defects: &[TransportAutomationDefect],
    overall_narrow_reason: AutomationNarrowReasonClass,
) -> AutomationNarrowReasonClass {
    // A withdrawal reason taints the whole packet; every row is withdrawn.
    if overall_narrow_reason.is_withdrawal_reason() {
        return overall_narrow_reason;
    }
    // Otherwise a record-specific defect governs the row.
    if let Some(defect) = page_defects.iter().find(|d| d.source == record.record_id) {
        return defect.narrow_reason;
    }
    AutomationNarrowReasonClass::NotNarrowed
}

fn build_row_summary(
    surface_token: &str,
    qual: &AutomationQualificationClass,
    narrow_reason: AutomationNarrowReasonClass,
) -> String {
    match qual {
        AutomationQualificationClass::Stable => format!(
            "Activity on surface '{}' qualifies stable: it resolved through the shared governance \
             layer, its disposition and canonical denial code agree, and it carries no raw \
             private material.",
            surface_token
        ),
        _ => format!(
            "Activity on surface '{}' narrowed to {} ({}): see defect list for details.",
            surface_token,
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable automation page consumed by the headless example,
/// the integration tests, and the fixture generator.
///
/// The seeded page produces zero defects: every required surface has at least
/// one activity record, all eight canonical denial codes appear across the
/// history, no raw private material is present, every record resolved through
/// the shared governance layer, every deferred action is idempotent-only, every
/// denied record carries a canonical denial code, and every record's
/// disposition and denial code agree.
pub fn seeded_transport_automation_page() -> TransportAutomationPage {
    TransportAutomationPage::new(
        "remote:networked_surface_transport_automation:default",
        "Networked-surface transport automation — stable packet",
        "2026-06-01T00:00:00Z",
        seeded_transport_automation_snapshot(),
    )
}

/// Build the seeded activity snapshot used by the seeded page.
///
/// Each required surface contributes one fully-typed, clean activity record, and
/// the records together surface all eight canonical denial codes plus one
/// allowed and one deferred action.
pub fn seeded_transport_automation_snapshot() -> TransportAutomationSnapshot {
    TransportAutomationSnapshot {
        records: vec![
            // AI gateway — allowed over a manual proxy; no denial.
            NetworkActivityRecord::new(
                "remote:transport_automation:activity:0001",
                1,
                "2026-06-01T00:00:01Z",
                SurfaceClass::AiGateway,
                OriginScopeClass::ThirdParty,
                EndpointClass::InferenceGateway,
                EgressClass::PublicInternet,
                RouteChoiceClass::ManualProxy,
                ActivityDispositionClass::Allowed,
                TransportDenialClass::None,
                true,
                "AI gateway request resolved through the policy-pinned proxy and was allowed; no \
                 denial applied.",
            ),
            // Docs / browser fetcher — denied: certificate authority untrusted.
            NetworkActivityRecord::new(
                "remote:transport_automation:activity:0002",
                2,
                "2026-06-01T00:00:02Z",
                SurfaceClass::DocsBrowserFetcher,
                OriginScopeClass::ThirdParty,
                EndpointClass::ContentOrigin,
                EgressClass::PublicInternet,
                RouteChoiceClass::Direct,
                ActivityDispositionClass::Denied,
                TransportDenialClass::CaUntrusted,
                true,
                "Docs fetch was denied because the content origin's certificate chain could not be \
                 trusted by the active trust store.",
            ),
            // Request / API client — denied: proxy authentication required.
            NetworkActivityRecord::new(
                "remote:transport_automation:activity:0003",
                3,
                "2026-06-01T00:00:03Z",
                SurfaceClass::RequestApiClient,
                OriginScopeClass::UserConfigured,
                EndpointClass::RestApi,
                EgressClass::PublicInternet,
                RouteChoiceClass::ManualProxy,
                ActivityDispositionClass::Denied,
                TransportDenialClass::ProxyAuthRequired,
                true,
                "Request/API client call was denied because the configured proxy demanded \
                 authentication that was not satisfied.",
            ),
            // Database / cloud connector — denied: proxy misconfigured.
            NetworkActivityRecord::new(
                "remote:transport_automation:activity:0004",
                4,
                "2026-06-01T00:00:04Z",
                SurfaceClass::DatabaseCloudConnector,
                OriginScopeClass::UserConfigured,
                EndpointClass::DataStore,
                EgressClass::ManagedEndpoint,
                RouteChoiceClass::SystemProxy,
                ActivityDispositionClass::Denied,
                TransportDenialClass::ProxyMisconfigured,
                true,
                "Database connector call was denied because the resolved system-proxy \
                 configuration was invalid.",
            ),
            // Registry read — denied: mirror unreachable.
            NetworkActivityRecord::new(
                "remote:transport_automation:activity:0005",
                5,
                "2026-06-01T00:00:05Z",
                SurfaceClass::RegistryRead,
                OriginScopeClass::FirstParty,
                EndpointClass::ArtifactRegistry,
                EgressClass::MirrorOnly,
                RouteChoiceClass::MirrorFirst,
                ActivityDispositionClass::Denied,
                TransportDenialClass::MirrorUnreachable,
                true,
                "Registry read was denied because the declared signed mirror could not be reached \
                 and a mirror-only profile forbids public fall-through.",
            ),
            // Companion handoff — deferred: offline mode, idempotent replay.
            NetworkActivityRecord::new(
                "remote:transport_automation:activity:0006",
                6,
                "2026-06-01T00:00:06Z",
                SurfaceClass::CompanionHandoff,
                OriginScopeClass::LoopbackLocal,
                EndpointClass::PeerDevice,
                EgressClass::LoopbackOnly,
                RouteChoiceClass::Offline,
                ActivityDispositionClass::Deferred,
                TransportDenialClass::OfflineMode,
                true,
                "Companion handoff was deferred for idempotent offline replay because the surface \
                 is offline and no in-policy route is available.",
            ),
            // Provider mutation — denied: egress blocked by policy.
            NetworkActivityRecord::new(
                "remote:transport_automation:activity:0007",
                7,
                "2026-06-01T00:00:07Z",
                SurfaceClass::ProviderMutation,
                OriginScopeClass::ThirdParty,
                EndpointClass::VcsHost,
                EgressClass::PublicInternet,
                RouteChoiceClass::Direct,
                ActivityDispositionClass::Denied,
                TransportDenialClass::EgressBlockedPolicy,
                true,
                "Provider mutation was denied because transport policy blocks egress for this \
                 action.",
            ),
            // Sync / offboarding — denied: SSH host key unknown.
            NetworkActivityRecord::new(
                "remote:transport_automation:activity:0008",
                8,
                "2026-06-01T00:00:08Z",
                SurfaceClass::SyncOffboarding,
                OriginScopeClass::ManagedTenant,
                EndpointClass::SyncService,
                EgressClass::ManagedEndpoint,
                RouteChoiceClass::Direct,
                ActivityDispositionClass::Denied,
                TransportDenialClass::SshHostKeyUnknown,
                true,
                "Sync/offboarding transfer was denied because the recorded SSH host key did not \
                 match the presented host proof.",
            ),
            // Remote preview route — denied: origin scope ambiguous.
            NetworkActivityRecord::new(
                "remote:transport_automation:activity:0009",
                9,
                "2026-06-01T00:00:09Z",
                SurfaceClass::RemotePreviewRoute,
                OriginScopeClass::ThirdParty,
                EndpointClass::PreviewOrigin,
                EgressClass::PublicInternet,
                RouteChoiceClass::Direct,
                ActivityDispositionClass::Denied,
                TransportDenialClass::OriginScopeAmbiguous,
                true,
                "Remote preview route was denied because the endpoint's origin ownership scope \
                 could not be resolved unambiguously.",
            ),
        ],
    }
}
