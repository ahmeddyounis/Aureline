//! Freeze gate for the M5 attention-qualification bundle.
//!
//! The checked-in fixture `fixtures/activity/m5-attention-qualification/canonical_bundle.json`
//! is the published certification. This gate rebuilds the bundle in code and asserts it equals
//! the fixture after a serialize round-trip, so the family proof bindings, profile dependency
//! graph, and derived claim-narrowing rules cannot drift from the published artifact without
//! failing CI. It also re-proves support-export safety, that every claimed family maps to a
//! complete proof packet, that the canonical (all-fresh) bundle promotes every profile at full
//! strength, that a stale dependency narrows exactly the dependent profiles, that a failing
//! dependency withdraws them, that the routing/quiet-hours/fanout spines narrow every claim,
//! that certification never silences a security advisory, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_activity::m5_attention_qualification::{
    attention_qualification_bundle, evaluate_profile_claim, recompute_profiles, AttentionFamily,
    AttentionQualificationBundle, ClaimState, ClaimedProfile, EvidenceState,
    M5_ATTENTION_QUALIFICATION_RECORD_KIND, M5_ATTENTION_QUALIFICATION_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/activity/m5-attention-qualification/canonical_bundle.json")
}

fn load_fixture() -> AttentionQualificationBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = attention_qualification_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code attention-qualification bundle drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-activity --example dump_m5_attention_qualification`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ATTENTION_QUALIFICATION_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ATTENTION_QUALIFICATION_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: AttentionQualificationBundle =
        serde_json::from_str(&serde_json::to_string(&fixture).expect("serializes"))
            .expect("round-trips");
    assert_eq!(roundtrip, fixture);
}

#[test]
fn every_frozen_invariant_holds() {
    let fixture = load_fixture();
    assert!(!fixture.invariants.is_empty());
    for invariant in &fixture.invariants {
        assert!(
            invariant.holds,
            "frozen invariant must hold: {}",
            invariant.invariant_id
        );
    }
    assert!(fixture.all_invariants_hold());
}

#[test]
fn canonical_claims_are_all_full() {
    let fixture = load_fixture();
    for profile in &fixture.profiles {
        assert_eq!(
            profile.claim_state,
            ClaimState::Full,
            "{} full when all evidence is fresh",
            profile.profile.as_str()
        );
        assert!(profile.narrowed_by.is_empty());
    }
}

#[test]
fn stale_proof_narrows_exactly_the_dependent_profiles() {
    let fixture = load_fixture();
    for family in AttentionFamily::ALL {
        let rows = recompute_profiles(&fixture.families, &[(family, EvidenceState::Stale)]);
        for row in &rows {
            if row.depends_on_family(family) {
                assert_eq!(
                    row.claim_state,
                    ClaimState::Narrowed,
                    "{} narrows on stale {}",
                    row.profile.as_str(),
                    family.as_str()
                );
                assert!(row.narrowed_by.iter().any(|r| r.family == family));
            } else {
                assert_eq!(
                    row.claim_state,
                    ClaimState::Full,
                    "{} unaffected by stale {}",
                    row.profile.as_str(),
                    family.as_str()
                );
            }
        }
    }
}

#[test]
fn failing_proof_withdraws_the_dependent_profiles() {
    let fixture = load_fixture();
    for family in AttentionFamily::ALL {
        let rows = recompute_profiles(&fixture.families, &[(family, EvidenceState::Failing)]);
        for row in &rows {
            if row.depends_on_family(family) {
                assert_eq!(
                    row.claim_state,
                    ClaimState::Withdrawn,
                    "{} withdrawn on failing {}",
                    row.profile.as_str(),
                    family.as_str()
                );
            }
        }
    }
}

#[test]
fn routing_privacy_and_fanout_spines_narrow_every_claim() {
    let fixture = load_fixture();
    for spine in [
        AttentionFamily::AttentionRoutingMatrix,
        AttentionFamily::QuietHoursSuppression,
        AttentionFamily::FanoutReceipt,
    ] {
        let rows = recompute_profiles(&fixture.families, &[(spine, EvidenceState::Stale)]);
        assert!(
            rows.iter().all(|r| r.claim_state == ClaimState::Narrowed),
            "stale {} narrows every claim",
            spine.as_str()
        );
    }
}

#[test]
fn missing_proof_is_treated_as_blocking() {
    let fixture = load_fixture();
    // A dependency with no qualification row at all evaluates as missing → withdrawn.
    let without_fanout: Vec<_> = fixture
        .families
        .iter()
        .filter(|f| f.family != AttentionFamily::FanoutReceipt)
        .cloned()
        .collect();
    let shell_deps = ClaimedProfile::ShellAttention.dependencies();
    let outcome = evaluate_profile_claim(&shell_deps, &without_fanout);
    assert_eq!(outcome.claim_state, ClaimState::Withdrawn);
    assert!(outcome
        .narrowed_by
        .iter()
        .any(|r| r.family == AttentionFamily::FanoutReceipt
            && r.evidence_state == EvidenceState::Missing));
}

#[test]
fn certification_never_silences_a_security_advisory() {
    let fixture = load_fixture();
    assert!(fixture
        .families
        .iter()
        .all(|f| f.preserves_security_escalation));
}
