//! System-browser auth callback seed, local-versus-managed shell vocabulary,
//! and credential-state / provider-account registry seed.
//!
//! This crate owns the auth seed lane. It provides:
//!
//! - one inspectable [`browser_callback::BrowserCallbackPacket`] record that
//!   freezes the outbound system-browser handoff, the callback-correlation
//!   envelope, the return route, the preserved-local-work block, and the typed
//!   recovery / retry-path vocabulary;
//! - one [`browser_callback::ShellAuthVocabulary`] projection that distinguishes
//!   `account_free_local`, `signed_in_managed`, `reauth_required`, and
//!   `not_configured` postures without blocking local work; and
//! - one [`credential_state::CredentialStateRow`] +
//!   [`credential_state::ProviderAccountRegistry`] seed that explains storage
//!   mode, scope, expiry, revoke action, and locked / unavailable store
//!   posture for credentials and delegated handles used by the initial
//!   managed / provider lanes; and
//! - one [`system_browser::ClaimedIdentityRow`] alpha contract that defaults
//!   claimed managed / provider rows to system-browser auth while surfacing
//!   device-code and stay-local fallback paths; and
//! - one [`identity_modes::IdentityModeBaselinePacket`] alpha contract that
//!   keeps account-free local, self-hosted, and managed convenience rows
//!   separate while exposing policy-source and offline-entitlement inspectors;
//!   and
//! - one [`secrets::SecretBrokerAlphaPacket`] alpha contract that lets
//!   provider, registry, database, and remote lanes reference OS credential-
//!   store handles, session-only broker memory, and delegated credentials
//!   while exporting only redaction-safe metadata and first-class continuity
//!   results; and
//! - one [`workspace_trust::WorkspaceTrustBetaPage`] beta audit that keeps
//!   open, run, debug, extension, AI, provider, review, support, and admin
//!   rows on the same restricted-mode and trust-elevation vocabulary; and
//! - one [`policy_packs::PolicyPackBetaPage`] beta projection that turns
//!   effective policy packs, mirror and manual-import receipts, before/after
//!   diffs, and product-denial explain traces into a single inspectable record
//!   for admin, support, mirror, and offline lanes.
//!
//! Surfaces (terminal pane, task / debug-prep seeds, provider/auth entry
//! points, activity center, status bar, support / export flows) read these
//! records by reference. They never invent a local `is_signed_in` boolean,
//! never collapse `restricted_managed_only` into `managed`, never present
//! an embedded credential collector as a silent fallback for a blocked
//! system-browser launch, and never silently downgrade a locked or
//! unavailable secure store to a plaintext-file credential.
//!
//! The reviewer-facing landing pages are
//! [`/docs/auth/system_browser_seed.md`](../../../docs/auth/system_browser_seed.md),
//! [`/docs/auth/credential_state_seed.md`](../../../docs/auth/credential_state_seed.md),
//! [`/docs/identity/local_vs_managed_alpha.md`](../../../docs/identity/local_vs_managed_alpha.md),
//! and
//! [`/docs/security/secret_broker_alpha.md`](../../../docs/security/secret_broker_alpha.md).
//! The frozen cross-tool boundary vocabularies live in
//! [`/docs/auth/system_browser_callback_packet.md`](../../../docs/auth/system_browser_callback_packet.md),
//! [`/schemas/auth/auth_callback_state.schema.json`](../../../schemas/auth/auth_callback_state.schema.json),
//! [`/docs/auth/credential_state_and_secret_prompt_contract.md`](../../../docs/auth/credential_state_and_secret_prompt_contract.md),
//! and
//! [`/schemas/auth/credential_state.schema.json`](../../../schemas/auth/credential_state.schema.json).
//! These seeds deliberately cover a subset of those vocabularies — enough for
//! one honest protected row in the live shell — and grow additively without
//! forking truth.

#![doc(html_root_url = "https://docs.rs/aureline-auth/0.0.0")]

