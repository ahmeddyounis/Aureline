//! Canonical seed builders for the secret-access-prompt / store-capability controls.
//!
//! These builders are the single producer of the checked-in support export and
//! the scenario fixtures. The headless emitter and the inline tests both call
//! them so the in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical secret-access-prompt / store-capability packet.
pub const SECRET_ACCESS_PROMPT_STORE_CAPABILITY_PACKET_ID: &str =
    "m5-secret-access-prompt-store-capability-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn prompt_source_refs() -> Vec<String> {
    strings(&[
        M5_SECRET_ACCESS_PROMPT_SHEET_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_ACCESS_PROMPT_REF,
    ])
}

fn store_row_source_refs() -> Vec<String> {
    strings(&[
        M5_CREDENTIAL_STORE_CAPABILITY_ROW_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_HANDLE_REF,
    ])
}

/// Builds a secret-access prompt sheet, deriving the handle-availability class, the
/// handle-only claim, and the required notes from the honest inputs so the seed is
/// always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn secret_access_prompt(
    sheet_id: &str,
    actor: SecretRequestActor,
    actor_label: &str,
    purpose_note: &str,
    requested_scope_note: &str,
    credential_class: M5CredentialClass,
    reveal_posture: M5CredentialRevealPosture,
) -> SecretAccessPromptSheet {
    let disclosure = resolve_handle_availability(reveal_posture);
    SecretAccessPromptSheet {
        component: M5CredentialComponentFamily::SecretAccessPromptSheet,
        sheet_id: sheet_id.to_owned(),
        actor,
        actor_label: actor_label.to_owned(),
        purpose_note: purpose_note.to_owned(),
        requested_scope_note: requested_scope_note.to_owned(),
        credential_class,
        reveal_posture,
        handle_availability_class: disclosure.handle_availability_class,
        claims_handle_only_path: disclosure.is_handle_only_available,
        handle_only_note: if disclosure.needs_handle_only_note {
            "A handle-only path exists; you can grant access without exposing the raw secret"
                .to_owned()
        } else {
            String::new()
        },
        raw_reveal_disclosure_note: if disclosure.needs_raw_reveal_disclosure_note {
            "This request needs the raw secret; granting it reveals the value to the actor"
                .to_owned()
        } else {
            String::new()
        },
        reveal_blocked_note: if disclosure.needs_reveal_blocked_note {
            "A raw reveal is blocked by policy; only a handle reference can be granted".to_owned()
        } else {
            String::new()
        },
        retention_note: format!(
            "Retention: a grant is held as {} and can be reviewed or withdrawn later",
            reveal_posture.as_str()
        ),
        denied_fallback_note:
            "If denied, read-only cached context still works; nothing is sent or published"
                .to_owned(),
        storage_and_reveal_note: format!(
            "Requested class {}; reveal posture {}",
            credential_class.as_str(),
            reveal_posture.as_str()
        ),
        default_actions: SecretAccessPromptAction::ALL.to_vec(),
        degraded_states: M5CredentialDegradedState::ALL.to_vec(),
        required_labels: M5CredentialRequiredLabel::ALL.to_vec(),
        surface_families: M5CredentialSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CredentialDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CredentialAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CredentialConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "actor",
            "purpose",
            "requested_scope",
            "credential_class",
            "reveal_posture",
            "handle_availability",
            "retention",
            "denied_fallback",
        ]),
        source_contract_refs: prompt_source_refs(),
        masks_storage_or_reveal_posture: false,
        implies_raw_secret_exportable: false,
        uses_friendly_connected_wording: false,
    }
}

