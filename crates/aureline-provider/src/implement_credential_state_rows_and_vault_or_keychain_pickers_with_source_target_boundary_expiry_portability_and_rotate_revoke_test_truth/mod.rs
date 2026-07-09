//! Credential-state rows and vault-or-keychain pickers carrying storage mode,
//! source class, target boundary, expiry / rotation / revoke lifecycle, health
//! state, auditability, keyboard-complete rotate/revoke/test actions, available
//! source, access scope, reveal policy, portability / export note, and
//! open-source-of-truth actions.
//!
//! This module narrows two components frozen in
//! [`crate::freeze_the_m5_credential_component_matrix`] — the `credential_state_row`
//! and the `vault_or_keychain_picker` — into one implemented, export-safe packet
//! with two co-equal control vectors. Together they make everyday credential state
//! visible and explainable *before* a user opens a secondary auth or recovery flow:
//! a user can tell where authority lives and what boundary it applies to without
//! reading logs or provider docs.
//!
//! A [`CredentialStateRow`] always names its storage mode, credential (source)
//! class, reveal posture, and the target boundary it applies to (provider,
//! registry, request, database, remote, or release), and its health state is
//! *derived* from the credential lifecycle state rather than asserted: a revoked or
//! expired credential can never read as healthy, so a user sees expiry, rotation,
//! and revoke truth directly. It always names its auditability and always offers
//! keyboard-complete rotate, revoke, and test actions.
//!
//! A [`VaultOrKeychainPicker`] always names the available source it will write to,
//! its access scope, its reveal policy, and a *derived* portability / export note:
//! a store-export-blocked or session-only store can never read as freely portable,
//! and no picker normalizes raw-secret handling. It always offers an
//! open-source-of-truth action so a user can inspect the store of record.
//!
//! The storage modes ([`M5CredentialStorageMode`]), credential classes
//! ([`M5CredentialClass`]), reveal postures ([`M5CredentialRevealPosture`]),
//! lifecycle states ([`M5CredentialLifecycleState`]), store capabilities
//! ([`M5CredentialStoreCapability`]), degraded states
//! ([`M5CredentialDegradedState`]), required labels
//! ([`M5CredentialRequiredLabel`]), surface families
//! ([`M5CredentialSurfaceFamily`]), deployment lines
//! ([`M5CredentialDeploymentLine`]), consumer surfaces
//! ([`M5CredentialConsumerSurface`]), accessibility routes
//! ([`M5CredentialAccessibilityRoute`]), and downgrade triggers
//! ([`M5CredentialDowngradeTrigger`]) are reused directly from the frozen matrix, so
//! this lane never invents a parallel credential vocabulary. It mints new vocabulary
//! only for what that matrix left implicit about these two controls: the target
//! boundary, the derived health class, the keyboard-complete state-row actions, the
//! vault access scope, the derived portability class, and the vault picker actions.
//!
//! Raw secret values, pasted tokens, passphrases, and private endpoints stay
//! outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-credential-state-row-vault-picker-controls.schema.json`](../../../../schemas/ui/m5-credential-state-row-vault-picker-controls.schema.json).
//! The contract doc is
//! [`docs/security/implement_credential_state_rows_and_vault_or_keychain_pickers.md`](../../../../docs/security/implement_credential_state_rows_and_vault_or_keychain_pickers.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_credential_state_row_vault_picker_controls,
    seeded_credential_state_row_vault_picker_controls_credential_state_row_revoked,
    seeded_credential_state_row_vault_picker_controls_vault_picker_export_blocked,
    CREDENTIAL_STATE_ROW_VAULT_PICKER_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The storage mode, credential class, reveal posture, lifecycle state, store
// capability, degraded state, required labels, surface family, deployment line,
// consumer surface, accessibility route, and downgrade triggers are frozen once,
// in the credential component matrix. This lane reuses them verbatim so it never
// invents a parallel credential vocabulary.
use crate::freeze_the_m5_credential_component_matrix::{
    M5CredentialAccessibilityRoute, M5CredentialClass, M5CredentialComponentFamily,
    M5CredentialConsumerSurface, M5CredentialDegradedState, M5CredentialDeploymentLine,
    M5CredentialDowngradeTrigger, M5CredentialLifecycleState, M5CredentialRequiredLabel,
    M5CredentialRevealPosture, M5CredentialStorageMode, M5CredentialStoreCapability,
    M5CredentialSurfaceFamily, M5_CREDENTIAL_COMPONENT_DOC_REF,
    M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_PICKER_REF,
    M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_STATE_REF, M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
    M5_CREDENTIAL_STATE_ROW_SCHEMA_REF, M5_VAULT_KEYCHAIN_PICKER_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`CredentialStateRowVaultPickerControlsPacket`].
pub const CREDENTIAL_STATE_ROW_VAULT_PICKER_RECORD_KIND: &str =
    "credential_state_row_vault_picker_controls";

/// Schema version for credential-state-row / vault-picker control records.
pub const CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_REF: &str =
    "schemas/ui/m5-credential-state-row-vault-picker-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const CREDENTIAL_STATE_ROW_VAULT_PICKER_DOC_REF: &str =
    "docs/security/implement_credential_state_rows_and_vault_or_keychain_pickers.md";

/// Repo-relative path of the protected fixture directory.
pub const CREDENTIAL_STATE_ROW_VAULT_PICKER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-credential-state-row-vault-picker-controls";

