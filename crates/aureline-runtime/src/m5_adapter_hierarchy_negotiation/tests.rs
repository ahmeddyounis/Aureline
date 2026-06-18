//! Unit coverage for the M5 adapter hierarchy negotiation baseline.

use super::*;

fn stable() -> AdapterNegotiationBaseline {
    seeded_adapter_hierarchy_negotiation_baseline()
}

#[test]
fn seed_baseline_validates_clean_and_is_stable() {
    let baseline = stable();
    assert_eq!(baseline.record_kind, ADAPTER_NEGOTIATION_RECORD_KIND);
    assert_eq!(baseline.schema_version, ADAPTER_NEGOTIATION_SCHEMA_VERSION);
    assert!(
        baseline.validate().is_empty(),
        "seed baseline must validate clean: {:?}",
        baseline.validate()
    );
    assert!(baseline.is_stable());
    assert_eq!(baseline.promotion_state.as_str(), "stable");
}

#[test]
fn every_ecosystem_resolves_in_priority_order() {
    let baseline = stable();
    assert_eq!(baseline.resolutions.len(), Ecosystem::ALL.len());
    let resolved: Vec<(&str, &str, &str)> = baseline
        .resolutions
        .iter()
        .map(|r| {
            (
                r.ecosystem.as_str(),
                r.selected_source_kind.as_str(),
                r.fallback_class.as_str(),
            )
        })
        .collect();
    assert_eq!(
        resolved,
        vec![
            ("cargo", "native", "native_authoritative"),
            ("gradle_jvm", "bsp", "negotiated_protocol"),
            ("bazel", "bazel-bep", "negotiated_protocol"),
            ("python_pytest", "structured-output", "structured_import"),
            ("node_js", "structured-output", "structured_import"),
            ("generic", "heuristic-parser", "heuristic_last_resort"),
        ]
    );
}

#[test]
fn each_candidate_ladder_covers_the_full_native_first_order() {
    let baseline = stable();
    for resolution in &baseline.resolutions {
        let ranks: Vec<(&str, u8)> = resolution
            .candidate_ladder
            .iter()
            .map(|c| (c.source_kind.as_str(), c.priority_rank))
            .collect();
        assert_eq!(
            ranks,
            vec![
                ("native", 1),
                ("bsp", 2),
                ("bazel-bep", 3),
                ("structured-output", 4),
                ("heuristic-parser", 5),
            ],
            "{} ladder drift",
            resolution.ecosystem.as_str()
        );
        assert_eq!(
            resolution
                .candidate_ladder
                .iter()
                .filter(|c| c.selected)
                .count(),
            1,
            "exactly one selected candidate per ecosystem"
        );
    }
}

#[test]
fn authoritative_resolutions_are_not_downgraded_and_fallbacks_are() {
    let baseline = stable();
    for resolution in &baseline.resolutions {
        let authoritative = source_is_authoritative(resolution.selected_source_kind);
        assert_eq!(
            resolution.downgraded,
            !authoritative,
            "{} downgrade posture drift",
            resolution.ecosystem.as_str()
        );
        assert_eq!(resolution.downgrade_reason.is_some(), resolution.downgraded);
    }
}

#[test]
fn heuristic_resolution_is_a_visible_last_resort() {
    let baseline = stable();
    let generic = baseline
        .resolutions
        .iter()
        .find(|r| r.ecosystem == Ecosystem::Generic)
        .expect("generic resolution present");
    assert_eq!(
        generic.selected_source_kind,
        BuildTestEventSourceKind::HeuristicParser
    );
    assert!(generic.downgraded);
    assert_eq!(
        generic.downgrade_reason,
        Some(DowngradeReason::HeuristicFallback)
    );
    assert_eq!(generic.confidence, BuildTestEventConfidence::Low);
    // Every higher rung is named in the explicit fallback-reason packet.
    assert_eq!(generic.fallback_reasons.len(), 4);
    let structured = generic
        .fallback_reasons
        .iter()
        .find(|r| r.source_kind == BuildTestEventSourceKind::StructuredOutput)
        .expect("structured rung named");
    assert_eq!(structured.skip_reason, SkipReason::CapabilityUnsupported);
}

