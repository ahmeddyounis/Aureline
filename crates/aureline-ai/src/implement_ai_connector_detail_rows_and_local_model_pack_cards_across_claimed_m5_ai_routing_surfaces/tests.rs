use super::*;

fn warm_connector(id: &str) -> M5AiConnectorRowResolutionInput {
    M5AiConnectorRowResolutionInput {
        canonical_id: id.to_owned(),
        publisher_source: "aureline first-party".to_owned(),
        execution_locus: M5AiExecutionLocus::InProcessLocal,
        declared_capabilities: vec![M5AiConnectorCapability::ReadOnlyQuery],
        auth_posture: M5AiAuthPosture::Unauthenticated,
        policy_blocked: false,
        reachable: true,
        session_warmed: true,
        discloses_side_effects: false,
    }
}

fn ready_model(id: &str) -> M5AiModelPackResolutionInput {
    M5AiModelPackResolutionInput {
        model_identity: id.to_owned(),
        digest: "sha256-test".to_owned(),
        size_on_disk_mb: 4200,
        hardware_expectation_label: "8 GB RAM".to_owned(),
        required_memory_mb: 4000,
        available_memory_mb: 16000,
        requires_accelerator: false,
        accelerator_present: false,
        pack_state: M5AiModelPackState::Installed,
        provenance_verified: true,
        requires_network_fetch: false,
    }
}

// ---- connector resolver -------------------------------------------------

#[test]
fn connector_warm_read_only_is_invocable_without_authority() {
    let resolved = resolve_connector_detail_row(&warm_connector("connector.x")).expect("resolves");
    assert_eq!(resolved.connector_readiness, M5AiConnectorReadiness::Warm);
    assert!(resolved.is_invocable);
    assert!(!resolved.needs_attention);
    assert!(resolved.locus_is_local);
    assert!(!resolved.requires_authority_before_invocation);
}

#[test]
fn connector_readiness_ladder_is_blocking_first() {
    let policy = resolve_connector_detail_row(&M5AiConnectorRowResolutionInput {
        policy_blocked: true,
        ..warm_connector("c")
    })
    .expect("resolves");
    assert_eq!(
        policy.connector_readiness,
        M5AiConnectorReadiness::PolicyBlocked
    );
    assert!(policy.needs_attention);
    assert!(!policy.is_invocable);

    let unavailable = resolve_connector_detail_row(&M5AiConnectorRowResolutionInput {
        reachable: false,
        ..warm_connector("c")
    })
    .expect("resolves");
    assert_eq!(
        unavailable.connector_readiness,
        M5AiConnectorReadiness::Unavailable
    );

    let cold = resolve_connector_detail_row(&M5AiConnectorRowResolutionInput {
        session_warmed: false,
        ..warm_connector("c")
    })
    .expect("resolves");
    assert_eq!(cold.connector_readiness, M5AiConnectorReadiness::Cold);
    assert!(cold.is_invocable);
}

#[test]
fn connector_side_effecting_capability_requires_disclosure_and_authority() {
    // A side-effecting capability without disclosure is rejected.
    let undisclosed = resolve_connector_detail_row(&M5AiConnectorRowResolutionInput {
        declared_capabilities: vec![M5AiConnectorCapability::FileMutation],
        discloses_side_effects: false,
        ..warm_connector("c")
    });
    assert_eq!(
        undisclosed,
        Err(M5AiConnectorRowResolutionError::SideEffectingCapabilityUndisclosed)
    );

    // Disclosed, it resolves and requires authority before invocation.
    let disclosed = resolve_connector_detail_row(&M5AiConnectorRowResolutionInput {
        declared_capabilities: vec![M5AiConnectorCapability::FileMutation],
        discloses_side_effects: true,
        ..warm_connector("c")
    })
    .expect("resolves");
    assert!(disclosed.requires_authority_before_invocation);
    assert!(disclosed.discloses_side_effects);
}

#[test]
fn connector_authenticated_read_only_still_requires_authority() {
    let resolved = resolve_connector_detail_row(&M5AiConnectorRowResolutionInput {
        auth_posture: M5AiAuthPosture::OauthDelegated,
        ..warm_connector("c")
    })
    .expect("resolves");
    assert!(resolved.requires_authority_before_invocation);
}

