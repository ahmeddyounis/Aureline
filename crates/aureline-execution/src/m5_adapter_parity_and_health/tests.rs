use super::*;

fn packet() -> M5AdapterHealthMatrix {
    current_m5_adapter_health_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_ADAPTER_HEALTH_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_ADAPTER_HEALTH_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_flow_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.flows.len(), packet.adapter_flows.len());
    for &flow in &packet.adapter_flows {
        assert!(
            packet.flow(flow).is_some(),
            "missing row for flow {}",
            flow.as_str()
        );
    }
}

#[test]
fn every_flow_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_flows_gate_consistent());
    for row in &packet.flows {
        assert_eq!(
            row.published_health,
            row.effective_health(),
            "flow {} publishes beyond the gate",
            row.flow_id
        );
        assert_eq!(
            row.health_decision,
            row.required_decision(),
            "flow {} decision diverges from the gate",
            row.flow_id
        );
        assert_eq!(
            row.fallback_reasons,
            row.computed_fallback_reasons(),
            "flow {} fallback reasons diverge from the gate",
            row.flow_id
        );
    }
}

#[test]
fn every_flow_carries_its_own_evidence() {
    let packet = packet();
    for row in &packet.flows {
        assert!(
            row.has_required_evidence(),
            "flow {} is missing required evidence refs",
            row.flow_id
        );
        assert!(
            !row.execution_ref.trim().is_empty(),
            "flow {} has no execution ref to join in support exports",
            row.flow_id
        );
        assert!(
            !row.health_receipt_ref.trim().is_empty(),
            "flow {} has no health receipt for audit reconstruction",
            row.flow_id
        );
    }
}

#[test]
fn conditional_refs_are_present_when_required() {
    let packet = packet();
    for row in &packet.flows {
        if row.adapter_source.requires_snapshot() {
            assert!(
                !row.source_snapshot_ref.trim().is_empty(),
                "flow {} requires a source snapshot ref",
                row.flow_id
            );
        }
        if row.adapter_flow.joins_support_bundle() {
            assert!(
                !row.support_bundle_ref.trim().is_empty(),
                "flow {} requires a support bundle ref",
                row.flow_id
            );
        }
    }
}

#[test]
fn narrowed_flows_offer_a_recovery() {
    let packet = packet();
    for row in &packet.flows {
        if row.health_decision.is_narrowed() {
            assert!(
                row.recovery_path.is_offered(),
                "narrowed flow {} must offer a recovery",
                row.flow_id
            );
        }
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.flows.len(), packet.flows.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_flows_gate_consistent,
        packet.all_flows_gate_consistent()
    );
    assert_eq!(
        projection.authoritative_count,
        packet.authoritative_flows().count()
    );
    assert_eq!(projection.narrowed_count, packet.narrowed_flows().count());
    assert_eq!(projection.withheld_count, packet.withheld_flows().count());
    for (row, export) in packet.flows.iter().zip(projection.flows.iter()) {
        assert_eq!(export.execution_ref, row.execution_ref);
        assert_eq!(export.authoritative, row.is_authoritative());
        assert_eq!(export.downgraded, row.is_downgraded());
        assert_eq!(
            export.joins_support_bundle,
            row.adapter_flow.joins_support_bundle()
        );
        assert_eq!(export.published_health, row.published_health.as_str());
    }
}

#[test]
fn published_healths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<HealthClass> = packet.flows.iter().map(|f| f.published_health).collect();
    for health in HealthClass::ALL {
        assert!(
            present.contains(&health),
            "no flow publishes health {}",
            health.as_str()
        );
    }
}

#[test]
fn health_decisions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<HealthDecision> =
        packet.flows.iter().map(|f| f.health_decision).collect();
    for decision in HealthDecision::ALL {
        assert!(
            present.contains(&decision),
            "no flow exercises decision {}",
            decision.as_str()
        );
    }
}

#[test]
fn adapter_sources_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<AdapterSource> = packet.flows.iter().map(|f| f.adapter_source).collect();
    for source in AdapterSource::ALL {
        assert!(
            present.contains(&source),
            "no flow exercises adapter source {}",
            source.as_str()
        );
    }
}

#[test]
fn freshness_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<FreshnessState> = packet.flows.iter().map(|f| f.freshness).collect();
    for state in FreshnessState::ALL {
        assert!(
            present.contains(&state),
            "no flow exercises freshness state {}",
            state.as_str()
        );
    }
}

#[test]
fn coverage_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CoverageState> = packet.flows.iter().map(|f| f.coverage).collect();
    for state in CoverageState::ALL {
        assert!(
            present.contains(&state),
            "no flow exercises coverage state {}",
            state.as_str()
        );
    }
}

