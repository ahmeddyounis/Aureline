//! Browser-or-device-code handoff cards and delegated-credential rows carrying the
//! provider / org, the auth-handoff flow kind, the derived handoff-boundary class,
//! the fallback state, the local-continuity note, the code / expiry where relevant,
//! and why a safer boundary is preferred — and, for the delegated row, the source
//! identity, the target scope, the storage class, the expiration, the policy owner,
//! the stop-forward / rotate actions, and the derived local-versus-forwarded-versus-
//! remote-vault-versus-service-issued identity origin.
//!
//! This module narrows two components frozen in
//! [`crate::freeze_the_m5_credential_component_matrix`] — the
//! `browser_device_code_handoff_card` and the `delegated_credential_row` — into one
//! implemented, export-safe packet with two co-equal control vectors. Together they
//! keep every crossing from the local shell into remote / provider-controlled
//! authority explicit about **which handoff path** is being used and **which identity**
//! is being forwarded or delegated.
//!
//! A [`BrowserDeviceCodeHandoffCard`] always names the provider / org, the auth-handoff
//! flow kind, the fallback state, the local-continuity note, and why a safer boundary
//! is preferred. Its handoff-boundary class is *derived* from the auth-handoff class
//! rather than asserted: a system-browser redirect, a device-code poll, an in-app
//! local capture, and a delegated / offline-deferred handoff can never blur into one
//! generic sign-in state, and an in-app local capture can never quietly present as an
//! out-of-app system-browser boundary. It names its device code / expiry whenever a
//! device-code poll is in play and offers keyboard-complete continue / cancel actions.
//!
//! A [`DelegatedCredentialRow`] always names the source identity, the target scope, the
//! storage class, the expiration, and the policy owner, and offers keyboard-complete
//! stop-forward / rotate actions. Its identity origin is *derived* from the
//! delegated-identity state and the storage mode rather than asserted: a forwarded, a
//! remote-vault-held, or a service-issued identity can never read as a locally stored
//! credential, so delegated and forwarded identity always stay visually distinct from
//! locally stored credentials on every claimed M5 surface.
//!
//! The auth-handoff classes ([`M5AuthHandoffClass`]), delegated-identity states
//! ([`M5DelegatedIdentityState`]), storage modes ([`M5CredentialStorageMode`]),
//! credential classes ([`M5CredentialClass`]), reveal postures
//! ([`M5CredentialRevealPosture`]), lifecycle states ([`M5CredentialLifecycleState`]),
//! degraded states ([`M5CredentialDegradedState`]), required labels
//! ([`M5CredentialRequiredLabel`]), surface families ([`M5CredentialSurfaceFamily`]),
//! deployment lines ([`M5CredentialDeploymentLine`]), consumer surfaces
//! ([`M5CredentialConsumerSurface`]), accessibility routes
//! ([`M5CredentialAccessibilityRoute`]), and downgrade triggers
//! ([`M5CredentialDowngradeTrigger`]) are reused directly from the frozen matrix, so
//! this lane never invents a parallel credential vocabulary. It mints new vocabulary
//! only for what that matrix left implicit about these two controls: the derived
//! handoff-boundary class, the keyboard-complete handoff actions, the derived
//! delegated-identity origin, the delegated target scope, and the delegated-row
//! actions.
//!
//! Raw secret values, pasted tokens, passphrases, and private endpoints stay outside
//! the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-browser-device-code-handoff-delegated-credential-controls.schema.json`](../../../../schemas/ui/m5-browser-device-code-handoff-delegated-credential-controls.schema.json).
//! The contract doc is
//! [`docs/security/implement_browser_or_device_code_handoff_cards_and_delegated_credential_rows.md`](../../../../docs/security/implement_browser_or_device_code_handoff_cards_and_delegated_credential_rows.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_browser_handoff_delegated_credential_controls,
    seeded_browser_handoff_delegated_credential_controls_delegated_forwarded_identity,
    seeded_browser_handoff_delegated_credential_controls_handoff_local_capture,
    BROWSER_HANDOFF_DELEGATED_CREDENTIAL_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The auth-handoff class, delegated-identity state, storage mode, credential class,