/// Builds a credential-store-capability row, deriving the trust class and the
/// required notes from the honest inputs so the seed is always self-consistent.
fn store_capability_row(
    row_id: &str,
    store_label: &str,
    storage_mode: M5CredentialStorageMode,
    store_capabilities: &[M5CredentialStoreCapability],
    verification_state: StoreVerificationState,
    verification_label: &str,
    export_safety_class: M5CredentialExportSafetyClass,
) -> CredentialStoreCapabilityRow {
    let disclosure = resolve_store_trust(verification_state, store_capabilities);
    CredentialStoreCapabilityRow {
        component: M5CredentialComponentFamily::CredentialStoreCapabilityRow,
        row_id: row_id.to_owned(),
        store_label: store_label.to_owned(),
        storage_mode,
        store_capabilities: store_capabilities.to_vec(),
        verification_state,
        verification_label: verification_label.to_owned(),
        export_safety_class,
        trust_class: disclosure.trust_class,
        claims_securely_stored: disclosure.is_securely_stored,
        portability_export_note: format!(
            "Export posture {}: only export-safe references leave the store",
            export_safety_class.as_str()
        ),
        platform_limitations_note: format!(
            "Platform limitations apply to the {} store on this build",
            storage_mode.as_str()
        ),
        unverified_note: if disclosure.needs_unverified_note {
            "This store is not verified; its security is unproven and not 'saved securely'"
                .to_owned()
        } else {
            String::new()
        },
        unsupported_note: if disclosure.needs_unsupported_note {
            "This store is unsupported on this platform; choose a different store".to_owned()
        } else {
            String::new()
        },
        session_only_fallback_note: if disclosure.needs_session_only_fallback_note {
            "Session-only fallback: material is held for this session only and not persisted"
                .to_owned()
        } else {
            String::new()
        },
        storage_and_capability_note: format!(
            "Store type {}; verification {}",
            storage_mode.as_str(),
            verification_state.as_str()
        ),
        default_actions: CredentialStoreCapabilityRowAction::ALL.to_vec(),
        degraded_states: M5CredentialDegradedState::ALL.to_vec(),
        required_labels: M5CredentialRequiredLabel::ALL.to_vec(),
        surface_families: M5CredentialSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CredentialDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CredentialAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CredentialConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "store_label",
            "storage_mode",
            "store_capabilities",
            "verification_state",
            "export_safety_class",
            "trust_class",
        ]),
        source_contract_refs: store_row_source_refs(),
        masks_storage_or_reveal_posture: false,
        implies_raw_secret_exportable: false,
        uses_friendly_connected_wording: false,
    }
}

fn secret_access_prompts() -> Vec<SecretAccessPromptSheet> {
    use M5CredentialClass as Class;
    use M5CredentialRevealPosture as Reveal;
    use SecretRequestActor as Actor;

    vec![
        // 1. First-party feature, handle-only: a handle-only path exists, so the user
        //    is never nudged toward a raw-secret reveal.
        secret_access_prompt(
            "prompt-first-party-handle-only",
            Actor::FirstPartyFeature,
            "Aureline sync engine",
            "Authenticate the local sync engine to its provider",
            "read-only work-item metadata for the current workspace",
            Class::OauthToken,
            Reveal::HandleOnly,
        ),
        // 2. Provider connector, raw reveal requested: must disclose the raw reveal
        //    explicitly and never nudge the user toward it.
        secret_access_prompt(
            "prompt-provider-raw-reveal",
            Actor::ProviderConnector,
            "GitHub connector",
            "Attach a personal token so the connector can act as you",
            "repository read and issue write for the selected org",
            Class::PersonalAccessToken,
            Reveal::RevealOnDemand,
        ),
        // 3. Registry client, scoped reveal: only a masked / scoped-copy path exists.
        secret_access_prompt(
            "prompt-registry-scoped",
            Actor::RegistryClient,
            "npm registry client",
            "Authenticate publishes to the registry",
            "publish scope for the selected package namespace",
            Class::ApiKey,
            Reveal::MaskedLastFour,
        ),
        // 4. Remote/database attach, reveal blocked: a raw reveal is blocked by policy.
        secret_access_prompt(
            "prompt-remote-blocked",
            Actor::RemoteOrDatabaseAttach,
            "Warehouse attach",
            "Attach to the analytics warehouse for a read session",
            "read-only connection to the analytics schema",
            Class::ClientCertificate,
            Reveal::PolicyBlockedReveal,
        ),
        // 5. Release publisher, handle-only: a handle-only signing path exists.
        secret_access_prompt(
            "prompt-release-handle-only",
            Actor::ReleasePublisher,
            "Release signer",
            "Sign the release artifact with the signing key",
            "sign scope for the current release channel",
            Class::SshOrSigningKey,
            Reveal::NeverRevealed,
        ),
        // 6. Delegated agent, scoped reveal: acting on behalf, scoped clipboard path.
        secret_access_prompt(
            "prompt-delegated-scoped",
            Actor::DelegatedAgent,
            "Delegated automation agent",
            "Act on behalf of the release owner for a device-code grant",
            "delegated scope limited to the selected pipeline",
            Class::DeviceCodeGrant,
            Reveal::ClipboardScoped,
        ),
    ]
}

