//! Fixture replay and contract proofs for the locale diagnostics packet.

use std::path::{Path, PathBuf};

use aureline_shell::i18n::{
    seeded_locale_diagnostics_packet, LocaleClaimGateState, LocaleDiagnosticsPacket,
    LocaleDiagnosticsSupportExport, LocaleProblemOrigin,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_json<T: serde::de::DeserializeOwned>(rel: &str) -> T {
    let path = repo_root().join(rel);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn packet_fixture_matches_seeded_packet() {
    let from_file: LocaleDiagnosticsPacket =
        load_json("fixtures/i18n/locale-diagnostics-exports/locale-diagnostics-packet.json");
    let from_code = seeded_locale_diagnostics_packet();

    assert_eq!(from_file, from_code);
    from_file.validate().expect("packet validates");
}

#[test]
fn support_export_fixture_matches_derived_projection() {
    let from_file: LocaleDiagnosticsSupportExport = load_json(
        "fixtures/i18n/locale-diagnostics-exports/locale-diagnostics-support-export.json",
    );
    let from_code = seeded_locale_diagnostics_packet().support_export;
    assert_eq!(from_file, from_code);
}

#[test]
fn published_schema_exists_and_is_draft_2020_12() {
    let schema_path = repo_root().join("schemas/i18n/locale-diagnostics.schema.json");
    let body = std::fs::read_to_string(&schema_path)
        .unwrap_or_else(|err| panic!("missing {}: {err}", schema_path.display()));
    let parsed: serde_json::Value = serde_json::from_str(&body).expect("schema parses");
    assert_eq!(
        parsed["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(
        parsed["properties"]["record_kind"]["const"],
        "locale_diagnostics_packet"
    );
}

#[test]
fn support_export_tells_support_the_problem_origin_without_private_content() {
    let export = seeded_locale_diagnostics_packet().support_export;

    // Metadata-only: no translated bodies, signing keys, or provider payloads.
    assert!(!export.raw_translated_bodies_exported);
    assert!(export
        .installed_pack_rows
        .iter()
        .all(|row| row.raw_translated_body_omitted));
    assert!(export
        .profile_rows
        .iter()
        .all(|row| row.raw_translated_body_omitted));
    assert!(export
        .omitted_material_classes
        .contains(&"locale_pack_signing_keys".to_owned()));
    assert!(export
        .omitted_material_classes
        .contains(&"raw_provider_payloads".to_owned()));

    // Source-language anchors and stable ids are preserved for escalation.
    assert!(export
        .preserved_stable_anchor_refs
        .contains(&export.source_language_locale));

    // Each of the five problem-origin buckets is representable, and the seeded
    // spread distinguishes pack skew from a source-language fallback.
    let origins: std::collections::BTreeSet<LocaleProblemOrigin> = export
        .profile_rows
        .iter()
        .map(|row| row.problem_origin)
        .collect();
    assert!(origins.contains(&LocaleProblemOrigin::PackSkew));
    assert!(origins.contains(&LocaleProblemOrigin::SourceLanguageFallback));
    assert!(origins.contains(&LocaleProblemOrigin::RequestedLocale));
}

#[test]
fn release_gate_narrows_incompatible_or_degraded_localization_claims() {
    let gate = seeded_locale_diagnostics_packet().release_gate;
    assert!(gate.any_claim_narrowed);
    assert!(gate.any_claim_blocked);

    // No incompatible or source-language claim may remain publishable.
    for row in &gate.rows {
        if matches!(
            row.gate_state,
            LocaleClaimGateState::ClaimBlockedIncompatiblePack
                | LocaleClaimGateState::ClaimNarrowedSourceLanguage
        ) {
            assert!(row.narrowed);
            assert!(!row.publishable_localized_claim);
        }
    }
}
