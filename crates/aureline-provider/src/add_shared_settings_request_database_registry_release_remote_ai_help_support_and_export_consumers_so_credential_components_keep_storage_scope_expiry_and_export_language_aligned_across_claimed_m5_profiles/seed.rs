//! Canonical seed builders for the M5 credential component-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked bindings, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical credential component-consumer packet.
pub const M5_CREDENTIAL_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-credential-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5CredentialComponentConsumer,
    component_family: M5CredentialComponentFamily,
    parity_health: M5CredentialConsumerParityHealth,
    export_caveats: &[M5CredentialConsumerExportCaveat],
    note: &str,
) -> M5CredentialComponentBindingCase {
    M5CredentialComponentBindingCase::resolved(M5CredentialComponentBindingInput {
        consumer,
        component_family,
        descriptor_families: M5CredentialComponentDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5CredentialComponentFamily,
    example_bindings: Vec<M5CredentialComponentBindingCase>,
) -> M5CredentialComponentBinding {
    M5CredentialComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5CredentialComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5CredentialComponentBinding>,
) -> M5CredentialComponentConsumerRow {
    M5CredentialComponentConsumerRow {
        consumer,
        qualification: M5CredentialQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5CredentialSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CredentialDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5CredentialConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5CredentialComponentDescriptor::ALL.to_vec(),
        parity_health_modes: M5CredentialConsumerParityHealth::ALL.to_vec(),
        export_caveats: M5CredentialConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5CredentialClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5CredentialConsumerNarrowingReason::ALL.to_vec(),
        recovery_actions: M5CredentialConsumerRecoveryAction::ALL.to_vec(),
        export_fields: M5CredentialConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5CredentialAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CredentialConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5CredentialDowngradeTrigger::StorageModeUnstated,
            M5CredentialDowngradeTrigger::RevealPostureUnstated,
            M5CredentialDowngradeTrigger::DelegatedIdentityUnstated,
            M5CredentialDowngradeTrigger::LifecycleStateHidden,
            M5CredentialDowngradeTrigger::ExportSafetyBoundaryHidden,
            M5CredentialDowngradeTrigger::FriendlyConnectedWordingUsed,
            M5CredentialDowngradeTrigger::SessionOnlyFallbackHidden,
            M5CredentialDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_CREDENTIAL_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_CREDENTIAL_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_credential_grammar: false,
        drops_storage_reveal_delegation_expiry_or_export_truth_when_narrowed: false,
        shows_unusable_or_forwarded_state_as_usable_and_local: false,
        inherits_stronger_label_from_healthier_profile: false,
        uses_friendly_connected_wording: false,
    }
}

#[allow(clippy::vec_init_then_push)]
fn consumer_rows() -> Vec<M5CredentialComponentConsumerRow> {
    use M5CredentialComponentConsumer as Consumer;
    use M5CredentialComponentFamily as Family;
    use M5CredentialConsumerExportCaveat as Caveat;
    use M5CredentialConsumerParityHealth as Health;

    let mut rows = Vec::new();

    // 1. Credential settings — the credential-state row and vault-or-keychain picker at full
    //    parity (storage mode, credential class, reveal posture, expiry), plus the
    //    rotation/revoke-event row auto-narrowed because the credential it reports is expired or
    //    revoked and no longer usable.
    rows.push(base_row(
        Consumer::Settings,
        "Credential-settings surface owner",
        "Credential settings adopt the credential-state row and vault-or-keychain picker at full parity, pointing at the canonical component schemas so storage mode, credential class, handle-only-versus-raw-reveal posture, forwarded/delegated identity, expiry, and raw-secret-excluded export safety match what request, database, registry, release, remote, AI, Help / docs, the support / export desk, and the export packet read; the rotation/revoke-event row auto-narrows when the credential is expired or revoked",
        "evidence:m5-credential-consumer-settings:001",
        vec![
            binding(
                Family::CredentialStateRow,
                vec![case(
                    Consumer::Settings,
                    Family::CredentialStateRow,
                    Health::FullParity,
                    &[],
                    "settings credential-state row at full parity",
                )],
            ),
            binding(
                Family::VaultOrKeychainPicker,
                vec![case(
                    Consumer::Settings,
                    Family::VaultOrKeychainPicker,
                    Health::FullParity,
                    &[],
                    "settings vault-or-keychain picker at full parity",
                )],
            ),
            binding(
                Family::RotationRevokeEventRow,
                vec![case(
                    Consumer::Settings,
                    Family::RotationRevokeEventRow,
                    Health::ExpiredOrRevokedNarrowed,
                    &[Caveat::ExpiredOrRevokedNotUsable],
                    "settings rotation/revoke row narrowed by expired or revoked credential",
                )],
            ),
        ],
    ));

    // 2. Request auth surface — the secret-access-prompt sheet auto-narrowed to a handle-only
    //    path (no raw secret exposed) and the credential-state row at full parity.
    rows.push(base_row(
        Consumer::Request,
        "Request-auth surface owner",
        "The request auth surface adopts the secret-access-prompt sheet auto-narrowed to a handle-only path because no raw secret is exposed here, and the credential-state row at full parity, keeping storage mode, credential class, reveal posture, forwarded/delegated identity, expiry, and raw-secret-excluded export safety explicit so a handle-only path is never mistaken for a raw-reveal one",
        "evidence:m5-credential-consumer-request:001",
        vec![
            binding(
                Family::SecretAccessPromptSheet,
                vec![case(
                    Consumer::Request,
                    Family::SecretAccessPromptSheet,
                    Health::HandleOnlyNarrowed,
                    &[Caveat::HandleOnlyNoRawExport],
                    "request secret-access prompt narrowed to handle-only path",
                )],
            ),
            binding(
                Family::CredentialStateRow,
                vec![case(
                    Consumer::Request,
                    Family::CredentialStateRow,
                    Health::FullParity,
                    &[],
                    "request credential-state row at full parity",
                )],
            ),
        ],
    ));

    // 3. Database attach — the credential-state row at full parity, plus the delegated-credential
    //    row auto-narrowed because the identity is forwarded or delegated from another principal
    //    and is not a locally stored credential.
    rows.push(base_row(
        Consumer::Database,
        "Database-attach surface owner",
        "The database attach surface adopts the credential-state row at full parity and the delegated-credential row auto-narrowed because the identity is forwarded or delegated from another principal, keeping storage mode, credential class, reveal posture, forwarded/delegated identity, expiry, and raw-secret-excluded export safety explicit so a forwarded identity never reads as a locally stored credential",
        "evidence:m5-credential-consumer-database:001",
        vec![
            binding(
                Family::CredentialStateRow,
                vec![case(
                    Consumer::Database,
                    Family::CredentialStateRow,
                    Health::FullParity,
                    &[],
                    "database credential-state row at full parity",
                )],
            ),
            binding(
                Family::DelegatedCredentialRow,
                vec![case(
                    Consumer::Database,
                    Family::DelegatedCredentialRow,
                    Health::DelegatedOrForwardedNarrowed,
                    &[Caveat::ForwardedOrDelegatedNotLocal],
                    "database delegated-credential row narrowed by forwarded identity",
                )],
            ),
        ],
    ));

    // 4. Registry / provider authorization — the secret-access-prompt sheet and
    //    credential-store-capability row at full parity, plus the browser/device-code handoff
    //    card auto-narrowed because the handoff credential is held only for this session or is
    //    blocked by policy.
    rows.push(base_row(
        Consumer::Registry,
        "Registry-authorization surface owner",
        "The registry/provider authorization surface adopts the secret-access-prompt sheet and credential-store-capability row at full parity, and the browser/device-code handoff card auto-narrowed because the handoff credential is held only for this session or is policy-blocked, keeping storage mode, credential class, reveal posture, forwarded/delegated identity, expiry, and raw-secret-excluded export safety disclosed so a session-only fallback never reads as durably stored before send, run, or publish",
        "evidence:m5-credential-consumer-registry:001",
        vec![
            binding(
                Family::SecretAccessPromptSheet,
                vec![case(
                    Consumer::Registry,
                    Family::SecretAccessPromptSheet,
                    Health::FullParity,
                    &[],
                    "registry secret-access prompt at full parity",
                )],
            ),
            binding(
                Family::BrowserDeviceCodeHandoffCard,
                vec![case(
                    Consumer::Registry,
                    Family::BrowserDeviceCodeHandoffCard,
                    Health::SessionOnlyOrPolicyBlockedNarrowed,
                    &[Caveat::SessionOnlyOrPolicyBlockedNotDurable],
                    "registry handoff card narrowed by session-only credential",
                )],
            ),
            binding(
                Family::CredentialStoreCapabilityRow,
                vec![case(
                    Consumer::Registry,
                    Family::CredentialStoreCapabilityRow,
                    Health::FullParity,
                    &[],
                    "registry credential-store-capability row at full parity",
                )],
            ),
        ],
    ));

    // 5. Release publish — the rotation/revoke-event row and export-safety banner at full parity:
    //    a signing credential's lifecycle and the raw-secret-excluded export boundary are exact.
    rows.push(base_row(
        Consumer::Release,
        "Release-publish surface owner",
        "The release publish surface adopts the rotation/revoke-event row and export-safety banner at full parity, referencing the canonical component schemas so credential lifecycle state and the raw-secret-excluded export boundary stay one truth and a signing credential's expiry or revocation is never left to inference",
        "evidence:m5-credential-consumer-release:001",
        vec![
            binding(
                Family::RotationRevokeEventRow,
                vec![case(
                    Consumer::Release,
                    Family::RotationRevokeEventRow,
                    Health::FullParity,
                    &[],
                    "release rotation/revoke row at full parity",
                )],
            ),
            binding(
                Family::ExportSafetyBanner,
                vec![case(
                    Consumer::Release,
                    Family::ExportSafetyBanner,
                    Health::FullParity,
                    &[],
                    "release export-safety banner at full parity",
                )],
            ),
        ],
    ));

    // 6. Remote-target attach — the delegated-credential row and browser/device-code handoff card
    //    at full parity: a forwarded target credential and its handoff class stay explicit.
    rows.push(base_row(
        Consumer::Remote,
        "Remote-target attach surface owner",
        "The remote-target attach surface adopts the delegated-credential row and browser/device-code handoff card at full parity, keeping storage mode, credential class, reveal posture, forwarded/delegated identity, expiry, and raw-secret-excluded export safety explicit so an attached remote target discloses exactly which identity it forwards and how the handoff completes",
        "evidence:m5-credential-consumer-remote:001",
        vec![
            binding(
                Family::DelegatedCredentialRow,
                vec![case(
                    Consumer::Remote,
                    Family::DelegatedCredentialRow,
                    Health::FullParity,
                    &[],
                    "remote delegated-credential row at full parity",
                )],
            ),
            binding(
                Family::BrowserDeviceCodeHandoffCard,
                vec![case(
                    Consumer::Remote,
                    Family::BrowserDeviceCodeHandoffCard,
                    Health::FullParity,
                    &[],
                    "remote handoff card at full parity",
                )],
            ),
        ],
    ));

    // 7. AI model provider — the secret-access-prompt sheet at full parity, plus the
    //    credential-state row auto-narrowed to a handle-only path so a model-provider key is
    //    referenced by handle and never raw-copied here.
    rows.push(base_row(
        Consumer::AiAssistant,
        "AI model-provider surface owner",
        "The AI model-provider surface adopts the secret-access-prompt sheet at full parity and the credential-state row auto-narrowed to a handle-only path because a model-provider credential is referenced by handle and never raw-copied here, keeping storage mode, credential class, reveal posture, forwarded/delegated identity, expiry, and raw-secret-excluded export safety explicit so a handle-only path is never nudged toward a raw reveal",
        "evidence:m5-credential-consumer-ai:001",
        vec![
            binding(
                Family::SecretAccessPromptSheet,
                vec![case(
                    Consumer::AiAssistant,
                    Family::SecretAccessPromptSheet,
                    Health::FullParity,
                    &[],
                    "ai secret-access prompt at full parity",
                )],
            ),
            binding(
                Family::CredentialStateRow,
                vec![case(
                    Consumer::AiAssistant,
                    Family::CredentialStateRow,
                    Health::HandleOnlyNarrowed,
                    &[Caveat::HandleOnlyNoRawExport],
                    "ai credential-state row narrowed to handle-only path",
                )],
            ),
        ],
    ));

    // 8. Help / docs — the credential-state row, secret-access-prompt sheet, and export-safety
    //    banner all at full parity: documentation describes the same storage, reveal, and export
    //    truth the product renders.
    rows.push(base_row(
        Consumer::Help,
        "Help / docs surface owner",
        "Help / docs adopt the credential-state row, secret-access-prompt sheet, and export-safety banner at full parity, referencing the canonical component schemas so storage mode, credential class, reveal posture, forwarded/delegated identity, expiry, and raw-secret-excluded export safety stay one truth across every claimed credential surface rather than being re-worded in prose",
        "evidence:m5-credential-consumer-help:001",
        vec![
            binding(
                Family::CredentialStateRow,
                vec![case(
                    Consumer::Help,
                    Family::CredentialStateRow,
                    Health::FullParity,
                    &[],
                    "help credential-state row at full parity",
                )],
            ),
            binding(
                Family::SecretAccessPromptSheet,
                vec![case(
                    Consumer::Help,
                    Family::SecretAccessPromptSheet,
                    Health::FullParity,
                    &[],
                    "help secret-access prompt at full parity",
                )],
            ),
            binding(
                Family::ExportSafetyBanner,
                vec![case(
                    Consumer::Help,
                    Family::ExportSafetyBanner,
                    Health::FullParity,
                    &[],
                    "help export-safety banner at full parity",
                )],
            ),
        ],
    ));

    // 9. Support / export desk — all eight families, referencing the canonical schemas so its
    //    prose can never drift from the product truth. This is the authoritative rendering every
    //    other surface keeps parity with.
    rows.push(base_row(
        Consumer::Support,
        "Support / export desk surface owner",
        "The support / export desk adopts the credential-state row, secret-access-prompt sheet, vault-or-keychain picker, credential-store-capability row, browser/device-code handoff card, delegated-credential row, rotation/revoke-event row, and export-safety banner, referencing the canonical component schemas so its prose can never drift from the product truth and keeping storage mode, credential class, reveal posture, forwarded/delegated identity, expiry, and raw-secret-excluded export safety exact in every exported case",
        "evidence:m5-credential-consumer-support:001",
        vec![
            binding(
                Family::CredentialStateRow,
                vec![case(
                    Consumer::Support,
                    Family::CredentialStateRow,
                    Health::FullParity,
                    &[],
                    "support credential-state row at full parity",
                )],
            ),
            binding(
                Family::SecretAccessPromptSheet,
                vec![case(
                    Consumer::Support,
                    Family::SecretAccessPromptSheet,
                    Health::FullParity,
                    &[],
                    "support secret-access prompt at full parity",
                )],
            ),
            binding(
                Family::VaultOrKeychainPicker,
                vec![case(
                    Consumer::Support,
                    Family::VaultOrKeychainPicker,
                    Health::FullParity,
                    &[],
                    "support vault-or-keychain picker at full parity",
                )],
            ),
            binding(
                Family::CredentialStoreCapabilityRow,
                vec![case(
                    Consumer::Support,
                    Family::CredentialStoreCapabilityRow,
                    Health::FullParity,
                    &[],
                    "support credential-store-capability row at full parity",
                )],
            ),
            binding(
                Family::BrowserDeviceCodeHandoffCard,
                vec![case(
                    Consumer::Support,
                    Family::BrowserDeviceCodeHandoffCard,
                    Health::FullParity,
                    &[],
                    "support handoff card at full parity",
                )],
            ),
            binding(
                Family::DelegatedCredentialRow,
                vec![case(
                    Consumer::Support,
                    Family::DelegatedCredentialRow,
                    Health::FullParity,
                    &[],
                    "support delegated-credential row at full parity",
                )],
            ),
            binding(
                Family::RotationRevokeEventRow,
                vec![case(
                    Consumer::Support,
                    Family::RotationRevokeEventRow,
                    Health::FullParity,
                    &[],
                    "support rotation/revoke row at full parity",
                )],
            ),
            binding(
                Family::ExportSafetyBanner,
                vec![case(
                    Consumer::Support,
                    Family::ExportSafetyBanner,
                    Health::FullParity,
                    &[],
                    "support export-safety banner at full parity",
                )],
            ),
        ],
    ));

    // 10. Export packet — the export-safety banner, credential-store-capability row, and
    //     vault-or-keychain picker at full parity, so an exported credential packet carries the
    //     raw-secret-excluded boundary and the store it came from without leaking material.
    rows.push(base_row(
        Consumer::Export,
        "Export packet surface owner",
        "The export packet adopts the export-safety banner, credential-store-capability row, and vault-or-keychain picker at full parity, keeping storage mode, credential class, reveal posture, forwarded/delegated identity, expiry, and raw-secret-excluded export safety explicit so an exported packet always states its raw-secret-excluded boundary and never implies a raw secret is exportable",
        "evidence:m5-credential-consumer-export:001",
        vec![
            binding(
                Family::ExportSafetyBanner,
                vec![case(
                    Consumer::Export,
                    Family::ExportSafetyBanner,
                    Health::FullParity,
                    &[],
                    "export export-safety banner at full parity",
                )],
            ),
            binding(
                Family::CredentialStoreCapabilityRow,
                vec![case(
                    Consumer::Export,
                    Family::CredentialStoreCapabilityRow,
                    Health::FullParity,
                    &[],
                    "export credential-store-capability row at full parity",
                )],
            ),
            binding(
                Family::VaultOrKeychainPicker,
                vec![case(
                    Consumer::Export,
                    Family::VaultOrKeychainPicker,
                    Health::FullParity,
                    &[],
                    "export vault-or-keychain picker at full parity",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5CredentialComponentConsumerGovernanceReview {
    M5CredentialComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        storage_class_reveal_delegation_expiry_export_explicit_on_every_surface: true,
        degraded_state_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        unusable_or_forwarded_state_never_shown_as_usable_and_local: true,
        no_friendly_connected_wording_conceals_storage_delegation_or_reveal: true,
        help_support_export_present_same_credential_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5CredentialComponentConsumerProjection {
    M5CredentialComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        storage_mode_reads_single_source: true,
        credential_class_reads_single_source: true,
        reveal_posture_reads_single_source: true,
        delegated_identity_reads_single_source: true,
        export_safety_reads_single_source: true,
    }
}

fn proof_freshness() -> M5CredentialComponentConsumerProofFreshness {
    M5CredentialComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CredentialComponentConsumerReleasePosture {
    M5CredentialComponentConsumerReleasePosture {
        release_packet_ref: M5_CREDENTIAL_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        credential_consumer_audit_ref: M5_CREDENTIAL_COMPONENT_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CREDENTIAL_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_CONSUMER_DOC_REF,
        M5_CREDENTIAL_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_CREDENTIAL_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        family_canonical_schema_ref(M5CredentialComponentFamily::CredentialStateRow),
        family_canonical_schema_ref(M5CredentialComponentFamily::SecretAccessPromptSheet),
        family_canonical_schema_ref(M5CredentialComponentFamily::BrowserDeviceCodeHandoffCard),
        family_canonical_schema_ref(M5CredentialComponentFamily::RotationRevokeEventRow),
    ])
}

/// Builds the canonical M5 credential component-consumer packet.
pub fn seeded_m5_credential_component_consumer_packet() -> M5CredentialComponentConsumerPacket {
    M5CredentialComponentConsumerPacket::new(M5CredentialComponentConsumerPacketInput {
        packet_id: M5_CREDENTIAL_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 credential component consumers: credential settings, the request auth surface, database attach, registry/provider auth, release publish, remote attach, the AI model provider, Help / docs, the support / export desk, and the export packet keep storage, scope, expiry, and export parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5CredentialComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the registry/provider authorization surface is held at Beta because a slice
/// of registry renderings still resolve a session-only or policy-blocked handoff credential;
/// every consumer stays visible.
pub fn seeded_m5_credential_component_consumer_registry_beta_narrowed(
) -> M5CredentialComponentConsumerPacket {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.packet_id = "m5-credential-component-consumer:registry-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5CredentialComponentConsumer::Registry)
        .expect("registry row present");
    row.qualification = M5CredentialQualificationClass::Beta;
    packet
}

/// Narrowed variant: the database attach surface is held at Preview because a slice of database
/// renderings still resolve a forwarded or delegated identity; every consumer stays visible.
pub fn seeded_m5_credential_component_consumer_database_preview_narrowed(
) -> M5CredentialComponentConsumerPacket {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.packet_id = "m5-credential-component-consumer:database-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5CredentialComponentConsumer::Database)
        .expect("database row present");
    row.qualification = M5CredentialQualificationClass::Preview;
    packet
}
