use super::*;

fn publishable_input(prior: &str, next: &str) -> M5PublicationReviewInput {
    M5PublicationReviewInput {
        proposal_label: format!("aureline release {next}"),
        prior_version_repr: prior.to_owned(),
        next_version_repr: next.to_owned(),
        version_bump_class: M5VersionBumpClass::Minor,
        compatibility_impact: M5CompatibilityImpact::BackwardCompatible,
        changed_artifact_set: vec!["artifact:core-runtime".to_owned()],
        target_class: M5PublishTargetClass::RegistryTarget,
        visibility: M5PublishTargetVisibility::PublicListed,
        mutability: M5TargetMutability::AppendOnly,
        auth_source: M5TargetAuthSource::CiFederatedIdentity,
        auth_disclosure_state: M5AuthDisclosureState::AuthScopedDisclosed,
        dry_run: M5DryRunAvailability::DryRunSupported,
        rollout_ring: M5RolloutRing::BroadRing,
        surface_impact_analysis: M5SurfaceImpactAnalysis::SurfaceImpactFresh,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_clean_publication_is_publishable_with_no_banner() {
    let resolved =
        resolve_publication_review(&publishable_input("5.1.4", "5.2.0")).expect("resolves");
    assert_eq!(resolved.readiness, M5PublicationReadiness::Publishable);
    assert!(resolved.is_publishable);
    assert!(!resolved.is_blocked);
    assert!(!resolved.is_narrowed);
    assert!(resolved.publication_banner.is_none());
    assert_eq!(
        resolved.destination_reversibility,
        M5DestinationReversibility::DryRunProven
    );
    assert_eq!(
        resolved.public_surface_impact,
        M5PublicSurfaceImpact::AdditivePublicSurface
    );
    assert_eq!(resolved.changed_artifact_count, 1);
}

#[test]
fn resolver_ambient_credential_blocks_with_self_contained_banner() {
    let input = M5PublicationReviewInput {
        auth_disclosure_state: M5AuthDisclosureState::AmbientCredentialInherited,
        ..publishable_input("5.1.4", "6.0.0")
    };
    let resolved = resolve_publication_review(&input).expect("resolves");
    assert_eq!(
        resolved.readiness,
        M5PublicationReadiness::BlockedAmbientCredential
    );
    assert!(resolved.is_blocked);
    let banner = resolved.publication_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5PublicationBlockReason::AmbientCredentialInheritance
    );
    assert_eq!(
        banner.next_action,
        M5PublicationNextAction::DiscloseAuthSource
    );
    assert_eq!(
        banner.blocked_target_class,
        M5PublishTargetClass::RegistryTarget
    );
    assert!(!banner.changed_artifact_set.is_empty());
    assert!(!banner.headline.trim().is_empty());
    // The banner is not a generic "cannot publish".
    assert!(banner.headline.to_lowercase().contains("credential"));
}

#[test]
fn resolver_stale_and_missing_surface_analysis_block_with_distinct_reasons() {
    let stale = resolve_publication_review(&M5PublicationReviewInput {
        surface_impact_analysis: M5SurfaceImpactAnalysis::SurfaceImpactStale,
        ..publishable_input("5.1.4", "5.2.0")
    })
    .expect("resolves");
    assert_eq!(
        stale.readiness,
        M5PublicationReadiness::BlockedSurfaceImpactStale
    );
    assert_eq!(
        stale.publication_banner.unwrap().reason,
        M5PublicationBlockReason::SurfaceImpactStale
    );

    let missing = resolve_publication_review(&M5PublicationReviewInput {
        surface_impact_analysis: M5SurfaceImpactAnalysis::SurfaceImpactMissing,
        ..publishable_input("5.1.4", "5.2.1")
    })
    .expect("resolves");
    assert_eq!(
        missing.readiness,
        M5PublicationReadiness::BlockedSurfaceImpactMissing
    );
    assert_eq!(
        missing.publication_banner.unwrap().next_action,
        M5PublicationNextAction::ProvideSurfaceImpact
    );
}

#[test]
fn resolver_unknown_state_blocks_first() {
    let input = M5PublicationReviewInput {
        auth_disclosure_state: M5AuthDisclosureState::AuthDisclosureUnknown,
        surface_impact_analysis: M5SurfaceImpactAnalysis::SurfaceImpactUnknown,
        mutability: M5TargetMutability::ImmutableOncePublished,
        dry_run: M5DryRunAvailability::DryRunUnavailable,
        ..publishable_input("5.2.9", "5.3.0")
    };
    let resolved = resolve_publication_review(&input).expect("resolves");
    assert_eq!(
        resolved.readiness,
        M5PublicationReadiness::BlockedUnknownState
    );
    // An immutable target with no dry-run is immutable by design, not "unproven".
    assert_eq!(
        resolved.destination_reversibility,
        M5DestinationReversibility::ImmutableByDesign
    );
}

#[test]
fn resolver_unproven_reversibility_narrows_not_read_as_immutable() {
    let input = M5PublicationReviewInput {
        mutability: M5TargetMutability::MutableTagRepointable,
        dry_run: M5DryRunAvailability::DryRunUnavailable,
        ..publishable_input("4.9.6", "4.9.7")
    };
    let resolved = resolve_publication_review(&input).expect("resolves");
    assert_eq!(
        resolved.readiness,
        M5PublicationReadiness::NarrowedReversibilityUnproven
    );
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.destination_reversibility,
        M5DestinationReversibility::ReversibilityUnproven
    );
    assert_eq!(
        resolved.publication_banner.unwrap().reason,
        M5PublicationBlockReason::DestinationReversibilityUnproven
    );
    // A mutable target is never silently treated as immutable.
    assert_ne!(
        resolved.destination_reversibility,
        M5DestinationReversibility::ImmutableByDesign
    );
}

