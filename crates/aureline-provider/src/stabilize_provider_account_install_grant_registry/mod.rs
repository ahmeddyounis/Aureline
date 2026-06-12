//! Stabilized provider account/install-grant registry, project mapping, and
//! work-item lane authority for stable provider-linked rows.
//!
//! This module turns the beta account-scope and target-mapping contracts into
//! one stable registry packet. Every claimed provider lane MUST be able to
//! answer: who is Aureline acting as, which project/board/space is targeted,
//! what is the current health state, which object types are supported, and
//! which action mode is available (read-only, comment/link, full-edit,
//! offline-capture-only, publish-later, or handoff-only).
//!
//! The record family includes:
//!
//! - [`StableProviderAccountRecord`] — stable connected-account entry with
//!   provider descriptor, canonical host, org/tenant scope, acting-as identity,
//!   health state, and supported object types.
//! - [`StableInstallGrantRecord`] — stable installation-grant entry with
//!   issuer, bounded scope, grant lifecycle, and supported object types.
//! - [`StableMappingReviewRow`] — mapping-review row showing current target,
//!   fallback mapping, stale or policy-blocked state, and the action mode
//!   available on the lane.
//! - [`StableRegistryRecord`] — top-level registry record binding the page.
//! - [`StableRegistrySupportExportPacket`] — redaction-safe support export.
//! - [`StableRegistryInspectionRecord`] — compact boolean projection for CLI
//!   and inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/providers/stable_provider_account_install_grant_registry.schema.json`.
//! Canonical fixtures live under
//! `fixtures/providers/m4/stabilize-provider-account-install-grant-registry/`.

use std::collections::BTreeSet;
use std::fmt;

use aureline_auth::{
    secret_boundary_use_audit_result_for_health, seeded_secret_boundary_profile_parity_rows,
    SecretBoundaryActingIdentityClass, SecretBoundaryConsumerIdentityClass,
    SecretBoundaryConsumerIdentityReceipt, SecretBoundaryCredentialMode,
    SecretBoundaryCredentialStateRow, SecretBoundaryDeclinePath,
    SecretBoundaryDelegatedCredentialRow, SecretBoundaryDelegatedUseClass,
    SecretBoundaryExportSafetyBanner, SecretBoundaryHealthStateClass,
    SecretBoundaryProjectionControl, SecretBoundaryProjectionControlClass,
    SecretBoundaryProjectionMode, SecretBoundaryProjectionModeAudit,
    SecretBoundaryRepairOwnerClass, SecretBoundarySecretAccessPrompt,
    SecretBoundarySecretClass, SecretBoundaryStorageClass, SecretBoundarySurfaceState,
    SecretBoundaryVaultPickerOption, SecretBoundaryVaultPickerState,
    SecretBoundaryWorkflowDependency, M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF,
};
use serde::{Deserialize, Serialize};

use crate::account_scope::ActingIdentityClass;
use crate::project_mapping::{MappingLaneClass, TargetKindClass};
use crate::registry::{ProviderFamily, ProviderObjectKind};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable provider-account registry record.
pub const STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every stable registry record.
pub const STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SHARED_CONTRACT_REF: &str =
    "providers:stable_account_install_grant_registry:v1";

/// Stable record-kind tag for [`StableProviderAccountInstallGrantRegistryPacket`].
pub const STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_PACKET_RECORD_KIND: &str =
    "providers_stable_account_install_grant_registry_packet";

/// Stable record-kind tag for [`StableRegistryRecord`].
pub const STABLE_REGISTRY_RECORD_KIND: &str = "providers_stable_registry_record";

/// Stable record-kind tag for [`StableProviderAccountRecord`].
pub const STABLE_PROVIDER_ACCOUNT_RECORD_KIND: &str = "providers_stable_provider_account_record";

const REGISTRY_AUTH_MATRIX_ROW_ID: &str = "m5.secret.registry.package_auth";

/// Stable record-kind tag for [`StableInstallGrantRecord`].
pub const STABLE_INSTALL_GRANT_RECORD_KIND: &str = "providers_stable_install_grant_record";

/// Stable record-kind tag for [`StableMappingReviewRow`].
pub const STABLE_MAPPING_REVIEW_ROW_RECORD_KIND: &str =
    "providers_stable_mapping_review_row_record";

/// Stable record-kind tag for [`StableRegistrySupportExportPacket`].
pub const STABLE_REGISTRY_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "providers_stable_registry_support_export_packet";

/// Stable record-kind tag for [`StableRegistryInspectionRecord`].
pub const STABLE_REGISTRY_INSPECTION_RECORD_KIND: &str =
    "providers_stable_registry_inspection_record";

/// Closed set of registry health states.
pub const REGISTRY_HEALTH_STATES: &[&str] = &[
    "healthy",
    "degraded_stale_credentials",
    "degraded_limited_scope_session",
    "blocked_policy_locked_mapping",
    "blocked_provider_unreachable",
    "blocked_auth_loss",
    "offline_capture_only",
];

/// Closed set of action mode classes available on a provider lane.
pub const ACTION_MODE_CLASSES: &[&str] = &[
    "read_only",
    "comment_or_link",
    "full_edit",
    "offline_capture_only",
    "publish_later",
    "handoff_only",
];

/// Closed set of supported object type classes for issue/work-item lanes.
pub const SUPPORTED_OBJECT_TYPE_CLASSES: &[&str] = &[
    "issue",
    "work_item",
    "pull_request",
    "comment",
    "label",
    "milestone",
    "review_decision",
    "incident",
];

/// Closed set of mapping stale-state classes.
pub const MAPPING_STALE_STATE_CLASSES: &[&str] = &[
    "current",
    "stale_within_grace",
    "stale_blocks_mutation",
    "policy_blocked",
    "fallback_active",
];

/// Closed set of consumer surfaces for stable registry packets.
pub const STABLE_REGISTRY_CONSUMER_SURFACES: &[&str] = &[
    "provider_account_inspector",
    "project_mapping_inspector",
    "work_item_lane",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "browser_companion",
    "offline_handoff",
];

/// Closed set of invalidation reasons that mark a registry stale.
pub const STABLE_REGISTRY_INVALIDATION_REASONS: &[&str] = &[
    "stale_credentials",
    "provider_drift",
    "mapping_changed",
    "policy_epoch_changed",
    "actor_scope_changed",
    "auth_revoked",
    "reconnect_required",
    "limited_scope_session",
];

