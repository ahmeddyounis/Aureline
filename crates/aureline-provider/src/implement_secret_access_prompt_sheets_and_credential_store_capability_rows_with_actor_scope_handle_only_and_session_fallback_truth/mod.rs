//! Secret-access prompt sheets and credential-store-capability rows carrying the
//! asking actor, the purpose, the requested scope, the raw-secret-versus-handle-only
//! posture, the retention note, the allow / deny / once semantics, what still works
//! if denied, and — for the store row — the store type, the verification state, the
//! portability / export posture, the platform limitations, and the session-only
//! fallback the current policy allows.
//!
//! This module narrows two components frozen in
//! [`crate::freeze_the_m5_credential_component_matrix`] — the
//! `secret_access_prompt_sheet` and the `credential_store_capability_row` — into one
//! implemented, export-safe packet with two co-equal control vectors. Together they
//! keep every request for secret access explicit about who is asking, what can be
//! avoided, and what the current store can actually guarantee.
//!
//! A [`SecretAccessPromptSheet`] always names the asking actor, the purpose, and the
//! requested scope, and its handle-availability class is *derived* from the reveal
//! posture rather than asserted: when a handle-only path exists a user sees it
//! surfaced instead of being nudged toward raw-secret sprawl, and a flow that
//! requests the raw secret can never quietly present as handle-only. It always names
//! its retention note, always offers keyboard-complete allow / deny / once actions,
//! and always names what still works if the request is denied.
//!
//! A [`CredentialStoreCapabilityRow`] always names the store type, the store
//! capabilities, and the verification state, and its trust class is *derived* from
//! the verification state and capabilities rather than asserted: an unverified,
//! verification-failed, or unsupported store can never read as "securely stored", so
//! a user never sees a vague "saved securely" message stand in for an unproven store.
//! It always names its portability / export posture and platform limitations, and it
//! names its session-only fallback whenever policy allows one.
//!
//! The storage modes ([`M5CredentialStorageMode`]), credential classes
//! ([`M5CredentialClass`]), reveal postures ([`M5CredentialRevealPosture`]), store
//! capabilities ([`M5CredentialStoreCapability`]), export-safety classes
//! ([`M5CredentialExportSafetyClass`]), degraded states
//! ([`M5CredentialDegradedState`]), required labels ([`M5CredentialRequiredLabel`]),
//! surface families ([`M5CredentialSurfaceFamily`]), deployment lines
//! ([`M5CredentialDeploymentLine`]), consumer surfaces
//! ([`M5CredentialConsumerSurface`]), accessibility routes
//! ([`M5CredentialAccessibilityRoute`]), and downgrade triggers
//! ([`M5CredentialDowngradeTrigger`]) are reused directly from the frozen matrix, so
//! this lane never invents a parallel credential vocabulary. It mints new vocabulary
//! only for what that matrix left implicit about these two controls: the asking
//! actor, the derived handle-availability class, the keyboard-complete prompt
//! actions, the store verification state, the derived store trust class, and the
//! store-capability-row actions.
//!
//! Raw secret values, pasted tokens, passphrases, and private endpoints stay
//! outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-secret-access-prompt-store-capability-controls.schema.json`](../../../../schemas/ui/m5-secret-access-prompt-store-capability-controls.schema.json).
//! The contract doc is
//! [`docs/security/implement_secret_access_prompt_sheets_and_credential_store_capability_rows.md`](../../../../docs/security/implement_secret_access_prompt_sheets_and_credential_store_capability_rows.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_secret_access_prompt_store_capability_controls,
    seeded_secret_access_prompt_store_capability_controls_secret_access_prompt_raw_reveal,
    seeded_secret_access_prompt_store_capability_controls_store_capability_unverified,
    SECRET_ACCESS_PROMPT_STORE_CAPABILITY_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The storage mode, credential class, reveal posture, store capability, export-safety
// class, degraded state, required labels, surface family, deployment line, consumer
// surface, accessibility route, and downgrade triggers are frozen once, in the
// credential component matrix. This lane reuses them verbatim so it never invents a
// parallel credential vocabulary.
use crate::freeze_the_m5_credential_component_matrix::{
    M5CredentialAccessibilityRoute, M5CredentialClass, M5CredentialComponentFamily,
    M5CredentialConsumerSurface, M5CredentialDegradedState, M5CredentialDeploymentLine,
    M5CredentialDowngradeTrigger, M5CredentialExportSafetyClass, M5CredentialRequiredLabel,
    M5CredentialRevealPosture, M5CredentialStorageMode, M5CredentialStoreCapability,
    M5CredentialSurfaceFamily, M5_CREDENTIAL_COMPONENT_DOC_REF,
    M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_ACCESS_PROMPT_REF,
    M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_HANDLE_REF, M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
    M5_CREDENTIAL_STORE_CAPABILITY_ROW_SCHEMA_REF, M5_SECRET_ACCESS_PROMPT_SHEET_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`SecretAccessPromptStoreCapabilityControlsPacket`].
pub const SECRET_ACCESS_PROMPT_STORE_CAPABILITY_RECORD_KIND: &str =
    "secret_access_prompt_store_capability_controls";

/// Schema version for secret-access-prompt / store-capability control records.
pub const SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_REF: &str =
    "schemas/ui/m5-secret-access-prompt-store-capability-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const SECRET_ACCESS_PROMPT_STORE_CAPABILITY_DOC_REF: &str =
    "docs/security/implement_secret_access_prompt_sheets_and_credential_store_capability_rows.md";

/// Repo-relative path of the protected fixture directory.
pub const SECRET_ACCESS_PROMPT_STORE_CAPABILITY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-secret-access-prompt-store-capability-controls";

/// Repo-relative path of the checked support-export artifact.
pub const SECRET_ACCESS_PROMPT_STORE_CAPABILITY_ARTIFACT_REF: &str =
    "artifacts/release/m5-secret-access-prompt-store-capability-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SUMMARY_REF: &str =
    "artifacts/release/m5-secret-access-prompt-store-capability-proof/summary.md";

