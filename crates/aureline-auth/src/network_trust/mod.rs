//! Beta enterprise network-trust inspection projection.
//!
//! This module promotes the proxy / trust-store / SSH host-proof /
//! client-certificate lab matrix into a single inspectable beta projection.
//! It gives one record for each network facet — proxy resolution, CA / trust
//! store, SSH host-proof, and client certificate — with per-profile effective
//! value, source, and lock reason, plus the consumer-lane fan-out that proves
//! enterprise network settings are reused consistently across runtime,
//! extension, AI, provider, and update lanes.
//!
//! Surfaces (admin/settings center, support export wrapper, shell network
//! summary, headless inspector, docs fixtures) consume
//! [`seeded_network_trust_beta_page`] rather than re-deriving local
//! `is_proxy_set` or `has_org_ca` checks. The seed covers the connected,
//! mirror-only, offline, and enterprise-managed beta profiles and refuses to
//! fall back to undeclared public endpoints, plaintext secrets, or private
//! keys.
//!
//! The reviewer-facing landing page is
//! [`/docs/network/m3/proxy_ca_ssh_beta.md`](../../../../docs/network/m3/proxy_ca_ssh_beta.md).

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Beta schema version exported with every network-trust record.
pub const NETWORK_TRUST_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every beta network-trust record.
pub const NETWORK_TRUST_BETA_SHARED_CONTRACT_REF: &str = "network:network_trust_beta:v1";

/// Stable record kind for [`NetworkTrustBetaPage`] payloads.
pub const NETWORK_TRUST_BETA_PAGE_RECORD_KIND: &str = "network_network_trust_beta_page_record";

/// Stable record kind for [`NetworkTrustBetaRow`] payloads.
pub const NETWORK_TRUST_BETA_ROW_RECORD_KIND: &str = "network_network_trust_beta_row_record";

/// Stable record kind for [`NetworkTrustBetaProfileBinding`] payloads.
pub const NETWORK_TRUST_BETA_PROFILE_BINDING_RECORD_KIND: &str =
    "network_network_trust_beta_profile_binding_record";

/// Stable record kind for [`NetworkTrustBetaSupportRow`] payloads.
pub const NETWORK_TRUST_BETA_SUPPORT_ROW_RECORD_KIND: &str =
    "network_network_trust_beta_support_row_record";

/// Stable record kind for [`NetworkTrustBetaDefect`] payloads.
pub const NETWORK_TRUST_BETA_DEFECT_RECORD_KIND: &str = "network_network_trust_beta_defect_record";

/// Stable record kind for [`NetworkTrustBetaSupportExport`] payloads.
pub const NETWORK_TRUST_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "network_network_trust_beta_support_export_record";

/// Stable record kind for [`NetworkTrustBetaSummary`] payloads.
pub const NETWORK_TRUST_BETA_SUMMARY_RECORD_KIND: &str =
    "network_network_trust_beta_summary_record";

/// Source matrix this beta projection consumes.
pub const NETWORK_TRUST_BETA_SOURCE_MATRIX_REF: &str = "artifacts/network/proxy_lab_matrix.yaml";

/// Network facet covered by a beta row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkTrustBetaFacetClass {
    /// Effective proxy resolution (manual, environment, PAC, system, direct).
    Proxy,
    /// CA / trust-store posture (OS bundle, org overlay, pinned, mirror, air-gapped).
    TrustStore,
    /// SSH host-proof posture (strict pin, known-hosts match, mismatched).
    SshHostProof,
    /// Client-certificate (mTLS) posture.
    ClientCertificate,
}

impl NetworkTrustBetaFacetClass {
    /// All required beta facets in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Proxy,
        Self::TrustStore,
        Self::SshHostProof,
        Self::ClientCertificate,
    ];

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proxy => "proxy",
            Self::TrustStore => "trust_store",
            Self::SshHostProof => "ssh_host_proof",
            Self::ClientCertificate => "client_certificate",
        }
    }
}

/// Connectedness or enterprise profile under which a row is inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkTrustBetaProfileClass {
    /// Normal connected beta profile.
    Connected,
    /// Mirror-only profile where public endpoints are not fallback targets.
    MirrorOnly,
    /// Offline or air-gapped profile.
    Offline,
    /// Enterprise-managed profile with signed managed policy narrowing.
    EnterpriseManaged,
}

impl NetworkTrustBetaProfileClass {
    /// All required beta profiles in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::MirrorOnly,
        Self::Offline,
        Self::EnterpriseManaged,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
            Self::EnterpriseManaged => "enterprise_managed",
        }
    }
}

/// Source of the effective network setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkSettingSourceClass {
    /// Live admin-managed policy from a vendor-managed or self-hosted origin.
    AdminManagedPolicy,
    /// Signed mirror copy of an admin-managed policy.
    SignedMirrorPolicy,
    /// User-editable workspace setting.
    UserWorkspaceSetting,
    /// HTTPS_PROXY / NO_PROXY / SSL_CERT_FILE process environment variables.
    ProcessEnvironment,
    /// Platform PAC auto-config script.
    PlatformPacAutoConfig,
    /// Platform system-proxy / system trust-store / system known-hosts.
    PlatformSystemSetting,
    /// Manual signed-file import (admin-side air-gapped delivery).
    ManualSignedFileImport,
    /// Air-gapped signed transfer bundle.
    AirGappedSignedTransfer,
    /// Built-in baseline default (only valid for empty-but-known facets).
    BaselineDefault,
}

impl NetworkSettingSourceClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdminManagedPolicy => "admin_managed_policy",
            Self::SignedMirrorPolicy => "signed_mirror_policy",
            Self::UserWorkspaceSetting => "user_workspace_setting",
            Self::ProcessEnvironment => "process_environment",
            Self::PlatformPacAutoConfig => "platform_pac_auto_config",
            Self::PlatformSystemSetting => "platform_system_setting",
            Self::ManualSignedFileImport => "manual_signed_file_import",
            Self::AirGappedSignedTransfer => "air_gapped_signed_transfer",
            Self::BaselineDefault => "baseline_default",
        }
    }

    /// True when this source carries managed authority and must present a
    /// verified signature or admin attribution before widening the row.
    pub const fn is_managed(self) -> bool {
        matches!(
            self,
            Self::AdminManagedPolicy
                | Self::SignedMirrorPolicy
                | Self::ManualSignedFileImport
                | Self::AirGappedSignedTransfer
        )
    }
}

