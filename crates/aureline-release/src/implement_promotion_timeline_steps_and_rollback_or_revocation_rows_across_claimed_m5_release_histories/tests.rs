use super::*;

fn promotion_input(identity: &str) -> M5ReleaseHistoryEventInput {
    M5ReleaseHistoryEventInput {
        event_identity_repr: identity.to_owned(),
        event_kind: M5ReleaseHistoryEventKind::PromotionStep,
        source_stage_repr: "canary_ring".to_owned(),
        destination_stage_repr: "pilot_ring".to_owned(),
        stage_state: M5PromotionStageState::StagePromoted,
        rollout_ring: M5RolloutRing::CanaryRing,
        reversible_window: M5ReversibleWindowState::ReversibleWithinWindow,
        digest_refs: vec!["sha256:aa11".to_owned()],
        evidence_refs: vec!["evidence:qual".to_owned()],
        approving_actors: vec!["actor:captain".to_owned()],
        effective_time_repr: "2026-07-05T09:00:00Z".to_owned(),
        break_glass_posture: M5BreakGlassPosture::StandardChangeControl,
        affected_node_set: Vec::new(),
        blast_radius: M5RollbackBlastRadius::SingleArtifact,
        node_targeting: M5NodeTargeting::NotApplicableTargeting,
        revocation_scope: M5RevocationScope::NoRevocation,
        last_known_good_target_repr: String::new(),
        continuity_note_repr: String::new(),
    }
}

