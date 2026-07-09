//! Canonical seed builders for the credential-state-row / vault-picker controls.
//!
//! These builders are the single producer of the checked-in support export and
//! the scenario fixtures. The headless emitter and the inline tests both call
//! them so the in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical credential-state-row / vault-picker packet.
pub const CREDENTIAL_STATE_ROW_VAULT_PICKER_PACKET_ID: &str =
    "m5-credential-state-row-vault-picker-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn row_source_refs() -> Vec<String> {
    strings(&[
        M5_CREDENTIAL_STATE_ROW_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_STATE_REF,
    ])
}

fn picker_source_refs() -> Vec<String> {
    strings(&[
        M5_VAULT_KEYCHAIN_PICKER_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_PICKER_REF,
    ])
}

/// Builds a credential-state row, deriving the health class, the healthy claim, and
/// the required notes from the honest inputs so the seed is always self-consistent
/// with the resolver.
#[allow(clippy::too_many_arguments)]
fn credential_state_row(
    row_id: &str,
    credential_label: &str,
    storage_mode: M5CredentialStorageMode,
    credential_class: M5CredentialClass,
    reveal_posture: M5CredentialRevealPosture,
    target_boundary: CredentialTargetBoundary,
    target_label: &str,
    lifecycle_state: M5CredentialLifecycleState,
) -> CredentialStateRow {
    let disclosure = resolve_credential_health(lifecycle_state);
    CredentialStateRow {
        component: M5CredentialComponentFamily::CredentialStateRow,
        row_id: row_id.to_owned(),
        credential_label: credential_label.to_owned(),
        storage_mode,
        credential_class,
        reveal_posture,
        target_boundary,
        target_label: target_label.to_owned(),
        lifecycle_state,
        health_class: disclosure.health_class,
        claims_healthy: disclosure.is_healthy,
        attention_note: if disclosure.needs_attention_note {
            format!(
                "Lifecycle state {}: refresh or rotation is due before continued use",
                lifecycle_state.as_str()
            )
        } else {
            String::new()
        },
        revoked_note: if disclosure.needs_revoked_note {
            "This credential has been revoked and no longer authorizes its target".to_owned()
        } else {
            String::new()
        },
        expired_note: if disclosure.needs_expired_note {
            "This credential has expired and no longer authorizes its target".to_owned()
        } else {
            String::new()
        },
        superseded_note: if disclosure.needs_superseded_note {
            "Superseded by a newer credential; this one should be retired".to_owned()
        } else {
            String::new()
        },
        storage_and_reveal_note: format!(
            "Stored as {}; reveal posture {}",
            storage_mode.as_str(),
            reveal_posture.as_str()
        ),
        is_auditable: true,
        audit_note: format!(
            "Rotate, revoke, and test actions are recorded in the {} audit trail",
            target_boundary.as_str()
        ),
        default_actions: CredentialStateRowAction::ALL.to_vec(),
        degraded_states: M5CredentialDegradedState::ALL.to_vec(),
        required_labels: M5CredentialRequiredLabel::ALL.to_vec(),
        surface_families: M5CredentialSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CredentialDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CredentialAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CredentialConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "credential_label",
            "storage_mode",
            "credential_class",
            "reveal_posture",
            "target_boundary",
            "lifecycle_state",
            "health",
        ]),
        source_contract_refs: row_source_refs(),
        masks_storage_or_reveal_posture: false,
        implies_raw_secret_exportable: false,
        uses_friendly_connected_wording: false,
    }
}

/// Builds a vault-or-keychain picker, deriving the portability class and the
/// required notes from the honest inputs so the seed is always self-consistent.
fn vault_picker(
    picker_id: &str,
    store_label: &str,
    storage_mode: M5CredentialStorageMode,
    access_scope: VaultAccessScope,
    access_scope_label: &str,
    store_capabilities: &[M5CredentialStoreCapability],
    reveal_policy: M5CredentialRevealPosture,
) -> VaultOrKeychainPicker {
    let disclosure = resolve_vault_portability(storage_mode, store_capabilities, reveal_policy);
    VaultOrKeychainPicker {
        component: M5CredentialComponentFamily::VaultOrKeychainPicker,
        picker_id: picker_id.to_owned(),
        store_label: store_label.to_owned(),
        storage_mode,
        access_scope,
        access_scope_label: access_scope_label.to_owned(),
        store_capabilities: store_capabilities.to_vec(),
        reveal_policy,
        portability_class: disclosure.portability_class,
        claims_portable: disclosure.is_portable,
        portability_note: format!(
            "Portability: {} (derived from storage {} and reveal {})",
            disclosure.portability_class.as_str(),
            storage_mode.as_str(),
            reveal_policy.as_str()
        ),
        export_blocked_note: if disclosure.needs_export_blocked_note {
            "This store blocks export; nothing leaves the store of record".to_owned()
        } else {
            String::new()
        },
        session_only_note: if disclosure.needs_session_only_note {
            "Session-only store; nothing survives exit and nothing is portable".to_owned()
        } else {
            String::new()
        },
        handle_only_note: if disclosure.needs_handle_only_note {
            "Only a handle reference is portable; the raw value never leaves the store".to_owned()
        } else {
            String::new()
        },
        storage_and_reveal_note: format!(
            "Writes to {}; reveal policy {}",
            storage_mode.as_str(),
            reveal_policy.as_str()
        ),
        default_actions: VaultPickerAction::ALL.to_vec(),
        degraded_states: M5CredentialDegradedState::ALL.to_vec(),
        required_labels: M5CredentialRequiredLabel::ALL.to_vec(),
        surface_families: M5CredentialSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CredentialDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CredentialAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CredentialConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "store_label",
            "storage_mode",
            "access_scope",
            "store_capabilities",
            "reveal_policy",
            "portability",
        ]),
        source_contract_refs: picker_source_refs(),
        masks_storage_or_reveal_posture: false,
        implies_raw_secret_exportable: false,
        uses_friendly_connected_wording: false,
    }
}