/// Repo-relative path of the checked support-export artifact.
pub const CREDENTIAL_STATE_ROW_VAULT_PICKER_ARTIFACT_REF: &str =
    "artifacts/release/m5-credential-state-row-vault-picker-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const CREDENTIAL_STATE_ROW_VAULT_PICKER_SUMMARY_REF: &str =
    "artifacts/release/m5-credential-state-row-vault-picker-proof/summary.md";

// ---- credential-state-row vocabulary ------------------------------------

/// Target boundary a credential-state row applies to.
///
/// This is the "what boundary it applies to" axis: a credential never leaves its
/// blast radius implicit, so a user can tell that a token authorizes the registry
/// rather than the database directly in the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialTargetBoundary {
    /// Authorizes a provider (issue tracker, VCS host, cloud).
    Provider,
    /// Authorizes a package / artifact registry.
    Registry,
    /// Authorizes outbound request / API access.
    Request,
    /// Authorizes a database / datastore connection.
    Database,
    /// Authorizes a remote-target / SSH / attach connection.
    Remote,
    /// Authorizes a package / release publish or signing boundary.
    Release,
}

impl CredentialTargetBoundary {
    /// Every target boundary, in declaration order.
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

/// Derived health class a credential-state row may present.
///
/// This is the row honesty axis: the class is derived from the credential lifecycle
/// state, never asserted, so a revoked or expired credential can never present as
/// healthy in a list surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialHealthClass {
    /// Active and current.
    Healthy,
    /// A refresh or rotation is due; still usable but needs attention.
    AttentionNeeded,
    /// The credential has been revoked and is no longer valid.
    Revoked,
    /// The credential has expired and is no longer valid.
    Expired,
    /// Superseded by a newer credential.
    Superseded,
}

impl CredentialHealthClass {
    /// Every health class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Healthy,
        Self::AttentionNeeded,
        Self::Revoked,
        Self::Expired,
        Self::Superseded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::AttentionNeeded => "attention_needed",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
            Self::Superseded => "superseded",
        }
    }
}

/// One keyboard-complete default action a credential-state row offers, so a row
/// never hides its rotate, revoke, or test affordance behind a pointer-only
/// gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialStateRowAction {
    /// Copy the handle reference (never the raw secret).
    CopyHandleReference,
    /// Rotate the credential.
    Rotate,
    /// Revoke the credential.
    Revoke,
    /// Test the credential against its target boundary.
    Test,
    /// Open the audit / activity trail for this credential.
    OpenAudit,
    /// Export the row as export-safe credential evidence.
    ExportRow,
}

impl CredentialStateRowAction {
    /// Every row action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CopyHandleReference,
        Self::Rotate,
        Self::Revoke,
        Self::Test,
        Self::OpenAudit,
        Self::ExportRow,
    ];

    /// The default actions every keyboard-complete row must offer.
    pub const MANDATORY: [Self; 3] = [Self::Rotate, Self::Revoke, Self::Test];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CopyHandleReference => "copy_handle_reference",
            Self::Rotate => "rotate",
            Self::Revoke => "revoke",
            Self::Test => "test",
            Self::OpenAudit => "open_audit",
            Self::ExportRow => "export_row",
        }
    }
}

/// Disclosures a credential-state row must carry, derived from the lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CredentialStateRowDisclosure {
    /// The derived health class this row may present.
    pub health_class: CredentialHealthClass,
    /// Whether the credential is healthy (active and current).
    pub is_healthy: bool,
    /// Whether the credential is revoked or expired (no longer usable).
    pub is_revoked_or_expired: bool,
    /// Whether the row must carry an explicit attention (refresh / rotation) note.
    pub needs_attention_note: bool,
    /// Whether the row must carry an explicit revoked note.
    pub needs_revoked_note: bool,
    /// Whether the row must carry an explicit expired note.
    pub needs_expired_note: bool,
    /// Whether the row must carry an explicit superseded note.
    pub needs_superseded_note: bool,
}

/// Resolves the health truth a credential-state row may present.
///
/// An active credential is healthy. A refresh-needed or rotation-due credential
/// needs attention but is still usable. A revoked credential is revoked, an expired
/// credential is expired, and a superseded credential is superseded — none of which
/// can ever read as healthy.
pub fn resolve_credential_health(
    lifecycle_state: M5CredentialLifecycleState,
) -> CredentialStateRowDisclosure {
    use CredentialHealthClass as Health;
    use M5CredentialLifecycleState as Life;

    let health_class = match lifecycle_state {
        Life::ActiveCurrent => Health::Healthy,
        Life::RefreshNeeded | Life::RotationDue => Health::AttentionNeeded,
        Life::Revoked => Health::Revoked,
        Life::Expired => Health::Expired,
        Life::Superseded => Health::Superseded,
    };

    CredentialStateRowDisclosure {
        health_class,
        is_healthy: matches!(health_class, Health::Healthy),
        is_revoked_or_expired: matches!(health_class, Health::Revoked | Health::Expired),
        needs_attention_note: matches!(health_class, Health::AttentionNeeded),
        needs_revoked_note: matches!(health_class, Health::Revoked),
        needs_expired_note: matches!(health_class, Health::Expired),
        needs_superseded_note: matches!(health_class, Health::Superseded),
    }
}

