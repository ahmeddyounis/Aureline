//! Fixture replay for stable locale-pack lifecycle and translated-surface parity.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_i18n::{
    seeded_stable_locale_lifecycle_parity_packet, ClaimGateState, LocaleLifecycleState,
    LocalePackSignatureState, LocalizationClaimClass, StableLocaleLifecycleParityPacket,
    TranslatedSurfaceKind, TranslatedSurfaceParityResult, VersionMatchState,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity",
    )
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn manifest_fixture_matches_seeded_stable_packet() {
    let from_file: StableLocaleLifecycleParityPacket = load_json("manifest.json");
    let from_code = seeded_stable_locale_lifecycle_parity_packet();

    assert_eq!(from_file, from_code);
    from_file
        .validate()
        .expect("stable locale lifecycle packet validates");
}

#[test]
fn lifecycle_windows_cover_signed_first_party_mirrored_and_reviewed_community_packs() {
    let packet = seeded_stable_locale_lifecycle_parity_packet();
    let states = packet
        .lifecycle_windows
        .iter()
        .map(|window| window.lifecycle_state)
        .collect::<BTreeSet<_>>();

    assert!(states.contains(&LocaleLifecycleState::SourceLanguageBuiltInCurrent));
    assert!(states.contains(&LocaleLifecycleState::MirrorSignedCompatible));
    assert!(states.contains(&LocaleLifecycleState::ReviewedCommunitySignedCompatible));
    assert!(states.contains(&LocaleLifecycleState::MissingPackSourceLanguageOnly));

    let claimed_windows = packet
        .lifecycle_windows
        .iter()
        .filter(|window| window.backs_claimed_localized_row)
        .collect::<Vec<_>>();
    assert!(!claimed_windows.is_empty());
    assert!(claimed_windows.iter().all(|window| {
        window.signature_state == LocalePackSignatureState::SignedVerified
            && window.version_match_state == VersionMatchState::ExactBuildMatch
            && !window.mirror_receipt_refs.is_empty()
            && window.offline_bundle_ref.is_some()
    }));
}

#[test]
fn fallback_truth_is_visible_and_non_blocking_for_degraded_rows() {
    let packet = seeded_stable_locale_lifecycle_parity_packet();

    for row in &packet.fallback_truth_rows {
        assert_eq!(row.fallback_chain.first(), Some(&row.requested_locale));
        assert_eq!(row.fallback_chain.last(), Some(&row.source_language_locale));
        assert!(row.visible_in_settings);
        assert!(row.visible_in_diagnostics);
        assert!(row.visible_in_support_export);
        assert!(row
            .open_in_source_language_route_ref
            .contains("source-language"));
        assert!(row.non_blocking_core_use);
    }

    assert!(packet.fallback_truth_rows.iter().any(|row| {
        row.claim_class == LocalizationClaimClass::DegradedSourceLanguageOnly
            && row.requested_locale == "ja-JP"
            && row.effective_locale == "en-US"
    }));
    assert!(packet.fallback_truth_rows.iter().any(|row| {
        row.claim_class == LocalizationClaimClass::DegradedSourceLanguageOnly
            && row.requested_locale == "de-DE"
            && row.effective_locale == "en-US"
    }));
}

#[test]
fn stable_message_ids_are_bound_to_non_prose_identifiers() {
    let packet = seeded_stable_locale_lifecycle_parity_packet();

    for row in &packet.stable_message_id_rows {
        assert!(
            row.stable_refs.has_anchor(),
            "{} lacks stable refs",
            row.row_id
        );
        assert!(!row.translation_pack_refs.is_empty());
        assert!(row.command_id_stable);
        assert!(row.schema_or_diagnostic_id_stable);
        assert!(row.semantic_action_id_stable);
        assert!(row.machine_identifier_locale_neutral);
        assert!(!row.routed_by_localized_prose);
    }

    assert!(packet.stable_message_id_rows.iter().any(|row| {
        row.message_id == "msg:doctor:profile-schema-drift:human"
            && row.stable_refs.diagnostic_id_ref.as_deref()
                == Some("doctor.finding.profile.schema_drift")
    }));
}

#[test]
fn translated_docs_tour_auth_help_and_cli_preserve_source_truth() {
    let packet = seeded_stable_locale_lifecycle_parity_packet();
    let surface_kinds = packet
        .translated_surface_rows
        .iter()
        .map(|row| row.surface_kind)
        .collect::<BTreeSet<_>>();

    for required in [
        TranslatedSurfaceKind::Docs,
        TranslatedSurfaceKind::GuidedTour,
        TranslatedSurfaceKind::AuthRecovery,
        TranslatedSurfaceKind::HelpGlossaryCard,
        TranslatedSurfaceKind::CliHumanHelp,
    ] {
        assert!(surface_kinds.contains(&required), "missing {required:?}");
    }

    for row in &packet.translated_surface_rows {
        assert_ne!(row.parity_result, TranslatedSurfaceParityResult::Blocked);
        assert!(!row.message_id_refs.is_empty());
        assert!(!row.keyboard_path_refs.is_empty());
        assert!(!row.screen_reader_label_refs.is_empty());
        assert!(!row.recovery_route_refs.is_empty());
        assert!(!row.source_language_route_ref.is_empty());
        assert!(!row.accessibility_fixture_refs.is_empty());
    }
}

#[test]
fn cli_human_help_localizes_without_changing_machine_keys() {
    let packet = seeded_stable_locale_lifecycle_parity_packet();
    let cli = packet
        .translated_surface_rows
        .iter()
        .find(|row| row.surface_kind == TranslatedSurfaceKind::CliHumanHelp)
        .expect("CLI parity row exists");

    assert!(cli.cli_machine_keys_locale_neutral);
    assert!(cli.source_language_route_ref.contains("--locale-neutral"));
    assert!(cli
        .command_id_refs
        .iter()
        .any(|id| id == "cmd:doctor:profile"));
    assert!(cli
        .citation_anchor_refs
        .iter()
        .any(|anchor| anchor == "citation:cli:doctor#profile-schema-drift"));
}

#[test]
fn every_claimed_localized_row_is_green_and_release_gated() {
    let packet = seeded_stable_locale_lifecycle_parity_packet();

    assert_eq!(packet.summary.promotion_state, ClaimGateState::Green);
    assert_eq!(
        packet.summary.claimed_localized_rows,
        packet.summary.green_claimed_localized_rows
    );
    assert_eq!(packet.summary.blocked_rows, 0);
    assert!(packet.release_gate_rows.iter().all(|row| {
        row.required_for_claimed_localized_rows
            && row.gate_state == ClaimGateState::Green
            && row
                .command
                .contains("stable_locale_lifecycle_translated_surface_parity")
            && !row.fixture_refs.is_empty()
            && !row.artifact_refs.is_empty()
    }));
}