#[test]
fn resolver_waiver_and_review_stay_publishable() {
    let waiver = resolve_publication_review(&M5PublicationReviewInput {
        auth_disclosure_state: M5AuthDisclosureState::AuthDisclosedUnderWaiver,
        dry_run: M5DryRunAvailability::DryRunRequiredBeforePublish,
        ..publishable_input("5.9.9", "6.0.0")
    })
    .expect("resolves");
    assert_eq!(
        waiver.readiness,
        M5PublicationReadiness::PublishableDryRunFirst
    );
    assert!(waiver.is_publishable);

    let aging = resolve_publication_review(&M5PublicationReviewInput {
        surface_impact_analysis: M5SurfaceImpactAnalysis::SurfaceImpactAging,
        ..publishable_input("5.1.4", "5.2.0")
    })
    .expect("resolves");
    assert_eq!(
        aging.readiness,
        M5PublicationReadiness::PublishableWithReview
    );

    let broad = resolve_publication_review(&M5PublicationReviewInput {
        auth_disclosure_state: M5AuthDisclosureState::AuthBroadDisclosed,
        ..publishable_input("5.1.4", "5.2.1")
    })
    .expect("resolves");
    assert_eq!(
        broad.readiness,
        M5PublicationReadiness::PublishableWithReview
    );
}