fn rollback_input(identity: &str) -> M5ReleaseHistoryEventInput {
    M5ReleaseHistoryEventInput {
        event_identity_repr: identity.to_owned(),
        event_kind: M5ReleaseHistoryEventKind::RollbackRevocationRow,
        source_stage_repr: String::new(),
        destination_stage_repr: String::new(),
        stage_state: M5PromotionStageState::StagePromoted,
        rollout_ring: M5RolloutRing::HeldNotPromoted,
        reversible_window: M5ReversibleWindowState::NotApplicableWindow,
        digest_refs: vec!["sha256:bb22".to_owned()],
        evidence_refs: vec!["evidence:incident".to_owned()],
        approving_actors: vec!["actor:commander".to_owned()],
        effective_time_repr: "2026-07-05T16:00:00Z".to_owned(),
        break_glass_posture: M5BreakGlassPosture::StandardChangeControl,
        affected_node_set: vec!["node:a".to_owned(), "node:b".to_owned()],
        blast_radius: M5RollbackBlastRadius::FamilyScoped,
        node_targeting: M5NodeTargeting::PartialNodeSetExplicit,
        revocation_scope: M5RevocationScope::TagRepointOnly,
        last_known_good_target_repr: "release:prior".to_owned(),
        continuity_note_repr: "unaffected nodes stay current".to_owned(),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_promotion_promoted_within_window_is_reversible() {
    let resolved = resolve_release_history_event(&promotion_input("event:p1")).expect("resolves");
    assert_eq!(
        resolved.history_posture,
        M5ReleaseHistoryPosture::PromotionRecordedReversible
    );
    assert!(resolved.is_recorded);
    assert!(!resolved.is_blocked);
    assert!(resolved.history_banner.is_none());
    let view = resolved.promotion_view.expect("promotion view present");
    assert!(view.reversible);
    assert!(resolved.rollback_view.is_none());
    assert!(resolved.reconstruction.is_reconstructable);
}

#[test]
fn resolver_promotion_expired_window_is_irreversible() {
    let input = M5ReleaseHistoryEventInput {
        reversible_window: M5ReversibleWindowState::ReversibleWindowExpired,
        ..promotion_input("event:p2")
    };
    let resolved = resolve_release_history_event(&input).expect("resolves");
    assert_eq!(
        resolved.history_posture,
        M5ReleaseHistoryPosture::PromotionRecordedIrreversible
    );
    assert!(resolved.is_recorded);
    assert!(!resolved.promotion_view.unwrap().reversible);
}

#[test]
fn resolver_promotion_in_progress_and_blocked() {
    let in_progress = resolve_release_history_event(&M5ReleaseHistoryEventInput {
        stage_state: M5PromotionStageState::StageInProgress,
        ..promotion_input("event:p3")
    })
    .expect("resolves");
    assert_eq!(
        in_progress.history_posture,
        M5ReleaseHistoryPosture::PromotionInProgress
    );

    let blocked = resolve_release_history_event(&M5ReleaseHistoryEventInput {
        stage_state: M5PromotionStageState::StageBlocked,
        ..promotion_input("event:p4")
    })
    .expect("resolves");
    assert_eq!(
        blocked.history_posture,
        M5ReleaseHistoryPosture::PromotionBlocked
    );
    assert_eq!(
        blocked.history_banner.unwrap().reason,
        M5ReleaseHistoryBlockReason::StagePromotionBlocked
    );
}

#[test]
fn resolver_rollback_bounded_is_not_generic() {
    let resolved = resolve_release_history_event(&rollback_input("event:r1")).expect("resolves");
    assert_eq!(
        resolved.history_posture,
        M5ReleaseHistoryPosture::RollbackRecordedBounded
    );
    let view = resolved.rollback_view.expect("rollback view present");
    assert!(!view.revokes_trust_material);
    assert_eq!(view.affected_node_count, 2);
    assert!(view.node_targeting.is_partial());
    assert!(!matches!(view.blast_radius, M5RollbackBlastRadius::SingleArtifact));
}

#[test]
fn resolver_revocation_of_trust_material_is_revocation() {
    let resolved = resolve_release_history_event(&M5ReleaseHistoryEventInput {
        revocation_scope: M5RevocationScope::SigningKeyRevoked,
        ..rollback_input("event:r2")
    })
    .expect("resolves");
    assert_eq!(
        resolved.history_posture,
        M5ReleaseHistoryPosture::RevocationRecorded
    );
    assert!(resolved.rollback_view.unwrap().revokes_trust_material);
}

#[test]
fn resolver_emergency_break_glass_stays_visible_in_history() {
    let resolved = resolve_release_history_event(&M5ReleaseHistoryEventInput {
        break_glass_posture: M5BreakGlassPosture::BreakGlassAttributed,
        ..rollback_input("event:r3")
    })
    .expect("resolves");
    assert_eq!(
        resolved.history_posture,
        M5ReleaseHistoryPosture::EmergencyBreakGlassRecorded
    );
    assert!(resolved.is_emergency);
    assert!(resolved.emergency_visible_in_history);
    assert!(resolved.is_recorded);
}

#[test]
fn resolver_unattributed_break_glass_is_blocked() {
    let resolved = resolve_release_history_event(&M5ReleaseHistoryEventInput {
        break_glass_posture: M5BreakGlassPosture::BreakGlassUnattributed,
        approving_actors: vec![],
        ..rollback_input("event:r4")
    })
    .expect("resolves");
    assert_eq!(
        resolved.history_posture,
        M5ReleaseHistoryPosture::HistoryBlockedUnattributed
    );
    let banner = resolved.history_banner.expect("banner present");
    assert_eq!(banner.reason, M5ReleaseHistoryBlockReason::EmergencyActionUnattributed);
    assert_eq!(
        banner.next_action,
        M5ReleaseHistoryNextAction::AttributeEmergencyActor
    );
    // The emergency stays visible in the history model even while blocked.
    assert!(resolved.emergency_visible_in_history);
}

#[test]
fn resolver_missing_last_known_good_and_digest_join_block() {
    let no_lkg = resolve_release_history_event(&M5ReleaseHistoryEventInput {
        last_known_good_target_repr: "  ".to_owned(),
        ..rollback_input("event:r5")
    })
    .expect("resolves");
    assert_eq!(
        no_lkg.history_posture,
        M5ReleaseHistoryPosture::HistoryBlockedMissingLastKnownGood
    );

    let no_digest = resolve_release_history_event(&M5ReleaseHistoryEventInput {
        digest_refs: vec![],
        ..rollback_input("event:r6")
    })
    .expect("resolves");
    assert_eq!(
        no_digest.history_posture,
        M5ReleaseHistoryPosture::HistoryBlockedMissingDigestJoin
    );
    assert_eq!(
        no_digest.history_banner.unwrap().next_action,
        M5ReleaseHistoryNextAction::RecordImmutableDigestJoin
    );
}

#[test]
fn resolver_missing_digest_blocks_before_unattributed() {
    // Digest guard runs first: an unattributed emergency with no digest reads as
    // missing-digest-join, not unattributed.
    let resolved = resolve_release_history_event(&M5ReleaseHistoryEventInput {
        digest_refs: vec![],
        break_glass_posture: M5BreakGlassPosture::BreakGlassUnattributed,
        approving_actors: vec![],
        ..rollback_input("event:r7")
    })
    .expect("resolves");
    assert_eq!(
        resolved.history_posture,
        M5ReleaseHistoryPosture::HistoryBlockedMissingDigestJoin
    );
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_identity = M5ReleaseHistoryEventInput {
        event_identity_repr: "  ".to_owned(),
        ..promotion_input("event:p")
    };
    assert_eq!(
        resolve_release_history_event(&empty_identity),
        Err(M5ReleaseHistoryEventError::EmptyEventIdentity)
    );

    let empty_time = M5ReleaseHistoryEventInput {
        effective_time_repr: String::new(),
        ..promotion_input("event:p")
    };
    assert_eq!(
        resolve_release_history_event(&empty_time),
        Err(M5ReleaseHistoryEventError::EmptyEffectiveTime)
    );

    let forbidden = M5ReleaseHistoryEventInput {
        affected_node_set: vec!["https://mirror.example/node".to_owned()],
        ..rollback_input("event:r")
    };
    assert_eq!(
        resolve_release_history_event(&forbidden),
        Err(M5ReleaseHistoryEventError::ForbiddenHistoryMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_release_history_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_RELEASE_HISTORY_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_release_history_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .history_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5ReleaseHistoryConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.history_rows.len(),
        M5ReleaseHistoryConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_release_history_primitive_packet();
    for row in &packet.history_rows {
        for part in M5ReleaseHistoryAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5ReleaseHistoryExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ReleaseCenterAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_resolutions.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_release_history_primitive_packet();
    let cases: Vec<&M5ReleaseHistoryResolutionCase> = packet
        .history_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for posture in M5ReleaseHistoryPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.history_posture == posture),
            "no worked resolution exercises history posture {}",
            posture.as_str()
        );
    }
    for kind in M5ReleaseHistoryEventKind::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.event_kind == kind),
            "no worked resolution exercises event kind {}",
            kind.as_str()
        );
    }
    for stage in M5PromotionStageState::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .promotion_view
                .as_ref()
                .is_some_and(|v| v.stage_state == stage)),
            "no worked resolution exercises promotion stage state {}",
            stage.as_str()
        );
    }
    for window in M5ReversibleWindowState::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .promotion_view
                .as_ref()
                .is_some_and(|v| v.reversible_window == window)),
            "no worked resolution exercises reversible window {}",
            window.as_str()
        );
    }
    for ring in M5RolloutRing::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.rollout_ring == ring),
            "no worked resolution exercises rollout ring {}",
            ring.as_str()
        );
    }
    for blast in M5RollbackBlastRadius::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .rollback_view
                .as_ref()
                .is_some_and(|v| v.blast_radius == blast)),
            "no worked resolution exercises blast radius {}",
            blast.as_str()
        );
    }
    for scope in M5RevocationScope::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .rollback_view
                .as_ref()
                .is_some_and(|v| v.revocation_scope == scope)),
            "no worked resolution exercises revocation scope {}",
            scope.as_str()
        );
    }
    for targeting in M5NodeTargeting::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.node_targeting == targeting),
            "no worked resolution exercises node targeting {}",
            targeting.as_str()
        );
    }
    for posture in M5BreakGlassPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.break_glass_posture == posture),
            "no worked resolution exercises break-glass posture {}",
            posture.as_str()
        );
    }
    for reason in M5ReleaseHistoryBlockReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .history_banner
                .as_ref()
                .is_some_and(|b| b.reason == reason)),
            "no worked resolution exercises block reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_release_history_primitive_packet();
    for row in &packet.history_rows {
        for case in &row.example_resolutions {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet
        .history_rows
        .retain(|row| row.consumer_surface != M5ReleaseHistoryConsumerSurface::CliHistoryInspect);
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.vocabulary_set.history_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.history_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ReleaseHistoryAnatomyPart::HistoryVerdict);
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.history_rows[0]
        .export_fields
        .retain(|f| *f != M5ReleaseHistoryExportField::DigestRefs);
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.history_rows[0].example_resolutions[0]
        .resolved
        .is_recorded = false;
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn example_resolution_missing_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.history_rows[2].example_resolutions.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::ExampleResolutionMissing));
}

