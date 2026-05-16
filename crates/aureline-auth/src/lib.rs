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
//!   for admin, support, mirror, and offline lanes; and
//! - one [`offline_entitlements::OfflineEntitlementVerifierBetaPage`] beta
//!   projection that runs a signed policy-bundle and offline-entitlement
//!   verifier across connected, mirror, offline, and enterprise-managed
//!   profiles for both policy and entitlement bundle kinds, downgrades
//!   managed capability authority on expired, missing, or unverifiable
//!   bundles, and preserves local editing through every failure mode; and
//! - one [`oidc::OidcSystemBrowserBetaPage`] beta projection that audits
//!   enterprise OIDC system-browser sign-in, recovery, and session-continuity
//!   flows so claimed-enterprise rows disclose issuer source, tenant /
//!   workspace binding, and return path, degrade truthfully on outage or
//!   denial, and preserve local editing through sign-out and refresh; and
//! - one [`passkey::PasskeyStepUpBetaPage`] beta projection that audits
//!   passkey-capable step-up, reauth, and recovery lanes so claimed rows
//!   name their lane, disclose lifecycle state and client scope, name a
//!   typed fallback when the platform or policy denies passkey, and
//!   preserve the originating target / action identity across reauth and
//!   recovery without widening authority; and
//! - one [`provisioning::AdminAuditExportBetaPage`] beta projection that
//!   turns SCIM and signed-file provisioning hooks, policy-bundle history
//!   transitions, and entitlement changes into one auditable admin-audit
//!   export so enterprise pilots inspect provisioning, history, and
//!   entitlement state with one record kind across connected, mirror-only,
//!   offline, and enterprise-managed beta profiles without silently widening
//!   managed authority; and
//! - one [`secret_broker::SecretBrokerBetaPage`] beta projection that turns
//!   vault/keychain integration into handle-only projection rows paired with
//!   a consumer-identity audit stream so admin, support, and reviewer
//!   surfaces inspect which consumer, target, workspace scope, and projection
//!   mode requested a secret across connected, mirror-only, offline, and
//!   enterprise-managed beta profiles without exposing raw secret material,
//!   raw handle ids, plaintext persistence, silent in-memory promotion,
//!   stale handle reuse, or undeclared public-endpoint fallback; and
//! - one [`keychain_state::SecretRepairBetaPage`] beta projection that turns
//!   keychain lock-state, denied projection, and secret-repair flows into
//!   three reviewable record kinds so admin, support, and reviewer surfaces
//!   can name the affected consumer, the blocked target, the typed repair
//!   action, and the typed repair outcome across connected, mirror-only,
//!   offline, and enterprise-managed beta profiles while preserving the
//!   no-plaintext-fallback invariant and local editing.
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
pub mod keychain_state;
pub mod network_trust;
pub mod offline_entitlements;
pub mod oidc;
pub mod passkey;
pub mod policy_packs;
pub mod provisioning;
pub mod secret_broker;
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

pub use offline_entitlements::{
    audit_offline_entitlement_verifier_beta_rows, seeded_offline_entitlement_verifier_beta_page,
    validate_offline_entitlement_verifier_beta_page, LocalEditingPreservationClass,
    ManagedCapabilityImpactClass, OfflineEntitlementVerifierBetaDefect,
    OfflineEntitlementVerifierBetaDefectKind, OfflineEntitlementVerifierBetaPage,
    OfflineEntitlementVerifierBetaProfileClass, OfflineEntitlementVerifierBetaRow,
    OfflineEntitlementVerifierBetaSummary, OfflineEntitlementVerifierBetaSupportExport,
    OfflineEntitlementVerifierBetaSupportRow, StageOfflineEntitlementVerifierBetaRowRequest,
    TrustAnchorSourceClass, VerifiedBundleKindClass, VerifierBundleSubject,
    VerifierOutcomeClass, VerifierRecoveryActionClass, VerifierTrustAnchor,
    OFFLINE_ENTITLEMENT_VERIFIER_BETA_DEFECT_RECORD_KIND,
    OFFLINE_ENTITLEMENT_VERIFIER_BETA_PAGE_RECORD_KIND,
    OFFLINE_ENTITLEMENT_VERIFIER_BETA_ROW_RECORD_KIND,
    OFFLINE_ENTITLEMENT_VERIFIER_BETA_SCHEMA_VERSION,
    OFFLINE_ENTITLEMENT_VERIFIER_BETA_SHARED_CONTRACT_REF,
    OFFLINE_ENTITLEMENT_VERIFIER_BETA_SOURCE_MATRIX_REF,
    OFFLINE_ENTITLEMENT_VERIFIER_BETA_SUMMARY_RECORD_KIND,
    OFFLINE_ENTITLEMENT_VERIFIER_BETA_SUPPORT_EXPORT_RECORD_KIND,
    OFFLINE_ENTITLEMENT_VERIFIER_BETA_SUPPORT_ROW_RECORD_KIND,
};

