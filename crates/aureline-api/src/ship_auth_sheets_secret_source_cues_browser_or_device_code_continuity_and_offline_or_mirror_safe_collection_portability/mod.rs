//! Auth sheets, secret-source cues, browser/device-code continuity, and offline
//! or mirror-safe collection-portability records.
//!
//! This module owns the typed records that make API auth configuration and
//! collection portability explicit and honest. Each auth sheet states the auth
//! scheme, the secret source mode, the token lifetime, the expiry label, the
//! browser/device-code continuity state, and the policy note, and never carries
//! or persists a raw secret. Each secret-source cue names where a credential
//! resolves from — secret broker, local encrypted store, managed rotation, or
//! policy lock — and its provenance, without exposing the value. Each
//! browser/device-code continuity row keeps an interrupted browser or
//! device-code flow resumable with a non-secret verification handle and a
//! user-action prompt. Each collection-portability row keeps export and import
//! contract-source, retention-mode, and redaction-posture state intact, labels
//! contract freshness honestly when a collection reopens offline or from a
//! mirror, and never widens secrets into the export.
//!
//! These records reuse the canonical frozen vocabulary
//! ([`ContractSourceClass`], [`ContractFreshnessState`], [`RetentionMode`],
//! [`OfflineMirrorBehavior`], [`RequestOriginKind`]) from the
//! [`freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix`](crate::freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix)
//! packet, the auth-source vocabulary ([`AuthSourceMode`],
//! [`AuthSourceProvenance`]) from the request-workspace lane, the secret-safe
//! storage and mirror/offline vocabulary ([`SecretSafeAuthStorageMode`],
//! [`MirrorOrOfflineStateClass`]) from the query-history lane, and the
//! [`ExportRedactionClass`] vocabulary from the composer redaction-safe export
//! lane, rather than minting a local synonym set.
//!
//! Raw secrets, raw tokens, raw credential bodies, raw cookies, and raw
//! certificate keys do not belong in these records. Auth sheets and cues carry
//! opaque, non-secret handle refs, closed posture vocabularies, and reviewable
//! summaries that UI, CLI, export, support, and public-proof surfaces can ingest
//! safely. Secrets are never written into versioned request files; exported or
//! imported collections never lose contract/source or redaction state; offline
//! and mirror-safe collections never masquerade stale or imported truth as live;
//! and browser-companion and managed origins never inherit desktop-local trust.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix::{
    ContractFreshnessState, ContractSourceClass, OfflineMirrorBehavior, RequestOriginKind,
    RetentionMode, API_MATRIX_QUALIFICATION_RECORD_KIND,
};
use crate::implement_the_request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export::ExportRedactionClass;
use crate::materialize_versioned_request_workspace_documents_environment_sets_and_auth_source_inspectors::{
    AuthSourceMode, AuthSourceProvenance, REQUEST_QUALIFICATION_RECORD_KIND,
};
use crate::ship_query_history_connection_profile_portability_secret_safe_auth_storage_and_mirror_or_offline_truth::{
    MirrorOrOfflineStateClass, SecretSafeAuthStorageMode,
    SHIP_QUERY_HISTORY_QUALIFICATION_RECORD_KIND,
};

/// Supported schema version for auth/portability qualification packets.
pub const AUTH_PORTABILITY_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`AuthPortabilityQualificationPacket`].
pub const AUTH_PORTABILITY_QUALIFICATION_RECORD_KIND: &str =
    "ship_auth_sheets_secret_source_cues_browser_or_device_code_continuity_and_offline_or_mirror_safe_collection_portability";

/// Repo-relative path to the checked-in auth/portability packet.
pub const AUTH_PORTABILITY_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/ship-auth-sheets-secret-source-cues-browser-or-device-code-continuity-and-offline-or-mirror-safe-collection-portability.json";

/// Embedded checked-in packet JSON.
pub const AUTH_PORTABILITY_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/ship-auth-sheets-secret-source-cues-browser-or-device-code-continuity-and-offline-or-mirror-safe-collection-portability.json"
));

/// Qualification label shown on promoted auth/portability surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthPortabilityQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl AuthPortabilityQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Auth/portability consumer surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthPortabilitySurfaceKind {
    /// Auth sheet inside a request workspace.
    AuthSheetPanel,
    /// Secret-source cue element shown beside the auth control.
    SecretSourceCue,
    /// Browser or device-code continuity flow surface.
    BrowserDeviceCodeFlow,
    /// Collection export/import portability surface.
    CollectionPortability,
    /// CLI or headless auth/portability output.
    CliHeadlessOutput,
    /// Support-export bundle carrying auth/portability truth.
    SupportExport,
    /// Help/About surface describing the auth/portability contract.
    HelpAbout,
}

/// Authentication scheme an auth sheet configures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthSchemeClass {
    /// No authentication is configured.
    NoAuth,
    /// HTTP Basic authentication.
    Basic,
    /// Bearer token authentication.
    Bearer,
    /// API-key authentication.
    ApiKey,
    /// OAuth 2.0 authorization-code (browser redirect) flow.
    OAuth2AuthorizationCode,
    /// OAuth 2.0 client-credentials (machine-to-machine) flow.
    OAuth2ClientCredentials,
    /// OAuth 2.0 device-code flow.
    OAuth2DeviceCode,
    /// Browser-session (companion cookie or interactive sign-in) auth.
    BrowserSession,
    /// mTLS or client-certificate authentication.
    Mtls,
}

impl AuthSchemeClass {
    /// Returns true when the scheme drives a browser-redirect or device-code
    /// flow whose continuity state must be tracked.
    pub const fn is_browser_or_device_flow(self) -> bool {
        matches!(
            self,
            Self::OAuth2AuthorizationCode | Self::OAuth2DeviceCode | Self::BrowserSession
        )
    }
}

/// Token lifetime / expiry posture shown on an auth sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenLifetimeClass {
    /// Credential does not expire (static secret or certificate).
    NoExpiry,
    /// Token is short-lived and expires soon.
    ShortLived,
    /// Token can be refreshed automatically before expiry.
    Refreshable,
    /// Token has already expired and must be re-acquired.
    Expired,
    /// Credential lives only for the duration of a session.
    SessionBound,
    /// Lifetime is unknown to the client.
    Unknown,
}

/// Browser or device-code continuity state for an auth flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserDeviceCodeState {
    /// The scheme does not use a browser or device-code flow.
    NotApplicable,
    /// Authorization has started and is in progress.
    Pending,
    /// Awaiting the user to complete a browser or device-code step.
    AwaitingUserAuthorization,
    /// The flow completed and a token was obtained.
    Authorized,
    /// The browser or device-code grant expired before completion.
    Expired,
    /// The user denied the authorization request.
    Denied,
}

impl BrowserDeviceCodeState {
    /// Returns true when the state is a real browser/device-code state rather
    /// than the not-applicable sentinel.
    pub const fn is_applicable(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }

