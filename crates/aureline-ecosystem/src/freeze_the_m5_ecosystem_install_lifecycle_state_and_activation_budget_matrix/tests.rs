use super::*;

fn packet() -> M5EcosystemGovernanceMatrix {
    current_m5_ecosystem_governance_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_ECOSYSTEM_GOVERNANCE_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_ECOSYSTEM_GOVERNANCE_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_family_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.families.len(), packet.artifact_families.len());
    for &family in &packet.artifact_families {
        assert!(
            packet.family(family).is_some(),
            "missing row for family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_family_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_families_gate_consistent());
    for row in &packet.families {
        assert_eq!(
            row.published_support_class,
            row.effective_support_class(),
            "family {} publishes beyond the gate",
            row.family_id
        );
        assert_eq!(
            row.promotion_decision,
            row.required_decision(),
            "family {} decision diverges from the gate",
            row.family_id
        );
        assert_eq!(
            row.narrowing_reasons,
            row.computed_narrowing_reasons(),
            "family {} narrowing reasons diverge from the gate",
            row.family_id
        );
    }
}

#[test]
fn every_family_carries_its_own_evidence() {
    let packet = packet();
    for row in &packet.families {
        assert!(
            row.has_required_evidence(),
            "family {} is missing required evidence refs",
            row.family_id
        );
        assert!(
            !row.provenance_ref.trim().is_empty(),
            "family {} has no provenance ref",
            row.family_id
        );
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.families.len(), packet.families.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_families_gate_consistent,
        packet.all_families_gate_consistent()
    );
    assert_eq!(
        projection.promotable_count,
        packet.promotable_families().count()
    );
    assert_eq!(
        projection.narrowed_count,
        packet.narrowed_families().count()
    );
    assert_eq!(
        projection.failed_promotion_count,
        packet.failed_promotion_families().count()
    );
}

#[test]
fn published_support_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<SupportClass> = packet
        .families
        .iter()
        .map(|f| f.published_support_class)
        .collect();
    for support in SupportClass::ALL {
        assert!(
            present.contains(&support),
            "no family publishes support class {}",
            support.as_str()
        );
    }
}

#[test]
fn promotion_decisions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<PromotionDecision> = packet
        .families
        .iter()
        .map(|f| f.promotion_decision)
        .collect();
    for decision in PromotionDecision::ALL {
        assert!(
            present.contains(&decision),
            "no family exercises decision {}",
            decision.as_str()
        );
    }
}

#[test]
fn runtime_origins_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RuntimeOrigin> =
        packet.families.iter().map(|f| f.runtime_origin).collect();
    for origin in RuntimeOrigin::ALL {
        assert!(
            present.contains(&origin),
            "no family exercises runtime origin {}",
            origin.as_str()
        );
    }
}

#[test]
fn compatibility_labels_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CompatibilityLabel> = packet
        .families
        .iter()
        .map(|f| f.compatibility_label)
        .collect();
    for label in CompatibilityLabel::ALL {
        assert!(
            present.contains(&label),
            "no family exercises compatibility label {}",
            label.as_str()
        );
    }
}

#[test]
fn permission_manifest_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<PermissionManifestState> = packet
        .families
        .iter()
        .map(|f| f.permission_manifest_state)
        .collect();
    for state in PermissionManifestState::ALL {
        assert!(
            present.contains(&state),
            "no family exercises permission state {}",
            state.as_str()
        );
    }
}

#[test]
fn activation_budget_bands_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ActivationBudgetBand> = packet
        .families
        .iter()
        .map(|f| f.activation_budget_band)
        .collect();
    for band in ActivationBudgetBand::ALL {
        assert!(
            present.contains(&band),
            "no family exercises activation budget band {}",
            band.as_str()
        );
    }
}

#[test]
fn lifecycle_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<LifecycleState> =
        packet.families.iter().map(|f| f.lifecycle_state).collect();
    for state in LifecycleState::ALL {
        assert!(
            present.contains(&state),
            "no family exercises lifecycle state {}",
            state.as_str()
        );
    }
}

#[test]
fn freshness_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EvidenceFreshness> = packet
        .families
        .iter()
        .map(|f| f.evidence_freshness)
        .collect();
    for freshness in EvidenceFreshness::ALL {
        assert!(
            present.contains(&freshness),
            "no family exercises freshness {}",
            freshness.as_str()
        );
    }
}

