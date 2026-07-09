//! Canonical seed builders for the frozen M5 credential component matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical credential component matrix.
pub const M5_CREDENTIAL_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-credential-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5CredentialRequiredLabel> {
    M5CredentialRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5CredentialRequiredLabel]) -> Vec<M5CredentialRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5CredentialComponentFamily,
    qualification: M5CredentialQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5CredentialComponentRow {
    M5CredentialComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5CredentialSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CredentialDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        storage_modes: vec![],
        credential_classes: vec![],
        reveal_postures: vec![],
        auth_handoff_classes: vec![],
        delegated_identity_states: vec![],
        lifecycle_states: vec![],
        store_capabilities: vec![],
        export_safety_classes: vec![],
        degraded_states: M5CredentialDegradedState::ALL.to_vec(),
        accessibility_routes: M5CredentialAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5CredentialConsumerSurface::CredentialSettingsUi,
            M5CredentialConsumerSurface::SupportExport,
            M5CredentialConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5CredentialDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        masks_storage_or_reveal_posture: false,
        hides_identity_delegation: false,
        invents_alternate_state_label: false,
        implies_raw_secret_exportable: false,
    }
}

fn component_rows() -> Vec<M5CredentialComponentRow> {
    use M5AuthHandoffClass as AH;
    use M5CredentialClass as CC;
    use M5CredentialComponentFamily as F;
    use M5CredentialConsumerSurface as C;
    use M5CredentialDowngradeTrigger as D;
    use M5CredentialExportSafetyClass as ES;
    use M5CredentialLifecycleState as LC;
    use M5CredentialQualificationClass as Q;
    use M5CredentialRequiredLabel as L;
    use M5CredentialRevealPosture as RP;
    use M5CredentialStorageMode as SO;
    use M5CredentialStoreCapability as SC;
    use M5DelegatedIdentityState as DI;

    let mut rows = Vec::new();

    // 1. Credential-state row.
    let mut row = base_row(
        F::CredentialStateRow,
        Q::Stable,
        "Credential-state row owner",
        "One credential-state-row model naming where a secret is stored (os keychain, encrypted vault, secret broker handle, session memory only, external reference, or no secret stored), which credential class it holds, whether a handle-only path exists or a raw reveal is possible, and its expiry / rotation / revoke lifecycle state, so a user never has to infer where a secret lives or whether it can be shown right now",
        "evidence:m5-credential-state-row-parity:001",
        &[
            M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
            M5_CREDENTIAL_STATE_ROW_SCHEMA_REF,
            M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_STATE_REF,
        ],
    );
    row.storage_modes = SO::ALL.to_vec();
    row.credential_classes = CC::ALL.to_vec();
    row.reveal_postures = RP::ALL.to_vec();
    row.lifecycle_states = LC::ALL.to_vec();
    row.required_labels = labels_with(&[L::StorageAndRevealPosture, L::ExpiryAndExportBoundary]);
    row.consumer_surfaces = vec![
        C::CredentialSettingsUi,
        C::StatusBarUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::StorageModeUnstated,
        D::RevealPostureUnstated,
        D::LifecycleStateHidden,
        D::FriendlyConnectedWordingUsed,
        D::SessionOnlyFallbackHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Secret-access-prompt sheet.
    let mut row = base_row(
        F::SecretAccessPromptSheet,
        Q::Stable,
        "Secret-access-prompt sheet owner",
        "One secret-access-prompt-sheet model naming which credential class is being requested, the reveal posture it honours (handle only, masked, reveal on demand, clipboard scoped, never revealed, or policy blocked), and the auth-handoff class it will complete (system browser redirect, device code poll, embedded prompt, passkey step up, delegated forward, or offline deferred), so a prompt never conceals whether a raw secret will be exposed or how the handoff completes",
        "evidence:m5-secret-access-prompt-sheet-parity:001",
        &[
            M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
            M5_SECRET_ACCESS_PROMPT_SHEET_SCHEMA_REF,
            M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_ACCESS_PROMPT_REF,
        ],
    );
    row.credential_classes = CC::ALL.to_vec();
    row.reveal_postures = RP::ALL.to_vec();
    row.auth_handoff_classes = AH::ALL.to_vec();
    row.required_labels = labels_with(&[L::StorageAndRevealPosture]);
    row.consumer_surfaces = vec![
        C::SecretPromptUi,
        C::DeviceCodeHandoffUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RevealPostureUnstated,
        D::CredentialClassUnstated,
        D::AuthHandoffClassUnstated,
        D::FriendlyConnectedWordingUsed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Vault-or-keychain picker.
    let mut row = base_row(
        F::VaultOrKeychainPicker,
        Q::Stable,
        "Vault/keychain picker owner",
        "One vault-or-keychain-picker model naming the storage mode a secret will be written to and the store's capability (persist across restart, os locked at rest, sync across devices, hardware backed, export blocked by store, or session only), so a user always sees where a secret will land and what that store can and cannot guarantee before it is written",
        "evidence:m5-vault-keychain-picker-parity:001",
        &[
            M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
            M5_VAULT_KEYCHAIN_PICKER_SCHEMA_REF,
            M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_PICKER_REF,
        ],
    );
    row.storage_modes = SO::ALL.to_vec();
    row.store_capabilities = SC::ALL.to_vec();
    row.required_labels = labels_with(&[L::StorageAndRevealPosture]);
    row.consumer_surfaces = vec![
        C::VaultPickerUi,
        C::CredentialSettingsUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::StorageModeUnstated,
        D::StoreCapabilityUnstated,
        D::SessionOnlyFallbackHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Credential-store-capability row.
    let mut row = base_row(
        F::CredentialStoreCapabilityRow,
        Q::Stable,
        "Credential-store-capability row owner",
        "One credential-store-capability-row model naming the storage mode of a configured store and the exact capabilities it offers alongside its degraded state, so a user never has to infer whether a store persists, is locked at rest, syncs, is hardware backed, blocks export, or is session only — and whether it is fully available, limited, stale, offline, policy blocked, or unavailable",
        "evidence:m5-credential-store-capability-row-parity:001",
        &[
            M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
            M5_CREDENTIAL_STORE_CAPABILITY_ROW_SCHEMA_REF,
            M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_HANDLE_REF,
        ],
    );
    row.storage_modes = SO::ALL.to_vec();
    row.store_capabilities = SC::ALL.to_vec();
    row.required_labels = labels_with(&[L::StorageAndRevealPosture]);
    row.consumer_surfaces = vec![
        C::CredentialSettingsUi,
        C::VaultPickerUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::StorageModeUnstated,
        D::StoreCapabilityUnstated,
        D::SessionOnlyFallbackHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Browser/device-code handoff card.
    let mut row = base_row(
        F::BrowserDeviceCodeHandoffCard,
        Q::Stable,
        "Browser/device-code handoff card owner",
        "One browser-device-code-handoff-card model naming the auth-handoff class currently in flight — a system browser redirect, a device code poll, an embedded prompt, a passkey step up, a delegated forward, or an offline-deferred handoff — and which identity it carries, so a user always knows how authentication will complete and never has to infer whether a browser or device-code flow is under way",
        "evidence:m5-browser-device-code-handoff-card-parity:001",
        &[
            M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
            M5_BROWSER_DEVICE_CODE_HANDOFF_CARD_SCHEMA_REF,
            M5_CREDENTIAL_COMPONENT_FOUNDATION_SYSTEM_BROWSER_REF,
        ],
    );
    row.auth_handoff_classes = AH::ALL.to_vec();
    row.required_labels = labels_with(&[L::IdentityAndDelegation]);
    row.consumer_surfaces = vec![
        C::DeviceCodeHandoffUi,
        C::SecretPromptUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AuthHandoffClassUnstated,
        D::FriendlyConnectedWordingUsed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Delegated-credential row.
    let mut row = base_row(
        F::DelegatedCredentialRow,
        Q::Stable,
        "Delegated-credential row owner",
        "One delegated-credential-row model naming which identity is acting — a local identity, a forwarded identity, a delegated-on-behalf identity, an impersonation-scoped identity, a service account, or a revoked delegation — and the credential class behind it, so a user never has to infer whether the identity being forwarded or delegated is their own and no friendly wording conceals a delegated principal",
        "evidence:m5-delegated-credential-row-parity:001",
        &[
            M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
            M5_DELEGATED_CREDENTIAL_ROW_SCHEMA_REF,
            M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_PROJECTION_REF,
        ],
    );
    row.credential_classes = CC::ALL.to_vec();
    row.delegated_identity_states = DI::ALL.to_vec();
    row.required_labels = labels_with(&[L::IdentityAndDelegation]);
    row.consumer_surfaces = vec![
        C::DelegatedIdentityUi,
        C::CredentialSettingsUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::DelegatedIdentityUnstated,
        D::CredentialClassUnstated,
        D::FriendlyConnectedWordingUsed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Rotation/revoke-event row.
    let mut row = base_row(
        F::RotationRevokeEventRow,
        Q::Stable,
        "Rotation/revoke-event row owner",
        "One rotation-revoke-event-row model naming the credential lifecycle state — active current, refresh needed, rotation due, revoked, expired, or superseded — so a user always sees what rotation or revoke will impact and a pending rotation or revoke is never shown as active or silently dropped",
        "evidence:m5-rotation-revoke-event-row-parity:001",
        &[
            M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
            M5_ROTATION_REVOKE_EVENT_ROW_SCHEMA_REF,
            M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_HANDLE_REF,
        ],
    );
    row.lifecycle_states = LC::ALL.to_vec();
    row.required_labels = labels_with(&[L::ExpiryAndExportBoundary]);
    row.consumer_surfaces = vec![
        C::CredentialSettingsUi,
        C::StatusBarUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::LifecycleStateHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Export-safety banner.
    let mut row = base_row(
        F::ExportSafetyBanner,
        Q::Stable,
        "Export-safety banner owner",
        "One export-safety-banner model naming what an export will and will not reveal — raw secret excluded, metadata only, handle reference only, redacted share, endpoints masked, or export blocked — alongside the reveal posture it honours, so no support or export flow ever implies a raw secret is export-safe and every export names its boundary before it runs",
        "evidence:m5-export-safety-banner-parity:001",
        &[
            M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
            M5_EXPORT_SAFETY_BANNER_SCHEMA_REF,
            M5_CREDENTIAL_COMPONENT_FOUNDATION_EXPORT_REDACTION_REF,
        ],
    );
    row.reveal_postures = RP::ALL.to_vec();
    row.export_safety_classes = ES::ALL.to_vec();
    row.required_labels = labels_with(&[L::ExpiryAndExportBoundary]);
    row.consumer_surfaces = vec![
        C::SupportExport,
        C::CredentialSettingsUi,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ExportSafetyBoundaryHidden,
        D::RevealPostureUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5CredentialComponentGovernanceReview {
    M5CredentialComponentGovernanceReview {
        credential_state_row_shows_storage_and_reveal: true,
        secret_access_prompt_shows_class_and_handoff: true,
        vault_picker_shows_storage_and_capability: true,
        store_capability_row_shows_capability_and_degraded: true,
        handoff_card_shows_auth_handoff_class: true,
        delegated_row_shows_identity_state: true,
        rotation_revoke_row_shows_lifecycle_state: true,
        export_safety_banner_shows_export_boundary: true,
        no_surface_invents_alternate_state_label: true,
        storage_mode_vocabulary_named_once: true,
        reveal_and_export_safety_named_once: true,
        delegated_identity_always_explicit: true,
        session_only_fallback_always_explicit: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5CredentialComponentConsumerProjection {
    M5CredentialComponentConsumerProjection {
        credential_surfaces_consume_storage_vocabulary: true,
        prompt_surfaces_consume_handoff_vocabulary: true,
        delegated_surfaces_consume_identity_vocabulary: true,
        lifecycle_surfaces_consume_rotation_vocabulary: true,
        export_surfaces_consume_safety_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5CredentialComponentProofFreshness {
    M5CredentialComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CredentialComponentReleasePosture {
    M5CredentialComponentReleasePosture {
        proof_packet_ref: M5_CREDENTIAL_COMPONENT_ARTIFACT_REF.to_owned(),
        credential_audit_ref: M5_CREDENTIAL_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_DOC_REF,
        M5_CREDENTIAL_STATE_ROW_SCHEMA_REF,
        M5_SECRET_ACCESS_PROMPT_SHEET_SCHEMA_REF,
        M5_VAULT_KEYCHAIN_PICKER_SCHEMA_REF,
        M5_CREDENTIAL_STORE_CAPABILITY_ROW_SCHEMA_REF,
        M5_BROWSER_DEVICE_CODE_HANDOFF_CARD_SCHEMA_REF,
        M5_DELEGATED_CREDENTIAL_ROW_SCHEMA_REF,
        M5_ROTATION_REVOKE_EVENT_ROW_SCHEMA_REF,
        M5_EXPORT_SAFETY_BANNER_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_STATE_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_ACCESS_PROMPT_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_PICKER_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_HANDLE_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_SYSTEM_BROWSER_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_PROJECTION_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_EXPORT_REDACTION_REF,
    ])
}

/// Builds the canonical frozen M5 credential component matrix packet.
pub fn seeded_m5_credential_component_matrix() -> M5CredentialComponentMatrixPacket {
    M5CredentialComponentMatrixPacket::new(M5CredentialComponentMatrixPacketInput {
        packet_id: M5_CREDENTIAL_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 credential-state-row, secret-access-prompt-sheet, vault-or-keychain-picker, credential-store-capability-row, browser-device-code-handoff-card, delegated-credential-row, rotation-revoke-event-row, and export-safety-banner component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5CredentialComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the browser/device-code handoff card is held at Beta because a slice
/// of the passkey-step-up handoff does not yet round-trip across every surface; every
/// component stays visible.
pub fn seeded_m5_credential_component_matrix_browser_device_code_handoff_card_beta_narrowed(
) -> M5CredentialComponentMatrixPacket {
    let mut packet = seeded_m5_credential_component_matrix();
    packet.packet_id =
        "m5-credential-components:browser-device-code-handoff-card-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5CredentialComponentFamily::BrowserDeviceCodeHandoffCard
        })
        .expect("browser-device-code-handoff-card row present");
    row.qualification = M5CredentialQualificationClass::Beta;
    packet
}

/// Narrowed variant: the export-safety banner is narrowed to Preview pending export-safety
/// parity proof across every surface; every component stays visible.
pub fn seeded_m5_credential_component_matrix_export_safety_banner_preview_narrowed(
) -> M5CredentialComponentMatrixPacket {
    let mut packet = seeded_m5_credential_component_matrix();
    packet.packet_id = "m5-credential-components:export-safety-banner-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CredentialComponentFamily::ExportSafetyBanner)
        .expect("export-safety-banner row present");
    row.qualification = M5CredentialQualificationClass::Preview;
    packet
}
