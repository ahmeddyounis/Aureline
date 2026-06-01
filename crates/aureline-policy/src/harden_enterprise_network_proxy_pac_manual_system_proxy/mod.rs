//! Hardened enterprise network, proxy, and trust bootstrap proof packet.
//!
//! This module produces a stable proof packet that demonstrates enterprise
//! network behavior follows a typed bootstrap-credential and proxy-resolution
//! contract. Every connection and bootstrap review surface can explain:
//!
//! 1. Why a given route used system proxy vs PAC vs manual vs policy — via a
//!    single inspectable precedence model with a ranked [`ProxyPrecedenceClass`]
//!    and a closed-vocabulary [`ProxySelectorReasonClass`].
//! 2. Which trust material or client certificate participated — via an explicit
//!    [`BootstrapCredentialKind`] list and typed TLS, CA, and client-cert state.
//! 3. What narrower local-only fallback exists when enterprise routing
//!    prerequisites are unavailable — every route row names a
//!    `local_only_fallback_route_token` and a `fallback_condition_label`.
//! 4. That no raw secret, private key, or raw credential reaches logs or
//!    default export paths — verified by a per-row and page-level guardrail.
//!
//! The six required proxy routes are: `system`, `pac`, `manual`,
//! `policy_pinned`, `mirror_only`, and `offline`. Each route carries explicit
//! host-key, TLS, and client-certificate state. The upstream
//! [`aureline_auth::network_trust`] beta page is embedded as evidence.
//!
//! Surfaces (admin/settings center, support export, shell network summary,
//! headless inspector) read [`seeded_harden_enterprise_network_proxy_page`]
//! rather than minting parallel proxy-routing checks.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/harden-enterprise-network-proxy-pac-manual-system-proxy.md`
//! - Artifact: `artifacts/enterprise/m4/harden-enterprise-network-proxy-pac-manual-system-proxy.md`
//! - Contract ref: [`HARDEN_ENTERPRISE_NETWORK_PROXY_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use aureline_auth::network_trust::{
    audit_network_trust_beta_rows, seeded_network_trust_beta_page, NetworkTrustBetaDefectKind,
    NetworkTrustBetaPage,
};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const HARDEN_ENTERPRISE_NETWORK_PROXY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const HARDEN_ENTERPRISE_NETWORK_PROXY_SHARED_CONTRACT_REF: &str =
    "policy:harden_enterprise_network_proxy:v1";

/// Record-kind tag for [`HardenEnterpriseNetworkProxyPage`] payloads.
pub const HARDEN_ENTERPRISE_NETWORK_PROXY_PAGE_RECORD_KIND: &str =
    "policy_harden_enterprise_network_proxy_page_record";

/// Record-kind tag for [`HardenEnterpriseNetworkProxyRow`] payloads.
pub const HARDEN_ENTERPRISE_NETWORK_PROXY_ROW_RECORD_KIND: &str =
    "policy_harden_enterprise_network_proxy_row_record";

/// Record-kind tag for [`HardenEnterpriseNetworkProxyDefect`] payloads.
pub const HARDEN_ENTERPRISE_NETWORK_PROXY_DEFECT_RECORD_KIND: &str =
    "policy_harden_enterprise_network_proxy_defect_record";

/// Record-kind tag for [`HardenEnterpriseNetworkProxySummary`] payloads.
pub const HARDEN_ENTERPRISE_NETWORK_PROXY_SUMMARY_RECORD_KIND: &str =
    "policy_harden_enterprise_network_proxy_summary_record";

/// Record-kind tag for [`HardenEnterpriseNetworkProxySupportExport`] payloads.
pub const HARDEN_ENTERPRISE_NETWORK_PROXY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_harden_enterprise_network_proxy_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const HARDEN_ENTERPRISE_NETWORK_PROXY_DOC_REF: &str =
    "docs/enterprise/m4/harden-enterprise-network-proxy-pac-manual-system-proxy.md";

/// Repo-relative path of the artifact summary for this lane.
pub const HARDEN_ENTERPRISE_NETWORK_PROXY_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/harden-enterprise-network-proxy-pac-manual-system-proxy.md";

/// Upstream network-trust beta contract ref.
pub const NETWORK_TRUST_BETA_CONTRACT_REF: &str = "network:network_trust_beta:v1";

// ---------------------------------------------------------------------------
// Proxy route vocabulary
// ---------------------------------------------------------------------------

/// Proxy resolution route class under which a row is inspected.
///
/// These six routes form the required coverage set for the hardened enterprise
/// proof packet. Each row explains which route was selected, why, and what
/// local-only fallback applies when enterprise prerequisites are unavailable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyRouteClass {
    /// Platform system proxy (OS-level proxy settings).
    System,
    /// Platform PAC auto-config script.
    Pac,
    /// Manually configured proxy (admin policy or user workspace setting).
    Manual,
    /// Policy-pinned proxy locked by a signed managed policy; cannot be
    /// overridden by user settings or process environment.
    PolicyPinned,
    /// Mirror-only route where all egress is limited to declared signed mirrors;
    /// public endpoints are not reachable.
    MirrorOnly,
    /// Offline route; no proxy is used and no external network calls are made.
    Offline,
}

impl ProxyRouteClass {
    /// All required proxy routes in canonical order.
    pub const ALL: [Self; 6] = [
        Self::System,
        Self::Pac,
        Self::Manual,
        Self::PolicyPinned,
        Self::MirrorOnly,
        Self::Offline,
    ];

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Pac => "pac",
            Self::Manual => "manual",
            Self::PolicyPinned => "policy_pinned",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
        }
    }

    /// True when this route requires an explicit local-only fallback declaration
    /// (enterprise-bearing routes where the fallback would otherwise be opaque).
    pub const fn requires_local_fallback_declaration(self) -> bool {
        matches!(
            self,
            Self::PolicyPinned | Self::MirrorOnly | Self::Offline
        )
    }

    /// True when this route may carry managed authority (signed policy origin).
    pub const fn may_carry_managed_authority(self) -> bool {
        matches!(self, Self::PolicyPinned | Self::MirrorOnly)
    }
}

