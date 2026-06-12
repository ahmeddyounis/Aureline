use super::*;

fn packet() -> M5EntryBundleGovernanceMatrix {
    current_m5_entry_bundle_governance_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_ENTRY_BUNDLE_GOVERNANCE_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_ENTRY_BUNDLE_GOVERNANCE_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_lane_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.lane_rows.len(), packet.lanes.len());
    for &lane in &packet.lanes {
        assert!(
            packet.lane_row(lane).is_some(),
            "missing row for lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn every_lane_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_lanes_gate_consistent());
    for row in &packet.lane_rows {
        assert_eq!(
            row.published_assurance,
            row.effective_assurance(),
            "lane {} publishes beyond the gate",
            row.lane_id
        );
        assert_eq!(
            row.admission_outcome,
            row.required_outcome(),
            "lane {} outcome diverges from the gate",
            row.lane_id
        );
        assert_eq!(
            row.downgrade_reasons,
            row.computed_downgrade_reasons(),
            "lane {} downgrade reasons diverge from the gate",
            row.lane_id
        );
    }
}

#[test]
fn every_lane_carries_its_own_evidence() {
    let packet = packet();
    for row in &packet.lane_rows {
        assert!(
            row.has_required_evidence(),
            "lane {} is missing required evidence refs",
            row.lane_id
        );
    }
}

#[test]
fn every_lane_binds_to_its_canonical_source_packet() {
    let packet = packet();
    for row in &packet.lane_rows {
        assert_eq!(
            row.packet_ref,
            row.lane.source_packet(),
            "lane {} governs a packet other than its canonical source",
            row.lane_id
        );
    }
}

#[test]
fn entry_verbs_stay_distinct_per_lane() {
    // Clone, open, import, and resume remain distinct verbs pinned to their lanes.
    let packet = packet();
    for row in &packet.lane_rows {
        assert_eq!(
            row.entry_verb,
            row.lane.entry_verb(),
            "lane {} carries a verb other than its pinned verb",
            row.lane_id
        );
    }
}

#[test]
fn narrowed_lanes_offer_a_recovery_and_caveats() {
    let packet = packet();
    for row in &packet.lane_rows {
        if row.admission_outcome.is_narrowed() {
            assert!(
                row.downgrade_path.is_offered(),
                "narrowed lane {} must offer a recovery path",
                row.lane_id
            );
            assert!(
                !row.caveats.is_empty(),
                "narrowed lane {} must list a caveat",
                row.lane_id
            );
            assert!(
                !row.stale_or_missing_fields.is_empty(),
                "narrowed lane {} must name a stale or narrowing field",
                row.lane_id
            );
        }
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.lanes.len(), packet.lane_rows.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_lanes_gate_consistent,
        packet.all_lanes_gate_consistent()
    );
    assert_eq!(projection.verified_count, packet.verified_lanes().count());
    assert_eq!(projection.narrowed_count, packet.narrowed_lanes().count());
    assert_eq!(projection.refused_count, packet.refused_lanes().count());
    for (row, export) in packet.lane_rows.iter().zip(projection.lanes.iter()) {
        assert_eq!(export.packet_ref, row.packet_ref);
        assert_eq!(export.entry_verb, row.entry_verb.as_str());
        assert_eq!(export.verified, row.is_verified());
        assert_eq!(export.downgraded, row.is_downgraded());
        assert_eq!(export.trust_sensitive, row.lane.is_trust_sensitive());
        assert_eq!(export.published_assurance, row.published_assurance.as_str());
        assert_eq!(export.deferred_setup_count, row.deferred_setup_count);
        assert_eq!(export.missing_root_count, row.missing_root_count);
    }
}

#[test]
fn published_labels_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EntryAssurance> = packet
        .lane_rows
        .iter()
        .map(|c| c.published_assurance)
        .collect();
    for label in EntryAssurance::ALL {
        assert!(
            present.contains(&label),
            "no lane publishes label {}",
            label.as_str()
        );
    }
}

#[test]
fn admission_outcomes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<AdmissionOutcome> = packet
        .lane_rows
        .iter()
        .map(|c| c.admission_outcome)
        .collect();
    for outcome in AdmissionOutcome::ALL {
        assert!(
            present.contains(&outcome),
            "no lane exercises outcome {}",
            outcome.as_str()
        );
    }
}