pub mod browser_callback;
pub mod credential_state;
pub mod identity_modes;
pub mod network_trust;
pub mod policy_packs;
pub mod secrets;
pub mod system_browser;
pub mod trust;
pub mod workspace_trust;

pub use browser_callback::{
    AccountBoundaryClass, AuthFlowClass, BrowserCallbackHandoff, BrowserCallbackPacket,
    BrowserCallbackValidationError, BrowserLaunchPolicyClass, CallbackCorrelation,
    EmbeddedFallbackPosture, IdentityModeAlias, PendingSessionDeniedReason, PendingSessionState,
    PreservedLocalWork, PreservedLocalWorkPostureClass, RecoveryPath, RetryPathClass,
    ReturnModeClass, ReturnOriginValidationClass, ReturnRoute, ReturnTenantOrWorkspaceMatchRule,
    ReturnedCallbackInputs, ShellAuthChip, ShellAuthVocabulary, StageAccountFreeLocalRequest,
    StageSystemBrowserHandoffRequest, TrustState, BROWSER_CALLBACK_PACKET_RECORD_KIND,
    BROWSER_CALLBACK_PACKET_SCHEMA_VERSION,
};

pub use credential_state::{
    CredentialLifetime, CredentialScope, CredentialStateChip, CredentialStateClass,
    CredentialStateRow, CredentialUnavailableReason, LifetimeClass, ProviderAccountRecord,
    ProviderAccountRegistry, RevokeActionClass, StorageModeClass, StoragePosture, StoreSourceClass,
    CREDENTIAL_STATE_ROW_RECORD_KIND, CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
    PROVIDER_ACCOUNT_RECORD_KIND, PROVIDER_ACCOUNT_REGISTRY_RECORD_KIND,
};

pub use policy_packs::{
    audit_policy_pack_beta_page, seeded_policy_pack_beta_page, validate_policy_pack_beta_page,
    PolicyPackApplyStateClass, PolicyPackBetaDefect, PolicyPackBetaDefectKind, PolicyPackBetaDiff,
    PolicyPackBetaDiffEntry, PolicyPackBetaDenialTrace, PolicyPackBetaImportReceipt,
    PolicyPackBetaPack, PolicyPackBetaPage, PolicyPackBetaProfileClass, PolicyPackBetaRule,
    PolicyPackBetaSummary, PolicyPackBetaSupportExport, PolicyPackDiffEntryKind, PolicyPackProvenance,
    PolicyPackRuleEffectClass, PolicyPackSignatureStateClass, PolicyPackSourceClass,
    POLICY_PACK_BETA_DEFECT_RECORD_KIND, POLICY_PACK_BETA_DENIAL_TRACE_RECORD_KIND,
    POLICY_PACK_BETA_DIFF_ENTRY_RECORD_KIND, POLICY_PACK_BETA_DIFF_RECORD_KIND,
    POLICY_PACK_BETA_IMPORT_RECEIPT_RECORD_KIND, POLICY_PACK_BETA_PACK_RECORD_KIND,
    POLICY_PACK_BETA_PAGE_RECORD_KIND, POLICY_PACK_BETA_RULE_RECORD_KIND,
    POLICY_PACK_BETA_SCHEMA_VERSION, POLICY_PACK_BETA_SHARED_CONTRACT_REF,
    POLICY_PACK_BETA_SUMMARY_RECORD_KIND, POLICY_PACK_BETA_SUPPORT_EXPORT_RECORD_KIND,
};

