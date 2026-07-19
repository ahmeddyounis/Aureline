use super::*;

fn local_apply(title: &str) -> M5RepairActionResolutionInput {
    M5RepairActionResolutionInput {
        repair_title: title.to_owned(),
        repair_class: M5RepairClass::RepairEnvironmentConfig,
        target_scope_repr: "scope:workspace-env".to_owned(),
        blast_radius: M5RepairBlastRadius::WorkspaceScoped,
        host_boundary: M5HostBoundaryClass::LocalHost,
        reversibility: M5ReversibilityClass::FullyReversibleCheckpoint,
        trust_requirement: M5RepairTrustRequirement::NoElevation,
        changed_classes: vec![M5RepairChangeClass::WorkspaceConfig],
        unchanged_classes: vec![M5RepairChangeClass::UserSourceFiles],
        preview_only: false,
        approval_required: false,
        rerunnable: true,
        factory_reset_out_of_band: false,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_local_exact_reversible_reads_as_ordinary_apply() {
    let resolved = resolve_repair_action(&local_apply("x")).expect("resolves");
    assert_eq!(
        resolved.target_boundary,
        M5RepairTargetBoundary::LocalTarget
    );
    assert!(resolved.reversal_is_exact);
    assert!(!resolved.requires_approval);
    assert_eq!(
        resolved.action_label_class,
        M5RepairActionLabelClass::ApplyLocalReversible
    );
    assert!(!resolved.action_label_class.is_explicit());
    assert!(resolved
        .available_actions
        .contains(&M5RepairAction::ApplyRepair));
    assert!(resolved
        .available_actions
        .contains(&M5RepairAction::RollbackRepair));
    assert!(resolved.blast_radius_reviewable);
    assert!(resolved.changed_and_unchanged_disclosed);
}

#[test]
fn resolver_remote_target_reads_as_off_device_review() {
    let input = M5RepairActionResolutionInput {
        host_boundary: M5HostBoundaryClass::RemoteSshHost,
        ..local_apply("x")
    };
    let resolved = resolve_repair_action(&input).expect("resolves");
    assert_eq!(
        resolved.target_boundary,
        M5RepairTargetBoundary::RemoteTarget
    );
    assert_eq!(
        resolved.action_label_class,
        M5RepairActionLabelClass::ReviewOffDeviceRepair
    );
    assert!(resolved.action_label_class.is_explicit());
}

#[test]
fn resolver_managed_target_derives_managed_boundary() {
    for host in [
        M5HostBoundaryClass::ContainerHost,
        M5HostBoundaryClass::ManagedWorkspaceHost,
        M5HostBoundaryClass::WasmSandboxHost,
    ] {
        let input = M5RepairActionResolutionInput {
            host_boundary: host,
            ..local_apply("x")
        };
        let resolved = resolve_repair_action(&input).expect("resolves");
        assert_eq!(
            resolved.target_boundary,
            M5RepairTargetBoundary::ManagedTarget,
            "host {} did not derive a managed boundary",
            host.as_str()
        );
        assert!(!resolved.target_boundary.is_local());
    }
}

#[test]
fn resolver_policy_gated_repair_requests_approval_not_apply() {
    let input = M5RepairActionResolutionInput {
        trust_requirement: M5RepairTrustRequirement::PolicyApprovalRequired,
        ..local_apply("x")
    };
    let resolved = resolve_repair_action(&input).expect("resolves");
    assert!(resolved.requires_approval);
    assert_eq!(
        resolved.action_label_class,
        M5RepairActionLabelClass::RequestPolicyApproval
    );
    assert!(resolved
        .available_actions
        .contains(&M5RepairAction::RequestApproval));
    assert!(!resolved
        .available_actions
        .contains(&M5RepairAction::ApplyRepair));
}

#[test]
fn resolver_explicit_approval_flag_requests_approval() {
    let input = M5RepairActionResolutionInput {
        approval_required: true,
        ..local_apply("x")
    };
    let resolved = resolve_repair_action(&input).expect("resolves");
    assert!(resolved.requires_approval);
    assert_eq!(
        resolved.action_label_class,
        M5RepairActionLabelClass::RequestPolicyApproval
    );
}

#[test]
fn resolver_non_exact_local_reversal_reads_as_non_exact() {
    let input = M5RepairActionResolutionInput {
        reversibility: M5ReversibilityClass::ReversibleWithBackup,
        ..local_apply("x")
    };
    let resolved = resolve_repair_action(&input).expect("resolves");
    assert!(!resolved.reversal_is_exact);
    assert_eq!(
        resolved.action_label_class,
        M5RepairActionLabelClass::ApplyNonExactRepair
    );
    assert!(resolved.action_label_class.is_explicit());
}

#[test]
fn resolver_factory_reset_opens_out_of_band() {
    let input = M5RepairActionResolutionInput {
        repair_class: M5RepairClass::FactoryResetComponent,
        blast_radius: M5RepairBlastRadius::MultiTargetScoped,
        reversibility: M5ReversibilityClass::ReversalRequiresManualSteps,
        factory_reset_out_of_band: true,
        ..local_apply("x")
    };
    let resolved = resolve_repair_action(&input).expect("resolves");
    assert_eq!(
        resolved.action_label_class,
        M5RepairActionLabelClass::OpenFactoryResetOutOfBand
    );
    assert!(resolved
        .available_actions
        .contains(&M5RepairAction::OpenFactoryReset));
    assert!(!resolved
        .available_actions
        .contains(&M5RepairAction::ApplyRepair));
    assert!(!resolved
        .available_actions
        .contains(&M5RepairAction::RollbackRepair));
}

#[test]
fn resolver_preview_only_stays_preview_and_offers_no_apply() {
    let input = M5RepairActionResolutionInput {
        blast_radius: M5RepairBlastRadius::NoWritesPreview,
        changed_classes: vec![],
        unchanged_classes: vec![M5RepairChangeClass::WorkspaceConfig],
        preview_only: true,
        ..local_apply("x")
    };
    let resolved = resolve_repair_action(&input).expect("resolves");
    assert_eq!(
        resolved.action_label_class,
        M5RepairActionLabelClass::PreviewOnly
    );
    assert!(resolved
        .available_actions
        .contains(&M5RepairAction::PreviewRepair));
    assert!(!resolved
        .available_actions
        .contains(&M5RepairAction::ApplyRepair));
}

#[test]
fn resolver_preview_action_always_available() {
    for host in M5HostBoundaryClass::ALL {
        let input = M5RepairActionResolutionInput {
            host_boundary: host,
            ..local_apply("x")
        };
        let resolved = resolve_repair_action(&input).expect("resolves");
        assert!(
            resolved
                .available_actions
                .contains(&M5RepairAction::PreviewRepair),
            "host {} dropped the preview action",
            host.as_str()
        );
        assert!(resolved.blast_radius_reviewable);
        assert!(resolved
            .available_actions
            .contains(&M5RepairAction::CancelRepair));
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_title = M5RepairActionResolutionInput {
        repair_title: "  ".to_owned(),
        ..local_apply("x")
    };
    assert_eq!(
        resolve_repair_action(&empty_title),
        Err(M5RepairActionResolutionError::EmptyRepairTitle)
    );

    let empty_scope = M5RepairActionResolutionInput {
        target_scope_repr: "  ".to_owned(),
        ..local_apply("x")
    };
    assert_eq!(
        resolve_repair_action(&empty_scope),
        Err(M5RepairActionResolutionError::EmptyTargetScope)
    );

    let forbidden = M5RepairActionResolutionInput {
        target_scope_repr: "scope:https://example.test".to_owned(),
        ..local_apply("x")
    };
    assert_eq!(
        resolve_repair_action(&forbidden),
        Err(M5RepairActionResolutionError::ForbiddenRepairMaterial)
    );

    let dup = M5RepairActionResolutionInput {
        changed_classes: vec![
            M5RepairChangeClass::WorkspaceConfig,
            M5RepairChangeClass::WorkspaceConfig,
        ],
        ..local_apply("x")
    };
    assert_eq!(
        resolve_repair_action(&dup),
        Err(M5RepairActionResolutionError::DuplicateChangeClass)
    );

    let overlap = M5RepairActionResolutionInput {
        changed_classes: vec![M5RepairChangeClass::WorkspaceConfig],
        unchanged_classes: vec![M5RepairChangeClass::WorkspaceConfig],
        ..local_apply("x")
    };
    assert_eq!(
        resolve_repair_action(&overlap),
        Err(M5RepairActionResolutionError::OverlappingChangeClasses)
    );

    let preview_claims_changes = M5RepairActionResolutionInput {
        blast_radius: M5RepairBlastRadius::NoWritesPreview,
        changed_classes: vec![M5RepairChangeClass::WorkspaceConfig],
        ..local_apply("x")
    };
    assert_eq!(
        resolve_repair_action(&preview_claims_changes),
        Err(M5RepairActionResolutionError::PreviewBlastRadiusClaimsChanges)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_repair_action_card_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_REPAIR_ACTION_CARD_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_repair_action_card_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .consumer_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5RepairConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5RepairConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_repair_action_card_primitive_packet();
    for row in &packet.consumer_rows {
        for part in M5RepairActionCardPart::MANDATORY {
            assert!(row.card_parts.contains(&part));
        }
        for part in M5RepairPreviewRowPart::MANDATORY {
            assert!(row.preview_row_parts.contains(&part));
        }
        for field in M5RepairExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .preview_row_parts
            .contains(&M5RepairPreviewRowPart::ChangedClassList));
        assert!(row
            .preview_row_parts
            .contains(&M5RepairPreviewRowPart::UnchangedClassList));
        assert!(row
            .accessibility_routes
            .contains(&M5RuntimeBoundaryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_resolutions.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_repair_action_card_primitive_packet();
    let cases: Vec<&M5RepairActionResolutionCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for class in M5RepairClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.repair_class == class),
            "no worked resolution exercises repair class {}",
            class.as_str()
        );
    }
    for blast in M5RepairBlastRadius::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.blast_radius == blast),
            "no worked resolution exercises blast radius {}",
            blast.as_str()
        );
    }
    for boundary in M5RepairTargetBoundary::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.target_boundary == boundary),
            "no worked resolution exercises target boundary {}",
            boundary.as_str()
        );
    }
    for rev in M5ReversibilityClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.reversibility == rev),
            "no worked resolution exercises reversibility {}",
            rev.as_str()
        );
    }
    for trust in M5RepairTrustRequirement::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.trust_requirement == trust),
            "no worked resolution exercises trust requirement {}",
            trust.as_str()
        );
    }
    for change in M5RepairChangeClass::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.changed_classes.contains(&change)
                    || c.resolved.unchanged_classes.contains(&change)),
            "no worked resolution exercises change class {}",
            change.as_str()
        );
    }
    for label in M5RepairActionLabelClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.action_label_class == label),
            "no worked resolution exercises action-label class {}",
            label.as_str()
        );
    }
    for action in M5RepairAction::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no worked resolution exercises action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_repair_action_card_primitive_packet();
    for row in &packet.consumer_rows {
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
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer_surface != M5RepairConsumerSurface::RepairPreviewSheet);
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::RequiredSurfaceMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.vocabulary_set.blast_radii.pop();
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_card_part_missing_fails() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.consumer_rows[0]
        .card_parts
        .retain(|p| *p != M5RepairActionCardPart::ReversalClassBadge);
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::MandatoryCardPartMissing));
}

