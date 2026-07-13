//! Canonical seed builders for the frozen M5 platform-fit matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical platform-fit matrix.
pub const M5_PLATFORM_FIT_MATRIX_PACKET_ID: &str = "m5-platform-fit:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every family must be able to show.
fn mandatory_labels() -> Vec<M5PlatformFitRequiredLabel> {
    M5PlatformFitRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a family carries.
fn labels_with(extra: &[M5PlatformFitRequiredLabel]) -> Vec<M5PlatformFitRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every family filled in and every family-specific vocabulary left
/// empty for the caller to populate.
fn base_row(
    platform_fit_family: M5PlatformFitFamily,
    qualification: M5PlatformFitQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5PlatformFitRow {
    M5PlatformFitRow {
        platform_fit_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5PlatformFitSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5PlatformFitDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        platform_convention_roles: vec![],
        shortcut_notation_roles: vec![],
        file_path_reveal_roles: vec![],
        theme_contrast_live_change_roles: vec![],
        credential_store_wording_roles: vec![],
        input_method_roles: vec![],
        degraded_reasons: M5PlatformFitDegradedReason::ALL.to_vec(),
        accessibility_routes: M5PlatformFitAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5PlatformFitConsumerSurface::SupportExport,
            M5PlatformFitConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5PlatformFitDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        platform_wording_changes_command_or_permission_meaning: false,
        hides_primary_action_only_in_os_chrome: false,
        falls_back_to_plaintext_secret_storage_silently: false,
        input_method_corrupts_text_or_trust_fidelity: false,
        screenshot_or_docs_mislabels_shortcut_or_path_verb: false,
    }
}

fn platform_fit_rows() -> Vec<M5PlatformFitRow> {
    use M5PlatformFitConsumerSurface as C;
    use M5PlatformFitDowngradeTrigger as D;
    use M5PlatformFitFamily as F;
    use M5PlatformFitQualificationClass as Q;
    use M5PlatformFitRequiredLabel as L;
    use M5PlatformFitRole as R;

    let mut rows = Vec::new();

    // 1. Platform conventions.
    let mut row = base_row(
        F::PlatformConvention,
        Q::Stable,
        "Native desktop integration owner",
        "One platform-convention table naming window-control placement, menu-bar behavior, title-bar convention, and system-chrome integration for macOS, Windows, and Linux so high-frequency actions are never hidden in OS chrome alone and command IDs stay stable while platform labels adapt",
        "evidence:m5-platform-convention-parity:001",
        &[
            M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
            M5_SHORTCUT_NOTATION_SCHEMA_REF,
            M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
        ],
    );
    row.platform_convention_roles = M5PlatformConventionRole::ALL.to_vec();
    row.semantic_roles = vec![R::WindowMenu, R::CommandStability];
    row.required_labels = labels_with(&[L::HostPlatform]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::SettingsUi,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PrimaryActionHiddenOnlyInOsChrome,
        D::PlatformWordingChangedCommandOrPermissionMeaning,
        D::HostPlatformUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Shortcut notation.
    let mut row = base_row(
        F::ShortcutNotation,
        Q::Stable,
        "Keyboard and command owner",
        "One shortcut-notation contract naming the modifier glyphs, accelerator labels, and chord sequences so notation adapts per platform (⌘/⌥/⌃/⇧ on macOS, Ctrl/Alt/Shift elsewhere) while the underlying command ID stays stable and is never hard-coded for one platform",
        "evidence:m5-shortcut-notation-parity:001",
        &[
            M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
            M5_SHORTCUT_NOTATION_SCHEMA_REF,
            M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
        ],
    );
    row.shortcut_notation_roles = M5ShortcutNotationRole::ALL.to_vec();
    row.semantic_roles = vec![R::Shortcut, R::CommandStability];
    row.required_labels = labels_with(&[L::ShortcutNotation]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::SettingsUi,
        C::DocsHelp,
        C::Onboarding,
        C::CliExport,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ShortcutNotationDriftedByPlatform,
        D::ScreenshotOrDocsMislabeledShortcutOrPathVerb,
        D::ShortcutNotationUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. File / path / reveal terminology.
    let mut row = base_row(
        F::FilePathReveal,
        Q::Stable,
        "File and path terminology owner",
        "One file-path-reveal contract naming the reveal verb (Reveal in Finder / Show in Explorer / Open Containing Folder), the save-dialog terminology, and host-matched separators and case so file, path, reveal, and save wording matches the host platform and is never mislabeled in screenshots or help",
        "evidence:m5-file-path-reveal-parity:001",
        &[
            M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
            M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
            M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
        ],
    );
    row.file_path_reveal_roles = M5FilePathRevealRole::ALL.to_vec();
    row.semantic_roles = vec![R::PathTerminology];
    row.required_labels = labels_with(&[L::PathVerb]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::SettingsUi,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ScreenshotOrDocsMislabeledShortcutOrPathVerb,
        D::PathVerbUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Theme / contrast live change.
    let mut row = base_row(
        F::ThemeContrastLiveChange,
        Q::Stable,
        "Appearance-session owner",
        "One theme-contrast-live-change contract naming the live theme, contrast, accent, and text-scale response so system appearance changes apply live or explain their fallback rather than silently drifting, and so appearance survives zoom and high-contrast modes",
        "evidence:m5-theme-contrast-live-parity:001",
        &[
            M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
            M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
            M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
        ],
    );
    row.theme_contrast_live_change_roles = M5ThemeContrastLiveChangeRole::ALL.to_vec();
    row.semantic_roles = vec![R::Appearance];
    row.required_labels = labels_with(&[L::HostPlatform]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::SettingsUi,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback,
        D::HostPlatformUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Credential-store wording.
    let mut row = base_row(
        F::CredentialStoreWording,
        Q::Stable,
        "Credential-state owner",
        "One credential-store-wording contract naming the host store (Keychain / Credential Manager / Secret Service), the truthful storage claim, non-leaky wording, and any disclosed fallback so credential messaging never claims stronger protection than it has and never silently falls back to plaintext storage",
        "evidence:m5-credential-store-wording-parity:001",
        &[
            M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
            M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
            M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
        ],
    );
    row.credential_store_wording_roles = M5CredentialStoreWordingRole::ALL.to_vec();
    row.semantic_roles = vec![R::CredentialWording];
    row.required_labels = labels_with(&[L::HostPlatform]);
    row.consumer_surfaces = vec![
        C::AuthUi,
        C::SettingsUi,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SecretStorageFellBackToPlaintextSilently,
        D::PlatformWordingChangedCommandOrPermissionMeaning,
        D::HostPlatformUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Input-method behavior.
    let mut row = base_row(
        F::InputMethod,
        Q::Stable,
        "Input-handling owner",
        "One input-method contract naming IME composition, dead keys and AltGr, dictation and emoji, and layout switching so text and trust fidelity are preserved under every input method and layout, and so no input path corrupts committed text or trust semantics",
        "evidence:m5-input-method-parity:001",
        &[
            M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
            M5_INPUT_METHOD_BEHAVIOR_SCHEMA_REF,
            M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
        ],
    );
    row.input_method_roles = M5InputMethodRole::ALL.to_vec();
    row.semantic_roles = vec![R::InputFidelity];
    row.required_labels = labels_with(&[L::HostPlatform]);
    row.consumer_surfaces = vec![
        C::InputUi,
        C::ShellUi,
        C::SettingsUi,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::InputMethodCorruptedTextOrTrust,
        D::HostPlatformUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5PlatformFitGovernanceReview {
    M5PlatformFitGovernanceReview {
        command_ids_stable_while_labels_adapt: true,
        high_frequency_actions_never_hidden_in_os_chrome_alone: true,
        file_path_reveal_save_terminology_matches_host: true,
        theme_contrast_accent_text_scale_apply_live_or_explain_fallback: true,
        credential_store_wording_stays_truthful_and_non_leaky: true,
        input_method_never_corrupts_text_or_trust_fidelity: true,
        shortcut_notation_adapts_per_platform: true,
        no_primary_action_hidden_only_in_os_chrome: true,
        secrets_never_fall_back_to_plaintext_silently: true,
        every_family_declares_deployment_lines: true,
        every_family_declares_accessibility_route: true,
        support_export_reads_single_platform_fit_source: true,
        screenshots_and_docs_bind_to_single_platform_fit_source: true,
        later_rows_cannot_invent_parallel_platform_vocabulary: true,
        platform_fit_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_registry_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5PlatformFitConsumerProjection {
    M5PlatformFitConsumerProjection {
        shell_and_settings_consume_shared_shortcut_and_menu_grammar: true,
        auth_and_settings_consume_shared_credential_wording: true,
        input_surfaces_consume_shared_input_method_behavior: true,
        docs_help_and_screenshots_read_single_platform_fit_source: true,
        appearance_consumers_bind_to_shared_theme_response: true,
        support_export_reads_single_platform_fit_source: true,
    }
}

fn proof_freshness() -> M5PlatformFitProofFreshness {
    M5PlatformFitProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5PlatformFitReleasePosture {
    M5PlatformFitReleasePosture {
        proof_packet_ref: M5_PLATFORM_FIT_ARTIFACT_REF.to_owned(),
        platform_fit_audit_ref: M5_PLATFORM_FIT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
        M5_PLATFORM_FIT_MATRIX_DOC_REF,
        M5_SHORTCUT_NOTATION_SCHEMA_REF,
        M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
        M5_INPUT_METHOD_BEHAVIOR_SCHEMA_REF,
        M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 platform-fit matrix packet.
pub fn seeded_m5_platform_fit_matrix() -> M5PlatformFitMatrixPacket {
    M5PlatformFitMatrixPacket::new(M5PlatformFitMatrixPacketInput {
        packet_id: M5_PLATFORM_FIT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 platform-convention, shortcut-notation, file-path-reveal, theme/contrast live-change, credential-store wording, and input-method platform-fit matrix"
                .to_owned(),
        platform_fit_rows: platform_fit_rows(),
        vocabulary_set: M5PlatformFitVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: theme / contrast live change is held at Beta because live appearance response is not
/// yet proven across every deployment line; every family stays visible.
pub fn seeded_m5_platform_fit_matrix_theme_contrast_live_change_beta_narrowed(
) -> M5PlatformFitMatrixPacket {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.packet_id = "m5-platform-fit:theme-contrast-live-change-beta:0001".to_owned();
    let row = packet
        .platform_fit_rows
        .iter_mut()
        .find(|row| row.platform_fit_family == M5PlatformFitFamily::ThemeContrastLiveChange)
        .expect("theme-contrast-live-change row present");
    row.qualification = M5PlatformFitQualificationClass::Beta;
    packet
}

/// Narrowed variant: input-method behavior is narrowed to Preview pending IME / dead-key / dictation
/// coverage across every deployment line; every family stays visible.
pub fn seeded_m5_platform_fit_matrix_input_method_preview_narrowed() -> M5PlatformFitMatrixPacket {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.packet_id = "m5-platform-fit:input-method-preview:0001".to_owned();
    let row = packet
        .platform_fit_rows
        .iter_mut()
        .find(|row| row.platform_fit_family == M5PlatformFitFamily::InputMethod)
        .expect("input-method row present");
    row.qualification = M5PlatformFitQualificationClass::Preview;
    packet
}
