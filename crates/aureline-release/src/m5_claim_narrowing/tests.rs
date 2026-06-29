//! Inline tests for the M5 claim-narrowing lane.

use super::*;

fn registry() -> M5ClaimNarrowingRegistry {
    seeded_m5_claim_narrowing_registry()
}

#[test]
fn canonical_registry_validates() {
    let registry = registry();
    assert!(registry.validate().is_empty(), "{:?}", registry.validate());
    assert_eq!(registry.registry_id, M5_CLAIM_NARROWING_REGISTRY_ID);
    assert_eq!(registry.record_kind, M5_CLAIM_NARROWING_RECORD_KIND);
    assert_eq!(registry.cases.len(), 6);
    assert!(registry.conformance.all_hold());
    assert!(registry.vocabulary.matches_canonical());
}

#[test]
fn every_case_validates() {
    for case in registry().cases {
        assert_eq!(case.record_kind, M5_CLAIM_NARROWING_CASE_RECORD_KIND);
        assert!(case.validate().is_empty(), "{:?}", case.validate());
    }
}

#[test]
fn controlled_vocabulary_freezes_every_claim_state() {
    let vocab = ClaimNarrowingVocabulary::canonical();
    assert_eq!(vocab.claim_states.len(), NarrowedClaimState::ALL.len());
    assert_eq!(
        vocab.restoration_actions.len(),
        RestorationAction::ALL.len()
    );
    // The spec's named controlled states are first-class tokens.
    for needle in [
        "fully_supported",
        "limited",
        "retest_pending",
        "evidence_stale",
        "unsupported_client",
        "unsupported",
    ] {
        assert!(
            vocab.claim_states.contains(&needle.to_owned()),
            "claim-state vocabulary dropped `{needle}`"
        );
    }
}

#[test]
fn fully_supported_case_stands_at_stable() {
    let case = seeded_fully_supported_case();
    assert!(case.is_fully_supported());
    assert!(case.reasons.is_empty());
    assert!(case.restoration.is_empty());
    assert_eq!(
        case.canonical_claim_state,
        NarrowedClaimState::FullySupported
    );
    assert_eq!(
        case.canonical_effective_qualification,
        QualificationClass::Stable
    );
}

#[test]
fn each_degraded_case_yields_its_state() {
    let expectations = [
        (seeded_limited_case(), NarrowedClaimState::Limited),
        (
            seeded_retest_pending_case(),
            NarrowedClaimState::RetestPending,
        ),
        (
            seeded_evidence_stale_case(),
            NarrowedClaimState::EvidenceStale,
        ),
        (
            seeded_unsupported_client_case(),
            NarrowedClaimState::UnsupportedClient,
        ),
        (seeded_unsupported_case(), NarrowedClaimState::Unsupported),
    ];
    for (case, expected) in expectations {
        assert_eq!(
            case.canonical_claim_state, expected,
            "case `{}` derived the wrong state",
            case.case_id
        );
        assert!(!case.reasons.is_empty(), "degraded case needs a reason");
    }
}

#[test]
fn stale_or_narrowed_descriptor_never_reads_fully_supported() {
    // Acceptance criterion 1: a stale/narrowed supporting descriptor cannot leave any consumer
    // surface green by accident.
    for case in registry().cases {
        if case.descriptor.narrowings.is_empty() {
            continue;
        }
        for projection in &case.consumer_projections {
            assert!(
                !projection.claim_state.is_fully_supported(),
                "case `{}` left consumer `{}` fully supported",
                case.case_id,
                projection.consumer.as_str()
            );
        }
    }
}

#[test]
fn consumers_converge_on_the_same_state() {
    // Acceptance criterion 2: different consumers converge on the same downgraded state for the
    // same underlying evidence condition.
    let expected_consumers: Vec<&str> = PublicTruthConsumer::ALL
        .iter()
        .map(|c| c.as_str())
        .collect();
    for case in registry().cases {
        let projected: Vec<&str> = case
            .consumer_projections
            .iter()
            .map(|p| p.consumer.as_str())
            .collect();
        assert_eq!(
            projected, expected_consumers,
            "case `{}` did not project every consumer",
            case.case_id
        );
        for projection in &case.consumer_projections {
            assert_eq!(projection.claim_state, case.canonical_claim_state);
            assert_eq!(
                projection.effective_qualification,
                case.canonical_effective_qualification
            );
            assert!(projection.converges_with_canonical);
        }
    }
}

#[test]
fn reasons_and_restoration_are_inspectable() {
    // Acceptance criterion 3: users and support can inspect why a claim narrowed and what would
    // restore it.
    let case = seeded_evidence_stale_case();
    assert!(case
        .reasons
        .iter()
        .any(|r| matches!(r.facet, DescriptorFacet::FreshnessState)));
    assert!(case.reasons.iter().all(|r| r
        .reason_message_id
        .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)));
    // Stale evidence restores by refreshing it.
    assert!(case
        .restoration
        .iter()
        .any(|s| matches!(s.action, RestorationAction::RefreshEvidence)));
    // Every narrowing has a paired restoration step.
    assert_eq!(case.reasons.len(), case.restoration.len());
}