#[test]
fn rollback_postures_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RollbackPosture> =
        packet.families.iter().map(|f| f.rollback_posture).collect();
    for posture in RollbackPosture::ALL {
        assert!(
            present.contains(&posture),
            "no family exercises rollback posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn narrowing_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<NarrowingReason> = packet
        .families
        .iter()
        .flat_map(|f| f.narrowing_reasons.iter().copied())
        .collect();
    for reason in NarrowingReason::ALL {
        assert!(
            present.contains(&reason),
            "no family exercises narrowing reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn promotable_families_are_clean() {
    let packet = packet();
    assert!(
        packet.promotable_families().count() > 0,
        "fixture needs a fully-supported family"
    );
    for row in packet.promotable_families() {
        assert!(row.evidence_freshness.is_current());
        assert_eq!(row.capability_floor(), SupportClass::FullySupported);
        assert!(row.narrowing_reasons.is_empty());
        assert_eq!(row.published_support_class, SupportClass::FullySupported);
        assert_eq!(row.promotion_decision, PromotionDecision::Promote);
    }
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        RuntimeOrigin::UnsignedSideLoaded.support_ceiling(),
        SupportClass::CommunitySupported
    );
    assert_eq!(
        RuntimeOrigin::BridgeRuntime.support_ceiling(),
        SupportClass::BestEffortSupported
    );
    assert_eq!(
        EvidenceFreshness::Stale.support_ceiling(),
        SupportClass::BestEffortSupported
    );
    assert_eq!(
        EvidenceFreshness::Expired.support_ceiling(),
        SupportClass::CommunitySupported
    );
    assert_eq!(
        PermissionManifestState::ExpandedUnreviewed.support_ceiling(),
        SupportClass::CommunitySupported
    );
    assert_eq!(
        ActivationBudgetBand::OverBudget.support_ceiling(),
        SupportClass::CommunitySupported
    );
    assert_eq!(
        CompatibilityLabel::UnsupportedOnTarget.support_ceiling(),
        SupportClass::Unsupported
    );
    assert_eq!(
        RollbackPosture::Irreversible.support_ceiling(),
        SupportClass::CommunitySupported
    );
    assert_eq!(
        LifecycleState::Quarantined.support_ceiling(),
        SupportClass::Unsupported
    );
    assert_eq!(
        LifecycleState::RolledBack.support_ceiling(),
        SupportClass::Unsupported
    );
    assert_eq!(
        LifecycleState::Retired.support_ceiling(),
        SupportClass::CommunitySupported
    );
}

#[test]
fn validate_flags_overstated_published_support_class() {
    let mut packet = packet();
    if let Some(row) = packet
        .families
        .iter_mut()
        .find(|f| f.effective_support_class() != SupportClass::FullySupported)
    {
        row.published_support_class = SupportClass::FullySupported;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5EcosystemGovernanceViolation::OverstatedPublishedSupportClass { .. }
        )));
    }
}

#[test]
fn validate_flags_decision_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .families
        .iter_mut()
        .find(|f| f.promotion_decision != PromotionDecision::FailPromotion)
    {
        row.promotion_decision = PromotionDecision::FailPromotion;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5EcosystemGovernanceViolation::DecisionMismatch { .. })));
    }
}

#[test]
fn validate_flags_narrowing_reasons_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.families.iter_mut().find(|f| {
        !f.narrowing_reasons
            .contains(&NarrowingReason::EvidenceStale)
    }) {
        row.narrowing_reasons.push(NarrowingReason::EvidenceStale);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5EcosystemGovernanceViolation::NarrowingReasonsMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_family_row() {
    let mut packet = packet();
    let removed = packet.families.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5EcosystemGovernanceViolation::MissingFamilyRow { .. })));
}

#[test]
fn validate_flags_unclaimed_family_row() {
    let mut packet = packet();
    packet
        .artifact_families
        .retain(|f| *f != ArtifactFamily::MirroredRegistryVariant);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5EcosystemGovernanceViolation::UnclaimedFamilyRow { .. })));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5EcosystemGovernanceViolation::ClosedVocabularyMismatch {
            field: "artifact_families"
        }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_families = packet.summary.total_families.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5EcosystemGovernanceViolation::SummaryMismatch));
}

#[test]
fn fail_promotion_withholds_the_claim() {
    let packet = packet();
    let failed: Vec<_> = packet.failed_promotion_families().collect();
    assert!(!failed.is_empty(), "fixture needs a withheld family");
    for row in failed {
        assert_eq!(row.published_support_class, SupportClass::Unsupported);
        assert_eq!(row.promotion_decision, PromotionDecision::FailPromotion);
    }
}

#[test]
fn side_loaded_never_inherits_first_party_trust() {
    let packet = packet();
    let row = packet
        .family(ArtifactFamily::SideLoadedPackage)
        .expect("side-loaded row");
    assert!(row.runtime_origin.is_provenance_unverified_trigger());
    assert!(row.published_support_class.rank() < SupportClass::FullySupported.rank());
    assert!(row
        .narrowing_reasons
        .contains(&NarrowingReason::ProvenanceUnverified));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        ArtifactFamily::FirstPartyFrameworkPack.as_str(),
        "first_party_framework_pack"
    );
    assert_eq!(
        ArtifactFamily::MirroredRegistryVariant.as_str(),
        "mirrored_registry_variant"
    );
    assert_eq!(SupportClass::FullySupported.as_str(), "fully_supported");
    assert_eq!(
        RuntimeOrigin::UnsignedSideLoaded.as_str(),
        "unsigned_side_loaded"
    );
    assert_eq!(ActivationBudgetBand::OverBudget.as_str(), "over_budget");
    assert_eq!(LifecycleState::Quarantined.as_str(), "quarantined");
    assert_eq!(
        NarrowingReason::ActivationBudgetExceeded.as_str(),
        "activation_budget_exceeded"
    );
    assert_eq!(PromotionDecision::FailPromotion.as_str(), "fail_promotion");
}

#[test]
fn support_rank_orders_low_to_high() {
    assert!(SupportClass::Unsupported.rank() < SupportClass::CommunitySupported.rank());
    assert!(SupportClass::CommunitySupported.rank() < SupportClass::BestEffortSupported.rank());
    assert!(SupportClass::BestEffortSupported.rank() < SupportClass::FullySupported.rank());
    assert_eq!(
        SupportClass::FullySupported.min(SupportClass::BestEffortSupported),
        SupportClass::BestEffortSupported
    );
}
