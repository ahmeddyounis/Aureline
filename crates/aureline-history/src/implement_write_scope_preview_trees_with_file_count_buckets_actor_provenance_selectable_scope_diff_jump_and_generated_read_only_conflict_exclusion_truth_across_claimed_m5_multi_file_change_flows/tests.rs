use super::*;

fn focused_tree() -> M5WriteScopePreviewTreeResolutionInput {
    M5WriteScopePreviewTreeResolutionInput {
        write_scope_class: M5WriteScopeClass::SingleFile,
        mutation_class: M5MutationClass::TextEdit,
        total_file_count: 1,
        included_file_count: 1,
        excluded_file_count: 0,
        distinct_workspace_root_count: 1,
        touches_generated_or_managed: false,
        has_out_of_workspace_target: false,
        has_conflict: false,
        has_policy_blocked: false,
        scope_is_reviewable: true,
        apply_path_ready: true,
        scope_label: "rename scope: main.rs".to_owned(),
    }
}

fn included_node() -> M5WriteScopeFileNodeResolutionInput {
    M5WriteScopeFileNodeResolutionInput {
        change_type: M5WriteScopeChangeType::Modified,
        change_actor: M5WriteScopeChangeActor::HumanEdit,
        content_class: M5WriteScopeFileContentClass::TextSource,
        managed_caveat: M5ManagedFileCaveat::Unmanaged,
        is_policy_blocked: false,
        is_read_only: false,
        has_conflict: false,
        is_out_of_workspace: false,
        opt_out_of_apply: false,
        diff_available: true,
        node_label: "src/main.rs".to_owned(),
    }
}

// ---- write-scope-preview-tree resolver ----------------------------------

#[test]
fn tree_focused_scope_applies_with_no_narrowing() {
    let resolved = resolve_write_scope_preview_tree(&focused_tree()).expect("resolves");
    assert_eq!(resolved.tree_posture, M5WriteScopeTreePosture::FocusedScope);
    assert_eq!(resolved.file_count_bucket, M5WriteScopeFileCountBucket::Single);
    assert!(resolved.can_apply);
    assert!(!resolved.can_narrow);
    assert!(resolved.preserves_all_files);
    assert!(!resolved.understates_scope);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5WriteScopeTreeAction::InspectTree,
            M5WriteScopeTreeAction::ExpandAll,
            M5WriteScopeTreeAction::JumpToDiff,
            M5WriteScopeTreeAction::ApplyScope,
        ]
    );
    assert_eq!(resolved.scope_label, "rename scope: main.rs");
}

#[test]
fn tree_posture_ladder_is_blocking_first() {
    // Blocked wins even over a conflict.
    let blocked = resolve_write_scope_preview_tree(&M5WriteScopePreviewTreeResolutionInput {
        apply_path_ready: false,
        has_conflict: true,
        ..focused_tree()
    })
    .expect("resolves");
    assert_eq!(blocked.tree_posture, M5WriteScopeTreePosture::BlockedScope);
    assert!(!blocked.can_apply);
    assert!(!blocked
        .available_actions
        .contains(&M5WriteScopeTreeAction::ApplyScope));

    // Conflict next.
    let conflict = resolve_write_scope_preview_tree(&M5WriteScopePreviewTreeResolutionInput {
        has_conflict: true,
        ..focused_tree()
    })
    .expect("resolves");
    assert_eq!(conflict.tree_posture, M5WriteScopeTreePosture::ConflictScope);
    assert!(!conflict.can_apply);
    assert!(conflict
        .available_actions
        .contains(&M5WriteScopeTreeAction::ResolveConflict));

    // Out-of-workspace next (wins over a generated caveat).
    let oow = resolve_write_scope_preview_tree(&M5WriteScopePreviewTreeResolutionInput {
        has_out_of_workspace_target: true,
        touches_generated_or_managed: true,
        ..focused_tree()
    })
    .expect("resolves");
    assert_eq!(oow.tree_posture, M5WriteScopeTreePosture::OutOfWorkspaceScope);
    assert!(oow.has_out_of_workspace_target);

    // Generated / managed next.
    let managed = resolve_write_scope_preview_tree(&M5WriteScopePreviewTreeResolutionInput {
        touches_generated_or_managed: true,
        ..focused_tree()
    })
    .expect("resolves");
    assert_eq!(
        managed.tree_posture,
        M5WriteScopeTreePosture::GeneratedManagedScope
    );
    assert!(managed
        .available_actions
        .contains(&M5WriteScopeTreeAction::ExcludeGenerated));

    // Broad next.
    let broad = resolve_write_scope_preview_tree(&M5WriteScopePreviewTreeResolutionInput {
        write_scope_class: M5WriteScopeClass::CrossPackage,
        total_file_count: 12,
        included_file_count: 12,
        excluded_file_count: 0,
        ..focused_tree()
    })
    .expect("resolves");
    assert_eq!(broad.tree_posture, M5WriteScopeTreePosture::BroadScope);
    assert!(broad.can_narrow);
    assert!(broad
        .available_actions
        .contains(&M5WriteScopeTreeAction::NarrowScope));
}