// reveal posture, lifecycle state, degraded state, required labels, surface family,
// deployment line, consumer surface, accessibility route, and downgrade triggers are
// frozen once, in the credential component matrix. This lane reuses them verbatim so it
// never invents a parallel credential vocabulary.
use crate::freeze_the_m5_credential_component_matrix::{
    M5AuthHandoffClass, M5CredentialAccessibilityRoute, M5CredentialClass,
    M5CredentialComponentFamily, M5CredentialConsumerSurface, M5CredentialDegradedState,
    M5CredentialDeploymentLine, M5CredentialDowngradeTrigger, M5CredentialLifecycleState,
    M5CredentialRequiredLabel, M5CredentialRevealPosture, M5CredentialStorageMode,
    M5CredentialSurfaceFamily, M5DelegatedIdentityState,
    M5_BROWSER_DEVICE_CODE_HANDOFF_CARD_SCHEMA_REF, M5_CREDENTIAL_COMPONENT_DOC_REF,
    M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_PROJECTION_REF,
    M5_CREDENTIAL_COMPONENT_FOUNDATION_SYSTEM_BROWSER_REF, M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
    M5_DELEGATED_CREDENTIAL_ROW_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket`].
pub const BROWSER_HANDOFF_DELEGATED_CREDENTIAL_RECORD_KIND: &str =
    "browser_device_code_handoff_delegated_credential_controls";

/// Schema version for browser-handoff / delegated-credential control records.
pub const BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_REF: &str =
    "schemas/ui/m5-browser-device-code-handoff-delegated-credential-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const BROWSER_HANDOFF_DELEGATED_CREDENTIAL_DOC_REF: &str =
    "docs/security/implement_browser_or_device_code_handoff_cards_and_delegated_credential_rows.md";

/// Repo-relative path of the protected fixture directory.
pub const BROWSER_HANDOFF_DELEGATED_CREDENTIAL_FIXTURE_DIR: &str =
    "fixtures/ui/m5-browser-device-code-handoff-delegated-credential-controls";

/// Repo-relative path of the checked support-export artifact.
pub const BROWSER_HANDOFF_DELEGATED_CREDENTIAL_ARTIFACT_REF: &str =
    "artifacts/release/m5-browser-device-code-handoff-delegated-credential-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SUMMARY_REF: &str =
    "artifacts/release/m5-browser-device-code-handoff-delegated-credential-proof/summary.md";

// ---- browser-device-code-handoff-card vocabulary ------------------------

/// Derived handoff-boundary class a browser-or-device-code handoff card may present.
///
/// This is the handoff honesty axis: the class is derived from the auth-handoff class,
/// never asserted, so a system-browser redirect, a device-code poll, an in-app local
/// capture, and a delegated / offline-deferred handoff can never blur into one generic
/// sign-in state, and an in-app local capture can never quietly present as an
/// out-of-app system-browser boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffBoundaryClass {
    /// An out-of-app system-browser redirect boundary.
    SystemBrowserBoundary,
    /// A device-code poll boundary completed on a secondary device.
    DeviceCodeBoundary,
    /// An in-app / local-capture boundary; a safer boundary is preferred.
    LocalCaptureBoundary,
    /// A delegated-forward or offline-deferred handoff boundary.
    DelegatedOrDeferredBoundary,
}

impl HandoffBoundaryClass {
    /// Every handoff-boundary class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SystemBrowserBoundary,
        Self::DeviceCodeBoundary,
        Self::LocalCaptureBoundary,
        Self::DelegatedOrDeferredBoundary,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowserBoundary => "system_browser_boundary",
            Self::DeviceCodeBoundary => "device_code_boundary",
            Self::LocalCaptureBoundary => "local_capture_boundary",
            Self::DelegatedOrDeferredBoundary => "delegated_or_deferred_boundary",
        }
    }
}

/// One keyboard-complete default action a browser-or-device-code handoff card offers, so
/// a card never hides its continue or cancel affordance behind a pointer-only gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserDeviceCodeHandoffAction {
    /// Continue the handoff through its preferred, safer boundary.
    ContinuePreferredBoundary,
    /// Copy the device code (never a raw secret).
    CopyDeviceCode,
    /// Switch to a device-code poll instead of the current path.
    SwitchToDeviceCode,
    /// Use the local fallback when the preferred boundary is unavailable.
    UseLocalFallback,
    /// View why a safer boundary is preferred.
    ViewSaferBoundaryRationale,
    /// Cancel the handoff.
    Cancel,
}

impl BrowserDeviceCodeHandoffAction {
    /// Every handoff action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ContinuePreferredBoundary,
        Self::CopyDeviceCode,
        Self::SwitchToDeviceCode,
        Self::UseLocalFallback,
        Self::ViewSaferBoundaryRationale,
        Self::Cancel,
    ];

    /// The continue / cancel semantics every keyboard-complete handoff card must offer.
    pub const MANDATORY: [Self; 2] = [Self::ContinuePreferredBoundary, Self::Cancel];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinuePreferredBoundary => "continue_preferred_boundary",
            Self::CopyDeviceCode => "copy_device_code",
            Self::SwitchToDeviceCode => "switch_to_device_code",
            Self::UseLocalFallback => "use_local_fallback",
            Self::ViewSaferBoundaryRationale => "view_safer_boundary_rationale",
            Self::Cancel => "cancel",
        }
    }
}

/// Disclosures a browser-or-device-code handoff card must carry, derived from the
/// auth-handoff class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandoffBoundaryDisclosure {
    /// The derived handoff-boundary class this card may present.
    pub handoff_boundary_class: HandoffBoundaryClass,
    /// Whether the boundary is an out-of-app system-browser redirect.
    pub is_out_of_app_system_browser: bool,
    /// Whether the boundary is an in-app / local capture (a safer boundary is preferred).
    pub is_local_capture: bool,
    /// Whether the card must carry an explicit system-browser note.
    pub needs_system_browser_note: bool,
    /// Whether the card must carry an explicit device-code / expiry note.
    pub needs_device_code_note: bool,
    /// Whether the card must disclose that a safer boundary is preferred over local capture.
    pub needs_local_capture_disclosure_note: bool,
    /// Whether the card must carry an explicit delegated / deferred note.
    pub needs_delegated_deferred_note: bool,
}

/// Resolves the handoff-boundary truth a browser-or-device-code handoff card may present.
///
/// A system-browser redirect is an out-of-app boundary. A device-code poll is a
/// secondary-device boundary that must name its code / expiry. An embedded prompt or a
/// passkey step-up is an in-app local capture that must disclose why a safer boundary is
/// preferred. A delegated-forward or offline-deferred handoff is neither, and must say
/// so — none of which can ever blur into one generic sign-in state.
pub fn resolve_handoff_boundary(
    auth_handoff_class: M5AuthHandoffClass,
) -> HandoffBoundaryDisclosure {
    use HandoffBoundaryClass as Boundary;
    use M5AuthHandoffClass as Handoff;

    let handoff_boundary_class = match auth_handoff_class {
        Handoff::SystemBrowserRedirect => Boundary::SystemBrowserBoundary,
        Handoff::DeviceCodePoll => Boundary::DeviceCodeBoundary,
        Handoff::EmbeddedPrompt | Handoff::PasskeyStepUp => Boundary::LocalCaptureBoundary,
        Handoff::DelegatedForward | Handoff::OfflineDeferred => {
            Boundary::DelegatedOrDeferredBoundary
        }
    };

    HandoffBoundaryDisclosure {
        handoff_boundary_class,
        is_out_of_app_system_browser: matches!(
            handoff_boundary_class,
            Boundary::SystemBrowserBoundary
        ),
        is_local_capture: matches!(handoff_boundary_class, Boundary::LocalCaptureBoundary),
        needs_system_browser_note: matches!(
            handoff_boundary_class,
            Boundary::SystemBrowserBoundary
        ),
        needs_device_code_note: matches!(handoff_boundary_class, Boundary::DeviceCodeBoundary),
        needs_local_capture_disclosure_note: matches!(
            handoff_boundary_class,
            Boundary::LocalCaptureBoundary
        ),
        needs_delegated_deferred_note: matches!(
            handoff_boundary_class,
            Boundary::DelegatedOrDeferredBoundary
        ),
    }
}

/// A browser-or-device-code handoff card naming provider / org, auth-handoff flow kind,
/// derived handoff boundary, fallback state, local continuity, device code / expiry, and
/// why a safer boundary is preferred.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserDeviceCodeHandoffCard {
    /// Frozen component this control implements; must be `browser_device_code_handoff_card`.
    pub component: M5CredentialComponentFamily,
    /// Stable card id.
    pub card_id: String,
    /// Provider / org this handoff names; required and non-empty.
    pub provider_org_label: String,
    /// The auth-handoff flow kind, reused from the frozen matrix.
    pub auth_handoff_class: M5AuthHandoffClass,
    /// Human-readable flow-kind label; required and non-empty (never a generic "sign in").
    pub flow_kind_label: String,
    /// Credential class this handoff obtains, reused from the frozen matrix.
    pub credential_class: M5CredentialClass,
    /// Reveal posture behind this handoff, reused from the frozen matrix.
    pub reveal_posture: M5CredentialRevealPosture,
    /// Derived handoff-boundary class (must equal the resolved class).
    pub handoff_boundary_class: HandoffBoundaryClass,
    /// Whether the card claims an out-of-app system-browser boundary (must equal the derived truth).
    pub claims_out_of_app_boundary: bool,
    /// System-browser note; required when the boundary is a system-browser redirect.
    pub system_browser_note: String,
    /// Device-code / expiry note; required when the boundary is a device-code poll.
    pub device_code_note: String,
    /// Local-capture disclosure note; required when the boundary is an in-app local capture.
    pub local_capture_disclosure_note: String,
    /// Delegated / deferred note; required when the boundary is delegated-forward or offline-deferred.
    pub delegated_deferred_note: String,
    /// Fallback-state note; always required so the fallback state stays explicit.
    pub fallback_state_note: String,
    /// Local-continuity note / what still works locally; always required.
    pub local_continuity_note: String,
    /// Safer-boundary rationale / why a safer boundary is preferred; always required.
    pub safer_boundary_rationale_note: String,
    /// Storage-mode / reveal-posture note; always required so storage stays explicit.
    pub storage_and_reveal_note: String,
    /// Keyboard-complete default actions (must include the mandatory continue/cancel).
    pub default_actions: Vec<BrowserDeviceCodeHandoffAction>,
    /// Degraded states this card can name (required, matching the frozen matrix).
    pub degraded_states: Vec<M5CredentialDegradedState>,
    /// Mandatory labels this card can show (must include the mandatory labels).
    pub required_labels: Vec<M5CredentialRequiredLabel>,
    /// Claimed M5 surface families that render this card.
    pub surface_families: Vec<M5CredentialSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5CredentialDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5CredentialAccessibilityRoute>,
    /// Credential subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks storage mode or reveal posture. MUST be `false`.
    pub masks_storage_or_reveal_posture: bool,
    /// Hard invariant: never blurs system-browser / device-code / local capture into one
    /// generic sign-in state. MUST be `false`.
    pub blurs_handoff_into_generic_sign_in: bool,
    /// Hard invariant: friendly "connected" / "signed in" wording never conceals the
    /// handoff boundary. MUST be `false`.
    pub uses_friendly_connected_wording: bool,
}

impl BrowserDeviceCodeHandoffCard {
    /// Handoff-boundary disclosures this card must carry, derived from the auth-handoff class.
    pub fn handoff_boundary_disclosure(&self) -> HandoffBoundaryDisclosure {
        resolve_handoff_boundary(self.auth_handoff_class)
    }

    /// Whether the card offers every mandatory keyboard-complete default action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<BrowserDeviceCodeHandoffAction> =
            self.default_actions.iter().copied().collect();
        BrowserDeviceCodeHandoffAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the card declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CredentialRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CredentialRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }
}

// ---- delegated-credential-row vocabulary --------------------------------

/// Derived identity origin a delegated-credential row may present.
///
/// This is the delegated honesty axis: the origin is derived from the delegated-identity
/// state and the storage mode, never asserted, so a forwarded, a remote-vault-held, or a
/// service-issued identity can never read as a locally stored credential and delegated /
/// forwarded identity always stays visually distinct from locally stored credentials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DelegatedIdentityOrigin {
    /// A locally stored credential acting as the local identity.
    LocallyStored,
    /// A forwarded or delegated identity acting on behalf of another principal.
    Forwarded,
    /// An identity whose material is held in a remote vault / broker.
    RemoteVault,
    /// A service-issued identity (a service account).
    ServiceIssued,
}

impl DelegatedIdentityOrigin {
    /// Every identity origin, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocallyStored,
        Self::Forwarded,
        Self::RemoteVault,
        Self::ServiceIssued,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocallyStored => "locally_stored",
            Self::Forwarded => "forwarded",
            Self::RemoteVault => "remote_vault",
            Self::ServiceIssued => "service_issued",
        }
    }
}

/// Target scope a delegated-credential row names as the boundary the delegation grants
/// access to, so a delegation never leaves what it can reach implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DelegatedTargetScope {
    /// The provider / VCS-host boundary.
    Provider,
    /// The package / artifact registry boundary.
    Registry,
    /// A request / API boundary.
    Request,
    /// A database boundary.
    Database,
    /// A remote-target boundary.
    Remote,
    /// The package / release publish boundary.
    Release,
}

impl DelegatedTargetScope {
    /// Every target scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Provider,
        Self::Registry,
        Self::Request,
        Self::Database,
        Self::Remote,
        Self::Release,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provider => "provider",
            Self::Registry => "registry",
            Self::Request => "request",
            Self::Database => "database",
            Self::Remote => "remote",
            Self::Release => "release",
        }
    }
}

/// One keyboard-complete default action a delegated-credential row offers, so a row never
/// hides its stop-forward or rotate affordance behind a pointer-only gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DelegatedCredentialRowAction {
    /// Stop forwarding this identity.
    StopForward,
    /// Rotate the delegated credential.
    Rotate,
    /// Revoke the delegation entirely.
    RevokeDelegation,
    /// View the policy owner responsible for this delegation.
    ViewPolicyOwner,
    /// Open the store of record (the source of truth) to inspect it.
    OpenSourceOfTruth,
    /// Export the row as export-safe delegation evidence.
    ExportRow,
}

impl DelegatedCredentialRowAction {
    /// Every row action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StopForward,
        Self::Rotate,
        Self::RevokeDelegation,
        Self::ViewPolicyOwner,
        Self::OpenSourceOfTruth,
        Self::ExportRow,
    ];

    /// The stop-forward / rotate actions every keyboard-complete delegated row must offer.
    pub const MANDATORY: [Self; 2] = [Self::StopForward, Self::Rotate];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StopForward => "stop_forward",
            Self::Rotate => "rotate",
            Self::RevokeDelegation => "revoke_delegation",
            Self::ViewPolicyOwner => "view_policy_owner",
            Self::OpenSourceOfTruth => "open_source_of_truth",
            Self::ExportRow => "export_row",
        }
    }
}

/// Disclosures a delegated-credential row must carry, derived from the delegated-identity
/// state and the storage mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DelegatedIdentityDisclosure {
    /// The derived identity origin this row may present.
    pub identity_origin: DelegatedIdentityOrigin,
    /// Whether the identity is a locally stored credential.
    pub is_locally_stored: bool,
    /// Whether the identity is forwarded, remote-vault-held, or service-issued (never local).
    pub is_forwarded_or_delegated: bool,
    /// Whether the row must carry an explicit forwarded note.
    pub needs_forwarded_note: bool,
    /// Whether the row must carry an explicit remote-vault note.
    pub needs_remote_vault_note: bool,
    /// Whether the row must carry an explicit service-issued note.
    pub needs_service_issued_note: bool,
    /// Whether the row must carry an explicit revoked-delegation note.
    pub needs_revoked_note: bool,
}

/// Resolves the identity-origin truth a delegated-credential row may present.
///
/// A service-account identity is service-issued. An identity whose material is held as a
/// broker handle or an external reference is remote-vault-held. A forwarded, delegated,
/// impersonation-scoped, or revoked-delegation identity is forwarded. Only a local
/// identity backed by locally held storage is locally stored — so a forwarded,
/// remote-vault-held, or service-issued identity can never claim to be locally stored.
pub fn resolve_delegated_identity_origin(
    identity_state: M5DelegatedIdentityState,
    storage_mode: M5CredentialStorageMode,
) -> DelegatedIdentityDisclosure {
    use DelegatedIdentityOrigin as Origin;
    use M5CredentialStorageMode as Storage;
    use M5DelegatedIdentityState as Identity;

    let remote_vault_backed = matches!(
        storage_mode,
        Storage::SecretBrokerHandle | Storage::ExternalReference
    );
    let forwarded_state = matches!(
        identity_state,
        Identity::ForwardedIdentity
            | Identity::DelegatedOnBehalf
            | Identity::ImpersonationScoped
            | Identity::DelegationRevoked
    );

    let identity_origin = if matches!(identity_state, Identity::ServiceAccountActing) {
        Origin::ServiceIssued
    } else if remote_vault_backed {
        Origin::RemoteVault
    } else if forwarded_state {
        Origin::Forwarded
    } else {
        Origin::LocallyStored
    };

    DelegatedIdentityDisclosure {
        identity_origin,
        is_locally_stored: matches!(identity_origin, Origin::LocallyStored),
        is_forwarded_or_delegated: !matches!(identity_origin, Origin::LocallyStored),
        needs_forwarded_note: matches!(identity_origin, Origin::Forwarded),
        needs_remote_vault_note: matches!(identity_origin, Origin::RemoteVault),
        needs_service_issued_note: matches!(identity_origin, Origin::ServiceIssued),
        needs_revoked_note: matches!(identity_state, Identity::DelegationRevoked),
    }
}

/// A delegated-credential row naming source identity, target scope, storage class,
/// expiration, policy owner, and derived local-versus-forwarded-versus-remote-vault-
/// versus-service-issued identity origin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DelegatedCredentialRow {
    /// Frozen component this control implements; must be `delegated_credential_row`.
    pub component: M5CredentialComponentFamily,
    /// Stable row id.
    pub row_id: String,
    /// Delegated-identity state (source identity), reused from the frozen matrix.
    pub delegated_identity_state: M5DelegatedIdentityState,
    /// Source-identity label / who the credential acts as; required and non-empty.
    pub source_identity_label: String,
    /// Target scope this delegation grants access to.
    pub target_scope: DelegatedTargetScope,
    /// Target-scope note; always required so the delegated boundary stays explicit.
    pub target_scope_note: String,
    /// Storage class (where the credential material lives), reused from the frozen matrix.
    pub storage_mode: M5CredentialStorageMode,
    /// Lifecycle state (expiry / refresh / rotation / revoke), reused from the frozen matrix.
    pub lifecycle_state: M5CredentialLifecycleState,
    /// Expiration note; always required so the expiration stays explicit.
    pub expiration_note: String,
    /// Policy-owner label / who owns this delegation policy; required and non-empty.
    pub policy_owner_label: String,
    /// Derived identity origin (must equal the resolved origin).
    pub identity_origin: DelegatedIdentityOrigin,
    /// Whether the row claims the credential is locally stored (must equal the derived truth).
    pub claims_locally_stored: bool,
    /// Forwarded note; required when the identity is forwarded / delegated.
    pub forwarded_note: String,
    /// Remote-vault note; required when the identity material is held in a remote vault.
    pub remote_vault_note: String,
    /// Service-issued note; required when the identity is a service account.
    pub service_issued_note: String,
    /// Revoked-delegation note; required when the delegation has been revoked.
    pub revoked_note: String,
    /// Storage-class note; always required so the storage class stays explicit.
    pub storage_class_note: String,
    /// Identity / delegation note; always required so the acting identity stays explicit.
    pub identity_and_delegation_note: String,
    /// Keyboard-complete default actions (must include the mandatory stop-forward/rotate).
    pub default_actions: Vec<DelegatedCredentialRowAction>,
    /// Degraded states this row can name (required, matching the frozen matrix).
    pub degraded_states: Vec<M5CredentialDegradedState>,
    /// Mandatory labels this row can show (must include the mandatory labels).
    pub required_labels: Vec<M5CredentialRequiredLabel>,
    /// Claimed M5 surface families that render this row.
    pub surface_families: Vec<M5CredentialSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5CredentialDeploymentLine>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5CredentialAccessibilityRoute>,
    /// Credential subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks a forwarded or delegated identity as locally stored.
    /// MUST be `false`.
    pub masks_forwarded_or_delegated_identity: bool,
    /// Hard invariant: never implies a raw secret is export-safe. MUST be `false`.
    pub implies_raw_secret_exportable: bool,
    /// Hard invariant: friendly "connected" / "signed in" wording never conceals a
    /// forwarded or delegated identity. MUST be `false`.
    pub uses_friendly_connected_wording: bool,
}

impl DelegatedCredentialRow {
    /// Identity-origin disclosures this row must carry, derived from state and storage.
    pub fn identity_disclosure(&self) -> DelegatedIdentityDisclosure {
        resolve_delegated_identity_origin(self.delegated_identity_state, self.storage_mode)
    }

    /// Whether the row offers every mandatory keyboard-complete default action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<DelegatedCredentialRowAction> =
            self.default_actions.iter().copied().collect();
        DelegatedCredentialRowAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CredentialRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CredentialRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }
}

// ---- review blocks ------------------------------------------------------

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffDelegatedCredentialTrustReview {
    /// The handoff card names its provider / org and its auth-handoff flow kind.
    pub card_shows_provider_org_and_flow_kind: bool,
    /// Handoff flows never blur into one generic sign-in state.
    pub handoff_flows_never_blur_into_generic_sign_in: bool,
    /// System-browser, device-code, and local capture stay distinct.
    pub system_browser_device_code_local_capture_stay_distinct: bool,
    /// The safer-boundary rationale is always shown.
    pub safer_boundary_rationale_always_shown: bool,
    /// The card names its fallback state and local continuity.
    pub card_shows_fallback_state_and_local_continuity: bool,
    /// The device code and expiry are shown wherever a device-code poll is relevant.
    pub device_code_and_expiry_shown_where_relevant: bool,
    /// The delegated row names its source identity and target scope.
    pub row_shows_source_identity_and_target_scope: bool,
    /// The delegated row names its storage class and expiration.
    pub row_shows_storage_class_and_expiration: bool,
    /// The identity origin is derived from state / storage, never asserted.
    pub identity_origin_derived_never_asserted: bool,
    /// A forwarded or delegated identity never reads as a locally stored credential.
    pub forwarded_delegated_never_reads_as_local: bool,
    /// Remote-vault and service-issued identity stay distinct from locally stored.
    pub remote_vault_and_service_issued_stay_distinct: bool,
    /// The policy owner and stop-forward / rotate actions are always present.
    pub policy_owner_and_stop_forward_rotate_present: bool,
    /// Raw-secret handling is never normalized on any surface.
    pub raw_secret_handling_never_normalized: bool,
    /// No friendly "connected" wording conceals the handoff boundary or delegated identity.
    pub no_friendly_connected_wording: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl BrowserHandoffDelegatedCredentialTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.card_shows_provider_org_and_flow_kind
            && self.handoff_flows_never_blur_into_generic_sign_in
            && self.system_browser_device_code_local_capture_stay_distinct
            && self.safer_boundary_rationale_always_shown
            && self.card_shows_fallback_state_and_local_continuity
            && self.device_code_and_expiry_shown_where_relevant
            && self.row_shows_source_identity_and_target_scope
            && self.row_shows_storage_class_and_expiration
            && self.identity_origin_derived_never_asserted
            && self.forwarded_delegated_never_reads_as_local
            && self.remote_vault_and_service_issued_stay_distinct
            && self.policy_owner_and_stop_forward_rotate_present
            && self.raw_secret_handling_never_normalized
            && self.no_friendly_connected_wording
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffDelegatedCredentialConsumerProjection {
    /// The handoff card shows its flow kind and boundary without docs.
    pub card_shows_flow_kind_and_boundary_without_docs: bool,
    /// The safer boundary is visible before any local capture.
    pub safer_boundary_visible_before_local_capture: bool,
    /// The delegated row shows its identity origin and scope inline.
    pub row_shows_identity_origin_and_scope_inline: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_control_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl BrowserHandoffDelegatedCredentialConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.card_shows_flow_kind_and_boundary_without_docs
            && self.safer_boundary_visible_before_local_capture
            && self.row_shows_identity_origin_and_scope_inline
            && self.cli_headless_shows_control_truth
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffDelegatedCredentialProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserDeviceCodeHandoffDelegatedCredentialControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Browser-or-device-code handoff cards.
    pub handoff_cards: Vec<BrowserDeviceCodeHandoffCard>,
    /// Delegated-credential rows.
    pub delegated_rows: Vec<DelegatedCredentialRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CredentialDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Trust review block.
    pub trust_review: BrowserHandoffDelegatedCredentialTrustReview,
    /// Consumer projection block.
    pub consumer_projection: BrowserHandoffDelegatedCredentialConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: BrowserHandoffDelegatedCredentialProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe browser-handoff / delegated-credential controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket {
    /// Record kind; must equal [`BROWSER_HANDOFF_DELEGATED_CREDENTIAL_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Browser-or-device-code handoff cards.
    pub handoff_cards: Vec<BrowserDeviceCodeHandoffCard>,
    /// Delegated-credential rows.
    pub delegated_rows: Vec<DelegatedCredentialRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CredentialDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Trust review block.
    pub trust_review: BrowserHandoffDelegatedCredentialTrustReview,
    /// Consumer projection block.
    pub consumer_projection: BrowserHandoffDelegatedCredentialConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: BrowserHandoffDelegatedCredentialProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket {
    /// Builds a browser-handoff / delegated-credential controls packet from stable-lane input.
    pub fn new(input: BrowserDeviceCodeHandoffDelegatedCredentialControlsPacketInput) -> Self {
        Self {
            record_kind: BROWSER_HANDOFF_DELEGATED_CREDENTIAL_RECORD_KIND.to_owned(),
            schema_version: BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            handoff_cards: input.handoff_cards,
            delegated_rows: input.delegated_rows,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the browser-handoff / delegated-credential control invariants.
    pub fn validate(&self) -> Vec<BrowserHandoffDelegatedCredentialViolation> {
        let mut violations = Vec::new();

        if self.record_kind != BROWSER_HANDOFF_DELEGATED_CREDENTIAL_RECORD_KIND {
            violations.push(BrowserHandoffDelegatedCredentialViolation::WrongRecordKind);
        }
        if self.schema_version != BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_VERSION {
            violations.push(BrowserHandoffDelegatedCredentialViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(BrowserHandoffDelegatedCredentialViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_handoff_cards(self, &mut violations);
        validate_delegated_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(BrowserHandoffDelegatedCredentialViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("browser handoff delegated credential packet serializes"),
        ) {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("browser handoff delegated credential packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "control,id,kind_or_identity,scope_or_class,state,derived,distinct_or_local\n",
        );
        for card in &self.handoff_cards {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "browser_device_code_handoff_card",
                csv_field(&card.card_id),
                card.auth_handoff_class.as_str(),
                card.credential_class.as_str(),
                card.reveal_posture.as_str(),
                card.handoff_boundary_disclosure()
                    .handoff_boundary_class
                    .as_str(),
                card.handoff_boundary_disclosure()
                    .is_out_of_app_system_browser,
            ));
        }
        for row in &self.delegated_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "delegated_credential_row",
                csv_field(&row.row_id),
                row.delegated_identity_state.as_str(),
                row.target_scope.as_str(),
                row.lifecycle_state.as_str(),
                row.identity_disclosure().identity_origin.as_str(),
                row.identity_disclosure().is_locally_stored,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let local_capture = self
            .handoff_cards
            .iter()
            .filter(|card| card.handoff_boundary_disclosure().is_local_capture)
            .count();
        let not_local = self
            .delegated_rows
            .iter()
            .filter(|row| !row.identity_disclosure().is_locally_stored)
            .count();

        let mut out = String::new();
        out.push_str("# Browser-or-device-code handoff cards and delegated-credential rows\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Handoff cards: {} ({} are in-app local captures)\n",
            self.handoff_cards.len(),
            local_capture
        ));
        out.push_str(&format!(
            "- Delegated-credential rows: {} ({} are not locally stored)\n",
            self.delegated_rows.len(),
            not_local
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Browser-or-device-code handoff cards\n\n");
        for card in &self.handoff_cards {
            out.push_str(&format!(
                "- **{}** ({}) — flow `{}` → boundary `{}`\n",
                card.provider_org_label,
                card.flow_kind_label,
                card.auth_handoff_class.as_str(),
                card.handoff_boundary_disclosure()
                    .handoff_boundary_class
                    .as_str(),
            ));
        }

        out.push_str("\n## Delegated-credential rows\n\n");
        for row in &self.delegated_rows {
            out.push_str(&format!(
                "- **{}** — state `{}`, scope `{}`, storage `{}` → origin `{}`\n",
                row.source_identity_label,
                row.delegated_identity_state.as_str(),
                row.target_scope.as_str(),
                row.storage_mode.as_str(),
                row.identity_disclosure().identity_origin.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in browser-handoff / delegated-credential export.
#[derive(Debug)]
pub enum BrowserHandoffDelegatedCredentialArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BrowserHandoffDelegatedCredentialViolation>),
}

impl fmt::Display for BrowserHandoffDelegatedCredentialArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "browser handoff delegated credential export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "browser handoff delegated credential export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for BrowserHandoffDelegatedCredentialArtifactError {}

/// Validation failures emitted by
/// [`BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrowserHandoffDelegatedCredentialViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No browser-or-device-code handoff cards are present.
    HandoffCardsMissing,
    /// A handoff card is incomplete.
    HandoffCardIncomplete,
    /// A handoff card carries the wrong frozen component class.
    HandoffCardWrongComponentClass,
    /// A handoff card does not name its provider / org or its flow kind.
    ProviderOrgOrFlowKindMissing,
    /// A handoff card misrepresents its derived handoff-boundary class.
    HandoffBoundaryMisrepresented,
    /// A system-browser handoff card does not name its system-browser boundary.
    SystemBrowserNoteMissing,
    /// A device-code handoff card does not name its code / expiry.
    DeviceCodeNoteMissing,
    /// A local-capture handoff card does not disclose why a safer boundary is preferred.
    LocalCaptureDisclosureMissing,
    /// A delegated / deferred handoff card does not name its delegated / deferred state.
    DelegatedDeferredNoteMissing,
    /// A handoff card does not name its fallback state.
    FallbackStateNoteMissing,
    /// A handoff card does not name its local continuity.
    LocalContinuityNoteMissing,
    /// A handoff card does not name why a safer boundary is preferred.
    SaferBoundaryRationaleMissing,
    /// A handoff card does not name its storage mode / reveal posture.
    StorageAndRevealNoteMissing,
    /// A handoff card omits a mandatory continue / cancel action.
    HandoffActionsIncomplete,
    /// The handoff cards do not cover every auth-handoff class.
    AuthHandoffClassCoverageMissing,
    /// The handoff cards do not cover every handoff-boundary class.
    HandoffBoundaryCoverageMissing,
    /// A handoff card blurs system-browser / device-code / local capture into generic sign-in.
    HandoffBlurredIntoGenericSignIn,
    /// No delegated-credential rows are present.
    DelegatedRowsMissing,
    /// A delegated-credential row is incomplete.
    DelegatedRowIncomplete,
    /// A delegated-credential row carries the wrong frozen component class.
    DelegatedRowWrongComponentClass,
    /// A delegated-credential row does not name its source identity or target scope.
    SourceIdentityOrScopeMissing,
    /// A delegated-credential row misrepresents its derived identity origin.
    DelegatedIdentityMisrepresented,
    /// A forwarded / delegated row does not name its forwarded identity.
    ForwardedNoteMissing,
    /// A remote-vault row does not name its remote-vault storage.
    RemoteVaultNoteMissing,
    /// A service-issued row does not name its service-issued identity.
    ServiceIssuedNoteMissing,
    /// A revoked-delegation row does not name its revoked state.
    RevokedNoteMissing,
    /// A delegated-credential row does not name its expiration.
    ExpirationNoteMissing,
    /// A delegated-credential row does not name its policy owner.
    PolicyOwnerMissing,
    /// A delegated-credential row does not name its storage class.
    StorageClassNoteMissing,
    /// A delegated-credential row does not name its identity / delegation.
    IdentityAndDelegationNoteMissing,
    /// A delegated-credential row omits a mandatory stop-forward / rotate action.
    DelegatedActionsIncomplete,
    /// The delegated-credential rows do not cover every delegated-identity state.
    DelegatedIdentityStateCoverageMissing,
    /// The delegated-credential rows do not cover every identity origin.
    DelegatedIdentityOriginCoverageMissing,
    /// A delegated row masks a forwarded or delegated identity as locally stored.
    ForwardedOrDelegatedIdentityMasked,
    /// A control does not declare its degraded states.
    DegradedStatesMissing,
    /// A control does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A control does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A control masks its storage mode or reveal posture.
    StorageOrRevealMasked,
    /// A control implies a raw secret is export-safe (normalizes raw-secret handling).
    RawSecretHandlingNormalized,
    /// A control uses friendly "connected" wording that conceals a boundary or identity.
    FriendlyConnectedWordingUsed,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl BrowserHandoffDelegatedCredentialViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::HandoffCardsMissing => "handoff_cards_missing",
            Self::HandoffCardIncomplete => "handoff_card_incomplete",
            Self::HandoffCardWrongComponentClass => "handoff_card_wrong_component_class",
            Self::ProviderOrgOrFlowKindMissing => "provider_org_or_flow_kind_missing",
            Self::HandoffBoundaryMisrepresented => "handoff_boundary_misrepresented",
            Self::SystemBrowserNoteMissing => "system_browser_note_missing",
            Self::DeviceCodeNoteMissing => "device_code_note_missing",
            Self::LocalCaptureDisclosureMissing => "local_capture_disclosure_missing",
            Self::DelegatedDeferredNoteMissing => "delegated_deferred_note_missing",
            Self::FallbackStateNoteMissing => "fallback_state_note_missing",
            Self::LocalContinuityNoteMissing => "local_continuity_note_missing",
            Self::SaferBoundaryRationaleMissing => "safer_boundary_rationale_missing",
            Self::StorageAndRevealNoteMissing => "storage_and_reveal_note_missing",
            Self::HandoffActionsIncomplete => "handoff_actions_incomplete",
            Self::AuthHandoffClassCoverageMissing => "auth_handoff_class_coverage_missing",
            Self::HandoffBoundaryCoverageMissing => "handoff_boundary_coverage_missing",
            Self::HandoffBlurredIntoGenericSignIn => "handoff_blurred_into_generic_sign_in",
            Self::DelegatedRowsMissing => "delegated_rows_missing",
            Self::DelegatedRowIncomplete => "delegated_row_incomplete",
            Self::DelegatedRowWrongComponentClass => "delegated_row_wrong_component_class",
            Self::SourceIdentityOrScopeMissing => "source_identity_or_scope_missing",
            Self::DelegatedIdentityMisrepresented => "delegated_identity_misrepresented",
            Self::ForwardedNoteMissing => "forwarded_note_missing",
            Self::RemoteVaultNoteMissing => "remote_vault_note_missing",
            Self::ServiceIssuedNoteMissing => "service_issued_note_missing",
            Self::RevokedNoteMissing => "revoked_note_missing",
            Self::ExpirationNoteMissing => "expiration_note_missing",
            Self::PolicyOwnerMissing => "policy_owner_missing",
            Self::StorageClassNoteMissing => "storage_class_note_missing",
            Self::IdentityAndDelegationNoteMissing => "identity_and_delegation_note_missing",
            Self::DelegatedActionsIncomplete => "delegated_actions_incomplete",
            Self::DelegatedIdentityStateCoverageMissing => {
                "delegated_identity_state_coverage_missing"
            }
            Self::DelegatedIdentityOriginCoverageMissing => {
                "delegated_identity_origin_coverage_missing"
            }
            Self::ForwardedOrDelegatedIdentityMasked => "forwarded_or_delegated_identity_masked",
            Self::DegradedStatesMissing => "degraded_states_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::StorageOrRevealMasked => "storage_or_reveal_masked",
            Self::RawSecretHandlingNormalized => "raw_secret_handling_normalized",
            Self::FriendlyConnectedWordingUsed => "friendly_connected_wording_used",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable browser-handoff / delegated-credential export.
pub fn current_browser_handoff_delegated_credential_export() -> Result<
    BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket,
    BrowserHandoffDelegatedCredentialArtifactError,
> {
    let packet: BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-browser-device-code-handoff-delegated-credential-proof/support_export.json"
        )))
        .map_err(BrowserHandoffDelegatedCredentialArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BrowserHandoffDelegatedCredentialArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket,
    violations: &mut Vec<BrowserHandoffDelegatedCredentialViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_REF,
        BROWSER_HANDOFF_DELEGATED_CREDENTIAL_DOC_REF,
        M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_DOC_REF,
        M5_BROWSER_DEVICE_CODE_HANDOFF_CARD_SCHEMA_REF,
        M5_DELEGATED_CREDENTIAL_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(BrowserHandoffDelegatedCredentialViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_handoff_cards(
    packet: &BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket,
    violations: &mut Vec<BrowserHandoffDelegatedCredentialViolation>,
) {
    if packet.handoff_cards.is_empty() {
        violations.push(BrowserHandoffDelegatedCredentialViolation::HandoffCardsMissing);
        return;
    }

    let mut handoff_classes: BTreeSet<M5AuthHandoffClass> = BTreeSet::new();
    let mut boundary_classes: BTreeSet<HandoffBoundaryClass> = BTreeSet::new();

    for card in &packet.handoff_cards {
        let disclosure = card.handoff_boundary_disclosure();
        handoff_classes.insert(card.auth_handoff_class);
        boundary_classes.insert(disclosure.handoff_boundary_class);

        if card.card_id.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(BrowserHandoffDelegatedCredentialViolation::HandoffCardIncomplete);
        }
        if card.component != M5CredentialComponentFamily::BrowserDeviceCodeHandoffCard {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::HandoffCardWrongComponentClass);
        }
        if card.provider_org_label.trim().is_empty() || card.flow_kind_label.trim().is_empty() {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::ProviderOrgOrFlowKindMissing);
        }
        if card.handoff_boundary_class != disclosure.handoff_boundary_class
            || card.claims_out_of_app_boundary != disclosure.is_out_of_app_system_browser
        {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::HandoffBoundaryMisrepresented);
        }
        if disclosure.needs_system_browser_note && card.system_browser_note.trim().is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::SystemBrowserNoteMissing);
        }
        if disclosure.needs_device_code_note && card.device_code_note.trim().is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::DeviceCodeNoteMissing);
        }
        if disclosure.needs_local_capture_disclosure_note
            && card.local_capture_disclosure_note.trim().is_empty()
        {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::LocalCaptureDisclosureMissing);
        }
        if disclosure.needs_delegated_deferred_note
            && card.delegated_deferred_note.trim().is_empty()
        {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::DelegatedDeferredNoteMissing);
        }
        if card.fallback_state_note.trim().is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::FallbackStateNoteMissing);
        }
        if card.local_continuity_note.trim().is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::LocalContinuityNoteMissing);
        }
        if card.safer_boundary_rationale_note.trim().is_empty() {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::SaferBoundaryRationaleMissing);
        }
        if card.storage_and_reveal_note.trim().is_empty() {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::StorageAndRevealNoteMissing);
        }
        if !card.declares_mandatory_actions() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::HandoffActionsIncomplete);
        }
        if card.degraded_states.is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::DegradedStatesMissing);
        }
        if !card.declares_mandatory_labels() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::RequiredLabelsIncomplete);
        }
        if card.accessibility_routes.is_empty()
            || !card
                .accessibility_routes
                .contains(&M5CredentialAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(BrowserHandoffDelegatedCredentialViolation::AccessibilityRouteMissing);
        }
        if card.masks_storage_or_reveal_posture {
            violations.push(BrowserHandoffDelegatedCredentialViolation::StorageOrRevealMasked);
        }
        if card.blurs_handoff_into_generic_sign_in {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::HandoffBlurredIntoGenericSignIn);
        }
        if card.uses_friendly_connected_wording {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::FriendlyConnectedWordingUsed);
        }
    }

    for required in M5AuthHandoffClass::ALL {
        if !handoff_classes.contains(&required) {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::AuthHandoffClassCoverageMissing);
            break;
        }
    }
    for required in HandoffBoundaryClass::ALL {
        if !boundary_classes.contains(&required) {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::HandoffBoundaryCoverageMissing);
            break;
        }
    }
}