#[test]
fn entry_verbs_are_exhaustive() {
    // Every distinct verb — clone, open, import, resume, install — is exercised.
    let packet = packet();
    let present: BTreeSet<EntryVerb> = packet.lane_rows.iter().map(|c| c.entry_verb).collect();
    for verb in EntryVerb::ALL {
        assert!(
            present.contains(&verb),
            "no lane exercises verb {}",
            verb.as_str()
        );
    }
}

#[test]
fn source_trust_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<SourceTrust> = packet.lane_rows.iter().map(|c| c.source_trust).collect();
    for state in SourceTrust::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises source trust {}",
            state.as_str()
        );
    }
}

#[test]
fn archetype_confidence_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ArchetypeConfidence> = packet
        .lane_rows
        .iter()
        .map(|c| c.archetype_confidence)
        .collect();
    for state in ArchetypeConfidence::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises archetype confidence {}",
            state.as_str()
        );
    }
}

#[test]
fn root_resolution_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RootResolution> =
        packet.lane_rows.iter().map(|c| c.root_resolution).collect();
    for state in RootResolution::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises root resolution {}",
            state.as_str()
        );
    }
}

#[test]
fn restore_fidelities_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RestoreFidelity> = packet
        .lane_rows
        .iter()
        .map(|c| c.restore_fidelity)
        .collect();
    for fidelity in RestoreFidelity::ALL {
        assert!(
            present.contains(&fidelity),
            "no lane exercises restore fidelity {}",
            fidelity.as_str()
        );
    }
}

#[test]
fn bundle_scorecard_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<BundleScorecard> = packet
        .lane_rows
        .iter()
        .map(|c| c.bundle_scorecard)
        .collect();
    for state in BundleScorecard::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises bundle scorecard {}",
            state.as_str()
        );
    }
}

#[test]
fn entry_topology_support_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EntryTopologySupport> = packet
        .lane_rows
        .iter()
        .map(|c| c.entry_topology_support)
        .collect();
    for state in EntryTopologySupport::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises entry topology support {}",
            state.as_str()
        );
    }
}