#[test]
fn tree_file_count_buckets_track_the_honest_total() {
    assert_eq!(M5WriteScopeFileCountBucket::from_total(0), M5WriteScopeFileCountBucket::Empty);
    assert_eq!(M5WriteScopeFileCountBucket::from_total(1), M5WriteScopeFileCountBucket::Single);
    assert_eq!(M5WriteScopeFileCountBucket::from_total(5), M5WriteScopeFileCountBucket::Small);
    assert_eq!(M5WriteScopeFileCountBucket::from_total(25), M5WriteScopeFileCountBucket::Medium);
    assert_eq!(M5WriteScopeFileCountBucket::from_total(100), M5WriteScopeFileCountBucket::Large);
    assert_eq!(M5WriteScopeFileCountBucket::from_total(101), M5WriteScopeFileCountBucket::Sweeping);
    // The excluded files still count toward the bucket — the blast radius is never
    // understated by counting only the applied files.
    let mostly_excluded = resolve_write_scope_preview_tree(&M5WriteScopePreviewTreeResolutionInput {
        write_scope_class: M5WriteScopeClass::MultiFile,
        total_file_count: 30,
        included_file_count: 2,
        excluded_file_count: 28,
        ..focused_tree()
    })
    .expect("resolves");
    assert_eq!(mostly_excluded.file_count_bucket, M5WriteScopeFileCountBucket::Large);
}

#[test]
fn tree_rejects_malformed_input() {
    assert_eq!(
        resolve_write_scope_preview_tree(&M5WriteScopePreviewTreeResolutionInput {
            scope_label: " ".to_owned(),
            ..focused_tree()
        }),
        Err(M5WriteScopePreviewTreeResolutionError::EmptyScopeLabel)
    );
    assert_eq!(
        resolve_write_scope_preview_tree(&M5WriteScopePreviewTreeResolutionInput {
            included_file_count: 2,
            excluded_file_count: 2,
            total_file_count: 3,
            ..focused_tree()
        }),
        Err(M5WriteScopePreviewTreeResolutionError::FileCountMismatch)
    );
    assert_eq!(
        resolve_write_scope_preview_tree(&M5WriteScopePreviewTreeResolutionInput {
            scope_label: "scope https://leak.test".to_owned(),
            ..focused_tree()
        }),
        Err(M5WriteScopePreviewTreeResolutionError::ForbiddenTreeMaterial)
    );
}

// ---- write-scope-file-node resolver -------------------------------------

#[test]
fn node_included_offers_jump_provenance_and_toggle() {
    let resolved = resolve_write_scope_file_node(&included_node()).expect("resolves");
    assert_eq!(
        resolved.node_disposition,
        M5WriteScopeNodeDisposition::IncludedInScope
    );
    assert!(resolved.is_included_in_apply);
    assert!(resolved.exclusion_reason.is_none());
    assert!(resolved.preserves_file_in_preview);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5WriteScopeNodeAction::JumpToDiff,
            M5WriteScopeNodeAction::ViewProvenance,
            M5WriteScopeNodeAction::ToggleInclude,
        ]
    );
    assert_eq!(resolved.node_label, "src/main.rs");
}