// ---- secret-access-prompt-sheet vocabulary ------------------------------

/// Actor a secret-access prompt names as the party asking for secret access.
///
/// This is the "who is asking" axis: a prompt never leaves the requester implicit, so
/// a user can tell that a provider connector — rather than an unnamed background job —
/// is asking for the secret.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretRequestActor {
    /// A first-party Aureline feature is asking.
    FirstPartyFeature,
    /// A provider / VCS-host connector is asking.
    ProviderConnector,
    /// A package / artifact registry client is asking.
    RegistryClient,
    /// A remote-target / database attach is asking.
    RemoteOrDatabaseAttach,
    /// A package / release publisher is asking.
    ReleasePublisher,
    /// A delegated agent acting on behalf of another principal is asking.
    DelegatedAgent,
}

impl SecretRequestActor {
    /// Every actor, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FirstPartyFeature,
        Self::ProviderConnector,
        Self::RegistryClient,
        Self::RemoteOrDatabaseAttach,
        Self::ReleasePublisher,
        Self::DelegatedAgent,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyFeature => "first_party_feature",
            Self::ProviderConnector => "provider_connector",
            Self::RegistryClient => "registry_client",
            Self::RemoteOrDatabaseAttach => "remote_or_database_attach",
            Self::ReleasePublisher => "release_publisher",
            Self::DelegatedAgent => "delegated_agent",
        }
    }
}

/// Derived handle-availability class a secret-access prompt may present.
///
/// This is the prompt honesty axis: the class is derived from the reveal posture,
/// never asserted, so when a handle-only path exists a user sees it surfaced instead
/// of being nudged toward raw-secret sprawl, and a flow that requests the raw secret
/// can never quietly present as handle-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandleAvailabilityClass {
    /// A handle-only path exists; the raw secret is never requested.
    HandleOnlyAvailable,
    /// Only a masked or scoped-copy path is offered; no raw on-screen reveal.
    ScopedRevealOnly,
    /// The flow requests the raw secret; this must be explicit and never nudged.
    RawRevealRequested,
    /// A raw reveal is blocked by policy.
    RevealPolicyBlocked,
}

impl HandleAvailabilityClass {
    /// Every handle-availability class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::HandleOnlyAvailable,
        Self::ScopedRevealOnly,
        Self::RawRevealRequested,
        Self::RevealPolicyBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HandleOnlyAvailable => "handle_only_available",
            Self::ScopedRevealOnly => "scoped_reveal_only",
            Self::RawRevealRequested => "raw_reveal_requested",
            Self::RevealPolicyBlocked => "reveal_policy_blocked",
        }
    }
}

/// One keyboard-complete default action a secret-access prompt offers, so a prompt
/// never hides its allow / deny / once affordance behind a pointer-only gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretAccessPromptAction {
    /// Allow the request for this one use only.
    AllowOnce,
    /// Allow the request for the rest of this session.
    AllowForSession,
    /// Allow the request and store the grant.
    AllowAndStore,
    /// Deny the request.
    Deny,
    /// Review the requested scope before deciding.
    ReviewScope,
    /// Copy the handle reference (never the raw secret).
    CopyHandleReference,
}

impl SecretAccessPromptAction {
    /// Every prompt action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AllowOnce,
        Self::AllowForSession,
        Self::AllowAndStore,
        Self::Deny,
        Self::ReviewScope,
        Self::CopyHandleReference,
    ];

    /// The allow / deny / once semantics every keyboard-complete prompt must offer.
    pub const MANDATORY: [Self; 3] = [Self::AllowOnce, Self::AllowAndStore, Self::Deny];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowOnce => "allow_once",
            Self::AllowForSession => "allow_for_session",
            Self::AllowAndStore => "allow_and_store",
            Self::Deny => "deny",
            Self::ReviewScope => "review_scope",
            Self::CopyHandleReference => "copy_handle_reference",
        }
    }
}

/// Disclosures a secret-access prompt must carry, derived from the reveal posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandleAvailabilityDisclosure {
    /// The derived handle-availability class this prompt may present.
    pub handle_availability_class: HandleAvailabilityClass,
    /// Whether a handle-only path exists (raw secret never requested).
    pub is_handle_only_available: bool,
    /// Whether the flow requests the raw secret.
    pub requests_raw_reveal: bool,
    /// Whether the prompt must surface the available handle-only / scoped path.
    pub needs_handle_only_note: bool,
    /// Whether the prompt must carry an explicit raw-reveal disclosure.
    pub needs_raw_reveal_disclosure_note: bool,
    /// Whether the prompt must carry an explicit reveal-blocked note.
    pub needs_reveal_blocked_note: bool,
}

/// Resolves the handle-availability truth a secret-access prompt may present.
///
/// A handle-only or never-revealed reveal posture means a handle-only path exists. A
/// masked or clipboard-scoped posture offers only a scoped path. A reveal-on-demand
/// posture requests the raw secret and must say so explicitly, never nudging a user
/// toward it. A policy-blocked posture blocks the raw reveal — none of which can ever
/// read as a plain handle-only path without saying which.
pub fn resolve_handle_availability(
    reveal_posture: M5CredentialRevealPosture,
) -> HandleAvailabilityDisclosure {
    use HandleAvailabilityClass as Handle;
    use M5CredentialRevealPosture as Reveal;

    let handle_availability_class = match reveal_posture {
        Reveal::HandleOnly | Reveal::NeverRevealed => Handle::HandleOnlyAvailable,
        Reveal::MaskedLastFour | Reveal::ClipboardScoped => Handle::ScopedRevealOnly,
        Reveal::RevealOnDemand => Handle::RawRevealRequested,
        Reveal::PolicyBlockedReveal => Handle::RevealPolicyBlocked,
    };

    HandleAvailabilityDisclosure {
        handle_availability_class,
        is_handle_only_available: matches!(handle_availability_class, Handle::HandleOnlyAvailable),
        requests_raw_reveal: matches!(handle_availability_class, Handle::RawRevealRequested),
        needs_handle_only_note: matches!(
            handle_availability_class,
            Handle::HandleOnlyAvailable | Handle::ScopedRevealOnly
        ),
        needs_raw_reveal_disclosure_note: matches!(
            handle_availability_class,
            Handle::RawRevealRequested
        ),
        needs_reveal_blocked_note: matches!(handle_availability_class, Handle::RevealPolicyBlocked),
    }
}