#[test]
fn connector_rejects_malformed_input() {
    assert_eq!(
        resolve_connector_detail_row(&M5AiConnectorRowResolutionInput {
            canonical_id: "  ".to_owned(),
            ..warm_connector("c")
        }),
        Err(M5AiConnectorRowResolutionError::EmptyCanonicalId)
    );
    assert_eq!(
        resolve_connector_detail_row(&M5AiConnectorRowResolutionInput {
            publisher_source: "".to_owned(),
            ..warm_connector("c")
        }),
        Err(M5AiConnectorRowResolutionError::EmptyPublisherSource)
    );
    assert_eq!(
        resolve_connector_detail_row(&M5AiConnectorRowResolutionInput {
            declared_capabilities: vec![],
            ..warm_connector("c")
        }),
        Err(M5AiConnectorRowResolutionError::EmptyCapabilities)
    );
    assert_eq!(
        resolve_connector_detail_row(&M5AiConnectorRowResolutionInput {
            publisher_source: "https://leak.test/x".to_owned(),
            ..warm_connector("c")
        }),
        Err(M5AiConnectorRowResolutionError::ForbiddenConnectorMaterial)
    );
}

// ---- model resolver -----------------------------------------------------

#[test]
fn model_installed_fits_is_ready_selectable_local_cached() {
    let resolved = resolve_local_model_pack_card(&ready_model("model.x")).expect("resolves");
    assert_eq!(resolved.hardware_fit, M5AiModelHardwareFit::Fits);
    assert_eq!(
        resolved.model_pack_readiness,
        M5AiModelPackReadiness::ReadySelectable
    );
    assert_eq!(
        resolved.offline_posture,
        M5AiModelOfflinePosture::LocalCached
    );
    assert!(resolved.is_selectable);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5AiModelPackAction::Select,
            M5AiModelPackAction::Verify,
            M5AiModelPackAction::Remove
        ]
    );
}

#[test]
fn model_readiness_ladder_is_blocking_first() {
    // Quarantine holds before hardware.
    let quarantined = resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
        pack_state: M5AiModelPackState::Quarantined,
        required_memory_mb: 32000,
        ..ready_model("m")
    })
    .expect("resolves");
    assert_eq!(
        quarantined.model_pack_readiness,
        M5AiModelPackReadiness::VerificationHeld
    );
    assert!(quarantined.needs_attention);
    assert_eq!(
        quarantined.available_actions,
        vec![M5AiModelPackAction::Verify, M5AiModelPackAction::Remove]
    );

    // Unverified provenance holds.
    let unverified = resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
        provenance_verified: false,
        ..ready_model("m")
    })
    .expect("resolves");
    assert_eq!(
        unverified.model_pack_readiness,
        M5AiModelPackReadiness::VerificationHeld
    );

    // Then hardware over memory blocks.
    let over_memory = resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
        required_memory_mb: 32000,
        ..ready_model("m")
    })
    .expect("resolves");
    assert_eq!(
        over_memory.hardware_fit,
        M5AiModelHardwareFit::ExceedsMemory
    );
    assert_eq!(
        over_memory.model_pack_readiness,
        M5AiModelPackReadiness::HardwareBlocked
    );
    assert_eq!(
        over_memory.available_actions,
        vec![
            M5AiModelPackAction::RunHardwareFitCheck,
            M5AiModelPackAction::Remove
        ]
    );

    // Missing accelerator blocks.
    let no_accel = resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
        requires_accelerator: true,
        accelerator_present: false,
        ..ready_model("m")
    })
    .expect("resolves");
    assert_eq!(
        no_accel.hardware_fit,
        M5AiModelHardwareFit::RequiresAccelerator
    );
    assert_eq!(
        no_accel.model_pack_readiness,
        M5AiModelPackReadiness::HardwareBlocked
    );
}

