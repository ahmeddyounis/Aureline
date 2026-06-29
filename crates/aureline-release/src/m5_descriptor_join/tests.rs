//! Inline tests for the M5 descriptor-join lane.

use super::*;

fn registry() -> M5DescriptorJoinRegistry {
    seeded_m5_descriptor_join_registry()
}

#[test]
fn canonical_registry_validates() {
    let registry = registry();
    assert!(registry.validate().is_empty(), "{:?}", registry.validate());
    assert_eq!(registry.registry_id, M5_DESCRIPTOR_JOIN_REGISTRY_ID);
    assert_eq!(
        registry.record_kind,
        M5_DESCRIPTOR_JOIN_REGISTRY_RECORD_KIND
    );
    assert_eq!(registry.joins.len(), 6);
    assert!(registry.conformance.all_hold());
    assert!(registry.vocabulary.matches_canonical());
}

#[test]
fn every_join_validates() {
    for join in registry().joins {
        assert_eq!(join.record_kind, M5_DESCRIPTOR_JOIN_RECORD_KIND);
        assert!(join.validate().is_empty(), "{:?}", join.validate());
    }
}

#[test]
fn every_join_carries_all_four_carriers() {
    for join in registry().joins {
        let carriers: Vec<JoinCarrier> = join.carriers.iter().map(|c| c.carrier).collect();
        assert_eq!(carriers, JoinCarrier::ALL.to_vec());
    }
}

#[test]
fn identity_and_binding_survive_on_every_carrier() {
    // Acceptance criterion 2: descriptor identity and artifact binding survive copy/export instead
    // of collapsing to plain text.
    for join in registry().joins {
        for carrier in &join.carriers {
            assert_eq!(carrier.descriptor_id, join.descriptor_id);
            assert_eq!(carrier.artifact_ref, join.artifact_ref);
            assert!(carrier.preserves_identity);
            assert!(carrier.preserves_binding);
            // The binding is typed, not flattened: every field is present.
            assert!(!carrier.artifact_ref.artifact_id.is_empty());
            assert!(!carrier.artifact_ref.artifact_family.is_empty());
            assert!(!carrier.artifact_ref.content_digest_ref.is_empty());
        }
    }
}

#[test]
fn full_truth_reconstructable_without_translation() {
    // Acceptance criterion 1: a support/admin/export artifact can reconstruct the current
    // provenance/freshness/qualification/client-scope truth from the join.
    for join in registry().joins {
        assert!(join.descriptor.validate().is_empty());
        // Every descriptor family is present as typed state.
        let d = &join.descriptor;
        assert_eq!(d.descriptor_id, join.descriptor_id);
        assert_eq!(d.artifact_ref, join.artifact_ref);
        assert_eq!(d.effective_qualification, join.effective_qualification);
        assert_eq!(d.qualification.support_class, join.claimed_support_class);
        // Provenance / freshness / qualification / client-scope are all carried, not prose.
        let _ = d.provenance.source_class.as_str();
        let _ = d.freshness.freshness_state.as_str();
        let _ = d.client_scope.client_kind.as_str();
    }
}

#[test]
fn downgrade_reasons_remain_attributable_in_carriers() {
    // Acceptance criterion 3: downgrade reasons remain visible and attributable in exported
    // artifacts.
    let join = seeded_limited_join();
    assert!(!join.downgrade_reasons.is_empty());
    // Each reason names a facet and a value token.
    for reason in &join.downgrade_reasons {
        assert!(!reason.token.is_empty());
        assert!(reason
            .reason_message_id
            .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX));
        assert!(reason
            .caveat_message_id
            .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX));
    }
    // Every carrier preserves the full reason count.
    for carrier in &join.carriers {
        assert!(carrier.preserves_downgrade_reasons);
        assert_eq!(
            carrier.downgrade_reason_count,
            join.downgrade_reasons.len() as u32
        );
    }
}

#[test]
fn fully_supported_join_stands_at_stable() {
    let join = seeded_fully_supported_join();
    assert!(join.is_fully_supported());
    assert!(join.downgrade_reasons.is_empty());
    assert_eq!(join.claim_state, NarrowedClaimState::FullySupported);
    assert_eq!(join.effective_qualification, QualificationClass::Stable);
}

#[test]
fn narrowed_join_never_reads_fully_supported_on_a_carrier() {
    for join in registry().joins {
        if join.descriptor.narrowings.is_empty() {
            continue;
        }
        for carrier in &join.carriers {
            assert!(
                !carrier.claim_state.is_fully_supported(),
                "join `{}` left carrier `{}` fully supported",
                join.join_id,
                carrier.carrier.as_str()
            );
        }
    }
}

#[test]
fn blocking_condition_holds_unsupported_across_carriers() {
    let join = seeded_unsupported_join();
    assert!(join.is_blocked());
    assert_eq!(join.claim_state, NarrowedClaimState::Unsupported);
    assert_eq!(
        join.effective_qualification,
        QualificationClass::Unavailable
    );
    for carrier in &join.carriers {
        assert_eq!(carrier.claim_state, NarrowedClaimState::Unsupported);
        assert_eq!(
            carrier.effective_qualification,
            QualificationClass::Unavailable
        );
    }
}

#[test]
fn weaker_origin_survives_as_a_reason() {
    // The blocked, side-loaded condition carries a not-provided origin — it must surface as a
    // reason, never collapse into omission.
    let join = seeded_unsupported_join();
    assert!(join
        .downgrade_reasons
        .iter()
        .any(|r| matches!(r.facet, DescriptorFacet::SourceClass) && r.token == "not_provided"));
}

