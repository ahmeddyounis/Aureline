use super::*;

fn promotable_input(version: &str) -> M5ReleaseCandidateResolutionInput {
    M5ReleaseCandidateResolutionInput {
        candidate_label: format!("aureline release {version}"),
        version_repr: version.to_owned(),
        channel_family: M5CandidateChannelFamily::StableChannel,
        scope_class: M5CandidateScopeClass::MultiFamilyCandidate,
        artifact_set: vec!["artifact:core-runtime".to_owned()],
        blocker_state: M5CandidateBlockerState::NoBlockers,
        evidence_freshness: M5EvidenceFreshnessState::EvidenceFresh,
        known_issue_classes: vec![],
        rollback_target_repr: Some("5.1.4".to_owned()),
        rollback_blast_radius: M5RollbackBlastRadius::TrainScoped,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_clean_candidate_is_promotable_with_no_banner() {
    let resolved = resolve_release_candidate(&promotable_input("5.2.0-rc.1")).expect("resolves");
    assert_eq!(resolved.promotability, M5CandidatePromotability::Promotable);
    assert!(resolved.is_promotable);
    assert!(!resolved.is_blocked);
    assert!(!resolved.is_narrowed);
    assert!(resolved.promotion_banner.is_none());
    assert_eq!(
        resolved.rollback_path_readiness,
        M5RollbackPathReadiness::RollbackTargetPinned
    );
    assert_eq!(resolved.artifact_count, 1);
}

#[test]
fn resolver_hard_blocker_blocks_with_self_contained_banner() {
    let input = M5ReleaseCandidateResolutionInput {
        blocker_state: M5CandidateBlockerState::HardBlockerOpen,
        ..promotable_input("5.2.0-rc.2")
    };
    let resolved = resolve_release_candidate(&input).expect("resolves");
    assert_eq!(
        resolved.promotability,
        M5CandidatePromotability::BlockedHardBlocker
    );
    assert!(resolved.is_blocked);
    let banner = resolved.promotion_banner.expect("banner present");
    assert_eq!(banner.reason, M5PromotionBlockReason::HardBlockerOpen);
    assert_eq!(
        banner.next_action,
        M5PromotionNextAction::ResolveHardBlocker
    );
    assert_eq!(
        banner.blocked_scope_class,
        M5CandidateScopeClass::MultiFamilyCandidate
    );
    assert!(!banner.blocked_artifact_set.is_empty());
    assert!(!banner.headline.trim().is_empty());
    // The banner is not a generic "cannot promote".
    assert!(banner.headline.to_lowercase().contains("blocker"));
}

#[test]
fn resolver_stale_and_missing_evidence_block_with_distinct_reasons() {
    let stale = resolve_release_candidate(&M5ReleaseCandidateResolutionInput {
        evidence_freshness: M5EvidenceFreshnessState::EvidenceStale,
        ..promotable_input("5.2.0-rc.3")
    })
    .expect("resolves");
    assert_eq!(
        stale.promotability,
        M5CandidatePromotability::BlockedStaleEvidence
    );
    assert_eq!(
        stale.promotion_banner.unwrap().reason,
        M5PromotionBlockReason::EvidenceStale
    );

    let missing = resolve_release_candidate(&M5ReleaseCandidateResolutionInput {
        evidence_freshness: M5EvidenceFreshnessState::EvidenceMissing,
        ..promotable_input("5.2.0-rc.4")
    })
    .expect("resolves");
    assert_eq!(
        missing.promotability,
        M5CandidatePromotability::BlockedMissingEvidence
    );
    assert_eq!(
        missing.promotion_banner.unwrap().next_action,
        M5PromotionNextAction::ProvideEvidence
    );
}

#[test]
fn resolver_unknown_state_blocks_first() {
    let input = M5ReleaseCandidateResolutionInput {
        blocker_state: M5CandidateBlockerState::BlockerStateUnknown,
        evidence_freshness: M5EvidenceFreshnessState::EvidenceFreshnessUnknown,
        channel_family: M5CandidateChannelFamily::NightlyChannel,
        rollback_target_repr: None,
        ..promotable_input("5.3.0-0.nightly")
    };
    let resolved = resolve_release_candidate(&input).expect("resolves");
    assert_eq!(
        resolved.promotability,
        M5CandidatePromotability::BlockedUnknownState
    );
    // A nightly channel with no target has nothing to roll back to.
    assert_eq!(
        resolved.rollback_path_readiness,
        M5RollbackPathReadiness::NoPriorToRollBackTo
    );
}

#[test]
fn resolver_undefined_rollback_target_narrows_not_inferred_from_version() {
    let input = M5ReleaseCandidateResolutionInput {
        channel_family: M5CandidateChannelFamily::LtsMaintenanceChannel,
        rollback_target_repr: None,
        ..promotable_input("4.9.7-rc.1")
    };
    let resolved = resolve_release_candidate(&input).expect("resolves");
    assert_eq!(
        resolved.promotability,
        M5CandidatePromotability::NarrowedRollbackUndefined
    );
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.rollback_path_readiness,
        M5RollbackPathReadiness::RollbackTargetUndefined
    );
    assert_eq!(
        resolved.promotion_banner.unwrap().reason,
        M5PromotionBlockReason::RollbackTargetUndefined
    );
    // The rollback target is not silently inferred from the semantic version.
    assert!(resolved.rollback_target_repr.is_none());
}

#[test]
fn resolver_waiver_and_reservations_stay_promotable() {
    let waiver = resolve_release_candidate(&M5ReleaseCandidateResolutionInput {
        blocker_state: M5CandidateBlockerState::BlockerWaived,
        ..promotable_input("5.1.4-hotfix.1")
    })
    .expect("resolves");
    assert_eq!(
        waiver.promotability,
        M5CandidatePromotability::PromotableUnderWaiver
    );
    assert!(waiver.is_promotable);

    let aging = resolve_release_candidate(&M5ReleaseCandidateResolutionInput {
        evidence_freshness: M5EvidenceFreshnessState::EvidenceAging,
        ..promotable_input("5.2.0-rc.5")
    })
    .expect("resolves");
    assert_eq!(
        aging.promotability,
        M5CandidatePromotability::PromotableWithReservations
    );

    let soft = resolve_release_candidate(&M5ReleaseCandidateResolutionInput {
        blocker_state: M5CandidateBlockerState::SoftBlockersOnly,
        ..promotable_input("5.2.0-rc.6")
    })
    .expect("resolves");
    assert_eq!(
        soft.promotability,
        M5CandidatePromotability::PromotableWithReservations
    );
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_label = M5ReleaseCandidateResolutionInput {
        candidate_label: "  ".to_owned(),
        ..promotable_input("5.2.0")
    };
    assert_eq!(
        resolve_release_candidate(&empty_label),
        Err(M5ReleaseCandidateResolutionError::EmptyCandidateLabel)
    );

    let empty_version = M5ReleaseCandidateResolutionInput {
        version_repr: "".to_owned(),
        ..promotable_input("5.2.0")
    };
    assert_eq!(
        resolve_release_candidate(&empty_version),
        Err(M5ReleaseCandidateResolutionError::EmptyVersion)
    );

    let empty_artifacts = M5ReleaseCandidateResolutionInput {
        artifact_set: vec![],
        ..promotable_input("5.2.0")
    };
    assert_eq!(
        resolve_release_candidate(&empty_artifacts),
        Err(M5ReleaseCandidateResolutionError::EmptyArtifactSet)
    );

    let rollback_self = M5ReleaseCandidateResolutionInput {
        rollback_target_repr: Some("5.2.0".to_owned()),
        ..promotable_input("5.2.0")
    };
    assert_eq!(
        resolve_release_candidate(&rollback_self),
        Err(M5ReleaseCandidateResolutionError::RollbackTargetEqualsCandidate)
    );

    let forbidden = M5ReleaseCandidateResolutionInput {
        artifact_set: vec!["https://example.test/artifact".to_owned()],
        ..promotable_input("5.2.0")
    };
    assert_eq!(
        resolve_release_candidate(&forbidden),
        Err(M5ReleaseCandidateResolutionError::ForbiddenCandidateMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_release_candidate_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_RELEASE_CANDIDATE_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_release_candidate_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .candidate_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5ReleaseCandidateConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.candidate_rows.len(),
        M5ReleaseCandidateConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_release_candidate_primitive_packet();
    for row in &packet.candidate_rows {
        for part in M5CandidateCardAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5CandidateCardExportField::MANDATORY {
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
    let packet = seeded_m5_release_candidate_primitive_packet();
    let cases: Vec<&M5ReleaseCandidateResolutionCase> = packet
        .candidate_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for posture in M5CandidatePromotability::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.promotability == posture),
            "no worked resolution exercises promotability {}",
            posture.as_str()
        );
    }
    for state in M5EvidenceFreshnessState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.evidence_freshness == state),
            "no worked resolution exercises evidence freshness {}",
            state.as_str()
        );
    }
    for readiness in M5RollbackPathReadiness::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.rollback_path_readiness == readiness),
            "no worked resolution exercises rollback readiness {}",
            readiness.as_str()
        );
    }
    for channel in M5CandidateChannelFamily::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.channel_family == channel),
            "no worked resolution exercises channel family {}",
            channel.as_str()
        );
    }
    for issue in M5KnownIssueClass::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.known_issue_classes.contains(&issue)),
            "no worked resolution exercises known issue {}",
            issue.as_str()
        );
    }
    for reason in M5PromotionBlockReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .promotion_banner
                .as_ref()
                .is_some_and(|b| b.reason == reason)),
            "no worked resolution exercises block reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_release_candidate_primitive_packet();
    for row in &packet.candidate_rows {
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
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet
        .candidate_rows
        .retain(|row| row.consumer_surface != M5ReleaseCandidateConsumerSurface::CliReleaseInspect);
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.vocabulary_set.promotability_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.candidate_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5CandidateCardAnatomyPart::ScopedArtifactSet);
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.candidate_rows[0]
        .export_fields
        .retain(|f| *f != M5CandidateCardExportField::RollbackTarget);
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.candidate_rows[0].example_resolutions[0]
        .resolved
        .is_promotable = false;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn example_resolution_missing_fails() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.candidate_rows[2].example_resolutions.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::ExampleResolutionMissing));
}

