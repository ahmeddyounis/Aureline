//! Canonical seed builders for the M5 input-method and credential-store-wording registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean input-composition and credential-wording
//! entries are built so the committed text arriving intact across the macOS / Windows / Linux input stacks,
//! the preserved command / shortcut / trust fidelity, the truthful and non-leaky credential copy, the
//! literal / canonical / accessible presentation forms, and the generic-wording / disclosure-route /
//! truthful disclosure triple are proven across the shell, settings, docs, onboarding, CLI, and support
//! surfaces without any hand-copied per-platform assumption, corrupted composition, shortcut-composition
//! fight, hidden plaintext downgrade, or presentation-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_INPUT_CREDENTIAL_REGISTRIES_PACKET_ID: &str =
    "m5-input-method-and-credential-store-wording-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn input(input: M5InputCompositionEntryResolutionInput) -> M5ResolvedInputCompositionEntry {
    resolve_input_composition_entry(input).expect("seed input-composition entry resolves")
}

fn credential(
    input: M5CredentialStoreWordingEntryResolutionInput,
) -> M5ResolvedCredentialStoreWordingEntry {
    resolve_credential_store_wording_entry(input).expect("seed credential-wording entry resolves")
}

fn all_forms() -> Vec<M5InputCredentialPresentationForm> {
    M5InputCredentialPresentationForm::ALL.to_vec()
}

// -- Clean input-composition entries (text arrives intact, bound to the shared registry) ---------

#[allow(clippy::too_many_arguments)]
fn clean_input_base(
    entry_id: &str,
    command_id: &str,
    token_name: &str,
    semantic_role: M5PlatformFitRole,
    input_role: M5InputMethodRole,
    input_stack: M5InputMethodStack,
    surface_context: M5InputSurfaceContext,
    committed_text: &str,
) -> M5InputCompositionEntryResolutionInput {
    M5InputCompositionEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        command_id: command_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        input_role,
        input_stack,
        surface_context,
        presentation_form_coverage: all_forms(),
        committed_text: committed_text.to_owned(),
        expected_text: committed_text.to_owned(),
        bound_to_registry: true,
        preserves_command_and_trust_fidelity: true,
        composition_unsupported_on_surface: false,
        fallback_input_path_explained: true,
        proof_fresh: true,
    }
}

fn input_editor_clean() -> M5ResolvedInputCompositionEntry {
    input(clean_input_base(
        "input:editor:ime:macos",
        "command.editor.insert",
        "input.ime.editor.macos",
        M5PlatformFitRole::InputFidelity,
        M5InputMethodRole::ImeCompositionFidelity,
        M5InputMethodStack::MacosInputMethods,
        M5InputSurfaceContext::EditorBuffer,
        "日本語",
    ))
}

fn input_terminal_clean() -> M5ResolvedInputCompositionEntry {
    input(clean_input_base(
        "input:terminal:deadkey:linux",
        "command.terminal.insert",
        "input.deadkey.terminal.linux",
        M5PlatformFitRole::InputFidelity,
        M5InputMethodRole::DeadKeyAndAltGrFidelity,
        M5InputMethodStack::LinuxImeIbusFcitx,
        M5InputSurfaceContext::TerminalInput,
        "café",
    ))
}

fn input_settings_clean() -> M5ResolvedInputCompositionEntry {
    input(clean_input_base(
        "input:settings:altgr:windows",
        "command.settings.edit",
        "input.altgr.settings.windows",
        M5PlatformFitRole::CommandStability,
        M5InputMethodRole::DeadKeyAndAltGrFidelity,
        M5InputMethodStack::WindowsImeTsf,
        M5InputSurfaceContext::SettingsField,
        "€uro",
    ))
}

fn input_dialog_clean() -> M5ResolvedInputCompositionEntry {
    input(clean_input_base(
        "input:dialog:emoji:macos",
        "command.dialog.insert",
        "input.emoji.dialog.macos",
        M5PlatformFitRole::InputFidelity,
        M5InputMethodRole::DictationAndEmojiFidelity,
        M5InputMethodStack::MacosInputMethods,
        M5InputSurfaceContext::ModalDialog,
        "👍 done",
    ))
}

