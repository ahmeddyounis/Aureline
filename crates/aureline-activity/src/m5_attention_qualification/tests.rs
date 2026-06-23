//! Unit tests for the attention-qualification bundle: proof binding, derived
//! claim narrowing, the consumer projection, invariants, and export safety.

use super::*;

fn bundle() -> AttentionQualificationBundle {
    attention_qualification_bundle()
}

#[test]
fn bundle_validates_and_all_invariants_hold() {
    let bundle = bundle();
    bundle.validate().expect("canonical bundle validates");
    assert!(bundle.all_invariants_hold());
    assert!(!bundle.invariants.is_empty());
}

#[test]
fn bundle_is_deterministic() {
    assert_eq!(
        attention_qualification_bundle(),
        attention_qualification_bundle()
    );
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn every_family_and_profile_present_exactly_once() {
    let bundle = bundle();
    assert_eq!(bundle.families.len(), AttentionFamily::ALL.len());
    assert_eq!(bundle.profiles.len(), ClaimedProfile::ALL.len());
    for family in AttentionFamily::ALL {
        assert!(
            bundle.family(family).is_some(),
            "{} present",
            family.as_str()
        );
    }
    for profile in ClaimedProfile::ALL {
        assert!(
            bundle.profile(profile).is_some(),
            "{} present",
            profile.as_str()
        );
    }
}

#[test]
fn canonical_bundle_promotes_every_profile_full() {
    let bundle = bundle();
    for profile in &bundle.profiles {
        assert_eq!(
            profile.claim_state,
            ClaimState::Full,
            "{} is full when all evidence is fresh",
            profile.profile.as_str()
        );
        assert!(profile.narrowed_by.is_empty());
    }
}

#[test]
fn stale_fanout_narrows_every_profile() {
    // Fanout is a shared dependency: a stale fanout proof narrows all three.
    let bundle = bundle();
    let rows = recompute_profiles(
        &bundle.families,
        &[(AttentionFamily::FanoutReceipt, EvidenceState::Stale)],
    );
    for row in &rows {
        assert_eq!(
            row.claim_state,
            ClaimState::Narrowed,
            "{} narrows on stale fanout",
            row.profile.as_str()
        );
        assert!(row
            .narrowed_by
            .iter()
            .any(|r| r.family == AttentionFamily::FanoutReceipt));
    }
}

#[test]
fn stale_action_semantics_narrows_only_the_shell() {
    // The action engine is a shell-only dependency: companion and operator stay full.
    let bundle = bundle();
    let rows = recompute_profiles(
        &bundle.families,
        &[(AttentionFamily::AttentionAction, EvidenceState::Stale)],
    );
    for row in &rows {
        match row.profile {
            ClaimedProfile::ShellAttention => {
                assert_eq!(row.claim_state, ClaimState::Narrowed);
                assert!(row
                    .narrowed_by
                    .iter()
                    .any(|r| r.family == AttentionFamily::AttentionAction));
            }
            _ => {
                assert_eq!(
                    row.claim_state,
                    ClaimState::Full,
                    "{} unaffected by action staleness",
                    row.profile.as_str()
                );
                assert!(row.narrowed_by.is_empty());
            }
        }
    }
}

#[test]
fn failing_matrix_withdraws_every_claim() {
    // The routing matrix is the spine: a failing gate withdraws every claim.
    let bundle = bundle();
    let rows = recompute_profiles(
        &bundle.families,
        &[(
            AttentionFamily::AttentionRoutingMatrix,
            EvidenceState::Failing,
        )],
    );
    for row in &rows {
        assert_eq!(
            row.claim_state,
            ClaimState::Withdrawn,
            "{} withdrawn on failing matrix",
            row.profile.as_str()
        );
    }
}

#[test]
fn worst_dependency_governs_the_claim() {
    // A stale and a failing dependency together still withdraw (worst wins).
    let bundle = bundle();
    let rows = recompute_profiles(
        &bundle.families,
        &[
            (AttentionFamily::BadgeAggregate, EvidenceState::Stale),
            (
                AttentionFamily::QuietHoursSuppression,
                EvidenceState::Failing,
            ),
        ],
    );
    for row in &rows {
        assert_eq!(row.claim_state, ClaimState::Withdrawn);
        // Both non-fresh dependencies are named, in family order.
        let named: Vec<AttentionFamily> = row.narrowed_by.iter().map(|r| r.family).collect();
        assert!(named.contains(&AttentionFamily::QuietHoursSuppression));
        assert!(named.contains(&AttentionFamily::BadgeAggregate));
    }
}

#[test]
fn narrowed_by_is_in_family_order() {
    let bundle = bundle();
    let rows = recompute_profiles(
        &bundle.families,
        &[
            (AttentionFamily::FanoutReceipt, EvidenceState::Stale),
            (AttentionFamily::NotificationEnvelope, EvidenceState::Stale),
        ],
    );
    let shell = rows
        .iter()
        .find(|r| r.profile == ClaimedProfile::ShellAttention)
        .expect("shell present");
    // NotificationEnvelope precedes FanoutReceipt in AttentionFamily::ALL.
    let positions: Vec<usize> = shell
        .narrowed_by
        .iter()
        .map(|r| {
            AttentionFamily::ALL
                .iter()
                .position(|f| *f == r.family)
                .unwrap()
        })
        .collect();
    let mut sorted = positions.clone();
    sorted.sort_unstable();
    assert_eq!(positions, sorted, "narrowed_by follows family order");
}

#[test]
fn every_release_evidence_row_is_covered() {
    let bundle = bundle();
    assert_eq!(
        bundle.covered_proof_checks().len(),
        ProofCheckTag::ALL.len(),
        "every release-evidence row is covered by some family"
    );
}

#[test]
fn projection_mirrors_the_derived_claims() {
    let bundle = bundle();
    let projection = bundle.projection();
    assert_eq!(projection.bundle_id, bundle.bundle_id);
    assert_eq!(projection.profiles.len(), bundle.profiles.len());
    for row in &projection.profiles {
        let source = bundle.profile(row.profile).expect("profile present");
        assert_eq!(row.claim_state, source.claim_state);
        assert_eq!(row.published_claim, source.published_claim);
    }
    for row in &projection.families {
        let source = bundle.family(row.family).expect("family present");
        assert_eq!(row.evidence_state, source.evidence_state);
    }
}

#[test]
fn evidence_severity_orders_claim_states() {
    assert!(EvidenceState::Fresh.severity() < EvidenceState::Stale.severity());
    assert!(EvidenceState::Stale.severity() < EvidenceState::Failing.severity());
    assert_eq!(
        EvidenceState::Failing.severity(),
        EvidenceState::Missing.severity()
    );
    assert_eq!(ClaimState::from_worst_severity(0), ClaimState::Full);
    assert_eq!(ClaimState::from_worst_severity(1), ClaimState::Narrowed);
    assert_eq!(ClaimState::from_worst_severity(2), ClaimState::Withdrawn);
}

#[test]
fn lines_render_every_family_and_profile() {
    let bundle = bundle();
    let text = attention_qualification_lines(&bundle).join("\n");
    for family in AttentionFamily::ALL {
        assert!(
            text.contains(family.as_str()),
            "lines mention {}",
            family.as_str()
        );
    }
    for profile in ClaimedProfile::ALL {
        assert!(
            text.contains(profile.as_str()),
            "lines mention {}",
            profile.as_str()
        );
    }
}

#[test]
fn validate_rejects_an_asserted_claim() {
    // Hand-edit a profile to claim full while a dependency is stale: validation
    // must reject it because the claim is no longer derived from evidence.
    let mut bundle = bundle();
    if let Some(family) = bundle
        .families
        .iter_mut()
        .find(|f| f.family == AttentionFamily::FanoutReceipt)
    {
        family.evidence_state = EvidenceState::Stale;
    }
    // The profile rows still say Full — inconsistent with the stale dependency.
    let err = bundle
        .validate()
        .expect_err("asserted claim must be rejected");
    assert!(err.reason.contains("not derived"));
}
