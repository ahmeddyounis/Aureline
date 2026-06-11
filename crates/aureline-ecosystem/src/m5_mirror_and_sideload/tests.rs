use super::*;

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::{
    current_m5_ecosystem_governance_matrix, ArtifactFamily,
};

fn packet() -> M5MirrorAndSideload {
    current_m5_mirror_and_sideload().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_MIRROR_AND_SIDELOAD_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_MIRROR_AND_SIDELOAD_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_records() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_packet_disposition_is_recomputed() {
    let packet = packet();
    assert!(!packet.review_packets.is_empty());
    for p in &packet.review_packets {
        assert_eq!(
            p.review_disposition,
            p.computed_review_disposition(),
            "packet {} disposition diverges from the recomputed value",
            p.review_id
        );
        assert_eq!(
            p.continuity_signals,
            p.computed_continuity_signals(),
            "packet {} signals diverge from the recomputed set",
            p.review_id
        );
    }
}

#[test]
fn every_packet_is_export_safe() {
    // The lane guardrail: a mirror, private, manual-import, or air-gapped review must
    // carry the same backing refs as a public-registry install.
    let packet = packet();
    for p in &packet.review_packets {
        assert!(
            p.is_export_safe(),
            "packet {} drops a backing ref",
            p.review_id
        );
    }
}

#[test]
fn mirror_and_sideload_lanes_carry_full_fields() {
    // Parity proof: a mirrored/private/side-loaded packet exposes every fact field a
    // public-registry install does — no reduced or special-case disclosure.
    let packet = packet();
    let public = packet
        .review_packet("review:first_party_framework_pack_public")
        .expect("public baseline exists");
    let mirror = packet
        .review_packet("review:mirrored_registry_variant_mirror")
        .expect("mirror variant exists");
    // Same package identity, distribution that differs only by lane.
    assert_eq!(public.package_kind, ArtifactFamily::FirstPartyFrameworkPack);
    assert_eq!(public.source_class, mirror.source_class);
    assert_eq!(public.publisher_ref, mirror.publisher_ref);
    assert_eq!(public.signing_root_ref, mirror.signing_root_ref);
    assert_eq!(public.namespace_ref, mirror.namespace_ref);
    // The public lane proceeds; the mirror lane keeps the fields but widens disclosure.
    assert_eq!(public.review_disposition, ReviewDisposition::Proceed);
    assert_eq!(mirror.review_disposition, ReviewDisposition::ReviewRequired);
    assert!(mirror.is_export_safe());
}

#[test]
fn guardrails_force_blocked_disposition() {
    // A mirror or side-load can never bypass permission, compatibility, rollback,
    // publisher-continuity, or quarantine guardrails.
    let packet = packet();
    for p in &packet.review_packets {
        let has_guardrail = p.continuity_signals.iter().any(|s| {
            matches!(
                s,
                ContinuitySignal::PermissionExpansionUnreviewed
                    | ContinuitySignal::CompatibilityUnsupported
                    | ContinuitySignal::RollbackIncomplete
                    | ContinuitySignal::Quarantined
                    | ContinuitySignal::SigningRootChanged
                    | ContinuitySignal::NamespaceDiscontinuous
                    | ContinuitySignal::Unmaintained
                    | ContinuitySignal::PublisherDiscontinuous
                    | ContinuitySignal::ProvenanceUnverifiable
            )
        });
        if has_guardrail {
            assert_eq!(
                p.review_disposition,
                ReviewDisposition::Blocked,
                "packet {} hits a guardrail but is not blocked",
                p.review_id
            );
        }
    }
}

#[test]
fn side_loaded_packets_still_block_on_guardrails() {
    let packet = packet();
    let side = packet
        .review_packet("review:side_loaded_package_manual")
        .expect("side-loaded packet exists");
    assert!(side.acquisition_channel.is_side_loaded());
    assert_eq!(side.review_disposition, ReviewDisposition::Blocked);
    assert!(side.is_export_safe());
    let airgap = packet
        .review_packet("review:bridge_backed_package_airgap")
        .expect("air-gapped packet exists");
    assert_eq!(
        airgap.acquisition_channel,
        AcquisitionChannel::AirGappedImport
    );
    assert_eq!(airgap.review_disposition, ReviewDisposition::Blocked);
    assert!(airgap.is_export_safe());
}

#[test]
fn every_evaluation_is_recomputed() {
    let packet = packet();
    assert!(!packet.policy_evaluations.is_empty());
    for e in &packet.policy_evaluations {
        let p = packet.review_packet(&e.review_ref).unwrap_or_else(|| {
            panic!("evaluation {} references a missing packet", e.evaluation_id)
        });
        assert_eq!(
            e.matched_filter_refs,
            packet.matching_filter_ids(p),
            "{}",
            e.evaluation_id
        );
        assert_eq!(
            e.strongest_effect,
            packet.strongest_effect(p),
            "{}",
            e.evaluation_id
        );
        assert_eq!(
            e.gate_decision,
            packet.computed_gate_decision(p),
            "{}",
            e.evaluation_id
        );
    }
}

#[test]
fn policy_cannot_loosen_below_review_disposition() {
    // A blocked review can never be allowed by policy; an approval-required review can
    // never drop to allowed.
    let packet = packet();
    for e in &packet.policy_evaluations {
        let p = packet.review_packet(&e.review_ref).expect("packet exists");
        assert!(
            e.gate_decision.rank() >= p.review_disposition.as_gate_decision().rank(),
            "evaluation {} loosened below the review disposition",
            e.evaluation_id
        );
    }
}

#[test]
fn policy_can_tighten_above_review_disposition() {
    // The over-budget block tightens a review that was only review-required at the
    // disposition level into a hard block.
    let packet = packet();
    let airgap = packet
        .review_packet("review:bridge_backed_package_airgap")
        .expect("packet exists");
    let eval = packet
        .policy_evaluations
        .iter()
        .find(|e| e.review_ref == airgap.review_id)
        .expect("evaluation exists");
    assert_eq!(eval.strongest_effect, PolicyEffect::Block);
    assert_eq!(eval.gate_decision, PolicyGateDecision::Blocked);
}

#[test]
fn export_projection_reflects_packets() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.review_packets.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_packets_consistent,
        packet.all_records_consistent()
    );
    for row in &projection.rows {
        let p = packet
            .review_packet(&row.review_id)
            .expect("export row resolves");
        assert_eq!(row.review_disposition, p.review_disposition.as_str());
        assert_eq!(row.export_safe, p.is_export_safe());
    }
}

