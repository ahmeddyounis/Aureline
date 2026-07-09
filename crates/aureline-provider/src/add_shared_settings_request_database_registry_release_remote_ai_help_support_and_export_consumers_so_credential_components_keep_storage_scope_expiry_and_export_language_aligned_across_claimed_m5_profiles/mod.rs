//! Shared consumers for the reusable M5 credential components, so the credential-state row,
//! secret-access-prompt sheet, vault-or-keychain picker, credential-store-capability row,
//! browser/device-code handoff card, delegated-credential row, rotation/revoke-event row, and
//! export-safety banner keep storage-mode, credential-class, handle-only-versus-raw-reveal,
//! local-versus-forwarded/delegated identity, expiry/refresh, and raw-secret-excluded export
//! language aligned across every claimed M5 credential-bearing surface where a user opens
//! credential settings, signs an outbound request, attaches a database, authorizes a
//! registry/provider, publishes a release, attaches a remote target, wires an AI model
//! provider, reads Help / docs, exports a support case, or hands a packet off for audit.
//!
//! Aureline's frozen credential component matrix
//! (`crate::freeze_the_m5_credential_component_matrix`) names the eight governed component
//! families, and four sibling implement lanes narrow those families into working primitives,
//! each with its own canonical schema, contract doc, and support-export artifact:
//!
//! * the credential-state row and vault-or-keychain picker
//!   (`implement_credential_state_rows_and_vault_or_keychain_pickers_...`),
//! * the secret-access-prompt sheet and credential-store-capability row
//!   (`implement_secret_access_prompt_sheets_and_credential_store_capability_rows_...`),
//! * the browser/device-code handoff card and delegated-credential row
//!   (`implement_browser_or_device_code_handoff_cards_and_delegated_credential_rows_...`), and
//! * the rotation/revoke-event row and export-safety banner
//!   (`implement_rotation_revoke_event_rows_and_export_safety_banners_...`).
//!
//! This module is the *adoption* lane over those primitives. It proves the eight families are
//! reusable components — not one sign-in view plus a few isolated export objects — by binding
//! every claimed M5 credential consumer (credential settings, the request auth surface, the
//! database attach surface, the registry/provider authorization surface, the release publish
//! surface, the remote-target attach surface, the AI model-provider surface, Help / docs, the
//! support / export desk, and the export packet) to the same canonical component schemas and
//! the same descriptor vocabulary. Each consumer points at the primitive's canonical schema and
//! support-export artifact rather than re-wording storage-mode, credential-class, reveal-posture,
//! delegated-identity, expiry, or export-safety facts in local prose, and each keeps that
//! vocabulary truthful even when only a credential handle is available, a credential is expired
//! or revoked, an identity is forwarded or delegated from another principal, or a secret is held
//! only for this session or blocked by policy.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_credential_component_binding`] — that takes one consumer's adoption
//!    of one component family, the descriptor set it surfaces, the parity-health mode it renders
//!    under, and any export caveats, and produces one [`M5CredentialComponentResolvedBinding`]
//!    carrying the derived claim-parity state and — whenever parity is weakened — a
//!    self-contained [`M5CredentialComponentAutoNarrowBanner`] that names the exact reason
//!    (a handle-only path, an expired/revoked credential, a forwarded/delegated identity, or a
//!    session-only/policy-blocked secret), the descriptors that stay preserved, and the recovery
//!    action, rather than a generic "degraded" note. The resolver never lets a narrowed context
//!    drop a required descriptor and never lets an expired/revoked, forwarded/delegated, or
//!    session-only/policy-blocked credential masquerade as a usable, locally stored one.
//! 2. A parity matrix — [`M5CredentialComponentConsumerPacket`] — that binds one row per claimed
//!    M5 credential consumer to the eight canonical component families, the one shared descriptor
//!    vocabulary, the same parity-health modes, export caveats, parity states, narrowing reasons,
//!    recovery actions, export fields, and non-visual accessibility routes, so storage-mode /
//!    credential-class / reveal-posture / delegated-identity / expiry / export-safety facts stop
//!    diverging between the primary UX, the docs, and the support / export artifact.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes,
//! qualification classes, downgrade triggers, and the eight component families themselves are
//! reused verbatim from the frozen credential component matrix. This module mints new vocabulary
//! only for what the adoption lane itself needs: its credential consumers, the shared descriptor
//! vocabulary, the parity-health modes, the export caveats, the claim-parity states, the
//! narrowing reasons and recovery actions, the consumer anatomy parts, and the export fields.
//!
//! Raw secrets, pasted tokens, private endpoints, and credential material stay outside the
//! support boundary; every label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is `schemas/ui/m5-credential-component-consumer.schema.json` and the
//! contract doc is `docs/security/m5_credential_component_consumers.md`. The protected fixture
//! directory is `fixtures/ui/m5-credential-component-consumers/`.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_credential_component_consumer_database_preview_narrowed,
    seeded_m5_credential_component_consumer_packet,
    seeded_m5_credential_component_consumer_registry_beta_narrowed,
    M5_CREDENTIAL_COMPONENT_CONSUMER_PACKET_ID,
};

// The surface families, deployment lines, consumer surfaces, accessibility routes,
// qualification classes, downgrade triggers, and the eight component families are frozen once,
// in the credential component matrix. This adoption lane reuses them verbatim so it never
// invents a parallel credential vocabulary.
pub use crate::freeze_the_m5_credential_component_matrix::{
    M5CredentialAccessibilityRoute, M5CredentialComponentFamily, M5CredentialConsumerSurface,
    M5CredentialDeploymentLine, M5CredentialDowngradeTrigger, M5CredentialQualificationClass,
    M5CredentialSurfaceFamily,
};

