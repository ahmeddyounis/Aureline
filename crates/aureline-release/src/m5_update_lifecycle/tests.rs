//! Inline tests for the M5 update / support-lifecycle governance lane.

use super::*;

fn packet() -> M5UpdateLifecycleGovernance {
    seeded_m5_update_lifecycle()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_UPDATE_LIFECYCLE_PACKET_ID);
    assert_eq!(packet.record_kind, M5_UPDATE_LIFECYCLE_RECORD_KIND);
    assert_eq!(packet.facets.len(), LifecycleFacet::ALL.len());
    assert_eq!(packet.consumers.len(), LifecycleConsumer::ALL.len());
    assert_eq!(packet.state_families.len(), LifecycleStateFamily::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn every_facet_is_governed_exactly_once() {
    let packet = packet();
    for facet in LifecycleFacet::ALL {
        let matches: Vec<&LifecycleFacetRow> =
            packet.facets.iter().filter(|r| r.facet == facet).collect();
        assert_eq!(
            matches.len(),
            1,
            "facet `{}` not governed once",
            facet.as_str()
        );
        let row = matches[0];
        assert_eq!(row.proof_ref, facet.proof_ref());
        assert_eq!(row.dimension, facet.dimension());
        assert_eq!(row.state_family, facet.state_family());
        assert!(row.state_family.contains_token(&row.current_state_token));
    }
}

#[test]
fn every_dimension_and_state_family_is_covered() {
    let packet = packet();
    for dimension in LifecycleDimension::ALL {
        assert!(
            packet.facets.iter().any(|f| f.dimension == dimension),
            "dimension `{}` not covered",
            dimension.as_str()
        );
    }
    for family in LifecycleStateFamily::ALL {
        assert!(
            packet.facets.iter().any(|f| f.state_family == family),
            "state family `{}` not referenced",
            family.as_str()
        );
    }
}

#[test]
fn canonical_packet_certifies_every_consumer() {
    // Acceptance criterion: every claimed consumer maps to current proofs and governed states.
    let packet = packet();
    for consumer in &packet.consumers {
        assert!(
            consumer.is_certified(),
            "consumer `{}` not certified when every facet is current",
            consumer.consumer.as_str()
        );
        assert_eq!(
            consumer.effective_qualification,
            consumer.claimed_qualification
        );
        assert!(consumer.gaps.is_empty());
    }
    assert_eq!(
        packet.summary.certified_consumer_count,
        LifecycleConsumer::ALL.len() as u32
    );
    assert_eq!(packet.summary.narrowed_consumer_count, 0);
    assert_eq!(packet.summary.blocked_consumer_count, 0);
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn every_consumer_maps_to_facets_owner_and_proof() {
    // Acceptance criterion: the matrix enumerates every consumer, current owner, and proof path.
    let packet = packet();
    for c in &packet.consumers {
        assert_eq!(c.owner_role, c.consumer.owner_role());
        assert!(!c.read_facets.is_empty());
        assert_eq!(c.proof_refs.len(), c.read_facets.len());
        assert!(!c.disclosed_artifact_classes.is_empty());
        assert!(!c.channel_scope.is_empty());
        assert!(!c.profiles.is_empty());
        // Proof refs resolve to the read facets' proof paths.
        for (facet, proof) in c.read_facets.iter().zip(c.proof_refs.iter()) {
            assert_eq!(proof, facet.proof_ref());
        }
    }
}

#[test]
fn canonical_state_vocabulary_is_bound_to_gate() {
    // Acceptance criterion: lifecycle classes are canonical and bound to descriptor/badge gate rows.
    let packet = packet();
    assert!(packet.conformance.state_vocabulary_bound_to_gate);
    for family in &packet.state_families {
        assert!(!family.states.is_empty());
        for state in &family.states {
            assert_eq!(state.effective_floor, floor_for_posture(state.gate_posture));
            assert!(state
                .message_id
                .starts_with(M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX));
        }
    }
    // The known blocking states floor at Unavailable.
    let eos = packet
        .state_families
        .iter()
        .find(|f| f.family == LifecycleStateFamily::EndOfSupport)
        .unwrap();
    let removed = eos.states.iter().find(|s| s.token == "removed").unwrap();
    assert_eq!(removed.gate_posture, DescriptorGate::Blocked);
    assert_eq!(removed.effective_floor, QualificationClass::Unavailable);
}

#[test]
fn stale_facet_proof_narrows_exactly_the_consumers_that_read_it() {
    // Acceptance criterion: a stale proof narrows claims deterministically.
    let packet = seeded_m5_update_lifecycle_stale_proof_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let stale = LifecycleFacet::ChangeImpact;
    assert_eq!(
        packet.facet(stale).unwrap().proof_freshness,
        FreshnessState::Stale
    );
    for c in &packet.consumers {
        if c.read_facets.contains(&stale) {
            assert!(
                c.is_narrowed(),
                "consumer `{}` reads the stale facet but did not narrow",
                c.consumer.as_str()
            );
            assert!(c
                .gaps
                .iter()
                .any(|g| g.facet == stale && g.gap_kind == LifecycleGapKind::ProofStale));
            assert_eq!(c.effective_qualification, QualificationClass::Beta);
        } else {
            assert!(
                c.is_certified(),
                "consumer `{}` does not read the stale facet but narrowed",
                c.consumer.as_str()
            );
        }
    }
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(packet.summary.narrowed_consumer_count, 5);
    assert_eq!(packet.summary.certified_consumer_count, 3);
    assert!(packet
        .release_gate
        .drifted_dimensions
        .contains(&LifecycleDimension::ChangeDisclosure.as_str().to_owned()));
}

#[test]
fn missing_facet_proof_blocks_exactly_the_consumers_that_read_it() {
    // Acceptance criterion: a missing/failing proof blocks Stable promotion deterministically.
    let packet = seeded_m5_update_lifecycle_missing_proof_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let failing = LifecycleFacet::ServiceHealth;
    assert_eq!(
        packet.facet(failing).unwrap().proof_freshness,
        FreshnessState::Missing
    );
    for c in &packet.consumers {
        if c.read_facets.contains(&failing) {
            assert!(
                c.is_blocked(),
                "consumer `{}` reads the failing facet but was not blocked",
                c.consumer.as_str()
            );
            assert_eq!(c.effective_qualification, QualificationClass::Unavailable);
            assert!(c
                .gaps
                .iter()
                .any(|g| g.facet == failing && g.gap_kind == LifecycleGapKind::ProofMissing));
        } else {
            assert!(
                c.is_certified(),
                "consumer `{}` does not read the failing facet but was blocked",
                c.consumer.as_str()
            );
        }
    }
    assert!(packet.blocks_stable_promotion());
    assert_eq!(packet.summary.blocked_consumer_count, 6);
    assert_eq!(packet.summary.certified_consumer_count, 2);
    assert_eq!(packet.release_gate.blocked_consumers.len(), 6);
}

#[test]
fn lifecycle_state_gap_blocks_when_facet_state_blocks() {
    // Acceptance criterion: gaps in lifecycle coverage (not just proof) fail the matrix.
    let mut packet = packet();
    // Force the end-of-support facet into a removed (blocking) lifecycle state.
    let removed = LifecycleFacetRow::new(
        LifecycleFacet::EndOfSupport,
        CanonicalState::EndOfSupport(EndOfSupportState::Removed),
        FreshnessState::Current,
        &[ArtifactClass::CoreRuntime, ArtifactClass::DocsHelpContent],
        &[ChannelScope::Stable, ChannelScope::Lts],
        &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
        StaleDataBehavior::MirroredLabelled,
    );
    let idx = packet
        .facets
        .iter()
        .position(|f| f.facet == LifecycleFacet::EndOfSupport)
        .unwrap();
    packet.facets[idx] = removed;
    for consumer in &mut packet.consumers {
        consumer.recompute(&packet.facets);
    }
    let release = packet.consumer(LifecycleConsumer::ReleaseCenter).unwrap();
    assert!(release.is_blocked());
    assert!(release
        .gaps
        .iter()
        .any(|g| g.facet == LifecycleFacet::EndOfSupport
            && g.gap_kind == LifecycleGapKind::LifecycleStateBlocked));
}

#[test]
fn coverage_gap_names_its_drifted_dimension() {
    let packet = seeded_m5_update_lifecycle_stale_proof_narrowed();
    let release = packet.consumer(LifecycleConsumer::ReleaseCenter).unwrap();
    let gap = release
        .gaps
        .iter()
        .find(|g| g.facet == LifecycleFacet::ChangeImpact)
        .expect("release center reads the stale change-impact facet");
    assert_eq!(gap.dimension, LifecycleDimension::ChangeDisclosure);
    assert_eq!(gap.gap_kind, LifecycleGapKind::ProofStale);
    assert!(gap
        .cause_message_id
        .starts_with(M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX));
}

#[test]
fn channels_produce_identical_output() {
    let packet = packet();
    let desktop = packet.render_for_channel(LifecycleChannel::DesktopUi);
    let cli = packet.render_for_channel(LifecycleChannel::CliHeadless);
    let offline = packet.render_for_channel(LifecycleChannel::OfflineMirror);
    assert_eq!(desktop, cli);
    assert_eq!(cli, offline);
    assert_eq!(desktop, packet.export_safe_json());
}

#[test]
fn controlled_vocabulary_is_frozen() {
    let vocab = LifecycleVocabulary::canonical();
    assert_eq!(vocab.facets.len(), LifecycleFacet::ALL.len());
    assert_eq!(vocab.state_families.len(), LifecycleStateFamily::ALL.len());
    assert_eq!(vocab.consumers.len(), LifecycleConsumer::ALL.len());
    for needle in [
        "update_availability",
        "change_impact",
        "release_note_evidence",
        "migration_assistant",
        "service_health",
        "support_window",
        "compatibility_window",
        "end_of_support",
    ] {
        assert!(vocab.facets.contains(&needle.to_owned()));
    }
    for needle in [
        "update",
        "readiness",
        "migration",
        "support_window",
        "end_of_support",
    ] {
        assert!(vocab.state_families.contains(&needle.to_owned()));
    }
    // Channels match the frozen release-channel vocabulary.
    for needle in ["stable", "beta", "preview", "nightly", "lts"] {
        assert!(vocab.channels.contains(&needle.to_owned()));
    }
}

#[test]
fn packet_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5UpdateLifecycleGovernance =
        serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn one_matrix_across_all_surfaces() {
    // Acceptance criterion: release, update-center, support, docs, diagnostics, and exports consume one matrix.
    let packet = packet();
    let expected = tokens(&LifecycleConsumer::ALL, |c| c.as_str());
    assert_eq!(packet.consumer_tokens, expected);
    assert!(packet.disclosure.all_consume());
    assert!(packet.conformance.surfaces_consume_one_matrix);
}

#[test]
fn matrix_csv_enumerates_consumer_owner_and_proof() {
    let csv = packet().render_matrix_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("consumer,consumer_owner,"));
    assert!(header.contains("proof_ref"));
    assert!(header.contains("gap_kind"));
    // Every consumer appears with its owner and a proof path.
    for c in LifecycleConsumer::ALL {
        assert!(csv.contains(&format!("{},{}", c.as_str(), c.owner_role())));
    }
    assert!(csv.contains("artifacts/release-proof/m5-update-lifecycle/"));
}

#[test]
fn governance_markdown_names_states_facets_and_consumers() {
    let md = seeded_m5_update_lifecycle_stale_proof_narrowed().render_governance_markdown();
    assert!(md.contains("# M5 Update / Support-Lifecycle Governance Matrix"));
    assert!(md.contains("Canonical lifecycle state families"));
    assert!(md.contains("Governed facets"));
    assert!(md.contains("Claimed consumers"));
    assert!(md.contains("change_impact"));
    assert!(md.contains("gap:"));
}

#[test]
fn tampered_consumer_verdict_is_rejected() {
    let mut packet = seeded_m5_update_lifecycle_stale_proof_narrowed();
    let idx = packet
        .consumers
        .iter()
        .position(|c| c.is_narrowed())
        .expect("a narrowed consumer exists");
    packet.consumers[idx].gate_decision = DescriptorGate::Governed;
    packet.consumers[idx].effective_qualification = QualificationClass::Stable;
    let violations = packet.validate();
    assert!(violations.contains(&M5UpdateLifecycleViolation::ConsumerVerdictDrift));
}

#[test]
fn tampered_facet_freshness_is_rejected() {
    let mut packet = packet();
    packet.facets[0].proof_freshness = FreshnessState::Stale;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5UpdateLifecycleViolation::FacetStatusDrift)
            || violations.contains(&M5UpdateLifecycleViolation::SummaryDrift)
            || violations.contains(&M5UpdateLifecycleViolation::ConsumerVerdictDrift),
        "{violations:?}"
    );
}

#[test]
fn dropping_a_facet_is_rejected() {
    let mut packet = packet();
    packet
        .facets
        .retain(|f| f.facet != LifecycleFacet::SupportWindow);
    let violations = packet.validate();
    assert!(violations.contains(&M5UpdateLifecycleViolation::FacetNotGoverned));
}

#[test]
fn summary_counts_match_canonical() {
    let packet = packet();
    let s = &packet.summary;
    assert_eq!(s.total_facets, LifecycleFacet::ALL.len() as u32);
    assert_eq!(s.current_facets, LifecycleFacet::ALL.len() as u32);
    assert_eq!(s.stale_facets, 0);
    assert_eq!(s.missing_facets, 0);
    assert_eq!(
        s.total_state_families,
        LifecycleStateFamily::ALL.len() as u32
    );
    assert_eq!(s.total_consumers, LifecycleConsumer::ALL.len() as u32);
    assert!(!s.blocks_stable_promotion);
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        seeded_m5_update_lifecycle(),
        seeded_m5_update_lifecycle_stale_proof_narrowed(),
        seeded_m5_update_lifecycle_missing_proof_blocked(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
    }
}