#[test]
fn all_records_are_consistent() {
    let packet = packet();
    assert!(packet.all_records_consistent());
}

#[test]
fn every_package_kind_is_represented() {
    let packet = packet();
    let present: BTreeSet<ArtifactFamily> = packet
        .review_packets
        .iter()
        .map(|p| p.package_kind)
        .collect();
    for kind in ArtifactFamily::ALL {
        assert!(
            present.contains(&kind),
            "no review packet exercises package kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn every_acquisition_channel_is_represented() {
    let packet = packet();
    let present: BTreeSet<AcquisitionChannel> = packet
        .review_packets
        .iter()
        .map(|p| p.acquisition_channel)
        .collect();
    for channel in AcquisitionChannel::ALL {
        assert!(
            present.contains(&channel),
            "no review packet exercises acquisition channel {}",
            channel.as_str()
        );
    }
}

#[test]
fn every_policy_dimension_is_exercised() {
    let packet = packet();
    let present: BTreeSet<PolicyDimension> =
        packet.policy_filters.iter().map(|f| f.dimension).collect();
    for dimension in PolicyDimension::ALL {
        assert!(
            present.contains(&dimension),
            "no policy filter exercises dimension {}",
            dimension.as_str()
        );
    }
}

#[test]
fn every_review_disposition_is_represented() {
    let packet = packet();
    let present: BTreeSet<ReviewDisposition> = packet
        .review_packets
        .iter()
        .map(|p| p.review_disposition)
        .collect();
    for disposition in ReviewDisposition::ALL {
        assert!(
            present.contains(&disposition),
            "no review packet exercises disposition {}",
            disposition.as_str()
        );
    }
}

#[test]
fn every_gate_decision_is_represented() {
    let packet = packet();
    let present: BTreeSet<PolicyGateDecision> = packet
        .policy_evaluations
        .iter()
        .map(|e| e.gate_decision)
        .collect();
    for decision in PolicyGateDecision::ALL {
        assert!(
            present.contains(&decision),
            "no evaluation exercises gate decision {}",
            decision.as_str()
        );
    }
}

#[test]
fn every_packet_resolves_to_a_governance_family() {
    let packet = packet();
    let governance = current_m5_ecosystem_governance_matrix().expect("governance matrix parses");
    for p in &packet.review_packets {
        let family = governance.family(p.package_kind).unwrap_or_else(|| {
            panic!(
                "package kind {} is not a governance family",
                p.package_kind.as_str()
            )
        });
        assert_eq!(
            p.governance_family_ref, family.family_id,
            "packet {} does not bind to its governance family",
            p.review_id
        );
    }
}

#[test]
fn validate_flags_disposition_mismatch() {
    let mut packet = packet();
    if let Some(p) = packet
        .review_packets
        .iter_mut()
        .find(|p| p.review_disposition != ReviewDisposition::Blocked)
    {
        p.review_disposition = ReviewDisposition::Blocked;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5MirrorAndSideloadViolation::ReviewDispositionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_signal_mismatch() {
    let mut packet = packet();
    if let Some(p) = packet.review_packets.iter_mut().find(|p| {
        !p.continuity_signals
            .contains(&ContinuitySignal::SupportNarrowed)
    }) {
        p.continuity_signals.push(ContinuitySignal::SupportNarrowed);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5MirrorAndSideloadViolation::ContinuitySignalsMismatch { .. }
                | M5MirrorAndSideloadViolation::ReviewDispositionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_dropped_backing_ref() {
    let mut packet = packet();
    if let Some(p) = packet.review_packets.first_mut() {
        p.rollback_ref = String::new();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5MirrorAndSideloadViolation::NotExportSafe { .. })));
    }
}

#[test]
fn validate_flags_gate_decision_mismatch() {
    let mut packet = packet();
    if let Some(e) = packet
        .policy_evaluations
        .iter_mut()
        .find(|e| e.gate_decision != PolicyGateDecision::Blocked)
    {
        e.gate_decision = PolicyGateDecision::Blocked;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5MirrorAndSideloadViolation::GateDecisionMismatch { .. })));
    }
}