/// Lock reason exposed alongside an effective setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkSettingLockClass {
    /// Locked by signed managed policy; user edits would be ignored.
    SignedManagedPolicyLocked,
    /// Locked by a verified admin-managed policy.
    AdminPolicyLocked,
    /// Locked by platform-level configuration (system trust store, system proxy).
    PlatformLocked,
    /// User-editable in a workspace setting.
    UserEditable,
    /// Not applicable for this facet on this profile.
    NotApplicable,
}

impl NetworkSettingLockClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedManagedPolicyLocked => "signed_managed_policy_locked",
            Self::AdminPolicyLocked => "admin_policy_locked",
            Self::PlatformLocked => "platform_locked",
            Self::UserEditable => "user_editable",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this lock requires a verified managed source.
    pub const fn requires_managed_source(self) -> bool {
        matches!(
            self,
            Self::SignedManagedPolicyLocked | Self::AdminPolicyLocked
        )
    }
}

/// Consumer lane that reads the effective network setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkConsumerLaneClass {
    /// Core runtime, build, and language-service lanes.
    Runtime,
    /// Extension host lanes.
    Extension,
    /// AI broker, prompt, and tool-call lanes.
    Ai,
    /// Connected-provider and remote-attach lanes.
    Provider,
    /// Self-update and signed mirror refresh lanes.
    Update,
}

impl NetworkConsumerLaneClass {
    /// All required lanes in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Runtime,
        Self::Extension,
        Self::Ai,
        Self::Provider,
        Self::Update,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::Extension => "extension",
            Self::Ai => "ai",
            Self::Provider => "provider",
            Self::Update => "update",
        }
    }
}

/// Proxy resolution mode reported by the proxy facet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyResolutionModeClass {
    /// Manual proxy entry (admin policy or user workspace setting).
    ManualProxy,
    /// Process environment proxy (HTTPS_PROXY / HTTP_PROXY / NO_PROXY).
    EnvironmentProxy,
    /// Platform PAC auto-config script.
    PacProxy,
    /// Platform system proxy.
    SystemProxy,
    /// Policy-permitted direct egress.
    DirectNoProxy,
}

impl ProxyResolutionModeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManualProxy => "manual_proxy",
            Self::EnvironmentProxy => "environment_proxy",
            Self::PacProxy => "pac_proxy",
            Self::SystemProxy => "system_proxy",
            Self::DirectNoProxy => "direct_no_proxy",
        }
    }
}

/// Trust-store provenance reported by the CA / trust-store facet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustStoreSourceClass {
    /// OS trust store only.
    OsTrustStore,
    /// OS trust store augmented with a locally-verified org CA bundle.
    OsTrustStorePlusOrgCaBundle,
    /// Pinned org bundle that replaces the OS trust store for this profile.
    PinnedOrgBundle,
    /// Pinned bundle delivered through a signed mirror.
    MirrorPinnedBundle,
    /// Pinned bundle delivered through an air-gapped signed transfer.
    AirGappedPinnedBundle,
}

impl TrustStoreSourceClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsTrustStore => "os_trust_store",
            Self::OsTrustStorePlusOrgCaBundle => "os_trust_store_plus_org_ca_bundle",
            Self::PinnedOrgBundle => "pinned_org_bundle",
            Self::MirrorPinnedBundle => "mirror_pinned_bundle",
            Self::AirGappedPinnedBundle => "air_gapped_pinned_bundle",
        }
    }
}

/// SSH host-proof posture reported by the SSH facet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SshHostProofClass {
    /// Strict-pinned policy host-key match.
    StrictPinned,
    /// User known-hosts match (consumer profile only).
    KnownHostsMatch,
    /// Mismatched host key; connection rejected.
    MismatchedRejected,
    /// Not applicable on this profile.
    NotApplicable,
}

impl SshHostProofClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StrictPinned => "strict_pinned",
            Self::KnownHostsMatch => "known_hosts_match",
            Self::MismatchedRejected => "mismatched_rejected",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Client-certificate (mTLS) posture reported by the client-certificate facet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientCertificateStateClass {
    /// No client certificate is required on this profile.
    NoneRequired,
    /// A valid client certificate is enrolled.
    PresentValid,
    /// A client certificate is enrolled but expired.
    Expired,
    /// A required client certificate is missing.
    Missing,
    /// A client certificate was revoked.
    Revoked,
}

impl ClientCertificateStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneRequired => "none_required",
            Self::PresentValid => "present_valid",
            Self::Expired => "expired",
            Self::Missing => "missing",
            Self::Revoked => "revoked",
        }
    }

    /// True when the posture would prevent a row from succeeding.
    pub const fn is_failing(self) -> bool {
        matches!(self, Self::Expired | Self::Missing | Self::Revoked)
    }
}

/// Effective-value posture authority on a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkAuthorityClass {
    /// Profile reports an inspectable effective value backed by the source.
    EffectiveValuePublished,
    /// Profile narrows the value to a degraded read-only / preview posture.
    DegradedPreviewOnly,
    /// Profile fails closed because the facet's required input is missing.
    BlockedMissingInput,
    /// Profile fails closed because a managed policy rejects the value.
    BlockedManagedPolicy,
    /// Facet is not applicable on this profile.
    NotApplicable,
}