fn store_capability_rows() -> Vec<CredentialStoreCapabilityRow> {
    use M5CredentialExportSafetyClass as Export;
    use M5CredentialStorageMode as Storage;
    use M5CredentialStoreCapability as Capability;
    use StoreVerificationState as Verification;

    vec![
        // 1. Securely stored: a hardware-attested, persistent store.
        store_capability_row(
            "store-hardware-attested",
            "Hardware security module",
            Storage::OsKeychain,
            &[
                Capability::HardwareBacked,
                Capability::PersistAcrossRestart,
                Capability::OsLockedAtRest,
            ],
            Verification::HardwareAttested,
            "Hardware attested",
            Export::HandleReferenceOnly,
        ),
        // 2. Securely stored: an OS-verified, persistent keychain.
        store_capability_row(
            "store-os-verified",
            "OS login keychain",
            Storage::OsKeychain,
            &[Capability::PersistAcrossRestart, Capability::OsLockedAtRest],
            Verification::OsVerified,
            "OS verified",
            Export::RawSecretExcluded,
        ),
        // 3. Limited assurance: a verified encrypted store that is session-only.
        store_capability_row(
            "store-encrypted-session",
            "Encrypted session store",
            Storage::EncryptedVault,
            &[Capability::SessionOnly],
            Verification::EncryptedVerified,
            "Encrypted, session-only",
            Export::MetadataOnly,
        ),
        // 4. Unverified store: security could not be verified — never "saved securely".
        store_capability_row(
            "store-unverified",
            "Unverified file store",
            Storage::ExternalReference,
            &[Capability::PersistAcrossRestart],
            Verification::Unverified,
            "Unverified",
            Export::RedactedShare,
        ),
        // 5. Unverified store: verification actively failed.
        store_capability_row(
            "store-verification-failed",
            "Failed keystore probe",
            Storage::EncryptedVault,
            &[Capability::PersistAcrossRestart],
            Verification::VerificationFailed,
            "Verification failed",
            Export::EndpointsMasked,
        ),
        // 6. Unsupported store: not available on this platform / build.
        store_capability_row(
            "store-unsupported",
            "Platform keyring (unsupported)",
            Storage::NoSecretStored,
            &[Capability::SessionOnly],
            Verification::Unsupported,
            "Unsupported on this build",
            Export::ExportBlocked,
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5CredentialDowngradeTrigger> {
    vec![
        M5CredentialDowngradeTrigger::StorageModeUnstated,
        M5CredentialDowngradeTrigger::RevealPostureUnstated,
        M5CredentialDowngradeTrigger::CredentialClassUnstated,
        M5CredentialDowngradeTrigger::StoreCapabilityUnstated,
        M5CredentialDowngradeTrigger::ExportSafetyBoundaryHidden,
        M5CredentialDowngradeTrigger::FriendlyConnectedWordingUsed,
        M5CredentialDowngradeTrigger::SessionOnlyFallbackHidden,
        M5CredentialDowngradeTrigger::ProofStale,
    ]
}

fn trust_review() -> SecretAccessPromptStoreCapabilityTrustReview {
    SecretAccessPromptStoreCapabilityTrustReview {
        prompt_shows_actor_purpose_and_scope: true,
        prompt_shows_raw_versus_handle_only_posture: true,
        handle_only_path_surfaced_when_available: true,
        raw_reveal_never_nudged: true,
        prompt_shows_retention_and_denied_fallback: true,
        allow_deny_once_semantics_present: true,
        store_row_shows_type_and_verification_state: true,
        store_row_shows_portability_and_platform_limits: true,
        trust_class_derived_never_asserted: true,
        unverified_or_unsupported_never_reads_as_secure: true,
        session_only_fallback_explicit_when_policy_allows: true,
        raw_secret_handling_never_normalized: true,
        no_vague_saved_securely_wording: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> SecretAccessPromptStoreCapabilityConsumerProjection {
    SecretAccessPromptStoreCapabilityConsumerProjection {
        prompt_shows_actor_scope_and_alternative_without_docs: true,
        handle_only_path_visible_before_raw_reveal: true,
        store_row_shows_type_verification_and_limits_inline: true,
        cli_headless_shows_control_truth: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> SecretAccessPromptStoreCapabilityProofFreshness {
    SecretAccessPromptStoreCapabilityProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_REF,
        SECRET_ACCESS_PROMPT_STORE_CAPABILITY_DOC_REF,
        M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_DOC_REF,
        M5_SECRET_ACCESS_PROMPT_SHEET_SCHEMA_REF,
        M5_CREDENTIAL_STORE_CAPABILITY_ROW_SCHEMA_REF,
    ])
}

/// Builds the canonical secret-access-prompt / store-capability controls packet.
pub fn seeded_secret_access_prompt_store_capability_controls(
) -> SecretAccessPromptStoreCapabilityControlsPacket {
    SecretAccessPromptStoreCapabilityControlsPacket::new(
        SecretAccessPromptStoreCapabilityControlsPacketInput {
            packet_id: SECRET_ACCESS_PROMPT_STORE_CAPABILITY_PACKET_ID.to_owned(),
            surface_label:
                "M5 secret-access prompt sheets and credential-store-capability rows: asking actor, purpose, requested scope, raw-secret-versus-handle-only posture, retention, allow/deny/once semantics, what still works if denied, store type, verification state, portability/export posture, platform limitations, and session-only fallback"
                    .to_owned(),
            secret_access_prompts: secret_access_prompts(),
            store_capability_rows: store_capability_rows(),
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

/// Scenario fixture: spotlights a secret-access prompt that requests a raw reveal and
/// must disclose it explicitly while surfacing the handle-only alternative wherever
/// one exists. Every actor and handle-availability class stays covered so the fixture
/// validates on its own.
pub fn seeded_secret_access_prompt_store_capability_controls_secret_access_prompt_raw_reveal(
) -> SecretAccessPromptStoreCapabilityControlsPacket {
    let mut packet = seeded_secret_access_prompt_store_capability_controls();
    packet.packet_id =
        "m5-secret-access-prompt-store-capability-controls:fixture:secret-access-prompt-raw-reveal"
            .to_owned();
    packet.surface_label =
        "M5 secret-access prompts: a raw reveal is always explicit, never nudged".to_owned();
    packet
}

/// Scenario fixture: spotlights an unverified credential store that must never read as
/// "securely stored". Every verification state and trust class stays covered so the
/// fixture validates on its own.
pub fn seeded_secret_access_prompt_store_capability_controls_store_capability_unverified(
) -> SecretAccessPromptStoreCapabilityControlsPacket {
    let mut packet = seeded_secret_access_prompt_store_capability_controls();
    packet.packet_id =
        "m5-secret-access-prompt-store-capability-controls:fixture:store-capability-unverified"
            .to_owned();
    packet.surface_label =
        "M5 store-capability rows: an unverified store never reads as securely stored".to_owned();
    packet
}