pub use provisioning::{
    audit_admin_audit_export_beta_page, seeded_admin_audit_export_beta_page,
    validate_admin_audit_export_beta_page, AdminAuditExportBetaDefect,
    AdminAuditExportBetaDefectKind, AdminAuditExportBetaPage,
    AdminAuditExportBetaProfileClass, AdminAuditExportBetaSummary,
    AdminAuditExportBetaSupportExport, EntitlementChangeClass, EntitlementChangeEvent,
    PolicyBundleHistoryEvent, PolicyBundleTransitionClass, ProvisioningEvent,
    ProvisioningEventClass, ProvisioningFreshnessClass, ProvisioningLifecycleStateClass,
    ProvisioningProvenance, ProvisioningSourceClass, ProvisioningSubjectKindClass,
    ADMIN_AUDIT_EXPORT_BETA_DEFECT_RECORD_KIND, ADMIN_AUDIT_EXPORT_BETA_PAGE_RECORD_KIND,
    ADMIN_AUDIT_EXPORT_BETA_SCHEMA_VERSION, ADMIN_AUDIT_EXPORT_BETA_SHARED_CONTRACT_REF,
    ADMIN_AUDIT_EXPORT_BETA_SOURCE_MATRIX_REF, ADMIN_AUDIT_EXPORT_BETA_SUMMARY_RECORD_KIND,
    ADMIN_AUDIT_EXPORT_BETA_SUPPORT_EXPORT_RECORD_KIND, ENTITLEMENT_CHANGE_EVENT_RECORD_KIND,
    POLICY_BUNDLE_HISTORY_EVENT_RECORD_KIND, PROVISIONING_EVENT_RECORD_KIND,
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

pub use secret_broker::{
    audit_secret_broker_beta_page, seeded_secret_broker_beta_page,
    validate_secret_broker_beta_page, ConsumerAuditOutcomeClass, HandleLifecycleStateClass,
    HandleProjectionModeClass, SecretBrokerBetaDefect, SecretBrokerBetaDefectKind,
    SecretBrokerBetaHandleRow, SecretBrokerBetaPage, SecretBrokerBetaProfileClass,
    SecretBrokerBetaSummary, SecretBrokerBetaSupportExport, SecretConsumerAuditEvent, VaultAdapterClass,
    VaultBinding, VaultSignatureStateClass, SECRET_BROKER_BETA_CONSUMER_AUDIT_RECORD_KIND,
    SECRET_BROKER_BETA_DEFECT_RECORD_KIND, SECRET_BROKER_BETA_HANDLE_ROW_RECORD_KIND,
    SECRET_BROKER_BETA_PAGE_RECORD_KIND, SECRET_BROKER_BETA_SCHEMA_VERSION,
    SECRET_BROKER_BETA_SHARED_CONTRACT_REF, SECRET_BROKER_BETA_SOURCE_MATRIX_REF,
    SECRET_BROKER_BETA_SUMMARY_RECORD_KIND, SECRET_BROKER_BETA_SUPPORT_EXPORT_RECORD_KIND,
};

pub use keychain_state::{
    audit_secret_repair_beta_page, seeded_secret_repair_beta_page,
    validate_secret_repair_beta_page, DeniedProjectionRow, DenialReasonClass,
    KeychainLockStateClass, KeychainLockStateRow, RepairActionClass, RepairOutcomeClass,
    SecretRepairActionEvent, SecretRepairBetaDefect, SecretRepairBetaDefectKind,
    SecretRepairBetaPage, SecretRepairBetaSummary, SecretRepairBetaSupportExport,
    SECRET_REPAIR_BETA_DEFECT_RECORD_KIND, SECRET_REPAIR_BETA_DENIED_PROJECTION_ROW_RECORD_KIND,
    SECRET_REPAIR_BETA_LOCK_STATE_ROW_RECORD_KIND, SECRET_REPAIR_BETA_PAGE_RECORD_KIND,
    SECRET_REPAIR_BETA_REPAIR_EVENT_RECORD_KIND, SECRET_REPAIR_BETA_SCHEMA_VERSION,
    SECRET_REPAIR_BETA_SHARED_CONTRACT_REF, SECRET_REPAIR_BETA_SOURCE_MATRIX_REF,
    SECRET_REPAIR_BETA_SUMMARY_RECORD_KIND, SECRET_REPAIR_BETA_SUPPORT_EXPORT_RECORD_KIND,
};

pub use oidc::{
    audit_oidc_system_browser_beta_rows, seeded_oidc_system_browser_beta_page,
    validate_oidc_system_browser_beta_page, OidcAuthorityScopeClass, OidcIdentityOutageBlock,
    OidcIdentityOutageClass, OidcIssuerDisclosure, OidcIssuerSourceClass, OidcRecoveryActionClass,
    OidcReturnPathLabel, OidcSessionContinuityBlock, OidcSessionStateClass,
    OidcSignOutContinuityClass, OidcSystemBrowserBetaAxis, OidcSystemBrowserBetaDefect,
    OidcSystemBrowserBetaDefectKind, OidcSystemBrowserBetaPage,
    OidcSystemBrowserBetaProfileClass, OidcSystemBrowserBetaRow, OidcSystemBrowserBetaSummary,
    OidcSystemBrowserBetaSupportExport, OidcSystemBrowserBetaSupportRow, OidcTenantBinding,
    OidcTenantBindingClass, StageOidcSystemBrowserBetaRowRequest,
    OIDC_SYSTEM_BROWSER_BETA_DEFECT_RECORD_KIND, OIDC_SYSTEM_BROWSER_BETA_PAGE_RECORD_KIND,
    OIDC_SYSTEM_BROWSER_BETA_ROW_RECORD_KIND, OIDC_SYSTEM_BROWSER_BETA_SCHEMA_VERSION,
    OIDC_SYSTEM_BROWSER_BETA_SHARED_CONTRACT_REF,
    OIDC_SYSTEM_BROWSER_BETA_SUMMARY_RECORD_KIND,
    OIDC_SYSTEM_BROWSER_BETA_SUPPORT_EXPORT_RECORD_KIND,
    OIDC_SYSTEM_BROWSER_BETA_SUPPORT_ROW_RECORD_KIND,
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

pub use passkey::{
    audit_passkey_step_up_beta_rows, seeded_passkey_step_up_beta_page,
    validate_passkey_step_up_beta_page, PasskeyAuthorityScopeClass, PasskeyBetaLaneClass,
    PasskeyBetaProfileClass, PasskeyClientScopeClass, PasskeyFallbackClass, PasskeyLaneBlock,
    PasskeyLifecycleBlock, PasskeyLifecycleStateClass, PasskeyOutcomeBlock, PasskeyOutcomeClass,
    PasskeyStepUpBetaAxis, PasskeyStepUpBetaDefect, PasskeyStepUpBetaDefectKind,
    PasskeyStepUpBetaPage, PasskeyStepUpBetaRow, PasskeyStepUpBetaSummary,
    PasskeyStepUpBetaSupportExport, PasskeyStepUpBetaSupportRow,
    PasskeyTargetActionPreservationBlock, PasskeyTargetActionPreservationClass,
    StagePasskeyStepUpBetaRowRequest, PASSKEY_STEP_UP_BETA_DEFECT_RECORD_KIND,
    PASSKEY_STEP_UP_BETA_PAGE_RECORD_KIND, PASSKEY_STEP_UP_BETA_ROW_RECORD_KIND,
    PASSKEY_STEP_UP_BETA_SCHEMA_VERSION, PASSKEY_STEP_UP_BETA_SHARED_CONTRACT_REF,
    PASSKEY_STEP_UP_BETA_SUPPORT_EXPORT_RECORD_KIND,
    PASSKEY_STEP_UP_BETA_SUPPORT_ROW_RECORD_KIND,
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