#[test]
fn mandatory_preview_row_part_missing_fails() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.consumer_rows[0]
        .preview_row_parts
        .retain(|p| *p != M5RepairPreviewRowPart::UnchangedClassList);
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::MandatoryPreviewRowPartMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5RepairExportField::UnchangedClasses);
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.consumer_rows[0].example_resolutions[0]
        .resolved
        .action_label_class = M5RepairActionLabelClass::OpenFactoryResetOutOfBand;
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn blast_radius_review_unproven_fails_when_all_examples_are_previews() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    for row in &mut packet.consumer_rows {
        row.example_resolutions = vec![M5RepairActionResolutionCase::resolved(
            M5RepairActionResolutionInput {
                repair_title: "preview".to_owned(),
                repair_class: M5RepairClass::RebuildIndex,
                target_scope_repr: "scope:index".to_owned(),
                blast_radius: M5RepairBlastRadius::NoWritesPreview,
                host_boundary: M5HostBoundaryClass::LocalHost,
                reversibility: M5ReversibilityClass::FullyReversibleCheckpoint,
                trust_requirement: M5RepairTrustRequirement::NoElevation,
                changed_classes: vec![],
                unchanged_classes: vec![M5RepairChangeClass::IndexData],
                preview_only: true,
                approval_required: false,
                rerunnable: true,
                factory_reset_out_of_band: false,
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::BlastRadiusReviewUnproven));
}

#[test]
fn non_generic_label_unproven_fails_when_all_examples_are_local_exact() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    for row in &mut packet.consumer_rows {
        for case in &mut row.example_resolutions {
            case.input = local_apply("only-local-exact");
            case.resolved = resolve_repair_action(&case.input).unwrap();
        }
    }
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::NonGenericLabelUnproven));
}