/// Closed set of command classes for the stable registry lane.
pub const STABLE_REGISTRY_COMMAND_CLASSES: &[&str] = &[
    "preview_account_scope",
    "preview_mapping_target",
    "refresh_credentials",
    "reselect_account",
    "reconsent_installation_grant",
    "queue_publish_later",
    "export_offline_handoff",
    "open_in_provider",
];

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Health state of a stable provider-account or install-grant registry entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryHealthStateClass {
    /// Entry is healthy and authoritative.
    Healthy,
    /// Credentials are stale but within a grace window.
    DegradedStaleCredentials,
    /// Session has limited scope and may downgrade mutations.
    DegradedLimitedScopeSession,
    /// Mapping is policy-locked and mutations are blocked.
    BlockedPolicyLockedMapping,
    /// Provider is unreachable.
    BlockedProviderUnreachable,
    /// Authentication was lost.
    BlockedAuthLoss,
    /// Only offline capture is available.
    OfflineCaptureOnly,
}

impl RegistryHealthStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::DegradedStaleCredentials => "degraded_stale_credentials",
            Self::DegradedLimitedScopeSession => "degraded_limited_scope_session",
            Self::BlockedPolicyLockedMapping => "blocked_policy_locked_mapping",
            Self::BlockedProviderUnreachable => "blocked_provider_unreachable",
            Self::BlockedAuthLoss => "blocked_auth_loss",
            Self::OfflineCaptureOnly => "offline_capture_only",
        }
    }

    /// True when this health state blocks mutation authority.
    pub const fn blocks_mutation(self) -> bool {
        matches!(
            self,
            Self::BlockedPolicyLockedMapping
                | Self::BlockedProviderUnreachable
                | Self::BlockedAuthLoss
        )
    }
}

/// Action mode available on a provider-linked lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionModeClass {
    /// Lane supports read-only inspection.
    ReadOnly,
    /// Lane supports comment or link creation.
    CommentOrLink,
    /// Lane supports full edit mutation.
    FullEdit,
    /// Lane supports only offline capture.
    OfflineCaptureOnly,
    /// Lane supports publish-later queueing.
    PublishLater,
    /// Lane supports handoff-only mode.
    HandoffOnly,
}

impl ActionModeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::CommentOrLink => "comment_or_link",
            Self::FullEdit => "full_edit",
            Self::OfflineCaptureOnly => "offline_capture_only",
            Self::PublishLater => "publish_later",
            Self::HandoffOnly => "handoff_only",
        }
    }

    /// True when this action mode admits provider mutation.
    pub const fn admits_mutation(self) -> bool {
        matches!(self, Self::FullEdit)
    }
}

/// Stale-state class for a mapping-review row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingStaleStateClass {
    /// Mapping is current.
    Current,
    /// Mapping is stale but within a grace window.
    StaleWithinGrace,
    /// Mapping is stale and blocks mutation.
    StaleBlocksMutation,
    /// Mapping is blocked by policy.
    PolicyBlocked,
    /// Fallback mapping is active.
    FallbackActive,
}