fn validate_delegated_rows(
    packet: &BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket,
    violations: &mut Vec<BrowserHandoffDelegatedCredentialViolation>,
) {
    if packet.delegated_rows.is_empty() {
        violations.push(BrowserHandoffDelegatedCredentialViolation::DelegatedRowsMissing);
        return;
    }

    let mut identity_states: BTreeSet<M5DelegatedIdentityState> = BTreeSet::new();
    let mut identity_origins: BTreeSet<DelegatedIdentityOrigin> = BTreeSet::new();

    for row in &packet.delegated_rows {
        let disclosure = row.identity_disclosure();
        identity_states.insert(row.delegated_identity_state);
        identity_origins.insert(disclosure.identity_origin);

        if row.row_id.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.consumer_surfaces.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(BrowserHandoffDelegatedCredentialViolation::DelegatedRowIncomplete);
        }
        if row.component != M5CredentialComponentFamily::DelegatedCredentialRow {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::DelegatedRowWrongComponentClass);
        }
        if row.source_identity_label.trim().is_empty() || row.target_scope_note.trim().is_empty() {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::SourceIdentityOrScopeMissing);
        }
        if row.identity_origin != disclosure.identity_origin
            || row.claims_locally_stored != disclosure.is_locally_stored
        {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::DelegatedIdentityMisrepresented);
        }
        if disclosure.needs_forwarded_note && row.forwarded_note.trim().is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::ForwardedNoteMissing);
        }
        if disclosure.needs_remote_vault_note && row.remote_vault_note.trim().is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::RemoteVaultNoteMissing);
        }
        if disclosure.needs_service_issued_note && row.service_issued_note.trim().is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::ServiceIssuedNoteMissing);
        }
        if disclosure.needs_revoked_note && row.revoked_note.trim().is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::RevokedNoteMissing);
        }
        if row.expiration_note.trim().is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::ExpirationNoteMissing);
        }
        if row.policy_owner_label.trim().is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::PolicyOwnerMissing);
        }
        if row.storage_class_note.trim().is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::StorageClassNoteMissing);
        }
        if row.identity_and_delegation_note.trim().is_empty() {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::IdentityAndDelegationNoteMissing);
        }
        if !row.declares_mandatory_actions() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::DelegatedActionsIncomplete);
        }
        if row.degraded_states.is_empty() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::DegradedStatesMissing);
        }
        if !row.declares_mandatory_labels() {
            violations.push(BrowserHandoffDelegatedCredentialViolation::RequiredLabelsIncomplete);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5CredentialAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(BrowserHandoffDelegatedCredentialViolation::AccessibilityRouteMissing);
        }
        if row.masks_forwarded_or_delegated_identity {
            violations.push(
                BrowserHandoffDelegatedCredentialViolation::ForwardedOrDelegatedIdentityMasked,
            );
        }
        if row.implies_raw_secret_exportable {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::RawSecretHandlingNormalized);
        }
        if row.uses_friendly_connected_wording {
            violations
                .push(BrowserHandoffDelegatedCredentialViolation::FriendlyConnectedWordingUsed);
        }
    }

    for required in M5DelegatedIdentityState::ALL {
        if !identity_states.contains(&required) {
            violations.push(
                BrowserHandoffDelegatedCredentialViolation::DelegatedIdentityStateCoverageMissing,
            );
            break;
        }
    }
    for required in DelegatedIdentityOrigin::ALL {
        if !identity_origins.contains(&required) {
            violations.push(
                BrowserHandoffDelegatedCredentialViolation::DelegatedIdentityOriginCoverageMissing,
            );
            break;
        }
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
///
/// The credential vocabulary uses the words "secret", "credential", and "api_key"
/// pervasively as governed tokens, so this check flags only raw-*value* shapes: a
/// password / passphrase literal, a bearer literal, a URL scheme, or a PEM header.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