fn input_prompt_clean() -> M5ResolvedInputCompositionEntry {
    input(clean_input_base(
        "input:prompt:layout:windows",
        "command.prompt.insert",
        "input.layout.prompt.windows",
        M5PlatformFitRole::InputFidelity,
        M5InputMethodRole::LayoutSwitchFidelity,
        M5InputMethodStack::WindowsImeTsf,
        M5InputSurfaceContext::PromptOrSupportForm,
        "naïve",
    ))
}

// -- Degraded input-composition entries ---------------------------------------------------------

/// Degraded input entry: the behavior is a hand-copied per-platform assumption instead of tracing to the
/// registry, and names the disallowed corruption role.
fn input_unbound() -> M5ResolvedInputCompositionEntry {
    let mut base = clean_input_base(
        "input:editor:unbound",
        "command.editor.insert",
        "input.ime.editor.macos",
        M5PlatformFitRole::InputFidelity,
        M5InputMethodRole::TextOrTrustCorruptionDisallowed,
        M5InputMethodStack::MacosInputMethods,
        M5InputSurfaceContext::EditorBuffer,
        "日本語",
    );
    base.bound_to_registry = false;
    input(base)
}

/// Degraded input entry: the committed text drifts from the expected text — composition corrupted it.
fn input_text_corrupted() -> M5ResolvedInputCompositionEntry {
    let mut base = clean_input_base(
        "input:terminal:corrupted:linux",
        "command.terminal.insert",
        "input.deadkey.terminal.linux",
        M5PlatformFitRole::InputFidelity,
        M5InputMethodRole::DeadKeyAndAltGrFidelity,
        M5InputMethodStack::LinuxImeIbusFcitx,
        M5InputSurfaceContext::TerminalInput,
        "cafe",
    );
    // The dead-key composition should have delivered `café`, not `cafe`.
    base.expected_text = "café".to_owned();
    input(base)
}

/// Degraded input entry: the composition fights shortcut routing / rewrites trust copy — command and trust
/// fidelity is not preserved.
fn input_fidelity_lost() -> M5ResolvedInputCompositionEntry {
    let mut base = clean_input_base(
        "input:dialog:fidelity-lost:macos",
        "command.dialog.insert",
        "input.emoji.dialog.macos",
        M5PlatformFitRole::InputFidelity,
        M5InputMethodRole::ImeCompositionFidelity,
        M5InputMethodStack::MacosInputMethods,
        M5InputSurfaceContext::ModalDialog,
        "👍 done",
    );
    base.preserves_command_and_trust_fidelity = false;
    input(base)
}

/// Degraded input entry: the literal / canonical / accessible presentation-form coverage is incomplete.
fn input_form_incomplete() -> M5ResolvedInputCompositionEntry {
    let mut base = clean_input_base(
        "input:settings:form-incomplete:windows",
        "command.settings.edit",
        "input.altgr.settings.windows",
        M5PlatformFitRole::CommandStability,
        M5InputMethodRole::DeadKeyAndAltGrFidelity,
        M5InputMethodStack::WindowsImeTsf,
        M5InputSurfaceContext::SettingsField,
        "€uro",
    );
    base.presentation_form_coverage = vec![M5InputCredentialPresentationForm::LiteralRendering];
    input(base)
}

/// Degraded input entry: composition is unsupported on this surface and no fallback input path is explained.
fn input_composition_unhandled() -> M5ResolvedInputCompositionEntry {
    let mut base = clean_input_base(
        "input:prompt:unhandled:windows",
        "command.prompt.insert",
        "input.layout.prompt.windows",
        M5PlatformFitRole::InputFidelity,
        M5InputMethodRole::LayoutSwitchFidelity,
        M5InputMethodStack::WindowsImeTsf,
        M5InputSurfaceContext::PromptOrSupportForm,
        "naïve",
    );
    base.composition_unsupported_on_surface = true;
    base.fallback_input_path_explained = false;
    input(base)
}

/// Degraded input entry: the canonical registry token name is unstated.
fn input_token_unstated() -> M5ResolvedInputCompositionEntry {
    let mut base = clean_input_base(
        "input:support:token-unstated:macos",
        "command.editor.insert",
        "  ",
        M5PlatformFitRole::InputFidelity,
        M5InputMethodRole::ImeCompositionFidelity,
        M5InputMethodStack::MacosInputMethods,
        M5InputSurfaceContext::EditorBuffer,
        "日本語",
    );
    base.token_name = "  ".to_owned();
    input(base)
}