/// A credential-state row naming storage, source class, target boundary, health,
/// auditability, and rotate/revoke/test actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialStateRow {
    /// Frozen component this control implements; must be `credential_state_row`.
    pub component: M5CredentialComponentFamily,
    /// Stable row id.
    pub row_id: String,
    /// Credential label / what credential object this row represents; required.
    pub credential_label: String,
    /// Where the secret actually lives, reused from the frozen matrix.
    pub storage_mode: M5CredentialStorageMode,
    /// Credential (source) class, reused from the frozen matrix.
    pub credential_class: M5CredentialClass,
    /// Handle-only-versus-raw-reveal posture, reused from the frozen matrix.
    pub reveal_posture: M5CredentialRevealPosture,
    /// Target boundary this credential applies to.
    pub target_boundary: CredentialTargetBoundary,
    /// Human-readable target-boundary label; required and non-empty.
    pub target_label: String,
    /// Credential lifecycle state, reused from the frozen matrix.
    pub lifecycle_state: M5CredentialLifecycleState,
    /// Derived health class (must equal the resolved class).
    pub health_class: CredentialHealthClass,
    /// Whether the row claims the credential is healthy (must equal the derived truth).
    pub claims_healthy: bool,
    /// Attention note; required when the credential needs a refresh / rotation.
    pub attention_note: String,
    /// Revoked note; required when the credential is revoked.
    pub revoked_note: String,
    /// Expired note; required when the credential is expired.
    pub expired_note: String,
    /// Superseded note; required when the credential is superseded.
    pub superseded_note: String,
    /// Storage-mode / reveal-posture note; always required so storage stays explicit.
    pub storage_and_reveal_note: String,
    /// Whether this credential's actions are auditable.
    pub is_auditable: bool,
    /// Audit note; required when the credential is auditable.
    pub audit_note: String,
    /// Keyboard-complete default actions (must include the mandatory rotate/revoke/test).
    pub default_actions: Vec<CredentialStateRowAction>,
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
    /// Hard invariant: friendly "connected" wording never conceals storage, reveal,
    /// or lifecycle truth. MUST be `false`.
    pub uses_friendly_connected_wording: bool,
}

impl CredentialStateRow {
    /// Health disclosures this row must carry, derived from the lifecycle state.
    pub fn health_disclosure(&self) -> CredentialStateRowDisclosure {
        resolve_credential_health(self.lifecycle_state)
    }

    /// Whether the row offers every mandatory keyboard-complete default action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<CredentialStateRowAction> =
            self.default_actions.iter().copied().collect();
        CredentialStateRowAction::MANDATORY
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

// ---- vault-or-keychain-picker vocabulary --------------------------------

/// Access scope a vault-or-keychain picker offers.
///
/// This is the "access scope" axis a user must be able to tell apart: a local
/// device store, a per-user profile store, a team-shared store, an org-managed
/// store, or a session-only store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VaultAccessScope {
    /// Local to this device only.
    DeviceLocal,
    /// Scoped to this user's profile.
    UserProfile,
    /// Shared with a team.
    TeamShared,
    /// Managed by the organization.
    OrgManaged,
    /// Available for this session only.
    SessionOnly,
}

impl VaultAccessScope {
    /// Every access scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DeviceLocal,
        Self::UserProfile,
        Self::TeamShared,
        Self::OrgManaged,
        Self::SessionOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeviceLocal => "device_local",
            Self::UserProfile => "user_profile",
            Self::TeamShared => "team_shared",
            Self::OrgManaged => "org_managed",
            Self::SessionOnly => "session_only",
        }
    }
}

/// Derived portability class a vault-or-keychain picker may present.
///
/// This is the picker honesty axis: the class is derived from the storage mode,
/// store capability, and reveal policy, never asserted, so a store-export-blocked
/// or session-only store can never present as freely portable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VaultPortabilityClass {
    /// Portable: the stored material can move with the user across restart.
    Portable,
    /// Only a handle reference is portable; the raw secret never leaves the store.
    HandleReferenceOnly,
    /// Export is blocked by the store itself.
    ExportBlocked,
    /// Session-only and non-portable; nothing survives exit.
    SessionOnlyNonPortable,
}

impl VaultPortabilityClass {
    /// Every portability class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Portable,
        Self::HandleReferenceOnly,
        Self::ExportBlocked,
        Self::SessionOnlyNonPortable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Portable => "portable",
            Self::HandleReferenceOnly => "handle_reference_only",
            Self::ExportBlocked => "export_blocked",
            Self::SessionOnlyNonPortable => "session_only_non_portable",
        }
    }
}

/// One keyboard-complete default action a vault-or-keychain picker offers, so a
/// picker never hides its open-source-of-truth affordance behind a pointer-only
/// gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VaultPickerAction {
    /// Open the store of record (the source of truth) to inspect it.
    OpenSourceOfTruth,
    /// Select this store as the write target.
    SelectStore,
    /// Copy the store's handle reference (never the raw secret).
    CopyStoreReference,
    /// Review the store's capabilities.
    ReviewCapability,
    /// Export the picker as export-safe evidence.
    ExportPicker,
}