impl MappingStaleStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleWithinGrace => "stale_within_grace",
            Self::StaleBlocksMutation => "stale_blocks_mutation",
            Self::PolicyBlocked => "policy_blocked",
            Self::FallbackActive => "fallback_active",
        }
    }
}

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a stable provider-account registry entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableProviderAccountInput {
    /// Stable entry identity.
    pub entry_id: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Canonical host ref.
    pub canonical_host_ref: String,
    /// Org or tenant scope ref.
    pub org_tenant_scope_ref: String,
    /// Acting-as identity class.
    pub acting_identity_class: ActingIdentityClass,
    /// Health state.
    pub health_state: RegistryHealthStateClass,
    /// Supported object types.
    pub supported_object_types: Vec<ProviderObjectKind>,
    /// Available action mode.
    pub action_mode: ActionModeClass,
    /// Connected-account row ref from the beta page.
    pub connected_account_row_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing a stable installation-grant registry entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableInstallGrantInput {
    /// Stable entry identity.
    pub entry_id: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Canonical host ref.
    pub canonical_host_ref: String,
    /// Org or tenant scope ref.
    pub org_tenant_scope_ref: String,
    /// Acting-as identity class.
    pub acting_identity_class: ActingIdentityClass,
    /// Health state.
    pub health_state: RegistryHealthStateClass,
    /// Supported object types.
    pub supported_object_types: Vec<ProviderObjectKind>,
    /// Available action mode.
    pub action_mode: ActionModeClass,
    /// Installation-grant row ref from the beta page.
    pub installation_grant_row_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing a stable mapping-review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableMappingReviewInput {
    /// Stable mapping row identity.
    pub mapping_row_id: String,
    /// Mapping lane class.
    pub lane: MappingLaneClass,
    /// Target kind.
    pub target_kind: TargetKindClass,
    /// Target ref.
    pub target_ref: String,
    /// Fallback target ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_target_ref: Option<String>,
    /// Stale state.
    pub stale_state: MappingStaleStateClass,
    /// Action mode available on this lane.
    pub action_mode: ActionModeClass,
    /// True when the mapping is policy-blocked.
    pub policy_blocked: bool,
    /// True when the mapping is stale.
    pub stale: bool,
    /// Mapping review row ref from the beta page.
    pub beta_mapping_review_row_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing a stable registry packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableRegistryInput {
    /// Stable registry identity.
    pub registry_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Provider account entries.
    pub provider_accounts: Vec<StableProviderAccountInput>,
    /// Installation grant entries.
    pub install_grants: Vec<StableInstallGrantInput>,
    /// Mapping review rows.
    pub mapping_rows: Vec<StableMappingReviewInput>,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing the support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableRegistrySupportExportInput {
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the registry.
    pub reopen_context_ref: String,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Stable provider-account registry entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableProviderAccountRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable entry identity.
    pub entry_id: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Stable token for provider family.
    pub provider_family_token: String,
    /// Canonical host ref.
    pub canonical_host_ref: String,
    /// Org or tenant scope ref.
    pub org_tenant_scope_ref: String,
    /// Acting-as identity class.
    pub acting_identity_class: ActingIdentityClass,
    /// Stable token for acting identity class.
    pub acting_identity_class_token: String,
    /// Health state.
    pub health_state: RegistryHealthStateClass,
    /// Stable token for health state.
    pub health_state_token: String,
    /// Supported object types.
    pub supported_object_types: Vec<ProviderObjectKind>,
    /// Available action mode.
    pub action_mode: ActionModeClass,
    /// Stable token for action mode.
    pub action_mode_token: String,
    /// Connected-account row ref from the beta page.
    pub connected_account_row_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Stable installation-grant registry entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableInstallGrantRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable entry identity.
    pub entry_id: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Stable token for provider family.
    pub provider_family_token: String,
    /// Canonical host ref.
    pub canonical_host_ref: String,
    /// Org or tenant scope ref.
    pub org_tenant_scope_ref: String,
    /// Acting-as identity class.
    pub acting_identity_class: ActingIdentityClass,
    /// Stable token for acting identity class.
    pub acting_identity_class_token: String,
    /// Health state.
    pub health_state: RegistryHealthStateClass,
    /// Stable token for health state.
    pub health_state_token: String,
    /// Supported object types.
    pub supported_object_types: Vec<ProviderObjectKind>,
    /// Available action mode.
    pub action_mode: ActionModeClass,
    /// Stable token for action mode.
    pub action_mode_token: String,
    /// Installation-grant row ref from the beta page.
    pub installation_grant_row_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Stable mapping-review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableMappingReviewRow {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable mapping row identity.
    pub mapping_row_id: String,
    /// Mapping lane class.
    pub lane: MappingLaneClass,
    /// Stable token for lane.
    pub lane_token: String,
    /// Target kind.
    pub target_kind: TargetKindClass,
    /// Stable token for target kind.
    pub target_kind_token: String,
    /// Target ref.
    pub target_ref: String,
    /// Fallback target ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_target_ref: Option<String>,
    /// Stale state.
    pub stale_state: MappingStaleStateClass,
    /// Stable token for stale state.
    pub stale_state_token: String,
    /// Action mode available on this lane.
    pub action_mode: ActionModeClass,
    /// Stable token for action mode.
    pub action_mode_token: String,
    /// True when the mapping is policy-blocked.
    pub policy_blocked: bool,
    /// True when the mapping is stale.
    pub stale: bool,
    /// Mapping review row ref from the beta page.
    pub beta_mapping_review_row_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Stable registry record binding the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableRegistryRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable registry identity.
    pub registry_id: String,
    /// Number of provider account entries.
    pub provider_account_count: usize,
    /// Number of installation grant entries.
    pub install_grant_count: usize,
    /// Number of mapping review rows.
    pub mapping_row_count: usize,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// True when the registry is actionable.
    pub actionable: bool,
    /// Timestamp the registry was frozen.
    pub generated_at: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Support/export packet for the stable registry lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableRegistrySupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the registry.
    pub reopen_context_ref: String,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the export cites.
    pub source_schema_refs: Vec<String>,
    /// False so raw URLs cannot cross the support boundary.
    pub raw_url_export_allowed: bool,
    /// False so raw provider payloads cannot cross the support boundary.
    pub raw_provider_payload_export_allowed: bool,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Inspection row used by support/export and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableRegistryInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Registry inspected by this row.
    pub registry_id_ref: String,
    /// True when at least one provider account is healthy.
    pub healthy_account_present: bool,
    /// True when at least one provider account is degraded.
    pub degraded_account_present: bool,
    /// True when at least one install grant is healthy.
    pub healthy_grant_present: bool,
    /// True when at least one install grant is degraded.
    pub degraded_grant_present: bool,
    /// True when at least one mapping is policy-blocked.
    pub policy_blocked_mapping_present: bool,
    /// True when at least one mapping is stale.
    pub stale_mapping_present: bool,
    /// True when at least one lane supports full edit.
    pub full_edit_lane_present: bool,
    /// True when at least one lane supports publish-later.
    pub publish_later_lane_present: bool,
    /// True when at least one lane supports offline-capture-only.
    pub offline_capture_only_lane_present: bool,
    /// True when at least one lane supports handoff-only.
    pub handoff_only_lane_present: bool,
    /// True when the registry is actionable.
    pub actionable: bool,
    /// True when the registry is invalidated by any reason.
    pub invalidated: bool,
    /// Number of provider account entries.
    pub provider_account_count: usize,
    /// Number of installation grant entries.
    pub install_grant_count: usize,
    /// Number of mapping review rows.
    pub mapping_row_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Stable provider-account/install-grant registry packet consumed by provider
/// surfaces and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableProviderAccountInstallGrantRegistryPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Stable registry record.
    pub registry: StableRegistryRecord,
    /// Provider account entries.
    pub provider_accounts: Vec<StableProviderAccountRecord>,
    /// Installation grant entries.
    pub install_grants: Vec<StableInstallGrantRecord>,
    /// Mapping review rows.
    pub mapping_rows: Vec<StableMappingReviewRow>,
    /// Support/export packet.
    pub support_export: StableRegistrySupportExportPacket,
    /// Inspection row.
    pub inspection: StableRegistryInspectionRecord,
}

impl StableProviderAccountInstallGrantRegistryPacket {
    /// Builds a stable registry packet from inputs.
    ///
    /// # Errors
    ///
    /// Returns [`StableRegistryValidationError`] when the input violates a
    /// registry invariant.
    pub fn from_input(
        input: StableRegistryInput,
        support_export_input: StableRegistrySupportExportInput,
    ) -> Result<Self, StableRegistryValidationError> {
        validate_input(&input)?;

        let registry = registry_record(&input);
        let provider_accounts = input
            .provider_accounts
            .iter()
            .map(|a| provider_account_record(a))
            .collect::<Vec<_>>();
        let install_grants = input
            .install_grants
            .iter()
            .map(|g| install_grant_record(g))
            .collect::<Vec<_>>();
        let mapping_rows = input
            .mapping_rows
            .iter()
            .map(|m| mapping_review_row_record(m))
            .collect::<Vec<_>>();
        let support_export = support_export_packet(&support_export_input, &registry);
        let inspection = inspection_record(
            &registry,
            &provider_accounts,
            &install_grants,
            &mapping_rows,
        );

        let packet = Self {
            record_kind: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_PACKET_RECORD_KIND
                .to_string(),
            schema_version: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
            shared_contract_ref: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SHARED_CONTRACT_REF
                .to_string(),
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            registry,
            provider_accounts,
            install_grants,
            mapping_rows,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable registry invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StableRegistryValidationError`] when an invariant is violated.
    pub fn validate(&self) -> Result<(), StableRegistryValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_registry_record(&self.registry)?;
        for account in &self.provider_accounts {
            validate_provider_account_record(account)?;
        }
        for grant in &self.install_grants {
            validate_install_grant_record(grant)?;
        }
        for row in &self.mapping_rows {
            validate_mapping_review_row(row)?;
        }
        validate_support_export(&self.support_export)?;
        validate_inspection(&self.inspection, self)?;

        // Cross-record invariants
        let account_ids: BTreeSet<&str> = self
            .provider_accounts
            .iter()
            .map(|a| a.entry_id.as_str())
            .collect();
        let grant_ids: BTreeSet<&str> = self
            .install_grants
            .iter()
            .map(|g| g.entry_id.as_str())
            .collect();
        let mapping_ids: BTreeSet<&str> = self
            .mapping_rows
            .iter()
            .map(|m| m.mapping_row_id.as_str())
            .collect();

        if account_ids.len() != self.provider_accounts.len() {
            return Err(stable_registry_validation_error(
                "provider account entry_ids must be unique".to_string(),
            ));
        }
        if grant_ids.len() != self.install_grants.len() {
            return Err(stable_registry_validation_error(
                "install grant entry_ids must be unique".to_string(),
            ));
        }
        if mapping_ids.len() != self.mapping_rows.len() {
            return Err(stable_registry_validation_error(
                "mapping row mapping_row_ids must be unique".to_string(),
            ));
        }

        // Health/action-mode coherence
        for account in &self.provider_accounts {
            if account.health_state.blocks_mutation() && account.action_mode.admits_mutation() {
                return Err(stable_registry_validation_error(format!(
                    "provider account {} blocks_mutation but action_mode admits_mutation",
                    account.entry_id
                )));
            }
        }
        for grant in &self.install_grants {
            if grant.health_state.blocks_mutation() && grant.action_mode.admits_mutation() {
                return Err(stable_registry_validation_error(format!(
                    "install grant {} blocks_mutation but action_mode admits_mutation",
                    grant.entry_id
                )));
            }
        }

        Ok(())
    }

    /// Projects the shared M5 secret-boundary state for registry auth.
    pub fn secret_boundary_states(&self) -> Vec<SecretBoundarySurfaceState> {
        let account = self.provider_accounts.first();
        let grant = self.install_grants.first();

        let (display_label, health_state, credential_mode, storage_class, delegated_use_class) =
            if let Some(row) = grant {
                (
                    row.summary_label.clone(),
                    registry_health_state(row.health_state),
                    SecretBoundaryCredentialMode::HandleOnly,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryDelegatedUseClass::RemoteVaultFetch,
                )
            } else if let Some(row) = account {
                (
                    row.summary_label.clone(),
                    registry_health_state(row.health_state),
                    SecretBoundaryCredentialMode::OsStore,
                    SecretBoundaryStorageClass::OsStore,
                    SecretBoundaryDelegatedUseClass::LocalSecretHandle,
                )
            } else {
                return Vec::new();
            };
        let actor_identity = if grant.is_some() {
            SecretBoundaryActingIdentityClass::ServiceIssuedAuthority
        } else {
            SecretBoundaryActingIdentityClass::HumanAccount
        };
        let consumer_identity = SecretBoundaryConsumerIdentityClass::RegistryClient;

        let decline_path = SecretBoundaryDeclinePath {
            decline_label: "Continue with local dependency review".to_owned(),
            still_works_summary:
                "Declining keeps lockfile review, offline handoff, and metadata-only registry inspection available."
                    .to_owned(),
        };
        let workflows = vec![
            registry_workflow("workflow:registry.install", "Authenticate install or restore"),
            registry_workflow("workflow:registry.publish", "Authenticate publish or queue"),
        ];
        let projection_controls =
            registry_projection_controls(REGISTRY_AUTH_MATRIX_ROW_ID, grant.is_some());
        let audit_result = secret_boundary_use_audit_result_for_health(health_state);

        vec![SecretBoundarySurfaceState {
            matrix_row_id: REGISTRY_AUTH_MATRIX_ROW_ID.to_owned(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            secret_access_prompt: SecretBoundarySecretAccessPrompt {
                matrix_row_id: REGISTRY_AUTH_MATRIX_ROW_ID.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                requester_label: "Registry auth".to_owned(),
                secret_class: SecretBoundarySecretClass::CodeHostOrRegistryToken,
                target_workflow_label: display_label.clone(),
                storage_class,
                credential_mode,
                projection_mode: SecretBoundaryProjectionMode::RequestHeader,
                lifetime_label: "Registry token or grant".to_owned(),
                expires_at: None,
                dependent_workflows: workflows.clone(),
                decline_path: decline_path.clone(),
            },
            credential_state_row: SecretBoundaryCredentialStateRow {
                matrix_row_id: REGISTRY_AUTH_MATRIX_ROW_ID.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                display_label: "Registry credential state".to_owned(),
                secret_class: SecretBoundarySecretClass::CodeHostOrRegistryToken,
                source_class: credential_mode,
                target_boundary_label: display_label.clone(),
                storage_class,
                projection_mode: SecretBoundaryProjectionMode::RequestHeader,
                health_state,
                expires_at: None,
                rotate_action_label: "Rotate registry token".to_owned(),
                revoke_action_label: "Revoke registry auth".to_owned(),
                test_action_label: "Test registry auth".to_owned(),
                dependent_workflows: workflows,
                decline_path,
            },
            vault_picker: Some(SecretBoundaryVaultPickerState {
                matrix_row_id: REGISTRY_AUTH_MATRIX_ROW_ID.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                picker_label: "Registry auth source picker".to_owned(),
                options: vec![
                    SecretBoundaryVaultPickerOption {
                        option_id: "registry-auth:os-store".to_owned(),
                        option_label: "OS credential store".to_owned(),
                        source_class: SecretBoundaryCredentialMode::OsStore,
                        storage_class: SecretBoundaryStorageClass::OsStore,
                        access_scope_label: "Registry host auth".to_owned(),
                        reveal_policy_label: "Handle only".to_owned(),
                        portability_note: "Portable exports omit raw values.".to_owned(),
                        open_source_of_truth_action_label: "Open keychain detail".to_owned(),
                        selectable: true,
                    },
                    SecretBoundaryVaultPickerOption {
                        option_id: "registry-auth:vault".to_owned(),
                        option_label: "Enterprise vault".to_owned(),
                        source_class: SecretBoundaryCredentialMode::EnterpriseVault,
                        storage_class: SecretBoundaryStorageClass::EnterpriseVault,
                        access_scope_label: "Registry host auth".to_owned(),
                        reveal_policy_label: "Vault ref only".to_owned(),
                        portability_note: "Exports preserve aliases and posture only.".to_owned(),
                        open_source_of_truth_action_label: "Open vault source".to_owned(),
                        selectable: true,
                    },
                ],
            }),
            delegated_credential_row: Some(SecretBoundaryDelegatedCredentialRow {
                matrix_row_id: REGISTRY_AUTH_MATRIX_ROW_ID.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                delegated_use_class,
                target_host_or_workspace_label: display_label,
                expires_at: None,
                policy_owner_label: "Registry or release operator".to_owned(),
                projection_controls: projection_controls.clone(),
            }),
            consumer_identity_receipt: SecretBoundaryConsumerIdentityReceipt::new(
                format!("{REGISTRY_AUTH_MATRIX_ROW_ID}:consumer-receipt"),
                REGISTRY_AUTH_MATRIX_ROW_ID,
                actor_identity,
                consumer_identity,
                "Registry or release operator",
                "Registry host auth",
                credential_mode,
                SecretBoundaryProjectionMode::RequestHeader,
                storage_class,
                audit_result,
            ),
            projection_mode_audit: SecretBoundaryProjectionModeAudit::new(
                format!("{REGISTRY_AUTH_MATRIX_ROW_ID}:projection-audit"),
                REGISTRY_AUTH_MATRIX_ROW_ID,
                actor_identity,
                consumer_identity,
                "Registry or release operator",
                "Registry host auth",
                SecretBoundaryProjectionMode::RequestHeader,
                audit_result,
                SecretBoundaryRepairOwnerClass::User,
                projection_controls
                    .iter()
                    .map(|control| control.control_class)
                    .collect(),
            ),
            profile_parity_rows: seeded_secret_boundary_profile_parity_rows(
                REGISTRY_AUTH_MATRIX_ROW_ID,
            ),
            export_safety_banner: SecretBoundaryExportSafetyBanner::standard(
                REGISTRY_AUTH_MATRIX_ROW_ID,
                "Raw registry tokens stay excluded from profiles, lockfile handoffs, support bundles, and publish evidence packets.",
            ),
        }]
    }

    /// Returns true when no raw escape hatch crosses the support boundary.
    pub fn raw_escape_hatches_absent(&self) -> bool {
        !self.support_export.raw_url_export_allowed
            && !self.support_export.raw_provider_payload_export_allowed
    }
}

fn registry_projection_controls(
    matrix_row_id: &str,
    grant_present: bool,
) -> Vec<SecretBoundaryProjectionControl> {
    let local_safe_note =
        "Lockfile review, offline handoff, and metadata-only registry inspection remain available.";
    let mut controls = vec![
        SecretBoundaryProjectionControl::new(
            matrix_row_id,
            SecretBoundaryProjectionControlClass::PauseForwarding,
            "Pause forwarded registry credential",
            local_safe_note,
        ),
        SecretBoundaryProjectionControl::new(
            matrix_row_id,
            SecretBoundaryProjectionControlClass::StopUsingSecret,
            "Stop registry auth reuse",
            local_safe_note,
        ),
    ];
    if grant_present {
        controls.push(SecretBoundaryProjectionControl::new(
            matrix_row_id,
            SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            "Drop delegated registry identity",
            local_safe_note,
        ));
    }
    controls
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable registry operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableRegistryError {
    /// Validation failed.
    Validation(StableRegistryValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StableRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StableRegistryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableRegistryValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StableRegistryValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StableRegistryValidationError {}

fn stable_registry_validation_error(message: impl Into<String>) -> StableRegistryValidationError {
    StableRegistryValidationError {
        message: message.into(),
    }
}

// ---------------------------------------------------------------------------
// Builder / validation helpers
// ---------------------------------------------------------------------------

fn validate_input(input: &StableRegistryInput) -> Result<(), StableRegistryValidationError> {
    ensure_nonempty(&input.registry_id, "registry_id")?;
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    for reason in &input.invalidation_reasons {
        ensure_token(
            STABLE_REGISTRY_INVALIDATION_REASONS,
            reason,
            "invalidation_reason",
        )?;
    }

    for account in &input.provider_accounts {
        ensure_nonempty(&account.entry_id, "provider_account.entry_id")?;
        ensure_nonempty(
            &account.canonical_host_ref,
            "provider_account.canonical_host_ref",
        )?;
        ensure_nonempty(
            &account.org_tenant_scope_ref,
            "provider_account.org_tenant_scope_ref",
        )?;
        ensure_nonempty(
            &account.connected_account_row_ref,
            "provider_account.connected_account_row_ref",
        )?;
    }

    for grant in &input.install_grants {
        ensure_nonempty(&grant.entry_id, "install_grant.entry_id")?;
        ensure_nonempty(
            &grant.canonical_host_ref,
            "install_grant.canonical_host_ref",
        )?;
        ensure_nonempty(
            &grant.org_tenant_scope_ref,
            "install_grant.org_tenant_scope_ref",
        )?;
        ensure_nonempty(
            &grant.installation_grant_row_ref,
            "install_grant.installation_grant_row_ref",
        )?;
    }

    for row in &input.mapping_rows {
        ensure_nonempty(&row.mapping_row_id, "mapping_row.mapping_row_id")?;
        ensure_nonempty(&row.target_ref, "mapping_row.target_ref")?;
        ensure_nonempty(
            &row.beta_mapping_review_row_ref,
            "mapping_row.beta_mapping_review_row_ref",
        )?;
    }

    Ok(())
}

fn validate_registry_record(
    record: &StableRegistryRecord,
) -> Result<(), StableRegistryValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        STABLE_REGISTRY_RECORD_KIND,
        "record.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
        "record.schema_version",
    )?;
    ensure_nonempty(&record.registry_id, "registry.registry_id")?;
    ensure_nonempty(&record.generated_at, "registry.generated_at")?;
    ensure_nonempty(&record.summary_label, "registry.summary_label")?;

    for reason in &record.invalidation_reasons {
        ensure_token(
            STABLE_REGISTRY_INVALIDATION_REASONS,
            reason,
            "registry.invalidation_reason",
        )?;
    }

    Ok(())
}

fn validate_provider_account_record(
    record: &StableProviderAccountRecord,
) -> Result<(), StableRegistryValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        STABLE_PROVIDER_ACCOUNT_RECORD_KIND,
        "account.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
        "account.schema_version",
    )?;
    ensure_nonempty(&record.entry_id, "account.entry_id")?;
    ensure_nonempty(&record.canonical_host_ref, "account.canonical_host_ref")?;
    ensure_nonempty(&record.org_tenant_scope_ref, "account.org_tenant_scope_ref")?;
    ensure_nonempty(
        &record.connected_account_row_ref,
        "account.connected_account_row_ref",
    )?;
    ensure_nonempty(&record.summary_label, "account.summary_label")?;

    ensure_eq(
        record.provider_family_token.as_str(),
        provider_family_token(record.provider_family),
        "account.provider_family_token",
    )?;
    ensure_eq(
        record.acting_identity_class_token.as_str(),
        record.acting_identity_class.as_str(),
        "account.acting_identity_class_token",
    )?;
    ensure_eq(
        record.health_state_token.as_str(),
        record.health_state.as_str(),
        "account.health_state_token",
    )?;
    ensure_eq(
        record.action_mode_token.as_str(),
        record.action_mode.as_str(),
        "account.action_mode_token",
    )?;

    Ok(())
}

fn validate_install_grant_record(
    record: &StableInstallGrantRecord,
) -> Result<(), StableRegistryValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        STABLE_INSTALL_GRANT_RECORD_KIND,
        "grant.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
        "grant.schema_version",
    )?;
    ensure_nonempty(&record.entry_id, "grant.entry_id")?;
    ensure_nonempty(&record.canonical_host_ref, "grant.canonical_host_ref")?;
    ensure_nonempty(&record.org_tenant_scope_ref, "grant.org_tenant_scope_ref")?;
    ensure_nonempty(
        &record.installation_grant_row_ref,
        "grant.installation_grant_row_ref",
    )?;
    ensure_nonempty(&record.summary_label, "grant.summary_label")?;

    ensure_eq(
        record.provider_family_token.as_str(),
        provider_family_token(record.provider_family),
        "grant.provider_family_token",
    )?;
    ensure_eq(
        record.acting_identity_class_token.as_str(),
        record.acting_identity_class.as_str(),
        "grant.acting_identity_class_token",
    )?;
    ensure_eq(
        record.health_state_token.as_str(),
        record.health_state.as_str(),
        "grant.health_state_token",
    )?;
    ensure_eq(
        record.action_mode_token.as_str(),
        record.action_mode.as_str(),
        "grant.action_mode_token",
    )?;

    Ok(())
}

fn validate_mapping_review_row(
    record: &StableMappingReviewRow,
) -> Result<(), StableRegistryValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        STABLE_MAPPING_REVIEW_ROW_RECORD_KIND,
        "mapping.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
        "mapping.schema_version",
    )?;
    ensure_nonempty(&record.mapping_row_id, "mapping.mapping_row_id")?;
    ensure_nonempty(&record.target_ref, "mapping.target_ref")?;
    ensure_nonempty(
        &record.beta_mapping_review_row_ref,
        "mapping.beta_mapping_review_row_ref",
    )?;
    ensure_nonempty(&record.summary_label, "mapping.summary_label")?;

    ensure_eq(
        record.lane_token.as_str(),
        record.lane.as_str(),
        "mapping.lane_token",
    )?;
    ensure_eq(
        record.target_kind_token.as_str(),
        record.target_kind.as_str(),
        "mapping.target_kind_token",
    )?;
    ensure_eq(
        record.stale_state_token.as_str(),
        record.stale_state.as_str(),
        "mapping.stale_state_token",
    )?;
    ensure_eq(
        record.action_mode_token.as_str(),
        record.action_mode.as_str(),
        "mapping.action_mode_token",
    )?;

    Ok(())
}

fn validate_support_export(
    packet: &StableRegistrySupportExportPacket,
) -> Result<(), StableRegistryValidationError> {
    ensure_eq(
        packet.record_kind.as_str(),
        STABLE_REGISTRY_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export.record_kind",
    )?;
    ensure_eq_u32(
        packet.schema_version,
        STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
        "support_export.schema_version",
    )?;
    ensure_nonempty(
        &packet.support_export_id,
        "support_export.support_export_id",
    )?;
    ensure_nonempty(
        &packet.reopen_context_ref,
        "support_export.reopen_context_ref",
    )?;
    ensure_nonempty(&packet.redaction_class, "support_export.redaction_class")?;
    ensure_nonempty(&packet.summary_label, "support_export.summary_label")?;

    Ok(())
}

fn validate_inspection(
    inspection: &StableRegistryInspectionRecord,
    packet: &StableProviderAccountInstallGrantRegistryPacket,
) -> Result<(), StableRegistryValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        STABLE_REGISTRY_INSPECTION_RECORD_KIND,
        "inspection.record_kind",
    )?;
    ensure_eq_u32(
        inspection.schema_version,
        STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
        "inspection.schema_version",
    )?;
    ensure_nonempty(&inspection.registry_id_ref, "inspection.registry_id_ref")?;
    ensure_nonempty(&inspection.summary_label, "inspection.summary_label")?;

    if inspection.provider_account_count != packet.provider_accounts.len() {
        return Err(stable_registry_validation_error(
            "inspection.provider_account_count mismatch".to_string(),
        ));
    }
    if inspection.install_grant_count != packet.install_grants.len() {
        return Err(stable_registry_validation_error(
            "inspection.install_grant_count mismatch".to_string(),
        ));
    }
    if inspection.mapping_row_count != packet.mapping_rows.len() {
        return Err(stable_registry_validation_error(
            "inspection.mapping_row_count mismatch".to_string(),
        ));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Record builders
// ---------------------------------------------------------------------------

fn registry_record(input: &StableRegistryInput) -> StableRegistryRecord {
    StableRegistryRecord {
        record_kind: STABLE_REGISTRY_RECORD_KIND.to_string(),
        schema_version: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
        shared_contract_ref: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SHARED_CONTRACT_REF
            .to_string(),
        registry_id: input.registry_id.clone(),
        provider_account_count: input.provider_accounts.len(),
        install_grant_count: input.install_grants.len(),
        mapping_row_count: input.mapping_rows.len(),
        invalidation_reasons: input.invalidation_reasons.clone(),
        actionable: input.invalidation_reasons.is_empty(),
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn provider_account_record(input: &StableProviderAccountInput) -> StableProviderAccountRecord {
    StableProviderAccountRecord {
        record_kind: STABLE_PROVIDER_ACCOUNT_RECORD_KIND.to_string(),
        schema_version: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
        shared_contract_ref: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SHARED_CONTRACT_REF
            .to_string(),
        entry_id: input.entry_id.clone(),
        provider_family: input.provider_family,
        provider_family_token: provider_family_token(input.provider_family).to_string(),
        canonical_host_ref: input.canonical_host_ref.clone(),
        org_tenant_scope_ref: input.org_tenant_scope_ref.clone(),
        acting_identity_class: input.acting_identity_class,
        acting_identity_class_token: input.acting_identity_class.as_str().to_string(),
        health_state: input.health_state,
        health_state_token: input.health_state.as_str().to_string(),
        supported_object_types: input.supported_object_types.clone(),
        action_mode: input.action_mode,
        action_mode_token: input.action_mode.as_str().to_string(),
        connected_account_row_ref: input.connected_account_row_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn install_grant_record(input: &StableInstallGrantInput) -> StableInstallGrantRecord {
    StableInstallGrantRecord {
        record_kind: STABLE_INSTALL_GRANT_RECORD_KIND.to_string(),
        schema_version: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
        shared_contract_ref: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SHARED_CONTRACT_REF
            .to_string(),
        entry_id: input.entry_id.clone(),
        provider_family: input.provider_family,
        provider_family_token: provider_family_token(input.provider_family).to_string(),
        canonical_host_ref: input.canonical_host_ref.clone(),
        org_tenant_scope_ref: input.org_tenant_scope_ref.clone(),
        acting_identity_class: input.acting_identity_class,
        acting_identity_class_token: input.acting_identity_class.as_str().to_string(),
        health_state: input.health_state,
        health_state_token: input.health_state.as_str().to_string(),
        supported_object_types: input.supported_object_types.clone(),
        action_mode: input.action_mode,
        action_mode_token: input.action_mode.as_str().to_string(),
        installation_grant_row_ref: input.installation_grant_row_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn mapping_review_row_record(input: &StableMappingReviewInput) -> StableMappingReviewRow {
    StableMappingReviewRow {
        record_kind: STABLE_MAPPING_REVIEW_ROW_RECORD_KIND.to_string(),
        schema_version: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
        shared_contract_ref: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SHARED_CONTRACT_REF
            .to_string(),
        mapping_row_id: input.mapping_row_id.clone(),
        lane: input.lane,
        lane_token: input.lane.as_str().to_string(),
        target_kind: input.target_kind,
        target_kind_token: input.target_kind.as_str().to_string(),
        target_ref: input.target_ref.clone(),
        fallback_target_ref: input.fallback_target_ref.clone(),
        stale_state: input.stale_state,
        stale_state_token: input.stale_state.as_str().to_string(),
        action_mode: input.action_mode,
        action_mode_token: input.action_mode.as_str().to_string(),
        policy_blocked: input.policy_blocked,
        stale: input.stale,
        beta_mapping_review_row_ref: input.beta_mapping_review_row_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn support_export_packet(
    input: &StableRegistrySupportExportInput,
    _registry: &StableRegistryRecord,
) -> StableRegistrySupportExportPacket {
    StableRegistrySupportExportPacket {
        record_kind: STABLE_REGISTRY_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
        shared_contract_ref: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SHARED_CONTRACT_REF
            .to_string(),
        support_export_id: input.support_export_id.clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/providers/stable_provider_account_install_grant_registry.schema.json"
                .to_string(),
        ],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn inspection_record(
    registry: &StableRegistryRecord,
    accounts: &[StableProviderAccountRecord],
    grants: &[StableInstallGrantRecord],
    mappings: &[StableMappingReviewRow],
) -> StableRegistryInspectionRecord {
    let healthy_account_present = accounts
        .iter()
        .any(|a| a.health_state == RegistryHealthStateClass::Healthy);
    let degraded_account_present = accounts
        .iter()
        .any(|a| a.health_state != RegistryHealthStateClass::Healthy);
    let healthy_grant_present = grants
        .iter()
        .any(|g| g.health_state == RegistryHealthStateClass::Healthy);
    let degraded_grant_present = grants
        .iter()
        .any(|g| g.health_state != RegistryHealthStateClass::Healthy);
    let policy_blocked_mapping_present = mappings.iter().any(|m| m.policy_blocked);
    let stale_mapping_present = mappings.iter().any(|m| m.stale);
    let full_edit_lane_present = mappings
        .iter()
        .any(|m| m.action_mode == ActionModeClass::FullEdit);
    let publish_later_lane_present = mappings
        .iter()
        .any(|m| m.action_mode == ActionModeClass::PublishLater);
    let offline_capture_only_lane_present = mappings
        .iter()
        .any(|m| m.action_mode == ActionModeClass::OfflineCaptureOnly);
    let handoff_only_lane_present = mappings
        .iter()
        .any(|m| m.action_mode == ActionModeClass::HandoffOnly);

    StableRegistryInspectionRecord {
        record_kind: STABLE_REGISTRY_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
        shared_contract_ref: STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SHARED_CONTRACT_REF
            .to_string(),
        registry_id_ref: registry.registry_id.clone(),
        healthy_account_present,
        degraded_account_present,
        healthy_grant_present,
        degraded_grant_present,
        policy_blocked_mapping_present,
        stale_mapping_present,
        full_edit_lane_present,
        publish_later_lane_present,
        offline_capture_only_lane_present,
        handoff_only_lane_present,
        actionable: registry.actionable,
        invalidated: !registry.invalidation_reasons.is_empty(),
        provider_account_count: accounts.len(),
        install_grant_count: grants.len(),
        mapping_row_count: mappings.len(),
        summary_label: format!(
            "Registry inspection: {} accounts, {} grants, {} mappings",
            accounts.len(),
            grants.len(),
            mappings.len()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded data
// ---------------------------------------------------------------------------

/// Builds a seeded stable provider-account/install-grant registry packet for
/// testing and fixture generation.
pub fn seeded_stable_provider_account_install_grant_registry_packet(
) -> StableProviderAccountInstallGrantRegistryPacket {
    let provider_accounts = vec![
        StableProviderAccountInput {
            entry_id: "stable-registry:account:github:human-dev".to_string(),
            provider_family: ProviderFamily::IssueTracker,
            canonical_host_ref: "provider-host:github:public".to_string(),
            org_tenant_scope_ref: "tenant:org:payments".to_string(),
            acting_identity_class: ActingIdentityClass::ConnectedAccount,
            health_state: RegistryHealthStateClass::Healthy,
            supported_object_types: vec![ProviderObjectKind::IssueOrWorkItem],
            action_mode: ActionModeClass::FullEdit,
            connected_account_row_ref: "account-scope-beta:connected-account:connected:human-dev"
                .to_string(),
            summary_label: "Healthy GitHub human account with full edit on issues".to_string(),
        },
        StableProviderAccountInput {
            entry_id: "stable-registry:account:github:human-reviewer".to_string(),
            provider_family: ProviderFamily::CodeHost,
            canonical_host_ref: "provider-host:github:enterprise-mirror".to_string(),
            org_tenant_scope_ref: "tenant:org:payments".to_string(),
            acting_identity_class: ActingIdentityClass::ConnectedAccount,
            health_state: RegistryHealthStateClass::DegradedStaleCredentials,
            supported_object_types: vec![ProviderObjectKind::PullRequest],
            action_mode: ActionModeClass::CommentOrLink,
            connected_account_row_ref:
                "account-scope-beta:connected-account:mirror_only:human-reviewer".to_string(),
            summary_label: "Degraded mirror account limited to comment/link".to_string(),
        },
    ];

    let install_grants = vec![
        StableInstallGrantInput {
            entry_id: "stable-registry:grant:github:app-install".to_string(),
            provider_family: ProviderFamily::IssueTracker,
            canonical_host_ref: "provider-host:github:public".to_string(),
            org_tenant_scope_ref: "tenant:org:payments".to_string(),
            acting_identity_class: ActingIdentityClass::InstallationGrant,
            health_state: RegistryHealthStateClass::Healthy,
            supported_object_types: vec![ProviderObjectKind::IssueOrWorkItem],
            action_mode: ActionModeClass::FullEdit,
            installation_grant_row_ref:
                "account-scope-beta:installation-grant:connected:app-payments".to_string(),
            summary_label: "Healthy GitHub app installation with full edit".to_string(),
        },
        StableInstallGrantInput {
            entry_id: "stable-registry:grant:gitlab:project-token".to_string(),
            provider_family: ProviderFamily::IssueTracker,
            canonical_host_ref: "provider-host:gitlab:enterprise".to_string(),
            org_tenant_scope_ref: "tenant:group:platform".to_string(),
            acting_identity_class: ActingIdentityClass::InstallationGrant,
            health_state: RegistryHealthStateClass::BlockedPolicyLockedMapping,
            supported_object_types: vec![ProviderObjectKind::IssueOrWorkItem],
            action_mode: ActionModeClass::ReadOnly,
            installation_grant_row_ref:
                "account-scope-beta:installation-grant:enterprise:project-token".to_string(),
            summary_label: "Policy-locked GitLab project token read-only".to_string(),
        },
    ];

    let mapping_rows = vec![
        StableMappingReviewInput {
            mapping_row_id: "stable-registry:mapping:github:issues".to_string(),
            lane: MappingLaneClass::IssueOrWorkItem,
            target_kind: TargetKindClass::Project,
            target_ref: "project:payments:backend".to_string(),
            fallback_target_ref: None,
            stale_state: MappingStaleStateClass::Current,
            action_mode: ActionModeClass::FullEdit,
            policy_blocked: false,
            stale: false,
            beta_mapping_review_row_ref: "target-mapping-beta:row:github:issues:payments-backend"
                .to_string(),
            summary_label: "Current project mapping with full edit".to_string(),
        },
        StableMappingReviewInput {
            mapping_row_id: "stable-registry:mapping:github:reviews".to_string(),
            lane: MappingLaneClass::ReviewDecision,
            target_kind: TargetKindClass::Repository,
            target_ref: "repo:payments:frontend".to_string(),
            fallback_target_ref: Some("repo:payments:frontend-fallback".to_string()),
            stale_state: MappingStaleStateClass::StaleWithinGrace,
            action_mode: ActionModeClass::CommentOrLink,
            policy_blocked: false,
            stale: true,
            beta_mapping_review_row_ref: "target-mapping-beta:row:github:reviews:payments-frontend"
                .to_string(),
            summary_label: "Stale review mapping with comment/link and fallback".to_string(),
        },
        StableMappingReviewInput {
            mapping_row_id: "stable-registry:mapping:github:offline".to_string(),
            lane: MappingLaneClass::PublishLater,
            target_kind: TargetKindClass::Board,
            target_ref: "board:payments:sprint-24".to_string(),
            fallback_target_ref: None,
            stale_state: MappingStaleStateClass::PolicyBlocked,
            action_mode: ActionModeClass::OfflineCaptureOnly,
            policy_blocked: true,
            stale: false,
            beta_mapping_review_row_ref: "target-mapping-beta:row:github:offline:sprint-24"
                .to_string(),
            summary_label: "Policy-blocked board mapping offline-capture-only".to_string(),
        },
    ];

    let input = StableRegistryInput {
        registry_id: "stable-registry:m4:001".to_string(),
        packet_id: "stable-registry-packet:m4:001".to_string(),
        generated_at: "2026-06-03T09:55:00Z".to_string(),
        provider_accounts,
        install_grants,
        mapping_rows,
        invalidation_reasons: vec![],
        summary_label: "Stable provider account/install-grant registry M4".to_string(),
    };

    let support_export_input = StableRegistrySupportExportInput {
        support_export_id: "stable-registry-support-export:m4:001".to_string(),
        reopen_context_ref: "stable-registry:m4:001".to_string(),
        consumer_surfaces: STABLE_REGISTRY_CONSUMER_SURFACES
            .iter()
            .map(|s| s.to_string())
            .collect(),
        redaction_class: "metadata_only".to_string(),
        summary_label: "Stable registry support export".to_string(),
    };

    StableProviderAccountInstallGrantRegistryPacket::from_input(input, support_export_input)
        .expect("seeded stable registry packet must be valid")
}

// ---------------------------------------------------------------------------
// Audit helpers
// ---------------------------------------------------------------------------

/// Validates a stable registry packet and returns typed defects on failure.
pub fn validate_stable_registry_packet(
    packet: &StableProviderAccountInstallGrantRegistryPacket,
) -> Result<(), Vec<StableRegistryValidationError>> {
    match packet.validate() {
        Ok(()) => Ok(()),
        Err(e) => Err(vec![e]),
    }
}

/// Recomputes defects for a stable registry packet.
pub fn audit_stable_registry_packet(
    packet: &StableProviderAccountInstallGrantRegistryPacket,
) -> Vec<StableRegistryValidationError> {
    match packet.validate() {
        Ok(()) => vec![],
        Err(e) => vec![e],
    }
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

fn ensure_eq(
    actual: &str,
    expected: &str,
    field: &str,
) -> Result<(), StableRegistryValidationError> {
    if actual != expected {
        Err(stable_registry_validation_error(format!(
            "{field} must be '{expected}', got '{actual}'"
        )))
    } else {
        Ok(())
    }
}

fn registry_workflow(
    workflow_ref: impl Into<String>,
    workflow_label: impl Into<String>,
) -> SecretBoundaryWorkflowDependency {
    SecretBoundaryWorkflowDependency {
        workflow_ref: workflow_ref.into(),
        workflow_label: workflow_label.into(),
    }
}

fn registry_health_state(health_state: RegistryHealthStateClass) -> SecretBoundaryHealthStateClass {
    match health_state {
        RegistryHealthStateClass::Healthy => SecretBoundaryHealthStateClass::Healthy,
        RegistryHealthStateClass::DegradedStaleCredentials => {
            SecretBoundaryHealthStateClass::ExpiringSoon
        }
        RegistryHealthStateClass::DegradedLimitedScopeSession => {
            SecretBoundaryHealthStateClass::PolicyBlocked
        }
        RegistryHealthStateClass::BlockedPolicyLockedMapping => {
            SecretBoundaryHealthStateClass::PolicyBlocked
        }
        RegistryHealthStateClass::BlockedProviderUnreachable => {
            SecretBoundaryHealthStateClass::RemoteVaultUnavailable
        }
        RegistryHealthStateClass::BlockedAuthLoss => SecretBoundaryHealthStateClass::Revoked,
        RegistryHealthStateClass::OfflineCaptureOnly => SecretBoundaryHealthStateClass::Missing,
    }
}

fn ensure_eq_u32(
    actual: u32,
    expected: u32,
    field: &str,
) -> Result<(), StableRegistryValidationError> {
    if actual != expected {
        Err(stable_registry_validation_error(format!(
            "{field} must be {expected}, got {actual}"
        )))
    } else {
        Ok(())
    }
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), StableRegistryValidationError> {
    if value.trim().is_empty() {
        Err(stable_registry_validation_error(format!(
            "{field} must be non-empty"
        )))
    } else {
        Ok(())
    }
}

fn ensure_token(
    vocabulary: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StableRegistryValidationError> {
    if !vocabulary.contains(&value) {
        Err(stable_registry_validation_error(format!(
            "{field} '{value}' is not in the closed vocabulary"
        )))
    } else {
        Ok(())
    }
}

fn provider_family_token(family: ProviderFamily) -> &'static str {
    match family {
        ProviderFamily::CodeHost => "code_host",
        ProviderFamily::IssueTracker => "issue_tracker",
        ProviderFamily::CiChecks => "ci_checks",
        // Note: additional families can be added here when the registry expands
    }
}