// ---------------------------------------------------------------------------
// Proxy precedence vocabulary
// ---------------------------------------------------------------------------

/// Precedence tier for proxy selection.
///
/// When multiple proxy sources are available simultaneously, routes resolve
/// through this canonical precedence order (lower rank = higher priority).
/// The page carries one row per route with its rank, so a review surface can
/// always explain which route won and which alternatives were considered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyPrecedenceClass {
    /// Highest priority: policy-pinned proxy from a signed managed policy.
    PolicyPinned,
    /// Manual proxy entry (admin configured or user workspace setting).
    Manual,
    /// Process environment proxy (`HTTPS_PROXY`, `HTTP_PROXY`, `NO_PROXY`).
    ProcessEnvironment,
    /// Platform PAC auto-config script.
    Pac,
    /// Platform system proxy (OS-level settings).
    System,
    /// Direct no-proxy (offline or mirror-only — no egress beyond declared endpoints).
    DirectNoProxy,
}

impl ProxyPrecedenceClass {
    /// Canonical precedence rank (lower = higher priority).
    pub const fn rank(self) -> u8 {
        match self {
            Self::PolicyPinned => 1,
            Self::Manual => 2,
            Self::ProcessEnvironment => 3,
            Self::Pac => 4,
            Self::System => 5,
            Self::DirectNoProxy => 6,
        }
    }

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyPinned => "policy_pinned",
            Self::Manual => "manual",
            Self::ProcessEnvironment => "process_environment",
            Self::Pac => "pac",
            Self::System => "system",
            Self::DirectNoProxy => "direct_no_proxy",
        }
    }
}

// ---------------------------------------------------------------------------
// Proxy selector reason vocabulary
// ---------------------------------------------------------------------------

/// Closed-vocabulary reason why a proxy route was selected over alternatives.
///
/// Every row must name at least one selector reason so the review surface can
/// explain the selection decision without exposing raw policy text or raw
/// credential values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxySelectorReasonClass {
    /// Signed managed policy named this proxy; user settings cannot override.
    ManagedPolicyPinned,
    /// Admin-configured manual proxy entry is present and takes precedence over environment.
    AdminManualEntry,
    /// User workspace setting specifies a proxy for this connection type.
    UserWorkspaceSetting,
    /// Process environment variable (`HTTPS_PROXY`/`HTTP_PROXY`) is set.
    ProcessEnvironmentVariable,
    /// PAC script is available and evaluated; no higher-priority source present.
    PacScriptEvaluated,
    /// OS system proxy is active; no higher-priority source present.
    OsSystemProxyActive,
    /// Mirror-only profile restricts egress to declared signed mirrors; no general proxy.
    MirrorOnlyProfileActive,
    /// Offline or air-gapped profile; all network egress is suppressed.
    OfflineProfileActive,
}

impl ProxySelectorReasonClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedPolicyPinned => "managed_policy_pinned",
            Self::AdminManualEntry => "admin_manual_entry",
            Self::UserWorkspaceSetting => "user_workspace_setting",
            Self::ProcessEnvironmentVariable => "process_environment_variable",
            Self::PacScriptEvaluated => "pac_script_evaluated",
            Self::OsSystemProxyActive => "os_system_proxy_active",
            Self::MirrorOnlyProfileActive => "mirror_only_profile_active",
            Self::OfflineProfileActive => "offline_profile_active",
        }
    }
}

// ---------------------------------------------------------------------------
// Bootstrap credential vocabulary
// ---------------------------------------------------------------------------

/// Kind of bootstrap credential or trust material that participated in the
/// connection attempt.
///
/// Every row declares which credential kinds were involved so a review surface
/// can answer "which trust material participated?" without exposing raw key
/// material or raw certificate data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapCredentialKind {
    /// No credential required for this route (e.g., offline, direct no-proxy).
    NoneRequired,
    /// OS or platform CA trust store (system CAs only).
    TlsSystemCa,
    /// Custom or org-overlay CA bundle augmenting or replacing the OS trust store.
    CustomCa,
    /// Client certificate (mTLS) enrolled and presented.
    ClientCertificate,
    /// SSH host key pinned or matched against known-hosts.
    SshHostKey,
    /// Admin-signed proxy credential (opaque; never raw).
    AdminSignedProxyCredential,
}

impl BootstrapCredentialKind {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneRequired => "none_required",
            Self::TlsSystemCa => "tls_system_ca",
            Self::CustomCa => "custom_ca",
            Self::ClientCertificate => "client_certificate",
            Self::SshHostKey => "ssh_host_key",
            Self::AdminSignedProxyCredential => "admin_signed_proxy_credential",
        }
    }
}

// ---------------------------------------------------------------------------
// TLS and cert state
// ---------------------------------------------------------------------------

/// TLS verification posture for a proxy route connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TlsVerificationPostureClass {
    /// Full TLS verification against the effective trust store.
    FullVerification,
    /// Verification against a custom or org-overlay CA bundle.
    CustomCaVerification,
    /// TLS verification is not applicable (no TLS on this route, e.g., offline).
    NotApplicable,
}

