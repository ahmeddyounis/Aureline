//! Inline unit coverage for the docs version-freshness packet.

use super::*;

fn stable_packet() -> DocsVersionFreshnessPacket {
    DocsVersionFreshnessPacket::materialize(seeded_stable_docs_version_freshness_input())
}

#[test]
fn seeded_packet_is_clean_stable() {
    let packet = stable_packet();
    assert_eq!(packet.record_kind, DOCS_VERSION_FRESHNESS_RECORD_KIND);
    assert_eq!(packet.schema_version, DOCS_VERSION_FRESHNESS_SCHEMA_VERSION);
    assert_eq!(
        packet.promotion_state,
        DocsVersionFreshnessPromotionState::Stable
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_clean_stable());
    assert!(packet.is_stable());
}

#[test]
fn seeded_packet_exercises_every_state_in_the_vocabulary() {
    let packet = stable_packet();
    let tokens = packet.state_tokens();
    for state in DocsVersionFreshnessState::ALL {
        assert!(
            tokens.contains(&state.as_str()),
            "missing state {}",
            state.as_str()
        );
    }
}

#[test]
fn the_eight_state_tokens_are_pinned() {
    let expected = [
        "exact",
        "nearby",
        "project_specific",
        "mirrored",
        "cached",
        "stale",
        "policy_blocked",
        "browser_handoff_required",
    ];
    let observed: Vec<&str> = DocsVersionFreshnessState::ALL
        .iter()
        .map(|state| state.as_str())
        .collect();
    assert_eq!(observed, expected);
}

#[test]
fn only_exact_is_current_exact_confidence() {
    for state in DocsVersionFreshnessState::ALL {
        let is_exact = state == DocsVersionFreshnessState::Exact;
        assert_eq!(
            state.confidence_class().is_current_exact(),
            is_exact,
            "state {} confidence treatment",
            state.as_str()
        );
    }
}

#[test]
fn cached_and_nearby_never_share_exact_confidence() {
    assert_ne!(
        DocsVersionFreshnessState::Cached.confidence_class(),
        DocsVersionFreshnessState::Exact.confidence_class()
    );
    assert_ne!(
        DocsVersionFreshnessState::Nearby.confidence_class(),
        DocsVersionFreshnessState::Exact.confidence_class()
    );
    // The distinct states keep distinct treatments rather than collapsing.
    assert_ne!(
        DocsVersionFreshnessState::Cached.confidence_class(),
        DocsVersionFreshnessState::Nearby.confidence_class()
    );
}

#[test]
fn version_mismatch_states_require_disclosure() {
    for state in DocsVersionFreshnessState::ALL {
        let expected = matches!(
            state,
            DocsVersionFreshnessState::Nearby
                | DocsVersionFreshnessState::Mirrored
                | DocsVersionFreshnessState::Cached
                | DocsVersionFreshnessState::Stale
        );
        assert_eq!(state.requires_version_disclosure(), expected);
    }
}

#[test]
fn not_inline_states_require_reason() {
    assert!(DocsVersionFreshnessState::PolicyBlocked.requires_state_reason());
    assert!(DocsVersionFreshnessState::BrowserHandoffRequired.requires_state_reason());
    assert!(!DocsVersionFreshnessState::Exact.requires_state_reason());
    assert!(!DocsVersionFreshnessState::PolicyBlocked.answered_inline());
    assert!(DocsVersionFreshnessState::Exact.answered_inline());
}

#[test]
fn cached_card_claiming_exact_confidence_blocks_stable() {
    let mut input = seeded_stable_docs_version_freshness_input();
    let card = input
        .cards
        .iter_mut()
        .find(|card| card.state == DocsVersionFreshnessState::Cached)
        .expect("cached card present");
    card.confidence = DocsVersionFreshnessConfidence::CurrentExact;
    let packet = DocsVersionFreshnessPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsVersionFreshnessPromotionState::BlocksStable
    );
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == DocsVersionFreshnessValidationKind::CardConfidenceCollapsed
    }));
}

#[test]
fn version_mismatch_without_disclosure_blocks_stable() {
    let mut input = seeded_stable_docs_version_freshness_input();
    let card = input
        .cards
        .iter_mut()
        .find(|card| card.state == DocsVersionFreshnessState::Nearby)
        .expect("nearby card present");
    card.version_disclosure = None;
    let packet = DocsVersionFreshnessPacket::materialize(input);
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == DocsVersionFreshnessValidationKind::VersionDisclosureMissing
    }));
}

