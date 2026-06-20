use super::*;

use crate::m5_reactive_governance::seeded_m5_reactive_governance_packet;

#[test]
fn seeded_packet_validates_and_covers_every_surface() {
    let packet = seeded_reactive_truth_surfaces_packet();
    validate_reactive_truth_surfaces_packet(&packet).expect("seeded audit must validate");
    let governance = seeded_m5_reactive_governance_packet();
    assert_eq!(
        packet.surfaces.len(),
        governance.surfaces.len(),
        "audit must cover every governed surface"
    );
}

#[test]
fn no_audited_surface_overclaims_exact_current_truth() {
    let packet = seeded_reactive_truth_surfaces_packet();
    for audit in &packet.surfaces {
        assert_eq!(audit.derivation_class, DerivationClass::Derived);
        assert_ne!(audit.healthy_claim, TruthClaim::ExactCurrentTruth);
        assert_eq!(audit.healthy_claim, TruthClaim::ConsistentSnapshot);
        // Derived actions are live only at the consistent-snapshot ceiling.
        assert_eq!(audit.healthy_action_gate, ActionGate::Enabled);
    }
}

#[test]
fn seeded_fixtures_validate() {
    let fixtures = seeded_reactive_truth_surfaces_fixtures();
    assert_eq!(fixtures.len(), 11);
    for fixture in &fixtures {
        validate_reactive_truth_surfaces_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn trigger_claims_match_engine_single_axis() {
    // For every trigger, the cue layer's single-axis claim must equal what
    // the canonical engine narrows to when only that axis degrades.
    let cases = [
        (
            NarrowingTrigger::FreshnessStale,
            ObservedReactiveState {
                freshness: Freshness::Stale,
                ..ObservedReactiveState::healthy()
            },
        ),
        (
            NarrowingTrigger::FreshnessCached,
            ObservedReactiveState {
                freshness: Freshness::Cached,
                ..ObservedReactiveState::healthy()
            },
        ),
        (
            NarrowingTrigger::CompletenessPartial,
            ObservedReactiveState {
                completeness: Completeness::Partial,
                ..ObservedReactiveState::healthy()
            },
        ),
        (
            NarrowingTrigger::BackpressureCoalesced,
            ObservedReactiveState {
                backpressure_mode: BackpressureMode::Coalesced,
                ..ObservedReactiveState::healthy()
            },
        ),
        (
            NarrowingTrigger::PolicyLimited,
            ObservedReactiveState {
                policy_limited: true,
                ..ObservedReactiveState::healthy()
            },
        ),
    ];
    for (trigger, observed) in cases {
        let narrowed = narrow_truth_claim(DerivationClass::Derived, &observed);
        assert_eq!(
            claim_for_trigger(trigger),
            narrowed.claim,
            "trigger {} forked the engine",
            trigger.as_str()
        );
    }
}

#[test]
fn stale_surface_blocks_dangerous_actions() {
    let observed = ObservedReactiveState {
        freshness: Freshness::Stale,
        ..ObservedReactiveState::healthy()
    };
    let cue = build_reactive_truth_cue(ReactiveSurfaceClass::SearchResults, observed).expect("cue");
    assert_eq!(cue.narrowed_claim, TruthClaim::StaleSnapshot);
    assert_eq!(cue.action_gate, ActionGate::Blocked);
    assert!(!cue.dangerous_action_enabled);
    assert_eq!(
        cue.invalidation_reason,
        Some(InvalidationReason::UpstreamInputStale)
    );
    assert!(!cue.resubscribe_required);
}

#[test]
fn partial_surface_narrows_to_read_only() {
    let observed = ObservedReactiveState {
        completeness: Completeness::Partial,
        ..ObservedReactiveState::healthy()
    };
    let cue =
        build_reactive_truth_cue(ReactiveSurfaceClass::GraphNeighborhood, observed).expect("cue");
    assert_eq!(cue.narrowed_claim, TruthClaim::PartialProjection);
    assert_eq!(cue.action_gate, ActionGate::NarrowedToReadOnly);
    assert!(!cue.dangerous_action_enabled);
}

#[test]
fn unavailable_provider_blocks_and_requires_resubscribe() {
    let observed = ObservedReactiveState {
        completeness: Completeness::Unavailable,
        terminal_reason: Some(TerminalReason::Unavailable),
        ..ObservedReactiveState::healthy()
    };
    let cue =
        build_reactive_truth_cue(ReactiveSurfaceClass::CompanionPanel, observed).expect("cue");
    assert_eq!(cue.narrowed_claim, TruthClaim::ProviderUnavailable);
    assert_eq!(cue.action_gate, ActionGate::Blocked);
    assert!(cue.resubscribe_required);
}

#[test]
fn snapshot_required_requests_resubscribe_without_blocking() {
    let observed = ObservedReactiveState {
        backpressure_mode: BackpressureMode::SnapshotRequired,
        ..ObservedReactiveState::healthy()
    };
    let cue = build_reactive_truth_cue(ReactiveSurfaceClass::SearchResults, observed).expect("cue");
    assert_eq!(cue.narrowed_claim, TruthClaim::CoalescedStream);
    assert_eq!(cue.action_gate, ActionGate::RevalidateBeforeAct);
    assert!(cue.resubscribe_required);
}

#[test]
fn healthy_surface_presents_consistent_snapshot() {
    let cue = build_reactive_truth_cue(
        ReactiveSurfaceClass::ShellWorkspaceTree,
        ObservedReactiveState::healthy(),
    )
    .expect("cue");
    assert_eq!(cue.narrowed_claim, TruthClaim::ConsistentSnapshot);
    assert!(!cue.narrowed);
    assert_eq!(cue.action_gate, ActionGate::Enabled);
    assert!(cue.dangerous_action_enabled);
    assert_eq!(cue.invalidation_reason, None);
    assert!(!cue.presents_exact_current_truth());
}

#[test]
fn channels_carry_the_same_claim_and_gate_tokens() {
    let observed = ObservedReactiveState {
        freshness: Freshness::Stale,
        ..ObservedReactiveState::healthy()
    };
    let cue = build_reactive_truth_cue(ReactiveSurfaceClass::SearchResults, observed).expect("cue");
    for channel in [
        CueChannel::UiStrip,
        CueChannel::CliHeadless,
        CueChannel::ActivityCenter,
        CueChannel::KeyboardHelp,
    ] {
        let rendered = render_cue(&cue, channel);
        assert!(
            rendered.contains(cue.narrowed_claim.as_str()),
            "channel {} dropped the claim token",
            channel.as_str()
        );
        assert!(
            rendered.contains(cue.action_gate.as_str()),
            "channel {} dropped the gate token",
            channel.as_str()
        );
    }
    // Accessibility narration carries the source and invalidation too.
    let narration = render_cue(&cue, CueChannel::Accessibility);
    assert!(narration.contains("upstream_input_stale"));
    assert!(narration.contains("workspace_vfs") || narration.contains("derived_knowledge"));
}

#[test]
fn audit_plaintext_is_deterministic() {
    let packet = seeded_reactive_truth_surfaces_packet();
    let first = render_reactive_truth_surfaces_audit_plaintext(&packet);
    let second = render_reactive_truth_surfaces_audit_plaintext(&packet);
    assert_eq!(first, second);
    assert!(first.contains("search_results"));
    assert!(first.contains("consistent_snapshot"));
}
