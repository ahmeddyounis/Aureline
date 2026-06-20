//! Fixture replay and continuity proofs for the M5 message-id registry.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_i18n::{
    seeded_m5_message_id_baseline, seeded_m5_message_registry, M5MessageRegistry, M5MessageSurface,
    MessageIdBaselineSnapshot, MessageIdContinuityState,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/i18n/message-id-stability")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn registry_fixture_matches_seeded_packet() {
    let from_file: M5MessageRegistry = load_json("registry.json");
    let from_code = seeded_m5_message_registry();

    assert_eq!(from_file, from_code);
    from_file.validate().expect("registry validates");
}

#[test]
fn baseline_fixture_matches_seeded_snapshot() {
    let from_file: MessageIdBaselineSnapshot = load_json("baseline-ids.json");
    let from_code = seeded_m5_message_id_baseline();

    assert_eq!(from_file, from_code);
    from_file.validate().expect("baseline validates");
}

#[test]
fn every_named_m5_surface_is_registered() {
    let registry = seeded_m5_message_registry();
    let surfaces: BTreeSet<M5MessageSurface> =
        registry.entries.iter().map(|entry| entry.surface).collect();
    for required in M5MessageSurface::ALL {
        assert!(
            surfaces.contains(&required),
            "registry is missing surface {required:?}"
        );
    }
}

#[test]
fn every_entry_is_bound_to_a_stable_non_prose_anchor() {
    let registry = seeded_m5_message_registry();
    for entry in &registry.entries {
        assert!(
            entry.stable_refs.has_anchor(),
            "{} has no stable anchor",
            entry.message_id
        );
        // Guardrail: behavior never routes by localized prose.
        assert!(
            !entry.routed_by_localized_prose,
            "{} routes by localized prose",
            entry.message_id
        );
        assert!(
            entry.machine_identifier_fields_locale_neutral,
            "{} has non-neutral machine ids",
            entry.message_id
        );
        // Source-language keys are preserved and locale-neutral.
        assert!(!entry.source_language_key.is_empty());
        assert_eq!(
            entry.source_language_locale,
            registry.source_language_locale
        );
    }
}

#[test]
fn message_ids_are_continuous_across_locale_changes() {
    let registry = seeded_m5_message_registry();

    // Rendering in any locale yields the same id sequence; only the effective
    // locale and the source-language fallback flag may change.
    let baseline_ids: Vec<String> = registry
        .render("en-US")
        .into_iter()
        .map(|rendered| rendered.message_id)
        .collect();

    for locale in ["es-MX", "ja-JP", "ar-SA", "de-DE", "fr-FR"] {
        let ids: Vec<String> = registry
            .render(locale)
            .into_iter()
            .map(|rendered| rendered.message_id)
            .collect();
        assert_eq!(
            ids, baseline_ids,
            "message ids changed under locale {locale}"
        );
    }

    // Source-language keys are likewise locale-independent.
    let keys_en: Vec<String> = registry
        .render("en-US")
        .into_iter()
        .map(|rendered| rendered.source_language_key)
        .collect();
    let keys_ja: Vec<String> = registry
        .render("ja-JP")
        .into_iter()
        .map(|rendered| rendered.source_language_key)
        .collect();
    assert_eq!(keys_en, keys_ja);
}

#[test]
fn message_ids_are_continuous_across_release_builds() {
    let registry = seeded_m5_message_registry();
    let baseline = seeded_m5_message_id_baseline();
    let report = registry.continuity_against(&baseline);

    // No baseline id was dropped and no source-language key drifted.
    assert!(report.is_stable(), "continuity broke: {report:?}");
    assert_eq!(report.removed_count, 0);
    assert_eq!(report.key_drift_count, 0);
    assert_eq!(report.preserved_count, baseline.ids.len());
    assert!(report.added_count >= 1, "expected at least one new id");

    // Every baseline id is preserved with its original key.
    for row in &report.rows {
        assert_ne!(
            row.state,
            MessageIdContinuityState::RemovedWithoutGovernance
        );
        assert_ne!(row.state, MessageIdContinuityState::KeyDrift);
    }
}

#[test]
fn dropping_a_baseline_id_breaks_continuity() {
    let registry = seeded_m5_message_registry();
    let mut baseline = seeded_m5_message_id_baseline();
    // Inject a baseline id the current registry does not carry, simulating a
    // renamed-or-removed message between releases.
    baseline.ids.push(aureline_i18n::MessageIdBaselineRow {
        message_id: "msg:shell:title-bar:retired-label".to_owned(),
        source_language_key: "shell.title_bar.retired_label".to_owned(),
        surface: M5MessageSurface::ShellChrome,
    });

    let report = registry.continuity_against(&baseline);
    assert!(!report.is_stable());
    assert_eq!(report.removed_count, 1);
}

#[test]
fn missing_key_counts_track_locale_coverage() {
    let registry = seeded_m5_message_registry();

    assert_eq!(registry.missing_key_count("en-US"), 0);
    assert_eq!(registry.missing_key_count("es-MX"), 0);
    // de-DE has no translated keys, so every key falls back to source.
    assert_eq!(registry.missing_key_count("de-DE"), registry.entries.len());
    // ja-JP is partial: some keys missing, but not all.
    let ja_missing = registry.missing_key_count("ja-JP");
    assert!(ja_missing > 0 && ja_missing < registry.entries.len());

    // Per-surface counts sum to the total.
    let summed: usize = M5MessageSurface::ALL
        .into_iter()
        .map(|surface| registry.missing_key_count_for_surface("ja-JP", surface))
        .sum();
    assert_eq!(summed, ja_missing);
}