#[test]
fn restoration_action_matches_narrowing_kind() {
    // Client-scope narrowing restores by using the desktop client.
    let client = seeded_unsupported_client_case();
    assert!(client
        .restoration
        .iter()
        .any(|s| matches!(s.action, RestorationAction::UseDesktopClient)));
    // A limited-evidence narrowing restores by completing the evidence; the unsigned origin by
    // providing provenance.
    let limited = seeded_limited_case();
    assert!(limited
        .restoration
        .iter()
        .any(|s| matches!(s.action, RestorationAction::CompleteEvidence)));
    assert!(limited
        .restoration
        .iter()
        .any(|s| matches!(s.action, RestorationAction::ProvideProvenance)));
}

#[test]
fn blocking_condition_holds_unsupported() {
    let case = seeded_unsupported_case();
    assert!(case.is_blocked());
    assert_eq!(case.canonical_claim_state, NarrowedClaimState::Unsupported);
    assert_eq!(
        case.canonical_effective_qualification,
        QualificationClass::Unavailable
    );
    for projection in &case.consumer_projections {
        assert_eq!(projection.claim_state, NarrowedClaimState::Unsupported);
        assert_eq!(
            projection.effective_qualification,
            QualificationClass::Unavailable
        );
    }
}

#[test]
fn weaker_origins_survive_as_reasons() {
    // The blocked, side-loaded case carries a not-provided origin — it must surface as a reason,
    // never collapse into omission.
    let case = seeded_unsupported_case();
    assert!(case
        .reasons
        .iter()
        .any(|r| matches!(r.facet, DescriptorFacet::SourceClass) && r.token == "not_provided"));
}

#[test]
fn registry_round_trips() {
    let registry = registry();
    let json = registry.export_safe_json();
    let parsed: M5ClaimNarrowingRegistry =
        serde_json::from_str(&json).expect("registry deserializes");
    assert_eq!(parsed, registry);
    assert!(parsed.validate().is_empty());
}

#[test]
fn case_round_trips_and_preserves_descriptor() {
    for case in registry().cases {
        let json = case.export_safe_json();
        let parsed: ClaimNarrowingCase = serde_json::from_str(&json).expect("case deserializes");
        assert_eq!(parsed, case);
        assert_eq!(
            parsed.descriptor.descriptor_id,
            case.condition_descriptor_id
        );
    }
}

#[test]
fn tampered_claim_state_is_rejected() {
    let mut case = seeded_evidence_stale_case();
    case.canonical_claim_state = NarrowedClaimState::FullySupported;
    let violations = case.validate();
    assert!(violations.contains(&M5ClaimNarrowingViolation::ClaimStateDrift));
}

#[test]
fn tampered_projection_is_rejected() {
    // Forcing a narrowed surface back to fully supported must be rejected — the AC1 guard.
    let mut case = seeded_evidence_stale_case();
    case.consumer_projections[0].claim_state = NarrowedClaimState::FullySupported;
    let violations = case.validate();
    assert!(violations.contains(&M5ClaimNarrowingViolation::ConsumerDiverged));
    assert!(violations.contains(&M5ClaimNarrowingViolation::NarrowedSurfaceReadsSupported));
}

#[test]
fn tampered_reasons_are_rejected() {
    let mut case = seeded_limited_case();
    case.reasons.clear();
    assert!(case
        .validate()
        .contains(&M5ClaimNarrowingViolation::ReasonDrift));
}

#[test]
fn markdown_render_names_cases_and_convergence() {
    let md = registry().render_markdown_summary();
    assert!(md.contains("# M5 claim-narrowing parity"));
    assert!(md.contains("Consumer convergence"));
    assert!(md.contains("evidence_stale"));
    assert!(md.contains("unsupported"));
    assert!(md.contains("Restores when"));
}

#[test]
fn registry_consumes_one_runtime_across_consumers() {
    let registry = registry();
    let expected: Vec<String> = PublicTruthConsumer::ALL
        .iter()
        .map(|c| c.as_str().to_owned())
        .collect();
    assert_eq!(registry.consumers, expected);
    assert!(registry.conformance.shared_across_consumers);
    // Every case projects exactly the eight public-truth consumers.
    for case in registry.cases {
        assert_eq!(
            case.consumer_projections.len(),
            PublicTruthConsumer::ALL.len()
        );
    }
}

#[test]
fn summary_counts_match() {
    let registry = registry();
    let s = &registry.summary;
    assert_eq!(s.total_cases, 6);
    assert_eq!(s.fully_supported_cases, 1);
    assert_eq!(s.blocked_cases, 1);
    assert_eq!(s.narrowed_cases, 4);
    // Every projection converges in the canonical corpus.
    assert_eq!(s.total_projections, s.converged_projections);
    assert_eq!(
        s.total_projections,
        6 * PublicTruthConsumer::ALL.len() as u32
    );
}