fn credential_state_rows() -> Vec<CredentialStateRow> {
    use CredentialTargetBoundary as Boundary;
    use M5CredentialClass as Class;
    use M5CredentialLifecycleState as Life;
    use M5CredentialRevealPosture as Reveal;
    use M5CredentialStorageMode as Storage;

    vec![
        // 1. Provider boundary, healthy: an active OAuth token in the OS keychain — the
        //    highest-trust credential a user can act on directly.
        credential_state_row(
            "cred-provider-active",
            "GitHub provider sign-in",
            Storage::OsKeychain,
            Class::OauthToken,
            Reveal::HandleOnly,
            Boundary::Provider,
            "GitHub provider API",
            Life::ActiveCurrent,
        ),
        // 2. Registry boundary, attention needed: a registry token whose refresh is due.
        credential_state_row(
            "cred-registry-refresh",
            "npm registry publish token",
            Storage::EncryptedVault,
            Class::ApiKey,
            Reveal::MaskedLastFour,
            Boundary::Registry,
            "npm registry",
            Life::RefreshNeeded,
        ),
        // 3. Request boundary, attention needed: an API key whose rotation is due.
        credential_state_row(
            "cred-request-rotation-due",
            "Outbound webhook signing key",
            Storage::SecretBrokerHandle,
            Class::PersonalAccessToken,
            Reveal::HandleOnly,
            Boundary::Request,
            "Outbound request / API boundary",
            Life::RotationDue,
        ),
        // 4. Database boundary, revoked: a database credential that has been revoked and
        //    must never read as healthy.
        credential_state_row(
            "cred-database-revoked",
            "Analytics warehouse connection",
            Storage::EncryptedVault,
            Class::ClientCertificate,
            Reveal::NeverRevealed,
            Boundary::Database,
            "Analytics warehouse",
            Life::Revoked,
        ),
        // 5. Remote boundary, expired: an SSH key that has expired and must never read as
        //    healthy.
        credential_state_row(
            "cred-remote-expired",
            "Remote build host access",
            Storage::OsKeychain,
            Class::SshOrSigningKey,
            Reveal::HandleOnly,
            Boundary::Remote,
            "Remote build host",
            Life::Expired,
        ),
        // 6. Release boundary, superseded: a signing key superseded by a rotated key.
        credential_state_row(
            "cred-release-superseded",
            "Release artifact signing key",
            Storage::OsKeychain,
            Class::SshOrSigningKey,
            Reveal::HandleOnly,
            Boundary::Release,
            "Release signing boundary",
            Life::Superseded,
        ),
    ]
}