#[test]
fn changed_unchanged_disclosure_unproven_fails_when_no_example_has_both() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    for row in &mut packet.consumer_rows {
        for case in &mut row.example_resolutions {
            case.input = M5RepairActionResolutionInput {
                changed_classes: vec![M5RepairChangeClass::WorkspaceConfig],
                unchanged_classes: vec![],
                ..local_apply("changed-only")
            };
            case.resolved = resolve_repair_action(&case.input).unwrap();
        }
    }
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::ChangedUnchangedDisclosureUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.consumer_rows[0].understates_blast_radius = true;
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.governance_review.reversibility_never_overstated = false;
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet
        .consumer_projection
        .target_boundary_reads_single_host_source = false;
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5RepairActionCardPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_repair_action_card_primitive_packet().render_markdown_summary();
    for surface in M5RepairConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer surface {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_surface() {
    let csv = seeded_m5_repair_action_card_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5RepairConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5RepairConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_repair_action_card_primitive_export()
        .expect("checked M5 repair-action-card primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_REPAIR_ACTION_CARD_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_repair_action_card_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_surfaces_visible() {
    for packet in [
        seeded_m5_repair_action_card_primitive_remote_host_repair_card_beta_narrowed(),
        seeded_m5_repair_action_card_primitive_repair_preview_sheet_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5RepairConsumerSurface::ALL.len()
        );
    }

    let remote = seeded_m5_repair_action_card_primitive_remote_host_repair_card_beta_narrowed();
    let row = remote
        .consumer_rows
        .iter()
        .find(|r| r.consumer_surface == M5RepairConsumerSurface::RemoteHostRepairCard)
        .expect("remote-host repair card row present");
    assert_eq!(row.qualification, M5RuntimeBoundaryQualificationClass::Beta);

    let preview = seeded_m5_repair_action_card_primitive_repair_preview_sheet_preview_narrowed();
    let row = preview
        .consumer_rows
        .iter()
        .find(|r| r.consumer_surface == M5RepairConsumerSurface::RepairPreviewSheet)
        .expect("repair preview sheet row present");
    assert_eq!(
        row.qualification,
        M5RuntimeBoundaryQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let remote: M5RepairActionCardPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-repair-action-card-primitive/remote_host_repair_card_beta_narrowed.json"
    )))
    .expect("remote fixture parses");
    assert!(remote.validate().is_empty());
    assert_eq!(
        remote,
        seeded_m5_repair_action_card_primitive_remote_host_repair_card_beta_narrowed()
    );

    let preview: M5RepairActionCardPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-repair-action-card-primitive/repair_preview_sheet_preview_narrowed.json"
    )))
    .expect("preview fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_repair_action_card_primitive_repair_preview_sheet_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_repair_action_card_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