#[test]
fn evidence_refs_are_refs_only() {
    for join in registry().joins {
        assert!(!join.evidence_refs.is_empty());
        let kinds: Vec<EvidenceRefKind> = join.evidence_refs.iter().map(|r| r.ref_kind).collect();
        assert_eq!(kinds, EvidenceRefKind::ALL.to_vec());
        for r in &join.evidence_refs {
            assert!(!r.ref_value.trim().is_empty());
        }
    }
}

#[test]
fn claim_state_matches_shared_narrowing_runtime() {
    use crate::m5_claim_narrowing::ClaimNarrowingCase;
    for join in registry().joins {
        let case = ClaimNarrowingCase::from_descriptor("check", "check", join.descriptor.clone());
        assert_eq!(join.claim_state, case.canonical_claim_state);
        assert_eq!(
            join.downgrade_reasons.len(),
            case.reasons.len(),
            "join `{}` reason count diverged from claim-narrowing runtime",
            join.join_id
        );
    }
}

#[test]
fn channels_produce_identical_output() {
    for join in registry().joins {
        let desktop = join.render_for_channel(JoinChannel::DesktopUi);
        let cli = join.render_for_channel(JoinChannel::CliHeadless);
        let offline = join.render_for_channel(JoinChannel::OfflineMirror);
        assert_eq!(desktop, cli);
        assert_eq!(cli, offline);
        assert_eq!(desktop, join.export_safe_json());
    }
}

#[test]
fn controlled_vocabulary_is_frozen() {
    let vocab = DescriptorJoinVocabulary::canonical();
    assert_eq!(vocab.carriers.len(), JoinCarrier::ALL.len());
    assert_eq!(vocab.channels.len(), JoinChannel::ALL.len());
    assert_eq!(vocab.evidence_ref_kinds.len(), EvidenceRefKind::ALL.len());
    for needle in [
        "export_packet",
        "support_bundle",
        "admin_report",
        "copy_safe_summary",
    ] {
        assert!(vocab.carriers.contains(&needle.to_owned()));
    }
    for needle in ["desktop_ui", "cli_headless", "offline_mirror"] {
        assert!(vocab.channels.contains(&needle.to_owned()));
    }
}

#[test]
fn registry_round_trips() {
    let registry = registry();
    let json = registry.export_safe_json();
    let parsed: M5DescriptorJoinRegistry =
        serde_json::from_str(&json).expect("registry deserializes");
    assert_eq!(parsed, registry);
    assert!(parsed.validate().is_empty());
}

#[test]
fn join_round_trips_and_preserves_identity_and_binding() {
    for join in registry().joins {
        let json = join.export_safe_json();
        let parsed: DescriptorJoin = serde_json::from_str(&json).expect("join deserializes");
        assert_eq!(parsed, join);
        assert_eq!(parsed.descriptor_id, join.descriptor_id);
        assert_eq!(parsed.artifact_ref, join.artifact_ref);
        assert_eq!(parsed.descriptor.descriptor_id, join.descriptor_id);
    }
}

#[test]
fn tampered_claim_state_is_rejected() {
    let mut join = seeded_evidence_stale_join();
    join.claim_state = NarrowedClaimState::FullySupported;
    let violations = join.validate();
    assert!(violations.contains(&M5DescriptorJoinViolation::ClaimStateDrift));
}

#[test]
fn carrier_dropping_binding_is_rejected() {
    let mut join = seeded_evidence_stale_join();
    join.carriers[0].preserves_binding = false;
    let violations = join.validate();
    assert!(violations.contains(&M5DescriptorJoinViolation::CarrierDrift));
    assert!(violations.contains(&M5DescriptorJoinViolation::CarrierDropsBinding));
}

#[test]
fn carrier_reading_supported_on_narrowed_join_is_rejected() {
    let mut join = seeded_evidence_stale_join();
    join.carriers[0].claim_state = NarrowedClaimState::FullySupported;
    let violations = join.validate();
    assert!(violations.contains(&M5DescriptorJoinViolation::NarrowedCarrierReadsSupported));
}

#[test]
fn tampered_reasons_are_rejected() {
    let mut join = seeded_limited_join();
    join.downgrade_reasons.clear();
    let violations = join.validate();
    assert!(violations.contains(&M5DescriptorJoinViolation::DowngradeReasonDrift));
}

#[test]
fn markdown_render_names_joins_and_carriers() {
    let md = registry().render_markdown_summary();
    assert!(md.contains("# M5 descriptor join parity"));
    assert!(md.contains("Carrier parity"));
    assert!(md.contains("Copy-safe summary"));
    assert!(md.contains("export_packet"));
    assert!(md.contains("support_bundle"));
    assert!(md.contains("admin_report"));
    assert!(md.contains("Downgrade reasons (attributable)"));
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
}

#[test]
fn summary_counts_match() {
    let registry = registry();
    let s = &registry.summary;
    assert_eq!(s.total_joins, 6);
    assert_eq!(s.fully_supported_joins, 1);
    assert_eq!(s.blocked_joins, 1);
    assert_eq!(s.narrowed_joins, 4);
    assert_eq!(
        s.total_carrier_renderings,
        6 * JoinCarrier::ALL.len() as u32
    );
}