    /// Returns true when the state is waiting on a user action and therefore
    /// must carry a user-action prompt.
    pub const fn is_pending_user_action(self) -> bool {
        matches!(self, Self::Pending | Self::AwaitingUserAuthorization)
    }
}

/// Direction of a collection-portability operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortabilityDirection {
    /// Exporting a collection to a portable artifact.
    Export,
    /// Importing or reopening a collection from a portable artifact.
    Import,
}

/// Returns true when a secret-source mode resolves a live credential and
/// therefore must be backed by a secret-source cue.
const fn auth_source_requires_cue(mode: AuthSourceMode) -> bool {
    matches!(
        mode,
        AuthSourceMode::SecretBrokerHandle
            | AuthSourceMode::DelegatedIdentity
            | AuthSourceMode::PolicyInjectedCredential
            | AuthSourceMode::ManagedServiceIdentity
            | AuthSourceMode::Mtls
    )
}

/// Returns true when a mirror/offline state is an offline or network-disabled
/// state rather than an online one.
const fn mirror_state_is_offline(state: MirrorOrOfflineStateClass) -> bool {
    matches!(
        state,
        MirrorOrOfflineStateClass::OfflineGraceWindow
            | MirrorOrOfflineStateClass::OfflineLocalOnly
            | MirrorOrOfflineStateClass::NetworkDisabled
    )
}