#[test]
fn fallbacks_name_their_unsupported_capabilities() {
    let baseline = stable();
    let pytest = baseline
        .resolutions
        .iter()
        .find(|r| r.ecosystem == Ecosystem::PythonPytest)
        .expect("pytest resolution present");
    assert_eq!(
        pytest.unsupported_capabilities,
        vec![
            NegotiatedCapability::TargetGraph,
            NegotiatedCapability::Progress
        ]
    );
}

#[test]
fn lower_priority_cannot_displace_an_eligible_higher_adapter() {
    let mut input = current_stable_adapter_hierarchy_negotiation_input();
    let gradle = input
        .resolutions
        .iter_mut()
        .find(|r| r.ecosystem == Ecosystem::GradleJvm)
        .expect("gradle resolution present");
    let native = gradle
        .candidate_ladder
        .iter_mut()
        .find(|c| c.source_kind == BuildTestEventSourceKind::Native)
        .expect("native candidate present");
    // Make native available and capable while BSP stays selected — silent displacement.
    native.available = true;
    native.capabilities = vec![CapabilityNegotiation {
        capability: NegotiatedCapability::LifecycleEvents,
        state: AdapterCapabilityState::Negotiated,
        capability_packet_ref: "capability-packet:gradle_jvm:native:lifecycle_events".to_owned(),
        note: "native is reachable".to_owned(),
    }];
    let baseline = AdapterNegotiationBaseline::materialize(input);
    assert!(baseline
        .validate()
        .iter()
        .any(|f| f.finding_kind == NegotiationFindingKind::LowerPriorityDisplacedHigher));
    assert_eq!(baseline.promotion_state.as_str(), "blocks_stable");
}

#[test]
fn confidence_overclaim_blocks_stable() {
    let mut input = current_stable_adapter_hierarchy_negotiation_input();
    let generic = input
        .resolutions
        .iter_mut()
        .find(|r| r.ecosystem == Ecosystem::Generic)
        .expect("generic resolution present");
    generic.confidence = BuildTestEventConfidence::High;
    let baseline = AdapterNegotiationBaseline::materialize(input);
    assert!(baseline
        .validate()
        .iter()
        .any(|f| f.finding_kind == NegotiationFindingKind::ConfidenceOverclaim));
}

#[test]
fn invisible_drift_blocks_stable() {
    let mut input = current_stable_adapter_hierarchy_negotiation_input();
    input.drift_signals[0].visible_before_trust_loss = false;
    let baseline = AdapterNegotiationBaseline::materialize(input);
    assert!(baseline
        .validate()
        .iter()
        .any(|f| f.finding_kind == NegotiationFindingKind::DriftNotVisible));
}

#[test]
fn dropping_a_disclosure_surface_blocks_stable() {
    let mut input = current_stable_adapter_hierarchy_negotiation_input();
    input
        .disclosure_surfaces
        .retain(|b| b.surface != DisclosureSurface::AiEvidence);
    let baseline = AdapterNegotiationBaseline::materialize(input);
    assert!(baseline
        .validate()
        .iter()
        .any(|f| f.finding_kind == NegotiationFindingKind::DisclosureSurfaceMissing));
}

#[test]
fn support_export_round_trips_and_is_safe() {
    let baseline = stable();
    let export = baseline.support_export(
        ADAPTER_NEGOTIATION_SUPPORT_EXPORT_ID,
        "2026-06-17T00:01:00Z",
    );
    assert!(export.is_export_safe());
    let json = serde_json::to_string(&export).expect("serialize");
    let round: AdapterNegotiationSupportExport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(round, export);
}
