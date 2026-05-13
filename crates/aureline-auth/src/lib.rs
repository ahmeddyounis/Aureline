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
//!   separate while exposing policy-source and offline-entitlement inspectors.
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
//! and
//! [`/docs/identity/local_vs_managed_alpha.md`](../../../docs/identity/local_vs_managed_alpha.md).
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
pub mod system_browser;
pub mod trust;

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

pub use identity_modes::{
    CurrentDeploymentBoundaryClass, DeploymentBoundaryDisclosure, EntitlementStateClass,
    IdentityAuthModeClass, IdentityModeArtifactRefs, IdentityModeBaselinePacket,
    IdentityModeBaselineRow, IdentityModeBaselineRowError, IdentityModeBaselineViolation,
    IdentityModeDeploymentProfileClass, IdentityModeSurfaceRow, IdentityPolicySourceInspector,
    IdentityPolicySourceInspectorRequest, LocalCoreContinuity, OfflineBehaviorClass,
    OfflineEntitlementInspector, OfflineEntitlementInspectorRequest, PolicyFreshnessClass,
    PolicySourceClass, ProvisioningClass, StageIdentityModeBaselineRowRequest,
    IDENTITY_MODE_BASELINE_PACKET_RECORD_KIND, IDENTITY_MODE_BASELINE_ROW_RECORD_KIND,
    IDENTITY_MODE_BASELINE_SCHEMA_VERSION, REQUIRED_LOCAL_CORE_CAPABILITY_IDS,
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

pub use trust::{
    CapabilityAuthorityClass, CapabilityDecisionSource, CapabilityDisclosureRow, CapabilityScope,
    ExternalEffectClass, LaunchWedgeCapabilityFamily, RememberedDecisionScopeClass,
    RestrictedModeAlphaPacket, RestrictedModeEntryTransitionClass,
    RestrictedModeLaunchWedgeDisclosure, RestrictedModeTrustSource, RestrictedModeTrustStateClass,
    RestrictedModeValidationError, StageRestrictedModeLaunchRequest, TrustAuditEventClass,
    TrustDecisionSourceClass, TrustEscalationCueClass, TrustReasonClass, TrustRecoveryActionClass,
    RESTRICTED_MODE_ALPHA_PACKET_RECORD_KIND, RESTRICTED_MODE_ALPHA_SCHEMA_VERSION,
};