#[test]
fn node_disposition_ladder_is_blocking_first() {
    // Policy-blocked wins even over a conflict.
    let policy = resolve_write_scope_file_node(&M5WriteScopeFileNodeResolutionInput {
        is_policy_blocked: true,
        has_conflict: true,
        ..included_node()
    })
    .expect("resolves");
    assert_eq!(
        policy.node_disposition,
        M5WriteScopeNodeDisposition::PolicyBlockedExcluded
    );
    assert!(!policy.is_included_in_apply);
    assert_eq!(policy.exclusion_reason, Some(M5WriteScopeExclusionReason::PolicyBlocked));
    assert!(!policy
        .available_actions
        .contains(&M5WriteScopeNodeAction::ToggleInclude));

    // Conflict next.
    let conflict = resolve_write_scope_file_node(&M5WriteScopeFileNodeResolutionInput {
        has_conflict: true,
        ..included_node()
    })
    .expect("resolves");
    assert_eq!(conflict.node_disposition, M5WriteScopeNodeDisposition::ConflictHeld);
    assert_eq!(conflict.exclusion_reason, Some(M5WriteScopeExclusionReason::ConflictPending));
    assert!(conflict
        .available_actions
        .contains(&M5WriteScopeNodeAction::ResolveConflict));

    // Read-only next.
    let read_only = resolve_write_scope_file_node(&M5WriteScopeFileNodeResolutionInput {
        is_read_only: true,
        ..included_node()
    })
    .expect("resolves");
    assert_eq!(read_only.node_disposition, M5WriteScopeNodeDisposition::ReadOnlyExcluded);
    assert_eq!(read_only.exclusion_reason, Some(M5WriteScopeExclusionReason::ReadOnlyProtected));

    // Generated / managed next (excludable, included until opted out).
    let generated = resolve_write_scope_file_node(&M5WriteScopeFileNodeResolutionInput {
        managed_caveat: M5ManagedFileCaveat::GeneratedFile,
        ..included_node()
    })
    .expect("resolves");
    assert_eq!(generated.node_disposition, M5WriteScopeNodeDisposition::GeneratedExcludable);
    assert!(generated.is_included_in_apply);
    assert!(generated.touches_generated_or_managed);

    // Binary next.
    let binary = resolve_write_scope_file_node(&M5WriteScopeFileNodeResolutionInput {
        content_class: M5WriteScopeFileContentClass::BinaryBlob,
        diff_available: false,
        ..included_node()
    })
    .expect("resolves");
    assert_eq!(binary.node_disposition, M5WriteScopeNodeDisposition::BinaryIncluded);
    assert!(!binary.can_jump_to_diff);
    assert!(binary.preserves_file_in_preview);
}

#[test]
fn node_opt_out_of_generated_carries_reason() {
    let opted_out = resolve_write_scope_file_node(&M5WriteScopeFileNodeResolutionInput {
        managed_caveat: M5ManagedFileCaveat::GeneratedFile,
        opt_out_of_apply: true,
        ..included_node()
    })
    .expect("resolves");
    assert!(!opted_out.is_included_in_apply);
    assert_eq!(
        opted_out.exclusion_reason,
        Some(M5WriteScopeExclusionReason::GeneratedOptedOut)
    );
    assert!(opted_out
        .available_actions
        .contains(&M5WriteScopeNodeAction::ViewExclusionReason));

    let out_of_workspace = resolve_write_scope_file_node(&M5WriteScopeFileNodeResolutionInput {
        is_out_of_workspace: true,
        opt_out_of_apply: true,
        ..included_node()
    })
    .expect("resolves");
    assert_eq!(
        out_of_workspace.exclusion_reason,
        Some(M5WriteScopeExclusionReason::OutOfWorkspace)
    );
}

