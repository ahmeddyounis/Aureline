use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_docs_browser_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DOCS_BROWSER_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_docs_browser_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5DocsBrowserComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5DocsBrowserComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_and_deployment_lines() {
    let packet = seeded_m5_docs_browser_component_matrix();
    for row in &packet.component_rows {
        for label in M5DocsRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5DocsAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_docs_browser_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.corpus_classes.is_empty(),
            family.is_search_bar(),
            "corpus_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.version_scopes.is_empty(),
            family.is_scope_switcher(),
            "version_scopes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.match_states.is_empty(),
            family.is_result_row(),
            "match_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.override_reasons.is_empty(),
            family.is_result_row(),
            "override_reasons presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.symbol_anchors.is_empty(),
            family.is_symbol_card(),
            "symbol_anchors presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.source_providers.is_empty(),
            family.is_source_badge(),
            "source_providers presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.freshness_states.is_empty(),
            family.is_source_badge(),
            "freshness_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.pack_states.is_empty(),
            family.is_pack_row(),
            "pack_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.stale_example_statuses.is_empty(),
            family.is_stale_example(),
            "stale_example_statuses presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.handoff_reasons.is_empty(),
            family.is_handoff_banner(),
            "handoff_reasons presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_docs_browser_component_matrix();
    for corpus in M5DocsCorpusClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.corpus_classes.contains(&corpus)),
            "no component declares corpus class {}",
            corpus.as_str()
        );
    }
    for scope in M5DocsVersionScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.version_scopes.contains(&scope)),
            "no component declares version scope {}",
            scope.as_str()
        );
    }
    for state in M5DocsMatchState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.match_states.contains(&state)),
            "no component declares match state {}",
            state.as_str()
        );
    }
    for reason in M5DocsOverrideReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.override_reasons.contains(&reason)),
            "no component declares override reason {}",
            reason.as_str()
        );
    }
    for anchor in M5DocsSymbolAnchor::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.symbol_anchors.contains(&anchor)),
            "no component declares symbol anchor {}",
            anchor.as_str()
        );
    }
    for provider in M5DocsSourceProvider::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.source_providers.contains(&provider)),
            "no component declares source provider {}",
            provider.as_str()
        );
    }
    for freshness in M5DocsFreshnessState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.freshness_states.contains(&freshness)),
            "no component declares freshness state {}",
            freshness.as_str()
        );
    }
    for pack in M5DocsPackState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.pack_states.contains(&pack)),
            "no component declares pack state {}",
            pack.as_str()
        );
    }
    for stale in M5DocsStaleExampleStatus::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.stale_example_statuses.contains(&stale)),
            "no component declares stale-example status {}",
            stale.as_str()
        );
    }
    for handoff in M5DocsHandoffReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.handoff_reasons.contains(&handoff)),
            "no component declares handoff reason {}",
            handoff.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5DocsBrowserComponentFamily::DocsResultRow);
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.vocabulary_set.corpus_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5DocsRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn search_bar_vocab_missing_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DocsBrowserComponentFamily::DocsSearchBar)
        .expect("search bar present");
    row.corpus_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::CorpusClassMissing));
}

#[test]
fn scope_switcher_vocab_missing_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DocsBrowserComponentFamily::DocsScopeSwitcher)
        .expect("scope switcher present");
    row.version_scopes.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::VersionScopeMissing));
}

#[test]
fn result_row_vocab_missing_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DocsBrowserComponentFamily::DocsResultRow)
        .expect("result row present");
    row.match_states.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::MatchStateMissing));

    let mut packet = seeded_m5_docs_browser_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DocsBrowserComponentFamily::DocsResultRow)
        .expect("result row present");
    row.override_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::OverrideReasonMissing));
}

#[test]
fn symbol_card_vocab_missing_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DocsBrowserComponentFamily::SymbolLinkedReferenceCard)
        .expect("symbol card present");
    row.symbol_anchors.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::SymbolAnchorMissing));
}

#[test]
fn source_badge_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_docs_browser_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5DocsBrowserComponentFamily::DocsSourceVersionBadge
            })
            .expect("source badge present");
        let expected = if clear == 0 {
            row.source_providers.clear();
            M5DocsBrowserMatrixViolation::SourceProviderMissing
        } else {
            row.freshness_states.clear();
            M5DocsBrowserMatrixViolation::FreshnessStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn pack_row_vocab_missing_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DocsBrowserComponentFamily::DocsPackRow)
        .expect("pack row present");
    row.pack_states.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::PackStateMissing));
}

#[test]
fn stale_example_vocab_missing_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DocsBrowserComponentFamily::StaleExampleFindingRow)
        .expect("stale example row present");
    row.stale_example_statuses.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::StaleExampleStatusMissing));
}

#[test]
fn handoff_banner_vocab_missing_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DocsBrowserComponentFamily::DocsHandoffBanner)
        .expect("handoff banner present");
    row.handoff_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::HandoffReasonMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.component_rows[0].masks_corpus_or_source_provenance = true;
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.component_rows[2].shows_stale_or_cached_as_live_current = true;
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.component_rows[4].invents_private_docs_status_grammar = true;
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.component_rows[7].hides_handoff_reason_or_override_reason = true;
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DocsBrowserComponentFamily::DocsSearchBar)
        .expect("search bar present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet
        .governance_review
        .no_component_invents_second_status_grammar = false;
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet
        .consumer_projection
        .handoff_surfaces_consume_handoff_reason_vocabulary = false;
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DocsBrowserMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_docs_browser_component_matrix().render_markdown_summary();
    for family in M5DocsBrowserComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_docs_browser_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5DocsBrowserComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5DocsBrowserComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_docs_browser_component_matrix_export()
        .expect("checked M5 docs browser matrix export validates");
    assert_eq!(packet.packet_id, M5_DOCS_BROWSER_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_docs_browser_component_matrix_export()
        .expect("checked M5 docs browser matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_docs_browser_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_docs_browser_component_matrix_stale_example_finding_row_beta_narrowed(),
        seeded_m5_docs_browser_component_matrix_handoff_banner_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5DocsBrowserComponentFamily::ALL.len()
        );
    }

    let stale = seeded_m5_docs_browser_component_matrix_stale_example_finding_row_beta_narrowed();
    let row = stale
        .component_rows
        .iter()
        .find(|r| r.component_family == M5DocsBrowserComponentFamily::StaleExampleFindingRow)
        .expect("stale-example-finding-row row present");
    assert_eq!(row.qualification, M5DocsQualificationClass::Beta);

    let handoff = seeded_m5_docs_browser_component_matrix_handoff_banner_preview_narrowed();
    let row = handoff
        .component_rows
        .iter()
        .find(|r| r.component_family == M5DocsBrowserComponentFamily::DocsHandoffBanner)
        .expect("docs-handoff-banner row present");
    assert_eq!(row.qualification, M5DocsQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let stale: M5DocsBrowserMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix/stale_example_finding_row_beta_narrowed.json"
    )))
    .expect("stale-example fixture parses");
    assert!(stale.validate().is_empty());
    assert_eq!(
        stale,
        seeded_m5_docs_browser_component_matrix_stale_example_finding_row_beta_narrowed()
    );

    let handoff: M5DocsBrowserMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix/handoff_banner_preview_narrowed.json"
    )))
    .expect("handoff fixture parses");
    assert!(handoff.validate().is_empty());
    assert_eq!(
        handoff,
        seeded_m5_docs_browser_component_matrix_handoff_banner_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_docs_browser_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