// The canonical matrix schema / doc refs this adoption lane points every consumer at, rather
// than re-wording their facts in local prose.
use crate::freeze_the_m5_credential_component_matrix::{
    M5_CREDENTIAL_COMPONENT_DOC_REF, M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
};
// The canonical primitive schema / doc / artifact refs each family maps to.
use crate::implement_browser_or_device_code_handoff_cards_and_delegated_credential_rows_with_handoff_boundary_and_delegated_identity_origin_truth::{
    BROWSER_HANDOFF_DELEGATED_CREDENTIAL_ARTIFACT_REF, BROWSER_HANDOFF_DELEGATED_CREDENTIAL_DOC_REF,
    BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_REF,
};
use crate::implement_credential_state_rows_and_vault_or_keychain_pickers_with_source_target_boundary_expiry_portability_and_rotate_revoke_test_truth::{
    CREDENTIAL_STATE_ROW_VAULT_PICKER_ARTIFACT_REF, CREDENTIAL_STATE_ROW_VAULT_PICKER_DOC_REF,
    CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_REF,
};
use crate::implement_rotation_revoke_event_rows_and_export_safety_banners_with_impacted_workflow_remembered_decision_and_raw_secret_excluded_continuity_truth::{
    ROTATION_REVOKE_EXPORT_SAFETY_ARTIFACT_REF, ROTATION_REVOKE_EXPORT_SAFETY_DOC_REF,
    ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_REF,
};
use crate::implement_secret_access_prompt_sheets_and_credential_store_capability_rows_with_actor_scope_handle_only_and_session_fallback_truth::{
    SECRET_ACCESS_PROMPT_STORE_CAPABILITY_ARTIFACT_REF, SECRET_ACCESS_PROMPT_STORE_CAPABILITY_DOC_REF,
    SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5CredentialComponentConsumerPacket`].
pub const M5_CREDENTIAL_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_settings_request_database_registry_release_remote_ai_help_support_and_export_consumers_so_credential_components_keep_storage_scope_expiry_and_export_language_aligned_across_claimed_m5_profiles";

/// Schema version for M5 credential component-consumer records.
pub const M5_CREDENTIAL_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the credential component-consumer boundary schema.
pub const M5_CREDENTIAL_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-credential-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CREDENTIAL_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/security/m5_credential_component_consumers.md";

/// Repo-relative path of the frozen credential component matrix this lane adopts from.
pub const M5_CREDENTIAL_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_CREDENTIAL_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_CREDENTIAL_COMPONENT_CONSUMER_OBJECT_MODEL_REF: &str = M5_CREDENTIAL_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_CREDENTIAL_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-credential-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CREDENTIAL_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-credential-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_CREDENTIAL_COMPONENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-credential-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_CREDENTIAL_COMPONENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-credential-component-consumer-proof/report.md";

/// The canonical boundary schema ref of the narrowed primitive that owns a family. A consumer
/// that adopts a family must point at this schema, not a local re-description.
pub const fn family_canonical_schema_ref(family: M5CredentialComponentFamily) -> &'static str {
    use M5CredentialComponentFamily as Family;
    match family {
        Family::CredentialStateRow | Family::VaultOrKeychainPicker => {
            CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_REF
        }
        Family::SecretAccessPromptSheet | Family::CredentialStoreCapabilityRow => {
            SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_REF
        }
        Family::BrowserDeviceCodeHandoffCard | Family::DelegatedCredentialRow => {
            BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_REF
        }
        Family::RotationRevokeEventRow | Family::ExportSafetyBanner => {
            ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_REF
        }
    }
}

/// The canonical contract-doc ref of the narrowed primitive that owns a family.
pub const fn family_canonical_doc_ref(family: M5CredentialComponentFamily) -> &'static str {
    use M5CredentialComponentFamily as Family;
    match family {
        Family::CredentialStateRow | Family::VaultOrKeychainPicker => {
            CREDENTIAL_STATE_ROW_VAULT_PICKER_DOC_REF
        }
        Family::SecretAccessPromptSheet | Family::CredentialStoreCapabilityRow => {
            SECRET_ACCESS_PROMPT_STORE_CAPABILITY_DOC_REF
        }
        Family::BrowserDeviceCodeHandoffCard | Family::DelegatedCredentialRow => {
            BROWSER_HANDOFF_DELEGATED_CREDENTIAL_DOC_REF
        }
        Family::RotationRevokeEventRow | Family::ExportSafetyBanner => {
            ROTATION_REVOKE_EXPORT_SAFETY_DOC_REF
        }
    }
}

/// The canonical support-export artifact ref of the narrowed primitive that owns a family.
pub const fn family_canonical_artifact_ref(family: M5CredentialComponentFamily) -> &'static str {
    use M5CredentialComponentFamily as Family;
    match family {
        Family::CredentialStateRow | Family::VaultOrKeychainPicker => {
            CREDENTIAL_STATE_ROW_VAULT_PICKER_ARTIFACT_REF
        }
        Family::SecretAccessPromptSheet | Family::CredentialStoreCapabilityRow => {
            SECRET_ACCESS_PROMPT_STORE_CAPABILITY_ARTIFACT_REF
        }
        Family::BrowserDeviceCodeHandoffCard | Family::DelegatedCredentialRow => {
            BROWSER_HANDOFF_DELEGATED_CREDENTIAL_ARTIFACT_REF
        }
        Family::RotationRevokeEventRow | Family::ExportSafetyBanner => {
            ROTATION_REVOKE_EXPORT_SAFETY_ARTIFACT_REF
        }
    }
}

/// One claimed M5 credential consumer that adopts the shared components. These are the consumers
/// the spec names — credential settings, the request auth surface, the database attach surface,
/// the registry/provider authorization surface, the release publish surface, the remote-target
/// attach surface, the AI model-provider surface, Help / docs, the support / export desk, and the
/// export packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialComponentConsumer {
    /// The credential-settings surface.
    Settings,
    /// The outbound request / API auth surface.
    Request,
    /// The database attach surface.
    Database,
    /// The registry / provider authorization surface.
    Registry,
    /// The release / publish signing surface.
    Release,
    /// The remote-target attach surface.
    Remote,
    /// The AI model-provider credential surface.
    AiAssistant,
    /// The Help / docs surface.
    Help,
    /// The support / export desk and support-bundle preview.
    Support,
    /// The export packet / exported view.
    Export,
}

impl M5CredentialComponentConsumer {
    /// Every claimed credential consumer, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::Settings,
        Self::Request,
        Self::Database,
        Self::Registry,
        Self::Release,
        Self::Remote,
        Self::AiAssistant,
        Self::Help,
        Self::Support,
        Self::Export,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Settings => "settings",
            Self::Request => "request",
            Self::Database => "database",
            Self::Registry => "registry",
            Self::Release => "release",
            Self::Remote => "remote",
            Self::AiAssistant => "ai_assistant",
            Self::Help => "help",
            Self::Support => "support",
            Self::Export => "export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Settings => "Credential Settings",
            Self::Request => "Request Auth Surface",
            Self::Database => "Database Attach",
            Self::Registry => "Registry / Provider Auth",
            Self::Release => "Release Publish",
            Self::Remote => "Remote Target Attach",
            Self::AiAssistant => "AI Model Provider",
            Self::Help => "Help / Docs",
            Self::Support => "Support / Export Desk",
            Self::Export => "Export Packet",
        }
    }

    /// True when this consumer is a help, support, or export surface — the surfaces singled out
    /// for a canonical-schema reference so their prose can never drift from the product truth.
    pub const fn is_help_support_or_export(self) -> bool {
        matches!(self, Self::Help | Self::Support | Self::Export)
    }
}

/// The one shared descriptor vocabulary every credential component keeps aligned across
/// surfaces, so no consumer invents a new grammar or stale wording. The descriptors in
/// [`M5CredentialComponentDescriptor::REQUIRED`] must be present on every binding — the
/// acceptance-criterion that storage mode, credential class, handle-only-versus-raw-reveal
/// posture, local-versus-forwarded/delegated identity, expiry/refresh state, and
/// raw-secret-excluded export safety stay one truth across in-product and exported credential
/// surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialComponentDescriptor {
    /// Where the secret actually lives (the storage mode).
    StorageMode,
    /// The kind of credential the component represents (the credential class).
    CredentialClass,
    /// Whether a raw reveal is possible or the path is handle-only (the reveal posture).
    RevealPosture,
    /// Whether the identity is local or forwarded / delegated from another principal.
    DelegatedIdentity,
    /// The expiry / refresh / rotation / revoke lifecycle state.
    ExpiryLifecycle,
    /// The raw-secret-excluded export-safety boundary.
    ExportSafety,
}

impl M5CredentialComponentDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StorageMode,
        Self::CredentialClass,
        Self::RevealPosture,
        Self::DelegatedIdentity,
        Self::ExpiryLifecycle,
        Self::ExportSafety,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 6] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StorageMode => "storage_mode",
            Self::CredentialClass => "credential_class",
            Self::RevealPosture => "reveal_posture",
            Self::DelegatedIdentity => "delegated_identity",
            Self::ExpiryLifecycle => "expiry_lifecycle",
            Self::ExportSafety => "export_safety",
        }
    }
}

/// The parity-health mode a consumer renders a component under. A weakened mode still keeps the
/// descriptor vocabulary — it only discloses that parity is narrowed relative to the
/// authoritative credential rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialConsumerParityHealth {
    /// Full parity: the authoritative credential rendering.
    FullParity,
    /// Only a credential handle is available; no raw secret is exposed here.
    HandleOnlyNarrowed,
    /// The credential is expired or revoked, so it is no longer usable.
    ExpiredOrRevokedNarrowed,
    /// The identity is forwarded or delegated from another principal, not locally stored.
    DelegatedOrForwardedNarrowed,
    /// The secret is held only for this session or blocked by policy, so it is not durable.
    SessionOnlyOrPolicyBlockedNarrowed,
}

impl M5CredentialConsumerParityHealth {
    /// Every parity-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::HandleOnlyNarrowed,
        Self::ExpiredOrRevokedNarrowed,
        Self::DelegatedOrForwardedNarrowed,
        Self::SessionOnlyOrPolicyBlockedNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::HandleOnlyNarrowed => "handle_only_narrowed",
            Self::ExpiredOrRevokedNarrowed => "expired_or_revoked_narrowed",
            Self::DelegatedOrForwardedNarrowed => "delegated_or_forwarded_narrowed",
            Self::SessionOnlyOrPolicyBlockedNarrowed => "session_only_or_policy_blocked_narrowed",
        }
    }

    /// True when the mode renders below the authoritative full parity and so must disclose a
    /// self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5CredentialConsumerNarrowingReason> {
        Some(match self {
            Self::HandleOnlyNarrowed => M5CredentialConsumerNarrowingReason::HandleOnlyPath,
            Self::ExpiredOrRevokedNarrowed => {
                M5CredentialConsumerNarrowingReason::CredentialExpiredOrRevoked
            }
            Self::DelegatedOrForwardedNarrowed => {
                M5CredentialConsumerNarrowingReason::IdentityForwardedOrDelegated
            }
            Self::SessionOnlyOrPolicyBlockedNarrowed => {
                M5CredentialConsumerNarrowingReason::SessionOnlyOrPolicyBlocked
            }
            Self::FullParity => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its parity claim language, so an auto-narrow banner
/// never reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialConsumerNarrowingReason {
    /// Only a credential handle is available; no raw secret is exposed.
    HandleOnlyPath,
    /// The credential is expired or revoked, so it is no longer usable.
    CredentialExpiredOrRevoked,
    /// The identity is forwarded or delegated from another principal, not locally stored.
    IdentityForwardedOrDelegated,
    /// The secret is held only for this session or blocked by policy, so it is not durable.
    SessionOnlyOrPolicyBlocked,
}

impl M5CredentialConsumerNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::HandleOnlyPath,
        Self::CredentialExpiredOrRevoked,
        Self::IdentityForwardedOrDelegated,
        Self::SessionOnlyOrPolicyBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HandleOnlyPath => "handle_only_path",
            Self::CredentialExpiredOrRevoked => "credential_expired_or_revoked",
            Self::IdentityForwardedOrDelegated => "identity_forwarded_or_delegated",
            Self::SessionOnlyOrPolicyBlocked => "session_only_or_policy_blocked",
        }
    }

    /// True when the reason reflects an expired/revoked, forwarded/delegated, or
    /// session-only/policy-blocked credential that must never masquerade as a usable, locally
    /// stored one — the acceptance-criterion boundary for a credential that cannot honestly read
    /// as "still usable" and "locally stored" on any surface. A handle-only path does *not* make
    /// a credential unusable, so it is excluded.
    pub const fn reflects_unusable_or_forwarded(self) -> bool {
        matches!(
            self,
            Self::CredentialExpiredOrRevoked
                | Self::IdentityForwardedOrDelegated
                | Self::SessionOnlyOrPolicyBlocked
        )
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::HandleOnlyPath => {
                "only a credential handle is available here, so no raw secret is exposed and a raw copy is not offered"
            }
            Self::CredentialExpiredOrRevoked => {
                "the credential is expired or revoked, so it is no longer usable and must be rotated or re-authenticated before it is trusted"
            }
            Self::IdentityForwardedOrDelegated => {
                "the identity is forwarded or delegated from another principal, so it is not a locally stored credential"
            }
            Self::SessionOnlyOrPolicyBlocked => {
                "the credential is held only for this session or is blocked by policy, so it is not durably stored and may not survive send, run, or publish"
            }
        }
    }

    /// The recovery action a reader should take before trusting full parity.
    pub const fn recovery_action(self) -> M5CredentialConsumerRecoveryAction {
        match self {
            Self::HandleOnlyPath => M5CredentialConsumerRecoveryAction::UseHandleReferenceNoRawCopy,
            Self::CredentialExpiredOrRevoked => {
                M5CredentialConsumerRecoveryAction::RotateOrReauthenticate
            }
            Self::IdentityForwardedOrDelegated => {
                M5CredentialConsumerRecoveryAction::ReviewDelegationSource
            }
            Self::SessionOnlyOrPolicyBlocked => {
                M5CredentialConsumerRecoveryAction::StoreDurablyOrRequestPolicyGrant
            }
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is actionable
/// from the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialConsumerRecoveryAction {
    /// Use the handle reference; a raw copy is not offered on this surface.
    UseHandleReferenceNoRawCopy,
    /// Rotate or re-authenticate before treating the credential as usable.
    RotateOrReauthenticate,
    /// Review the delegation / forwarding source before treating it as a local credential.
    ReviewDelegationSource,
    /// Store the secret durably or request a policy grant before treating it as durable.
    StoreDurablyOrRequestPolicyGrant,
}

impl M5CredentialConsumerRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UseHandleReferenceNoRawCopy,
        Self::RotateOrReauthenticate,
        Self::ReviewDelegationSource,
        Self::StoreDurablyOrRequestPolicyGrant,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseHandleReferenceNoRawCopy => "use_handle_reference_no_raw_copy",
            Self::RotateOrReauthenticate => "rotate_or_reauthenticate",
            Self::ReviewDelegationSource => "review_delegation_source",
            Self::StoreDurablyOrRequestPolicyGrant => "store_durably_or_request_policy_grant",
        }
    }
}

/// An export caveat a consumer preserves when a component renders outside the authoritative
/// credential surface (a handle-only path, an expired/revoked credential, a forwarded/delegated
/// identity, or a session-only/policy-blocked secret).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialConsumerExportCaveat {
    /// Only a handle is available, so no raw secret is present in the export.
    HandleOnlyNoRawExport,
    /// The credential is expired or revoked, so it is not usable.
    ExpiredOrRevokedNotUsable,
    /// The identity is forwarded or delegated, so it is not a locally stored credential.
    ForwardedOrDelegatedNotLocal,
    /// The secret is session-only or policy-blocked, so it is not durably stored.
    SessionOnlyOrPolicyBlockedNotDurable,
}

impl M5CredentialConsumerExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::HandleOnlyNoRawExport,
        Self::ExpiredOrRevokedNotUsable,
        Self::ForwardedOrDelegatedNotLocal,
        Self::SessionOnlyOrPolicyBlockedNotDurable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HandleOnlyNoRawExport => "handle_only_no_raw_export",
            Self::ExpiredOrRevokedNotUsable => "expired_or_revoked_not_usable",
            Self::ForwardedOrDelegatedNotLocal => "forwarded_or_delegated_not_local",
            Self::SessionOnlyOrPolicyBlockedNotDurable => {
                "session_only_or_policy_blocked_not_durable"
            }
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor vocabulary is
/// preserved as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialClaimParityState {
    /// The descriptor vocabulary is preserved at full parity.
    ClaimsPreserved,
    /// The descriptor vocabulary is preserved, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5CredentialClaimParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [Self::ClaimsPreserved, Self::ClaimsAutoNarrowed];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimsPreserved => "claims_preserved",
            Self::ClaimsAutoNarrowed => "claims_auto_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5CredentialConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialConsumerAnatomyPart {
    /// The adopted component identity.
    ComponentIdentity,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The shared descriptor set.
    DescriptorSet,
    /// The parity-health cue.
    ParityHealthCue,
    /// The export-caveat list.
    ExportCaveats,
    /// The derived claim-parity verdict.
    ClaimParityVerdict,
    /// The auto-narrow banner (shown when narrowed).
    AutoNarrowBanner,
}

impl M5CredentialConsumerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealthCue,
        Self::ExportCaveats,
        Self::ClaimParityVerdict,
        Self::AutoNarrowBanner,
    ];

    /// The anatomy parts every consumer projection must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentIdentity => "component_identity",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealthCue => "parity_health_cue",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityVerdict => "claim_parity_verdict",
            Self::AutoNarrowBanner => "auto_narrow_banner",
        }
    }
}

/// A field the support / export packet carries so consumer parity is reconstructable from the
/// shared model. The fields in [`M5CredentialConsumerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialConsumerExportField {
    /// The consumer identity.
    Consumer,
    /// The adopted component family.
    ComponentFamily,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The descriptor set.
    DescriptorSet,
    /// The parity-health mode.
    ParityHealth,
    /// The export caveats.
    ExportCaveats,
    /// The claim-parity state.
    ClaimParityState,
    /// The narrowing reason (when narrowed).
    NarrowingReason,
}

impl M5CredentialConsumerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealth,
        Self::ExportCaveats,
        Self::ClaimParityState,
        Self::NarrowingReason,
    ];

    /// The export fields every consumer export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consumer => "consumer",
            Self::ComponentFamily => "component_family",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealth => "parity_health",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityState => "claim_parity_state",
            Self::NarrowingReason => "narrowing_reason",
        }
    }
}