#[test]
fn promotability_coverage_unproven_fails_when_no_blocked_example_present() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    // Replace every example with a clean promotable one so the coverage lint fires.
    for row in &mut packet.candidate_rows {
        row.example_resolutions = vec![M5ReleaseCandidateResolutionCase::resolved(
            promotable_input("5.9.9-rc.1"),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::PromotabilityCoverageUnproven));
}

#[test]
fn candidate_invariant_violation_fails() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.candidate_rows[0].infers_scope_from_semver_alone = true;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::CandidateInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.candidate_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet
        .governance_review
        .stale_or_missing_evidence_never_shown_clear = false;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.consumer_projection.rollback_path_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCandidatePrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_release_candidate_primitive_packet().render_markdown_summary();
    for surface in M5ReleaseCandidateConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_release_candidate_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5ReleaseCandidateConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5ReleaseCandidateConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_release_candidate_primitive_export()
        .expect("checked M5 release-candidate primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_RELEASE_CANDIDATE_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_release_candidate_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_release_candidate_primitive_update_center_card_beta_narrowed(),
        seeded_m5_release_candidate_primitive_cli_release_inspect_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.candidate_rows.len(),
            M5ReleaseCandidateConsumerSurface::ALL.len()
        );
    }

    let update = seeded_m5_release_candidate_primitive_update_center_card_beta_narrowed();
    let row = update
        .candidate_rows
        .iter()
        .find(|r| r.consumer_surface == M5ReleaseCandidateConsumerSurface::UpdateCenterCard)
        .expect("update-center row present");
    assert_eq!(row.qualification, M5ReleaseCenterQualificationClass::Beta);

    let cli = seeded_m5_release_candidate_primitive_cli_release_inspect_preview_narrowed();
    let row = cli
        .candidate_rows
        .iter()
        .find(|r| r.consumer_surface == M5ReleaseCandidateConsumerSurface::CliReleaseInspect)
        .expect("cli row present");
    assert_eq!(
        row.qualification,
        M5ReleaseCenterQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let update: M5ReleaseCandidatePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-release-candidate-card-primitive/update_center_card_beta_narrowed.json"
    )))
    .expect("update-center fixture parses");
    assert!(update.validate().is_empty());
    assert_eq!(
        update,
        seeded_m5_release_candidate_primitive_update_center_card_beta_narrowed()
    );

    let cli: M5ReleaseCandidatePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-release-candidate-card-primitive/cli_release_inspect_preview_narrowed.json"
    )))
    .expect("cli fixture parses");
    assert!(cli.validate().is_empty());
    assert_eq!(
        cli,
        seeded_m5_release_candidate_primitive_cli_release_inspect_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_release_candidate_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