// -- Clean credential-store-wording entries -----------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_credential_base(
    entry_id: &str,
    command_id: &str,
    token_name: &str,
    wording_role: M5CredentialStoreWordingRole,
    wording_surface: M5CredentialWordingSurface,
    surface_context: M5InputSurfaceContext,
    generic_wording: &str,
    disclosure_route: &str,
) -> M5CredentialStoreWordingEntryResolutionInput {
    M5CredentialStoreWordingEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        command_id: command_id.to_owned(),
        token_name: token_name.to_owned(),
        wording_role,
        semantic_role: M5PlatformFitRole::CredentialWording,
        wording_surface,
        surface_context,
        presentation_form_coverage: all_forms(),
        generic_wording: generic_wording.to_owned(),
        disclosure_route: disclosure_route.to_owned(),
        storage_is_truthful: true,
        non_leaky: true,
        plaintext_fallback_used: false,
        plaintext_fallback_disclosed: false,
        platform_detail_disclosed: false,
        platform_detail_justified: false,
        proof_fresh: true,
    }
}

fn cred_settings_clean() -> M5ResolvedCredentialStoreWordingEntry {
    credential(clean_credential_base(
        "cred:settings:store",
        "command.auth.store.inspect",
        "cred.store.settings",
        M5CredentialStoreWordingRole::TruthfulStorageClaim,
        M5CredentialWordingSurface::SettingsCredentialPanel,
        M5InputSurfaceContext::SettingsField,
        "Your sign-in credentials are kept in the system secure store and are never shown here.",
        "settings.credentials.help",
    ))
}

fn cred_auth_clean() -> M5ResolvedCredentialStoreWordingEntry {
    // A justified platform-detail disclosure stays clean: it materially helps recovery on this host.
    let mut base = clean_credential_base(
        "cred:auth:recovery",
        "command.auth.recover",
        "cred.recovery.auth",
        M5CredentialStoreWordingRole::HostCredentialStoreName,
        M5CredentialWordingSurface::AuthErrorDialog,
        M5InputSurfaceContext::ModalDialog,
        "We could not read your saved credentials from the system secure store; sign in again to repair them.",
        "auth.recovery.repair",
    );
    base.platform_detail_disclosed = true;
    base.platform_detail_justified = true;
    credential(base)
}

fn cred_support_clean() -> M5ResolvedCredentialStoreWordingEntry {
    // A disclosed plaintext fallback stays clean: it is surfaced honestly rather than hidden.
    let mut base = clean_credential_base(
        "cred:support:diagnostics",
        "command.support.credential.diagnose",
        "cred.diagnostics.support",
        M5CredentialStoreWordingRole::FallbackDisclosure,
        M5CredentialWordingSurface::SupportDiagnostics,
        M5InputSurfaceContext::PromptOrSupportForm,
        "This report shows only whether a credential is present, and discloses that an encrypted-file fallback is in use on this profile.",
        "support.credential.diagnostics",
    );
    base.plaintext_fallback_used = true;
    base.plaintext_fallback_disclosed = true;
    credential(base)
}

// -- Degraded credential-store-wording entries --------------------------------------------------

/// Degraded credential entry: a plaintext fallback was used but not disclosed — the wording hides a storage
/// downgrade and reads as truthful when it is not.
fn cred_untruthful() -> M5ResolvedCredentialStoreWordingEntry {
    let mut base = clean_credential_base(
        "cred:settings:hidden-downgrade",
        "command.auth.store.inspect",
        "cred.store.settings",
        M5CredentialStoreWordingRole::PlaintextFallbackHiddenDisallowed,
        M5CredentialWordingSurface::SettingsCredentialPanel,
        M5InputSurfaceContext::SettingsField,
        "Your sign-in credentials are securely encrypted in the system secure store.",
        "settings.credentials.help",
    );
    base.plaintext_fallback_used = true;
    base.plaintext_fallback_disclosed = false;
    credential(base)
}

/// Degraded credential entry: the literal / canonical / accessible presentation-form coverage of the wording
/// is incomplete.
fn cred_phrasing_incomplete() -> M5ResolvedCredentialStoreWordingEntry {
    let mut base = clean_credential_base(
        "cred:support:phrasing-incomplete",
        "command.support.credential.diagnose",
        "cred.diagnostics.support",
        M5CredentialStoreWordingRole::NonLeakyWording,
        M5CredentialWordingSurface::SupportDiagnostics,
        M5InputSurfaceContext::PromptOrSupportForm,
        "This report shows only whether a credential is present.",
        "support.credential.diagnostics",
    );
    base.presentation_form_coverage = vec![M5InputCredentialPresentationForm::LiteralRendering];
    credential(base)
}