#[test]
fn model_offline_and_mirror_states_surface_locality() {
    let offline = resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
        pack_state: M5AiModelPackState::OfflineOnly,
        ..ready_model("m")
    })
    .expect("resolves");
    assert_eq!(
        offline.model_pack_readiness,
        M5AiModelPackReadiness::OfflineReady
    );
    assert_eq!(
        offline.offline_posture,
        M5AiModelOfflinePosture::RunsFullyOffline
    );
    assert!(offline.offline_posture.is_offline_capable());

    let mirrored = resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
        pack_state: M5AiModelPackState::Mirrored,
        ..ready_model("m")
    })
    .expect("resolves");
    assert_eq!(
        mirrored.model_pack_readiness,
        M5AiModelPackReadiness::MirroredReady
    );
    assert_eq!(
        mirrored.offline_posture,
        M5AiModelOfflinePosture::MirrorServed
    );

    let update = resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
        pack_state: M5AiModelPackState::UpdateAvailable,
        requires_network_fetch: true,
        ..ready_model("m")
    })
    .expect("resolves");
    assert_eq!(
        update.model_pack_readiness,
        M5AiModelPackReadiness::UpdatePending
    );
    assert_eq!(
        update.offline_posture,
        M5AiModelOfflinePosture::RequiresNetworkFetch
    );
    assert!(update
        .available_actions
        .contains(&M5AiModelPackAction::Update));
}

#[test]
fn model_fits_with_swap_when_memory_is_tight() {
    let tight = resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
        required_memory_mb: 13000,
        available_memory_mb: 16000,
        ..ready_model("m")
    })
    .expect("resolves");
    assert_eq!(tight.hardware_fit, M5AiModelHardwareFit::FitsWithSwap);
    // Fits-with-swap is not blocking — still selectable.
    assert_eq!(
        tight.model_pack_readiness,
        M5AiModelPackReadiness::ReadySelectable
    );
}

