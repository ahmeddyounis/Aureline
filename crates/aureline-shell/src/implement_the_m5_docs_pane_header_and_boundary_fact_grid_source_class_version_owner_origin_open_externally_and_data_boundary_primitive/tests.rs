use super::*;

fn clean_header_input() -> M5DocsPaneHeaderResolutionInput {
    M5DocsPaneHeaderResolutionInput {
        header_id: "docs-header:test".to_owned(),
        source_class: M5DocsSourceClass::ProjectLocal,
        owner_class: WebviewOwnerClass::FirstPartyEmbedded,
        owner_disclosed: true,
        pack_identity: "aureline-docs v2026.07".to_owned(),
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        last_updated_stated: true,
        capability_limits: vec![CapabilityLimitClass::NotNativeTrustChrome],
        handoff_required: false,
        open_externally_available: true,
        find_in_page_applicable: true,
        find_in_page_available: true,
        proof_fresh: true,
    }
}

fn clean_grid_input() -> M5BoundaryFactGridResolutionInput {
    M5BoundaryFactGridResolutionInput {
        grid_id: "boundary-grid:test".to_owned(),
        source_class: M5DocsSourceClass::ProjectLocal,
        data_exit_boundary: DataExitBoundary::NoPayloadLeavesProduct,
        data_boundary_stated: true,
        reading_posture: M5PaneReadingPosture::LocalReadingSafe,
        posture_stated: true,
        reading_trust_explained: true,
        trustworthy_for_in_product_reading: true,
        claims_approval_or_policy_authority: false,
        suitable_for_high_risk_approval: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_docs_boundary_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DOCS_BOUNDARY_CONTROLS_PACKET_ID);
}

#[test]
fn header_clean_names_source_and_is_distinguishable() {
    let resolved = resolve_docs_pane_header(clean_header_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.distinguishable_source);
    assert!(!resolved.hides_required_handoff);
    assert_eq!(resolved.source_class, "project_local");
    assert_eq!(resolved.owner_origin, "first_party_embedded");
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::LiveFirstPartyLocal
    );
    assert_eq!(
        resolved.next_action,
        M5DocsBoundaryNextAction::NoActionNeeded
    );
}

#[test]
fn header_unknown_source_degrades_never_clean() {
    let mut input = clean_header_input();
    input.source_class = M5DocsSourceClass::SourceUnknown;
    let resolved = resolve_docs_pane_header(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(!resolved.distinguishable_source);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DocsPaneHeaderDegradeReason::SourceClassUnstated)
    );
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::NotEvaluated
    );
}

#[test]
fn header_owner_undisclosed_degrades() {
    let mut input = clean_header_input();
    input.owner_disclosed = false;
    let resolved = resolve_docs_pane_header(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DocsPaneHeaderDegradeReason::OwnerOrOriginUnstated)
    );
}

#[test]
fn header_missing_pack_identity_degrades() {
    let mut input = clean_header_input();
    input.pack_identity = "  ".to_owned();
    let resolved = resolve_docs_pane_header(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DocsPaneHeaderDegradeReason::VersionOrPackIdentityMissing)
    );
}

#[test]
fn header_last_updated_unstated_degrades() {
    let mut input = clean_header_input();
    input.last_updated_stated = false;
    let resolved = resolve_docs_pane_header(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DocsPaneHeaderDegradeReason::LastUpdatedUnstated)
    );
}

#[test]
fn header_handoff_required_but_not_exposed_degrades_ac2() {
    let mut input = clean_header_input();
    input.source_class = M5DocsSourceClass::BrowserHandoffRequired;
    input.handoff_required = true;
    input.open_externally_available = false;
    let resolved = resolve_docs_pane_header(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.hides_required_handoff);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DocsPaneHeaderDegradeReason::HandoffRequiredButNotExposed)
    );
    assert_eq!(
        resolved.next_action,
        M5DocsBoundaryNextAction::OpenExternally
    );
}

#[test]
fn header_stale_freshness_never_reads_fresh_first_party() {
    let mut input = clean_header_input();
    input.freshness = M5EmbeddedFreshnessState::StaleSnapshot;
    let resolved = resolve_docs_pane_header(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::StaleSnapshot
    );
}

#[test]
fn header_empty_id_and_forbidden_material_error() {
    let mut input = clean_header_input();
    input.header_id = "".to_owned();
    assert_eq!(
        resolve_docs_pane_header(input).unwrap_err(),
        M5DocsBoundaryResolutionError::EmptyHeaderId
    );

    let mut input = clean_header_input();
    input.pack_identity = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_docs_pane_header(input).unwrap_err(),
        M5DocsBoundaryResolutionError::ForbiddenMaterial
    );
}

#[test]
fn grid_clean_names_boundary_and_posture() {
    let resolved = resolve_boundary_fact_grid(clean_grid_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.masquerades_as_approval_authority);
    assert_eq!(resolved.data_exit_boundary, "no_payload_leaves_product");
    assert_eq!(resolved.reading_posture, "local_reading_safe");
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::LiveFirstPartyLocal
    );
}

#[test]
fn grid_masquerade_degrades_ac2() {
    let mut input = clean_grid_input();
    input.claims_approval_or_policy_authority = true;
    let resolved = resolve_boundary_fact_grid(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.masquerades_as_approval_authority);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BoundaryFactGridDegradeReason::MasqueradesAsApprovalAuthority)
    );
}

#[test]
fn grid_suitable_for_high_risk_approval_is_masquerade() {
    let mut input = clean_grid_input();
    input.suitable_for_high_risk_approval = true;
    let resolved = resolve_boundary_fact_grid(input).unwrap();
    assert!(resolved.masquerades_as_approval_authority);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BoundaryFactGridDegradeReason::MasqueradesAsApprovalAuthority)
    );
}