/// A secret-access prompt sheet naming actor, purpose, scope, reveal posture, derived
/// handle availability, retention, allow / deny / once actions, and what still works
/// if denied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretAccessPromptSheet {
    /// Frozen component this control implements; must be `secret_access_prompt_sheet`.
    pub component: M5CredentialComponentFamily,
    /// Stable sheet id.
    pub sheet_id: String,
    /// The actor asking for secret access.
    pub actor: SecretRequestActor,
    /// Human-readable actor label / who is asking; required and non-empty.
    pub actor_label: String,
    /// Purpose note / why the secret is being requested; required and non-empty.
    pub purpose_note: String,
    /// Requested-scope note / what scope is being requested; required and non-empty.
    pub requested_scope_note: String,
    /// Credential class being requested, reused from the frozen matrix.
    pub credential_class: M5CredentialClass,
    /// Handle-only-versus-raw-reveal posture, reused from the frozen matrix.
    pub reveal_posture: M5CredentialRevealPosture,
    /// Derived handle-availability class (must equal the resolved class).
    pub handle_availability_class: HandleAvailabilityClass,
    /// Whether the prompt claims a handle-only path exists (must equal the derived truth).
    pub claims_handle_only_path: bool,
    /// Handle-only / scoped-path note; required when a handle-only or scoped path exists.
    pub handle_only_note: String,
    /// Raw-reveal disclosure note; required when the flow requests the raw secret.
    pub raw_reveal_disclosure_note: String,
    /// Reveal-blocked note; required when a raw reveal is blocked by policy.
    pub reveal_blocked_note: String,
    /// Retention note; always required so retention stays explicit.
    pub retention_note: String,
    /// Denied-fallback note / what still works if denied; always required.
    pub denied_fallback_note: String,
    /// Storage-mode / reveal-posture note; always required so storage stays explicit.
    pub storage_and_reveal_note: String,
    /// Keyboard-complete default actions (must include the mandatory allow/deny/once).
    pub default_actions: Vec<SecretAccessPromptAction>,
    /// Degraded states this prompt can name (required, matching the frozen matrix).
    pub degraded_states: Vec<M5CredentialDegradedState>,
    /// Mandatory labels this prompt can show (must include the mandatory labels).
    pub required_labels: Vec<M5CredentialRequiredLabel>,
    /// Claimed M5 surface families that render this prompt.
    pub surface_families: Vec<M5CredentialSurfaceFamily>,
    /// Deployment lines this prompt keeps the same truth across.
    pub deployment_lines: Vec<M5CredentialDeploymentLine>,
    /// Non-visual accessibility routes this prompt offers.
    pub accessibility_routes: Vec<M5CredentialAccessibilityRoute>,
    /// Credential subsystems that consume this prompt's projection.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this prompt.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks storage mode or reveal posture. MUST be `false`.
    pub masks_storage_or_reveal_posture: bool,
    /// Hard invariant: never implies a raw secret is export-safe. MUST be `false`.
    pub implies_raw_secret_exportable: bool,
    /// Hard invariant: friendly "connected" wording never conceals storage, reveal,
    /// or retention truth. MUST be `false`.
    pub uses_friendly_connected_wording: bool,
}

impl SecretAccessPromptSheet {
    /// Handle-availability disclosures this prompt must carry, derived from the reveal posture.
    pub fn handle_availability_disclosure(&self) -> HandleAvailabilityDisclosure {
        resolve_handle_availability(self.reveal_posture)
    }

    /// Whether the prompt offers every mandatory keyboard-complete default action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<SecretAccessPromptAction> =
            self.default_actions.iter().copied().collect();
        SecretAccessPromptAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the prompt declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CredentialRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CredentialRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }
}

// ---- credential-store-capability-row vocabulary -------------------------

/// Verification state a credential-store-capability row names for its store.
///
/// This is the "what do we actually know" axis a user must be able to tell apart: a
/// hardware-attested store, an OS-verified store, an encrypted-verified store, an
/// unverified store, a store whose verification failed, or an unsupported store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StoreVerificationState {
    /// The store is hardware-attested.
    HardwareAttested,
    /// The store is OS-verified.
    OsVerified,
    /// The store is verified as an encrypted store.
    EncryptedVerified,
    /// The store's security could not be verified.
    Unverified,
    /// The store's verification actively failed.
    VerificationFailed,
    /// The store is unsupported on this platform / build.
    Unsupported,
}

impl StoreVerificationState {
    /// Every verification state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HardwareAttested,
        Self::OsVerified,
        Self::EncryptedVerified,
        Self::Unverified,
        Self::VerificationFailed,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HardwareAttested => "hardware_attested",
            Self::OsVerified => "os_verified",
            Self::EncryptedVerified => "encrypted_verified",
            Self::Unverified => "unverified",
            Self::VerificationFailed => "verification_failed",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Derived trust class a credential-store-capability row may present.
///
/// This is the store honesty axis: the class is derived from the verification state
/// and store capabilities, never asserted, so an unverified, verification-failed, or
/// unsupported store can never read as "securely stored" and no vague "saved
/// securely" message ever stands in for an unproven store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialStoreTrustClass {
    /// Verified and persistent: securely stored.
    SecurelyStored,
    /// Verified but session-only or otherwise limited: limited assurance.
    LimitedAssurance,
    /// Not verified: the store's security is unproven.
    UnverifiedStore,
    /// Unsupported on this platform / build.
    UnsupportedStore,
}