#[test]
fn history_coverage_unproven_fails_without_blocked_example() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    for row in &mut packet.history_rows {
        row.example_resolutions = vec![
            M5ReleaseHistoryResolutionCase::resolved(promotion_input("event:p")),
            M5ReleaseHistoryResolutionCase::resolved(rollback_input("event:r")),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::HistoryCoverageUnproven));
}

#[test]
fn rollback_not_generic_unproven_fails_without_example() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    // Every rollback single-artifact scoped with no partial targeting.
    for row in &mut packet.history_rows {
        row.example_resolutions = vec![
            M5ReleaseHistoryResolutionCase::resolved(promotion_input("event:p")),
            M5ReleaseHistoryResolutionCase::resolved(M5ReleaseHistoryEventInput {
                blast_radius: M5RollbackBlastRadius::SingleArtifact,
                node_targeting: M5NodeTargeting::AllNodes,
                break_glass_posture: M5BreakGlassPosture::BreakGlassUnattributed,
                approving_actors: vec![],
                ..rollback_input("event:r")
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::RollbackNotGenericUnproven));
}

#[test]
fn emergency_visible_in_history_unproven_fails_without_example() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    for row in &mut packet.history_rows {
        row.example_resolutions = vec![
            M5ReleaseHistoryResolutionCase::resolved(promotion_input("event:p")),
            M5ReleaseHistoryResolutionCase::resolved(M5ReleaseHistoryEventInput {
                blast_radius: M5RollbackBlastRadius::FamilyScoped,
                ..rollback_input("event:r")
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::EmergencyVisibleInHistoryUnproven));
}

#[test]
fn break_glass_attribution_unproven_fails_without_example() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    // No unattributed break-glass example anywhere.
    for row in &mut packet.history_rows {
        row.example_resolutions = vec![
            M5ReleaseHistoryResolutionCase::resolved(M5ReleaseHistoryEventInput {
                break_glass_posture: M5BreakGlassPosture::BreakGlassAttributed,
                ..promotion_input("event:p")
            }),
            M5ReleaseHistoryResolutionCase::resolved(M5ReleaseHistoryEventInput {
                stage_state: M5PromotionStageState::StageBlocked,
                ..promotion_input("event:p2")
            }),
            M5ReleaseHistoryResolutionCase::resolved(rollback_input("event:r")),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::BreakGlassAttributionUnproven));
}

#[test]
fn reversible_window_unproven_fails_without_example() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    // No irreversible promotion example anywhere.
    for row in &mut packet.history_rows {
        row.example_resolutions = vec![
            M5ReleaseHistoryResolutionCase::resolved(promotion_input("event:p")),
            M5ReleaseHistoryResolutionCase::resolved(M5ReleaseHistoryEventInput {
                break_glass_posture: M5BreakGlassPosture::BreakGlassUnattributed,
                approving_actors: vec![],
                ..rollback_input("event:r")
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::ReversibleWindowUnproven));
}

#[test]
fn history_invariant_violation_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.history_rows[0].reads_rollback_as_generic_status = true;
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::HistoryInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.history_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet
        .governance_review
        .emergency_stays_visible_in_history_model = false;
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.consumer_projection.rollback_view_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ReleaseHistoryPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_release_history_primitive_packet().render_markdown_summary();
    for surface in M5ReleaseHistoryConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_release_history_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ReleaseHistoryConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5ReleaseHistoryConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_release_history_primitive_export()
        .expect("checked M5 release-history primitive export validates");
    assert_eq!(from_disk.packet_id, M5_RELEASE_HISTORY_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_release_history_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_release_history_primitive_update_center_history_beta_narrowed(),
        seeded_m5_release_history_primitive_cli_history_inspect_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.history_rows.len(),
            M5ReleaseHistoryConsumerSurface::ALL.len()
        );
    }

    let update = seeded_m5_release_history_primitive_update_center_history_beta_narrowed();
    let row = update
        .history_rows
        .iter()
        .find(|r| r.consumer_surface == M5ReleaseHistoryConsumerSurface::UpdateCenterHistory)
        .expect("update-center row present");
    assert_eq!(row.qualification, M5ReleaseCenterQualificationClass::Beta);

    let cli = seeded_m5_release_history_primitive_cli_history_inspect_preview_narrowed();
    let row = cli
        .history_rows
        .iter()
        .find(|r| r.consumer_surface == M5ReleaseHistoryConsumerSurface::CliHistoryInspect)
        .expect("cli row present");
    assert_eq!(row.qualification, M5ReleaseCenterQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let update: M5ReleaseHistoryPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-promotion-timeline-and-rollback-revocation-primitive/update_center_history_beta_narrowed.json"
    )))
    .expect("update-center fixture parses");
    assert!(update.validate().is_empty());
    assert_eq!(
        update,
        seeded_m5_release_history_primitive_update_center_history_beta_narrowed()
    );

    let cli: M5ReleaseHistoryPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-promotion-timeline-and-rollback-revocation-primitive/cli_history_inspect_preview_narrowed.json"
    )))
    .expect("cli fixture parses");
    assert!(cli.validate().is_empty());
    assert_eq!(
        cli,
        seeded_m5_release_history_primitive_cli_history_inspect_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_release_history_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