#[test]
fn resolver_public_surface_impact_is_derived_not_collapsed() {
    let breaking = resolve_publication_review(&M5PublicationReviewInput {
        compatibility_impact: M5CompatibilityImpact::BreakingChange,
        ..publishable_input("5.1.4", "6.0.0")
    })
    .expect("resolves");
    assert_eq!(
        breaking.public_surface_impact,
        M5PublicSurfaceImpact::BreakingPublicSurface
    );

    let migration = resolve_publication_review(&M5PublicationReviewInput {
        compatibility_impact: M5CompatibilityImpact::SchemaMigrationRequired,
        ..publishable_input("5.1.4", "6.0.0")
    })
    .expect("resolves");
    assert_eq!(
        migration.public_surface_impact,
        M5PublicSurfaceImpact::MigrationRequiredPublicSurface
    );

    let republish = resolve_publication_review(&M5PublicationReviewInput {
        version_bump_class: M5VersionBumpClass::RepublishNoVersionChange,
        next_version_repr: "5.1.4".to_owned(),
        ..publishable_input("5.1.4", "5.1.4")
    })
    .expect("resolves");
    assert_eq!(
        republish.public_surface_impact,
        M5PublicSurfaceImpact::NoPublicSurfaceChange
    );
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_label = M5PublicationReviewInput {
        proposal_label: "  ".to_owned(),
        ..publishable_input("5.1.4", "5.2.0")
    };
    assert_eq!(
        resolve_publication_review(&empty_label),
        Err(M5PublicationReviewError::EmptyProposalLabel)
    );

    let empty_version = M5PublicationReviewInput {
        next_version_repr: "".to_owned(),
        ..publishable_input("5.1.4", "5.2.0")
    };
    assert_eq!(
        resolve_publication_review(&empty_version),
        Err(M5PublicationReviewError::EmptyVersion)
    );

    let empty_artifacts = M5PublicationReviewInput {
        changed_artifact_set: vec![],
        ..publishable_input("5.1.4", "5.2.0")
    };
    assert_eq!(
        resolve_publication_review(&empty_artifacts),
        Err(M5PublicationReviewError::EmptyChangedArtifactSet)
    );

    let same_version = M5PublicationReviewInput {
        prior_version_repr: "5.2.0".to_owned(),
        next_version_repr: "5.2.0".to_owned(),
        version_bump_class: M5VersionBumpClass::Minor,
        ..publishable_input("5.1.4", "5.2.0")
    };
    assert_eq!(
        resolve_publication_review(&same_version),
        Err(M5PublicationReviewError::NextVersionEqualsPriorForBump)
    );

    let forbidden = M5PublicationReviewInput {
        changed_artifact_set: vec!["https://example.test/artifact".to_owned()],
        ..publishable_input("5.1.4", "5.2.0")
    };
    assert_eq!(
        resolve_publication_review(&forbidden),
        Err(M5PublicationReviewError::ForbiddenPublicationMaterial)
    );
}

#[test]
fn resolver_republish_no_version_change_allows_equal_versions() {
    let resolved = resolve_publication_review(&M5PublicationReviewInput {
        prior_version_repr: "5.1.4".to_owned(),
        next_version_repr: "5.1.4".to_owned(),
        version_bump_class: M5VersionBumpClass::RepublishNoVersionChange,
        ..publishable_input("5.1.4", "5.1.4")
    })
    .expect("republish resolves");
    assert!(resolved.is_publishable);
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_publication_review_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_PUBLICATION_REVIEW_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_publication_review_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .publication_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5PublicationReviewConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.publication_rows.len(),
        M5PublicationReviewConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_publication_review_primitive_packet();
    for row in &packet.publication_rows {
        for part in M5PublicationReviewAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5PublicationExportField::MANDATORY {
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
    let packet = seeded_m5_publication_review_primitive_packet();
    let cases: Vec<&M5PublicationReviewResolutionCase> = packet
        .publication_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for posture in M5PublicationReadiness::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.readiness == posture),
            "no worked resolution exercises readiness {}",
            posture.as_str()
        );
    }
    for state in M5SurfaceImpactAnalysis::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.surface_impact_analysis == state),
            "no worked resolution exercises surface analysis {}",
            state.as_str()
        );
    }
    for reversibility in M5DestinationReversibility::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.destination_reversibility == reversibility),
            "no worked resolution exercises reversibility {}",
            reversibility.as_str()
        );
    }
    for impact in M5PublicSurfaceImpact::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.public_surface_impact == impact),
            "no worked resolution exercises public-surface impact {}",
            impact.as_str()
        );
    }
    for bump in M5VersionBumpClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.version_bump_class == bump),
            "no worked resolution exercises version-bump class {}",
            bump.as_str()
        );
    }
    for target in M5PublishTargetClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.target_class == target),
            "no worked resolution exercises target class {}",
            target.as_str()
        );
    }
    for auth in M5TargetAuthSource::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.auth_source == auth),
            "no worked resolution exercises auth source {}",
            auth.as_str()
        );
    }
    for reason in M5PublicationBlockReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .publication_banner
                .as_ref()
                .is_some_and(|b| b.reason == reason)),
            "no worked resolution exercises block reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_publication_review_primitive_packet();
    for row in &packet.publication_rows {
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
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.publication_rows.retain(|row| {
        row.consumer_surface != M5PublicationReviewConsumerSurface::CliPublishInspect
    });
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.vocabulary_set.readiness_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.publication_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5PublicationReviewAnatomyPart::AuthSourceDisclosure);
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.publication_rows[0]
        .export_fields
        .retain(|f| *f != M5PublicationExportField::AuthSource);
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.publication_rows[0].example_resolutions[0]
        .resolved
        .is_publishable = false;
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn example_resolution_missing_fails() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.publication_rows[2].example_resolutions.clear();
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::ExampleResolutionMissing));
}