/// Degraded credential entry: the credential-wording surface is unclassified.
fn cred_surface_unclassified() -> M5ResolvedCredentialStoreWordingEntry {
    credential(clean_credential_base(
        "cred:onboarding:surface-unclassified",
        "command.auth.store.inspect",
        "cred.store.unknown",
        M5CredentialStoreWordingRole::TruthfulStorageClaim,
        M5CredentialWordingSurface::SurfaceUnclassified,
        M5InputSurfaceContext::SettingsField,
        "Your sign-in credentials are kept in the system secure store.",
        "settings.credentials.help",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5InputCredentialRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5PlatformFitDowngradeTrigger>,
    input_composition_entries: Vec<M5ResolvedInputCompositionEntry>,
    credential_store_wording_entries: Vec<M5ResolvedCredentialStoreWordingEntry>,
) -> M5InputCredentialRegistriesRow {
    M5InputCredentialRegistriesRow {
        consumer_surface,
        qualification: M5PlatformFitQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5PlatformFitDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5PlatformFitRequiredLabel::Identity,
            M5PlatformFitRequiredLabel::SemanticRole,
            M5PlatformFitRequiredLabel::RegistryReference,
            M5PlatformFitRequiredLabel::HostPlatform,
            M5PlatformFitRequiredLabel::ShortcutNotation,
        ],
        accessibility_routes: M5PlatformFitAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5InputCredentialAnatomyPart::ALL.to_vec(),
        export_fields: M5InputCredentialExportField::ALL.to_vec(),
        downgrade_triggers,
        input_composition_entries,
        credential_store_wording_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_INPUT_CREDENTIAL_REGISTRIES_SCHEMA_REF,
            M5_INPUT_METHOD_BEHAVIOR_SCHEMA_REF,
            M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
        ]),
        input_method_corrupts_text_command_or_trust: false,
        shortcut_routing_and_composition_fight: false,
        credential_wording_hides_plaintext_downgrade_or_leaks: false,
        input_or_credential_wording_hardcoded_instead_of_registry: false,
    }
}

fn registry_rows() -> Vec<M5InputCredentialRegistriesRow> {
    use M5PlatformFitConsumerSurface as C;
    use M5PlatformFitDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The editor delivers macOS marked-text IME composition intact from the shared input registry and keeps command interpretation and trust copy uncorrupted; a hand-copied per-platform composition assumption degrades honestly instead of reading as a clean pass, and a settings credential message that hides a plaintext downgrade is caught before it reads as truthful",
            "evidence:m5-input-credential-shell-ui:001",
            vec![
                D::InputMethodCorruptedTextOrTrust,
                D::SecretStorageFellBackToPlaintextSilently,
                D::ProofStale,
            ],
            vec![input_editor_clean(), input_unbound()],
            vec![cred_settings_clean(), cred_untruthful()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The terminal delivers Linux IBus / fcitx dead-key composition intact and the auth recovery dialog keeps its credential wording truthful; a dead-key composition that drifts from its expected text is caught as corrupted for its stack",
            "evidence:m5-input-credential-settings-ui:001",
            vec![
                D::InputMethodCorruptedTextOrTrust,
                D::PlatformWordingChangedCommandOrPermissionMeaning,
                D::ProofStale,
            ],
            vec![input_terminal_clean(), input_text_corrupted()],
            vec![cred_auth_clean()],
        ),
        base_row(
            C::DocsHelp,
            "Docs/help surface owner",
            "Docs and help render the Windows AltGr settings composition across the literal, canonical, and accessible presentation forms and keep the support credential diagnostics wording truthful; an input entry and a credential entry that omit a presentation form degrade honestly so a screenshot cannot reintroduce a false-truth reading",
            "evidence:m5-input-credential-docs-help:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SecretStorageFellBackToPlaintextSilently,
                D::ProofStale,
            ],
            vec![input_settings_clean(), input_form_incomplete()],
            vec![cred_support_clean(), cred_phrasing_incomplete()],
        ),
        base_row(
            C::Onboarding,
            "Onboarding surface owner",
            "Onboarding delivers macOS emoji composition from the registry while preserving command and trust fidelity; a composition that fights shortcut routing and a credential message on an unclassified surface degrade honestly",
            "evidence:m5-input-credential-onboarding:001",
            vec![
                D::PlatformWordingChangedCommandOrPermissionMeaning,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![input_dialog_clean(), input_fidelity_lost()],
            vec![cred_surface_unclassified()],
        ),
        base_row(
            C::CliExport,
            "CLI/export owner",
            "The CLI export delivers Windows layout-switch composition from the input registry and keeps the settings credential wording truthful; a composition unsupported on its surface without an explained fallback degrades honestly instead of silently dropping text entry",
            "evidence:m5-input-credential-cli-export:001",
            vec![
                D::InputMethodCorruptedTextOrTrust,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![input_prompt_clean(), input_composition_unhandled()],
            vec![cred_settings_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved input-composition and credential-wording truth, so a hand-copied constant, an unstated registry token, or a hidden storage downgrade is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-input-credential-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::InputMethodCorruptedTextOrTrust,
                D::ProofStale,
            ],
            vec![input_editor_clean(), input_token_unstated()],
            vec![cred_auth_clean()],
        ),
    ]
}