impl TlsVerificationPostureClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullVerification => "full_verification",
            Self::CustomCaVerification => "custom_ca_verification",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Client certificate (mTLS) posture for a proxy route connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteClientCertPostureClass {
    /// No client certificate is required on this route.
    NoneRequired,
    /// A valid client certificate is enrolled and presented.
    PresentValid,
    /// A required client certificate is missing or expired; route is blocked.
    MissingOrExpired,
}

impl RouteClientCertPostureClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneRequired => "none_required",
            Self::PresentValid => "present_valid",
            Self::MissingOrExpired => "missing_or_expired",
        }
    }

    /// True when the posture blocks the route from completing.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::MissingOrExpired)
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Qualification tier for the hardened enterprise network proxy page.
///
/// The tier is derived, not asserted: it is set by the audit. A caller may
/// never assert `stable` without a clean audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenEnterpriseNetworkProxyQualificationClass {
    /// All required conditions hold and the upstream network-trust audit is clean.
    Stable,
    /// One or more non-critical conditions are unmet.
    Beta,
    /// A required proxy route has no row; coverage gap prevents any claim.
    Preview,
    /// A hard guardrail was triggered; the page is withdrawn immediately.
    Withdrawn,
}

impl HardenEnterpriseNetworkProxyQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Typed reason a packet or row was narrowed below
/// [`HardenEnterpriseNetworkProxyQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenEnterpriseNetworkProxyNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// The upstream network-trust beta page has defects.
    UpstreamNetworkTrustHasDefects,
    /// A required proxy route has no row; narrows to preview.
    MissingRouteCoverage,
    /// A row does not declare a selector reason for why this route was chosen.
    EmptySelectorReason,
    /// A route that requires a local fallback declaration is missing one.
    EmptyLocalFallback,
    /// Raw secret or private key material was exposed in a row or the upstream
    /// page; withdraws the packet immediately.
    RawSecretOrPrivateMaterialExposed,
    /// A row does not declare which bootstrap credential kinds participated.
    MissingBootstrapCredentialDeclaration,
    /// A row does not carry inspectable TLS posture.
    TlsStateNotInspectable,
    /// A mirror-only or offline row does not carry `local_core_continuity_explicit: true`.
    LocalCoreContinuityNotExplicit,
    /// A policy-pinned or managed row is missing a managed attribution ref.
    ManagedAttributionMissing,
}

impl HardenEnterpriseNetworkProxyNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::UpstreamNetworkTrustHasDefects => "upstream_network_trust_has_defects",
            Self::MissingRouteCoverage => "missing_route_coverage",
            Self::EmptySelectorReason => "empty_selector_reason",
            Self::EmptyLocalFallback => "empty_local_fallback",
            Self::RawSecretOrPrivateMaterialExposed => "raw_secret_or_private_material_exposed",
            Self::MissingBootstrapCredentialDeclaration => {
                "missing_bootstrap_credential_declaration"
            }
            Self::TlsStateNotInspectable => "tls_state_not_inspectable",
            Self::LocalCoreContinuityNotExplicit => "local_core_continuity_not_explicit",
            Self::ManagedAttributionMissing => "managed_attribution_missing",
        }
    }

    /// True when this reason triggers immediate withdrawal and cannot be overridden.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::RawSecretOrPrivateMaterialExposed)
    }
}

// ---------------------------------------------------------------------------
// Bootstrap credential declaration
// ---------------------------------------------------------------------------

/// Declaration of a bootstrap credential that participated in a proxy route
/// connection.
///
/// Every participating credential kind appears as one entry. Raw key or
/// certificate bytes are never included; only opaque refs and type tokens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BootstrapCredentialDeclaration {
    /// Credential kind.
    pub credential_kind: BootstrapCredentialKind,
    /// Stable token for [`Self::credential_kind`].
    pub credential_kind_token: String,
    /// Opaque ref to the credential or trust-material record (never raw bytes).
    pub credential_ref: String,
    /// Export-safe label describing the credential provenance.
    pub source_label: String,
    /// True when this credential is locked by a signed managed policy.
    pub policy_locked: bool,
}

// ---------------------------------------------------------------------------
// Proxy route row
// ---------------------------------------------------------------------------

/// Hardened proxy route row covering one proxy selection route.
///
/// Each row proves:
/// - why this route was selected (selector reason + precedence rank);
/// - which bootstrap credentials and trust material participated;
/// - what the TLS and client-cert state is;
/// - what local-only fallback applies when enterprise prerequisites are absent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenEnterpriseNetworkProxyRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Proxy route class.
    pub proxy_route: ProxyRouteClass,
    /// Stable token for [`Self::proxy_route`].
    pub proxy_route_token: String,
    /// Precedence tier for this route in the canonical selection model.
    pub precedence: ProxyPrecedenceClass,
    /// Stable token for [`Self::precedence`].
    pub precedence_token: String,
    /// Canonical precedence rank (lower = higher priority).
    pub precedence_rank: u8,
    /// Human-readable label summarising the precedence position.
    pub precedence_rank_label: String,
    /// Why this route was selected over lower-priority alternatives.
    pub selector_reason: ProxySelectorReasonClass,
    /// Stable token for [`Self::selector_reason`].
    pub selector_reason_token: String,
    /// Plain-language explanation of the route selection decision.
    pub selector_reason_label: String,
    /// Bootstrap credentials and trust material that participated.
    pub bootstrap_credentials: Vec<BootstrapCredentialDeclaration>,
    /// TLS verification posture for this route.
    pub tls_verification_posture: TlsVerificationPostureClass,
    /// Stable token for [`Self::tls_verification_posture`].
    pub tls_verification_posture_token: String,
    /// Client certificate posture for this route.
    pub client_cert_posture: RouteClientCertPostureClass,
    /// Stable token for [`Self::client_cert_posture`].
    pub client_cert_posture_token: String,
    /// SSH host key posture label (or empty when SSH is not applicable).
    pub ssh_host_key_posture_label: String,
    /// Token for the local-only fallback route when enterprise prerequisites
    /// are unavailable. Must be non-empty for routes that require explicit
    /// fallback declarations.
    pub local_only_fallback_route_token: String,
    /// Plain-language label describing the condition that triggers fallback.
    pub fallback_condition_label: String,
    /// True when local-core continuity is stated explicitly on this row.
    pub local_core_continuity_explicit: bool,
    /// True when raw secret or private-key material is excluded from the record.
    pub raw_secret_or_private_material_excluded: bool,
    /// Opaque ref to the managed attribution record (required when
    /// [`ProxyRouteClass::may_carry_managed_authority`] is true).
    pub managed_attribution_ref: String,
    /// Derived qualification tier for this row.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed`).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate summary for the hardened enterprise network proxy page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HardenEnterpriseNetworkProxySummary {
    /// Total row count.
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Proxy route tokens present on the page.
    pub routes_covered: Vec<String>,
    /// Selector reason tokens present across rows.
    pub selector_reasons_present: Vec<String>,
    /// Bootstrap credential kind tokens present across rows.
    pub bootstrap_credential_kinds_present: Vec<String>,
    /// Number of rows with `local_core_continuity_explicit: true`.
    pub local_core_continuity_explicit_row_count: usize,
    /// Number of rows with `raw_secret_or_private_material_excluded: true`.
    pub raw_secret_excluded_row_count: usize,
    /// Defect count from the upstream network-trust beta page.
    pub upstream_network_trust_defect_count: usize,
    /// Overall qualification token.
    pub overall_qualification_token: String,
}