#[test]
fn publishability_coverage_unproven_fails_when_no_blocked_example_present() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    // Replace every example with a clean publishable one so the coverage lint fires.
    for row in &mut packet.publication_rows {
        row.example_resolutions = vec![M5PublicationReviewResolutionCase::resolved(
            publishable_input("5.9.8", "5.9.9"),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::PublishabilityCoverageUnproven));
}

#[test]
fn ambient_credential_surfaced_unproven_fails_when_no_ambient_example_present() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    for row in &mut packet.publication_rows {
        row.example_resolutions = vec![
            M5PublicationReviewResolutionCase::resolved(publishable_input("5.9.8", "5.9.9")),
            M5PublicationReviewResolutionCase::resolved(M5PublicationReviewInput {
                surface_impact_analysis: M5SurfaceImpactAnalysis::SurfaceImpactStale,
                ..publishable_input("5.9.8", "6.0.0")
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::AmbientCredentialSurfacedUnproven));
}

#[test]
fn mutability_and_dry_run_explicit_unproven_fails_without_immutable_example() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    // Every example dry-run-proven, none immutable-by-design.
    for row in &mut packet.publication_rows {
        row.example_resolutions = vec![
            M5PublicationReviewResolutionCase::resolved(publishable_input("5.9.8", "5.9.9")),
            M5PublicationReviewResolutionCase::resolved(M5PublicationReviewInput {
                auth_disclosure_state: M5AuthDisclosureState::AmbientCredentialInherited,
                ..publishable_input("5.9.8", "6.0.0")
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::MutabilityAndDryRunExplicitUnproven));
}

#[test]
fn publication_invariant_violation_fails() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.publication_rows[0].masks_target_auth_source_or_destination_class = true;
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::PublicationInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.publication_rows[0]
        .required_proof_packet_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet
        .governance_review
        .ambient_credentials_never_inherited_silently = false;
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet
        .consumer_projection
        .auth_source_disclosure_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5PublicationReviewPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_publication_review_primitive_packet().render_markdown_summary();
    for surface in M5PublicationReviewConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_publication_review_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5PublicationReviewConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5PublicationReviewConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_publication_review_primitive_export()
        .expect("checked M5 publication-review primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_PUBLICATION_REVIEW_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_publication_review_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_publication_review_primitive_update_center_publish_row_beta_narrowed(),
        seeded_m5_publication_review_primitive_cli_publish_inspect_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.publication_rows.len(),
            M5PublicationReviewConsumerSurface::ALL.len()
        );
    }

    let update = seeded_m5_publication_review_primitive_update_center_publish_row_beta_narrowed();
    let row = update
        .publication_rows
        .iter()
        .find(|r| r.consumer_surface == M5PublicationReviewConsumerSurface::UpdateCenterPublishRow)
        .expect("update-center row present");
    assert_eq!(row.qualification, M5ReleaseCenterQualificationClass::Beta);

    let cli = seeded_m5_publication_review_primitive_cli_publish_inspect_preview_narrowed();
    let row = cli
        .publication_rows
        .iter()
        .find(|r| r.consumer_surface == M5PublicationReviewConsumerSurface::CliPublishInspect)
        .expect("cli row present");
    assert_eq!(
        row.qualification,
        M5ReleaseCenterQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let update: M5PublicationReviewPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-publish-target-review-sheet-primitive/update_center_publish_row_beta_narrowed.json"
    )))
    .expect("update-center fixture parses");
    assert!(update.validate().is_empty());
    assert_eq!(
        update,
        seeded_m5_publication_review_primitive_update_center_publish_row_beta_narrowed()
    );

    let cli: M5PublicationReviewPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-publish-target-review-sheet-primitive/cli_publish_inspect_preview_narrowed.json"
    )))
    .expect("cli fixture parses");
    assert!(cli.validate().is_empty());
    assert_eq!(
        cli,
        seeded_m5_publication_review_primitive_cli_publish_inspect_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_publication_review_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
