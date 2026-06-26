use super::*;

fn packet() -> DocsPrecedenceRankingPacket {
    DocsPrecedenceRankingPacket::materialize(seeded_stable_docs_precedence_ranking_input())
}

fn repo_candidate<'a>(
    packet: &'a mut DocsPrecedenceRankingPacket,
    candidate_id: &str,
) -> &'a mut RankedDocsCandidate {
    packet
        .ranking_sets
        .iter_mut()
        .flat_map(|set| set.candidates.iter_mut())
        .find(|candidate| candidate.candidate_id == candidate_id)
        .expect("candidate present")
}

#[test]
fn seeded_packet_is_stable() {
    let packet = packet();
    assert_eq!(
        packet.promotion_state,
        DocsPrecedenceRankingPromotionState::Stable
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_clean_stable());
}

#[test]
fn seeded_packet_keeps_seven_lanes_distinguishable() {
    let packet = packet();
    let present: std::collections::HashSet<DocsSourceLane> =
        packet.lanes_present().into_iter().collect();
    for lane in DocsSourceLane::ALL {
        assert!(present.contains(&lane), "missing lane {}", lane.as_str());
    }
}

#[test]
fn seeded_packet_covers_every_required_surface() {
    let packet = packet();
    let present: std::collections::BTreeSet<RankExplanationSurface> =
        packet.covered_surfaces().into_iter().collect();
    for surface in RankExplanationSurface::REQUIRED {
        assert!(
            present.contains(&surface),
            "missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn project_docs_outrank_vendor_but_keep_difference_visible() {
    let packet = packet();
    let set = packet
        .ranking_set(seed::REPO_SUBJECT_ID)
        .expect("repo set present");
    let project = set
        .candidate("candidate:repo:project-runbook")
        .expect("project candidate present");
    assert_eq!(
        project.precedence_class,
        SourcePrecedenceClass::ProjectOutranksVendorDefault
    );
    assert_eq!(project.rank_position, 1);
    // The vendor / mirrored alternative is still present and referenced.
    assert!(set
        .candidates
        .iter()
        .any(|candidate| candidate.lane.is_vendor_alternative()));
    assert!(project
        .outranks_refs
        .iter()
        .any(|outranked| outranked == "candidate:repo:mirror-std"));
}

#[test]
fn project_labeled_as_vendor_trust_has_no_lane() {
    let mut packet = packet();
    repo_candidate(&mut packet, "candidate:repo:project-runbook").trust_class =
        DocsObjectTrustClass::LiveProviderHandoff;
    assert!(packet
        .validate()
        .iter()
        .any(|finding| finding.finding_kind
            == DocsPrecedenceRankingFindingKind::CandidateLaneUnresolved));
}

#[test]
fn declared_lane_mismatch_blocks_stable() {
    let mut packet = packet();
    repo_candidate(&mut packet, "candidate:repo:project-runbook").lane =
        DocsSourceLane::LiveExternalDocs;
    assert!(packet
        .validate()
        .iter()
        .any(|finding| finding.finding_kind
            == DocsPrecedenceRankingFindingKind::CandidateLaneMismatch));
}

#[test]
fn missing_distinguishable_lane_blocks_stable() {
    let mut packet = packet();
    for set in packet.ranking_sets.iter_mut() {
        set.candidates
            .retain(|candidate| candidate.lane != DocsSourceLane::DerivedExplanation);
    }
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DocsPrecedenceRankingFindingKind::SourceClassDistinguishabilityMissing));
}

#[test]
fn unexplained_rank_inversion_blocks_stable() {
    let mut packet = packet();
    // The mirror candidate (more authoritative) keeps rank 2 while the project
    // candidate's justifying reason is replaced by a non-justifying one.
    repo_candidate(&mut packet, "candidate:repo:project-runbook").precedence_reason =
        PrecedenceReason::OfficialUpstreamAuthority;
    // Keep reason/class consistent so the inversion finding is the one observed.
    repo_candidate(&mut packet, "candidate:repo:project-runbook").precedence_class =
        SourcePrecedenceClass::NotApplicable;
    let kinds: Vec<_> = packet
        .validate()
        .into_iter()
        .map(|finding| finding.finding_kind)
        .collect();
    assert!(kinds.contains(&DocsPrecedenceRankingFindingKind::UnexplainedRankInversion));
}

#[test]
fn outrank_without_visible_alternative_blocks_stable() {
    let mut packet = packet();
    // Drop the vendor / mirrored alternatives so the project outrank has nothing
    // visible to outrank.
    let repo = packet
        .ranking_sets
        .iter_mut()
        .find(|set| set.subject_id == seed::REPO_SUBJECT_ID)
        .expect("repo set present");
    repo.candidates
        .retain(|candidate| !candidate.lane.is_vendor_alternative());
    repo_candidate(&mut packet, "candidate:repo:project-runbook").outranks_refs = Vec::new();
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DocsPrecedenceRankingFindingKind::OutrankWithoutVisibleAlternative));
}

#[test]
fn derived_explanation_ranked_primary_blocks_stable() {
    let mut packet = packet();
    let derived = repo_candidate(&mut packet, "candidate:repo:derived-summary");
    derived.rank_position = 1;
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DocsPrecedenceRankingFindingKind::DerivedExplanationRankedAsPrimary));
}