/// A self-contained auto-narrow banner: the exact reason, the descriptors that stay preserved,
/// the export caveats, and the recovery action, so a narrowed rendering is understood from the
/// banner alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5CredentialConsumerNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5CredentialConsumerRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5CredentialComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5CredentialComponentFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5CredentialComponentDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5CredentialConsumerExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved descriptors, and
    /// the recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the credential component-binding resolver for one consumer/family adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5CredentialComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5CredentialComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor so storage
    /// mode, credential class, reveal posture, delegated identity, expiry, and export safety stay
    /// explicit.
    pub descriptor_families: Vec<M5CredentialComponentDescriptor>,
    /// The parity-health mode the binding renders under.
    pub parity_health: M5CredentialConsumerParityHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5CredentialConsumerExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentResolvedBinding {
    /// The consumer.
    pub consumer: M5CredentialComponentConsumer,
    /// The component family.
    pub component_family: M5CredentialComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5CredentialComponentDescriptor>,
    /// The parity-health mode.
    pub parity_health: M5CredentialConsumerParityHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5CredentialConsumerExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5CredentialClaimParityState,
    /// True when the binding renders under a weakened parity-health mode.
    pub is_narrowed: bool,
    /// True when the binding reflects an expired/revoked, forwarded/delegated, or
    /// session-only/policy-blocked credential. Such a binding must always be narrowed and never
    /// asserts that the credential is usable and locally stored.
    pub reflects_unusable_or_forwarded_state: bool,
    /// Hard invariant: whether this binding claims the credential is usable and locally stored.
    /// Only a full-parity binding may assert usable-and-local; every narrowed binding — and in
    /// particular any expired/revoked, forwarded/delegated, or session-only one — resolves this
    /// to `false`.
    pub asserts_credential_usable_and_local: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5CredentialComponentAutoNarrowBanner>,
}

