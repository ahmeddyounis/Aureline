//! Inline tests for the M5 runbook source register.

use super::*;

fn canonical() -> M5RunbookSourceRegister {
    seeded_m5_runbook_source_register()
}

#[test]
fn canonical_register_validates() {
    let register = canonical();
    assert!(register.validate().is_empty(), "{:?}", register.validate());
    assert_eq!(register.register_id, M5_RUNBOOK_SOURCE_REGISTER_ID);
    assert_eq!(register.record_kind, M5_RUNBOOK_SOURCE_REGISTER_RECORD_KIND);
}

#[test]
fn every_provenance_class_is_represented() {
    let register = canonical();
    let classes: std::collections::BTreeSet<RunbookSourceProvenance> = register
        .sources
        .iter()
        .map(|s| s.provenance_class)
        .collect();
    for class in RunbookSourceProvenance::ALL {
        assert!(classes.contains(&class), "class {} absent", class.as_str());
    }
}

#[test]
fn each_class_maps_to_its_authority_posture() {
    let register = canonical();
    let posture = |id: &str| register.source(id).unwrap().effective_authority_posture;
    assert_eq!(
        posture("src:repo-pipeline-restart"),
        RunbookAuthorityPosture::Authoritative
    );
    assert_eq!(
        posture("src:mirror-observability-pack"),
        RunbookAuthorityPosture::Mirrored
    );
    assert_eq!(
        posture("src:catalog-failover"),
        RunbookAuthorityPosture::Managed
    );
    assert_eq!(
        posture("src:browser-vendor-scaling"),
        RunbookAuthorityPosture::ReferenceOnly
    );
}

#[test]
fn unpromoted_browser_reference_is_reference_only_and_not_executable() {
    let register = canonical();
    let src = register.source("src:browser-vendor-scaling").unwrap();
    assert!(src.is_reference_only());
    assert!(!src.is_executable());
    let badge = src.badge();
    assert!(badge.reference_only);
    assert!(!badge.executable);
    assert_eq!(badge.authority_posture, "reference_only");
}

#[test]
fn promoted_browser_reference_rises_to_authoritative() {
    let register = canonical();
    let src = register.source("src:browser-promoted-dr").unwrap();
    assert_eq!(
        src.effective_authority_posture,
        RunbookAuthorityPosture::Authoritative
    );
    assert!(src.is_executable());
    assert!(src.badge().promoted);
    // The voucher is a governed, authority-bearing, non-browser source.
    let promotion = src.promotion.as_ref().unwrap();
    let voucher = register.source(&promotion.promoted_by_source_id).unwrap();
    assert!(!voucher.provenance_class.is_browser_reference());
    assert!(voucher.effective_authority_posture.is_authority_bearing());
}

#[test]
fn a_promotion_pointing_at_a_missing_voucher_is_rejected() {
    let mut register = canonical();
    for src in &mut register.sources {
        if let Some(promotion) = &mut src.promotion {
            promotion.promoted_by_source_id = "src:does-not-exist".to_owned();
        }
    }
    register.badges = register.sources.iter().map(|s| s.badge()).collect();
    assert!(register
        .validate()
        .contains(&M5RunbookSourceViolation::PromotionReferenceInvalid));
}

#[test]
fn a_browser_capture_cannot_claim_a_verified_first_party_signature() {
    let mut src = seeded_runbook_sources()
        .into_iter()
        .find(|s| s.source_id == "src:browser-vendor-scaling")
        .unwrap();
    // Forge a verified signature on the capture.
    src.signer.signature_verified = true;
    src.freshness.provenance_verified = true;
    let violations = src.validate();
    assert!(violations.contains(&M5RunbookSourceViolation::FirstPartyMasquerade));
}

#[test]
fn a_browser_capture_cannot_use_a_first_party_provenance_kind() {
    let mut src = seeded_runbook_sources()
        .into_iter()
        .find(|s| s.source_id == "src:browser-vendor-scaling")
        .unwrap();
    src.signer.provenance_kind = RunbookProvenanceKind::SignedFirstParty;
    let violations = src.validate();
    assert!(violations.contains(&M5RunbookSourceViolation::FirstPartyMasquerade));
    assert!(violations.contains(&M5RunbookSourceViolation::ProvenanceKindMismatch));
}