impl CredentialStoreTrustClass {
    /// Every trust class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SecurelyStored,
        Self::LimitedAssurance,
        Self::UnverifiedStore,
        Self::UnsupportedStore,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SecurelyStored => "securely_stored",
            Self::LimitedAssurance => "limited_assurance",
            Self::UnverifiedStore => "unverified_store",
            Self::UnsupportedStore => "unsupported_store",
        }
    }
}

/// One keyboard-complete default action a credential-store-capability row offers, so a
/// row never hides its verify or choose-different-store affordance behind a
/// pointer-only gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialStoreCapabilityRowAction {
    /// Verify the store now.
    VerifyStore,
    /// Choose a different store.
    ChooseDifferentStore,
    /// View the store's full capability list.
    ViewCapabilities,
    /// Open the store of record (the source of truth) to inspect it.
    OpenSourceOfTruth,
    /// Export the row as export-safe capability evidence.
    ExportRow,
}

impl CredentialStoreCapabilityRowAction {
    /// Every row action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::VerifyStore,
        Self::ChooseDifferentStore,
        Self::ViewCapabilities,
        Self::OpenSourceOfTruth,
        Self::ExportRow,
    ];

    /// The default actions every keyboard-complete store-capability row must offer.
    pub const MANDATORY: [Self; 2] = [Self::VerifyStore, Self::ChooseDifferentStore];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifyStore => "verify_store",
            Self::ChooseDifferentStore => "choose_different_store",
            Self::ViewCapabilities => "view_capabilities",
            Self::OpenSourceOfTruth => "open_source_of_truth",
            Self::ExportRow => "export_row",
        }
    }
}

/// Disclosures a credential-store-capability row must carry, derived from the
/// verification state and store capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreTrustDisclosure {
    /// The derived trust class this row may present.
    pub trust_class: CredentialStoreTrustClass,
    /// Whether the store is securely stored (verified and persistent).
    pub is_securely_stored: bool,
    /// Whether the store is unverified or unsupported (never "securely stored").
    pub is_unverified_or_unsupported: bool,
    /// Whether the row must carry an explicit unverified note.
    pub needs_unverified_note: bool,
    /// Whether the row must carry an explicit unsupported note.
    pub needs_unsupported_note: bool,
    /// Whether the row must carry an explicit session-only fallback note.
    pub needs_session_only_fallback_note: bool,
}

/// Resolves the trust truth a credential-store-capability row may present.
///
/// An unsupported store is unsupported. An unverified or verification-failed store is
/// an unverified store. A verified store that is session-only (or does not persist
/// across restart) has limited assurance and must name its session-only fallback.
/// Only a verified, persistent store is securely stored — so an unverified,
/// verification-failed, or unsupported store can never claim to be securely stored.
pub fn resolve_store_trust(
    verification_state: StoreVerificationState,
    store_capabilities: &[M5CredentialStoreCapability],
) -> StoreTrustDisclosure {
    use CredentialStoreTrustClass as Trust;
    use M5CredentialStoreCapability as Capability;
    use StoreVerificationState as Verification;

    let unsupported = matches!(verification_state, Verification::Unsupported);
    let unverified = matches!(
        verification_state,
        Verification::Unverified | Verification::VerificationFailed
    );
    let session_only = store_capabilities.contains(&Capability::SessionOnly)
        || !store_capabilities.contains(&Capability::PersistAcrossRestart);

    let trust_class = if unsupported {
        Trust::UnsupportedStore
    } else if unverified {
        Trust::UnverifiedStore
    } else if session_only {
        Trust::LimitedAssurance
    } else {
        Trust::SecurelyStored
    };

    StoreTrustDisclosure {
        trust_class,
        is_securely_stored: matches!(trust_class, Trust::SecurelyStored),
        is_unverified_or_unsupported: matches!(
            trust_class,
            Trust::UnverifiedStore | Trust::UnsupportedStore
        ),
        needs_unverified_note: matches!(trust_class, Trust::UnverifiedStore),
        needs_unsupported_note: matches!(trust_class, Trust::UnsupportedStore),
        needs_session_only_fallback_note: matches!(trust_class, Trust::LimitedAssurance),
    }
}

/// A credential-store-capability row naming store type, verification state,
/// portability / export posture, platform limitations, and session-only fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialStoreCapabilityRow {
    /// Frozen component this control implements; must be `credential_store_capability_row`.
    pub component: M5CredentialComponentFamily,
    /// Stable row id.
    pub row_id: String,
    /// Store label / what store this row represents; required and non-empty.
    pub store_label: String,
    /// Store type (where the store keeps its material), reused from the frozen matrix.
    pub storage_mode: M5CredentialStorageMode,
    /// Store capabilities this row names, reused from the frozen matrix.
    pub store_capabilities: Vec<M5CredentialStoreCapability>,
    /// Verification state this row names.
    pub verification_state: StoreVerificationState,
    /// Human-readable verification-state label; required and non-empty.
    pub verification_label: String,
    /// Portability / export-safety class this store applies, reused from the frozen matrix.
    pub export_safety_class: M5CredentialExportSafetyClass,
    /// Derived trust class (must equal the resolved class).
    pub trust_class: CredentialStoreTrustClass,
    /// Whether the row claims the store is securely stored (must equal the derived truth).
    pub claims_securely_stored: bool,
    /// Portability / export note; always required so the export posture stays explicit.
    pub portability_export_note: String,
    /// Platform-limitations note; always required so platform limits stay explicit.
    pub platform_limitations_note: String,
    /// Unverified note; required when the store is unverified.
    pub unverified_note: String,
    /// Unsupported note; required when the store is unsupported.
    pub unsupported_note: String,
    /// Session-only fallback note; required when the store has limited (session-only) assurance.
    pub session_only_fallback_note: String,
    /// Storage-mode / capability note; always required so storage stays explicit.
    pub storage_and_capability_note: String,
    /// Keyboard-complete default actions (must include the mandatory verify/choose actions).
    pub default_actions: Vec<CredentialStoreCapabilityRowAction>,
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
    /// Hard invariant: never masks storage mode or reveal posture. MUST be `false`.
    pub masks_storage_or_reveal_posture: bool,
    /// Hard invariant: never implies a raw secret is export-safe. MUST be `false`.
    pub implies_raw_secret_exportable: bool,
    /// Hard invariant: friendly "connected" / "saved securely" wording never conceals
    /// an unverified or unsupported store. MUST be `false`.
    pub uses_friendly_connected_wording: bool,
}

