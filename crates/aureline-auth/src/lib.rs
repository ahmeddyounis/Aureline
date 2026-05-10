//! System-browser auth callback seed and local-versus-managed shell vocabulary.
//!
//! This crate is the M1 seed for the auth lane. It owns:
//!
//! - one inspectable [`browser_callback::BrowserCallbackPacket`] record that
//!   freezes the outbound system-browser handoff, the callback-correlation
//!   envelope, the return route, the preserved-local-work block, and the typed
//!   recovery / retry-path vocabulary; and
//! - one [`browser_callback::ShellAuthVocabulary`] projection that distinguishes
//!   `account_free_local`, `signed_in_managed`, `reauth_required`, and
//!   `not_configured` postures without blocking local work.
//!
//! Surfaces (terminal pane, task / debug-prep seeds, provider/auth entry
//! points, activity center, status bar, support / export flows) read these
//! records by reference. They never invent a local `is_signed_in` boolean,
//! never collapse `restricted_managed_only` into `managed`, and never present
//! an embedded credential collector as a silent fallback for a blocked
//! system-browser launch.
//!
//! The reviewer-facing landing page is
//! [`/docs/auth/system_browser_seed.md`](../../../docs/auth/system_browser_seed.md).
//! The frozen cross-tool boundary vocabulary lives in
//! [`/docs/auth/system_browser_callback_packet.md`](../../../docs/auth/system_browser_callback_packet.md)
//! and [`/schemas/auth/auth_callback_state.schema.json`](../../../schemas/auth/auth_callback_state.schema.json).
//! This seed deliberately covers a subset of those vocabularies — enough for
//! one honest protected row in the live shell — and grows additively without
//! forking truth.

#![doc(html_root_url = "https://docs.rs/aureline-auth/0.0.0")]

pub mod browser_callback;

pub use browser_callback::{
    AccountBoundaryClass, AuthFlowClass, BrowserCallbackHandoff, BrowserCallbackPacket,
    BrowserCallbackValidationError, BrowserLaunchPolicyClass, CallbackCorrelation,
    EmbeddedFallbackPosture, IdentityModeAlias, PendingSessionDeniedReason, PendingSessionState,
    PreservedLocalWork, PreservedLocalWorkPostureClass, RecoveryPath, ReturnModeClass,
    ReturnOriginValidationClass, ReturnRoute, ReturnTenantOrWorkspaceMatchRule,
    ReturnedCallbackInputs, RetryPathClass, ShellAuthChip, ShellAuthVocabulary,
    StageAccountFreeLocalRequest, StageSystemBrowserHandoffRequest, TrustState,
    BROWSER_CALLBACK_PACKET_RECORD_KIND, BROWSER_CALLBACK_PACKET_SCHEMA_VERSION,
};