#[test]
fn model_rejects_malformed_input() {
    assert_eq!(
        resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
            model_identity: " ".to_owned(),
            ..ready_model("m")
        }),
        Err(M5AiModelPackResolutionError::EmptyModelIdentity)
    );
    assert_eq!(
        resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
            digest: "".to_owned(),
            ..ready_model("m")
        }),
        Err(M5AiModelPackResolutionError::EmptyDigest)
    );
    assert_eq!(
        resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
            hardware_expectation_label: "".to_owned(),
            ..ready_model("m")
        }),
        Err(M5AiModelPackResolutionError::EmptyHardwareExpectation)
    );
    assert_eq!(
        resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
            size_on_disk_mb: 0,
            ..ready_model("m")
        }),
        Err(M5AiModelPackResolutionError::ZeroDiskSize)
    );
    assert_eq!(
        resolve_local_model_pack_card(&M5AiModelPackResolutionInput {
            digest: "s3://bucket/model".to_owned(),
            ..ready_model("m")
        }),
        Err(M5AiModelPackResolutionError::ForbiddenModelMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_ai_connector_model_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_AI_CONNECTOR_MODEL_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_ai_connector_model_primitive_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5AiConnectorModelConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5AiConnectorModelConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_ai_connector_model_primitive_packet();
    for row in &packet.rows {
        for part in M5AiConnectorAnatomyPart::MANDATORY {
            assert!(row.connector_anatomy_parts.contains(&part));
        }
        for part in M5AiModelPackAnatomyPart::MANDATORY {
            assert!(row.model_anatomy_parts.contains(&part));
        }
        for field in M5AiConnectorExportField::MANDATORY {
            assert!(row.connector_export_fields.contains(&field));
        }
        for field in M5AiModelPackExportField::MANDATORY {
            assert!(row.model_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5AiAccessibilityRoute::KeyboardFocusable));
        assert!(!row.connector_examples.is_empty());
        assert!(!row.model_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_ai_connector_model_primitive_packet();
    let conn: Vec<&M5AiConnectorRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.connector_examples.iter())
        .collect();
    let model: Vec<&M5AiModelPackResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.model_examples.iter())
        .collect();

    for readiness in M5AiConnectorReadiness::ALL {
        assert!(
            conn.iter()
                .any(|c| c.resolved.connector_readiness == readiness),
            "no connector example exercises readiness {}",
            readiness.as_str()
        );
    }
    for locus in M5AiExecutionLocus::ALL {
        assert!(
            conn.iter().any(|c| c.resolved.execution_locus == locus),
            "no connector example exercises locus {}",
            locus.as_str()
        );
    }
    for readiness in M5AiModelPackReadiness::ALL {
        assert!(
            model
                .iter()
                .any(|c| c.resolved.model_pack_readiness == readiness),
            "no model example exercises readiness {}",
            readiness.as_str()
        );
    }
    for fit in M5AiModelHardwareFit::ALL {
        assert!(
            model.iter().any(|c| c.resolved.hardware_fit == fit),
            "no model example exercises hardware fit {}",
            fit.as_str()
        );
    }
    for posture in M5AiModelOfflinePosture::ALL {
        assert!(
            model.iter().any(|c| c.resolved.offline_posture == posture),
            "no model example exercises offline posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_ai_connector_model_primitive_packet();
    for row in &packet.rows {
        for case in &row.connector_examples {
            assert!(
                case.is_self_consistent(),
                "connector case for {} drifted",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.model_examples {
            assert!(
                case.is_self_consistent(),
                "model case for {} drifted",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5AiConnectorModelConsumerSurface::RouteInspector);
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet.vocabulary_set.execution_loci.pop();
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_connector_anatomy_missing_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet.rows[0]
        .connector_anatomy_parts
        .retain(|p| *p != M5AiConnectorAnatomyPart::ExecutionLocusCue);
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::MandatoryConnectorAnatomyMissing));
}

#[test]
fn mandatory_model_export_missing_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet.rows[0]
        .model_export_fields
        .retain(|f| *f != M5AiModelPackExportField::DiskSizeMb);
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::MandatoryModelExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet.rows[0].model_examples[0].resolved.is_selectable = false;
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn connector_example_missing_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet.rows[1].connector_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::ConnectorExampleMissing));
}

#[test]
fn connector_availability_coverage_unproven_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    // Replace every connector example with a warm, invocable one so the needs-attention
    // half of the coverage lint fires.
    for row in &mut packet.rows {
        row.connector_examples = vec![M5AiConnectorRowResolutionCase::resolved(warm_connector(
            "connector.all-warm",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::ConnectorAvailabilityCoverageUnproven));
}

#[test]
fn model_readiness_coverage_unproven_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    // Replace every model example with a freely selectable one so the needs-attention
    // half of the coverage lint fires.
    for row in &mut packet.rows {
        row.model_examples = vec![M5AiModelPackResolutionCase::resolved(ready_model(
            "model.all-ready",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::ModelReadinessCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet.rows[0].shows_blocked_connector_as_ready = true;
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet
        .governance_review
        .connector_readiness_never_masks_blocked = false;
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet
        .consumer_projection
        .offline_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AiConnectorModelPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_ai_connector_model_primitive_packet().render_markdown_summary();
    for surface in M5AiConnectorModelConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_ai_connector_model_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5AiConnectorModelConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5AiConnectorModelConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_ai_connector_model_primitive_export()
        .expect("checked M5 ai connector/local-model primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_AI_CONNECTOR_MODEL_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_ai_connector_model_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_ai_connector_model_primitive_route_inspector_preview_narrowed(),
        seeded_m5_ai_connector_model_primitive_evidence_view_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5AiConnectorModelConsumerSurface::ALL.len()
        );
    }

    let route = seeded_m5_ai_connector_model_primitive_route_inspector_preview_narrowed();
    let row = route
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5AiConnectorModelConsumerSurface::RouteInspector)
        .expect("route-inspector row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Preview);

    let evidence = seeded_m5_ai_connector_model_primitive_evidence_view_beta_narrowed();
    let row = evidence
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5AiConnectorModelConsumerSurface::EvidenceView)
        .expect("evidence-view row present");
    assert_eq!(row.qualification, M5AiQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let route: M5AiConnectorModelPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces/route_inspector_preview_narrowed.json"
    )))
    .expect("route-inspector fixture parses");
    assert!(route.validate().is_empty());
    assert_eq!(
        route,
        seeded_m5_ai_connector_model_primitive_route_inspector_preview_narrowed()
    );

    let evidence: M5AiConnectorModelPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces/evidence_view_beta_narrowed.json"
    )))
    .expect("evidence-view fixture parses");
    assert!(evidence.validate().is_empty());
    assert_eq!(
        evidence,
        seeded_m5_ai_connector_model_primitive_evidence_view_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_ai_connector_model_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
