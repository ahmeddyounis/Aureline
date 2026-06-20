//! Inline tests for the Doctor report localization posture packet.

use aureline_i18n::LocaleFallbackOriginClass;

use super::{
    seeded_doctor_report_localization_packet, DoctorMessageSurface, DoctorStableRefs,
    DOCTOR_REPORT_SUPPORT_EXPORT_RECORD_KIND,
};

#[test]
fn seeded_packet_validates() {
    seeded_doctor_report_localization_packet()
        .validate()
        .expect("packet validates");
}

#[test]
fn every_doctor_surface_is_covered() {
    let packet = seeded_doctor_report_localization_packet();
    let surfaces: std::collections::BTreeSet<DoctorMessageSurface> =
        packet.entries.iter().map(|entry| entry.surface).collect();
    for required in DoctorMessageSurface::ALL {
        assert!(surfaces.contains(&required), "missing surface {required:?}");
    }
}

#[test]
fn finding_codes_and_exit_classes_are_identical_across_locales() {
    let packet = seeded_doctor_report_localization_packet();
    let source = packet.render(&packet.source_language_locale);

    for locale in ["es-MX", "ja-JP", "ar-SA", "de-DE", "fr-FR"] {
        let render = packet.render(locale);
        assert_eq!(render.len(), source.len());
        for (rendered, base) in render.iter().zip(&source) {
            assert_eq!(rendered.message_id, base.message_id);
            // Finding codes, exit classes, evidence refs, and scope labels never
            // drift when prose localizes.
            assert_eq!(
                rendered.doctor_refs, base.doctor_refs,
                "{}",
                base.message_id
            );
        }
    }
}

#[test]
fn every_finding_bound_message_carries_a_finding_code() {
    let packet = seeded_doctor_report_localization_packet();
    for entry in &packet.entries {
        if entry.surface.is_finding_bound() {
            assert!(
                entry.doctor_refs.finding_code_ref.is_some(),
                "{} lacks a finding code",
                entry.message_id
            );
        }
    }
}

#[test]
fn parity_report_is_clean_for_every_locale() {
    let report = seeded_doctor_report_localization_packet().parity_report();
    assert!(report.parity_clean);
    for row in &report.rows {
        assert!(row.id_set_matches_source, "{}", row.requested_locale);
        assert!(row.finding_codes_preserved, "{}", row.requested_locale);
        assert!(row.exit_classes_preserved, "{}", row.requested_locale);
        assert!(row.evidence_refs_preserved, "{}", row.requested_locale);
        assert!(row.scope_labels_preserved, "{}", row.requested_locale);
    }
}

#[test]
fn fallback_state_is_inspectable_per_locale() {
    let packet = seeded_doctor_report_localization_packet();
    let total = packet.entries.len();

    assert_eq!(packet.missing_key_count("en-US"), 0);
    assert_eq!(packet.missing_key_count("es-MX"), 0);
    assert_eq!(packet.missing_key_count("de-DE"), total);
    let ja = packet.missing_key_count("ja-JP");
    assert!(ja > 0 && ja < total);

    let de = packet.locale_profile("de-DE").expect("de profile");
    assert_eq!(
        de.fallback_origin,
        LocaleFallbackOriginClass::PackMissingSourceLanguageOnly
    );
    assert!(de.source_language_route_active);
}

#[test]
fn support_export_exposes_locale_and_fallback_and_omits_raw_bodies() {
    let packet = seeded_doctor_report_localization_packet();
    let export = &packet.support_export;

    assert_eq!(export.record_kind, DOCTOR_REPORT_SUPPORT_EXPORT_RECORD_KIND);
    assert_eq!(export.requested_locale, "ja-JP");
    assert!(!export.raw_translated_bodies_exported);
    // Locale state and fallback are inspectable on the exported artifact.
    assert!(!export.fallback_chain.is_empty());
    assert!(export.missing_key_count > 0);
    assert!(!export.preserved_finding_codes.is_empty());
    assert!(export
        .preserved_finding_codes
        .contains(&"doctor.provider_auth.expired".to_owned()));

    for row in &export.rows {
        assert!(row.raw_translated_body_omitted);
        assert!(!row.source_language_key.is_empty());
    }
}

#[test]
fn support_export_for_a_missing_profile_falls_back_to_source() {
    let packet = seeded_doctor_report_localization_packet();
    let export = packet.build_support_export("zh-CN");
    assert_eq!(export.effective_locale, packet.source_language_locale);
    assert_eq!(export.missing_key_count, packet.entries.len());
    assert!(!export.raw_translated_bodies_exported);
    assert!(!export.preserved_finding_codes.is_empty());
}

#[test]
fn dropping_a_finding_code_breaks_validation() {
    let mut packet = seeded_doctor_report_localization_packet();
    let entry = packet
        .entries
        .iter_mut()
        .find(|entry| entry.message_id == "msg:doctor:provider-auth-expired:title")
        .expect("entry exists");
    entry.doctor_refs = DoctorStableRefs {
        probe_id_ref: Some("probe.provider_auth.v2".to_owned()),
        ..DoctorStableRefs::default()
    };

    let findings = packet.validate().expect_err("validation should fail");
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("finding code")));
}

#[test]
fn packet_serializes_to_export_safe_json() {
    let packet = seeded_doctor_report_localization_packet();
    let json = serde_json::to_string(&packet).expect("serializes");
    assert!(json.contains("doctor_report_localization_packet"));
    assert!(json.contains("preserved_finding_codes"));
    assert!(json.contains("raw_translated_bodies_exported"));
}
