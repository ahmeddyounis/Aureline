//! Fixture replay for the beta locale-pack contract.

use std::path::{Path, PathBuf};

use aureline_i18n::{
    seeded_locale_pack_beta_contract, seeded_locale_pack_help_about_projection,
    seeded_locale_pack_settings_projection, seeded_locale_pack_support_export,
    seeded_locale_pack_support_projection, CommandIdPreservationState, LocaleFallbackOriginClass,
    LocalePackBetaContract, LocalePackSignatureState, LocalePackSupportExport,
    LocalePackSurfaceProjection, LocaleProjectionSurface, MachineOutputLocaleClass,
    MessageSurfaceFamily,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/i18n/m3/locale_fallback")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn manifest_fixture_matches_seeded_contract() {
    let from_file: LocalePackBetaContract = load_json("manifest.json");
    let from_code = seeded_locale_pack_beta_contract();

    assert_eq!(from_file, from_code);
    from_file
        .validate()
        .expect("fixture manifest validates against beta contract");
}

#[test]
fn settings_projection_exposes_active_fallback_chain_and_pack_signatures() {
    let projection: LocalePackSurfaceProjection = load_json("settings_projection.json");

    assert_eq!(projection, seeded_locale_pack_settings_projection());
    assert_eq!(
        projection.projection_surface,
        LocaleProjectionSurface::Settings
    );
    assert!(projection.rows.iter().any(|row| {
        row.row_kind == "active_locale_state"
            && row.requested_locale == "pt-BR"
            && row.fallback_chain == ["pt-BR", "pt", "en-US"]
    }));
    assert!(projection.rows.iter().any(|row| {
        row.row_kind == "locale_pack_version_signature"
            && row.pack_id_ref.as_deref() == Some("locale-pack:community:pt-br:beta")
            && row.signature_state == Some(LocalePackSignatureState::SignedVerified)
    }));
}

#[test]
fn help_about_projection_uses_same_active_state_as_settings() {
    let settings: LocalePackSurfaceProjection = load_json("settings_projection.json");
    let help_about: LocalePackSurfaceProjection = load_json("help_about_projection.json");

    assert_eq!(help_about, seeded_locale_pack_help_about_projection());
    assert_eq!(
        settings.active_locale_state, help_about.active_locale_state,
        "settings and Help/About must quote the same active locale state"
    );
    assert!(help_about.rows.iter().any(|row| {
        row.fallback_state_ref.as_deref()
            == Some("locale-fallback:docs:glossary:pt-br:source-language")
            && row.source_language_escape_hatches.contains(
                &aureline_i18n::SourceLanguageEscapeHatchClass::DocsPaneSourceLanguageRoute,
            )
    }));
}

#[test]
fn support_export_is_metadata_only_and_quotes_compatibility_results() {
    let export: LocalePackSupportExport = load_json("support_export.json");
    let contract = seeded_locale_pack_beta_contract();

    assert_eq!(export, seeded_locale_pack_support_export());
    export
        .validate_against_contract(&contract)
        .expect("support export validates against contract");
    assert!(!export.raw_translated_bodies_exported);
    assert!(export
        .omitted_material_classes
        .iter()
        .any(|class| class == "raw_translated_message_body"));
    assert!(export.compatibility_results.iter().any(|result| {
        result.pack_ref == "locale-pack:community:pt-br:beta"
            && result.waiver_ref.as_deref()
                == Some("waiver:locale-pack:community:pt-br:glossary-partial:2026.05.25")
    }));
}

#[test]
fn support_projection_matches_seeded_projection() {
    let projection: LocalePackSurfaceProjection = load_json("support_projection.json");

    assert_eq!(projection, seeded_locale_pack_support_projection());
    assert_eq!(
        projection.projection_surface,
        LocaleProjectionSurface::SupportExport
    );
    assert!(projection
        .rows
        .iter()
        .all(|row| row.raw_translated_body_omitted));
}

#[test]
fn message_bindings_keep_machine_identifiers_locale_neutral() {
    let contract = seeded_locale_pack_beta_contract();

    for message in &contract.message_bindings {
        assert!(
            message.stable_refs.has_anchor(),
            "{} must have a stable non-prose anchor",
            message.message_id
        );
        assert!(
            message.machine_identifier_fields_locale_neutral,
            "{} must keep machine identifiers neutral",
            message.message_id
        );
        assert!(
            !message.routed_by_localized_prose,
            "{} must not route by localized prose",
            message.message_id
        );
    }

    let doctor = contract
        .message("msg:doctor:profile-schema-drift:human")
        .expect("doctor message exists");
    assert_eq!(
        doctor.machine_output_locale_class,
        MachineOutputLocaleClass::LocaleNeutralWithTranslatedHumanField
    );
    assert_eq!(
        doctor.stable_refs.diagnostic_id_ref.as_deref(),
        Some("doctor.finding.profile.schema_drift")
    );
}

#[test]
fn fallback_states_disclose_source_language_and_preserve_command_identity() {
    let contract = seeded_locale_pack_beta_contract();
    let docs_fallback = contract
        .fallback_state("locale-fallback:docs:glossary:pt-br:source-language")
        .expect("docs fallback exists");

    assert_eq!(
        docs_fallback.fallback_origin_class,
        LocaleFallbackOriginClass::SourceLanguageFallback
    );
    assert!(docs_fallback.disclosed_to_reviewer);
    assert_eq!(
        docs_fallback.command_id_preservation_state,
        CommandIdPreservationState::CommandIdUnchangedAcrossFallback
    );
    assert_eq!(
        docs_fallback.command_id_ref.as_deref(),
        Some("cmd:core:open_folder")
    );

    let extension_failure = contract
        .fallback_state("locale-fallback:extension:docs-helper:de-de:signature-failed")
        .expect("extension fallback exists");
    assert_eq!(
        extension_failure.fallback_origin_class,
        LocaleFallbackOriginClass::PackSignatureFailedSourceLanguageOnly
    );
    assert_eq!(
        extension_failure.denial_reason_on_deny.as_deref(),
        Some("localization_locale_pack_signature_failed")
    );
    assert!(extension_failure.packs_consulted.iter().any(|pack| {
        pack.signature_state == LocalePackSignatureState::SignatureFailedBlocked
            && !pack.produced_message
    }));
}

#[test]
fn extension_declarations_cannot_override_host_ids() {
    let contract = seeded_locale_pack_beta_contract();

    assert!(contract
        .extension_locale_declarations
        .iter()
        .all(|declaration| !declaration.may_override_host_stable_ids));
    assert!(contract
        .message_bindings
        .iter()
        .filter(|message| message.surface_family == MessageSurfaceFamily::ExtensionContributedUi)
        .all(|message| message.extension_namespace_ref.is_some()));
}