impl VaultPickerAction {
    /// Every picker action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenSourceOfTruth,
        Self::SelectStore,
        Self::CopyStoreReference,
        Self::ReviewCapability,
        Self::ExportPicker,
    ];

    /// The default actions every keyboard-complete picker must offer.
    pub const MANDATORY: [Self; 1] = [Self::OpenSourceOfTruth];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenSourceOfTruth => "open_source_of_truth",
            Self::SelectStore => "select_store",
            Self::CopyStoreReference => "copy_store_reference",
            Self::ReviewCapability => "review_capability",
            Self::ExportPicker => "export_picker",
        }
    }
}

/// Disclosures a vault-or-keychain picker must carry, derived from storage,
/// capability, and reveal policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VaultPickerDisclosure {
    /// The derived portability class this picker may present.
    pub portability_class: VaultPortabilityClass,
    /// Whether the store is freely portable.
    pub is_portable: bool,
    /// Whether export is blocked by the store.
    pub is_export_blocked: bool,
    /// Whether the picker must carry an explicit export-blocked note.
    pub needs_export_blocked_note: bool,
    /// Whether the picker must carry an explicit session-only note.
    pub needs_session_only_note: bool,
    /// Whether the picker must carry an explicit handle-only note.
    pub needs_handle_only_note: bool,
}

/// Resolves the portability truth a vault-or-keychain picker may present.
///
/// A store that blocks its own export is export-blocked. A session-memory-only or
/// session-only store is session-only and non-portable. A broker-handle store or a
/// handle-only / never-revealed / policy-blocked reveal policy is
/// handle-reference-only. Anything else is portable, so an export-blocked or
/// session-only store can never claim to be portable.
pub fn resolve_vault_portability(
    storage_mode: M5CredentialStorageMode,
    store_capabilities: &[M5CredentialStoreCapability],
    reveal_policy: M5CredentialRevealPosture,
) -> VaultPickerDisclosure {
    use M5CredentialRevealPosture as Reveal;
    use M5CredentialStorageMode as Storage;
    use M5CredentialStoreCapability as Capability;
    use VaultPortabilityClass as Portability;

    let export_blocked = store_capabilities.contains(&Capability::StoreExportBlocked);
    let session_only = matches!(storage_mode, Storage::SessionMemoryOnly)
        || store_capabilities.contains(&Capability::SessionOnly);
    let handle_only = matches!(storage_mode, Storage::SecretBrokerHandle)
        || matches!(
            reveal_policy,
            Reveal::HandleOnly | Reveal::NeverRevealed | Reveal::PolicyBlockedReveal
        );

    let portability_class = if export_blocked {
        Portability::ExportBlocked
    } else if session_only {
        Portability::SessionOnlyNonPortable
    } else if handle_only {
        Portability::HandleReferenceOnly
    } else {
        Portability::Portable
    };

    VaultPickerDisclosure {
        portability_class,
        is_portable: matches!(portability_class, Portability::Portable),
        is_export_blocked: export_blocked,
        needs_export_blocked_note: matches!(portability_class, Portability::ExportBlocked),
        needs_session_only_note: matches!(portability_class, Portability::SessionOnlyNonPortable),
        needs_handle_only_note: matches!(portability_class, Portability::HandleReferenceOnly),
    }
}

/// A vault-or-keychain picker naming available source, access scope, reveal policy,
/// portability, and open-source-of-truth actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultOrKeychainPicker {
    /// Frozen component this control implements; must be `vault_or_keychain_picker`.
    pub component: M5CredentialComponentFamily,
    /// Stable picker id.
    pub picker_id: String,
    /// Available-source / store label; required and non-empty.
    pub store_label: String,
    /// Where the store keeps its material, reused from the frozen matrix.
    pub storage_mode: M5CredentialStorageMode,
    /// Access scope this store offers.
    pub access_scope: VaultAccessScope,
    /// Human-readable access-scope label; required and non-empty.
    pub access_scope_label: String,
    /// Store capabilities this picker names, reused from the frozen matrix.
    pub store_capabilities: Vec<M5CredentialStoreCapability>,
    /// Reveal policy this store applies, reused from the frozen matrix.
    pub reveal_policy: M5CredentialRevealPosture,
    /// Derived portability class (must equal the resolved class).
    pub portability_class: VaultPortabilityClass,
    /// Whether the picker claims the store is portable (must equal the derived truth).
    pub claims_portable: bool,
    /// Portability / export note; always required so portability stays explicit.
    pub portability_note: String,
    /// Export-blocked note; required when the store blocks export.
    pub export_blocked_note: String,
    /// Session-only note; required when the store is session-only.
    pub session_only_note: String,
    /// Handle-only note; required when only a handle reference is portable.
    pub handle_only_note: String,
    /// Storage-mode / reveal-posture note; always required so storage stays explicit.
    pub storage_and_reveal_note: String,
    /// Keyboard-complete default actions (must include the open-source-of-truth action).
    pub default_actions: Vec<VaultPickerAction>,
    /// Degraded states this picker can name (required, matching the frozen matrix).
    pub degraded_states: Vec<M5CredentialDegradedState>,
    /// Mandatory labels this picker can show (must include the mandatory labels).
    pub required_labels: Vec<M5CredentialRequiredLabel>,
    /// Claimed M5 surface families that render this picker.
    pub surface_families: Vec<M5CredentialSurfaceFamily>,
    /// Deployment lines this picker keeps the same truth across.
    pub deployment_lines: Vec<M5CredentialDeploymentLine>,
    /// Non-visual accessibility routes this picker offers.
    pub accessibility_routes: Vec<M5CredentialAccessibilityRoute>,
    /// Credential subsystems that consume this picker's projection.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this picker.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks storage mode or reveal posture. MUST be `false`.
    pub masks_storage_or_reveal_posture: bool,
    /// Hard invariant: never implies a raw secret is export-safe (never normalizes
    /// raw-secret handling). MUST be `false`.
    pub implies_raw_secret_exportable: bool,
    /// Hard invariant: friendly "connected" wording never conceals storage, reveal,
    /// or portability truth. MUST be `false`.
    pub uses_friendly_connected_wording: bool,
}