#[test]
fn setup_queue_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<SetupQueueClass> = packet
        .lane_rows
        .iter()
        .map(|c| c.setup_queue_class)
        .collect();
    for class in SetupQueueClass::ALL {
        assert!(
            present.contains(&class),
            "no lane exercises setup queue class {}",
            class.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DowngradeReason> = packet
        .lane_rows
        .iter()
        .flat_map(|c| c.downgrade_reasons.iter().copied())
        .collect();
    for reason in DowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no lane exercises downgrade reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn verified_lanes_are_whole() {
    let packet = packet();
    assert!(
        packet.verified_lanes().count() >= 2,
        "fixture needs at least two verified lanes to prove the gate is not a blanket downgrade"
    );
    for row in packet.verified_lanes() {
        assert_eq!(row.capability_floor(), EntryAssurance::Verified);
        assert_eq!(row.source_trust, SourceTrust::FirstParty);
        assert_eq!(row.archetype_confidence, ArchetypeConfidence::Confirmed);
        assert_eq!(row.setup_queue_class, SetupQueueClass::Ready);
        assert!(row.downgrade_reasons.is_empty());
        assert!(row.caveats.is_empty());
        assert!(row.stale_or_missing_fields.is_empty());
        assert!(!row.downgrade_path.is_offered());
        assert!(!row.supported_scopes.is_empty());
        assert!(!row.is_downgraded());
        assert_eq!(row.deferred_setup_count, 0);
        assert_eq!(row.missing_root_count, 0);
    }
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        SourceTrust::TrustedRemote.assurance_ceiling(),
        EntryAssurance::Bounded
    );
    assert_eq!(
        SourceTrust::Untrusted.assurance_ceiling(),
        EntryAssurance::Withheld
    );
    assert_eq!(
        ArchetypeConfidence::Probable.assurance_ceiling(),
        EntryAssurance::Bounded
    );
    assert_eq!(
        ArchetypeConfidence::Mixed.assurance_ceiling(),
        EntryAssurance::RetestPending
    );
    assert_eq!(
        ArchetypeConfidence::Undetected.assurance_ceiling(),
        EntryAssurance::Withheld
    );
    assert_eq!(
        RestoreFidelity::Degraded.assurance_ceiling(),
        EntryAssurance::RetestPending
    );
    assert_eq!(
        BundleScorecard::Stale.assurance_ceiling(),
        EntryAssurance::RetestPending
    );
    assert_eq!(
        EntryTopologySupport::Unsupported.assurance_ceiling(),
        EntryAssurance::Withheld
    );
    assert_eq!(
        RootResolution::Missing.assurance_ceiling(),
        EntryAssurance::Withheld
    );
}

#[test]
fn clone_lane_is_bounded_not_left_verified() {
    // A probable, trusted-remote clone is narrowed to its slice rather than widening trust to a
    // verified first-party label.
    let packet = packet();
    let row = packet
        .lane_row(EntryBundleLane::SourceAcquisition)
        .expect("source-acquisition row");
    assert!(row.source_trust.is_unverified_trigger());
    assert!(row.is_downgraded());
    assert!(row.lane.is_trust_sensitive());
    assert!(row
        .downgrade_reasons
        .contains(&DowngradeReason::ProbableOrMixedDetection));
    assert_eq!(row.published_assurance, EntryAssurance::Bounded);
    assert!(row.deferred_setup_count > 0);
}

#[test]
fn open_lane_publishes() {
    // A first-party, confirmed, exact open publishes verified — the gate is not a blanket
    // downgrade.
    let packet = packet();
    let row = packet
        .lane_row(EntryBundleLane::ProjectOpen)
        .expect("project-open row");
    assert_eq!(row.published_assurance, EntryAssurance::Verified);
    assert_eq!(row.admission_outcome, AdmissionOutcome::AdmitFull);
    assert!(row.downgrade_reasons.is_empty());
}

#[test]
fn missing_root_admission_is_withheld() {
    // An untrusted, undetected, root-missing admission drops to withheld rather than implying a
    // routable workspace.
    let packet = packet();
    let row = packet
        .lane_row(EntryBundleLane::WorkspaceAdmission)
        .expect("workspace-admission row");
    assert_eq!(row.published_assurance, EntryAssurance::Withheld);
    assert_eq!(row.admission_outcome, AdmissionOutcome::Refuse);
    assert!(row.lane.is_trust_sensitive());
    assert!(row.downgrade_path.is_offered());
    assert!(row.supported_scopes.is_empty());
    assert_eq!(row.downgrade_reasons, DowngradeReason::ALL.to_vec());
    assert_eq!(row.setup_queue_class, SetupQueueClass::MissingRoot);
    assert!(row.missing_root_count > 0);
}

#[test]
fn trust_sensitive_lanes_never_publish_above_their_evidence() {
    // A clone, import, resume, bundle, or admission lane narrows safely instead of inheriting a
    // broader stable claim.
    let packet = packet();
    for row in &packet.lane_rows {
        if row.lane.is_trust_sensitive() {
            assert_eq!(
                row.published_assurance,
                row.effective_assurance(),
                "trust-sensitive lane {} publishes beyond its evidence",
                row.lane_id
            );
        }
    }
}

#[test]
fn setup_queue_classes_stay_distinct() {
    // A ready entry hides nothing; setup-later and blocked-on-setup always report a non-zero
    // deferred count; missing-root always reports a non-zero missing-root count.
    let packet = packet();
    for row in &packet.lane_rows {
        match row.setup_queue_class {
            SetupQueueClass::Ready => {
                assert_eq!(row.deferred_setup_count, 0);
                assert_eq!(row.missing_root_count, 0);
            }
            SetupQueueClass::SetupLater | SetupQueueClass::BlockedOnSetup => {
                assert!(row.deferred_setup_count > 0)
            }
            SetupQueueClass::MissingRoot => assert!(row.missing_root_count > 0),
        }
    }
}

#[test]
fn workspace_identities_are_namespaced() {
    let packet = packet();
    assert!(!packet.workspace_identity_scheme.trim().is_empty());
    for row in &packet.lane_rows {
        assert!(
            !row.root_id_namespace.trim().is_empty(),
            "lane {} has no root identity namespace",
            row.lane_id
        );
        assert!(
            !row.bundle_id_namespace.trim().is_empty(),
            "lane {} has no bundle identity namespace",
            row.lane_id
        );
    }
}

#[test]
fn validate_flags_overstated_claim() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.effective_assurance() != EntryAssurance::Verified)
    {
        row.published_assurance = EntryAssurance::Verified;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5EntryBundleGovernanceViolation::OverstatedClaim { .. })));
    }
}

#[test]
fn validate_flags_outcome_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.admission_outcome != AdmissionOutcome::Refuse)
    {
        row.admission_outcome = AdmissionOutcome::Refuse;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5EntryBundleGovernanceViolation::OutcomeMismatch { .. })));
    }
}