/// Returns true when a contract source is something other than a live contract.
const fn contract_source_is_not_live(source: ContractSourceClass) -> bool {
    matches!(
        source,
        ContractSourceClass::CachedSchema
            | ContractSourceClass::ImportedSnapshot
            | ContractSourceClass::ContractUnavailable
    )
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthPortabilityQualificationProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable auth/portability surfaces honest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthPortabilitySurfaceGuardSet {
    /// The auth scheme is visible.
    pub auth_scheme_visible: bool,
    /// The secret source mode and cue are visible.
    pub secret_source_visible: bool,
    /// The token lifetime / expiry is visible.
    pub expiry_visible: bool,
    /// The browser/device-code continuity state is visible.
    pub browser_device_code_visible: bool,
    /// Policy notes are visible.
    pub policy_note_visible: bool,
    /// No raw secret is persisted into request files or exports.
    pub no_raw_secret_persistence: bool,
    /// Contract source is preserved across export/import.
    pub contract_source_preserved: bool,
    /// Retention mode is preserved across export/import.
    pub retention_mode_preserved: bool,
    /// Redaction posture is preserved across export/import.
    pub redaction_posture_preserved: bool,
    /// Contract freshness is labeled honestly when a collection reopens.
    pub contract_freshness_labeled: bool,
    /// Offline or mirror-safe collections never masquerade as live.
    pub offline_mirror_honest: bool,
    /// Browser-companion and managed origins are isolated from desktop-local trust.
    pub origin_trust_isolated: bool,
    /// Request files stay text-first and versionable.
    pub text_first_versionable: bool,
}

impl AuthPortabilitySurfaceGuardSet {
    /// Returns true when every required guard is present.
    pub const fn all_visible(&self) -> bool {
        self.auth_scheme_visible
            && self.secret_source_visible
            && self.expiry_visible
            && self.browser_device_code_visible
            && self.policy_note_visible
            && self.no_raw_secret_persistence
            && self.contract_source_preserved
            && self.retention_mode_preserved
            && self.redaction_posture_preserved
            && self.contract_freshness_labeled
            && self.offline_mirror_honest
            && self.origin_trust_isolated
            && self.text_first_versionable
    }
}

/// One governed auth/portability consumer surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthPortabilitySurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: AuthPortabilitySurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: AuthPortabilityQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: AuthPortabilityQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<AuthPortabilityQualificationProof>,
    /// Visible guard set.
    pub guards: AuthPortabilitySurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One auth-sheet row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthSheetRow {
    /// Stable auth-sheet id.
    pub auth_sheet_id: String,
    /// Owning surface ref.
    pub surface_ref: String,
    /// Opaque request identity ref (not raw request text).
    pub request_identity_ref: String,
    /// Auth scheme configured on the sheet.
    pub auth_scheme: AuthSchemeClass,
    /// Origin scope the request resolves to.
    pub origin_scope: RequestOriginKind,
    /// Secret source mode (where the credential resolves from).
    pub secret_source_mode: AuthSourceMode,
    /// Ref to the secret-source cue, or empty when no live secret is used.
    pub secret_cue_ref: String,
    /// Token lifetime / expiry posture.
    pub token_lifetime: TokenLifetimeClass,
    /// Human-readable, non-secret expiry label.
    pub expiry_label: String,
    /// Browser/device-code continuity state.
    pub browser_device_code_state: BrowserDeviceCodeState,
    /// Plain-language policy note.
    pub policy_note: String,
    /// Whether the auth scheme is visible.
    pub auth_scheme_visible: bool,
    /// Whether the secret source mode and cue are visible.
    pub secret_source_visible: bool,
    /// Whether the token lifetime / expiry is visible.
    pub expiry_visible: bool,
    /// Whether the browser/device-code continuity state is visible.
    pub browser_device_code_visible: bool,
    /// Whether the policy note is visible.
    pub policy_note_visible: bool,
    /// Whether the sheet includes a raw secret value (must be false).
    pub includes_raw_secret: bool,
    /// Whether the sheet persists a secret into a versioned request file (must be false).
    pub persists_secret_in_request_file: bool,
    /// Whether managed/companion origin trust is isolated from desktop-local trust.
    pub local_trust_isolated: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

impl AuthSheetRow {
    /// Returns true when the sheet drives a browser-redirect or device-code flow.
    pub const fn is_browser_or_device_flow(&self) -> bool {
        self.auth_scheme.is_browser_or_device_flow()
    }

    /// Returns true when the sheet's secret source mode requires a backing cue.
    pub const fn requires_secret_cue(&self) -> bool {
        auth_source_requires_cue(self.secret_source_mode)
    }
}

/// One secret-source cue row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretSourceCueRow {
    /// Stable cue id.
    pub cue_id: String,
    /// Auth-sheet this cue backs.
    pub auth_sheet_ref: String,
    /// Secret-safe storage mode the value lives under.
    pub storage_mode: SecretSafeAuthStorageMode,
    /// Provenance of the credential reference.
    pub provenance: AuthSourceProvenance,
    /// Opaque, non-secret handle ref.
    pub handle_ref: String,
    /// Reviewer-facing cue label (where the secret resolves from).
    pub cue_label: String,
    /// Whether the cue is visible without exposing raw secret material.
    pub visible_without_secret: bool,
    /// Whether the cue includes a raw secret value (must be false).
    pub includes_raw_secret: bool,
    /// Whether the cue persists a secret into the repo (must be false).
    pub persists_secret_in_repo: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One browser/device-code continuity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BrowserDeviceCodeContinuityRow {
    /// Stable continuity id.
    pub continuity_id: String,
    /// Auth-sheet this continuity row tracks.
    pub auth_sheet_ref: String,
    /// Flow scheme (must be a browser or device-code flow).
    pub flow_scheme: AuthSchemeClass,
    /// Continuity state.
    pub state: BrowserDeviceCodeState,
    /// Whether an interrupted flow can be resumed.
    pub resumable: bool,
    /// Non-secret user-action prompt (empty unless a user action is pending).
    pub user_action_label: String,
    /// Opaque, non-secret verification handle ref.
    pub verification_handle_ref: String,
    /// Human-readable, non-secret expiry label.
    pub expiry_label: String,
    /// Origin scope the flow resolves to.
    pub origin_scope: RequestOriginKind,
    /// Whether managed/companion origin trust is isolated from desktop-local trust.
    pub local_trust_isolated: bool,
    /// Whether the row includes a raw token (must be false).
    pub includes_raw_token: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// One collection-portability row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollectionPortabilityRow {
    /// Stable portability id.
    pub portability_id: String,
    /// Owning surface ref.
    pub surface_ref: String,
    /// Opaque collection identity ref.
    pub collection_ref: String,
    /// Direction of the operation.
    pub direction: PortabilityDirection,
    /// Contract source preserved across the operation.
    pub contract_source: ContractSourceClass,
    /// Contract freshness state after the operation.
    pub contract_freshness: ContractFreshnessState,
    /// Retention mode preserved across the operation.
    pub retention_mode: RetentionMode,
    /// Export redaction class.
    pub export_redaction_class: ExportRedactionClass,
    /// Offline mirror behavior.
    pub offline_mirror_behavior: OfflineMirrorBehavior,
    /// Mirror or offline state.
    pub mirror_state: MirrorOrOfflineStateClass,
    /// Whether contract source is preserved (must be true).
    pub preserves_contract_source: bool,
    /// Whether retention mode is preserved (must be true).
    pub preserves_retention_mode: bool,
    /// Whether redaction posture is preserved (must be true).
    pub preserves_redaction_posture: bool,
    /// Whether contract freshness is labeled honestly (must be true).
    pub contract_freshness_labeled: bool,
    /// Whether the operation includes a raw secret value (must be false).
    pub includes_raw_secret: bool,
    /// Whether the operation persists a secret into the export (must be false).
    pub persists_secret_in_export: bool,
    /// Whether request definitions stay text-first and versionable (must be true).
    pub text_first: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

impl CollectionPortabilityRow {
    /// Returns true when the operation reopens or carries a collection from a
    /// non-live source or an offline/mirror state.
    pub const fn is_offline_or_imported(&self) -> bool {
        contract_source_is_not_live(self.contract_source)
            || mirror_state_is_offline(self.mirror_state)
    }

    /// Returns true when an offline or imported operation labels its freshness
    /// honestly and never claims a live contract.
    pub const fn reopens_honest_offline(&self) -> bool {
        self.is_offline_or_imported()
            && self.contract_freshness_labeled
            && !matches!(
                self.contract_freshness,
                ContractFreshnessState::LiveContract
            )
    }

    /// Returns true when the contract source and freshness state agree, so an
    /// imported or cached source never reads as a live contract and vice versa.
    pub const fn freshness_consistent(&self) -> bool {
        match self.contract_source {
            ContractSourceClass::LiveContract => {
                matches!(
                    self.contract_freshness,
                    ContractFreshnessState::LiveContract
                )
            }
            ContractSourceClass::ImportedSnapshot => {
                matches!(
                    self.contract_freshness,
                    ContractFreshnessState::ImportedSnapshot
                )
            }
            ContractSourceClass::ContractUnavailable => {
                matches!(
                    self.contract_freshness,
                    ContractFreshnessState::ContractUnavailable
                )
            }
            ContractSourceClass::CachedSchema => matches!(
                self.contract_freshness,
                ContractFreshnessState::CachedSchema | ContractFreshnessState::SchemaStale
            ),
            // A plugin-provided contract may be live, cached, stale, or an
            // imported snapshot, but never claims the no-contract state.
            ContractSourceClass::PluginProvided => !matches!(
                self.contract_freshness,
                ContractFreshnessState::ContractUnavailable
            ),
        }
    }
}

/// Reference to an upstream packet this lane consumes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthPortabilityUpstreamRefRow {
    /// Stable reference id.
    pub ref_id: String,
    /// Upstream record kind.
    pub upstream_record_kind: String,
    /// Repo-relative path to the upstream packet.
    pub upstream_packet_path: String,
    /// Repo-relative path to the upstream schema.
    pub upstream_schema_path: String,
    /// Whether integration has been verified.
    pub integration_verified: bool,
    /// Plain-language rationale.
    pub rationale: String,
}

/// Summary counts for an auth/portability qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthPortabilityQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of auth sheets.
    pub auth_sheet_count: usize,
    /// Number of auth sheets that drive a browser/device-code flow.
    pub device_code_auth_sheet_count: usize,
    /// Number of auth sheets whose origin must isolate desktop-local trust.
    pub trust_isolated_auth_sheet_count: usize,
    /// Number of secret-source cues.
    pub secret_source_cue_count: usize,
    /// Number of browser/device-code continuity rows.
    pub continuity_count: usize,
    /// Number of continuity rows that keep an interrupted flow resumable.
    pub resumable_continuity_count: usize,
    /// Number of collection-portability rows.
    pub collection_portability_count: usize,
    /// Number of portability rows that reopen honestly when offline or imported.
    pub offline_safe_portability_count: usize,
    /// Number of upstream reference rows.
    pub upstream_ref_count: usize,
    /// Number of upstream integrations that passed verification.
    pub integration_pass_count: usize,
}

/// Canonical auth/portability qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthPortabilityQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<AuthPortabilitySurfaceQualificationRow>,
    /// Auth-sheet rows.
    pub auth_sheets: Vec<AuthSheetRow>,
    /// Secret-source cue rows.
    pub secret_source_cues: Vec<SecretSourceCueRow>,
    /// Browser/device-code continuity rows.
    pub continuities: Vec<BrowserDeviceCodeContinuityRow>,
    /// Collection-portability rows.
    pub collection_portabilities: Vec<CollectionPortabilityRow>,
    /// Upstream reference rows.
    pub upstream_refs: Vec<AuthPortabilityUpstreamRefRow>,
    /// Summary counts.
    pub summary: AuthPortabilityQualificationSummary,
}