impl VaultOrKeychainPicker {
    /// Portability disclosures this picker must carry, derived from storage, capability,
    /// and reveal policy.
    pub fn portability_disclosure(&self) -> VaultPickerDisclosure {
        resolve_vault_portability(
            self.storage_mode,
            &self.store_capabilities,
            self.reveal_policy,
        )
    }

    /// Whether the picker offers every mandatory keyboard-complete default action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<VaultPickerAction> = self.default_actions.iter().copied().collect();
        VaultPickerAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the picker declares all mandatory labels.
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
pub struct CredentialStateRowVaultPickerTrustReview {
    /// The credential-state row names its storage mode and reveal posture.
    pub credential_state_shows_storage_and_reveal_posture: bool,
    /// The credential-state row names the target boundary it applies to.
    pub credential_state_shows_target_boundary: bool,
    /// The health state is derived from the lifecycle state, never asserted.
    pub health_state_derived_never_asserted: bool,
    /// A revoked or expired credential never reads as healthy.
    pub revoked_or_expired_never_reads_as_healthy: bool,
    /// The rotate, revoke, and test actions are always present and keyboard-complete.
    pub rotate_revoke_test_actions_present: bool,
    /// The credential's auditability is always named.
    pub auditability_always_named: bool,
    /// The vault picker names its available source and access scope.
    pub vault_picker_shows_available_source_and_scope: bool,
    /// The vault picker names its reveal policy.
    pub vault_picker_shows_reveal_policy: bool,
    /// The portability note is derived from storage / capability, never asserted.
    pub portability_note_derived_never_asserted: bool,
    /// Export-blocked and session-only stores stay explicit.
    pub export_blocked_and_session_only_explicit: bool,
    /// Raw-secret handling is never normalized on any surface.
    pub raw_secret_handling_never_normalized: bool,
    /// No friendly "connected" wording conceals storage mode or delegation.
    pub no_friendly_wording_conceals_storage_or_delegation: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl CredentialStateRowVaultPickerTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.credential_state_shows_storage_and_reveal_posture
            && self.credential_state_shows_target_boundary
            && self.health_state_derived_never_asserted
            && self.revoked_or_expired_never_reads_as_healthy
            && self.rotate_revoke_test_actions_present
            && self.auditability_always_named
            && self.vault_picker_shows_available_source_and_scope
            && self.vault_picker_shows_reveal_policy
            && self.portability_note_derived_never_asserted
            && self.export_blocked_and_session_only_explicit
            && self.raw_secret_handling_never_normalized
            && self.no_friendly_wording_conceals_storage_or_delegation
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialStateRowVaultPickerConsumerProjection {
    /// Credential rows let a user tell where authority lives and what boundary it
    /// applies to without reading logs or provider docs.
    pub credential_rows_show_authority_and_boundary_without_docs: bool,
    /// Storage-mode clarity is preserved across every surface.
    pub storage_mode_clarity_preserved_across_surfaces: bool,
    /// The vault picker shows source, scope, and portability inline.
    pub vault_picker_shows_source_scope_and_portability_inline: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_control_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl CredentialStateRowVaultPickerConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.credential_rows_show_authority_and_boundary_without_docs
            && self.storage_mode_clarity_preserved_across_surfaces
            && self.vault_picker_shows_source_scope_and_portability_inline
            && self.cli_headless_shows_control_truth
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialStateRowVaultPickerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`CredentialStateRowVaultPickerControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialStateRowVaultPickerControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Credential-state rows.
    pub credential_state_rows: Vec<CredentialStateRow>,
    /// Vault-or-keychain pickers.
    pub vault_pickers: Vec<VaultOrKeychainPicker>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CredentialDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Trust review block.
    pub trust_review: CredentialStateRowVaultPickerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: CredentialStateRowVaultPickerConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CredentialStateRowVaultPickerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe credential-state-row / vault-picker controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialStateRowVaultPickerControlsPacket {
    /// Record kind; must equal [`CREDENTIAL_STATE_ROW_VAULT_PICKER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Credential-state rows.
    pub credential_state_rows: Vec<CredentialStateRow>,
    /// Vault-or-keychain pickers.
    pub vault_pickers: Vec<VaultOrKeychainPicker>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CredentialDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Trust review block.
    pub trust_review: CredentialStateRowVaultPickerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: CredentialStateRowVaultPickerConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CredentialStateRowVaultPickerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl CredentialStateRowVaultPickerControlsPacket {
    /// Builds a credential-state-row / vault-picker controls packet from stable-lane input.
    pub fn new(input: CredentialStateRowVaultPickerControlsPacketInput) -> Self {
        Self {
            record_kind: CREDENTIAL_STATE_ROW_VAULT_PICKER_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            credential_state_rows: input.credential_state_rows,
            vault_pickers: input.vault_pickers,
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

    /// Validates the credential-state-row / vault-picker control invariants.
    pub fn validate(&self) -> Vec<CredentialStateRowVaultPickerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != CREDENTIAL_STATE_ROW_VAULT_PICKER_RECORD_KIND {
            violations.push(CredentialStateRowVaultPickerViolation::WrongRecordKind);
        }
        if self.schema_version != CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_VERSION {
            violations.push(CredentialStateRowVaultPickerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CredentialStateRowVaultPickerViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_credential_state_rows(self, &mut violations);
        validate_vault_pickers(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(CredentialStateRowVaultPickerViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(CredentialStateRowVaultPickerViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(CredentialStateRowVaultPickerViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("credential state row vault picker packet serializes"),
        ) {
            violations.push(CredentialStateRowVaultPickerViolation::RawBoundaryMaterialInExport);
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
            .expect("credential state row vault picker packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("control,id,scope_or_boundary,storage_mode,state_or_policy,derived,healthy_or_portable\n");
        for row in &self.credential_state_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "credential_state_row",
                csv_field(&row.row_id),
                row.target_boundary.as_str(),
                row.storage_mode.as_str(),
                row.lifecycle_state.as_str(),
                row.health_disclosure().health_class.as_str(),
                row.health_disclosure().is_healthy,
            ));
        }
        for picker in &self.vault_pickers {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "vault_or_keychain_picker",
                csv_field(&picker.picker_id),
                picker.access_scope.as_str(),
                picker.storage_mode.as_str(),
                picker.reveal_policy.as_str(),
                picker.portability_disclosure().portability_class.as_str(),
                picker.portability_disclosure().is_portable,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let unhealthy = self
            .credential_state_rows
            .iter()
            .filter(|row| !row.health_disclosure().is_healthy)
            .count();
        let non_portable = self
            .vault_pickers
            .iter()
            .filter(|picker| !picker.portability_disclosure().is_portable)
            .count();

        let mut out = String::new();
        out.push_str("# Credential-state rows and vault-or-keychain pickers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Credential-state rows: {} ({} not healthy)\n",
            self.credential_state_rows.len(),
            unhealthy
        ));
        out.push_str(&format!(
            "- Vault/keychain pickers: {} ({} not freely portable)\n",
            self.vault_pickers.len(),
            non_portable
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Credential-state rows\n\n");
        for row in &self.credential_state_rows {
            out.push_str(&format!(
                "- **{}** ({}) — storage `{}`, target `{}` [{}] → `{}`\n",
                row.credential_label,
                row.credential_class.as_str(),
                row.storage_mode.as_str(),
                row.target_boundary.as_str(),
                row.lifecycle_state.as_str(),
                row.health_disclosure().health_class.as_str(),
            ));
        }

        out.push_str("\n## Vault/keychain pickers\n\n");
        for picker in &self.vault_pickers {
            out.push_str(&format!(
                "- **{}** — scope `{}`, storage `{}`, reveal `{}` → `{}`\n",
                picker.store_label,
                picker.access_scope.as_str(),
                picker.storage_mode.as_str(),
                picker.reveal_policy.as_str(),
                picker.portability_disclosure().portability_class.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in credential-state-row / vault-picker export.
#[derive(Debug)]
pub enum CredentialStateRowVaultPickerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CredentialStateRowVaultPickerViolation>),
}

impl fmt::Display for CredentialStateRowVaultPickerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "credential state row vault picker export parse failed: {error}"
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
                    "credential state row vault picker export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CredentialStateRowVaultPickerArtifactError {}

/// Validation failures emitted by [`CredentialStateRowVaultPickerControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CredentialStateRowVaultPickerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No credential-state rows are present.
    CredentialStateRowsMissing,
    /// A credential-state row is incomplete.
    CredentialStateRowIncomplete,
    /// A credential-state row carries the wrong frozen component class.
    CredentialStateRowWrongComponentClass,
    /// A credential-state row does not name its target boundary.
    TargetBoundaryMissing,
    /// A credential-state row misrepresents its derived health state.
    HealthStateMisrepresented,
    /// A credential-state row does not name its storage mode / reveal posture.
    StorageAndRevealNoteMissing,
    /// A credential needing attention does not name its refresh / rotation state.
    AttentionNoteMissing,
    /// A revoked credential does not name its revoked state.
    RevokedNoteMissing,
    /// An expired credential does not name its expired state.
    ExpiredNoteMissing,
    /// A superseded credential does not name its superseded state.
    SupersededNoteMissing,
    /// An auditable credential does not name its audit trail.
    AuditNoteMissing,
    /// A credential-state row omits a mandatory rotate/revoke/test action.
    StateRowActionsIncomplete,
    /// The credential-state rows do not cover every derived health class.
    HealthClassCoverageMissing,
    /// The credential-state rows do not cover every target boundary.
    TargetBoundaryCoverageMissing,
    /// No vault-or-keychain pickers are present.
    VaultPickersMissing,
    /// A vault-or-keychain picker is incomplete.
    VaultPickerIncomplete,
    /// A vault-or-keychain picker carries the wrong frozen component class.
    VaultPickerWrongComponentClass,
    /// A vault-or-keychain picker does not name its available source or access scope.
    AvailableSourceOrScopeMissing,
    /// A vault-or-keychain picker misrepresents its derived portability.
    PortabilityMisrepresented,
    /// A vault-or-keychain picker does not name its portability / export note.
    PortabilityNoteMissing,
    /// An export-blocked picker does not name its export-blocked state.
    ExportBlockedNoteMissing,
    /// A session-only picker does not name its session-only state.
    SessionOnlyNoteMissing,
    /// A handle-only picker does not name its handle-only state.
    HandleOnlyNoteMissing,
    /// A vault-or-keychain picker omits the open-source-of-truth action.
    VaultPickerActionsIncomplete,
    /// The vault-or-keychain pickers do not cover every portability class.
    PortabilityCoverageMissing,
    /// The vault-or-keychain pickers do not cover every access scope.
    AccessScopeCoverageMissing,
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
    /// A control uses friendly "connected" wording that conceals storage or delegation.
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

impl CredentialStateRowVaultPickerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::CredentialStateRowsMissing => "credential_state_rows_missing",
            Self::CredentialStateRowIncomplete => "credential_state_row_incomplete",
            Self::CredentialStateRowWrongComponentClass => {
                "credential_state_row_wrong_component_class"
            }
            Self::TargetBoundaryMissing => "target_boundary_missing",
            Self::HealthStateMisrepresented => "health_state_misrepresented",
            Self::StorageAndRevealNoteMissing => "storage_and_reveal_note_missing",
            Self::AttentionNoteMissing => "attention_note_missing",
            Self::RevokedNoteMissing => "revoked_note_missing",
            Self::ExpiredNoteMissing => "expired_note_missing",
            Self::SupersededNoteMissing => "superseded_note_missing",
            Self::AuditNoteMissing => "audit_note_missing",
            Self::StateRowActionsIncomplete => "state_row_actions_incomplete",
            Self::HealthClassCoverageMissing => "health_class_coverage_missing",
            Self::TargetBoundaryCoverageMissing => "target_boundary_coverage_missing",
            Self::VaultPickersMissing => "vault_pickers_missing",
            Self::VaultPickerIncomplete => "vault_picker_incomplete",
            Self::VaultPickerWrongComponentClass => "vault_picker_wrong_component_class",
            Self::AvailableSourceOrScopeMissing => "available_source_or_scope_missing",
            Self::PortabilityMisrepresented => "portability_misrepresented",
            Self::PortabilityNoteMissing => "portability_note_missing",
            Self::ExportBlockedNoteMissing => "export_blocked_note_missing",
            Self::SessionOnlyNoteMissing => "session_only_note_missing",
            Self::HandleOnlyNoteMissing => "handle_only_note_missing",
            Self::VaultPickerActionsIncomplete => "vault_picker_actions_incomplete",
            Self::PortabilityCoverageMissing => "portability_coverage_missing",
            Self::AccessScopeCoverageMissing => "access_scope_coverage_missing",
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

/// Reads and validates the checked-in stable credential-state-row / vault-picker export.
pub fn current_credential_state_row_vault_picker_export(
) -> Result<CredentialStateRowVaultPickerControlsPacket, CredentialStateRowVaultPickerArtifactError>
{
    let packet: CredentialStateRowVaultPickerControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-credential-state-row-vault-picker-proof/support_export.json"
        )))
        .map_err(CredentialStateRowVaultPickerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CredentialStateRowVaultPickerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &CredentialStateRowVaultPickerControlsPacket,
    violations: &mut Vec<CredentialStateRowVaultPickerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_REF,
        CREDENTIAL_STATE_ROW_VAULT_PICKER_DOC_REF,
        M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_DOC_REF,
        M5_CREDENTIAL_STATE_ROW_SCHEMA_REF,
        M5_VAULT_KEYCHAIN_PICKER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(CredentialStateRowVaultPickerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_credential_state_rows(
    packet: &CredentialStateRowVaultPickerControlsPacket,
    violations: &mut Vec<CredentialStateRowVaultPickerViolation>,
) {
    if packet.credential_state_rows.is_empty() {
        violations.push(CredentialStateRowVaultPickerViolation::CredentialStateRowsMissing);
        return;
    }

    let mut health_classes: BTreeSet<CredentialHealthClass> = BTreeSet::new();
    let mut boundaries: BTreeSet<CredentialTargetBoundary> = BTreeSet::new();

    for row in &packet.credential_state_rows {
        let disclosure = row.health_disclosure();
        health_classes.insert(disclosure.health_class);
        boundaries.insert(row.target_boundary);

        if row.row_id.trim().is_empty()
            || row.credential_label.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.consumer_surfaces.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(CredentialStateRowVaultPickerViolation::CredentialStateRowIncomplete);
        }
        if row.component != M5CredentialComponentFamily::CredentialStateRow {
            violations.push(
                CredentialStateRowVaultPickerViolation::CredentialStateRowWrongComponentClass,
            );
        }
        if row.target_label.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::TargetBoundaryMissing);
        }
        if row.health_class != disclosure.health_class
            || row.claims_healthy != disclosure.is_healthy
        {
            violations.push(CredentialStateRowVaultPickerViolation::HealthStateMisrepresented);
        }
        if row.storage_and_reveal_note.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::StorageAndRevealNoteMissing);
        }
        if disclosure.needs_attention_note && row.attention_note.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::AttentionNoteMissing);
        }
        if disclosure.needs_revoked_note && row.revoked_note.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::RevokedNoteMissing);
        }
        if disclosure.needs_expired_note && row.expired_note.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::ExpiredNoteMissing);
        }
        if disclosure.needs_superseded_note && row.superseded_note.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::SupersededNoteMissing);
        }
        if row.is_auditable && row.audit_note.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::AuditNoteMissing);
        }
        if !row.declares_mandatory_actions() {
            violations.push(CredentialStateRowVaultPickerViolation::StateRowActionsIncomplete);
        }
        if row.degraded_states.is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::DegradedStatesMissing);
        }
        if !row.declares_mandatory_labels() {
            violations.push(CredentialStateRowVaultPickerViolation::RequiredLabelsIncomplete);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5CredentialAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(CredentialStateRowVaultPickerViolation::AccessibilityRouteMissing);
        }
        if row.masks_storage_or_reveal_posture {
            violations.push(CredentialStateRowVaultPickerViolation::StorageOrRevealMasked);
        }
        if row.implies_raw_secret_exportable {
            violations.push(CredentialStateRowVaultPickerViolation::RawSecretHandlingNormalized);
        }
        if row.uses_friendly_connected_wording {
            violations.push(CredentialStateRowVaultPickerViolation::FriendlyConnectedWordingUsed);
        }
    }

    for required in CredentialHealthClass::ALL {
        if !health_classes.contains(&required) {
            violations.push(CredentialStateRowVaultPickerViolation::HealthClassCoverageMissing);
            break;
        }
    }
    for required in CredentialTargetBoundary::ALL {
        if !boundaries.contains(&required) {
            violations.push(CredentialStateRowVaultPickerViolation::TargetBoundaryCoverageMissing);
            break;
        }
    }
}