pub use identity_modes::{
    CurrentDeploymentBoundaryClass, DeploymentBoundaryDisclosure, EntitlementStateClass,
    IdentityAuthModeClass, IdentityModeArtifactRefs, IdentityModeBaselinePacket,
    IdentityModeBaselineRow, IdentityModeBaselineRowError, IdentityModeBaselineViolation,
    IdentityModeDeploymentProfileClass, IdentityModeSurfaceRow, IdentityPolicySourceInspector,
    IdentityPolicySourceInspectorRequest, KeyMode, LocalCoreContinuity, OfflineBehaviorClass,
    OfflineEntitlementInspector, OfflineEntitlementInspectorRequest, PolicyFreshnessClass,
    PolicySourceClass, ProvisioningClass, RegionMode, ResidencyMode,
    StageIdentityModeBaselineRowRequest, IDENTITY_MODE_BASELINE_PACKET_RECORD_KIND,
    IDENTITY_MODE_BASELINE_ROW_RECORD_KIND, IDENTITY_MODE_BASELINE_SCHEMA_VERSION,
    REQUIRED_LOCAL_CORE_CAPABILITY_IDS,
};

pub use secrets::{
    AffectedCapabilityClass, ContinuityStateClass, LocalContinuationClass, ProjectionModeClass,
    RecoveryActionClass, SecretBrokerAlphaPacket, SecretBrokerAlphaRow, SecretBrokerDenialReason,
    SecretBrokerPacketError, SecretBrokerRowError, SecretBrokerSupportExport,
    SecretBrokerSupportExportRow, SecretBrokerSurfaceRow, SecretClass, SecretConsumerIdentity,
    SecretContinuityResult, SecretExportPosture, SecretReference, SecretReferenceMode,
    SecretStorageBinding, TrustStoreClass, UnlockStateClass,
    SECRET_BROKER_ALPHA_PACKET_RECORD_KIND, SECRET_BROKER_ALPHA_SCHEMA_VERSION,
    SECRET_BROKER_ROW_RECORD_KIND, SECRET_BROKER_SUPPORT_EXPORT_RECORD_KIND,
    SECRET_BROKER_SUPPORT_EXPORT_ROW_RECORD_KIND,
};

pub use system_browser::{
    ClaimedIdentityAlternativeClass, ClaimedIdentityAuthAlternative, ClaimedIdentityAuthPolicy,
    ClaimedIdentityDefaultActionClass, ClaimedIdentityHandoffRefs,
    ClaimedIdentityLocalContinuation, ClaimedIdentityProviderScope, ClaimedIdentityRow,
    ClaimedIdentityRowError, ClaimedIdentitySessionWindow, ClaimedIdentityStateClass,
    ClaimedIdentitySurfaceRow, StageClaimedIdentityRowRequest, SystemBrowserAlphaPacket,
    CLAIMED_IDENTITY_ROW_RECORD_KIND, SYSTEM_BROWSER_ALPHA_PACKET_RECORD_KIND,
    SYSTEM_BROWSER_ALPHA_SCHEMA_VERSION,
};

pub use system_browser::beta::{
    audit_rows as audit_system_browser_return_paths_beta_rows,
    seeded_system_browser_return_paths_beta_page, validate_system_browser_return_paths_beta_page,
    AuthorityScopeClass, PasskeyStepUpBlock, PasskeyStepUpPostureClass, ReturnPathLabel,
    StageSystemBrowserReturnPathBetaRowRequest, SystemBrowserPolicyExceptionClass,
    SystemBrowserReturnPathBetaAxis, SystemBrowserReturnPathBetaDefect,
    SystemBrowserReturnPathBetaDefectKind, SystemBrowserReturnPathBetaRow,
    SystemBrowserReturnPathBetaSupportRow, SystemBrowserReturnPathsBetaPage,
    SystemBrowserReturnPathsBetaSummary, SystemBrowserReturnPathsBetaSupportExport,
    SYSTEM_BROWSER_RETURN_PATHS_BETA_DEFECT_RECORD_KIND,
    SYSTEM_BROWSER_RETURN_PATHS_BETA_PAGE_RECORD_KIND,
    SYSTEM_BROWSER_RETURN_PATHS_BETA_ROW_RECORD_KIND,
    SYSTEM_BROWSER_RETURN_PATHS_BETA_SCHEMA_VERSION,
    SYSTEM_BROWSER_RETURN_PATHS_BETA_SHARED_CONTRACT_REF,
    SYSTEM_BROWSER_RETURN_PATHS_BETA_SUPPORT_EXPORT_RECORD_KIND,
    SYSTEM_BROWSER_RETURN_PATHS_BETA_SUPPORT_ROW_RECORD_KIND,
};