fn vault_pickers() -> Vec<VaultOrKeychainPicker> {
    use M5CredentialRevealPosture as Reveal;
    use M5CredentialStorageMode as Storage;
    use M5CredentialStoreCapability as Capability;
    use VaultAccessScope as Scope;

    vec![
        // 1. Portable: a local OS keychain that persists across restart.
        vault_picker(
            "vault-device-keychain",
            "macOS login keychain",
            Storage::OsKeychain,
            Scope::DeviceLocal,
            "Local to this device",
            &[Capability::PersistAcrossRestart, Capability::OsLockedAtRest],
            Reveal::RevealOnDemand,
        ),
        // 2. Handle-reference-only: an encrypted vault revealed handle-only.
        vault_picker(
            "vault-user-encrypted",
            "Per-user encrypted vault",
            Storage::EncryptedVault,
            Scope::UserProfile,
            "Scoped to this user's profile",
            &[
                Capability::PersistAcrossRestart,
                Capability::SyncAcrossDevices,
            ],
            Reveal::HandleOnly,
        ),
        // 3. Export-blocked: a team-shared external store that blocks its own export.
        vault_picker(
            "vault-team-external",
            "Team secrets manager",
            Storage::ExternalReference,
            Scope::TeamShared,
            "Shared with the platform team",
            &[
                Capability::StoreExportBlocked,
                Capability::PersistAcrossRestart,
            ],
            Reveal::MaskedLastFour,
        ),
        // 4. Handle-reference-only: an org-managed broker-handle store.
        vault_picker(
            "vault-org-broker",
            "Org secret broker",
            Storage::SecretBrokerHandle,
            Scope::OrgManaged,
            "Managed by the organization",
            &[Capability::HardwareBacked, Capability::PersistAcrossRestart],
            Reveal::HandleOnly,
        ),
        // 5. Session-only, non-portable: a session-memory store that survives nothing.
        vault_picker(
            "vault-session-memory",
            "Session memory store",
            Storage::SessionMemoryOnly,
            Scope::SessionOnly,
            "Available for this session only",
            &[Capability::SessionOnly],
            Reveal::ClipboardScoped,
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5CredentialDowngradeTrigger> {
    vec![
        M5CredentialDowngradeTrigger::StorageModeUnstated,
        M5CredentialDowngradeTrigger::RevealPostureUnstated,
        M5CredentialDowngradeTrigger::LifecycleStateHidden,
        M5CredentialDowngradeTrigger::StoreCapabilityUnstated,
        M5CredentialDowngradeTrigger::ExportSafetyBoundaryHidden,
        M5CredentialDowngradeTrigger::FriendlyConnectedWordingUsed,
        M5CredentialDowngradeTrigger::SessionOnlyFallbackHidden,
        M5CredentialDowngradeTrigger::ProofStale,
    ]
}

fn trust_review() -> CredentialStateRowVaultPickerTrustReview {
    CredentialStateRowVaultPickerTrustReview {
        credential_state_shows_storage_and_reveal_posture: true,
        credential_state_shows_target_boundary: true,
        health_state_derived_never_asserted: true,
        revoked_or_expired_never_reads_as_healthy: true,
        rotate_revoke_test_actions_present: true,
        auditability_always_named: true,
        vault_picker_shows_available_source_and_scope: true,
        vault_picker_shows_reveal_policy: true,
        portability_note_derived_never_asserted: true,
        export_blocked_and_session_only_explicit: true,
        raw_secret_handling_never_normalized: true,
        no_friendly_wording_conceals_storage_or_delegation: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> CredentialStateRowVaultPickerConsumerProjection {
    CredentialStateRowVaultPickerConsumerProjection {
        credential_rows_show_authority_and_boundary_without_docs: true,
        storage_mode_clarity_preserved_across_surfaces: true,
        vault_picker_shows_source_scope_and_portability_inline: true,
        cli_headless_shows_control_truth: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> CredentialStateRowVaultPickerProofFreshness {
    CredentialStateRowVaultPickerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_REF,
        CREDENTIAL_STATE_ROW_VAULT_PICKER_DOC_REF,
        M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_DOC_REF,
        M5_CREDENTIAL_STATE_ROW_SCHEMA_REF,
        M5_VAULT_KEYCHAIN_PICKER_SCHEMA_REF,
    ])
}

/// Builds the canonical credential-state-row / vault-picker controls packet.
pub fn seeded_credential_state_row_vault_picker_controls(
) -> CredentialStateRowVaultPickerControlsPacket {
    CredentialStateRowVaultPickerControlsPacket::new(
        CredentialStateRowVaultPickerControlsPacketInput {
            packet_id: CREDENTIAL_STATE_ROW_VAULT_PICKER_PACKET_ID.to_owned(),
            surface_label:
                "M5 credential-state rows and vault/keychain pickers: storage mode, source class, target boundary, expiry/rotation/revoke lifecycle, derived health, auditability, keyboard-complete rotate/revoke/test actions, available source, access scope, reveal policy, derived portability, and open-source-of-truth actions"
                    .to_owned(),
            credential_state_rows: credential_state_rows(),
            vault_pickers: vault_pickers(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5CredentialConsumerSurface::ALL.to_vec(),
            trust_review: trust_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a revoked credential-state row that must never read
/// as healthy. Every health class and target boundary stays covered so the fixture
/// validates on its own.
pub fn seeded_credential_state_row_vault_picker_controls_credential_state_row_revoked(
) -> CredentialStateRowVaultPickerControlsPacket {
    let mut packet = seeded_credential_state_row_vault_picker_controls();
    packet.packet_id =
        "m5-credential-state-row-vault-picker-controls:fixture:credential-state-row-revoked"
            .to_owned();
    packet.surface_label =
        "M5 credential-state rows: a revoked credential never reads as healthy".to_owned();
    packet
}

/// Scenario fixture: spotlights an export-blocked vault picker that must never
/// present as freely portable. Every portability class and access scope stays
/// covered so the fixture validates on its own.
pub fn seeded_credential_state_row_vault_picker_controls_vault_picker_export_blocked(
) -> CredentialStateRowVaultPickerControlsPacket {
    let mut packet = seeded_credential_state_row_vault_picker_controls();
    packet.packet_id =
        "m5-credential-state-row-vault-picker-controls:fixture:vault-picker-export-blocked"
            .to_owned();
    packet.surface_label =
        "M5 vault/keychain pickers: an export-blocked store never reads as portable".to_owned();
    packet
}