#[test]
fn connection_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ConnectionState> = packet.flows.iter().map(|f| f.connection).collect();
    for state in ConnectionState::ALL {
        assert!(
            present.contains(&state),
            "no flow exercises connection state {}",
            state.as_str()
        );
    }
}

#[test]
fn verification_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<VerificationState> =
        packet.flows.iter().map(|f| f.verification).collect();
    for state in VerificationState::ALL {
        assert!(
            present.contains(&state),
            "no flow exercises verification state {}",
            state.as_str()
        );
    }
}

#[test]
fn recovery_paths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RecoveryPath> = packet.flows.iter().map(|f| f.recovery_path).collect();
    for recovery in RecoveryPath::ALL {
        assert!(
            present.contains(&recovery),
            "no flow exercises recovery path {}",
            recovery.as_str()
        );
    }
}

#[test]
fn fallback_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<FallbackReason> = packet
        .flows
        .iter()
        .flat_map(|f| f.fallback_reasons.iter().copied())
        .collect();
    for reason in FallbackReason::ALL {
        assert!(
            present.contains(&reason),
            "no flow exercises fallback reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn authoritative_flows_are_clean() {
    let packet = packet();
    assert!(
        packet.authoritative_flows().count() > 0,
        "fixture needs an authoritative flow"
    );
    for row in packet.authoritative_flows() {
        assert_eq!(row.capability_floor(), HealthClass::LiveAuthoritative);
        assert!(row.fallback_reasons.is_empty());
        assert!(row.adapter_source.is_live());
        assert_eq!(row.published_health, HealthClass::LiveAuthoritative);
        assert_eq!(row.health_decision, HealthDecision::Publish);
        assert!(!row.is_downgraded());
    }
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        AdapterSource::Imported.health_ceiling(),
        HealthClass::ImportQualified
    );
    assert_eq!(
        AdapterSource::Heuristic.health_ceiling(),
        HealthClass::HeuristicProvisional
    );
    assert_eq!(
        FreshnessState::Stale.health_ceiling(),
        HealthClass::HeuristicProvisional
    );
    assert_eq!(
        FreshnessState::Expired.health_ceiling(),
        HealthClass::Unavailable
    );
    assert_eq!(
        CoverageState::Partial.health_ceiling(),
        HealthClass::ImportQualified
    );
    assert_eq!(
        CoverageState::Absent.health_ceiling(),
        HealthClass::Unavailable
    );
    assert_eq!(
        ConnectionState::Bridged.health_ceiling(),
        HealthClass::HeuristicProvisional
    );
    assert_eq!(
        ConnectionState::Disconnected.health_ceiling(),
        HealthClass::Unavailable
    );
    assert_eq!(
        VerificationState::Unverifiable.health_ceiling(),
        HealthClass::Unavailable
    );
}

#[test]
fn imported_source_never_publishes_authoritative() {
    // The guardrail: an imported or heuristic source can never present as authoritative live
    // truth, no matter how fresh or complete the rest of the flow is.
    let packet = packet();
    for row in &packet.flows {
        if row.adapter_source.requires_snapshot() {
            assert_ne!(
                row.published_health,
                HealthClass::LiveAuthoritative,
                "flow {} publishes authoritative health from an imported/heuristic source",
                row.flow_id
            );
            assert_ne!(row.health_decision, HealthDecision::Publish);
        }
    }
}

#[test]
fn stale_flow_is_downgraded_not_left_green() {
    // A stale snapshot is downgraded automatically rather than quietly remaining green.
    let packet = packet();
    let row = packet
        .flow(AdapterFlow::FrameworkToolingAction)
        .expect("framework-tooling-action row");
    assert!(row.freshness.is_stale_trigger());
    assert!(row.is_downgraded());
    assert!(row
        .fallback_reasons
        .contains(&FallbackReason::StaleSnapshot));
    assert_eq!(row.published_health, HealthClass::HeuristicProvisional);
}

#[test]
fn live_native_flow_publishes_authoritative() {
    // A live native build-event stream that is fresh, complete, connected, and verified
    // publishes live authoritative health — the parity model is not a blanket downgrade.
    let packet = packet();
    let row = packet
        .flow(AdapterFlow::PipelineBuildRun)
        .expect("pipeline-build-run row");
    assert_eq!(row.adapter_source, AdapterSource::Native);
    assert_eq!(row.published_health, HealthClass::LiveAuthoritative);
    assert_eq!(row.health_decision, HealthDecision::Publish);
    assert!(row.fallback_reasons.is_empty());
}

#[test]
fn dead_import_is_withheld() {
    // An expired, absent, disconnected, unverifiable imported snapshot drops to unavailable
    // rather than reading as usable execution truth.
    let packet = packet();
    let row = packet
        .flow(AdapterFlow::SupportBundleJoin)
        .expect("support-bundle-join row");
    assert_eq!(row.published_health, HealthClass::Unavailable);
    assert_eq!(row.health_decision, HealthDecision::Withhold);
    assert!(row.recovery_path.is_offered());
    assert!(!row.support_bundle_ref.trim().is_empty());
}