impl NetworkAuthorityClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EffectiveValuePublished => "effective_value_published",
            Self::DegradedPreviewOnly => "degraded_preview_only",
            Self::BlockedMissingInput => "blocked_missing_input",
            Self::BlockedManagedPolicy => "blocked_managed_policy",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Typed defect kind for the network-trust beta validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkTrustBetaDefectKind {
    /// A required facet is absent from the page.
    MissingFacetCoverage,
    /// A row is missing a required profile binding.
    MissingProfileCoverage,
    /// A row's facet token does not match its facet class.
    FacetTokenDrift,
    /// A binding's source token does not match its source class.
    SourceTokenDrift,
    /// A binding's lock token does not match its lock class.
    LockTokenDrift,
    /// A profile binding declares a managed lock without a managed source.
    LockInconsistentWithSource,
    /// A managed-source binding presents no verified signature pointer.
    UnsignedManagedAuthority,
    /// A binding permits an undeclared public endpoint fallback.
    HiddenPublicEndpointFallback,
    /// A row leaks raw secret or private-key material.
    RawSecretOrPrivateMaterialExposed,
    /// A row does not name the runtime/extension/ai/provider/update consumers.
    MissingConsumerLaneCoverage,
    /// The support row drifted from the live row vocabulary.
    SupportRowVocabularyDrift,
    /// A binding's effective_value_label is empty for a published authority.
    EmptyEffectiveValueLabel,
}

impl NetworkTrustBetaDefectKind {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingFacetCoverage => "missing_facet_coverage",
            Self::MissingProfileCoverage => "missing_profile_coverage",
            Self::FacetTokenDrift => "facet_token_drift",
            Self::SourceTokenDrift => "source_token_drift",
            Self::LockTokenDrift => "lock_token_drift",
            Self::LockInconsistentWithSource => "lock_inconsistent_with_source",
            Self::UnsignedManagedAuthority => "unsigned_managed_authority",
            Self::HiddenPublicEndpointFallback => "hidden_public_endpoint_fallback",
            Self::RawSecretOrPrivateMaterialExposed => "raw_secret_or_private_material_exposed",
            Self::MissingConsumerLaneCoverage => "missing_consumer_lane_coverage",
            Self::SupportRowVocabularyDrift => "support_row_vocabulary_drift",
            Self::EmptyEffectiveValueLabel => "empty_effective_value_label",
        }
    }
}

/// One profile binding for a beta row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTrustBetaProfileBinding {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Profile class.
    pub profile_class: NetworkTrustBetaProfileClass,
    /// Stable token for [`Self::profile_class`].
    pub profile_token: String,
    /// Source class.
    pub source_class: NetworkSettingSourceClass,
    /// Stable token for [`Self::source_class`].
    pub source_token: String,
    /// Lock class.
    pub lock_class: NetworkSettingLockClass,
    /// Stable token for [`Self::lock_class`].
    pub lock_token: String,
    /// Effective-value authority.
    pub authority: NetworkAuthorityClass,
    /// Stable token for [`Self::authority`].
    pub authority_token: String,
    /// Export-safe label naming the effective value (never a raw secret).
    pub effective_value_label: String,
    /// Reviewable source label naming who supplied the setting.
    pub source_label: String,
    /// Reviewable lock reason rendered next to the value.
    pub lock_reason: String,
    /// Optional reviewable pointer to the signature blob or attribution ref.
    pub managed_attribution_ref: String,
    /// True when no undeclared public endpoint fallback is permitted.
    pub no_public_endpoint_fallback: bool,
}

/// One live beta row covering an effective network facet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTrustBetaRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Facet class.
    pub facet: NetworkTrustBetaFacetClass,
    /// Stable token for [`Self::facet`].
    pub facet_token: String,
    /// Source matrix lab-row anchor this beta row implements.
    pub source_matrix_ref: String,
    /// Profile bindings (one per profile).
    pub profile_bindings: Vec<NetworkTrustBetaProfileBinding>,
    /// Consumer lanes that read this row's effective value.
    pub consumer_lanes: Vec<NetworkConsumerLaneClass>,
    /// Stable tokens for [`Self::consumer_lanes`].
    pub consumer_lane_tokens: Vec<String>,
    /// Export-safe summary shown in support packets.
    pub support_export_summary: String,
    /// True when no undeclared public endpoint fallback is allowed on any binding.
    pub no_public_endpoint_fallback: bool,
    /// True when raw secret / private-key material is excluded from the record.
    pub raw_secret_or_private_material_excluded: bool,
}

/// Export-safe support row aligned one-to-one with a live beta row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTrustBetaSupportRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Live row id.
    pub row_id: String,
    /// Facet token copied from the live row.
    pub facet_token: String,
    /// Per-profile source tokens.
    pub source_tokens_by_profile: BTreeMap<String, String>,
    /// Per-profile lock tokens.
    pub lock_tokens_by_profile: BTreeMap<String, String>,
    /// Per-profile authority tokens.
    pub authority_tokens_by_profile: BTreeMap<String, String>,
    /// Per-profile effective-value labels (export-safe, no raw secrets).
    pub effective_value_labels_by_profile: BTreeMap<String, String>,
    /// Consumer-lane tokens copied from the live row.
    pub consumer_lane_tokens: Vec<String>,
    /// Export-safe support summary.
    pub support_export_summary: String,
    /// True when no undeclared public endpoint fallback is allowed.
    pub no_public_endpoint_fallback: bool,
    /// True when raw secret / private-key material is excluded.
    pub raw_secret_or_private_material_excluded: bool,
}

impl NetworkTrustBetaSupportRow {
    /// Builds an export-safe row from a live beta row.
    pub fn from_row(row: &NetworkTrustBetaRow) -> Self {
        let mut source_tokens_by_profile = BTreeMap::new();
        let mut lock_tokens_by_profile = BTreeMap::new();
        let mut authority_tokens_by_profile = BTreeMap::new();
        let mut effective_value_labels_by_profile = BTreeMap::new();
        for binding in &row.profile_bindings {
            source_tokens_by_profile
                .insert(binding.profile_token.clone(), binding.source_token.clone());
            lock_tokens_by_profile
                .insert(binding.profile_token.clone(), binding.lock_token.clone());
            authority_tokens_by_profile.insert(
                binding.profile_token.clone(),
                binding.authority_token.clone(),
            );
            effective_value_labels_by_profile.insert(
                binding.profile_token.clone(),
                binding.effective_value_label.clone(),
            );
        }
        Self {
            record_kind: NETWORK_TRUST_BETA_SUPPORT_ROW_RECORD_KIND.to_owned(),
            schema_version: NETWORK_TRUST_BETA_SCHEMA_VERSION,
            shared_contract_ref: NETWORK_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: row.row_id.clone(),
            facet_token: row.facet_token.clone(),
            source_tokens_by_profile,
            lock_tokens_by_profile,
            authority_tokens_by_profile,
            effective_value_labels_by_profile,
            consumer_lane_tokens: row.consumer_lane_tokens.clone(),
            support_export_summary: row.support_export_summary.clone(),
            no_public_endpoint_fallback: row.no_public_endpoint_fallback,
            raw_secret_or_private_material_excluded: row.raw_secret_or_private_material_excluded,
        }
    }
}