#[test]
fn node_metadata_only_stays_in_scope() {
    let metadata = resolve_write_scope_file_node(&M5WriteScopeFileNodeResolutionInput {
        content_class: M5WriteScopeFileContentClass::MetadataOnly,
        ..included_node()
    })
    .expect("resolves");
    assert_eq!(metadata.node_disposition, M5WriteScopeNodeDisposition::IncludedInScope);
    assert!(metadata.is_included_in_apply);
    assert!(metadata.preserves_file_in_preview);
}

#[test]
fn node_rejects_malformed_input() {
    assert_eq!(
        resolve_write_scope_file_node(&M5WriteScopeFileNodeResolutionInput {
            node_label: " ".to_owned(),
            ..included_node()
        }),
        Err(M5WriteScopeFileNodeResolutionError::EmptyNodeLabel)
    );
    assert_eq!(
        resolve_write_scope_file_node(&M5WriteScopeFileNodeResolutionInput {
            node_label: "s3://bucket/file".to_owned(),
            ..included_node()
        }),
        Err(M5WriteScopeFileNodeResolutionError::ForbiddenNodeMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_write_scope_preview_tree_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_WRITE_SCOPE_PREVIEW_TREE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_write_scope_preview_tree_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5WriteScopeConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5WriteScopeConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_write_scope_preview_tree_packet();
    for row in &packet.rows {
        for part in M5WriteScopeTreeAnatomyPart::MANDATORY {
            assert!(row.tree_anatomy_parts.contains(&part));
        }
        for part in M5WriteScopeNodeAnatomyPart::MANDATORY {
            assert!(row.node_anatomy_parts.contains(&part));
        }
        for field in M5WriteScopeTreeExportField::MANDATORY {
            assert!(row.tree_export_fields.contains(&field));
        }
        for field in M5WriteScopeNodeExportField::MANDATORY {
            assert!(row.node_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5HistoryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.tree_examples.is_empty());
        assert!(!row.node_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_write_scope_preview_tree_packet();
    let trees: Vec<&M5WriteScopePreviewTreeResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.tree_examples.iter())
        .collect();
    let nodes: Vec<&M5WriteScopeFileNodeResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.node_examples.iter())
        .collect();

    for posture in M5WriteScopeTreePosture::ALL {
        assert!(
            trees.iter().any(|c| c.resolved.tree_posture == posture),
            "no tree example exercises posture {}",
            posture.as_str()
        );
    }
    for disposition in M5WriteScopeNodeDisposition::ALL {
        assert!(
            nodes.iter().any(|c| c.resolved.node_disposition == disposition),
            "no node example exercises disposition {}",
            disposition.as_str()
        );
    }
    for action in M5WriteScopeTreeAction::ALL {
        assert!(
            trees.iter().any(|c| c.resolved.available_actions.contains(&action)),
            "no tree example exercises action {}",
            action.as_str()
        );
    }
    for action in M5WriteScopeNodeAction::ALL {
        assert!(
            nodes.iter().any(|c| c.resolved.available_actions.contains(&action)),
            "no node example exercises action {}",
            action.as_str()
        );
    }
    for reason in M5WriteScopeExclusionReason::ALL {
        assert!(
            nodes
                .iter()
                .any(|c| c.resolved.exclusion_reason == Some(reason)),
            "no node example exercises exclusion reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_write_scope_preview_tree_packet();
    for row in &packet.rows {
        for case in &row.tree_examples {
            assert!(case.is_self_consistent(), "tree case for {} drifted", row.consumer_surface.as_str());
            assert!(case.preserves_identity(), "tree case for {} lost identity", row.consumer_surface.as_str());
        }
        for case in &row.node_examples {
            assert!(case.is_self_consistent(), "node case for {} drifted", row.consumer_surface.as_str());
            assert!(case.preserves_identity(), "node case for {} lost identity", row.consumer_surface.as_str());
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5WriteScopeConsumerSurface::ImportPreview);
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.vocabulary_set.tree_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::VocabularySetDrift));
}

#[test]
fn mandatory_tree_anatomy_missing_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.rows[0]
        .tree_anatomy_parts
        .retain(|p| *p != M5WriteScopeTreeAnatomyPart::WorkspaceRootCue);
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::MandatoryTreeAnatomyMissing));
}

#[test]
fn mandatory_node_export_missing_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.rows[0]
        .node_export_fields
        .retain(|f| *f != M5WriteScopeNodeExportField::NodeDisposition);
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::MandatoryNodeExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.rows[0].tree_examples[0].resolved.can_apply = false;
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::ExampleResolutionDrift));
}

