use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_reactive_governance_packet();
    validate_m5_reactive_governance_packet(&packet)
        .expect("seeded matrix must satisfy the frozen contract");
}

#[test]
fn seeded_fixtures_validate() {
    let packet = seeded_m5_reactive_governance_packet();
    let fixtures = seeded_m5_reactive_governance_fixtures();
    assert_eq!(fixtures.len(), 10);
    for fixture in &fixtures {
        validate_m5_reactive_governance_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn no_derived_surface_claims_exact_current_truth() {
    let packet = seeded_m5_reactive_governance_packet();
    for row in &packet.surfaces {
        assert_eq!(row.derivation_class, DerivationClass::Derived);
        assert_ne!(
            row.healthy_claim,
            TruthClaim::ExactCurrentTruth,
            "surface {} must not present exact current truth",
            row.surface_class.as_str()
        );
        assert_eq!(row.healthy_claim, TruthClaim::ConsistentSnapshot);
    }
}

#[test]
fn narrowing_picks_the_narrowest_candidate() {
    // Imported (strength 2) beats partial (strength 7): the narrowest wins.
    let observed = ObservedReactiveState {
        freshness: Freshness::Imported,
        completeness: Completeness::Partial,
        backpressure_mode: BackpressureMode::Realtime,
        terminal_reason: None,
        policy_limited: false,
    };
    let narrowed = narrow_truth_claim(DerivationClass::Derived, &observed);
    assert_eq!(narrowed.claim, TruthClaim::ImportedSnapshot);
    // Triggers are reported sorted by their declaration order.
    assert_eq!(
        narrowed.triggers,
        vec![
            NarrowingTrigger::FreshnessImported,
            NarrowingTrigger::CompletenessPartial,
        ]
    );
}

#[test]
fn provider_unavailable_dominates_every_other_trigger() {
    let observed = ObservedReactiveState {
        freshness: Freshness::Stale,
        completeness: Completeness::Unavailable,
        backpressure_mode: BackpressureMode::Coalesced,
        terminal_reason: Some(TerminalReason::Unavailable),
        policy_limited: true,
    };
    let narrowed = narrow_truth_claim(DerivationClass::Derived, &observed);
    assert_eq!(narrowed.claim, TruthClaim::ProviderUnavailable);
}

#[test]
fn healthy_state_yields_consistent_snapshot_with_no_triggers() {
    let narrowed = narrow_truth_claim(DerivationClass::Derived, &ObservedReactiveState::healthy());
    assert_eq!(narrowed.claim, TruthClaim::ConsistentSnapshot);
    assert!(narrowed.triggers.is_empty());
}

#[test]
fn matrix_covers_all_authority_and_view_classes() {
    let packet = seeded_m5_reactive_governance_packet();
    let authorities: BTreeSet<_> = packet.surfaces.iter().map(|r| r.authority_class).collect();
    for required in [
        AuthorityClass::WorkspaceVfs,
        AuthorityClass::BufferEditor,
        AuthorityClass::DerivedKnowledge,
        AuthorityClass::Execution,
        AuthorityClass::PolicyEntitlement,
        AuthorityClass::ProviderOverlay,
    ] {
        assert!(
            authorities.contains(&required),
            "matrix must cover authority {}",
            required.as_str()
        );
    }
    let views: BTreeSet<_> = packet.surfaces.iter().map(|r| r.view_class).collect();
    assert_eq!(views.len(), 4, "matrix must cover all four view classes");
}

#[test]
fn epoch_parity_groups_partition_by_authority() {
    let packet = seeded_m5_reactive_governance_packet();
    for group in &packet.epoch_parity_groups {
        for member in &group.member_surfaces {
            let row = packet
                .surfaces
                .iter()
                .find(|r| r.surface_class == *member)
                .expect("member surface must exist");
            assert_eq!(row.authority_class, group.authority_class);
        }
    }
}

#[test]
fn materialized_views_follow_view_class_lifecycle() {
    let packet = seeded_m5_reactive_governance_packet();
    for view in &packet.materialized_views {
        match view.view_class {
            ViewClass::ExportableSnapshot => {
                assert!(!view.rebuildable_from_authority);
                assert_eq!(
                    view.delete_semantics,
                    DeleteSemantics::ReplacedByNewSnapshot
                );
            }
            ViewClass::EphemeralProjection => {
                assert_eq!(view.persistence, PersistenceClass::MemoryOnly);
            }
            ViewClass::ManagedReplicatedView => {
                assert_eq!(view.delete_semantics, DeleteSemantics::ReconcileOnReconnect);
            }
            ViewClass::DurableLocalMaterialization => {
                assert_eq!(view.persistence, PersistenceClass::LocalCacheOrDb);
            }
        }
    }
}