#[test]
fn validate_flags_dangling_review_ref() {
    let mut packet = packet();
    if let Some(e) = packet.policy_evaluations.first_mut() {
        e.review_ref = "review:does_not_exist".to_owned();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5MirrorAndSideloadViolation::DanglingReviewRef { .. })));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_review_packets = packet.summary.total_review_packets.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5MirrorAndSideloadViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        AcquisitionChannel::AirGappedImport.as_str(),
        "air_gapped_import"
    );
    assert_eq!(MaintenanceState::Orphaned.as_str(), "orphaned");
    assert_eq!(MirrorFreshness::MirrorStale.as_str(), "mirror_stale");
    assert_eq!(ProvenanceLevel::Unverifiable.as_str(), "unverifiable");
    assert_eq!(NetworkClass::OpenNetwork.as_str(), "open_network");
    assert_eq!(
        ContinuitySignal::PermissionExpansionUnreviewed.as_str(),
        "permission_expansion_unreviewed"
    );
    assert_eq!(ReviewDisposition::Blocked.as_str(), "blocked");
    assert_eq!(
        PolicyDimension::ActivationBudgetBand.as_str(),
        "activation_budget_band"
    );
    assert_eq!(PolicyEffect::RequireApproval.as_str(), "require_approval");
    assert_eq!(
        PolicyGateDecision::ApprovalRequired.as_str(),
        "approval_required"
    );
}

#[test]
fn disposition_widens_monotonically() {
    assert!(ReviewDisposition::Proceed.rank() < ReviewDisposition::ReviewRequired.rank());
    assert!(ReviewDisposition::ReviewRequired.rank() < ReviewDisposition::Blocked.rank());
    assert_eq!(
        ReviewDisposition::Proceed.widen(ReviewDisposition::Blocked),
        ReviewDisposition::Blocked
    );
    assert_eq!(
        PolicyEffect::Allow.widen(PolicyEffect::Block),
        PolicyEffect::Block
    );
    assert_eq!(
        PolicyGateDecision::Allowed.widen(PolicyGateDecision::ApprovalRequired),
        PolicyGateDecision::ApprovalRequired
    );
}