/// Typed validation defect for the network-trust beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTrustBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: NetworkTrustBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Row id, or `page` for page-level defects.
    pub row_id: String,
    /// Facet token, or `page` for page-level defects.
    pub facet_token: String,
    /// Profile token, or `*` when not bound to a single profile.
    pub profile_token: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl NetworkTrustBetaDefect {
    fn new(
        defect_kind: NetworkTrustBetaDefectKind,
        row_id: impl Into<String>,
        facet_token: impl Into<String>,
        profile_token: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: NETWORK_TRUST_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: NETWORK_TRUST_BETA_SCHEMA_VERSION,
            shared_contract_ref: NETWORK_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            row_id: row_id.into(),
            facet_token: facet_token.into(),
            profile_token: profile_token.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the network-trust beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTrustBetaSummary {
    /// Stable record kind for the parent page.
    pub page_record_kind: String,
    /// Stable record kind for the summary itself.
    pub record_kind: String,
    /// Number of live rows.
    pub row_count: usize,
    /// Number of support rows.
    pub support_row_count: usize,
    /// Facet tokens present on the page.
    pub facets_present: Vec<String>,
    /// Profile tokens present on every valid row.
    pub profiles_present: Vec<String>,
    /// Source tokens present across the page.
    pub source_tokens_present: Vec<String>,
    /// Lock tokens present across the page.
    pub lock_tokens_present: Vec<String>,
    /// Consumer-lane tokens present across the page.
    pub consumer_lane_tokens_present: Vec<String>,
    /// Number of bindings whose authority is `effective_value_published`.
    pub effective_value_published_count: usize,
    /// Number of bindings whose authority blocks (missing input or managed policy).
    pub blocked_binding_count: usize,
    /// Defect count.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl NetworkTrustBetaSummary {
    /// Builds a summary from live rows, support rows, and defects.
    pub fn from_rows(
        rows: &[NetworkTrustBetaRow],
        support_rows: &[NetworkTrustBetaSupportRow],
        defects: &[NetworkTrustBetaDefect],
    ) -> Self {
        let facets_present: BTreeSet<String> =
            rows.iter().map(|row| row.facet_token.clone()).collect();
        let profiles_present: BTreeSet<String> = rows
            .iter()
            .flat_map(|row| row.profile_bindings.iter().map(|b| b.profile_token.clone()))
            .collect();
        let source_tokens_present: BTreeSet<String> = rows
            .iter()
            .flat_map(|row| row.profile_bindings.iter().map(|b| b.source_token.clone()))
            .collect();
        let lock_tokens_present: BTreeSet<String> = rows
            .iter()
            .flat_map(|row| row.profile_bindings.iter().map(|b| b.lock_token.clone()))
            .collect();
        let consumer_lane_tokens_present: BTreeSet<String> = rows
            .iter()
            .flat_map(|row| row.consumer_lane_tokens.iter().cloned())
            .collect();
        let mut effective_value_published_count = 0_usize;
        let mut blocked_binding_count = 0_usize;
        for binding in rows.iter().flat_map(|row| row.profile_bindings.iter()) {
            match binding.authority {
                NetworkAuthorityClass::EffectiveValuePublished => {
                    effective_value_published_count += 1;
                }
                NetworkAuthorityClass::BlockedMissingInput
                | NetworkAuthorityClass::BlockedManagedPolicy => {
                    blocked_binding_count += 1;
                }
                _ => {}
            }
        }
        let mut defect_counts_by_kind = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }
        Self {
            page_record_kind: NETWORK_TRUST_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: NETWORK_TRUST_BETA_SUMMARY_RECORD_KIND.to_owned(),
            row_count: rows.len(),
            support_row_count: support_rows.len(),
            facets_present: facets_present.into_iter().collect(),
            profiles_present: profiles_present.into_iter().collect(),
            source_tokens_present: source_tokens_present.into_iter().collect(),
            lock_tokens_present: lock_tokens_present.into_iter().collect(),
            consumer_lane_tokens_present: consumer_lane_tokens_present.into_iter().collect(),
            effective_value_published_count,
            blocked_binding_count,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level beta page consumed by admin/settings center, support export,
/// shell summary, headless inspector, and fixture replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTrustBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Live beta rows.
    pub rows: Vec<NetworkTrustBetaRow>,
    /// Support/export rows.
    pub support_rows: Vec<NetworkTrustBetaSupportRow>,
    /// Typed validation defects.
    pub defects: Vec<NetworkTrustBetaDefect>,
    /// Aggregate summary.
    pub summary: NetworkTrustBetaSummary,
}

/// Support-export wrapper for the network-trust beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTrustBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported page.
    pub page: NetworkTrustBetaPage,
    /// Defect kind tokens present.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw secret / private-key material is excluded.
    pub raw_secret_or_private_material_excluded: bool,
}

impl NetworkTrustBetaSupportExport {
    /// Builds a support-export wrapper from a beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: NetworkTrustBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        Self {
            record_kind: NETWORK_TRUST_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: NETWORK_TRUST_BETA_SCHEMA_VERSION,
            shared_contract_ref: NETWORK_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_secret_or_private_material_excluded: true,
        }
    }
}