/// Errors returned by [`resolve_credential_component_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5CredentialComponentBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5CredentialComponentBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5CredentialComponentBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "credential component binding error: {}",
            self.as_str()
        )
    }
}

impl Error for M5CredentialComponentBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the acceptance-criterion that storage mode,
/// credential class, reveal posture, delegated identity, expiry, and export safety stay explicit
/// on every surface. The claim-parity state is preserved at full parity and auto-narrowed under
/// any weakened parity-health mode, and a weakened mode always produces a self-contained banner
/// naming the exact reason and recovery action while keeping the descriptor vocabulary intact. An
/// expired/revoked, forwarded/delegated, or session-only/policy-blocked credential always narrows
/// and never asserts that the credential is usable and locally stored.
pub fn resolve_credential_component_binding(
    input: &M5CredentialComponentBindingInput,
) -> Result<M5CredentialComponentResolvedBinding, M5CredentialComponentBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5CredentialComponentBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5CredentialComponentDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5CredentialComponentDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5CredentialComponentBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5CredentialComponentBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future free-text extension
        // from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5CredentialComponentBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.parity_health.is_narrowed();
    let narrowing_reason = input.parity_health.narrowing_reason();
    let reflects_unusable_or_forwarded_state = narrowing_reason
        .is_some_and(M5CredentialConsumerNarrowingReason::reflects_unusable_or_forwarded);
    // Only a full-parity binding may assert the credential is usable and locally stored. Every
    // narrowed binding — and every expired/revoked, forwarded/delegated, or session-only one in
    // particular — is not asserted usable-and-local.
    let asserts_credential_usable_and_local = !is_narrowed;
    let claim_parity_state = if is_narrowed {
        M5CredentialClaimParityState::ClaimsAutoNarrowed
    } else {
        M5CredentialClaimParityState::ClaimsPreserved
    };

    let auto_narrow_banner = narrowing_reason.map(|reason| {
        let recovery_action = reason.recovery_action();
        let headline = format!(
            "Claim auto-narrowed: {} — {} renders {} with {} descriptor(s) preserved; recovery: {}",
            reason.phrase(),
            input.consumer.as_str(),
            input.component_family.as_str(),
            input.descriptor_families.len(),
            recovery_action.as_str()
        );
        M5CredentialComponentAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5CredentialComponentResolvedBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: family_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        parity_health: input.parity_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        reflects_unusable_or_forwarded_state,
        asserts_credential_usable_and_local,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet reconstructs
/// consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentBindingCase {
    /// The resolver input.
    pub input: M5CredentialComponentBindingInput,
    /// The resolved truth. Must equal `resolve_credential_component_binding(&input)`.
    pub resolved: M5CredentialComponentResolvedBinding,
}

impl M5CredentialComponentBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5CredentialComponentBindingInput) -> Self {
        let resolved =
            resolve_credential_component_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_credential_component_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the consumer
/// points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5CredentialComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's canonical schema
    /// ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal the family's
    /// canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local re-description of
    /// its facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5CredentialComponentBindingCase>,
}

