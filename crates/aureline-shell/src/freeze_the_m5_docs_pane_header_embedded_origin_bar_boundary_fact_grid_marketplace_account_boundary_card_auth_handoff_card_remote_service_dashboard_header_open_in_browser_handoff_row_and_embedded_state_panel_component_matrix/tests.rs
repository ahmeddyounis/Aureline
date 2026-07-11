use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_embedded_boundary_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_embedded_boundary_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5EmbeddedBoundaryComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5EmbeddedBoundaryComponentFamily::ALL.len()
    );
}

#[test]
fn frozen_boundary_disposition_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: a stale, offline, or provider-blocked pane stays in
    // one controlled token set and never reads as fresh first-party local truth.
    let tokens: Vec<&str> = M5EmbeddedBoundaryDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "live_first_party_local",
            "live_first_party_hosted",
            "live_provider_owned",
            "stale_snapshot",
            "offline_snapshot",
            "provider_blocked",
            "browser_handoff_only",
            "capability_limited",
            "not_evaluated",
        ]
    );
    assert!(M5EmbeddedBoundaryDisposition::LiveFirstPartyLocal.is_fresh_first_party_local());
    assert!(!M5EmbeddedBoundaryDisposition::StaleSnapshot.is_fresh_first_party_local());
}

#[test]
fn every_component_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_embedded_boundary_component_matrix();
    for row in &packet.component_rows {
        for label in M5EmbeddedRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs.contains(
                &row.component_family
                    .canonical_component_schema_ref()
                    .to_owned()
            ),
            "component {} does not point at its canonical schema",
            row.component_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.boundary_dispositions.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5EmbeddedAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_embedded_boundary_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.owner_classes.is_empty(),
            family.declares_owner_class(),
            "owner_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.data_exit_boundaries.is_empty(),
            family.declares_data_boundary(),
            "data_exit_boundaries presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.browser_handoff_kinds.is_empty(),
            family.declares_browser_handoff(),
            "browser_handoff_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.capability_limits.is_empty(),
            family.declares_capability_limits(),
            "capability_limits presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.freshness_states.is_empty(),
            family.declares_freshness(),
            "freshness_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.account_scopes.is_empty(),
            family.declares_account_scope(),
            "account_scopes presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_embedded_boundary_component_matrix();
    for disposition in M5EmbeddedBoundaryDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.boundary_dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for owner in WebviewOwnerClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.owner_classes.contains(&owner)),
            "no component declares owner class {}",
            owner.as_str()
        );
    }
    for boundary in BOUND_DATA_EXIT_BOUNDARIES {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.data_exit_boundaries.contains(&boundary)),
            "no component declares data-exit boundary {}",
            boundary.as_str()
        );
    }
    for kind in BrowserHandoffKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.browser_handoff_kinds.contains(&kind)),
            "no component declares browser-handoff kind {}",
            kind.as_str()
        );
    }
    for limit in CapabilityLimitClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.capability_limits.contains(&limit)),
            "no component declares capability limit {}",
            limit.as_str()
        );
    }
    for state in M5EmbeddedFreshnessState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.freshness_states.contains(&state)),
            "no component declares freshness state {}",
            state.as_str()
        );
    }
    for scope in M5EmbeddedAccountScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.account_scopes.contains(&scope)),
            "no component declares account scope {}",
            scope.as_str()
        );
    }
    for reason in M5EmbeddedDegradedReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no component declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5EmbeddedBoundaryComponentFamily::BoundaryFactGrid);
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.vocabulary_set.boundary_dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5EmbeddedRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    let own = M5EmbeddedBoundaryComponentFamily::DocsPaneHeader.canonical_component_schema_ref();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EmbeddedBoundaryComponentFamily::DocsPaneHeader)
        .expect("docs-pane header present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::ComponentSchemaRefMissing));
}

#[test]
fn boundary_disposition_missing_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.component_rows[0].boundary_dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::BoundaryDispositionMissing));
}