impl HardenEnterpriseNetworkProxySummary {
    fn from_rows(
        rows: &[HardenEnterpriseNetworkProxyRow],
        network_trust_page: &NetworkTrustBetaPage,
    ) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        let mut routes: BTreeSet<String> = BTreeSet::new();
        let mut selector_reasons: BTreeSet<String> = BTreeSet::new();
        let mut cred_kinds: BTreeSet<String> = BTreeSet::new();
        let mut local_core_ok = 0usize;
        let mut raw_secret_ok = 0usize;

        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
            routes.insert(row.proxy_route_token.clone());
            selector_reasons.insert(row.selector_reason_token.clone());
            for cred in &row.bootstrap_credentials {
                cred_kinds.insert(cred.credential_kind_token.clone());
            }
            if row.local_core_continuity_explicit {
                local_core_ok += 1;
            }
            if row.raw_secret_or_private_material_excluded {
                raw_secret_ok += 1;
            }
        }

        let overall = if withdrawn > 0 {
            HardenEnterpriseNetworkProxyQualificationClass::Withdrawn
        } else if preview > 0 {
            HardenEnterpriseNetworkProxyQualificationClass::Preview
        } else if beta > 0 {
            HardenEnterpriseNetworkProxyQualificationClass::Beta
        } else {
            HardenEnterpriseNetworkProxyQualificationClass::Stable
        };

        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            routes_covered: routes.into_iter().collect(),
            selector_reasons_present: selector_reasons.into_iter().collect(),
            bootstrap_credential_kinds_present: cred_kinds.into_iter().collect(),
            local_core_continuity_explicit_row_count: local_core_ok,
            raw_secret_excluded_row_count: raw_secret_ok,
            upstream_network_trust_defect_count: network_trust_page.defects.len(),
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the hardened enterprise network proxy audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenEnterpriseNetworkProxyDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: HardenEnterpriseNetworkProxyNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (row id, route token, or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl HardenEnterpriseNetworkProxyDefect {
    fn new(
        narrow_reason: HardenEnterpriseNetworkProxyNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: HARDEN_ENTERPRISE_NETWORK_PROXY_DEFECT_RECORD_KIND.to_owned(),
            schema_version: HARDEN_ENTERPRISE_NETWORK_PROXY_SCHEMA_VERSION,
            shared_contract_ref: HARDEN_ENTERPRISE_NETWORK_PROXY_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:harden-enterprise-network-proxy:{}:{}",
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
// Main page
// ---------------------------------------------------------------------------

/// Stable proof packet for hardened enterprise network, proxy, and trust
/// bootstrap.
///
/// This is the single inspectable record that proves all required proxy routes
/// are covered with explicit precedence, trust material, and local-only
/// fallback declarations. Dashboards, docs, Help/About surfaces, and support
/// exports should ingest it rather than cloning status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenEnterpriseNetworkProxyPage {
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
    pub summary: HardenEnterpriseNetworkProxySummary,
    /// Per-route qualification rows (one per proxy route).
    pub rows: Vec<HardenEnterpriseNetworkProxyRow>,
    /// Typed validation defects.
    pub defects: Vec<HardenEnterpriseNetworkProxyDefect>,
    /// Upstream network-trust beta page embedded as evidence.
    pub network_trust_beta_page: NetworkTrustBetaPage,
}

impl HardenEnterpriseNetworkProxyPage {
    /// Build the page from a set of route rows and an embedded network-trust page.
    ///
    /// Defects are derived automatically from the audit.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<HardenEnterpriseNetworkProxyRow>,
        network_trust_beta_page: NetworkTrustBetaPage,
    ) -> Self {
        let defects = audit_harden_rows(&rows, &network_trust_beta_page);
        let qualified_rows = qualify_rows(rows, &defects);
        let summary =
            HardenEnterpriseNetworkProxySummary::from_rows(&qualified_rows, &network_trust_beta_page);
        Self {
            record_kind: HARDEN_ENTERPRISE_NETWORK_PROXY_PAGE_RECORD_KIND.to_owned(),
            schema_version: HARDEN_ENTERPRISE_NETWORK_PROXY_SCHEMA_VERSION,
            shared_contract_ref: HARDEN_ENTERPRISE_NETWORK_PROXY_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows: qualified_rows,
            defects,
            network_trust_beta_page,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == HardenEnterpriseNetworkProxyQualificationClass::Stable.as_str()
    }

    /// True when all six required proxy routes are covered.
    pub fn covers_all_required_routes(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .map(|r| r.proxy_route_token.as_str())
            .collect();
        ProxyRouteClass::ALL
            .iter()
            .all(|r| covered.contains(r.as_str()))
    }

    /// True when every row excludes raw secret or private-key material.
    pub fn all_rows_exclude_raw_secret_material(&self) -> bool {
        self.rows
            .iter()
            .all(|r| r.raw_secret_or_private_material_excluded)
    }

    /// True when all rows that require local fallback declarations carry one.
    pub fn all_required_fallbacks_declared(&self) -> bool {
        self.rows.iter().all(|r| {
            if r.proxy_route.requires_local_fallback_declaration() {
                !r.local_only_fallback_route_token.is_empty()
            } else {
                true
            }
        })
    }

    /// True when all rows carry explicit selector reasons.
    pub fn all_rows_have_selector_reasons(&self) -> bool {
        self.rows
            .iter()
            .all(|r| !r.selector_reason_token.is_empty())
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper for the hardened enterprise network proxy page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenEnterpriseNetworkProxySupportExport {
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
    /// The hardened network proxy page embedded as evidence.
    pub page: HardenEnterpriseNetworkProxyPage,
    /// Narrow-reason tokens present in the page's defect list.
    pub narrow_reasons_present: Vec<HardenEnterpriseNetworkProxyNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw secret or private-key material is excluded from the export.
    pub raw_secret_or_private_material_excluded: bool,
}

impl HardenEnterpriseNetworkProxySupportExport {
    /// Wrap a page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: HardenEnterpriseNetworkProxyPage,
    ) -> Self {
        let mut reasons: Vec<HardenEnterpriseNetworkProxyNarrowReasonClass> = Vec::new();
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
            record_kind: HARDEN_ENTERPRISE_NETWORK_PROXY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: HARDEN_ENTERPRISE_NETWORK_PROXY_SCHEMA_VERSION,
            shared_contract_ref: HARDEN_ENTERPRISE_NETWORK_PROXY_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_secret_or_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Public audit and validate functions
// ---------------------------------------------------------------------------

/// Re-run the harden-proxy audit over the rows and embedded network-trust page.
pub fn audit_harden_enterprise_network_proxy_page(
    page: &HardenEnterpriseNetworkProxyPage,
) -> Vec<HardenEnterpriseNetworkProxyDefect> {
    audit_harden_rows(&page.rows, &page.network_trust_beta_page)
}

/// Validate the page; returns `Ok` when the audit is clean.
pub fn validate_harden_enterprise_network_proxy_page(
    page: &HardenEnterpriseNetworkProxyPage,
) -> Result<(), Vec<HardenEnterpriseNetworkProxyDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

/// Build the seeded hardened enterprise network proxy page covering all six
/// required proxy routes with explicit precedence, trust material, TLS state,
/// and local-only fallback declarations.
pub fn seeded_harden_enterprise_network_proxy_page() -> HardenEnterpriseNetworkProxyPage {
    let network_trust_page = seeded_network_trust_beta_page();
    let rows = seeded_rows();
    HardenEnterpriseNetworkProxyPage::new(
        "policy:harden-enterprise-network-proxy:seeded:0001",
        "Hardened enterprise network, proxy, PAC/manual/system proxy, custom CA, client certificate, and SSH trust",
        "2026-06-01T00:00:00Z",
        rows,
        network_trust_page,
    )
}

// ---------------------------------------------------------------------------
// Internal audit helpers
// ---------------------------------------------------------------------------

fn audit_harden_rows(
    rows: &[HardenEnterpriseNetworkProxyRow],
    network_trust_page: &NetworkTrustBetaPage,
) -> Vec<HardenEnterpriseNetworkProxyDefect> {
    let mut defects: Vec<HardenEnterpriseNetworkProxyDefect> = Vec::new();

    // Hard guardrail: raw secret or private key material in upstream network-trust page.
    let upstream_defects =
        audit_network_trust_beta_rows(&network_trust_page.rows, &network_trust_page.support_rows);
    let has_raw_material = upstream_defects.iter().any(|d| {
        d.defect_kind == NetworkTrustBetaDefectKind::RawSecretOrPrivateMaterialExposed
    });
    if has_raw_material {
        defects.push(HardenEnterpriseNetworkProxyDefect::new(
            HardenEnterpriseNetworkProxyNarrowReasonClass::RawSecretOrPrivateMaterialExposed,
            "network_trust_beta_page",
            "upstream network-trust beta page has a raw_secret_or_private_material_exposed defect; packet is withdrawn",
        ));
        return defects;
    }

    // Non-critical: upstream network-trust page has other defects.
    if !network_trust_page.defects.is_empty() {
        defects.push(HardenEnterpriseNetworkProxyDefect::new(
            HardenEnterpriseNetworkProxyNarrowReasonClass::UpstreamNetworkTrustHasDefects,
            "network_trust_beta_page",
            "upstream network-trust beta page has defects; packet is narrowed to beta",
        ));
    }

    for row in rows {
        // Hard guardrail: raw secret material in any row.
        if !row.raw_secret_or_private_material_excluded {
            defects.push(HardenEnterpriseNetworkProxyDefect::new(
                HardenEnterpriseNetworkProxyNarrowReasonClass::RawSecretOrPrivateMaterialExposed,
                row.row_id.clone(),
                "row does not exclude raw secret or private-key material",
            ));
        }

        // Each row must name a selector reason.
        if row.selector_reason_token.is_empty() {
            defects.push(HardenEnterpriseNetworkProxyDefect::new(
                HardenEnterpriseNetworkProxyNarrowReasonClass::EmptySelectorReason,
                row.row_id.clone(),
                "row does not name a selector reason for why this proxy route was chosen",
            ));
        }

        // Routes requiring explicit fallback declarations must carry one.
        if row.proxy_route.requires_local_fallback_declaration()
            && row.local_only_fallback_route_token.is_empty()
        {
            defects.push(HardenEnterpriseNetworkProxyDefect::new(
                HardenEnterpriseNetworkProxyNarrowReasonClass::EmptyLocalFallback,
                row.row_id.clone(),
                "enterprise-bearing route requires an explicit local-only fallback route token",
            ));
        }

        // Bootstrap credentials must be declared (at minimum `none_required`).
        if row.bootstrap_credentials.is_empty() {
            defects.push(HardenEnterpriseNetworkProxyDefect::new(
                HardenEnterpriseNetworkProxyNarrowReasonClass::MissingBootstrapCredentialDeclaration,
                row.row_id.clone(),
                "row declares no bootstrap credential kinds; must include at least none_required",
            ));
        }

        // TLS posture token must be non-empty.
        if row.tls_verification_posture_token.is_empty() {
            defects.push(HardenEnterpriseNetworkProxyDefect::new(
                HardenEnterpriseNetworkProxyNarrowReasonClass::TlsStateNotInspectable,
                row.row_id.clone(),
                "row has an empty tls_verification_posture_token",
            ));
        }

        // Mirror-only and offline routes must carry local_core_continuity_explicit.
        if !row.local_core_continuity_explicit
            && matches!(
                row.proxy_route,
                ProxyRouteClass::MirrorOnly | ProxyRouteClass::Offline
            )
        {
            defects.push(HardenEnterpriseNetworkProxyDefect::new(
                HardenEnterpriseNetworkProxyNarrowReasonClass::LocalCoreContinuityNotExplicit,
                row.row_id.clone(),
                "mirror-only and offline rows must carry local_core_continuity_explicit: true",
            ));
        }

        // Policy-pinned and managed-authority routes require a managed attribution ref.
        if row.proxy_route.may_carry_managed_authority()
            && row.managed_attribution_ref.is_empty()
        {
            defects.push(HardenEnterpriseNetworkProxyDefect::new(
                HardenEnterpriseNetworkProxyNarrowReasonClass::ManagedAttributionMissing,
                row.row_id.clone(),
                "policy-pinned and mirror-only routes must carry a managed_attribution_ref",
            ));
        }
    }

    // Coverage check: all six required routes must appear at least once.
    let required_routes: BTreeSet<&str> =
        ProxyRouteClass::ALL.iter().map(|r| r.as_str()).collect();
    let observed_routes: BTreeSet<&str> =
        rows.iter().map(|r| r.proxy_route_token.as_str()).collect();
    for missing in required_routes.difference(&observed_routes) {
        defects.push(HardenEnterpriseNetworkProxyDefect::new(
            HardenEnterpriseNetworkProxyNarrowReasonClass::MissingRouteCoverage,
            "page",
            format!("missing row for required proxy route '{missing}'; packet narrowed to preview"),
        ));
    }

    defects
}

fn qualify_rows(
    mut rows: Vec<HardenEnterpriseNetworkProxyRow>,
    page_defects: &[HardenEnterpriseNetworkProxyDefect],
) -> Vec<HardenEnterpriseNetworkProxyRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects.iter().any(|d| {
        d.narrow_reason == HardenEnterpriseNetworkProxyNarrowReasonClass::MissingRouteCoverage
    });

    let (overall_qual, overall_reason) = if has_withdrawal {
        let r = page_defects
            .iter()
            .find(|d| d.narrow_reason.is_withdrawal_reason())
            .map(|d| d.narrow_reason)
            .unwrap_or(
                HardenEnterpriseNetworkProxyNarrowReasonClass::RawSecretOrPrivateMaterialExposed,
            );
        (HardenEnterpriseNetworkProxyQualificationClass::Withdrawn, r)
    } else if has_preview {
        (
            HardenEnterpriseNetworkProxyQualificationClass::Preview,
            HardenEnterpriseNetworkProxyNarrowReasonClass::MissingRouteCoverage,
        )
    } else if !page_defects.is_empty() {
        let r = page_defects[0].narrow_reason;
        (HardenEnterpriseNetworkProxyQualificationClass::Beta, r)
    } else {
        (
            HardenEnterpriseNetworkProxyQualificationClass::Stable,
            HardenEnterpriseNetworkProxyNarrowReasonClass::NotNarrowed,
        )
    };

    for row in &mut rows {
        let row_qual = if has_withdrawal {
            HardenEnterpriseNetworkProxyQualificationClass::Withdrawn
        } else if has_preview {
            HardenEnterpriseNetworkProxyQualificationClass::Preview
        } else {
            let row_has_defect = page_defects.iter().any(|d| {
                d.source == row.row_id
                    && d.narrow_reason
                        != HardenEnterpriseNetworkProxyNarrowReasonClass::UpstreamNetworkTrustHasDefects
            });
            if row_has_defect || !page_defects.is_empty() {
                HardenEnterpriseNetworkProxyQualificationClass::Beta
            } else {
                HardenEnterpriseNetworkProxyQualificationClass::Stable
            }
        };

        let row_reason = if row_qual == overall_qual {
            overall_reason
        } else {
            page_defects
                .iter()
                .find(|d| d.source == row.row_id)
                .map(|d| d.narrow_reason)
                .unwrap_or(HardenEnterpriseNetworkProxyNarrowReasonClass::NotNarrowed)
        };

        row.qualification_token = row_qual.as_str().to_owned();
        row.narrow_reason_token = row_reason.as_str().to_owned();
        row.plain_language_summary = build_row_summary(
            &row.row_id,
            &row.proxy_route_token,
            row_qual,
            row_reason,
        );
    }

    rows
}

fn build_row_summary(
    row_id: &str,
    route_token: &str,
    qual: HardenEnterpriseNetworkProxyQualificationClass,
    narrow_reason: HardenEnterpriseNetworkProxyNarrowReasonClass,
) -> String {
    match qual {
        HardenEnterpriseNetworkProxyQualificationClass::Stable => format!(
            "Row '{row_id}' (route: {route_token}) qualifies stable: \
             precedence declared, selector reason explicit, bootstrap credentials listed, \
             TLS state inspectable, local-core continuity explicit, \
             upstream network-trust clean."
        ),
        HardenEnterpriseNetworkProxyQualificationClass::Beta => format!(
            "Row '{row_id}' (route: {route_token}) narrowed to beta \
             (reason: {}): one or more required conditions are unmet.",
            narrow_reason.as_str()
        ),
        HardenEnterpriseNetworkProxyQualificationClass::Preview => format!(
            "Row '{row_id}' (route: {route_token}) narrowed to preview: \
             a required proxy route is missing from the page."
        ),
        HardenEnterpriseNetworkProxyQualificationClass::Withdrawn => format!(
            "Row '{row_id}' (route: {route_token}) is withdrawn \
             (reason: {}): hard guardrail triggered.",
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded rows
// ---------------------------------------------------------------------------

fn seeded_rows() -> Vec<HardenEnterpriseNetworkProxyRow> {
    vec![
        row_system(),
        row_pac(),
        row_manual(),
        row_policy_pinned(),
        row_mirror_only(),
        row_offline(),
    ]
}

fn make_row(
    row_id: &str,
    proxy_route: ProxyRouteClass,
    precedence: ProxyPrecedenceClass,
    precedence_rank_label: &str,
    selector_reason: ProxySelectorReasonClass,
    selector_reason_label: &str,
    bootstrap_credentials: Vec<BootstrapCredentialDeclaration>,
    tls_posture: TlsVerificationPostureClass,
    client_cert_posture: RouteClientCertPostureClass,
    ssh_host_key_posture_label: &str,
    local_only_fallback_route_token: &str,
    fallback_condition_label: &str,
    local_core_continuity_explicit: bool,
    managed_attribution_ref: &str,
) -> HardenEnterpriseNetworkProxyRow {
    HardenEnterpriseNetworkProxyRow {
        record_kind: HARDEN_ENTERPRISE_NETWORK_PROXY_ROW_RECORD_KIND.to_owned(),
        schema_version: HARDEN_ENTERPRISE_NETWORK_PROXY_SCHEMA_VERSION,
        shared_contract_ref: HARDEN_ENTERPRISE_NETWORK_PROXY_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        proxy_route,
        proxy_route_token: proxy_route.as_str().to_owned(),
        precedence,
        precedence_token: precedence.as_str().to_owned(),
        precedence_rank: precedence.rank(),
        precedence_rank_label: precedence_rank_label.to_owned(),
        selector_reason,
        selector_reason_token: selector_reason.as_str().to_owned(),
        selector_reason_label: selector_reason_label.to_owned(),
        bootstrap_credentials,
        tls_verification_posture: tls_posture,
        tls_verification_posture_token: tls_posture.as_str().to_owned(),
        client_cert_posture,
        client_cert_posture_token: client_cert_posture.as_str().to_owned(),
        ssh_host_key_posture_label: ssh_host_key_posture_label.to_owned(),
        local_only_fallback_route_token: local_only_fallback_route_token.to_owned(),
        fallback_condition_label: fallback_condition_label.to_owned(),
        local_core_continuity_explicit,
        raw_secret_or_private_material_excluded: true,
        managed_attribution_ref: managed_attribution_ref.to_owned(),
        // Filled in by qualify_rows.
        qualification_token: HardenEnterpriseNetworkProxyQualificationClass::Stable
            .as_str()
            .to_owned(),
        narrow_reason_token: HardenEnterpriseNetworkProxyNarrowReasonClass::NotNarrowed
            .as_str()
            .to_owned(),
        plain_language_summary: String::new(),
    }
}

fn cred(kind: BootstrapCredentialKind, cred_ref: &str, source_label: &str, policy_locked: bool) -> BootstrapCredentialDeclaration {
    BootstrapCredentialDeclaration {
        credential_kind: kind,
        credential_kind_token: kind.as_str().to_owned(),
        credential_ref: cred_ref.to_owned(),
        source_label: source_label.to_owned(),
        policy_locked,
    }
}

fn row_system() -> HardenEnterpriseNetworkProxyRow {
    make_row(
        "harden-enterprise-network-proxy:system",
        ProxyRouteClass::System,
        ProxyPrecedenceClass::System,
        "Rank 5 of 6 — OS system proxy; active when no higher-priority proxy source is present",
        ProxySelectorReasonClass::OsSystemProxyActive,
        "No policy-pinned, manual, process-environment, or PAC source is present; \
         the OS system proxy setting is used.",
        vec![
            cred(
                BootstrapCredentialKind::TlsSystemCa,
                "network:trust_store:system_ca_bundle:ref",
                "OS platform CA trust store",
                false,
            ),
        ],
        TlsVerificationPostureClass::FullVerification,
        RouteClientCertPostureClass::NoneRequired,
        "not_applicable",
        "",
        "",
        true,
        "",
    )
}

fn row_pac() -> HardenEnterpriseNetworkProxyRow {
    make_row(
        "harden-enterprise-network-proxy:pac",
        ProxyRouteClass::Pac,
        ProxyPrecedenceClass::Pac,
        "Rank 4 of 6 — PAC auto-config; evaluated when no policy-pinned, manual, or environment proxy is present",
        ProxySelectorReasonClass::PacScriptEvaluated,
        "No policy-pinned, manual, or process-environment proxy is set; \
         the platform PAC script is evaluated to determine the proxy.",
        vec![
            cred(
                BootstrapCredentialKind::TlsSystemCa,
                "network:trust_store:system_ca_bundle:ref",
                "OS platform CA trust store used for PAC retrieval and proxied connections",
                false,
            ),
        ],
        TlsVerificationPostureClass::FullVerification,
        RouteClientCertPostureClass::NoneRequired,
        "not_applicable",
        "",
        "",
        true,
        "",
    )
}

fn row_manual() -> HardenEnterpriseNetworkProxyRow {
    make_row(
        "harden-enterprise-network-proxy:manual",
        ProxyRouteClass::Manual,
        ProxyPrecedenceClass::Manual,
        "Rank 2 of 6 — manual proxy entry; takes precedence over process environment, PAC, and system proxy",
        ProxySelectorReasonClass::AdminManualEntry,
        "An admin-configured or user workspace manual proxy entry is present; \
         it takes precedence over process environment, PAC, and system proxy sources.",
        vec![
            cred(
                BootstrapCredentialKind::TlsSystemCa,
                "network:trust_store:system_ca_bundle:ref",
                "OS platform CA trust store for TLS to the proxy endpoint",
                false,
            ),
            cred(
                BootstrapCredentialKind::AdminSignedProxyCredential,
                "network:proxy:manual:credential_ref:opaque",
                "Admin-configured proxy credential (opaque; not a raw secret)",
                false,
            ),
        ],
        TlsVerificationPostureClass::FullVerification,
        RouteClientCertPostureClass::NoneRequired,
        "not_applicable",
        "",
        "",
        true,
        "",
    )
}

fn row_policy_pinned() -> HardenEnterpriseNetworkProxyRow {
    make_row(
        "harden-enterprise-network-proxy:policy_pinned",
        ProxyRouteClass::PolicyPinned,
        ProxyPrecedenceClass::PolicyPinned,
        "Rank 1 of 6 — policy-pinned proxy; highest precedence, cannot be overridden by user settings",
        ProxySelectorReasonClass::ManagedPolicyPinned,
        "A signed managed policy pins this proxy route; user workspace settings and \
         process environment variables cannot override it.",
        vec![
            cred(
                BootstrapCredentialKind::CustomCa,
                "network:trust_store:org_ca_bundle:ref",
                "Org-overlay CA bundle from signed managed policy",
                true,
            ),
            cred(
                BootstrapCredentialKind::ClientCertificate,
                "network:client_cert:policy_enrolled:ref",
                "Client certificate enrolled via managed policy (mTLS)",
                true,
            ),
        ],
        TlsVerificationPostureClass::CustomCaVerification,
        RouteClientCertPostureClass::PresentValid,
        "policy-pinned SSH host key verified against managed known-hosts",
        "offline",
        "When the policy-pinned proxy is unreachable, the connection falls back to \
         the offline route with no external egress; managed capabilities are narrowed.",
        true,
        "policy:signed_policy_bundle:proxy_pin:attribution_ref:opaque",
    )
}

fn row_mirror_only() -> HardenEnterpriseNetworkProxyRow {
    make_row(
        "harden-enterprise-network-proxy:mirror_only",
        ProxyRouteClass::MirrorOnly,
        ProxyPrecedenceClass::DirectNoProxy,
        "Rank 6 of 6 — mirror-only direct route; no general proxy, egress limited to declared signed mirrors",
        ProxySelectorReasonClass::MirrorOnlyProfileActive,
        "The mirror-only profile is active; all egress is limited to declared signed mirror \
         endpoints. Public endpoints are not reachable and no general proxy is used.",
        vec![
            cred(
                BootstrapCredentialKind::CustomCa,
                "network:trust_store:mirror_pinned_bundle:ref",
                "Pinned CA bundle delivered through the signed mirror for mirror-endpoint TLS",
                true,
            ),
        ],
        TlsVerificationPostureClass::CustomCaVerification,
        RouteClientCertPostureClass::NoneRequired,
        "not_applicable",
        "offline",
        "When the declared mirror endpoints are unreachable, the connection falls back to \
         the offline route; local-core editing continues without external access.",
        true,
        "policy:signed_policy_bundle:mirror_pin:attribution_ref:opaque",
    )
}

fn row_offline() -> HardenEnterpriseNetworkProxyRow {
    make_row(
        "harden-enterprise-network-proxy:offline",
        ProxyRouteClass::Offline,
        ProxyPrecedenceClass::DirectNoProxy,
        "Rank 6 of 6 — offline/air-gapped route; all external network egress is suppressed",
        ProxySelectorReasonClass::OfflineProfileActive,
        "The offline or air-gapped profile is active; all external network calls \
         are suppressed. Local-core editing and local AI capabilities remain available.",
        vec![
            cred(
                BootstrapCredentialKind::NoneRequired,
                "",
                "No network credential required; offline route uses no external endpoints",
                false,
            ),
        ],
        TlsVerificationPostureClass::NotApplicable,
        RouteClientCertPostureClass::NoneRequired,
        "not_applicable",
        "offline",
        "This route is itself the local-only fallback; no further fallback is required.",
        true,
        "",
    )
}