#[test]
fn grid_data_boundary_unstated_degrades_first() {
    let mut input = clean_grid_input();
    input.data_boundary_stated = false;
    input.claims_approval_or_policy_authority = true;
    let resolved = resolve_boundary_fact_grid(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BoundaryFactGridDegradeReason::DataBoundaryUnstated)
    );
}

#[test]
fn grid_posture_unknown_degrades() {
    let mut input = clean_grid_input();
    input.reading_posture = M5PaneReadingPosture::PostureUnknown;
    let resolved = resolve_boundary_fact_grid(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BoundaryFactGridDegradeReason::OfflineOrMirroredPostureUnstated)
    );
}

#[test]
fn grid_reading_trust_unexplained_degrades() {
    let mut input = clean_grid_input();
    input.reading_trust_explained = false;
    let resolved = resolve_boundary_fact_grid(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BoundaryFactGridDegradeReason::ReadingTrustNotExplained)
    );
}

#[test]
fn grid_empty_id_and_forbidden_material_error() {
    let mut input = clean_grid_input();
    input.grid_id = "   ".to_owned();
    assert_eq!(
        resolve_boundary_fact_grid(input).unwrap_err(),
        M5DocsBoundaryResolutionError::EmptyGridId
    );

    let mut input = clean_grid_input();
    input.grid_id = "grid://leak".to_owned();
    assert_eq!(
        resolve_boundary_fact_grid(input).unwrap_err(),
        M5DocsBoundaryResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_docs_boundary_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_docs_boundary_controls();
    packet.vocabulary_set.source_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_docs_boundary_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_docs_boundary_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_DOCS_PANE_HEADER_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_docs_boundary_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DocsBoundaryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_docs_boundary_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5DocsBoundaryExportField::BoundaryDispositions);
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_docs_boundary_controls();
    packet.controls_rows[0].boundary_fact_grid_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_docs_boundary_controls();
    // Force a clean grid to also read as masquerading — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.boundary_fact_grid_examples[0].degrade_reason = None;
    row.boundary_fact_grid_examples[0].masquerades_as_approval_authority = true;
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_docs_boundary_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.masquerades_as_native_approval_chrome = true,
            1 => row.hides_owner_origin_or_handoff_in_menus_only = true,
            2 => row.renders_stale_or_blocked_as_fresh_first_party_truth = true,
            _ => row.embeds_high_risk_approval_without_native_step_up = true,
        }
        assert!(packet
            .validate()
            .contains(&M5DocsBoundaryControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn ac1_not_proven_when_unknown_source_example_removed() {
    let mut packet = seeded_m5_docs_boundary_controls();
    // Drop every header that degrades to SourceClassUnstated.
    for row in &mut packet.controls_rows {
        row.docs_pane_header_examples.retain(|ex| {
            ex.degrade_reason != Some(M5DocsPaneHeaderDegradeReason::SourceClassUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::Ac1NotProven));
}

#[test]
fn ac1_not_proven_when_a_source_class_uncovered() {
    let mut packet = seeded_m5_docs_boundary_controls();
    // Drop every clean browser-handoff header so the required source-class coverage breaks.
    for row in &mut packet.controls_rows {
        row.docs_pane_header_examples
            .retain(|ex| !(ex.is_clean() && ex.source_class == "browser_handoff_required"));
    }
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::Ac1NotProven));
}

#[test]
fn ac2_not_proven_when_masquerade_example_removed() {
    let mut packet = seeded_m5_docs_boundary_controls();
    for row in &mut packet.controls_rows {
        row.boundary_fact_grid_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5BoundaryFactGridDegradeReason::MasqueradesAsApprovalAuthority)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::Ac2NotProven));
}

#[test]
fn ac2_not_proven_when_hidden_handoff_example_removed() {
    let mut packet = seeded_m5_docs_boundary_controls();
    for row in &mut packet.controls_rows {
        row.docs_pane_header_examples.retain(|ex| {
            ex.degrade_reason != Some(M5DocsPaneHeaderDegradeReason::HandoffRequiredButNotExposed)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::Ac2NotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_docs_boundary_controls();
    packet
        .governance_review
        .no_pane_masquerades_as_approval_authority = false;
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_docs_boundary_controls();
    packet
        .consumer_projection
        .support_export_reads_single_boundary_source = false;
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_docs_boundary_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_docs_boundary_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_docs_boundary_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5DocsBoundaryControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_docs_boundary_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_docs_boundary_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_docs_boundary_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_docs_boundary_controls_export()
        .expect("checked M5 docs-boundary controls export validates");
    assert_eq!(from_disk.packet_id, M5_DOCS_BOUNDARY_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_docs_boundary_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_docs_boundary_controls_docs_browser_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EmbeddedConsumerSurface::DocsBrowserUi)
        .unwrap();
    assert_eq!(row.qualification, M5EmbeddedQualificationClass::Beta);

    let preview = seeded_m5_docs_boundary_controls_embedded_webview_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EmbeddedConsumerSurface::EmbeddedWebviewUi)
        .unwrap();
    assert_eq!(row.qualification, M5EmbeddedQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5DocsBoundaryControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-docs-pane-header-boundary-fact-grid-controls/docs_browser_beta_narrowed.json"
    )))
    .expect("docs-browser fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_docs_boundary_controls_docs_browser_beta_narrowed()
    );

    let preview: M5DocsBoundaryControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-docs-pane-header-boundary-fact-grid-controls/embedded_webview_preview_narrowed.json"
    )))
    .expect("embedded-webview fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_docs_boundary_controls_embedded_webview_preview_narrowed()
    );
}