/// Builds the seeded network-trust beta page covering all four facets across
/// connected, mirror-only, offline, and enterprise-managed profiles.
pub fn seeded_network_trust_beta_page() -> NetworkTrustBetaPage {
    let rows: Vec<NetworkTrustBetaRow> = NetworkTrustBetaFacetClass::ALL
        .iter()
        .copied()
        .map(seed_row)
        .collect();
    let support_rows: Vec<NetworkTrustBetaSupportRow> = rows
        .iter()
        .map(NetworkTrustBetaSupportRow::from_row)
        .collect();
    let defects = audit_network_trust_beta_rows(&rows, &support_rows);
    let summary = NetworkTrustBetaSummary::from_rows(&rows, &support_rows, &defects);
    NetworkTrustBetaPage {
        record_kind: NETWORK_TRUST_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: NETWORK_TRUST_BETA_SCHEMA_VERSION,
        shared_contract_ref: NETWORK_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: NETWORK_TRUST_BETA_SOURCE_MATRIX_REF.to_owned(),
        rows,
        support_rows,
        defects,
        summary,
    }
}

/// Validates a beta page and returns typed defects on failure.
pub fn validate_network_trust_beta_page(
    page: &NetworkTrustBetaPage,
) -> Result<(), Vec<NetworkTrustBetaDefect>> {
    let defects = audit_network_trust_beta_rows(&page.rows, &page.support_rows);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for network-trust beta rows and support rows.
pub fn audit_network_trust_beta_rows(
    rows: &[NetworkTrustBetaRow],
    support_rows: &[NetworkTrustBetaSupportRow],
) -> Vec<NetworkTrustBetaDefect> {
    let mut defects = Vec::new();
    let expected_facets: BTreeSet<&str> = NetworkTrustBetaFacetClass::ALL
        .iter()
        .map(|facet| facet.as_str())
        .collect();
    let observed_facets: BTreeSet<&str> = rows.iter().map(|row| row.facet_token.as_str()).collect();
    for missing in expected_facets.difference(&observed_facets) {
        defects.push(NetworkTrustBetaDefect::new(
            NetworkTrustBetaDefectKind::MissingFacetCoverage,
            "page",
            *missing,
            "*",
            "facet",
            "claimed beta facet is missing from the network-trust page",
        ));
    }

    let support_by_row: BTreeMap<&str, &NetworkTrustBetaSupportRow> = support_rows
        .iter()
        .map(|support| (support.row_id.as_str(), support))
        .collect();

    for row in rows {
        if row.facet_token != row.facet.as_str() {
            defects.push(row_defect(
                row,
                "*",
                NetworkTrustBetaDefectKind::FacetTokenDrift,
                "facet_token",
                "facet_token must match facet class",
            ));
        }

        if !row.no_public_endpoint_fallback {
            defects.push(row_defect(
                row,
                "*",
                NetworkTrustBetaDefectKind::HiddenPublicEndpointFallback,
                "no_public_endpoint_fallback",
                "row permits undeclared public endpoint fallback",
            ));
        }

        if !row.raw_secret_or_private_material_excluded {
            defects.push(row_defect(
                row,
                "*",
                NetworkTrustBetaDefectKind::RawSecretOrPrivateMaterialExposed,
                "raw_secret_or_private_material_excluded",
                "network-trust rows must be export-safe metadata",
            ));
        }

        let observed_profiles: BTreeSet<&str> = row
            .profile_bindings
            .iter()
            .map(|binding| binding.profile_token.as_str())
            .collect();
        for expected in NetworkTrustBetaProfileClass::ALL {
            if !observed_profiles.contains(expected.as_str()) {
                defects.push(row_defect(
                    row,
                    expected.as_str(),
                    NetworkTrustBetaDefectKind::MissingProfileCoverage,
                    "profile_bindings",
                    format!("missing {} profile binding", expected.as_str()),
                ));
            }
        }

        let observed_lanes: BTreeSet<&str> = row
            .consumer_lane_tokens
            .iter()
            .map(String::as_str)
            .collect();
        for expected in NetworkConsumerLaneClass::ALL {
            if !observed_lanes.contains(expected.as_str()) {
                defects.push(row_defect(
                    row,
                    "*",
                    NetworkTrustBetaDefectKind::MissingConsumerLaneCoverage,
                    "consumer_lane_tokens",
                    format!("missing {} consumer lane", expected.as_str()),
                ));
            }
        }

        for binding in &row.profile_bindings {
            if binding.source_token != binding.source_class.as_str() {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkTrustBetaDefectKind::SourceTokenDrift,
                    "source_token",
                    "source_token must match source_class",
                ));
            }
            if binding.lock_token != binding.lock_class.as_str() {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkTrustBetaDefectKind::LockTokenDrift,
                    "lock_token",
                    "lock_token must match lock_class",
                ));
            }
            if binding.authority_token != binding.authority.as_str() {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkTrustBetaDefectKind::LockTokenDrift,
                    "authority_token",
                    "authority_token must match authority",
                ));
            }
            if binding.lock_class.requires_managed_source() && !binding.source_class.is_managed() {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkTrustBetaDefectKind::LockInconsistentWithSource,
                    "lock_class",
                    "managed lock requires a managed source",
                ));
            }
            if binding.source_class.is_managed() && binding.managed_attribution_ref.is_empty() {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkTrustBetaDefectKind::UnsignedManagedAuthority,
                    "managed_attribution_ref",
                    "managed source must publish a signature or attribution pointer",
                ));
            }
            if !binding.no_public_endpoint_fallback {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkTrustBetaDefectKind::HiddenPublicEndpointFallback,
                    "no_public_endpoint_fallback",
                    "profile binding permits undeclared public endpoint fallback",
                ));
            }
            if binding.authority == NetworkAuthorityClass::EffectiveValuePublished
                && binding.effective_value_label.is_empty()
            {
                defects.push(row_defect(
                    row,
                    binding.profile_token.as_str(),
                    NetworkTrustBetaDefectKind::EmptyEffectiveValueLabel,
                    "effective_value_label",
                    "published authority must carry a reviewable label",
                ));
            }
        }

        match support_by_row.get(row.row_id.as_str()) {
            Some(support) => compare_support_row(row, support, &mut defects),
            None => defects.push(row_defect(
                row,
                "*",
                NetworkTrustBetaDefectKind::SupportRowVocabularyDrift,
                "support_rows",
                "missing support row for live network-trust row",
            )),
        }
    }

    defects
}