impl CredentialStoreCapabilityRow {
    /// Trust disclosures this row must carry, derived from verification and capabilities.
    pub fn trust_disclosure(&self) -> StoreTrustDisclosure {
        resolve_store_trust(self.verification_state, &self.store_capabilities)
    }

    /// Whether the row offers every mandatory keyboard-complete default action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<CredentialStoreCapabilityRowAction> =
            self.default_actions.iter().copied().collect();
        CredentialStoreCapabilityRowAction::MANDATORY
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
pub struct SecretAccessPromptStoreCapabilityTrustReview {
    /// The secret-access prompt names its actor, purpose, and requested scope.
    pub prompt_shows_actor_purpose_and_scope: bool,
    /// The secret-access prompt names its raw-secret-versus-handle-only posture.
    pub prompt_shows_raw_versus_handle_only_posture: bool,
    /// A handle-only path is surfaced whenever one exists.
    pub handle_only_path_surfaced_when_available: bool,
    /// The raw secret is never nudged; a raw reveal is always explicit.
    pub raw_reveal_never_nudged: bool,
    /// The prompt names its retention note and what still works if denied.
    pub prompt_shows_retention_and_denied_fallback: bool,
    /// The allow, deny, and once semantics are always present and keyboard-complete.
    pub allow_deny_once_semantics_present: bool,
    /// The store row names its store type and verification state.
    pub store_row_shows_type_and_verification_state: bool,
    /// The store row names its portability / export posture and platform limitations.
    pub store_row_shows_portability_and_platform_limits: bool,
    /// The trust class is derived from verification / capability, never asserted.
    pub trust_class_derived_never_asserted: bool,
    /// An unverified or unsupported store never reads as securely stored.
    pub unverified_or_unsupported_never_reads_as_secure: bool,
    /// The session-only fallback stays explicit whenever policy allows one.
    pub session_only_fallback_explicit_when_policy_allows: bool,
    /// Raw-secret handling is never normalized on any surface.
    pub raw_secret_handling_never_normalized: bool,
    /// No vague "saved securely" / friendly wording conceals an unverified store.
    pub no_vague_saved_securely_wording: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl SecretAccessPromptStoreCapabilityTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.prompt_shows_actor_purpose_and_scope
            && self.prompt_shows_raw_versus_handle_only_posture
            && self.handle_only_path_surfaced_when_available
            && self.raw_reveal_never_nudged
            && self.prompt_shows_retention_and_denied_fallback
            && self.allow_deny_once_semantics_present
            && self.store_row_shows_type_and_verification_state
            && self.store_row_shows_portability_and_platform_limits
            && self.trust_class_derived_never_asserted
            && self.unverified_or_unsupported_never_reads_as_secure
            && self.session_only_fallback_explicit_when_policy_allows
            && self.raw_secret_handling_never_normalized
            && self.no_vague_saved_securely_wording
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretAccessPromptStoreCapabilityConsumerProjection {
    /// The secret prompt shows actor, scope, and the handle-only alternative without docs.
    pub prompt_shows_actor_scope_and_alternative_without_docs: bool,
    /// A handle-only path is visible before any raw reveal.
    pub handle_only_path_visible_before_raw_reveal: bool,
    /// The store row shows type, verification, and limits inline.
    pub store_row_shows_type_verification_and_limits_inline: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_control_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl SecretAccessPromptStoreCapabilityConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.prompt_shows_actor_scope_and_alternative_without_docs
            && self.handle_only_path_visible_before_raw_reveal
            && self.store_row_shows_type_verification_and_limits_inline
            && self.cli_headless_shows_control_truth
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretAccessPromptStoreCapabilityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`SecretAccessPromptStoreCapabilityControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretAccessPromptStoreCapabilityControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Secret-access prompt sheets.
    pub secret_access_prompts: Vec<SecretAccessPromptSheet>,
    /// Credential-store-capability rows.
    pub store_capability_rows: Vec<CredentialStoreCapabilityRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CredentialDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Trust review block.
    pub trust_review: SecretAccessPromptStoreCapabilityTrustReview,
    /// Consumer projection block.
    pub consumer_projection: SecretAccessPromptStoreCapabilityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: SecretAccessPromptStoreCapabilityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe secret-access-prompt / store-capability controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretAccessPromptStoreCapabilityControlsPacket {
    /// Record kind; must equal [`SECRET_ACCESS_PROMPT_STORE_CAPABILITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Secret-access prompt sheets.
    pub secret_access_prompts: Vec<SecretAccessPromptSheet>,
    /// Credential-store-capability rows.
    pub store_capability_rows: Vec<CredentialStoreCapabilityRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CredentialDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Trust review block.
    pub trust_review: SecretAccessPromptStoreCapabilityTrustReview,
    /// Consumer projection block.
    pub consumer_projection: SecretAccessPromptStoreCapabilityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: SecretAccessPromptStoreCapabilityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl SecretAccessPromptStoreCapabilityControlsPacket {
    /// Builds a secret-access-prompt / store-capability controls packet from stable-lane input.
    pub fn new(input: SecretAccessPromptStoreCapabilityControlsPacketInput) -> Self {
        Self {
            record_kind: SECRET_ACCESS_PROMPT_STORE_CAPABILITY_RECORD_KIND.to_owned(),
            schema_version: SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            secret_access_prompts: input.secret_access_prompts,
            store_capability_rows: input.store_capability_rows,
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

    /// Validates the secret-access-prompt / store-capability control invariants.
    pub fn validate(&self) -> Vec<SecretAccessPromptStoreCapabilityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SECRET_ACCESS_PROMPT_STORE_CAPABILITY_RECORD_KIND {
            violations.push(SecretAccessPromptStoreCapabilityViolation::WrongRecordKind);
        }
        if self.schema_version != SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_VERSION {
            violations.push(SecretAccessPromptStoreCapabilityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(SecretAccessPromptStoreCapabilityViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_secret_access_prompts(self, &mut violations);
        validate_store_capability_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(SecretAccessPromptStoreCapabilityViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("secret access prompt store capability packet serializes"),
        ) {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::RawBoundaryMaterialInExport);
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
            .expect("secret access prompt store capability packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "control,id,actor_or_type,reveal_or_export,state,derived,handle_only_or_secure\n",
        );
        for prompt in &self.secret_access_prompts {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "secret_access_prompt_sheet",
                csv_field(&prompt.sheet_id),
                prompt.actor.as_str(),
                prompt.reveal_posture.as_str(),
                prompt.credential_class.as_str(),
                prompt
                    .handle_availability_disclosure()
                    .handle_availability_class
                    .as_str(),
                prompt
                    .handle_availability_disclosure()
                    .is_handle_only_available,
            ));
        }
        for row in &self.store_capability_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "credential_store_capability_row",
                csv_field(&row.row_id),
                row.storage_mode.as_str(),
                row.export_safety_class.as_str(),
                row.verification_state.as_str(),
                row.trust_disclosure().trust_class.as_str(),
                row.trust_disclosure().is_securely_stored,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let raw_requested = self
            .secret_access_prompts
            .iter()
            .filter(|prompt| prompt.handle_availability_disclosure().requests_raw_reveal)
            .count();
        let not_secure = self
            .store_capability_rows
            .iter()
            .filter(|row| !row.trust_disclosure().is_securely_stored)
            .count();

        let mut out = String::new();
        out.push_str("# Secret-access prompt sheets and credential-store-capability rows\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Secret-access prompts: {} ({} request a raw reveal)\n",
            self.secret_access_prompts.len(),
            raw_requested
        ));
        out.push_str(&format!(
            "- Store-capability rows: {} ({} not securely stored)\n",
            self.store_capability_rows.len(),
            not_secure
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Secret-access prompt sheets\n\n");
        for prompt in &self.secret_access_prompts {
            out.push_str(&format!(
                "- **{}** ({}) — class `{}`, reveal `{}` → `{}`\n",
                prompt.actor_label,
                prompt.actor.as_str(),
                prompt.credential_class.as_str(),
                prompt.reveal_posture.as_str(),
                prompt
                    .handle_availability_disclosure()
                    .handle_availability_class
                    .as_str(),
            ));
        }

        out.push_str("\n## Credential-store-capability rows\n\n");
        for row in &self.store_capability_rows {
            out.push_str(&format!(
                "- **{}** — type `{}`, verification `{}`, export `{}` → `{}`\n",
                row.store_label,
                row.storage_mode.as_str(),
                row.verification_state.as_str(),
                row.export_safety_class.as_str(),
                row.trust_disclosure().trust_class.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in secret-access-prompt / store-capability export.
#[derive(Debug)]
pub enum SecretAccessPromptStoreCapabilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<SecretAccessPromptStoreCapabilityViolation>),
}

impl fmt::Display for SecretAccessPromptStoreCapabilityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "secret access prompt store capability export parse failed: {error}"
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
                    "secret access prompt store capability export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SecretAccessPromptStoreCapabilityArtifactError {}

/// Validation failures emitted by [`SecretAccessPromptStoreCapabilityControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecretAccessPromptStoreCapabilityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No secret-access prompts are present.
    SecretAccessPromptsMissing,
    /// A secret-access prompt is incomplete.
    SecretAccessPromptIncomplete,
    /// A secret-access prompt carries the wrong frozen component class.
    SecretAccessPromptWrongComponentClass,
    /// A secret-access prompt does not name its actor, purpose, or requested scope.
    ActorPurposeOrScopeMissing,
    /// A secret-access prompt misrepresents its derived handle-availability class.
    HandleAvailabilityMisrepresented,
    /// A prompt with a handle-only / scoped path does not surface it.
    HandleOnlyNoteMissing,
    /// A prompt requesting a raw reveal does not disclose it explicitly.
    RawRevealDisclosureMissing,
    /// A prompt whose reveal is policy-blocked does not name it.
    RevealBlockedNoteMissing,
    /// A secret-access prompt does not name its retention note.
    RetentionNoteMissing,
    /// A secret-access prompt does not name what still works if denied.
    DeniedFallbackNoteMissing,
    /// A secret-access prompt does not name its storage mode / reveal posture.
    StorageAndRevealNoteMissing,
    /// A secret-access prompt omits a mandatory allow/deny/once action.
    PromptActionsIncomplete,
    /// The secret-access prompts do not cover every asking actor.
    ActorCoverageMissing,
    /// The secret-access prompts do not cover every handle-availability class.
    HandleAvailabilityCoverageMissing,
    /// No credential-store-capability rows are present.
    StoreCapabilityRowsMissing,
    /// A credential-store-capability row is incomplete.
    StoreCapabilityRowIncomplete,
    /// A credential-store-capability row carries the wrong frozen component class.
    StoreCapabilityRowWrongComponentClass,
    /// A credential-store-capability row does not name its verification state.
    VerificationStateMissing,
    /// A credential-store-capability row misrepresents its derived trust class.
    TrustClassMisrepresented,
    /// A credential-store-capability row does not name its portability / export posture.
    PortabilityExportNoteMissing,
    /// A credential-store-capability row does not name its platform limitations.
    PlatformLimitationsNoteMissing,
    /// An unverified store does not name its unverified state.
    UnverifiedNoteMissing,
    /// An unsupported store does not name its unsupported state.
    UnsupportedNoteMissing,
    /// A limited-assurance store does not name its session-only fallback.
    SessionOnlyFallbackNoteMissing,
    /// A credential-store-capability row omits a mandatory verify/choose action.
    StoreRowActionsIncomplete,
    /// The credential-store-capability rows do not cover every verification state.
    VerificationStateCoverageMissing,
    /// The credential-store-capability rows do not cover every trust class.
    TrustClassCoverageMissing,
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
    /// A control uses vague "saved securely" / friendly wording that conceals a store.
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

impl SecretAccessPromptStoreCapabilityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::SecretAccessPromptsMissing => "secret_access_prompts_missing",
            Self::SecretAccessPromptIncomplete => "secret_access_prompt_incomplete",
            Self::SecretAccessPromptWrongComponentClass => {
                "secret_access_prompt_wrong_component_class"
            }
            Self::ActorPurposeOrScopeMissing => "actor_purpose_or_scope_missing",
            Self::HandleAvailabilityMisrepresented => "handle_availability_misrepresented",
            Self::HandleOnlyNoteMissing => "handle_only_note_missing",
            Self::RawRevealDisclosureMissing => "raw_reveal_disclosure_missing",
            Self::RevealBlockedNoteMissing => "reveal_blocked_note_missing",
            Self::RetentionNoteMissing => "retention_note_missing",
            Self::DeniedFallbackNoteMissing => "denied_fallback_note_missing",
            Self::StorageAndRevealNoteMissing => "storage_and_reveal_note_missing",
            Self::PromptActionsIncomplete => "prompt_actions_incomplete",
            Self::ActorCoverageMissing => "actor_coverage_missing",
            Self::HandleAvailabilityCoverageMissing => "handle_availability_coverage_missing",
            Self::StoreCapabilityRowsMissing => "store_capability_rows_missing",
            Self::StoreCapabilityRowIncomplete => "store_capability_row_incomplete",
            Self::StoreCapabilityRowWrongComponentClass => {
                "store_capability_row_wrong_component_class"
            }
            Self::VerificationStateMissing => "verification_state_missing",
            Self::TrustClassMisrepresented => "trust_class_misrepresented",
            Self::PortabilityExportNoteMissing => "portability_export_note_missing",
            Self::PlatformLimitationsNoteMissing => "platform_limitations_note_missing",
            Self::UnverifiedNoteMissing => "unverified_note_missing",
            Self::UnsupportedNoteMissing => "unsupported_note_missing",
            Self::SessionOnlyFallbackNoteMissing => "session_only_fallback_note_missing",
            Self::StoreRowActionsIncomplete => "store_row_actions_incomplete",
            Self::VerificationStateCoverageMissing => "verification_state_coverage_missing",
            Self::TrustClassCoverageMissing => "trust_class_coverage_missing",
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

/// Reads and validates the checked-in stable secret-access-prompt / store-capability export.
pub fn current_secret_access_prompt_store_capability_export() -> Result<
    SecretAccessPromptStoreCapabilityControlsPacket,
    SecretAccessPromptStoreCapabilityArtifactError,
> {
    let packet: SecretAccessPromptStoreCapabilityControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-secret-access-prompt-store-capability-proof/support_export.json"
        )))
        .map_err(SecretAccessPromptStoreCapabilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(SecretAccessPromptStoreCapabilityArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &SecretAccessPromptStoreCapabilityControlsPacket,
    violations: &mut Vec<SecretAccessPromptStoreCapabilityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_REF,
        SECRET_ACCESS_PROMPT_STORE_CAPABILITY_DOC_REF,
        M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_DOC_REF,
        M5_SECRET_ACCESS_PROMPT_SHEET_SCHEMA_REF,
        M5_CREDENTIAL_STORE_CAPABILITY_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(SecretAccessPromptStoreCapabilityViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_secret_access_prompts(
    packet: &SecretAccessPromptStoreCapabilityControlsPacket,
    violations: &mut Vec<SecretAccessPromptStoreCapabilityViolation>,
) {
    if packet.secret_access_prompts.is_empty() {
        violations.push(SecretAccessPromptStoreCapabilityViolation::SecretAccessPromptsMissing);
        return;
    }

    let mut actors: BTreeSet<SecretRequestActor> = BTreeSet::new();
    let mut handle_classes: BTreeSet<HandleAvailabilityClass> = BTreeSet::new();

    for prompt in &packet.secret_access_prompts {
        let disclosure = prompt.handle_availability_disclosure();
        actors.insert(prompt.actor);
        handle_classes.insert(disclosure.handle_availability_class);

        if prompt.sheet_id.trim().is_empty()
            || prompt.actor_label.trim().is_empty()
            || prompt.fields_shown.is_empty()
            || prompt.surface_families.is_empty()
            || prompt.deployment_lines.is_empty()
            || prompt.consumer_surfaces.is_empty()
            || prompt.source_contract_refs.is_empty()
        {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::SecretAccessPromptIncomplete);
        }
        if prompt.component != M5CredentialComponentFamily::SecretAccessPromptSheet {
            violations.push(
                SecretAccessPromptStoreCapabilityViolation::SecretAccessPromptWrongComponentClass,
            );
        }
        if prompt.purpose_note.trim().is_empty() || prompt.requested_scope_note.trim().is_empty() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::ActorPurposeOrScopeMissing);
        }
        if prompt.handle_availability_class != disclosure.handle_availability_class
            || prompt.claims_handle_only_path != disclosure.is_handle_only_available
        {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::HandleAvailabilityMisrepresented);
        }
        if disclosure.needs_handle_only_note && prompt.handle_only_note.trim().is_empty() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::HandleOnlyNoteMissing);
        }
        if disclosure.needs_raw_reveal_disclosure_note
            && prompt.raw_reveal_disclosure_note.trim().is_empty()
        {
            violations.push(SecretAccessPromptStoreCapabilityViolation::RawRevealDisclosureMissing);
        }
        if disclosure.needs_reveal_blocked_note && prompt.reveal_blocked_note.trim().is_empty() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::RevealBlockedNoteMissing);
        }
        if prompt.retention_note.trim().is_empty() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::RetentionNoteMissing);
        }
        if prompt.denied_fallback_note.trim().is_empty() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::DeniedFallbackNoteMissing);
        }
        if prompt.storage_and_reveal_note.trim().is_empty() {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::StorageAndRevealNoteMissing);
        }
        if !prompt.declares_mandatory_actions() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::PromptActionsIncomplete);
        }
        if prompt.degraded_states.is_empty() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::DegradedStatesMissing);
        }
        if !prompt.declares_mandatory_labels() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::RequiredLabelsIncomplete);
        }
        if prompt.accessibility_routes.is_empty()
            || !prompt
                .accessibility_routes
                .contains(&M5CredentialAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(SecretAccessPromptStoreCapabilityViolation::AccessibilityRouteMissing);
        }
        if prompt.masks_storage_or_reveal_posture {
            violations.push(SecretAccessPromptStoreCapabilityViolation::StorageOrRevealMasked);
        }
        if prompt.implies_raw_secret_exportable {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::RawSecretHandlingNormalized);
        }
        if prompt.uses_friendly_connected_wording {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::FriendlyConnectedWordingUsed);
        }
    }

    for required in SecretRequestActor::ALL {
        if !actors.contains(&required) {
            violations.push(SecretAccessPromptStoreCapabilityViolation::ActorCoverageMissing);
            break;
        }
    }
    for required in HandleAvailabilityClass::ALL {
        if !handle_classes.contains(&required) {
            violations.push(
                SecretAccessPromptStoreCapabilityViolation::HandleAvailabilityCoverageMissing,
            );
            break;
        }
    }
}