fn validate_vault_pickers(
    packet: &CredentialStateRowVaultPickerControlsPacket,
    violations: &mut Vec<CredentialStateRowVaultPickerViolation>,
) {
    if packet.vault_pickers.is_empty() {
        violations.push(CredentialStateRowVaultPickerViolation::VaultPickersMissing);
        return;
    }

    let mut portability_classes: BTreeSet<VaultPortabilityClass> = BTreeSet::new();
    let mut access_scopes: BTreeSet<VaultAccessScope> = BTreeSet::new();

    for picker in &packet.vault_pickers {
        let disclosure = picker.portability_disclosure();
        portability_classes.insert(disclosure.portability_class);
        access_scopes.insert(picker.access_scope);

        if picker.picker_id.trim().is_empty()
            || picker.store_label.trim().is_empty()
            || picker.store_capabilities.is_empty()
            || picker.fields_shown.is_empty()
            || picker.surface_families.is_empty()
            || picker.deployment_lines.is_empty()
            || picker.consumer_surfaces.is_empty()
            || picker.source_contract_refs.is_empty()
        {
            violations.push(CredentialStateRowVaultPickerViolation::VaultPickerIncomplete);
        }
        if picker.component != M5CredentialComponentFamily::VaultOrKeychainPicker {
            violations.push(CredentialStateRowVaultPickerViolation::VaultPickerWrongComponentClass);
        }
        if picker.access_scope_label.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::AvailableSourceOrScopeMissing);
        }
        if picker.portability_class != disclosure.portability_class
            || picker.claims_portable != disclosure.is_portable
        {
            violations.push(CredentialStateRowVaultPickerViolation::PortabilityMisrepresented);
        }
        if picker.portability_note.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::PortabilityNoteMissing);
        }
        if disclosure.needs_export_blocked_note && picker.export_blocked_note.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::ExportBlockedNoteMissing);
        }
        if disclosure.needs_session_only_note && picker.session_only_note.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::SessionOnlyNoteMissing);
        }
        if disclosure.needs_handle_only_note && picker.handle_only_note.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::HandleOnlyNoteMissing);
        }
        if picker.storage_and_reveal_note.trim().is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::StorageAndRevealNoteMissing);
        }
        if !picker.declares_mandatory_actions() {
            violations.push(CredentialStateRowVaultPickerViolation::VaultPickerActionsIncomplete);
        }
        if picker.degraded_states.is_empty() {
            violations.push(CredentialStateRowVaultPickerViolation::DegradedStatesMissing);
        }
        if !picker.declares_mandatory_labels() {
            violations.push(CredentialStateRowVaultPickerViolation::RequiredLabelsIncomplete);
        }
        if picker.accessibility_routes.is_empty()
            || !picker
                .accessibility_routes
                .contains(&M5CredentialAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(CredentialStateRowVaultPickerViolation::AccessibilityRouteMissing);
        }
        if picker.masks_storage_or_reveal_posture {
            violations.push(CredentialStateRowVaultPickerViolation::StorageOrRevealMasked);
        }
        if picker.implies_raw_secret_exportable {
            violations.push(CredentialStateRowVaultPickerViolation::RawSecretHandlingNormalized);
        }
        if picker.uses_friendly_connected_wording {
            violations.push(CredentialStateRowVaultPickerViolation::FriendlyConnectedWordingUsed);
        }
    }

    for required in VaultPortabilityClass::ALL {
        if !portability_classes.contains(&required) {
            violations.push(CredentialStateRowVaultPickerViolation::PortabilityCoverageMissing);
            break;
        }
    }
    for required in VaultAccessScope::ALL {
        if !access_scopes.contains(&required) {
            violations.push(CredentialStateRowVaultPickerViolation::AccessScopeCoverageMissing);
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