#[test]
fn origin_bar_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_embedded_boundary_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5EmbeddedBoundaryComponentFamily::EmbeddedOriginBar
            })
            .expect("embedded-origin bar present");
        let expected = if clear == 0 {
            row.owner_classes.clear();
            M5EmbeddedBoundaryComponentMatrixViolation::OwnerClassMissing
        } else {
            row.capability_limits.clear();
            M5EmbeddedBoundaryComponentMatrixViolation::CapabilityLimitsMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn auth_handoff_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_embedded_boundary_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5EmbeddedBoundaryComponentFamily::AuthHandoffCard)
            .expect("auth-handoff card present");
        let expected = if clear == 0 {
            row.browser_handoff_kinds.clear();
            M5EmbeddedBoundaryComponentMatrixViolation::BrowserHandoffMissing
        } else {
            row.account_scopes.clear();
            M5EmbeddedBoundaryComponentMatrixViolation::AccountScopeMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn boundary_fact_grid_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_embedded_boundary_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5EmbeddedBoundaryComponentFamily::BoundaryFactGrid)
            .expect("boundary-fact grid present");
        let expected = if clear == 0 {
            row.data_exit_boundaries.clear();
            M5EmbeddedBoundaryComponentMatrixViolation::DataBoundaryMissing
        } else {
            row.freshness_states.clear();
            M5EmbeddedBoundaryComponentMatrixViolation::FreshnessStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.component_rows[2].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::DegradedReasonMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.component_rows[1].imitates_native_permission_or_approval_ui = true;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.component_rows[6].hides_owner_origin_or_browser_fallback_in_menus_only = true;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.component_rows[7].renders_stale_or_blocked_as_fresh_first_party_truth = true;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.component_rows[4].embeds_high_risk_approval_without_native_step_up = true;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EmbeddedBoundaryComponentFamily::DocsPaneHeader)
        .expect("docs-pane header present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet
        .governance_review
        .no_embedded_surface_imitates_native_approval_chrome = false;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_boundary_source = false;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_embedded_boundary_component_matrix().render_markdown_summary();
    for family in M5EmbeddedBoundaryComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_embedded_boundary_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5EmbeddedBoundaryComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,canonical_schema,"));
    for family in M5EmbeddedBoundaryComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.canonical_component_schema_ref()),
            "csv missing canonical schema for {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_embedded_boundary_component_matrix_export()
        .expect("checked M5 embedded-boundary component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_embedded_boundary_component_matrix_export()
        .expect("checked M5 embedded-boundary component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_embedded_boundary_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_embedded_boundary_component_matrix_docs_pane_header_beta_narrowed(),
        seeded_m5_embedded_boundary_component_matrix_embedded_state_panel_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5EmbeddedBoundaryComponentFamily::ALL.len()
        );
    }

    let docs = seeded_m5_embedded_boundary_component_matrix_docs_pane_header_beta_narrowed();
    let row = docs
        .component_rows
        .iter()
        .find(|r| r.component_family == M5EmbeddedBoundaryComponentFamily::DocsPaneHeader)
        .expect("docs-pane header row present");
    assert_eq!(row.qualification, M5EmbeddedQualificationClass::Beta);

    let panel =
        seeded_m5_embedded_boundary_component_matrix_embedded_state_panel_preview_narrowed();
    let row = panel
        .component_rows
        .iter()
        .find(|r| r.component_family == M5EmbeddedBoundaryComponentFamily::EmbeddedStatePanel)
        .expect("embedded-state panel row present");
    assert_eq!(row.qualification, M5EmbeddedQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let docs: M5EmbeddedBoundaryComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-embedded-boundary-components/docs_pane_header_beta_narrowed.json"
    )))
        .expect("docs-pane-header fixture parses");
    assert!(docs.validate().is_empty());
    assert_eq!(
        docs,
        seeded_m5_embedded_boundary_component_matrix_docs_pane_header_beta_narrowed()
    );

    let panel: M5EmbeddedBoundaryComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-embedded-boundary-components/embedded_state_panel_preview_narrowed.json"
    )))
    .expect("embedded-state-panel fixture parses");
    assert!(panel.validate().is_empty());
    assert_eq!(
        panel,
        seeded_m5_embedded_boundary_component_matrix_embedded_state_panel_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_embedded_boundary_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.component_rows[0].scope_summary =
        "raw endpoint https://provider.example/docs leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5EmbeddedBoundaryComponentMatrixViolation::RawMaterialInExport));
}

#[test]
fn binding_refs_point_at_auth_boundary_object_model() {
    assert!(M5_EMBEDDED_BOUNDARY_BINDING_REFS.contains(&M5_AUTH_BOUNDARY_CONTRACT_DOC_REF));
    assert!(M5_EMBEDDED_BOUNDARY_BINDING_REFS.contains(&M5_BROWSER_HANDOFF_CARD_SCHEMA_REF));
    assert!(M5_EMBEDDED_BOUNDARY_BINDING_REFS.contains(&M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF));
    let packet = seeded_m5_embedded_boundary_component_matrix();
    for binding in M5_EMBEDDED_BOUNDARY_BINDING_REFS {
        assert!(
            packet.source_contract_refs.iter().any(|r| r == binding),
            "matrix omits auth-boundary binding ref {binding}"
        );
    }
}