fn governance_review() -> M5InputCredentialRegistriesGovernanceReview {
    M5InputCredentialRegistriesGovernanceReview {
        input_registry_names_token_role_and_stack: true,
        text_arrives_intact_from_shared_registry: true,
        command_shortcut_and_trust_fidelity_preserved: true,
        shortcut_handling_and_composition_do_not_fight: true,
        credential_copy_truthful_and_generic_by_default: true,
        credential_wording_never_hides_downgrade_or_leaks: true,
        every_entry_covers_all_presentation_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        docs_help_and_screenshots_generated_from_registry: true,
        input_or_wording_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5InputCredentialRegistriesConsumerProjection {
    M5InputCredentialRegistriesConsumerProjection {
        editor_and_terminal_consume_shared_registries: true,
        settings_and_dialogs_consume_shared_registries: true,
        auth_consumes_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5InputCredentialRegistriesProofFreshness {
    M5InputCredentialRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5InputCredentialRegistriesReleasePosture {
    M5InputCredentialRegistriesReleasePosture {
        proof_packet_ref: M5_INPUT_CREDENTIAL_REGISTRIES_ARTIFACT_REF.to_owned(),
        platform_fit_audit_ref: M5_INPUT_CREDENTIAL_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_INPUT_CREDENTIAL_REGISTRIES_SCHEMA_REF,
        M5_INPUT_CREDENTIAL_REGISTRIES_DOC_REF,
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
        M5_PLATFORM_FIT_MATRIX_DOC_REF,
        M5_INPUT_METHOD_BEHAVIOR_SCHEMA_REF,
        M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 input-method and credential-store-wording registries packet.
pub fn seeded_m5_input_method_and_credential_store_wording_registries(
) -> M5InputCredentialRegistriesPacket {
    M5InputCredentialRegistriesPacket::new(M5InputCredentialRegistriesPacketInput {
        packet_id: M5_INPUT_CREDENTIAL_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 input-method and credential-store-wording registries with committed text arriving intact across the macOS / Windows / Linux input stacks, preserved command / shortcut / trust fidelity, truthful and non-leaky credential copy, literal / canonical / accessible presentation-form coverage, and the generic-wording / disclosure-route / truthful disclosure triple across shell, settings, docs, onboarding, CLI, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5InputCredentialRegistriesVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the docs/help row is held at Beta pending input-composition screenshot-generation parity
/// on every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_input_method_and_credential_store_wording_registries_composition_beta_narrowed(
) -> M5InputCredentialRegistriesPacket {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet.packet_id =
        "m5-input-method-and-credential-store-wording-registries:composition-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5PlatformFitConsumerSurface::DocsHelp)
        .expect("docs-help row present");
    row.qualification = M5PlatformFitQualificationClass::Beta;
    packet
}

/// Narrowed variant: the CLI/export row is narrowed to Preview pending credential-wording parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_input_method_and_credential_store_wording_registries_credential_preview_narrowed(
) -> M5InputCredentialRegistriesPacket {
    let mut packet = seeded_m5_input_method_and_credential_store_wording_registries();
    packet.packet_id =
        "m5-input-method-and-credential-store-wording-registries:credential-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5PlatformFitConsumerSurface::CliExport)
        .expect("cli-export row present");
    row.qualification = M5PlatformFitQualificationClass::Preview;
    packet
}