#[test]
fn incident_join_carries_support_bundle() {
    let packet = packet();
    let row = packet
        .flow(AdapterFlow::IncidentReplay)
        .expect("incident-replay row");
    assert!(row.adapter_flow.joins_support_bundle());
    assert!(!row.support_bundle_ref.trim().is_empty());
    assert!(row
        .fallback_reasons
        .contains(&FallbackReason::HeuristicInference));
}

#[test]
fn validate_flags_overstated_health() {
    let mut packet = packet();
    if let Some(row) = packet
        .flows
        .iter_mut()
        .find(|f| f.effective_health() != HealthClass::LiveAuthoritative)
    {
        row.published_health = HealthClass::LiveAuthoritative;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5AdapterHealthViolation::OverstatedHealth { .. })));
    }
}

#[test]
fn validate_flags_decision_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .flows
        .iter_mut()
        .find(|f| f.health_decision != HealthDecision::Withhold)
    {
        row.health_decision = HealthDecision::Withhold;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5AdapterHealthViolation::DecisionMismatch { .. })));
    }
}

#[test]
fn validate_flags_fallback_reasons_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.flows.iter_mut().find(|f| {
        !f.fallback_reasons
            .contains(&FallbackReason::HeuristicInference)
    }) {
        row.fallback_reasons
            .push(FallbackReason::HeuristicInference);
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5AdapterHealthViolation::FallbackReasonsMismatch { .. })));
    }
}

#[test]
fn validate_flags_missing_support_bundle() {
    let mut packet = packet();
    if let Some(row) = packet
        .flows
        .iter_mut()
        .find(|f| f.adapter_flow.joins_support_bundle())
    {
        row.support_bundle_ref = String::new();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5AdapterHealthViolation::SupportJoinMissing { .. })));
    }
}

#[test]
fn validate_flags_missing_recovery() {
    let mut packet = packet();
    if let Some(row) = packet
        .flows
        .iter_mut()
        .find(|f| f.health_decision.is_narrowed())
    {
        row.recovery_path = RecoveryPath::NoneNeeded;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5AdapterHealthViolation::MissingRecovery { .. })));
    }
}

#[test]
fn validate_flags_missing_flow_row() {
    let mut packet = packet();
    let removed = packet.flows.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5AdapterHealthViolation::MissingFlowRow { .. })));
}

#[test]
fn validate_flags_unclaimed_flow_row() {
    let mut packet = packet();
    packet
        .adapter_flows
        .retain(|f| *f != AdapterFlow::SupportBundleJoin);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5AdapterHealthViolation::UnclaimedFlowRow { .. })));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AdapterHealthViolation::ClosedVocabularyMismatch {
            field: "adapter_flows"
        }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_flows = packet.summary.total_flows.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5AdapterHealthViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(AdapterFlow::PipelineBuildRun.as_str(), "pipeline_build_run");
    assert_eq!(
        AdapterFlow::SupportBundleJoin.as_str(),
        "support_bundle_join"
    );
    assert_eq!(
        HealthClass::LiveAuthoritative.as_str(),
        "live_authoritative"
    );
    assert_eq!(HealthClass::Unavailable.as_str(), "unavailable");
    assert_eq!(AdapterSource::ProtocolBacked.as_str(), "protocol_backed");
    assert_eq!(
        AdapterSource::StructuredImport.as_str(),
        "structured_import"
    );
    assert_eq!(FreshnessState::Expired.as_str(), "expired");
    assert_eq!(CoverageState::Degraded.as_str(), "degraded");
    assert_eq!(ConnectionState::Bridged.as_str(), "bridged");
    assert_eq!(VerificationState::Unverifiable.as_str(), "unverifiable");
    assert_eq!(
        RecoveryPath::AwaitLiveAdapter.as_str(),
        "await_live_adapter"
    );
    assert_eq!(RecoveryPath::NoneNeeded.as_str(), "none");
    assert_eq!(
        FallbackReason::ConnectionUnstable.as_str(),
        "connection_unstable"
    );
    assert_eq!(HealthDecision::Provisional.as_str(), "provisional");
}

#[test]
fn health_rank_orders_low_to_high() {
    assert!(HealthClass::Unavailable.rank() < HealthClass::HeuristicProvisional.rank());
    assert!(HealthClass::HeuristicProvisional.rank() < HealthClass::ImportQualified.rank());
    assert!(HealthClass::ImportQualified.rank() < HealthClass::LiveAuthoritative.rank());
    assert_eq!(
        HealthClass::LiveAuthoritative.min(HealthClass::HeuristicProvisional),
        HealthClass::HeuristicProvisional
    );
}