#[test]
fn broken_link_finding_must_be_about_a_link() {
    let mut input = seeded_stable_docs_version_freshness_input();
    let finding = input
        .findings
        .iter_mut()
        .find(|finding| finding.finding_class == DocsVersionFreshnessFindingClass::BrokenLink)
        .expect("broken-link finding present");
    finding.subject_kind = DocsVersionFreshnessSubjectKind::CodeBlock;
    let packet = DocsVersionFreshnessPacket::materialize(input);
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == DocsVersionFreshnessValidationKind::FindingSubjectClassMismatch
    }));
}

#[test]
fn blocking_finding_blocks_stable_but_suppression_clears_it() {
    let mut input = seeded_stable_docs_version_freshness_input();
    input.findings[0].severity = DocsVersionFreshnessFindingSeverity::Blocking;
    let packet = DocsVersionFreshnessPacket::materialize(input.clone());
    assert_eq!(
        packet.promotion_state,
        DocsVersionFreshnessPromotionState::BlocksStable
    );

    // Suppressing the finding (with a disclosed reason) drops it back to advisory.
    input.findings[0].actions.suppression_state =
        DocsVersionFreshnessSuppressionState::SuppressedByReviewer;
    input.findings[0].actions.suppression_reason =
        Some("reviewer confirmed the example is intentionally retained".to_owned());
    let packet = DocsVersionFreshnessPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsVersionFreshnessPromotionState::Stable
    );
}

#[test]
fn finding_dropping_actions_blocks_stable() {
    let mut input = seeded_stable_docs_version_freshness_input();
    input.findings[0].actions.compare_ref = String::new();
    let packet = DocsVersionFreshnessPacket::materialize(input);
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == DocsVersionFreshnessValidationKind::FindingActionsMissing
    }));
}

#[test]
fn orphan_finding_blocks_stable() {
    let mut input = seeded_stable_docs_version_freshness_input();
    input.findings[0].card_id_ref = "card:does-not-exist".to_owned();
    let packet = DocsVersionFreshnessPacket::materialize(input);
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == DocsVersionFreshnessValidationKind::FindingOrphan
    }));
}

#[test]
fn collapsing_state_distinctions_blocks_stable() {
    let mut input = seeded_stable_docs_version_freshness_input();
    input.consumer_projections[0].preserves_state_distinctions = false;
    let packet = DocsVersionFreshnessPacket::materialize(input);
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == DocsVersionFreshnessValidationKind::StateDistinctionCollapsed
    }));
}

#[test]
fn missing_state_coverage_blocks_stable() {
    let mut input = seeded_stable_docs_version_freshness_input();
    input
        .cards
        .retain(|card| card.state != DocsVersionFreshnessState::BrowserHandoffRequired);
    let packet = DocsVersionFreshnessPacket::materialize(input);
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == DocsVersionFreshnessValidationKind::VocabularyCoverageMissing
    }));
}

#[test]
fn policy_blocked_without_reason_blocks_stable() {
    let mut input = seeded_stable_docs_version_freshness_input();
    let card = input
        .cards
        .iter_mut()
        .find(|card| card.state == DocsVersionFreshnessState::PolicyBlocked)
        .expect("policy-blocked card present");
    card.state_reason = None;
    let packet = DocsVersionFreshnessPacket::materialize(input);
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == DocsVersionFreshnessValidationKind::StateReasonMissing
    }));
}

#[test]
fn every_required_surface_has_a_projection() {
    let packet = stable_packet();
    for surface in DocsVersionFreshnessConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "missing projection for {}",
            surface.as_str()
        );
    }
}

#[test]
fn support_export_round_trips_and_is_safe() {
    let packet = stable_packet();
    let export = packet.support_export("export:test:001", "2026-06-26T00:00:00Z");
    assert!(export.is_export_safe());
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: DocsVersionFreshnessSupportExport =
        serde_json::from_str(&json).expect("round trips");
    assert_eq!(parsed, export);
}

#[test]
fn promotion_mismatch_is_detected_on_revalidation() {
    let mut packet = stable_packet();
    packet.promotion_state = DocsVersionFreshnessPromotionState::BlocksStable;
    assert!(packet.validate().iter().any(|finding| {
        finding.finding_kind == DocsVersionFreshnessValidationKind::PromotionStateMismatch
    }));
}