#[test]
fn tree_scope_coverage_unproven_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    // Replace every tree example with a focused one so the broad half of the coverage lint
    // fires.
    for row in &mut packet.rows {
        row.tree_examples = vec![M5WriteScopePreviewTreeResolutionCase::resolved(focused_tree())];
    }
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::TreeScopeCoverageUnproven));
}

#[test]
fn tree_managed_caveat_coverage_unproven_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    for row in &mut packet.rows {
        row.tree_examples = vec![M5WriteScopePreviewTreeResolutionCase::resolved(focused_tree())];
    }
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::TreeManagedCaveatCoverageUnproven));
}

#[test]
fn tree_apply_coverage_unproven_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    for row in &mut packet.rows {
        row.tree_examples = vec![M5WriteScopePreviewTreeResolutionCase::resolved(focused_tree())];
    }
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::TreeApplyCoverageUnproven));
}

#[test]
fn node_exclusion_coverage_unproven_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    // Replace every node example with an included one so the excluded half of the lint fires.
    for row in &mut packet.rows {
        row.node_examples = vec![M5WriteScopeFileNodeResolutionCase::resolved(included_node())];
    }
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::NodeExclusionCoverageUnproven));
}

#[test]
fn node_ineligible_preservation_unproven_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    for row in &mut packet.rows {
        row.node_examples = vec![M5WriteScopeFileNodeResolutionCase::resolved(included_node())];
    }
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::NodeIneligiblePreservationUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.rows[0].drops_ineligible_files = true;
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.governance_review.ineligible_files_never_dropped = false;
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.consumer_projection.node_disposition_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5WriteScopePreviewTreeViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_write_scope_preview_tree_packet().render_markdown_summary();
    for surface in M5WriteScopeConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_write_scope_preview_tree_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5WriteScopeConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5WriteScopeConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_write_scope_preview_tree_export()
        .expect("checked M5 write-scope preview tree export validates");
    assert_eq!(from_disk.packet_id, M5_WRITE_SCOPE_PREVIEW_TREE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_write_scope_preview_tree_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_write_scope_preview_tree_import_preview_preview_narrowed(),
        seeded_m5_write_scope_preview_tree_ai_apply_preview_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5WriteScopeConsumerSurface::ALL.len());
    }

    let import = seeded_m5_write_scope_preview_tree_import_preview_preview_narrowed();
    let row = import
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5WriteScopeConsumerSurface::ImportPreview)
        .expect("import-preview row present");
    assert_eq!(row.qualification, M5HistoryQualificationClass::Preview);

    let ai_apply = seeded_m5_write_scope_preview_tree_ai_apply_preview_beta_narrowed();
    let row = ai_apply
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5WriteScopeConsumerSurface::AiApplyPreview)
        .expect("ai-apply-preview row present");
    assert_eq!(row.qualification, M5HistoryQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let import: M5WriteScopePreviewTreePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-write-scope-preview-tree-primitive/import_preview_preview_narrowed.json"
    )))
    .expect("import-preview fixture parses");
    assert!(import.validate().is_empty());
    assert_eq!(
        import,
        seeded_m5_write_scope_preview_tree_import_preview_preview_narrowed()
    );

    let ai_apply: M5WriteScopePreviewTreePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-write-scope-preview-tree-primitive/ai_apply_preview_beta_narrowed.json"
    )))
    .expect("ai-apply-preview fixture parses");
    assert!(ai_apply.validate().is_empty());
    assert_eq!(
        ai_apply,
        seeded_m5_write_scope_preview_tree_ai_apply_preview_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_write_scope_preview_tree_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