pub use trust::{
    authority_for_trust_state, external_effect_for_capability, CapabilityAuthorityClass,
    CapabilityDecisionSource, CapabilityDisclosureRow, CapabilityScope, ExternalEffectClass,
    LaunchWedgeCapabilityFamily, RememberedDecisionScopeClass, RestrictedModeAlphaPacket,
    RestrictedModeEntryTransitionClass, RestrictedModeLaunchWedgeDisclosure,
    RestrictedModeTrustSource, RestrictedModeTrustStateClass, RestrictedModeValidationError,
    StageRestrictedModeLaunchRequest, TrustAuditEventClass, TrustDecisionSourceClass,
    TrustEscalationCueClass, TrustReasonClass, TrustRecoveryActionClass,
    RESTRICTED_MODE_ALPHA_PACKET_RECORD_KIND, RESTRICTED_MODE_ALPHA_SCHEMA_VERSION,
};

pub use network_trust::{
    audit_network_trust_beta_rows, seeded_network_trust_beta_page,
    validate_network_trust_beta_page, ClientCertificateStateClass, NetworkAuthorityClass,
    NetworkConsumerLaneClass, NetworkSettingLockClass, NetworkSettingSourceClass,
    NetworkTrustBetaDefect, NetworkTrustBetaDefectKind, NetworkTrustBetaFacetClass,
    NetworkTrustBetaPage, NetworkTrustBetaProfileBinding, NetworkTrustBetaProfileClass,
    NetworkTrustBetaRow, NetworkTrustBetaSummary, NetworkTrustBetaSupportExport,
    NetworkTrustBetaSupportRow, ProxyResolutionModeClass, SshHostProofClass, TrustStoreSourceClass,
    NETWORK_TRUST_BETA_DEFECT_RECORD_KIND, NETWORK_TRUST_BETA_PAGE_RECORD_KIND,
    NETWORK_TRUST_BETA_PROFILE_BINDING_RECORD_KIND, NETWORK_TRUST_BETA_ROW_RECORD_KIND,
    NETWORK_TRUST_BETA_SCHEMA_VERSION, NETWORK_TRUST_BETA_SHARED_CONTRACT_REF,
    NETWORK_TRUST_BETA_SOURCE_MATRIX_REF, NETWORK_TRUST_BETA_SUMMARY_RECORD_KIND,
    NETWORK_TRUST_BETA_SUPPORT_EXPORT_RECORD_KIND, NETWORK_TRUST_BETA_SUPPORT_ROW_RECORD_KIND,
};

pub use workspace_trust::{
    audit_workspace_trust_beta_rows, seeded_workspace_trust_beta_page,
    validate_workspace_trust_beta_page, WorkspaceTrustBetaDefect, WorkspaceTrustBetaDefectKind,
    WorkspaceTrustBetaLaneClass, WorkspaceTrustBetaPage, WorkspaceTrustBetaProfileAuthority,
    WorkspaceTrustBetaProfileClass, WorkspaceTrustBetaRow, WorkspaceTrustBetaSummary,
    WorkspaceTrustBetaSupportExport, WorkspaceTrustBetaSupportRow,
    WORKSPACE_TRUST_BETA_DEFECT_RECORD_KIND, WORKSPACE_TRUST_BETA_PAGE_RECORD_KIND,
    WORKSPACE_TRUST_BETA_ROW_RECORD_KIND, WORKSPACE_TRUST_BETA_SCHEMA_VERSION,
    WORKSPACE_TRUST_BETA_SHARED_CONTRACT_REF, WORKSPACE_TRUST_BETA_SUPPORT_EXPORT_RECORD_KIND,
    WORKSPACE_TRUST_BETA_SUPPORT_ROW_RECORD_KIND, WORKSPACE_TRUST_BETA_SURFACE_FAMILIES,
};