fn compare_support_row(
    row: &NetworkTrustBetaRow,
    support: &NetworkTrustBetaSupportRow,
    defects: &mut Vec<NetworkTrustBetaDefect>,
) {
    let mut sources: BTreeMap<String, String> = BTreeMap::new();
    let mut locks: BTreeMap<String, String> = BTreeMap::new();
    let mut authorities: BTreeMap<String, String> = BTreeMap::new();
    let mut labels: BTreeMap<String, String> = BTreeMap::new();
    for binding in &row.profile_bindings {
        sources.insert(binding.profile_token.clone(), binding.source_token.clone());
        locks.insert(binding.profile_token.clone(), binding.lock_token.clone());
        authorities.insert(
            binding.profile_token.clone(),
            binding.authority_token.clone(),
        );
        labels.insert(
            binding.profile_token.clone(),
            binding.effective_value_label.clone(),
        );
    }
    if support.facet_token != row.facet_token
        || support.source_tokens_by_profile != sources
        || support.lock_tokens_by_profile != locks
        || support.authority_tokens_by_profile != authorities
        || support.effective_value_labels_by_profile != labels
        || support.consumer_lane_tokens != row.consumer_lane_tokens
        || support.support_export_summary != row.support_export_summary
        || support.no_public_endpoint_fallback != row.no_public_endpoint_fallback
        || support.raw_secret_or_private_material_excluded
            != row.raw_secret_or_private_material_excluded
    {
        defects.push(row_defect(
            row,
            "*",
            NetworkTrustBetaDefectKind::SupportRowVocabularyDrift,
            "support_row",
            "support/export row drifted from live row vocabulary",
        ));
    }
}

fn row_defect(
    row: &NetworkTrustBetaRow,
    profile_token: impl Into<String>,
    kind: NetworkTrustBetaDefectKind,
    field: impl Into<String>,
    note: impl Into<String>,
) -> NetworkTrustBetaDefect {
    NetworkTrustBetaDefect::new(
        kind,
        row.row_id.clone(),
        row.facet_token.clone(),
        profile_token,
        field,
        note,
    )
}

fn seed_row(facet: NetworkTrustBetaFacetClass) -> NetworkTrustBetaRow {
    let bindings: Vec<NetworkTrustBetaProfileBinding> = NetworkTrustBetaProfileClass::ALL
        .iter()
        .copied()
        .map(|profile| seed_binding(facet, profile))
        .collect();
    let support_export_summary = format!(
        "{}: connected={}, mirror_only={}, offline={}, enterprise_managed={}; no public fallback; raw secrets excluded.",
        facet.as_str(),
        bindings[0].authority_token,
        bindings[1].authority_token,
        bindings[2].authority_token,
        bindings[3].authority_token,
    );
    NetworkTrustBetaRow {
        record_kind: NETWORK_TRUST_BETA_ROW_RECORD_KIND.to_owned(),
        schema_version: NETWORK_TRUST_BETA_SCHEMA_VERSION,
        shared_contract_ref: NETWORK_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
        row_id: format!("network_trust_beta:{}", facet.as_str()),
        facet,
        facet_token: facet.as_str().to_owned(),
        source_matrix_ref: format!(
            "{}#{}",
            NETWORK_TRUST_BETA_SOURCE_MATRIX_REF,
            facet.as_str()
        ),
        profile_bindings: bindings,
        consumer_lanes: NetworkConsumerLaneClass::ALL.to_vec(),
        consumer_lane_tokens: NetworkConsumerLaneClass::ALL
            .iter()
            .map(|lane| lane.as_str().to_owned())
            .collect(),
        support_export_summary,
        no_public_endpoint_fallback: true,
        raw_secret_or_private_material_excluded: true,
    }
}

fn seed_binding(
    facet: NetworkTrustBetaFacetClass,
    profile: NetworkTrustBetaProfileClass,
) -> NetworkTrustBetaProfileBinding {
    let (source, lock, authority, value, source_label, lock_reason, attribution) =
        seed_binding_fields(facet, profile);
    NetworkTrustBetaProfileBinding {
        record_kind: NETWORK_TRUST_BETA_PROFILE_BINDING_RECORD_KIND.to_owned(),
        schema_version: NETWORK_TRUST_BETA_SCHEMA_VERSION,
        shared_contract_ref: NETWORK_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
        profile_class: profile,
        profile_token: profile.as_str().to_owned(),
        source_class: source,
        source_token: source.as_str().to_owned(),
        lock_class: lock,
        lock_token: lock.as_str().to_owned(),
        authority,
        authority_token: authority.as_str().to_owned(),
        effective_value_label: value.to_owned(),
        source_label: source_label.to_owned(),
        lock_reason: lock_reason.to_owned(),
        managed_attribution_ref: attribution.to_owned(),
        no_public_endpoint_fallback: true,
    }
}

