//! Fixture replay for the localized-profile matrix and surface inventory.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_i18n::{
    seeded_localized_profile_matrix_packet, ClaimNarrowReason, ConsumerKind,
    LocalizableSurfaceFamily, LocalizedProfileMatrixPacket, MatrixGateState, ProfileClaimClass,
    SurfaceLocalizationState,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/i18n/m5-surface-inventory/manifest.json")
}

fn load_packet() -> LocalizedProfileMatrixPacket {
    let path = fixture_path();
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn manifest_fixture_matches_seeded_packet() {
    let from_file = load_packet();
    let from_code = seeded_localized_profile_matrix_packet();

    assert_eq!(from_file, from_code);
    from_file
        .validate()
        .expect("localized profile matrix packet validates");
}

#[test]
fn surface_inventory_freezes_the_named_families() {
    let packet = seeded_localized_profile_matrix_packet();
    let families = packet
        .surface_inventory
        .iter()
        .map(|row| row.surface_family)
        .collect::<BTreeSet<_>>();

    for required in [
        LocalizableSurfaceFamily::ShellChrome,
        LocalizableSurfaceFamily::CommandPalette,
        LocalizableSurfaceFamily::HelpAndDocs,
        LocalizableSurfaceFamily::CliAndDoctor,
        LocalizableSurfaceFamily::Notifications,
        LocalizableSurfaceFamily::ExtensionContributedUi,
        LocalizableSurfaceFamily::CompanionHandoff,
    ] {
        assert!(
            families.contains(&required),
            "inventory missing {required:?}"
        );
    }

    for row in &packet.surface_inventory {
        assert!(
            !row.stable_element_kinds.is_empty(),
            "{} lacks stable elements",
            row.surface_id
        );
        assert!(
            !row.owning_pack_ref.is_empty(),
            "{} lacks an owning pack",
            row.surface_id
        );
        assert!(
            !row.source_language_route_ref.is_empty(),
            "{} lacks a source-language route",
            row.surface_id
        );
    }
}

#[test]
fn matrix_answers_localized_fallback_and_not_localized() {
    let packet = seeded_localized_profile_matrix_packet();

    // Localized: the flagship profile renders the shell in the requested locale.
    assert_eq!(
        packet.effective_state("profile:es-MX:desktop", "surface:shell:chrome"),
        Some(SurfaceLocalizationState::Localized)
    );
    // Source-language fallback: a missing pack narrows the requested locale.
    assert_eq!(
        packet.effective_state("profile:ja-JP:desktop", "surface:shell:chrome"),
        Some(SurfaceLocalizationState::SourceLanguageFallbackOnly)
    );
    // Not localized: an explicitly non-localized profile makes no claim.
    assert_eq!(
        packet.effective_state("profile:fr-FR:not-localized", "surface:shell:chrome"),
        Some(SurfaceLocalizationState::NotLocalized)
    );
}

#[test]
fn claims_narrow_when_pack_or_evidence_is_missing() {
    let packet = seeded_localized_profile_matrix_packet();

    // Stale proof narrows the community profile despite a compatible pack.
    let pt = packet
        .localized_profiles
        .iter()
        .find(|p| p.profile_id == "profile:pt-BR:community")
        .expect("community profile exists");
    assert_eq!(pt.intended_claim_class, ProfileClaimClass::ClaimedLocalized);
    assert_eq!(
        pt.claim_class,
        ProfileClaimClass::SourceLanguageFallbackOnly
    );
    assert!(pt.narrowed);
    assert_eq!(pt.narrow_reason, Some(ClaimNarrowReason::EvidenceStale));

    // A missing pack narrows the Japanese profile.
    let ja = packet
        .localized_profiles
        .iter()
        .find(|p| p.profile_id == "profile:ja-JP:desktop")
        .expect("japanese profile exists");
    assert_eq!(
        ja.claim_class,
        ProfileClaimClass::SourceLanguageFallbackOnly
    );
    assert!(ja.narrowed);
    assert_eq!(ja.narrow_reason, Some(ClaimNarrowReason::PackMissing));

    // Every narrowed coverage cell cites a reason and reports the narrowed gate.
    for row in packet
        .profile_surface_coverage
        .iter()
        .filter(|r| r.narrowed)
    {
        assert_eq!(row.gate_state, MatrixGateState::Narrowed);
        assert!(
            row.narrow_reason.is_some(),
            "{} lacks a narrow reason",
            row.row_id
        );
        assert_eq!(
            row.effective_localization_state,
            SurfaceLocalizationState::SourceLanguageFallbackOnly
        );
    }
}

#[test]
fn claimed_localized_profiles_are_never_narrowed() {
    let packet = seeded_localized_profile_matrix_packet();

    let claimed = packet
        .localized_profiles
        .iter()
        .filter(|p| p.claim_class == ProfileClaimClass::ClaimedLocalized)
        .collect::<Vec<_>>();
    assert!(
        !claimed.is_empty(),
        "matrix claims at least one localized profile"
    );
    for profile in claimed {
        assert!(
            !profile.narrowed,
            "{} claims localized while narrowed",
            profile.profile_id
        );
        assert!(profile.intended_claim_class == ProfileClaimClass::ClaimedLocalized);
    }
}

#[test]
fn profiles_disclose_inspectable_fallback_chains() {
    let packet = seeded_localized_profile_matrix_packet();

    for profile in &packet.localized_profiles {
        assert_eq!(
            profile.fallback_chain.first(),
            Some(&profile.requested_locale)
        );
        assert_eq!(
            profile.fallback_chain.last(),
            Some(&profile.source_language_locale)
        );
        assert!(profile.visible_in_settings);
        assert!(profile.visible_in_diagnostics);
        assert!(profile.visible_in_support_export);
        assert!(profile.visible_in_help_about);
        assert!(profile.non_blocking_core_use);
    }
}

#[test]
fn downstream_consumers_are_bound_to_the_register() {
    let packet = seeded_localized_profile_matrix_packet();
    let consumers = packet
        .consumption_bindings
        .iter()
        .map(|binding| binding.consumer_kind)
        .collect::<BTreeSet<_>>();

    for required in [
        ConsumerKind::ReleaseCenter,
        ConsumerKind::HelpAbout,
        ConsumerKind::Diagnostics,
        ConsumerKind::ClaimNarrowing,
    ] {
        assert!(
            consumers.contains(&required),
            "missing consumer {required:?}"
        );
    }
}

#[test]
fn release_gates_are_green_and_proof_backed() {
    let packet = seeded_localized_profile_matrix_packet();

    assert_eq!(packet.summary.promotion_state, MatrixGateState::Green);
    assert_eq!(packet.summary.blocked_rows, 0);
    assert!(packet.summary.claimed_localized_profiles >= 1);
    assert!(packet.release_gate_rows.iter().all(|row| {
        row.required_for_claimed_profiles
            && row.gate_state == MatrixGateState::Green
            && row.command.contains("localized_profile_matrix")
            && !row.fixture_refs.is_empty()
            && !row.artifact_refs.is_empty()
    }));
}