impl M5CredentialComponentBinding {
    /// True when the binding points at the family's canonical refs and references the canonical
    /// family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == family_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref == family_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one credential consumer bound to the canonical component
/// families, the shared descriptor vocabulary, the parity-health modes, export caveats, parity
/// states, narrowing reasons, recovery actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentConsumerRow {
    /// Credential consumer.
    pub consumer: M5CredentialComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5CredentialQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 credential surface families that render / consume this projection.
    pub surface_families: Vec<M5CredentialSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5CredentialDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5CredentialConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5CredentialComponentDescriptor>,
    /// Parity-health modes this consumer distinguishes.
    pub parity_health_modes: Vec<M5CredentialConsumerParityHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5CredentialConsumerExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5CredentialClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5CredentialConsumerNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5CredentialConsumerRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5CredentialConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5CredentialAccessibilityRoute>,
    /// Credential subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5CredentialDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5CredentialComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the claims per surface. MUST be `false`.
    pub rewords_claims_per_surface: bool,
    /// Hard invariant: this consumer never invents a new credential grammar. MUST be `false`.
    pub invents_new_credential_grammar: bool,
    /// Hard invariant: this consumer never drops storage-mode, reveal, delegation, expiry, or
    /// export-safety truth when narrowed. MUST be `false`.
    pub drops_storage_reveal_delegation_expiry_or_export_truth_when_narrowed: bool,
    /// Hard invariant: this consumer never shows an expired/revoked, forwarded/delegated, or
    /// session-only credential as usable and locally stored. MUST be `false`.
    pub shows_unusable_or_forwarded_state_as_usable_and_local: bool,
    /// Hard invariant: this consumer never inherits a stronger label from a healthier profile
    /// instead of narrowing visibly. MUST be `false`.
    pub inherits_stronger_label_from_healthier_profile: bool,
    /// Hard invariant: this consumer never lets friendly "connected" / "signed in" wording
    /// conceal storage mode, forwarded/delegated identity, reveal posture, or export-safety
    /// limits. MUST be `false`.
    pub uses_friendly_connected_wording: bool,
}