fn seed_binding_fields(
    facet: NetworkTrustBetaFacetClass,
    profile: NetworkTrustBetaProfileClass,
) -> (
    NetworkSettingSourceClass,
    NetworkSettingLockClass,
    NetworkAuthorityClass,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    match (facet, profile) {
        // Proxy
        (NetworkTrustBetaFacetClass::Proxy, NetworkTrustBetaProfileClass::Connected) => (
            NetworkSettingSourceClass::ProcessEnvironment,
            NetworkSettingLockClass::UserEditable,
            NetworkAuthorityClass::EffectiveValuePublished,
            "HTTPS_PROXY=https://proxy.example:8443 (NO_PROXY=localhost)",
            "Process environment HTTPS_PROXY / NO_PROXY",
            "User-editable; precedence yields to manual or PAC proxy when policy locks them.",
            "",
        ),
        (NetworkTrustBetaFacetClass::Proxy, NetworkTrustBetaProfileClass::MirrorOnly) => (
            NetworkSettingSourceClass::SignedMirrorPolicy,
            NetworkSettingLockClass::SignedManagedPolicyLocked,
            NetworkAuthorityClass::EffectiveValuePublished,
            "manual_proxy https://mirror-proxy.example:443 (no public fallback)",
            "Signed mirror policy of the admin proxy",
            "Locked by signed managed policy delivered through the signed mirror.",
            "artifacts/network/signatures/mirror-proxy.sig",
        ),
        (NetworkTrustBetaFacetClass::Proxy, NetworkTrustBetaProfileClass::Offline) => (
            NetworkSettingSourceClass::AirGappedSignedTransfer,
            NetworkSettingLockClass::SignedManagedPolicyLocked,
            NetworkAuthorityClass::BlockedMissingInput,
            "direct egress disabled; offline-deferred queue is idempotent only",
            "Air-gapped signed transfer of admin policy",
            "Locked by signed managed policy delivered through the air-gapped bundle.",
            "artifacts/network/signatures/airgap-proxy.sig",
        ),
        (NetworkTrustBetaFacetClass::Proxy, NetworkTrustBetaProfileClass::EnterpriseManaged) => (
            NetworkSettingSourceClass::AdminManagedPolicy,
            NetworkSettingLockClass::SignedManagedPolicyLocked,
            NetworkAuthorityClass::EffectiveValuePublished,
            "manual_proxy https://proxy.corp.example:8443 (NO_PROXY=*.corp.example)",
            "Admin-managed policy from vendor-managed origin",
            "Locked by signed managed policy; user proxy settings are ignored.",
            "artifacts/network/signatures/managed-proxy.sig",
        ),

        // Trust store / CA
        (NetworkTrustBetaFacetClass::TrustStore, NetworkTrustBetaProfileClass::Connected) => (
            NetworkSettingSourceClass::PlatformSystemSetting,
            NetworkSettingLockClass::PlatformLocked,
            NetworkAuthorityClass::EffectiveValuePublished,
            "os_trust_store",
            "Platform OS trust store",
            "Platform-locked; admin or signed policy can overlay an org CA bundle.",
            "",
        ),
        (NetworkTrustBetaFacetClass::TrustStore, NetworkTrustBetaProfileClass::MirrorOnly) => (
            NetworkSettingSourceClass::SignedMirrorPolicy,
            NetworkSettingLockClass::SignedManagedPolicyLocked,
            NetworkAuthorityClass::EffectiveValuePublished,
            "mirror_pinned_bundle sha256:7a0d...e1 (no public fallback)",
            "Signed mirror policy delivering pinned bundle",
            "Locked by signed mirror policy; OS trust store is bypassed.",
            "artifacts/network/signatures/mirror-ca-bundle.sig",
        ),
        (NetworkTrustBetaFacetClass::TrustStore, NetworkTrustBetaProfileClass::Offline) => (
            NetworkSettingSourceClass::AirGappedSignedTransfer,
            NetworkSettingLockClass::SignedManagedPolicyLocked,
            NetworkAuthorityClass::EffectiveValuePublished,
            "air_gapped_pinned_bundle sha256:4b2c...90 (offline)",
            "Air-gapped signed transfer delivering pinned bundle",
            "Locked by signed managed policy delivered through the air-gapped bundle.",
            "artifacts/network/signatures/airgap-ca-bundle.sig",
        ),
        (
            NetworkTrustBetaFacetClass::TrustStore,
            NetworkTrustBetaProfileClass::EnterpriseManaged,
        ) => (
            NetworkSettingSourceClass::AdminManagedPolicy,
            NetworkSettingLockClass::SignedManagedPolicyLocked,
            NetworkAuthorityClass::EffectiveValuePublished,
            "os_trust_store_plus_org_ca_bundle sha256:c0ff...ee",
            "Admin-managed policy from vendor-managed origin",
            "Locked by signed managed policy; org CA bundle overlay is mandatory.",
            "artifacts/network/signatures/managed-ca-bundle.sig",
        ),

        // SSH host proof
        (NetworkTrustBetaFacetClass::SshHostProof, NetworkTrustBetaProfileClass::Connected) => (
            NetworkSettingSourceClass::UserWorkspaceSetting,
            NetworkSettingLockClass::UserEditable,
            NetworkAuthorityClass::EffectiveValuePublished,
            "known_hosts_match (user known_hosts)",
            "User workspace setting",
            "User-editable; remote-attach connections still rely on the matched host key.",
            "",
        ),
        (NetworkTrustBetaFacetClass::SshHostProof, NetworkTrustBetaProfileClass::MirrorOnly) => (
            NetworkSettingSourceClass::SignedMirrorPolicy,
            NetworkSettingLockClass::SignedManagedPolicyLocked,
            NetworkAuthorityClass::EffectiveValuePublished,
            "strict_pinned sha256:9f8e...12 (mirror-delivered known_hosts)",
            "Signed mirror policy",
            "Locked by signed mirror policy; mismatched host keys are rejected.",
            "artifacts/network/signatures/mirror-known-hosts.sig",
        ),
        (NetworkTrustBetaFacetClass::SshHostProof, NetworkTrustBetaProfileClass::Offline) => (
            NetworkSettingSourceClass::AirGappedSignedTransfer,
            NetworkSettingLockClass::SignedManagedPolicyLocked,
            NetworkAuthorityClass::DegradedPreviewOnly,
            "strict_pinned sha256:9f8e...12 (offline cache, read-only attach)",
            "Air-gapped signed transfer",
            "Locked by signed managed policy; remote attach is read-only offline.",
            "artifacts/network/signatures/airgap-known-hosts.sig",
        ),
        (
            NetworkTrustBetaFacetClass::SshHostProof,
            NetworkTrustBetaProfileClass::EnterpriseManaged,
        ) => (
            NetworkSettingSourceClass::AdminManagedPolicy,
            NetworkSettingLockClass::SignedManagedPolicyLocked,
            NetworkAuthorityClass::EffectiveValuePublished,
            "strict_pinned sha256:1234...ab (admin known_hosts)",
            "Admin-managed policy from vendor-managed origin",
            "Locked by signed managed policy; user known_hosts edits are ignored.",
            "artifacts/network/signatures/managed-known-hosts.sig",
        ),

        // Client certificate (mTLS)
        (
            NetworkTrustBetaFacetClass::ClientCertificate,
            NetworkTrustBetaProfileClass::Connected,
        ) => (
            NetworkSettingSourceClass::BaselineDefault,
            NetworkSettingLockClass::NotApplicable,
            NetworkAuthorityClass::NotApplicable,
            "none_required",
            "Baseline default",
            "Not applicable on the connected consumer profile.",
            "",
        ),
        (
            NetworkTrustBetaFacetClass::ClientCertificate,
            NetworkTrustBetaProfileClass::MirrorOnly,
        ) => (
            NetworkSettingSourceClass::SignedMirrorPolicy,
            NetworkSettingLockClass::SignedManagedPolicyLocked,
            NetworkAuthorityClass::EffectiveValuePublished,
            "present_valid: cn=workstation.dev.example, exp=2026-12-31",
            "Signed mirror policy",
            "Locked by signed managed policy; private key remains in OS keychain.",
            "artifacts/network/signatures/mirror-client-cert.sig",
        ),
        (NetworkTrustBetaFacetClass::ClientCertificate, NetworkTrustBetaProfileClass::Offline) => (
            NetworkSettingSourceClass::AirGappedSignedTransfer,
            NetworkSettingLockClass::SignedManagedPolicyLocked,
            NetworkAuthorityClass::BlockedMissingInput,
            "missing (air-gapped: enrollment requires connected network)",
            "Air-gapped signed transfer",
            "Locked by signed managed policy; mTLS enrollment unavailable offline.",
            "artifacts/network/signatures/airgap-client-cert.sig",
        ),
        (
            NetworkTrustBetaFacetClass::ClientCertificate,
            NetworkTrustBetaProfileClass::EnterpriseManaged,
        ) => (
            NetworkSettingSourceClass::AdminManagedPolicy,
            NetworkSettingLockClass::SignedManagedPolicyLocked,
            NetworkAuthorityClass::EffectiveValuePublished,
            "present_valid: cn=workstation.corp.example, exp=2026-09-30",
            "Admin-managed policy from vendor-managed origin",
            "Locked by signed managed policy; private key remains in OS keychain.",
            "artifacts/network/signatures/managed-client-cert.sig",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_covers_every_facet_and_profile_with_zero_defects() {
        let page = seeded_network_trust_beta_page();
        assert_eq!(page.rows.len(), NetworkTrustBetaFacetClass::ALL.len());
        assert_eq!(page.support_rows.len(), page.rows.len());
        assert!(page.defects.is_empty());
        validate_network_trust_beta_page(&page).expect("seeded page validates");
        for facet in NetworkTrustBetaFacetClass::ALL {
            let row = page
                .rows
                .iter()
                .find(|row| row.facet == facet)
                .expect("row");
            assert_eq!(
                row.profile_bindings.len(),
                NetworkTrustBetaProfileClass::ALL.len()
            );
            assert_eq!(
                row.consumer_lanes.len(),
                NetworkConsumerLaneClass::ALL.len()
            );
        }
        assert!(page
            .summary
            .profiles_present
            .contains(&"mirror_only".to_owned()));
        assert!(page
            .summary
            .consumer_lane_tokens_present
            .contains(&"extension".to_owned()));
        assert!(page
            .summary
            .source_tokens_present
            .contains(&"admin_managed_policy".to_owned()));
        assert!(page
            .summary
            .lock_tokens_present
            .contains(&"signed_managed_policy_locked".to_owned()));
    }

    #[test]
    fn validator_rejects_managed_lock_without_managed_source() {
        let mut page = seeded_network_trust_beta_page();
        let binding = page.rows[0]
            .profile_bindings
            .iter_mut()
            .find(|b| b.profile_class == NetworkTrustBetaProfileClass::Connected)
            .expect("binding");
        binding.lock_class = NetworkSettingLockClass::SignedManagedPolicyLocked;
        binding.lock_token = NetworkSettingLockClass::SignedManagedPolicyLocked
            .as_str()
            .to_owned();
        let defects = audit_network_trust_beta_rows(&page.rows, &page.support_rows);
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == NetworkTrustBetaDefectKind::LockInconsistentWithSource));
    }

    #[test]
    fn validator_rejects_unsigned_managed_authority() {
        let mut page = seeded_network_trust_beta_page();
        let binding = page.rows[0]
            .profile_bindings
            .iter_mut()
            .find(|b| b.profile_class == NetworkTrustBetaProfileClass::EnterpriseManaged)
            .expect("binding");
        binding.managed_attribution_ref.clear();
        let defects = audit_network_trust_beta_rows(&page.rows, &page.support_rows);
        assert!(defects.iter().any(
            |defect| defect.defect_kind == NetworkTrustBetaDefectKind::UnsignedManagedAuthority
        ));
    }

    #[test]
    fn validator_rejects_support_row_drift() {
        let mut page = seeded_network_trust_beta_page();
        page.support_rows[0]
            .effective_value_labels_by_profile
            .insert("connected".to_owned(), "drifted_label".to_owned());
        let defects = audit_network_trust_beta_rows(&page.rows, &page.support_rows);
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == NetworkTrustBetaDefectKind::SupportRowVocabularyDrift));
    }

    #[test]
    fn validator_rejects_missing_consumer_lane() {
        let mut page = seeded_network_trust_beta_page();
        page.rows[0]
            .consumer_lane_tokens
            .retain(|lane| lane != "ai");
        page.rows[0]
            .consumer_lanes
            .retain(|lane| *lane != NetworkConsumerLaneClass::Ai);
        // Resync support row to avoid a drift defect masking the lane defect:
        let row = page.rows[0].clone();
        page.support_rows[0] = NetworkTrustBetaSupportRow::from_row(&row);
        let defects = audit_network_trust_beta_rows(&page.rows, &page.support_rows);
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == NetworkTrustBetaDefectKind::MissingConsumerLaneCoverage));
    }
}
