//! Inline tests for the CLI/help localization posture packet.

use aureline_i18n::{LocaleFallbackOriginClass, MachineOutputLocaleClass};

use super::{
    seeded_cli_localization_packet, CliMessageSurface, CLI_LOCALE_SUPPORT_EXPORT_RECORD_KIND,
};

#[test]
fn seeded_packet_validates() {
    seeded_cli_localization_packet()
        .validate()
        .expect("packet validates");
}

#[test]
fn every_cli_surface_is_covered() {
    let packet = seeded_cli_localization_packet();
    let surfaces: std::collections::BTreeSet<CliMessageSurface> =
        packet.entries.iter().map(|entry| entry.surface).collect();
    for required in CliMessageSurface::ALL {
        assert!(surfaces.contains(&required), "missing surface {required:?}");
    }
}

#[test]
fn render_ids_and_anchors_are_identical_across_locales() {
    let packet = seeded_cli_localization_packet();
    let source = packet.render(&packet.source_language_locale);

    for locale in ["es-MX", "ja-JP", "ar-SA", "de-DE", "fr-FR"] {
        let render = packet.render(locale);
        assert_eq!(render.len(), source.len());
        for (rendered, base) in render.iter().zip(&source) {
            assert_eq!(rendered.message_id, base.message_id);
            assert_eq!(rendered.source_language_key, base.source_language_key);
            // Flags, JSON keys, exit classes, and subcommand paths never drift.
            assert_eq!(rendered.cli_refs, base.cli_refs, "{}", base.message_id);
        }
    }
}

#[test]
fn flags_subcommands_and_json_keys_are_never_localized() {
    let contract = seeded_cli_localization_packet().machine_output_contract;
    assert!(!contract.json_keys_localized);
    assert!(!contract.flags_localized);
    assert!(!contract.subcommand_names_localized);
    assert_eq!(contract.locale_neutral_output_flag, "--locale-neutral");
    // Only one optional human field may carry translated prose.
    assert_eq!(
        contract.optional_translated_human_field.as_deref(),
        Some("message")
    );
}

#[test]
fn json_human_field_is_the_only_machine_output_translatable_message() {
    let packet = seeded_cli_localization_packet();
    for entry in &packet.entries {
        if entry.surface == CliMessageSurface::JsonHumanField {
            assert_eq!(
                entry.machine_output_locale_class,
                MachineOutputLocaleClass::LocaleNeutralWithTranslatedHumanField
            );
            assert!(entry
                .cli_refs
                .json_output_key_refs
                .contains(&"message".to_owned()));
        }
    }
}

#[test]
fn parity_report_is_clean_for_every_locale() {
    let report = seeded_cli_localization_packet().parity_report();
    assert!(report.parity_clean);
    for row in &report.rows {
        assert!(row.id_set_matches_source, "{}", row.requested_locale);
        assert!(row.flag_tokens_preserved, "{}", row.requested_locale);
        assert!(row.json_keys_preserved, "{}", row.requested_locale);
        assert!(row.exit_classes_preserved, "{}", row.requested_locale);
        assert!(row.subcommand_paths_preserved, "{}", row.requested_locale);
    }
}

#[test]
fn fallback_state_is_inspectable_per_locale() {
    let packet = seeded_cli_localization_packet();
    let total = packet.entries.len();

    assert_eq!(packet.missing_key_count("en-US"), 0);
    assert_eq!(packet.missing_key_count("es-MX"), 0);
    assert_eq!(packet.missing_key_count("de-DE"), total);
    let ja = packet.missing_key_count("ja-JP");
    assert!(ja > 0 && ja < total);

    let de = packet.locale_profile("de-DE").expect("de profile");
    assert_eq!(
        de.fallback_origin,
        LocaleFallbackOriginClass::PackSignatureFailedSourceLanguageOnly
    );
    assert!(de.source_language_route_active);
}

#[test]
fn support_export_preserves_anchors_and_omits_raw_bodies() {
    let packet = seeded_cli_localization_packet();
    let export = &packet.support_export;

    assert_eq!(export.record_kind, CLI_LOCALE_SUPPORT_EXPORT_RECORD_KIND);
    assert_eq!(export.requested_locale, "ja-JP");
    assert!(!export.raw_translated_bodies_exported);
    assert!(export.missing_key_count > 0, "fallback should be visible");
    assert!(!export.fallback_chain.is_empty());

    // Every export row keeps the stable anchors and drops raw translated text.
    for row in &export.rows {
        assert!(row.raw_translated_body_omitted);
        assert!(!row.source_language_key.is_empty());
    }

    // The format flag and a known exit class survive export for escalation.
    let unknown = export
        .rows
        .iter()
        .find(|row| row.message_id == "msg:cli:error:unknown-subcommand")
        .expect("error row");
    assert!(unknown
        .stable_anchor_refs
        .contains(&"cli.exit.usage_error".to_owned()));
}

#[test]
fn support_export_for_a_missing_profile_falls_back_to_source() {
    let packet = seeded_cli_localization_packet();
    // A locale with no profile still produces an inspectable, source-language
    // export rather than a panic or empty record.
    let export = packet.build_support_export("zh-CN");
    assert_eq!(export.effective_locale, packet.source_language_locale);
    assert_eq!(export.missing_key_count, packet.entries.len());
    assert!(!export.raw_translated_bodies_exported);
}

#[test]
fn dropping_a_baseline_anchor_breaks_validation() {
    let mut packet = seeded_cli_localization_packet();
    // Corrupt one entry's anchors so it routes only by prose.
    let entry = packet
        .entries
        .iter_mut()
        .find(|entry| entry.message_id == "msg:cli:flag:format")
        .expect("entry exists");
    entry.cli_refs = super::CliStableRefs::default();

    let findings = packet.validate().expect_err("validation should fail");
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("locale-neutral anchor")));
}

#[test]
fn packet_serializes_to_export_safe_json() {
    let packet = seeded_cli_localization_packet();
    let json = serde_json::to_string(&packet).expect("serializes");
    assert!(json.contains("cli_help_localization_packet"));
    assert!(json.contains("locale_neutral_output_flag"));
    assert!(json.contains("raw_translated_bodies_exported"));
}