impl M5CredentialComponentConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5CredentialConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5CredentialConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5CredentialConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5CredentialConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5CredentialComponentDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5CredentialComponentDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5CredentialComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5CredentialComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_claims_per_surface
            && !self.invents_new_credential_grammar
            && !self.drops_storage_reveal_delegation_expiry_or_export_truth_when_narrowed
            && !self.shows_unusable_or_forwarded_state_as_usable_and_local
            && !self.inherits_stronger_label_from_healthier_profile
            && !self.uses_friendly_connected_wording
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentConsumerVocabularySet {
    /// Credential-consumer tokens.
    pub consumers: Vec<String>,
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Descriptor tokens.
    pub descriptors: Vec<String>,
    /// Parity-health-mode tokens.
    pub parity_health_modes: Vec<String>,
    /// Export-caveat tokens.
    pub export_caveats: Vec<String>,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Recovery-action tokens.
    pub recovery_actions: Vec<String>,
    /// Claim-parity-state tokens.
    pub claim_parity_states: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5CredentialComponentConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5CredentialComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5CredentialComponentFamily::ALL, |v| v.as_str()),
            descriptors: tokens(&M5CredentialComponentDescriptor::ALL, |v| v.as_str()),
            parity_health_modes: tokens(&M5CredentialConsumerParityHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5CredentialConsumerExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5CredentialConsumerNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5CredentialConsumerRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5CredentialClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5CredentialConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5CredentialConsumerExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5CredentialAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new credential grammar.
    pub no_consumer_invents_new_grammar: bool,
    /// Storage mode, credential class, reveal posture, delegated identity, expiry, and export
    /// safety stay explicit everywhere.
    pub storage_class_reveal_delegation_expiry_export_explicit_on_every_surface: bool,
    /// A handle-only path, an expired/revoked credential, a forwarded/delegated identity, and a
    /// session-only/policy-blocked secret auto-narrow the claim.
    pub degraded_state_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// An expired/revoked, forwarded/delegated, or session-only credential never masquerades as a
    /// usable, locally stored one.
    pub unusable_or_forwarded_state_never_shown_as_usable_and_local: bool,
    /// Friendly "connected" / "signed in" wording never conceals storage mode, forwarded/delegated
    /// identity, reveal posture, or export-safety limits.
    pub no_friendly_connected_wording_conceals_storage_delegation_or_reveal: bool,
    /// The help / support / export surfaces present the same credential truth shown in-product.
    pub help_support_export_present_same_credential_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentConsumerProjection {
    /// Settings, request, database, registry, release, remote, AI, Help / docs, the support /
    /// export desk, and the export packet all adopt the shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The storage-mode descriptor reads a single canonical source.
    pub storage_mode_reads_single_source: bool,
    /// The credential-class descriptor reads a single canonical source.
    pub credential_class_reads_single_source: bool,
    /// The reveal-posture descriptor reads a single canonical source.
    pub reveal_posture_reads_single_source: bool,
    /// The delegated-identity descriptor reads a single canonical source.
    pub delegated_identity_reads_single_source: bool,
    /// The export-safety descriptor reads a single canonical source.
    pub export_safety_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting credential consumer audit.
    pub credential_consumer_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CredentialComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CredentialComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5CredentialComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CredentialComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CredentialComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CredentialComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CredentialComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CredentialComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 credential component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentConsumerPacket {
    /// Record kind; must equal [`M5_CREDENTIAL_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CREDENTIAL_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5CredentialComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CredentialComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CredentialComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CredentialComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CredentialComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CredentialComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CredentialComponentConsumerPacket {
    /// Builds an M5 credential component-consumer packet from stable-lane input.
    pub fn new(input: M5CredentialComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_CREDENTIAL_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_CREDENTIAL_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 credential component-consumer invariants.
    pub fn validate(&self) -> Vec<M5CredentialComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CREDENTIAL_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5CredentialComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CREDENTIAL_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5CredentialComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CredentialComponentConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_usability_honesty(self, &mut violations);
        validate_help_support_export_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 credential component consumer packet serializes"),
        ) {
            violations.push(M5CredentialComponentConsumerViolation::RawMaterialInExport);
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
            .expect("m5 credential component consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,qualification,owner,adopted_families,parity_health_modes,claim_parity_states,narrowing_reasons,export_fields,binding_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.component_bindings, |b| b.component_family.as_str()),
                join_tokens(&row.parity_health_modes, |v| v.as_str()),
                join_tokens(&row.claim_parity_states, |v| v.as_str()),
                join_tokens(&row.narrowing_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.component_bindings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Credential Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Credential consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Component families: {}\n",
            self.vocabulary_set.component_families.join(", ")
        ));
        out.push_str(&format!(
            "- Descriptors: {}\n",
            self.vocabulary_set.descriptors.join(", ")
        ));
        out.push_str(&format!(
            "- Parity-health modes: {}\n",
            self.vocabulary_set.parity_health_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Credential consumers\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Adopted families: {}\n",
                row.component_bindings.len()
            ));
            for binding in &row.component_bindings {
                out.push_str(&format!(
                    "    - `{}` → `{}` ({} worked binding(s))\n",
                    binding.component_family.as_str(),
                    binding.canonical_schema_ref,
                    binding.example_bindings.len()
                ));
                for case in &binding.example_bindings {
                    let banner = match &case.resolved.auto_narrow_banner {
                        Some(banner) => banner.reason.as_str(),
                        None => "full",
                    };
                    out.push_str(&format!(
                        "      - `{}` → `{}` (banner `{}`)\n",
                        case.resolved.parity_health.as_str(),
                        case.resolved.claim_parity_state.as_str(),
                        banner
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 credential component-consumer export.
#[derive(Debug)]
pub enum M5CredentialComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CredentialComponentConsumerViolation>),
}

impl fmt::Display for M5CredentialComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 credential component consumer export parse failed: {error}"
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
                    "m5 credential component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CredentialComponentConsumerArtifactError {}

/// Validation failures emitted by [`M5CredentialComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CredentialComponentConsumerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required credential consumer is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A consumer row does not keep every required descriptor.
    RequiredDescriptorMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no component bindings.
    ComponentBindingMissing,
    /// A component binding does not point to its canonical family.
    CanonicalRefMismatch,
    /// A component binding declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// A required component family is never adopted, or is adopted by only one consumer (reuse
    /// across surfaces unproven).
    ComponentFamilyReuseUnproven,
    /// No worked binding proves a narrowed rendering with a self-contained banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-parity rendering with preserved parity and no banner.
    ScopePreservedUnproven,
    /// No worked binding proves that an expired/revoked, forwarded/delegated, or session-only
    /// credential narrows and never asserts usable-and-local, or a binding does so incorrectly.
    UsabilityHonestyUnproven,
    /// A help / support / export consumer does not reference the canonical component schema.
    HelpSupportExportReferenceMissing,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5CredentialComponentConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::RequiredDescriptorMissing => "required_descriptor_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ComponentBindingMissing => "component_binding_missing",
            Self::CanonicalRefMismatch => "canonical_ref_mismatch",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ComponentFamilyReuseUnproven => "component_family_reuse_unproven",
            Self::NarrowingDisclosureUnproven => "narrowing_disclosure_unproven",
            Self::ScopePreservedUnproven => "scope_preserved_unproven",
            Self::UsabilityHonestyUnproven => "usability_honesty_unproven",
            Self::HelpSupportExportReferenceMissing => "help_support_export_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 credential component-consumer export.
pub fn current_stable_m5_credential_component_consumer_export(
) -> Result<M5CredentialComponentConsumerPacket, M5CredentialComponentConsumerArtifactError> {
    let packet: M5CredentialComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-credential-component-consumer-proof/support_export.json"
    )))
    .map_err(M5CredentialComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CredentialComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5CredentialComponentConsumerPacket,
    violations: &mut Vec<M5CredentialComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CREDENTIAL_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_CONSUMER_DOC_REF,
        M5_CREDENTIAL_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_CREDENTIAL_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_REF,
        SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_REF,
        BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_REF,
        ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CredentialComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5CredentialComponentConsumerPacket,
    violations: &mut Vec<M5CredentialComponentConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5CredentialComponentConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5CredentialComponentConsumerPacket,
    violations: &mut Vec<M5CredentialComponentConsumerViolation>,
) {
    let present: BTreeSet<M5CredentialComponentConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5CredentialComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5CredentialComponentConsumerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.parity_health_modes.is_empty()
            || row.export_caveats.is_empty()
            || row.claim_parity_states.is_empty()
            || row.narrowing_reasons.is_empty()
            || row.recovery_actions.is_empty()
        {
            violations.push(M5CredentialComponentConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5CredentialComponentConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5CredentialComponentConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5CredentialComponentConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5CredentialAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5CredentialComponentConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5CredentialComponentConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5CredentialComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5CredentialComponentConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5CredentialComponentConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5CredentialComponentConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5CredentialComponentConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5CredentialComponentConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5CredentialComponentConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct consumers — the
/// acceptance-criterion proof that the families are reusable components rather than one sign-in
/// view plus a few isolated export objects.
fn validate_family_reuse(
    packet: &M5CredentialComponentConsumerPacket,
    violations: &mut Vec<M5CredentialComponentConsumerViolation>,
) {
    for family in M5CredentialComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5CredentialComponentConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering whose banner
/// carries a specific reason, a recovery action, and a non-empty set of preserved descriptors —
/// the acceptance-criterion example that a consumer which cannot preserve parity is visibly
/// narrowed rather than inheriting stronger labels from healthier profiles.
fn validate_narrowing_disclosure(
    packet: &M5CredentialComponentConsumerPacket,
    violations: &mut Vec<M5CredentialComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        case.resolved.is_narrowed
            && case
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|banner| {
                    !banner.headline.trim().is_empty() && !banner.preserved_descriptors.is_empty()
                })
    });
    if !proven {
        violations.push(M5CredentialComponentConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-parity rendering with
/// preserved parity and no banner — the acceptance-criterion example that full-parity consumers
/// keep the descriptor vocabulary without a spurious narrowing note.
fn validate_scope_preserved(
    packet: &M5CredentialComponentConsumerPacket,
    violations: &mut Vec<M5CredentialComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5CredentialClaimParityState::ClaimsPreserved
    });
    if !proven {
        violations.push(M5CredentialComponentConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every worked binding that reflects an expired/revoked, forwarded/delegated, or
/// session-only/policy-blocked credential must be narrowed and must not assert usable-and-local,
/// and at least one such binding must be present — the acceptance-criterion that an unusable or
/// forwarded credential no longer masquerades as a usable, locally stored one on any claimed
/// consumer.
fn validate_usability_honesty(
    packet: &M5CredentialComponentConsumerPacket,
    violations: &mut Vec<M5CredentialComponentConsumerViolation>,
) {
    let mut proven = false;
    for case in all_cases(packet) {
        let resolved = &case.resolved;
        if resolved.reflects_unusable_or_forwarded_state {
            // An unusable / forwarded binding that claims usable-and-local, or fails to narrow,
            // breaks the honesty invariant.
            if resolved.asserts_credential_usable_and_local
                || !resolved.is_narrowed
                || resolved.claim_parity_state != M5CredentialClaimParityState::ClaimsAutoNarrowed
            {
                violations.push(M5CredentialComponentConsumerViolation::UsabilityHonestyUnproven);
                return;
            }
            proven = true;
        }
    }
    if !proven {
        violations.push(M5CredentialComponentConsumerViolation::UsabilityHonestyUnproven);
    }
}

/// The help / support / export consumers must reference the canonical component schema for each
/// family they adopt — the acceptance-criterion that a help, support, or export lane can never
/// drift from the product truth.
fn validate_help_support_export_reference(
    packet: &M5CredentialComponentConsumerPacket,
    violations: &mut Vec<M5CredentialComponentConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_help_support_or_export() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5CredentialComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations
                .push(M5CredentialComponentConsumerViolation::HelpSupportExportReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5CredentialComponentConsumerPacket,
    violations: &mut Vec<M5CredentialComponentConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_grammar,
        review.storage_class_reveal_delegation_expiry_export_explicit_on_every_surface,
        review.degraded_state_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.unusable_or_forwarded_state_never_shown_as_usable_and_local,
        review.no_friendly_connected_wording_conceals_storage_delegation_or_reveal,
        review.help_support_export_present_same_credential_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5CredentialComponentConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CredentialComponentConsumerPacket,
    violations: &mut Vec<M5CredentialComponentConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.storage_mode_reads_single_source,
        projection.credential_class_reads_single_source,
        projection.reveal_posture_reads_single_source,
        projection.delegated_identity_reads_single_source,
        projection.export_safety_reads_single_source,
    ] {
        if !ok {
            violations.push(M5CredentialComponentConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CredentialComponentConsumerPacket,
    violations: &mut Vec<M5CredentialComponentConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CredentialComponentConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CredentialComponentConsumerPacket,
    violations: &mut Vec<M5CredentialComponentConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.credential_consumer_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CredentialComponentConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5CredentialComponentConsumerPacket,
) -> impl Iterator<Item = &M5CredentialComponentBindingCase> {
    packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|binding| binding.example_bindings.iter())
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden raw material.
///
/// The credential vocabulary uses "secret" and "api_key" pervasively as controlled tokens (a
/// secret-access prompt, an API-key credential class, a raw-secret-excluded export boundary), so
/// those words are *not* treated as forbidden here. This guard targets the raw-value shapes that
/// must never appear in an export: a password / passphrase, a bearer token value, a URL with an
/// embedded credential, or a PEM private-key header.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