#[test]
fn stale_mirror_auto_narrows_to_reference_only() {
    let register = seeded_m5_runbook_source_register_stale_mirror_narrowed();
    assert!(register.validate().is_empty(), "{:?}", register.validate());
    let mirror = register.source("src:mirror-observability-pack").unwrap();
    assert_eq!(mirror.freshness_state(), RunbookSourceFreshnessState::Stale);
    // The mirror was authoritative-bearing (`mirrored`); staleness narrows it.
    assert_eq!(
        mirror.declared_authority_posture,
        RunbookAuthorityPosture::Mirrored
    );
    assert_eq!(
        mirror.effective_authority_posture,
        RunbookAuthorityPosture::ReferenceOnly
    );
    assert!(!mirror.is_executable());
    // Every other source is unchanged.
    let repo = register.source("src:repo-pipeline-restart").unwrap();
    assert_eq!(
        repo.effective_authority_posture,
        RunbookAuthorityPosture::Authoritative
    );
}

#[test]
fn freshness_states_derive_from_the_window() {
    let window = |age: u32, verified: bool| FreshnessWindow {
        fresh_within_days: 30,
        stale_after_days: 90,
        days_since_verification: age,
        provenance_verified: verified,
    };
    assert_eq!(window(10, true).state(), RunbookSourceFreshnessState::Fresh);
    assert_eq!(window(60, true).state(), RunbookSourceFreshnessState::Aging);
    assert_eq!(
        window(200, true).state(),
        RunbookSourceFreshnessState::Stale
    );
    assert_eq!(
        window(10, false).state(),
        RunbookSourceFreshnessState::Expired
    );
}

#[test]
fn a_source_cannot_lie_about_its_declared_posture() {
    let mut src = repo_local_source();
    src.declared_authority_posture = RunbookAuthorityPosture::ReferenceOnly;
    assert!(src
        .validate()
        .contains(&M5RunbookSourceViolation::DeclaredPostureMismatch));
}

#[test]
fn the_same_badge_is_exposed_on_every_surface() {
    let register = canonical();
    // Every surface exposes the register.
    assert!(register.surface_exposure.all_expose());
    let docs = register.badges_for_surface(RunbookSourceSurface::DocsBrowser);
    let incident = register.badges_for_surface(RunbookSourceSurface::IncidentWorkspace);
    let operator = register.badges_for_surface(RunbookSourceSurface::OperatorDashboard);
    // The non-export surfaces render identical truth.
    assert_eq!(docs, incident);
    assert_eq!(docs, operator);
    assert_eq!(docs, register.badges);
    // The support export carries the exportable subset with the same truth.
    let export = register.badges_for_surface(RunbookSourceSurface::SupportExport);
    for badge in &export {
        assert!(docs.contains(badge));
    }
}

#[test]
fn version_attestation_must_match() {
    let mut src = repo_local_source();
    src.signer.attested_version = "pipeline-restart@v6".to_owned();
    assert!(src
        .validate()
        .contains(&M5RunbookSourceViolation::VersionAttestationMismatch));
}

#[test]
fn conformance_review_holds_and_is_derived() {
    let register = canonical();
    assert!(register.conformance.all_hold());
    assert!(register.vocabulary.matches_canonical());
    // Tampering with the stored conformance is caught.
    let mut tampered = register.clone();
    tampered
        .conformance
        .reference_only_cannot_masquerade_as_first_party_executable = false;
    assert!(tampered
        .validate()
        .contains(&M5RunbookSourceViolation::ConformanceReviewFailed));
}

#[test]
fn duplicate_source_ids_are_rejected() {
    let mut register = canonical();
    let dup = register.sources[0].clone();
    register.sources.push(dup);
    register.badges = register.sources.iter().map(|s| s.badge()).collect();
    assert!(register
        .validate()
        .contains(&M5RunbookSourceViolation::DuplicateSourceId));
}

#[test]
fn round_trips_through_json() {
    let register = canonical();
    let json = register.export_safe_json();
    let parsed: M5RunbookSourceRegister = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(parsed, register);
    assert!(parsed.validate().is_empty());
}

#[test]
fn markdown_summary_names_sources_and_postures() {
    let summary = canonical().render_markdown_summary();
    assert!(summary.contains("Governed runbook sources"));
    assert!(summary.contains("repo_local"));
    assert!(summary.contains("Reference only"));
    assert!(summary.contains("src:browser-vendor-scaling"));
}

fn repo_local_source() -> GovernedRunbookSource {
    seeded_runbook_sources()
        .into_iter()
        .find(|s| s.source_id == "src:repo-pipeline-restart")
        .unwrap()
}