#[test]
fn precedence_reason_class_mismatch_blocks_stable() {
    let mut packet = packet();
    // A vendor-override reason while declaring a project-authoritative class.
    let mirror = repo_candidate(&mut packet, "candidate:repo:mirror-std");
    mirror.precedence_reason = PrecedenceReason::VendorOverridePolicy;
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DocsPrecedenceRankingFindingKind::PrecedenceReasonClassMismatch));
}

#[test]
fn missing_reason_note_blocks_stable() {
    let mut packet = packet();
    repo_candidate(&mut packet, "candidate:repo:generated-ref")
        .precedence_reason_note
        .clear();
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DocsPrecedenceRankingFindingKind::PrecedenceReasonNoteMissing));
}

#[test]
fn offline_unavailable_without_reason_blocks_stable() {
    let mut packet = packet();
    let mirror = repo_candidate(&mut packet, "candidate:repo:mirror-std");
    mirror.available_in_offline_profile = false;
    mirror.unavailable_reason = None;
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DocsPrecedenceRankingFindingKind::OfflineUnavailableReasonMissing));
}

#[test]
fn offline_unavailable_with_reason_narrows_below_stable() {
    let mut packet = packet();
    {
        let mirror = repo_candidate(&mut packet, "candidate:repo:mirror-std");
        mirror.available_in_offline_profile = false;
        mirror.unavailable_reason =
            Some("Mirror pack is not installed in this air-gapped profile.".to_owned());
    }
    // Re-materialize so the stored promotion state is recomputed.
    let packet = DocsPrecedenceRankingPacket::materialize(DocsPrecedenceRankingPacketInput {
        packet_id: packet.packet_id,
        surface_label: packet.surface_label,
        generated_at: packet.generated_at,
        ranking_sets: packet.ranking_sets,
        surface_projections: packet.surface_projections,
        source_contract_refs: packet.source_contract_refs,
        redaction_class_token: packet.redaction_class_token,
    });
    assert_eq!(
        packet.promotion_state,
        DocsPrecedenceRankingPromotionState::NarrowedBelowStable
    );
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DocsPrecedenceRankingFindingKind::AirGappedCandidateNarrowed));
}

#[test]
fn hidden_ranking_model_blocks_stable() {
    let mut packet = packet();
    packet.surface_projections[3].uses_shared_ranking_vocabulary = false;
    assert!(packet.validate().iter().any(
        |finding| finding.finding_kind == DocsPrecedenceRankingFindingKind::HiddenRankingModel
    ));
}

#[test]
fn projection_dropping_truth_blocks_stable() {
    let mut packet = packet();
    packet.surface_projections[0].shows_precedence_reason = true;
    packet.surface_projections[0].shows_freshness = false;
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DocsPrecedenceRankingFindingKind::RankExplanationDropsTruth));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut packet = packet();
    packet
        .surface_projections
        .retain(|projection| projection.surface != RankExplanationSurface::Onboarding);
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DocsPrecedenceRankingFindingKind::MissingRankExplanationSurface));
}

#[test]
fn support_export_dropping_ranking_set_blocks_stable() {
    let mut packet = packet();
    packet.surface_projections.retain(|projection| {
        !(projection.surface == RankExplanationSurface::SupportExport
            && projection.ranking_set_ref == seed::LIBRARY_SUBJECT_ID)
    });
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DocsPrecedenceRankingFindingKind::SupportExportDropsRankingSet));
}

#[test]
fn projection_referencing_unknown_set_blocks_stable() {
    let mut packet = packet();
    packet.surface_projections[0].ranking_set_ref = "subject:nonexistent".to_owned();
    assert!(packet
        .validate()
        .iter()
        .any(|finding| finding.finding_kind
            == DocsPrecedenceRankingFindingKind::ProjectionRefUnresolved));
}

#[test]
fn missing_source_contracts_blocks_stable() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .iter()
        .any(|finding| finding.finding_kind
            == DocsPrecedenceRankingFindingKind::MissingSourceContracts));
}

#[test]
fn promotion_state_mismatch_is_detected() {
    let mut packet = packet();
    packet.promotion_state = DocsPrecedenceRankingPromotionState::BlocksStable;
    assert!(packet
        .validate()
        .iter()
        .any(|finding| finding.finding_kind
            == DocsPrecedenceRankingFindingKind::PromotionStateMismatch));
}

#[test]
fn markdown_summary_lists_every_surface() {
    let summary = packet().render_markdown_summary();
    for surface in RankExplanationSurface::REQUIRED {
        assert!(
            summary.contains(surface.as_str()),
            "summary missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn support_export_round_trips_safely() {
    let packet = packet();
    let export = packet.support_export(
        "support-export:docs_source_precedence_and_ranking_parity:test",
        "2026-06-26T00:30:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.export_packet_id_ref, packet.packet_id);
    assert_eq!(export.export_packet, packet);
}

#[test]
fn checked_support_export_validates() {
    let export =
        current_stable_docs_precedence_ranking_export().expect("checked support export validates");
    assert!(export.is_export_safe());
    assert_eq!(
        export.export_packet.promotion_state,
        DocsPrecedenceRankingPromotionState::Stable
    );
}

#[test]
fn current_packet_helper_validates() {
    let packet = current_stable_docs_precedence_ranking_packet().expect("seeded packet validates");
    assert!(packet.validate().is_empty());
}