#[test]
fn validate_flags_downgrade_reasons_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| !c.downgrade_reasons.contains(&DowngradeReason::MissingRoots))
    {
        row.downgrade_reasons.push(DowngradeReason::MissingRoots);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5EntryBundleGovernanceViolation::DowngradeReasonsMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_source_packet_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.lane_rows.first_mut() {
        row.packet_ref = "artifacts/workspace/m5/not-the-source.json".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5EntryBundleGovernanceViolation::SourcePacketMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_entry_verb_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.entry_verb != EntryVerb::Resume)
    {
        row.entry_verb = EntryVerb::Resume;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5EntryBundleGovernanceViolation::EntryVerbMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_downgrade_path() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.admission_outcome.is_narrowed())
    {
        row.downgrade_path = DowngradePath::NoneNeeded;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5EntryBundleGovernanceViolation::MissingDowngradePath { .. }
        )));
    }
}

#[test]
fn validate_flags_setup_queue_count_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.setup_queue_class == SetupQueueClass::MissingRoot)
    {
        row.missing_root_count = 0;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5EntryBundleGovernanceViolation::SetupQueueCountMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_lane_row() {
    let mut packet = packet();
    let removed = packet.lane_rows.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5EntryBundleGovernanceViolation::MissingLaneRow { .. })));
}

#[test]
fn validate_flags_unclaimed_lane_row() {
    let mut packet = packet();
    packet
        .lanes
        .retain(|l| *l != EntryBundleLane::WorkspaceAdmission);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5EntryBundleGovernanceViolation::UnclaimedLaneRow { .. })));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5EntryBundleGovernanceViolation::ClosedVocabularyMismatch { field: "lanes" }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_lanes = packet.summary.total_lanes.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5EntryBundleGovernanceViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(EntryBundleLane::WorkflowBundle.as_str(), "workflow_bundle");
    assert_eq!(
        EntryBundleLane::WorkspaceAdmission.as_str(),
        "workspace_admission"
    );
    assert_eq!(EntryVerb::Clone.as_str(), "clone");
    assert_eq!(EntryVerb::Install.as_str(), "install");
    assert_eq!(EntryAssurance::Verified.as_str(), "verified");
    assert_eq!(EntryAssurance::Withheld.as_str(), "withheld");
    assert_eq!(SourceTrust::UnverifiedRemote.as_str(), "unverified_remote");
    assert_eq!(ArchetypeConfidence::Mixed.as_str(), "mixed");
    assert_eq!(
        RootResolution::ProbableMultiRoot.as_str(),
        "probable_multi_root"
    );
    assert_eq!(RestoreFidelity::Degraded.as_str(), "degraded");
    assert_eq!(BundleScorecard::Stale.as_str(), "stale");
    assert_eq!(EntryTopologySupport::Experimental.as_str(), "experimental");
    assert_eq!(SetupQueueClass::MissingRoot.as_str(), "missing_root");
    assert_eq!(DowngradePath::VerifySource.as_str(), "verify_source");
    assert_eq!(DowngradePath::NoneNeeded.as_str(), "none");
    assert_eq!(
        DowngradeReason::ProbableOrMixedDetection.as_str(),
        "probable_or_mixed_detection"
    );
    assert_eq!(AdmissionOutcome::AdmitRetest.as_str(), "admit_retest");
}

#[test]
fn assurance_rank_orders_low_to_high() {
    assert!(EntryAssurance::Withheld.rank() < EntryAssurance::RetestPending.rank());
    assert!(EntryAssurance::RetestPending.rank() < EntryAssurance::Bounded.rank());
    assert!(EntryAssurance::Bounded.rank() < EntryAssurance::Verified.rank());
    assert_eq!(
        EntryAssurance::Verified.min(EntryAssurance::RetestPending),
        EntryAssurance::RetestPending
    );
}

#[test]
fn source_packets_point_at_real_entry_packets() {
    for lane in EntryBundleLane::ALL {
        assert!(
            lane.source_packet().starts_with("artifacts/"),
            "lane {} does not bind to a checked-in entry packet",
            lane.as_str()
        );
    }
}

#[test]
fn underqualified_rows_narrow_to_bounded_or_retest_pending() {
    // Underqualified entry/bundle rows narrow automatically to bounded or retest-pending labels
    // before publication rather than staying verified.
    let packet = packet();
    for row in &packet.lane_rows {
        if row.is_downgraded() && row.published_assurance != EntryAssurance::Withheld {
            assert!(
                matches!(
                    row.published_assurance,
                    EntryAssurance::Bounded | EntryAssurance::RetestPending
                ),
                "downgraded lane {} should narrow to bounded or retest-pending",
                row.lane_id
            );
        }
    }
}