impl AuthPortabilityQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> AuthPortabilityQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        let device_code_auth_sheet_count = self
            .auth_sheets
            .iter()
            .filter(|sheet| sheet.is_browser_or_device_flow())
            .count();
        let trust_isolated_auth_sheet_count = self
            .auth_sheets
            .iter()
            .filter(|sheet| sheet.origin_scope.must_isolate_local_trust())
            .count();
        let resumable_continuity_count =
            self.continuities.iter().filter(|row| row.resumable).count();
        let offline_safe_portability_count = self
            .collection_portabilities
            .iter()
            .filter(|row| row.reopens_honest_offline())
            .count();
        let integration_pass_count = self
            .upstream_refs
            .iter()
            .filter(|ref_row| ref_row.integration_verified)
            .count();
        AuthPortabilityQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            auth_sheet_count: self.auth_sheets.len(),
            device_code_auth_sheet_count,
            trust_isolated_auth_sheet_count,
            secret_source_cue_count: self.secret_source_cues.len(),
            continuity_count: self.continuities.len(),
            resumable_continuity_count,
            collection_portability_count: self.collection_portabilities.len(),
            offline_safe_portability_count,
            upstream_ref_count: self.upstream_refs.len(),
            integration_pass_count,
        }
    }

    /// Returns the ids of auth sheets that drive a browser/device-code flow.
    pub fn device_code_auth_sheet_ids(&self) -> Vec<String> {
        self.auth_sheets
            .iter()
            .filter(|sheet| sheet.is_browser_or_device_flow())
            .map(|sheet| sheet.auth_sheet_id.clone())
            .collect()
    }

    /// Returns the ids of auth sheets whose origin must isolate desktop-local
    /// trust (managed-workspace and browser-companion origins).
    pub fn trust_isolated_auth_sheet_ids(&self) -> Vec<String> {
        self.auth_sheets
            .iter()
            .filter(|sheet| sheet.origin_scope.must_isolate_local_trust())
            .map(|sheet| sheet.auth_sheet_id.clone())
            .collect()
    }

    /// Returns the ids of continuity rows that keep an interrupted flow resumable.
    pub fn resumable_continuity_ids(&self) -> Vec<String> {
        self.continuities
            .iter()
            .filter(|row| row.resumable)
            .map(|row| row.continuity_id.clone())
            .collect()
    }

    /// Returns the ids of portability rows that reopen honestly when offline or
    /// imported, with freshness labeled and no live-contract masquerade.
    pub fn offline_safe_portability_ids(&self) -> Vec<String> {
        self.collection_portabilities
            .iter()
            .filter(|row| row.reopens_honest_offline())
            .map(|row| row.portability_id.clone())
            .collect()
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<AuthPortabilityQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != AUTH_PORTABILITY_QUALIFICATION_SCHEMA_VERSION {
            violations.push(AuthPortabilityQualificationViolation::SchemaVersion {
                expected: AUTH_PORTABILITY_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != AUTH_PORTABILITY_QUALIFICATION_RECORD_KIND {
            violations.push(AuthPortabilityQualificationViolation::RecordKind {
                expected: AUTH_PORTABILITY_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let surface_ids = collect_ids(
            self.surfaces.iter().map(|row| row.surface_id.as_str()),
            &mut violations,
            AuthPortabilityQualificationViolationKind::Surface,
        );
        let auth_sheet_ids = collect_ids(
            self.auth_sheets
                .iter()
                .map(|row| row.auth_sheet_id.as_str()),
            &mut violations,
            AuthPortabilityQualificationViolationKind::AuthSheet,
        );
        let cue_ids = collect_ids(
            self.secret_source_cues
                .iter()
                .map(|row| row.cue_id.as_str()),
            &mut violations,
            AuthPortabilityQualificationViolationKind::SecretCue,
        );
        collect_ids(
            self.continuities
                .iter()
                .map(|row| row.continuity_id.as_str()),
            &mut violations,
            AuthPortabilityQualificationViolationKind::Continuity,
        );
        collect_ids(
            self.collection_portabilities
                .iter()
                .map(|row| row.portability_id.as_str()),
            &mut violations,
            AuthPortabilityQualificationViolationKind::Portability,
        );
        collect_ids(
            self.upstream_refs.iter().map(|row| row.ref_id.as_str()),
            &mut violations,
            AuthPortabilityQualificationViolationKind::UpstreamRef,
        );

        self.validate_surfaces(&mut violations);
        self.validate_auth_sheets(&mut violations, &surface_ids, &cue_ids);
        self.validate_secret_cues(&mut violations, &auth_sheet_ids);
        self.validate_continuities(&mut violations, &auth_sheet_ids);
        self.validate_portabilities(&mut violations, &surface_ids);
        self.validate_upstream_refs(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(AuthPortabilityQualificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_surfaces(&self, violations: &mut Vec<AuthPortabilityQualificationViolation>) {
        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        AuthPortabilityQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        AuthPortabilityQualificationViolation::StableSurfaceMissingGuard {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    AuthPortabilityQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let surface_kinds: BTreeSet<_> = self.surfaces.iter().map(|row| row.surface_kind).collect();
        for required_kind in [
            AuthPortabilitySurfaceKind::AuthSheetPanel,
            AuthPortabilitySurfaceKind::SecretSourceCue,
            AuthPortabilitySurfaceKind::BrowserDeviceCodeFlow,
            AuthPortabilitySurfaceKind::CollectionPortability,
            AuthPortabilitySurfaceKind::CliHeadlessOutput,
            AuthPortabilitySurfaceKind::SupportExport,
            AuthPortabilitySurfaceKind::HelpAbout,
        ] {
            if !surface_kinds.contains(&required_kind) {
                violations.push(AuthPortabilityQualificationViolation::MissingSurfaceKind {
                    surface_kind: required_kind,
                });
            }
        }
    }

    fn validate_auth_sheets(
        &self,
        violations: &mut Vec<AuthPortabilityQualificationViolation>,
        surface_ids: &BTreeSet<String>,
        cue_ids: &BTreeSet<String>,
    ) {
        for row in &self.auth_sheets {
            // Every auth sheet projects its identity columns: scheme, secret
            // source, expiry, browser/device-code state, and policy note are
            // never hidden.
            if !surface_ids.contains(&row.surface_ref)
                || row.request_identity_ref.is_empty()
                || row.expiry_label.is_empty()
                || row.policy_note.is_empty()
                || row.rationale.is_empty()
                || !row.auth_scheme_visible
                || !row.secret_source_visible
                || !row.expiry_visible
                || !row.browser_device_code_visible
                || !row.policy_note_visible
            {
                violations.push(AuthPortabilityQualificationViolation::IncompleteAuthSheet {
                    auth_sheet_id: row.auth_sheet_id.clone(),
                });
            }

            // A sheet never carries or persists a raw secret.
            if row.includes_raw_secret || row.persists_secret_in_request_file {
                violations.push(
                    AuthPortabilityQualificationViolation::AuthSheetLeaksSecret {
                        auth_sheet_id: row.auth_sheet_id.clone(),
                    },
                );
            }

            // Browser/device-code schemes carry a real continuity state; other
            // schemes carry the not-applicable sentinel.
            if row.is_browser_or_device_flow() != row.browser_device_code_state.is_applicable() {
                violations.push(
                    AuthPortabilityQualificationViolation::BrowserFlowStateMismatch {
                        auth_sheet_id: row.auth_sheet_id.clone(),
                    },
                );
            }

            // A live-secret source mode must be backed by an existing cue; a
            // no-live-secret mode carries no cue ref.
            let cue_ok = if row.requires_secret_cue() {
                !row.secret_cue_ref.is_empty() && cue_ids.contains(&row.secret_cue_ref)
            } else {
                row.secret_cue_ref.is_empty()
            };
            if !cue_ok {
                violations.push(
                    AuthPortabilityQualificationViolation::AuthSheetCueMismatch {
                        auth_sheet_id: row.auth_sheet_id.clone(),
                    },
                );
            }

            // Managed-workspace and browser-companion origins must never inherit
            // desktop-local trust.
            if row.origin_scope.must_isolate_local_trust() && !row.local_trust_isolated {
                violations.push(
                    AuthPortabilityQualificationViolation::AuthSheetOriginTrustNotIsolated {
                        auth_sheet_id: row.auth_sheet_id.clone(),
                    },
                );
            }
        }

        // Coverage: every scheme, token lifetime, and secret source mode must be
        // exercised so the auth lane is proven, not asserted.
        let schemes: BTreeSet<_> = self.auth_sheets.iter().map(|row| row.auth_scheme).collect();
        for required in [
            AuthSchemeClass::NoAuth,
            AuthSchemeClass::Basic,
            AuthSchemeClass::Bearer,
            AuthSchemeClass::ApiKey,
            AuthSchemeClass::OAuth2AuthorizationCode,
            AuthSchemeClass::OAuth2ClientCredentials,
            AuthSchemeClass::OAuth2DeviceCode,
            AuthSchemeClass::BrowserSession,
            AuthSchemeClass::Mtls,
        ] {
            if !schemes.contains(&required) {
                violations.push(AuthPortabilityQualificationViolation::MissingAuthScheme {
                    auth_scheme: required,
                });
            }
        }

        let lifetimes: BTreeSet<_> = self
            .auth_sheets
            .iter()
            .map(|row| row.token_lifetime)
            .collect();
        for required in [
            TokenLifetimeClass::NoExpiry,
            TokenLifetimeClass::ShortLived,
            TokenLifetimeClass::Refreshable,
            TokenLifetimeClass::Expired,
            TokenLifetimeClass::SessionBound,
            TokenLifetimeClass::Unknown,
        ] {
            if !lifetimes.contains(&required) {
                violations.push(
                    AuthPortabilityQualificationViolation::MissingTokenLifetime {
                        token_lifetime: required,
                    },
                );
            }
        }

        let modes: BTreeSet<_> = self
            .auth_sheets
            .iter()
            .map(|row| row.secret_source_mode)
            .collect();
        for required in [
            AuthSourceMode::NoAuth,
            AuthSourceMode::SecretBrokerHandle,
            AuthSourceMode::DelegatedIdentity,
            AuthSourceMode::PolicyInjectedCredential,
            AuthSourceMode::ManagedServiceIdentity,
            AuthSourceMode::Mtls,
            AuthSourceMode::ImportedNoLiveAuth,
            AuthSourceMode::PolicyBlocked,
        ] {
            if !modes.contains(&required) {
                violations.push(
                    AuthPortabilityQualificationViolation::MissingAuthSourceMode {
                        auth_source_mode: required,
                    },
                );
            }
        }

        // Managed and companion origins are claimed surfaces; at least one must
        // be exercised so trust isolation is proven, not asserted.
        if !self
            .auth_sheets
            .iter()
            .any(|row| row.origin_scope.must_isolate_local_trust())
        {
            violations.push(AuthPortabilityQualificationViolation::NoTrustIsolatedAuthSheetCovered);
        }
    }

    fn validate_secret_cues(
        &self,
        violations: &mut Vec<AuthPortabilityQualificationViolation>,
        auth_sheet_ids: &BTreeSet<String>,
    ) {
        for row in &self.secret_source_cues {
            if !auth_sheet_ids.contains(&row.auth_sheet_ref)
                || row.handle_ref.is_empty()
                || row.cue_label.is_empty()
                || row.rationale.is_empty()
                || !row.visible_without_secret
            {
                violations.push(AuthPortabilityQualificationViolation::IncompleteSecretCue {
                    cue_id: row.cue_id.clone(),
                });
            }

            // A cue never carries or persists a raw secret.
            if row.includes_raw_secret || row.persists_secret_in_repo {
                violations.push(AuthPortabilityQualificationViolation::SecretCueLeaks {
                    cue_id: row.cue_id.clone(),
                });
            }
        }

        // Coverage: every storage mode and provenance must be exercised.
        let storage_modes: BTreeSet<_> = self
            .secret_source_cues
            .iter()
            .map(|row| row.storage_mode)
            .collect();
        for required in [
            SecretSafeAuthStorageMode::LocalEncrypted,
            SecretSafeAuthStorageMode::SecretBrokerOnly,
            SecretSafeAuthStorageMode::ManagedRotation,
            SecretSafeAuthStorageMode::PolicyLocked,
        ] {
            if !storage_modes.contains(&required) {
                violations.push(AuthPortabilityQualificationViolation::MissingStorageMode {
                    storage_mode: required,
                });
            }
        }

        let provenances: BTreeSet<_> = self
            .secret_source_cues
            .iter()
            .map(|row| row.provenance)
            .collect();
        for required in [
            AuthSourceProvenance::RequestFile,
            AuthSourceProvenance::WorkspaceDefault,
            AuthSourceProvenance::PolicyInjection,
            AuthSourceProvenance::AdHocOverride,
            AuthSourceProvenance::SecretBroker,
        ] {
            if !provenances.contains(&required) {
                violations.push(AuthPortabilityQualificationViolation::MissingProvenance {
                    provenance: required,
                });
            }
        }
    }

    fn validate_continuities(
        &self,
        violations: &mut Vec<AuthPortabilityQualificationViolation>,
        auth_sheet_ids: &BTreeSet<String>,
    ) {
        for row in &self.continuities {
            if !auth_sheet_ids.contains(&row.auth_sheet_ref)
                || row.verification_handle_ref.is_empty()
                || row.expiry_label.is_empty()
                || row.rationale.is_empty()
            {
                violations.push(
                    AuthPortabilityQualificationViolation::IncompleteContinuity {
                        continuity_id: row.continuity_id.clone(),
                    },
                );
            }

            // A continuity row only tracks browser-redirect or device-code flows.
            if !row.flow_scheme.is_browser_or_device_flow() {
                violations.push(
                    AuthPortabilityQualificationViolation::NonBrowserContinuity {
                        continuity_id: row.continuity_id.clone(),
                    },
                );
            }

            // A continuity row carries a real continuity state.
            if !row.state.is_applicable() {
                violations.push(
                    AuthPortabilityQualificationViolation::ContinuityStateNotApplicable {
                        continuity_id: row.continuity_id.clone(),
                    },
                );
            }

            // A pending-user-action state must surface a user-action prompt.
            if row.state.is_pending_user_action() && row.user_action_label.is_empty() {
                violations.push(
                    AuthPortabilityQualificationViolation::ContinuityMissingUserAction {
                        continuity_id: row.continuity_id.clone(),
                    },
                );
            }

            // A continuity row never carries a raw token.
            if row.includes_raw_token {
                violations.push(
                    AuthPortabilityQualificationViolation::ContinuityLeaksToken {
                        continuity_id: row.continuity_id.clone(),
                    },
                );
            }

            // Managed-workspace and browser-companion origins must never inherit
            // desktop-local trust.
            if row.origin_scope.must_isolate_local_trust() && !row.local_trust_isolated {
                violations.push(
                    AuthPortabilityQualificationViolation::ContinuityOriginTrustNotIsolated {
                        continuity_id: row.continuity_id.clone(),
                    },
                );
            }
        }

        // Coverage: every applicable continuity state and every flow scheme must
        // be exercised, and at least one interrupted flow must stay resumable.
        let states: BTreeSet<_> = self.continuities.iter().map(|row| row.state).collect();
        for required in [
            BrowserDeviceCodeState::Pending,
            BrowserDeviceCodeState::AwaitingUserAuthorization,
            BrowserDeviceCodeState::Authorized,
            BrowserDeviceCodeState::Expired,
            BrowserDeviceCodeState::Denied,
        ] {
            if !states.contains(&required) {
                violations.push(
                    AuthPortabilityQualificationViolation::MissingContinuityState {
                        state: required,
                    },
                );
            }
        }

        let flow_schemes: BTreeSet<_> = self
            .continuities
            .iter()
            .map(|row| row.flow_scheme)
            .collect();
        for required in [
            AuthSchemeClass::OAuth2AuthorizationCode,
            AuthSchemeClass::OAuth2DeviceCode,
            AuthSchemeClass::BrowserSession,
        ] {
            if !flow_schemes.contains(&required) {
                violations.push(
                    AuthPortabilityQualificationViolation::MissingContinuityFlowScheme {
                        flow_scheme: required,
                    },
                );
            }
        }

        if !self.continuities.iter().any(|row| row.resumable) {
            violations.push(AuthPortabilityQualificationViolation::NoResumableContinuityCovered);
        }
    }

    fn validate_portabilities(
        &self,
        violations: &mut Vec<AuthPortabilityQualificationViolation>,
        surface_ids: &BTreeSet<String>,
    ) {
        for row in &self.collection_portabilities {
            if !surface_ids.contains(&row.surface_ref)
                || row.collection_ref.is_empty()
                || row.rationale.is_empty()
            {
                violations.push(
                    AuthPortabilityQualificationViolation::IncompletePortability {
                        portability_id: row.portability_id.clone(),
                    },
                );
            }

            // Export/import never drops contract source, retention mode, or
            // redaction posture.
            if !row.preserves_contract_source
                || !row.preserves_retention_mode
                || !row.preserves_redaction_posture
            {
                violations.push(
                    AuthPortabilityQualificationViolation::PortabilityDropsState {
                        portability_id: row.portability_id.clone(),
                    },
                );
            }

            // Export/import never carries or persists a raw secret.
            if row.includes_raw_secret || row.persists_secret_in_export {
                violations.push(
                    AuthPortabilityQualificationViolation::PortabilityLeaksSecret {
                        portability_id: row.portability_id.clone(),
                    },
                );
            }

            // Contract freshness is always labeled honestly.
            if !row.contract_freshness_labeled {
                violations.push(
                    AuthPortabilityQualificationViolation::PortabilityHidesFreshness {
                        portability_id: row.portability_id.clone(),
                    },
                );
            }

            // An offline or imported collection never masquerades as a live
            // contract.
            if row.is_offline_or_imported()
                && matches!(row.contract_freshness, ContractFreshnessState::LiveContract)
            {
                violations.push(
                    AuthPortabilityQualificationViolation::PortabilityMasqueradesLive {
                        portability_id: row.portability_id.clone(),
                    },
                );
            }

            // Contract source and freshness state agree.
            if !row.freshness_consistent() {
                violations.push(
                    AuthPortabilityQualificationViolation::PortabilityFreshnessInconsistent {
                        portability_id: row.portability_id.clone(),
                    },
                );
            }

            // Request definitions stay text-first and versionable.
            if !row.text_first {
                violations.push(
                    AuthPortabilityQualificationViolation::PortabilityNotTextFirst {
                        portability_id: row.portability_id.clone(),
                    },
                );
            }
        }

        // Coverage: both directions and every reused source, retention, export,
        // and mirror vocabulary value must be exercised.
        let directions: BTreeSet<_> = self
            .collection_portabilities
            .iter()
            .map(|row| row.direction)
            .collect();
        for required in [PortabilityDirection::Export, PortabilityDirection::Import] {
            if !directions.contains(&required) {
                violations.push(
                    AuthPortabilityQualificationViolation::MissingPortabilityDirection {
                        direction: required,
                    },
                );
            }
        }

        let sources: BTreeSet<_> = self
            .collection_portabilities
            .iter()
            .map(|row| row.contract_source)
            .collect();
        for required in [
            ContractSourceClass::LiveContract,
            ContractSourceClass::CachedSchema,
            ContractSourceClass::ImportedSnapshot,
            ContractSourceClass::PluginProvided,
            ContractSourceClass::ContractUnavailable,
        ] {
            if !sources.contains(&required) {
                violations.push(
                    AuthPortabilityQualificationViolation::MissingContractSource {
                        source_class: required,
                    },
                );
            }
        }

        let retentions: BTreeSet<_> = self
            .collection_portabilities
            .iter()
            .map(|row| row.retention_mode)
            .collect();
        for required in [
            RetentionMode::TextFirstVersioned,
            RetentionMode::MetadataOnly,
            RetentionMode::RedactedReplayable,
            RetentionMode::OptInFullCapture,
        ] {
            if !retentions.contains(&required) {
                violations.push(
                    AuthPortabilityQualificationViolation::MissingRetentionMode {
                        retention_mode: required,
                    },
                );
            }
        }

        let export_classes: BTreeSet<_> = self
            .collection_portabilities
            .iter()
            .map(|row| row.export_redaction_class)
            .collect();
        for required in [
            ExportRedactionClass::FullRedaction,
            ExportRedactionClass::MetadataOnly,
            ExportRedactionClass::SafePreview,
            ExportRedactionClass::UnredactedLocalOnly,
        ] {
            if !export_classes.contains(&required) {
                violations.push(AuthPortabilityQualificationViolation::MissingExportClass {
                    export_class: required,
                });
            }
        }

        let mirror_behaviors: BTreeSet<_> = self
            .collection_portabilities
            .iter()
            .map(|row| row.offline_mirror_behavior)
            .collect();
        for required in [
            OfflineMirrorBehavior::MirrorMaintained,
            OfflineMirrorBehavior::OfflineDegraded,
            OfflineMirrorBehavior::NoMirror,
        ] {
            if !mirror_behaviors.contains(&required) {
                violations.push(
                    AuthPortabilityQualificationViolation::MissingMirrorBehavior {
                        mirror_behavior: required,
                    },
                );
            }
        }

        let mirror_states: BTreeSet<_> = self
            .collection_portabilities
            .iter()
            .map(|row| row.mirror_state)
            .collect();
        for required in [
            MirrorOrOfflineStateClass::OnlineDefault,
            MirrorOrOfflineStateClass::OnlineReplica,
            MirrorOrOfflineStateClass::OfflineGraceWindow,
            MirrorOrOfflineStateClass::OfflineLocalOnly,
            MirrorOrOfflineStateClass::NetworkDisabled,
        ] {
            if !mirror_states.contains(&required) {
                violations.push(AuthPortabilityQualificationViolation::MissingMirrorState {
                    mirror_state: required,
                });
            }
        }

        // At least one offline or imported reopen must stay honest so the
        // mirror-safe path is proven, not asserted.
        if !self
            .collection_portabilities
            .iter()
            .any(|row| row.reopens_honest_offline())
        {
            violations.push(AuthPortabilityQualificationViolation::NoOfflineSafeReopenCovered);
        }
    }

    fn validate_upstream_refs(&self, violations: &mut Vec<AuthPortabilityQualificationViolation>) {
        for row in &self.upstream_refs {
            if row.upstream_record_kind.is_empty()
                || row.upstream_packet_path.is_empty()
                || row.upstream_schema_path.is_empty()
                || row.rationale.is_empty()
            {
                violations.push(
                    AuthPortabilityQualificationViolation::IncompleteUpstreamRef {
                        ref_id: row.ref_id.clone(),
                    },
                );
            }
        }

        // The auth/portability lane must consume the frozen API-collection matrix,
        // the request-workspace auth-source lane, and the query-history
        // secret-safe storage lane as verified upstream packets so its origin,
        // retention, contract, auth-source, and secret-storage vocabularies stay
        // aligned.
        let consumes = |record_kind: &str| -> bool {
            self.upstream_refs
                .iter()
                .any(|row| row.upstream_record_kind == record_kind && row.integration_verified)
        };
        if !consumes(API_MATRIX_QUALIFICATION_RECORD_KIND) {
            violations.push(AuthPortabilityQualificationViolation::MatrixUpstreamNotIntegrated);
        }
        if !consumes(REQUEST_QUALIFICATION_RECORD_KIND) {
            violations.push(AuthPortabilityQualificationViolation::WorkspaceUpstreamNotIntegrated);
        }
        if !consumes(SHIP_QUERY_HISTORY_QUALIFICATION_RECORD_KIND) {
            violations
                .push(AuthPortabilityQualificationViolation::QueryHistoryUpstreamNotIntegrated);
        }
    }
}

/// Loads the checked-in auth/portability qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_auth_portability_qualification(
) -> Result<AuthPortabilityQualificationPacket, serde_json::Error> {
    serde_json::from_str(AUTH_PORTABILITY_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthPortabilityQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Auth-sheet rows.
    AuthSheet,
    /// Secret-source cue rows.
    SecretCue,
    /// Continuity rows.
    Continuity,
    /// Portability rows.
    Portability,
    /// Upstream reference rows.
    UpstreamRef,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<AuthPortabilityQualificationViolation>,
    kind: AuthPortabilityQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(AuthPortabilityQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for auth/portability qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthPortabilityQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: AuthPortabilityQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required consumer surface kind is missing.
    MissingSurfaceKind {
        surface_kind: AuthPortabilitySurfaceKind,
    },
    /// Auth sheet does not project its identity columns.
    IncompleteAuthSheet { auth_sheet_id: String },
    /// Auth sheet carries or persists a raw secret.
    AuthSheetLeaksSecret { auth_sheet_id: String },
    /// Auth sheet's browser/device-code state disagrees with its scheme.
    BrowserFlowStateMismatch { auth_sheet_id: String },
    /// Auth sheet's secret-cue ref disagrees with its secret source mode.
    AuthSheetCueMismatch { auth_sheet_id: String },
    /// Managed or companion auth sheet does not isolate desktop-local trust.
    AuthSheetOriginTrustNotIsolated { auth_sheet_id: String },
    /// Required auth scheme is missing.
    MissingAuthScheme { auth_scheme: AuthSchemeClass },
    /// Required token lifetime is missing.
    MissingTokenLifetime { token_lifetime: TokenLifetimeClass },
    /// Required secret source mode is missing.
    MissingAuthSourceMode { auth_source_mode: AuthSourceMode },
    /// No managed or companion auth sheet is covered.
    NoTrustIsolatedAuthSheetCovered,
    /// Secret-source cue is incomplete.
    IncompleteSecretCue { cue_id: String },
    /// Secret-source cue carries or persists a raw secret.
    SecretCueLeaks { cue_id: String },
    /// Required secret-safe storage mode is missing.
    MissingStorageMode {
        storage_mode: SecretSafeAuthStorageMode,
    },
    /// Required provenance is missing.
    MissingProvenance { provenance: AuthSourceProvenance },
    /// Continuity row is incomplete.
    IncompleteContinuity { continuity_id: String },
    /// Continuity row tracks a non-browser, non-device-code scheme.
    NonBrowserContinuity { continuity_id: String },
    /// Continuity row carries the not-applicable sentinel state.
    ContinuityStateNotApplicable { continuity_id: String },
    /// Pending-user-action continuity row lacks a user-action prompt.
    ContinuityMissingUserAction { continuity_id: String },
    /// Continuity row carries a raw token.
    ContinuityLeaksToken { continuity_id: String },
    /// Managed or companion continuity row does not isolate desktop-local trust.
    ContinuityOriginTrustNotIsolated { continuity_id: String },
    /// Required continuity state is missing.
    MissingContinuityState { state: BrowserDeviceCodeState },
    /// Required continuity flow scheme is missing.
    MissingContinuityFlowScheme { flow_scheme: AuthSchemeClass },
    /// No resumable continuity row is covered.
    NoResumableContinuityCovered,
    /// Portability row is incomplete.
    IncompletePortability { portability_id: String },
    /// Portability drops contract source, retention mode, or redaction posture.
    PortabilityDropsState { portability_id: String },
    /// Portability carries or persists a raw secret.
    PortabilityLeaksSecret { portability_id: String },
    /// Portability hides contract freshness.
    PortabilityHidesFreshness { portability_id: String },
    /// Offline or imported portability masquerades as a live contract.
    PortabilityMasqueradesLive { portability_id: String },
    /// Portability contract source and freshness state disagree.
    PortabilityFreshnessInconsistent { portability_id: String },
    /// Portability does not keep request definitions text-first.
    PortabilityNotTextFirst { portability_id: String },
    /// Required portability direction is missing.
    MissingPortabilityDirection { direction: PortabilityDirection },
    /// Required contract source class is missing.
    MissingContractSource { source_class: ContractSourceClass },
    /// Required retention mode is missing.
    MissingRetentionMode { retention_mode: RetentionMode },
    /// Required export redaction class is missing.
    MissingExportClass { export_class: ExportRedactionClass },
    /// Required offline mirror behavior is missing.
    MissingMirrorBehavior {
        mirror_behavior: OfflineMirrorBehavior,
    },
    /// Required mirror/offline state is missing.
    MissingMirrorState {
        mirror_state: MirrorOrOfflineStateClass,
    },
    /// No offline or imported honest reopen is covered.
    NoOfflineSafeReopenCovered,
    /// Upstream reference is incomplete.
    IncompleteUpstreamRef { ref_id: String },
    /// The lane does not consume the API-collection matrix as a verified upstream packet.
    MatrixUpstreamNotIntegrated,
    /// The lane does not consume the request-workspace lane as a verified upstream packet.
    WorkspaceUpstreamNotIntegrated,
    /// The lane does not consume the query-history lane as a verified upstream packet.
    QueryHistoryUpstreamNotIntegrated,
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for AuthPortabilityQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedSurfaceLacksDowngradeRule { surface_id } => {
                write!(f, "{surface_id} is narrowed without a downgrade rule")
            }
            Self::MissingSurfaceKind { surface_kind } => {
                write!(f, "consumer surface kind {surface_kind:?} is not covered")
            }
            Self::IncompleteAuthSheet { auth_sheet_id } => {
                write!(
                    f,
                    "{auth_sheet_id} does not project auth-sheet truth everywhere"
                )
            }
            Self::AuthSheetLeaksSecret { auth_sheet_id } => {
                write!(f, "{auth_sheet_id} carries or persists a raw secret")
            }
            Self::BrowserFlowStateMismatch { auth_sheet_id } => {
                write!(
                    f,
                    "{auth_sheet_id} browser/device-code state disagrees with its scheme"
                )
            }
            Self::AuthSheetCueMismatch { auth_sheet_id } => {
                write!(
                    f,
                    "{auth_sheet_id} secret-cue ref disagrees with its secret source mode"
                )
            }
            Self::AuthSheetOriginTrustNotIsolated { auth_sheet_id } => {
                write!(
                    f,
                    "{auth_sheet_id} is a managed or companion origin without isolated trust"
                )
            }
            Self::MissingAuthScheme { auth_scheme } => {
                write!(f, "auth scheme {auth_scheme:?} is not covered")
            }
            Self::MissingTokenLifetime { token_lifetime } => {
                write!(f, "token lifetime {token_lifetime:?} is not covered")
            }
            Self::MissingAuthSourceMode { auth_source_mode } => {
                write!(f, "secret source mode {auth_source_mode:?} is not covered")
            }
            Self::NoTrustIsolatedAuthSheetCovered => {
                write!(f, "no managed or companion auth sheet is covered")
            }
            Self::IncompleteSecretCue { cue_id } => {
                write!(
                    f,
                    "{cue_id} does not project secret-source cue truth everywhere"
                )
            }
            Self::SecretCueLeaks { cue_id } => {
                write!(f, "{cue_id} carries or persists a raw secret")
            }
            Self::MissingStorageMode { storage_mode } => {
                write!(
                    f,
                    "secret-safe storage mode {storage_mode:?} is not covered"
                )
            }
            Self::MissingProvenance { provenance } => {
                write!(f, "provenance {provenance:?} is not covered")
            }
            Self::IncompleteContinuity { continuity_id } => {
                write!(
                    f,
                    "{continuity_id} does not project continuity truth everywhere"
                )
            }
            Self::NonBrowserContinuity { continuity_id } => {
                write!(
                    f,
                    "{continuity_id} tracks a non-browser, non-device-code scheme"
                )
            }
            Self::ContinuityStateNotApplicable { continuity_id } => {
                write!(
                    f,
                    "{continuity_id} carries the not-applicable sentinel state"
                )
            }
            Self::ContinuityMissingUserAction { continuity_id } => {
                write!(
                    f,
                    "{continuity_id} is pending a user action without a user-action prompt"
                )
            }
            Self::ContinuityLeaksToken { continuity_id } => {
                write!(f, "{continuity_id} carries a raw token")
            }
            Self::ContinuityOriginTrustNotIsolated { continuity_id } => {
                write!(
                    f,
                    "{continuity_id} is a managed or companion origin without isolated trust"
                )
            }
            Self::MissingContinuityState { state } => {
                write!(f, "continuity state {state:?} is not covered")
            }
            Self::MissingContinuityFlowScheme { flow_scheme } => {
                write!(f, "continuity flow scheme {flow_scheme:?} is not covered")
            }
            Self::NoResumableContinuityCovered => {
                write!(f, "no resumable continuity row is covered")
            }
            Self::IncompletePortability { portability_id } => {
                write!(
                    f,
                    "{portability_id} does not project portability truth everywhere"
                )
            }
            Self::PortabilityDropsState { portability_id } => {
                write!(
                    f,
                    "{portability_id} drops contract source, retention mode, or redaction posture"
                )
            }
            Self::PortabilityLeaksSecret { portability_id } => {
                write!(f, "{portability_id} carries or persists a raw secret")
            }
            Self::PortabilityHidesFreshness { portability_id } => {
                write!(f, "{portability_id} hides contract freshness")
            }
            Self::PortabilityMasqueradesLive { portability_id } => {
                write!(
                    f,
                    "{portability_id} is offline or imported but claims a live contract"
                )
            }
            Self::PortabilityFreshnessInconsistent { portability_id } => {
                write!(
                    f,
                    "{portability_id} contract source and freshness state disagree"
                )
            }
            Self::PortabilityNotTextFirst { portability_id } => {
                write!(
                    f,
                    "{portability_id} does not keep request definitions text-first"
                )
            }
            Self::MissingPortabilityDirection { direction } => {
                write!(f, "portability direction {direction:?} is not covered")
            }
            Self::MissingContractSource { source_class } => {
                write!(f, "contract source class {source_class:?} is not covered")
            }
            Self::MissingRetentionMode { retention_mode } => {
                write!(f, "retention mode {retention_mode:?} is not covered")
            }
            Self::MissingExportClass { export_class } => {
                write!(f, "export redaction class {export_class:?} is not covered")
            }
            Self::MissingMirrorBehavior { mirror_behavior } => {
                write!(
                    f,
                    "offline mirror behavior {mirror_behavior:?} is not covered"
                )
            }
            Self::MissingMirrorState { mirror_state } => {
                write!(f, "mirror/offline state {mirror_state:?} is not covered")
            }
            Self::NoOfflineSafeReopenCovered => {
                write!(f, "no offline or imported honest reopen is covered")
            }
            Self::IncompleteUpstreamRef { ref_id } => {
                write!(
                    f,
                    "{ref_id} does not project upstream reference truth everywhere"
                )
            }
            Self::MatrixUpstreamNotIntegrated => {
                write!(
                    f,
                    "lane does not consume the API-collection matrix as a verified upstream packet"
                )
            }
            Self::WorkspaceUpstreamNotIntegrated => {
                write!(f, "lane does not consume the request-workspace lane as a verified upstream packet")
            }
            Self::QueryHistoryUpstreamNotIntegrated => {
                write!(
                    f,
                    "lane does not consume the query-history lane as a verified upstream packet"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for AuthPortabilityQualificationViolation {}