fn validate_store_capability_rows(
    packet: &SecretAccessPromptStoreCapabilityControlsPacket,
    violations: &mut Vec<SecretAccessPromptStoreCapabilityViolation>,
) {
    if packet.store_capability_rows.is_empty() {
        violations.push(SecretAccessPromptStoreCapabilityViolation::StoreCapabilityRowsMissing);
        return;
    }

    let mut verification_states: BTreeSet<StoreVerificationState> = BTreeSet::new();
    let mut trust_classes: BTreeSet<CredentialStoreTrustClass> = BTreeSet::new();

    for row in &packet.store_capability_rows {
        let disclosure = row.trust_disclosure();
        verification_states.insert(row.verification_state);
        trust_classes.insert(disclosure.trust_class);

        if row.row_id.trim().is_empty()
            || row.store_label.trim().is_empty()
            || row.store_capabilities.is_empty()
            || row.fields_shown.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.consumer_surfaces.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::StoreCapabilityRowIncomplete);
        }
        if row.component != M5CredentialComponentFamily::CredentialStoreCapabilityRow {
            violations.push(
                SecretAccessPromptStoreCapabilityViolation::StoreCapabilityRowWrongComponentClass,
            );
        }
        if row.verification_label.trim().is_empty() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::VerificationStateMissing);
        }
        if row.trust_class != disclosure.trust_class
            || row.claims_securely_stored != disclosure.is_securely_stored
        {
            violations.push(SecretAccessPromptStoreCapabilityViolation::TrustClassMisrepresented);
        }
        if row.portability_export_note.trim().is_empty() {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::PortabilityExportNoteMissing);
        }
        if row.platform_limitations_note.trim().is_empty() {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::PlatformLimitationsNoteMissing);
        }
        if disclosure.needs_unverified_note && row.unverified_note.trim().is_empty() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::UnverifiedNoteMissing);
        }
        if disclosure.needs_unsupported_note && row.unsupported_note.trim().is_empty() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::UnsupportedNoteMissing);
        }
        if disclosure.needs_session_only_fallback_note
            && row.session_only_fallback_note.trim().is_empty()
        {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::SessionOnlyFallbackNoteMissing);
        }
        if row.storage_and_capability_note.trim().is_empty() {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::StorageAndRevealNoteMissing);
        }
        if !row.declares_mandatory_actions() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::StoreRowActionsIncomplete);
        }
        if row.degraded_states.is_empty() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::DegradedStatesMissing);
        }
        if !row.declares_mandatory_labels() {
            violations.push(SecretAccessPromptStoreCapabilityViolation::RequiredLabelsIncomplete);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5CredentialAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(SecretAccessPromptStoreCapabilityViolation::AccessibilityRouteMissing);
        }
        if row.masks_storage_or_reveal_posture {
            violations.push(SecretAccessPromptStoreCapabilityViolation::StorageOrRevealMasked);
        }
        if row.implies_raw_secret_exportable {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::RawSecretHandlingNormalized);
        }
        if row.uses_friendly_connected_wording {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::FriendlyConnectedWordingUsed);
        }
    }

    for required in StoreVerificationState::ALL {
        if !verification_states.contains(&required) {
            violations
                .push(SecretAccessPromptStoreCapabilityViolation::VerificationStateCoverageMissing);
            break;
        }
    }
    for required in CredentialStoreTrustClass::ALL {
        if !trust_classes.contains(&required) {
            violations.push(SecretAccessPromptStoreCapabilityViolation::TrustClassCoverageMissing);
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
